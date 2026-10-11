#!/usr/bin/env python3
"""Oracle tool-call labeler for head-to-head benchmark runs (stdlib only).

Labels each tool call in a run folder as useful / unnecessary / neutral / failed,
using the task pack's oracle_labels.json plus the generic rules of
docs/research/13-tool-efficiency.md section 3.4.

Output never contains tool arguments content, tool results, or model text.
Only tool names, categories, labels, normalized paths for read/write targets
are kept internally; the emitted sequence holds names + labels only.

Usage:
  label_tool_calls.py run RUN_DIR [--pack PACK_DIR] [--out FILE]
  label_tool_calls.py summary RUN_DIR... [--json FILE] [--csv FILE] [--per-run-dir DIR]
"""
import argparse
import csv
import json
import os
import re
import statistics
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PACKS_ROOT = os.path.join(os.path.dirname(HERE), "head_to_head")
PACK_BY_PREFIX = [
    ("T1v2", "T1v2_broken_calc"),
    ("T7", "T7_large_repo"),
    ("T9", "T9_noisy_tests"),
    ("T1", "T1_broken_calc"),
]
LABELS = ("useful", "unnecessary", "neutral", "failed")

# Tool name -> category. Harness names and fx names.
CATEGORY = {
    # Harness
    "read_file": "read", "search": "search", "list_files": "list",
    "apply_patch": "edit", "write_file": "write", "run_tests": "test",
    "run_shell": "shell", "git_diff": "shell", "git_status": "shell",
    # fx
    "glob_files": "list", "grep_files": "search", "edit_file": "edit",
    "shell": "shell",
}

FAILED_RE = re.compile(r"^test (\S+) \.\.\. FAILED", re.M)
RESULT_RE = re.compile(r"test result: (ok|FAILED)\. (\d+) passed; (\d+) failed")
FX_ACT_RE = re.compile(r"^(Reading|Editing|Writing|Matching|Searching|Running|Listing) (.*)$")


class ParseError(Exception):
    pass


# ---------------------------------------------------------------- helpers

def norm_path(p, workdir=None):
    if not p:
        return ""
    p = p.strip()
    if workdir and p.startswith(workdir.rstrip("/") + "/"):
        p = p[len(workdir.rstrip("/")) + 1:]
    elif p.startswith("/tmp/"):
        # /tmp/<workdir>/rel -> rel
        parts = p.split("/", 3)
        p = parts[3] if len(parts) > 3 else ""
    while p.startswith("./"):
        p = p[2:]
    return p


def match_any(path, patterns):
    for pat in patterns:
        if pat.endswith("/"):
            if path.startswith(pat):
                return True
        elif path == pat:
            return True
    return False


def test_signature(command, text, exit_code):
    failed = sorted(set(FAILED_RE.findall(text or "")))
    results = RESULT_RE.findall(text or "")
    if exit_code is None:
        passed = bool(results) and not failed and all(r[0] == "ok" for r in results)
    else:
        passed = exit_code == 0
    return {"command": (command or "").strip(), "failed": failed,
            "passed": passed, "exit": exit_code, "has_output": bool(results or failed)}


def is_test_command(cmd):
    return bool(re.match(r"^\s*cargo\s+test\b", cmd or ""))


def find_pack(run_dir):
    base = os.path.basename(os.path.normpath(run_dir))
    for prefix, pack in PACK_BY_PREFIX:
        if base.startswith(prefix + "-"):
            return os.path.join(PACKS_ROOT, pack)
    raise ParseError("cannot infer task pack from folder name")


def agent_of(run_dir):
    base = os.path.basename(os.path.normpath(run_dir))
    if "-harness-" in base:
        return "harness"
    if "-fx-" in base:
        return "fx"
    return "unknown"


def mode_of(run_dir):
    base = os.path.basename(os.path.normpath(run_dir))
    for m in ("paced15", "paced", "proxy", "cerebras"):
        if "-" + m + "-" in base:
            return m
    return "plain"


def read_text(path):
    try:
        with open(path, encoding="utf-8", errors="replace") as f:
            return f.read()
    except OSError:
        return None


# ---------------------------------------------------------------- parsers
# Each parser returns (calls, info). A call is a dict:
#   name, category, ok (bool), path (normalized or ""), key (dedupe key str),
#   test (signature dict or None), result_bytes (int or None)

