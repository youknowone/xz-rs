//! liblzma-sys compatible API layer backed by pure Rust xz
//!
//! Re-exports symbols from xz with the same names and signatures
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

/************************
 * Canonical type aliases
 ************************/

/*
 * Match liblzma-sys' platform-dependent C enum ABI directly.
 */
#[cfg(target_env = "msvc")]
type c_enum = c_int;
#[cfg(not(target_env = "msvc"))]
type c_enum = c_uint;

pub type lzma_ret = c_enum;
pub type lzma_action = c_enum;
pub type lzma_check = c_enum;
pub type lzma_mode = c_enum;
pub type lzma_match_finder = c_enum;
pub type lzma_bool = c_uchar;
pub type lzma_vli = u64;
pub type lzma_delta_type = c_uint;

/****************************
 * Canonical struct re-exports
 ****************************/
pub use xz::common::index_hash::lzma_index_hash;
pub use xz::types::lzma_allocator;
pub use xz::types::lzma_block;
pub use xz::types::lzma_filter;
pub use xz::types::lzma_index;
pub use xz::types::lzma_index_iter;
pub use xz::types::lzma_index_iter_mode;
pub use xz::types::lzma_mt;
pub use xz::types::lzma_options_lzma;
pub use xz::types::lzma_stream;
pub use xz::types::lzma_stream_flags;

#[repr(C)]
pub struct lzma_options_bcj {
    pub start_offset: u32,
}

#[repr(C)]
pub struct lzma_options_delta {
    pub type_: lzma_delta_type,
    pub dist: u32,
    pub reserved_int1: u32,
    pub reserved_int2: u32,
    pub reserved_int3: u32,
    pub reserved_int4: u32,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
}

pub enum lzma_internal {}

/******************
 * Basic Features *
 ******************/

/* `lzma/version.h`: compile-time version constants */
pub const LZMA_VERSION_MAJOR: u32 = xz::common::common::LZMA_VERSION_MAJOR as u32;
pub const LZMA_VERSION_MINOR: u32 = xz::common::common::LZMA_VERSION_MINOR as u32;
pub const LZMA_VERSION_PATCH: u32 = xz::common::common::LZMA_VERSION_PATCH as u32;
pub const LZMA_VERSION: u32 = xz::common::common::LZMA_VERSION as u32;

/* `lzma/base.h`: return codes */
pub use xz::types::{
    LZMA_BUF_ERROR, LZMA_DATA_ERROR, LZMA_FORMAT_ERROR, LZMA_GET_CHECK, LZMA_MEMLIMIT_ERROR,
    LZMA_MEM_ERROR, LZMA_NO_CHECK, LZMA_OK, LZMA_OPTIONS_ERROR, LZMA_PROG_ERROR, LZMA_SEEK_NEEDED,
    LZMA_STREAM_END, LZMA_UNSUPPORTED_CHECK,
};

/* `lzma/base.h`: actions */
pub use xz::types::{LZMA_FINISH, LZMA_FULL_BARRIER, LZMA_FULL_FLUSH, LZMA_RUN, LZMA_SYNC_FLUSH};

/* `lzma/vli.h`: backward size / VLI */
pub const LZMA_BACKWARD_SIZE_MIN: lzma_vli = xz::types::LZMA_BACKWARD_SIZE_MIN as lzma_vli;
pub const LZMA_BACKWARD_SIZE_MAX: lzma_vli = xz::types::LZMA_BACKWARD_SIZE_MAX as lzma_vli;
pub use xz::types::{LZMA_VLI_MAX, LZMA_VLI_UNKNOWN};
pub const LZMA_VLI_BYTES_MAX: usize = xz::types::LZMA_VLI_BYTES_MAX as usize;

/* `lzma/check.h`: check types */
pub use xz::types::{LZMA_CHECK_CRC32, LZMA_CHECK_CRC64, LZMA_CHECK_NONE, LZMA_CHECK_SHA256};
pub const LZMA_CHECK_ID_MAX: u32 = xz::types::LZMA_CHECK_ID_MAX as u32;
pub const LZMA_CHECK_SIZE_MAX: u32 = xz::types::LZMA_CHECK_SIZE_MAX as u32;

/***********
 * Filters *
 ***********/

