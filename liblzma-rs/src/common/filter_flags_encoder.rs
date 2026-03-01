use crate::types::*;
extern "C" {
    fn lzma_vli_encode(
        vli: lzma_vli,
        vli_pos: *mut size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> lzma_ret;
    fn lzma_vli_size(vli: lzma_vli) -> u32;
    fn lzma_properties_size(size: *mut u32, filter: *const lzma_filter) -> lzma_ret;
    fn lzma_properties_encode(filter: *const lzma_filter, props: *mut u8) -> lzma_ret;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_size(
    size: *mut u32,
    filter: *const lzma_filter,
) -> lzma_ret {
    if (*filter).id >= LZMA_FILTER_RESERVED_START as lzma_vli {
        return LZMA_PROG_ERROR;
    }
    let ret_: lzma_ret = lzma_properties_size(size, filter) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    *size = (*size)
        .wrapping_add(lzma_vli_size((*filter).id).wrapping_add(lzma_vli_size(*size as lzma_vli)));
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_encode(
    filter: *const lzma_filter,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    if (*filter).id >= LZMA_FILTER_RESERVED_START as lzma_vli {
        return LZMA_PROG_ERROR;
    }
    let ret_: lzma_ret =
        lzma_vli_encode((*filter).id, core::ptr::null_mut(), out, out_pos, out_size) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let mut props_size: u32 = 0;
    let ret__0: lzma_ret = lzma_properties_size(&raw mut props_size, filter) as lzma_ret;
    if ret__0 != LZMA_OK {
        return ret__0;
    }
    let ret__1: lzma_ret = lzma_vli_encode(
        props_size as lzma_vli,
        core::ptr::null_mut(),
        out,
        out_pos,
        out_size,
    ) as lzma_ret;
    if ret__1 != LZMA_OK {
        return ret__1;
    }
    if out_size.wrapping_sub(*out_pos) < props_size as size_t {
        return LZMA_PROG_ERROR;
    }
    let ret__2: lzma_ret =
        lzma_properties_encode(filter, out.offset(*out_pos as isize)) as lzma_ret;
    if ret__2 != LZMA_OK {
        return ret__2;
    }
    *out_pos = (*out_pos).wrapping_add(props_size as size_t);
    return LZMA_OK;
}
