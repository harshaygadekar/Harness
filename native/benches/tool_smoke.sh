#!/usr/bin/env bash
# Smoke: coding tools work under allowlist; deny-all still holds. Not a beat-fx claim.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${ROOT}/target/release/harness"
WS=$(mktemp -d)
echo 'fn main() { println!("hi"); }' > "$WS/main.rs"
(cd "$ROOT" && cargo build --release -p harness-cli)
echo "== deny-all =="
"$BIN" run --prompt hi --workspace "$WS"
echo "== coding allow =="
"$BIN" run --prompt hi --workspace "$WS" --allow coding
echo "ok"
