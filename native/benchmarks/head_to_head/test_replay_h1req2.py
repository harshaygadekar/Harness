#!/usr/bin/env python3
"""Offline tests for replay_h1req2.py. No network, no key: a local fake upstream only.
Parts:
  1. build: request 2 shape from the real Harness e0fc51e binary (SKIP if HB is missing or has another sha256)
  2. send: 669 gate (stop on mismatch, nothing counts)
  3. send + summarize: 429 tries do not count
  4. no message text in any record; reply model id logged; odd model value not logged as text
  5. real proxy.py (this repo) between sender and fake upstream: finish_reason_raw and model joined per try
  6. run_isolated.sh BENCH_REPLAY=h1req2 end to end with the pinned proxy and no key (SKIP without sudo/users/pin/HB)
usage: python3 test_replay_h1req2.py   env: HB (harness e0fc51e binary), PROXY_COMMIT (pin for part 6)"""
import json, os, shutil, stat, subprocess, sys, tempfile, threading, time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import replay_h1req2 as R

HB = os.environ.get("HB", "/workspace/wt-t1pair-h/native/target/release/harness")
NATIVE = os.path.abspath(os.path.join(HERE, "..", ".."))
H1_RESULT = ".git/\nCargo.lock\nCargo.toml\ndata/\ndocs/\nnotes/\nsrc/\ntarget/"
H1_EVENTS = [
    {"type": "state_change", "ts_ms": 1, "status": "running"},
    {"type": "model_request", "ts_ms": 1, "message_count": 1, "tool_count": 9},
    {"type": "model_response", "ts_ms": 2, "content": None, "tool_calls": [{"id": "call_1430681", "name": "list_files", "arguments": {}}]},
    {"type": "tool_call", "ts_ms": 2, "id": "call_1430681", "name": "list_files", "arguments": {}},
    {"type": "tool_result", "ts_ms": 2, "id": "call_1430681", "name": "list_files", "content": H1_RESULT, "ok": True},
    {"type": "model_request", "ts_ms": 2, "message_count": 3, "tool_count": 9},
    {"type": "model_response", "ts_ms": 3, "content": None, "tool_calls": []},
    {"type": "done", "ts_ms": 3, "status": "Succeeded", "steps": 2},
]
SECRETS = ("SECRET_REPLY_TEXT_77", "BODY_MARKER_5a1", "SECRET_IN_MODEL_FIELD")
fails = skips = 0


def check(name, cond, detail=""):
    global fails
    print(("PASS  " if cond else "FAIL  ") + name + ("" if cond or not detail else "  (%s)" % detail))
    fails += 0 if cond else 1


def skip(name, why):
    global skips
    skips += 1
    print("SKIP  %s  (%s)" % (name, why))


class Upstream:
    """Fake upstream. plan: list of (status, prompt_tokens, finish_reason, model); the last entry repeats."""

    def __init__(self, plan):
        self.plan, self.n, self.seen = plan, 0, []
        up = self

        class H(BaseHTTPRequestHandler):
            def log_message(self, *a):
                pass

            def do_POST(self):
                body = self.rfile.read(int(self.headers.get("Content-Length") or 0))
                up.seen.append(body)
                st, pt, fr, mdl = up.plan[min(up.n, len(up.plan) - 1)]
                up.n += 1
                if st == 200:
                    out = json.dumps({"id": "x", "object": "chat.completion", "model": mdl,
                                      "choices": [{"index": 0, "finish_reason": fr,
                                                   "message": {"role": "assistant", "content": "SECRET_REPLY_TEXT_77"}}],
                                      "usage": {"prompt_tokens": pt, "completion_tokens": 3, "total_tokens": pt + 3}}).encode()
                else:  # error body; fr is used as error.status (None = no status field)
                    err = {"code": st, "message": "SECRET_REPLY_TEXT_77 error text"}
                    if fr is not None:
                        err["status"] = fr
                    out = json.dumps([{"error": err}] if mdl == "list" else {"error": err}).encode()
                self.send_response(st)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(out)))
                self.end_headers()
                self.wfile.write(out)

        self.srv = ThreadingHTTPServer(("127.0.0.1", 0), H)
        self.port = self.srv.server_address[1]
        threading.Thread(target=self.srv.serve_forever, daemon=True).start()

    def close(self):
        self.srv.shutdown()


