"""Shared workspace root guard for coding plugins."""

from __future__ import annotations

from pathlib import Path


class Workspace:
    def __init__(self, root: Path) -> None:
        self.root = root.resolve()

    def resolve(self, rel: str) -> Path:
        rel = (rel or ".").lstrip("/")
        target = (self.root / rel).resolve()
        try:
            target.relative_to(self.root)
        except ValueError as exc:
            raise PermissionError("path escapes workspace") from exc
        return target
