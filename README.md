# xz

[![CI](https://github.com/youknowone/xz-rs/actions/workflows/main.yml/badge.svg)](https://github.com/youknowone/xz-rs/actions/workflows/main.yml)
[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/xz.svg
[crates-url]: https://crates.io/crates/xz

[Documentation](https://docs.rs/xz)

`xz` provides xz2/liblzma-compatible APIs backed by a pure Rust implementation
of xz. Its core was built from a c2rust translation of the upstream xz codebase,
then cleaned up to remove translation scaffolding and unnecessary C-shaped
overhead. Unlike `xz2` or `liblzma`, `xz` lets applications eliminate the C
dependency completely while using a Rust-only xz implementation, and it is a
little faster in current benchmarks.

**This crate is forked from [liblzma](https://crates.io/crates/liblzma) and `xz = "0.4.x"` is fully compatible with `liblzma = "0.4.6"`,**
so you can migrate simply.

## Migrate from liblzma

```diff
# Cargo.toml
[dependencies]
-liblzma = "0.4.6"
+xz = "0.4.6"
```

```diff
// *.rs
-use liblzma;
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
- `liblzma-sys` calls the external C liblzma implementation through the
  `liblzma-sys` crate. Use it when you need the C backend instead of the bundled
  Rust implementation.

To use the original C backend:

```toml
xz = { version = "0.4", default-features = false, features = ["liblzma-sys"] }
```

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