def body_file(d):
    p = os.path.join(d, "body.json")
    json.dump({"model": "gemini-3.5-flash-lite", "stream": False,
               "messages": [{"role": "user", "content": "BODY_MARKER_5a1"}]}, open(p, "w"))
    return p


def no_secrets(paths):
    raw = "".join(open(p).read() for p in paths if os.path.exists(p))
    return [s for s in SECRETS if s in raw]


def make_ws(base):
    ws = os.path.join(base, "ws")
    shutil.copytree(os.path.join(NATIVE, "fixtures", "broken_calc_v2"), ws)
    shutil.rmtree(os.path.join(ws, "target"), ignore_errors=True)
    g = lambda *a: subprocess.run(["git", "-C", ws] + list(a), check=True, capture_output=True)
    g("init", "-q"); open(os.path.join(ws, ".git", "info", "exclude"), "w").write("target/\n")
    g("add", "-A"); g("-c", "user.email=b@b", "-c", "user.name=b", "commit", "-qm", "base")
    os.mkdir(os.path.join(ws, "target"))
    return ws


def part_build(base):
    if not os.path.exists(HB) or R.sha256_file(HB) != R.EXPECTED_HARNESS_SHA256:
        return skip("build: request 2 shape", "HB missing or not the e0fc51e build")
    ws = make_ws(base)
    rec = os.path.join(base, "run.jsonl"); open(rec, "w").write("".join(json.dumps(e) + "\n" for e in H1_EVENTS))
    pf = os.path.join(base, "prompt.txt"); open(pf, "w").write("Fix the calculator. BODY_MARKER_5a1")
    out, fo = os.path.join(base, "b", "body.json"), os.path.join(base, "b", "fields.json"); os.mkdir(os.path.join(base, "b"))
    rc = R.main(["build", "--harness", HB, "--ws", ws, "--h1-rec", rec, "--prompt-file", pf, "--out", out, "--fields-out", fo])
    check("build: exit 0", rc == 0)
    check("build: body mode 600", stat.S_IMODE(os.stat(out).st_mode) == 0o600)
    b = json.load(open(out)); f = json.load(open(fo))
    check("build: top-level keys are messages, model, stream, tools", sorted(b) == ["messages", "model", "stream", "tools"], sorted(b))
    check("build: model gemini-3.5-flash-lite, stream false", b["model"] == "gemini-3.5-flash-lite" and b["stream"] is False)
    check("build: roles user, assistant, tool", [m["role"] for m in b["messages"]] == ["user", "assistant", "tool"])
    check("build: tool result word for word", b["messages"][2]["content"] == H1_RESULT)
    check("build: tool call id from log", b["messages"][1]["tool_calls"][0]["id"] == "call_1430681")
    check("build: 9 tools", len(b["tools"]) == 9 and f["tools"]["count"] == 9)
    check("build: fields.json lists temperature/tool_choice/max_tokens as absent",
          all(f["fields"][k] == "absent" for k in ("temperature", "tool_choice", "max_tokens", "max_completion_tokens")))
    check("build: no message text in fields.json", not no_secrets([fo]) and H1_RESULT not in open(fo).read())
    tcs = [t for m in f["messages"] for t in m.get("tool_calls", [])]
    check("build: tool call arguments kept as sha256 and length only",
          tcs and all("arguments" not in t and len(t["arguments_sha256"]) == 64 and t["arguments_chars"] == 2 for t in tcs), tcs)
    # mismatch: the log says another tool result -> build stops, no body
    bad = [dict(e, content="other.txt") if e["type"] == "tool_result" else e for e in H1_EVENTS]
    rec2 = os.path.join(base, "run2.jsonl"); open(rec2, "w").write("".join(json.dumps(e) + "\n" for e in bad))
    out2 = os.path.join(base, "b", "body2.json")
    try:
        R.main(["build", "--harness", HB, "--ws", ws, "--h1-rec", rec2, "--prompt-file", pf, "--out", out2, "--fields-out", fo + "2"])
        ok = False
    except SystemExit as e:
        ok = "word for word" in str(e)
    check("build: stops when the tool result differs from the log; no body", ok and not os.path.exists(out2))


