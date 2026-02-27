use crate::types::*;
use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
extern "C" {
    fn malloc(__size: size_t) -> *mut c_void;
    fn calloc(__count: size_t, __size: size_t) -> *mut c_void;
    fn free(_: *mut c_void);
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn memset(__b: *mut c_void, __c: c_int, __len: size_t) -> *mut c_void;
}
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_internal_s {
    pub next: lzma_next_coder,
    pub sequence: C2RustUnnamed,
    pub avail_in: size_t,
    pub supported_actions: [bool; 5],
    pub allow_buf_error: bool,
}
pub type C2RustUnnamed = c_uint;
pub const ISEQ_ERROR: C2RustUnnamed = 6;
pub const ISEQ_END: C2RustUnnamed = 5;
pub const ISEQ_FULL_BARRIER: C2RustUnnamed = 4;
pub const ISEQ_FINISH: C2RustUnnamed = 3;
pub const ISEQ_FULL_FLUSH: C2RustUnnamed = 2;
pub const ISEQ_SYNC_FLUSH: C2RustUnnamed = 1;
pub const ISEQ_RUN: C2RustUnnamed = 0;
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
pub type lzma_internal = lzma_internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream {
    pub next_in: *const u8,
    pub avail_in: size_t,
    pub total_in: u64,
    pub next_out: *mut u8,
    pub avail_out: size_t,
    pub total_out: u64,
    pub allocator: *const lzma_allocator,
    pub internal: *mut lzma_internal,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
    pub reserved_ptr3: *mut c_void,
    pub reserved_ptr4: *mut c_void,
    pub seek_pos: u64,
    pub reserved_int2: u64,
    pub reserved_int3: size_t,
    pub reserved_int4: size_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
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
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
pub const LZMA_VERSION_MAJOR: c_int = 5 as c_int;
pub const LZMA_VERSION_MINOR: c_int = 8 as c_int;
pub const LZMA_VERSION_PATCH: c_int = 2 as c_int;
pub const LZMA_VERSION_STABILITY: c_int = LZMA_VERSION_STABILITY_STABLE;
pub const LZMA_VERSION_STABILITY_STABLE: c_int = 2 as c_int;
pub const LZMA_VERSION: c_uint = (LZMA_VERSION_MAJOR as u32)
    .wrapping_mul(10000000)
    .wrapping_add((LZMA_VERSION_MINOR as u32).wrapping_mul(10000))
    .wrapping_add((LZMA_VERSION_PATCH as u32).wrapping_mul(10))
    .wrapping_add(LZMA_VERSION_STABILITY as u32);
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_TIMED_OUT: c_uint = 101;
#[no_mangle]
pub unsafe extern "C" fn lzma_version_number() -> u32 {
    return LZMA_VERSION as u32;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_version_string() -> *const c_char {
    return b"5.8.2\0" as *const u8 as *const c_char;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_alloc(
    mut size: size_t,
    mut allocator: *const lzma_allocator,
) -> *mut c_void {
    if size == 0 as size_t {
        size = 1 as size_t;
    }
    let mut ptr: *mut c_void = ::core::ptr::null_mut::<c_void>();
    if !allocator.is_null() && (*allocator).alloc.is_some() {
        ptr = (*allocator).alloc.expect("non-null function pointer")(
            (*allocator).opaque,
            1 as size_t,
            size,
        );
    } else {
        ptr = malloc(size);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_alloc_zero(
    mut size: size_t,
    mut allocator: *const lzma_allocator,
) -> *mut c_void {
    if size == 0 as size_t {
        size = 1 as size_t;
    }
    let mut ptr: *mut c_void = ::core::ptr::null_mut::<c_void>();
    if !allocator.is_null() && (*allocator).alloc.is_some() {
        ptr = (*allocator).alloc.expect("non-null function pointer")(
            (*allocator).opaque,
            1 as size_t,
            size,
        );
        if !ptr.is_null() {
            memset(ptr, 0 as c_int, size);
        }
    } else {
        ptr = calloc(1 as size_t, size);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_free(mut ptr: *mut c_void, mut allocator: *const lzma_allocator) {
    if !allocator.is_null() && (*allocator).free.is_some() {
        (*allocator).free.expect("non-null function pointer")((*allocator).opaque, ptr);
    } else {
        free(ptr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bufcpy(
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> size_t {
    let in_avail: size_t = in_size.wrapping_sub(*in_pos);
    let out_avail: size_t = out_size.wrapping_sub(*out_pos);
    let copy_size: size_t = if in_avail < out_avail {
        in_avail
    } else {
        out_avail
    };
    if copy_size > 0 as size_t {
        memcpy(
            out.offset(*out_pos as isize) as *mut c_void,
            in_0.offset(*in_pos as isize) as *const c_void,
            copy_size,
        );
    }
    *in_pos = (*in_pos).wrapping_add(copy_size);
    *out_pos = (*out_pos).wrapping_add(copy_size);
    return copy_size;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_next_filter_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
) -> lzma_ret {
    if ::core::mem::transmute::<lzma_init_function, uintptr_t>((*filters.offset(0)).init)
        != (*next).init
    {
        lzma_next_end(next, allocator);
    }
    (*next).init =
        ::core::mem::transmute::<lzma_init_function, uintptr_t>((*filters.offset(0)).init);
    (*next).id = (*filters.offset(0)).id;
    return (if (*filters.offset(0)).init.is_none() {
        LZMA_OK
    } else {
        (*filters.offset(0))
            .init
            .expect("non-null function pointer")(next, allocator, filters)
    }) as lzma_ret;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_next_filter_update(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut reversed_filters: *const lzma_filter,
) -> lzma_ret {
    if (*reversed_filters.offset(0)).id != (*next).id {
        return LZMA_PROG_ERROR;
    }
    if (*reversed_filters.offset(0)).id == LZMA_VLI_UNKNOWN as lzma_vli {
        return LZMA_OK;
    }
    return (*next).update.expect("non-null function pointer")(
        (*next).coder,
        allocator,
        ::core::ptr::null::<lzma_filter>(),
        reversed_filters,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_next_end(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
) {
    if (*next).init != ::core::ptr::null_mut::<c_void>() as uintptr_t {
        if (*next).end.is_some() {
            (*next).end.expect("non-null function pointer")((*next).coder, allocator);
        } else {
            lzma_free((*next).coder, allocator);
        }
        *next = lzma_next_coder_s {
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
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_strm_init(mut strm: *mut lzma_stream) -> lzma_ret {
    if strm.is_null() {
        return LZMA_PROG_ERROR;
    }
    if (*strm).internal.is_null() {
        (*strm).internal = lzma_alloc(
            ::core::mem::size_of::<lzma_internal>() as size_t,
            (*strm).allocator,
        ) as *mut lzma_internal;
        if (*strm).internal.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*(*strm).internal).next = lzma_next_coder_s {
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
    }
    memset(
        &raw mut (*(*strm).internal).supported_actions as *mut bool as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[bool; 5]>() as size_t,
    );
    (*(*strm).internal).sequence = ISEQ_RUN;
    (*(*strm).internal).allow_buf_error = false_0 != 0;
    (*strm).total_in = 0 as u64;
    (*strm).total_out = 0 as u64;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_code(
    mut strm: *mut lzma_stream,
    mut action: lzma_action,
) -> lzma_ret {
    if (*strm).next_in.is_null() && (*strm).avail_in != 0 as size_t
        || (*strm).next_out.is_null() && (*strm).avail_out != 0 as size_t
        || (*strm).internal.is_null()
        || (*(*strm).internal).next.code.is_none()
        || action > LZMA_FULL_BARRIER
        || !(*(*strm).internal).supported_actions[action as usize]
    {
        return LZMA_PROG_ERROR;
    }
    if !(*strm).reserved_ptr1.is_null()
        || !(*strm).reserved_ptr2.is_null()
        || !(*strm).reserved_ptr3.is_null()
        || !(*strm).reserved_ptr4.is_null()
        || (*strm).reserved_int2 != 0 as u64
        || (*strm).reserved_int3 != 0 as size_t
        || (*strm).reserved_int4 != 0 as size_t
        || (*strm).reserved_enum1 != LZMA_RESERVED_ENUM
        || (*strm).reserved_enum2 != LZMA_RESERVED_ENUM
    {
        return LZMA_OPTIONS_ERROR;
    }
    match (*(*strm).internal).sequence {
        0 => match action {
            1 => {
                (*(*strm).internal).sequence = ISEQ_SYNC_FLUSH;
            }
            2 => {
                (*(*strm).internal).sequence = ISEQ_FULL_FLUSH;
            }
            3 => {
                (*(*strm).internal).sequence = ISEQ_FINISH;
            }
            4 => {
                (*(*strm).internal).sequence = ISEQ_FULL_BARRIER;
            }
            0 | _ => {}
        },
        1 => {
            if action != LZMA_SYNC_FLUSH || (*(*strm).internal).avail_in != (*strm).avail_in {
                return LZMA_PROG_ERROR;
            }
        }
        2 => {
            if action != LZMA_FULL_FLUSH || (*(*strm).internal).avail_in != (*strm).avail_in {
                return LZMA_PROG_ERROR;
            }
        }
        3 => {
            if action != LZMA_FINISH || (*(*strm).internal).avail_in != (*strm).avail_in {
                return LZMA_PROG_ERROR;
            }
        }
        4 => {
            if action != LZMA_FULL_BARRIER || (*(*strm).internal).avail_in != (*strm).avail_in {
                return LZMA_PROG_ERROR;
            }
        }
        5 => return LZMA_STREAM_END,
        6 | _ => return LZMA_PROG_ERROR,
    }
    let mut in_pos: size_t = 0 as size_t;
    let mut out_pos: size_t = 0 as size_t;
    let mut ret: lzma_ret = (*(*strm).internal)
        .next
        .code
        .expect("non-null function pointer")(
        (*(*strm).internal).next.coder,
        (*strm).allocator,
        (*strm).next_in,
        &raw mut in_pos,
        (*strm).avail_in,
        (*strm).next_out,
        &raw mut out_pos,
        (*strm).avail_out,
        action,
    );
    if in_pos > 0 as size_t {
        (*strm).next_in = (*strm).next_in.offset(in_pos as isize);
        (*strm).avail_in = (*strm).avail_in.wrapping_sub(in_pos);
        (*strm).total_in = (*strm).total_in.wrapping_add(in_pos as u64);
    }
    if out_pos > 0 as size_t {
        (*strm).next_out = (*strm).next_out.offset(out_pos as isize);
        (*strm).avail_out = (*strm).avail_out.wrapping_sub(out_pos);
        (*strm).total_out = (*strm).total_out.wrapping_add(out_pos as u64);
    }
    (*(*strm).internal).avail_in = (*strm).avail_in;
    let mut current_block_49: u64;
    match ret {
        0 => {
            if out_pos == 0 as size_t && in_pos == 0 as size_t {
                if (*(*strm).internal).allow_buf_error {
                    ret = LZMA_BUF_ERROR;
                } else {
                    (*(*strm).internal).allow_buf_error = true_0 != 0;
                }
            } else {
                (*(*strm).internal).allow_buf_error = false_0 != 0;
            }
            current_block_49 = 12556861819962772176;
        }
        101 => {
            (*(*strm).internal).allow_buf_error = false_0 != 0;
            ret = LZMA_OK;
            current_block_49 = 12556861819962772176;
        }
        12 => {
            (*(*strm).internal).allow_buf_error = false_0 != 0;
            if (*(*strm).internal).sequence == ISEQ_FINISH {
                (*(*strm).internal).sequence = ISEQ_RUN;
            }
            current_block_49 = 12556861819962772176;
        }
        1 => {
            if (*(*strm).internal).sequence == ISEQ_SYNC_FLUSH
                || (*(*strm).internal).sequence == ISEQ_FULL_FLUSH
                || (*(*strm).internal).sequence == ISEQ_FULL_BARRIER
            {
                (*(*strm).internal).sequence = ISEQ_RUN;
            } else {
                (*(*strm).internal).sequence = ISEQ_END;
            }
            current_block_49 = 16143107162343188004;
        }
        2 | 3 | 4 | 6 => {
            current_block_49 = 16143107162343188004;
        }
        _ => {
            (*(*strm).internal).sequence = ISEQ_ERROR;
            current_block_49 = 12556861819962772176;
        }
    }
    match current_block_49 {
        16143107162343188004 => {
            (*(*strm).internal).allow_buf_error = false_0 != 0;
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_end(mut strm: *mut lzma_stream) {
    if !strm.is_null() && !(*strm).internal.is_null() {
        lzma_next_end(&raw mut (*(*strm).internal).next, (*strm).allocator);
        lzma_free((*strm).internal as *mut c_void, (*strm).allocator);
        (*strm).internal = ::core::ptr::null_mut::<lzma_internal>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_get_progress(
    mut strm: *mut lzma_stream,
    mut progress_in: *mut u64,
    mut progress_out: *mut u64,
) {
    if (*(*strm).internal).next.get_progress.is_some() {
        (*(*strm).internal)
            .next
            .get_progress
            .expect("non-null function pointer")(
            (*(*strm).internal).next.coder,
            progress_in,
            progress_out,
        );
    } else {
        *progress_in = (*strm).total_in;
        *progress_out = (*strm).total_out;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_get_check(mut strm: *const lzma_stream) -> lzma_check {
    if (*(*strm).internal).next.get_check.is_none() {
        return LZMA_CHECK_NONE;
    }
    return (*(*strm).internal)
        .next
        .get_check
        .expect("non-null function pointer")((*(*strm).internal).next.coder);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_memusage(mut strm: *const lzma_stream) -> u64 {
    let mut memusage: u64 = 0;
    let mut old_memlimit: u64 = 0;
    if strm.is_null()
        || (*strm).internal.is_null()
        || (*(*strm).internal).next.memconfig.is_none()
        || (*(*strm).internal)
            .next
            .memconfig
            .expect("non-null function pointer")(
            (*(*strm).internal).next.coder,
            &raw mut memusage,
            &raw mut old_memlimit,
            0 as u64,
        ) != LZMA_OK
    {
        return 0 as u64;
    }
    return memusage;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_memlimit_get(mut strm: *const lzma_stream) -> u64 {
    let mut old_memlimit: u64 = 0;
    let mut memusage: u64 = 0;
    if strm.is_null()
        || (*strm).internal.is_null()
        || (*(*strm).internal).next.memconfig.is_none()
        || (*(*strm).internal)
            .next
            .memconfig
            .expect("non-null function pointer")(
            (*(*strm).internal).next.coder,
            &raw mut memusage,
            &raw mut old_memlimit,
            0 as u64,
        ) != LZMA_OK
    {
        return 0 as u64;
    }
    return old_memlimit;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_memlimit_set(
    mut strm: *mut lzma_stream,
    mut new_memlimit: u64,
) -> lzma_ret {
    let mut old_memlimit: u64 = 0;
    let mut memusage: u64 = 0;
    if strm.is_null() || (*strm).internal.is_null() || (*(*strm).internal).next.memconfig.is_none()
    {
        return LZMA_PROG_ERROR;
    }
    if new_memlimit == 0 as u64 {
        new_memlimit = 1 as u64;
    }
    return (*(*strm).internal)
        .next
        .memconfig
        .expect("non-null function pointer")(
        (*(*strm).internal).next.coder,
        &raw mut memusage,
        &raw mut old_memlimit,
        new_memlimit,
    );
}
