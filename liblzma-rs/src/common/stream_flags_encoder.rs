use crate::types::*;
extern "C" {
    static lzma_header_magic: [u8; 6];
    static lzma_footer_magic: [u8; 2];
}
extern "C" fn stream_flags_encode(options: *const lzma_stream_flags, out: *mut u8) -> bool {
    return unsafe {
        if (*options).check > LZMA_CHECK_ID_MAX {
            return true;
        }
        *out = 0;
        *out.offset(1) = (*options).check as u8;
        false
    };
}
pub unsafe extern "C" fn lzma_stream_header_encode(
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
    if stream_flags_encode(
        options,
        out.offset(core::mem::size_of::<[u8; 6]>() as isize),
    ) {
        return LZMA_PROG_ERROR;
    }
    let crc: u32 = lzma_crc32(
        out.offset(core::mem::size_of::<[u8; 6]>() as isize),
        LZMA_STREAM_FLAGS_SIZE as size_t,
        0,
    ) as u32;
    write32le(
        out.offset(core::mem::size_of::<[u8; 6]>() as isize)
            .offset(LZMA_STREAM_FLAGS_SIZE as isize),
        crc,
    );
    LZMA_OK
}
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
    core::ptr::copy_nonoverlapping(
        ::core::ptr::addr_of!(lzma_footer_magic) as *const u8,
        out.offset((2 * 4) as isize)
            .offset(LZMA_STREAM_FLAGS_SIZE as isize) as *mut u8,
        core::mem::size_of::<[u8; 2]>(),
    );
    LZMA_OK
}
