"""Coding-agent plugins (host-side). Not core."""

from .pack import coding_tools
from .filesystem import make_filesystem_tools
from .search import make_search_tool
from .patch import make_patch_tool
from .git_status import make_git_status_tool
from .shell import make_shell_tool

__all__ = [
    "coding_tools",
    "make_filesystem_tools",
    "make_search_tool",
    "make_patch_tool",
    "make_git_status_tool",
    "make_shell_tool",
]
