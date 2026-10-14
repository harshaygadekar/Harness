"""Unit tests with made-up sample chunks (no real content, no secrets). Run: python3 -m unittest test_proxy"""
import json, unittest, proxy

SIG = {"google": {"thought_signature": "FAKE_SIG_1"}}
RAW = [
    {"choices": [{"delta": {"role": "assistant", "tool_calls": [{"extra_content": SIG, "function": {"arguments": "{\"path\":\"a.rs\"}", "name": "read_file"}, "id": "call_1", "type": "function"}]}, "index": 0}], "usage": {"total_tokens": 10}},
    {"choices": [{"delta": {"role": "assistant", "tool_calls": [{"function": {"arguments": "{\"path\":\"b.rs\"}", "name": "read_file"}, "id": "call_2", "type": "function"}]}, "index": 0}], "usage": {"total_tokens": 20}},
    {"choices": [{"delta": {"extra_content": SIG}, "index": 0}], "usage": {"total_tokens": 30}},
    {"choices": [{"delta": {"role": "assistant"}, "finish_reason": "tool_calls", "index": 0}], "usage": {"total_tokens": 40}},
]


def lines():
    return [("data: " + json.dumps(c) + "\n").encode() for c in RAW] + [b"\n", b"data: [DONE]\n"]


class T(unittest.TestCase):
    def setUp(self):
        proxy._SIGS.clear()
        out = b"".join(proxy.sse_rewrite(lines(), proxy.StreamRewriter())).decode()
        self.frames = [f[6:] for f in out.split("\n\n") if f]

    def test_done_passthrough_and_drop(self):
        self.assertEqual(self.frames[-1], "[DONE]")
        self.assertEqual(len(self.frames), 4)  # extra_content-only chunk dropped

    def test_index_added_stable(self):
        objs = [json.loads(f) for f in self.frames[:-1]]
        self.assertEqual(objs[0]["choices"][0]["delta"]["tool_calls"][0]["index"], 0)
        self.assertEqual(objs[1]["choices"][0]["delta"]["tool_calls"][0]["index"], 1)
        rw = proxy.StreamRewriter()
        a = rw.chunk(json.loads(json.dumps(RAW[0])))
        b = rw.chunk({"choices": [{"delta": {"tool_calls": [{"id": "call_1", "function": {"arguments": "x"}}]}, "index": 0}]})
        c = rw.chunk({"choices": [{"delta": {"tool_calls": [{"function": {"arguments": "y"}}]}, "index": 0}]})
        self.assertEqual(b["choices"][0]["delta"]["tool_calls"][0]["index"], 0)
        self.assertEqual(c["choices"][0]["delta"]["tool_calls"][0]["index"], 0)

    def test_no_extra_content_and_usage_last_only(self):
        self.assertNotIn("extra_content", "".join(self.frames))
        objs = [json.loads(f) for f in self.frames[:-1]]
        self.assertEqual([("usage" in o) for o in objs], [False, False, True])
        self.assertEqual(objs[-1]["usage"]["total_tokens"], 40)

    def test_signature_reinjected(self):
        req = {"messages": [{"role": "assistant", "tool_calls": [{"id": "call_1", "type": "function", "function": {"name": "read_file", "arguments": "{}"}}]}]}
        proxy.rewrite_request(req)
        self.assertEqual(req["messages"][0]["tool_calls"][0]["extra_content"], SIG)

    def test_finish_reason_stop_after_tools(self):
        rw = proxy.StreamRewriter()
        rw.chunk(json.loads(json.dumps(RAW[0])))
        f = rw.chunk({"choices": [{"delta": {"role": "assistant"}, "finish_reason": "stop", "index": 0}]})
        self.assertEqual(f["choices"][0]["finish_reason"], "tool_calls")
        f2 = proxy.StreamRewriter().chunk({"choices": [{"delta": {"content": "hi"}, "finish_reason": "stop", "index": 0}]})
        self.assertEqual(f2["choices"][0]["finish_reason"], "stop")

    def test_nonstream(self):
        o = proxy.rewrite_nonstream({"choices": [{"message": {"tool_calls": [{"id": "z", "extra_content": SIG}]}}]})
        self.assertNotIn("extra_content", json.dumps(o))

    def test_binds_loopback_only(self):
        self.assertEqual(proxy.BIND_HOST, "127.0.0.1")
        srv = proxy.make_server(0)
        try:
            self.assertEqual(srv.server_address[0], "127.0.0.1")
        finally:
            srv.server_close()

    def test_no_header_logging(self):
        src = open(proxy.__file__).read()
        self.assertNotIn("print(", src)
        self.assertNotIn("self.headers)", src)



