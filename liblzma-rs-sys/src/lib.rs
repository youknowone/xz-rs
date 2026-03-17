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

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use libc::size_t;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use libc::{c_char, c_int, c_uchar, c_uint, c_void};
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_void};
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
type wasm_size_t = usize;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use self::wasm_size_t as size_t;

// === Canonical type aliases ===
// On MSVC, C enums are c_int; elsewhere c_uint (matching liblzma-sys manual.rs)
#[cfg(target_env = "msvc")]
#[doc(hidden)]
pub type __enum_ty = c_int;
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
pub use liblzma_rs::common::index_hash::lzma_index_hash;
pub use liblzma_rs::types::lzma_allocator;
pub use liblzma_rs::types::lzma_block;
pub use liblzma_rs::types::lzma_filter;
pub use liblzma_rs::types::lzma_index;
pub use liblzma_rs::types::lzma_index_iter;
pub use liblzma_rs::types::lzma_index_iter_mode;
pub use liblzma_rs::types::lzma_mt;
pub use liblzma_rs::types::lzma_options_lzma;
pub use liblzma_rs::types::lzma_stream;
pub use liblzma_rs::types::lzma_stream_flags;

#[repr(C)]
pub struct lzma_options_bcj {
    pub start_offset: u32,
}

pub enum lzma_internal {}

// =========================================================================
// Constants
// =========================================================================

// --- Return codes ---
pub use liblzma_rs::types::{
    LZMA_BUF_ERROR, LZMA_DATA_ERROR, LZMA_FORMAT_ERROR, LZMA_GET_CHECK, LZMA_MEMLIMIT_ERROR,
    LZMA_MEM_ERROR, LZMA_NO_CHECK, LZMA_OK, LZMA_OPTIONS_ERROR, LZMA_PROG_ERROR, LZMA_SEEK_NEEDED,
    LZMA_STREAM_END, LZMA_UNSUPPORTED_CHECK,
};

// --- Actions ---
pub use liblzma_rs::types::{
    LZMA_FINISH, LZMA_FULL_BARRIER, LZMA_FULL_FLUSH, LZMA_RUN, LZMA_SYNC_FLUSH,
};

// --- Check types ---
pub use liblzma_rs::types::{
    LZMA_CHECK_CRC32, LZMA_CHECK_CRC64, LZMA_CHECK_NONE, LZMA_CHECK_SHA256,
};

// --- Modes / match finders ---
pub use liblzma_rs::types::{
    LZMA_MF_BT2, LZMA_MF_BT3, LZMA_MF_BT4, LZMA_MF_HC3, LZMA_MF_HC4, LZMA_MODE_FAST,
    LZMA_MODE_NORMAL,
};

// --- Filter IDs ---
pub use liblzma_rs::types::{
    LZMA_FILTER_ARM, LZMA_FILTER_ARM64, LZMA_FILTER_ARMTHUMB, LZMA_FILTER_DELTA, LZMA_FILTER_IA64,
    LZMA_FILTER_LZMA1, LZMA_FILTER_LZMA2, LZMA_FILTER_POWERPC, LZMA_FILTER_RISCV,
    LZMA_FILTER_SPARC, LZMA_FILTER_X86,
};

// --- Decoder flags ---
pub use liblzma_rs::types::{
    LZMA_CONCATENATED, LZMA_IGNORE_CHECK, LZMA_TELL_ANY_CHECK, LZMA_TELL_NO_CHECK,
    LZMA_TELL_UNSUPPORTED_CHECK,
};

// --- Presets / option limits ---
pub const LZMA_PRESET_DEFAULT: u32 =
    liblzma_rs::common::string_conversion::LZMA_PRESET_DEFAULT as u32;
pub const LZMA_PRESET_LEVEL_MASK: u32 =
    liblzma_rs::lzma::lzma_encoder_presets::LZMA_PRESET_LEVEL_MASK as u32;
