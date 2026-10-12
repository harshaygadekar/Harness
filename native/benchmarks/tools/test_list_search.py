import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import list_search as S  # noqa: E402


class GlobTests(unittest.TestCase):
    def test_targets(self):
        self.assertEqual(S.glob_target("*"), ("", False))
        self.assertEqual(S.glob_target("**/*.rs"), ("", True))
        self.assertEqual(S.glob_target("src/**/*.rs"), ("src", True))
        self.assertEqual(S.glob_target("src/billing/proration/*"), ("src/billing/proration", False))
        self.assertEqual(S.glob_target("src/billing"), ("src/billing", False))

    def test_norm_dir(self):
        self.assertEqual(S.norm_dir("./src/"), "src")
        self.assertEqual(S.norm_dir("."), "")
        self.assertEqual(S.norm_dir(None), "")


class ListTests(unittest.TestCase):
    def test_drilldown_chain(self):
        s = S.list_stats([("", False), ("src", False), ("tests", False),
                          ("src/billing", False), ("src/billing/proration", False), ("src/util", False)])
        self.assertEqual(s, {"list_calls": 6, "list_root": 1, "list_drilldown": 5, "list_recursive": 0})

    def test_jump_is_not_drilldown(self):
        s = S.list_stats([("", False), ("src/billing", False)])
        self.assertEqual(s["list_drilldown"], 0)

    def test_recursive_not_drilldown(self):
        s = S.list_stats([("", False), ("src", True)])
        self.assertEqual(s["list_drilldown"], 0)
        self.assertEqual(s["list_recursive"], 1)


class SearchTests(unittest.TestCase):
    def k(self, p, d=""):
        return {"kind": "search", "pattern": p, "path": d}

    def test_narrower_pattern_within_two(self):
        seq = [self.k("Policy"), {"kind": "other"}, self.k("Policy::default")]
        self.assertEqual(S.search_follow(seq), 1)

    def test_too_far(self):
        seq = [self.k("Policy"), {"kind": "other"}, {"kind": "other"}, self.k("Policy::default")]
        self.assertEqual(S.search_follow(seq), 0)

    def test_deeper_path(self):
        self.assertEqual(S.search_follow([self.k("x"), self.k("x", "src")]), 1)
        self.assertEqual(S.search_follow([self.k("x", "src"), self.k("x", "tests")]), 0)

    def test_same_or_broader_not_narrower(self):
        self.assertEqual(S.search_follow([self.k("Policy"), self.k("Policy")]), 0)
        self.assertEqual(S.search_follow([self.k("Policy::default"), self.k("Policy")]), 0)

    def test_cap(self):
        self.assertTrue(S.search_capped("\n".join("a.rs:%d:x" % i for i in range(50))))
        self.assertFalse(S.search_capped("a.rs:1:x\nb.rs:2:y"))
        self.assertFalse(S.search_capped("(no hits)"))
        self.assertTrue(S.search_capped("a\n…[truncated; full length 20000 bytes]"))
        self.assertIsNone(S.search_capped(None))


if __name__ == "__main__":
    unittest.main()
