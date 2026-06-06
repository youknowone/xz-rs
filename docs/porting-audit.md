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

### `common/index_decoder`

Compared:

- `vendor/xz/src/liblzma/common/index_decoder.c`
- `xz-core/src/common/index_decoder.rs`

Finding:

- No source-code mismatch found in Index indicator parsing, VLI Record count and
  size parsing, post-decrement Record counting, padding consumption, CRC32
  accumulation/verification, memlimit handling, reset/init/end behavior, or the
  single-call `lzma_index_buffer_decode` wrapper.

Notes:

- The Rust loop/`match` structure preserves the C `switch` fallthrough between
  `SEQ_COUNT` -> `SEQ_MEMUSAGE`, `SEQ_PADDING_INIT` -> `SEQ_PADDING`, and
  `SEQ_PADDING` -> `SEQ_CRC32`.
- The Rust `count -= 1` occurs before the zero check, matching C
  `--coder->count == 0`.
- Existing stream corpus tests exercise Index decoding indirectly. A later pass
  should port the upstream raw Index tests from `vendor/xz/tests/test_index.c`
  and `vendor/xz/tests/test_index_hash.c`.

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

### `lz/lz_encoder`

Compared:

- `vendor/xz/src/liblzma/lz/lz_encoder.c`
- `vendor/xz/src/liblzma/lz/lz_encoder.h`
- `vendor/xz/src/liblzma/common/memcmplen.h`
- `xz-core/src/lz/lz_encoder.rs`

Finding:

- No source-code mismatch found in the sliding-window move, window fill,
  flush/finish read-limit handling, pending-byte match-finder rewind, main
  encode loop, match-finder selection and sizing, preset dictionary handling,
  memory-usage calculation, end/update behavior, output-limit delegation, or
  match-finder support reporting.

Notes:

- C `memmove()` in `move_window()` is represented by Rust `ptr::copy()`, which
  permits overlap.
- Rust keeps `LZMA_MEMCMPLEN_EXTRA` at zero because its `lzma_memcmplen`
  implementation uses the portable byte-by-byte path instead of C's optional
  unaligned word/SIMD reads.
- The `lz.coder` free path differs mechanically because the default Rust
  allocator requires typed deallocation; this matches the same policy used in
  `lz/lz_decoder.rs` and doesn't change valid encoder behavior.

### `lzma/lzma2_encoder`

Compared:

- `vendor/xz/src/liblzma/lzma/lzma2_encoder.c`
- `vendor/xz/src/liblzma/lzma/lzma2_encoder.h`
- `xz-core/src/lzma/lzma2_encoder.rs`

Finding:

- No source-code mismatch found in LZMA2 chunk header construction,
  uncompressed-chunk fallback, state/property/dictionary reset flags,
  partial header/data copy state, options update, initialization, memory-usage
  reporting, properties encoding, or recommended block-size calculation.

Notes:

- The Rust sequence checks preserve the C fallthrough among
  `SEQ_INIT` -> `SEQ_LZMA_ENCODE` -> `SEQ_LZMA_COPY` and
  `SEQ_UNCOMPRESSED_HEADER` -> `SEQ_UNCOMPRESSED_COPY`.
- C `--d` in property encoding is represented by Rust `d -= 1`; the preceding
  `max(dict_size, LZMA_DICT_SIZE_MIN)` keeps it away from underflow.

### `lzma/lzma2_decoder`

Compared:

- `vendor/xz/src/liblzma/lzma/lzma2_decoder.c`
- `xz-core/src/lzma/lzma2_decoder.rs`

Finding:

- No source-code mismatch found in control-byte parsing, reserved control-byte
  rejection, dictionary/property/state reset requirements, compressed and
  uncompressed size decoding, LZMA chunk completion checks, raw copy chunks,
  initialization/end behavior, memory-usage reporting, or properties decoding.

Test coverage added:

- Added `tests/lzma2_corpus.rs`, exercising the upstream LZMA2-specific `.xz`
  corpus fixtures:
  - `good-1-lzma2-1.xz` through `good-1-lzma2-5.xz`
  - `good-2-lzma2.xz`
  - `bad-1-lzma2-1.xz` through `bad-1-lzma2-11.xz`

### `lzma/lzma_encoder`

Compared:

- `vendor/xz/src/liblzma/lzma/lzma_encoder.c`
- `vendor/xz/src/liblzma/lzma/lzma_encoder.h`
- `vendor/xz/src/liblzma/lzma/lzma_encoder_private.h`
- `vendor/xz/src/liblzma/lzma/lzma_common.h`
- `vendor/xz/src/liblzma/lzma/fastpos.h`
- `vendor/xz/src/liblzma/rangecoder/range_encoder.h`
- `vendor/xz/src/liblzma/rangecoder/range_common.h`
- `vendor/xz/src/liblzma/rangecoder/price.h`
- `xz-core/src/lzma/lzma_encoder.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in range-encoder symbol buffering and
  partial-output resume, dummy output-limit simulation, literal subcoder
  selection, state transitions, length price updates, match and repeated-match
  distance rotation, LZMA1/LZMA2 encode-loop limits, EOPM/flush completion,
  option validation, LZ option setup, reset/create behavior, memory-usage
  reporting, property encoding, or mode support reporting.

Notes:

- The Rust file includes the C `range_encoder.h` inline logic directly; its
  `rc_encode()` implementation caches range-coder fields locally and writes
  them back at the same partial-output boundaries as C.
- C state, literal, bit-reset, price, and distance-slot macros are represented
  by direct Rust expressions or shared helpers in `types.rs`.
- Rust adds pointer-access helper functions and an `lzma_encoder_end()` typed
  deallocator; these are mechanical differences from the C allocation model.
- `rc_encode_dummy()` ignores `RC_FLUSH` in its Rust fallback arm, but the
  audited call path invokes the dummy simulation only before `rc_flush()` queues
  flush symbols.

### `lzma/lzma_encoder_optimum_fast`

Compared:

- `vendor/xz/src/liblzma/lzma/lzma_encoder_optimum_fast.c`
- `vendor/xz/src/liblzma/lzma/lzma_encoder_private.h`
- `vendor/xz/src/liblzma/lz/memcmplen.h`
- `vendor/xz/src/liblzma/lz/lz_encoder.h`
- `xz-core/src/lzma/lzma_encoder_optimum_fast.rs`
- `xz-core/src/lz/lz_encoder_mf.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in fast-mode match selection, repeated-match
  scanning, `change_pair()` distance heuristics, look-ahead match comparison,
  read-ahead preservation, literal fallback decisions, or final match skip
  accounting.

Notes:

- Rust's `not_equal_16()` uses `read_unaligned::<u16>()`; this preserves the C
  two-byte inequality test because only equality/inequality of the two-byte
  sequence is observed.
- Rust caches the `mf.find` and `mf.skip` callbacks and calls raw helper
  variants. The helper bodies preserve the C `mf_find()`/`mf_skip()` ordering,
  including `read_ahead` updates.

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
- `lzma/lzma_encoder_optimum_normal`
- `lzma/lzma_decoder`
