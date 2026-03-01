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
        if *buffer.offset(i as isize) as c_int >> 2 == 0x12 as c_int
            && *buffer.offset(i.wrapping_add(3) as isize) as c_int & 3 as c_int == 1 as c_int
        {
            let src: u32 = (*buffer.offset(i.wrapping_add(0) as isize) as u32 & 3) << 24
                | (*buffer.offset(i.wrapping_add(1) as isize) as u32) << 16
                | (*buffer.offset(i.wrapping_add(2) as isize) as u32) << 8
                | *buffer.offset(i.wrapping_add(3) as isize) as u32 & !(3);
            let mut dest: u32 = 0;
            if is_encoder {
                dest = now_pos.wrapping_add(i as u32).wrapping_add(src);
            } else {
                dest = src.wrapping_sub(now_pos.wrapping_add(i as u32));
            }
            *buffer.offset(i.wrapping_add(0) as isize) =
                (0x48 as u32 | dest >> 24 & 0x3 as u32) as u8;
            *buffer.offset(i.wrapping_add(1) as isize) = (dest >> 16) as u8;
            *buffer.offset(i.wrapping_add(2) as isize) = (dest >> 8) as u8;
            let ref mut fresh0 = *buffer.offset(i.wrapping_add(3) as isize);
            *fresh0 = (*fresh0 as c_int & 0x3 as c_int) as u8;
            let ref mut fresh1 = *buffer.offset(i.wrapping_add(3) as isize);
            *fresh1 = (*fresh1 as u32 | dest) as u8;
        }
        i = i.wrapping_add(4);
    }
    return i;
}
extern "C" fn powerpc_coder_init(
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
                powerpc_code
                    as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t,
            ),
            0,
            4,
            4,
            is_encoder,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_powerpc_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    return powerpc_coder_init(next, allocator, filters, true);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_powerpc_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    return powerpc_coder_init(next, allocator, filters, false);
}
