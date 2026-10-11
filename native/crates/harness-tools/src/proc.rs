//! Shared child-process runner for tools: concurrent stdout/stderr drain,
//! bounded head+tail capture, own process group, group kill on timeout.
//! Output still goes through the core scrub in the runner before the model sees it.

use std::collections::VecDeque;
use std::io::Read;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Bytes kept from the start and from the end of each stream.
pub const HEAD_BYTES: usize = 32 * 1024;
pub const TAIL_BYTES: usize = 32 * 1024;
/// After the direct child exits, how long to wait for the pipes to close
/// (a lingering grandchild may still hold them) before killing the group.
const DRAIN_GRACE: Duration = Duration::from_secs(2);

#[derive(Debug, Default)]
pub struct Capture {
    head: Vec<u8>,
    tail: VecDeque<u8>,
    total: usize,
    head_cap: usize,
    tail_cap: usize,
}

impl Capture {
    fn new(head_cap: usize, tail_cap: usize) -> Self {
        Self { head_cap, tail_cap, ..Default::default() }
    }
    fn push(&mut self, mut buf: &[u8]) {
        self.total += buf.len();
        if self.head.len() < self.head_cap {
            let n = (self.head_cap - self.head.len()).min(buf.len());
            self.head.extend_from_slice(&buf[..n]);
            buf = &buf[n..];
        }
        if self.tail_cap == 0 {
            return;
        }
        if buf.len() >= self.tail_cap {
            self.tail.clear();
            self.tail.extend(&buf[buf.len() - self.tail_cap..]);
        } else {
            let over = (self.tail.len() + buf.len()).saturating_sub(self.tail_cap);
            self.tail.drain(..over);
            self.tail.extend(buf);
        }
    }
    pub fn total(&self) -> usize {
        self.total
    }
    pub fn omitted(&self) -> usize {
        self.total - self.head.len() - self.tail.len()
    }
    /// Head, an omitted-bytes marker if anything was dropped, then tail.
    pub fn render(&self) -> String {
        let mut s = String::from_utf8_lossy(&self.head).into_owned();
        let tail: Vec<u8> = self.tail.iter().copied().collect();
        if self.omitted() > 0 {
            s.push_str(&format!(
                "\n…[omitted {} bytes of {} total; showing first {} and last {}]…\n",
                self.omitted(),
                self.total,
                self.head.len(),
                tail.len()
            ));
        }
        s.push_str(&String::from_utf8_lossy(&tail));
        s
    }
}

#[derive(Debug)]
pub struct Outcome {
    /// None if killed by signal / timeout.
    pub code: Option<i32>,
    pub success: bool,
    pub timed_out: bool,
    pub stdout: Capture,
    pub stderr: Capture,
}

fn drain<R: Read + Send + 'static>(
    mut r: R,
    head: usize,
    tail: usize,
    tx: mpsc::Sender<()>,
) -> thread::JoinHandle<Capture> {
    thread::spawn(move || {
        let mut cap = Capture::new(head, tail);
        let mut buf = [0u8; 8192];
        loop {
            match r.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => cap.push(&buf[..n]),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => break,
            }
        }
        let _ = tx.send(());
        cap
    })
}

fn kill_group(pgid: u32) {
    // SAFETY: plain syscall; negative pid targets the whole process group.
    unsafe {
        libc::kill(-(pgid as libc::pid_t), libc::SIGKILL);
    }
}

/// Run `cmd` with piped output in its own process group.
pub fn run(mut cmd: Command, timeout: Duration) -> std::io::Result<Outcome> {
    run_with_caps(&mut cmd, timeout, HEAD_BYTES, TAIL_BYTES)
}

