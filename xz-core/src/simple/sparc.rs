use crate::types::*;
fn sparc_code_impl(now_pos: u32, is_encoder: bool, buffer: &mut [u8]) -> size_t {
    let size = buffer.len() & !3;
    let ptr = buffer.as_mut_ptr();
    let mut i: size_t = 0;
    while i < size {
        let cur = unsafe { ptr.add(i) };
        let b0 = unsafe { *cur };
        let b1 = unsafe { *cur.add(1) };
        if b0 == 0x40 && b1 & 0xc0 == 0 || b0 == 0x7f && b1 & 0xc0 == 0xc0 {
            let mut src: u32 = unsafe {
                (b0 as u32) << 24
                    | (b1 as u32) << 16
                    | (*cur.add(2) as u32) << 8
                    | *cur.add(3) as u32
            };
            src <<= 2;
            let dest = if is_encoder {
                now_pos.wrapping_add(i as u32).wrapping_add(src)
            } else {
                src.wrapping_sub(now_pos.wrapping_add(i as u32))
            };
            let mut dest = dest >> 2;
            dest =
                0u32.wrapping_sub(dest >> 22 & 1) << 22 & 0x3fffffff | dest & 0x3fffff | 0x40000000;
            unsafe {
                *cur = (dest >> 24) as u8;
                *cur.add(1) = (dest >> 16) as u8;
                *cur.add(2) = (dest >> 8) as u8;
                *cur.add(3) = dest as u8;
            }
        }
        i += 4;
    }
    i
}
unsafe fn sparc_code(
    _simple: *mut c_void,
    now_pos: u32,
    is_encoder: bool,
    buffer: *mut u8,
    size: size_t,
) -> size_t {
    if size == 0 {
        return 0;
    }
    sparc_code_impl(
        now_pos,
        is_encoder,
        core::slice::from_raw_parts_mut(buffer, size),
    )
}
unsafe fn sparc_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    is_encoder: bool,
) -> lzma_ret {
    lzma_simple_coder_init(
        next,
        allocator,
        filters,
        sparc_code as lzma_simple_filter_function,
        0,
        4,
        4,
        is_encoder,
    )
}
pub(crate) unsafe fn lzma_simple_sparc_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    sparc_coder_init(next, allocator, filters, true)
}
pub(crate) unsafe fn lzma_simple_sparc_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    sparc_coder_init(next, allocator, filters, false)
}
