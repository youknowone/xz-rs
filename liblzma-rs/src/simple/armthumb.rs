use crate::types::*;
use core::ffi::{c_int, c_void};
extern "C" {
    fn lzma_simple_coder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
        filter: Option<unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t>,
        simple_size: size_t,
        unfiltered_max: size_t,
        alignment: u32,
        is_encoder: bool,
    ) -> lzma_ret;
}
unsafe extern "C" fn armthumb_code(
    _simple: *mut c_void,
    now_pos: u32,
    is_encoder: bool,
    buffer: *mut u8,
    mut size: size_t,
) -> size_t {
    if size < 4 {
        return 0;
    }
    size = size.wrapping_sub(4);
    let mut i: size_t = 0;
    i = 0;
    while i <= size {
        if *buffer.offset(i.wrapping_add(1) as isize) as c_int & 0xf8 as c_int == 0xf0 as c_int
            && *buffer.offset(i.wrapping_add(3) as isize) as c_int & 0xf8 as c_int == 0xf8 as c_int
        {
            let mut src: u32 = (*buffer.offset(i.wrapping_add(1) as isize) as u32 & 7) << 19
                | (*buffer.offset(i.wrapping_add(0) as isize) as u32) << 11
                | (*buffer.offset(i.wrapping_add(3) as isize) as u32 & 7) << 8
                | *buffer.offset(i.wrapping_add(2) as isize) as u32;
            src <<= 1 as c_int;
            let mut dest: u32 = 0;
            if is_encoder {
                dest = now_pos
                    .wrapping_add(i as u32)
                    .wrapping_add(4)
                    .wrapping_add(src);
            } else {
                dest = src.wrapping_sub(now_pos.wrapping_add(i as u32).wrapping_add(4));
            }
            dest >>= 1 as c_int;
            *buffer.offset(i.wrapping_add(1) as isize) =
                (0xf0 as u32 | dest >> 19 & 0x7 as u32) as u8;
            *buffer.offset(i.wrapping_add(0) as isize) = (dest >> 11) as u8;
            *buffer.offset(i.wrapping_add(3) as isize) =
                (0xf8 as u32 | dest >> 8 & 0x7 as u32) as u8;
            *buffer.offset(i.wrapping_add(2) as isize) = dest as u8;
            i = i.wrapping_add(2);
        }
        i = i.wrapping_add(2);
    }
    return i;
}
extern "C" fn armthumb_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    is_encoder: bool,
) -> lzma_ret {
    return unsafe {
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
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_armthumb_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    return armthumb_coder_init(next, allocator, filters, true);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_armthumb_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    return armthumb_coder_init(next, allocator, filters, false);
}
