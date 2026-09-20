# CLI (`agentic-harness`)

ADR-004 Candidate A. Thin wrapper over the library + coding plugins.

## Install

```bash
pip install -e .
agentic-harness version
```

## Run (fake model)

```bash
agentic-harness run \
  --workspace ./examples/coding_agent/workspace_sample \
  --allow-coding-pack \
  --prompt "List files and read notes.txt"
```

## Security defaults (Jessie)

- **Deny-all** unless `--allow` and/or `--allow-coding-pack`.
- **Shell** only with `--enable-shell` (also Allows `run_shell`).
- Workspace tools are bound with `Path.relative_to` (no escape).
- This CLI is **not** an OS sandbox. Plugins run in-process (full host trust).

## Gemini (optional)

```bash
export GEMINI_API_KEY=...   # local .env only — never commit
agentic-harness run --model gemini --workspace . --allow-coding-pack --prompt "..."
```
