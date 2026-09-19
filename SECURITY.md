# Security

## Tool gate

Core default is **deny all tools**. The host must allow each tool by name (or an allow list). Core does not guess “read-only” or “pure.”

## No built-in side-effect tools

Core ships no shell, `python_eval`, filesystem, or network tools.

## Plugins

V0 plugins run **in-process** and are fully trusted. Do not load remote plugins. Do not claim isolation.

## Errors

Tool exceptions become `TOOL_FAILED` observations with secret scrubbing (`sk-`, Bearer tokens, `api_key=`, Google `AIza` / `AQ.` keys, home paths, and similar).

## Tool.call

Do not call `Tool.call` outside `Runner`. The registry has no execute helper. The tool object still exposes `call` inside the host process. Treat that as trusted code.

## Credentials

No credential store in core. Keep keys in the host environment (gitignored `.env`) for provider plugins. Never commit keys. Rotate any key that was pasted into chat.
