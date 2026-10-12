import unittest
from token_overhead import request_sizes, split

EV = [
    {"type": "model_request"},
    {"type": "model_response", "content": None, "tool_calls": [{"name": "ls", "arguments": {}}]},
    {"type": "tool_result", "content": "x" * 98},
    {"type": "model_request"},
]

class T(unittest.TestCase):
    def test_sizes(self):
        self.assertEqual(request_sizes(EV, 100), [(100, 0), (100, 2 + 2 + 98)])
    def test_split(self):
        r = split(EV, 100, 76)  # chars = 200 + 102 = 302 -> k ~ 3.97
        self.assertEqual(r["requests"], 2)
        self.assertEqual(r["fixed_tokens_total"] + r["growth_tokens_total"], 76)
        self.assertEqual(r["largest_tool_result_chars"], 98)
    def test_zero_tokens(self):
        self.assertIsNone(split(EV, 100, 0)["chars_per_token_fit"])

class Carried(unittest.TestCase):
    def test_kinds(self):
        from token_overhead import carried_by_kind
        ev = [{"type": "model_request"},
              {"type": "tool_call", "id": "a", "name": "run_shell", "arguments": {"cmd": "cargo test"}},
              {"type": "tool_result", "id": "a", "content": "x" * 10},
              {"type": "tool_call", "id": "b", "name": "read_file", "arguments": {}},
              {"type": "tool_result", "id": "b", "content": "y" * 3},
              {"type": "model_request"}, {"type": "model_request"}]
        self.assertEqual(carried_by_kind(ev), (20, 6))

if __name__ == "__main__":
    unittest.main()
