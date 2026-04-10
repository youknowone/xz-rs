use crate::types::*;
fn armthumb_code_impl(now_pos: u32, is_encoder: bool, buffer: &mut [u8]) -> size_t {
    if buffer.len() < 4 {
        return 0;
    }
    let size = buffer.len() - 4;
    let mut i: size_t = 0;
    while i <= size {
        if buffer[i + 1] & 0xf8 == 0xf0 && buffer[i + 3] & 0xf8 == 0xf8 {
            let mut src: u32 = (buffer[i + 1] as u32 & 7) << 19
                | (buffer[i] as u32) << 11
                | (buffer[i + 3] as u32 & 7) << 8
                | buffer[i + 2] as u32;
            src <<= 1;
            let dest = if is_encoder {
                now_pos
                    .wrapping_add(i as u32)
                    .wrapping_add(4)
                    .wrapping_add(src)
            } else {
                src.wrapping_sub(now_pos.wrapping_add(i as u32).wrapping_add(4))
            };
            let dest = dest >> 1;
            buffer[i + 1] = (0xf0 | dest >> 19 & 0x7) as u8;
            buffer[i] = (dest >> 11) as u8;
            buffer[i + 3] = (0xf8 | dest >> 8 & 0x7) as u8;
            buffer[i + 2] = dest as u8;
            i += 2;
        }
        i += 2;
    }
    i
}
unsafe extern "C" fn armthumb_code(
    _simple: *mut c_void,
    now_pos: u32,
    is_encoder: bool,
    buffer: *mut u8,
    size: size_t,
) -> size_t {
    if size == 0 {
        return 0;
    }
    armthumb_code_impl(
        now_pos,
        is_encoder,
        core::slice::from_raw_parts_mut(buffer, size),
    )
}
unsafe fn armthumb_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    is_encoder: bool,
) -> lzma_ret {
    lzma_simple_coder_init(
        next,
        allocator,
        filters,
        Some(
            armthumb_code
                as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t,
        ),
        0,
        4,
        2,
        is_encoder,
    )
}
pub(crate) unsafe extern "C" fn lzma_simple_armthumb_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    armthumb_coder_init(next, allocator, filters, true)
}
pub(crate) unsafe extern "C" fn lzma_simple_armthumb_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    armthumb_coder_init(next, allocator, filters, false)
}
