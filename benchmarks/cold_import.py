"""Cold import timing for core package."""

from __future__ import annotations

import statistics
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
    cmd = [
        sys.executable,
        "-c",
        "import time; t=time.perf_counter(); import agentic_harness; "
        "print(f'{(time.perf_counter()-t)*1000:.3f}')",
    ]
    env = {**dict(__import__("os").environ), "PYTHONPATH": str(ROOT / "src")}
    times = []
    for _ in range(7):
        p = subprocess.run(cmd, cwd=str(ROOT), env=env, check=True, capture_output=True, text=True)
        times.append(float(p.stdout.strip()))
    print(f"cold_import_ms_median={statistics.median(times):.3f}")
    print(f"cold_import_ms_samples={times}")


if __name__ == "__main__":
    main()
