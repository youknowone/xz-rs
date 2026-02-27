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
pub const LZMA_RET_INTERNAL8: lzma_ret = 108;
pub const LZMA_RET_INTERNAL7: lzma_ret = 107;
pub const LZMA_RET_INTERNAL6: lzma_ret = 106;
pub const LZMA_RET_INTERNAL5: lzma_ret = 105;
pub const LZMA_RET_INTERNAL4: lzma_ret = 104;
pub const LZMA_RET_INTERNAL3: lzma_ret = 103;
pub const LZMA_RET_INTERNAL2: lzma_ret = 102;
pub const LZMA_RET_INTERNAL1: lzma_ret = 101;
pub const LZMA_SEEK_NEEDED: lzma_ret = 12;
pub const LZMA_PROG_ERROR: lzma_ret = 11;
pub const LZMA_BUF_ERROR: lzma_ret = 10;
pub const LZMA_DATA_ERROR: lzma_ret = 9;
pub const LZMA_OPTIONS_ERROR: lzma_ret = 8;
pub const LZMA_FORMAT_ERROR: lzma_ret = 7;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret = 6;
pub const LZMA_MEM_ERROR: lzma_ret = 5;
pub const LZMA_GET_CHECK: lzma_ret = 4;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret = 3;
pub const LZMA_NO_CHECK: lzma_ret = 2;
pub const LZMA_STREAM_END: lzma_ret = 1;
pub const LZMA_OK: lzma_ret = 0;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, size_t, size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    pub opaque: *mut c_void,
}
pub type lzma_next_coder = lzma_next_coder_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_next_coder_s {
    pub coder: *mut c_void,
    pub id: lzma_vli,
    pub init: uintptr_t,
    pub code: lzma_code_function,
    pub end: lzma_end_function,
    pub get_progress: Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64) -> ()>,
    pub get_check: Option<unsafe extern "C" fn(*const c_void) -> lzma_check>,
    pub memconfig: Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret>,
    pub update: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const lzma_allocator,
            *const lzma_filter,
            *const lzma_filter,
        ) -> lzma_ret,
    >,
    pub set_out_limit: Option<unsafe extern "C" fn(*mut c_void, *mut u64, u64) -> lzma_ret>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut c_void,
}
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
pub type lzma_end_function = Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>;
pub type lzma_code_function = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const lzma_allocator,
        *const u8,
        *mut size_t,
        size_t,
        *mut u8,
        *mut size_t,
        size_t,
        lzma_action,
    ) -> lzma_ret,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_simple_x86 {
    pub prev_mask: u32,
    pub prev_pos: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_info_s {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub options: *mut c_void,
}
pub type lzma_init_function = Option<
    unsafe extern "C" fn(
        *mut lzma_next_coder,
        *const lzma_allocator,
        *const lzma_filter_info,
    ) -> lzma_ret,
>;
pub type lzma_filter_info = lzma_filter_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_simple_coder {
    pub next: lzma_next_coder,
    pub end_was_reached: bool,
    pub is_encoder: bool,
    pub filter: Option<unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t>,
    pub simple: *mut c_void,
    pub now_pos: u32,
    pub allocated: size_t,
    pub pos: size_t,
    pub filtered: size_t,
    pub size: size_t,
    pub buffer: [u8; 0],
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
                x86_code as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t
            ),
            core::mem::size_of::<lzma_simple_x86>() as size_t,
            5,
            1,
            is_encoder,
        ) as lzma_ret;
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
