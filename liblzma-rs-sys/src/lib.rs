//! liblzma-sys compatible API layer backed by pure Rust liblzma-rs
//!
//! Re-exports symbols from liblzma-rs with the same names and signatures
//! as liblzma-sys, enabling drop-in replacement.
//!
//! Because c2rust generates per-file type definitions, this layer provides
//! canonical types and thin wrapper functions that cast between structurally
//! identical `#[repr(C)]` types.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    clippy::all
)]

use core::ffi::{c_char, c_uchar, c_uint, c_void};
use libc::size_t;

// === Canonical type aliases ===
// On MSVC, C enums are c_int; elsewhere c_uint (matching liblzma-sys manual.rs)
#[cfg(target_env = "msvc")]
#[doc(hidden)]
pub type __enum_ty = core::ffi::c_int;
#[cfg(not(target_env = "msvc"))]
#[doc(hidden)]
pub type __enum_ty = c_uint;

pub type lzma_ret = __enum_ty;
pub type lzma_action = __enum_ty;
pub type lzma_check = __enum_ty;
pub type lzma_mode = __enum_ty;
pub type lzma_match_finder = __enum_ty;
pub type lzma_bool = c_uchar;
pub type lzma_vli = u64;

// === Canonical struct re-exports ===
pub use liblzma_rs::common::common::lzma_allocator;
pub use liblzma_rs::common::common::lzma_stream;
pub use liblzma_rs::common::filter_common::lzma_options_lzma;
pub use liblzma_rs::common::index_decoder::lzma_index;
pub use liblzma_rs::common::stream_flags_decoder::lzma_stream_flags;
pub use liblzma_rs::delta::delta_common::lzma_filter;

#[repr(C)]
pub struct lzma_options_bcj {
    pub start_offset: u32,
}

pub enum lzma_internal {}

// =========================================================================
// Constants
// =========================================================================

// --- Return codes / actions ---
pub const LZMA_OK: lzma_ret = liblzma_rs::common::common::LZMA_OK as lzma_ret;
pub const LZMA_STREAM_END: lzma_ret = liblzma_rs::common::common::LZMA_STREAM_END as lzma_ret;
pub const LZMA_NO_CHECK: lzma_ret = liblzma_rs::common::common::LZMA_NO_CHECK as lzma_ret;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret =
    liblzma_rs::common::common::LZMA_UNSUPPORTED_CHECK as lzma_ret;
pub const LZMA_GET_CHECK: lzma_ret = liblzma_rs::common::common::LZMA_GET_CHECK as lzma_ret;
pub const LZMA_MEM_ERROR: lzma_ret = liblzma_rs::common::common::LZMA_MEM_ERROR as lzma_ret;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret =
    liblzma_rs::common::common::LZMA_MEMLIMIT_ERROR as lzma_ret;
pub const LZMA_FORMAT_ERROR: lzma_ret = liblzma_rs::common::common::LZMA_FORMAT_ERROR as lzma_ret;
pub const LZMA_OPTIONS_ERROR: lzma_ret = liblzma_rs::common::common::LZMA_OPTIONS_ERROR as lzma_ret;
pub const LZMA_DATA_ERROR: lzma_ret = liblzma_rs::common::common::LZMA_DATA_ERROR as lzma_ret;
pub const LZMA_BUF_ERROR: lzma_ret = liblzma_rs::common::common::LZMA_BUF_ERROR as lzma_ret;
pub const LZMA_PROG_ERROR: lzma_ret = liblzma_rs::common::common::LZMA_PROG_ERROR as lzma_ret;
pub const LZMA_SEEK_NEEDED: lzma_ret = liblzma_rs::common::common::LZMA_SEEK_NEEDED as lzma_ret;

pub const LZMA_RUN: lzma_action = liblzma_rs::common::common::LZMA_RUN as lzma_action;
pub const LZMA_SYNC_FLUSH: lzma_action = liblzma_rs::common::common::LZMA_SYNC_FLUSH as lzma_action;
pub const LZMA_FULL_FLUSH: lzma_action = liblzma_rs::common::common::LZMA_FULL_FLUSH as lzma_action;
pub const LZMA_FULL_BARRIER: lzma_action =
    liblzma_rs::common::common::LZMA_FULL_BARRIER as lzma_action;
pub const LZMA_FINISH: lzma_action = liblzma_rs::common::common::LZMA_FINISH as lzma_action;

