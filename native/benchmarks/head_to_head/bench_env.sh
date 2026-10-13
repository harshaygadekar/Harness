# Source this before real runs:  . native/benchmarks/head_to_head/bench_env.sh
# Reviewed proxy pin (chore/proxy-timing e31767f: timing, transport-error rows, no_run gate).
# run_isolated.sh still REQUIRES PROXY_COMMIT; this file only gives the reviewed value.
export PROXY_COMMIT=e31767f3ac129d2d97dc1f4b8a2bdbcc23e98c62
# Summary builder in the repo (refuses fake and killed runs).
export SUMMARIZER="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/summ.py"