pub fn run_with_caps(
    cmd: &mut Command,
    timeout: Duration,
    head: usize,
    tail: usize,
) -> std::io::Result<Outcome> {
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0);
    let mut child = cmd.spawn()?;
    let pgid = child.id();
    let (tx, rx) = mpsc::channel();
    let out_h = drain(child.stdout.take().expect("piped"), head, tail, tx.clone());
    let err_h = drain(child.stderr.take().expect("piped"), head, tail, tx);

    let start = Instant::now();
    let mut timed_out = false;
    let status = loop {
        match child.try_wait()? {
            Some(st) => break Some(st),
            None if start.elapsed() > timeout => {
                timed_out = true;
                kill_group(pgid);
                break child.wait().ok();
            }
            None => thread::sleep(Duration::from_millis(20)),
        }
    };
    // Wait for both pipes to close; if a grandchild still holds them, kill the group.
    let mut closed = 0;
    let deadline = Instant::now() + DRAIN_GRACE;
    while closed < 2 {
        let left = deadline.saturating_duration_since(Instant::now());
        match rx.recv_timeout(left) {
            Ok(()) => closed += 1,
            Err(_) => {
                kill_group(pgid);
                break;
            }
        }
    }
    if timed_out {
        // Reap anything left in the group is not possible for non-children;
        // the group kill above makes them exit and the pipes close.
        kill_group(pgid);
    }
    let stdout = out_h.join().unwrap_or_default();
    let stderr = err_h.join().unwrap_or_default();
    Ok(Outcome {
        code: status.and_then(|s| s.code()),
        success: !timed_out && status.map(|s| s.success()).unwrap_or(false),
        timed_out,
        stdout,
        stderr,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sh(script: &str) -> Command {
        let mut c = Command::new("bash");
        c.arg("-c").arg(script);
        c
    }

    #[test]
    fn large_output_returns_quickly_with_tail() {
        let t = Instant::now();
        let o = run(
            sh("for i in $(seq 1 10000); do echo \"line $i ..................................\"; done; echo LAST_LINE_MARKER >&2; echo FINAL_STDOUT_LINE"),
            Duration::from_secs(20),
        )
        .unwrap();
        assert!(t.elapsed() < Duration::from_secs(10), "took {:?}", t.elapsed());
        assert!(!o.timed_out);
        assert!(o.stdout.total() > 400_000, "total {}", o.stdout.total());
        let r = o.stdout.render();
        assert!(r.starts_with("line 1 "));
        assert!(r.contains("…[omitted "));
        assert!(r.trim_end().ends_with("FINAL_STDOUT_LINE"));
        assert!(r.len() <= HEAD_BYTES + TAIL_BYTES + 200);
        assert!(o.stderr.render().contains("LAST_LINE_MARKER"));
    }

    #[test]
    fn capture_keeps_head_and_tail() {
        let mut c = Capture::new(4, 4);
        c.push(b"abcdef");
        c.push(b"ghij");
        assert_eq!(c.render(), "abcd\n…[omitted 2 bytes of 10 total; showing first 4 and last 4]…\nghij");
        let mut small = Capture::new(4, 4);
        small.push(b"abc");
        assert_eq!(small.render(), "abc");
    }

    #[test]
    fn timeout_kills_grandchild() {
        let marker = std::env::temp_dir().join(format!("harness-proc-gc-{}", std::process::id()));
        let _ = std::fs::remove_file(&marker);
        let t = Instant::now();
        let script = format!("(sleep 3; touch {}) & sleep 30", marker.display());
        let o = run(sh(&script), Duration::from_millis(500)).unwrap();
        assert!(o.timed_out);
        assert!(t.elapsed() < Duration::from_secs(5), "took {:?}", t.elapsed());
        std::thread::sleep(Duration::from_secs(4));
        assert!(!marker.exists(), "grandchild survived the timeout");
    }

    #[test]
    fn lingering_grandchild_after_exit_does_not_hang() {
        let t = Instant::now();
        // Parent exits at once; background child keeps the pipe open.
        let o = run(sh("sleep 30 & echo done"), Duration::from_secs(20)).unwrap();
        assert!(!o.timed_out);
        assert!(t.elapsed() < Duration::from_secs(6), "took {:?}", t.elapsed());
        assert!(o.stdout.render().contains("done"));
    }
}
