use crate::types::*;
use core::ffi::c_void;
extern "C" {
    fn lzma_vli_encode(
        vli: lzma_vli,
        vli_pos: *mut size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> lzma_ret;
    fn lzma_vli_size(vli: lzma_vli) -> u32;
    fn lzma_crc32(buf: *const u8, size: size_t, crc: u32) -> u32;
    fn lzma_filter_flags_size(size: *mut u32, filter: *const lzma_filter) -> lzma_ret;
    fn lzma_filter_flags_encode(
        filter: *const lzma_filter,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> lzma_ret;
    fn lzma_block_unpadded_size(block: *const lzma_block) -> lzma_vli;
}
#[inline]
extern "C" fn write32le(buf: *mut u8, num: u32) {
    unsafe {
        *buf.offset(0) = num as u8;
        *buf.offset(1) = (num >> 8) as u8;
        *buf.offset(2) = (num >> 16) as u8;
        *buf.offset(3) = (num >> 24) as u8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_header_size(block: *mut lzma_block) -> lzma_ret {
    if (*block).version > 1 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut size: u32 = (1 + 1 + 4) as u32;
    if (*block).compressed_size != LZMA_VLI_UNKNOWN {
        let add: u32 = lzma_vli_size((*block).compressed_size) as u32;
        if add == 0 || (*block).compressed_size == 0 {
            return LZMA_PROG_ERROR;
        }
        size = size.wrapping_add(add);
    }
    if (*block).uncompressed_size != LZMA_VLI_UNKNOWN {
        let add_0: u32 = lzma_vli_size((*block).uncompressed_size) as u32;
        if add_0 == 0 {
            return LZMA_PROG_ERROR;
        }
        size = size.wrapping_add(add_0);
    }
    if (*block).filters.is_null() || (*(*block).filters.offset(0)).id == LZMA_VLI_UNKNOWN {
        return LZMA_PROG_ERROR;
    }
    let mut i: size_t = 0;
    while (*(*block).filters.offset(i as isize)).id != LZMA_VLI_UNKNOWN {
        if i == LZMA_FILTERS_MAX as size_t {
            return LZMA_PROG_ERROR;
        }
        let mut add_1: u32 = 0;
        let ret_: lzma_ret =
            lzma_filter_flags_size(&raw mut add_1, (*block).filters.offset(i as isize));
        if ret_ != LZMA_OK {
            return ret_;
        }
        size = size.wrapping_add(add_1);
        i = i.wrapping_add(1);
    }
    (*block).header_size = size.wrapping_add(3) & !(3);
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_header_encode(
    block: *const lzma_block,
    out: *mut u8,
) -> lzma_ret {
    if lzma_block_unpadded_size(block) == 0
        || !((*block).uncompressed_size <= LZMA_VLI_MAX
            || (*block).uncompressed_size == LZMA_VLI_UNKNOWN)
    {
        return LZMA_PROG_ERROR;
    }
    let out_size: size_t = (*block).header_size.wrapping_sub(4) as size_t;
    *out.offset(0) = out_size.wrapping_div(4) as u8;
    *out.offset(1) = 0;
    let mut out_pos: size_t = 2;
    if (*block).compressed_size != LZMA_VLI_UNKNOWN {
        let ret_: lzma_ret = lzma_vli_encode(
            (*block).compressed_size,
            core::ptr::null_mut(),
            out,
            &raw mut out_pos,
            out_size,
        );
        if ret_ != LZMA_OK {
            return ret_;
        }
        let ref mut fresh0 = *out.offset(1);
        *fresh0 |= 0x40;
    }
    if (*block).uncompressed_size != LZMA_VLI_UNKNOWN {
        let ret__0: lzma_ret = lzma_vli_encode(
            (*block).uncompressed_size,
            core::ptr::null_mut(),
            out,
            &raw mut out_pos,
            out_size,
        );
        if ret__0 != LZMA_OK {
            return ret__0;
        }
        let ref mut fresh1 = *out.offset(1);
        *fresh1 |= 0x80;
    }
    if (*block).filters.is_null() || (*(*block).filters.offset(0)).id == LZMA_VLI_UNKNOWN {
        return LZMA_PROG_ERROR;
    }
    let mut filter_count: size_t = 0;
    loop {
        if filter_count == LZMA_FILTERS_MAX as size_t {
            return LZMA_PROG_ERROR;
        }
        let ret__1: lzma_ret = lzma_filter_flags_encode(
            (*block).filters.offset(filter_count as isize),
            out,
            &raw mut out_pos,
            out_size,
        );
        if ret__1 != LZMA_OK {
            return ret__1;
        }
        filter_count = filter_count.wrapping_add(1);
        if !((*(*block).filters.offset(filter_count as isize)).id != LZMA_VLI_UNKNOWN) {
            break;
        }
    }
    let ref mut fresh2 = *out.offset(1);
    *fresh2 = (*fresh2 as size_t | filter_count.wrapping_sub(1)) as u8;
    memset(
        out.offset(out_pos as isize) as *mut c_void,
        0,
        out_size.wrapping_sub(out_pos),
    );
    write32le(out.offset(out_size as isize), lzma_crc32(out, out_size, 0));
    return LZMA_OK;
}
