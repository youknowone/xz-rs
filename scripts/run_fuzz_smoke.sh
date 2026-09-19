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

# FUZZ_TARGETS selects a subset; otherwise every target defined in fuzz/ runs,
# so a newly added target cannot be left out of the smoke run.
targets=()
if [ -n "${FUZZ_TARGETS:-}" ]; then
    read -r -a targets <<< "$FUZZ_TARGETS"
else
    while IFS= read -r line; do
        if [ -n "$line" ]; then
            targets+=("$line")
        fi
    done < <(cd "$ROOT_DIR" && "${CARGO_CMD[@]}" fuzz list)
fi

if [ "${#targets[@]}" -eq 0 ]; then
    echo "no fuzz targets found" >&2
    exit 1
fi

for target in "${targets[@]}"; do
    echo "Running fuzz smoke target: ${target} (sanitizer=${SANITIZER}, runs=${RUNS})"
    (cd "$ROOT_DIR" && "${CARGO_CMD[@]}" fuzz run --sanitizer "$SANITIZER" "$target" -- -runs="$RUNS")
done
