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


if __name__ == "__main__":
    unittest.main()
