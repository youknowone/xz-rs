use crate::types::*;
use core::ffi::c_void;
extern "C" {
    static lzma_header_magic: [u8; 6];
    static lzma_footer_magic: [u8; 2];
}
extern "C" fn stream_flags_decode(options: *mut lzma_stream_flags, in_0: *const u8) -> bool {
    return unsafe {
        if *in_0 != 0 || *in_0.offset(1) & 0xf0 != 0 {
            return true;
        }
        (*options).version = 0;
        (*options).check = (*in_0.offset(1) & 0xf) as lzma_check;
        false
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_header_decode(
    options: *mut lzma_stream_flags,
    in_0: *const u8,
) -> lzma_ret {
    if memcmp(
        in_0 as *const c_void,
        &raw const lzma_header_magic as *const c_void,
        core::mem::size_of::<[u8; 6]>(),
    ) != 0
    {
        return LZMA_FORMAT_ERROR;
    }
    let crc: u32 = lzma_crc32(
        in_0.offset(core::mem::size_of::<[u8; 6]>() as isize),
        LZMA_STREAM_FLAGS_SIZE as size_t,
        0,
    ) as u32;
    if crc
        != read32le(
            in_0.offset(core::mem::size_of::<[u8; 6]>() as isize)
                .offset(LZMA_STREAM_FLAGS_SIZE as isize),
        )
    {
        return LZMA_DATA_ERROR;
    }
    if stream_flags_decode(
        options,
        in_0.offset(core::mem::size_of::<[u8; 6]>() as isize),
    ) {
        return LZMA_OPTIONS_ERROR;
    }
    (*options).backward_size = LZMA_VLI_UNKNOWN;
    LZMA_OK
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_footer_decode(
    options: *mut lzma_stream_flags,
    in_0: *const u8,
) -> lzma_ret {
    if memcmp(
        in_0.offset((core::mem::size_of::<u32>()).wrapping_mul(2) as isize)
            .offset(LZMA_STREAM_FLAGS_SIZE as isize) as *const c_void,
        &raw const lzma_footer_magic as *const c_void,
        core::mem::size_of::<[u8; 2]>(),
    ) != 0
    {
        return LZMA_FORMAT_ERROR;
    }
    let crc: u32 = lzma_crc32(
        in_0.offset(core::mem::size_of::<u32>() as isize),
        (core::mem::size_of::<u32>()).wrapping_add(LZMA_STREAM_FLAGS_SIZE as size_t),
        0,
    ) as u32;
    if crc != read32le(in_0) {
        return LZMA_DATA_ERROR;
    }
    if stream_flags_decode(
        options,
        in_0.offset((core::mem::size_of::<u32>()).wrapping_mul(2) as isize),
    ) {
        return LZMA_OPTIONS_ERROR;
    }
    (*options).backward_size =
        read32le(in_0.offset(core::mem::size_of::<u32>() as isize)) as lzma_vli;
    (*options).backward_size = (*options).backward_size.wrapping_add(1).wrapping_mul(4);
    LZMA_OK
}
