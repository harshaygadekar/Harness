# ADR-001 prompt (Walter drafts from research)

**Problem:** Smallest useful agent core loop for a Python-first OSS harness.

**Constraints:** Minimal core; plugins for the rest; no `python_eval` / shell in core; deny-by-default tool gate; no secret leak in tool errors; Jessie red flags apply; Alice first demo = one model adapter + one user tool + loop.

**Input:** `docs/research/06-deepseek-harness.md` (primary **design** reference) + `05-lessons-for-architecture.md` + `02-core-primitives.md` + `04-keep-out-of-core.md` + `docs/useful-scope.md` + `sources.md`.

**DeepSeek = design source, not API:**
- Means **DeepSeek Harness** (`dsh`) + Cordis paper as **architecture inspiration**.
- Does **not** mean calling the DeepSeek LLM API (or any LLM API) to invent the ADR.
- Do **not** vendor Cordis or `dsh`.
- **No API key required.**

**Output:** `docs/decisions/ADR-001-core-loop.md` (human draft from evidence → Thomas review).
