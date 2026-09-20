# Sources

URLs and papers used for this research pack. Fetched or searched 2026-09-19 (IST). Do not invent citations; if a page blocked or 404’d, noted below.

**Correction:** DeepSeek entries refer to the **DeepSeek Harness** agent framework and Cordis paper (design inspiration). They are not invitations to call the DeepSeek LLM API for ADR decisions.

---

## Official docs / product pages

### DeepSeek Harness (PRIMARY — design inspiration, not LLM API)
- https://github.com/deepseek-ai/deepseek-harness
- https://deepseek.com/harness/en/
- https://deepseek-harness.github.io/deepseek-harness/en/reference/
- https://deepseek-harness.github.io/deepseek-harness/en/reference/cordis-primer
- https://deepseek-harness.github.io/deepseek-harness/en/develop/framework/
- https://deepseek-harness.github.io/deepseek-harness/en/reference/subsystems/tools
- https://deepseek-harness.github.io/deepseek-harness/en/develop/cordis-tutorial/07-into-the-harness
- Cordis meta-framework: https://github.com/cordisjs/cordis
- Cordis paper draft site: https://github.com/cordiverse/paper

### OpenAI Agents SDK
- https://openai.github.io/openai-agents-python/agents/
- https://openai.github.io/openai-agents-python/running_agents/
- https://openai.github.io/openai-agents-python/ (project)
- https://github.com/openai/openai-agents-python
- https://developers.openai.com/api/docs/guides/agents/sandboxes (SandboxAgent concepts; search hit)

### Anthropic / Claude Agent SDK
- https://code.claude.com/docs/en/agent-sdk/overview
- https://code.claude.com/docs/en/agent-sdk/quickstart
- https://code.claude.com/docs/en/agent-sdk/python
- https://code.claude.com/docs/en/agent-sdk/typescript
- https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code
- Related engineering (search): https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents
- Related engineering (search): https://www.anthropic.com/engineering/harness-design-long-running-apps

### LangGraph
- https://docs.langchain.com/oss/python/langgraph/graph-api
- https://docs.langchain.com/oss/python/langgraph/pregel
- https://docs.langchain.com/oss/python/langgraph/use-graph-api
- https://docs.langchain.com/oss/python/langgraph/choosing-apis
- https://github.com/langchain-ai/langgraph

### PydanticAI
- https://pydantic.dev/docs/ai/core-concepts/
- https://pydantic.dev/docs/ai/graph/graph/
- https://pydantic.dev/docs/ai/api/pydantic-ai/agent (search)
- https://pydantic.dev/docs/ai/api/pydantic-ai/run/ (search)

### smolagents
- https://huggingface.co/docs/smolagents/en/conceptual_guides/react
- https://huggingface.co/docs/smolagents/en/reference/agents
- https://huggingface.co/docs/smolagents/main/en/index
- https://github.com/huggingface/smolagents

### OpenHands
- https://docs.openhands.dev/sdk/arch/overview
- https://docs.openhands.dev/openhands/usage/architecture/runtime (search)
- https://github.com/OpenHands/software-agent-sdk / All-Hands-AI agent-sdk (search)

### SWE-agent / mini-SWE-agent
- https://swe-agent.com/latest/background/architecture/
- https://mini-swe-agent.com/latest/
- https://github.com/SWE-agent/SWE-agent
- https://github.com/SWE-agent/mini-swe-agent

### AutoGen / AG2 / Microsoft Agent Framework
- https://microsoft.github.io/autogen/stable/user-guide/core-user-guide/core-concepts/application-stack.html
- https://learn.microsoft.com/en-us/agent-framework/overview/
- https://github.com/ag2ai/ag2
- **Note:** Several AG2 docs URLs returned 404 during fetch; GitHub README blob returned empty. Treat AG2 network details as provisional.

### CrewAI
- https://docs.crewai.com/en/concepts/agents
- Crews / Tasks concept pages on docs.crewai.com (search)
- https://github.com/crewaiinc/crewai

### Letta (MemGPT)
- https://docs.letta.com/guides/core-concepts/stateful-agents/ (fetched; redirects may vary)
- https://www.letta.com/blog/agent-memory/
- Memory blocks / archival docs on docs.letta.com (search)

### MCP
- https://modelcontextprotocol.io/docs/learn/architecture
- https://modelcontextprotocol.io/specification/2026-07-28 (search)
- https://blog.modelcontextprotocol.io/posts/2026-07-28/ (search)

---


