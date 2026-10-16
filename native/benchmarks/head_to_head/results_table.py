#!/usr/bin/env python3
"""Build a results table (TSV) from run folders. Fake, killed and upstream-affected (any 503, 429 or transport error), oracle-less (oracle_success missing or null)
and zero-200 runs are refused: they are listed
on stderr and never count toward any N-of-N result.
usage: results_table.py <run_dir> [<run_dir> ...]"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from run_guard import refuse_reason

COLS = ["run", "oracle_success", "stop_reason", "wall_ms", "hold_ms", "tool_calls", "in_tok", "out_tok",
        "proxy_requests", "proxy_input_tokens", "proxy_output_tokens", "steps_to_first_edit", "proxy_429"]


def _jsonl_counts(path):
    """Counts from proxy_usage.jsonl: dict(n200, n429, n503, nerr)."""
    c = dict(n200=0, n429=0, n503=0, nerr=0)
    for line in open(path):
        if not line.strip():
            continue
        try:
            row = json.loads(line)
        except ValueError:
            continue
        st = row.get("status")
        c["n200"] += st == 200
        c["n429"] += st == 429
        c["n503"] += st == 503
        c["nerr"] += st == "error" and row.get("side") != "client"  # client-side rows are not upstream errors
    return c


STATUS_RE = re.compile(r" -> (\d{3})\s*$")


def _log_counts(path):
    """Counts from proxy.log (older run sets have no proxy_usage.jsonl).
    A status line is "POST /chat/completions -> 503" (an ISO time prefix is optional).
    A traceback block is a transport error, except when it fails on a write to the agent
    socket (wfile.write / sendall / BrokenPipeError): that is client side, as side:client in the jsonl."""
    c = dict(n200=0, n429=0, n503=0, nerr=0)
    block = None
    def close(b):
        if b is not None and not any(k in b for k in ("wfile.write", "sendall", "BrokenPipeError")):
            c["nerr"] += 1
    for line in open(path, errors="replace"):
        if line.startswith("Exception occurred during processing") or (line.startswith("Traceback") and block is None):
            block = line; continue
        if block is not None:
            if line.startswith("-" * 20):
                close(block); block = None
            else:
                block += line
            continue
        m = STATUS_RE.search(line)
        if m:
            st = int(m.group(1))
            c["n200"] += st == 200
            c["n429"] += st == 429
            c["n503"] += st == 503
    close(block)
    return c


def proxy_counts(d):
    """Counts from proxy_usage.jsonl; from proxy.log when the jsonl is missing; None if both are missing."""
    for name, fn in (("proxy_usage.jsonl", _jsonl_counts), ("proxy.log", _log_counts)):
        p = os.path.join(d, name)
        if os.path.exists(p):
            try:
                return fn(p)
            except OSError:
                pass
    return None


def upstream_counts(d):
    """(n503, n_transport_error); (0, 0) if no proxy log exists."""
    c = proxy_counts(d) or {}
    return c.get("n503", 0), c.get("nerr", 0)


def upstream_reason(d):
    """Doc 21: a run with any 503, 429 or transport error is upstream-affected and counts for nothing.
    The same rule applies to both agents (fx and harness)."""
    c = proxy_counts(d) or dict(n200=0, n429=0, n503=0, nerr=0)
    flag = False
    try:
        flag = json.load(open(os.path.join(d, "run_env.json"))).get("upstream_affected") is True
    except (OSError, ValueError):
        pass
    if flag or c["n503"] or c["n429"] or c["nerr"]:
        return (f"upstream-affected (503={c['n503']}, 429={c['n429']}, transport_errors={c['nerr']}, "
                f"run_env flag={flag})")
    if proxy_counts(d) is None:
        return "no proxy log (no proxy_usage.jsonl or proxy.log)"
    if c["n200"] == 0:
        return "zero 200 replies"
    return None


def main(dirs):
    rows, refused = [], []
    for d in dirs:
        why = refuse_reason(d) or upstream_reason(d)
        if why:
            refused.append((os.path.basename(d.rstrip("/")), why)); continue
        try:
            s = json.load(open(os.path.join(d, "summary.json")))
        except (OSError, ValueError):
            refused.append((os.path.basename(d.rstrip("/")), "no valid summary.json")); continue
        if s.get("oracle_success") is None:
            refused.append((os.path.basename(d.rstrip("/")), "no oracle result (oracle_success is missing or null)")); continue
        e = {}
        try:
            e = json.load(open(os.path.join(d, "run_env.json")))
        except (OSError, ValueError):
            pass
        m = {**s, **{k: e.get(k) for k in ("proxy_requests", "proxy_input_tokens", "proxy_output_tokens")}}
        m["run"] = os.path.basename(d.rstrip("/"))
        m["proxy_429"] = (proxy_counts(d) or {}).get("n429", 0)
        rows.append([str(m.get(c)) for c in COLS])
    for name, why in refused:
        print(f"REFUSED {name}: {why}. It does not count toward any result.", file=sys.stderr)
    print("\t".join(COLS))
    for r in rows:
        print("\t".join(r))
    for name, why in refused:
        print("\t".join([name, "REFUSED: " + why] + ["-"] * (len(COLS) - 2)))
    ok = sum(1 for r in rows if r[1] == "True")
    print(f"# counted runs: {len(rows)}; oracle success {ok} of {len(rows)}; refused: {len(refused)}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
