#!/usr/bin/env python3
"""Replay of H1 request 2 (T1 v2 pair 1, Harness e0fc51e). Bench tooling only.

Subcommands:
  build      Rebuild request 2 with the real Harness binary (e0fc51e) and a LOCAL fake upstream.
             No network, no key. The fake upstream gives request 1 the logged list_files call.
             The Harness binary then sends request 2. We capture that body and check it
             against the H1 log (rec/run.jsonl): tool call id, tool result (word for word),
             message count, tool count. Output: the body (mode 600, never in git or logs)
             and fields.json (field names and non-message values only).
  send       Send the body to the bench proxy (127.0.0.1:<port>). One calibration try,
             then N tries, one at a time. The proxy paces them (min interval).
             Gate: the calibration try must be HTTP 200 with prompt_tokens == expected (669).
             If not, stop. No try counts. The calibration try never counts.
             A try with HTTP 429 (or any non-200) is upstream-affected and does not count.
  summarize  Join the send records with the proxy usage rows (finish_reason_raw comes from
             the proxy row, never from message text). Write replay_record.json.

Records hold only: HTTP status, token counts, finish_reason_raw, the reply's model id,
timestamps, field names and non-message values, hashes and lengths. Never message text,
never headers with keys, never the key.
"""
import argparse, hashlib, json, os, re, socket, subprocess, sys, threading, time
import http.client
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

EXPECTED_HARNESS_SHA256 = "df9db25d33726e9d81122a6abd8aa69cf523ccae8e8a1de27fa901b574a599c9"
H1_MODEL = "gemini-3.5-flash-lite"
H1_ALLOW = "coding,shell"
H1_MAX_STEPS = "50"
# Fields the client could send. We list each one in fields.json, with "absent" if the body has none.
CHECK_FIELDS = ("model", "stream", "temperature", "top_p", "tool_choice", "max_tokens",
                "max_completion_tokens", "n", "stop", "seed", "stream_options", "reasoning_effort",
                "parallel_tool_calls", "response_format", "presence_penalty", "frequency_penalty",
                "user", "extra_body")
# Upstream error code (Google style error.status) for non-200 replies. Fixed words only.
# The pinned proxy (4b02f42) forwards a non-200, non-stream upstream body unchanged, so the client reads it here.
ERROR_STATUS_WORDS = ("INVALID_ARGUMENT", "FAILED_PRECONDITION", "PERMISSION_DENIED", "UNAUTHENTICATED",
                      "NOT_FOUND", "RESOURCE_EXHAUSTED", "UNAVAILABLE", "INTERNAL", "DEADLINE_EXCEEDED", "CANCELLED")
MODEL_ID_RE = re.compile(r"^[A-Za-z0-9._:/-]{1,64}$")


def sha256_bytes(b):
    return hashlib.sha256(b).hexdigest()


def sha256_file(p):
    with open(p, "rb") as f:
        return sha256_bytes(f.read())


def model_id(v):
    """Short identifier only. Anything else is not logged as text."""
    if v is None:
        return None
    if isinstance(v, str) and MODEL_ID_RE.match(v):
        return v
    return "unlisted_format"


def error_status(data):
    """error.status as an allow-listed word, "unlisted" if unknown, None if missing. Never message text.
    Gemini's OpenAI-compatible endpoint can send a list with one error object; both shapes are read."""
    try:
        o = json.loads(data)
    except (ValueError, TypeError):
        return None
    if isinstance(o, list) and o:
        o = o[0]
    e = o.get("error") if isinstance(o, dict) else None
    v = e.get("status") if isinstance(e, dict) else None
    if v is None or v == "":
        return None
    if isinstance(v, str) and v.strip().upper() in ERROR_STATUS_WORDS:
        return v.strip().upper()
    return "unlisted"


# ---------------------------------------------------------------- build

def read_h1_log(path):
    """From rec/run.jsonl: the first list_files call and its result, plus request shapes."""
    ev = [json.loads(l) for l in open(path) if l.strip()]
    reqs = [e for e in ev if e.get("type") == "model_request"]
    call = next(e for e in ev if e.get("type") == "tool_call")
    res = next(e for e in ev if e.get("type") == "tool_result" and e.get("id") == call["id"])
    if len(reqs) < 2:
        raise SystemExit("build: H1 log has fewer than 2 model requests")
    return {"call_id": call["id"], "call_name": call["name"], "call_args": call.get("arguments", {}),
            "result": res["content"], "req1": reqs[0], "req2": reqs[1]}


