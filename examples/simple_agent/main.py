"""Smallest useful agent: fake model, no tools. Core unchanged."""

from __future__ import annotations

from agentic_harness.messages import Message
from agentic_harness.model import ModelResponse
from agentic_harness.simple import Agent


class CapitalModel:
    """Fake model: answers a fixed geography question."""

    def complete(self, messages, tools=None) -> ModelResponse:
        return ModelResponse(
            message=Message(role="assistant", content="Paris")
        )


def main() -> None:
    agent = Agent(model=CapitalModel())
    result = agent.run("Find the capital of France.")
    print(result.status.value)
    print(result.messages[-1].content)


if __name__ == "__main__":
    main()