### fx (Vercel Labs) — beat-goals research (2026-09-19 IST evening)
- https://fx.sh/
- https://github.com/vercel-labs/fx
- https://raw.githubusercontent.com/vercel-labs/fx/main/README.md
- https://raw.githubusercontent.com/vercel-labs/fx/main/AGENTS.md
- https://raw.githubusercontent.com/vercel-labs/fx/main/benchmarks/check_budgets.py
- https://raw.githubusercontent.com/vercel-labs/fx/main/CHANGELOG.md
- https://fx.sh/docs/capabilities/tools.md
- https://fx.sh/llms.txt
- https://api.github.com/repos/vercel-labs/fx/releases/latest (tag **v0.0.10**)
- Local: `benchmarks/cold_import.py`; unpacked `fx-linux-x86_64` from release asset (11,821,040 bytes); `FX_BENCH=1 fx help` smoke timings on research host (informational only)
- Secondary (not SLA): https://devaireviews.com/blog/vercel-fx-coding-agent-review/

## Papers

- Shi, Zhang, Cui, **A Programming Paradigm for Spatiotemporal Composability** (Cordis / DeepSeek Harness foundation), 2026. https://arxiv.org/abs/2608.25512
- Yao et al., **ReAct: Synergizing Reasoning and Acting in Language Models**, 2022. https://arxiv.org/abs/2210.03629
- Packer et al., **MemGPT: Towards LLMs as Operating Systems**, 2023. https://arxiv.org/abs/2310.08560
- Yang et al., **SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering**, NeurIPS 2024. https://arxiv.org/abs/2405.15793 (commonly cited; PDF also via NeurIPS proceedings)
- Wang et al., **OpenHands: An Open Platform for AI Software Developers as Generalist Agents**, 2024. https://arxiv.org/abs/2407.16741
- Kim et al., **LLMCompiler: An LLM Compiler for Parallel Function Calling**, 2023. https://arxiv.org/abs/2312.04511
- OpenHands Software Agent SDK paper (search hit): https://arxiv.org/html/2511.03690v2 — use if verifying SDK-era architecture claims

---

## Fetch / access notes

| Target | Result |
|--------|--------|
| OpenAI Agents agents + running_agents | OK |
| LangGraph graph-api | OK |
| smolagents ReAct guide | OK |
| OpenHands SDK arch overview | OK |
| SWE-agent architecture | OK |
| mini-SWE-agent overview | OK |
| MCP architecture | OK |
| Claude Agent SDK overview | OK |
| Claude dynamic workflows blog | OK |
| PydanticAI core-concepts | OK (large page) |
| CrewAI agents | OK |
| Letta stateful agents | OK via docs.letta.com |
| AG2 introduction URLs tried | 404 |
| ag2ai/ag2 README via GitHub blob | empty body |
| mini-swe-agent docs/index.md via GitHub blob | empty body |

| DeepSeek Harness GitHub + product + architecture docs | OK |
| Cordis primer + plugins lifecycle + tools tutorial | OK |
| arXiv:2608.25512 abs | Confirmed via search; full PDF fetch timed out once — cite abs URL |

Where fetch failed, claims were limited to search snippets + other live pages, and flagged as provisional in `01-ecosystem-map.md`.

---

## Native language bake-off (`12-native-language.md`) — 2026-09-20 IST

### First-party / competitor
- https://fx.sh/ (homepage binary size line; fetch 2026-09-20)
- https://github.com/vercel-labs/fx (README: Zig 0.16+; WASM embed; binary size line)
- Prior pack measurements: `10-fx-beat-goals.md` (AGENTS.md 7.800 MiB ceiling; Linux 2 ms budgets; measured linux-x86_64 ~11.27 MiB)

### Language size / TLS
- https://github.com/golang/go/issues/81132 (Go 1.24+ crypto/tls / net/http binary size tables)
- https://rusty.today/blog/vgi-injector-a-tiny-https-downloader-in-zig/ (secondary: Zig ~460 KB / Rust ~1.2 MB / Go ~4.5 MB HTTPS injector)
- https://github.com/taliamax/httpget (Rust httpget TLS ~1.3 MB cited in README)
- https://github.com/roobie/zfetch (Zig zfetch ReleaseSmall 2.5 MB with BoringSSL)

### Plugins / WASM / cross-compile
- https://docs.wasmtime.dev/wasip2-plugins.html
- https://github.com/rust-cross/cargo-zigbuild
- https://tartanllama.xyz/posts/wasm-plugins (WASM component plugins overview)

### Local measured smokes (not committed)
- Host: Linux x86_64 Debian 13; rustc/cargo 1.85.1; go 1.24.4; gcc 14.2.0; zig **absent**
- Artifacts under `/tmp/native-bakeoff/` on research host (hello sizes/startups; Go blank-import floors; Rust ureq+rustls 1509912 bytes (~1.44 MiB))
