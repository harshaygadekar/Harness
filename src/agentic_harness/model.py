from __future__ import annotations

from dataclasses import dataclass, field
from typing import Protocol, runtime_checkable

from .messages import Message, ToolCall, Usage
from .tool import ToolSpec


@dataclass
class ModelResponse:
    message: Message
    usage: Usage | None = None


@runtime_checkable
class ModelClient(Protocol):
    def complete(
        self,
        messages: list[Message],
        tools: list[ToolSpec] | None = None,
    ) -> ModelResponse:
        """One model step. May return tool_calls on the assistant message."""
