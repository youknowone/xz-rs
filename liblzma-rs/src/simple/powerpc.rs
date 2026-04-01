use crate::types::*;
unsafe extern "C" fn powerpc_code(
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
        if *buffer.offset(i as isize) >> 2 == 0x12 && *buffer.offset((i + 3) as isize) & 3 == 1 {
            let src: u32 = (*buffer.offset(i as isize) as u32 & 3) << 24
                | (*buffer.offset((i + 1) as isize) as u32) << 16
                | (*buffer.offset((i + 2) as isize) as u32) << 8
                | *buffer.offset((i + 3) as isize) as u32 & !(3);
            let mut dest: u32 = 0;
            if is_encoder {
                dest = now_pos.wrapping_add(i as u32).wrapping_add(src);
            } else {
                dest = src.wrapping_sub(now_pos.wrapping_add(i as u32));
            }
            *buffer.offset(i as isize) = (0x48 | dest >> 24 & 0x3) as u8;
            *buffer.offset((i + 1) as isize) = (dest >> 16) as u8;
            *buffer.offset((i + 2) as isize) = (dest >> 8) as u8;
            *buffer.offset((i + 3) as isize) &= 0x3;
            *buffer.offset((i + 3) as isize) =
                (*buffer.offset((i + 3) as isize) as u32 | dest) as u8;
        }
        i += 4;
    }
    i
}
extern "C" fn powerpc_coder_init(
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
                powerpc_code
                    as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t,
            ),
            0,
            4,
            4,
            is_encoder,
        )
    }
}
pub unsafe extern "C" fn lzma_simple_powerpc_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    powerpc_coder_init(next, allocator, filters, true)
}
pub unsafe extern "C" fn lzma_simple_powerpc_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    powerpc_coder_init(next, allocator, filters, false)
}
