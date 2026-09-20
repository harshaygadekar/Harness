"""Thin coding-agent CLI (ADR-004 Candidate A).

Jessie gates:
- deny-all by default (no silent Allow of the full pack)
- shell only with --enable-shell
- workspace path binding for coding tools
- no secret echo in help / errors
- no OS-sandbox claim
"""

from __future__ import annotations

import argparse
import os
import sys
from pathlib import Path


DEFAULT_CODING_ALLOW = (
    "list_files",
    "read_file",
    "write_file",
    "search",
    "apply_patch",
    "git_status",
)


def _build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="agentic-harness",
        description=(
            "Minimal agentic harness CLI. Coding tools are plugins. "
            "Default tool policy is deny-all. This is not an OS sandbox."
        ),
    )
    sub = p.add_subparsers(dest="cmd", required=True)

    run = sub.add_parser("run", help="Run one agent turn (fake or Gemini model)")
    run.add_argument("--prompt", required=True, help="User prompt")
    run.add_argument(
        "--workspace",
        type=Path,
        default=None,
        help="Workspace root for coding plugins (required if tools are allowed)",
    )
    run.add_argument(
        "--allow",
        default="",
        help="Comma-separated tool names to Allow (deny-all if empty and no pack flag)",
    )
    run.add_argument(
        "--allow-coding-pack",
        action="store_true",
        help="Allow default coding pack tools (no shell)",
    )
    run.add_argument(
        "--enable-shell",
        action="store_true",
        help="Opt in to run_shell plugin (also Adds it to the Allow list)",
    )
    run.add_argument(
        "--model",
        choices=("fake", "gemini"),
        default="fake",
        help="Model backend (default: fake)",
    )
    run.add_argument("--max-steps", type=int, default=8)
    run.add_argument(
        "--system",
        default="You are a coding helper. Use only allowed tools. No shell unless enabled.",
    )

    sub.add_parser("version", help="Print package version")
    return p


def _parse_allow(args: argparse.Namespace) -> set[str]:
    names: set[str] = set()
    raw = (args.allow or "").strip()
    if raw:
        names.update(x.strip() for x in raw.split(",") if x.strip())
    if args.allow_coding_pack:
        names.update(DEFAULT_CODING_ALLOW)
    if args.enable_shell:
        names.add("run_shell")
    return names


def _make_fake_model(allow: set[str]):
    from agentic_harness.messages import Message, ToolCall
    from agentic_harness.model import ModelResponse

    class FakeCodingModel:
        def __init__(self) -> None:
            self.i = 0

        def complete(self, messages, tools=None) -> ModelResponse:
            self.i += 1
            # Prefer a safe read path when list/read are allowed.
            if self.i == 1 and "list_files" in allow:
                return ModelResponse(
                    message=Message(
                        role="assistant",
                        tool_calls=[
                            ToolCall(
                                id="1",
                                name="list_files",
                                arguments={"path": "."},
                            )
                        ],
                    )
                )
            if self.i == 2 and "read_file" in allow:
                return ModelResponse(
                    message=Message(
                        role="assistant",
                        tool_calls=[
                            ToolCall(
                                id="2",
                                name="read_file",
                                arguments={"path": "notes.txt"},
                            )
                        ],
                    )
                )
            return ModelResponse(
                message=Message(
                    role="assistant",
                    content="done (fake model; no network)",
                )
            )

    return FakeCodingModel()


def _make_gemini_model():
    from agentic_harness.plugins.gemini import GeminiModel

    return GeminiModel()


def _load_tools(workspace: Path | None, allow: set[str], enable_shell: bool):
    if not allow:
        return []
    if workspace is None:
        raise SystemExit("error: --workspace is required when tools are allowed")
    root = workspace.resolve()
    if not root.is_dir():
        raise SystemExit(f"error: workspace is not a directory: {root}")

    # Ensure repo-root plugins package is importable when running from source.
    repo = Path(__file__).resolve().parents[2]
    if str(repo) not in sys.path:
        sys.path.insert(0, str(repo))

    from plugins.coding import coding_tools

    tools = coding_tools(root, include_shell=enable_shell)
    # Filter to Allow list only — Jessie: no silent full pack when --allow is partial.
    return [t for t in tools if t.name in allow]


def cmd_run(args: argparse.Namespace) -> int:
    from agentic_harness.simple import Agent

    allow = _parse_allow(args)
    try:
        tools = _load_tools(args.workspace, allow, args.enable_shell)
    except SystemExit as exc:
        print(str(exc), file=sys.stderr)
        return 2

    if args.model == "gemini":
        # Never print key material.
        if not (os.environ.get("GEMINI_API_KEY") or os.environ.get("GOOGLE_API_KEY")):
            print(
                "error: GEMINI_API_KEY (or GOOGLE_API_KEY) missing for --model gemini",
                file=sys.stderr,
            )
            return 2
        model = _make_gemini_model()
    else:
        model = _make_fake_model(allow)

    agent = Agent(
        model=model,
        tools=tools,
        system=args.system,
        max_steps=args.max_steps,
    )
    result = agent.run(args.prompt)
    print(result.status.value)
    print(f"steps={result.steps}")
    if result.messages:
        last = result.messages[-1]
        print(last.content or "")
    if result.error:
        # SafeError is already scrubbed at the boundary when tools fail.
        print(f"error={result.error.code}:{result.error.message}", file=sys.stderr)
        return 1
    return 0 if result.status.value in {"succeeded", "max_steps"} else 1


def main(argv: list[str] | None = None) -> int:
    parser = _build_parser()
    args = parser.parse_args(argv)
    if args.cmd == "version":
        from agentic_harness import __version__

        print(__version__)
        return 0
    if args.cmd == "run":
        return cmd_run(args)
    parser.error(f"unknown command {args.cmd}")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
