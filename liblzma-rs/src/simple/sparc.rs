use crate::types::*;
unsafe extern "C" fn sparc_code(
    _simple: *mut c_void,
    now_pos: u32,
    is_encoder: bool,
    buffer: *mut u8,
    mut size: size_t,
) -> size_t {
    size &= !(3);
    let mut i: size_t = 0;
    i = 0;
    while i < size {
        if *buffer.offset(i as isize) == 0x40 && *buffer.offset((i + 1) as isize) & 0xc0 == 0
            || *buffer.offset(i as isize) == 0x7f && *buffer.offset((i + 1) as isize) & 0xc0 == 0xc0
        {
            let mut src: u32 = (*buffer.offset(i as isize) as u32) << 24
                | (*buffer.offset((i + 1) as isize) as u32) << 16
                | (*buffer.offset((i + 2) as isize) as u32) << 8
                | *buffer.offset((i + 3) as isize) as u32;
            src <<= 2;
            let mut dest: u32 = 0;
            if is_encoder {
                dest = now_pos.wrapping_add(i as u32).wrapping_add(src);
            } else {
                dest = src.wrapping_sub(now_pos.wrapping_add(i as u32));
            }
            dest >>= 2;
            dest =
                0u32.wrapping_sub(dest >> 22 & 1) << 22 & 0x3fffffff | dest & 0x3fffff | 0x40000000;
            *buffer.offset(i as isize) = (dest >> 24) as u8;
            *buffer.offset((i + 1) as isize) = (dest >> 16) as u8;
            *buffer.offset((i + 2) as isize) = (dest >> 8) as u8;
            *buffer.offset((i + 3) as isize) = dest as u8;
        }
        i += 4;
    }
    i
}
extern "C" fn sparc_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    is_encoder: bool,
) -> lzma_ret {
    unsafe {
        lzma_simple_coder_init(
            next,
            allocator,
            filters,
            Some(
                sparc_code
                    as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t,
            ),
            0,
            4,
            4,
            is_encoder,
        )
    }
}
pub unsafe extern "C" fn lzma_simple_sparc_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    sparc_coder_init(next, allocator, filters, true)
}
pub unsafe extern "C" fn lzma_simple_sparc_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    sparc_coder_init(next, allocator, filters, false)
}
