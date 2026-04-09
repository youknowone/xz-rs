#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  cat <<'EOF' >&2
Usage: scripts/compare_workloads.sh <encode|decode|size|crc32|crc64> [perf-probe args...]

Examples:
  scripts/compare_workloads.sh encode --input-kind random --size 1048576 --iters 20 --warmup 3
  scripts/compare_workloads.sh decode --input-kind random --size 1048576 --iters 50 --warmup 5
  scripts/compare_workloads.sh size --input-kind random --size 1048576 --iters 400 --warmup 40
EOF
  exit 2
fi

WORKLOAD="$1"
shift

command -v hyperfine >/dev/null 2>&1 || {
  echo "hyperfine is required for compare_workloads.sh" >&2
  exit 1
}

RESULTS_DIR="target/perf-results"
mkdir -p "$RESULTS_DIR"

RAW_ARGS=("$@")
RUST_TARGET="target/perf-probe-rust"
C_TARGET="target/perf-probe-c"
RUST_BIN="$RUST_TARGET/release/perf-probe"
C_BIN="$C_TARGET/release/perf-probe"

env CARGO_TARGET_DIR="$RUST_TARGET" cargo build -p perf-probe --release --no-default-features --features rs-sys >/dev/null
env CARGO_TARGET_DIR="$C_TARGET" cargo build -p perf-probe --release --no-default-features --features c-sys >/dev/null

if [[ "$WORKLOAD" == "decode" ]]; then
  SIZE=""
  for ((i = 0; i < ${#RAW_ARGS[@]}; i++)); do
    if [[ "${RAW_ARGS[$i]}" == "--size" ]]; then
      SIZE="${RAW_ARGS[$((i + 1))]}"
      break
    fi
  done
  if [[ -z "$SIZE" ]]; then
    echo "decode comparison currently requires --size so expected output size is known" >&2
    exit 2
  fi

  COMPRESSED_INPUT="$RESULTS_DIR/decode-input-${SIZE}.xz"
  "$C_BIN" \
    --workload encode "${RAW_ARGS[@]}" --iters 1 --warmup 0 --save-output "$COMPRESSED_INPUT" >/dev/null

  RUST_CMD=(
    "$RUST_BIN"
    --workload decode
    --compressed-input "$COMPRESSED_INPUT"
    --expected-size "$SIZE"
  )
  C_CMD=(
    "$C_BIN"
    --workload decode
    --compressed-input "$COMPRESSED_INPUT"
    --expected-size "$SIZE"
  )
else
  RUST_CMD=(
    "$RUST_BIN"
    --workload "$WORKLOAD" "${RAW_ARGS[@]}"
  )
  C_CMD=(
    "$C_BIN"
    --workload "$WORKLOAD" "${RAW_ARGS[@]}"
  )
fi

hyperfine \
  --shell=none \
  --export-json "$RESULTS_DIR/${WORKLOAD}.json" \
  --export-markdown "$RESULTS_DIR/${WORKLOAD}.md" \
  --command-name rust \
  "${RUST_CMD[*]}" \
  --command-name c \
  "${C_CMD[*]}"

cat <<EOF
Reports written to:
  $RESULTS_DIR/${WORKLOAD}.json
  $RESULTS_DIR/${WORKLOAD}.md
EOF