// --- Check types ---
pub const LZMA_CHECK_NONE: lzma_check = liblzma_rs::check::check::LZMA_CHECK_NONE as lzma_check;
pub const LZMA_CHECK_CRC32: lzma_check = liblzma_rs::check::check::LZMA_CHECK_CRC32 as lzma_check;
pub const LZMA_CHECK_CRC64: lzma_check = liblzma_rs::check::check::LZMA_CHECK_CRC64 as lzma_check;
pub const LZMA_CHECK_SHA256: lzma_check = liblzma_rs::check::check::LZMA_CHECK_SHA256 as lzma_check;

// --- Modes / match finders / filter IDs ---
pub const LZMA_MODE_FAST: lzma_mode =
    liblzma_rs::common::filter_common::LZMA_MODE_FAST as lzma_mode;
pub const LZMA_MODE_NORMAL: lzma_mode =
    liblzma_rs::common::filter_common::LZMA_MODE_NORMAL as lzma_mode;
pub const LZMA_MF_HC3: lzma_match_finder =
    liblzma_rs::common::filter_common::LZMA_MF_HC3 as lzma_match_finder;
pub const LZMA_MF_HC4: lzma_match_finder =
    liblzma_rs::common::filter_common::LZMA_MF_HC4 as lzma_match_finder;
pub const LZMA_MF_BT2: lzma_match_finder =
    liblzma_rs::common::filter_common::LZMA_MF_BT2 as lzma_match_finder;
pub const LZMA_MF_BT3: lzma_match_finder =
    liblzma_rs::common::filter_common::LZMA_MF_BT3 as lzma_match_finder;
pub const LZMA_MF_BT4: lzma_match_finder =
    liblzma_rs::common::filter_common::LZMA_MF_BT4 as lzma_match_finder;

pub const LZMA_FILTER_X86: lzma_vli =
    liblzma_rs::common::filter_common::LZMA_FILTER_X86 as lzma_vli;
pub const LZMA_FILTER_POWERPC: lzma_vli =
    liblzma_rs::common::filter_common::LZMA_FILTER_POWERPC as lzma_vli;
pub const LZMA_FILTER_IA64: lzma_vli =
    liblzma_rs::common::filter_common::LZMA_FILTER_IA64 as lzma_vli;
pub const LZMA_FILTER_ARM: lzma_vli =
    liblzma_rs::common::filter_common::LZMA_FILTER_ARM as lzma_vli;
pub const LZMA_FILTER_ARMTHUMB: lzma_vli =
    liblzma_rs::common::filter_common::LZMA_FILTER_ARMTHUMB as lzma_vli;
pub const LZMA_FILTER_SPARC: lzma_vli =
    liblzma_rs::common::filter_common::LZMA_FILTER_SPARC as lzma_vli;
pub const LZMA_FILTER_ARM64: lzma_vli =
    liblzma_rs::common::filter_common::LZMA_FILTER_ARM64 as lzma_vli;
pub const LZMA_FILTER_DELTA: lzma_vli =
    liblzma_rs::common::filter_common::LZMA_FILTER_DELTA as lzma_vli;
pub const LZMA_FILTER_RISCV: lzma_vli =
    liblzma_rs::common::filter_common::LZMA_FILTER_RISCV as lzma_vli;
pub const LZMA_FILTER_LZMA1: lzma_vli =
    liblzma_rs::common::filter_common::LZMA_FILTER_LZMA1 as lzma_vli;
pub const LZMA_FILTER_LZMA2: lzma_vli =
    liblzma_rs::common::filter_common::LZMA_FILTER_LZMA2 as lzma_vli;

// --- Decoder flags ---
pub const LZMA_TELL_NO_CHECK: u32 = liblzma_rs::common::auto_decoder::LZMA_TELL_NO_CHECK as u32;
pub const LZMA_TELL_UNSUPPORTED_CHECK: u32 =
    liblzma_rs::common::auto_decoder::LZMA_TELL_UNSUPPORTED_CHECK as u32;
pub const LZMA_TELL_ANY_CHECK: u32 = liblzma_rs::common::auto_decoder::LZMA_TELL_ANY_CHECK as u32;
pub const LZMA_IGNORE_CHECK: u32 = liblzma_rs::common::auto_decoder::LZMA_IGNORE_CHECK as u32;
pub const LZMA_CONCATENATED: u32 = liblzma_rs::common::auto_decoder::LZMA_CONCATENATED as u32;

// --- Presets / option limits ---
pub const LZMA_PRESET_DEFAULT: u32 =
    liblzma_rs::common::string_conversion::LZMA_PRESET_DEFAULT as u32;
