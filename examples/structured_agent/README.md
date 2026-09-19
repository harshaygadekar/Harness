# structured_agent

**Status:** Works without core change.

## Goal

Ask the agent for structured JSON-like output. Parse on the host.

## How

1. Set a system prompt that asks for a JSON object (optional fence).
2. Use a fake `ModelClient` that returns JSON in `Message.content`.
3. After `Agent.run`, parse with `json` + a small host helper.

No new core type. No pydantic in core. See `main.py` → `extract_json_object`.

## Optional path (also works)

Register a host `FunctionTool` whose handler returns a JSON string. The model
calls that tool; the host still parses the string. Same seam: Tool + prompt.

## Missing seam?

None for “get structured data after the run.” Host parse is enough.

Core does not validate JSON Schema against assistant text. That stays host/plugin
(see research brief: pydantic not in core).
