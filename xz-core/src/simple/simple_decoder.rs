use crate::types::*;
pub(crate) unsafe fn lzma_simple_props_decode(
    options: *mut *mut c_void,
    allocator: *const lzma_allocator,
    props: *const u8,
    props_size: size_t,
) -> lzma_ret {
    if props_size == 0 {
        return LZMA_OK;
    }
    if props_size != 4 {
        return LZMA_OPTIONS_ERROR;
    }
    let opt: *mut lzma_options_bcj =
        lzma_alloc(core::mem::size_of::<lzma_options_bcj>(), allocator) as *mut lzma_options_bcj;
    if opt.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*opt).start_offset = read32le(&*props.cast::<[u8; 4]>());
    if (*opt).start_offset == 0 {
        lzma_free(opt as *mut c_void, allocator);
    } else {
        *options = opt as *mut c_void;
    }
    LZMA_OK
}
