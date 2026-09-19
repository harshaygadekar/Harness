"""B3 — register + dispose N empty plugins (P2-6 SHOULD)."""

from __future__ import annotations

import argparse
import statistics
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "src"))

from agentic_harness.plugin import Registry


class EmptyPlugin:
    def __init__(self, name: str) -> None:
        self.name = name
        self.version = "0.0.0"

    def register(self, registry: Registry):
        # no tools — just lifecycle
        def dispose() -> None:
            return None

        return dispose


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--n", type=int, default=100)
    args = p.parse_args()

    def once() -> None:
        reg = Registry()
        disposers = []
        for i in range(args.n):
            disposers.append(reg.add_plugin(EmptyPlugin(f"p{i}")))
        for d in reversed(disposers):
            d()
        reg.dispose_all()

    once()
    samples = []
    for _ in range(21):
        t0 = time.perf_counter()
        once()
        samples.append((time.perf_counter() - t0) * 1000)
    med = statistics.median(samples)
    print(f"n={args.n}")
    print(f"register_dispose_ms_median={med:.3f}")
    print(f"p2_6_should_pass={med <= 5.0}")


if __name__ == "__main__":
    main()
