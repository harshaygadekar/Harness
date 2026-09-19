# Security

## Tool gate

Core default is **deny all tools**. The host must allow each tool by name (or an allow list). Core does not guess “read-only” or “pure.”

## No built-in side-effect tools

Core ships no shell, `python_eval`, filesystem, or network tools.

## Plugins

v0 plugins run **in-process** and are fully trusted. Do not load remote plugins. Do not claim isolation.

## Errors

Tool exceptions are turned into `TOOL_FAILED` observations with secret scrubbing (`sk-`, Bearer tokens, `api_key=`, home paths, …).

## Credentials

No credential store in core. Keep keys in the host environment for provider plugins.
