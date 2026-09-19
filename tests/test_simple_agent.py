from agentic_harness.messages import Message, ToolCall
from agentic_harness.model import ModelResponse
from agentic_harness.result import RunStatus
from agentic_harness.simple import Agent, function_tool


class HelloModel:
    def complete(self, messages, tools=None) -> ModelResponse:
        return ModelResponse(message=Message(role="assistant", content="hi"))


class ToolThenDone:
    def __init__(self) -> None:
        self.i = 0

    def complete(self, messages, tools=None) -> ModelResponse:
        self.i += 1
        if self.i == 1:
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[ToolCall(id="1", name="echo", arguments={"text": "x"})],
                )
            )
        return ModelResponse(message=Message(role="assistant", content="done"))


def test_simple_agent_no_tools():
    r = Agent(model=HelloModel()).run("hey")
    assert r.status == RunStatus.succeeded
    assert r.messages[-1].content == "hi"


def test_simple_agent_allowlists_tools():
    echo = function_tool(
        "echo",
        "echo",
        {"type": "object", "properties": {"text": {"type": "string"}}, "required": ["text"]},
        lambda args, ctx: args["text"],
    )
    r = Agent(model=ToolThenDone(), tools=[echo]).run("go")
    assert r.status == RunStatus.succeeded
    assert any(m.role == "tool" and m.content == "x" for m in r.messages)
