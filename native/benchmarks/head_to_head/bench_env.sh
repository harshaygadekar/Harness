# Source this before real runs:  . native/benchmarks/head_to_head/bench_env.sh
# Reviewed proxy pin (aa84ec3 on main = rebased e31767f, same proxy.py blob: timing, transport-error rows, no_run gate).
# run_isolated.sh still REQUIRES PROXY_COMMIT; this file only gives the reviewed value.
export PROXY_COMMIT=aa84ec372054ad02ad2c635925f317209faea452
# Summary builder in the repo (refuses fake and killed runs).
export SUMMARIZER="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/summ.py"
