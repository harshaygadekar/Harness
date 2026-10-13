#!/usr/bin/env python3
"""Test-only (no Gemini, no key): run_isolated.sh in BENCH_TEST_FAKE_AGENT mode against a local fake upstream.
(a) SIGINT / SIGTERM to the runner mid-run -> no benchagent procs, current_run_id empty, no proxy on the port.
(b) SIGKILL to the runner mid-run -> state left behind; the next runner cleans all 3 before its gate."""
import hashlib, json, os, re, signal, socket, subprocess, sys, tempfile, threading, time
from http.server import ThreadingHTTPServer, BaseHTTPRequestHandler
HERE = os.path.dirname(os.path.abspath(__file__)); RUNNER = os.path.join(HERE, "run_isolated.sh")
RIDF = "/opt/benchproxy/run/current_run_id"; AU = "benchagent"
PROXY_COMMIT = os.environ.get("PROXY_COMMIT", "eff7d948ed92fdb0dd64a5b7819ff93193a258e7")

class Up(BaseHTTPRequestHandler):
    def log_message(self, *a): pass
    def do_POST(self):
        self.rfile.read(int(self.headers.get("Content-Length") or 0))
        b = b'{"choices":[{"message":{"role":"assistant","content":"ok"},"finish_reason":"stop","index":0}],"usage":{"prompt_tokens":1,"completion_tokens":1}}'
        self.send_response(200); self.send_header("Content-Type", "application/json"); self.send_header("Content-Length", str(len(b))); self.end_headers(); self.wfile.write(b)
up = ThreadingHTTPServer(("127.0.0.1", 0), Up); threading.Thread(target=up.serve_forever, daemon=True).start()
UPSTREAM = "http://127.0.0.1:%d" % up.server_address[1]

fake = tempfile.NamedTemporaryFile("w", suffix=".sh", delete=False, dir="/tmp")
fake.write("""#!/bin/bash
trap '' TERM INT
curl -s -m 5 -X POST -H 'Content-Type: application/json' -d '{"model":"m","messages":[]}' "$HARNESS_BASE_URL/chat/completions" >/dev/null
setsid bash -c "trap '' TERM HUP INT; exec -a fake-stray-grandchild sleep 600" </dev/null >/dev/null 2>&1 &
exec -a fake-agent sleep 600
"""); fake.close(); os.chmod(fake.name, 0o755)
short = tempfile.NamedTemporaryFile("w", suffix=".sh", delete=False, dir="/tmp")
short.write("""#!/bin/bash
curl -s -m 5 -X POST -H 'Content-Type: application/json' -d '{"model":"m","messages":[]}' "$HARNESS_BASE_URL/chat/completions" >/dev/null
exit 0
"""); short.close(); os.chmod(short.name, 0o755)
import glob
ACT = "/opt/benchproxy/run/active"
def hd_of(run_dir):
    rec = os.path.join(ACT, os.path.basename(run_dir) + ".json")
    if not os.path.exists(rec): return None, None
    r = json.load(open(rec)); return r["hd"], r["ws"]
runs = tempfile.mkdtemp(prefix="sigtest-runs-"); os.chmod(runs, 0o777)

def sh(c): return subprocess.run(c, shell=True, capture_output=True, text=True).stdout.strip()
def n_agent(): return int(sh(f"pgrep -u {AU} | wc -l"))
def rid(): return sh(f"cat {RIDF}")
def proxies(): return sh("pgrep -u benchproxy -af proxy.py")
def listening(port):
    s = socket.socket(); s.settimeout(0.5)
    try: return s.connect_ex(("127.0.0.1", port)) == 0
    finally: s.close()
def start(label, agent=None):
    env = dict(os.environ, PROXY_COMMIT=PROXY_COMMIT, BENCH_TEST_FAKE_AGENT=agent or fake.name, BENCH_TEST_UPSTREAM=UPSTREAM, RUNS=runs, IV="0")
    return subprocess.Popen([RUNNER, "T1", label], env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
                            restore_signals=True, start_new_session=True)
def wait_mid_run(timeout=120):
    t = time.time()
    while time.time() - t < timeout:
        for d in os.listdir(runs):
            p = os.path.join(runs, d)
            if os.path.exists(os.path.join(p, "agent_cmd.txt")) and n_agent() >= 2 and os.path.exists(os.path.join(p, "proxy_port")):
                return p, int(open(os.path.join(p, "proxy_port")).read())
        time.sleep(0.5)
    raise SystemExit("FAIL: run never reached the agent")
