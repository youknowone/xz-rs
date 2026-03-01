use crate::types::*;
use core::ffi::c_void;
unsafe extern "C" fn arm_code(
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
        if *buffer.offset(i.wrapping_add(3) as isize) == 0xeb {
            let mut src: u32 = (*buffer.offset(i.wrapping_add(2) as isize) as u32) << 16
                | (*buffer.offset(i.wrapping_add(1) as isize) as u32) << 8
                | *buffer.offset(i as isize) as u32;
            src <<= 2;
            let mut dest: u32 = 0;
            if is_encoder {
                dest = now_pos
                    .wrapping_add(i as u32)
                    .wrapping_add(8)
                    .wrapping_add(src);
            } else {
                dest = src.wrapping_sub(now_pos.wrapping_add(i as u32).wrapping_add(8));
            }
            dest >>= 2;
            *buffer.offset(i.wrapping_add(2) as isize) = (dest >> 16) as u8;
            *buffer.offset(i.wrapping_add(1) as isize) = (dest >> 8) as u8;
            *buffer.offset(i as isize) = dest as u8;
        }
        i = i.wrapping_add(4);
    }
    i
}
extern "C" fn arm_coder_init(
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
                arm_code as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t,
            ),
            0,
            4,
            4,
            is_encoder,
        )
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_arm_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    arm_coder_init(next, allocator, filters, true)
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_arm_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    arm_coder_init(next, allocator, filters, false)
}