def parse_harness(run_dir):
    path = os.path.join(run_dir, "rec", "run.jsonl")
    if not os.path.exists(path):
        raise ParseError("no rec/run.jsonl")
    pending = {}
    order = []
    done = None
    with open(path, encoding="utf-8", errors="replace") as f:
        for n, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue
            try:
                ev = json.loads(line)
            except json.JSONDecodeError:
                raise ParseError("bad JSON at line %d" % n)
            t = ev.get("type")
            if t == "tool_call":
                cid = ev.get("id") or "idx%d" % len(order)
                pending[cid] = {"name": ev.get("name", ""),
                                "args": ev.get("arguments") or {}, "res": None}
                order.append(cid)
            elif t == "tool_result":
                cid = ev.get("id")
                if cid in pending:
                    pending[cid]["res"] = ev
            elif t == "done":
                done = ev.get("status")
    calls = []
    for cid in order:
        c = pending[cid]
        name, args, res = c["name"], c["args"], c["res"]
        cat = CATEGORY.get(name, "other")
        ok = bool(res and res.get("ok"))
        content = (res or {}).get("content") or ""
        call = {"name": name, "category": cat, "ok": ok, "path": "",
                "key": name + ":" + json.dumps(args, sort_keys=True),
                "test": None,
                "result_bytes": len(content.encode("utf-8")) if res else None}
        if cat in ("read", "edit", "write", "list"):
            call["path"] = norm_path(args.get("path", ""))
        if cat == "shell" and is_test_command(args.get("command", "")):
            cat = call["category"] = "test"
        if cat == "test":
            cmd = args.get("command", "run_tests")
            m = re.match(r"\[cargo test\] exit (\d+)", content)
            ex = int(m.group(1)) if m else None
            sig = test_signature(cmd, content, ex)
            if name == "run_tests":
                sig["tool_error"] = content.startswith("ERROR:")
            else:
                sig["tool_error"] = (not ok) and not sig["has_output"]
            call["test"] = sig
        calls.append(call)
    info = {"format": "harness_run_jsonl", "done_status": done,
            "complete": done is not None}
    return calls, info


def parse_fx(run_dir):
    raw = read_text(os.path.join(run_dir, "stdout.txt"))
    if raw is None:
        raise ParseError("no stdout.txt")
    try:
        data = json.loads(raw)
    except json.JSONDecodeError:
        raise ParseError("stdout.txt is not fx JSON (empty or cut off)")
    tcs = data.get("tool_calls")
    if tcs is None:
        raise ParseError("fx JSON has no tool_calls")
    err = read_text(os.path.join(run_dir, "stderr.txt")) or ""
    # Activity lines at column 0, with the text that follows each one.
    acts = []
    for line in err.splitlines():
        m = FX_ACT_RE.match(line)
        if m:
            acts.append([m.group(1), m.group(2), []])
        elif acts:
            acts[-1][2].append(line)
    if len(acts) != len(tcs):
        raise ParseError("stderr activity lines (%d) do not match tool_calls (%d)"
                         % (len(acts), len(tcs)))
    workdir = None
    for t in tcs:
        cr = t.get("command_result") or {}
        if cr.get("cwd"):
            workdir = cr["cwd"]
            break
    calls = []
    for t, (verb, arg, after) in zip(tcs, acts):
        name = t.get("name", "")
        cat = CATEGORY.get(name, "other")
        ok = t.get("status") == "success"
        cr = t.get("command_result") or {}
        call = {"name": name, "category": cat, "ok": ok, "path": "",
                "key": name + ":" + arg.strip(), "test": None, "result_bytes": None}
        if cat in ("read", "edit", "write"):
            p = norm_path(arg, workdir)
            call["path"] = "" if p == "file" else p  # "Editing file" = no path shown
        if cat == "shell":
            cmd = cr.get("command") or arg
            call["key"] = name + ":" + cmd.strip()
            if cr:
                call["result_bytes"] = (cr.get("stdout_bytes") or 0) + (cr.get("stderr_bytes") or 0)
            if is_test_command(cmd):
                cat = call["category"] = "test"
                sig = test_signature(cmd, "\n".join(after), cr.get("exit_code"))
                code = ((t.get("error") or {}).get("code"))
                sig["tool_error"] = (not cr) or code == "rejected"
                call["test"] = sig
            else:
                call["shell_exit"] = cr.get("exit_code")
        calls.append(call)
    complete = os.path.exists(os.path.join(run_dir, "agent_exit"))
    info = {"format": "fx_stdout_json+stderr_activity", "done_status": "exit=%s" % data.get("exit_code"),
            "complete": complete,
            "note": "fx read/search result sizes are not recorded; result_bytes covers shell only"}
    return calls, info


def parse_run(run_dir):
    if os.path.exists(os.path.join(run_dir, "rec", "run.jsonl")):
        return parse_harness(run_dir)
    if os.path.exists(os.path.join(run_dir, "stdout.txt")) and agent_of(run_dir) == "fx":
        return parse_fx(run_dir)
    raise ParseError("unknown run format (no rec/run.jsonl and no fx stdout.txt)")


