"""Guard for every step that builds results from run folders.
A run folder marked "fake": true (test-only fake agent) or "status": "killed" (runner did not finish)
stays on disk, but it must never count toward any result.
A run folder with ABORTED.txt (the run was stopped before it ended) is refused in the same way.
A run folder with an empty proxy_usage.jsonl, or with no proxy_usage.jsonl and no proxy.log, is refused (no proxy rows, so no model call is proven).
A run folder whose agent_exit is 127 is refused (the agent command was not found, so the agent did not start).
CLI: run_guard.py <run_dir> [...]  exit 3 if any folder is refused, else 0."""
import json, os, sys


def refuse_reason(run_dir):
    """Return a reason string if the run must not count, else None."""
    if os.path.exists(os.path.join(run_dir, "ABORTED.txt")):
        return "run is marked aborted (ABORTED.txt)"
    pu = os.path.join(run_dir, "proxy_usage.jsonl")
    if not os.path.isfile(pu):
        # Older run sets have only proxy.log; results_table.py reads that. Refuse only when both are missing.
        if not os.path.isfile(os.path.join(run_dir, "proxy.log")):
            return "no proxy log (no proxy_usage.jsonl or proxy.log)"
    else:
        with open(pu) as f:
            if not any(l.strip() for l in f):
                return "proxy_usage.jsonl is empty"
    ae = os.path.join(run_dir, "agent_exit")
    if os.path.isfile(ae):
        with open(ae) as f:
            if f.read().strip() == "127":
                return "agent exit code is 127 (command not found; agent did not start)"
    for name in ("killed.json", "run_env.json"):
        p = os.path.join(run_dir, name)
        if not os.path.exists(p):
            continue
        try:
            m = json.load(open(p))
        except ValueError:
            return f"{name} is not valid JSON"
        if m.get("status") == "killed":
            return "run is marked killed"
        if m.get("fake") is True:
            return "run is a fake (test-only) run"
    return None


def main(dirs):
    bad = 0
    for d in dirs:
        why = refuse_reason(d)
        if why:
            bad += 1
            print(f"run_guard.py: REFUSED {os.path.basename(d.rstrip('/'))}: {why}. It does not count toward any result.", file=sys.stderr)
    return 3 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
