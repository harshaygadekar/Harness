#!/usr/bin/env python3
"""results_table.py refuses upstream-affected runs (doc 21). Prints one PASS/FAIL line per check."""
import json, os, subprocess, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__)); fails = 0
def mk(root, name, usage, env):
    d = os.path.join(root, name); os.makedirs(d)
    json.dump({"oracle_success": True, "stop_reason": "exit 0"}, open(os.path.join(d, "summary.json"), "w"))
    json.dump(env, open(os.path.join(d, "run_env.json"), "w"))
    open(os.path.join(d, "proxy_usage.jsonl"), "w").write("".join(json.dumps(s if isinstance(s, dict) else {"status": s}) + "\n" for s in usage))
    return d
def check(label, ok):
    global fails; print(("PASS " if ok else "FAIL ") + label); fails += not ok
with tempfile.TemporaryDirectory() as t:
    ds = [mk(t, "clean", [200, 200], {}), mk(t, "r503", [200, 503], {}), mk(t, "rerr", [200, "error", "error"], {}),
          mk(t, "rflag", [200], {"upstream_affected": True}), mk(t, "r429", [429, 200], {}), mk(t, "rclient", [200, {"status": "error", "side": "client"}], {})]
    r = subprocess.run([sys.executable, os.path.join(HERE, "results_table.py")] + ds, capture_output=True, text=True)
    out, err = r.stdout, r.stderr
    rows = {l.split("\t")[0]: l for l in out.splitlines()[1:]}
    check("exit 0", r.returncode == 0)
    check("clean run counted", "REFUSED" not in rows["clean"])
    check("429-only run counted (429 is not upstream-affected)", "REFUSED" not in rows["r429"])
    check("client-side error row only: run counted", "REFUSED" not in rows["rclient"])
    check("503 run refused with counts", "REFUSED: upstream-affected (503=1, transport_errors=0" in rows["r503"])
    check("transport error run refused with counts", "REFUSED: upstream-affected (503=0, transport_errors=2" in rows["rerr"])
    check("run_env upstream_affected flag refused", "run_env flag=True" in rows["rflag"])
    check("summary line: 3 counted, 3 of 3, 3 refused", "counted runs: 3; oracle success 3 of 3; refused: 3" in err)
    check("refused runs listed on stderr", err.count("REFUSED ") == 3)
sys.exit(1 if fails else 0)