def send(base, name, plan, tries):
    up = Upstream(plan)
    out = os.path.join(base, name + ".jsonl")
    try:
        rc = R.main(["send", "--port", str(up.port), "--body", body_file(base), "--tries", str(tries), "--out", out])
    finally:
        up.close()
    return rc, out, [json.loads(l) for l in open(out)], up


def rows_for(recs, fr="STOP"):
    return [{"status": r["status"], "input_tokens": r["input_tokens"], "output_tokens": r["output_tokens"],
             "finish_reason_raw": fr if r["status"] == 200 else None} for r in recs]


def summarize(base, name, tries_path, rows):
    pu = os.path.join(base, name + "_pu.jsonl"); open(pu, "w").write("".join(json.dumps(r) + "\n" for r in rows))
    out = os.path.join(base, name + "_rec.json")
    rc = R.main(["summarize", "--tries", tries_path, "--proxy-usage", pu, "--pin", "/opt/benchproxy/PIN", "--out", out, "--planned", "3"])
    return rc, out, json.load(open(out))


def part_gate(base):
    rc, out, recs, up = send(base, "gate", [(200, 668, "stop", "gemini-3.5-flash-lite")], 3)
    check("gate: 668 != 669 -> send exit 5", rc == 5)
    check("gate: only the calibration request was sent", up.n == 1 and len(recs) == 1)
    check("gate: calibration gate_pass false, counted false", recs[0]["gate_pass"] is False and recs[0]["counted"] is False)
    rc2, _, rec = summarize(base, "gate", out, rows_for(recs))
    check("gate: summary gate_pass false, 0 counted", rec["gate_pass"] is False and rec["tries_counted"] == 0 and rc2 == 6)
    rc, out, recs, up = send(base, "gate429", [(429, 0, None, None)], 3)
    check("gate: calibration 429 -> stop, nothing counted", rc == 5 and up.n == 1 and recs[0]["gate_pass"] is False)
    rc, out, recs, up = send(base, "gateok", [(200, 669, "stop", "gemini-3.5-flash-lite")], 2)
    check("gate: 669 -> calibration + 2 tries; calibration not counted",
          rc == 0 and up.n == 3 and recs[0]["counted"] is False and [r["counted"] for r in recs[1:]] == [True, True])


def part_429(base):
    plan = [(200, 669, "stop", "gemini-3.5-flash-lite"), (200, 669, "stop", "gemini-3.5-flash-lite"),
            (429, 0, None, None), (200, 669, "stop", "gemini-3.5-flash-lite")]
    rc, out, recs, up = send(base, "r429", plan, 3)
    check("429: send exit 0, 4 requests", rc == 0 and up.n == 4)
    check("429: try 2 marked upstream_affected and not counted",
          recs[2]["status"] == 429 and recs[2]["counted"] is False and recs[2].get("upstream_affected") == "429")
    rc2, _, rec = summarize(base, "r429", out, rows_for(recs))
    check("429: summary counts 2 of 3, tries_429 = 1", rec["tries_counted"] == 2 and rec["tries_429"] == 1 and rc2 == 0, rec)
    # proxy row says 429 while the client saw 200 -> not counted, problem logged
    rows = rows_for(recs); rows[1]["status"] = 429
    _, _, rec = summarize(base, "r429b", out, rows)
    check("429: a proxy 429 row makes that try not count", rec["tries"][1]["counted"] is False and rec["problems"])


