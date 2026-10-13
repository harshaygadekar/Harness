#!/usr/bin/env python3
"""Split agent wall time of a head-to-head run (model-free, reads run folders only).

Parts: hold (proxy pacing, proxy_hold.json), retry (429 backoff, summary.json),
tool (Harness: tool_call->tool_result in rec/run.jsonl; fx: shell duration_ms in stdout.txt),
model (Harness: model_request->model_response minus hold and retry; fx: inferred rest),
overhead (Harness: rest; fx: not separable, included in model).
Label: weak model (Gemini Flash-Lite) through the proxy.
"""
import json, os, sys

def harness_parts(events):
    """Return (model_wait_ms, tool_ms, n_requests, first_edit_step) from run.jsonl events."""
    model = tool = 0; n = 0; open_req = None; calls = {}; step = 0; first_edit = None
    for e in events:
        t = e.get("type"); ts = e.get("ts_ms")
        if t == "model_request":
            open_req = ts; n += 1
        elif t == "model_response" and open_req is not None:
            model += ts - open_req; open_req = None
        elif t == "tool_call":
            step += 1; calls[e.get("id")] = ts
            if first_edit is None and e.get("name") in ("write_file", "apply_patch"):
                first_edit = step
        elif t == "tool_result" and e.get("id") in calls:
            tool += ts - calls.pop(e.get("id"))
    return model, tool, n, first_edit

def fx_tool_ms(stdout_obj):
    return sum((c.get("command_result") or {}).get("duration_ms") or 0 for c in stdout_obj.get("tool_calls", []))

def split(run_dir):
    s = json.load(open(os.path.join(run_dir, "summary.json")))
    hold = json.load(open(os.path.join(run_dir, "proxy_hold.json")))
    wall = s["wall_ms"]; h = hold["hold_ms"]; retry = s.get("retry_wait_ms") or 0
    usage = [json.loads(l) for l in open(os.path.join(run_dir, "proxy_usage.jsonl")) if l.strip()]
    out_tok = sum(u.get("output_tokens") or 0 for u in usage)
    r = {"run": os.path.basename(run_dir.rstrip("/")), "agent": s["agent"], "ok": s.get("oracle_success"),
         "wall_ms": wall, "hold_ms": h, "s429": hold.get("status_429", 0), "retry_ms": retry,
         "upstream": hold["upstream_requests"], "out_tok_proxy": out_tok,
         "out_tok_per_req": round(out_tok / len(usage), 1) if usage else None}
    if s["agent"] == "harness":
        ev = [json.loads(l) for l in open(os.path.join(run_dir, "rec", "run.jsonl")) if l.strip()]
        mw, tool, n, fe = harness_parts(ev)
        r.update(model_requests=n, first_edit=fe, tool_ms=tool,
                 model_ms=mw - h - retry, overhead_ms=wall - mw - tool, measured="hold,tool,model,overhead")
    else:
        so = json.loads(open(os.path.join(run_dir, "stdout.txt")).read())
        tool = fx_tool_ms(so)
        r.update(model_requests=s.get("steps", 0) + 1, first_edit=s.get("steps_to_first_edit"), tool_ms=tool,
                 model_ms=wall - h - retry - tool, overhead_ms=None, measured="hold,shell-tool; model=rest (incl. overhead)")
    r["model_ms_per_upstream"] = round(r["model_ms"] / r["upstream"]) if r["upstream"] else None
    return r

if __name__ == "__main__":
    for d in sys.argv[1:]:
        print(json.dumps(split(d)))
