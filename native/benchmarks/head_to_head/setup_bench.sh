#!/bin/bash
# Set up the bench box after a rebuild. Run as root.
# This script does not touch the Gemini key. It does not read, ask for, print or write keys.env.
# It does not delete paths. It is safe to run again.
# usage: sudo setup_bench.sh [repo_dir]
set -euo pipefail

die() { echo "FAIL: $*" >&2; exit 1; }
ok() { echo "OK    $*"; }

# Check functions. Tests source this file with SETUP_BENCH_LIB=1 and call them on temp paths.
# The dir must be a real dir (not a symlink), owner uid 0, mode 755.
check_root_dir() {
  [ -L "$1" ] && die "$1 is a symlink"
  [ -d "$1" ] || die "$1 is not a directory"
  [ "$(stat -c '%u %a' -- "$1")" = "0 755" ] || die "$1 must be owner uid 0 and mode 755 (it is: $(stat -c 'uid %u mode %a' -- "$1"))"
}
# Make the dir as root if it does not exist. mkdir without -p does not follow a symlink at the path.
ensure_root_dir() {
  if [ ! -e "$1" ] && [ ! -L "$1" ]; then
    mkdir -m 755 -- "$1" || die "cannot create $1"
  fi
  check_root_dir "$1"
}
# The file must be a regular file (not a symlink) with owner uid 0.
check_root_file() {
  [ -L "$1" ] && die "$1 is a symlink"
  [ -f "$1" ] || die "$1 is not a regular file"
  [ "$(stat -c '%u' -- "$1")" = 0 ] || die "$1 must be owner uid 0 (it is: uid $(stat -c '%u' -- "$1"))"
}
# runuser itself exits 1 on its own errors (no such user, exec failed), the same code as "test -w" false.
# So the probe maps the result to 10 (writable) or 11 (not writable). Any other code is an error, and the script stops.
agent_cannot_write() {
  local u=${AGENT_USER_CHECK:-benchagent} rc=0
  [ -e "$1" ] || die "write check: $1 does not exist"
  runuser -u "$u" -- /bin/sh -c 'if test -w "$1"; then exit 10; else exit 11; fi' sh "$1" || rc=$?
  case $rc in
    10) die "$u can write $1" ;;
    11) ;;
    *) die "write check for $1 as $u failed (exit $rc)" ;;
  esac
}
if [ "${SETUP_BENCH_LIB:-}" = 1 ]; then return 0 2>/dev/null || exit 0; fi

[ "$(id -u)" -eq 0 ] || { echo "FAIL: run this script as root (sudo)." >&2; exit 1; }
REPO=${1:-$(cd "$(dirname "$0")/../../.." && pwd)}

# Expected values. Change these only after review.
PIN_COMMIT=2246092e02cb1323d44e030464802c4985072104
REL=native/benchmarks/tools/oai_stream_proxy
PROXY_SHA256=a9af33f4712eca8f071e0a0d62e9ff44142fb67ea06da7e1a2ec2d0f9bed6285
START_SHA256=d626ee838027342ac543546e6f20fbdf9e92e16662256d46258d7aaf7bd03d09
# time 1.9-0.2 amd64. sha256 from Debian trixie main Packages.xz, checked with sqv against InRelease on 2026-11-02.
TIME_DEB_URL=http://deb.debian.org/debian/pool/main/t/time/time_1.9-0.2_amd64.deb
TIME_DEB_SHA256=10257c6fe888e6578550b1f20705d282dd630b76f8df0c006aec3031fefe5e23
# fx source: docs/benchmarks/size-metrics.md (URL pattern of the official install script).
FX_VERSION=v0.0.13
FX_URL=https://releases.fx.sh/$FX_VERSION/fx-linux-x86_64.tar.gz
FX_DIR=/tmp/fxbin
FX_SHA256=00def23f9c68ef538d7d967394c3491910793461884fad0b983cb74011b9120c

# 1. Users. No sudo rights. No password.
for u in benchagent benchproxy; do
  if id "$u" >/dev/null 2>&1; then ok "user $u exists (uid $(id -u "$u"))"
  else useradd --create-home --shell /bin/bash --user-group "$u"; ok "user $u created (uid $(id -u "$u"))"; fi
done
id -nG benchagent | tr ' ' '\n' | grep -qxE 'sudo|wheel|admin' && die "benchagent is in a sudo group"
id -nG benchproxy | tr ' ' '\n' | grep -qxE 'sudo|wheel|admin' && die "benchproxy is in a sudo group"
runuser -u benchagent -- true || die "runuser -u benchagent -- true failed; write checks cannot run"
ok "runuser works for benchagent"

# 2. Proxy home: owner benchproxy, mode 700.
install -d -o benchproxy -g benchproxy -m 700 /home/benchproxy
chown benchproxy:benchproxy /home/benchproxy; chmod 700 /home/benchproxy
ok "/home/benchproxy owner benchproxy mode 700"

