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

### `check/check`

Compared:

- `vendor/xz/src/liblzma/check/check.c`
- `vendor/xz/src/liblzma/check/check.h`
- `xz-core/src/check/check.rs`
- `xz-core/src/types.rs`

Finding:

- The Rust port wrote CRC32 and CRC64 results through native-endian `u32` and
  `u64` union fields instead of storing the final check bytes in the C
  `conv32le()`/`conv64le()` byte order. This was correct on little-endian hosts
  but wrong on big-endian targets.

Fix:

- Store CRC32 and CRC64 finished check values through `buffer.u8_0` using
  `to_le_bytes()`.
- Added check regression tests for CRC finish byte order and standard CRC
  vectors.

Additional audit result:

- No other source-code mismatch found in supported-check lookup, check-size
  lookup, check state initialization, update dispatch, SHA-256 delegation, or
  unsupported check no-op behavior.

Notes:

- Rust currently treats CRC32, CRC64, and SHA-256 as available, matching this
  vendored build configuration.
- Rust defensively ignores null `lzma_check_state` pointers in internal
  check init/update/finish helpers; C doesn't guard those internal calls.

### `check/crc32_fast`

Compared:

- `vendor/xz/src/liblzma/check/crc32_fast.c`
- `vendor/xz/src/liblzma/check/crc32_table_le.h`
- `vendor/xz/src/liblzma/check/crc32_table_be.h`
- `vendor/xz/src/liblzma/check/crc_common.h`
- `xz-core/src/check/crc32_fast.rs`
- `xz-core/src/types.rs`

Finding:

- The Rust generic CRC32 path used the little-endian generated table but read
  bulk words in native-endian order. This was correct on little-endian hosts but
  didn't match C's `WORDS_BIGENDIAN` path on big-endian targets.

Fix:

- Interpret CRC32 bulk reads and AArch64 CRC helper reads as little-endian using
  `from_le()`.
- Added a standard CRC32 check-vector regression test.

Additional audit result:

- Numeric comparison confirmed the Rust CRC32 table exactly matches
  `crc32_table_le.h`. No other source-code mismatch found in initial/final CRC
  inversion, alignment handling, slice-by-eight table lookup order, tail-byte
  handling, or public wrapper behavior.

Notes:

- Rust keeps an AArch64 CRC optimized path but doesn't port every upstream
  runtime dispatch variant such as x86 CLMUL or LoongArch. This is an
  implementation/performance difference, not a result mismatch for the generic
  API.

### `check/crc64_fast`

Compared:

- `vendor/xz/src/liblzma/check/crc64_fast.c`
- `vendor/xz/src/liblzma/check/crc64_table_le.h`
- `vendor/xz/src/liblzma/check/crc64_table_be.h`
- `vendor/xz/src/liblzma/check/crc_common.h`
- `xz-core/src/check/crc64_fast.rs`
- `xz-core/src/types.rs`

Finding:

- The Rust generic CRC64 path used the little-endian generated table but read
  bulk words in native-endian order. This was correct on little-endian hosts but
  didn't match C's `WORDS_BIGENDIAN` path on big-endian targets.

Fix:

- Interpret CRC64 bulk reads as little-endian using `from_le()`.
- Added a standard CRC64/XZ check-vector regression test.

Additional audit result:

- Numeric comparison confirmed the Rust CRC64 table exactly matches
  `crc64_table_le.h`. No other source-code mismatch found in initial/final CRC
  inversion, alignment handling, slice-by-four table lookup order, tail-byte
  handling, or public wrapper behavior.

Notes:

- Rust uses the generic CRC64 implementation and doesn't port the optional
  upstream arch-optimized dispatch. This is an implementation/performance
  difference, not a result mismatch for the generic API.

### `check/sha256`

Compared:

- `vendor/xz/src/liblzma/check/sha256.c`
- `vendor/xz/src/liblzma/check/check.h`
- `xz-core/src/check/sha256.rs`
- `xz-core/src/types.rs`

Finding:

- The Rust SHA-256 path manually byte-swapped input words, the message length,
  and final digest words before storing them through native-endian union fields.
  This was correct on little-endian hosts but wrong on big-endian targets.

Fix:

- Read SHA-256 block words with `from_be_bytes()` and write the message length
  and final digest through `buffer.u8_0` using `to_be_bytes()`.
- Added a SHA-256 `abc` standard-vector regression test.

Additional audit result:

- Numeric comparison confirmed the Rust `SHA256_K` table exactly matches C. No
  other source-code mismatch found in rotation functions, compression rounds,
  state initialization, buffered update behavior, padding, message-length
  accounting, or digest finalization.

Notes:

- Rust uses the internal SHA-256 implementation. The vendored C configuration
  may also use the internal implementation, or map the helper functions to an
  external platform SHA-256 provider when configured that way.

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

### `common/block_encoder`

Compared:

- `vendor/xz/src/liblzma/common/block_encoder.c`
- `vendor/xz/src/liblzma/common/block_encoder.h`
- `xz-core/src/common/block_encoder.rs`
- `xz-core/src/types.rs`

Finding:

- The shared `COMPRESSED_SIZE_MAX` constant missed C's final `& ~3` mask,
  allowing a theoretical compressed-data limit up to three bytes higher than
  upstream before Block Padding.

Fix:

- Applied the `& !3` mask to `COMPRESSED_SIZE_MAX`.
- Added `compressed_size_max_matches_c_masking`.

Additional audit result:

- No other source-code mismatch found in uncompressed/compressed size limit
  checks, raw filter delegation, Check update/finalization, Block Padding
  output, raw-check copying, update gating, initialization validation, end
  behavior, or supported action setup.

### `common/block_buffer_encoder`

Compared:

- `vendor/xz/src/liblzma/common/block_buffer_encoder.c`
- `vendor/xz/src/liblzma/common/block_encoder.h`
- `xz-core/src/common/block_buffer_encoder.rs`
- `xz-core/src/types.rs`

Finding:

- C `lzma_block_buffer_bound()` returns zero on 32-bit targets if the 64-bit
  bound exceeds `SIZE_MAX`; Rust cast the 64-bit result to `size_t`, which
  would truncate on 32-bit targets.

Fix:

- Added a `size_t` overflow guard before returning the bound.
- Added `block_buffer_bound_rejects_values_over_size_t_max`.

Additional audit result:

- No other source-code mismatch found in LZMA2 uncompressed-size bound
  calculation, single-call uncompressed LZMA2 fallback encoding, normal
  compression attempt and output-position restore behavior, argument
  validation, Check reservation/finalization, Block Padding, or uncomp wrapper
  behavior.

### `common/block_util`

Compared:

- `vendor/xz/src/liblzma/common/block_util.c`
- `vendor/xz/src/liblzma/common/index.h`
- `xz-core/src/common/block_util.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in compressed-size derivation from Unpadded
  Size, known Compressed Size consistency checks, Unpadded Size validation,
  unknown-size propagation, Total Size rounding, or check-size handling.

### `common/block_header_encoder`

Compared:

- `vendor/xz/src/liblzma/common/block_header_encoder.c`
- `vendor/xz/src/liblzma/common/filter_flags_encoder.c`
- `vendor/xz/src/liblzma/common/filter_encoder.c`
- `xz-core/src/common/block_header_encoder.rs`
- `xz-core/src/common/filter_flags_encoder.rs`
- `xz-core/src/common/filter_encoder.rs`
- `xz-core/src/common/block_util.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in Block Header Size calculation, compressed
  and uncompressed size VLI handling, filter-count limits, Filter Flags size
  calculation, Block Header validation before encode, Block Flags bits, Filter
  Flags encoding, header padding zero-fill, CRC32 emission, or property
  size/encode delegation.

### `common/block_header_decoder`

Compared:

- `vendor/xz/src/liblzma/common/block_header_decoder.c`
- `vendor/xz/src/liblzma/common/filter_flags_decoder.c`
- `vendor/xz/src/liblzma/common/filter_common.c`
- `xz-core/src/common/block_header_decoder.rs`
- `xz-core/src/common/filter_flags_decoder.rs`
- `xz-core/src/common/filter_common.rs`
- `xz-core/src/common/block_util.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in filter array sentinel initialization,
  version downgrade, `ignore_check` reset, header-size/check validation, CRC32
  verification, unsupported Block Flags rejection, compressed and uncompressed
  size VLI decoding, Compressed Size validation through
  `lzma_block_unpadded_size()`, Filter Flags decoding, filter-option cleanup on
  errors, or padding rejection.

### `common/filter_flags_encoder`

Compared:

- `vendor/xz/src/liblzma/common/filter_flags_encoder.c`
- `vendor/xz/src/liblzma/common/filter_encoder.c`
- `xz-core/src/common/filter_flags_encoder.rs`
- `xz-core/src/common/filter_encoder.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in reserved Filter ID rejection, property-size
  lookup, Filter ID and properties-size VLI encoding, output-space validation,
  Filter Properties encoding, or output-position advancement.

### `common/filter_flags_decoder`

Compared:

- `vendor/xz/src/liblzma/common/filter_flags_decoder.c`
- `vendor/xz/src/liblzma/common/filter_decoder.c`
- `xz-core/src/common/filter_flags_decoder.rs`
- `xz-core/src/common/filter_decoder.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in options-pointer clearing, Filter ID VLI
  decoding, reserved Filter ID rejection, properties-size VLI decoding,
  truncated properties rejection, Filter Properties decoding, or input-position
  advancement.

### `common/filter_common`

Compared:

- `vendor/xz/src/liblzma/common/filter_common.c`
- `vendor/xz/src/liblzma/common/filter_common.h`
- `xz-core/src/common/filter_common.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in the filter feature table, filter option
  copy/free behavior, maximum filter count checks, chain validation, encoder
  filter-order reversal, decoder filter-order preservation, raw coder
  initialization cleanup on error, or raw coder memory-usage calculation.

Notes:

- Rust uses `lzma_filter_options_free()` to preserve the allocation size needed
  by its allocator glue; this corresponds to C freeing each filter's copied
  `options` pointer.

### `common/filter_encoder`

Compared:

- `vendor/xz/src/liblzma/common/filter_encoder.c`
- `vendor/xz/src/liblzma/common/filter_encoder.h`
- `vendor/xz/src/liblzma/common/common.h`
- `xz-core/src/common/filter_encoder.rs`
- `xz-core/src/common/common.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in the encoder function table, encoder lookup,
  support query, filter update validation and reversed-chain construction, raw
  encoder initialization, supported-action setup, memory-usage delegation,
  multithreaded block-size selection, property-size lookup, or property
  encoding.

Notes:

- Rust spells out the C `lzma_next_strm_init()` macro expansion in
  `lzma_raw_encoder()`, including `lzma_strm_init()`, `lzma_end()` on init
  failure, and supported-action setup after success.

### `common/filter_decoder`

Compared:

- `vendor/xz/src/liblzma/common/filter_decoder.c`
- `vendor/xz/src/liblzma/common/filter_decoder.h`
- `vendor/xz/src/liblzma/common/common.h`
- `xz-core/src/common/filter_decoder.rs`
- `xz-core/src/common/common.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in the decoder function table, decoder lookup,
  support query, raw decoder initialization, supported-action setup,
  memory-usage delegation, Filter Properties option-pointer clearing,
  unsupported-filter rejection, empty-property handling for filters without a
  properties decoder, or property decoder delegation.

Notes:

- Rust spells out the C `lzma_next_strm_init()` macro expansion in
  `lzma_raw_decoder()`, matching the same initialization and cleanup behavior.

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

### `common/index_encoder`

Compared:

