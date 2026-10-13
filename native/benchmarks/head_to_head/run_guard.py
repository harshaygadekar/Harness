"""Guard for every step that builds results from run folders.
A run folder marked "fake": true (test-only fake agent) or "status": "killed" (runner did not finish)
stays on disk, but it must never count toward any result."""
import json, os


def refuse_reason(run_dir):
    """Return a reason string if the run must not count, else None."""
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
