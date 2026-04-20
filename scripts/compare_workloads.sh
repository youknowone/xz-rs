#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  cat <<'EOF' >&2
Usage: scripts/compare_workloads.sh <encode|decode|size|crc32|crc64> [perf-probe args...]

Examples:
  scripts/compare_workloads.sh encode --input-kind random --size 1048576 --iters 20 --warmup 3
  scripts/compare_workloads.sh decode --input-kind random --size 1048576 --iters 50 --warmup 5
  scripts/compare_workloads.sh size --input-kind random --size 1048576 --iters 2000000 --warmup 200000
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
RUST_SYS_TARGET="target/perf-probe-rust-sys"
C_TARGET="target/perf-probe-c"
RUST_BIN="$RUST_TARGET/release/perf-probe"
RUST_SYS_BIN="$RUST_SYS_TARGET/release/perf-probe"
C_BIN="$C_TARGET/release/perf-probe"

env CARGO_TARGET_DIR="$RUST_TARGET" cargo build -p perf-probe --release --no-default-features --features xz-core >/dev/null
env CARGO_TARGET_DIR="$RUST_SYS_TARGET" cargo build -p perf-probe --release --no-default-features --features xz-sys >/dev/null
env LZMA_API_STATIC=1 CARGO_TARGET_DIR="$C_TARGET" cargo build -p perf-probe --release --no-default-features --features liblzma-sys >/dev/null

if [[ "$WORKLOAD" == "decode" || "$WORKLOAD" == "size" ]]; then
  SIZE=""
  COMPRESSED_INPUT=""
  EXPECTED_SIZE=""
  PASSTHROUGH_ARGS=()
  for ((i = 0; i < ${#RAW_ARGS[@]}; i++)); do
    case "${RAW_ARGS[$i]}" in
      --size)
        SIZE="${RAW_ARGS[$((i + 1))]}"
        ((i += 1))
        ;;
      --compressed-input)
        COMPRESSED_INPUT="${RAW_ARGS[$((i + 1))]}"
        ((i += 1))
        ;;
      --expected-size)
        EXPECTED_SIZE="${RAW_ARGS[$((i + 1))]}"
        ((i += 1))
        ;;
      --input-kind)
        ((i += 1))
        ;;
      *)
        PASSTHROUGH_ARGS+=("${RAW_ARGS[$i]}")
        ;;
    esac
  done

  if [[ -z "$COMPRESSED_INPUT" ]]; then
    if [[ -z "$SIZE" ]]; then
      echo "$WORKLOAD comparison currently requires --size so expected output size is known" >&2
      exit 2
    fi

    COMPRESSED_INPUT="$RESULTS_DIR/${WORKLOAD}-input-${SIZE}.xz"
    "$C_BIN" \
      --workload encode "${RAW_ARGS[@]}" --iters 1 --warmup 0 --save-output "$COMPRESSED_INPUT" >/dev/null
    EXPECTED_SIZE="$SIZE"
  elif [[ -z "$EXPECTED_SIZE" ]]; then
    echo "$WORKLOAD comparison currently requires --expected-size with --compressed-input" >&2
    exit 2
  fi

  RUST_CMD=(
    "$RUST_BIN"
    --workload "$WORKLOAD"
    --compressed-input "$COMPRESSED_INPUT"
    --expected-size "$EXPECTED_SIZE"
    "${PASSTHROUGH_ARGS[@]}"
  )
  RUST_SYS_CMD=(
    "$RUST_SYS_BIN"
    --workload "$WORKLOAD"
    --compressed-input "$COMPRESSED_INPUT"
    --expected-size "$EXPECTED_SIZE"
    "${PASSTHROUGH_ARGS[@]}"
  )
  C_CMD=(
    "$C_BIN"
    --workload "$WORKLOAD"
    --compressed-input "$COMPRESSED_INPUT"
    --expected-size "$EXPECTED_SIZE"
    "${PASSTHROUGH_ARGS[@]}"
  )
else
  RUST_CMD=(
    "$RUST_BIN"
    --workload "$WORKLOAD" "${RAW_ARGS[@]}"
  )
  RUST_SYS_CMD=(
    "$RUST_SYS_BIN"
    --workload "$WORKLOAD" "${RAW_ARGS[@]}"
  )
  C_CMD=(
    "$C_BIN"
    --workload "$WORKLOAD" "${RAW_ARGS[@]}"
  )
fi

hyperfine \
  --shell=none \
  --warmup 2 \
  --export-json "$RESULTS_DIR/${WORKLOAD}.json" \
  --export-markdown "$RESULTS_DIR/${WORKLOAD}.md" \
  --command-name xz-core \
  "${RUST_CMD[*]}" \
  --command-name xz-sys \
  "${RUST_SYS_CMD[*]}" \
  --command-name c \
  "${C_CMD[*]}"

cat <<EOF
Reports written to:
  $RESULTS_DIR/${WORKLOAD}.json
  $RESULTS_DIR/${WORKLOAD}.md
EOF
