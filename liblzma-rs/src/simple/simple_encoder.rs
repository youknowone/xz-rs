use crate::types::*;
use core::ffi::{c_int, c_void};
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
