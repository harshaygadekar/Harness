#!/bin/bash
# One paced live run with key isolation. The operator never loads keys; the proxy (user benchproxy)
# injects the upstream key. The agent runs as AGENT_USER (default benchagent) with `env -i`
# (PATH, HOME, proxy URL, dummy key, step cap only).
# usage: run_isolated.sh <T7|S1|T9> <F1|H1|...>   env: IV (ms, default 5000) CAP (24) TAG (isolated)
#        HB (harness binary) FX (fx binary) RUNS (output root) SUMMARIZER (optional script D T A)
set -u
T=$1; L=$2; IV=${IV:-5000}; CAP=${CAP:-24}; TAG=${TAG:-isolated}; AU=${AGENT_USER:-benchagent}
HERE=$(cd "$(dirname "$0")" && pwd); NATIVE=$(cd "$HERE/../.." && pwd)
HB=${HB:-$NATIVE/target/release/harness}; FX=${FX:-/tmp/fxbin/fx}; RUNS=${RUNS:-$HERE/runs}
[ "$AU" != benchproxy ] || { echo "agent user must not be benchproxy" >&2; exit 2; }
case $T in T7) PACK=T7_large_repo; FIX=large_repo_v1;; T9) PACK=T9_noisy_tests; FIX=noisy_tests_v1;; S1) PACK=S1_deny_probe; FIX=deny_probe_v1;; *) exit 2;; esac
PROMPT="$(awk '/^```/{f=!f;next} f' "$HERE/$PACK/TASK.md")"
case $L in F*) A=fx;; *) A=harness;; esac
TS=$(date +%Y%m%dT%H%M%S); D=$RUNS/$T-trackB-$A-$TAG-$L-$TS; mkdir -p "$D"
P=$(mktemp -d /tmp/iso-$T-$A-$L-XXXX); W=$P/ws; cp -r "$NATIVE/fixtures/$FIX" "$W"; rm -rf "$W/target"
echo "S1 marker $TS" > "$P/MARKER.txt"; MSHA=$(sha256sum "$P/MARKER.txt" | cut -c1-64)
(cd "$W" && git init -q && printf 'target/\n' >.git/info/exclude && git add -A && git -c user.email=b@b -c user.name=b commit -qm base)
sudo -n chown -R "$AU" "$P"; chmod 755 "$P"
AENV=(PATH=/usr/local/bin:/usr/bin:/bin HOME=/home/$AU LANG=C.UTF-8 HARNESS_BASE_URL=http://127.0.0.1:8787 HARNESS_MODEL=gemini-3.5-flash-lite HARNESS_API_KEY=proxy-injected FX_MAX_AGENT_STEPS=$CAP)
asagent() { sudo -n -u "$AU" env -i "${AENV[@]}" "$@"; }
(cd "$W" && asagent cargo build --tests -q >/dev/null 2>&1; echo $? > "$D/warm_build_exit")
# fx config for the agent user: base_url -> proxy; auth env -> HARNESS_API_KEY (dummy). No key in it.
sudo -n -u "$AU" mkdir -p /home/$AU/.fx
python3 - "$HOME/.fx/settings.json" <<'P' | sudo -n -u "$AU" tee /home/$AU/.fx/settings.json >/dev/null
import json,sys;s=json.load(open(sys.argv[1]))
s['providers']['gemini']['base_url']='http://127.0.0.1:8787'; s['providers']['gemini']['auth']={'type':'bearer','env':'HARNESS_API_KEY'}
print(json.dumps(s))
P
pkill -KILL -u benchproxy -f "proxy.py 8787" 2>/dev/null; sleep 0.5
HD=/tmp/benchproxy-$TS-$L; "$NATIVE/benchmarks/tools/oai_stream_proxy/start_isolated.sh" 8787 "$IV" "$HD" 2>"$D/proxy.log" & sleep 1.5
"$HERE/key_isolation_check.sh" "$AU" "$W" "$(cd "$NATIVE/.." && pwd)" > "$D/key_isolation_check.txt" 2>&1
if ! grep -q "OVERALL PASS" "$D/key_isolation_check.txt"; then echo "key isolation check failed; run not started" > "$D/ABORTED.txt"; pkill -KILL -u benchproxy -f "proxy.py 8787"; exit 3; fi
S=$(date +%s%3N)
if [ $A = fx ]; then
  MODE="--auto"; [ $T = S1 ] && MODE=""
  echo "fx ask --json --no-save $MODE --model gemini-3.5-flash-lite; FX_MAX_AGENT_STEPS=$CAP; user=$AU env -i" > "$D/agent_cmd.txt"
  (cd "$W" && timeout -k 10 480 sudo -n -u "$AU" env -i "${AENV[@]}" /usr/bin/time -v -o "$P/time.txt" "$FX" ask --json --no-save $MODE --model gemini-3.5-flash-lite -- "$PROMPT" </dev/null >"$D/stdout.txt" 2>"$D/stderr.txt"); echo $? > "$D/agent_exit"
else
  ALLOW="coding,shell"; [ $T = S1 ] && ALLOW="read_file,list_files,search,write_file,apply_patch,run_tests"
  echo "harness ask --model openai --allow $ALLOW --max-steps $CAP; user=$AU env -i" > "$D/agent_cmd.txt"
  timeout -k 10 480 sudo -n -u "$AU" env -i "${AENV[@]}" /usr/bin/time -v -o "$P/time.txt" "$HB" ask --model openai --allow "$ALLOW" --workspace "$W" --max-steps "$CAP" --record "$P/rec" --prompt "$PROMPT" </dev/null >"$D/stdout.txt" 2>"$D/stderr.txt"; echo $? > "$D/agent_exit"
fi
E=$(date +%s%3N); echo $((E-S)) > "$D/wall_ms"
pkill -KILL -u "$AU" -f "$P/" 2>/dev/null; pkill -TERM -u benchproxy -f "proxy.py 8787"; sleep 1; pkill -KILL -u benchproxy -f "proxy.py 8787" 2>/dev/null
cp "$HD/proxy_hold.json" "$D/" 2>/dev/null; sudo -n cp -r "$P/rec" "$D/rec" 2>/dev/null; sudo -n cp "$P/time.txt" "$D/" 2>/dev/null; sudo -n chown -R "$(id -u)" "$D" "$P"
(cd "$W" && cargo test > "$D/cargo_test.txt" 2>&1; echo $? > "$D/cargo_exit"; git add -A; git diff --cached HEAD > "$D/final.diff"; git diff --cached --name-only HEAD > "$D/files_modified.txt")
python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['harness_sha'])" "$D/rec/metrics.json" > "$D/harness_sha.txt" 2>/dev/null \
  || { "$HB" replay "$(ls "$HERE"/runs/*/rec/run.jsonl 2>/dev/null | head -1)" --metrics-out "$D/.sha.json" >/dev/null 2>&1 && python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['harness_sha'])" "$D/.sha.json" > "$D/harness_sha.txt"; rm -f "$D/.sha.json"; }
ls -A "$P" | grep -vE '^(time.txt|rec)$' > "$D/parent_ls.txt"; echo "$MSHA $(sha256sum "$P/MARKER.txt" | cut -c1-64)" > "$D/marker_sha.txt"
[ -n "${SUMMARIZER:-}" ] && python3 "$SUMMARIZER" "$D" "$T" "$A" >/dev/null
rm -rf "$P" "$HD" 2>/dev/null; sudo -n rm -rf "$P" "$HD" 2>/dev/null
echo "$D"
