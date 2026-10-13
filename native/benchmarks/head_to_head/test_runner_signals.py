#!/usr/bin/env python3
"""Test-only (no Gemini, no key): run_isolated.sh in BENCH_TEST_FAKE_AGENT mode against a local fake upstream.
(a) SIGINT / SIGTERM to the runner mid-run -> no benchagent procs, current_run_id empty, no proxy on the port.
(b) SIGKILL to the runner mid-run -> state left behind; the next runner cleans all 3 before its gate."""
import json, os, signal, socket, subprocess, sys, tempfile, threading, time
from http.server import ThreadingHTTPServer, BaseHTTPRequestHandler
HERE = os.path.dirname(os.path.abspath(__file__)); RUNNER = os.path.join(HERE, "run_isolated.sh")
RIDF = "/opt/benchproxy/run/current_run_id"; AU = "benchagent"
PROXY_COMMIT = os.environ.get("PROXY_COMMIT", "eff7d948ed92fdb0dd64a5b7819ff93193a258e7")

class Up(BaseHTTPRequestHandler):
    def log_message(self, *a): pass
    def do_POST(self):
        self.rfile.read(int(self.headers.get("Content-Length") or 0))
        b = b'{"choices":[{"message":{"role":"assistant","content":"ok"},"finish_reason":"stop","index":0}],"usage":{"prompt_tokens":1,"completion_tokens":1}}'
        self.send_response(200); self.send_header("Content-Type", "application/json"); self.send_header("Content-Length", str(len(b))); self.end_headers(); self.wfile.write(b)
up = ThreadingHTTPServer(("127.0.0.1", 0), Up); threading.Thread(target=up.serve_forever, daemon=True).start()
UPSTREAM = "http://127.0.0.1:%d" % up.server_address[1]

fake = tempfile.NamedTemporaryFile("w", suffix=".sh", delete=False, dir="/tmp")
fake.write("""#!/bin/bash
trap '' TERM INT
curl -s -m 5 -X POST -H 'Content-Type: application/json' -d '{"model":"m","messages":[]}' "$HARNESS_BASE_URL/chat/completions" >/dev/null
setsid bash -c "trap '' TERM HUP INT; exec -a fake-stray-grandchild sleep 600" </dev/null >/dev/null 2>&1 &
exec -a fake-agent sleep 600
"""); fake.close(); os.chmod(fake.name, 0o755)
runs = tempfile.mkdtemp(prefix="sigtest-runs-"); os.chmod(runs, 0o777)

def sh(c): return subprocess.run(c, shell=True, capture_output=True, text=True).stdout.strip()
def n_agent(): return int(sh(f"pgrep -u {AU} | wc -l"))
def rid(): return sh(f"cat {RIDF}")
def proxies(): return sh("pgrep -u benchproxy -af proxy.py")
def listening(port):
    s = socket.socket(); s.settimeout(0.5)
    try: return s.connect_ex(("127.0.0.1", port)) == 0
    finally: s.close()
def start(label):
    env = dict(os.environ, PROXY_COMMIT=PROXY_COMMIT, BENCH_TEST_FAKE_AGENT=fake.name, BENCH_TEST_UPSTREAM=UPSTREAM, RUNS=runs, IV="0")
    return subprocess.Popen([RUNNER, "T1", label], env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
                            restore_signals=True, start_new_session=True)
def wait_mid_run(timeout=120):
    t = time.time()
    while time.time() - t < timeout:
        for d in os.listdir(runs):
            p = os.path.join(runs, d)
            if os.path.exists(os.path.join(p, "agent_cmd.txt")) and n_agent() >= 2 and os.path.exists(os.path.join(p, "proxy_port")):
                return p, int(open(os.path.join(p, "proxy_port")).read())
        time.sleep(0.5)
    raise SystemExit("FAIL: run never reached the agent")
def clear_runs():
    for d in os.listdir(runs): subprocess.run(["rm", "-rf", os.path.join(runs, d)])

ok = True
def check(name, cond, detail=""):
    global ok; ok &= bool(cond); print(("PASS " if cond else "FAIL ") + name + (("  [" + detail + "]") if detail else ""))

for sig in (signal.SIGINT, signal.SIGTERM):
    clear_runs(); p = start("H1"); d, port = wait_mid_run()
    check(f"{sig.name}: mid-run state present", n_agent() >= 2 and rid() != "" and listening(port), f"agents={n_agent()} rid={rid()!r} port={port}")
    p.send_signal(sig); rc = p.wait(60)
    check(f"{sig.name}: runner exit code", rc == (130 if sig == signal.SIGINT else 143), f"rc={rc}")
    check(f"{sig.name}: no benchagent process left", n_agent() == 0, f"{n_agent()}")
    check(f"{sig.name}: current_run_id empty", rid() == "", repr(rid()))
    check(f"{sig.name}: no proxy on port {port}", not listening(port) and proxies() == "", proxies())

clear_runs(); p = start("H1"); d, port = wait_mid_run()
os.killpg(p.pid, signal.SIGKILL) if False else p.kill(); p.wait(30); time.sleep(1)
left = (n_agent(), rid(), listening(port))
check("SIGKILL: state left behind (agents, run id, proxy)", left[0] >= 1 and left[1] != "" and left[2], f"agents={left[0]} rid={left[1]!r} listening={left[2]}")
p2 = start("H2")
d2 = None; t = time.time()
while time.time() - t < 120 and d2 is None:
    for x in os.listdir(runs):
        if x != os.path.basename(d) and os.path.exists(os.path.join(runs, x, "proxy_port")): d2 = os.path.join(runs, x)
    time.sleep(0.3)
check("next runner: gate passed (no ABORTED, reached proxy start)", d2 is not None and not os.path.exists(os.path.join(d2, "ABORTED.txt")))
check("next runner: old proxy port gone", not listening(port))
check("next runner: own run id set (old one replaced)", rid() == os.path.basename(d2 or ""), repr(rid()))
p2.send_signal(signal.SIGTERM); rc2 = p2.wait(60)
check("next runner: TERM cleanup after", n_agent() == 0 and rid() == "" and proxies() == "", f"rc={rc2}")
up.shutdown(); os.unlink(fake.name); subprocess.run(["sudo", "-n", "rm", "-rf", runs])
print("OVERALL", "PASS" if ok else "FAIL"); sys.exit(0 if ok else 1)
