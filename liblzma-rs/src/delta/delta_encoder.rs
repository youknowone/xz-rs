use crate::types::*;
unsafe fn copy_and_encode(
    coder: *mut lzma_delta_coder,
    in_0: *const u8,
    out: *mut u8,
    size: size_t,
) {
    let distance: size_t = (*coder).distance;
    let mut i: size_t = 0;
    while i < size {
        let tmp: u8 =
            (*coder).history[(distance.wrapping_add((*coder).pos as size_t) & 0xff) as usize];
        (*coder).history[((*coder).pos & 0xff) as usize] = *in_0.offset(i as isize);
        (*coder).pos = (*coder).pos.wrapping_sub(1);
        *out.offset(i as isize) = (*in_0.offset(i as isize)).wrapping_sub(tmp);
        i += 1;
    }
}
unsafe fn encode_in_place(coder: *mut lzma_delta_coder, buffer: *mut u8, size: size_t) {
    let distance: size_t = (*coder).distance;
    let mut i: size_t = 0;
    while i < size {
        let tmp: u8 =
            (*coder).history[(distance.wrapping_add((*coder).pos as size_t) & 0xff) as usize];
        (*coder).history[((*coder).pos & 0xff) as usize] = *buffer.offset(i as isize);
        (*coder).pos = (*coder).pos.wrapping_sub(1);
        *buffer.offset(i as isize) = (*buffer.offset(i as isize)).wrapping_sub(tmp);
        i += 1;
    }
}
unsafe extern "C" fn delta_encode(
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
    let mut ret: lzma_ret = LZMA_OK;
    if (*coder).next.code.is_none() {
        debug_assert!(in_size >= *in_pos);
        debug_assert!(out_size >= *out_pos);
        let in_avail: size_t = in_size - *in_pos;
        let out_avail: size_t = out_size - *out_pos;
        let size: size_t = if in_avail < out_avail {
            in_avail
        } else {
            out_avail
        };
        if size > 0 {
            copy_and_encode(
                coder,
                in_0.offset(*in_pos as isize),
                out.offset(*out_pos as isize),
                size,
            );
        }
        *in_pos += size;
        *out_pos += size;
        ret = if action != LZMA_RUN && *in_pos == in_size {
            LZMA_STREAM_END
        } else {
            LZMA_OK
        };
    } else {
        let out_start: size_t = *out_pos;
        ret = (*coder).next.code.unwrap()(
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
        debug_assert!(*out_pos >= out_start);
        let size_0: size_t = *out_pos - out_start;
        if size_0 > 0 {
            encode_in_place(coder, out.offset(out_start as isize), size_0);
        }
    }
    ret
}
unsafe extern "C" fn delta_encoder_update(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    _filters_null: *const lzma_filter,
    reversed_filters: *const lzma_filter,
) -> lzma_ret {
    let coder: *mut lzma_delta_coder = coder_ptr as *mut lzma_delta_coder;
    lzma_next_filter_update(
        ::core::ptr::addr_of_mut!((*coder).next),
        allocator,
        reversed_filters.offset(1),
    )
}
pub unsafe extern "C" fn lzma_delta_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
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
    );
    (*next).update = Some(
        delta_encoder_update
            as unsafe extern "C" fn(
                *mut c_void,
                *const lzma_allocator,
                *const lzma_filter,
                *const lzma_filter,
            ) -> lzma_ret,
    );
    lzma_delta_coder_init(next, allocator, filters)
}
pub unsafe extern "C" fn lzma_delta_props_encode(options: *const c_void, out: *mut u8) -> lzma_ret {
    if lzma_delta_coder_memusage(options) == UINT64_MAX {
        return LZMA_PROG_ERROR;
    }
    let opt: *const lzma_options_delta = options as *const lzma_options_delta;
    debug_assert!((*opt).dist >= LZMA_DELTA_DIST_MIN);
    *out = ((*opt).dist - LZMA_DELTA_DIST_MIN) as u8;
    LZMA_OK
}