class _Fake:
    """Local fake upstream for the build step. Answers request 1 with the logged tool call,
    request 2 with a plain stop reply, and keeps the raw bodies and the non-auth headers."""

    def __init__(self, h1):
        self.h1, self.bodies, self.headers = h1, [], []
        fake = self

        class H(BaseHTTPRequestHandler):
            def log_message(self, *a):
                pass

            def do_POST(self):
                n = int(self.headers.get("Content-Length") or 0)
                body = self.rfile.read(n)
                fake.bodies.append(body)
                fake.headers.append({k: v for k, v in self.headers.items() if k.lower() != "authorization"})
                if len(fake.bodies) == 1:
                    msg = {"role": "assistant", "content": None, "tool_calls": [{
                        "id": fake.h1["call_id"], "type": "function",
                        "function": {"name": fake.h1["call_name"], "arguments": json.dumps(fake.h1["call_args"])}}]}
                    fr = "tool_calls"
                else:
                    msg, fr = {"role": "assistant", "content": None}, "stop"
                out = json.dumps({"id": "fake", "object": "chat.completion", "model": H1_MODEL,
                                  "choices": [{"index": 0, "message": msg, "finish_reason": fr}],
                                  "usage": {"prompt_tokens": 1, "completion_tokens": 1, "total_tokens": 2}}).encode()
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(out)))
                self.end_headers()
                self.wfile.write(out)

        self.srv = ThreadingHTTPServer(("127.0.0.1", 0), H)
        self.port = self.srv.server_address[1]
        threading.Thread(target=self.srv.serve_forever, daemon=True).start()

    def close(self):
        self.srv.shutdown()


def describe_body(body, req_headers):
    """Field names and non-message values. Message text is replaced by its sha256 and length."""
    obj = json.loads(body)
    fields = {k: (obj[k] if k in obj else "absent") for k in CHECK_FIELDS}
    msgs = []
    for m in obj.get("messages", []):
        d = {"role": m.get("role"), "keys": sorted(m.keys())}
        if isinstance(m.get("content"), str):
            d["content_sha256"] = sha256_bytes(m["content"].encode())
            d["content_chars"] = len(m["content"])
        elif "content" in m:
            d["content"] = m["content"]  # null only
        if m.get("tool_calls"):
            # arguments is model output: keep only its sha256 and length
            d["tool_calls"] = [{"id": c.get("id"), "type": c.get("type"), "name": c["function"]["name"],
                                "arguments_sha256": sha256_bytes(str(c["function"]["arguments"]).encode()),
                                "arguments_chars": len(str(c["function"]["arguments"])), "keys": sorted(c.keys())}
                               for c in m["tool_calls"]]
        for k in ("tool_call_id", "name"):
            if k in m:
                d[k] = m[k]
        msgs.append(d)
    tools = obj.get("tools") or []
    return {
        "top_level_keys": sorted(obj.keys()),
        "fields": fields,
        "messages": msgs,
        "tools": {"count": len(tools), "names": [t["function"]["name"] for t in tools],
                  "sha256": sha256_bytes(json.dumps(tools, sort_keys=True).encode())},
        "body_sha256": sha256_bytes(body), "body_bytes": len(body),
        "client_headers": {k: v for k, v in req_headers.items() if k.lower() in ("content-type", "user-agent", "accept")},
    }