pub const LZMA_PRESET_LEVEL_MASK: u32 =
    liblzma_rs::lzma::lzma_encoder_presets::LZMA_PRESET_LEVEL_MASK as u32;
pub const LZMA_PRESET_EXTREME: u32 =
    liblzma_rs::common::string_conversion::LZMA_PRESET_EXTREME as u32;
pub const LZMA_DICT_SIZE_MIN: u32 =
    liblzma_rs::common::string_conversion::LZMA_DICT_SIZE_MIN as u32;
pub const LZMA_DICT_SIZE_DEFAULT: u32 =
    liblzma_rs::common::string_conversion::LZMA_DICT_SIZE_DEFAULT as u32;
pub const LZMA_LCLP_MIN: u32 = liblzma_rs::common::string_conversion::LZMA_LCLP_MIN as u32;
pub const LZMA_LCLP_MAX: u32 = liblzma_rs::common::string_conversion::LZMA_LCLP_MAX as u32;
pub const LZMA_LC_DEFAULT: u32 = liblzma_rs::lzma::lzma_encoder_presets::LZMA_LC_DEFAULT as u32;
pub const LZMA_LP_DEFAULT: u32 = liblzma_rs::lzma::lzma_encoder_presets::LZMA_LP_DEFAULT as u32;
pub const LZMA_PB_MIN: u32 = liblzma_rs::common::string_conversion::LZMA_PB_MIN as u32;
pub const LZMA_PB_MAX: u32 = liblzma_rs::common::string_conversion::LZMA_PB_MAX as u32;
pub const LZMA_PB_DEFAULT: u32 = liblzma_rs::lzma::lzma_encoder_presets::LZMA_PB_DEFAULT as u32;

// --- Backward size / VLI ---
pub const LZMA_BACKWARD_SIZE_MIN: lzma_vli =
    liblzma_rs::common::stream_flags_common::LZMA_BACKWARD_SIZE_MIN as lzma_vli;
pub const LZMA_BACKWARD_SIZE_MAX: lzma_vli =
    liblzma_rs::common::stream_flags_common::LZMA_BACKWARD_SIZE_MAX as lzma_vli;
pub const LZMA_VLI_MAX: lzma_vli = liblzma_rs::common::vli_encoder::LZMA_VLI_MAX as lzma_vli;
pub const LZMA_VLI_UNKNOWN: lzma_vli = liblzma_rs::common::common::LZMA_VLI_UNKNOWN as lzma_vli;
pub const LZMA_VLI_BYTES_MAX: usize = liblzma_rs::common::vli_encoder::LZMA_VLI_BYTES_MAX as usize;

// --- Stream header size ---
pub const LZMA_STREAM_HEADER_SIZE: u32 = liblzma_rs::common::index::LZMA_STREAM_HEADER_SIZE as u32;

// =========================================================================
// Functions
// =========================================================================
//
// Functions defined in common::common use the canonical lzma_stream type,
// so they can be re-exported directly. Functions in other modules use
// module-local struct definitions that are structurally identical (#[repr(C)])
// but nominally different, so we provide thin wrappers with pointer casts.

// --- Direct re-exports (canonical types from common::common) ---
pub use liblzma_rs::common::common::{lzma_code, lzma_end, lzma_memlimit_get, lzma_memlimit_set};

// --- Version ---

#[inline]
pub unsafe fn lzma_version_number() -> u32 {
    liblzma_rs::common::common::lzma_version_number()
}

#[inline]
pub unsafe fn lzma_version_string() -> *const c_char {
    liblzma_rs::common::common::lzma_version_string()
}

// --- Progress / memusage ---

#[inline]
pub unsafe fn lzma_get_progress(
    strm: *mut lzma_stream,
    progress_in: *mut u64,
    progress_out: *mut u64,
) {
    liblzma_rs::common::common::lzma_get_progress(strm.cast(), progress_in, progress_out)
}

#[inline]
pub unsafe fn lzma_memusage(strm: *const lzma_stream) -> u64 {
    liblzma_rs::common::common::lzma_memusage(strm.cast())
}

#[inline]
pub unsafe fn lzma_get_check(strm: *const lzma_stream) -> lzma_check {
    liblzma_rs::common::common::lzma_get_check(strm.cast())
}

// --- Check ---

#[inline]
pub unsafe fn lzma_check_is_supported(check: lzma_check) -> lzma_bool {
    liblzma_rs::check::check::lzma_check_is_supported(check)
}

