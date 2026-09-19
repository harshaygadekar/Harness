from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Callable, Protocol, runtime_checkable


@dataclass(frozen=True)
class ToolSpec:
    name: str
    description: str
    parameters: dict[str, Any]  # JSON Schema object


@dataclass(frozen=True)
class ToolResult:
    content: str
    ok: bool = True


@runtime_checkable
class Tool(Protocol):
    name: str
    description: str
    parameters: dict[str, Any]

    def call(self, args: dict[str, Any], ctx: Any) -> ToolResult:
        ...


@dataclass
class FunctionTool:
    """Host-supplied in-process tool. Not a built-in product tool."""

    name: str
    description: str
    parameters: dict[str, Any]
    handler: Callable[[dict[str, Any], Any], str]

    def call(self, args: dict[str, Any], ctx: Any) -> ToolResult:
        return ToolResult(content=self.handler(args, ctx), ok=True)

    def spec(self) -> ToolSpec:
        return ToolSpec(
            name=self.name,
            description=self.description,
            parameters=self.parameters,
        )
