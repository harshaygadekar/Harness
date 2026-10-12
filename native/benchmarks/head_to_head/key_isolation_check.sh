#!/bin/bash
# Key isolation pre-run check. Run as the operator (needs sudo); the probes run as the AGENT user.
# Prints only PASS/FAIL per case and match counts; never prints file contents or key values.
# usage: key_isolation_check.sh [agent_user=benchagent] [workspace_dir] [repo_dir]
set -u
AU=${1:-benchagent}; WS=${2:-/tmp/keyiso-ws}; REPO=${3:-$(cd "$(dirname "$0")/../../.." && pwd)}
KEYFILE=${KEYFILE:-/home/benchproxy/keys.env}
PAT='AIza[0-9A-Za-z_-]{10,}|csk-[0-9A-Za-z]{10,}'
fail=0
as_agent() { sudo -n -u "$AU" env -i PATH=/usr/local/bin:/usr/bin:/bin HOME="/home/$AU" bash -c "cd '$WS' && $1"; }
count() { grep -Eoa "$PAT" | wc -l; }
# Exact-value match count: root reads the key file and greps stdin; values never printed.
exact() { sudo -n bash -c "grep -aoF -f <(cut -d= -f2- '$KEYFILE' | grep .) | wc -l"; }
res() { if [ "$2" = PASS ]; then echo "PASS  $1  ($3)"; else echo "FAIL  $1  ($3)"; fail=1; fi; }

[ "$AU" != benchproxy ] || { echo "FAIL  agent user must not be benchproxy"; exit 1; }
sudo -n install -d -o "$AU" -m 755 "$WS"
PP=$(pgrep -u benchproxy -f "proxy.py" | head -1)

# 1. key file by absolute path
if as_agent "cat '$KEYFILE'" >/dev/null 2>&1; then res "cat key file" FAIL "readable"; else
  n=$(as_agent "cat '$KEYFILE'" 2>/dev/null | count); res "cat key file" PASS "unreadable, matches=$n"; fi
# 2. every readable /proc/*/environ
n=$(as_agent 'for f in /proc/[0-9]*/environ; do cat "$f" 2>/dev/null | tr "\0" "\n"; done' | tee >(exact > /tmp/.kic_e) | count); sleep 0.2; n=$((n + $(cat /tmp/.kic_e))); rm -f /tmp/.kic_e
[ "$n" -eq 0 ] && res "cat /proc/*/environ" PASS "matches=0" || res "cat /proc/*/environ" FAIL "matches=$n"
# 3. proxy environ specifically
if [ -z "$PP" ]; then res "proxy running as benchproxy" FAIL "no proxy pid"; else
  u=$(ps -o user= -p "$PP" | tr -d ' ')
  [ "$u" = benchproxy ] && res "ps -o user= proxy" PASS "user=$u" || res "ps -o user= proxy" FAIL "user=$u"
  if as_agent "cat /proc/$PP/environ" >/dev/null 2>&1; then
    n=$(as_agent "cat /proc/$PP/environ" | count); res "cat /proc/<proxy>/environ" FAIL "readable, matches=$n"
  else res "cat /proc/<proxy>/environ" PASS "unreadable"; fi
fi
# 4. .env in workspace and repo
for f in "$WS/.env" "$REPO/.env"; do
  n=$(as_agent "cat '$f' 2>/dev/null" | count)
  [ "$n" -eq 0 ] && res "cat $f" PASS "matches=0" || res "cat $f" FAIL "matches=$n"
done
# 5. env
n=$(as_agent env | count); [ "$n" -eq 0 ] && res "env" PASS "matches=0" || res "env" FAIL "matches=$n"
# 6. grep over agent home, workspace, repo: real key values (exact) must be 0.
#    Prefix-shaped matches are reported too; test fixtures hold fake prefix-shaped strings.
for d in "/home/$AU" "$WS" "$REPO"; do
  e=$(as_agent "grep -rah --exclude-dir=target -e '' '$d' 2>/dev/null" | exact)
  n=$(as_agent "grep -rEoa --exclude-dir=target '$PAT' '$d' 2>/dev/null" | wc -l)
  [ "$e" -eq 0 ] && res "grep keys in $d" PASS "real-key matches=0, prefix-shaped=$n" || res "grep keys in $d" FAIL "real-key matches=$e, prefix-shaped=$n"
done
# 7. operator home must not be readable by the agent
if as_agent "ls /home/box" >/dev/null 2>&1; then res "operator home /home/box" FAIL "listable by agent"; else res "operator home /home/box" PASS "not listable"; fi
# 8. agent user must not have sudo (sudo would reach the key file)
if as_agent "sudo -n true" >/dev/null 2>&1; then res "agent sudo" FAIL "passwordless sudo"; else res "agent sudo" PASS "no sudo"; fi
[ $fail -eq 0 ] && echo "OVERALL PASS" || echo "OVERALL FAIL"
exit $fail
