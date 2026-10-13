import json, os, tempfile, unittest
from time_split import harness_parts, fx_tool_ms, split, Refused

EV = [{"type": "model_request", "ts_ms": 0}, {"type": "model_response", "ts_ms": 5000},
      {"type": "tool_call", "id": "a", "name": "read_file", "ts_ms": 5000},
      {"type": "tool_result", "id": "a", "ts_ms": 5010},
      {"type": "model_request", "ts_ms": 5020}, {"type": "model_response", "ts_ms": 11020},
      {"type": "tool_call", "id": "b", "name": "apply_patch", "ts_ms": 11020},
      {"type": "tool_result", "id": "b", "ts_ms": 11030}]

class T(unittest.TestCase):
    def test_harness(self):
        self.assertEqual(harness_parts(EV), (11000, 20, 2, 2))
    def test_fx_tool(self):
        self.assertEqual(fx_tool_ms({"tool_calls": [{"command_result": {"duration_ms": 7}}, {"name": "read_file"}]}), 7)
    def test_split_harness(self):
        d = tempfile.mkdtemp(); os.mkdir(os.path.join(d, "rec"))
        json.dump({"agent": "harness", "wall_ms": 12000, "retry_wait_ms": 0}, open(os.path.join(d, "summary.json"), "w"))
        json.dump({"hold_ms": 8000, "upstream_requests": 2}, open(os.path.join(d, "proxy_hold.json"), "w"))
        open(os.path.join(d, "proxy_usage.jsonl"), "w").write('{"output_tokens": 10}\n{"output_tokens": 20}\n')
        open(os.path.join(d, "rec", "run.jsonl"), "w").write("\n".join(json.dumps(e) for e in EV))
        r = split(d)
        self.assertEqual((r["model_ms"], r["tool_ms"], r["overhead_ms"], r["out_tok_per_req"]), (3000, 20, 980, 15.0))

    def _marked(self, name, obj):
        d = tempfile.mkdtemp()
        json.dump(obj, open(os.path.join(d, name), "w"))
        with self.assertRaises(Refused):
            split(d)
    def test_refuse_fake(self):
        self._marked("run_env.json", {"fake": True})
    def test_refuse_killed_file(self):
        self._marked("killed.json", {"status": "killed"})
    def test_refuse_killed_env(self):
        self._marked("run_env.json", {"status": "killed"})

if __name__ == "__main__":
    unittest.main()
