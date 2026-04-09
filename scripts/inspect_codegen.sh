#!/usr/bin/env bash
set -euo pipefail

print_usage() {
  cat <<'EOF'
Usage: scripts/inspect_codegen.sh <symbol> [options]

Options:
  --package <name>     Cargo package to inspect (default: liblzma-rs)
  --format <asm|llvm|mir>  Output format for cargo-asm (default: asm)
  --features <list>    Cargo feature list passed to cargo-asm
  --target-dir <dir>   Cargo target dir (default: target/codegen)

Example:
  scripts/inspect_codegen.sh liblzma_rs::lzma::lzma2_encoder::lzma2_encode \
    --package liblzma-rs --format asm
EOF
}

if [[ $# -gt 0 && ( "$1" == "--help" || "$1" == "-h" ) ]]; then
  print_usage
  exit 0
fi

if [[ $# -lt 1 ]]; then
  print_usage
  exit 2
fi

SYMBOL="$1"
shift

PACKAGE="liblzma-rs"
FORMAT="asm"
FEATURES=""
TARGET_DIR="target/codegen"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --package)
      PACKAGE="$2"
      shift 2
      ;;
    --format)
      FORMAT="$2"
      shift 2
      ;;
    --features)
      FEATURES="$2"
      shift 2
      ;;
    --target-dir)
      TARGET_DIR="$2"
      shift 2
      ;;
    --help|-h)
      print_usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

command -v cargo-asm >/dev/null 2>&1 || {
  echo "cargo-asm is required for inspect_codegen.sh" >&2
  exit 1
}

ARGS=(
  --manifest-path Cargo.toml
  --target-dir "$TARGET_DIR"
  -p "$PACKAGE"
  --lib
  --release
  --this-workspace
  "--$FORMAT"
  "$SYMBOL"
)

if [[ -n "$FEATURES" ]]; then
  ARGS+=(--features "$FEATURES")
fi

cargo-asm "${ARGS[@]}"
