use crate::common::stream_flags_common::{lzma_footer_magic, lzma_header_magic};
use crate::types::*;
fn stream_flags_decode(options: *mut lzma_stream_flags, input: &[u8; 2]) -> bool {
    unsafe {
        if input[0] != 0 || input[1] & 0xf0 != 0 {
            return true;
        }
        (*options).version = 0;
        (*options).check = (input[1] & 0xf) as lzma_check;
        false
    }
}
pub unsafe fn lzma_stream_header_decode(
    options: *mut lzma_stream_flags,
    input: *const u8,
) -> lzma_ret {
    if memcmp(
        input as *const c_void,
        ::core::ptr::addr_of!(lzma_header_magic) as *const c_void,
        core::mem::size_of::<[u8; 6]>(),
    ) != 0
    {
        return LZMA_FORMAT_ERROR;
    }
    let flags_offset = core::mem::size_of::<[u8; 6]>();
    let crc: u32 = lzma_crc32(input.add(flags_offset), LZMA_STREAM_FLAGS_SIZE as size_t, 0) as u32;
    if crc
        != read32le(
            &*input
                .add(flags_offset + LZMA_STREAM_FLAGS_SIZE as usize)
                .cast::<[u8; 4]>(),
        )
    {
        return LZMA_DATA_ERROR;
    }
    if stream_flags_decode(options, &*input.add(flags_offset).cast::<[u8; 2]>()) {
        return LZMA_OPTIONS_ERROR;
    }
    (*options).backward_size = LZMA_VLI_UNKNOWN;
    LZMA_OK
}
pub unsafe fn lzma_stream_footer_decode(
    options: *mut lzma_stream_flags,
    input: *const u8,
) -> lzma_ret {
    if memcmp(
        input
            .offset((core::mem::size_of::<u32>() * 2) as isize)
            .offset(LZMA_STREAM_FLAGS_SIZE as isize) as *const c_void,
        ::core::ptr::addr_of!(lzma_footer_magic) as *const c_void,
        core::mem::size_of::<[u8; 2]>(),
    ) != 0
    {
        return LZMA_FORMAT_ERROR;
    }
    let flags_offset = 2 * core::mem::size_of::<u32>();
    let crc: u32 = lzma_crc32(
        input.add(core::mem::size_of::<u32>()),
        core::mem::size_of::<u32>() + LZMA_STREAM_FLAGS_SIZE as size_t,
        0,
    ) as u32;
    if crc != read32le(&*input.cast::<[u8; 4]>()) {
        return LZMA_DATA_ERROR;
    }
    if stream_flags_decode(options, &*input.add(flags_offset).cast::<[u8; 2]>()) {
        return LZMA_OPTIONS_ERROR;
    }
    (*options).backward_size =
        read32le(&*input.add(core::mem::size_of::<u32>()).cast::<[u8; 4]>()) as lzma_vli;
    (*options).backward_size = ((*options).backward_size + 1) * 4;
    LZMA_OK
}