def clear_runs():
    for d in os.listdir(runs): subprocess.run(["rm", "-rf", os.path.join(runs, d)])

ok = True
def check(name, cond, detail=""):
    global ok; ok &= bool(cond); print(("PASS " if cond else "FAIL ") + name + (("  [" + detail + "]") if detail else ""))

for sig in (signal.SIGINT, signal.SIGTERM):
    clear_runs(); p = start("H1"); d, port = wait_mid_run()
    check(f"{sig.name}: mid-run state present", n_agent() >= 2 and rid() != "" and listening(port), f"agents={n_agent()} rid={rid()!r} port={port}")
    p.send_signal(sig); rc = p.wait(60)
    check(f"{sig.name}: runner exit code", rc == (130 if sig == signal.SIGINT else 143), f"rc={rc}")
    check(f"{sig.name}: no benchagent process left", n_agent() == 0, f"{n_agent()}")
    check(f"{sig.name}: current_run_id empty", rid() == "", repr(rid()))
    check(f"{sig.name}: no proxy on port {port}", not listening(port) and proxies() == "", proxies())

clear_runs(); p = start("H1"); d, port = wait_mid_run()
hd_k, ws_k = hd_of(d)
check("SIGKILL: root-owned run record exists", hd_k is not None and (ws_k or "").startswith("/tmp/iso-"), f"{hd_k} {ws_k}")
time.sleep(1.5)  # let the fake agent's request reach the proxy log
p.kill(); p.wait(30); time.sleep(1)
# Plant a symlink at the copy destination inside the killed run's folder; it points at a sentinel file.
SENT_F = tempfile.NamedTemporaryFile("w", prefix="sentinel-file-", dir="/tmp", delete=False); SENT_F.write("sentinel\n"); SENT_F.close()
sha_f = lambda p_: hashlib.sha256(open(p_, "rb").read()).hexdigest()
sent_sha = sha_f(SENT_F.name)
dst = os.path.join(d, "proxy_usage.jsonl")
if os.path.lexists(dst): os.unlink(dst)
os.symlink(SENT_F.name, dst)
SENT_K = {}
for nm_ in ("killed.json", "run_env.json"):
    sf = tempfile.NamedTemporaryFile("w", prefix="sentinel-file-", dir="/tmp", delete=False); sf.write(nm_ + " sentinel\n"); sf.close()
    dp = os.path.join(d, nm_)
    if os.path.lexists(dp): os.unlink(dp)
    os.symlink(sf.name, dp); SENT_K[nm_] = (sf.name, sha_f(sf.name), dp)
left = (n_agent(), rid(), listening(port))
check("SIGKILL: state left behind (agents, run id, proxy)", left[0] >= 1 and left[1] != "" and left[2], f"agents={left[0]} rid={left[1]!r} listening={left[2]}")
p2 = start("H2")
d2 = None; t = time.time()
while time.time() - t < 120 and d2 is None:
    for x in os.listdir(runs):
        if x != os.path.basename(d) and os.path.exists(os.path.join(runs, x, "proxy_port")): d2 = os.path.join(runs, x)
    time.sleep(0.3)
check("next runner: gate passed (no ABORTED, reached proxy start)", d2 is not None and not os.path.exists(os.path.join(d2, "ABORTED.txt")))
check("next runner: old proxy port gone", not listening(port))
km = json.load(open(os.path.join(d, "killed.json"))) if os.path.exists(os.path.join(d, "killed.json")) else {}
check("next runner: killed run marked status killed", km.get("status") == "killed", str(km.get("status")))
ul = open(os.path.join(d, "proxy_usage.jsonl")).read().splitlines() if os.path.exists(os.path.join(d, "proxy_usage.jsonl")) else []
check("next runner: killed run's proxy usage log copied into its folder", len(ul) >= 1, f"{len(ul)} row(s)")
check("rescue: planted symlink at destination did not change the sentinel (sha256)", sha_f(SENT_F.name) == sent_sha)
check("rescue: destination is now a regular file (not a symlink) with the copied log", os.path.isfile(dst) and not os.path.islink(dst) and len(open(dst).read().splitlines()) >= 1)
os.unlink(SENT_F.name)
for nm_, (sf, sh_, dp) in SENT_K.items():
    check(f"rescue: planted symlink at {nm_} did not change its sentinel (sha256); dst is a regular file",
          sha_f(sf) == sh_ and os.path.isfile(dp) and not os.path.islink(dp) and json.load(open(dp)).get("status") == "killed")
    os.unlink(sf)
