#!/usr/bin/env python3
"""Bench-only OpenAI-compat stream normalizer for Gemini. Benchmark tooling, not product.

See README.md for exactly what is rewritten. Stdlib only. Never logs headers or keys.
"""
import json, os, sys, threading, time, http.client, urllib.parse
from http.server import ThreadingHTTPServer, BaseHTTPRequestHandler

UPSTREAM = os.environ.get("PROXY_UPSTREAM", "https://generativelanguage.googleapis.com/v1beta/openai")
# SIG_MODE: "reinject" (default) strips extra_content from responses, remembers thought
# signatures by tool_call id, and re-adds them to outgoing requests that lack them.
# "strip" strips and never re-adds (used only to test whether signatures are required).
SIG_MODE = os.environ.get("PROXY_SIG_MODE", "reinject")

# Upstream key injection (key isolation): when PROXY_KEY_FILE is set, the proxy reads
# KEY_NAME=value from that file (owned by the proxy user, mode 600) and sends its own
# Authorization header; any client Authorization header is dropped. The key lives only
# in this process's memory, never in its environment, logs or responses.
UPSTREAM_KEY = None
ALLOWED_POST_PATHS = {"/chat/completions"}


def load_key_file(path, name):
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line.startswith(name + "="):
                v = line.split("=", 1)[1].strip().strip('"').strip("'")
                if v:
                    return v
    raise SystemExit(f"key {name} not found in key file")


_SIGS = {}
_LOCK = threading.Lock()
STATS = {"sigs_stored": 0, "sigs_reinjected": 0, "chunks_dropped": 0, "indices_added": 0}


def remember(tc):
    ec = tc.get("extra_content")
    if ec and tc.get("id"):
        with _LOCK:
            _SIGS[tc["id"]] = ec
            STATS["sigs_stored"] += 1


class StreamRewriter:
    """Stateful per-response rewriter for SSE chunk objects."""

    def __init__(self):
        self.ids = {}      # (choice_index, tool_call_id) -> index
        self.next = {}     # choice_index -> next index
        self.last = {}     # choice_index -> last assigned index
        self.pending_usage = None
        self.last_usage = None  # final usage seen in the stream (counts only; for the usage log)

    def chunk(self, obj):
        """Return rewritten obj, or None to drop it."""
        usage = obj.pop("usage", None)
        if usage is not None:
            self.pending_usage = usage  # keep only the last one; emitted on final chunk
            self.last_usage = usage
        obj.pop("extra_content", None)
        keep = []
        for ch in obj.get("choices") or []:
            ci = ch.get("index", 0)
            ch.pop("extra_content", None)
            d = ch.get("delta")
            if isinstance(d, dict):
                d.pop("extra_content", None)
                tcs = d.get("tool_calls")
                if isinstance(tcs, list):
                    for tc in tcs:
                        remember(tc)
                        tc.pop("extra_content", None)
                        if "index" not in tc:
                            STATS["indices_added"] += 1
                            tid = tc.get("id")
                            if tid is None and ci in self.last:
                                tc["index"] = self.last[ci]
                            else:
                                key = (ci, tid)
                                if key not in self.ids:
                                    self.ids[key] = self.next.get(ci, 0)
                                    self.next[ci] = self.ids[key] + 1
                                tc["index"] = self.ids[key]
                        self.last[ci] = tc["index"]
                    if not tcs:
                        d.pop("tool_calls")
                if not d and ch.get("finish_reason") is None:
                    continue
            # 5. finish_reason "stop" after tool calls in this choice -> "tool_calls"
            if ch.get("finish_reason") == "stop" and ci in self.next:
                ch["finish_reason"] = "tool_calls"
                STATS["finish_rewritten"] = STATS.get("finish_rewritten", 0) + 1
            keep.append(ch)
        if "choices" in obj:
            if not keep:
                STATS["chunks_dropped"] += 1
                return None
            obj["choices"] = keep
        if any(c.get("finish_reason") is not None for c in keep) and self.pending_usage is not None:
            obj["usage"] = self.pending_usage
            self.pending_usage = None
        return obj

    def flush_usage(self):
        """If usage never got attached (no finish chunk), emit it as a final usage-only chunk."""
        u, self.pending_usage = self.pending_usage, None
        return u


