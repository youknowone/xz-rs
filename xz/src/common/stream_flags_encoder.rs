use crate::common::stream_flags_common::{lzma_footer_magic, lzma_header_magic};
use crate::types::*;
fn stream_flags_encode(options: *const lzma_stream_flags, out: &mut [u8; 2]) -> bool {
    unsafe {
        if (*options).check > LZMA_CHECK_ID_MAX {
            return true;
        }
        out[0] = 0;
        out[1] = (*options).check as u8;
        false
    }
}
pub unsafe fn lzma_stream_header_encode(
    options: *const lzma_stream_flags,
    out: *mut u8,
) -> lzma_ret {
    if (*options).version != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    core::ptr::copy_nonoverlapping(
        ::core::ptr::addr_of!(lzma_header_magic) as *const u8,
        out as *mut u8,
        core::mem::size_of::<[u8; 6]>(),
    );
    let flags_offset = core::mem::size_of::<[u8; 6]>();
    if stream_flags_encode(options, &mut *out.add(flags_offset).cast::<[u8; 2]>()) {
        return LZMA_PROG_ERROR;
    }
    let crc: u32 = lzma_crc32(out.add(flags_offset), LZMA_STREAM_FLAGS_SIZE as size_t, 0) as u32;
    write32le(
        &mut *out
            .add(flags_offset + LZMA_STREAM_FLAGS_SIZE as usize)
            .cast::<[u8; 4]>(),
        crc,
    );
    LZMA_OK
}
pub unsafe fn lzma_stream_footer_encode(
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
        &mut *out.add(4).cast::<[u8; 4]>(),
        (*options).backward_size.wrapping_div(4).wrapping_sub(1) as u32,
    );
    let flags_offset = 2 * core::mem::size_of::<u32>();
    if stream_flags_encode(options, &mut *out.add(flags_offset).cast::<[u8; 2]>()) {
        return LZMA_PROG_ERROR;
    }
    let crc: u32 = lzma_crc32(out.add(4), (4 + LZMA_STREAM_FLAGS_SIZE) as size_t, 0) as u32;
    write32le(&mut *out.cast::<[u8; 4]>(), crc);
    core::ptr::copy_nonoverlapping(
        ::core::ptr::addr_of!(lzma_footer_magic) as *const u8,
        out.add(flags_offset + LZMA_STREAM_FLAGS_SIZE as usize),
        core::mem::size_of::<[u8; 2]>(),
    );
    LZMA_OK
}
