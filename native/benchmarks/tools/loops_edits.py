#!/usr/bin/env python3
"""Loop and repeat-edit counter for head-to-head runs (stdlib only, read-only).

Reuses the parsers of label_tool_calls.py. Per run it reports:
  (a) identical calls: calls whose key (tool + args) was seen before in the run;
      max consecutive identical streak; paper thresholds (arXiv 2609.20804 App. A.4):
      reminder when a streak reaches 5 identical calls (any status) or 5 identical
      failing calls; stop when an identical failing streak reaches 8.
  (b) repeat edits: edit/write calls to a file beyond the first, per file.

Limits: fx keys come from stderr activity lines (path or command only), so two fx
reads of one file at different offsets look identical. "Failing" means the tool
reported not-ok (Harness: run_tests with failing tests is not-ok).
Output holds counts and normalized paths only; no args content or results.

Usage: loops_edits.py RUN_DIR... [--csv FILE] [--json FILE]
"""
import argparse
import csv
import json
import os
import statistics
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import label_tool_calls as L  # noqa: E402

REMIND_AT = 5
STOP_FAIL_AT = 8


def mode_of(run_dir):
    base = os.path.basename(os.path.normpath(run_dir))
    for m in ("paced15", "postfix", "paced", "proxy", "cerebras"):
        if "-" + m + "-" in base:
            return m
    return "plain"


def task_of(run_dir):
    base = os.path.basename(os.path.normpath(run_dir))
    for p in ("T1v2", "T7", "T9", "S1", "T1"):
        if base.startswith(p + "-"):
            return p
    return "unknown"


def loop_stats(calls):
    """Pure. calls: list of dicts with key, ok."""
    seen = set()
    repeats = 0
    max_streak = streak = 0
    fail_streak = 0
    prev = None
    reminders = 0
    stop_at = None
    reminded_this_streak = False
    for i, c in enumerate(calls):
        k = c["key"]
        if k in seen:
            repeats += 1
        seen.add(k)
        if k == prev:
            streak += 1
            fail_streak = fail_streak + 1 if not c["ok"] else 0
        else:
            streak = 1
            fail_streak = 0 if c["ok"] else 1
            reminded_this_streak = False
        prev = k
        max_streak = max(max_streak, streak)
        if not reminded_this_streak and (streak >= REMIND_AT or fail_streak >= REMIND_AT):
            reminders += 1
            reminded_this_streak = True
        if stop_at is None and fail_streak >= STOP_FAIL_AT:
            stop_at = i
    return {"identical_repeats": repeats, "max_consecutive_identical": max_streak,
            "paper_reminders": reminders, "paper_stop_call_index": stop_at}


def edit_stats(calls):
    """Pure. Counts edit/write calls per path beyond the first."""
    per = {}
    unknown = 0
    for c in calls:
        if c["category"] in ("edit", "write"):
            if c["path"]:
                per[c["path"]] = per.get(c["path"], 0) + 1
            else:
                unknown += 1
    return {"edit_calls": sum(per.values()) + unknown,
            "repeat_edits": sum(n - 1 for n in per.values()),
            "max_edits_one_file": max(per.values()) if per else 0,
            "edits_path_unknown": unknown,
            "files_edited": len(per)}


def analyze(run_dir):
    calls, info = L.parse_run(run_dir)
    r = {"run": os.path.basename(os.path.normpath(run_dir)), "task": task_of(run_dir),
         "agent": L.agent_of(run_dir), "mode": mode_of(run_dir),
         "complete": info["complete"], "done_status": info.get("done_status"),
         "green": L.run_green(run_dir), "total_calls": len(calls)}
    r.update(loop_stats(calls))
    r.update(edit_stats(calls))
    return r


METRICS = ("total_calls", "identical_repeats", "max_consecutive_identical", "paper_reminders",
           "paper_stops", "edit_calls", "repeat_edits", "max_edits_one_file", "edits_path_unknown")


def summarize(results):
    groups = {}
    for r in results:
        r["paper_stops"] = 0 if r["paper_stop_call_index"] is None else 1
        groups.setdefault((r["task"], r["agent"], r["mode"]), []).append(r)
    rows = []
    for (t, a, m), rs in sorted(groups.items()):
        row = {"task": t, "agent": a, "mode": m, "n": len(rs), "n_green": sum(r["green"] for r in rs)}
        for k in METRICS:
            v = [r[k] for r in rs]
            row[k + "_median"], row[k + "_min"], row[k + "_max"] = statistics.median(v), min(v), max(v)
        rows.append(row)
    return rows


def main(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("run_dirs", nargs="+")
    ap.add_argument("--csv")
    ap.add_argument("--json")
    a = ap.parse_args(argv)
    res = []
    for d in a.run_dirs:
        try:
            r = analyze(d)
        except (L.ParseError, OSError) as e:
            print("skipped %s: %s" % (os.path.basename(d), e), file=sys.stderr)
            continue
        if not r["complete"]:
            print("skipped %s: incomplete" % r["run"], file=sys.stderr)
            continue
        res.append(r)
    rows = summarize(res)
    if a.csv:
        with open(a.csv, "w", newline="") as f:
            w = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
            w.writeheader()
            w.writerows(rows)
    out = {"rows": rows, "runs": res}
    if a.json:
        with open(a.json, "w") as f:
            json.dump(out, f, indent=1)
    if not a.csv and not a.json:
        print(json.dumps(out, indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main())