- `vendor/xz/src/liblzma/common/index_encoder.c`
- `vendor/xz/src/liblzma/common/index.h`
- `vendor/xz/src/liblzma/common/common.h`
- `xz-core/src/common/index_encoder.rs`
- `xz-core/src/common/index.rs`
- `xz-core/src/common/common.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in Index indicator emission, Record count VLI
  encoding, Index iterator traversal, Unpadded and Uncompressed Size VLI
  encoding, Index Padding emission, CRC32 accumulation/finalization, init/reset
  behavior, public stream initialization, supported-action setup, stack-backed
  single-call buffer encoding, output-space validation, or output-position
  rollback on unexpected errors.

Notes:

- Rust preserves the C `switch` fallthrough from `SEQ_NEXT` to `SEQ_UNPADDED`
  and from `SEQ_PADDING` to `SEQ_CRC32` with explicit loop continuation and
  shared post-`match` handling.

### `common/index_hash`

Compared:

- `vendor/xz/src/liblzma/common/index_hash.c`
- `vendor/xz/src/liblzma/common/index.h`
- `vendor/xz/src/liblzma/check/check.h`
- `xz-core/src/common/index_hash.rs`
- `xz-core/src/common/index.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in hash allocation/reset, hash-size reporting,
  block append validation, accumulated block/index size limit checks, Index
  indicator and count decoding, Record VLI decoding, Unpadded Size validation,
  record hash accumulation, decoded-record limit checks, padding-size
  calculation and zero-padding validation, final size comparison, hash
  finalization/comparison, CRC32 accumulation/checking, or streaming return
  values.

Notes:

- The vendored C configuration resolves `LZMA_CHECK_BEST` to SHA-256; Rust uses
  `LZMA_CHECK_SHA256` directly in the corresponding index-hash paths.
- Rust matches the normal upstream build. The C CRC32 check has a fuzzing-only
  `FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION` branch; Rust has no corresponding
  build cfg.

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

### `common/alone_encoder`

Compared:

- `vendor/xz/src/liblzma/common/alone_encoder.c`
- `vendor/xz/src/liblzma/common/common.h`
- `xz-core/src/common/alone_encoder.rs`
- `xz-core/src/common/common.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in LZMA_Alone header emission, partial header
  buffering, dictionary-size validation and rounding, unknown uncompressed-size
  marker writing, LZMA1 filter initialization, end cleanup, public stream
  initialization, or supported-action setup.

Notes:

- Rust spells out the C `lzma_next_strm_init()` and `lzma_next_coder_init()`
  macro expansions while preserving their cleanup behavior.

### `common/easy_*`

Compared:

- `vendor/xz/src/liblzma/common/easy_encoder.c`
- `vendor/xz/src/liblzma/common/easy_buffer_encoder.c`
- `vendor/xz/src/liblzma/common/easy_encoder_memusage.c`
- `vendor/xz/src/liblzma/common/easy_decoder_memusage.c`
- `vendor/xz/src/liblzma/common/easy_preset.c`
- `vendor/xz/src/liblzma/common/easy_preset.h`
- `xz-core/src/common/easy_encoder.rs`
- `xz-core/src/common/easy_buffer_encoder.rs`
- `xz-core/src/common/easy_encoder_memusage.rs`
- `xz-core/src/common/easy_decoder_memusage.rs`
- `xz-core/src/common/easy_preset.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in preset expansion, LZMA2 filter-chain
  construction, invalid-preset return values, stream encoder delegation,
  single-call stream buffer encoder delegation, or encoder/decoder memory-usage
  delegation.

Notes:

- Rust uses `MaybeUninit<lzma_options_easy>` for C's stack-allocated
  `lzma_options_easy`; initialization is completed only after
  `lzma_easy_preset()` succeeds.

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

### `common/stream_encoder`

Compared:

- `vendor/xz/src/liblzma/common/stream_encoder.c`
- `vendor/xz/src/liblzma/common/common.h`
- `vendor/xz/src/liblzma/common/stream_flags_encoder.c`
- `xz-core/src/common/stream_encoder.rs`
- `xz-core/src/common/common.rs`
- `xz-core/src/common/stream_flags_encoder.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in Stream Header output buffering, empty-block
  finish/full-flush behavior, Block encoder initialization boundaries, Block
  Header encoding, action conversion for Block encoding, Index record
  appending, Index encoder invocation, Stream Footer encoding, final
  `LZMA_STREAM_END` reporting, end/free behavior, filter-chain update behavior,
  public initialization, or supported action setup.

Notes:

- Rust represents C fallthrough between buffered Stream Header/Block
  Header/Footer output and the next sequence by updating `sequence` and
  continuing the outer loop.
- The Rust `convert_action()` table matches C's `convert[]`, mapping
  `LZMA_FULL_FLUSH`, `LZMA_FINISH`, and `LZMA_FULL_BARRIER` to
  `LZMA_FINISH` for the Block encoder.
- Both C and Rust initialize only `version` and `check` before Stream Header
  encoding; `lzma_stream_header_encode()` doesn't read `backward_size`.
- The filter-chain update path preserves C's temporary-copy-before-commit
  behavior. On success, ownership of the copied filter options moves into
  `coder->filters`; on failure, the temporary copy is freed and the current
  chain is left unchanged.

### `common/stream_decoder_mt`

Compared:

- `vendor/xz/src/liblzma/common/stream_decoder_mt.c`
- `vendor/xz/src/liblzma/common/common.h`
- `xz-core/src/common/stream_decoder_mt.rs`
- `xz-core/src/common/common.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in worker-thread state transitions, partial
  output updates, worker completion/error reporting, thread stop/end behavior,
  thread reuse and cache accounting, `read_output_and_wait()` blocking and
  fail-fast behavior, Block Header detection/decoding, threaded/direct mode
  selection, memlimit handling, output queue cache clearing, threaded Block
  startup, direct Block decoding, Index/Footer/Padding state transitions,
  pending-error flushing, memconfig/get_progress, or public decoder
  initialization.

Notes:

- Rust uses internal return names `LZMA_RET_INTERNAL1` and
  `LZMA_RET_INTERNAL2`; these correspond to C's `LZMA_TIMED_OUT` and
  `LZMA_INDEX_DETECTED` macros in `common.h`.
- The C `switch` fallthrough in `stream_decode_mt()` is represented by Rust's
  `StreamMtBlockState` dispatch and restart-loop sentinel.
- Rust stores the worker allocator only when `custom_allocator` is enabled and
  uses helper functions to preserve the same allocator passed to C worker
  operations.

### `common/stream_decoder`

Compared:

- `vendor/xz/src/liblzma/common/stream_decoder.c`
- `vendor/xz/src/liblzma/common/stream_decoder.h`
- `vendor/xz/src/liblzma/common/common.h`
- `vendor/xz/src/liblzma/common/block_header_decoder.c`
- `vendor/xz/src/liblzma/common/filter_common.c`
- `xz-core/src/common/stream_decoder.rs`
- `xz-core/src/common/common.rs`
- `xz-core/src/common/block_header_decoder.rs`
- `xz-core/src/common/filter_common.rs`
- `xz-core/src/common/common_types.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in Stream Header decoding, check notification
  flags, Block Header buffering and Index detection, Block Header decoding and
  filter-option cleanup, Block decoder startup and run behavior, Index hash
  appending/decoding, Stream Footer verification, concatenated Stream Padding
  handling, reset/end behavior, get-check/memconfig behavior, public
  initialization, or supported action setup.

Notes:

- Rust represents C fallthrough between Stream Header -> Block Header, Block
  Header -> Block Init, Block Init -> Block Run, Index -> Footer, Footer ->
  Padding, and Padding -> next Stream by setting `sequence` and continuing the
  outer loop.
- C's `return_if_error(lzma_block_header_decode(...))` leaves the local filter
  array cleanup to `lzma_block_header_decode()` on error. Rust additionally
  calls `lzma_filters_free()` before returning from that error path; the Block
  Header decoder initializes the array with a terminating sentinel before
  returning errors and resets allocated entries on its internal cleanup paths,
  so this is a safe mechanical difference.
- `LZMA_FAIL_FAST` is accepted through `LZMA_SUPPORTED_FLAGS` in both C and
  Rust, but the single-threaded stream decoder doesn't otherwise use it.

### `common/stream_buffer_*`

Compared:

- `vendor/xz/src/liblzma/common/stream_buffer_encoder.c`
- `vendor/xz/src/liblzma/common/stream_buffer_decoder.c`
- `vendor/xz/src/liblzma/common/index.h`
- `xz-core/src/common/stream_buffer_encoder.rs`
- `xz-core/src/common/stream_buffer_decoder.rs`
- `xz-core/src/common/block_buffer_encoder.rs`
- `xz-core/src/common/stream_decoder.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in Stream buffer bound calculation, check
  validation, caller position validation, successful-output-position commit,
  Stream Header emission, optional empty-input Block omission, Block buffer
  encoding delegation, Index creation/append/encode cleanup, Footer Backward
  Size assignment, Stream Footer emission, single-call Stream decoder
  initialization, forbidden flag rejection, error-position rollback, truncated
  input versus full output classification, memlimit reporting, or decoder
  cleanup.

