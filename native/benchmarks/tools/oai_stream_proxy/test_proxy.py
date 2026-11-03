import time
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
        self.assertEqual(set(rows[0]), {"status", "start_ms", "end_ms", "dur_ms", "exc_type", "phase", "sig_put_back", "sig_put_back_count"})
        self.assertEqual(rows[0]["status"], "error")
        self.assertEqual(rows[0]["exc_type"], "ConnectionRefusedError")
        self.assertEqual(rows[0]["phase"], "connect")
        self.assertEqual(rows[0]["dur_ms"], rows[0]["end_ms"] - rows[0]["start_ms"])
        raw = open(self.usage_path).read()
        for m in ("MARKER_ERR_BODY", "MARKER_ERR_AUTH", "fake-test-key-123", "Errno", "refused"):
            self.assertNotIn(m, raw)

    def _raw_upstream(self, reply, more=None):
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
                for part in (more or []):
                    time.sleep(0.05)
                    try:
                        c.sendall(part)
                    except OSError:
                        break
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
        self.assertEqual(set(rows[0]), {"status", "start_ms", "end_ms", "dur_ms", "exc_type", "phase", "af", "sig_put_back", "sig_put_back_count"})
        self.assertEqual((rows[0]["status"], rows[0]["phase"]), ("error", "read"))
        self.assertEqual(rows[0]["exc_type"], "http.client.RemoteDisconnected")
        self.assertEqual(rows[0]["af"], "IPv4")

    def test_transport_error_phase_mid_stream(self):
        reply = (b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\n\r\n"
                 b"40\r\ndata: {\"choices\":[{\"index\":0,\"delta\":{\"content\":\"hi\"}}]}\n\n")
        rows = self._error_row_via(self._raw_upstream(reply), "MARKER_MS")
        self.assertEqual(len(rows), 1)
        self.assertEqual(set(rows[0]), {"status", "start_ms", "end_ms", "dur_ms", "exc_type", "phase", "af", "sig_put_back", "sig_put_back_count"})
        self.assertEqual((rows[0]["status"], rows[0]["phase"]), ("error", "read"))
        self.assertTrue(rows[0]["exc_type"])
        self.assertEqual(rows[0]["af"], "IPv4")

    def test_client_side_write_error_is_not_transport_error(self):
        import socket
        chunk = b'data: {"choices":[{"index":0,"delta":{"content":"%s"}}]}\n\n' % (b"x" * 2000)
        hexlen = b"%x" % len(chunk)
        head = b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\n\r\n"
        more = [hexlen + b"\r\n" + chunk + b"\r\n"] * 200 + [b"0\r\n\r\n"]
        port = self._raw_upstream(head + more[0], more[1:])
        saved = proxy.UPSTREAM
        proxy.UPSTREAM = "http://127.0.0.1:%d/v1beta/openai" % port
        try:
            open(self.usage_path, "w").close()
            c = socket.create_connection(("127.0.0.1", self.port))
            body = b'{"model":"m","messages":[],"stream":true,"x":"MARKER_CL_BODY"}'
            c.sendall(b"POST /chat/completions HTTP/1.1\r\nHost: x\r\nContent-Type: application/json\r\nContent-Length: %d\r\n\r\n" % len(body) + body)
            c.recv(100)
            c.setsockopt(socket.SOL_SOCKET, socket.SO_LINGER, b"\x01\x00\x00\x00\x00\x00\x00\x00")
            c.close()  # agent side goes away mid-stream
            for _ in range(100):
                if open(self.usage_path).read().strip():
                    break
                time.sleep(0.1)
        finally:
            proxy.UPSTREAM = saved
        rows = [json.loads(l) for l in open(self.usage_path)]
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]["side"], "client")
        self.assertEqual(rows[0]["status"], "error")
        self.assertNotIn("phase", rows[0]); self.assertNotIn("af", rows[0])
        self.assertIn(rows[0]["exc_type"], ("BrokenPipeError", "ConnectionResetError"))
        self.assertNotIn("MARKER_CL_BODY", open(self.usage_path).read())

    def test_upstream_read_error_has_no_client_side(self):
        reply = (b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\n\r\n"
                 b"40\r\ndata: {\"choices\":[{\"index\":0,\"delta\":{\"content\":\"hi\"}}]}\n\n")
        rows = self._error_row_via(self._raw_upstream(reply), "MARKER_UR")
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]["phase"], "read")
        self.assertNotIn("side", rows[0])

    def test_finish_label_allow_list(self):
        for v in ("stop", "length", "tool_calls", "content_filter", "safety", "recitation", "other", "missing", "STOP"):
            self.assertIn(proxy.finish_label(v), proxy.FINISH_LABELS)
        self.assertEqual(proxy.finish_label("STOP"), "stop")
        self.assertEqual(proxy.finish_label(None), "missing")
        self.assertEqual(proxy.finish_label(""), "missing")
        self.assertEqual(proxy.finish_label("SECRET_FREE_TEXT_x"), "other")
        self.assertEqual(proxy.finish_label({"x": 1}), "other")
        self.assertEqual(proxy.finish_label(7), "other")

    def test_finish_raw_case_insensitive(self):
        for v in ("MAX_TOKENS", "max_tokens", "Max_Tokens"):
            self.assertEqual(proxy.finish_raw(v), "MAX_TOKENS")
        for w in ("OTHER", "MALFORMED_FUNCTION_CALL", "UNEXPECTED_TOOL_CALL", "SAFETY", "RECITATION",
                  "BLOCKLIST", "PROHIBITED_CONTENT", "SPII"):
            self.assertEqual(proxy.finish_raw(w.lower()), w)
            self.assertEqual(proxy.finish_raw(w.title()), w)
        for w in ("stop", "length", "tool_calls", "content_filter"):
            self.assertEqual(proxy.finish_raw(w.upper()), w)

    def test_finish_raw_upper_stop_logs_lower(self):
        self.assertEqual(proxy.finish_raw("STOP"), "stop")
        self.assertEqual(proxy.usage_record(200, {}, False, finish="STOP")["finish_reason_raw"], "stop")

    def test_finish_raw_lower_other_logs_upper(self):
        self.assertEqual(proxy.finish_raw("other"), "OTHER")
        self.assertEqual(proxy.usage_record(200, {}, False, finish="other")["finish_reason_raw"], "OTHER")

    def test_finish_raw_unknown_is_unlisted_not_other(self):
        self.assertEqual(proxy.finish_raw("FINISH_REASON_UNSPECIFIED"), "unlisted")
        self.assertEqual(proxy.finish_raw(7), "unlisted")
        self.assertEqual(proxy.finish_raw({"x": 1}), "unlisted")

    def test_finish_raw_missing_is_null(self):
        self.assertIsNone(proxy.finish_raw(None))
        self.assertIsNone(proxy.finish_raw(""))
        self.assertIsNone(proxy.usage_record(200, {}, False)["finish_reason_raw"])
        self.assertNotIn("finish_reason_raw", proxy.usage_record(503, {}, False, finish="stop"))

    def test_finish_raw_message_like_text_not_logged(self):
        msg = "SECRET_MSG_TEXT: stopped because MAX_TOKENS was hit"
        r = proxy.usage_record(200, {}, False, finish=msg)
        self.assertEqual(r["finish_reason_raw"], "unlisted")
        self.assertNotIn("SECRET_MSG_TEXT", json.dumps(r))
        # value only from the finish_reason field, never from message content
        obj = {"choices": [{"message": {"content": "MAX_TOKENS"}, "finish_reason": None}]}
        self.assertIsNone(proxy.finish_raw(proxy.nonstream_finish(obj)))
        rw = proxy.StreamRewriter()
        rw.chunk({"choices": [{"index": 0, "delta": {"content": "SAFETY"}}]})
        rw.chunk({"choices": [{"index": 0, "delta": {}, "finish_reason": "max_tokens"}]})
        self.assertEqual(proxy.finish_raw(rw.upstream_finish), "MAX_TOKENS")

    def test_finish_reason_on_200_rows_only_and_label_only(self):
        r = proxy.usage_record(200, {}, False, finish="SECRET_FREE_TEXT_y")
        self.assertEqual(r["finish_reason"], "other")
        self.assertNotIn("SECRET", json.dumps(r))
        self.assertNotIn("finish_reason", proxy.usage_record(503, {}, False, finish="stop"))
        self.assertEqual(proxy.usage_record(200, {}, False)["finish_reason"], "missing")

    def test_stream_finish_is_upstream_value_before_rewrite(self):
        # upstream says "stop" after a tool call; the agent gets "tool_calls", but the log keeps the upstream "stop".
        rw = proxy.StreamRewriter()
        rw.chunk({"choices": [{"index": 0, "delta": {"tool_calls": [{"id": "a", "function": {"name": "f", "arguments": "{}"}}]}}]})
        out = rw.chunk({"choices": [{"index": 0, "delta": {}, "finish_reason": "stop"}]})
        self.assertEqual(out["choices"][0]["finish_reason"], "tool_calls")
        self.assertEqual(proxy.finish_label(rw.upstream_finish), "stop")

    def test_nonstream_finish_logged_end_to_end(self):
        open(self.usage_path, "w").close()
        st = self.req("POST", "/chat/completions", body=b'{"model":"m","messages":[]}')
        rows = [json.loads(l) for l in open(self.usage_path)]
        self.assertEqual(st, 200)
        self.assertIn(rows[-1]["finish_reason"], proxy.FINISH_LABELS)

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
        self.assertEqual(set(rows[0]), {"ts_ms", "run_id", "status", "input_tokens", "output_tokens", "start_ms", "end_ms", "dur_ms", "finish_reason", "finish_reason_raw",
                                    "sig_in_reply", "sig_put_back", "sig_put_back_count", "reply_model"})
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
        self.assertEqual(set(rec), {"ts_ms", "run_id", "status", "input_tokens", "output_tokens", "start_ms", "end_ms", "dur_ms", "finish_reason", "finish_reason_raw",
                                    "sig_in_reply", "sig_put_back", "sig_put_back_count", "reply_model"})


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


