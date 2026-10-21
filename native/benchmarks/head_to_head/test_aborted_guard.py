#!/usr/bin/env python3
"""Test: a run folder with ABORTED.txt is refused by run_guard.py, summ.py and results_table.py.
A control folder without ABORTED.txt is accepted by all 3. No model, no network.
usage: python3 test_aborted_guard.py"""
import json, os, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))


def make_run(base, name, aborted):
    d = os.path.join(base, name)
    os.makedirs(d)
    open(os.path.join(d, "cargo_exit"), "w").write("0")
    open(os.path.join(d, "wall_ms"), "w").write("1000")
    json.dump({"run": name, "oracle_success": True}, open(os.path.join(d, "summary.json"), "w"))
    # A clean proxy log: main's guard refuses a folder with no proxy log or with zero 200 replies.
    open(os.path.join(d, "proxy_usage.jsonl"), "w").write('{"status": 200}\n')
    if aborted:
        open(os.path.join(d, "ABORTED.txt"), "w").write("stopped by operator\n")
    return d


def run(args):
    return subprocess.run([sys.executable] + args, capture_output=True, text=True)


def main():
    fails = 0

    def check(name, cond, detail=""):
        nonlocal fails
        print(f"{'PASS' if cond else 'FAIL'}  {name}" + (f"  ({detail})" if detail and not cond else ""))
        fails += 0 if cond else 1

    with tempfile.TemporaryDirectory() as base:
        bad = make_run(base, "run_aborted", True)
        good = make_run(base, "run_ok", False)
        summ_before = open(os.path.join(bad, "summary.json")).read()

        r = run([os.path.join(HERE, "run_guard.py"), bad])
        check("run_guard.py refuses ABORTED.txt folder", r.returncode == 3 and "aborted" in r.stderr, r.stderr)
        r = run([os.path.join(HERE, "run_guard.py"), good])
        check("run_guard.py accepts control folder", r.returncode == 0, r.stderr)

        r = run([os.path.join(HERE, "summ.py"), bad, "T1", "harness", "summary_test.json"])
        check("summ.py refuses ABORTED.txt folder", r.returncode == 3 and "REFUSED run_aborted" in r.stderr
              and not os.path.exists(os.path.join(bad, "summary_test.json")), r.stderr)
        check("summ.py leaves the old summary.json unchanged", open(os.path.join(bad, "summary.json")).read() == summ_before)
        r = run([os.path.join(HERE, "summ.py"), good, "T1", "harness", "summary_test.json"])
        check("summ.py accepts control folder", r.returncode == 0 and os.path.exists(os.path.join(good, "summary_test.json")), r.stderr[-300:])

        r = run([os.path.join(HERE, "results_table.py"), bad, good])
        rows = [l for l in r.stdout.splitlines()[1:] if l.strip()]
        # Main may list a refused run as a row, but only with a REFUSED mark.
        counted = [l for l in rows if not l.split("\t")[1].startswith("REFUSED")]
        check("results_table.py refuses ABORTED.txt folder", "REFUSED run_aborted" in r.stderr
              and not any(l.startswith("run_aborted\t") for l in counted), r.stderr)
        check("results_table.py counts only the control folder", [l.split("\t")[0] for l in counted] == ["run_ok"]
              and "counted runs: 1;" in r.stderr and "refused: 1" in r.stderr, r.stdout + r.stderr)

    print(f"# {'ALL PASS' if not fails else f'{fails} FAIL'}")
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())
