"""B4 — end-to-end fake coding loop with host plugins (P2-7 SHOULD)."""

from __future__ import annotations

import argparse
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "src"))
sys.path.insert(0, str(ROOT))

from agentic_harness.messages import Message, ToolCall
from agentic_harness.model import ModelResponse
from agentic_harness.result import RunStatus
from agentic_harness.simple import Agent
from plugins.coding import coding_tools


class CodingScript:
    def __init__(self) -> None:
        self.i = 0

    def complete(self, messages, tools=None) -> ModelResponse:
        self.i += 1
        if self.i == 1:
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[
                        ToolCall(id="1", name="list_files", arguments={"path": "."})
                    ],
                )
            )
        if self.i == 2:
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[
                        ToolCall(
                            id="2",
                            name="write_file",
                            arguments={"path": "out.txt", "content": "ok"},
                        )
                    ],
                )
            )
        if self.i == 3:
            return ModelResponse(
                message=Message(
                    role="assistant",
                    tool_calls=[
                        ToolCall(id="3", name="read_file", arguments={"path": "out.txt"})
                    ],
                )
            )
        return ModelResponse(message=Message(role="assistant", content="completed"))


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--max-steps", type=int, default=20)
    args = p.parse_args()
    with tempfile.TemporaryDirectory() as td:
        root = Path(td)
        (root / "seed.txt").write_text("seed", encoding="utf-8")
        tools = coding_tools(root, include_shell=False)
        result = Agent(
            model=CodingScript(),
            tools=tools,
            max_steps=args.max_steps,
        ).run("list, write out.txt, read it")
        tools_ok = (root / "out.txt").read_text(encoding="utf-8") == "ok"
        status = "completed" if result.status == RunStatus.succeeded and tools_ok else "failed"
        print(f"status={status}")
        print(f"steps={result.steps}")
        print(f"tools_ok={tools_ok}")
        if status != "completed":
            raise SystemExit(1)


if __name__ == "__main__":
    main()