def rewrite_nonstream(obj):
    obj.pop("extra_content", None)
    for ch in obj.get("choices") or []:
        ch.pop("extra_content", None)
        m = ch.get("message") or {}
        m.pop("extra_content", None)
        if m.get("tool_calls") and ch.get("finish_reason") == "stop":
            ch["finish_reason"] = "tool_calls"
        for i, tc in enumerate(m.get("tool_calls") or []):
            remember(tc)
            tc.pop("extra_content", None)
    return obj


def rewrite_request(obj):
    if SIG_MODE != "reinject":
        return obj
    for m in obj.get("messages") or []:
        for tc in (m.get("tool_calls") or []) if isinstance(m, dict) else []:
            if "extra_content" not in tc and tc.get("id") in _SIGS:
                with _LOCK:
                    tc["extra_content"] = _SIGS[tc["id"]]
                    STATS["sigs_reinjected"] += 1
    return obj


class Pacer:
    """Holds each upstream request until min_interval_ms has passed since the previous one started.
    Counts only (no headers, keys or bodies). min_interval_ms=0 disables holding."""

    def __init__(self, min_interval_ms=0, clock=None, sleep=None):
        import time
        self.min_s = max(0, min_interval_ms) / 1000.0
        self.clock = clock or time.monotonic
        self.sleep = sleep or time.sleep
        self.lock = threading.Lock()
        self.reset()

    def reset(self):
        self.last_start = None
        self.requests = 0
        self.held = 0
        self.hold_ms = 0.0
        self.status_429 = 0

    def wait(self):
        """Block as needed, then mark this request as started. Returns hold ms for this request."""
        with self.lock:  # serialises starts so the interval holds under concurrency
            hold = 0.0
            now = self.clock()
            if self.min_s > 0 and self.last_start is not None:
                hold = max(0.0, self.last_start + self.min_s - now)
                if hold > 0:
                    self.sleep(hold)
                    now = self.clock()
            self.last_start = now
            self.requests += 1
            if hold > 0:
                self.held += 1
                self.hold_ms += hold * 1000.0
            return hold * 1000.0

    def note_status(self, status):
        if status == 429:
            with self.lock:
                self.status_429 += 1

    def snapshot(self):
        return {"min_interval_ms": int(self.min_s * 1000), "upstream_requests": self.requests,
                "held_requests": self.held, "hold_ms": int(round(self.hold_ms)), "status_429": self.status_429}


PACER = Pacer(0)
HOLD_FILE = None  # optional path; rewritten with PACER.snapshot() after each upstream response


USAGE_FILE = None  # optional path; one JSON line per forwarded request, counts only
# Run ID comes ONLY from this operator-owned file (root-owned, not writable by the agent user),
# re-read on every request. Never from request headers or body.
RUN_ID_FILE = "/opt/benchproxy/run/current_run_id"


def current_run_id():
    try:
        with open(RUN_ID_FILE) as f:
            v = f.read(200).strip()
        return v if v and all(c.isalnum() or c in "-_.:" for c in v) else "invalid"
    except OSError:
        return None
_USAGE_LOCK = threading.Lock()


def _now_ms():
    return int(time.time() * 1000)


def no_active_run():
    """True when the operator's run-id file is empty or missing: the runner cleared it at run end."""
    try:
        with open(RUN_ID_FILE) as f:
            return f.read(200).strip() == ""
    except OSError:
        return True


def log_no_run(start_ms):
    """Request outside any run: status "no_run" plus times; nothing else."""
    if not USAGE_FILE:
        return
    end = _now_ms()
    line = json.dumps({"status": "no_run", "start_ms": start_ms, "end_ms": end, "dur_ms": end - start_ms}) + "\n"
    with _USAGE_LOCK:
        with open(USAGE_FILE, "a") as f:
            f.write(line)