# 3. GNU time.
if [ ! -x /usr/bin/time ]; then
  if apt-get update -q >/dev/null 2>&1 && DEBIAN_FRONTEND=noninteractive apt-get install -y -q time >/dev/null 2>&1; then
    ok "GNU time installed with apt-get"
  else
    # Fallback: apt-get failed (on this box the apt mirror path can give HTTP 500).
    # Get the same .deb with curl. Check the sha256 from the signed Debian trixie index.
    echo "WARN  apt-get failed; using the Debian pool .deb with a fixed sha256"
    TMPD=$(mktemp -d)
    curl -fsSL -o "$TMPD/time.deb" "$TIME_DEB_URL" || die "download of $TIME_DEB_URL failed"
    [ "$(sha256sum < "$TMPD/time.deb" | cut -c1-64)" = "$TIME_DEB_SHA256" ] || die "time .deb has the wrong sha256"
    dpkg -i "$TMPD/time.deb" >/dev/null || die "dpkg -i time.deb failed"
    rm -f "$TMPD/time.deb"; rmdir "$TMPD"
    ok "GNU time installed from the checked .deb"
  fi
fi
/usr/bin/time --version 2>&1 | grep -q 'GNU' || die "/usr/bin/time is not GNU time"
ok "GNU time: $(/usr/bin/time --version 2>&1 | head -1)"

# 4. Proxy pin dir from git blobs. Owner root, so benchagent and benchproxy cannot write it.
PIN=/opt/benchproxy/$PIN_COMMIT
git -c safe.directory="*" -C "$REPO" cat-file -e "$PIN_COMMIT^{commit}" 2>/dev/null || die "commit $PIN_COMMIT is not in $REPO"
install -d -o root -g root -m 755 /opt/benchproxy "$PIN"
TMPX=$(mktemp -d)
git -c safe.directory="*" -C "$REPO" show "$PIN_COMMIT:$REL/proxy.py" > "$TMPX/proxy.py"
git -c safe.directory="*" -C "$REPO" show "$PIN_COMMIT:$REL/start_isolated.sh" > "$TMPX/start_isolated.sh"
[ "$(sha256sum < "$TMPX/proxy.py" | cut -c1-64)" = "$PROXY_SHA256" ] || die "proxy.py from git has the wrong sha256"
[ "$(sha256sum < "$TMPX/start_isolated.sh" | cut -c1-64)" = "$START_SHA256" ] || die "start_isolated.sh from git has the wrong sha256"
install -o root -g root -m 644 "$TMPX/proxy.py" "$PIN/proxy.py"
install -o root -g root -m 755 "$TMPX/start_isolated.sh" "$PIN/start_isolated.sh"
rm -f "$TMPX/proxy.py" "$TMPX/start_isolated.sh"; rmdir "$TMPX"
[ "$(sha256sum < "$PIN/proxy.py" | cut -c1-64)" = "$PROXY_SHA256" ] || die "installed proxy.py has the wrong sha256"
[ "$(sha256sum < "$PIN/start_isolated.sh" | cut -c1-64)" = "$START_SHA256" ] || die "installed start_isolated.sh has the wrong sha256"
for f in "$PIN" "$PIN/proxy.py" "$PIN/start_isolated.sh"; do agent_cannot_write "$f"; done
ok "pin dir $PIN: proxy.py and start_isolated.sh sha256 match, owner root, benchagent cannot write"

# 5. fx. Download the release archive and check the binary sha256 before install.
ensure_root_dir "$FX_DIR"
ok "$FX_DIR is a real dir, owner root, mode 755"
if [ -e "$FX_DIR/fx" ] || [ -L "$FX_DIR/fx" ]; then check_root_file "$FX_DIR/fx"; fi
if [ -x "$FX_DIR/fx" ] && [ "$(sha256sum < "$FX_DIR/fx" | cut -c1-64)" = "$FX_SHA256" ]; then
  ok "fx already present with the right sha256"
else
  command -v curl >/dev/null || die "curl is missing; cannot get fx"
  TMPF=$(mktemp -d)
  curl -fsSL --proto '=https' -o "$TMPF/fx.tar.gz" "$FX_URL" || die "download of $FX_URL failed"
  tar -xzf "$TMPF/fx.tar.gz" -C "$TMPF" fx 2>/dev/null || tar -xzf "$TMPF/fx.tar.gz" -C "$TMPF"
  FXF=$(find "$TMPF" -type f -name fx | head -1); [ -n "$FXF" ] || die "no fx binary in the archive"
  GOT=$(sha256sum < "$FXF" | cut -c1-64)
  [ "$GOT" = "$FX_SHA256" ] || die "fx binary sha256 is $GOT, expected $FX_SHA256. Do not use it."
  check_root_dir "$FX_DIR"
  install -o root -g root -m 755 "$FXF" "$FX_DIR/fx"
  rm -f "$TMPF/fx.tar.gz" "$FXF"; find "$TMPF" -depth -mindepth 1 -user root -delete; rmdir "$TMPF"
fi
check_root_dir "$FX_DIR"
check_root_file "$FX_DIR/fx"
[ "$(sha256sum < "$FX_DIR/fx" | cut -c1-64)" = "$FX_SHA256" ] || die "installed fx has the wrong sha256"
agent_cannot_write "$FX_DIR"
agent_cannot_write "$FX_DIR/fx"
ok "fx $FX_VERSION at $FX_DIR/fx, sha256 match, owner root"

echo "DONE. Next step (separate, by the operator): place /home/benchproxy/keys.env with one line GEMINI_API_KEY=..., owner benchproxy, mode 600."
