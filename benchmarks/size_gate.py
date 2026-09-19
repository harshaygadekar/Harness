"""B5 — core LOC ≤ 800; dependencies == [] (P2-2 / P2-3 MUST)."""

from __future__ import annotations

import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover
    import tomli as tomllib  # type: ignore

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src" / "agentic_harness"


def core_loc() -> int:
    total = 0
    for path in SRC.rglob("*.py"):
        if "plugins" in path.parts:
            continue
        total += len(path.read_text(encoding="utf-8").splitlines())
    return total


def main() -> None:
    loc = core_loc()
    data = tomllib.loads((ROOT / "pyproject.toml").read_text(encoding="utf-8"))
    deps = data.get("project", {}).get("dependencies", [])
    print(f"core_loc={loc}")
    print(f"deps={deps}")
    print(f"core_loc_ok={loc <= 800}")
    print(f"deps_ok={deps == []}")
    if loc > 800 or deps != []:
        raise SystemExit(1)
    print("size_gate_pass")


if __name__ == "__main__":
    main()
