# xz

[![CI](https://github.com/youknowone/xz-rs/actions/workflows/main.yml/badge.svg)](https://github.com/youknowone/xz-rs/actions/workflows/main.yml)
[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/xz.svg
[crates-url]: https://crates.io/crates/xz

[Documentation](https://docs.rs/xz)

Pure Rust xz2/liblzma-compatible crates for reading and writing xz streams.

**This crate is forked from [xz2](https://crates.io/crates/xz2) and `xz = "0.1.x"` is fully compatible with `xz2 = "0.1.7"`,**
so you can migrate simply.

## Migrate from xz2

```diff
# Cargo.toml
[dependencies]
-xz2 = "0.1.7"
+xz = "0.1.7"
```

```diff
// *.rs
-use xz2;
+use xz;
```

## Version 0.2.x breaking changes

- XZ upgraded to 5.4
- Multithreading is disabled by default.
  This feature is available by enabling the `parallel` feature
- Support for compiling to WebAssembly

## Version 0.3.x breaking changes

- XZ upgraded to 5.6

## Version 0.4.x breaking changes

- XZ upgraded to 5.8
- Dropped `tokio` support (If you need async I/O, use [`async-compression`](https://github.com/Nullus157/async-compression) crate with `lzma` feature flag)

## Crates and backend selection

This repository contains three pure Rust crates:

- `xz-core` is a direct port of the xz C library internals.
- `xz-sys` is a C ABI compatibility layer backed by `xz-core`. It is intended
  to be compatible with `xz2-sys` and `liblzma-sys`, and should be easy to link
  from C as a liblzma-compatible library.
- `xz` is the high-level Rust interface intended as a replacement for `xz2`
  and `liblzma`.

The high-level `xz` crate defaults to the pure Rust `xz-core` backend. You can
disable default features and choose exactly one backend explicitly:

- `xz-core` calls the pure Rust core directly.
- `xz-sys` calls the pure Rust core through the liblzma-compatible C ABI layer.
- `liblzma-sys` calls the original C liblzma implementation.

To use the original C backend:

```toml
xz = { version = "0.4", default-features = false, features = ["liblzma-sys"] }
```

## Performance snapshot

The table below is a local benchmark snapshot from 2026-04-28 on macOS arm64
(Apple M4 Pro). Lower is better. The C backend was built from the vendored
`liblzma-sys/xz` source tree with `LZMA_API_STATIC=1`, not from a system
`liblzma`.

| Workload | xz-core | xz-sys | C liblzma |
|:--|--:|--:|--:|
| Root test bundle, `scripts/compare_backends.sh --runs 5 --warmup 1` | 632.6 ms | - | 628.8 ms |
| Systest, same command | - | 74.3 ms | 64.2 ms |
| `standard-files --mode good --iters 1000 --warmup 100` | 6.235 s | - | 8.606 s |
| Encode, random 1 MiB, 20 iterations | 2.414 s | 2.446 s | 2.487 s |
| Decode, random 1 MiB, 50 iterations | 50.4 ms | 50.1 ms | 57.1 ms |
| `uncompressed_size`, random 1 MiB, 2,000,000 iterations | 144.8 ms | 139.9 ms | 167.5 ms |
| CRC32, 16 MiB, 400 iterations | 730.1 ms | 927.6 ms | 3.068 s |
| CRC64, 16 MiB, 400 iterations | 4.719 s | 4.784 s | 9.741 s |

Some rows are close enough that scheduler noise can matter, and the CRC64 C
run had large outliers on this machine. Reproduce with the scripts documented
in [`docs/performance-workflow.md`](docs/performance-workflow.md).

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in xz by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