class TimingFieldsTest(unittest.TestCase):
    def test_timing_fields(self):
        rec = proxy.usage_record(200, {"prompt_tokens": 3}, False, 1000, 1250)
        self.assertEqual((rec["start_ms"], rec["end_ms"], rec["dur_ms"]), (1000, 1250, 250))
        rec = proxy.usage_record(200, None, False)
        self.assertIsNone(rec["dur_ms"])


if __name__ == "__main__":
    unittest.main()


class Pacing(unittest.TestCase):
    def _fake(self):
        st = {"t": 100.0, "slept": []}
        def sleep(s):
            st["slept"].append(s); st["t"] += s
        return st, (lambda: st["t"]), sleep

    def test_default_zero_never_holds(self):
        st, clock, sleep = self._fake()
        p = proxy.Pacer(0, clock, sleep)
        for _ in range(3):
            self.assertEqual(p.wait(), 0.0)
        self.assertEqual(st["slept"], [])
        self.assertEqual(p.snapshot()["hold_ms"], 0)

    def test_min_interval_holds_and_counts(self):
        st, clock, sleep = self._fake()
        p = proxy.Pacer(5000, clock, sleep)
        p.wait()                      # first: no hold
        st["t"] += 2.0
        self.assertAlmostEqual(p.wait(), 3000.0)  # held 3 s
        st["t"] += 6.0
        self.assertEqual(p.wait(), 0.0)           # already past interval
        p.note_status(429); p.note_status(200)
        snap = p.snapshot()
        self.assertEqual(snap, {"min_interval_ms": 5000, "upstream_requests": 3, "held_requests": 1,
                                "hold_ms": 3000, "status_429": 1})
        p.reset()
        self.assertEqual(p.snapshot()["upstream_requests"], 0)
        self.assertEqual(p.wait(), 0.0)  # reset clears last start too


class KeyFileTests(__import__("unittest").TestCase):
    def test_load_key_file_reads_named_key(self):
        import tempfile, os
        import proxy
        with tempfile.NamedTemporaryFile("w", delete=False) as f:
            f.write("OTHER=x\nGEMINI_API_KEY=test-not-a-real-key\n")
        try:
            self.assertEqual(proxy.load_key_file(f.name, "GEMINI_API_KEY"), "test-not-a-real-key")
            with self.assertRaises(SystemExit):
                proxy.load_key_file(f.name, "MISSING_KEY")
        finally:
            os.unlink(f.name)


