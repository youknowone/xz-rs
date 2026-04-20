# Performance Workflow

This repository now has a repeatable loop for backend comparison, profiling, and code generation inspection.

Important: the C and Rust sys backends must never be linked into the same process when comparing performance. Both export the same `lzma_*` symbols, so shared-process comparisons can silently resolve to the wrong implementation.

The root crate now has three backend modes:

- `xz-core`: direct Rust ABI calls into the pure Rust port
- `xz-sys`: C ABI calls into the pure Rust port through the `xz-sys` shell
- `liblzma-sys`: C ABI calls into vendored C `liblzma`

The comparison scripts in this document force the C backend to build from the vendored
`liblzma-sys/xz` source tree by setting `LZMA_API_STATIC=1`. This avoids silently
benchmarking a system `liblzma` from `pkg-config`, which would make the C/Rust
comparison depend on host packaging and compiler choices instead of the ported source.

## 1. Baseline correctness

Keep correctness green before taking timings:

```bash
cargo test
cargo test --no-default-features --features xz-sys
cargo test --no-default-features --features liblzma-sys
cargo test --test sys_equivalence
```

## 2. Compare the full test suite

Use `hyperfine` to compare end-to-end wall clock time of the deterministic root test bundle (`xz-core` vs C) and `systest` with isolated target directories per backend:

```bash
scripts/compare_backends.sh --runs 10 --warmup 2
```

For the full no-regression gate across the standard root, systest, focused, and
API workloads, use the trimmed comparison wrapper:

```bash
scripts/compare_all_trimmed.sh
```

It measures each Rust/C pair five times, discards the fastest and slowest runs,
averages the remaining runs, and exits with failure if Rust is slower than C
after trimming. Reports are written under `target/perf-results/trimmed/`.

This writes:

- `target/perf-results/root-tests.json`
- `target/perf-results/root-tests.md`
- `target/perf-results/systest.json`
- `target/perf-results/systest.md`

Those reports are the coarse regression gate. Run them after major porting or optimization work.

The root bundle intentionally skips QuickCheck-based unit tests because they generate different random inputs in separate backend processes. Cover those property-style paths with the focused `qc` and `size` workloads instead.

## 3. Compare focused workloads

Use `perf-probe`, a small standalone binary crate that links exactly one backend at a time. `scripts/compare_workloads.sh` compares all three backends in separate processes: direct `xz-core`, `xz-sys`, and vendored C `liblzma-sys`.

Examples:

```bash
scripts/compare_workloads.sh encode --input-kind random --size 1048576 --iters 20 --warmup 3
scripts/compare_workloads.sh decode --input-kind random --size 1048576 --iters 50 --warmup 5
scripts/compare_workloads.sh size --input-kind random --size 1048576 --iters 2000000 --warmup 200000
scripts/compare_workloads.sh crc64 --size 16777216 --iters 400 --warmup 20
scripts/compare_workloads.sh crc64 --size 1048576 --chunk-size 16 --iters 400 --warmup 40
```

This writes workload-specific `hyperfine` reports under `target/perf-results/`.

For tiny inputs, increase `--iters` until one benchmarked command takes at least a few
hundred milliseconds. Otherwise process startup noise can dominate the comparison even when
the actual backend work is near parity. The `size` workload is especially small, so it
still needs millions of in-process iterations; start around `--iters 2000000 --warmup
200000` for a 1 MiB random input and scale up only if the results remain noisy. The
comparison scripts also pre-generate its compressed input so the one-time encode setup
doesn't hide the `uncompressed_size()` path.

There is still a criterion bench in [`benches/backend_comparison.rs`](../benches/backend_comparison.rs), but it now measures one backend per run. Use it only with exactly one backend feature enabled.

For high-level API regressions, compare the root crate against the upstream XZ corpus:

```bash
scripts/compare_api_workloads.sh standard-files --mode all --iters 200 --warmup 20
scripts/compare_api_workloads.sh standard-files --mode good --iters 1000 --warmup 100
scripts/compare_api_workloads.sh standard-files --mode good --name-pattern delta --iters 400 --warmup 40
scripts/compare_api_workloads.sh qc --mode both --cases 128 --max-size 4096 --iters 200 --warmup 20
scripts/compare_api_workloads.sh bufread-trailing --mode both --input-size 1024 --trailing-size 123 --iters 1000 --warmup 100
```

This uses [`examples/standard_files_probe.rs`](../examples/standard_files_probe.rs), which mirrors the `tests/standard_files.rs` `standard_files` path and writes reports to:

- `target/perf-results/api-standard-files.json`
- `target/perf-results/api-standard-files.md`

The `good` subset is small enough that a few hundred iterations can still leave
too much wall-clock noise. Prefer around `--iters 1000 --warmup 100` when
comparing `--mode good` so the in-process work dominates process scheduling
variance.

The `qc` workload uses [`examples/qc_probe.rs`](../examples/qc_probe.rs) to reproduce the
small-input repeated round-trip pattern from the root crate tests. This is useful when
overall regressions show up in `root-tests` even though large encode/decode probes look
good.

The `bufread-trailing` workload uses
[`examples/bufread_trailing_probe.rs`](../examples/bufread_trailing_probe.rs) to reproduce
the `bufread::tests::compressed_and_trailing_data` path with enough in-process repetition
to reduce test process startup noise.

The `size` workload isolates the `uncompressed_size()` path from [`src/lib.rs`](../src/lib.rs),
which corresponds to the QuickCheck-based `tests::size` unit test but with deterministic input.

Use `--name-pattern <substring>` to isolate a file family inside the XZ corpus when a full-corpus comparison is too mixed to identify the remaining gap.

## 4. Profile a focused workload

`perf-probe` is a profiler-friendly executable that runs a single workload many times with deterministic input.

Examples:

```bash
scripts/profile_backend.sh xz-core decode --size 1048576 --iters 800 --warmup 80
scripts/profile_backend.sh xz-core size --input-kind random --size 1048576 --iters 800 --warmup 80
scripts/profile_backend.sh xz-core encode --input-kind random --size 8388608 --iters 150 --warmup 20
scripts/profile_backend.sh c crc64 --size 16777216 --iters 400
```

Useful flags passed through to `perf-probe`:

- `--workload encode|decode|size|crc32|crc64`
- `--input <path>`
- `--compressed-input <path>`
- `--save-output <path>`
- `--input-kind text|random`
- `--size <bytes>`
- `--chunk-size <bytes>`
- `--expected-size <bytes>`
- `--iters <n>`
- `--warmup <n>`
- `--preset <level>`

On macOS the script prefers `samply`; on Linux it falls back to `perf`; otherwise it runs the workload plainly with release debuginfo enabled.

## 5. Inspect the generated code

After a profile points to a hot Rust function, inspect its optimized output:

```bash
scripts/inspect_codegen.sh xz_core::lzma::lzma_encoder::lzma_encode --package xz-core
scripts/inspect_codegen.sh xz_core::check::crc64_fast::lzma_crc64 --package xz-core --format llvm
```

This uses `cargo-asm` and builds under `target/codegen` by default.

## 6. Iterate

The expected loop is:

1. Reproduce the gap with `scripts/compare_backends.sh`.
2. Use `scripts/compare_workloads.sh` to isolate the subsystem.
3. Capture a focused profile with `scripts/profile_backend.sh`.
4. Pick the hottest Rust function from the profile.
5. Inspect its assembly or LLVM IR with `scripts/inspect_codegen.sh`.
6. Change the Rust port, then repeat the same commands.

Keep the input shape, iteration count, and profiler command stable while working a hotspot so before/after numbers stay comparable.
