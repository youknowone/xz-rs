#!/usr/bin/env bash
set -euo pipefail

RUNS=5
WARMUP=2
ROOT_REPEATS=5
RESULTS_DIR="target/perf-results/trimmed"
TOLERANCE="0.0"

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
    --root-repeats)
      ROOT_REPEATS="$2"
      shift 2
      ;;
    --results-dir)
      RESULTS_DIR="$2"
      shift 2
      ;;
    --tolerance)
      TOLERANCE="$2"
      shift 2
      ;;
    --help|-h)
      cat <<'EOF'
Usage: scripts/compare_all_trimmed.sh [options]

Run the standard C-vs-Rust backend comparison set. Each pair is measured
five times by default; the fastest and slowest runs are discarded and the
remaining runs are averaged. The script exits with failure if any Rust
measurement is slower than C after trimming.

Options:
  --runs <n>          Number of measured runs per pair (default: 5)
  --warmup <n>        Hyperfine warmup runs per pair (default: 2)
  --root-repeats <n>  Repeat root test binaries inside each timed command (default: 5)
  --results-dir <dir> Output directory (default: target/perf-results/trimmed)
  --tolerance <frac>  Allowed Rust slowdown fraction before failure (default: 0.0)
EOF
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

if [[ "$RUNS" -lt 3 ]]; then
  echo "--runs must be at least 3 so min/max can be discarded" >&2
  exit 2
fi

command -v hyperfine >/dev/null 2>&1 || {
  echo "hyperfine is required" >&2
  exit 1
}

command -v python3 >/dev/null 2>&1 || {
  echo "python3 is required" >&2
  exit 1
}

mkdir -p "$RESULTS_DIR"
SUMMARY_TSV="$RESULTS_DIR/benchmarks.tsv"
: >"$SUMMARY_TSV"

RUST_TARGET="target/perf-rust"
C_TARGET="target/perf-c"

PROBE_RUST_BIN="$RUST_TARGET/release/perf-probe"
PROBE_C_BIN="$C_TARGET/release/perf-probe"

API_RUST_EXAMPLES="$RUST_TARGET/release/examples"
API_C_EXAMPLES="$C_TARGET/release/examples"

record_pair() {
  local stem="$1"
  local label="$2"
  local rust_name="$3"
  local rust_cmd="$4"
  local c_name="$5"
  local c_cmd="$6"

  printf '%s\t%s\t%s\t%s\n' "$stem" "$label" "$rust_name" "$c_name" >>"$SUMMARY_TSV"

  hyperfine \
    --shell=none \
    --runs "$RUNS" \
    --warmup "$WARMUP" \
    --export-json "$RESULTS_DIR/${stem}.json" \
    --export-markdown "$RESULTS_DIR/${stem}.md" \
    --command-name "$rust_name" \
    "$rust_cmd" \
    --command-name "$c_name" \
    "$c_cmd"
}

echo "prebuilding root test binaries..."
env CARGO_TARGET_DIR="$RUST_TARGET" \
  cargo test -p xz --release --no-default-features --features xz-core --lib --tests --no-run >/dev/null
env LZMA_API_STATIC=1 CARGO_TARGET_DIR="$C_TARGET" \
  cargo test --release --no-default-features --features liblzma-sys --lib --tests --no-run >/dev/null

echo "prebuilding systest binaries..."
env CARGO_TARGET_DIR="$RUST_TARGET" \
  cargo test -p systest --release --no-default-features --features xz-sys --no-run >/dev/null
env LZMA_API_STATIC=1 CARGO_TARGET_DIR="$C_TARGET" \
  cargo test -p systest --release --no-default-features --features liblzma-sys --no-run >/dev/null

echo "prebuilding focused workload probes..."
env CARGO_TARGET_DIR="$RUST_TARGET" \
  cargo build -p perf-probe --release --no-default-features --features xz-core >/dev/null
env LZMA_API_STATIC=1 CARGO_TARGET_DIR="$C_TARGET" \
  cargo build -p perf-probe --release --no-default-features --features liblzma-sys >/dev/null