### `common/filter_buffer_*`

Compared:

- `vendor/xz/src/liblzma/common/filter_buffer_encoder.c`
- `vendor/xz/src/liblzma/common/filter_buffer_decoder.c`
- `vendor/xz/src/liblzma/common/filter_encoder.h`
- `vendor/xz/src/liblzma/common/filter_decoder.h`
- `xz-core/src/common/filter_buffer_encoder.rs`
- `xz-core/src/common/filter_buffer_decoder.rs`
- `xz-core/src/common/filter_encoder.rs`
- `xz-core/src/common/filter_decoder.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in raw buffer input/output argument validation,
  raw encoder/decoder initialization, single-call `LZMA_FINISH` execution,
  `LZMA_STREAM_END` to `LZMA_OK` conversion, output-position rollback on raw
  encode errors, input/output-position rollback on raw decode errors,
  truncated-input versus too-small-output classification, one-byte decode probe
  behavior for ambiguous full-input/full-output cases, or coder cleanup.

### `common/stream_flags_*`

Compared:

- `vendor/xz/src/liblzma/common/stream_flags_common.c`
- `vendor/xz/src/liblzma/common/stream_flags_common.h`
- `vendor/xz/src/liblzma/common/stream_flags_encoder.c`
- `vendor/xz/src/liblzma/common/stream_flags_decoder.c`
- `xz-core/src/common/stream_flags_common.rs`
- `xz-core/src/common/stream_flags_encoder.rs`
- `xz-core/src/common/stream_flags_decoder.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in Header/Footer magic constants, Stream Flags
  comparison, check-ID validation, Backward Size validation, Header/Footer flag
  encoding, Header/Footer CRC emission, reserved-bit rejection, Header/Footer
  CRC verification, unknown Header Backward Size handling, or Footer Backward
  Size decoding.

Notes:

- Rust matches the normal upstream build. The C decoder has fuzzing-only
  `FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION` branches that skip CRC errors;
  Rust has no corresponding build cfg.

### `common/vli_*`

Compared:

