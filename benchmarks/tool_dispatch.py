"""B2 — fake-model gated tool dispatch throughput (P2-5 MUST)."""

from __future__ import annotations

import argparse
import statistics
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "src"))

from agentic_harness.messages import Message, ToolCall
from agentic_harness.model import ModelResponse
from agentic_harness.simple import Agent, function_tool


class ScriptedModel:
    """Emits exactly `steps` tool calls, then stops."""

    def __init__(self, steps: int, tool_names: list[str]) -> None:
        self.steps = steps
        self.tool_names = tool_names
        self.i = 0

    def complete(self, messages, tools=None) -> ModelResponse:
        if self.i < self.steps:
            name = self.tool_names[self.i % len(self.tool_names)]
            self.i += 1
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[
                        ToolCall(id=str(self.i), name=name, arguments={"n": self.i})
                    ],
                )
            )
        return ModelResponse(message=Message(role="assistant", content="done"))


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--steps", type=int, default=1000)
    p.add_argument("--tools", type=int, default=8)
    args = p.parse_args()

    tools = [
        function_tool(
            f"noop_{i}",
            "noop",
            {
                "type": "object",
                "properties": {"n": {"type": "integer"}},
                "required": ["n"],
            },
            lambda a, c: "ok",
        )
        for i in range(args.tools)
    ]
    names = [t.name for t in tools]

    def once() -> None:
        Agent(
            model=ScriptedModel(args.steps, names),
            tools=tools,
            max_steps=args.steps + 2,
        ).run("go")

    # warmup
    once()
    samples = []
    for _ in range(5):
        t0 = time.perf_counter()
        once()
        samples.append(time.perf_counter() - t0)
    median_s = statistics.median(samples)
    dispatch_per_s = args.steps / median_s
    median_us = (median_s / args.steps) * 1_000_000
    print(f"steps={args.steps}")
    print(f"tools={args.tools}")
    print(f"dispatch_per_s={dispatch_per_s:.1f}")
    print(f"median_us_per_step={median_us:.2f}")
    print(f"median_s={median_s:.6f}")
    # MUST P2-5: >= 1000 / s
    print(f"p2_5_must_pass={dispatch_per_s >= 1000.0}")


if __name__ == "__main__":
    main()
