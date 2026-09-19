#!/usr/bin/env python3
"""Minimal plugin pack checks. Exit 0 on pass."""

from __future__ import annotations

import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "src"))
sys.path.insert(0, str(ROOT))

from plugins.coding import coding_tools


def _content(result) -> str:
    return result.content if hasattr(result, "content") else str(result)


def main() -> int:
    checks = []
    with tempfile.TemporaryDirectory() as td:
        root = Path(td)
        (root / "a.txt").write_text("hello world", encoding="utf-8")
        tools = {t.name: t for t in coding_tools(root)}
        for name in (
            "list_files",
            "read_file",
            "write_file",
            "search",
            "apply_patch",
            "git_status",
        ):
            checks.append(("has_" + name, name in tools))
        out = _content(tools["read_file"].call({"path": "../etc/passwd"}, None))
        checks.append(("escape_blocked", "ERROR" in out))
        out2 = _content(tools["search"].call({"query": "hello"}, None))
        checks.append(("search_hit", "hello" in out2))
        out3 = _content(
            tools["apply_patch"].call(
                {"path": "a.txt", "old": "hello", "new": "hi"}, None
            )
        )
        checks.append(("patch_ok", "patched" in out3))
        text = (root / "a.txt").read_text(encoding="utf-8")
        checks.append(("patch_applied", text.startswith("hi")))

    failed = [n for n, ok in checks if not ok]
    for n, ok in checks:
        print(("PASS" if ok else "FAIL"), n)
    if failed:
        print("FAILED", failed)
        return 1
    print("plugin_conformance_ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