check("next runner: killed run's /tmp/benchproxy-* and /tmp/iso-* deleted", hd_k and not os.path.exists(hd_k) and not os.path.exists(ws_k))
check("next runner: own run id set (old one replaced)", rid() == os.path.basename(d2 or ""), repr(rid()))
p2.send_signal(signal.SIGTERM); rc2 = p2.wait(60)
check("next runner: TERM cleanup after", n_agent() == 0 and rid() == "" and proxies() == "", f"rc={rc2}")
# A full fake run (agent exits at once): run_env.json must carry fake:true.
clear_runs(); p3 = start("H3", short.name); out3, _ = p3.communicate(timeout=300)
d3 = [os.path.join(runs, x) for x in os.listdir(runs)][0]
e3 = json.load(open(os.path.join(d3, "run_env.json")))
check("fake run: run_env.json has fake:true", e3.get("fake") is True, f"rc={p3.returncode}")
open(os.path.join(d3, "summary.json"), "w").write('{"oracle_success": true}')
kd = os.path.join(runs, "killed-copy"); os.makedirs(kd); open(os.path.join(kd, "killed.json"), "w").write('{"status":"killed"}')
open(os.path.join(kd, "summary.json"), "w").write('{"oracle_success": true}')
for x in (d3, kd):
    r = subprocess.run([sys.executable, os.path.join(HERE, "summ.py"), x, "T1", "harness"], capture_output=True, text=True)
    check(f"summ.py refuses {os.path.basename(x)}", r.returncode == 3 and "REFUSED" in r.stderr, r.stderr.strip())
r = subprocess.run([sys.executable, os.path.join(HERE, "results_table.py"), d3, kd], capture_output=True, text=True)
check("results_table.py refuses fake and killed runs (0 counted)", r.stderr.count("REFUSED") == 2 and len(r.stdout.strip().splitlines()) == 1, r.stderr.strip().splitlines()[-1])
# ---- Planted folders and forged pointers (Jessie's hole). Nothing outside legit run folders may change.
import hashlib
def snapshot(paths):
    out = {}
    for root in paths:
        out[root] = sh(f"sudo -n find {root} -xdev \\( -type f -o -type l -o -type d \\) -printf '%p %y %u %m %l\\n' 2>/dev/null | sort") + "|" + \
                    sh(f"sudo -n find {root} -xdev -type f -exec sha256sum {{}} + 2>/dev/null | sort")
    return out
clear_runs()
SENT = tempfile.mkdtemp(prefix="sentinel-", dir="/tmp"); open(os.path.join(SENT, "canary.txt"), "w").write("canary\n"); os.chmod(SENT, 0o755)
DECOY = os.path.join(runs, "decoy-run"); os.makedirs(DECOY); open(os.path.join(DECOY, "canary.txt"), "w").write("decoy\n")
bp_home = "/home/benchproxy"
A = "sudo -n -u benchagent bash -c"
planted = ["/tmp/benchproxy-20991231T000001-X1", "/tmp/benchproxy-20991231T000002-X2", "/tmp/iso-T1-harness-X3-abcd", "/tmp/iso-T1-harness-X4-efgh"]
subprocess.run(A.split() + [f"""
mkdir -p {planted[0]} && printf '%s\\n' '{DECOY}' > {planted[0]}/run_dir && printf '%s\\n' '/tmp/iso-x/../../home/benchproxy' > {planted[0]}/ws_dir && echo '{{}}' > {planted[0]}/proxy_usage.jsonl
ln -s {SENT} {planted[1]}
mkdir -p {planted[2]} && echo planted > {planted[2]}/f
ln -s {SENT} {planted[3]}
"""], check=False)
# Forged records (worst case: even a root-owned record with hostile paths must be refused).
DEC = {}
for k in ("dotdot", "symlinkws", "wrongownerhd", "symlinkhd"):
    DEC[k] = os.path.join(runs, "decoy-" + k); os.makedirs(DEC[k]); open(os.path.join(DEC[k], "canary.txt"), "w").write(k + "\n")
