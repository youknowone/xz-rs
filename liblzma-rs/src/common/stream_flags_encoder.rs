use crate::types::*;
use core::ffi::{c_int, c_uchar, c_uint, c_ulonglong, c_void};
extern "C" {
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn lzma_crc32(buf: *const u8, size: size_t, crc: u32) -> u32;
    static lzma_header_magic: [u8; 6];
    static lzma_footer_magic: [u8; 2];
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
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream_flags {
    pub version: u32,
    pub backward_size: lzma_vli,
    pub check: lzma_check,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_bool1: lzma_bool,
    pub reserved_bool2: lzma_bool,
    pub reserved_bool3: lzma_bool,
    pub reserved_bool4: lzma_bool,
    pub reserved_bool5: lzma_bool,
    pub reserved_bool6: lzma_bool,
    pub reserved_bool7: lzma_bool,
    pub reserved_bool8: lzma_bool,
    pub reserved_int1: u32,
    pub reserved_int2: u32,
}
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
#[inline]
unsafe extern "C" fn write32le(mut buf: *mut u8, mut num: u32) {
    *buf.offset(0) = num as u8;
    *buf.offset(1) = (num >> 8) as u8;
    *buf.offset(2) = (num >> 16) as u8;
    *buf.offset(3) = (num >> 24) as u8;
}
pub const LZMA_CHECK_ID_MAX: lzma_check = 15;
pub const LZMA_BACKWARD_SIZE_MIN: c_int = 4 as c_int;
pub const LZMA_BACKWARD_SIZE_MAX: c_ulonglong = 1 << 34;
pub const LZMA_STREAM_FLAGS_SIZE: c_int = 2 as c_int;
#[inline]
unsafe extern "C" fn is_backward_size_valid(mut options: *const lzma_stream_flags) -> bool {
    return (*options).backward_size >= LZMA_BACKWARD_SIZE_MIN as lzma_vli
        && (*options).backward_size <= LZMA_BACKWARD_SIZE_MAX as lzma_vli
        && (*options).backward_size & 3 as lzma_vli == 0 as lzma_vli;
}
unsafe extern "C" fn stream_flags_encode(
    mut options: *const lzma_stream_flags,
    mut out: *mut u8,
) -> bool {
    if (*options).check > LZMA_CHECK_ID_MAX {
        return true_0 != 0;
    }
    *out.offset(0) = 0 as u8;
    *out.offset(1) = (*options).check as u8;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_header_encode(
    mut options: *const lzma_stream_flags,
    mut out: *mut u8,
) -> lzma_ret {
    if (*options).version != 0 as u32 {
        return LZMA_OPTIONS_ERROR;
    }
    memcpy(
        out as *mut c_void,
        &raw const lzma_header_magic as *const u8 as *const c_void,
        ::core::mem::size_of::<[u8; 6]>() as size_t,
    );
    if stream_flags_encode(
        options,
        out.offset(::core::mem::size_of::<[u8; 6]>() as usize as isize),
    ) {
        return LZMA_PROG_ERROR;
    }
    let crc: u32 = lzma_crc32(
        out.offset(::core::mem::size_of::<[u8; 6]>() as usize as isize),
        LZMA_STREAM_FLAGS_SIZE as size_t,
        0 as u32,
    ) as u32;
    write32le(
        out.offset(::core::mem::size_of::<[u8; 6]>() as usize as isize)
            .offset(LZMA_STREAM_FLAGS_SIZE as isize),
        crc,
    );
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_footer_encode(
    mut options: *const lzma_stream_flags,
    mut out: *mut u8,
) -> lzma_ret {
    if (*options).version != 0 as u32 {
        return LZMA_OPTIONS_ERROR;
    }
    if !is_backward_size_valid(options) {
        return LZMA_PROG_ERROR;
    }
    write32le(
        out.offset(4),
        (*options)
            .backward_size
            .wrapping_div(4 as lzma_vli)
            .wrapping_sub(1 as lzma_vli) as u32,
    );
    if stream_flags_encode(options, out.offset((2 as c_int * 4) as isize)) {
        return LZMA_PROG_ERROR;
    }
    let crc: u32 = lzma_crc32(
        out.offset(4),
        (4 as c_int + LZMA_STREAM_FLAGS_SIZE) as size_t,
        0 as u32,
    ) as u32;
    write32le(out, crc);
    memcpy(
        out.offset((2 as c_int * 4) as isize)
            .offset(LZMA_STREAM_FLAGS_SIZE as isize) as *mut c_void,
        &raw const lzma_footer_magic as *const u8 as *const c_void,
        ::core::mem::size_of::<[u8; 2]>() as size_t,
    );
    return LZMA_OK;
}
