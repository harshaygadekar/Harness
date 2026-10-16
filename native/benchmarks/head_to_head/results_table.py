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


CLIENT_WRITE_EXC = ("BrokenPipeError", "ConnectionResetError", "ConnectionAbortedError")
CHAIN_MARKERS = ("During handling of the above exception", "The above exception was the direct cause")


def _segment_is_client(seg):
    """One traceback (no chain) is client side only if its LAST frame's code line is a
    self.wfile.write call and its final exception is a client-write error. A bare sendall is
    not enough: http.client also calls sock.sendall when it sends upstream.
    One exception to "last frame": the stdlib wfile wrapper frame (socketserver.py, in write,
    self._sock.sendall) is the body of self.wfile.write itself, so it is skipped and the frame
    above it is the last frame. A sendall frame in any other file (for example http/client.py)
    is not skipped."""
    frames, exc = [], None
    for i, line in enumerate(seg):
        if line.lstrip().startswith('File "'):
            code = ""
            for nxt in seg[i + 1:]:
                t = nxt.strip()
                if t.startswith('File "') or (t and not nxt[0].isspace()):
                    break
                if t and not set(t) <= set("~^ "):
                    code = t; break
            frames.append((line.strip(), code))
        elif line.strip() and not line[0].isspace() and not line.startswith(("Traceback", "Exception occurred")):
            exc = line.strip()
    while frames and "socketserver.py" in frames[-1][0] and frames[-1][0].endswith("in write") \
            and frames[-1][1].startswith("self._sock.sendall"):
        frames.pop()
    return (bool(frames) and "self.wfile.write" in frames[-1][1] and exc is not None
            and exc.split(":")[0].split(".")[-1] in CLIENT_WRITE_EXC)


def _block_is_transport_error(block):
    """A block counts as 1 transport error if ANY of its chained tracebacks is not a client-side
    write failure (for example an upstream error, then a BrokenPipe on the error write)."""
    segs, cur = [], []
    for line in block:
        if line.startswith(CHAIN_MARKERS):
            segs.append(cur); cur = []
        else:
            cur.append(line)
    segs.append(cur)
    segs = [s for s in segs if any(l.strip() for l in s)]  # a block with no frame is not client side
    return bool(segs) and not all(_segment_is_client(s) for s in segs)


def _log_counts(path):
    """Counts from proxy.log (older run sets have no proxy_usage.jsonl).
    A status line is "POST /chat/completions -> 503" (an ISO time prefix is optional).
    A traceback block starts at "Exception occurred during processing" or "Traceback" and closes at a
    dash line, a status line (which is also counted), the next block start, or EOF. Each block is
    0 or 1 transport errors (see _block_is_transport_error)."""
    c = dict(n200=0, n429=0, n503=0, nerr=0)
    block = None
    def close():
        nonlocal block
        if block is not None and _block_is_transport_error(block):
            c["nerr"] += 1
        block = None
    for line in open(path, errors="replace"):
        line = line.rstrip("\n")
        if line.startswith("Exception occurred during processing") or (line.startswith("Traceback") and block is None):
            close(); block = [line]; continue
        m = STATUS_RE.search(line)
        if m:
            close()
            st = int(m.group(1))
            c["n200"] += st == 200
            c["n429"] += st == 429
            c["n503"] += st == 503
            continue
        if block is not None:
            if line.startswith("-" * 20):
                close()
            else:
                block.append(line)
    close()
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