forged = {
  "dotdot": {"hd": planted[0], "ws": "/tmp/iso-x/../../home/benchproxy", "run_dir": DEC["dotdot"]},
  "symlinkws": {"hd": "/tmp/benchproxy-20991231T000009-Z9", "ws": planted[3], "run_dir": DEC["symlinkws"]},
  "wrongownerhd": {"hd": planted[0], "ws": planted[2], "run_dir": DEC["wrongownerhd"]},
  "symlinkhd": {"hd": planted[1], "ws": planted[2], "run_dir": DEC["symlinkhd"]},
  "outside-runs": {"hd": planted[0], "ws": planted[2], "run_dir": SENT},
}
for k, v in forged.items():
    nm = os.path.basename(v["run_dir"]) + ".json" if k != "outside-runs" else os.path.basename(SENT) + ".json"
    subprocess.run(f"echo '{json.dumps(v)}' | sudo -n tee {ACT}/{nm} >/dev/null && sudo -n chmod 644 {ACT}/{nm}", shell=True)
    snap_rec = nm
watch = [SENT, DECOY, bp_home] + list(DEC.values()) + [p for p in planted if os.path.lexists(p)]
before = snapshot(watch); recs_before = sorted(os.listdir(ACT))
# (i) Rescue never runs while a benchagent process is alive.
alive = subprocess.Popen(["sudo", "-n", "-u", "benchagent", "sleep", "60"]); time.sleep(0.5)
r = subprocess.run([sys.executable, os.path.join(HERE, "rescue_killed.py"), runs, AU], capture_output=True, text=True)
check("rescue refuses while a benchagent process is alive", r.returncode == 4 and "process alive" in r.stderr and snapshot(watch) == before and sorted(os.listdir(ACT)) == recs_before, r.stderr.strip())
subprocess.run(["sudo", "-n", "pkill", "-KILL", "-u", "benchagent"]); alive.wait(10); time.sleep(0.5)
# (ii) The next runner (full fake run) must skip every planted or forged item.
p4 = start("H4", short.name); out4, _ = p4.communicate(timeout=300)
after = snapshot(watch)
check("planted: next runner finished", p4.returncode == 0, f"rc={p4.returncode}")
check("planted: nothing outside legit run folders changed (listing + sha256)", after == before, ",".join(k for k in watch if before.get(k) != after.get(k)))
check("planted: no killed.json written anywhere (decoy, sentinel, real runs)", sh(f"find {runs} {SENT} -name killed.json") == "")
for k in forged:
    nm = os.path.basename(forged[k]["run_dir"])
    line = [l for l in out4.splitlines() if l.startswith(f"rescue_killed: skip record {nm}.json")]
    check(f"planted: forged record '{k}' skipped and logged", len(line) == 1, line[0][len("rescue_killed: "):] if line else "no line")
real = [os.path.join(runs, x) for x in os.listdir(runs) if not x.startswith("decoy-")]
check("planted: real run not marked killed", all("status" not in json.load(open(os.path.join(x, "run_env.json"))) for x in real if os.path.exists(os.path.join(x, "run_env.json"))))
# Clean up planted items, forged records and this test's own folders.
for k, v in forged.items():
    nm = os.path.basename(v["run_dir"]) + ".json" if k != "outside-runs" else os.path.basename(SENT) + ".json"
    subprocess.run(["sudo", "-n", "rm", "-f", "--", os.path.join(ACT, nm)])
for pth in planted: subprocess.run(["sudo", "-n", "rm", "-rf", "--", pth])
subprocess.run(["rm", "-rf", "--", SENT])
for rec in glob.glob(ACT + "/*.json"):
    try: r_ = json.load(open(rec))
    except ValueError: continue
    if str(r_.get("run_dir", "")).startswith(runs):
        for pth in (r_.get("ws"), r_.get("hd")):
            if pth and re.match(r"^/tmp/(iso|benchproxy)-", pth): subprocess.run(["sudo", "-n", "rm", "-rf", "--", pth])
        subprocess.run(["sudo", "-n", "rm", "-f", "--", rec])
up.shutdown(); os.unlink(short.name); os.unlink(fake.name); subprocess.run(["sudo", "-n", "rm", "-rf", runs])
print("OVERALL", "PASS" if ok else "FAIL"); sys.exit(0 if ok else 1)
