"""Unit tests for label_tool_calls.py (stdlib unittest, synthetic inputs)."""
import json
import os
import tempfile
import unittest

import label_tool_calls as L

ORACLE = {
    "task": "TX",
    "bug_files": ["src/bug.rs"],
    "useful_reads": ["tests/t.rs"],
    "decoy_files": ["src/decoy.rs", "docs/"],
    "unnecessary_reads": ["src/noise.rs"],
    "other_reads": "neutral",
    "forbidden_edit_files": ["tests/"],
}

FAIL_OUT = "[cargo test] exit 101\ntest a::x ... FAILED\ntest result: FAILED. 1 passed; 1 failed; 0 ignored"
PASS_OUT = "[cargo test] exit 0\ntest a::x ... ok\ntest result: ok. 2 passed; 0 failed; 0 ignored"


def ev_call(i, name, args, content, ok=True):
    cid = "c%d" % i
    return [
        {"type": "tool_call", "id": cid, "name": name, "arguments": args},
        {"type": "tool_result", "id": cid, "name": name, "content": content, "ok": ok},
    ]


def make_harness_run(root, events, files_modified=("src/bug.rs",), cargo_exit="0", done=True):
    d = os.path.join(root, "TX-trackB-harness-paced-H1-20260101T000000")
    os.makedirs(os.path.join(d, "rec"))
    with open(os.path.join(d, "rec", "run.jsonl"), "w") as f:
        for e in events:
            f.write(json.dumps(e) + "\n")
        if done:
            f.write(json.dumps({"type": "done", "status": "Succeeded", "steps": 1}) + "\n")
    with open(os.path.join(d, "files_modified.txt"), "w") as f:
        f.write("\n".join(files_modified) + "\n")
    with open(os.path.join(d, "cargo_exit"), "w") as f:
        f.write(cargo_exit + "\n")
    return d


def make_pack(root):
    p = os.path.join(root, "pack")
    os.makedirs(p)
    with open(os.path.join(p, "oracle_labels.json"), "w") as f:
        json.dump(ORACLE, f)
    return p


class HarnessTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = self.tmp.name
        self.pack = make_pack(self.root)

    def tearDown(self):
        self.tmp.cleanup()

    def labels(self, events, **kw):
        d = make_harness_run(self.root, events, **kw)
        r = L.label_run(d, self.pack)
        return r, [s["label"] for s in r["sequence"]]

    def test_core_sequence(self):
        evs = []
        evs += ev_call(0, "run_tests", {}, FAIL_OUT, ok=False)          # useful: first failing names
        evs += ev_call(1, "read_file", {"path": "src/bug.rs"}, "x")      # useful: bug file
        evs += ev_call(2, "read_file", {"path": "src/bug.rs"}, "x")      # unnecessary: repeat
        evs += ev_call(3, "read_file", {"path": "src/decoy.rs"}, "x")    # neutral, decoy read
        evs += ev_call(4, "read_file", {"path": "src/noise.rs"}, "x")    # unnecessary (oracle)
        evs += ev_call(5, "search", {"pattern": "foo"}, "a")             # neutral
        evs += ev_call(6, "search", {"pattern": "foo"}, "a")             # unnecessary: repeat
        evs += ev_call(7, "apply_patch", {"path": "src/decoy.rs", "old_string": "a", "new_string": "b"}, "ok")  # unnecessary
        evs += ev_call(8, "apply_patch", {"path": "src/bug.rs", "old_string": "a", "new_string": "b"}, "ok")    # useful
        evs += ev_call(9, "apply_patch", {"path": "src/bug.rs", "old_string": "q", "new_string": "b"}, "no match", ok=False)  # failed
        evs += ev_call(10, "run_tests", {}, PASS_OUT)                    # useful: flip
        evs += ev_call(11, "run_tests", {}, PASS_OUT)                    # unnecessary: identical
        r, labs = self.labels(evs, files_modified=("src/bug.rs", "src/decoy.rs"))
        self.assertEqual(labs, ["useful", "useful", "unnecessary", "neutral", "unnecessary",
                                "neutral", "unnecessary", "unnecessary", "useful", "failed",
                                "useful", "unnecessary"])
        self.assertEqual(r["decoy_reads"], 1)
        self.assertEqual(r["counts"]["useful"], 4)
        self.assertAlmostEqual(r["useful_rate"], 4 / 12)
        self.assertTrue(r["complete"])
        self.assertEqual(r["largest_tool_result_bytes"], len(FAIL_OUT))

    def test_reread_after_write_is_not_repeat(self):
        evs = []
        evs += ev_call(0, "read_file", {"path": "src/bug.rs"}, "x")
        evs += ev_call(1, "apply_patch", {"path": "src/bug.rs", "old_string": "a", "new_string": "b"}, "ok")
        evs += ev_call(2, "read_file", {"path": "src/bug.rs"}, "y")
        _, labs = self.labels(evs)
        self.assertEqual(labs, ["useful", "useful", "neutral"])

    def test_edit_not_green_is_neutral(self):
        evs = ev_call(0, "apply_patch", {"path": "src/bug.rs", "old_string": "a", "new_string": "b"}, "ok")
        _, labs = self.labels(evs, cargo_exit="101")
        self.assertEqual(labs, ["neutral"])

    def test_forbidden_edit(self):
        evs = ev_call(0, "write_file", {"path": "tests/t.rs", "content": "x"}, "ok")
        _, labs = self.labels(evs, files_modified=("tests/t.rs",))
        self.assertEqual(labs, ["unnecessary"])

    def test_shell_cargo_test_timeout_is_failed(self):
        evs = ev_call(0, "run_shell", {"command": "cargo test --test orders"}, "ERROR: tool failed: timeout 30s", ok=False)
        _, labs = self.labels(evs)
        self.assertEqual(labs, ["failed"])

    def test_no_done_is_incomplete(self):
        r, _ = self.labels(ev_call(0, "list_files", {}, "a"), done=False)
        self.assertFalse(r["complete"])

    def test_output_has_no_content(self):
        evs = ev_call(0, "read_file", {"path": "src/bug.rs"}, "SECRET_CONTENT_123")
        r, _ = self.labels(evs)
        self.assertNotIn("SECRET_CONTENT_123", json.dumps(r))


