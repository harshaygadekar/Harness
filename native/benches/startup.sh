#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${ROOT}/target/release/harness"
if [[ ! -x "$BIN" ]]; then
  (cd "$ROOT" && cargo build --release -p harness-cli)
fi
N="${1:-50}"
python3 - <<PY
import subprocess, statistics, time
bin = "$BIN"
n = int("$N")
samples = []
for _ in range(n):
    t0 = time.perf_counter()
    subprocess.run([bin, "version"], check=True, capture_output=True)
    samples.append((time.perf_counter() - t0) * 1e3)
print(f"startup_ms_median={statistics.median(samples):.3f}")
print(f"startup_ms_p95={sorted(samples)[int(0.95*(n-1))]:.3f}")
print(f"n={n}")
PY
