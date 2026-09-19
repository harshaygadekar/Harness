"""Minimal agentic harness core (ADR-001).

Core = Runner loop + ModelClient/Tool protocols + registry + deny-all gate + plugins.
No provider SDKs, shell, MCP, or built-in side-effect tools.
"""

from .messages import Message, ToolCall, Usage
from .model import ModelClient, ModelResponse
from .plugin import Plugin, Registry
from .result import RunResult, RunStatus, SafeError
from .runner import Runner, RunConfig
from .sanitize import sanitize_text
from .tool import Tool, ToolResult, ToolSpec

__all__ = [
    "Message",
    "ModelClient",
    "ModelResponse",
    "Plugin",
    "Registry",
    "RunConfig",
    "RunResult",
    "RunStatus",
    "Runner",
    "SafeError",
    "Tool",
    "ToolCall",
    "ToolResult",
    "ToolSpec",
    "Usage",
    "sanitize_text",
]

__version__ = "0.1.0"
