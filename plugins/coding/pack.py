from __future__ import annotations

from pathlib import Path

from agentic_harness.tool import FunctionTool

from .filesystem import make_filesystem_tools
from .git_status import make_git_status_tool
from .patch import make_patch_tool
from .search import make_search_tool


def coding_tools(root: Path, *, include_shell: bool = False) -> list[FunctionTool]:
    """Default coding pack. Shell is opt-in."""
    tools: list[FunctionTool] = []
    tools.extend(make_filesystem_tools(root))
    tools.append(make_search_tool(root))
    tools.append(make_patch_tool(root))
    tools.append(make_git_status_tool(root))
    if include_shell:
        from .shell import make_shell_tool

        tools.append(make_shell_tool(root))
    return tools
