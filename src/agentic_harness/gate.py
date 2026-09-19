from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Callable


@dataclass(frozen=True)
class Allow:
    pass


@dataclass(frozen=True)
class Deny:
    reason: str = "denied by policy"


Decision = Allow | Deny

# Host sets this. Core default: deny every tool (Jessie C1).
CanUseTool = Callable[[str, dict[str, Any], Any], Decision]


def deny_all(tool_name: str, args: dict[str, Any], ctx: Any) -> Decision:
    return Deny(reason=f"tool '{tool_name}' not allowed (deny-all default)")