/* `lzma/lzma12.h`: modes / match finders */
pub use xz::types::{
    LZMA_MF_BT2, LZMA_MF_BT3, LZMA_MF_BT4, LZMA_MF_HC3, LZMA_MF_HC4, LZMA_MODE_FAST,
    LZMA_MODE_NORMAL,
};

/* `lzma/filter.h`: filter IDs */
pub use xz::types::{
    LZMA_FILTER_ARM, LZMA_FILTER_ARM64, LZMA_FILTER_ARMTHUMB, LZMA_FILTER_DELTA, LZMA_FILTER_IA64,
    LZMA_FILTER_LZMA1, LZMA_FILTER_LZMA2, LZMA_FILTER_POWERPC, LZMA_FILTER_RISCV,
    LZMA_FILTER_SPARC, LZMA_FILTER_X86,
};
pub const LZMA_FILTERS_MAX: u32 = xz::types::LZMA_FILTERS_MAX as u32;
pub const LZMA_DELTA_DIST_MIN: u32 = xz::types::LZMA_DELTA_DIST_MIN as u32;
pub const LZMA_DELTA_DIST_MAX: u32 = xz::types::LZMA_DELTA_DIST_MAX as u32;
pub const LZMA_DELTA_TYPE_BYTE: lzma_delta_type =
    xz::types::LZMA_DELTA_TYPE_BYTE as lzma_delta_type;

/*********************
 * Container Formats *
 *********************/

/* `lzma/container.h`: decoder flags */
pub use xz::types::{
    LZMA_CONCATENATED, LZMA_IGNORE_CHECK, LZMA_TELL_ANY_CHECK, LZMA_TELL_NO_CHECK,
    LZMA_TELL_UNSUPPORTED_CHECK,
};

/* `lzma/container.h`: presets / option limits */
pub const LZMA_PRESET_DEFAULT: u32 = xz::common::string_conversion::LZMA_PRESET_DEFAULT as u32;
pub const LZMA_PRESET_LEVEL_MASK: u32 =
    xz::lzma::lzma_encoder_presets::LZMA_PRESET_LEVEL_MASK as u32;
pub const LZMA_PRESET_EXTREME: u32 = xz::types::LZMA_PRESET_EXTREME as u32;
pub const LZMA_DICT_SIZE_MIN: u32 = xz::types::LZMA_DICT_SIZE_MIN as u32;
pub const LZMA_DICT_SIZE_DEFAULT: u32 =
    xz::common::string_conversion::LZMA_DICT_SIZE_DEFAULT as u32;
pub const LZMA_LCLP_MIN: u32 = xz::common::string_conversion::LZMA_LCLP_MIN as u32;
pub const LZMA_LCLP_MAX: u32 = xz::types::LZMA_LCLP_MAX as u32;
pub const LZMA_LC_DEFAULT: u32 = xz::lzma::lzma_encoder_presets::LZMA_LC_DEFAULT as u32;
pub const LZMA_LP_DEFAULT: u32 = xz::lzma::lzma_encoder_presets::LZMA_LP_DEFAULT as u32;
pub const LZMA_PB_MIN: u32 = xz::common::string_conversion::LZMA_PB_MIN as u32;
pub const LZMA_PB_MAX: u32 = xz::types::LZMA_PB_MAX as u32;
pub const LZMA_PB_DEFAULT: u32 = xz::lzma::lzma_encoder_presets::LZMA_PB_DEFAULT as u32;

/*********************
 * Advanced Features *
 *********************/

/* `lzma/stream_flags.h`: stream header size */
pub const LZMA_STREAM_HEADER_SIZE: u32 = xz::types::LZMA_STREAM_HEADER_SIZE as u32;
pub const LZMA_BLOCK_HEADER_SIZE_MIN: u32 =
    xz::common::block_util::LZMA_BLOCK_HEADER_SIZE_MIN as u32;
pub const LZMA_BLOCK_HEADER_SIZE_MAX: u32 = xz::types::LZMA_BLOCK_HEADER_SIZE_MAX as u32;

/*******************
 * Function Wrappers
 *******************/

/*
 * Wrappers follow `lzma.h`'s public subheader layout where practical.
 * They cast to canonical Rust implementation types and forward the call.
 */

/******************
 * Basic Features *
 ******************/

