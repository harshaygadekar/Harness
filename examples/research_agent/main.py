"""Multi-step research stub: search tool + fake model. Core unchanged."""

from __future__ import annotations

from agentic_harness.messages import Message, ToolCall
from agentic_harness.model import ModelResponse
from agentic_harness.simple import Agent, function_tool

# Tiny in-memory “index” — no network, no API keys.
_FAKE_DOCS = {
    "harness": "An agentic harness runs a model loop and tools behind a gate.",
    "deny-all": "Default can_use_tool denies every tool until the host Allows.",
    "plugin": "Providers and tool packs register via Plugin + Registry.",
}


def _search(args: dict, ctx) -> str:
    query = str(args.get("query", "")).lower()
    hits = [f"{k}: {v}" for k, v in _FAKE_DOCS.items() if k in query or query in v.lower()]
    if not hits:
        hits = [f"{k}: {v}" for k, v in _FAKE_DOCS.items()]
    return "\n".join(hits[:3])


class ResearchModel:
    """Two tool calls, then a short answer from tool observations."""

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
                            id="s1",
                            name="search",
                            arguments={"query": "harness"},
                        )
                    ],
                )
            )
        if self._i == 2:
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[
                        ToolCall(
                            id="s2",
                            name="search",
                            arguments={"query": "deny-all"},
                        )
                    ],
                )
            )
        # Final answer: use last tool contents if present.
        notes = [m.content for m in messages if m.role == "tool" and m.content]
        summary = " | ".join(notes[-2:]) if notes else "no notes"
        return ModelResponse(
            message=Message(
                role="assistant",
                content=f"Findings: {summary}",
            )
        )


def main() -> None:
    search = function_tool(
        name="search",
        description="Search a tiny fake document index",
        parameters={
            "type": "object",
            "properties": {"query": {"type": "string"}},
            "required": ["query"],
        },
        handler=_search,
    )
    agent = Agent(
        model=ResearchModel(),
        tools=[search],
        system="Use search for facts. Then answer briefly.",
        max_steps=8,
    )
    result = agent.run("What is the harness deny-all gate?")
    print(result.status.value)
    print("steps:", result.steps)
    print(result.messages[-1].content)
    tool_msgs = [m for m in result.messages if m.role == "tool"]
    print("tool_rounds:", len(tool_msgs))


if __name__ == "__main__":
    main()
