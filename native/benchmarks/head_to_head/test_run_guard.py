#!/usr/bin/env python3
"""Test: run_guard.py refuses a run folder with a empty proxy_usage.jsonl, or no proxy_usage.jsonl and no proxy.log,
and a run folder whose agent_exit is 127. A control folder is accepted. No model, no network.
usage: python3 test_run_guard.py"""
import json, os, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import run_guard


def make_run(base, name, usage='{"status": 200}\n', agent_exit="0"):
    d = os.path.join(base, name)
    os.makedirs(d)
    if usage is not None:
        open(os.path.join(d, "proxy_usage.jsonl"), "w").write(usage)
    if agent_exit is not None:
        open(os.path.join(d, "agent_exit"), "w").write(agent_exit + "\n")
    return d


def main():
    fails = 0

    def check(name, cond, detail=""):
        nonlocal fails
        print(f"{'PASS' if cond else 'FAIL'}  {name}" + (f"  ({detail})" if detail and not cond else ""))
        fails += 0 if cond else 1

    with tempfile.TemporaryDirectory() as base:
        good = make_run(base, "good")
        empty = make_run(base, "empty_log", usage="")
        blank = make_run(base, "blank_log", usage="\n\n")
        missing = make_run(base, "missing_log", usage=None)
        e127 = make_run(base, "exit127", agent_exit="127")
        e1 = make_run(base, "exit1", agent_exit="1")
        check("accepts control folder", run_guard.refuse_reason(good) is None)
        check("refuses empty proxy_usage.jsonl", "empty" in (run_guard.refuse_reason(empty) or ""))
        check("refuses blank-line proxy_usage.jsonl", "empty" in (run_guard.refuse_reason(blank) or ""))
        check("refuses missing proxy_usage.jsonl and missing proxy.log", "no proxy log" in (run_guard.refuse_reason(missing) or ""))
        legacy = make_run(base, "legacy_proxy_log", usage=None)
        open(os.path.join(legacy, "proxy.log"), "w").write("x\n")
        check("accepts older folder with proxy.log only (results_table.py reads it)", run_guard.refuse_reason(legacy) is None)
        check("refuses agent exit 127", "127" in (run_guard.refuse_reason(e127) or ""))
        check("accepts agent exit 1 (a real agent failure still counts)", run_guard.refuse_reason(e1) is None)
        r = subprocess.run([sys.executable, os.path.join(HERE, "run_guard.py"), empty, e127], capture_output=True, text=True)
        rep = make_run(base, "replay")
        json.dump({"replay": "h1req2", "run_user": "benchagent"}, open(os.path.join(rep, "run_env.json"), "w"))
        check("refuses a replay folder (run_env.json has replay)", "replay" in (run_guard.refuse_reason(rep) or ""))
        good2 = make_run(base, "good2")
        open(os.path.join(good2, "cargo_exit"), "w").write("0"); open(os.path.join(good2, "wall_ms"), "w").write("1000")
        json.dump({"run": "good2", "oracle_success": True}, open(os.path.join(good2, "summary.json"), "w"))
        for x in (rep,):
            open(os.path.join(x, "cargo_exit"), "w").write("0"); open(os.path.join(x, "wall_ms"), "w").write("1000")
            json.dump({"run": "replay", "oracle_success": True}, open(os.path.join(x, "summary.json"), "w"))
        rt = subprocess.run([sys.executable, os.path.join(HERE, "results_table.py"), rep, good2], capture_output=True, text=True)
        outp = rt.stdout + rt.stderr
        check("results_table refuses the replay folder", "REFUSED" in outp and "replay" in outp, outp[-300:])
        check("results_table counts only the control folder", "counted runs: 1;" in outp, outp[-300:])
        check("CLI exit 3 and two REFUSED lines", r.returncode == 3 and r.stderr.count("REFUSED") == 2, r.stderr)
        r = subprocess.run([sys.executable, os.path.join(HERE, "run_guard.py"), good], capture_output=True, text=True)
        check("CLI exit 0 for control folder", r.returncode == 0, r.stderr)
    print("OVERALL", "PASS" if fails == 0 else "FAIL")
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())
