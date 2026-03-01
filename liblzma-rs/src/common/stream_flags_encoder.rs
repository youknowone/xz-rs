use crate::types::*;
use core::ffi::c_void;
extern "C" {
    fn lzma_crc32(buf: *const u8, size: size_t, crc: u32) -> u32;
    static lzma_header_magic: [u8; 6];
    static lzma_footer_magic: [u8; 2];
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
pub const LZMA_STREAM_FLAGS_SIZE: u32 = 2;
#[inline]
extern "C" fn is_backward_size_valid(options: *const lzma_stream_flags) -> bool {
    return unsafe {
        (*options).backward_size >= LZMA_BACKWARD_SIZE_MIN as lzma_vli
            && (*options).backward_size <= LZMA_BACKWARD_SIZE_MAX as lzma_vli
            && (*options).backward_size & 3 == 0
    };
}
extern "C" fn stream_flags_encode(options: *const lzma_stream_flags, out: *mut u8) -> bool {
    return unsafe {
        if (*options).check > LZMA_CHECK_ID_MAX {
            return true;
        }
        *out.offset(0) = 0;
        *out.offset(1) = (*options).check as u8;
        false
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_header_encode(
    options: *const lzma_stream_flags,
    out: *mut u8,
) -> lzma_ret {
    if (*options).version != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    memcpy(
        out as *mut c_void,
        &raw const lzma_header_magic as *const u8 as *const c_void,
        core::mem::size_of::<[u8; 6]>(),
    );
    if stream_flags_encode(
        options,
        out.offset(core::mem::size_of::<[u8; 6]>() as usize as isize),
    ) {
        return LZMA_PROG_ERROR;
    }
    let crc: u32 = lzma_crc32(
        out.offset(core::mem::size_of::<[u8; 6]>() as usize as isize),
        LZMA_STREAM_FLAGS_SIZE as size_t,
        0,
    ) as u32;
    write32le(
        out.offset(core::mem::size_of::<[u8; 6]>() as usize as isize)
            .offset(LZMA_STREAM_FLAGS_SIZE as isize),
        crc,
    );
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_footer_encode(
    options: *const lzma_stream_flags,
    out: *mut u8,
) -> lzma_ret {
    if (*options).version != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    if !is_backward_size_valid(options) {
        return LZMA_PROG_ERROR;
    }
    write32le(
        out.offset(4),
        (*options).backward_size.wrapping_div(4).wrapping_sub(1) as u32,
    );
    if stream_flags_encode(options, out.offset((2 * 4) as isize)) {
        return LZMA_PROG_ERROR;
    }
    let crc: u32 = lzma_crc32(out.offset(4), (4 + LZMA_STREAM_FLAGS_SIZE) as size_t, 0) as u32;
    write32le(out, crc);
    memcpy(
        out.offset((2 * 4) as isize)
            .offset(LZMA_STREAM_FLAGS_SIZE as isize) as *mut c_void,
        &raw const lzma_footer_magic as *const u8 as *const c_void,
        core::mem::size_of::<[u8; 2]>(),
    );
    return LZMA_OK;
}
