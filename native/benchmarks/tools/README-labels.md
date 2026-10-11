# Oracle tool-call labeler

Script: `label_tool_calls.py`. Python 3, standard library only. Bench tooling. It does not touch `native/crates/`.

## What it does

The script reads one run folder. It labels each tool call with one label:

| Label | Rule |
|-------|------|
| useful | First successful read of a real-bug file, or of a file on the oracle "useful read" list. |
| useful | Successful write or patch to a file in the final diff, when the run is green (`cargo_exit` = 0). |
| useful | Test run that first shows failing test names, or that flips fail to pass. |
| unnecessary | Repeat read with the same arguments, and no write to that file between. |
| unnecessary | Repeat search or list with the same arguments, and no write between. |
| unnecessary | Test run with the same command and the same result as an earlier test run. |
| unnecessary | Shell command with the same command and exit code, and no write between. |
| unnecessary | Edit to a decoy file, a forbidden file, or a file that is not in the final diff. |
| unnecessary | Read of a file on the oracle "unnecessary read" list, or any other read when the pack sets `other_reads` to `unnecessary` (T7). |
| neutral | All other calls. A read of a decoy file is neutral, but it is counted in `decoy_reads`. |
| failed | The tool returned an error (for example a patch that did not match, a rejected shell command, or a test tool timeout). |

An edit to a final-diff file in a run that is not green is neutral.

## Oracle data

Each pack has `oracle_labels.json` next to its `ORACLE.md`:

- `bug_files`: files with the real bugs.
- `useful_reads`: other files that the oracle marks as useful to read.
- `decoy_files`: decoys. A path that ends in `/` is a prefix.
- `unnecessary_reads`: reads that the oracle marks as unnecessary.
- `other_reads`: label for all other reads (`neutral` or `unnecessary`).
- `forbidden_edit_files`: edits here are always unnecessary.

If you change `ORACLE.md`, update `oracle_labels.json` too.

## Run formats

- Harness: `rec/run.jsonl`. The script pairs `tool_call` and `tool_result` events by `id`. `run_tests`, and `run_shell` with `cargo test`, are test calls. A run is complete if it has a `done` event.
- fx: `stdout.txt` holds one JSON object with `tool_calls` (name and status, no arguments). `stderr.txt` holds one activity line per tool call at column 0 (`Reading <path>`, `Editing <path>`, `Writing <path>`, `Matching <glob>`, `Searching <query>`, `Running <command>`). The script zips the two lists in order. If the counts differ, the script stops with an error for that run. Test output for a `Running cargo test` call is the text after that line. A run is complete if it has an `agent_exit` file and a valid `stdout.txt`.

Category map:

| Category | Harness | fx |
|----------|---------|----|
| read | read_file | read_file |
| search | search | grep_files |
| list | list_files | glob_files |
| edit | apply_patch | edit_file |
| write | write_file | write_file |
| test | run_tests, run_shell `cargo test...` | shell `cargo test...` |
| shell | run_shell, git_diff, git_status | shell |

## Limits

- fx does not record read sizes. For fx, `largest_tool_result_bytes` covers shell calls only (stdout + stderr bytes).
- fx shows "Editing file" with no path for some failed edits. Such an edit has no known target.
- The script cannot see read ranges. Any first read of a useful file is useful.
- Labels for a live model are a best effort. They are not a full replay check.

## Usage

```bash
cd native/benchmarks/head_to_head

# One run, JSON to stdout
python3 ../tools/label_tool_calls.py run runs/<run_folder>

# Many runs: per-run files, summary JSON and CSV (median/min/max per task, agent, mode)
python3 ../tools/label_tool_calls.py summary runs/T1v2-*trackB* runs/T7-* runs/T9-* \
  --per-run-dir labels_out/runs --json labels_out/summary.json --csv labels_out/summary.csv
```

The summary skips runs that it cannot parse, and runs that are not complete. It prints the reason to stderr. Use `--include-incomplete` to keep incomplete runs.

Output holds tool names, categories, labels and reasons only. It holds no tool arguments, no tool results and no model text. `labels_out/` is in `native/.gitignore`. The committed table is `docs/benchmarks/data/phase4_tool_labels.csv` (counts only).

## Tests

```bash
cd native/benchmarks/tools && python3 -m unittest test_label_tool_calls -v
```
