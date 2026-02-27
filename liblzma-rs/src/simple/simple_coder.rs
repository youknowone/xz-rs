use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
extern "C" {
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn memmove(__dst: *mut c_void, __src: *const c_void, __len: size_t) -> *mut c_void;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_next_filter_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_next_filter_update(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        reversed_filters: *const lzma_filter,
    ) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_bufcpy(
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> size_t;
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
pub struct lzma_options_bcj {
    pub start_offset: u32,
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
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
unsafe extern "C" fn copy_or_code(
    mut coder: *mut lzma_simple_coder,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut action: lzma_action,
) -> lzma_ret {
    if (*coder).next.code.is_none() {
        lzma_bufcpy(in_0, in_pos, in_size, out, out_pos, out_size);
        if (*coder).is_encoder as c_int != 0 && action == LZMA_FINISH && *in_pos == in_size {
            (*coder).end_was_reached = true_0 != 0;
        }
    } else {
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
        if ret == LZMA_STREAM_END {
            (*coder).end_was_reached = true_0 != 0;
        } else if ret != LZMA_OK {
            return ret;
        }
    }
    return LZMA_OK;
}
unsafe extern "C" fn call_filter(
    mut coder: *mut lzma_simple_coder,
    mut buffer: *mut u8,
    mut size: size_t,
) -> size_t {
    let filtered: size_t = (*coder).filter.expect("non-null function pointer")(
        (*coder).simple,
        (*coder).now_pos,
        (*coder).is_encoder,
        buffer,
        size,
    ) as size_t;
    (*coder).now_pos = ((*coder).now_pos as size_t).wrapping_add(filtered) as u32;
    return filtered;
}
unsafe extern "C" fn simple_code(
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
    let mut coder: *mut lzma_simple_coder = coder_ptr as *mut lzma_simple_coder;
    if action == LZMA_SYNC_FLUSH {
        return LZMA_OPTIONS_ERROR;
    }
    if (*coder).pos < (*coder).filtered {
        lzma_bufcpy(
            &raw mut (*coder).buffer as *mut u8,
            &raw mut (*coder).pos,
            (*coder).filtered,
            out,
            out_pos,
            out_size,
        );
        if (*coder).pos < (*coder).filtered {
            return LZMA_OK;
        }
        if (*coder).end_was_reached {
            return LZMA_STREAM_END;
        }
    }
    (*coder).filtered = 0 as size_t;
    let out_avail: size_t = out_size.wrapping_sub(*out_pos);
    let buf_avail: size_t = (*coder).size.wrapping_sub((*coder).pos);
    if out_avail > buf_avail || buf_avail == 0 as size_t {
        let out_start: size_t = *out_pos;
        if buf_avail > 0 as size_t {
            memcpy(
                out.offset(*out_pos as isize) as *mut c_void,
                (&raw mut (*coder).buffer as *mut u8).offset((*coder).pos as isize)
                    as *const c_void,
                buf_avail,
            );
        }
        *out_pos = (*out_pos).wrapping_add(buf_avail);
        let ret: lzma_ret = copy_or_code(
            coder, allocator, in_0, in_pos, in_size, out, out_pos, out_size, action,
        ) as lzma_ret;
        if ret != LZMA_OK {
            return ret;
        }
        let size: size_t = (*out_pos).wrapping_sub(out_start);
        let filtered: size_t = if size == 0 as size_t {
            0 as size_t
        } else {
            call_filter(coder, out.offset(out_start as isize), size) as size_t
        };
        let unfiltered: size_t = size.wrapping_sub(filtered);
        (*coder).pos = 0 as size_t;
        (*coder).size = unfiltered;
        if (*coder).end_was_reached {
            (*coder).size = 0 as size_t;
        } else if unfiltered > 0 as size_t {
            *out_pos = (*out_pos).wrapping_sub(unfiltered);
            memcpy(
                &raw mut (*coder).buffer as *mut u8 as *mut c_void,
                out.offset(*out_pos as isize) as *const c_void,
                unfiltered,
            );
        }
    } else if (*coder).pos > 0 as size_t {
        memmove(
            &raw mut (*coder).buffer as *mut u8 as *mut c_void,
            (&raw mut (*coder).buffer as *mut u8).offset((*coder).pos as isize) as *const c_void,
            buf_avail,
        );
        (*coder).size = (*coder).size.wrapping_sub((*coder).pos);
        (*coder).pos = 0 as size_t;
    }
    if (*coder).size > 0 as size_t {
        let ret_0: lzma_ret = copy_or_code(
            coder,
            allocator,
            in_0,
            in_pos,
            in_size,
            &raw mut (*coder).buffer as *mut u8,
            &raw mut (*coder).size,
            (*coder).allocated,
            action,
        ) as lzma_ret;
        if ret_0 != LZMA_OK {
            return ret_0;
        }
        (*coder).filtered = call_filter(coder, &raw mut (*coder).buffer as *mut u8, (*coder).size);
        if (*coder).end_was_reached {
            (*coder).filtered = (*coder).size;
        }
        lzma_bufcpy(
            &raw mut (*coder).buffer as *mut u8,
            &raw mut (*coder).pos,
            (*coder).filtered,
            out,
            out_pos,
            out_size,
        );
    }
    if (*coder).end_was_reached as c_int != 0 && (*coder).pos == (*coder).size {
        return LZMA_STREAM_END;
    }
    return LZMA_OK;
}
unsafe extern "C" fn simple_coder_end(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_simple_coder = coder_ptr as *mut lzma_simple_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free((*coder).simple, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn simple_coder_update(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
    mut filters_null: *const lzma_filter,
    mut reversed_filters: *const lzma_filter,
) -> lzma_ret {
    let mut coder: *mut lzma_simple_coder = coder_ptr as *mut lzma_simple_coder;
    return lzma_next_filter_update(
        &raw mut (*coder).next,
        allocator,
        reversed_filters.offset(1),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_coder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
    mut filter: Option<unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t>,
    mut simple_size: size_t,
    mut unfiltered_max: size_t,
    mut alignment: u32,
    mut is_encoder: bool,
) -> lzma_ret {
    let mut coder: *mut lzma_simple_coder = (*next).coder as *mut lzma_simple_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            (::core::mem::size_of::<lzma_simple_coder>() as size_t)
                .wrapping_add((2 as size_t).wrapping_mul(unfiltered_max)),
            allocator,
        ) as *mut lzma_simple_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            simple_code
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
        (*next).end = Some(
            simple_coder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).update = Some(
            simple_coder_update
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
        (*coder).next = lzma_next_coder_s {
            coder: NULL,
            id: LZMA_VLI_UNKNOWN as lzma_vli,
            init: ::core::ptr::null_mut::<c_void>() as uintptr_t,
            code: None,
            end: None,
            get_progress: None,
            get_check: None,
            memconfig: None,
            update: None,
            set_out_limit: None,
        };
        (*coder).filter = filter;
        (*coder).allocated = (2 as size_t).wrapping_mul(unfiltered_max);
        if simple_size > 0 as size_t {
            (*coder).simple = lzma_alloc(simple_size, allocator);
            if (*coder).simple.is_null() {
                return LZMA_MEM_ERROR;
            }
        } else {
            (*coder).simple = NULL;
        }
    }
    if !(*filters.offset(0)).options.is_null() {
        let mut simple: *const lzma_options_bcj =
            (*filters.offset(0)).options as *const lzma_options_bcj;
        (*coder).now_pos = (*simple).start_offset;
        if (*coder).now_pos & alignment.wrapping_sub(1 as u32) != 0 {
            return LZMA_OPTIONS_ERROR;
        }
    } else {
        (*coder).now_pos = 0 as u32;
    }
    (*coder).is_encoder = is_encoder;
    (*coder).end_was_reached = false_0 != 0;
    (*coder).pos = 0 as size_t;
    (*coder).filtered = 0 as size_t;
    (*coder).size = 0 as size_t;
    return lzma_next_filter_init(&raw mut (*coder).next, allocator, filters.offset(1));
}
