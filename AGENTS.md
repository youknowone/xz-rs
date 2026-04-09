# AGENTS

## Performance Work

Use the dedicated backend-performance workflow in [`docs/performance-workflow.md`](docs/performance-workflow.md).

Key entry points:

- Full deterministic regression gate: `scripts/compare_backends.sh`
- Focused backend workloads: `scripts/compare_workloads.sh`
- High-level API workloads: `scripts/compare_api_workloads.sh`
- Single-backend profiling: `scripts/profile_backend.sh`
- Optimized code inspection: `scripts/inspect_codegen.sh`

Important constraints:

- Never compare C and Rust sys backends in the same process. They export the same `lzma_*` symbols.
- Treat `scripts/compare_backends.sh` as the coarse deterministic gate.
- Use `qc` and `size` focused workloads for paths that are property-test-like or otherwise input-sensitive.
- Keep workload shape, iteration count, and profiler command stable while iterating on a hotspot.
