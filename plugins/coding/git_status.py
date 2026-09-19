from __future__ import annotations

import subprocess
from pathlib import Path

from agentic_harness.simple import function_tool
from agentic_harness.tool import FunctionTool

from .workspace import Workspace


def make_git_status_tool(root: Path) -> FunctionTool:
    ws = Workspace(root)

    def git_status(args: dict, ctx) -> str:
        try:
            proc = subprocess.run(
                ["git", "status", "--short"],
                cwd=str(ws.root),
                capture_output=True,
                text=True,
                timeout=10,
                check=False,
            )
        except (OSError, subprocess.TimeoutExpired) as exc:
            return f"ERROR: {type(exc).__name__}"
        out = (proc.stdout or "") + (proc.stderr or "")
        return out.strip() or "(clean)"

    return function_tool(
        "git_status",
        "Show git status --short for the workspace (read-only)",
        {"type": "object", "properties": {}},
        git_status,
    )