/* `lzma/version.h` */

#[no_mangle]
pub unsafe extern "C" fn lzma_version_number() -> u32 {
    xz::common::common::lzma_version_number()
}

#[no_mangle]
pub unsafe extern "C" fn lzma_version_string() -> *const c_char {
    xz::common::common::lzma_version_string()
}

/* `lzma/base.h`: common stream API */

#[no_mangle]
pub unsafe extern "C" fn lzma_code(strm: *mut lzma_stream, action: lzma_action) -> lzma_ret {
    xz::common::common::lzma_code(strm.cast(), action)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_end(strm: *mut lzma_stream) {
    xz::common::common::lzma_end(strm.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_memlimit_get(strm: *const lzma_stream) -> u64 {
    xz::common::common::lzma_memlimit_get(strm.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_memlimit_set(strm: *mut lzma_stream, new_memlimit: u64) -> lzma_ret {
    xz::common::common::lzma_memlimit_set(strm.cast(), new_memlimit)
}

/* `lzma/base.h`: allocation helpers */

#[no_mangle]
pub unsafe extern "C" fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void {
    xz::common::common::lzma_alloc(size, allocator)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_alloc_zero(
    size: size_t,
    allocator: *const lzma_allocator,
) -> *mut c_void {
    xz::common::common::lzma_alloc_zero(size, allocator)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator) {
    xz::common::common::lzma_free(ptr, allocator)
}

/* `lzma/base.h`: progress / memusage */

#[no_mangle]
pub unsafe extern "C" fn lzma_get_progress(
    strm: *mut lzma_stream,
    progress_in: *mut u64,
    progress_out: *mut u64,
) {
    xz::common::common::lzma_get_progress(strm.cast(), progress_in, progress_out)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_memusage(strm: *const lzma_stream) -> u64 {
    xz::common::common::lzma_memusage(strm.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_get_check(strm: *const lzma_stream) -> lzma_check {
    xz::common::common::lzma_get_check(strm.cast())
}

/* `lzma/vli.h` */

#[no_mangle]
pub unsafe extern "C" fn lzma_vli_encode(
    vli: lzma_vli,
    vli_pos: *mut size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    xz::common::vli_encoder::lzma_vli_encode(vli, vli_pos, out, out_pos, out_size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_vli_decode(
    vli: *mut lzma_vli,
    vli_pos: *mut size_t,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    xz::common::vli_decoder::lzma_vli_decode(vli, vli_pos, input, in_pos, in_size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_vli_size(vli: lzma_vli) -> u32 {
    xz::common::vli_size::lzma_vli_size(vli)
}

/* `lzma/check.h` */

#[no_mangle]
pub unsafe extern "C" fn lzma_check_is_supported(check: lzma_check) -> lzma_bool {
    xz::check::check::lzma_check_is_supported(check)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_check_size(check: lzma_check) -> u32 {
    xz::check::check::lzma_check_size(check)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_crc32(buf: *const u8, size: size_t, crc: u32) -> u32 {
    xz::check::crc32_fast::lzma_crc32(buf, size, crc)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_crc64(buf: *const u8, size: size_t, crc: u64) -> u64 {
    xz::check::crc64_fast::lzma_crc64(buf, size, crc)
}

/*********************
 * Container Formats *
 *********************/

/* `lzma/container.h`: easy encoder */

#[no_mangle]
pub unsafe extern "C" fn lzma_easy_encoder_memusage(preset: u32) -> u64 {
    xz::common::easy_encoder_memusage::lzma_easy_encoder_memusage(preset)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_easy_decoder_memusage(preset: u32) -> u64 {
    xz::common::easy_decoder_memusage::lzma_easy_decoder_memusage(preset)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_easy_encoder(
    strm: *mut lzma_stream,
    preset: u32,
    check: lzma_check,
) -> lzma_ret {
    xz::common::easy_encoder::lzma_easy_encoder(strm.cast(), preset, check)
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
    xz::common::easy_buffer_encoder::lzma_easy_buffer_encode(
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

/* `lzma/container.h`: stream encoder / decoder */

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_encoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
    check: lzma_check,
) -> lzma_ret {
    xz::common::stream_encoder::lzma_stream_encoder(strm.cast(), filters.cast(), check)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_decoder(
    strm: *mut lzma_stream,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    xz::common::stream_decoder::lzma_stream_decoder(strm.cast(), memlimit, flags)
}

/* `lzma/container.h`: alone encoder / decoder */

#[no_mangle]
pub unsafe extern "C" fn lzma_alone_encoder(
    strm: *mut lzma_stream,
    options: *const lzma_options_lzma,
) -> lzma_ret {
    xz::common::alone_encoder::lzma_alone_encoder(strm.cast(), options.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_alone_decoder(strm: *mut lzma_stream, memlimit: u64) -> lzma_ret {
    xz::common::alone_decoder::lzma_alone_decoder(strm.cast(), memlimit)
}

/* `lzma/container.h`: auto / lzip decoder */

#[no_mangle]
pub unsafe extern "C" fn lzma_auto_decoder(
    strm: *mut lzma_stream,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    xz::common::auto_decoder::lzma_auto_decoder(strm.cast(), memlimit, flags)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_lzip_decoder(
    strm: *mut lzma_stream,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    xz::common::lzip_decoder::lzma_lzip_decoder(strm.cast(), memlimit, flags)
}

/* `lzma/container.h`: stream buffer */

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_buffer_bound(uncompressed_size: size_t) -> size_t {
    xz::common::stream_buffer_encoder::lzma_stream_buffer_bound(uncompressed_size)
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
    xz::common::stream_buffer_encoder::lzma_stream_buffer_encode(
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
    xz::common::stream_buffer_decoder::lzma_stream_buffer_decode(
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

/***********
 * Filters *
 ***********/

/* `lzma/filter.h` */

#[no_mangle]
pub unsafe extern "C" fn lzma_filter_encoder_is_supported(id: lzma_vli) -> lzma_bool {
    xz::common::filter_encoder::lzma_filter_encoder_is_supported(id)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_filter_decoder_is_supported(id: lzma_vli) -> lzma_bool {
    xz::common::filter_decoder::lzma_filter_decoder_is_supported(id)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_filters_copy(
    src: *const lzma_filter,
    dest: *mut lzma_filter,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    xz::common::filter_common::lzma_filters_copy(src.cast(), dest.cast(), allocator.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_raw_encoder_memusage(filters: *const lzma_filter) -> u64 {
    xz::common::filter_encoder::lzma_raw_encoder_memusage(filters.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_raw_decoder_memusage(filters: *const lzma_filter) -> u64 {
    xz::common::filter_decoder::lzma_raw_decoder_memusage(filters.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_raw_encoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
) -> lzma_ret {
    xz::common::filter_encoder::lzma_raw_encoder(strm.cast(), filters.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_raw_decoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
) -> lzma_ret {
    xz::common::filter_decoder::lzma_raw_decoder(strm.cast(), filters.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_filters_update(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
) -> lzma_ret {
    xz::common::filter_encoder::lzma_filters_update(strm.cast(), filters.cast())
}

/* `lzma/filter.h`: raw buffer */

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
    xz::common::filter_buffer_encoder::lzma_raw_buffer_encode(
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
    xz::common::filter_buffer_decoder::lzma_raw_buffer_decode(
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

/* `lzma/filter.h`: properties */

#[no_mangle]
pub unsafe extern "C" fn lzma_properties_size(
    size: *mut u32,
    filter: *const lzma_filter,
) -> lzma_ret {
    xz::common::filter_encoder::lzma_properties_size(size, filter.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_properties_encode(
    filter: *const lzma_filter,
    props: *mut u8,
) -> lzma_ret {
    xz::common::filter_encoder::lzma_properties_encode(filter.cast(), props)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_properties_decode(
    filter: *mut lzma_filter,
    allocator: *const lzma_allocator,
    props: *const u8,
    props_size: size_t,
) -> lzma_ret {
    xz::common::filter_decoder::lzma_properties_decode(
        filter.cast(),
        allocator.cast(),
        props,
        props_size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn lzma_mt_block_size(filters: *const lzma_filter) -> u64 {
    xz::common::filter_encoder::lzma_mt_block_size(filters.cast())
}

/* `lzma/lzma12.h` */

#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_preset(
    options: *mut lzma_options_lzma,
    preset: u32,
) -> lzma_bool {
    xz::lzma::lzma_encoder_presets::lzma_lzma_preset(options.cast(), preset)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_mf_is_supported(mf: lzma_match_finder) -> lzma_bool {
    xz::lz::lz_encoder::lzma_mf_is_supported(mf)
}

#[no_mangle]
pub extern "C" fn lzma_mode_is_supported(mode: lzma_mode) -> lzma_bool {
    xz::lzma::lzma_encoder::lzma_mode_is_supported(mode)
}

/* `lzma/bcj.h` */

#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_arm64_encode(
    start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    xz::simple::arm64::lzma_bcj_arm64_encode(start_offset, buf, size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_arm64_decode(
    start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    xz::simple::arm64::lzma_bcj_arm64_decode(start_offset, buf, size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_riscv_encode(
    start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    xz::simple::riscv::lzma_bcj_riscv_encode(start_offset, buf, size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_riscv_decode(
    start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    xz::simple::riscv::lzma_bcj_riscv_decode(start_offset, buf, size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_x86_encode(
    start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    xz::simple::x86::lzma_bcj_x86_encode(start_offset, buf, size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_x86_decode(
    start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    xz::simple::x86::lzma_bcj_x86_decode(start_offset, buf, size)
}

/*********************
 * Advanced Features *
 *********************/

/* `lzma/stream_flags.h` */

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_header_encode(
    options: *const lzma_stream_flags,
    out: *mut u8,
) -> lzma_ret {
    xz::common::stream_flags_encoder::lzma_stream_header_encode(options.cast(), out)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_footer_encode(
    options: *const lzma_stream_flags,
    out: *mut u8,
) -> lzma_ret {
    xz::common::stream_flags_encoder::lzma_stream_footer_encode(options.cast(), out)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_header_decode(
    options: *mut lzma_stream_flags,
    input: *const u8,
) -> lzma_ret {
    xz::common::stream_flags_decoder::lzma_stream_header_decode(options.cast(), input)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_footer_decode(
    options: *mut lzma_stream_flags,
    input: *const u8,
) -> lzma_ret {
    xz::common::stream_flags_decoder::lzma_stream_footer_decode(options.cast(), input)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_stream_flags_compare(
    a: *const lzma_stream_flags,
    b: *const lzma_stream_flags,
) -> lzma_ret {
    xz::common::stream_flags_common::lzma_stream_flags_compare(a.cast(), b.cast())
}

/* `lzma/hardware.h` */

#[no_mangle]
pub unsafe extern "C" fn lzma_physmem() -> u64 {
    xz::common::hardware_physmem::lzma_physmem()
}

#[no_mangle]
pub unsafe extern "C" fn lzma_cputhreads() -> u32 {
    xz::common::hardware_cputhreads::lzma_cputhreads()
}

/* `lzma/index.h`: core index operations */

#[no_mangle]
pub unsafe extern "C" fn lzma_index_buffer_decode(
    i: *mut *mut lzma_index,
    memlimit: *mut u64,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    xz::common::index_decoder::lzma_index_buffer_decode(
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
    xz::common::index::lzma_index_uncompressed_size(i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_end(i: *mut lzma_index, allocator: *const lzma_allocator) {
    xz::common::index::lzma_index_end(i.cast(), allocator.cast())
}

/* `lzma/block.h` */

#[no_mangle]
pub unsafe extern "C" fn lzma_block_header_size(block: *mut lzma_block) -> lzma_ret {
    xz::common::block_header_encoder::lzma_block_header_size(block.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_header_encode(
    block: *const lzma_block,
    out: *mut u8,
) -> lzma_ret {
    xz::common::block_header_encoder::lzma_block_header_encode(block.cast(), out)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_header_decode(
    block: *mut lzma_block,
    allocator: *const lzma_allocator,
    input: *const u8,
) -> lzma_ret {
    xz::common::block_header_decoder::lzma_block_header_decode(
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
    xz::common::block_util::lzma_block_compressed_size(block.cast(), unpadded_size)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_unpadded_size(block: *const lzma_block) -> lzma_vli {
    xz::common::block_util::lzma_block_unpadded_size(block.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_total_size(block: *const lzma_block) -> lzma_vli {
    xz::common::block_util::lzma_block_total_size(block.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_encoder(
    strm: *mut lzma_stream,
    block: *mut lzma_block,
) -> lzma_ret {
    xz::common::block_encoder::lzma_block_encoder(strm.cast(), block.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_block_decoder(
    strm: *mut lzma_stream,
    block: *mut lzma_block,
) -> lzma_ret {
    xz::common::block_decoder::lzma_block_decoder(strm.cast(), block.cast())
}

#[no_mangle]
pub extern "C" fn lzma_block_buffer_bound(uncompressed_size: size_t) -> size_t {
    xz::common::block_buffer_encoder::lzma_block_buffer_bound(uncompressed_size)
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
    xz::common::block_buffer_encoder::lzma_block_buffer_encode(
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
    xz::common::block_buffer_encoder::lzma_block_uncomp_encode(
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
    xz::common::block_buffer_decoder::lzma_block_buffer_decode(
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

/* `lzma/index.h`: extended index operations */

#[no_mangle]
pub extern "C" fn lzma_index_memusage(streams: lzma_vli, blocks: lzma_vli) -> u64 {
    xz::common::index::lzma_index_memusage(streams, blocks)
}

#[no_mangle]
pub extern "C" fn lzma_index_memused(i: *const lzma_index) -> u64 {
    xz::common::index::lzma_index_memused(i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_init(allocator: *const lzma_allocator) -> *mut lzma_index {
    xz::common::index::lzma_index_init(allocator.cast()).cast()
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_append(
    i: *mut lzma_index,
    allocator: *const lzma_allocator,
    unpadded_size: lzma_vli,
    uncompressed_size: lzma_vli,
) -> lzma_ret {
    xz::common::index::lzma_index_append(
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
    xz::common::index::lzma_index_stream_flags(i.cast(), stream_flags.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_checks(i: *const lzma_index) -> u32 {
    xz::common::index::lzma_index_checks(i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_stream_padding(
    i: *mut lzma_index,
    stream_padding: lzma_vli,
) -> lzma_ret {
    xz::common::index::lzma_index_stream_padding(i.cast(), stream_padding)
}

#[no_mangle]
pub extern "C" fn lzma_index_stream_count(i: *const lzma_index) -> lzma_vli {
    xz::common::index::lzma_index_stream_count(i.cast())
}

#[no_mangle]
pub extern "C" fn lzma_index_block_count(i: *const lzma_index) -> lzma_vli {
    xz::common::index::lzma_index_block_count(i.cast())
}

#[no_mangle]
pub extern "C" fn lzma_index_size(i: *const lzma_index) -> lzma_vli {
    xz::common::index::lzma_index_size(i.cast())
}

#[no_mangle]
pub extern "C" fn lzma_index_stream_size(i: *const lzma_index) -> lzma_vli {
    xz::common::index::lzma_index_stream_size(i.cast())
}

#[no_mangle]
pub extern "C" fn lzma_index_total_size(i: *const lzma_index) -> lzma_vli {
    xz::common::index::lzma_index_total_size(i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_file_size(i: *const lzma_index) -> lzma_vli {
    xz::common::index::lzma_index_file_size(i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_init(iter: *mut lzma_index_iter, i: *const lzma_index) {
    xz::common::index::lzma_index_iter_init(iter.cast(), i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_rewind(iter: *mut lzma_index_iter) {
    xz::common::index::lzma_index_iter_rewind(iter.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_next(
    iter: *mut lzma_index_iter,
    mode: lzma_index_iter_mode,
) -> lzma_bool {
    xz::common::index::lzma_index_iter_next(iter.cast(), mode)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_locate(
    iter: *mut lzma_index_iter,
    target: lzma_vli,
) -> lzma_bool {
    xz::common::index::lzma_index_iter_locate(iter.cast(), target)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_cat(
    dest: *mut lzma_index,
    src: *mut lzma_index,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    xz::common::index::lzma_index_cat(dest.cast(), src.cast(), allocator.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_dup(
    i: *const lzma_index,
    allocator: *const lzma_allocator,
) -> *mut lzma_index {
    xz::common::index::lzma_index_dup(i.cast(), allocator.cast()).cast()
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_encoder(
    strm: *mut lzma_stream,
    i: *const lzma_index,
) -> lzma_ret {
    xz::common::index_encoder::lzma_index_encoder(strm.cast(), i.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_decoder(
    strm: *mut lzma_stream,
    i: *mut *mut lzma_index,
    memlimit: u64,
) -> lzma_ret {
    xz::common::index_decoder::lzma_index_decoder(strm.cast(), i.cast(), memlimit)
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_buffer_encode(
    i: *const lzma_index,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    xz::common::index_encoder::lzma_index_buffer_encode(i.cast(), out, out_pos, out_size)
}

/* `lzma/index_hash.h` */

#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_init(
    index_hash: *mut lzma_index_hash,
    allocator: *const lzma_allocator,
) -> *mut lzma_index_hash {
    xz::common::index_hash::lzma_index_hash_init(index_hash.cast(), allocator.cast()).cast()
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_end(
    index_hash: *mut lzma_index_hash,
    allocator: *const lzma_allocator,
) {
    xz::common::index_hash::lzma_index_hash_end(index_hash.cast(), allocator.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_append(
    index_hash: *mut lzma_index_hash,
    unpadded_size: lzma_vli,
    uncompressed_size: lzma_vli,
) -> lzma_ret {
    xz::common::index_hash::lzma_index_hash_append(
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
    xz::common::index_hash::lzma_index_hash_decode(index_hash.cast(), input, in_pos, in_size)
}

#[no_mangle]
pub extern "C" fn lzma_index_hash_size(index_hash: *const lzma_index_hash) -> lzma_vli {
    xz::common::index_hash::lzma_index_hash_size(index_hash.cast())
}

/* `lzma/filter.h`: filter flags / string conversion */

#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_size(
    size: *mut u32,
    filter: *const lzma_filter,
) -> lzma_ret {
    xz::common::filter_flags_encoder::lzma_filter_flags_size(size, filter.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_encode(
    filter: *const lzma_filter,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    xz::common::filter_flags_encoder::lzma_filter_flags_encode(
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
    xz::common::filter_flags_decoder::lzma_filter_flags_decode(
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
    xz::common::filter_common::lzma_filters_free(filters.cast(), allocator.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_str_to_filters(
    str_: *const c_char,
    error_pos: *mut c_int,
    filters: *mut lzma_filter,
    flags: u32,
    allocator: *const lzma_allocator,
) -> *const c_char {
    xz::common::string_conversion::lzma_str_to_filters(
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
    xz::common::string_conversion::lzma_str_from_filters(
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
    xz::common::string_conversion::lzma_str_list_filters(str_, filter_id, flags, allocator.cast())
}

/* `lzma/container.h`: additional container helpers */

#[no_mangle]
pub unsafe extern "C" fn lzma_microlzma_encoder(
    strm: *mut lzma_stream,
    options: *const lzma_options_lzma,
) -> lzma_ret {
    xz::common::microlzma_encoder::lzma_microlzma_encoder(strm.cast(), options.cast())
}

#[no_mangle]
pub unsafe extern "C" fn lzma_microlzma_decoder(
    strm: *mut lzma_stream,
    comp_size: u64,
    uncomp_size: u64,
    uncomp_size_is_exact: lzma_bool,
    dict_size: u32,
) -> lzma_ret {
    xz::common::microlzma_decoder::lzma_microlzma_decoder(
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
    xz::common::file_info::lzma_file_info_decoder(strm.cast(), i.cast(), memlimit, file_size)
}

/*********************
 * Container Formats *
 *********************/

/* `lzma/container.h`: multithreaded API */

#[cfg(feature = "parallel")]
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_encoder_mt(
    strm: *mut lzma_stream,
    options: *const lzma_mt,
) -> lzma_ret {
    xz::common::stream_mt::lzma_stream_encoder_mt(strm.cast(), options.cast())
}

#[cfg(feature = "parallel")]
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_decoder_mt(
    strm: *mut lzma_stream,
    options: *const lzma_mt,
) -> lzma_ret {
    xz::common::stream_mt::lzma_stream_decoder_mt(strm.cast(), options.cast())
}

#[cfg(feature = "parallel")]
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_encoder_mt_memusage(options: *const lzma_mt) -> u64 {
    xz::common::stream_mt::lzma_stream_encoder_mt_memusage(options.cast())
}
