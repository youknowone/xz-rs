use crate::common::filter_decoder::lzma_properties_decode;
use crate::types::*;
pub unsafe fn lzma_filter_flags_decode(
    filter: *mut lzma_filter,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    (*filter).options = core::ptr::null_mut();
    let ret: lzma_ret = lzma_vli_decode(
        ::core::ptr::addr_of_mut!((*filter).id),
        core::ptr::null_mut(),
        input,
        in_pos,
        in_size,
    );
    if ret != LZMA_OK {
        return ret;
    }
    if (*filter).id >= LZMA_FILTER_RESERVED_START {
        return LZMA_DATA_ERROR;
    }
    let mut props_size: lzma_vli = 0;
    let ret: lzma_ret = lzma_vli_decode(
        ::core::ptr::addr_of_mut!(props_size),
        core::ptr::null_mut(),
        input,
        in_pos,
        in_size,
    );
    if ret != LZMA_OK {
        return ret;
    }
    if ((in_size - *in_pos) as lzma_vli) < props_size {
        return LZMA_DATA_ERROR;
    }
    let ret: lzma_ret = lzma_properties_decode(
        filter,
        allocator,
        input.offset(*in_pos as isize),
        props_size as size_t,
    );
    *in_pos = (*in_pos as lzma_vli + props_size) as size_t;
    ret
}
