"""Optional shell tool. Host must Allow it. Not registered by default pack."""

from __future__ import annotations

import subprocess
from pathlib import Path

from agentic_harness.simple import function_tool
from agentic_harness.tool import FunctionTool

from .workspace import Workspace


def make_shell_tool(root: Path, *, timeout_s: float = 15.0) -> FunctionTool:
    ws = Workspace(root)

    def run_shell(args: dict, ctx) -> str:
        cmd = str(args.get("command", "")).strip()
        if not cmd:
            return "ERROR: empty command"
        # Soft deny of obvious escape attempts; Jessie gate is still required.
        banned = ("rm -rf /", "sudo ", ":(){", "mkfs")
        if any(b in cmd for b in banned):
            return "ERROR: command blocked by soft deny list"
        try:
            proc = subprocess.run(
                cmd,
                shell=True,
                cwd=str(ws.root),
                capture_output=True,
                text=True,
                timeout=timeout_s,
                check=False,
            )
        except subprocess.TimeoutExpired:
            return "ERROR: timeout"
        out = (proc.stdout or "")[-4000:]
        err = (proc.stderr or "")[-2000:]
        return f"exit={proc.returncode}\n{out}{err}"

    return function_tool(
        "run_shell",
        "Run a shell command in the workspace (dangerous; must be Allow-listed)",
        {
            "type": "object",
            "properties": {"command": {"type": "string"}},
            "required": ["command"],
        },
        run_shell,
    )
