#!/usr/bin/env python3
"""Rescue logs of runs that did not finish. Trusts ONLY root-owned records in ACTIVE
(/opt/benchproxy/run/active/<run_id>.json, written by the runner with sudo). It never reads pointers from /tmp.
Each path in a record is checked; on any failed check the record is skipped, a line is logged,
and nothing is deleted or written.
  hd  (proxy tmp dir): real dir (lstat, not a symlink), owner benchproxy, realpath == path,
                       matches ^/tmp/benchproxy-[0-9]{8}T[0-9]{6}-[A-Za-z0-9]+$
  ws  (workspace):     real dir, not a symlink, owner = agent user, realpath == path,
                       matches ^/tmp/iso-(T1|T7|T9|S1)-(fx|harness)-[A-Za-z0-9]+-[A-Za-z0-9]{4}$
                       (mktemp names: /tmp/iso-<task>-<agent>-<label>-XXXX)
  run (run folder):    real dir, not a symlink, owner = this runner's uid, realpath's parent == realpath(RUNS)
usage: rescue_killed.py <RUNS> <agent_user>"""
import json, os, pwd, re, stat, subprocess, sys

ACTIVE = "/opt/benchproxy/run/active"
HD_RE = re.compile(r"^/tmp/benchproxy-[0-9]{8}T[0-9]{6}-[A-Za-z0-9]+$")
WS_RE = re.compile(r"^/tmp/iso-(T1|T7|T9|S1)-(fx|harness)-[A-Za-z0-9]+-[A-Za-z0-9]{4}$")
NAME_RE = re.compile(r"^[A-Za-z0-9._-]+\.json$")


def log(msg):
    print("rescue_killed: " + msg, file=sys.stderr)


def real_dir(path, uid):
    """lstat-based: exists, is a directory, is not a symlink, owner uid, and realpath is the same path."""
    try:
        st = os.lstat(path)
    except OSError:
        return "missing"
    if stat.S_ISLNK(st.st_mode):
        return "is a symlink"
    if not stat.S_ISDIR(st.st_mode):
        return "not a directory"
    if st.st_uid != uid:
        return f"wrong owner uid {st.st_uid}"
    if os.path.realpath(path) != path:
        return "realpath differs"
    return None


def sudo(*a):
    subprocess.run(["sudo", "-n", *a], check=False)


def main(runs, agent_user):
    # Refuse while any agent or proxy process is alive: they could swap a checked folder for a link.
    for u in (agent_user, "benchproxy"):
        if subprocess.run(["pgrep", "-u", u], capture_output=True).stdout.strip():
            log(f"skip all: {u} process alive; nothing deleted or written"); return 4
    runs_real = os.path.realpath(runs)
    me = os.getuid(); bp = pwd.getpwnam("benchproxy").pw_uid; ag = pwd.getpwnam(agent_user).pw_uid
    try:
        st = os.lstat(ACTIVE)
    except OSError:
        return 0
    if stat.S_ISLNK(st.st_mode) or st.st_uid != 0 or (st.st_mode & 0o022):
        log(f"skip all: {ACTIVE} is not a root-owned, non-writable directory"); return 0
    for name in sorted(os.listdir(ACTIVE)):
        rec = os.path.join(ACTIVE, name)
        try:
            rs = os.lstat(rec)
        except OSError:
            continue
        if not NAME_RE.match(name) or not stat.S_ISREG(rs.st_mode) or rs.st_uid != 0 or (rs.st_mode & 0o022):
            log(f"skip record {name!r}: not a root-owned regular file"); continue
        try:
            r = json.load(open(rec))
            hd, ws, d = str(r["hd"]), str(r["ws"]), str(r["run_dir"])
        except (OSError, ValueError, KeyError, TypeError):
            log(f"skip record {name}: bad JSON"); continue
        why = []
        if not HD_RE.match(hd): why.append("hd name")
        if not WS_RE.match(ws): why.append("ws name")
        if os.path.dirname(os.path.realpath(d)) != runs_real or os.path.realpath(d) != d.rstrip("/"): why.append("run_dir not directly under RUNS")
        if os.path.basename(d.rstrip("/")) + ".json" != name: why.append("record name != run_dir name")
        if why:
            log(f"skip record {name}: " + ", ".join(why)); continue
        e_d = real_dir(d, me)
        if e_d:
            log(f"skip record {name}: run_dir {e_d}"); continue
        e_hd = real_dir(hd, bp)   # may be "missing" (then nothing to copy or delete)
        e_ws = real_dir(ws, ag)
        if e_hd not in (None, "missing") or e_ws not in (None, "missing"):
            log(f"skip record {name}: hd {e_hd or 'ok'}, ws {e_ws or 'ok'}"); continue
        copied = []
        if e_hd is None:
            for f in ("proxy_usage.jsonl", "proxy_hold.json"):
                src = os.path.join(hd, f)
                try:
                    fs = os.lstat(src)
                except OSError:
                    continue
                if stat.S_ISREG(fs.st_mode) and fs.st_uid == bp:
                    out = subprocess.run(["sudo", "-n", "cat", "--", src], capture_output=True).stdout
                    open(os.path.join(d, f), "wb").write(out); copied.append(f)
                else:
                    log(f"record {name}: {f} is not a regular benchproxy file; not copied")
        json.dump({"status": "killed", "reason": "runner did not finish; marked by the next run from a root-owned record",
                   "logs_copied": copied}, open(os.path.join(d, "killed.json"), "w"), indent=1)
        pe = os.path.join(d, "run_env.json")
        if os.path.isfile(pe) and not os.path.islink(pe):
            try:
                e = json.load(open(pe)); e["status"] = "killed"; json.dump(e, open(pe, "w"), indent=1)
            except ValueError:
                pass
        if e_ws is None: sudo("rm", "-rf", "--", ws)
        if e_hd is None: sudo("rm", "-rf", "--", hd)
        sudo("rm", "-f", "--", rec)
        log(f"run {name[:-5]} marked killed; logs copied: {copied or 'none'}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1], sys.argv[2]))
