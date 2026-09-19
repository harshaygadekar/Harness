"""Structured JSON-like output via prompt + host parse. Core unchanged."""

from __future__ import annotations

import json
import re

from agentic_harness.messages import Message
from agentic_harness.model import ModelResponse
from agentic_harness.simple import Agent


class JsonCapitalModel:
    """Fake model: returns a JSON object in assistant content."""

    def complete(self, messages, tools=None) -> ModelResponse:
        body = json.dumps(
            {"city": "Paris", "country": "France", "confidence": 0.99},
            separators=(",", ":"),
        )
        return ModelResponse(
            message=Message(
                role="assistant",
                content=f"Here is the result:\n```json\n{body}\n```",
            )
        )


def extract_json_object(text: str) -> dict:
    """Host-side parse. Core has no structured-output seam."""
    fence = re.search(r"```(?:json)?\s*(\{.*?\})\s*```", text, re.DOTALL)
    raw = fence.group(1) if fence else text
    start = raw.find("{")
    end = raw.rfind("}")
    if start < 0 or end < 0:
        raise ValueError("no JSON object in assistant text")
    return json.loads(raw[start : end + 1])


def main() -> None:
    agent = Agent(
        model=JsonCapitalModel(),
        system=(
            "Reply with one JSON object that has keys "
            "city, country, confidence. Put it in a json fence."
        ),
    )
    result = agent.run("What is the capital of France?")
    print(result.status.value)
    assert result.messages[-1].content is not None
    data = extract_json_object(result.messages[-1].content)
    print(json.dumps(data, sort_keys=True))


if __name__ == "__main__":
    main()
