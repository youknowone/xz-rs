#!/usr/bin/env bash
set -euo pipefail

RUNS=5
WARMUP=1
RESULTS_DIR="target/perf-results"
INCLUDE_SYSTEST=1
ROOT_REPEATS=5

while [[ $# -gt 0 ]]; do
  case "$1" in
    --runs)
      RUNS="$2"
      shift 2
      ;;
    --warmup)
      WARMUP="$2"
      shift 2
      ;;
    --results-dir)
      RESULTS_DIR="$2"
      shift 2
      ;;
    --skip-systest)
      INCLUDE_SYSTEST=0
      shift
      ;;
    --root-repeats)
      ROOT_REPEATS="$2"
      shift 2
      ;;
    --help|-h)
      cat <<'EOF'
Usage: scripts/compare_backends.sh [options]

Compare full test-suite wall clock time for the direct `xz-core` backend and C backend using hyperfine.
Results are written under target/perf-results by default.

Options:
  --runs <n>          Number of measured runs per command (default: 5)
  --warmup <n>        Warmup runs per command (default: 1)
  --root-repeats <n>  Repeat the root test-binary bundle inside each timed command
  --results-dir <dir> Where to write hyperfine reports
  --skip-systest      Only compare the root crate test suite
EOF
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

command -v hyperfine >/dev/null 2>&1 || {
  echo "hyperfine is required for compare_backends.sh" >&2
  exit 1
}

mkdir -p "$RESULTS_DIR"

ROOT_RUST_TARGET="target/perf-root-rust"
ROOT_C_TARGET="target/perf-root-c"
SYSTEST_RUST_TARGET="target/perf-systest-rust"
SYSTEST_C_TARGET="target/perf-systest-c"

ROOT_RUST_CMD="./scripts/run_root_test_bins.sh $ROOT_RUST_TARGET $ROOT_REPEATS"
ROOT_C_CMD="./scripts/run_root_test_bins.sh $ROOT_C_TARGET $ROOT_REPEATS"
SYSTEST_RUST_CMD="env CARGO_TARGET_DIR=$SYSTEST_RUST_TARGET cargo test -p systest --release --no-default-features --features xz-sys -- --test-threads=1"
SYSTEST_C_CMD="env LZMA_API_STATIC=1 CARGO_TARGET_DIR=$SYSTEST_C_TARGET cargo test -p systest --release --no-default-features --features liblzma-sys -- --test-threads=1"

echo "prebuilding root test binaries..."
env CARGO_TARGET_DIR="$ROOT_RUST_TARGET" cargo test -p xz --release --no-default-features --features xz-core --no-run >/dev/null
env LZMA_API_STATIC=1 CARGO_TARGET_DIR="$ROOT_C_TARGET" cargo test --release --no-default-features --features liblzma-sys --no-run >/dev/null

hyperfine \
  --shell=none \
  --runs "$RUNS" \
  --warmup "$WARMUP" \
  --export-json "$RESULTS_DIR/root-tests.json" \
  --export-markdown "$RESULTS_DIR/root-tests.md" \
  --command-name xz-core-tests \
  "$ROOT_RUST_CMD" \
  --command-name c-tests \
  "$ROOT_C_CMD"

if [[ "$INCLUDE_SYSTEST" -eq 1 ]]; then
  echo "prebuilding systest binaries..."
  env CARGO_TARGET_DIR="$SYSTEST_RUST_TARGET" cargo test -p systest --release --no-default-features --features xz-sys --no-run >/dev/null
  env LZMA_API_STATIC=1 CARGO_TARGET_DIR="$SYSTEST_C_TARGET" cargo test -p systest --release --no-default-features --features liblzma-sys --no-run >/dev/null

  hyperfine \
    --shell=none \
    --runs "$RUNS" \
    --warmup "$WARMUP" \
    --export-json "$RESULTS_DIR/systest.json" \
    --export-markdown "$RESULTS_DIR/systest.md" \
    --command-name xz-sys-systest \
    "$SYSTEST_RUST_CMD" \
    --command-name c-systest \
    "$SYSTEST_C_CMD"
fi

cat <<EOF
Reports written to:
  $RESULTS_DIR/root-tests.json
  $RESULTS_DIR/root-tests.md
EOF

if [[ "$INCLUDE_SYSTEST" -eq 1 ]]; then
  cat <<EOF
  $RESULTS_DIR/systest.json
  $RESULTS_DIR/systest.md
EOF
fi
