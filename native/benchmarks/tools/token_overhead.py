#!/usr/bin/env python3
"""Split Harness input tokens into fixed overhead x requests and conversation growth.

Model-free. Reads a Harness rec/run.jsonl (no request bodies are saved by the bench).
Rebuilds the char size of each model request: fixed part (tool schema JSON + task
prompt; Harness sends no system prompt) plus history (assistant tool calls, tool
results). Then fits one chars-per-token ratio k so that sum(chars)/k == tokens_prompt.

Label: weak model (Gemini Flash-Lite) through the proxy; raw, not reviewed.
"""
import json, sys

def request_sizes(events, fixed_chars):
    """Return list of (fixed_chars, history_chars) for each model_request event."""
    hist, out = 0, []
    for e in events:
        t = e.get("type")
        if t == "model_request":
            out.append((fixed_chars, hist))
        elif t == "model_response":
            hist += len(e.get("content") or "")
            for c in e.get("tool_calls") or []:
                hist += len(c.get("name", "")) + len(json.dumps(c.get("arguments", {})))
        elif t == "tool_result":
            hist += len(e.get("content") or "")
    return out

def split(events, fixed_chars, tokens_prompt):
    reqs = request_sizes(events, fixed_chars)
    n = len(reqs)
    fixed_total = sum(f for f, _ in reqs)
    hist_total = sum(h for _, h in reqs)
    total_chars = fixed_total + hist_total
    k = total_chars / tokens_prompt if tokens_prompt else None
    fixed_tok = fixed_total / k if k else None
    return {"requests": n, "fixed_chars_per_req": fixed_chars,
            "chars_per_token_fit": round(k, 2) if k else None,
            "fixed_tokens_total": round(fixed_tok) if k else None,
            "growth_tokens_total": round(tokens_prompt - fixed_tok) if k else None,
            "fixed_share": round(fixed_tok / tokens_prompt, 3) if k else None,
            "largest_tool_result_chars": max([len(e.get("content") or "") for e in events if e.get("type") == "tool_result"] or [0])}

def is_test_runner(name, args):
    """True for run_tests, or run_shell whose command runs cargo test."""
    if name == "run_tests":
        return True
    return name == "run_shell" and "cargo test" in json.dumps(args)

def carried_by_kind(events):
    """Chars of tool results re-sent in later requests, split test-runner vs other.
    A result present before request i is counted once for request i."""
    args, cur, test_c, other_c = {}, {"test": 0, "other": 0}, 0, 0
    for e in events:
        t = e.get("type")
        if t == "tool_call":
            args[e.get("id")] = (e.get("name"), e.get("arguments", {}))
        elif t == "tool_result":
            name, a = args.get(e.get("id"), (e.get("name"), {}))
            k = "test" if is_test_runner(name, a) else "other"
            cur[k] += len(e.get("content") or "")
        elif t == "model_request":
            test_c += cur["test"]; other_c += cur["other"]
    return test_c, other_c

def main(argv):
    if len(argv) != 4:
        print("usage: token_overhead.py RUN_JSONL METRICS_JSON FIXED_CHARS", file=sys.stderr); return 2
    events = [json.loads(l) for l in open(argv[1]) if l.strip()]
    tp = json.load(open(argv[2]))["tokens_prompt"]
    r = split(events, int(argv[3]), tp)
    tc, oc = carried_by_kind(events)
    k = r["chars_per_token_fit"]
    if k:
        r["test_output_tokens"] = round(tc / k)
        r["test_output_share"] = round(tc / k / tp, 3)
        r["other_result_tokens"] = round(oc / k)
    print(json.dumps(r))
    return 0

if __name__ == "__main__":
    sys.exit(main(sys.argv))
