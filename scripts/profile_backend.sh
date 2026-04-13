#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  cat <<'EOF' >&2
Usage: scripts/profile_backend.sh <c|liblzma-sys|xz|rust|xz-sys> <encode|decode|size|crc32|crc64> [backend_probe args...]

Examples:
  scripts/profile_backend.sh xz decode --size 1048576 --iters 500 --warmup 50
  scripts/profile_backend.sh xz size --input-kind random --size 1048576 --iters 800 --warmup 80

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
  c|liblzma-sys)
    FEATURE="liblzma-sys"
    TARGET_DIR="target/profile-bench-c"
    BACKEND_ENV=(LZMA_API_STATIC=1)
    ;;
  xz|rust)
    FEATURE="xz"
    TARGET_DIR="target/profile-bench-rust"
    BACKEND_ENV=()
    ;;
  xz-sys)
    FEATURE="xz-sys"
    TARGET_DIR="target/profile-bench-xz-sys"
    BACKEND_ENV=()
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
  env "${BACKEND_ENV[@]}" CARGO_TARGET_DIR="$TARGET_DIR" CARGO_PROFILE_RELEASE_DEBUG=1 \
    samply record --save-only -o "$OUT" -- "${COMMON_CMD[@]}"
  echo "profile written to $OUT"
elif [[ "$PROFILER" == "perf" ]]; then
  OUT="${PROFILE_OUT:-target/perf-results/${BACKEND}-${WORKLOAD}.perf.data}"
  env "${BACKEND_ENV[@]}" CARGO_TARGET_DIR="$TARGET_DIR" CARGO_PROFILE_RELEASE_DEBUG=1 \
    perf record -g --output "$OUT" -- "${COMMON_CMD[@]}"
  echo "profile written to $OUT"
elif [[ "$PROFILER" == "plain" ]]; then
  env "${BACKEND_ENV[@]}" CARGO_TARGET_DIR="$TARGET_DIR" CARGO_PROFILE_RELEASE_DEBUG=1 "${COMMON_CMD[@]}"
else
  echo "unsupported PROFILER=$PROFILER" >&2
  exit 2
fi
