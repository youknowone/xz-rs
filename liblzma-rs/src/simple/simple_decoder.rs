use crate::types::*;
use core::ffi::c_void;
#[inline]
extern "C" fn read32le(buf: *const u8) -> u32 {
    return unsafe {
        let mut num: u32 = *buf as u32;
        num |= (*buf.offset(1) as u32) << 8;
        num |= (*buf.offset(2) as u32) << 16;
        num |= (*buf.offset(3) as u32) << 24;
        num
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_props_decode(
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
    (*opt).start_offset = read32le(props);
    if (*opt).start_offset == 0 {
        lzma_free(opt as *mut c_void, allocator);
    } else {
        *options = opt as *mut c_void;
    }
    LZMA_OK
}