def part_text_and_model(base):
    plan = [(200, 669, "stop", "gemini-3.5-flash-lite"), (200, 669, "stop", "SECRET_IN_MODEL_FIELD please read this text")]
    rc, out, recs, up = send(base, "txt", plan, 1)
    check("model: reply model id logged", recs[0]["model"] == "gemini-3.5-flash-lite")
    check("model: odd model value logged as unlisted_format", recs[1]["model"] == "unlisted_format")
    _, recp, rec = summarize(base, "txt", out, rows_for(recs))
    check("model: model kept next to finish_reason_raw in the record",
          rec["tries"][0]["model"] == "gemini-3.5-flash-lite" and rec["tries"][0]["finish_reason_raw"] == "STOP")
    left = no_secrets([out, recp])
    check("no message text, reply text or body text in records", not left, left)


def part_error_status(base):
    rc, out, recs, up = send(base, "e400", [(200, 669, "stop", "gemini-3.5-flash-lite"), (400, 0, "invalid_argument", None),
                                            (400, 0, "SOMETHING_NEW", None), (400, 0, None, None), (503, 0, "Unavailable", "list")], 4)
    check("error: 400 INVALID_ARGUMENT (any case) logged as INVALID_ARGUMENT", recs[1]["error_status"] == "INVALID_ARGUMENT", recs[1])
    check("error: unknown error.status logged as unlisted", recs[2]["error_status"] == "unlisted")
    check("error: missing error.status logged as null", recs[3]["error_status"] is None)
    check("error: list-shaped error body read (UNAVAILABLE)", recs[4]["error_status"] == "UNAVAILABLE")
    check("error: 200 tries have error_status null", recs[0]["error_status"] is None)
    check("error: non-200 tries not counted", [r["counted"] for r in recs[1:]] == [False] * 4)
    _, recp, rec = summarize(base, "e400", out, rows_for(recs))
    check("error: error_status kept in the record", rec["tries"][1]["error_status"] == "INVALID_ARGUMENT")
    left = no_secrets([out, recp])
    check("error: no error message text in records", not left, left)
    rc, out, recs, up = send(base, "cal400", [(400, 0, "INVALID_ARGUMENT", None)], 3)
    _, _, rec = summarize(base, "cal400", out, rows_for(recs))
    check("error: calibration 400 stops the gate, 0 counted, code logged",
          rc == 5 and up.n == 1 and recs[0]["gate_pass"] is False and recs[0]["error_status"] == "INVALID_ARGUMENT"
          and rec["tries_counted"] == 0 and rec["gate_pass"] is False)


def part_rows_mismatch(base):
    plan = [(200, 669, "stop", "gemini-3.5-flash-lite")]
    rc, out, recs, up = send(base, "mm", plan, 2)
    # Make try 1 a client-side error that never reached the proxy: no status, no proxy row.
    recs[1].update({"status": None, "error_type": "ConnectionResetError", "counted": False, "input_tokens": None})
    open(out, "w").write("".join(json.dumps(r) + "\n" for r in recs))
    rows = rows_for([recs[0], recs[2]])
    rc2, _, rec = summarize(base, "mm", out, rows)
    check("rows mismatch: rows_mismatch true", rec["rows_mismatch"] is True)
    check("rows mismatch: every try not counted", all(t["counted"] is False for t in rec["tries"]) and rec["tries_counted"] == 0)
    check("rows mismatch: summarize exit not 0", rc2 != 0)
    _, _, ok = summarize(base, "mm_ok", out.replace("mm", "mm"), rows_for(recs))
    check("rows equal: rows_mismatch false", ok["rows_mismatch"] is False)