def usage_record(status, usage, stream, start_ms=None, end_ms=None):
    """Counts-only record: never bodies, headers or keys. start_ms = upstream send (after pacing), end_ms = response fully relayed."""
    u = usage if isinstance(usage, dict) else {}
    def num(*names):
        for n in names:
            v = u.get(n)
            if isinstance(v, (int, float)):
                return int(v)
        return None
    return {"ts_ms": int(__import__("time").time() * 1000), "run_id": current_run_id(), "status": int(status),
            "input_tokens": num("prompt_tokens", "input_tokens"),
            "output_tokens": num("completion_tokens", "output_tokens"),
            "start_ms": start_ms, "end_ms": end_ms,
            "dur_ms": (end_ms - start_ms) if start_ms is not None and end_ms is not None else None}


def log_usage(status, usage, stream, start_ms=None):
    if not USAGE_FILE:
        return
    line = json.dumps(usage_record(status, usage, stream, start_ms, _now_ms() if start_ms is not None else None)) + "\n"
    with _USAGE_LOCK:
        with open(USAGE_FILE, "a") as f:
            f.write(line)


def log_transport_error(start_ms):
    """Upstream transport error: status "error" plus times; no other fields, no exception text."""
    if not USAGE_FILE:
        return
    end = _now_ms()
    line = json.dumps({"status": "error", "start_ms": start_ms, "end_ms": end, "dur_ms": end - start_ms}) + "\n"
    with _USAGE_LOCK:
        with open(USAGE_FILE, "a") as f:
            f.write(line)


def write_hold_file():
    if HOLD_FILE:
        tmp = HOLD_FILE + ".tmp"
        with open(tmp, "w") as f:
            json.dump(PACER.snapshot(), f)
        os.replace(tmp, HOLD_FILE)


class H(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def _not_found(self):
        n = int(self.headers.get("Content-Length") or 0)
        if n:
            self.rfile.read(n)
        body = b'{"error":"not found"}'
        self.send_response(404)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)
        sys.stderr.write(f"{self.command} (blocked path) -> 404\n")

    def do_POST(self):
        # Allow-list: only the chat-completions call the agents make reaches upstream.
        if self.path.split("?")[0].rstrip("/") in ALLOWED_POST_PATHS:
            self._fwd()
        else:
            self._not_found()

    def do_PUT(self):
        self._not_found()

    do_DELETE = do_PATCH = do_HEAD = do_OPTIONS = do_PUT

    def do_GET(self):
        # Cheap local health check; never contacts upstream. Every other GET is 404.
        if self.path.split("?")[0].rstrip("/") == "/healthz":
            body = json.dumps({"ok": True, "key_injection": UPSTREAM_KEY is not None}).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
        else:
            self._not_found()

    def log_message(self, *a):  # never log headers; minimal path-only log
        pass

    def _fwd(self):
        t_in = _now_ms()
        n = int(self.headers.get("Content-Length") or 0)
        body = self.rfile.read(n) if n else None
        if no_active_run():
            # No run is active (run-id file empty): never forward upstream; 503 locally, times-only log row.
            log_no_run(t_in)
            self.send_response(503)
            self.send_header("Content-Length", "0")
            self.end_headers()
            self.close_connection = True
            return
        if body:
            try:
                body = json.dumps(rewrite_request(json.loads(body))).encode()
            except ValueError:
                pass
        u = urllib.parse.urlsplit(UPSTREAM)
        path = self.path if not self.path.startswith(u.path) else self.path[len(u.path):]
        if u.scheme == "http":  # tests only (local mock upstream)
            conn = http.client.HTTPConnection(u.hostname, u.port or 80, timeout=300)
        else:
            conn = http.client.HTTPSConnection(u.hostname, u.port or 443, timeout=300)
        hdrs = {"Content-Type": self.headers.get("Content-Type", "application/json"), "Accept-Encoding": "identity"}
        if UPSTREAM_KEY is not None:
            hdrs["Authorization"] = "Bearer " + UPSTREAM_KEY
        elif self.headers.get("Authorization"):
            hdrs["Authorization"] = self.headers["Authorization"]
        if self.headers.get("Accept"):
            hdrs["Accept"] = self.headers["Accept"]
        PACER.wait()
        t0 = _now_ms()
        try:
            conn.request(self.command, u.path + path, body=body, headers=hdrs)
            r = conn.getresponse()
        except Exception:
            # Transport error (connect/timeout/reset): log times only, never the exception text.
            log_transport_error(t0)
            try:
                conn.close()
                self.send_response(502)
                self.send_header("Content-Length", "0")
                self.end_headers()
            except Exception:
                pass
            self.close_connection = True
            return
        ctype = r.getheader("Content-Type", "")
        PACER.note_status(r.status)
        write_hold_file()
        sys.stderr.write(f"{self.command} {path.split('?')[0]} -> {r.status}\n")
        if "text/event-stream" in ctype and r.status == 200:
            self.send_response(200)
            self.send_header("Content-Type", "text/event-stream")
            self.send_header("Cache-Control", "no-cache")
            self.send_header("Connection", "close")
            self.end_headers()
            rw = StreamRewriter()
            try:
                for out in sse_rewrite(r, rw):
                    self.wfile.write(out)
                    self.wfile.flush()
            finally:
                log_usage(r.status, rw.last_usage, True, t0)
            self.close_connection = True
            return
        data = r.read()
        usage = None
        if "json" in ctype:
            try:
                usage = json.loads(data).get("usage")
            except (ValueError, AttributeError):
                usage = None
        log_usage(r.status, usage, False, t0)
        if r.status == 200 and "json" in ctype:
            try:
                data = json.dumps(rewrite_nonstream(json.loads(data))).encode()
            except ValueError:
                pass
        self.send_response(r.status)
        self.send_header("Content-Type", ctype or "application/json")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)