#[inline]
pub unsafe fn lzma_check_size(check: lzma_check) -> u32 {
    liblzma_rs::check::check::lzma_check_size(check)
}

#[inline]
pub unsafe fn lzma_crc32(buf: *const u8, size: size_t, crc: u32) -> u32 {
    liblzma_rs::check::crc32_fast::lzma_crc32(buf, size, crc)
}

#[inline]
pub unsafe fn lzma_crc64(buf: *const u8, size: size_t, crc: u64) -> u64 {
    liblzma_rs::check::crc64_fast::lzma_crc64(buf, size, crc)
}

// --- Easy encoder ---

#[inline]
pub unsafe fn lzma_easy_encoder_memusage(preset: u32) -> u64 {
    liblzma_rs::common::easy_encoder_memusage::lzma_easy_encoder_memusage(preset)
}

#[inline]
pub unsafe fn lzma_easy_decoder_memusage(preset: u32) -> u64 {
    liblzma_rs::common::easy_decoder_memusage::lzma_easy_decoder_memusage(preset)
}

#[inline]
pub unsafe fn lzma_easy_encoder(
    strm: *mut lzma_stream,
    preset: u32,
    check: lzma_check,
) -> lzma_ret {
    liblzma_rs::common::easy_encoder::lzma_easy_encoder(strm.cast(), preset, check)
}

#[inline]
pub unsafe fn lzma_easy_buffer_encode(
    preset: u32,
    check: lzma_check,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::easy_buffer_encoder::lzma_easy_buffer_encode(
        preset,
        check,
        allocator.cast(),
        input,
        in_size,
        out,
        out_pos,
        out_size,
    )
}

// --- Stream encoder/decoder ---

#[inline]
pub unsafe fn lzma_stream_encoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
    check: lzma_check,
) -> lzma_ret {
    liblzma_rs::common::stream_encoder::lzma_stream_encoder(strm.cast(), filters.cast(), check)
}

#[inline]
pub unsafe fn lzma_stream_decoder(strm: *mut lzma_stream, memlimit: u64, flags: u32) -> lzma_ret {
    liblzma_rs::common::stream_decoder::lzma_stream_decoder(strm.cast(), memlimit, flags)
}

// --- Alone encoder/decoder ---

#[inline]
pub unsafe fn lzma_alone_encoder(
    strm: *mut lzma_stream,
    options: *const lzma_options_lzma,
) -> lzma_ret {
    liblzma_rs::common::alone_encoder::lzma_alone_encoder(strm.cast(), options.cast())
}

#[inline]
pub unsafe fn lzma_alone_decoder(strm: *mut lzma_stream, memlimit: u64) -> lzma_ret {
    liblzma_rs::common::alone_decoder::lzma_alone_decoder(strm.cast(), memlimit)
}

// --- Auto/lzip decoder ---

#[inline]
pub unsafe fn lzma_auto_decoder(strm: *mut lzma_stream, memlimit: u64, flags: u32) -> lzma_ret {
    liblzma_rs::common::auto_decoder::lzma_auto_decoder(strm.cast(), memlimit, flags)
}

#[inline]
pub unsafe fn lzma_lzip_decoder(strm: *mut lzma_stream, memlimit: u64, flags: u32) -> lzma_ret {
    liblzma_rs::common::lzip_decoder::lzma_lzip_decoder(strm.cast(), memlimit, flags)
}

// --- Stream buffer ---

#[inline]
pub unsafe fn lzma_stream_buffer_bound(uncompressed_size: size_t) -> size_t {
    liblzma_rs::common::stream_buffer_encoder::lzma_stream_buffer_bound(uncompressed_size)
}

#[inline]
pub unsafe fn lzma_stream_buffer_encode(
    filters: *mut lzma_filter,
    check: lzma_check,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::stream_buffer_encoder::lzma_stream_buffer_encode(
        filters.cast(),
        check,
        allocator.cast(),
        input,
        in_size,
        out,
        out_pos,
        out_size,
    )
}

#[inline]
pub unsafe fn lzma_stream_buffer_decode(
    memlimit: *mut u64,
    flags: u32,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::stream_buffer_decoder::lzma_stream_buffer_decode(
        memlimit,
        flags,
        allocator.cast(),
        input,
        in_pos,
        in_size,
        out,
        out_pos,
        out_size,
    )
}

// --- Filter ---

#[inline]
pub unsafe fn lzma_filter_encoder_is_supported(id: lzma_vli) -> lzma_bool {
    liblzma_rs::common::filter_encoder::lzma_filter_encoder_is_supported(id)
}

