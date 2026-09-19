"""Alice first demo: one fake model + one user tool + loop. No network."""

from __future__ import annotations

from agentic_harness.gate import Allow, Deny
from agentic_harness.messages import Message, ToolCall
from agentic_harness.model import ModelResponse
from agentic_harness.plugin import Registry
from agentic_harness.runner import RunConfig, Runner
from agentic_harness.tool import FunctionTool


class DemoModel:
    def __init__(self) -> None:
        self._i = 0

    def complete(self, messages, tools=None) -> ModelResponse:
        self._i += 1
        if self._i == 1:
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[
                        ToolCall(id="c1", name="echo", arguments={"text": "hello"})
                    ],
                )
            )
        return ModelResponse(message=Message(role="assistant", content="said hello"))


def main() -> None:
    reg = Registry()
    reg.register_model(DemoModel())
    reg.register_tool(
        FunctionTool(
            name="echo",
            description="Return text",
            parameters={
                "type": "object",
                "properties": {"text": {"type": "string"}},
                "required": ["text"],
            },
            handler=lambda args, ctx: str(args.get("text", "")),
        )
    )

    def allow_echo(name, args, ctx):
        return Allow() if name == "echo" else Deny(reason="not allowlisted")

    result = Runner(reg).run(
        "please echo hello",
        config=RunConfig(can_use_tool=allow_echo, max_steps=4),
    )
    print(result.status.value)
    for m in result.messages:
        print(f"{m.role}: {m.content!r}")


if __name__ == "__main__":
    main()
