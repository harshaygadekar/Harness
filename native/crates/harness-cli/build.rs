// Records the Git commit the harness binary was built from (HARNESS_BUILD_SHA).
// metrics.json `commit_sha` is the cwd repo HEAD at run time; this is the binary's own SHA.
use std::process::Command;

fn main() {
    let sha = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into());
    let dirty = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .ok()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);
    let v = if dirty && sha != "unknown" { format!("{sha}-dirty") } else { sha };
    println!("cargo:rustc-env=HARNESS_BUILD_SHA={v}");
    // Rebuild when HEAD moves.
    if let Ok(o) = Command::new("git").args(["rev-parse", "--git-dir"]).output() {
        let gd = String::from_utf8_lossy(&o.stdout).trim().to_string();
        if !gd.is_empty() {
            println!("cargo:rerun-if-changed={gd}/HEAD");
            println!("cargo:rerun-if-changed={gd}/index");
        }
    }
    println!("cargo:rerun-if-changed=build.rs");
}
