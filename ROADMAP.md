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


## Phase 2 (in progress)

- Product lock: `docs/useful-scope-phase2.md`
- One-file Agent + examples (Mike)
- Disprove core / missing primitives (Thomas)
- Security attack suite (Jessie)
- Protocol docs freeze (Walter)
- Coding agent = plugins only; no core growth; no “beat fx” claims without numbers
