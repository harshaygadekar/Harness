"""Attempt token streaming with current ModelClient.complete only.

This example shows the wall: Runner and ModelClient expose one blocking
complete() step. There is no complete_stream, no token callback, and no
Runner hook that yields partial text to the host during a model call.
"""

from __future__ import annotations

from agentic_harness.messages import Message
from agentic_harness.model import ModelResponse
from agentic_harness.simple import Agent


class BufferedFakeStreamModel:
    """Pretend the provider streams tokens, then buffer until complete returns.

    A real adapter could call a streaming SDK inside complete() and still only
    hand the host a full Message when complete() returns. The host never sees
    tokens mid-call through Agent.run / Runner.run.
    """

    def complete(self, messages, tools=None) -> ModelResponse:
        tokens = ["Hel", "lo", ",", " world"]
        # Internal "stream" — invisible to Runner/host until we finish.
        text = "".join(tokens)
        return ModelResponse(message=Message(role="assistant", content=text))


def try_stream_through_agent() -> None:
    """What hosts can do today: wait for the full run, then print."""
    agent = Agent(model=BufferedFakeStreamModel())
    result = agent.run("Say hello")
    # No yield points between tokens. Only the final content.
    print("status:", result.status.value)
    print("final:", result.messages[-1].content)


def document_missing_api() -> None:
    """NOT available on ModelClient today — wall check only."""
    # Desired (does not exist):
    #   for chunk in model.complete_stream(messages, tools):
    #       yield chunk.text
    # Runner would need to call it and surface chunks to the host.
    model = BufferedFakeStreamModel()
    assert hasattr(model, "complete")
    assert not hasattr(model, "complete_stream")


def main() -> None:
    try_stream_through_agent()
    document_missing_api()
    print("WALL: no ModelClient.complete_stream; Runner has no token hook.")


if __name__ == "__main__":
    main()
