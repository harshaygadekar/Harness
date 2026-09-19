# Roadmap

Local `main` until GitHub push unlocks. Order is product-first.

## Done

- [x] Research pack (`docs/research/`)
- [x] ADR-001 core loop (Accepted) + spike + tests
- [x] Useful scope lock (`docs/useful-scope.md`)
- [x] README / ARCHITECTURE / CONTRIBUTING / SECURITY
- [x] Optional Gemini adapter plugin (key in `.env` only)

## Next

1. [x] ADR-002 — model adapter plugin boundary (Accepted)
2. Harden Gemini plugin (retry on transient 503) — Mike
3. Public docs pass after push (`BENCHMARKS.md` numbers from spike)
4. First external-facing example that stays under Alice demo lock

## Later (plugins only — not core)

- MCP adapter pack
- Condenser / history trim
- Checkpointer
- Observability exporter
- Tool packs (shell, etc.) behind host Allow + sandbox ADR

## Will not do in core

Graphs, crews, built-in shell/`python_eval`, MCP runtime, memory OS, RAG. See `docs/useful-scope.md`.


## Phase 2 (complete for night cut)

- [x] Product lock: `docs/useful-scope-phase2.md`
- [x] Product sign-off: `docs/product-signoff-phase2.md`
- [x] Plugin authoring: `docs/plugin-authoring.md`
- [x] One-file Agent + examples (Mike)
- [x] Disprove core / missing primitives (Thomas) — freeze kept
- [x] Security attack suite (Jessie)
- [x] Protocol docs freeze (Walter) — ADR-003 Accepted
- [x] Coding agent = plugins only; no core growth; no “beat fx” claims without numbers
