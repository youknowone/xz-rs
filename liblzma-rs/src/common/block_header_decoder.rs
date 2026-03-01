use crate::types::*;
extern "C" {
    fn lzma_vli_decode(
        vli: *mut lzma_vli,
        vli_pos: *mut size_t,
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
    ) -> lzma_ret;
    fn lzma_crc32(buf: *const u8, size: size_t, crc: u32) -> u32;
    fn lzma_filters_free(filters: *mut lzma_filter, allocator: *const lzma_allocator);
    fn lzma_filter_flags_decode(
        filter: *mut lzma_filter,
        allocator: *const lzma_allocator,
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
    ) -> lzma_ret;
    fn lzma_block_unpadded_size(block: *const lzma_block) -> lzma_vli;
}
#[inline]
extern "C" fn read32le(buf: *const u8) -> u32 {
    return unsafe {
        let mut num: u32 = *buf as u32;
        num |= (*buf.offset(1) as u32) << 8;
        num |= (*buf.offset(2) as u32) << 16;
        num |= (*buf.offset(3) as u32) << 24;
        num
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_header_decode(
    block: *mut lzma_block,
    allocator: *const lzma_allocator,
    in_0: *const u8,
) -> lzma_ret {
    if block.is_null() || (*block).filters.is_null() || in_0.is_null() {
        return LZMA_PROG_ERROR;
    }
    let mut i: size_t = 0;
    while i <= LZMA_FILTERS_MAX as size_t {
        (*(*block).filters.offset(i as isize)).id = LZMA_VLI_UNKNOWN;
        (*(*block).filters.offset(i as isize)).options = core::ptr::null_mut();
        i += 1;
    }
    if (*block).version > 1 {
        (*block).version = 1;
    }
    (*block).ignore_check = false as lzma_bool;
    if (*in_0 as u32).wrapping_add(1).wrapping_mul(4) != (*block).header_size
        || (*block).check > LZMA_CHECK_ID_MAX
    {
        return LZMA_PROG_ERROR;
    }
    let in_size: size_t = (*block).header_size.wrapping_sub(4) as size_t;
    if lzma_crc32(in_0, in_size, 0) != read32le(in_0.offset(in_size as isize)) {
        return LZMA_DATA_ERROR;
    }
    if *in_0.offset(1) & 0x3c != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut in_pos: size_t = 2;
    if *in_0.offset(1) & 0x40 != 0 {
        let ret_: lzma_ret = lzma_vli_decode(
            &raw mut (*block).compressed_size,
            core::ptr::null_mut(),
            in_0,
            &raw mut in_pos,
            in_size,
        );
        if ret_ != LZMA_OK {
            return ret_;
        }
        if lzma_block_unpadded_size(block) == 0 {
            return LZMA_DATA_ERROR;
        }
    } else {
        (*block).compressed_size = LZMA_VLI_UNKNOWN;
    }
    if *in_0.offset(1) & 0x80 != 0 {
        let ret__0: lzma_ret = lzma_vli_decode(
            &raw mut (*block).uncompressed_size,
            core::ptr::null_mut(),
            in_0,
            &raw mut in_pos,
            in_size,
        );
        if ret__0 != LZMA_OK {
            return ret__0;
        }
    } else {
        (*block).uncompressed_size = LZMA_VLI_UNKNOWN;
    }
    let filter_count: size_t = (u32::from(*in_0.offset(1)) & 3).wrapping_add(1) as size_t;
    let mut i_0: size_t = 0;
    while i_0 < filter_count {
        let ret: lzma_ret = lzma_filter_flags_decode(
            (*block).filters.offset(i_0 as isize) as *mut lzma_filter,
            allocator,
            in_0,
            &raw mut in_pos,
            in_size,
        );
        if ret != LZMA_OK {
            lzma_filters_free((*block).filters, allocator);
            return ret;
        }
        i_0 += 1;
    }
    while in_pos < in_size {
        if *in_0.offset(in_pos as isize) != 0 {
            lzma_filters_free((*block).filters, allocator);
            return LZMA_OPTIONS_ERROR;
        }
        in_pos += 1;
    }
    LZMA_OK
}
