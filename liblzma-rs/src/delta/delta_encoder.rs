use crate::types::*;
use core::ffi::{c_int, c_void};
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
pub const LZMA_DELTA_DIST_MIN: c_int = 1;
unsafe extern "C" fn copy_and_encode(
    coder: *mut lzma_delta_coder,
    in_0: *const u8,
    out: *mut u8,
    size: size_t,
) {
    let distance: size_t = (*coder).distance;
    let mut i: size_t = 0;
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
unsafe extern "C" fn encode_in_place(coder: *mut lzma_delta_coder, buffer: *mut u8, size: size_t) {
    let distance: size_t = (*coder).distance;
    let mut i: size_t = 0;
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
        let in_avail: size_t = in_size.wrapping_sub(*in_pos);
        let out_avail: size_t = out_size.wrapping_sub(*out_pos);
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
        *in_pos = (*in_pos).wrapping_add(size);
        *out_pos = (*out_pos).wrapping_add(size);
        ret = if action != LZMA_RUN && *in_pos == in_size {
            LZMA_STREAM_END
        } else {
            LZMA_OK
        };
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
        if size_0 > 0 {
            encode_in_place(coder, out.offset(out_start as isize), size_0);
        }
    }
    return ret;
}
unsafe extern "C" fn delta_encoder_update(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    _filters_null: *const lzma_filter,
    reversed_filters: *const lzma_filter,
) -> lzma_ret {
    let coder: *mut lzma_delta_coder = coder_ptr as *mut lzma_delta_coder;
    return lzma_next_filter_update(
        &raw mut (*coder).next,
        allocator,
        reversed_filters.offset(1),
    );
}
#[no_mangle]
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
pub unsafe extern "C" fn lzma_delta_props_encode(options: *const c_void, out: *mut u8) -> lzma_ret {
    if lzma_delta_coder_memusage(options) == UINT64_MAX {
        return LZMA_PROG_ERROR;
    }
    let opt: *const lzma_options_delta = options as *const lzma_options_delta;
    *out.offset(0) = (*opt).dist.wrapping_sub(LZMA_DELTA_DIST_MIN as u32) as u8;
    return LZMA_OK;
}
