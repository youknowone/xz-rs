use crate::types::*;
use core::ffi::{c_uint, c_ulonglong, c_void};
extern "C" {
    fn lzma_check_is_supported(check: lzma_check) -> lzma_bool;
    fn lzma_check_size(check: lzma_check) -> u32;
    fn lzma_block_header_size(block: *mut lzma_block) -> lzma_ret;
    fn lzma_block_header_encode(block: *const lzma_block, out: *mut u8) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_raw_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter,
    ) -> lzma_ret;
    fn lzma_check_init(check: *mut lzma_check_state, type_0: lzma_check);
    fn lzma_check_update(
        check: *mut lzma_check_state,
        type_0: lzma_check,
        buf: *const u8,
        size: size_t,
    );
    fn lzma_check_finish(check: *mut lzma_check_state, type_0: lzma_check);
}
pub const LZMA_CHECK_SIZE_MAX: u32 = 64;
pub const COMPRESSED_SIZE_MAX: c_ulonglong = LZMA_VLI_MAX
    .wrapping_sub(LZMA_BLOCK_HEADER_SIZE_MAX as u64)
    .wrapping_sub(LZMA_CHECK_SIZE_MAX as u64)
    & !3;
pub const LZMA2_CHUNK_MAX: c_uint = 1u32 << 16;
pub const LZMA2_HEADER_UNCOMPRESSED: u32 = 3;
pub const HEADERS_BOUND: u32 =
    1 + 1 + 2 * LZMA_VLI_BYTES_MAX + 3 + 4 + LZMA_CHECK_SIZE_MAX + 3 & !(3);
