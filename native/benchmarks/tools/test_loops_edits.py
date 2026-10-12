import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import loops_edits as E  # noqa: E402


def c(key, ok=True, cat="read", path=""):
    return {"key": key, "ok": ok, "category": cat, "path": path}


class LoopTests(unittest.TestCase):
    def test_no_repeats(self):
        s = E.loop_stats([c("a"), c("b"), c("c")])
        self.assertEqual(s, {"identical_repeats": 0, "max_consecutive_identical": 1,
                             "paper_reminders": 0, "paper_stop_call_index": None})

    def test_non_consecutive_repeat(self):
        s = E.loop_stats([c("a"), c("b"), c("a")])
        self.assertEqual(s["identical_repeats"], 1)
        self.assertEqual(s["max_consecutive_identical"], 1)

    def test_reminder_at_five_once_per_streak(self):
        s = E.loop_stats([c("a")] * 7)
        self.assertEqual(s["max_consecutive_identical"], 7)
        self.assertEqual(s["paper_reminders"], 1)
        self.assertIsNone(s["paper_stop_call_index"])

    def test_four_is_below_threshold(self):
        self.assertEqual(E.loop_stats([c("a")] * 4)["paper_reminders"], 0)

    def test_stop_at_eight_failing(self):
        s = E.loop_stats([c("x", ok=False)] * 9)
        self.assertEqual(s["paper_stop_call_index"], 7)

    def test_success_breaks_fail_streak(self):
        calls = [c("x", ok=False)] * 4 + [c("x", ok=True)] + [c("x", ok=False)] * 4
        s = E.loop_stats(calls)
        self.assertIsNone(s["paper_stop_call_index"])
        self.assertEqual(s["paper_reminders"], 1)  # 5 identical, any status

    def test_two_streaks_two_reminders(self):
        s = E.loop_stats([c("a")] * 5 + [c("b")] + [c("a")] * 5)
        self.assertEqual(s["paper_reminders"], 2)


class EditTests(unittest.TestCase):
    def test_repeat_edits(self):
        calls = [c("1", cat="edit", path="src/a.rs"), c("2", cat="edit", path="src/a.rs"),
                 c("3", cat="write", path="src/b.rs"), c("4", cat="edit", path="src/a.rs"),
                 c("5", cat="edit", path=""), c("6", cat="read", path="src/a.rs")]
        s = E.edit_stats(calls)
        self.assertEqual(s, {"edit_calls": 5, "repeat_edits": 2, "max_edits_one_file": 3,
                             "edits_path_unknown": 1, "files_edited": 2})

    def test_no_edits(self):
        self.assertEqual(E.edit_stats([c("a")])["repeat_edits"], 0)


class ModeTests(unittest.TestCase):
    def test_mode_task(self):
        self.assertEqual(E.mode_of("/r/T7-trackB-fx-postfix-F1-x"), "postfix")
        self.assertEqual(E.mode_of("/r/T9-trackB-fx-paced15-F1-x"), "paced15")
        self.assertEqual(E.task_of("/r/T1v2-broken_calc-trackB-fx-paced-P-F1"), "T1v2")


if __name__ == "__main__":
    unittest.main()