echo "prebuilding API workload probes..."
for example in standard_files_probe qc_probe bufread_trailing_probe; do
  env CARGO_TARGET_DIR="$RUST_TARGET" \
    cargo build --example "$example" --release --no-default-features --features xz-core >/dev/null
  env LZMA_API_STATIC=1 CARGO_TARGET_DIR="$C_TARGET" \
    cargo build --example "$example" --release --no-default-features --features liblzma-sys >/dev/null
done

DECODE_INPUT="$RESULTS_DIR/decode-input-1048576.xz"
"$PROBE_C_BIN" \
  --workload encode \
  --input-kind random \
  --size 1048576 \
  --iters 1 \
  --warmup 0 \
  --save-output "$DECODE_INPUT" >/dev/null

record_pair \
  root-tests \
  "root test bundle" \
  xz-core-tests \
  "./scripts/run_root_test_bins.sh $RUST_TARGET $ROOT_REPEATS" \
  c-tests \
  "./scripts/run_root_test_bins.sh $C_TARGET $ROOT_REPEATS"

record_pair \
  systest \
  "systest" \
  xz-sys-systest \
  "env CARGO_TARGET_DIR=$RUST_TARGET cargo test -p systest --release --no-default-features --features xz-sys -- --test-threads=1" \
  c-systest \
  "env LZMA_API_STATIC=1 CARGO_TARGET_DIR=$C_TARGET cargo test -p systest --release --no-default-features --features liblzma-sys -- --test-threads=1"

record_pair \
  encode \
  "focused encode random 1MiB" \
  xz-core \
  "$PROBE_RUST_BIN --workload encode --input-kind random --size 1048576 --iters 300 --warmup 20" \
  c \
  "$PROBE_C_BIN --workload encode --input-kind random --size 1048576 --iters 300 --warmup 20"

record_pair \
  decode \
  "focused decode random 1MiB" \
  xz-core \
  "$PROBE_RUST_BIN --workload decode --compressed-input $DECODE_INPUT --expected-size 1048576 --iters 800 --warmup 80" \
  c \
  "$PROBE_C_BIN --workload decode --compressed-input $DECODE_INPUT --expected-size 1048576 --iters 800 --warmup 80"

record_pair \
  size \
  "focused uncompressed_size random 1MiB" \
  xz-core \
  "$PROBE_RUST_BIN --workload size --compressed-input $DECODE_INPUT --expected-size 1048576 --iters 2000000 --warmup 200000" \
  c \
  "$PROBE_C_BIN --workload size --compressed-input $DECODE_INPUT --expected-size 1048576 --iters 2000000 --warmup 200000"

record_pair \
  crc32 \
  "focused crc32 16MiB" \
  xz-core \
  "$PROBE_RUST_BIN --workload crc32 --size 16777216 --iters 300 --warmup 20" \
  c \
  "$PROBE_C_BIN --workload crc32 --size 16777216 --iters 300 --warmup 20"

record_pair \
  crc64-large \
  "focused crc64 16MiB" \
  xz-core \
  "$PROBE_RUST_BIN --workload crc64 --size 16777216 --iters 400 --warmup 20" \
  c \
  "$PROBE_C_BIN --workload crc64 --size 16777216 --iters 400 --warmup 20"

record_pair \
  crc64-smallchunk \
  "focused crc64 1MiB chunk 16" \
  xz-core \
  "$PROBE_RUST_BIN --workload crc64 --size 1048576 --chunk-size 16 --iters 400 --warmup 40" \
  c \
  "$PROBE_C_BIN --workload crc64 --size 1048576 --chunk-size 16 --iters 400 --warmup 40"

record_pair \
  api-standard-files-all \
  "API standard files all" \
  xz-core \
  "$API_RUST_EXAMPLES/standard_files_probe --mode all --iters 200 --warmup 20" \
  c \
  "$API_C_EXAMPLES/standard_files_probe --mode all --iters 200 --warmup 20"