#[inline]
pub unsafe fn lzma_filter_decoder_is_supported(id: lzma_vli) -> lzma_bool {
    liblzma_rs::common::filter_decoder::lzma_filter_decoder_is_supported(id)
}

#[inline]
pub unsafe fn lzma_filters_copy(
    src: *const lzma_filter,
    dest: *mut lzma_filter,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    liblzma_rs::common::filter_common::lzma_filters_copy(src.cast(), dest.cast(), allocator.cast())
}

#[inline]
pub unsafe fn lzma_raw_encoder_memusage(filters: *const lzma_filter) -> u64 {
    liblzma_rs::common::filter_encoder::lzma_raw_encoder_memusage(filters.cast())
}

#[inline]
pub unsafe fn lzma_raw_decoder_memusage(filters: *const lzma_filter) -> u64 {
    liblzma_rs::common::filter_decoder::lzma_raw_decoder_memusage(filters.cast())
}

#[inline]
pub unsafe fn lzma_raw_encoder(strm: *mut lzma_stream, filters: *const lzma_filter) -> lzma_ret {
    liblzma_rs::common::filter_encoder::lzma_raw_encoder(strm.cast(), filters.cast())
}

#[inline]
pub unsafe fn lzma_raw_decoder(strm: *mut lzma_stream, filters: *const lzma_filter) -> lzma_ret {
    liblzma_rs::common::filter_decoder::lzma_raw_decoder(strm.cast(), filters.cast())
}

#[inline]
pub unsafe fn lzma_filters_update(strm: *mut lzma_stream, filters: *const lzma_filter) -> lzma_ret {
    liblzma_rs::common::filter_encoder::lzma_filters_update(strm.cast(), filters.cast())
}

// --- Raw buffer ---

#[inline]
pub unsafe fn lzma_raw_buffer_encode(
    filters: *const lzma_filter,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::filter_buffer_encoder::lzma_raw_buffer_encode(
        filters.cast(),
        allocator.cast(),
        input,
        in_size,
        out,
        out_pos,
        out_size,
    )
}

#[inline]
pub unsafe fn lzma_raw_buffer_decode(
    filters: *const lzma_filter,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::filter_buffer_decoder::lzma_raw_buffer_decode(
        filters.cast(),
        allocator.cast(),
        input,
        in_pos,
        in_size,
        out,
        out_pos,
        out_size,
    )
}

// --- Properties ---

#[inline]
pub unsafe fn lzma_properties_size(size: *mut u32, filter: *const lzma_filter) -> lzma_ret {
    liblzma_rs::common::filter_encoder::lzma_properties_size(size, filter.cast())
}

#[inline]
pub unsafe fn lzma_properties_encode(filter: *const lzma_filter, props: *mut u8) -> lzma_ret {
    liblzma_rs::common::filter_encoder::lzma_properties_encode(filter.cast(), props)
}

#[inline]
pub unsafe fn lzma_properties_decode(
    filter: *mut lzma_filter,
    allocator: *const lzma_allocator,
    props: *const u8,
    props_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::filter_decoder::lzma_properties_decode(
        filter.cast(),
        allocator.cast(),
        props,
        props_size,
    )
}

#[inline]
pub unsafe fn lzma_mt_block_size(filters: *const lzma_filter) -> u64 {
    liblzma_rs::common::filter_encoder::lzma_mt_block_size(filters.cast())
}

// --- LZMA preset ---

#[inline]
pub unsafe fn lzma_lzma_preset(options: *mut lzma_options_lzma, preset: u32) -> lzma_bool {
    liblzma_rs::lzma::lzma_encoder_presets::lzma_lzma_preset(options.cast(), preset)
}

#[inline]
pub unsafe fn lzma_mf_is_supported(mf: lzma_match_finder) -> lzma_bool {
    liblzma_rs::lz::lz_encoder::lzma_mf_is_supported(mf)
}

// --- Stream header/footer ---

#[inline]
pub unsafe fn lzma_stream_header_encode(
    options: *const lzma_stream_flags,
    out: *mut u8,
) -> lzma_ret {
    liblzma_rs::common::stream_flags_encoder::lzma_stream_header_encode(options.cast(), out)
}

#[inline]
pub unsafe fn lzma_stream_footer_encode(
    options: *const lzma_stream_flags,
    out: *mut u8,
) -> lzma_ret {
    liblzma_rs::common::stream_flags_encoder::lzma_stream_footer_encode(options.cast(), out)
}

