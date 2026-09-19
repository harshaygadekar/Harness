from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(ROOT / "src"))

from plugins.coding import coding_tools


def test_coding_pack_confined(tmp_path: Path):
    (tmp_path / "x.txt").write_text("alpha beta", encoding="utf-8")
    tools = {t.name: t for t in coding_tools(tmp_path)}
    listed = tools["list_files"].call({"path": "."}, None).content
    assert "x.txt" in listed
    bad = tools["read_file"].call({"path": "../../etc/passwd"}, None).content
    assert "ERROR" in bad
    hit = tools["search"].call({"query": "alpha"}, None).content
    assert "alpha" in hit


def test_patch_and_write(tmp_path: Path):
    (tmp_path / "f.txt").write_text("one two", encoding="utf-8")
    tools = {t.name: t for t in coding_tools(tmp_path)}
    tools["write_file"].call({"path": "g.txt", "content": "gg"}, None)
    assert (tmp_path / "g.txt").read_text(encoding="utf-8") == "gg"
    tools["apply_patch"].call(
        {"path": "f.txt", "old": "one", "new": "ONE"}, None
    )
    assert (tmp_path / "f.txt").read_text(encoding="utf-8").startswith("ONE")