- `vendor/xz/src/liblzma/common/vli_encoder.c`
- `vendor/xz/src/liblzma/common/vli_decoder.c`
- `vendor/xz/src/liblzma/common/vli_size.c`
- `xz-core/src/common/vli_encoder.rs`
- `xz-core/src/common/vli_decoder.rs`
- `xz-core/src/common/vli_size.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in single-call versus multi-call VLI mode,
  empty input/output error selection, argument validation, partial encode
  resume position handling, partial decode resume initialization, compact-form
  rejection, maximum encoded-length rejection, return values, or encoded-size
  calculation.

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

### `lzma/lzma_encoder_optimum_normal`

Compared:

- `vendor/xz/src/liblzma/lzma/lzma_encoder_optimum_normal.c`
- `vendor/xz/src/liblzma/lzma/lzma_encoder_private.h`
- `vendor/xz/src/liblzma/lzma/lzma_common.h`
- `vendor/xz/src/liblzma/lzma/fastpos.h`
- `vendor/xz/src/liblzma/lz/memcmplen.h`
- `vendor/xz/src/liblzma/lz/lz_encoder.h`
- `vendor/xz/src/liblzma/rangecoder/price.h`
- `xz-core/src/lzma/lzma_encoder_optimum_normal.rs`
- `xz-core/src/lz/lz_encoder_mf.rs`
- `xz-core/src/types.rs`

Finding:

- C `uint32_t` position arithmetic wraps around 4 GiB boundaries. The Rust port
  used checked debug-build addition in the normal optimum path for
  `position + cur`, `position + 1`, and literal-after-match/rep offsets.

Fix:

- Added a small `position_add()` wrapper using `wrapping_add()` and used it for
  all position-offset additions in `lzma_encoder_optimum_normal.rs`.
- Added `position_add_wraps_like_uint32_t`.

Additional audit result:

- No other source-code mismatch found in literal price calculation, repeated
  match prices, distance/align price table refresh, optimum backward traversal,
  initial candidate setup, state/reps reconstruction, literal/short-rep updates,
  repeated-match expansion, normal-match expansion, match+literal+rep0
  expansion, pending-symbol replay, or price-table refresh gating.

### `lzma/lzma_decoder`

Compared:

- `vendor/xz/src/liblzma/lzma/lzma_decoder.c`
- `vendor/xz/src/liblzma/lzma/lzma_decoder.h`
- `vendor/xz/src/liblzma/lzma/lzma_common.h`
- `vendor/xz/src/liblzma/lz/lz_decoder.h`
- `vendor/xz/src/liblzma/rangecoder/range_decoder.h`
- `xz-core/src/lzma/lzma_decoder.rs`
- `xz-core/src/lz/lz_decoder.rs`
- `xz-core/src/types.rs`

Finding:

- No source-code mismatch found in range-decoder initialization and
  normalization, dictionary get/repeat/write helpers, literal subcoder
  selection, literal and matched-literal decoding, simple-match and
  repeated-match state transitions, length decoders, distance-slot and
  distance-model decoding, direct and align-bit decoding, EOPM validation,
  known-uncompressed-size termination, output-limit resume points, state
  save/restore, decoder reset, LZMA1EXT setup, property decoding, or memory
  usage reporting.

Notes:

- Rust represents the C `switch`/`goto` resumable decoder as explicit
  `BLOCK_*` states. The `SEQ_*` resume values still map to the same C labels,
  including `SEQ_DIST_MODEL`, `SEQ_DIRECT`, `SEQ_ALIGN`, and the length decoder
  choice/choice2/bittree labels.
- The non-resumable fast path keeps C's unrolled length, distance, direct-bit,
  and align-bit decoding, while the slow path keeps the C safe-normalization
  re-entry boundaries.
- Rust uses `wrapping_*` in the dictionary and range/distance arithmetic where
  C relies on unsigned arithmetic. Bounded loop counters and bit-tree symbols
  keep the remaining plain shifts and increments within the same ranges as C.
- `lzma_lzma_decoder_create()` installs a typed `end` deallocator in Rust; C
  leaves `lz->end` null and relies on the generic fallback free. This is a
  mechanical allocator difference and matches the Rust LZ wrapper's typed-free
  policy.

## Wrapper API Findings

### `stream::IGNORE_CHECK`

Finding:

- Public `IGNORE_CHECK` was mapped to `LZMA_TELL_UNSUPPORTED_CHECK` instead of
  `LZMA_IGNORE_CHECK`.

Fix:

- Corrected the constant mapping.
- Added `stream_decoder_ignore_check_skips_integrity_check_verification`.

## Next Audit Targets

The initial high-risk decoder/encoder state-machine pass is now complete. The
next pass should continue through the remaining same-stem pairs from
`scripts/porting_audit_map.sh`, prioritizing shared utility modules and files
with allocator, CRC/check, filter-chain, or feature-condition branches.
