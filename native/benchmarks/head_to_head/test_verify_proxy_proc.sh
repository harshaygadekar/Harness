#!/bin/bash
# Quick test for verify_proxy_proc.sh with synthetic ps output (no processes started).
set -u; H=$(cd "$(dirname "$0")" && pwd); E=/opt/benchproxy/abc/proxy.py; t=$(mktemp); fail=0
check() { printf '%s\n' "$2" > "$t"; PS_INPUT=$t "$H/verify_proxy_proc.sh" "$E" >/dev/null; r=$?; [ "$r" = "$1" ] && echo "ok   $3" || { echo "FAIL $3 (exit $r)"; fail=1; }; }
L1="10 root sudo sudo -n -u benchproxy env -i python3 $E 8787"
L2="11 benchproxy python3 python3 $E 8787 --min-interval-ms 5000"
check 0 "$L1
$L2" "one pinned proxy as benchproxy"
check 1 "$L1" "no proxy"
check 1 "$L2
12 benchproxy python3 python3 $E 8788" "two proxies"
check 1 "13 box python3 python3 $E 8787" "wrong user"
check 1 "14 benchproxy python3 python3 /workspace/wt/oai_stream_proxy/proxy.py 8787" "unpinned path"
check 1 "$L2
15 box python3 python3 /tmp/x/proxy.py 9999" "extra stray proxy"
rm -f "$t"; exit $fail