def sse_rewrite(lines, rw):
    """Yield rewritten SSE bytes from an iterable of raw lines."""
    for raw in lines:
        line = raw.decode("utf-8").rstrip("\r\n")
        if not line.startswith("data:"):
            continue
        payload = line[5:].strip()
        if payload == "[DONE]":
            u = rw.flush_usage()
            if u is not None:
                yield b"data: " + json.dumps({"choices": [], "usage": u}).encode() + b"\n\n"
            yield b"data: [DONE]\n\n"
            continue
        try:
            obj = rw.chunk(json.loads(payload))
        except ValueError:
            yield (line + "\n\n").encode()
            continue
        if obj is not None:
            yield b"data: " + json.dumps(obj, separators=(",", ":")).encode() + b"\n\n"


BIND_HOST = "127.0.0.1"  # loopback only; never 0.0.0.0


def make_server(port):
    srv = ThreadingHTTPServer((BIND_HOST, port), H)
    assert srv.server_address[0] == "127.0.0.1", "proxy must bind loopback only"
    return srv


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("port", nargs="?", type=int, default=8787)
    ap.add_argument("--min-interval-ms", type=int, default=0,
                    help="hold each upstream request until this long after the previous one started (0 = off)")
    ap.add_argument("--hold-file", default=None, help="write per-run pacing counts (JSON, counts only) here")
    ap.add_argument("--usage-file", default=None, help="append one counts-only JSON line per forwarded request")
    a = ap.parse_args()
    port = a.port
    PACER = Pacer(a.min_interval_ms)
    HOLD_FILE = a.hold_file
    USAGE_FILE = a.usage_file
    RUN_ID_FILE = os.environ.get("PROXY_RUN_ID_FILE", RUN_ID_FILE)
    kf = os.environ.get("PROXY_KEY_FILE")
    if kf:
        UPSTREAM_KEY = load_key_file(kf, os.environ.get("PROXY_KEY_NAME", "GEMINI_API_KEY"))
    write_hold_file()
    srv = make_server(port)
    sys.stderr.write(f"oai_stream_proxy on 127.0.0.1:{port} sig_mode={SIG_MODE} min_interval_ms={a.min_interval_ms}\n")
    try:
        srv.serve_forever()
    finally:
        sys.stderr.write(f"stats {json.dumps(STATS)}\n")
        sys.stderr.write(f"pacing {json.dumps(PACER.snapshot())}\n")
