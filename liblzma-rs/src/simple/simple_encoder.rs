use crate::types::*;
use core::ffi::{c_int, c_void};
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
pub struct lzma_options_bcj {
    pub start_offset: u32,
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
pub unsafe extern "C" fn lzma_simple_props_size(
    size: *mut u32,
    options: *const c_void,
) -> lzma_ret {
    let opt: *const lzma_options_bcj = options as *const lzma_options_bcj;
    *size = (if opt.is_null() || (*opt).start_offset == 0 {
        0 as c_int
    } else {
        4 as c_int
    }) as u32;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_props_encode(
    options: *const c_void,
    out: *mut u8,
) -> lzma_ret {
    let opt: *const lzma_options_bcj = options as *const lzma_options_bcj;
    if opt.is_null() || (*opt).start_offset == 0 {
        return LZMA_OK;
    }
    write32le(out, (*opt).start_offset);
    return LZMA_OK;
}