pub const LZMA_PRESET_EXTREME: u32 = liblzma_rs::types::LZMA_PRESET_EXTREME as u32;
pub const LZMA_DICT_SIZE_MIN: u32 = liblzma_rs::types::LZMA_DICT_SIZE_MIN as u32;
pub const LZMA_DICT_SIZE_DEFAULT: u32 =
    liblzma_rs::common::string_conversion::LZMA_DICT_SIZE_DEFAULT as u32;
pub const LZMA_LCLP_MIN: u32 = liblzma_rs::common::string_conversion::LZMA_LCLP_MIN as u32;
pub const LZMA_LCLP_MAX: u32 = liblzma_rs::types::LZMA_LCLP_MAX as u32;
pub const LZMA_LC_DEFAULT: u32 = liblzma_rs::lzma::lzma_encoder_presets::LZMA_LC_DEFAULT as u32;
pub const LZMA_LP_DEFAULT: u32 = liblzma_rs::lzma::lzma_encoder_presets::LZMA_LP_DEFAULT as u32;
pub const LZMA_PB_MIN: u32 = liblzma_rs::common::string_conversion::LZMA_PB_MIN as u32;
pub const LZMA_PB_MAX: u32 = liblzma_rs::types::LZMA_PB_MAX as u32;
pub const LZMA_PB_DEFAULT: u32 = liblzma_rs::lzma::lzma_encoder_presets::LZMA_PB_DEFAULT as u32;

// --- Backward size / VLI ---
pub const LZMA_BACKWARD_SIZE_MIN: lzma_vli = liblzma_rs::types::LZMA_BACKWARD_SIZE_MIN as lzma_vli;
pub const LZMA_BACKWARD_SIZE_MAX: lzma_vli = liblzma_rs::types::LZMA_BACKWARD_SIZE_MAX as lzma_vli;
pub use liblzma_rs::types::{LZMA_VLI_MAX, LZMA_VLI_UNKNOWN};
pub const LZMA_VLI_BYTES_MAX: usize = liblzma_rs::types::LZMA_VLI_BYTES_MAX as usize;

// --- Stream header size ---
pub const LZMA_STREAM_HEADER_SIZE: u32 = liblzma_rs::types::LZMA_STREAM_HEADER_SIZE as u32;

// =========================================================================
// Functions
// =========================================================================
//
// Functions are exposed as thin C ABI wrappers from this crate.
// Wrappers cast to canonical Rust implementation types and forward the call.

// --- Common stream API ---