extern "C" fn lzma2_bound(uncompressed_size: u64) -> u64 {
    if uncompressed_size > COMPRESSED_SIZE_MAX as u64 {
        return 0;
    }
    let overhead: u64 = uncompressed_size
        .wrapping_add(LZMA2_CHUNK_MAX as u64)
        .wrapping_sub(1)
        .wrapping_div(LZMA2_CHUNK_MAX as u64)
        .wrapping_mul(LZMA2_HEADER_UNCOMPRESSED as u64)
        .wrapping_add(1);
    if (COMPRESSED_SIZE_MAX as u64).wrapping_sub(overhead) < uncompressed_size {
        return 0;
    }
    return uncompressed_size.wrapping_add(overhead);
}
#[no_mangle]
pub extern "C" fn lzma_block_buffer_bound64(uncompressed_size: u64) -> u64 {
    let mut lzma2_size: u64 = lzma2_bound(uncompressed_size);
    if lzma2_size == 0 {
        return 0;
    }
    lzma2_size = lzma2_size.wrapping_add(3) & !(3);
    return (HEADERS_BOUND as u64).wrapping_add(lzma2_size);
}
#[no_mangle]
pub extern "C" fn lzma_block_buffer_bound(uncompressed_size: size_t) -> size_t {
    let ret: u64 = lzma_block_buffer_bound64(uncompressed_size as u64);
    return ret as size_t;
}
unsafe extern "C" fn block_encode_uncompressed(
    block: *mut lzma_block,
    in_0: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    let mut lzma2: lzma_options_lzma = lzma_options_lzma {
        dict_size: LZMA_DICT_SIZE_MIN as u32,
        preset_dict: ::core::ptr::null::<u8>(),
        preset_dict_size: 0,
        lc: 0,
        lp: 0,
        pb: 0,
        mode: 0 as lzma_mode,
        nice_len: 0,
        mf: 0 as lzma_match_finder,
        depth: 0,
        ext_flags: 0,
        ext_size_low: 0,
        ext_size_high: 0,
        reserved_int4: 0,
        reserved_int5: 0,
        reserved_int6: 0,
        reserved_int7: 0,
        reserved_int8: 0,
        reserved_enum1: LZMA_RESERVED_ENUM,
        reserved_enum2: LZMA_RESERVED_ENUM,
        reserved_enum3: LZMA_RESERVED_ENUM,
        reserved_enum4: LZMA_RESERVED_ENUM,
        reserved_ptr1: core::ptr::null_mut(),
        reserved_ptr2: core::ptr::null_mut(),
    };
    let mut filters: [lzma_filter; 2] = [lzma_filter {
        id: 0,
        options: core::ptr::null_mut(),
    }; 2];
    filters[0].id = LZMA_FILTER_LZMA2;
    filters[0].options = &raw mut lzma2 as *mut c_void;
    filters[1].id = LZMA_VLI_UNKNOWN;
    let filters_orig: *mut lzma_filter = (*block).filters;
    (*block).filters = &raw mut filters as *mut lzma_filter;
    if lzma_block_header_size(block) != LZMA_OK {
        (*block).filters = filters_orig;
        return LZMA_PROG_ERROR;
    }
    if (out_size.wrapping_sub(*out_pos) as lzma_vli)
        < ((*block).header_size as lzma_vli).wrapping_add((*block).compressed_size)
    {
        (*block).filters = filters_orig;
        return LZMA_BUF_ERROR;
    }
    if lzma_block_header_encode(block, out.offset(*out_pos as isize)) != LZMA_OK {
        (*block).filters = filters_orig;
        return LZMA_PROG_ERROR;
    }
    (*block).filters = filters_orig;
    *out_pos = (*out_pos).wrapping_add((*block).header_size as size_t);
    let mut in_pos: size_t = 0;
    let mut control: u8 = 0x1 as u8;
    while in_pos < in_size {
        let fresh1 = *out_pos;
        *out_pos = (*out_pos).wrapping_add(1);
        *out.offset(fresh1 as isize) = control;
        control = 0x2 as u8;
        let copy_size: size_t = if in_size.wrapping_sub(in_pos) < (1u32 << 16) as size_t {
            in_size.wrapping_sub(in_pos)
        } else {
            (1u32 << 16) as size_t
        };
        let fresh2 = *out_pos;
        *out_pos = (*out_pos).wrapping_add(1);
        *out.offset(fresh2 as isize) = (copy_size.wrapping_sub(1) >> 8) as u8;
        let fresh3 = *out_pos;
        *out_pos = (*out_pos).wrapping_add(1);
        *out.offset(fresh3 as isize) = (copy_size.wrapping_sub(1) & 0xff) as u8;
        memcpy(
            out.offset(*out_pos as isize) as *mut c_void,
            in_0.offset(in_pos as isize) as *const c_void,
            copy_size,
        );
        in_pos = in_pos.wrapping_add(copy_size);
        *out_pos = (*out_pos).wrapping_add(copy_size);
    }
    let fresh4 = *out_pos;
    *out_pos = (*out_pos).wrapping_add(1);
    *out.offset(fresh4 as isize) = 0;
    return LZMA_OK;
}
unsafe extern "C" fn block_encode_normal(
    block: *mut lzma_block,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_block_header_size(block);
    if ret_ != LZMA_OK {
        return ret_;
    }
    if out_size.wrapping_sub(*out_pos) <= (*block).header_size as size_t {
        return LZMA_BUF_ERROR;
    }
    let out_start: size_t = *out_pos;
    *out_pos = (*out_pos).wrapping_add((*block).header_size as size_t);
    if out_size.wrapping_sub(*out_pos) as lzma_vli > (*block).compressed_size {
        out_size = (*out_pos as lzma_vli).wrapping_add((*block).compressed_size) as size_t;
    }
    let mut raw_encoder: lzma_next_coder = lzma_next_coder_s {
        coder: core::ptr::null_mut(),
        id: LZMA_VLI_UNKNOWN,
        init: 0,
        code: None,
        end: None,
        get_progress: None,
        get_check: None,
        memconfig: None,
        update: None,
        set_out_limit: None,
    };
    let mut ret: lzma_ret =
        lzma_raw_encoder_init(&raw mut raw_encoder, allocator, (*block).filters);
    if ret == LZMA_OK {
        let mut in_pos: size_t = 0;
        ret = raw_encoder.code.expect("non-null function pointer")(
            raw_encoder.coder,
            allocator,
            in_0,
            &raw mut in_pos,
            in_size,
            out,
            out_pos,
            out_size,
            LZMA_FINISH,
        );
    }
    lzma_next_end(&raw mut raw_encoder, allocator);
    if ret == LZMA_STREAM_END {
        (*block).compressed_size = (*out_pos)
            .wrapping_sub(out_start.wrapping_add((*block).header_size as size_t))
            as lzma_vli;
        ret = lzma_block_header_encode(block, out.offset(out_start as isize));
        if ret != LZMA_OK {
            ret = LZMA_PROG_ERROR;
        }
    } else if ret == LZMA_OK {
        ret = LZMA_BUF_ERROR;
    }
    if ret != LZMA_OK {
        *out_pos = out_start;
    }
    return ret;
}
unsafe extern "C" fn block_buffer_encode(
    block: *mut lzma_block,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    mut out_size: size_t,
    try_to_compress: bool,
) -> lzma_ret {
    if block.is_null()
        || in_0.is_null() && in_size != 0
        || out.is_null()
        || out_pos.is_null()
        || *out_pos > out_size
    {
        return LZMA_PROG_ERROR;
    }
    if (*block).version > 1 {
        return LZMA_OPTIONS_ERROR;
    }
    if (*block).check > LZMA_CHECK_ID_MAX || try_to_compress && (*block).filters.is_null() {
        return LZMA_PROG_ERROR;
    }
    if lzma_check_is_supported((*block).check) == 0 {
        return LZMA_UNSUPPORTED_CHECK;
    }
    out_size = out_size.wrapping_sub(out_size.wrapping_sub(*out_pos) & 3);
    let check_size: size_t = lzma_check_size((*block).check) as size_t;
    if out_size.wrapping_sub(*out_pos) <= check_size {
        return LZMA_BUF_ERROR;
    }
    out_size = out_size.wrapping_sub(check_size);
    (*block).uncompressed_size = in_size as lzma_vli;
    (*block).compressed_size = lzma2_bound(in_size as u64) as lzma_vli;
    if (*block).compressed_size == 0 {
        return LZMA_DATA_ERROR;
    }
    let mut ret: lzma_ret = LZMA_BUF_ERROR;
    if try_to_compress {
        ret = block_encode_normal(block, allocator, in_0, in_size, out, out_pos, out_size);
    }
    if ret != LZMA_OK {
        if ret != LZMA_BUF_ERROR {
            return ret;
        }
        let ret_: lzma_ret =
            block_encode_uncompressed(block, in_0, in_size, out, out_pos, out_size);
        if ret_ != LZMA_OK {
            return ret_;
        }
    }
    let mut i: size_t = (*block).compressed_size as size_t;
    while i & 3 != 0 {
        let fresh0 = *out_pos;
        *out_pos = (*out_pos).wrapping_add(1);
        *out.offset(fresh0 as isize) = 0;
        i += 1;
    }
    if check_size > 0 {
        let mut check: lzma_check_state = lzma_check_state {
            buffer: lzma_check_state_buffer { u8_0: [0; 64] },
            state: lzma_check_state_inner { crc32: 0 },
        };
        lzma_check_init(&raw mut check, (*block).check);
        lzma_check_update(&raw mut check, (*block).check, in_0, in_size);
        lzma_check_finish(&raw mut check, (*block).check);
        memcpy(
            &raw mut (*block).raw_check as *mut u8 as *mut c_void,
            &raw mut check.buffer.u8_0 as *mut u8 as *const c_void,
            check_size,
        );
        memcpy(
            out.offset(*out_pos as isize) as *mut c_void,
            &raw mut check.buffer.u8_0 as *mut u8 as *const c_void,
            check_size,
        );
        *out_pos = (*out_pos).wrapping_add(check_size);
    }
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_buffer_encode(
    block: *mut lzma_block,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    return block_buffer_encode(
        block, allocator, in_0, in_size, out, out_pos, out_size, true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_uncomp_encode(
    block: *mut lzma_block,
    in_0: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    return block_buffer_encode(
        block,
        ::core::ptr::null::<lzma_allocator>(),
        in_0,
        in_size,
        out,
        out_pos,
        out_size,
        false,
    );
}
