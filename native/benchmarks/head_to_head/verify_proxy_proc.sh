#!/bin/bash
# Verify exactly one proxy.py process runs, as benchproxy, from the pinned path.
# The root `sudo` launcher line is ignored; every other process whose args mention proxy.py counts.
# usage: verify_proxy_proc.sh <expected_proxy_py_path>   (PS_INPUT=file overrides `ps` for tests)
# stdout: one JSON object; exit 0 = ok, 1 = fail
EXP=$1
if [ -n "${PS_INPUT:-}" ]; then PS=$(cat "$PS_INPUT"); else PS=$(ps -eo pid=,user:32=,comm=,args= -ww); fi
python3 - "$EXP" "$PS" <<'P'
import json,sys
exp,ps=sys.argv[1],sys.argv[2]
procs=[]
for line in ps.splitlines():
    f=line.split(None,3)
    if len(f)<4 or "proxy.py" not in f[3]: continue
    pid,user,comm,args=f
    if comm=="sudo": continue
    a=args.split()
    # The process must BE a proxy.py run: argv[0] is proxy.py, or argv[1] is (interpreter form).
    # Lines that only mention proxy.py as a later argument (e.g. this checker) are ignored.
    if a[0].endswith("proxy.py"): path=a[0]
    elif len(a)>1 and a[1].endswith("proxy.py"): path=a[1]
    else: continue
    procs.append({"pid":int(pid),"user":user,"path":path})
ok=len(procs)==1 and procs[0]["user"]=="benchproxy" and procs[0]["path"]==exp
why=[] if ok else (["count=%d"%len(procs)] if len(procs)!=1 else [])+[("user=%s"%p["user"]) for p in procs if p["user"]!="benchproxy"]+[("path=%s"%p["path"]) for p in procs if p["path"]!=exp]
print(json.dumps({"proxy_proc_ok":ok,"proxy_procs":procs,"expected_path":exp,"reasons":why}))
sys.exit(0 if ok else 1)
P
