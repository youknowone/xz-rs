use crate::types::*;
use core::ffi::c_void;
unsafe extern "C" fn copy_or_code(
    coder: *mut lzma_simple_coder,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
    action: lzma_action,
) -> lzma_ret {
    if (*coder).next.code.is_none() {
        lzma_bufcpy(in_0, in_pos, in_size, out, out_pos, out_size);
        if (*coder).is_encoder && action == LZMA_FINISH && *in_pos == in_size {
            (*coder).end_was_reached = true;
        }
    } else {
        let ret: lzma_ret = (*coder).next.code.unwrap()(
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
        if ret == LZMA_STREAM_END {
            (*coder).end_was_reached = true;
        } else if ret != LZMA_OK {
            return ret;
        }
    }
    LZMA_OK
}
unsafe extern "C" fn call_filter(
    coder: *mut lzma_simple_coder,
    buffer: *mut u8,
    size: size_t,
) -> size_t {
    let filtered: size_t = (*coder).filter.unwrap()(
        (*coder).simple,
        (*coder).now_pos,
        (*coder).is_encoder,
        buffer,
        size,
    ) as size_t;
    (*coder).now_pos = ((*coder).now_pos as size_t).wrapping_add(filtered) as u32;
    filtered
}
unsafe extern "C" fn simple_code(
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
    let coder: *mut lzma_simple_coder = coder_ptr as *mut lzma_simple_coder;
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
    (*coder).filtered = 0;
    let out_avail: size_t = out_size.wrapping_sub(*out_pos);
    let buf_avail: size_t = (*coder).size.wrapping_sub((*coder).pos);
    if out_avail > buf_avail || buf_avail == 0 {
        let out_start: size_t = *out_pos;
        if buf_avail > 0 {
            core::ptr::copy_nonoverlapping(
                (&raw mut (*coder).buffer as *mut u8).offset((*coder).pos as isize) as *const u8,
                out.offset(*out_pos as isize) as *mut u8,
                buf_avail,
            );
        }
        *out_pos = (*out_pos).wrapping_add(buf_avail);
        let ret: lzma_ret = copy_or_code(
            coder, allocator, in_0, in_pos, in_size, out, out_pos, out_size, action,
        );
        if ret != LZMA_OK {
            return ret;
        }
        let size: size_t = (*out_pos).wrapping_sub(out_start);
        let filtered: size_t = if size == 0 {
            0
        } else {
            call_filter(coder, out.offset(out_start as isize), size) as size_t
        };
        let unfiltered: size_t = size.wrapping_sub(filtered);
        (*coder).pos = 0;
        (*coder).size = unfiltered;
        if (*coder).end_was_reached {
            (*coder).size = 0;
        } else if unfiltered > 0 {
            *out_pos = (*out_pos).wrapping_sub(unfiltered);
            core::ptr::copy_nonoverlapping(
                out.offset(*out_pos as isize) as *const u8,
                &raw mut (*coder).buffer as *mut u8,
                unfiltered,
            );
        }
    } else if (*coder).pos > 0 {
        core::ptr::copy(
            (&raw mut (*coder).buffer as *mut u8).offset((*coder).pos as isize) as *const u8,
            &raw mut (*coder).buffer as *mut u8,
            buf_avail,
        );
        (*coder).size = (*coder).size.wrapping_sub((*coder).pos);
        (*coder).pos = 0;
    }
    if (*coder).size > 0 {
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
        );
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
    if (*coder).end_was_reached && (*coder).pos == (*coder).size {
        return LZMA_STREAM_END;
    }
    LZMA_OK
}
unsafe extern "C" fn simple_coder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_simple_coder = coder_ptr as *mut lzma_simple_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free((*coder).simple, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn simple_coder_update(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    _filters_null: *const lzma_filter,
    reversed_filters: *const lzma_filter,
) -> lzma_ret {
    let coder: *mut lzma_simple_coder = coder_ptr as *mut lzma_simple_coder;
    lzma_next_filter_update(
        &raw mut (*coder).next,
        allocator,
        reversed_filters.offset(1),
    )
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    filter: Option<unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t>,
    simple_size: size_t,
    unfiltered_max: size_t,
    alignment: u32,
    is_encoder: bool,
) -> lzma_ret {
    let mut coder: *mut lzma_simple_coder = (*next).coder as *mut lzma_simple_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            (core::mem::size_of::<lzma_simple_coder>())
                .wrapping_add((2_usize).wrapping_mul(unfiltered_max)),
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
        );
        (*next).end = Some(
            simple_coder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        );
        (*next).update = Some(
            simple_coder_update
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const lzma_allocator,
                    *const lzma_filter,
                    *const lzma_filter,
                ) -> lzma_ret,
        );
        (*coder).next = lzma_next_coder_s {
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
        (*coder).filter = filter;
        (*coder).allocated = (2_usize).wrapping_mul(unfiltered_max);
        if simple_size > 0 {
            (*coder).simple = lzma_alloc(simple_size, allocator);
            if (*coder).simple.is_null() {
                return LZMA_MEM_ERROR;
            }
        } else {
            (*coder).simple = core::ptr::null_mut();
        }
    }
    if !(*filters).options.is_null() {
        let simple: *const lzma_options_bcj = (*filters).options as *const lzma_options_bcj;
        (*coder).now_pos = (*simple).start_offset;
        if (*coder).now_pos & alignment.wrapping_sub(1) != 0 {
            return LZMA_OPTIONS_ERROR;
        }
    } else {
        (*coder).now_pos = 0;
    }
    (*coder).is_encoder = is_encoder;
    (*coder).end_was_reached = false;
    (*coder).pos = 0;
    (*coder).filtered = 0;
    (*coder).size = 0;
    lzma_next_filter_init(&raw mut (*coder).next, allocator, filters.offset(1))
}
