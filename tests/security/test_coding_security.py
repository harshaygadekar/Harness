"""Security attacks on coding plugins (above core). Jessie."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(ROOT / "src"))

from plugins.coding import coding_tools, make_shell_tool
from plugins.coding.workspace import Workspace


def test_prefix_path_escape_blocked(tmp_path: Path):
    """Regression: root /tmp/ws must not allow /tmp/ws_evil via startswith."""
    root = tmp_path / "ws"
    root.mkdir()
    evil = tmp_path / "ws_evil"
    evil.mkdir()
    (evil / "secret.txt").write_text("nope", encoding="utf-8")
    ws = Workspace(root)
    try:
        ws.resolve("../ws_evil/secret.txt")
        raise AssertionError("expected PermissionError")
    except PermissionError:
        pass


def test_default_coding_pack_has_no_shell(tmp_path: Path):
    names = {t.name for t in coding_tools(tmp_path)}
    assert "run_shell" not in names


def test_shell_soft_deny(tmp_path: Path):
    tool = make_shell_tool(tmp_path)
    out = tool.call({"command": "sudo ls"}, None).content
    assert "ERROR" in out
