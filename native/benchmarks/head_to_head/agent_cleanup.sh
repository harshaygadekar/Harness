#!/bin/bash
# Kill every process of agent user $1 with SIGKILL, in a loop (max 10 tries, 0.5 s apart) until
# `pgrep -u $1` is empty. Prints the number left (0 = clean). Exit 0 if clean, 1 otherwise (caller aborts).
set -u; AU=$1; n=-1
for i in $(seq 1 10); do
  n=$(pgrep -u "$AU" | wc -l); [ "$n" -eq 0 ] && break
  sudo -n pkill -KILL -u "$AU" 2>/dev/null; sleep 0.5
done
n=$(pgrep -u "$AU" | wc -l); echo "$n"; [ "$n" -eq 0 ]
