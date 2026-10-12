# Postfix runs: list/search, loops/edits, labels

Label: weak model (Gemini Flash-Lite) through the proxy; raw, not reviewed.

Input: T7-trackB-*-postfix-*-20261012T00*, S1-trackB-*-postfix-*, T9-trackB-*-postfix-*, T9-trackB-*-postfix15-*.
Excluded: T7-trackB-fx-postfix-F1-20261012T154240 (EXCLUDED.txt). Skipped by tools: T9-trackB-harness-postfix15-H3 (ABORTED, no rec/).
label_tool_calls.py skipped all S1 runs (no task pack match). The tools put postfix15 runs in mode "plain".

Files: list_search.{csv,json}, loops_edits.{csv,json}, labels.{csv,json}.

## Steps to first edit (tool-call index of first edit, 1-based)

| Task | fx | Harness |
|---|---|---|
| S1 | 7, 7, 8 | 12, 12, 11 |
| T7 | 11, 10, 11 | 20, none, 21 |
| T9 postfix | 17, 19, 15 | none, none, none |
| T9 postfix15 | 16, 16, 16 | 22, 17 |

fx: first `edit_file` in `stdout.txt`. Harness: first `write_file` or `apply_patch` in `rec/run.jsonl`. "none" = no edit-tool call and no file change: final.diff is empty, files_modified is empty, cargo_exit 101, oracle failed. See docs/research/19-shell-edits.md.

## List/search (median; min-max)

| Task | Agent | List calls | Search calls | Search-cap hits |
|---|---|---|---|---|
| S1 | fx | 1 | 0 | 0 |
| S1 | harness | 3 | 0 | 0 |
| T7 | fx | 1 | 0 | unknown |
| T7 | harness | 6 (5-6) | 7 | 0 |
| T9 postfix | fx | 2 (1-2) | 1 | unknown |
| T9 postfix | harness | 4 | 4 | 2 (max 3) |
| T9 postfix15 | fx | 1 (1-2) | 1 | unknown |
| T9 postfix15 | harness | 3.5 (3-4) | 4.5 | 2.5 (max 3) |
