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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_simple_x86 {
    pub prev_mask: u32,
    pub prev_pos: u32,
}
unsafe extern "C" fn x86_code(
    simple_ptr: *mut c_void,
    now_pos: u32,
    is_encoder: bool,
    buffer: *mut u8,
    size: size_t,
) -> size_t {
    static mut MASK_TO_BIT_NUMBER: [u32; 5] = [0, 1, 2, 2, 3];
    let simple: *mut lzma_simple_x86 = simple_ptr as *mut lzma_simple_x86;
    let mut prev_mask: u32 = (*simple).prev_mask;
    let mut prev_pos: u32 = (*simple).prev_pos;
    if size < 5 {
        return 0;
    }
    if now_pos.wrapping_sub(prev_pos) > 5 {
        prev_pos = now_pos.wrapping_sub(5);
    }
    let limit: size_t = size.wrapping_sub(5);
    let mut buffer_pos: size_t = 0;
    while buffer_pos <= limit {
        let mut b: u8 = *buffer.offset(buffer_pos as isize);
        if b as c_int != 0xe8 as c_int && b as c_int != 0xe9 as c_int {
            buffer_pos = buffer_pos.wrapping_add(1);
        } else {
            let offset: u32 = now_pos
                .wrapping_add(buffer_pos as u32)
                .wrapping_sub(prev_pos);
            prev_pos = now_pos.wrapping_add(buffer_pos as u32);
            if offset > 5 {
                prev_mask = 0;
            } else {
                let mut i: u32 = 0;
                while i < offset {
                    prev_mask &= 0x77 as u32;
                    prev_mask <<= 1 as c_int;
                    i = i.wrapping_add(1);
                }
            }
            b = *buffer.offset(buffer_pos.wrapping_add(4) as isize);
            if (b as c_int == 0 as c_int || b as c_int == 0xff as c_int)
                && prev_mask >> 1 <= 4
                && prev_mask >> 1 != 3
            {
                let mut src: u32 = (b as u32) << 24
                    | (*buffer.offset(buffer_pos.wrapping_add(3) as isize) as u32) << 16
                    | (*buffer.offset(buffer_pos.wrapping_add(2) as isize) as u32) << 8
                    | *buffer.offset(buffer_pos.wrapping_add(1) as isize) as u32;
                let mut dest: u32 = 0;
                loop {
                    if is_encoder {
                        dest = src
                            .wrapping_add(now_pos.wrapping_add(buffer_pos as u32).wrapping_add(5));
                    } else {
                        dest = src
                            .wrapping_sub(now_pos.wrapping_add(buffer_pos as u32).wrapping_add(5));
                    }
                    if prev_mask == 0 {
                        break;
                    }
                    let i_0: u32 = MASK_TO_BIT_NUMBER[(prev_mask >> 1) as usize];
                    b = (dest >> (24u32).wrapping_sub(i_0.wrapping_mul(8))) as u8;
                    if !(b as c_int == 0 as c_int || b as c_int == 0xff as c_int) {
                        break;
                    }
                    src =
                        dest ^ (1u32 << (32u32).wrapping_sub(i_0.wrapping_mul(8))).wrapping_sub(1);
                }
                *buffer.offset(buffer_pos.wrapping_add(4) as isize) =
                    !(dest >> 24 & 1).wrapping_sub(1) as u8;
                *buffer.offset(buffer_pos.wrapping_add(3) as isize) = (dest >> 16) as u8;
                *buffer.offset(buffer_pos.wrapping_add(2) as isize) = (dest >> 8) as u8;
                *buffer.offset(buffer_pos.wrapping_add(1) as isize) = dest as u8;
                buffer_pos = buffer_pos.wrapping_add(5);
                prev_mask = 0;
            } else {
                buffer_pos = buffer_pos.wrapping_add(1);
                prev_mask |= 1;
                if b as c_int == 0 as c_int || b as c_int == 0xff as c_int {
                    prev_mask |= 0x10 as u32;
                }
            }
        }
    }
    (*simple).prev_mask = prev_mask;
    (*simple).prev_pos = prev_pos;
    return buffer_pos;
}
extern "C" fn x86_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    is_encoder: bool,
) -> lzma_ret {
    return unsafe {
        let ret: lzma_ret = lzma_simple_coder_init(
            next,
            allocator,
            filters,
            Some(
                x86_code as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t,
            ),
            core::mem::size_of::<lzma_simple_x86>(),
            5,
            1,
            is_encoder,
        );
        if ret == LZMA_OK {
            let coder: *mut lzma_simple_coder = (*next).coder as *mut lzma_simple_coder;
            let simple: *mut lzma_simple_x86 = (*coder).simple as *mut lzma_simple_x86;
            (*simple).prev_mask = 0;
            (*simple).prev_pos = -(5 as c_int) as u32;
        }
        ret
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_x86_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    return x86_coder_init(next, allocator, filters, true);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_x86_encode(
    start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    let mut simple: lzma_simple_x86 = lzma_simple_x86 {
        prev_mask: 0,
        prev_pos: -(5 as c_int) as u32,
    };
    return x86_code(
        &raw mut simple as *mut c_void,
        start_offset,
        true,
        buf,
        size,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_x86_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    return x86_coder_init(next, allocator, filters, false);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_x86_decode(
    start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    let mut simple: lzma_simple_x86 = lzma_simple_x86 {
        prev_mask: 0,
        prev_pos: -(5 as c_int) as u32,
    };
    return x86_code(
        &raw mut simple as *mut c_void,
        start_offset,
        false,
        buf,
        size,
    );
}
