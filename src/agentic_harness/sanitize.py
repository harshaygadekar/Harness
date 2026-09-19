from __future__ import annotations

import re

# Concrete patterns (Jessie C3). Prefer codes + short safe text.
_PATTERNS: list[re.Pattern[str]] = [
    re.compile(r"sk-[A-Za-z0-9_\-]{8,}"),
    re.compile(r"Bearer\s+[A-Za-z0-9\-._~+/]+=*", re.I),
    re.compile(r"(?i)(api[_-]?key|password|secret|token)\s*[:=]\s*\S+"),
    re.compile(r"AKIA[0-9A-Z]{16}"),
    # Google API keys (AIza… and AQ.… style)
    re.compile(r"AIza[0-9A-Za-z\-_]{20,}"),
    re.compile(r"AQ\.[A-Za-z0-9_\-]{20,}"),
    re.compile(r"(?i)/(?:home|Users)/[^\s/]+"),
]

_REPLACEMENT = "[REDACTED]"


def sanitize_text(text: str, *, max_len: int = 500) -> str:
    out = text or ""
    for pat in _PATTERNS:
        out = pat.sub(_REPLACEMENT, out)
    if len(out) > max_len:
        out = out[: max_len - 3] + "..."
    return out


def safe_tool_error(exc: BaseException) -> str:
    raw = f"{type(exc).__name__}: {exc}"
    return f"TOOL_FAILED: {sanitize_text(raw, max_len=200)}"
