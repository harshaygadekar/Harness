from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from typing import Any

from .messages import Message, Usage


class RunStatus(str, Enum):
    succeeded = "succeeded"
    failed = "failed"
    cancelled = "cancelled"
    max_steps = "max_steps"


@dataclass(frozen=True)
class SafeError:
    code: str
    message: str


@dataclass
class RunResult:
    status: RunStatus
    messages: list[Message] = field(default_factory=list)
    usage: Usage | None = None
    error: SafeError | None = None
    steps: int = 0
