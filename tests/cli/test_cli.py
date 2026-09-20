from __future__ import annotations

from pathlib import Path

from agentic_harness.cli import main


def test_version():
    assert main(["version"]) == 0


def test_deny_all_by_default(tmp_path: Path, capsys):
    # No --allow → no tools loaded; fake model just answers.
    code = main(["run", "--prompt", "hi", "--workspace", str(tmp_path)])
    assert code == 0
    out = capsys.readouterr().out
    assert "succeeded" in out or "done" in out


def test_coding_pack_list_read(tmp_path: Path, capsys):
    (tmp_path / "notes.txt").write_text("hello notes", encoding="utf-8")
    code = main(
        [
            "run",
            "--prompt",
            "list and read",
            "--workspace",
            str(tmp_path),
            "--allow-coding-pack",
        ]
    )
    assert code == 0
    out = capsys.readouterr().out
    assert "succeeded" in out


def test_shell_not_in_default_pack(tmp_path: Path):
    from plugins.coding import coding_tools

    names = {t.name for t in coding_tools(tmp_path, include_shell=False)}
    assert "run_shell" not in names


def test_help_has_no_sandbox_claim():
    import agentic_harness.cli as cli

    help_txt = cli._build_parser().format_help()
    assert "not an OS sandbox" in help_txt
    assert "deny-all" in help_txt.lower() or "Deny-all" in help_txt or "deny-all" in help_txt