class AllowListAndAuthTests(unittest.TestCase):
    """Local mock upstream; no network, fake key only."""

    @classmethod
    def setUpClass(cls):
        import threading
        from http.server import ThreadingHTTPServer, BaseHTTPRequestHandler
        cls.seen = []

        class Up(BaseHTTPRequestHandler):
            protocol_version = "HTTP/1.1"
            def log_message(self, *a):
                pass
            def _any(self):
                n = int(self.headers.get("Content-Length") or 0)
                if n:
                    self.rfile.read(n)
                cls.seen.append((self.command, self.path, self.headers.get("Authorization")))
                body = b'{"choices":[{"message":{"role":"assistant","content":"SECRET_BODY_TEXT"},"finish_reason":"stop","index":0}],"usage":{"prompt_tokens":11,"completion_tokens":3,"total_tokens":14}}'
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                self.wfile.write(body)
            do_GET = do_POST = do_PUT = do_DELETE = _any

        cls.up = ThreadingHTTPServer(("127.0.0.1", 0), Up)
        threading.Thread(target=cls.up.serve_forever, daemon=True).start()
        cls.old = (proxy.UPSTREAM, proxy.UPSTREAM_KEY)
        proxy.UPSTREAM = "http://127.0.0.1:%d/v1beta/openai" % cls.up.server_address[1]
        proxy.UPSTREAM_KEY = "fake-test-key-123"
        import tempfile
        cls.usage_path = tempfile.mktemp(suffix=".jsonl")
        proxy.USAGE_FILE = cls.usage_path
        cls.runid_path = tempfile.mktemp()
        open(cls.runid_path, "w").write("T7-run-A\n")
        proxy.RUN_ID_FILE = cls.runid_path
        cls.px = proxy.make_server(0)
        threading.Thread(target=cls.px.serve_forever, daemon=True).start()
        cls.port = cls.px.server_address[1]

    @classmethod
    def tearDownClass(cls):
        cls.px.shutdown(); cls.up.shutdown()
        proxy.UPSTREAM, proxy.UPSTREAM_KEY = cls.old

    def req(self, method, path, body=None, auth=None):
        import http.client
        c = http.client.HTTPConnection("127.0.0.1", self.port, timeout=10)
        h = {"Content-Type": "application/json"}
        if auth:
            h["Authorization"] = auth
        c.request(method, path, body=body, headers=h)
        r = c.getresponse(); r.read(); c.close()
        return r.status

    def test_transport_error_logs_times_only(self):
        import socket
        sk = socket.socket(); sk.bind(("127.0.0.1", 0)); dead = sk.getsockname()[1]; sk.close()
        saved = proxy.UPSTREAM
        proxy.UPSTREAM = "http://127.0.0.1:%d/v1beta/openai" % dead
        try:
            open(self.usage_path, "w").close()
            st = self.req("POST", "/chat/completions", body=b'{"model":"m","messages":[],"x":"MARKER_ERR_BODY"}', auth="Bearer MARKER_ERR_AUTH")
        finally:
            proxy.UPSTREAM = saved
        self.assertEqual(st, 502)
        rows = [json.loads(l) for l in open(self.usage_path)]
        self.assertEqual(len(rows), 1)
        self.assertEqual(set(rows[0]), {"status", "start_ms", "end_ms", "dur_ms", "exc_type", "phase"})
        self.assertEqual(rows[0]["status"], "error")
        self.assertEqual(rows[0]["exc_type"], "ConnectionRefusedError")
        self.assertEqual(rows[0]["phase"], "connect")
        self.assertEqual(rows[0]["dur_ms"], rows[0]["end_ms"] - rows[0]["start_ms"])
        raw = open(self.usage_path).read()
        for m in ("MARKER_ERR_BODY", "MARKER_ERR_AUTH", "fake-test-key-123", "Errno", "refused"):
            self.assertNotIn(m, raw)

    def _raw_upstream(self, reply):
        """One-shot raw TCP upstream: read the request, send `reply` bytes, then close."""
        import socket, threading
        srv = socket.socket(); srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        srv.bind(("127.0.0.1", 0)); srv.listen(1); port = srv.getsockname()[1]
        def run():
            c, _ = srv.accept(); c.settimeout(5)
            buf = b""
            try:
                while b"\r\n\r\n" not in buf:
                    buf += c.recv(65536)
                h, _, rest = buf.partition(b"\r\n\r\n")
                n = int([l.split(b":")[1] for l in h.split(b"\r\n") if l.lower().startswith(b"content-length")][0])
                while len(rest) < n:
                    rest += c.recv(65536)
                if reply:
                    c.sendall(reply)
            finally:
                c.close(); srv.close()
        threading.Thread(target=run, daemon=True).start()
        return port

    def _error_row_via(self, port, marker):
        saved = proxy.UPSTREAM
        proxy.UPSTREAM = "http://127.0.0.1:%d/v1beta/openai" % port
        try:
            open(self.usage_path, "w").close()
            try:
                self.req("POST", "/chat/completions", body=('{"model":"m","messages":[],"x":"%s_BODY"}' % marker).encode(), auth="Bearer %s_AUTH" % marker)
            except Exception:
                pass
        finally:
            proxy.UPSTREAM = saved
        rows = [json.loads(l) for l in open(self.usage_path)]
        raw = open(self.usage_path).read()
        for m in (marker + "_BODY", marker + "_AUTH", "fake-test-key-123", "127.0.0.1", "http://", "/v1beta", "closed", "Remote end"):
            self.assertNotIn(m, raw)
        return rows

    def test_transport_error_phase_first_byte(self):
        rows = self._error_row_via(self._raw_upstream(b""), "MARKER_FB")
        self.assertEqual(len(rows), 1)
        self.assertEqual(set(rows[0]), {"status", "start_ms", "end_ms", "dur_ms", "exc_type", "phase", "af"})
        self.assertEqual((rows[0]["status"], rows[0]["phase"]), ("error", "read"))
        self.assertEqual(rows[0]["exc_type"], "http.client.RemoteDisconnected")
        self.assertEqual(rows[0]["af"], "IPv4")

    def test_transport_error_phase_mid_stream(self):
        reply = (b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\n\r\n"
                 b"40\r\ndata: {\"choices\":[{\"index\":0,\"delta\":{\"content\":\"hi\"}}]}\n\n")
        rows = self._error_row_via(self._raw_upstream(reply), "MARKER_MS")
        self.assertEqual(len(rows), 1)
        self.assertEqual(set(rows[0]), {"status", "start_ms", "end_ms", "dur_ms", "exc_type", "phase", "af"})
        self.assertEqual((rows[0]["status"], rows[0]["phase"]), ("error", "read"))
        self.assertTrue(rows[0]["exc_type"])
        self.assertEqual(rows[0]["af"], "IPv4")

    def test_exc_type_name_only(self):
        import ssl
        self.assertEqual(proxy.exc_type_name(TimeoutError("SECRET_TEXT")), "TimeoutError")
        self.assertEqual(proxy.exc_type_name(ssl.SSLError("SECRET_TEXT")), "ssl.SSLError")
        self.assertNotIn("SECRET", proxy.exc_type_name(ConnectionResetError("SECRET_TEXT")))

    def test_no_run_gate_returns_503_and_logs_times_only(self):
        self.seen.clear()
        for content in ("", "  \n"):
            open(self.runid_path, "w").write(content)
            open(self.usage_path, "w").close()
            try:
                st = self.req("POST", "/chat/completions", body=b'{"model":"m","messages":[],"x":"MARKER_NORUN_BODY"}', auth="Bearer MARKER_NORUN_AUTH")
            finally:
                open(self.runid_path, "w").write("T7-run-A\n")
            self.assertEqual(st, 503)
            self.assertEqual(self.seen, [])  # never forwarded upstream
            rows = [json.loads(l) for l in open(self.usage_path)]
            self.assertEqual(len(rows), 1)
            self.assertEqual(set(rows[0]), {"status", "start_ms", "end_ms", "dur_ms"})
            self.assertEqual(rows[0]["status"], "no_run")
            raw = open(self.usage_path).read()
            for m in ("MARKER_NORUN_BODY", "MARKER_NORUN_AUTH", "fake-test-key-123"):
                self.assertNotIn(m, raw)
        # Missing file is also "no run".
        saved = proxy.RUN_ID_FILE
        proxy.RUN_ID_FILE = self.runid_path + ".missing"
        try:
            self.assertEqual(self.req("POST", "/chat/completions", body=b"{}"), 503)
        finally:
            proxy.RUN_ID_FILE = saved
        self.assertEqual(self.seen, [])
        # With a run id set again, requests forward normally.
        self.assertEqual(self.req("POST", "/chat/completions", body=b'{"model":"m","messages":[]}'), 200)

    def test_other_paths_404_without_upstream_call(self):
        self.seen.clear()
        for m, p in [("GET", "/models"), ("GET", "/v1beta/models"), ("POST", "/embeddings"),
                     ("POST", "/chat/completions/../models"), ("PUT", "/chat/completions"),
                     ("DELETE", "/chat/completions"), ("GET", "/chat/completions")]:
            self.assertEqual(self.req(m, p, body=b"{}" if m != "GET" else None), 404, (m, p))
        self.assertEqual(self.seen, [])
        self.assertEqual(self.req("GET", "/healthz"), 200)
        self.assertEqual(self.seen, [])

    def test_client_auth_never_reaches_upstream(self):
        self.seen.clear()
        body = json.dumps({"model": "m", "messages": [{"role": "user", "content": "hi"}]}).encode()
        self.assertEqual(self.req("POST", "/chat/completions", body=body, auth="Bearer client-secret-should-not-pass"), 200)
        self.assertEqual(len(self.seen), 1)
        m, p, a = self.seen[0]
        self.assertEqual((m, p), ("POST", "/v1beta/openai/chat/completions"))
        self.assertEqual(a, "Bearer fake-test-key-123")


    def test_usage_log_counts_only(self):
        import http.client
        open(self.usage_path, "w").close()
        body = json.dumps({"model": "m", "messages": [{"role": "user", "content": "BODY_MARKER_7f3a"}]}).encode()
        c = http.client.HTTPConnection("127.0.0.1", self.port, timeout=10)
        c.request("POST", "/chat/completions", body=body, headers={
            "Content-Type": "application/json", "Authorization": "Bearer FAKE_AUTH_MARKER_91c2",
            "X-Test-Marker": "HEADER_MARKER_c0de", "X-Run-Id": "client-run-B"})
        r = c.getresponse(); r.read(); c.close()
        self.assertEqual(r.status, 200)
        self.assertEqual(self.req("GET", "/models"), 404)  # blocked locally: not forwarded, not logged
        rows = [json.loads(l) for l in open(self.usage_path)]
        self.assertEqual(len(rows), 1)
        self.assertEqual(set(rows[0]), {"ts_ms", "run_id", "status", "input_tokens", "output_tokens", "start_ms", "end_ms", "dur_ms"})
        self.assertEqual(rows[0]["run_id"], "T7-run-A")  # from the operator file, not X-Run-Id
        self.assertEqual((rows[0]["status"], rows[0]["input_tokens"], rows[0]["output_tokens"]), (200, 11, 3))
        raw = open(self.usage_path).read()
        for bad in ("BODY_MARKER_7f3a", "FAKE_AUTH_MARKER_91c2", "HEADER_MARKER_c0de", "SECRET_BODY_TEXT",
                    "fake-test-key", "Authorization", "Bearer", "messages", "client-run-B"):
            self.assertNotIn(bad, raw)

    def test_stream_usage_record_takes_final_usage(self):
        rw = proxy.StreamRewriter()
        list(proxy.sse_rewrite(lines(), rw))
        rec = proxy.usage_record(200, rw.last_usage, True)
        self.assertEqual(set(rec), {"ts_ms", "run_id", "status", "input_tokens", "output_tokens", "start_ms", "end_ms", "dur_ms"})


    def test_run_id_file_reread_and_header_ignored(self):
        import http.client
        open(self.usage_path, "w").close()
        body = json.dumps({"model": "m", "messages": []}).encode()
        for rid in ("T7-run-A", "T7-run-C"):
            open(self.runid_path, "w").write(rid)
            c = http.client.HTTPConnection("127.0.0.1", self.port, timeout=10)
            c.request("POST", "/chat/completions", body=body, headers={"Content-Type": "application/json", "X-Run-Id": "client-run-B"})
            c.getresponse().read(); c.close()
        rows = [json.loads(l) for l in open(self.usage_path)]
        self.assertEqual([r["run_id"] for r in rows], ["T7-run-A", "T7-run-C"])
        open(self.runid_path, "w").write("T7-run-A\n")