#[no_mangle]
pub unsafe extern "C" fn lzma_code(strm: *mut lzma_stream, action: lzma_action) -> lzma_ret {
    liblzma_rs::common::common::lzma_code(strm.cast(), action)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_end(strm: *mut lzma_stream) {
    liblzma_rs::common::common::lzma_end(strm.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_memlimit_get(strm: *const lzma_stream) -> u64 {
    liblzma_rs::common::common::lzma_memlimit_get(strm.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_memlimit_set(strm: *mut lzma_stream, new_memlimit: u64) -> lzma_ret {
    liblzma_rs::common::common::lzma_memlimit_set(strm.cast(), new_memlimit)
}

// --- Version ---

#[no_mangle]
pub unsafe extern "C" fn lzma_version_number() -> u32 {
    liblzma_rs::common::common::lzma_version_number()
}

#[no_mangle]
pub unsafe extern "C" fn lzma_version_string() -> *const c_char {
    liblzma_rs::common::common::lzma_version_string()
}

// --- Progress / memusage ---

#[no_mangle]
pub unsafe extern "C" fn lzma_get_progress(
    strm: *mut lzma_stream,
    progress_in: *mut u64,
    progress_out: *mut u64,
) {
    liblzma_rs::common::common::lzma_get_progress(strm.cast(), progress_in, progress_out)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_memusage(strm: *const lzma_stream) -> u64 {
    liblzma_rs::common::common::lzma_memusage(strm.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_get_check(strm: *const lzma_stream) -> lzma_check {
    liblzma_rs::common::common::lzma_get_check(strm.cast())
}

// --- Check ---

#[no_mangle]
pub unsafe extern "C" fn lzma_check_is_supported(check: lzma_check) -> lzma_bool {
    liblzma_rs::check::check::lzma_check_is_supported(check)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_check_size(check: lzma_check) -> u32 {
    liblzma_rs::check::check::lzma_check_size(check)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_crc32(buf: *const u8, size: size_t, crc: u32) -> u32 {
    liblzma_rs::check::crc32_fast::lzma_crc32(buf, size, crc)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_crc64(buf: *const u8, size: size_t, crc: u64) -> u64 {
    liblzma_rs::check::crc64_fast::lzma_crc64(buf, size, crc)
}

// --- Easy encoder ---

#[no_mangle]
pub unsafe extern "C" fn lzma_easy_encoder_memusage(preset: u32) -> u64 {
    liblzma_rs::common::easy_encoder_memusage::lzma_easy_encoder_memusage(preset)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_easy_decoder_memusage(preset: u32) -> u64 {
    liblzma_rs::common::easy_decoder_memusage::lzma_easy_decoder_memusage(preset)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_easy_encoder(
    strm: *mut lzma_stream,
    preset: u32,
    check: lzma_check,
) -> lzma_ret {
    liblzma_rs::common::easy_encoder::lzma_easy_encoder(strm.cast(), preset, check)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_easy_buffer_encode(
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

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_encoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
    check: lzma_check,
) -> lzma_ret {
    liblzma_rs::common::stream_encoder::lzma_stream_encoder(strm.cast(), filters.cast(), check)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_decoder(
    strm: *mut lzma_stream,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    liblzma_rs::common::stream_decoder::lzma_stream_decoder(strm.cast(), memlimit, flags)
}

// --- Alone encoder/decoder ---

#[no_mangle]
pub unsafe extern "C" fn lzma_alone_encoder(
    strm: *mut lzma_stream,
    options: *const lzma_options_lzma,
) -> lzma_ret {
    liblzma_rs::common::alone_encoder::lzma_alone_encoder(strm.cast(), options.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_alone_decoder(strm: *mut lzma_stream, memlimit: u64) -> lzma_ret {
    liblzma_rs::common::alone_decoder::lzma_alone_decoder(strm.cast(), memlimit)
}

// --- Auto/lzip decoder ---

#[no_mangle]
pub unsafe extern "C" fn lzma_auto_decoder(
    strm: *mut lzma_stream,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    liblzma_rs::common::auto_decoder::lzma_auto_decoder(strm.cast(), memlimit, flags)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_lzip_decoder(
    strm: *mut lzma_stream,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    liblzma_rs::common::lzip_decoder::lzma_lzip_decoder(strm.cast(), memlimit, flags)
}

// --- Stream buffer ---

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_buffer_bound(uncompressed_size: size_t) -> size_t {
    liblzma_rs::common::stream_buffer_encoder::lzma_stream_buffer_bound(uncompressed_size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_buffer_encode(
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

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_buffer_decode(
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

#[no_mangle]
pub unsafe extern "C" fn lzma_filter_encoder_is_supported(id: lzma_vli) -> lzma_bool {
    liblzma_rs::common::filter_encoder::lzma_filter_encoder_is_supported(id)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_filter_decoder_is_supported(id: lzma_vli) -> lzma_bool {
    liblzma_rs::common::filter_decoder::lzma_filter_decoder_is_supported(id)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_filters_copy(
    src: *const lzma_filter,
    dest: *mut lzma_filter,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    liblzma_rs::common::filter_common::lzma_filters_copy(src.cast(), dest.cast(), allocator.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_raw_encoder_memusage(filters: *const lzma_filter) -> u64 {
    liblzma_rs::common::filter_encoder::lzma_raw_encoder_memusage(filters.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_raw_decoder_memusage(filters: *const lzma_filter) -> u64 {
    liblzma_rs::common::filter_decoder::lzma_raw_decoder_memusage(filters.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_raw_encoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
) -> lzma_ret {
    liblzma_rs::common::filter_encoder::lzma_raw_encoder(strm.cast(), filters.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_raw_decoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
) -> lzma_ret {
    liblzma_rs::common::filter_decoder::lzma_raw_decoder(strm.cast(), filters.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_filters_update(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
) -> lzma_ret {
    liblzma_rs::common::filter_encoder::lzma_filters_update(strm.cast(), filters.cast())
}

// --- Raw buffer ---

#[no_mangle]
pub unsafe extern "C" fn lzma_raw_buffer_encode(
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

#[no_mangle]
pub unsafe extern "C" fn lzma_raw_buffer_decode(
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

#[no_mangle]
pub unsafe extern "C" fn lzma_properties_size(
    size: *mut u32,
    filter: *const lzma_filter,
) -> lzma_ret {
    liblzma_rs::common::filter_encoder::lzma_properties_size(size, filter.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_properties_encode(
    filter: *const lzma_filter,
    props: *mut u8,
) -> lzma_ret {
    liblzma_rs::common::filter_encoder::lzma_properties_encode(filter.cast(), props)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_properties_decode(
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

#[no_mangle]
pub unsafe extern "C" fn lzma_mt_block_size(filters: *const lzma_filter) -> u64 {
    liblzma_rs::common::filter_encoder::lzma_mt_block_size(filters.cast())
}

// --- LZMA preset ---

#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_preset(
    options: *mut lzma_options_lzma,
    preset: u32,
) -> lzma_bool {
    liblzma_rs::lzma::lzma_encoder_presets::lzma_lzma_preset(options.cast(), preset)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_mf_is_supported(mf: lzma_match_finder) -> lzma_bool {
    liblzma_rs::lz::lz_encoder::lzma_mf_is_supported(mf)
}

// --- Stream header/footer ---

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_header_encode(
    options: *const lzma_stream_flags,
    out: *mut u8,
) -> lzma_ret {
    liblzma_rs::common::stream_flags_encoder::lzma_stream_header_encode(options.cast(), out)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_footer_encode(
    options: *const lzma_stream_flags,
    out: *mut u8,
) -> lzma_ret {
    liblzma_rs::common::stream_flags_encoder::lzma_stream_footer_encode(options.cast(), out)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_header_decode(
    options: *mut lzma_stream_flags,
    input: *const u8,
) -> lzma_ret {
    liblzma_rs::common::stream_flags_decoder::lzma_stream_header_decode(options.cast(), input)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_footer_decode(
    options: *mut lzma_stream_flags,
    input: *const u8,
) -> lzma_ret {
    liblzma_rs::common::stream_flags_decoder::lzma_stream_footer_decode(options.cast(), input)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_flags_compare(
    a: *const lzma_stream_flags,
    b: *const lzma_stream_flags,
) -> lzma_ret {
    liblzma_rs::common::stream_flags_common::lzma_stream_flags_compare(a.cast(), b.cast())
}

// --- VLI ---

#[no_mangle]
pub unsafe extern "C" fn lzma_vli_encode(
    vli: lzma_vli,
    vli_pos: *mut size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::vli_encoder::lzma_vli_encode(vli, vli_pos, out, out_pos, out_size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_vli_decode(
    vli: *mut lzma_vli,
    vli_pos: *mut size_t,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::vli_decoder::lzma_vli_decode(vli, vli_pos, input, in_pos, in_size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_vli_size(vli: lzma_vli) -> u32 {
    liblzma_rs::common::vli_size::lzma_vli_size(vli)
}

// --- Hardware ---

#[no_mangle]
pub unsafe extern "C" fn lzma_physmem() -> u64 {
    liblzma_rs::common::hardware_physmem::lzma_physmem()
}

#[no_mangle]
pub unsafe extern "C" fn lzma_cputhreads() -> u32 {
    liblzma_rs::common::hardware_cputhreads::lzma_cputhreads()
}

// --- Index ---

#[no_mangle]
pub unsafe extern "C" fn lzma_index_buffer_decode(
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

#[no_mangle]
pub unsafe extern "C" fn lzma_index_uncompressed_size(i: *const lzma_index) -> lzma_vli {
    liblzma_rs::common::index::lzma_index_uncompressed_size(i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_end(i: *mut lzma_index, allocator: *const lzma_allocator) {
    liblzma_rs::common::index::lzma_index_end(i.cast(), allocator.cast())
}

// --- Block ---

#[no_mangle]
pub unsafe extern "C" fn lzma_block_header_size(block: *mut lzma_block) -> lzma_ret {
    liblzma_rs::common::block_header_encoder::lzma_block_header_size(block.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_header_encode(
    block: *const lzma_block,
    out: *mut u8,
) -> lzma_ret {
    liblzma_rs::common::block_header_encoder::lzma_block_header_encode(block.cast(), out)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_header_decode(
    block: *mut lzma_block,
    allocator: *const lzma_allocator,
    input: *const u8,
) -> lzma_ret {
    liblzma_rs::common::block_header_decoder::lzma_block_header_decode(
        block.cast(),
        allocator.cast(),
        input,
    )
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_compressed_size(
    block: *mut lzma_block,
    unpadded_size: lzma_vli,
) -> lzma_ret {
    liblzma_rs::common::block_util::lzma_block_compressed_size(block.cast(), unpadded_size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_unpadded_size(block: *const lzma_block) -> lzma_vli {
    liblzma_rs::common::block_util::lzma_block_unpadded_size(block.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_total_size(block: *const lzma_block) -> lzma_vli {
    liblzma_rs::common::block_util::lzma_block_total_size(block.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_encoder(
    strm: *mut lzma_stream,
    block: *mut lzma_block,
) -> lzma_ret {
    liblzma_rs::common::block_encoder::lzma_block_encoder(strm.cast(), block.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_decoder(
    strm: *mut lzma_stream,
    block: *mut lzma_block,
) -> lzma_ret {
    liblzma_rs::common::block_decoder::lzma_block_decoder(strm.cast(), block.cast())
}

#[no_mangle]
pub extern "C" fn lzma_block_buffer_bound(uncompressed_size: size_t) -> size_t {
    liblzma_rs::common::block_buffer_encoder::lzma_block_buffer_bound(uncompressed_size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_buffer_encode(
    block: *mut lzma_block,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::block_buffer_encoder::lzma_block_buffer_encode(
        block.cast(),
        allocator.cast(),
        input,
        in_size,
        out,
        out_pos,
        out_size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_uncomp_encode(
    block: *mut lzma_block,
    input: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::block_buffer_encoder::lzma_block_uncomp_encode(
        block.cast(),
        input,
        in_size,
        out,
        out_pos,
        out_size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_buffer_decode(
    block: *mut lzma_block,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::block_buffer_decoder::lzma_block_buffer_decode(
        block.cast(),
        allocator.cast(),
        input,
        in_pos,
        in_size,
        out,
        out_pos,
        out_size,
    )
}

// --- Index (extended) ---

#[no_mangle]
pub extern "C" fn lzma_index_memusage(streams: lzma_vli, blocks: lzma_vli) -> u64 {
    liblzma_rs::common::index::lzma_index_memusage(streams, blocks)
}

#[no_mangle]
pub extern "C" fn lzma_index_memused(i: *const lzma_index) -> u64 {
    liblzma_rs::common::index::lzma_index_memused(i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_init(allocator: *const lzma_allocator) -> *mut lzma_index {
    liblzma_rs::common::index::lzma_index_init(allocator.cast()).cast()
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_append(
    i: *mut lzma_index,
    allocator: *const lzma_allocator,
    unpadded_size: lzma_vli,
    uncompressed_size: lzma_vli,
) -> lzma_ret {
    liblzma_rs::common::index::lzma_index_append(
        i.cast(),
        allocator.cast(),
        unpadded_size,
        uncompressed_size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_stream_flags(
    i: *mut lzma_index,
    stream_flags: *const lzma_stream_flags,
) -> lzma_ret {
    liblzma_rs::common::index::lzma_index_stream_flags(i.cast(), stream_flags.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_checks(i: *const lzma_index) -> u32 {
    liblzma_rs::common::index::lzma_index_checks(i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_stream_padding(
    i: *mut lzma_index,
    stream_padding: lzma_vli,
) -> lzma_ret {
    liblzma_rs::common::index::lzma_index_stream_padding(i.cast(), stream_padding)
}

#[no_mangle]
pub extern "C" fn lzma_index_stream_count(i: *const lzma_index) -> lzma_vli {
    liblzma_rs::common::index::lzma_index_stream_count(i.cast())
}

#[no_mangle]
pub extern "C" fn lzma_index_block_count(i: *const lzma_index) -> lzma_vli {
    liblzma_rs::common::index::lzma_index_block_count(i.cast())
}

#[no_mangle]
pub extern "C" fn lzma_index_size(i: *const lzma_index) -> lzma_vli {
    liblzma_rs::common::index::lzma_index_size(i.cast())
}

#[no_mangle]
pub extern "C" fn lzma_index_stream_size(i: *const lzma_index) -> lzma_vli {
    liblzma_rs::common::index::lzma_index_stream_size(i.cast())
}

#[no_mangle]
pub extern "C" fn lzma_index_total_size(i: *const lzma_index) -> lzma_vli {
    liblzma_rs::common::index::lzma_index_total_size(i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_file_size(i: *const lzma_index) -> lzma_vli {
    liblzma_rs::common::index::lzma_index_file_size(i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_init(iter: *mut lzma_index_iter, i: *const lzma_index) {
    liblzma_rs::common::index::lzma_index_iter_init(iter.cast(), i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_rewind(iter: *mut lzma_index_iter) {
    liblzma_rs::common::index::lzma_index_iter_rewind(iter.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_next(
    iter: *mut lzma_index_iter,
    mode: lzma_index_iter_mode,
) -> lzma_bool {
    liblzma_rs::common::index::lzma_index_iter_next(iter.cast(), mode)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_locate(
    iter: *mut lzma_index_iter,
    target: lzma_vli,
) -> lzma_bool {
    liblzma_rs::common::index::lzma_index_iter_locate(iter.cast(), target)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_cat(
    dest: *mut lzma_index,
    src: *mut lzma_index,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    liblzma_rs::common::index::lzma_index_cat(dest.cast(), src.cast(), allocator.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_dup(
    i: *const lzma_index,
    allocator: *const lzma_allocator,
) -> *mut lzma_index {
    liblzma_rs::common::index::lzma_index_dup(i.cast(), allocator.cast()).cast()
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_encoder(
    strm: *mut lzma_stream,
    i: *const lzma_index,
) -> lzma_ret {
    liblzma_rs::common::index_encoder::lzma_index_encoder(strm.cast(), i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_decoder(
    strm: *mut lzma_stream,
    i: *mut *mut lzma_index,
    memlimit: u64,
) -> lzma_ret {
    liblzma_rs::common::index_decoder::lzma_index_decoder(strm.cast(), i.cast(), memlimit)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_buffer_encode(
    i: *const lzma_index,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::index_encoder::lzma_index_buffer_encode(i.cast(), out, out_pos, out_size)
}

// --- Index hash ---

#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_init(
    index_hash: *mut lzma_index_hash,
    allocator: *const lzma_allocator,
) -> *mut lzma_index_hash {
    liblzma_rs::common::index_hash::lzma_index_hash_init(index_hash.cast(), allocator.cast()).cast()
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_end(
    index_hash: *mut lzma_index_hash,
    allocator: *const lzma_allocator,
) {
    liblzma_rs::common::index_hash::lzma_index_hash_end(index_hash.cast(), allocator.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_append(
    index_hash: *mut lzma_index_hash,
    unpadded_size: lzma_vli,
    uncompressed_size: lzma_vli,
) -> lzma_ret {
    liblzma_rs::common::index_hash::lzma_index_hash_append(
        index_hash.cast(),
        unpadded_size,
        uncompressed_size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_decode(
    index_hash: *mut lzma_index_hash,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::index_hash::lzma_index_hash_decode(
        index_hash.cast(),
        input,
        in_pos,
        in_size,
    )
}

#[no_mangle]
pub extern "C" fn lzma_index_hash_size(index_hash: *const lzma_index_hash) -> lzma_vli {
    liblzma_rs::common::index_hash::lzma_index_hash_size(index_hash.cast())
}

// --- Filter flags/string ---

#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_size(
    size: *mut u32,
    filter: *const lzma_filter,
) -> lzma_ret {
    liblzma_rs::common::filter_flags_encoder::lzma_filter_flags_size(size, filter.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_encode(
    filter: *const lzma_filter,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::filter_flags_encoder::lzma_filter_flags_encode(
        filter.cast(),
        out,
        out_pos,
        out_size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_decode(
    filter: *mut lzma_filter,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    liblzma_rs::common::filter_flags_decoder::lzma_filter_flags_decode(
        filter.cast(),
        allocator.cast(),
        input,
        in_pos,
        in_size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn lzma_filters_free(
    filters: *mut lzma_filter,
    allocator: *const lzma_allocator,
) {
    liblzma_rs::common::filter_common::lzma_filters_free(filters.cast(), allocator.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_str_to_filters(
    str_: *const c_char,
    error_pos: *mut c_int,
    filters: *mut lzma_filter,
    flags: u32,
    allocator: *const lzma_allocator,
) -> *const c_char {
    liblzma_rs::common::string_conversion::lzma_str_to_filters(
        str_,
        error_pos,
        filters.cast(),
        flags,
        allocator.cast(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn lzma_str_from_filters(
    str_: *mut *mut c_char,
    filters: *const lzma_filter,
    flags: u32,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    liblzma_rs::common::string_conversion::lzma_str_from_filters(
        str_,
        filters.cast(),
        flags,
        allocator.cast(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn lzma_str_list_filters(
    str_: *mut *mut c_char,
    filter_id: lzma_vli,
    flags: u32,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    liblzma_rs::common::string_conversion::lzma_str_list_filters(
        str_,
        filter_id,
        flags,
        allocator.cast(),
    )
}

// --- Misc ---

#[no_mangle]
pub extern "C" fn lzma_mode_is_supported(mode: lzma_mode) -> lzma_bool {
    liblzma_rs::lzma::lzma_encoder::lzma_mode_is_supported(mode)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_microlzma_encoder(
    strm: *mut lzma_stream,
    options: *const lzma_options_lzma,
) -> lzma_ret {
    liblzma_rs::common::microlzma_encoder::lzma_microlzma_encoder(strm.cast(), options.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_microlzma_decoder(
    strm: *mut lzma_stream,
    comp_size: u64,
    uncomp_size: u64,
    uncomp_size_is_exact: lzma_bool,
    dict_size: u32,
) -> lzma_ret {
    liblzma_rs::common::microlzma_decoder::lzma_microlzma_decoder(
        strm.cast(),
        comp_size,
        uncomp_size,
        uncomp_size_is_exact,
        dict_size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn lzma_file_info_decoder(
    strm: *mut lzma_stream,
    i: *mut *mut lzma_index,
    memlimit: u64,
    file_size: u64,
) -> lzma_ret {
    liblzma_rs::common::file_info::lzma_file_info_decoder(
        strm.cast(),
        i.cast(),
        memlimit,
        file_size,
    )
}

// =========================================================================
// Multithreaded API
// =========================================================================

#[cfg(feature = "parallel")]
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_encoder_mt(
    strm: *mut lzma_stream,
    options: *const lzma_mt,
) -> lzma_ret {
    liblzma_rs::common::stream_mt::lzma_stream_encoder_mt(strm.cast(), options.cast())
}

#[cfg(feature = "parallel")]
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_decoder_mt(
    strm: *mut lzma_stream,
    options: *const lzma_mt,
) -> lzma_ret {
    liblzma_rs::common::stream_mt::lzma_stream_decoder_mt(strm.cast(), options.cast())
}

#[cfg(feature = "parallel")]
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_encoder_mt_memusage(options: *const lzma_mt) -> u64 {
    liblzma_rs::common::stream_mt::lzma_stream_encoder_mt_memusage(options.cast())
}
