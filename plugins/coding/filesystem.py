from __future__ import annotations

from pathlib import Path

from agentic_harness.simple import function_tool
from agentic_harness.tool import FunctionTool

from .workspace import Workspace


def make_filesystem_tools(root: Path, *, max_read: int = 8000) -> list[FunctionTool]:
    ws = Workspace(root)

    def list_files(args: dict, ctx) -> str:
        try:
            target = ws.resolve(str(args.get("path", ".")))
        except PermissionError as exc:
            return f"ERROR: {exc}"
        if not target.is_dir():
            return f"ERROR: not a directory: {args.get('path')}"
        names = sorted(
            p.name + ("/" if p.is_dir() else "") for p in target.iterdir()
        )
        return "\n".join(names) if names else "(empty)"

    def read_file(args: dict, ctx) -> str:
        try:
            target = ws.resolve(str(args.get("path", "")))
        except PermissionError as e:
            return f"ERROR: {e}"
        if not target.is_file():
            return f"ERROR: not a file: {args.get('path')}"
        data = target.read_text(encoding="utf-8", errors="replace")
        if len(data) > max_read:
            data = data[:max_read] + "\n...[truncated]"
        return data

    def write_file(args: dict, ctx) -> str:
        try:
            target = ws.resolve(str(args.get("path", "")))
        except PermissionError as e:
            return f"ERROR: {e}"
        content = str(args.get("content", ""))
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content, encoding="utf-8")
        return f"wrote {len(content)} bytes to {args.get('path')}"

    return [
        function_tool(
            "list_files",
            "List files under the workspace",
            {
                "type": "object",
                "properties": {"path": {"type": "string"}},
                "required": ["path"],
            },
            list_files,
        ),
        function_tool(
            "read_file",
            "Read a UTF-8 text file under the workspace",
            {
                "type": "object",
                "properties": {"path": {"type": "string"}},
                "required": ["path"],
            },
            read_file,
        ),
        function_tool(
            "write_file",
            "Write a UTF-8 text file under the workspace",
            {
                "type": "object",
                "properties": {
                    "path": {"type": "string"},
                    "content": {"type": "string"},
                },
                "required": ["path", "content"],
            },
            write_file,
        ),
    ]
