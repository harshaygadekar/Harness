from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Callable, Protocol

from .tool import FunctionTool, Tool, ToolSpec


Disposer = Callable[[], None]


class Plugin(Protocol):
    name: str
    version: str

    def register(self, registry: "Registry") -> Disposer:
        ...


@dataclass
class Registry:
    """In-process tool + model registration. Tools are only run via Runner."""

    _tools: dict[str, Tool] = field(default_factory=dict)
    _disposers: list[Disposer] = field(default_factory=list)
    model: Any | None = None

    def register_tool(self, tool: Tool) -> Disposer:
        name = tool.name
        if name in self._tools:
            raise ValueError(f"tool already registered: {name}")
        self._tools[name] = tool

        def dispose() -> None:
            self._tools.pop(name, None)

        self._disposers.append(dispose)
        return dispose

    def register_model(self, model: Any) -> Disposer:
        self.model = model

        def dispose() -> None:
            if self.model is model:
                self.model = None

        self._disposers.append(dispose)
        return dispose

    def add_plugin(self, plugin: Plugin) -> Disposer:
        disposer = plugin.register(self)
        self._disposers.append(disposer)
        return disposer

    def get_tool(self, name: str) -> Tool | None:
        return self._tools.get(name)

    def list_specs(self) -> list[ToolSpec]:
        out: list[ToolSpec] = []
        for t in self._tools.values():
            if isinstance(t, FunctionTool):
                out.append(t.spec())
            else:
                out.append(
                    ToolSpec(
                        name=t.name,
                        description=t.description,
                        parameters=t.parameters,
                    )
                )
        return out

    def dispose_all(self) -> None:
        while self._disposers:
            d = self._disposers.pop()
            d()
