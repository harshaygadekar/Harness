"""Thin DX layer above core. Not new primitives — wraps Registry + Runner.

Keeps ADR-001 core unchanged. Hosts that want the short Agent() API import here.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Iterable, Sequence

from agentic_harness.gate import Allow, Deny
from agentic_harness.model import ModelClient
from agentic_harness.plugin import Registry
from agentic_harness.result import RunResult
from agentic_harness.runner import RunConfig, Runner
from agentic_harness.tool import FunctionTool, Tool


def _as_tool(item: Tool | FunctionTool | Any) -> Tool:
    if isinstance(item, FunctionTool) or hasattr(item, "call"):
        return item  # type: ignore[return-value]
    raise TypeError(f"not a Tool: {type(item)!r}")


def allow_listed(names: set[str]):
    def _gate(name: str, args: dict[str, Any], ctx: Any):
        if name in names:
            return Allow()
        return Deny(reason=f"tool '{name}' not in agent allowlist")

    return _gate


@dataclass
class Agent:
    """One-file style façade: model + optional tools + run(prompt)."""

    model: ModelClient
    tools: Sequence[Tool | FunctionTool] = field(default_factory=tuple)
    system: str | None = None
    max_steps: int = 8

    def run(self, prompt: str) -> RunResult:
        reg = Registry()
        reg.register_model(self.model)
        names: set[str] = set()
        for t in self.tools:
            tool = _as_tool(t)
            reg.register_tool(tool)
            names.add(tool.name)
        # No tools → deny-all stays correct. With tools → allow only those names.
        gate = allow_listed(names) if names else None
        cfg = RunConfig(
            max_steps=self.max_steps,
            can_use_tool=gate if gate is not None else RunConfig().can_use_tool,
            system=self.system,
        )
        return Runner(reg).run(prompt, config=cfg)


def function_tool(
    name: str,
    description: str,
    parameters: dict[str, Any],
    handler,
) -> FunctionTool:
    """Helper so examples stay short."""
    return FunctionTool(
        name=name,
        description=description,
        parameters=parameters,
        handler=handler,
    )