class FxTests(unittest.TestCase):
    def test_fx_alignment_and_labels(self):
        with tempfile.TemporaryDirectory() as root:
            pack = make_pack(root)
            d = os.path.join(root, "TX-trackB-fx-paced-F1-20260101T000000")
            os.makedirs(d)
            cr_fail = {"kind": "command", "command": "cargo test", "cwd": "/tmp/w-x", "exit_code": 101,
                       "stdout_bytes": 100, "stderr_bytes": 20}
            cr_ok = dict(cr_fail, exit_code=0, stdout_bytes=50)
            out = {"exit_code": 0, "usage": {"input_tokens": 1, "output_tokens": 1}, "tool_calls": [
                {"name": "shell", "status": "error", "command_result": cr_fail},
                {"name": "glob_files", "status": "success"},
                {"name": "read_file", "status": "success"},
                {"name": "read_file", "status": "success"},
                {"name": "edit_file", "status": "error"},
                {"name": "edit_file", "status": "success"},
                {"name": "shell", "status": "success", "command_result": cr_ok},
            ]}
            with open(os.path.join(d, "stdout.txt"), "w") as f:
                json.dump(out, f)
            err = "\n".join([
                "Running cargo test",
                "     Running unittests src/lib.rs (indented, not an activity)",
                "test a::x ... FAILED",
                "test result: FAILED. 1 passed; 1 failed; 0 ignored",
                "Matching src/**/*.rs",
                "Reading src/bug.rs",
                "Reading docs/x.md",
                "Editing file",
                "Editing src/bug.rs",
                "Running cargo test",
                "test result: ok. 2 passed; 0 failed; 0 ignored",
            ])
            with open(os.path.join(d, "stderr.txt"), "w") as f:
                f.write(err)
            for name, val in (("files_modified.txt", "src/bug.rs\n"), ("cargo_exit", "0\n"), ("agent_exit", "0\n")):
                with open(os.path.join(d, name), "w") as f:
                    f.write(val)
            r = L.label_run(d, pack)
            labs = [s["label"] for s in r["sequence"]]
            self.assertEqual(labs, ["useful", "neutral", "useful", "neutral", "failed", "useful", "useful"])
            self.assertEqual(r["decoy_reads"], 1)
            self.assertEqual(r["largest_tool_result_bytes"], 120)
            self.assertTrue(r["complete"])

    def test_fx_mismatch_raises(self):
        with tempfile.TemporaryDirectory() as root:
            d = os.path.join(root, "TX-trackB-fx-paced-F1-20260101T000000")
            os.makedirs(d)
            with open(os.path.join(d, "stdout.txt"), "w") as f:
                json.dump({"tool_calls": [{"name": "read_file", "status": "success"}]}, f)
            with open(os.path.join(d, "stderr.txt"), "w") as f:
                f.write("")
            with self.assertRaises(L.ParseError):
                L.parse_run(d)

    def test_fx_empty_stdout_raises(self):
        with tempfile.TemporaryDirectory() as root:
            d = os.path.join(root, "TX-trackB-fx-paced-F1-20260101T000000")
            os.makedirs(d)
            open(os.path.join(d, "stdout.txt"), "w").close()
            with self.assertRaises(L.ParseError):
                L.parse_run(d)


class SummaryTests(unittest.TestCase):
    def test_summary_median(self):
        def r(u, tot):
            return {"task": "TX", "agent": "fx", "mode": "paced", "green": True, "total_calls": tot,
                    "counts": {"useful": u, "unnecessary": 0, "neutral": tot - u, "failed": 0},
                    "useful_rate": u / tot, "decoy_reads": 0}
        rows = L.summarize([r(1, 4), r(2, 4), r(3, 4)])
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]["n"], 3)
        self.assertEqual(rows[0]["useful_median"], 2)
        self.assertEqual(rows[0]["useful_min"], 1)
        self.assertEqual(rows[0]["useful_max"], 3)
        self.assertEqual(rows[0]["useful_rate_median"], 0.5)


if __name__ == "__main__":
    unittest.main()
