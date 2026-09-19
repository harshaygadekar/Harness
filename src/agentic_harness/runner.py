from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Callable

from .gate import Allow, CanUseTool, Deny, deny_all
from .messages import Message, ToolCall, Usage
from .plugin import Registry
from .result import RunResult, RunStatus, SafeError
from .sanitize import safe_tool_error, sanitize_text


CancelCheck = Callable[[], bool]


@dataclass
class RunContext:
    """Opaque host context passed into tools. Not a credential store."""

    data: dict[str, Any] = field(default_factory=dict)
    step: int = 0


@dataclass
class RunConfig:
    max_steps: int = 8
    can_use_tool: CanUseTool = deny_all
    cancel: CancelCheck | None = None
    system: str | None = None


class Runner:
    """ADR-001 ReAct loop. Only path that executes tools (Jessie C2)."""

    def __init__(self, registry: Registry) -> None:
        self._registry = registry

    def run(
        self,
        user_input: str,
        *,
        config: RunConfig | None = None,
        messages: list[Message] | None = None,
    ) -> RunResult:
        cfg = config or RunConfig()
        model = self._registry.model
        if model is None:
            return RunResult(
                status=RunStatus.failed,
                error=SafeError(code="NO_MODEL", message="no model registered"),
            )

        history: list[Message] = list(messages or [])
        if cfg.system and not any(m.role == "system" for m in history):
            history.insert(0, Message(role="system", content=cfg.system))
        history.append(Message(role="user", content=user_input))

        usage_total = Usage()
        ctx = RunContext()

        for step in range(cfg.max_steps):
            if cfg.cancel and cfg.cancel():
                return RunResult(
                    status=RunStatus.cancelled,
                    messages=history,
                    usage=usage_total,
                    steps=step,
                )

            ctx.step = step
            specs = self._registry.list_specs()
            response = model.complete(history, specs or None)
            assistant = response.message
            history.append(assistant)
            if response.usage:
                usage_total = Usage(
                    input_tokens=usage_total.input_tokens + response.usage.input_tokens,
                    output_tokens=usage_total.output_tokens + response.usage.output_tokens,
                )

            if not assistant.tool_calls:
                return RunResult(
                    status=RunStatus.succeeded,
                    messages=history,
                    usage=usage_total,
                    steps=step + 1,
                )

            # v0: sequential even if model returns many calls (ADR-001 / ADR-010).
            for call in assistant.tool_calls:
                if cfg.cancel and cfg.cancel():
                    return RunResult(
                        status=RunStatus.cancelled,
                        messages=history,
                        usage=usage_total,
                        steps=step + 1,
                    )
                history.append(self._run_one_tool(call, cfg, ctx))

        return RunResult(
            status=RunStatus.max_steps,
            messages=history,
            usage=usage_total,
            steps=cfg.max_steps,
            error=SafeError(code="MAX_STEPS", message="max_steps reached"),
        )

    def _run_one_tool(
        self,
        call: ToolCall,
        cfg: RunConfig,
        ctx: RunContext,
    ) -> Message:
        tool = self._registry.get_tool(call.name)
        if tool is None:
            return Message(
                role="tool",
                tool_call_id=call.id,
                name=call.name,
                content=sanitize_text(f"TOOL_FAILED: unknown tool '{call.name}'"),
            )

        decision = cfg.can_use_tool(call.name, call.arguments, ctx)
        if isinstance(decision, Deny) or not isinstance(decision, Allow):
            reason = decision.reason if isinstance(decision, Deny) else "denied"
            return Message(
                role="tool",
                tool_call_id=call.id,
                name=call.name,
                content=sanitize_text(f"DENIED: {reason}"),
            )

        # Gate passed — only Runner reaches tool.call (no public execute helper).
        try:
            result = tool.call(call.arguments, ctx)
            content = sanitize_text(result.content)
            if not result.ok:
                content = sanitize_text(f"TOOL_FAILED: {result.content}")
        except Exception as exc:  # noqa: BLE001 — must not leak secrets
            content = safe_tool_error(exc)

        return Message(
            role="tool",
            tool_call_id=call.id,
            name=call.name,
            content=content,
        )
