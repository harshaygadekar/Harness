# Write a plugin (host guide)

Core stays frozen (ADR-003). You extend with plugins.

## Rules

1. Implement `Plugin` / register tools or a `ModelClient` on `Registry`.
2. Return a disposer from `register` so unload cleans up.
3. Do **not** call `Tool.call` outside `Runner`.
4. Host must **Allow** each tool by name (deny-all default).
5. No shell / `python_eval` / MCP in core — put them in your pack.
6. Scrub secrets in errors (see `SECURITY.md`).

## Minimal tool plugin

```python
from agentic_harness.plugin import Registry
from agentic_harness.tool import FunctionTool

def register(reg: Registry):
    reg.register_tool(
        FunctionTool(
            name="echo",
            description="Echo text",
            parameters={"type": "object", "properties": {"text": {"type": "string"}}, "required": ["text"]},
            handler=lambda args, ctx: str(args.get("text", "")),
        )
    )
    return lambda: None  # disposer
```

## Coding pack

Host coding tools live under `plugins/coding/` (list/read/write/search/patch/git_status). Shell is opt-in and off by default. Workspace paths must stay under the workspace root.

## Conformance

```bash
python test_plugin_conformance.py
```

## Product lock

See `docs/useful-scope.md` and `docs/useful-scope-phase2.md`. Do not grow core for convenience.
