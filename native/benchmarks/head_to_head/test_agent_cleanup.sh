#!/bin/bash
# Dry run (no model, no proxy): a fake agent that ignores SIGTERM and spawns a setsid'd child
# (its own session, outside the agent's process group) that also ignores SIGTERM.
# Runs it exactly like run_isolated.sh does, with a 2 s timeout, then shows agent_cleanup.sh kills all.
set -u; AU=${AGENT_USER:-benchagent}; HERE=$(cd "$(dirname "$0")" && pwd)
FAKE=$(mktemp /tmp/fake_agent.XXXX.sh)
cat > "$FAKE" <<'F'
#!/bin/bash
trap '' TERM
setsid bash -c "trap '' TERM HUP; exec -a fake-stray-grandchild sleep 600" </dev/null >/dev/null 2>&1 &
bash -c "trap '' TERM; exec -a fake-child sleep 600" &
sleep 600
F
chmod 755 "$FAKE"
[ "$(pgrep -u "$AU" | wc -l)" -eq 0 ] || { echo "FAIL precondition: $AU already has processes"; exit 1; }
t0=$(date +%s%3N)
sudo -n -u "$AU" env -i PATH=/usr/bin:/bin HOME=/home/$AU setsid --wait timeout -k 1 2 "$FAKE"; rc=$?
echo "fake agent exit=$rc after $(( $(date +%s%3N) - t0 )) ms"
echo "alive after timeout (before cleanup): $(pgrep -u "$AU" | wc -l)"; ps -o pid,pgid,sid,args -u "$AU" | sed 's/^/  /'
n=$("$HERE/agent_cleanup.sh" "$AU"); echo "stray_procs_after_cleanup=$n"
rm -f "$FAKE"
[ "$n" = 0 ] && echo "PASS: cleanup killed every $AU process" || { echo "FAIL"; exit 1; }
