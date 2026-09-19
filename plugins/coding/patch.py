from __future__ import annotations

from pathlib import Path

from agentic_harness.simple import function_tool
from agentic_harness.tool import FunctionTool

from .workspace import Workspace


def make_patch_tool(root: Path) -> FunctionTool:
    ws = Workspace(root)

    def apply_patch(args: dict, ctx) -> str:
        try:
            target = ws.resolve(str(args.get("path", "")))
        except PermissionError as e:
            return f"ERROR: {e}"
        old = str(args.get("old", ""))
        new = str(args.get("new", ""))
        if not target.is_file():
            return f"ERROR: not a file: {args.get('path')}"
        text = target.read_text(encoding="utf-8")
        if old not in text:
            return "ERROR: old text not found"
        count = text.count(old)
        target.write_text(text.replace(old, new, 1), encoding="utf-8")
        return f"patched {args.get('path')} ({count} occurrence(s) matched, 1 replaced)"

    return function_tool(
        "apply_patch",
        "Replace the first occurrence of old with new in a workspace file",
        {
            "type": "object",
            "properties": {
                "path": {"type": "string"},
                "old": {"type": "string"},
                "new": {"type": "string"},
            },
            "required": ["path", "old", "new"],
        },
        apply_patch,
    )
