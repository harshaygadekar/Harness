from __future__ import annotations

from pathlib import Path

from agentic_harness.simple import function_tool
from agentic_harness.tool import FunctionTool

from .workspace import Workspace


def make_search_tool(root: Path, *, max_hits: int = 50) -> FunctionTool:
    ws = Workspace(root)

    def search(args: dict, ctx) -> str:
        query = str(args.get("query", ""))
        if not query:
            return "ERROR: empty query"
        hits: list[str] = []
        for path in ws.root.rglob("*"):
            if not path.is_file():
                continue
            if path.suffix.lower() in {".pyc", ".png", ".jpg", ".gif", ".zip"}:
                continue
            try:
                text = path.read_text(encoding="utf-8", errors="ignore")
            except OSError:
                continue
            if query not in text:
                continue
            rel = path.relative_to(ws.root)
            for i, line in enumerate(text.splitlines(), 1):
                if query in line:
                    hits.append(f"{rel}:{i}:{line.strip()[:120]}")
                    if len(hits) >= max_hits:
                        return "\n".join(hits)
        return "\n".join(hits) if hits else "(no matches)"

    return function_tool(
        "search",
        "Search text files under the workspace for a substring",
        {
            "type": "object",
            "properties": {"query": {"type": "string"}},
            "required": ["query"],
        },
        search,
    )
