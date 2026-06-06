#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
RUNS="${FUZZ_RUNS:-256}"
TOOLCHAIN="${FUZZ_TOOLCHAIN:-nightly}"
SANITIZER="${FUZZ_SANITIZER:-none}"
CARGO_CMD=(cargo)

if [ -n "$TOOLCHAIN" ]; then
    CARGO_CMD=(cargo "+$TOOLCHAIN")
fi

if ! (cd "$ROOT_DIR" && "${CARGO_CMD[@]}" fuzz --version >/dev/null 2>&1); then
    echo "cargo-fuzz is required. Install with: cargo install cargo-fuzz" >&2
    exit 1
fi

targets=(
    stream_encode
    auto_decode
    stream_decode
    alone_decode
    mt_stream_decode
    mt_stream_encode
)

for target in "${targets[@]}"; do
    echo "Running fuzz smoke target: ${target}"
    (cd "$ROOT_DIR" && "${CARGO_CMD[@]}" fuzz run --sanitizer "$SANITIZER" "$target" -- -runs="$RUNS")
done