# ---------------------------------------------------------------- labeling

def load_pack(pack_dir):
    p = os.path.join(pack_dir, "oracle_labels.json")
    with open(p, encoding="utf-8") as f:
        return json.load(f)


def final_diff_files(run_dir):
    txt = read_text(os.path.join(run_dir, "files_modified.txt"))
    if txt is None:
        return None
    return {norm_path(l) for l in txt.splitlines() if l.strip()}


def run_green(run_dir):
    txt = read_text(os.path.join(run_dir, "cargo_exit"))
    return txt is not None and txt.strip() == "0"


def label_calls(calls, oracle, diff_files, green):
    """Return list of (label, reason, decoy_read) per call. Pure function."""
    bug = oracle.get("bug_files", [])
    useful_reads = oracle.get("useful_reads", [])
    decoys = oracle.get("decoy_files", [])
    unn_reads = oracle.get("unnecessary_reads", [])
    other_reads = oracle.get("other_reads", "neutral")
    forbidden = oracle.get("forbidden_edit_files", [])
    diff_files = diff_files or set()

    seen_keys = {}      # key -> write counter at time of call
    read_paths = set()
    writes = 0
    path_writes = {}    # path -> number of writes to that path so far
    test_sigs = []
    prev_test_passed = None
    seen_failed_names = False
    out = []
    for c in calls:
        cat, ok, path = c["category"], c["ok"], c["path"]
        decoy = False
        if cat == "test":
            sig = c["test"]
            if sig.get("tool_error"):
                lab, why = "failed", "test tool error"
            else:
                ident = (sig["command"], tuple(sig["failed"]), sig["passed"])
                if ident in test_sigs:
                    lab, why = "unnecessary", "identical test run, identical result"
                elif sig["failed"] and not seen_failed_names:
                    lab, why = "useful", "first view of failing test names"
                elif sig["passed"] and prev_test_passed is False:
                    lab, why = "useful", "fail->pass flip"
                else:
                    lab, why = "neutral", "test run"
                test_sigs.append(ident)
                if sig["failed"]:
                    seen_failed_names = True
                prev_test_passed = sig["passed"]
        elif not ok:
            lab, why = "failed", "tool returned error"
        elif cat == "read":
            k = c["key"]
            pw = path_writes.get(path, 0) if path else writes
            if k in seen_keys and seen_keys[k] == pw:
                lab, why = "unnecessary", "repeat read, same args, file not written between"
            elif match_any(path, bug) and path not in read_paths:
                lab, why = "useful", "first read of real-bug file"
            elif match_any(path, useful_reads) and path not in read_paths:
                lab, why = "useful", "first read of oracle-useful file"
            elif match_any(path, decoys):
                lab, why, decoy = "neutral", "decoy read", True
            elif match_any(path, unn_reads):
                lab, why = "unnecessary", "oracle-unnecessary read"
            elif match_any(path, bug + useful_reads):
                lab, why = "neutral", "re-read after write"
            else:
                lab, why = other_reads, "other read"
            read_paths.add(path)
            seen_keys[k] = pw
        elif cat in ("search", "list"):
            k = c["key"]
            if k in seen_keys and seen_keys[k] == writes:
                lab, why = "unnecessary", "repeat %s, same args, no write between" % cat
            else:
                lab, why = "neutral", cat
            seen_keys[k] = writes
        elif cat in ("edit", "write"):
            writes += 1
            if path:
                path_writes[path] = path_writes.get(path, 0) + 1
            if not path:
                lab, why = "neutral", "edit target unknown"
            elif match_any(path, forbidden) or match_any(path, decoys):
                lab, why = "unnecessary", "edit to decoy/forbidden file"
            elif path in diff_files:
                if green:
                    lab, why = "useful", "edit to file in final green diff"
                else:
                    lab, why = "neutral", "edit to file in final diff, run not green"
            else:
                lab, why = "unnecessary", "edit not in final diff"
        elif cat == "shell":
            k = c["key"]
            ident = (k, c.get("shell_exit"))
            if ident in seen_keys and seen_keys[ident] == writes:
                lab, why = "unnecessary", "identical shell command, same exit, no write between"
            else:
                lab, why = "neutral", "shell"
            seen_keys[ident] = writes
        else:
            lab, why = "neutral", "other tool"
        out.append((lab, why, decoy))
    return out


