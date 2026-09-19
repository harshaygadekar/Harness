"""Fake-model benches: import, construct, tool dispatch, N iterations.

No network. Prints lines suitable for docs/benchmarks/phase2-numbers.md.
"""

from __future__ import annotations

import statistics
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "src"))
sys.path.insert(0, str(ROOT))

from agentic_harness.messages import Message, ToolCall
from agentic_harness.model import ModelResponse
from agentic_harness.simple import Agent, function_tool


class EchoModel:
    def __init__(self, n_tools: int = 0) -> None:
        self.n_tools = n_tools
        self.i = 0

    def complete(self, messages, tools=None) -> ModelResponse:
        self.i += 1
        if self.i <= self.n_tools:
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[
                        ToolCall(id=str(self.i), name="echo", arguments={"text": "x"})
                    ],
                )
            )
        return ModelResponse(message=Message(role="assistant", content="done"))


def _median_ms(fn, repeats: int = 21) -> float:
    samples = []
    for _ in range(repeats):
        t0 = time.perf_counter()
        fn()
        samples.append((time.perf_counter() - t0) * 1000)
    return statistics.median(samples)


def main() -> None:
    # cold-ish import already happened; measure re-import cost of submodule
    def reimport():
        import importlib
        import agentic_harness

        importlib.reload(agentic_harness)

    echo = function_tool(
        "echo",
        "echo",
        {
            "type": "object",
            "properties": {"text": {"type": "string"}},
            "required": ["text"],
        },
        lambda args, ctx: args.get("text", ""),
    )

    def one_shot():
        Agent(model=EchoModel(0)).run("hi")

    def with_tools():
        Agent(model=EchoModel(1), tools=[echo]).run("hi")

    def many_iters():
        Agent(model=EchoModel(5), tools=[echo], max_steps=12).run("hi")

    print(f"reload_agentic_harness_ms_median={_median_ms(reimport):.3f}")
    print(f"agent_run_no_tools_ms_median={_median_ms(one_shot):.3f}")
    print(f"agent_run_one_tool_ms_median={_median_ms(with_tools):.3f}")
    print(f"agent_run_five_tools_ms_median={_median_ms(many_iters):.3f}")

    # high-volume synthetic tool dispatch via Agent
    t0 = time.perf_counter()
    for _ in range(200):
        Agent(model=EchoModel(1), tools=[echo]).run("x")
    elapsed = time.perf_counter() - t0
    print(f"agent_runs_200_one_tool_s={elapsed:.3f}")
    print(f"agent_runs_200_one_tool_per_s={200 / elapsed:.1f}")


if __name__ == "__main__":
    main()
