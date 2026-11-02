#!/bin/bash
# Test the check functions in setup_bench.sh on temp paths. Run as root: sudo bash test_setup_bench.sh
# It does not touch /tmp/fxbin, /opt/benchproxy or the users. It deletes only its own temp dir.
set -u
[ "$(id -u)" -eq 0 ] || { echo "FAIL: run as root"; exit 1; }
id benchagent >/dev/null 2>&1 || { echo "FAIL: user benchagent is missing (run setup_bench.sh first)"; exit 1; }
HERE=$(cd "$(dirname "$0")" && pwd)
SETUP_BENCH_LIB=1 . "$HERE/setup_bench.sh"
set +e +o pipefail  # the sourced file sets -e; this test checks exit codes itself
T=$(mktemp -d); chmod 755 "$T"; fails=0
# expect <want: pass|fail> <name> <command...>; each call runs in a subshell, because die exits.
expect() { local want=$1 name=$2; shift 2; local got=pass
  ( "$@" ) >/dev/null 2>&1 || got=fail
  if [ "$got" = "$want" ]; then echo "PASS  $name"; else echo "FAIL  $name (want $want, got $got)"; fails=$((fails+1)); fi; }

mkdir -m 755 "$T/good"
mkdir -m 755 "$T/agentdir"; chown benchagent "$T/agentdir"
mkdir -m 777 "$T/wide"; chmod 777 "$T/wide"
ln -s "$T/good" "$T/link"
expect pass "root dir 755 is accepted" check_root_dir "$T/good"
expect fail "symlink to a good dir is refused" check_root_dir "$T/link"
expect fail "dir owned by benchagent is refused" check_root_dir "$T/agentdir"
expect fail "root dir mode 777 is refused" check_root_dir "$T/wide"
expect fail "missing dir is refused by check" check_root_dir "$T/none"
expect pass "ensure creates a missing dir" ensure_root_dir "$T/new"
[ "$(stat -c '%u %a' "$T/new")" = "0 755" ] && echo "PASS  created dir is uid 0 mode 755" || { echo "FAIL  created dir mode/owner"; fails=$((fails+1)); }
expect fail "ensure refuses a symlink" ensure_root_dir "$T/link"
ln -s "$T/missing-target" "$T/dangling"
expect fail "ensure refuses a dangling symlink" ensure_root_dir "$T/dangling"
[ ! -e "$T/missing-target" ] && echo "PASS  dangling symlink target not created" || { echo "FAIL  dangling target created"; fails=$((fails+1)); }

install -o root -m 755 /dev/null "$T/good/fx"
install -o benchagent -m 755 /dev/null "$T/agentfx"
ln -s "$T/good/fx" "$T/fxlink"
expect pass "root-owned file is accepted" check_root_file "$T/good/fx"
expect fail "symlink file is refused" check_root_file "$T/fxlink"
expect fail "benchagent-owned file is refused" check_root_file "$T/agentfx"

expect pass "root file 755 is not writable by benchagent" agent_cannot_write "$T/good/fx"
expect pass "root dir 755 is not writable by benchagent" agent_cannot_write "$T/good"
expect fail "benchagent-owned file is writable (refused)" agent_cannot_write "$T/agentfx"
expect fail "missing path is refused" agent_cannot_write "$T/none"
bad_user_check() { AGENT_USER_CHECK=no_such_user_x agent_cannot_write "$1"; }
out=$( bad_user_check "$T/good/fx" 2>&1 ); rc=$?
if [ $rc -ne 0 ] && printf '%s' "$out" | grep -q "failed (exit 1)"; then echo "PASS  runuser error (no such user) stops the check with an error"; else echo "FAIL  runuser error not caught (rc=$rc: $out)"; fails=$((fails+1)); fi

rm -f "$T/good/fx" "$T/agentfx" "$T/fxlink" "$T/link" "$T/dangling"
rmdir "$T/good" "$T/agentdir" "$T/wide" "$T/new" "$T"
[ $fails -eq 0 ] && echo "OVERALL PASS" || echo "OVERALL FAIL ($fails)"
exit $fails
