use crate::types::*;
fn arm_code_impl(now_pos: u32, is_encoder: bool, buffer: &mut [u8]) -> size_t {
    let size = buffer.len() & !3;
    let mut i: size_t = 0;
    while i < size {
        if buffer[i + 3] == 0xeb {
            let mut src: u32 =
                (buffer[i + 2] as u32) << 16 | (buffer[i + 1] as u32) << 8 | buffer[i] as u32;
            src <<= 2;
            let dest = if is_encoder {
                now_pos
                    .wrapping_add(i as u32)
                    .wrapping_add(8)
                    .wrapping_add(src)
            } else {
                src.wrapping_sub(now_pos.wrapping_add(i as u32).wrapping_add(8))
            };
            let dest = dest >> 2;
            buffer[i + 2] = (dest >> 16) as u8;
            buffer[i + 1] = (dest >> 8) as u8;
            buffer[i] = dest as u8;
        }
        i += 4;
    }
    i
}
unsafe extern "C" fn arm_code(
    _simple: *mut c_void,
    now_pos: u32,
    is_encoder: bool,
    buffer: *mut u8,
    size: size_t,
) -> size_t {
    if size == 0 {
        return 0;
    }
    arm_code_impl(
        now_pos,
        is_encoder,
        core::slice::from_raw_parts_mut(buffer, size),
    )
}
unsafe fn arm_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    is_encoder: bool,
) -> lzma_ret {
    lzma_simple_coder_init(
        next,
        allocator,
        filters,
        Some(arm_code as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t),
        0,
        4,
        4,
        is_encoder,
    )
}
pub(crate) unsafe extern "C" fn lzma_simple_arm_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    arm_coder_init(next, allocator, filters, true)
}
pub(crate) unsafe extern "C" fn lzma_simple_arm_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    arm_coder_init(next, allocator, filters, false)
}
