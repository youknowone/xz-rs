use crate::types::*;
use core::ffi::{c_int, c_void};
extern "C" {
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_delta_coder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
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
pub const LZMA_DELTA_TYPE_BYTE: lzma_delta_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_delta {
    pub type_0: lzma_delta_type,
    pub dist: u32,
    pub reserved_int1: u32,
    pub reserved_int2: u32,
    pub reserved_int3: u32,
    pub reserved_int4: u32,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
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
pub struct lzma_delta_coder {
    pub next: lzma_next_coder,
    pub distance: size_t,
    pub pos: u8,
    pub history: [u8; LZMA_DELTA_DIST_MAX as usize],
}
pub const LZMA_DELTA_DIST_MAX: c_int = 256;
unsafe extern "C" fn decode_buffer(coder: *mut lzma_delta_coder, buffer: *mut u8, size: size_t) {
    let distance: size_t = (*coder).distance;
    let mut i: size_t = 0;
    while i < size {
        let ref mut fresh0 = *buffer.offset(i as isize);
        *fresh0 = (*fresh0 as c_int
            + (*coder).history
                [(distance.wrapping_add((*coder).pos as size_t) & 0xff as size_t) as usize]
                as c_int) as u8;
        let fresh1 = (*coder).pos;
        (*coder).pos = (*coder).pos.wrapping_sub(1);
        (*coder).history[(fresh1 as c_int & 0xff as c_int) as usize] = *buffer.offset(i as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn delta_decode(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
    action: lzma_action,
) -> lzma_ret {
    let coder: *mut lzma_delta_coder = coder_ptr as *mut lzma_delta_coder;
    let out_start: size_t = *out_pos;
    let ret: lzma_ret = (*coder).next.code.expect("non-null function pointer")(
        (*coder).next.coder,
        allocator,
        in_0,
        in_pos,
        in_size,
        out,
        out_pos,
        out_size,
        action,
    ) as lzma_ret;
    let size: size_t = (*out_pos).wrapping_sub(out_start);
    if size > 0 {
        decode_buffer(coder, out.offset(out_start as isize), size);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_delta_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    (*next).code = Some(
        delta_decode
            as unsafe extern "C" fn(
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
    ) as lzma_code_function;
    return lzma_delta_coder_init(next, allocator, filters);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_delta_props_decode(
    options: *mut *mut c_void,
    allocator: *const lzma_allocator,
    props: *const u8,
    props_size: size_t,
) -> lzma_ret {
    if props_size != 1 {
        return LZMA_OPTIONS_ERROR;
    }
    let opt: *mut lzma_options_delta = lzma_alloc(
        core::mem::size_of::<lzma_options_delta>() as size_t,
        allocator,
    ) as *mut lzma_options_delta;
    if opt.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*opt).type_0 = LZMA_DELTA_TYPE_BYTE;
    (*opt).dist = u32::from(*props.offset(0)).wrapping_add(1);
    *options = opt as *mut c_void;
    return LZMA_OK;
}
