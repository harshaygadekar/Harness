"""Agent with one allowlisted tool. Core unchanged."""

from __future__ import annotations

from agentic_harness.messages import Message, ToolCall
from agentic_harness.model import ModelResponse
from agentic_harness.simple import Agent, function_tool


class CalcModel:
    def __init__(self) -> None:
        self._i = 0

    def complete(self, messages, tools=None) -> ModelResponse:
        self._i += 1
        if self._i == 1:
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[
                        ToolCall(
                            id="1",
                            name="add",
                            arguments={"a": 2, "b": 3},
                        )
                    ],
                )
            )
        return ModelResponse(
            message=Message(role="assistant", content="5")
        )


def main() -> None:
    add = function_tool(
        name="add",
        description="Add two numbers",
        parameters={
            "type": "object",
            "properties": {
                "a": {"type": "number"},
                "b": {"type": "number"},
            },
            "required": ["a", "b"],
        },
        handler=lambda args, ctx: str(float(args["a"]) + float(args["b"])),
    )
    agent = Agent(model=CalcModel(), tools=[add])
    result = agent.run("Calculate 2+3")
    print(result.status.value)
    print(result.messages[-1].content)


if __name__ == "__main__":
    main()
