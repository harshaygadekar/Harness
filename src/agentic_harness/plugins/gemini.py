"""Gemini ModelClient plugin. Optional dependency: google-genai.

Reads GEMINI_API_KEY from the environment. Never log the key.
"""

from __future__ import annotations

import json
import os
import time
from dataclasses import dataclass
from typing import Any

from agentic_harness.messages import Message, ToolCall, Usage
from agentic_harness.model import ModelResponse
from agentic_harness.plugin import Disposer, Registry
from agentic_harness.tool import ToolSpec


def _messages_to_gemini(messages: list[Message]) -> tuple[str | None, list[dict[str, Any]]]:
    system = None
    contents: list[dict[str, Any]] = []
    for m in messages:
        if m.role == "system":
            system = (system + "\n" if system else "") + (m.content or "")
            continue
        if m.role == "user":
            contents.append({"role": "user", "parts": [{"text": m.content or ""}]})
        elif m.role == "assistant":
            parts: list[dict[str, Any]] = []
            if m.content:
                parts.append({"text": m.content})
            for tc in m.tool_calls:
                parts.append(
                    {
                        "function_call": {
                            "name": tc.name,
                            "args": tc.arguments,
                        }
                    }
                )
            contents.append({"role": "model", "parts": parts or [{"text": ""}]})
        elif m.role == "tool":
            contents.append(
                {
                    "role": "user",
                    "parts": [
                        {
                            "function_response": {
                                "name": m.name or "tool",
                                "response": {"result": m.content or ""},
                            }
                        }
                    ],
                }
            )
    return system, contents


def _tools_to_gemini(tools: list[ToolSpec] | None) -> list[dict[str, Any]] | None:
    if not tools:
        return None
    return [
        {
            "function_declarations": [
                {
                    "name": t.name,
                    "description": t.description,
                    "parameters": t.parameters,
                }
                for t in tools
            ]
        }
    ]


@dataclass
class GeminiModel:
    """ModelClient backed by Google Gemini."""

    model: str = "gemini-flash-latest"
    api_key: str | None = None

    def __post_init__(self) -> None:
        key = self.api_key or os.environ.get("GEMINI_API_KEY") or os.environ.get("GOOGLE_API_KEY")
        if not key:
            raise RuntimeError("GEMINI_API_KEY missing (set in .env)")
        self.api_key = key

    def complete(
        self,
        messages: list[Message],
        tools: list[ToolSpec] | None = None,
    ) -> ModelResponse:
        try:
            from google import genai
            from google.genai import types
        except ImportError as exc:  # pragma: no cover
            raise RuntimeError(
                "Install google-genai to use the Gemini plugin: pip install google-genai"
            ) from exc

        client = genai.Client(api_key=self.api_key)
        system, contents = _messages_to_gemini(messages)
        tool_cfg = _tools_to_gemini(tools)

        config_kwargs: dict[str, Any] = {}
        if system:
            config_kwargs["system_instruction"] = system
        if tool_cfg:
            config_kwargs["tools"] = tool_cfg

        config = types.GenerateContentConfig(**config_kwargs) if config_kwargs else None

        # Brief retries for transient Google 503 / UNAVAILABLE.
        last_exc: Exception | None = None
        response = None
        for attempt in range(3):
            try:
                response = client.models.generate_content(
                    model=self.model,
                    contents=contents,
                    config=config,
                )
                break
            except Exception as exc:  # noqa: BLE001 — narrow by message below
                last_exc = exc
                msg = str(exc).upper()
                transient = "503" in msg or "UNAVAILABLE" in msg or "HIGH DEMAND" in msg
                if not transient or attempt == 2:
                    raise
                time.sleep(0.8 * (attempt + 1))
        if response is None and last_exc is not None:
            raise last_exc

        text_parts: list[str] = []
        tool_calls: list[ToolCall] = []
        n = 0
        for cand in response.candidates or []:
            content = getattr(cand, "content", None)
            if not content:
                continue
            for part in content.parts or []:
                fc = getattr(part, "function_call", None)
                if fc:
                    n += 1
                    args = dict(fc.args) if fc.args else {}
                    tool_calls.append(
                        ToolCall(id=f"call_{n}", name=fc.name or "unknown", arguments=args)
                    )
                elif getattr(part, "text", None):
                    text_parts.append(part.text)

        usage = None
        um = getattr(response, "usage_metadata", None)
        if um:
            usage = Usage(
                input_tokens=int(getattr(um, "prompt_token_count", 0) or 0),
                output_tokens=int(getattr(um, "candidates_token_count", 0) or 0),
            )

        return ModelResponse(
            message=Message(
                role="assistant",
                content="\n".join(text_parts) if text_parts else None,
                tool_calls=tool_calls,
            ),
            usage=usage,
        )


@dataclass
class GeminiPlugin:
    name: str = "gemini"
    version: str = "0.1.0"
    model: str = "gemini-flash-latest"

    def register(self, registry: Registry) -> Disposer:
        return registry.register_model(GeminiModel(model=self.model))
