# xz-core Porting Audit

This tracks the line-by-line audit of the Rust `xz-core` port against
`vendor/xz/src/liblzma`. It is intentionally conservative: an item is not marked
audited until the relevant C and Rust files have been compared directly.

## Scope

- Direct same-stem C/Rust source pairs: 77
- Upstream C sources without same-stem Rust files: generator or alternate
  implementation sources not currently represented as one-to-one Rust modules:
  `check/crc_clmul_consts_gen`, `check/crc32_small`, `check/crc32_tablegen`,
  `check/crc64_small`, `check/crc64_tablegen`, `lzma/fastpos_tablegen`,
  `rangecoder/price_tablegen`.
- Rust-only files include module wrappers, allocator glue, public type exports,
  and tuklib/platform shims.

Regenerate the file map with:

```sh
scripts/porting_audit_map.sh
```

## Audited Items

### `common/auto_decoder`

Compared:

- `vendor/xz/src/liblzma/common/auto_decoder.c`
- `xz-core/src/common/auto_decoder.rs`

Finding:

- The Rust port missed the upstream `.lz`/lzip auto-detect branch for input
  byte `0x4C`.

Fix:

- Added the `lzma_lzip_decoder_init` branch to `xz-core`.
- Added `auto_decoder_accepts_lzip_streams`, comparing auto-decoder output with
  direct lzip decoder output on `vendor/xz/tests/files/good-1-v1.lz`.

### `common/lzip_decoder`

Compared:

- `vendor/xz/src/liblzma/common/lzip_decoder.c`
- `xz-core/src/common/lzip_decoder.rs`
- `vendor/xz/tests/test_lzip_decoder.c`

Finding:

- No source-code mismatch found in the decoded state machine, footer checks,
  `LZMA_CONCATENATED`, `LZMA_IGNORE_CHECK`, `LZMA_TELL_ANY_CHECK`, or memlimit
  retry paths.

Test coverage added:

- Added `tests/lzip_decoder.rs`, porting the core upstream lzip decoder cases to
  Rust integration tests using the vendor `.lz` fixtures:
  - v0/v1 single-member decode with one-byte input chunks
  - concatenated v0/v1 member combinations
  - trailing data handling, including the magic-prefix trailing-data case
  - invalid magic/version/dictionary/footer/checksum fixtures
  - truncated trailing magic returning `LZMA_BUF_ERROR` as `Status::MemNeeded`
  - `LZMA_IGNORE_CHECK`, `LZMA_TELL_ANY_CHECK`, invalid flags, and memlimit
    retry behavior

### `common/block_decoder`

Compared:

- `vendor/xz/src/liblzma/common/block_decoder.c`
- `xz-core/src/common/block_decoder.rs`
- `vendor/xz/src/liblzma/common/block_buffer_decoder.c`
- `xz-core/src/common/block_buffer_decoder.rs`

Finding:

- No source-code mismatch found in Block decoding state transitions, compressed
  and uncompressed size limit calculations, padding consumption, Check
  finalization/verification, `ignore_check`, or the single-call block buffer
  wrapper.

Notes:

- The Rust `SEQ_CODE`/`SEQ_PADDING`/`SEQ_CHECK` structure preserves the C
  fallthrough and re-entry behavior.
- The block buffer wrapper preserves the upstream position-restore behavior on
  errors and the upstream distinction between truncated input
  (`LZMA_DATA_ERROR`) and too-small output (`LZMA_BUF_ERROR`).

### `common/alone_decoder`

Compared:

- `vendor/xz/src/liblzma/common/alone_decoder.c`
- `xz-core/src/common/alone_decoder.rs`

Finding:

- C `uint32_t ++d` wraps in the picky dictionary-size check; Rust debug
  arithmetic panicked on the corresponding overflow.

Fix:

- Replaced `d += 1` with `d = d.wrapping_add(1)`.
- Added `alone_decoder_rejects_wrapping_dictionary_size`.

### `common/stream_encoder_mt`

Compared:

- `vendor/xz/src/liblzma/common/stream_encoder_mt.c`
- `xz-core/src/common/stream_encoder_mt.rs`

Finding:

- The Rust helper split continued into Index/Footer encoding after
  `FullFlush`/`FullBarrier` completed while still in `SEQ_BLOCK`.

Fix:

- Return `LZMA_STREAM_END` immediately when the sequence remains `SEQ_BLOCK`.
- Added `parallel_full_flush_short_chunks_round_trip`.

### `lz/lz_encoder_mf`

Compared:

- `vendor/xz/src/liblzma/lz/lz_encoder_mf.c`
- `xz-core/src/lz/lz_encoder_mf.rs`

Finding:

- C `depth-- == 0` semantics were translated as a Rust decrement before the
  early return, causing debug overflow for preset-0 HC match finding.

Fix:

- Test `depth == 0` before decrementing.
- Added `level_zero_long_round_trip`.

## Wrapper API Findings

### `stream::IGNORE_CHECK`

Finding:

- Public `IGNORE_CHECK` was mapped to `LZMA_TELL_UNSUPPORTED_CHECK` instead of
  `LZMA_IGNORE_CHECK`.

Fix:

- Corrected the constant mapping.
- Added `stream_decoder_ignore_check_skips_integrity_check_verification`.

## Next Audit Targets

The next pass should focus on high-risk files that combine C fallthrough,
unsigned arithmetic, pointer-window state, or feature-condition branches:

- `common/stream_decoder_mt`
- `common/index_decoder`
- `lz/lz_encoder`
- `lzma/lzma_encoder`
- `lzma/lzma_decoder`
- `lzma/lzma2_encoder`
- `lzma/lzma2_decoder`
