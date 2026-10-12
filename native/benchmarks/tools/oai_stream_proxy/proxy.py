#!/usr/bin/env python3
"""Bench-only OpenAI-compat stream normalizer for Gemini. Benchmark tooling, not product.

See README.md for exactly what is rewritten. Stdlib only. Never logs headers or keys.
"""
import json, os, sys, threading, http.client, urllib.parse
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

    def chunk(self, obj):
        """Return rewritten obj, or None to drop it."""
        usage = obj.pop("usage", None)
        if usage is not None:
            self.pending_usage = usage  # keep only the last one; emitted on final chunk
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


def write_hold_file():
    if HOLD_FILE:
        tmp = HOLD_FILE + ".tmp"
        with open(tmp, "w") as f:
            json.dump(PACER.snapshot(), f)
        os.replace(tmp, HOLD_FILE)


class H(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def do_GET(self):
        # Cheap local health check; never contacts upstream.
        if self.path.rstrip("/") == "/healthz":
            body = json.dumps({"ok": True, "key_injection": UPSTREAM_KEY is not None}).encode()
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
        else:
            self._fwd()

    def log_message(self, *a):  # never log headers; minimal path-only log
        pass

    def _fwd(self):
        n = int(self.headers.get("Content-Length") or 0)
        body = self.rfile.read(n) if n else None
        if body:
            try:
                body = json.dumps(rewrite_request(json.loads(body))).encode()
            except ValueError:
                pass
        u = urllib.parse.urlsplit(UPSTREAM)
        path = self.path if not self.path.startswith(u.path) else self.path[len(u.path):]
        conn = http.client.HTTPSConnection(u.hostname, u.port or 443, timeout=300)
        hdrs = {"Content-Type": self.headers.get("Content-Type", "application/json"), "Accept-Encoding": "identity"}
        if UPSTREAM_KEY is not None:
            hdrs["Authorization"] = "Bearer " + UPSTREAM_KEY
        elif self.headers.get("Authorization"):
            hdrs["Authorization"] = self.headers["Authorization"]
        if self.headers.get("Accept"):
            hdrs["Accept"] = self.headers["Accept"]
        PACER.wait()
        conn.request(self.command, u.path + path, body=body, headers=hdrs)
        r = conn.getresponse()
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
            for out in sse_rewrite(r, rw):
                self.wfile.write(out)
                self.wfile.flush()
            self.close_connection = True
            return
        data = r.read()
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

    do_POST = _fwd


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
    a = ap.parse_args()
    port = a.port
    PACER = Pacer(a.min_interval_ms)
    HOLD_FILE = a.hold_file
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