class SigFlagTests(unittest.TestCase):
    """sig_in_reply / sig_put_back / reply_model flags. Local mock upstream, fake values only."""
    SIGVAL = "FAKE_SIGVALUE_Zq81"
    TCID = "call_FAKEID_77k"

    @classmethod
    def setUpClass(cls):
        import threading, tempfile, os
        from http.server import ThreadingHTTPServer, BaseHTTPRequestHandler
        cls.reply = None  # (status, ctype, body bytes)
        cls.got = []

        class Up(BaseHTTPRequestHandler):
            protocol_version = "HTTP/1.1"
            def log_message(self, *a):
                pass
            def do_POST(self):
                n = int(self.headers.get("Content-Length") or 0)
                cls.got.append(self.rfile.read(n) if n else b"")
                st, ct, body = cls.reply
                self.send_response(st)
                self.send_header("Content-Type", ct)
                self.send_header("Content-Length", str(len(body)))
                if "event-stream" in ct:
                    self.send_header("Connection", "close")
                self.end_headers()
                self.wfile.write(body)

        cls.up = ThreadingHTTPServer(("127.0.0.1", 0), Up)
        threading.Thread(target=cls.up.serve_forever, daemon=True).start()
        cls.saved = (proxy.UPSTREAM, proxy.UPSTREAM_KEY, proxy.USAGE_FILE, proxy.RUN_ID_FILE, proxy.SIG_MODE)
        proxy.UPSTREAM = "http://127.0.0.1:%d/v1beta/openai" % cls.up.server_address[1]
        proxy.UPSTREAM_KEY = "fake-test-key-123"
        cls.dir = tempfile.mkdtemp()
        cls.usage_path = os.path.join(cls.dir, "proxy_usage.jsonl")
        proxy.USAGE_FILE = cls.usage_path
        proxy.RUN_ID_FILE = os.path.join(cls.dir, "run_id")
        open(proxy.RUN_ID_FILE, "w").write("SIG-run\n")
        cls.px = proxy.make_server(0)
        threading.Thread(target=cls.px.serve_forever, daemon=True).start()
        cls.port = cls.px.server_address[1]

    @classmethod
    def tearDownClass(cls):
        import shutil
        cls.px.shutdown(); cls.up.shutdown()
        cls.px.server_close(); cls.up.server_close()
        proxy.UPSTREAM, proxy.UPSTREAM_KEY, proxy.USAGE_FILE, proxy.RUN_ID_FILE, proxy.SIG_MODE = cls.saved
        shutil.rmtree(cls.dir, ignore_errors=True)

    def setUp(self):
        proxy._SIGS.clear()
        proxy.SIG_MODE = "reinject"
        open(self.usage_path, "w").close()

    def tc(self, sig=True):
        t = {"id": self.TCID, "type": "function", "function": {"name": "read_file", "arguments": "{}"}}
        if sig:
            t["extra_content"] = {"google": {"thought_signature": self.SIGVAL}}
        return t

    def nonstream(self, sig=True, model="gemini-2.5-flash", status=200):
        o = {"choices": [{"index": 0, "finish_reason": "stop", "message": {"role": "assistant", "tool_calls": [self.tc(sig)]}}],
             "usage": {"prompt_tokens": 1, "completion_tokens": 1}}
        if model is not None:
            o["model"] = model
        if status != 200:
            o = {"error": {"code": status, "status": "INVALID_ARGUMENT"}}
        return (status, "application/json", json.dumps(o).encode())

    def stream(self, sig=True, model="gemini-2.5-flash"):
        c1 = {"choices": [{"index": 0, "delta": {"role": "assistant", "tool_calls": [self.tc(sig)]}}]}
        c2 = {"choices": [{"index": 0, "delta": {}, "finish_reason": "stop"}], "usage": {"prompt_tokens": 1}}
        if model is not None:
            c1["model"] = model; c2["model"] = "second-chunk-model"
        b = "".join("data: %s\n\n" % json.dumps(c) for c in (c1, c2)) + "data: [DONE]\n\n"
        return (200, "text/event-stream", b.encode())

    def post(self, reply, with_prior_tool_call=False):
        import http.client
        type(self).reply = reply
        msgs = [{"role": "user", "content": "hi"}]
        if with_prior_tool_call:
            msgs.append({"role": "assistant", "tool_calls": [self.tc(sig=False)]})
        c = http.client.HTTPConnection("127.0.0.1", self.port, timeout=10)
        c.request("POST", "/chat/completions", body=json.dumps({"model": "m", "messages": msgs}).encode(),
                  headers={"Content-Type": "application/json"})
        r = c.getresponse(); r.read(); c.close()
        return r.status, [json.loads(l) for l in open(self.usage_path)][-1]

    def test_nonstream_with_sig(self):
        st, row = self.post(self.nonstream(sig=True))
        self.assertEqual((st, row["sig_in_reply"], row["sig_put_back"], row["sig_put_back_count"]), (200, True, False, 0))

    def test_nonstream_without_sig(self):
        st, row = self.post(self.nonstream(sig=False))
        self.assertIs(row["sig_in_reply"], False)

    def test_stream_with_sig_in_tool_call_delta(self):
        st, row = self.post(self.stream(sig=True))
        self.assertEqual((st, row["sig_in_reply"], row["reply_model"]), (200, True, "gemini-2.5-flash"))
        st, row = self.post(self.stream(sig=False))
        self.assertIs(row["sig_in_reply"], False)

    def test_put_back_on_later_request(self):
        self.post(self.nonstream(sig=True))
        st, row = self.post(self.nonstream(sig=False), with_prior_tool_call=True)
        self.assertEqual((row["sig_put_back"], row["sig_put_back_count"]), (True, 1))
        self.assertIn(self.SIGVAL.encode(), self.got[-1])  # behaviour unchanged: upstream gets it back

    def test_unknown_id_no_put_back(self):
        st, row = self.post(self.nonstream(sig=False), with_prior_tool_call=True)
        self.assertEqual((row["sig_put_back"], row["sig_put_back_count"]), (False, 0))

    def test_strip_mode_never_puts_back(self):
        proxy.SIG_MODE = "strip"
        _, r1 = self.post(self.nonstream(sig=True))
        self.assertIs(r1["sig_in_reply"], True)
        _, r2 = self.post(self.nonstream(sig=False), with_prior_tool_call=True)
        self.assertEqual((r2["sig_put_back"], r2["sig_put_back_count"]), (False, 0))
        self.assertNotIn(self.SIGVAL.encode(), self.got[-1])

    def test_non200_nulls_and_put_back_still_logged(self):
        self.post(self.nonstream(sig=True))
        st, row = self.post(self.nonstream(status=400), with_prior_tool_call=True)
        self.assertEqual(st, 400)
        self.assertIsNone(row["sig_in_reply"]); self.assertIsNone(row["reply_model"])
        self.assertIs(row["sig_put_back"], True)

    def test_reply_model_allow_unlisted_missing(self):
        self.assertEqual(self.post(self.nonstream(model="models/gemini-2.5-pro:x_1"))[1]["reply_model"], "models/gemini-2.5-pro:x_1")
        self.assertEqual(self.post(self.nonstream(model="bad model; text"))[1]["reply_model"], "unlisted")
        self.assertEqual(self.post(self.nonstream(model="a" * 65))[1]["reply_model"], "unlisted")
        self.assertEqual(self.post(self.nonstream(model=123))[1]["reply_model"], "unlisted")
        self.assertIsNone(self.post(self.nonstream(model=None))[1]["reply_model"])
        self.assertIsNone(self.post(self.stream(model=None))[1]["reply_model"])

    def test_per_request_ctx_not_shared(self):
        proxy.rewrite_request({"messages": [{"role": "assistant", "tool_calls": [{"id": "x1", "extra_content": {"k": 1}}]}]})
        proxy._SIGS["x2"] = {"k": 2}
        a, b = {}, {}
        proxy.rewrite_request({"messages": [{"role": "assistant", "tool_calls": [{"id": "x2"}]}]}, a)
        proxy.rewrite_request({"messages": [{"role": "assistant", "tool_calls": [{"id": "nope"}]}]}, b)
        self.assertEqual((a["put_back"], b["put_back"]), (1, 0))

    def test_no_signature_or_id_in_any_log(self):
        import io, sys
        old, buf = sys.stderr, io.StringIO()
        sys.stderr = buf
        try:
            self.post(self.nonstream(sig=True))
            self.post(self.stream(sig=True))
            self.post(self.nonstream(sig=False), with_prior_tool_call=True)
            self.post(self.nonstream(status=400), with_prior_tool_call=True)
        finally:
            sys.stderr = old
        rows = open(self.usage_path).read()
        self.assertEqual(len(rows.splitlines()), 4)
        for log in (rows, buf.getvalue()):
            for bad in (self.SIGVAL, self.TCID, "thought_signature", "extra_content", "FAKEID", "hi\""):
                self.assertNotIn(bad, log)

