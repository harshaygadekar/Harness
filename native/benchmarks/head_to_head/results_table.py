#!/usr/bin/env python3
"""Build a results table (TSV) from run folders. Fake and killed runs are refused: they are listed
on stderr and never count toward any N-of-N result.
usage: results_table.py <run_dir> [<run_dir> ...]"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from run_guard import refuse_reason

COLS = ["run", "oracle_success", "stop_reason", "wall_ms", "hold_ms", "tool_calls", "in_tok", "out_tok",
        "proxy_requests", "proxy_input_tokens", "proxy_output_tokens", "steps_to_first_edit"]


def main(dirs):
    rows, refused = [], []
    for d in dirs:
        why = refuse_reason(d)
        if why:
            refused.append((os.path.basename(d.rstrip("/")), why)); continue
        try:
            s = json.load(open(os.path.join(d, "summary.json")))
        except (OSError, ValueError):
            refused.append((os.path.basename(d.rstrip("/")), "no valid summary.json")); continue
        e = {}
        try:
            e = json.load(open(os.path.join(d, "run_env.json")))
        except (OSError, ValueError):
            pass
        m = {**s, **{k: e.get(k) for k in ("proxy_requests", "proxy_input_tokens", "proxy_output_tokens")}}
        m["run"] = os.path.basename(d.rstrip("/"))
        rows.append([str(m.get(c)) for c in COLS])
    for name, why in refused:
        print(f"REFUSED {name}: {why}. It does not count toward any result.", file=sys.stderr)
    print("\t".join(COLS))
    for r in rows:
        print("\t".join(r))
    ok = sum(1 for r in rows if r[1] == "True")
    print(f"# counted runs: {len(rows)}; oracle success {ok} of {len(rows)}; refused: {len(refused)}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
