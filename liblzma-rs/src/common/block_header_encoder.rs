use crate::types::*;
use core::ffi::{c_int, c_ulonglong, c_void};
extern "C" {
    fn memset(__b: *mut c_void, __c: c_int, __len: size_t) -> *mut c_void;
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
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
pub const LZMA_RET_INTERNAL8: lzma_ret = 108;
pub const LZMA_RET_INTERNAL7: lzma_ret = 107;
pub const LZMA_RET_INTERNAL6: lzma_ret = 106;
pub const LZMA_RET_INTERNAL5: lzma_ret = 105;
pub const LZMA_RET_INTERNAL4: lzma_ret = 104;
pub const LZMA_RET_INTERNAL3: lzma_ret = 103;
pub const LZMA_RET_INTERNAL2: lzma_ret = 102;
pub const LZMA_RET_INTERNAL1: lzma_ret = 101;
pub const LZMA_SEEK_NEEDED: lzma_ret = 12;
pub const LZMA_PROG_ERROR: lzma_ret = 11;
pub const LZMA_BUF_ERROR: lzma_ret = 10;
pub const LZMA_DATA_ERROR: lzma_ret = 9;
pub const LZMA_OPTIONS_ERROR: lzma_ret = 8;
pub const LZMA_FORMAT_ERROR: lzma_ret = 7;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret = 6;
pub const LZMA_MEM_ERROR: lzma_ret = 5;
pub const LZMA_GET_CHECK: lzma_ret = 4;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret = 3;
pub const LZMA_NO_CHECK: lzma_ret = 2;
pub const LZMA_STREAM_END: lzma_ret = 1;
pub const LZMA_OK: lzma_ret = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut c_void,
}
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_block {
    pub version: u32,
    pub header_size: u32,
    pub check: lzma_check,
    pub compressed_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub filters: *mut lzma_filter,
    pub raw_check: [u8; 64],
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
    pub reserved_ptr3: *mut c_void,
    pub reserved_int1: u32,
    pub reserved_int2: u32,
    pub reserved_int3: lzma_vli,
    pub reserved_int4: lzma_vli,
    pub reserved_int5: lzma_vli,
    pub reserved_int6: lzma_vli,
    pub reserved_int7: lzma_vli,
    pub reserved_int8: lzma_vli,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub ignore_check: lzma_bool,
    pub reserved_bool2: lzma_bool,
    pub reserved_bool3: lzma_bool,
    pub reserved_bool4: lzma_bool,
    pub reserved_bool5: lzma_bool,
    pub reserved_bool6: lzma_bool,
    pub reserved_bool7: lzma_bool,
    pub reserved_bool8: lzma_bool,
}
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
#[inline]
extern "C" fn write32le(buf: *mut u8, num: u32) {
    unsafe {
        *buf.offset(0) = num as u8;
        *buf.offset(1) = (num >> 8) as u8;
        *buf.offset(2) = (num >> 16) as u8;
        *buf.offset(3) = (num >> 24) as u8;
    }
}
pub const LZMA_VLI_MAX: c_ulonglong = UINT64_MAX.wrapping_div(2);
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_FILTERS_MAX: c_int = 4;
#[no_mangle]
pub unsafe extern "C" fn lzma_block_header_size(block: *mut lzma_block) -> lzma_ret {
    if (*block).version > 1 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut size: u32 = (1 as c_int + 1 as c_int + 4 as c_int) as u32;
    if (*block).compressed_size != LZMA_VLI_UNKNOWN as lzma_vli {
        let add: u32 = lzma_vli_size((*block).compressed_size) as u32;
        if add == 0 || (*block).compressed_size == 0 as lzma_vli {
            return LZMA_PROG_ERROR;
        }
        size = size.wrapping_add(add);
    }
    if (*block).uncompressed_size != LZMA_VLI_UNKNOWN as lzma_vli {
        let add_0: u32 = lzma_vli_size((*block).uncompressed_size) as u32;
        if add_0 == 0 {
            return LZMA_PROG_ERROR;
        }
        size = size.wrapping_add(add_0);
    }
    if (*block).filters.is_null()
        || (*(*block).filters.offset(0)).id == LZMA_VLI_UNKNOWN as lzma_vli
    {
        return LZMA_PROG_ERROR;
    }
    let mut i: size_t = 0;
    while (*(*block).filters.offset(i as isize)).id != LZMA_VLI_UNKNOWN as lzma_vli {
        if i == LZMA_FILTERS_MAX as size_t {
            return LZMA_PROG_ERROR;
        }
        let mut add_1: u32 = 0;
        let ret_: lzma_ret =
            lzma_filter_flags_size(&raw mut add_1, (*block).filters.offset(i as isize)) as lzma_ret;
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
    if lzma_block_unpadded_size(block) == 0 as lzma_vli
        || !((*block).uncompressed_size <= LZMA_VLI_MAX as lzma_vli
            || (*block).uncompressed_size == LZMA_VLI_UNKNOWN as lzma_vli)
    {
        return LZMA_PROG_ERROR;
    }
    let out_size: size_t = (*block).header_size.wrapping_sub(4) as size_t;
    *out.offset(0) = out_size.wrapping_div(4) as u8;
    *out.offset(1) = 0;
    let mut out_pos: size_t = 2;
    if (*block).compressed_size != LZMA_VLI_UNKNOWN as lzma_vli {
        let ret_: lzma_ret = lzma_vli_encode(
            (*block).compressed_size,
            core::ptr::null_mut(),
            out,
            &raw mut out_pos,
            out_size,
        ) as lzma_ret;
        if ret_ != LZMA_OK {
            return ret_;
        }
        let ref mut fresh0 = *out.offset(1);
        *fresh0 = (*fresh0 as c_int | 0x40 as c_int) as u8;
    }
    if (*block).uncompressed_size != LZMA_VLI_UNKNOWN as lzma_vli {
        let ret__0: lzma_ret = lzma_vli_encode(
            (*block).uncompressed_size,
            core::ptr::null_mut(),
            out,
            &raw mut out_pos,
            out_size,
        ) as lzma_ret;
        if ret__0 != LZMA_OK {
            return ret__0;
        }
        let ref mut fresh1 = *out.offset(1);
        *fresh1 = (*fresh1 as c_int | 0x80 as c_int) as u8;
    }
    if (*block).filters.is_null()
        || (*(*block).filters.offset(0)).id == LZMA_VLI_UNKNOWN as lzma_vli
    {
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
        ) as lzma_ret;
        if ret__1 != LZMA_OK {
            return ret__1;
        }
        filter_count = filter_count.wrapping_add(1);
        if !((*(*block).filters.offset(filter_count as isize)).id != LZMA_VLI_UNKNOWN as lzma_vli) {
            break;
        }
    }
    let ref mut fresh2 = *out.offset(1);
    *fresh2 = (*fresh2 as size_t | filter_count.wrapping_sub(1)) as u8;
    memset(
        out.offset(out_pos as isize) as *mut c_void,
        0 as c_int,
        out_size.wrapping_sub(out_pos),
    );
    write32le(out.offset(out_size as isize), lzma_crc32(out, out_size, 0));
    return LZMA_OK;
}