def part_real_proxy(base):
    proxy = os.path.join(NATIVE, "benchmarks", "tools", "oai_stream_proxy", "proxy.py")
    plan = [(200, 669, "max_tokens", "gemini-3.5-flash-lite"), (200, 669, "Other", "gemini-3.5-flash-lite"),
            (200, 669, "SECRET_REPLY_TEXT_77", "gemini-3.5-flash-lite")]
    up = Upstream(plan)
    rid = os.path.join(base, "rid"); open(rid, "w").write("replay-test-run\n")
    pu = os.path.join(base, "proxy_usage.jsonl")
    import socket
    s = socket.socket(); s.bind(("127.0.0.1", 0)); port = s.getsockname()[1]; s.close()
    env = {"PATH": "/usr/bin:/bin", "PROXY_UPSTREAM": "http://127.0.0.1:%d" % up.port, "PROXY_RUN_ID_FILE": rid}
    p = subprocess.Popen([sys.executable, proxy, str(port), "--usage-file", pu], env=env,
                         stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    try:
        for _ in range(50):
            try:
                socket.create_connection(("127.0.0.1", port), timeout=0.2).close(); break
            except OSError:
                time.sleep(0.1)
        out = os.path.join(base, "rp.jsonl")
        rc = R.main(["send", "--port", str(port), "--body", body_file(base), "--tries", "2", "--out", out])
    finally:
        p.terminate(); p.wait(); up.close()
    recp = os.path.join(base, "rp_rec.json")
    R.main(["summarize", "--tries", out, "--proxy-usage", pu, "--pin", "/opt/benchproxy/PIN", "--out", recp, "--planned", "2"])
    rec = json.load(open(recp))
    check("real proxy: send exit 0", rc == 0)
    check("real proxy: finish_reason_raw from proxy rows = MAX_TOKENS, OTHER, unlisted",
          [t["finish_reason_raw"] for t in rec["tries"]] == ["MAX_TOKENS", "OTHER", "unlisted"], [t["finish_reason_raw"] for t in rec["tries"]])
    check("real proxy: model per try", [t["model"] for t in rec["tries"]] == ["gemini-3.5-flash-lite"] * 3)
    check("real proxy: no problems, gate pass, 2 counted", rec["gate_pass"] and not rec["problems"] and rec["tries_counted"] == 2, rec["problems"])
    left = no_secrets([out, recp, pu])
    check("real proxy: no message text in records or proxy log", not left, left)


def part_runner(_unused):
    # Own temp dir with mode 755: the agent user must reach the H1 log copy in it.
    base = tempfile.mkdtemp(prefix="replaytest-"); os.chmod(base, 0o755)
    try:
        _part_runner(base)
    finally:
        subprocess.run(["sudo", "-n", "chown", "-R", str(os.getuid()), base])
        shutil.rmtree(base, ignore_errors=True)


def _part_runner(base):
    pc = os.environ.get("PROXY_COMMIT", "4b02f425bedc94b2829d1d94f84005049af4e602")
    need = [subprocess.run(["sudo", "-n", "true"], capture_output=True).returncode == 0,
            subprocess.run(["id", "benchagent"], capture_output=True).returncode == 0,
            os.path.exists("/opt/benchproxy/%s/proxy.py" % pc),
            os.path.exists(HB) and R.sha256_file(HB) == R.EXPECTED_HARNESS_SHA256]
    if not all(need):
        return skip("runner: BENCH_REPLAY end to end", "needs sudo, user benchagent, pin dir and HB")
    up = Upstream([(200, 669, "max_tokens", "gemini-3.5-flash-lite"), (429, 0, None, None),
                   (200, 669, "STOP", "gemini-3.5-flash-lite")])
    runs = os.path.join(base, "runs"); os.mkdir(runs)
    md = os.path.join(base, "meta"); ml = os.path.join(base, "meta.log")
    h1 = os.path.join(base, "H1"); os.makedirs(os.path.join(h1, "rec"))
    open(os.path.join(h1, "rec", "run.jsonl"), "w").write("".join(json.dumps(e) + "\n" for e in H1_EVENTS))
    os.chmod(h1, 0o755); os.chmod(os.path.join(h1, "rec"), 0o755)
    env = dict(os.environ, BENCH_REPLAY="h1req2", BENCH_TEST_UPSTREAM="http://127.0.0.1:%d" % up.port, HB=HB,
               PROXY_COMMIT=pc, RUNS=runs, META_DIR=md, META_LOG=ml, REPLAY_TRIES="2", REPLAY_H1_RUN=h1,
               IV="200", TAG="replaytest", SUMMARIZER="")
    try:
        r = subprocess.run([os.path.join(HERE, "run_isolated.sh"), "T1", "R1"], env=env, capture_output=True, text=True, timeout=600)
    finally:
        up.close()
    d = r.stdout.strip().splitlines()[-1] if r.stdout.strip() else ""
    check("runner: exit 0", r.returncode == 0, r.stderr[-400:])
    if not d or not os.path.isdir(d):
        return check("runner: run folder exists", False, r.stdout[-300:])
    rec = json.load(open(os.path.join(d, "replay_record.json")))
    check("runner: calibration + 2 tries, 1 counted (429 excluded)",
          len(rec["tries"]) == 3 and rec["tries_counted"] == 1 and rec["tries_429"] == 1, rec)
    check("runner: finish_reason_raw MAX_TOKENS on calibration", rec["tries"][0]["finish_reason_raw"] == "MAX_TOKENS")
    check("runner: pin path in record", rec["pin"] == "/opt/benchproxy/%s" % pc)
    st = os.stat(os.path.join(d, "run_meta.json"))
    check("runner: run_meta.json root-owned 644", st.st_uid == 0 and stat.S_IMODE(st.st_mode) == 0o644)
    meta_sha = R.sha256_file(os.path.join(d, "run_meta.json"))
    check("runner: run_meta sha256 logged", meta_sha in open(ml).read())
    check("runner: run_env marked fake (test upstream)", json.load(open(os.path.join(d, "run_env.json"))).get("fake") is True)
    left = [f for f in os.listdir(d) if f in ("body.json", "prompt.txt")]
    check("runner: no body or prompt file in the run folder", not left, left)
    allf = [os.path.join(d, f) for f in os.listdir(d) if os.path.isfile(os.path.join(d, f))]
    raw = "".join(open(p, errors="replace").read() for p in allf)
    check("runner: no reply text and no tool result text in the run folder",
          "SECRET_REPLY_TEXT_77" not in raw and "Cargo.lock\\nCargo.toml" not in raw and H1_RESULT not in raw)
    g = subprocess.run([sys.executable, os.path.join(HERE, "run_guard.py"), d], capture_output=True, text=True)
    check("runner: folder named replay-h1req2-<ts>", os.path.basename(d).startswith("replay-h1req2-"), d)
    check("runner: run_env.json has replay h1req2", json.load(open(os.path.join(d, "run_env.json"))).get("replay") == "h1req2")
    check("runner: run_guard refuses the replay folder", g.returncode == 3 and "replay" in g.stderr, g.stderr)
    t = subprocess.run([sys.executable, os.path.join(HERE, "results_table.py"), d], capture_output=True, text=True)
    check("runner: results_table does not count the replay folder", "REFUSED" in (t.stdout + t.stderr) and "replay" in (t.stdout + t.stderr), (t.stdout + t.stderr)[-300:])


def main():
    with tempfile.TemporaryDirectory() as base:
        for fn in (part_build, part_gate, part_429, part_text_and_model, part_error_status, part_rows_mismatch,
                   part_real_proxy, part_runner):
            sub = os.path.join(base, fn.__name__); os.mkdir(sub)
            fn(sub)
    print("OVERALL %s (fail=%d skip=%d)" % ("PASS" if fails == 0 else "FAIL", fails, skips))
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())
