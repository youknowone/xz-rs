use crate::types::*;
use core::ffi::{c_int, c_void};
extern "C" {
    fn lzma_delta_coder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
}
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
    );
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
    let opt: *mut lzma_options_delta =
        lzma_alloc(core::mem::size_of::<lzma_options_delta>(), allocator)
            as *mut lzma_options_delta;
    if opt.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*opt).type_0 = LZMA_DELTA_TYPE_BYTE;
    (*opt).dist = u32::from(*props.offset(0)).wrapping_add(1);
    *options = opt as *mut c_void;
    return LZMA_OK;
}
