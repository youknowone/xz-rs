use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
extern "C" {
    fn lzma_next_filter_update(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        reversed_filters: *const lzma_filter,
    ) -> lzma_ret;
    fn lzma_delta_coder_memusage(options: *const c_void) -> u64;
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
pub const LZMA_DELTA_DIST_MAX: c_int = 256 as c_int;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const LZMA_DELTA_DIST_MIN: c_int = 1 as c_int;
unsafe extern "C" fn copy_and_encode(
    mut coder: *mut lzma_delta_coder,
    mut in_0: *const u8,
    mut out: *mut u8,
    mut size: size_t,
) {
    let distance: size_t = (*coder).distance;
    let mut i: size_t = 0 as size_t;
    while i < size {
        let tmp: u8 = (*coder).history
            [(distance.wrapping_add((*coder).pos as size_t) & 0xff as size_t) as usize];
        let fresh2 = (*coder).pos;
        (*coder).pos = (*coder).pos.wrapping_sub(1);
        (*coder).history[(fresh2 as c_int & 0xff as c_int) as usize] = *in_0.offset(i as isize);
        *out.offset(i as isize) = (*in_0.offset(i as isize) as c_int - tmp as c_int) as u8;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn encode_in_place(
    mut coder: *mut lzma_delta_coder,
    mut buffer: *mut u8,
    mut size: size_t,
) {
    let distance: size_t = (*coder).distance;
    let mut i: size_t = 0 as size_t;
    while i < size {
        let tmp: u8 = (*coder).history
            [(distance.wrapping_add((*coder).pos as size_t) & 0xff as size_t) as usize];
        let fresh0 = (*coder).pos;
        (*coder).pos = (*coder).pos.wrapping_sub(1);
        (*coder).history[(fresh0 as c_int & 0xff as c_int) as usize] = *buffer.offset(i as isize);
        let ref mut fresh1 = *buffer.offset(i as isize);
        *fresh1 = (*fresh1 as c_int - tmp as c_int) as u8;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn delta_encode(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut action: lzma_action,
) -> lzma_ret {
    let mut coder: *mut lzma_delta_coder = coder_ptr as *mut lzma_delta_coder;
    let mut ret: lzma_ret = LZMA_OK;
    if (*coder).next.code.is_none() {
        let in_avail: size_t = in_size.wrapping_sub(*in_pos);
        let out_avail: size_t = out_size.wrapping_sub(*out_pos);
        let size: size_t = if in_avail < out_avail {
            in_avail
        } else {
            out_avail
        };
        if size > 0 as size_t {
            copy_and_encode(
                coder,
                in_0.offset(*in_pos as isize),
                out.offset(*out_pos as isize),
                size,
            );
        }
        *in_pos = (*in_pos).wrapping_add(size);
        *out_pos = (*out_pos).wrapping_add(size);
        ret = (if action != LZMA_RUN && *in_pos == in_size {
            LZMA_STREAM_END as c_int
        } else {
            LZMA_OK as c_int
        }) as lzma_ret;
    } else {
        let out_start: size_t = *out_pos;
        ret = (*coder).next.code.expect("non-null function pointer")(
            (*coder).next.coder,
            allocator,
            in_0,
            in_pos,
            in_size,
            out,
            out_pos,
            out_size,
            action,
        );
        let size_0: size_t = (*out_pos).wrapping_sub(out_start);
        if size_0 > 0 as size_t {
            encode_in_place(coder, out.offset(out_start as isize), size_0);
        }
    }
    return ret;
}
unsafe extern "C" fn delta_encoder_update(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
    mut filters_null: *const lzma_filter,
    mut reversed_filters: *const lzma_filter,
) -> lzma_ret {
    let mut coder: *mut lzma_delta_coder = coder_ptr as *mut lzma_delta_coder;
    return lzma_next_filter_update(
        &raw mut (*coder).next,
        allocator,
        reversed_filters.offset(1),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_delta_encoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
) -> lzma_ret {
    (*next).code = Some(
        delta_encode
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
    (*next).update = Some(
        delta_encoder_update
            as unsafe extern "C" fn(
                *mut c_void,
                *const lzma_allocator,
                *const lzma_filter,
                *const lzma_filter,
            ) -> lzma_ret,
    )
        as Option<
            unsafe extern "C" fn(
                *mut c_void,
                *const lzma_allocator,
                *const lzma_filter,
                *const lzma_filter,
            ) -> lzma_ret,
        >;
    return lzma_delta_coder_init(next, allocator, filters);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_delta_props_encode(
    mut options: *const c_void,
    mut out: *mut u8,
) -> lzma_ret {
    if lzma_delta_coder_memusage(options) == UINT64_MAX as u64 {
        return LZMA_PROG_ERROR;
    }
    let mut opt: *const lzma_options_delta = options as *const lzma_options_delta;
    *out.offset(0) = (*opt).dist.wrapping_sub(LZMA_DELTA_DIST_MIN as u32) as u8;
    return LZMA_OK;
}
