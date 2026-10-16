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
    check("429 run refused (429 is upstream-affected, like 503)", "REFUSED: upstream-affected (503=0, 429=1, transport_errors=0" in rows["r429"])
    check("client-side error row only: run counted", "REFUSED" not in rows["rclient"])
    check("503 run refused with counts", "REFUSED: upstream-affected (503=1, 429=0, transport_errors=0" in rows["r503"])
    check("transport error run refused with counts", "REFUSED: upstream-affected (503=0, 429=0, transport_errors=2" in rows["rerr"])
    check("run_env upstream_affected flag refused", "run_env flag=True" in rows["rflag"])
    check("summary line: 2 counted, 2 of 2, 4 refused", "counted runs: 2; oracle success 2 of 2; refused: 4" in err)
    check("refused runs listed on stderr", err.count("REFUSED ") == 4)
    check("proxy_429 column present, 0 for clean run", out.splitlines()[0].split("\t")[-1] == "proxy_429" and rows["clean"].split("\t")[-1] == "0")

def mklog(root, name, log_lines, summary=None):
    """A run folder in the older layout: proxy.log only, no proxy_usage.jsonl."""
    d = os.path.join(root, name); os.makedirs(d)
    json.dump(summary if summary is not None else {"oracle_success": True, "stop_reason": "exit 0"}, open(os.path.join(d, "summary.json"), "w"))
    open(os.path.join(d, "proxy.log"), "w").write("".join(l + "\n" for l in log_lines))
    return d
OK = "POST /chat/completions -> 200"
with tempfile.TemporaryDirectory() as t:
    ds = [mklog(t, "log_clean", ["oai_stream_proxy on 127.0.0.1:8787 sig_mode=reinject", OK, OK]),
          mklog(t, "log503", [OK, "POST /chat/completions -> 503", OK]),
          mklog(t, "log429", [OK, "2026-10-11T05:21:28.000+05:30 POST /chat/completions -> 429", OK]),
          mklog(t, "logpipe", [OK, "-" * 40, "Exception occurred during processing of request from ('127.0.0.1', 5)", "Traceback (most recent call last):",
                               '  File "/opt/benchproxy/x/proxy.py", line 315, in _fwd', "    self.wfile.write(out)",
                               '  File "/usr/lib/python3.13/socketserver.py", line 845, in write', "    self._sock.sendall(b)", "BrokenPipeError: [Errno 32] Broken pipe", "-" * 40]),
          mklog(t, "logconn", [OK, "-" * 40, "Exception occurred during processing of request from ('127.0.0.1', 5)", "Traceback (most recent call last):",
                               "    conn.connect()", "ConnectionRefusedError: [Errno 111] Connection refused", "-" * 40]),
          mklog(t, "oracle_none", [OK], {"oracle_success": None, "stop_reason": "exit 0"}),
          mklog(t, "oracle_missing", [OK], {"stop_reason": "exit 0"}),
          mklog(t, "zero200", ["POST /chat/completions -> 400"]),
          mk(t, "zero200_jsonl", [400], {}),
          os.path.join(t, "noproxy")]
    os.makedirs(ds[-1]); json.dump({"oracle_success": True, "stop_reason": "exit 0"}, open(os.path.join(ds[-1], "summary.json"), "w"))
    r = subprocess.run([sys.executable, os.path.join(HERE, "results_table.py")] + ds, capture_output=True, text=True)
    rows = {l.split("\t")[0]: l for l in r.stdout.splitlines()[1:]}
    check("proxy.log only: clean run counted", "REFUSED" not in rows["log_clean"])
    check("proxy.log only: 503 run refused", "REFUSED: upstream-affected (503=1, 429=0, transport_errors=0" in rows["log503"])
    check("proxy.log only: 429 run refused", "REFUSED: upstream-affected (503=0, 429=1, transport_errors=0" in rows["log429"])
    check("proxy.log only: client broken pipe traceback is not a transport error", "REFUSED" not in rows["logpipe"])
    check("proxy.log only: upstream traceback is a transport error", "transport_errors=1" in rows["logconn"])
    check("oracle_success null refused", "REFUSED: no oracle result" in rows["oracle_none"])
    check("oracle_success missing refused", "REFUSED: no oracle result" in rows["oracle_missing"])
    check("zero 200 replies (proxy.log) refused", "REFUSED: zero 200 replies" in rows["zero200"])
    check("zero 200 replies (jsonl) refused", "REFUSED: zero 200 replies" in rows["zero200_jsonl"])
    check("no proxy file refused as no proxy log", "REFUSED: no proxy log" in rows["noproxy"])
    check("summary line: 2 counted, 8 refused", "counted runs: 2; oracle success 2 of 2; refused: 8" in r.stderr)
D = "-" * 40; EXC = "Exception occurred during processing of request from ('127.0.0.1', 5)"; TB = "Traceback (most recent call last):"
with tempfile.TemporaryDirectory() as t:
    ds = [mklog(t, "up_sendall", [OK, D, EXC, TB, '  File "/opt/benchproxy/x/proxy.py", line 403, in _fwd',
                                  "    conn.request(self.command, u.path + path, body=body, headers=hdrs)",
                                  '  File "/usr/lib/python3.13/http/client.py", line 1057, in send',
                                  "    self.sock.sendall(data)", "    ~~~~~~~~~~~~~~~~~^^^^^^", "BrokenPipeError: [Errno 32] Broken pipe", D]),
          mklog(t, "chained", [OK, D, EXC, TB, '  File "/opt/benchproxy/x/proxy.py", line 405, in _fwd', "    r = conn.getresponse()",
                               "ConnectionResetError: [Errno 104] Connection reset by peer", "",
                               "During handling of the above exception, another exception occurred:", "", TB,
                               '  File "/opt/benchproxy/x/proxy.py", line 412, in _fwd', "    self.wfile.write(data)",
                               "BrokenPipeError: [Errno 32] Broken pipe", D]),
          mklog(t, "bare_then_503", [OK, TB, '  File "/opt/benchproxy/x/proxy.py", line 440, in _fwd', "    self.wfile.write(out)",
                                     "BrokenPipeError: [Errno 32] Broken pipe", "POST /chat/completions -> 503", OK])]
    r = subprocess.run([sys.executable, os.path.join(HERE, "results_table.py")] + ds, capture_output=True, text=True)
    rows = {l.split("\t")[0]: l for l in r.stdout.splitlines()[1:]}
    check("upstream sock.sendall failure in http.client: transport_errors=1", "transport_errors=1," in rows["up_sendall"])
    check("chained upstream error + BrokenPipe on error write: transport_errors=1 (1 block, 1 error)", "transport_errors=1," in rows["chained"])
    check("client bare traceback (no dash line) then -> 503: refused with 503=1, transport_errors=0",
          "REFUSED: upstream-affected (503=1, 429=0, transport_errors=0," in rows["bare_then_503"])
sys.exit(1 if fails else 0)