def cmd_build(a):
    if sha256_file(a.harness) != EXPECTED_HARNESS_SHA256:
        raise SystemExit("build: harness binary sha256 is not the e0fc51e build (%s)" % EXPECTED_HARNESS_SHA256)
    h1 = read_h1_log(a.h1_rec)
    prompt = open(a.prompt_file).read()
    fake = _Fake(h1)
    rec = os.path.join(os.path.dirname(os.path.abspath(a.out)), "build_rec")
    env = {"PATH": "/usr/local/bin:/usr/bin:/bin", "HOME": os.environ.get("HOME", "/tmp"), "LANG": "C.UTF-8",
           "HARNESS_BASE_URL": "http://127.0.0.1:%d" % fake.port, "HARNESS_MODEL": H1_MODEL,
           "HARNESS_API_KEY": "proxy-injected", "FX_MAX_AGENT_STEPS": H1_MAX_STEPS}
    try:
        r = subprocess.run([a.harness, "ask", "--model", "openai", "--allow", H1_ALLOW, "--workspace", a.ws,
                            "--max-steps", H1_MAX_STEPS, "--record", rec, "--prompt", prompt],
                           env=env, stdin=subprocess.DEVNULL, capture_output=True, timeout=120)
    finally:
        fake.close()
    if r.returncode != 0:
        raise SystemExit("build: harness exit %d" % r.returncode)
    if len(fake.bodies) != 2:
        raise SystemExit("build: expected 2 requests from harness, got %d" % len(fake.bodies))
    b1, b2 = (json.loads(b) for b in fake.bodies)
    m = b2.get("messages") or []
    errs = []
    if len(b1.get("messages") or []) != h1["req1"]["message_count"]:
        errs.append("request 1 message count")
    if len(m) != h1["req2"]["message_count"]:
        errs.append("request 2 message count")
    if len(b2.get("tools") or []) != h1["req2"]["tool_count"] or b1.get("tools") != b2.get("tools"):
        errs.append("tool count or tool list")
    if [x.get("role") for x in m] != ["user", "assistant", "tool"]:
        errs.append("request 2 roles")
    else:
        if m[0].get("content") != prompt:
            errs.append("task message is not the prompt file")
        tc = (m[1].get("tool_calls") or [{}])[0]
        if tc.get("id") != h1["call_id"] or tc.get("function", {}).get("name") != h1["call_name"]:
            errs.append("assistant tool call id or name")
        if m[2].get("content") != h1["result"]:
            errs.append("tool result is not word for word the H1 log")
        if m[2].get("tool_call_id") != h1["call_id"]:
            errs.append("tool_call_id")
    if errs:
        raise SystemExit("build: request 2 does not match H1: " + "; ".join(errs))
    fd = os.open(a.out, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(fd, "wb") as f:
        f.write(fake.bodies[1])
    desc = describe_body(fake.bodies[1], fake.headers[1])
    desc.update({"harness_sha256": EXPECTED_HARNESS_SHA256, "harness_args": ["ask", "--model", "openai", "--allow", H1_ALLOW,
                 "--max-steps", H1_MAX_STEPS], "harness_env_model": H1_MODEL,
                 "prompt_sha256": sha256_bytes(prompt.encode()), "h1_call_id": h1["call_id"],
                 "tool_result_sha256": sha256_bytes(h1["result"].encode()),
                 "note": "Upstream thought signature (extra_content) of H1 request 2 was held only in the H1 proxy memory. "
                         "It is not in this body, and the new proxy has none to add."})
    with open(a.fields_out, "w") as f:
        json.dump(desc, f, indent=1)
    print("build: ok body_sha256=%s bytes=%d" % (desc["body_sha256"], desc["body_bytes"]))


# ---------------------------------------------------------------- send

def _post(port, body, headers, timeout):
    c = http.client.HTTPConnection("127.0.0.1", port, timeout=timeout)
    try:
        c.request("POST", "/chat/completions", body=body, headers=headers)
        r = c.getresponse()
        data = r.read()
        return r.status, data
    finally:
        c.close()


def one_try(port, body, headers, idx, kind, timeout):
    t0 = int(time.time() * 1000)
    rec = {"try": idx, "kind": kind, "start_ms": t0}
    try:
        status, data = _post(port, body, headers, timeout)
    except (OSError, http.client.HTTPException) as e:
        rec.update({"status": None, "error_type": type(e).__name__})
        status, data = None, b""
    rec["end_ms"] = int(time.time() * 1000)
    rec["status"] = status
    inp = out = mdl = None
    if status == 200:
        try:
            o = json.loads(data)
            u = o.get("usage") or {}
            inp = u.get("prompt_tokens") if isinstance(u.get("prompt_tokens"), int) else None
            out = u.get("completion_tokens") if isinstance(u.get("completion_tokens"), int) else None
            mdl = model_id(o.get("model"))
        except (ValueError, AttributeError):
            rec["parse_error"] = True
    rec.update({"input_tokens": inp, "output_tokens": out, "model": mdl})
    rec["error_status"] = error_status(data) if status is not None and status != 200 else None
    if status == 429:
        rec["upstream_affected"] = "429"
    return rec


def cmd_send(a):
    body = open(a.body, "rb").read()
    fields = json.load(open(a.fields)) if a.fields else {}
    headers = {"Content-Type": "application/json", "Authorization": "Bearer proxy-injected"}
    ua = (fields.get("client_headers") or {}).get("User-Agent")
    if ua:
        headers["User-Agent"] = ua
    recs = []

    def emit(r):
        recs.append(r)
        with open(a.out, "a") as f:
            f.write(json.dumps(r) + "\n")

    cal = one_try(a.port, body, headers, 0, "calibration", a.timeout)
    cal["counted"] = False
    cal["gate_expected_input_tokens"] = a.expect_input
    cal["gate_pass"] = cal["status"] == 200 and cal["input_tokens"] == a.expect_input
    emit(cal)
    if not cal["gate_pass"]:
        print("send: GATE FAIL status=%s input_tokens=%s expected=%d; no try counts"
              % (cal["status"], cal["input_tokens"], a.expect_input), file=sys.stderr)
        return 5
    for i in range(1, a.tries + 1):
        r = one_try(a.port, body, headers, i, "try", a.timeout)
        r["counted"] = r["status"] == 200
        emit(r)
    print("send: done tries=%d counted=%d" % (a.tries, sum(r["counted"] for r in recs)))
    return 0


# ---------------------------------------------------------------- summarize

def cmd_summarize(a):
    tries = [json.loads(l) for l in open(a.tries) if l.strip()] if os.path.exists(a.tries) else []
    rows = [json.loads(l) for l in open(a.proxy_usage) if l.strip()] if os.path.exists(a.proxy_usage) else []
    fields = json.load(open(a.fields)) if a.fields and os.path.exists(a.fields) else None
    problems = []
    if len(rows) != len(tries):
        problems.append("proxy rows %d != tries %d" % (len(rows), len(tries)))
    out = []
    for i, t in enumerate(tries):
        row = rows[i] if i < len(rows) else {}
        r = {k: t.get(k) for k in ("try", "kind", "status", "error_status", "input_tokens", "output_tokens", "model",
                                   "start_ms", "end_ms", "counted")}
        r["finish_reason_raw"] = row.get("finish_reason_raw")
        r["proxy_status"] = row.get("status")
        r["proxy_input_tokens"] = row.get("input_tokens")
        r["proxy_output_tokens"] = row.get("output_tokens")
        if row and (row.get("status") != t.get("status")):
            problems.append("try %s: status differs from proxy row" % t.get("try"))
            r["counted"] = False
        if row.get("status") == 429:
            r["counted"] = False
        out.append(r)
    mismatch = len(rows) != len(tries)
    if mismatch:  # rows cannot be joined to tries with trust: nothing counts
        for r in out:
            r["counted"] = False
    cal = out[0] if out else None
    gate = bool(cal and cal.get("status") == 200 and cal.get("input_tokens") == a.expect_input)
    if not gate:
        for r in out:
            r["counted"] = False
    rec = {"replay": "H1 request 2", "h1_run": a.h1_run, "pin": a.pin, "proxy_py_sha256": a.proxy_sha256,
           "expected_input_tokens": a.expect_input, "gate_pass": gate, "rows_mismatch": mismatch,
           "calibration_counts": False, "tries_planned": a.planned,
           "tries_counted": sum(1 for r in out if r["counted"]),
           "tries_429": sum(1 for r in out if r.get("status") == 429 or r.get("proxy_status") == 429),
           "problems": problems, "request_fields": fields, "tries": out}
    with open(a.out, "w") as f:
        json.dump(rec, f, indent=1)
    print("summarize: gate_pass=%s counted=%d problems=%d" % (gate, rec["tries_counted"], len(problems)))
    return 0 if gate and not problems else 6


def main(argv=None):
    ap = argparse.ArgumentParser()
    sp = ap.add_subparsers(dest="cmd", required=True)
    b = sp.add_parser("build")
    b.add_argument("--harness", required=True)
    b.add_argument("--ws", required=True)
    b.add_argument("--h1-rec", required=True)
    b.add_argument("--prompt-file", required=True)
    b.add_argument("--out", required=True)
    b.add_argument("--fields-out", required=True)
    s = sp.add_parser("send")
    s.add_argument("--port", type=int, required=True)
    s.add_argument("--body", required=True)
    s.add_argument("--fields", default=None)
    s.add_argument("--tries", type=int, default=10)
    s.add_argument("--expect-input", type=int, default=669)
    s.add_argument("--timeout", type=float, default=120)
    s.add_argument("--out", required=True)
    m = sp.add_parser("summarize")
    m.add_argument("--tries", required=True)
    m.add_argument("--proxy-usage", required=True)
    m.add_argument("--fields", default=None)
    m.add_argument("--pin", required=True)
    m.add_argument("--proxy-sha256", default=None)
    m.add_argument("--h1-run", default=None)
    m.add_argument("--expect-input", type=int, default=669)
    m.add_argument("--planned", type=int, default=10)
    m.add_argument("--out", required=True)
    a = ap.parse_args(argv)
    return {"build": cmd_build, "send": cmd_send, "summarize": cmd_summarize}[a.cmd](a) or 0


if __name__ == "__main__":
    sys.exit(main())
