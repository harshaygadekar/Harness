# Harness

Minimal **agentic harness** (Python): a small core loop, tool protocol, deny-all gate, and plugins.

**Core stays thin. Everything else is a plugin.**

Design inspiration: [DeepSeek Harness](https://github.com/deepseek-ai/deepseek-harness) / Cordis paper — seams and tool gate only. We did **not** vendor Cordis or `dsh`.

Repo: https://github.com/harshaygadekar/Harness

## Status

ADR-001 **Accepted**. Spike + tests green. Stdlib-only core.

## Install

```bash
git clone https://github.com/harshaygadekar/Harness.git
cd Harness
python -m venv .venv
source .venv/bin/activate
pip install -e ".[dev]"
pytest
python examples/echo_demo.py
```

Optional Gemini adapter (plugin, not core):

```bash
pip install -e ".[gemini]"
cp .env.example .env
# put GEMINI_API_KEY in .env — never commit .env, never paste keys in chat
python examples/gemini_echo.py
```

## First demo (must work)

One model adapter + one user-supplied tool + `Runner.run`.

Host **must allowlist tools by name**. Default policy denies every tool call.

No shell. No `python_eval`. No MCP. No multi-agent. No server in core.

## Non-goals (core)

Graphs, crews, MCP runtime, Docker/sandbox, RAG, memory OS, observability SaaS, built-in shell/browser tools. Those are plugins if we ever need them.

## Docs

- `docs/useful-scope.md` — product lock
- `docs/decisions/ADR-001-core-loop.md` — core loop
- `docs/research/` — research pack (incl. DeepSeek Harness note)
- `CONTRIBUTING.md` — branch / PR workflow
- `SECURITY.md` — keys, gate, scrub rules
- `docs/plugin-authoring.md` — write a plugin
- `docs/useful-scope-phase2.md` — Phase 2 coding-agent lock

## Contribute

See `CONTRIBUTING.md`. Branch from `main`, open a PR. Do not commit secrets.
