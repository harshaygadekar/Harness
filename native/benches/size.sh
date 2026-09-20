#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${ROOT}/target/release/harness"
if [[ ! -x "$BIN" ]]; then
  (cd "$ROOT" && cargo build --release -p harness-cli)
fi
bytes=$(stat -c%s "$BIN")
python3 - <<PY
b=$bytes
print(f"binary_bytes={b}")
print(f"binary_kib={b/1024:.1f}")
print(f"binary_mib={b/1024/1024:.3f}")
PY
