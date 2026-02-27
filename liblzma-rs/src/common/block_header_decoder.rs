use crate::types::*;
use core::ffi::{c_int, c_uchar, c_uint, c_ulonglong, c_void};
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
pub struct lzma_allocator {
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, size_t, size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    pub opaque: *mut c_void,
}
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
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const false_0: c_int = 0 as c_int;
#[inline]
unsafe extern "C" fn read32le(mut buf: *const u8) -> u32 {
    let mut num: u32 = *buf.offset(0) as u32;
    num |= (*buf.offset(1) as u32) << 8;
    num |= (*buf.offset(2) as u32) << 16;
    num |= (*buf.offset(3) as u32) << 24;
    return num;
}
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_CHECK_ID_MAX: lzma_check = 15;
pub const LZMA_FILTERS_MAX: c_int = 4 as c_int;
#[no_mangle]
pub unsafe extern "C" fn lzma_block_header_decode(
    mut block: *mut lzma_block,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
) -> lzma_ret {
    if block.is_null() || (*block).filters.is_null() || in_0.is_null() {
        return LZMA_PROG_ERROR;
    }
    let mut i: size_t = 0 as size_t;
    while i <= LZMA_FILTERS_MAX as size_t {
        (*(*block).filters.offset(i as isize)).id = LZMA_VLI_UNKNOWN as lzma_vli;
        let ref mut fresh0 = (*(*block).filters.offset(i as isize)).options;
        *fresh0 = NULL;
        i = i.wrapping_add(1);
    }
    if (*block).version > 1 as u32 {
        (*block).version = 1 as u32;
    }
    (*block).ignore_check = false_0 as lzma_bool;
    if (*in_0.offset(0) as u32)
        .wrapping_add(1 as u32)
        .wrapping_mul(4 as u32)
        != (*block).header_size
        || (*block).check > LZMA_CHECK_ID_MAX
    {
        return LZMA_PROG_ERROR;
    }
    let in_size: size_t = (*block).header_size.wrapping_sub(4 as u32) as size_t;
    if lzma_crc32(in_0, in_size, 0 as u32) != read32le(in_0.offset(in_size as isize)) {
        return LZMA_DATA_ERROR;
    }
    if *in_0.offset(1) as c_int & 0x3c as c_int != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut in_pos: size_t = 2 as size_t;
    if *in_0.offset(1) as c_int & 0x40 as c_int != 0 {
        let ret_: lzma_ret = lzma_vli_decode(
            &raw mut (*block).compressed_size,
            ::core::ptr::null_mut::<size_t>(),
            in_0,
            &raw mut in_pos,
            in_size,
        ) as lzma_ret;
        if ret_ != LZMA_OK {
            return ret_;
        }
        if lzma_block_unpadded_size(block) == 0 as lzma_vli {
            return LZMA_DATA_ERROR;
        }
    } else {
        (*block).compressed_size = LZMA_VLI_UNKNOWN as lzma_vli;
    }
    if *in_0.offset(1) as c_int & 0x80 as c_int != 0 {
        let ret__0: lzma_ret = lzma_vli_decode(
            &raw mut (*block).uncompressed_size,
            ::core::ptr::null_mut::<size_t>(),
            in_0,
            &raw mut in_pos,
            in_size,
        ) as lzma_ret;
        if ret__0 != LZMA_OK {
            return ret__0;
        }
    } else {
        (*block).uncompressed_size = LZMA_VLI_UNKNOWN as lzma_vli;
    }
    let filter_count: size_t = (u32::from(*in_0.offset(1)) & 3).wrapping_add(1) as size_t;
    let mut i_0: size_t = 0 as size_t;
    while i_0 < filter_count {
        let ret: lzma_ret = lzma_filter_flags_decode(
            (*block).filters.offset(i_0 as isize) as *mut lzma_filter,
            allocator,
            in_0,
            &raw mut in_pos,
            in_size,
        ) as lzma_ret;
        if ret != LZMA_OK {
            lzma_filters_free((*block).filters, allocator);
            return ret;
        }
        i_0 = i_0.wrapping_add(1);
    }
    while in_pos < in_size {
        let fresh1 = in_pos;
        in_pos = in_pos.wrapping_add(1);
        if *in_0.offset(fresh1 as isize) as c_int != 0 as c_int {
            lzma_filters_free((*block).filters, allocator);
            return LZMA_OPTIONS_ERROR;
        }
    }
    return LZMA_OK;
}
