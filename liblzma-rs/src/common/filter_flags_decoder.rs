use crate::types::*;
extern "C" {
    fn lzma_properties_decode(
        filter: *mut lzma_filter,
        allocator: *const lzma_allocator,
        props: *const u8,
        props_size: size_t,
    ) -> lzma_ret;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_decode(
    filter: *mut lzma_filter,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    (*filter).options = core::ptr::null_mut();
    let ret_: lzma_ret = lzma_vli_decode(
        ::core::ptr::addr_of_mut!((*filter).id),
        core::ptr::null_mut(),
        in_0,
        in_pos,
        in_size,
    );
    if ret_ != LZMA_OK {
        return ret_;
    }
    if (*filter).id >= LZMA_FILTER_RESERVED_START {
        return LZMA_DATA_ERROR;
    }
    let mut props_size: lzma_vli = 0;
    let ret__0: lzma_ret = lzma_vli_decode(
        ::core::ptr::addr_of_mut!(props_size),
        core::ptr::null_mut(),
        in_0,
        in_pos,
        in_size,
    );
    if ret__0 != LZMA_OK {
        return ret__0;
    }
    if (in_size.wrapping_sub(*in_pos) as lzma_vli) < props_size {
        return LZMA_DATA_ERROR;
    }
    let ret: lzma_ret = lzma_properties_decode(
        filter,
        allocator,
        in_0.offset(*in_pos as isize),
        props_size as size_t,
    );
    *in_pos = (*in_pos as lzma_vli).wrapping_add(props_size) as size_t;
    ret
}