#[inline]
pub unsafe fn lzma_stream_header_decode(
    options: *mut lzma_stream_flags,
    input: *const u8,
) -> lzma_ret {
    liblzma_rs::common::stream_flags_decoder::lzma_stream_header_decode(options.cast(), input)
}

#[inline]
pub unsafe fn lzma_stream_footer_decode(
    options: *mut lzma_stream_flags,
    input: *const u8,
) -> lzma_ret {
    liblzma_rs::common::stream_flags_decoder::lzma_stream_footer_decode(options.cast(), input)
}

#[inline]
pub unsafe fn lzma_stream_flags_compare(
    a: *const lzma_stream_flags,
    b: *const lzma_stream_flags,
) -> lzma_ret {
    liblzma_rs::common::stream_flags_common::lzma_stream_flags_compare(a.cast(), b.cast())
}

// --- VLI ---

#[inline]
pub unsafe fn lzma_vli_encode(
    vli: lzma_vli,
    vli_pos: *mut size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::vli_encoder::lzma_vli_encode(vli, vli_pos, out, out_pos, out_size)
}

#[inline]
pub unsafe fn lzma_vli_decode(
    vli: *mut lzma_vli,
    vli_pos: *mut size_t,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::vli_decoder::lzma_vli_decode(vli, vli_pos, input, in_pos, in_size)
}

#[inline]
pub unsafe fn lzma_vli_size(vli: lzma_vli) -> u32 {
    liblzma_rs::common::vli_size::lzma_vli_size(vli)
}

// --- Hardware ---

#[inline]
pub unsafe fn lzma_physmem() -> u64 {
    liblzma_rs::common::hardware_physmem::lzma_physmem()
}

#[inline]
pub unsafe fn lzma_cputhreads() -> u32 {
    liblzma_rs::common::hardware_cputhreads::lzma_cputhreads()
}

// --- Index ---

#[inline]
pub unsafe fn lzma_index_buffer_decode(
    i: *mut *mut lzma_index,
    memlimit: *mut u64,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::index_decoder::lzma_index_buffer_decode(
        i.cast(),
        memlimit,
        allocator.cast(),
        input,
        in_pos,
        in_size,
    )
}

#[inline]
pub unsafe fn lzma_index_uncompressed_size(i: *const lzma_index) -> lzma_vli {
    liblzma_rs::common::index::lzma_index_uncompressed_size(i.cast())
}

#[inline]
pub unsafe fn lzma_index_end(i: *mut lzma_index, allocator: *const lzma_allocator) {
    liblzma_rs::common::index::lzma_index_end(i.cast(), allocator.cast())
}

// =========================================================================
// Multithreaded API
// =========================================================================

#[cfg(feature = "parallel")]
#[repr(C)]
pub struct lzma_mt {
    pub flags: u32,
    pub threads: u32,
    pub block_size: u64,
    pub timeout: u32,
    pub preset: u32,
    pub filters: *const lzma_filter,
    pub check: lzma_check,
    _reserved_enum1: __enum_ty,
    _reserved_enum2: __enum_ty,
    _reserved_enum3: __enum_ty,
    _reserved_int1: u32,
    _reserved_int2: u32,
    _reserved_int3: u32,
    _reserved_int4: u32,
    pub memlimit_threading: u64,
    pub memlimit_stop: u64,
    _reserved_int7: u64,
    _reserved_int8: u64,
    _reserved_ptr1: *mut c_void,
    _reserved_ptr2: *mut c_void,
    _reserved_ptr3: *mut c_void,
    _reserved_ptr4: *mut c_void,
}

#[cfg(feature = "parallel")]
pub unsafe fn lzma_stream_encoder_mt(strm: *mut lzma_stream, options: *const lzma_mt) -> lzma_ret {
    liblzma_rs::common::stream_mt::lzma_stream_encoder_mt(strm.cast(), options.cast())
}

#[cfg(feature = "parallel")]
pub unsafe fn lzma_stream_decoder_mt(strm: *mut lzma_stream, options: *const lzma_mt) -> lzma_ret {
    liblzma_rs::common::stream_mt::lzma_stream_decoder_mt(strm.cast(), options.cast())
}

#[cfg(feature = "parallel")]
pub unsafe fn lzma_stream_encoder_mt_memusage(options: *const lzma_mt) -> u64 {
    liblzma_rs::common::stream_mt::lzma_stream_encoder_mt_memusage(options.cast())
}
