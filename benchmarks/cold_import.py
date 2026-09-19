"""Cold import timing for core package (ADR-001 benchmark plan)."""

from __future__ import annotations

import subprocess
import sys
import time


def main() -> None:
    cmd = [
        sys.executable,
        "-c",
        "import time; t=time.perf_counter(); import agentic_harness; "
        "print(f'{(time.perf_counter()-t)*1000:.2f}')",
    ]
    env = {"PYTHONPATH": "src"}
    # warm subprocess start once
    subprocess.run(cmd, env={**dict(**{k: v for k, v in __import__('os').environ.items()}), **{"PYTHONPATH": "src"}}, cwd=".", check=False, capture_output=True)
    times = []
    for _ in range(5):
        p = subprocess.run(
            cmd,
            env={**dict(__import__("os").environ), "PYTHONPATH": "src"},
            cwd=".",
            check=True,
            capture_output=True,
            text=True,
        )
        times.append(float(p.stdout.strip()))
    print(f"cold_import_ms_median={sorted(times)[len(times)//2]:.2f}")
    print(f"cold_import_ms_samples={times}")
    # dep check: core must not import provider SDKs
    ban = ["openai", "anthropic", "google.generativeai", "litellm"]
    code = (
        "import sys; "
        + "; ".join(f"assert {b!r} not in sys.modules" for b in ban)
        + "; import agentic_harness; "
        + "; ".join(f"assert {b!r} not in sys.modules" for b in ban)
        + "; print('deps_ok')"
    )
    p2 = subprocess.run(
        [sys.executable, "-c", code],
        env={**dict(__import__("os").environ), "PYTHONPATH": "src"},
        cwd=".",
        check=True,
        capture_output=True,
        text=True,
    )
    print(p2.stdout.strip())


if __name__ == "__main__":
    main()
