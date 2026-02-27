use crate::types::*;
use core::ffi::{c_int, c_uchar, c_uint, c_ulonglong, c_void};
extern "C" {
    fn memcmp(__s1: *const c_void, __s2: *const c_void, __n: size_t) -> c_int;
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
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const true_0: c_int = 1 as c_int;
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
pub const LZMA_STREAM_FLAGS_SIZE: c_int = 2 as c_int;
unsafe extern "C" fn stream_flags_decode(
    mut options: *mut lzma_stream_flags,
    mut in_0: *const u8,
) -> bool {
    if *in_0.offset(0) as c_int != 0 as c_int || *in_0.offset(1) as c_int & 0xf0 as c_int != 0 {
        return true_0 != 0;
    }
    (*options).version = 0 as u32;
    (*options).check = (*in_0.offset(1) as c_int & 0xf as c_int) as lzma_check;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_header_decode(
    mut options: *mut lzma_stream_flags,
    mut in_0: *const u8,
) -> lzma_ret {
    if memcmp(
        in_0 as *const c_void,
        &raw const lzma_header_magic as *const u8 as *const c_void,
        ::core::mem::size_of::<[u8; 6]>() as size_t,
    ) != 0 as c_int
    {
        return LZMA_FORMAT_ERROR;
    }
    let crc: u32 = lzma_crc32(
        in_0.offset(::core::mem::size_of::<[u8; 6]>() as usize as isize),
        LZMA_STREAM_FLAGS_SIZE as size_t,
        0 as u32,
    ) as u32;
    if crc
        != read32le(
            in_0.offset(::core::mem::size_of::<[u8; 6]>() as usize as isize)
                .offset(LZMA_STREAM_FLAGS_SIZE as isize),
        )
    {
        return LZMA_DATA_ERROR;
    }
    if stream_flags_decode(
        options,
        in_0.offset(::core::mem::size_of::<[u8; 6]>() as usize as isize),
    ) {
        return LZMA_OPTIONS_ERROR;
    }
    (*options).backward_size = LZMA_VLI_UNKNOWN as lzma_vli;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_footer_decode(
    mut options: *mut lzma_stream_flags,
    mut in_0: *const u8,
) -> lzma_ret {
    if memcmp(
        in_0.offset((::core::mem::size_of::<u32>() as usize).wrapping_mul(2 as usize) as isize)
            .offset(LZMA_STREAM_FLAGS_SIZE as isize) as *const c_void,
        &raw const lzma_footer_magic as *const u8 as *const c_void,
        ::core::mem::size_of::<[u8; 2]>() as size_t,
    ) != 0 as c_int
    {
        return LZMA_FORMAT_ERROR;
    }
    let crc: u32 = lzma_crc32(
        in_0.offset(::core::mem::size_of::<u32>() as usize as isize),
        (::core::mem::size_of::<u32>() as size_t).wrapping_add(LZMA_STREAM_FLAGS_SIZE as size_t),
        0 as u32,
    ) as u32;
    if crc != read32le(in_0) {
        return LZMA_DATA_ERROR;
    }
    if stream_flags_decode(
        options,
        in_0.offset((::core::mem::size_of::<u32>() as usize).wrapping_mul(2 as usize) as isize),
    ) {
        return LZMA_OPTIONS_ERROR;
    }
    (*options).backward_size =
        read32le(in_0.offset(::core::mem::size_of::<u32>() as usize as isize)) as lzma_vli;
    (*options).backward_size = (*options)
        .backward_size
        .wrapping_add(1 as lzma_vli)
        .wrapping_mul(4 as lzma_vli);
    return LZMA_OK;
}
