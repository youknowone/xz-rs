#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
C_ROOT="$ROOT_DIR/vendor/xz/src/liblzma"
RS_ROOT="$ROOT_DIR/xz-core/src"
TMP_DIR="${TMPDIR:-/tmp}/xz2-rs-porting-audit.$$"
trap 'rm -rf "$TMP_DIR"' EXIT

mkdir -p "$TMP_DIR"

find "$C_ROOT" -type f -name '*.c' \
    | sed "s#^$C_ROOT/##; s#\\.c\$##" \
    | sort > "$TMP_DIR/c-files"

find "$RS_ROOT" -type f -name '*.rs' \
    | sed "s#^$RS_ROOT/##; s#\\.rs\$##" \
    | sort > "$TMP_DIR/rs-files"

echo "direct C/Rust source pairs:"
comm -12 "$TMP_DIR/c-files" "$TMP_DIR/rs-files"
echo

echo "C sources without same-stem Rust file:"
comm -23 "$TMP_DIR/c-files" "$TMP_DIR/rs-files"
echo

echo "Rust sources without same-stem C file:"
comm -13 "$TMP_DIR/c-files" "$TMP_DIR/rs-files"
echo

printf 'summary: direct_pairs=%s c_sources=%s rust_sources=%s\n' \
    "$(comm -12 "$TMP_DIR/c-files" "$TMP_DIR/rs-files" | wc -l | tr -d ' ')" \
    "$(wc -l < "$TMP_DIR/c-files" | tr -d ' ')" \
    "$(wc -l < "$TMP_DIR/rs-files" | tr -d ' ')"
