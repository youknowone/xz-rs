#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  cat <<'EOF' >&2
Usage: scripts/compare_api_workloads.sh <standard-files|qc> [example args...]

Examples:
  scripts/compare_api_workloads.sh standard-files --mode all --iters 200 --warmup 20
  scripts/compare_api_workloads.sh standard-files --mode good --iters 400 --warmup 40
  scripts/compare_api_workloads.sh standard-files --mode good --name-pattern delta --iters 400 --warmup 40
  scripts/compare_api_workloads.sh qc --mode both --cases 128 --max-size 4096 --iters 200 --warmup 20
EOF
  exit 2
fi

WORKLOAD="$1"
shift

if [[ "$WORKLOAD" != "standard-files" && "$WORKLOAD" != "qc" ]]; then
  echo "unsupported API workload: $WORKLOAD" >&2
  exit 2
fi

command -v hyperfine >/dev/null 2>&1 || {
  echo "hyperfine is required for compare_api_workloads.sh" >&2
  exit 1
}

RESULTS_DIR="target/perf-results"
mkdir -p "$RESULTS_DIR"

RAW_ARGS=("$@")
RUST_TARGET="target/api-probe-rust"
C_TARGET="target/api-probe-c"

case "$WORKLOAD" in
  standard-files)
    EXAMPLE_NAME="standard_files_probe"
    ;;
  qc)
    EXAMPLE_NAME="qc_probe"
    ;;
esac

RUST_BIN="$RUST_TARGET/release/examples/$EXAMPLE_NAME"
C_BIN="$C_TARGET/release/examples/$EXAMPLE_NAME"

env CARGO_TARGET_DIR="$RUST_TARGET" cargo build --example "$EXAMPLE_NAME" --release --no-default-features --features rust-backend >/dev/null
env CARGO_TARGET_DIR="$C_TARGET" cargo build --example "$EXAMPLE_NAME" --release --no-default-features --features c-backend >/dev/null

hyperfine \
  --shell=none \
  --export-json "$RESULTS_DIR/api-${WORKLOAD}.json" \
  --export-markdown "$RESULTS_DIR/api-${WORKLOAD}.md" \
  --command-name rust \
  "$RUST_BIN ${RAW_ARGS[*]}" \
  --command-name c \
  "$C_BIN ${RAW_ARGS[*]}"

cat <<EOF
Reports written to:
  $RESULTS_DIR/api-${WORKLOAD}.json
  $RESULTS_DIR/api-${WORKLOAD}.md
EOF