def label_run(run_dir, pack_dir=None):
    pack_dir = pack_dir or find_pack(run_dir)
    oracle = load_pack(pack_dir)
    calls, info = parse_run(run_dir)
    diff = final_diff_files(run_dir)
    green = run_green(run_dir)
    labs = label_calls(calls, oracle, diff, green)
    counts = {k: 0 for k in LABELS}
    for lab, _, _ in labs:
        counts[lab] += 1
    total = len(calls)
    sizes = [c["result_bytes"] for c in calls if c["result_bytes"] is not None]
    summ = read_text(os.path.join(run_dir, "summary.json"))
    summ_largest = None
    if summ:
        try:
            summ_largest = json.loads(summ).get("largest_tool_result")
        except json.JSONDecodeError:
            pass
    return {
        "run": os.path.basename(os.path.normpath(run_dir)),
        "task": oracle.get("task"),
        "agent": agent_of(run_dir),
        "mode": mode_of(run_dir),
        "format": info["format"],
        "complete": info["complete"],
        "done_status": info.get("done_status"),
        "green": green,
        "total_calls": total,
        "counts": counts,
        "useful_rate": (counts["useful"] / total) if total else None,
        "decoy_reads": sum(1 for l in labs if l[2]),
        "largest_tool_result_bytes": max(sizes) if sizes else "unknown",
        "summary_json_largest_tool_result": summ_largest if summ_largest is not None else "unknown",
        "notes": info.get("note", ""),
        "sequence": [{"i": i, "name": c["name"], "category": c["category"], "label": l[0], "reason": l[1]}
                     for i, (c, l) in enumerate(zip(calls, labs))],
    }


# ---------------------------------------------------------------- summary

def _stats(vals):
    vals = [v for v in vals if v is not None]
    if not vals:
        return ("unknown", "unknown", "unknown")
    return (statistics.median(vals), min(vals), max(vals))


def summarize(results):
    groups = {}
    for r in results:
        groups.setdefault((r["task"], r["agent"], r["mode"]), []).append(r)
    rows = []
    for (task, agent, mode), rs in sorted(groups.items()):
        row = {"task": task, "agent": agent, "mode": mode, "n": len(rs),
               "n_green": sum(1 for r in rs if r["green"])}
        metrics = {
            "total_calls": [r["total_calls"] for r in rs],
            "useful": [r["counts"]["useful"] for r in rs],
            "unnecessary": [r["counts"]["unnecessary"] for r in rs],
            "neutral": [r["counts"]["neutral"] for r in rs],
            "failed": [r["counts"]["failed"] for r in rs],
            "decoy_reads": [r["decoy_reads"] for r in rs],
            "useful_rate": [r["useful_rate"] for r in rs],
        }
        for k, v in metrics.items():
            med, lo, hi = _stats(v)
            if k == "useful_rate":
                med, lo, hi = [round(x, 3) if isinstance(x, float) else x for x in (med, lo, hi)]
            row[k + "_median"], row[k + "_min"], row[k + "_max"] = med, lo, hi
        rows.append(row)
    return rows


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    a = sub.add_parser("run")
    a.add_argument("run_dir")
    a.add_argument("--pack")
    a.add_argument("--out")
    b = sub.add_parser("summary")
    b.add_argument("run_dirs", nargs="+")
    b.add_argument("--json")
    b.add_argument("--csv")
    b.add_argument("--per-run-dir")
    b.add_argument("--include-incomplete", action="store_true")
    args = ap.parse_args(argv)

    if args.cmd == "run":
        res = label_run(args.run_dir, args.pack)
        txt = json.dumps(res, indent=1)
        if args.out:
            with open(args.out, "w") as f:
                f.write(txt + "\n")
        else:
            print(txt)
        return 0

    results, skipped = [], []
    for d in args.run_dirs:
        try:
            r = label_run(d)
        except (ParseError, OSError) as e:
            skipped.append({"run": os.path.basename(os.path.normpath(d)), "reason": str(e)})
            continue
        if not r["complete"] and not args.include_incomplete:
            skipped.append({"run": r["run"], "reason": "incomplete (no done event / agent_exit)"})
            continue
        results.append(r)
        if args.per_run_dir:
            os.makedirs(args.per_run_dir, exist_ok=True)
            with open(os.path.join(args.per_run_dir, r["run"] + ".labels.json"), "w") as f:
                json.dump(r, f, indent=1)
    rows = summarize(results)
    out = {"rows": rows, "skipped": skipped,
           "runs": [{k: r[k] for k in ("run", "task", "agent", "mode", "green", "total_calls",
                                       "counts", "useful_rate", "decoy_reads",
                                       "largest_tool_result_bytes")} for r in results]}
    if args.json:
        with open(args.json, "w") as f:
            json.dump(out, f, indent=1)
    if args.csv:
        with open(args.csv, "w", newline="") as f:
            if rows:
                w = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
                w.writeheader()
                w.writerows(rows)
    if not args.json and not args.csv:
        print(json.dumps(out, indent=1))
    for s in skipped:
        print("skipped %s: %s" % (s["run"], s["reason"]), file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
