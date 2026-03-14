use crate::types::*;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
extern "C" {
    fn malloc(__size: size_t) -> *mut c_void;
    fn calloc(__count: size_t, __size: size_t) -> *mut c_void;
    fn free(_: *mut c_void);
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
const LZMA_ALLOC_ALIGN: usize = 16;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
const LZMA_ALLOC_HEADER_SIZE: usize = 16;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
unsafe fn wasm_alloc_layout(size: size_t) -> Option<Layout> {
    let total_size = (size as usize).checked_add(LZMA_ALLOC_HEADER_SIZE)?;
    Layout::from_size_align(total_size, LZMA_ALLOC_ALIGN).ok()
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
unsafe fn malloc(size: size_t) -> *mut c_void {
    let Some(layout) = wasm_alloc_layout(size) else {
        return core::ptr::null_mut();
    };
    let ptr = alloc(layout);
    if ptr.is_null() {
        return core::ptr::null_mut();
    }
    *(ptr as *mut usize) = layout.size();
    ptr.add(LZMA_ALLOC_HEADER_SIZE) as *mut c_void
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
unsafe fn calloc(count: size_t, size: size_t) -> *mut c_void {
    let Some(total_size) = (count as usize).checked_mul(size as usize) else {
        return core::ptr::null_mut();
    };
    let Some(layout) = wasm_alloc_layout(total_size as size_t) else {
        return core::ptr::null_mut();
    };
    let ptr = alloc_zeroed(layout);
    if ptr.is_null() {
        return core::ptr::null_mut();
    }
    *(ptr as *mut usize) = layout.size();
    ptr.add(LZMA_ALLOC_HEADER_SIZE) as *mut c_void
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
unsafe fn free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let base = (ptr as *mut u8).sub(LZMA_ALLOC_HEADER_SIZE);
    let total_size = *(base as *const usize);
    let Ok(layout) = Layout::from_size_align(total_size, LZMA_ALLOC_ALIGN) else {
        return;
    };
    dealloc(base, layout);
}
pub const LZMA_VERSION_MAJOR: u32 = 5;
pub const LZMA_VERSION_MINOR: u32 = 8;
pub const LZMA_VERSION_PATCH: u32 = 2;
pub const LZMA_VERSION_STABILITY: u32 = LZMA_VERSION_STABILITY_STABLE;
pub const LZMA_VERSION_STABILITY_STABLE: u32 = 2;
pub const LZMA_VERSION: c_uint = (LZMA_VERSION_MAJOR)
    .wrapping_mul(10000000)
    .wrapping_add((LZMA_VERSION_MINOR).wrapping_mul(10000))
    .wrapping_add((LZMA_VERSION_PATCH).wrapping_mul(10))
    .wrapping_add(LZMA_VERSION_STABILITY);
pub const LZMA_TIMED_OUT: c_uint = 101;
pub extern "C" fn lzma_version_number() -> u32 {
    LZMA_VERSION as u32
}
pub extern "C" fn lzma_version_string() -> *const c_char {
    crate::c_str!("5.8.2")
}
#[no_mangle]
pub unsafe extern "C" fn lzma_alloc(
    mut size: size_t,
    allocator: *const lzma_allocator,
) -> *mut c_void {
    if size == 0 {
        size = 1;
    }
    let mut ptr: *mut c_void = core::ptr::null_mut();
    if !allocator.is_null() && (*allocator).alloc.is_some() {
        ptr = (*allocator).alloc.unwrap()((*allocator).opaque, 1, size);
    } else {
        ptr = malloc(size);
    }
    ptr
}
#[no_mangle]
pub unsafe extern "C" fn lzma_alloc_zero(
    mut size: size_t,
    allocator: *const lzma_allocator,
) -> *mut c_void {
    if size == 0 {
        size = 1;
    }
    let mut ptr: *mut c_void = core::ptr::null_mut();
    if !allocator.is_null() && (*allocator).alloc.is_some() {
        ptr = (*allocator).alloc.unwrap()((*allocator).opaque, 1, size);
        if !ptr.is_null() {
            core::ptr::write_bytes(ptr as *mut u8, 0, size);
        }
    } else {
        ptr = calloc(1, size);
    }
    ptr
}
#[no_mangle]
pub unsafe extern "C" fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator) {
    if !allocator.is_null() && (*allocator).free.is_some() {
        (*allocator).free.unwrap()((*allocator).opaque, ptr);
    } else {
        free(ptr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bufcpy(
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> size_t {
    let in_avail: size_t = in_size.wrapping_sub(*in_pos);
    let out_avail: size_t = out_size.wrapping_sub(*out_pos);
    let copy_size: size_t = if in_avail < out_avail {
        in_avail
    } else {
        out_avail
    };
    if copy_size > 0 {
        core::ptr::copy_nonoverlapping(
            in_0.offset(*in_pos as isize) as *const u8,
            out.offset(*out_pos as isize) as *mut u8,
            copy_size,
        );
    }
    *in_pos = (*in_pos).wrapping_add(copy_size);
    *out_pos = (*out_pos).wrapping_add(copy_size);
    copy_size
}
#[no_mangle]
pub unsafe extern "C" fn lzma_next_filter_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    if core::mem::transmute::<lzma_init_function, uintptr_t>((*filters).init) != (*next).init {
        lzma_next_end(next, allocator);
    }
    (*next).init = core::mem::transmute::<lzma_init_function, uintptr_t>((*filters).init);
    (*next).id = (*filters).id;
    if (*filters).init.is_none() {
        LZMA_OK
    } else {
        (*filters).init.unwrap()(next, allocator, filters)
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_next_filter_update(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    reversed_filters: *const lzma_filter,
) -> lzma_ret {
    if (*reversed_filters).id != (*next).id {
        return LZMA_PROG_ERROR;
    }
    if (*reversed_filters).id == LZMA_VLI_UNKNOWN {
        return LZMA_OK;
    }
    (*next).update.unwrap()(
        (*next).coder,
        allocator,
        core::ptr::null(),
        reversed_filters,
    )
}
#[no_mangle]
pub unsafe extern "C" fn lzma_next_end(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
) {
    if (*next).init != 0 {
        if (*next).end.is_some() {
            (*next).end.unwrap()((*next).coder, allocator);
        } else {
            lzma_free((*next).coder, allocator);
        }
        *next = lzma_next_coder_s {
            coder: core::ptr::null_mut(),
            id: LZMA_VLI_UNKNOWN,
            init: 0,
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
pub unsafe extern "C" fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret {
    if strm.is_null() {
        return LZMA_PROG_ERROR;
    }
    if (*strm).internal.is_null() {
        (*strm).internal = lzma_alloc(core::mem::size_of::<lzma_internal>(), (*strm).allocator)
            as *mut lzma_internal;
        if (*strm).internal.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*(*strm).internal).next = lzma_next_coder_s {
            coder: core::ptr::null_mut(),
            id: LZMA_VLI_UNKNOWN,
            init: 0,
            code: None,
            end: None,
            get_progress: None,
            get_check: None,
            memconfig: None,
            update: None,
            set_out_limit: None,
        };
    }
    core::ptr::write_bytes(
        ::core::ptr::addr_of_mut!((*(*strm).internal).supported_actions) as *mut u8,
        0 as u8,
        core::mem::size_of::<[bool; 5]>(),
    );
    (*(*strm).internal).sequence = ISEQ_RUN;
    (*(*strm).internal).allow_buf_error = false;
    (*strm).total_in = 0;
    (*strm).total_out = 0;
    LZMA_OK
}
pub unsafe extern "C" fn lzma_code(strm: *mut lzma_stream, action: lzma_action) -> lzma_ret {
    if (*strm).next_in.is_null() && (*strm).avail_in != 0
        || (*strm).next_out.is_null() && (*strm).avail_out != 0
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
        || (*strm).reserved_int2 != 0
        || (*strm).reserved_int3 != 0
        || (*strm).reserved_int4 != 0
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
    let mut in_pos: size_t = 0;
    let mut out_pos: size_t = 0;
    let mut ret: lzma_ret = (*(*strm).internal).next.code.unwrap()(
        (*(*strm).internal).next.coder,
        (*strm).allocator,
        (*strm).next_in,
        ::core::ptr::addr_of_mut!(in_pos),
        (*strm).avail_in,
        (*strm).next_out,
        ::core::ptr::addr_of_mut!(out_pos),
        (*strm).avail_out,
        action,
    );
    if in_pos > 0 {
        (*strm).next_in = (*strm).next_in.offset(in_pos as isize);
        (*strm).avail_in = (*strm).avail_in.wrapping_sub(in_pos);
        (*strm).total_in = (*strm).total_in.wrapping_add(in_pos as u64);
    }
    if out_pos > 0 {
        (*strm).next_out = (*strm).next_out.offset(out_pos as isize);
        (*strm).avail_out = (*strm).avail_out.wrapping_sub(out_pos);
        (*strm).total_out = (*strm).total_out.wrapping_add(out_pos as u64);
    }
    (*(*strm).internal).avail_in = (*strm).avail_in;
    let current_block_49: u64;
    match ret {
        0 => {
            if out_pos == 0 && in_pos == 0 {
                if (*(*strm).internal).allow_buf_error {
                    ret = LZMA_BUF_ERROR;
                } else {
                    (*(*strm).internal).allow_buf_error = true;
                }
            } else {
                (*(*strm).internal).allow_buf_error = false;
            }
            current_block_49 = 12556861819962772176;
        }
        101 => {
            (*(*strm).internal).allow_buf_error = false;
            ret = LZMA_OK;
            current_block_49 = 12556861819962772176;
        }
        12 => {
            (*(*strm).internal).allow_buf_error = false;
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
            (*(*strm).internal).allow_buf_error = false;
        }
        _ => {}
    }
    ret
}
pub unsafe extern "C" fn lzma_end(strm: *mut lzma_stream) {
    if !strm.is_null() && !(*strm).internal.is_null() {
        lzma_next_end(
            ::core::ptr::addr_of_mut!((*(*strm).internal).next),
            (*strm).allocator,
        );
        lzma_free((*strm).internal as *mut c_void, (*strm).allocator);
        (*strm).internal = core::ptr::null_mut();
    }
}
pub unsafe extern "C" fn lzma_get_progress(
    strm: *mut lzma_stream,
    progress_in: *mut u64,
    progress_out: *mut u64,
) {
    if (*(*strm).internal).next.get_progress.is_some() {
        (*(*strm).internal).next.get_progress.unwrap()(
            (*(*strm).internal).next.coder,
            progress_in,
            progress_out,
        );
    } else {
        *progress_in = (*strm).total_in;
        *progress_out = (*strm).total_out;
    };
}
pub extern "C" fn lzma_get_check(strm: *const lzma_stream) -> lzma_check {
    return unsafe {
        if (*(*strm).internal).next.get_check.is_none() {
            return LZMA_CHECK_NONE;
        }
        (*(*strm).internal).next.get_check.unwrap()((*(*strm).internal).next.coder)
    };
}
pub extern "C" fn lzma_memusage(strm: *const lzma_stream) -> u64 {
    return unsafe {
        let mut memusage: u64 = 0;
        let mut old_memlimit: u64 = 0;
        if strm.is_null()
            || (*strm).internal.is_null()
            || (*(*strm).internal).next.memconfig.is_none()
            || (*(*strm).internal).next.memconfig.unwrap()(
                (*(*strm).internal).next.coder,
                ::core::ptr::addr_of_mut!(memusage),
                ::core::ptr::addr_of_mut!(old_memlimit),
                0,
            ) != LZMA_OK
        {
            return 0;
        }
        memusage
    };
}
pub extern "C" fn lzma_memlimit_get(strm: *const lzma_stream) -> u64 {
    return unsafe {
        let mut old_memlimit: u64 = 0;
        let mut memusage: u64 = 0;
        if strm.is_null()
            || (*strm).internal.is_null()
            || (*(*strm).internal).next.memconfig.is_none()
            || (*(*strm).internal).next.memconfig.unwrap()(
                (*(*strm).internal).next.coder,
                ::core::ptr::addr_of_mut!(memusage),
                ::core::ptr::addr_of_mut!(old_memlimit),
                0,
            ) != LZMA_OK
        {
            return 0;
        }
        old_memlimit
    };
}
pub unsafe extern "C" fn lzma_memlimit_set(
    strm: *mut lzma_stream,
    mut new_memlimit: u64,
) -> lzma_ret {
    let mut old_memlimit: u64 = 0;
    let mut memusage: u64 = 0;
    if strm.is_null() || (*strm).internal.is_null() || (*(*strm).internal).next.memconfig.is_none()
    {
        return LZMA_PROG_ERROR;
    }
    if new_memlimit == 0 {
        new_memlimit = 1;
    }
    (*(*strm).internal).next.memconfig.unwrap()(
        (*(*strm).internal).next.coder,
        ::core::ptr::addr_of_mut!(memusage),
        ::core::ptr::addr_of_mut!(old_memlimit),
        new_memlimit,
    )
}
