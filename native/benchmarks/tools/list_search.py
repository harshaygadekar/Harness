#!/usr/bin/env python3
"""List drill-down and search-cap counter for head-to-head runs (stdlib, read-only).

Doc 17 candidates 1 and 2. Per run, both agents:
  (a) list calls: total, on root, drill-down (target's parent dir was listed
      earlier in the run), recursive (fx glob with '**').
      Harness: list_files {path}. fx: glob_files / list activity lines
      ("Matching <pattern>"); target = pattern prefix before the first glob char.
  (b) search calls: total; cap hits (Harness only: result has >= 50 hit lines,
      or the 16 KiB excerpt note); searches followed within 2 calls by a
      narrower search (pattern contains the earlier pattern and is longer, or
      same pattern in a deeper path).
fx search result sizes are not recorded, so fx cap hits are "unknown".
Output: counts only. No args content beyond list targets and no results.

Usage: list_search.py RUN_DIR... [--csv FILE] [--json FILE]
"""
import argparse
import csv
import json
import os
import re
import statistics
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import label_tool_calls as L  # noqa: E402
import loops_edits as E  # noqa: E402

SEARCH_CAP = 50
GLOB_CHARS = re.compile(r"[*?\[{]")


def norm_dir(p):
    p = (p or "").strip()
    while p.startswith("./"):
        p = p[2:]
    p = p.rstrip("/")
    return "" if p in ("", ".") else p


def glob_target(pattern):
    """Return (dir, recursive) for a glob pattern."""
    pattern = (pattern or "").strip()
    recursive = "**" in pattern
    m = GLOB_CHARS.search(pattern)
    prefix = pattern[:m.start()] if m else pattern
    if m and "/" not in prefix:
        prefix = ""
    elif m:
        prefix = prefix.rsplit("/", 1)[0]
    return norm_dir(prefix), recursive


def parent(d):
    return d.rsplit("/", 1)[0] if "/" in d else ""


def list_stats(lists):
    """Pure. lists: list of (target_dir, recursive)."""
    seen = set()
    root = drill = rec = 0
    for d, r in lists:
        if r:
            rec += 1
        if d == "":
            root += 1
        elif parent(d) in seen and not r:
            drill += 1
        seen.add(d)
    return {"list_calls": len(lists), "list_root": root, "list_drilldown": drill,
            "list_recursive": rec}


def is_narrower(a, b):
    """b narrower than a. a, b: (pattern, path)."""
    pa, da = a
    pb, db = b
    if pb != pa and pa.lower() in pb.lower():
        return True
    if pb == pa and da != db and (da == "" or db.startswith(da + "/")):
        return True
    return False


def search_follow(calls):
    """Pure. calls: sequence of dicts {kind: 'search'|'other', pattern, path}."""
    n = 0
    for i, c in enumerate(calls):
        if c["kind"] != "search":
            continue
        for nxt in calls[i + 1:i + 3]:
            if nxt["kind"] == "search" and is_narrower((c["pattern"], c["path"]),
                                                       (nxt["pattern"], nxt["path"])):
                n += 1
                break
    return n


def search_capped(content):
    if content is None:
        return None
    if "[truncated; full length" in content:
        return True
    if content.strip() == "(no hits)":
        return False
    return len([l for l in content.splitlines() if l.strip()]) >= SEARCH_CAP


def harness_seq(run_dir):
    calls, res = [], {}
    order = []
    with open(os.path.join(run_dir, "rec", "run.jsonl"), encoding="utf-8", errors="replace") as f:
        for line in f:
            if not line.strip():
                continue
            ev = json.loads(line)
            if ev.get("type") == "tool_call":
                order.append((ev.get("id"), ev.get("name"), ev.get("arguments") or {}))
            elif ev.get("type") == "tool_result":
                res[ev.get("id")] = ev.get("content") or ""
    for cid, name, args in order:
        if name == "list_files":
            calls.append({"kind": "list", "target": norm_dir(args.get("path")), "recursive": False})
        elif name == "search":
            calls.append({"kind": "search", "pattern": str(args.get("pattern", "")),
                          "path": norm_dir(args.get("path")), "capped": search_capped(res.get(cid))})
        else:
            calls.append({"kind": "other"})
    return calls


def fx_seq(run_dir):
    calls, _ = L.parse_fx(run_dir)
    out = []
    for c in calls:
        name, arg = c["name"], c["key"].split(":", 1)[1] if ":" in c["key"] else ""
        if c["category"] == "list":
            d, r = glob_target(arg)
            out.append({"kind": "list", "target": d, "recursive": r})
        elif c["category"] == "search":
            out.append({"kind": "search", "pattern": arg, "path": "", "capped": None})
        else:
            out.append({"kind": "other"})
    return out


def analyze(run_dir):
    agent = L.agent_of(run_dir)
    seq = harness_seq(run_dir) if agent == "harness" else fx_seq(run_dir)
    _, info = L.parse_run(run_dir)
    r = {"run": os.path.basename(os.path.normpath(run_dir)), "task": E.task_of(run_dir),
         "agent": agent, "mode": E.mode_of(run_dir), "complete": info["complete"],
         "green": L.run_green(run_dir)}
    r.update(list_stats([(c["target"], c["recursive"]) for c in seq if c["kind"] == "list"]))
    s = [c for c in seq if c["kind"] == "search"]
    r["search_calls"] = len(s)
    caps = [c["capped"] for c in s]
    r["search_cap_hits"] = "unknown" if any(x is None for x in caps) else sum(caps)
    r["search_then_narrower"] = search_follow(seq)
    return r


METRICS = ("list_calls", "list_root", "list_drilldown", "list_recursive",
           "search_calls", "search_cap_hits", "search_then_narrower")


def summarize(results):
    groups = {}
    for r in results:
        groups.setdefault((r["task"], r["agent"], r["mode"]), []).append(r)
    rows = []
    for (t, a, m), rs in sorted(groups.items()):
        row = {"task": t, "agent": a, "mode": m, "n": len(rs), "n_green": sum(r["green"] for r in rs)}
        for k in METRICS:
            v = [r[k] for r in rs]
            if any(x == "unknown" for x in v):
                row[k + "_median"] = row[k + "_min"] = row[k + "_max"] = "unknown"
            else:
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
        except (L.ParseError, OSError, ValueError) as e:
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
