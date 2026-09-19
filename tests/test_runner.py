from __future__ import annotations

import agentic_harness as ah
from agentic_harness.gate import Allow, Deny
from agentic_harness.messages import Message, ToolCall
from agentic_harness.model import ModelResponse
from agentic_harness.plugin import Registry
from agentic_harness.result import RunStatus
from agentic_harness.runner import RunConfig, Runner
from agentic_harness.sanitize import sanitize_text
from agentic_harness.tool import FunctionTool


class ScriptedModel:
    def __init__(self, responses: list[ModelResponse]) -> None:
        self._responses = list(responses)

    def complete(self, messages, tools=None) -> ModelResponse:
        if not self._responses:
            return ModelResponse(message=Message(role="assistant", content="done"))
        return self._responses.pop(0)


def _echo_tool() -> FunctionTool:
    return FunctionTool(
        name="echo",
        description="echo text",
        parameters={
            "type": "object",
            "properties": {"text": {"type": "string"}},
            "required": ["text"],
        },
        handler=lambda args, ctx: args.get("text", ""),
    )


def test_deny_all_default_never_runs_tool():
    ran = {"n": 0}

    def handler(args, ctx):
        ran["n"] += 1
        return "should-not-run"

    tool = FunctionTool(
        name="echo",
        description="echo",
        parameters={"type": "object", "properties": {}},
        handler=handler,
    )
    reg = Registry()
    reg.register_tool(tool)
    reg.register_model(
        ScriptedModel(
            [
                ModelResponse(
                    message=Message(
                        role="assistant",
                        tool_calls=[
                            ToolCall(id="1", name="echo", arguments={"text": "x"})
                        ],
                    )
                ),
                ModelResponse(message=Message(role="assistant", content="ok")),
            ]
        )
    )
    Runner(reg).run("hi")
    assert ran["n"] == 0


def test_allow_list_runs_tool():
    reg = Registry()
    reg.register_tool(_echo_tool())
    reg.register_model(
        ScriptedModel(
            [
                ModelResponse(
                    message=Message(
                        role="assistant",
                        tool_calls=[
                            ToolCall(id="1", name="echo", arguments={"text": "pong"})
                        ],
                    )
                ),
                ModelResponse(message=Message(role="assistant", content="done")),
            ]
        )
    )

    def allow_echo(name, args, ctx):
        return Allow() if name == "echo" else Deny()

    result = Runner(reg).run("hi", config=RunConfig(can_use_tool=allow_echo))
    assert result.status == RunStatus.succeeded
    tool_msgs = [m for m in result.messages if m.role == "tool"]
    assert tool_msgs and tool_msgs[0].content == "pong"


def test_secret_scrubbed_from_tool_error():
    def boom(args, ctx):
        raise RuntimeError("key=sk-test-SECRET leaked")

    tool = FunctionTool(
        name="boom",
        description="boom",
        parameters={"type": "object", "properties": {}},
        handler=boom,
    )
    reg = Registry()
    reg.register_tool(tool)
    reg.register_model(
        ScriptedModel(
            [
                ModelResponse(
                    message=Message(
                        role="assistant",
                        tool_calls=[ToolCall(id="1", name="boom", arguments={})],
                    )
                ),
                ModelResponse(message=Message(role="assistant", content="ok")),
            ]
        )
    )

    result = Runner(reg).run(
        "x", config=RunConfig(can_use_tool=lambda n, a, c: Allow())
    )
    blob = "\n".join(m.content or "" for m in result.messages)
    assert "sk-test-SECRET" not in blob
    assert "REDACTED" in blob or "TOOL_FAILED" in blob


def test_cancel_stops_before_next_tool():
    calls = {"n": 0}
    cancel_flag = {"on": False}

    def handler(args, ctx):
        calls["n"] += 1
        cancel_flag["on"] = True
        return "once"

    tool = FunctionTool(
        name="echo",
        description="echo",
        parameters={"type": "object", "properties": {"text": {"type": "string"}}},
        handler=handler,
    )
    reg = Registry()
    reg.register_tool(tool)
    reg.register_model(
        ScriptedModel(
            [
                ModelResponse(
                    message=Message(
                        role="assistant",
                        tool_calls=[
                            ToolCall(id="1", name="echo", arguments={"text": "a"}),
                            ToolCall(id="2", name="echo", arguments={"text": "b"}),
                        ],
                    )
                ),
            ]
        )
    )

    result = Runner(reg).run(
        "x",
        config=RunConfig(
            can_use_tool=lambda n, a, c: Allow(),
            cancel=lambda: cancel_flag["on"],
        ),
    )
    assert result.status == RunStatus.cancelled
    assert calls["n"] == 1


def test_plugin_disposer_removes_tool():
    class EchoPlugin:
        name = "echo-plugin"
        version = "0.0.1"

        def register(self, registry: Registry):
            return registry.register_tool(_echo_tool())

    reg = Registry()
    dispose = reg.add_plugin(EchoPlugin())
    assert reg.get_tool("echo") is not None
    dispose()
    assert reg.get_tool("echo") is None


def test_package_surface_has_no_shell():
    banned = {"shell", "python_eval", "subprocess", "os_system"}
    assert banned.isdisjoint({n.lower() for n in dir(ah)})


def test_sanitize_patterns():
    assert "sk-abcdefghijklmnop" not in sanitize_text("token sk-abcdefghijklmnop here")
    assert "[REDACTED]" in sanitize_text("api_key=SECRET")
    assert "AIzaSyDummyKeyValue123456789012" not in sanitize_text(
        "leak AIzaSyDummyKeyValue123456789012 end"
    )
    fake_aq = "AQ." + "x" * 24
    assert fake_aq not in sanitize_text(f"leak {fake_aq} end")
