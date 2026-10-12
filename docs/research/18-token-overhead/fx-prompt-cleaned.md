# fx 0.0.13 first request: counts only

Label: no model call. Captured with a fake upstream (see 18-token-overhead.md, section 8).

The fx license is not checked. Thus this file holds counts and short summaries only. It holds no fx prompt text and no fx tool text. The full local copy is outside git, in `/workspace/local-only/fx-prompt/`.

Token counts are approximate (tiktoken cl100k and chars/4; not the Gemini tokenizer). fx token numbers from runs are self-reported; fx --auto review requests not counted.

## Totals (T1v2; T7 in brackets)

| Item | Value |
|---|---|
| Total body chars | 28,257 (28,322) |
| Total tokens, cl100k | 6,334 (6,340) |
| Total tokens, chars/4 | 7,064 (7,081) |
| Messages | 8: 7 system, 1 user |
| System prompt chars | 8,161 (8,155) |
| System prompt tokens, cl100k | 1,647 (1,641) |
| Tool count | 14 |
| Tool schema chars | 18,642 |
| Tool schema tokens, cl100k | 4,243 |
| User prompt chars | 931 (1,016) |
| Other request fields | model, stream, stream_options, max_tokens (8,192) |

## System parts (chars, T1v2)

| # | Chars | Summary |
|---|---|---|
| 1 | 5,900 | Base prompt: identity, workspace behavior, source routing, interaction style, safety, tools and verification. |
| 2 | 440 | Note on a public web search capability. |
| 3 | 88 | Note that project rules were not loaded (workspace not below home). |
| 4 | 467 | Note on configured MCP servers and when to search for them. |
| 5 | 497 | Turn context: workspace root, current directory, OS, shell, date. |
| 6 | 564 | Runtime context: permission mode is auto and how actions are reviewed. |
| 7 | 205 | Response language control. |

## Tool names (14)

read_file, glob_files, grep_files, edit_file, write_file, shell, capability_search, skill, install_skill, mcp_select_tool, mcp_features, ask_user_question, web_fetch, read_tool_result.

## For comparison: Harness (from the Harness body in `bodies/`)

0 system messages, 9 tools, 2,378 tool schema chars (630 cl100k tokens), 3,452 total body chars (920 cl100k tokens) on T1v2.
