# Source this before real runs:  . native/benchmarks/head_to_head/bench_env.sh
# Reviewed proxy pin (aa84ec3 on main = rebased e31767f, same proxy.py blob: timing, transport-error rows, no_run gate).
# run_isolated.sh still REQUIRES PROXY_COMMIT; this file only gives the reviewed value.
export PROXY_COMMIT=8ce5135be68f0554e555a25e625b6f1890536e66
# Summary builder in the repo (refuses fake and killed runs).
export SUMMARIZER="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/summ.py"
