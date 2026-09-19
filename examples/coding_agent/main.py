"""Coding agent demo: plugins only. Core unchanged. Fake model, no network."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "src"))
sys.path.insert(0, str(ROOT))

from agentic_harness.messages import Message, ToolCall
from agentic_harness.model import ModelResponse
from agentic_harness.simple import Agent
from plugins.coding import coding_tools

WORKSPACE = Path(__file__).resolve().parent / "workspace_sample"


class CodingModel:
    """list → search → read → short answer."""

    def __init__(self) -> None:
        self._i = 0

    def complete(self, messages, tools=None) -> ModelResponse:
        self._i += 1
        if self._i == 1:
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[
                        ToolCall(id="1", name="list_files", arguments={"path": "."})
                    ],
                )
            )
        if self._i == 2:
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[
                        ToolCall(id="2", name="search", arguments={"query": "hello"})
                    ],
                )
            )
        if self._i == 3:
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[
                        ToolCall(
                            id="3",
                            name="read_file",
                            arguments={"path": "notes.txt"},
                        )
                    ],
                )
            )
        tool_bits = [m.content for m in messages if m.role == "tool"]
        return ModelResponse(
            message=Message(
                role="assistant",
                content="coding pass ok; last tool outs=" + repr(tool_bits[-1:]),
            )
        )


def main() -> None:
    WORKSPACE.mkdir(parents=True, exist_ok=True)
    (WORKSPACE / "notes.txt").write_text("hello from sample workspace\n", encoding="utf-8")
    (WORKSPACE / "hello.py").write_text('def hello():\n    return "hi"\n', encoding="utf-8")

    tools = coding_tools(WORKSPACE, include_shell=False)
    agent = Agent(
        model=CodingModel(),
        tools=tools,
        system="Use list_files, search, read_file only. No shell.",
        max_steps=10,
    )
    result = agent.run("Explore the workspace and read notes.txt")
    print(result.status.value)
    print("steps:", result.steps)
    print(result.messages[-1].content)


if __name__ == "__main__":
    main()
