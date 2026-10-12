#!/bin/bash
# Start oai_stream_proxy as the `benchproxy` user with upstream-key injection.
# The key file (/home/benchproxy/keys.env, 600, dir 700) is read by the proxy process only;
# the proxy's environment holds no key. Agents talk to http://127.0.0.1:PORT with a dummy key.
# usage: start_isolated.sh <port> <min_interval_ms> <hold_dir (writable by benchproxy)> [key_name]
set -eu
PORT=${1:-8787}; IV=${2:-5000}; HOLD_DIR=${3:-/tmp/benchproxy}; KEY_NAME=${4:-GEMINI_API_KEY}
HERE=$(cd "$(dirname "$0")" && pwd)
sudo -n install -d -o benchproxy -g benchproxy -m 755 "$HOLD_DIR"
exec sudo -n -u benchproxy env -i PATH=/usr/bin:/bin \
  PROXY_KEY_FILE=/home/benchproxy/keys.env PROXY_KEY_NAME="$KEY_NAME" PROXY_SIG_MODE=reinject \
  python3 "$HERE/proxy.py" "$PORT" --min-interval-ms "$IV" --hold-file "$HOLD_DIR/proxy_hold.json" --usage-file "$HOLD_DIR/proxy_usage.jsonl"
