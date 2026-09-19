"""Live Gemini demo: one model + echo tool. Needs GEMINI_API_KEY in .env."""

from __future__ import annotations

import os
from pathlib import Path

from agentic_harness.gate import Allow, Deny
from agentic_harness.plugin import Registry
from agentic_harness.plugins.gemini import GeminiPlugin
from agentic_harness.runner import RunConfig, Runner
from agentic_harness.tool import FunctionTool


def _load_dotenv() -> None:
    path = Path(__file__).resolve().parents[1] / ".env"
    if not path.exists():
        return
    for line in path.read_text().splitlines():
        if not line.strip() or line.strip().startswith("#") or "=" not in line:
            continue
        k, v = line.split("=", 1)
        os.environ.setdefault(k.strip(), v.strip())


def main() -> None:
    _load_dotenv()
    reg = Registry()
    reg.add_plugin(GeminiPlugin(model="gemini-flash-latest"))
    reg.register_tool(
        FunctionTool(
            name="echo",
            description="Return the given text unchanged",
            parameters={
                "type": "object",
                "properties": {"text": {"type": "string"}},
                "required": ["text"],
            },
            handler=lambda args, ctx: str(args.get("text", "")),
        )
    )

    def allow_echo(name, args, ctx):
        return Allow() if name == "echo" else Deny(reason="not allowlisted")

    result = Runner(reg).run(
        "Call the echo tool with text=hello-from-gemini, then stop.",
        config=RunConfig(can_use_tool=allow_echo, max_steps=4),
    )
    print(result.status.value)
    for m in result.messages:
        print(f"{m.role}: {m.content!r} tools={len(m.tool_calls)}")


if __name__ == "__main__":
    main()
