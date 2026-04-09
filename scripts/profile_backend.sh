#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  cat <<'EOF' >&2
Usage: scripts/profile_backend.sh <c|rust|both> <encode|decode|size|crc32|crc64> [backend_probe args...]

Examples:
  scripts/profile_backend.sh rust decode --size 1048576 --iters 500 --warmup 50
  scripts/profile_backend.sh both encode --input-kind random --size 8388608
  scripts/profile_backend.sh rust size --input-kind random --size 1048576 --iters 800 --warmup 80

Environment:
  PROFILER=auto|samply|perf|plain   default: auto
  PROFILE_OUT=<path>                optional profiler output file
EOF
  exit 2
fi

BACKEND="$1"
WORKLOAD="$2"
shift 2

PROFILER="${PROFILER:-auto}"
mkdir -p target/perf-results

case "$BACKEND" in
  c)
    FEATURE="c-sys"
    TARGET_DIR="target/profile-bench-c"
    ;;
  rust)
    FEATURE="rs-sys"
    TARGET_DIR="target/profile-bench-rust"
    ;;
  both)
    echo "profile_backend.sh profiles one backend at a time; use c or rust" >&2
    exit 2
    ;;
  *)
    echo "unknown backend: $BACKEND" >&2
    exit 2
    ;;
esac

COMMON_CMD=(
  cargo run
  -p perf-probe
  --release
  --no-default-features
  --features "$FEATURE"
  --
  --workload "$WORKLOAD"
)
COMMON_CMD+=("$@")

case "$PROFILER" in
  auto)
    if command -v samply >/dev/null 2>&1; then
      PROFILER="samply"
    elif command -v perf >/dev/null 2>&1; then
      PROFILER="perf"
    else
      PROFILER="plain"
    fi
    ;;
esac

if [[ "$PROFILER" == "samply" ]]; then
  OUT="${PROFILE_OUT:-target/perf-results/${BACKEND}-${WORKLOAD}.json}"
  CARGO_TARGET_DIR="$TARGET_DIR" CARGO_PROFILE_RELEASE_DEBUG=1 \
    samply record --save-only -o "$OUT" -- "${COMMON_CMD[@]}"
  echo "profile written to $OUT"
elif [[ "$PROFILER" == "perf" ]]; then
  OUT="${PROFILE_OUT:-target/perf-results/${BACKEND}-${WORKLOAD}.perf.data}"
  CARGO_TARGET_DIR="$TARGET_DIR" CARGO_PROFILE_RELEASE_DEBUG=1 \
    perf record -g --output "$OUT" -- "${COMMON_CMD[@]}"
  echo "profile written to $OUT"
elif [[ "$PROFILER" == "plain" ]]; then
  CARGO_TARGET_DIR="$TARGET_DIR" CARGO_PROFILE_RELEASE_DEBUG=1 "${COMMON_CMD[@]}"
else
  echo "unsupported PROFILER=$PROFILER" >&2
  exit 2
fi