record_pair \
  api-standard-files-good \
  "API standard files good" \
  xz-core \
  "$API_RUST_EXAMPLES/standard_files_probe --mode good --iters 1000 --warmup 100" \
  c \
  "$API_C_EXAMPLES/standard_files_probe --mode good --iters 1000 --warmup 100"

record_pair \
  api-standard-files-delta \
  "API standard files good delta" \
  xz-core \
  "$API_RUST_EXAMPLES/standard_files_probe --mode good --name-pattern delta --iters 400 --warmup 40" \
  c \
  "$API_C_EXAMPLES/standard_files_probe --mode good --name-pattern delta --iters 400 --warmup 40"

record_pair \
  api-qc \
  "API qc both" \
  xz-core \
  "$API_RUST_EXAMPLES/qc_probe --mode both --cases 128 --max-size 4096 --iters 200 --warmup 20" \
  c \
  "$API_C_EXAMPLES/qc_probe --mode both --cases 128 --max-size 4096 --iters 200 --warmup 20"

record_pair \
  api-bufread-trailing \
  "API bufread trailing" \
  xz-core \
  "$API_RUST_EXAMPLES/bufread_trailing_probe --mode both --input-size 1024 --trailing-size 123 --iters 1000 --warmup 100" \
  c \
  "$API_C_EXAMPLES/bufread_trailing_probe --mode both --input-size 1024 --trailing-size 123 --iters 1000 --warmup 100"

python3 - "$RESULTS_DIR" "$SUMMARY_TSV" "$TOLERANCE" <<'PY'
import json
import sys
from pathlib import Path

results_dir = Path(sys.argv[1])
summary_tsv = Path(sys.argv[2])
tolerance = float(sys.argv[3])


def trimmed_mean(times):
    values = sorted(float(t) for t in times)
    if len(values) >= 3:
        values = values[1:-1]
    return sum(values) / len(values)


rows = []
failed = []

for line in summary_tsv.read_text().splitlines():
    stem, label, rust_name, c_name = line.split("\t")
    data = json.loads((results_dir / f"{stem}.json").read_text())
    by_name = {entry["command"]: entry for entry in data["results"]}
    rust = trimmed_mean(by_name[rust_name]["times"])
    c = trimmed_mean(by_name[c_name]["times"])
    ratio = c / rust
    allowed = c * (1.0 + tolerance)
    status = "PASS" if rust <= allowed else "FAIL"
    if status == "FAIL":
        failed.append(label)
    rows.append((label, rust, c, ratio, status))

md = results_dir / "trimmed-summary.md"
json_path = results_dir / "trimmed-summary.json"

with md.open("w") as f:
    f.write("# Trimmed Backend Comparison\n\n")
    f.write(f"- measured runs per pair: {len(by_name[rust_name]['times'])}\n")
    f.write("- aggregation: discard min and max, average remaining runs\n")
    f.write(f"- allowed Rust slowdown tolerance: {tolerance:.3f}\n\n")
    f.write("| workload | Rust trimmed mean | C trimmed mean | C/Rust ratio | status |\n")
    f.write("|---|---:|---:|---:|---|\n")
    for label, rust, c, ratio, status in rows:
        f.write(
            f"| {label} | {rust:.6f}s | {c:.6f}s | {ratio:.3f}x | {status} |\n"
        )

json_path.write_text(
    json.dumps(
        [
            {
                "workload": label,
                "rust_trimmed_mean_seconds": rust,
                "c_trimmed_mean_seconds": c,
                "c_over_rust_ratio": ratio,
                "status": status,
            }
            for label, rust, c, ratio, status in rows
        ],
        indent=2,
    )
    + "\n"
)

print(f"Summary written to {md}")
print(f"Machine-readable summary written to {json_path}")

if failed:
    print("Rust was slower than C after trimming for:", file=sys.stderr)
    for label in failed:
        print(f"  - {label}", file=sys.stderr)
    sys.exit(1)
PY
