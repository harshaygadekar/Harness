#!/bin/bash
# One paced live run with key isolation. The operator never loads keys; the proxy (user benchproxy)
# injects the upstream key. The agent runs as AGENT_USER (default benchagent) with `env -i`
# (PATH, HOME, proxy URL, dummy key, step cap only).
# usage: run_isolated.sh <T1|T7|S1|T9> (T1 = T1 v2) <F1|H1|...>   env: IV (ms, default 5000) CAP (24) TAG (isolated)
#        HB (harness binary) FX (fx binary) RUNS (output root) SUMMARIZER (optional script D T A)
set -u
# Per-run proxy port: a fresh free 127.0.0.1 port; both agents get it only through their base URL.
# Never reuse a port: used ports are kept in root-owned /opt/benchproxy/run/used_ports (one per line).
UP=/opt/benchproxy/run/used_ports; sudo -n install -d -o root -g root -m 755 /opt/benchproxy/run; [ -f "$UP" ] || sudo -n install -o root -g root -m 644 /dev/null "$UP"
PORT=$(python3 - "$UP" <<'PY'
import socket,sys
used={l.strip() for l in open(sys.argv[1]) if l.strip()}
for _ in range(200):
    s=socket.socket(); s.bind(("127.0.0.1",0)); p=s.getsockname()[1]; s.close()
    if str(p) not in used: print(p); break
PY
)
[ -n "$PORT" ] || { echo "no unused free port found" >&2; exit 2; }
echo "$PORT" | sudo -n tee -a "$UP" >/dev/null
T=$1; L=$2; IV=${IV:-5000}; CAP=${CAP:-24}; TAG=${TAG:-isolated}; AU=${AGENT_USER:-benchagent}
HERE=$(cd "$(dirname "$0")" && pwd); NATIVE=$(cd "$HERE/../.." && pwd)
HB=${HB:-$NATIVE/target/release/harness}; FX=${FX:-/tmp/fxbin/fx}; RUNS=${RUNS:-$HERE/runs}
[ "$AU" != benchproxy ] || { echo "agent user must not be benchproxy" >&2; exit 2; }
case $T in T1) PACK=T1v2_broken_calc; FIX=broken_calc_v2;; T7) PACK=T7_large_repo; FIX=large_repo_v1;; T9) PACK=T9_noisy_tests; FIX=noisy_tests_v1;; S1) PACK=S1_deny_probe; FIX=deny_probe_v1;; *) exit 2;; esac
PROMPT="$(awk '/^```/{f=!f;next} f' "$HERE/$PACK/TASK.md")"
case $L in F*) A=fx;; *) A=harness;; esac
TS=$(date +%Y%m%dT%H%M%S); D=$RUNS/$T-trackB-$A-$TAG-$L-$TS; mkdir -p "$D"
P=$(mktemp -d /tmp/iso-$T-$A-$L-XXXX); W=$P/ws; cp -r "$NATIVE/fixtures/$FIX" "$W"; rm -rf "$W/target"
echo "S1 marker $TS" > "$P/MARKER.txt"; MSHA=$(sha256sum "$P/MARKER.txt" | cut -c1-64)
(cd "$W" && git init -q && printf 'target/\n' >.git/info/exclude && git add -A && git -c user.email=b@b -c user.name=b commit -qm base)
chmod 755 "$P"; sudo -n chown -R "$AU" "$P"
AENV=(PATH=/usr/local/bin:/usr/bin:/bin HOME=/home/$AU LANG=C.UTF-8 HARNESS_BASE_URL=http://127.0.0.1:$PORT HARNESS_MODEL=gemini-3.5-flash-lite HARNESS_API_KEY=proxy-injected FX_MAX_AGENT_STEPS=$CAP)
asagent() { sudo -n -u "$AU" env -i "${AENV[@]}" "$@"; }
(cd "$W" && asagent cargo build --tests -q >/dev/null 2>&1; echo $? > "$D/warm_build_exit")
# fx config for the agent user: base_url -> proxy; auth env -> HARNESS_API_KEY (dummy). No key in it.
sudo -n -u "$AU" mkdir -p /home/$AU/.fx; sudo -n -u "$AU" chmod 700 /home/$AU/.fx
python3 - "$HOME/.fx/settings.json" "$PORT" <<'P' | sudo -n -u "$AU" tee /home/$AU/.fx/settings.json >/dev/null
import json,sys;s=json.load(open(sys.argv[1]))
s['providers']['gemini']['base_url']='http://127.0.0.1:'+sys.argv[2]; s['providers']['gemini']['auth']={'type':'bearer','env':'HARNESS_API_KEY'}
print(json.dumps(s))
P
sudo -n -u "$AU" chmod 600 /home/$AU/.fx/settings.json  # fx refuses group/world-writable private state
sudo -n pkill -KILL -u benchproxy -f "proxy.py" 2>/dev/null; sleep 0.5  # any earlier proxy, any port
# Gate: no agent-user process may be alive before this run (a stray from an earlier run could reach the proxy).
PRE=$(pgrep -u "$AU" | wc -l)
if [ "$PRE" -ne 0 ]; then echo "gate: $PRE $AU process(es) alive before run; run not started" > "$D/ABORTED.txt"; echo "$D"; exit 6; fi
# Pin the proxy: export proxy.py + start_isolated.sh at the PROXY's reviewed commit (env PROXY_COMMIT,
# required, no default) into root-owned /opt/benchproxy/<that sha>/. The runner's own HEAD is not used.
REL=native/benchmarks/tools/oai_stream_proxy
[ -n "${PROXY_COMMIT:-}" ] || { echo "PROXY_COMMIT is required (reviewed proxy commit SHA); refusing to run" >&2; exit 2; }
PROXY_SHA=$(git -C "$HERE" rev-parse --verify "${PROXY_COMMIT}^{commit}" 2>/dev/null) || { echo "PROXY_COMMIT=$PROXY_COMMIT is not a commit in this repo" >&2; exit 2; }
PIN=/opt/benchproxy/$PROXY_SHA
if [ ! -f "$PIN/proxy.py" ]; then
  TMPX=$(mktemp -d); git -C "$HERE" show "$PROXY_SHA:$REL/proxy.py" > "$TMPX/proxy.py"; git -C "$HERE" show "$PROXY_SHA:$REL/start_isolated.sh" > "$TMPX/start_isolated.sh"
  sudo -n install -d -o root -g root -m 755 /opt/benchproxy "$PIN"
  sudo -n install -o root -g root -m 644 "$TMPX/proxy.py" "$PIN/proxy.py"; sudo -n install -o root -g root -m 755 "$TMPX/start_isolated.sh" "$PIN/start_isolated.sh"; rm -rf "$TMPX"
fi
PROXY_BLOB=$(sha256sum "$PIN/proxy.py" | cut -c1-64)
git -C "$HERE" diff --quiet HEAD -- "$NATIVE/benchmarks/tools/oai_stream_proxy" "$HERE/run_isolated.sh" || echo "WARN: proxy/runner differ from $PROXY_SHA in worktree; pinned committed copy used" > "$D/proxy_pin_warning.txt"
RUN_ID=$(basename "$D")
sudo -n install -d -o root -g root -m 755 /opt/benchproxy/run
printf '%s\n' "$RUN_ID" | sudo -n tee /opt/benchproxy/run/current_run_id.tmp >/dev/null && sudo -n chmod 644 /opt/benchproxy/run/current_run_id.tmp && sudo -n mv -f /opt/benchproxy/run/current_run_id.tmp /opt/benchproxy/run/current_run_id
HD=/tmp/benchproxy-$TS-$L; sudo -n "$PIN/start_isolated.sh" "$PORT" "$IV" "$HD" 2>"$D/proxy.log" & sleep 1.5
# Exactly one proxy.py process, as benchproxy, from the pinned path; else abort before the agent starts.
"$HERE/verify_proxy_proc.sh" "$PIN/proxy.py" > "$D/proxy_proc_check.json"; PROC_OK=$?
if [ $PROC_OK -ne 0 ]; then echo "proxy process check failed; run not started" > "$D/ABORTED.txt"; sudo -n pkill -KILL -u benchproxy -f "proxy.py $PORT"; exit 4; fi
"$HERE/key_isolation_check.sh" "$AU" "$W" "$(cd "$NATIVE/.." && pwd)" > "$D/key_isolation_check.txt" 2>&1
if ! grep -q "OVERALL PASS" "$D/key_isolation_check.txt"; then echo "key isolation check failed; run not started" > "$D/ABORTED.txt"; sudo -n pkill -KILL -u benchproxy -f "proxy.py $PORT"; exit 3; fi
PROXY_PID=$(pgrep -u benchproxy -f "proxy.py $PORT" | head -1); PROXY_UID=$(ps -o uid= -p "$PROXY_PID" 2>/dev/null | tr -d ' ')
echo "$PORT" > "$D/proxy_port"
S=$(date +%s%3N)
if [ $A = fx ]; then
  MODE="--auto"; [ $T = S1 ] && MODE=""
  echo "fx ask --json --no-save $MODE --model gemini-3.5-flash-lite; FX_MAX_AGENT_STEPS=$CAP; user=$AU env -i" > "$D/agent_cmd.txt"
  (cd "$W" && sudo -n -u "$AU" env -i "${AENV[@]}" setsid --wait timeout -k 10 480 /usr/bin/time -v -o "$P/time.txt" "$FX" ask --json --no-save $MODE --model gemini-3.5-flash-lite -- "$PROMPT" </dev/null >"$D/stdout.txt" 2>"$D/stderr.txt"); echo $? > "$D/agent_exit"
else
  ALLOW="coding,shell"; [ $T = S1 ] && ALLOW="read_file,list_files,search,write_file,apply_patch,run_tests"
  echo "harness ask --model openai --allow $ALLOW --max-steps $CAP; user=$AU env -i" > "$D/agent_cmd.txt"
  sudo -n -u "$AU" env -i "${AENV[@]}" setsid --wait timeout -k 10 480 /usr/bin/time -v -o "$P/time.txt" "$HB" ask --model openai --allow "$ALLOW" --workspace "$W" --max-steps "$CAP" --record "$P/rec" --prompt "$PROMPT" </dev/null >"$D/stdout.txt" 2>"$D/stderr.txt"; echo $? > "$D/agent_exit"
fi
E=$(date +%s%3N); echo $((E-S)) > "$D/wall_ms"
# Cleanup: kill ALL agent-user processes (setsid'd children, SIGTERM-ignorers), then verify none remain.
"$HERE/agent_cleanup.sh" "$AU" > "$D/stray_procs_after_cleanup"; STRAY=$(cat "$D/stray_procs_after_cleanup")
python3 - "$D/run_env.json" "$AU" "$(id -u "$AU")" "$PROXY_SHA" "$PROXY_BLOB" "$PROXY_UID" "${AENV[@]}" <<'P'
import json,sys
out,user,uid,psha,pblob,puid=sys.argv[1:7]; env=sys.argv[7:]
json.dump({"run_user":user,"run_uid":int(uid),"proxy_sha":psha,"proxy_py_sha256":pblob,"proxy_user":"benchproxy","proxy_uid":int(puid) if puid.isdigit() else None,
  "proxy_port":int(open(out.rsplit("/",1)[0]+"/proxy_port").read()),"agent_env_vars":sorted(e.split("=",1)[0] for e in env),"env_mode":"env -i",
  "stray_procs_after_cleanup":int(open(out.rsplit("/",1)[0]+"/stray_procs_after_cleanup").read().strip() or -1)},open(out,"w"),indent=1)
P
if [ "$STRAY" != 0 ]; then echo "cleanup: $STRAY $AU process(es) still alive after kill; stopping before any next proxy" > "$D/ABORTED.txt"; sudo -n pkill -KILL -u benchproxy -f "proxy.py $PORT"; echo "$D"; exit 7; fi
sudo -n pkill -TERM -u benchproxy -f "proxy.py $PORT"; sleep 1; sudo -n pkill -KILL -u benchproxy -f "proxy.py $PORT" 2>/dev/null
cp "$HD/proxy_hold.json" "$D/" 2>/dev/null; cp "$HD/proxy_usage.jsonl" "$D/" 2>/dev/null || : > "$D/proxy_usage.jsonl"; sudo -n cp -r "$P/rec" "$D/rec" 2>/dev/null; sudo -n cp "$P/time.txt" "$D/" 2>/dev/null; sudo -n chown -R "$(id -u)" "$D" "$P"
(cd "$W" && cargo test > "$D/cargo_test.txt" 2>&1; echo $? > "$D/cargo_exit"; git add -A; git diff --cached HEAD > "$D/final.diff"; git diff --cached --name-only HEAD > "$D/files_modified.txt")
python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['harness_sha'])" "$D/rec/metrics.json" > "$D/harness_sha.txt" 2>/dev/null \
  || { "$HB" replay "$(ls "$HERE"/runs/*/rec/run.jsonl 2>/dev/null | head -1)" --metrics-out "$D/.sha.json" >/dev/null 2>&1 && python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['harness_sha'])" "$D/.sha.json" > "$D/harness_sha.txt"; rm -f "$D/.sha.json"; }
python3 - "$D" <<'P'
import json,sys,os
D=sys.argv[1]; p=os.path.join(D,"run_env.json"); e=json.load(open(p))
rows=[json.loads(l) for l in open(os.path.join(D,"proxy_usage.jsonl")) if l.strip()]
e["run_id"]=os.path.basename(D.rstrip("/")); e["proxy_run_ids"]=sorted(set(str(r.get("run_id")) for r in rows)); e["proxy_run_id_match"]=all(r.get("run_id")==e["run_id"] for r in rows)
e["proxy_requests"]=len(rows); e["proxy_status_counts"]={}
for r in rows: e["proxy_status_counts"][str(r["status"])]=e["proxy_status_counts"].get(str(r["status"]),0)+1
e["proxy_input_tokens"]=sum(r.get("input_tokens") or 0 for r in rows); e["proxy_output_tokens"]=sum(r.get("output_tokens") or 0 for r in rows)
e["proxy_proc_check"]=json.load(open(os.path.join(D,"proxy_proc_check.json")))
json.dump(e,open(p,"w"),indent=1)
P
ls -A "$P" | grep -vE '^(time.txt|rec)$' > "$D/parent_ls.txt"; echo "$MSHA $(sha256sum "$P/MARKER.txt" | cut -c1-64)" > "$D/marker_sha.txt"
[ -n "${SUMMARIZER:-}" ] && python3 "$SUMMARIZER" "$D" "$T" "$A" >/dev/null
sudo -n truncate -s 0 /opt/benchproxy/run/current_run_id;  # empty = no active run (proxy answers 503 no_run)
 rm -rf "$P" "$HD" 2>/dev/null; sudo -n rm -rf "$P" "$HD" 2>/dev/null
echo "$D"
