use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
extern "C" {
    fn lzma_vli_decode(
        vli: *mut lzma_vli,
        vli_pos: *mut size_t,
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
    ) -> lzma_ret;
    fn lzma_properties_decode(
        filter: *mut lzma_filter,
        allocator: *const lzma_allocator,
        props: *const u8,
        props_size: size_t,
    ) -> lzma_ret;
}
pub const LZMA_RET_INTERNAL8: lzma_ret = 108;
pub const LZMA_RET_INTERNAL7: lzma_ret = 107;
pub const LZMA_RET_INTERNAL6: lzma_ret = 106;
pub const LZMA_RET_INTERNAL5: lzma_ret = 105;
pub const LZMA_RET_INTERNAL4: lzma_ret = 104;
pub const LZMA_RET_INTERNAL3: lzma_ret = 103;
pub const LZMA_RET_INTERNAL2: lzma_ret = 102;
pub const LZMA_RET_INTERNAL1: lzma_ret = 101;
pub const LZMA_SEEK_NEEDED: lzma_ret = 12;
pub const LZMA_PROG_ERROR: lzma_ret = 11;
pub const LZMA_BUF_ERROR: lzma_ret = 10;
pub const LZMA_DATA_ERROR: lzma_ret = 9;
pub const LZMA_OPTIONS_ERROR: lzma_ret = 8;
pub const LZMA_FORMAT_ERROR: lzma_ret = 7;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret = 6;
pub const LZMA_MEM_ERROR: lzma_ret = 5;
pub const LZMA_GET_CHECK: lzma_ret = 4;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret = 3;
pub const LZMA_NO_CHECK: lzma_ret = 2;
pub const LZMA_STREAM_END: lzma_ret = 1;
pub const LZMA_OK: lzma_ret = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, size_t, size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    pub opaque: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut c_void,
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const LZMA_FILTER_RESERVED_START: c_ulonglong = 1 << 62;
#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_decode(
    mut filter: *mut lzma_filter,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
) -> lzma_ret {
    (*filter).options = NULL;
    let ret_: lzma_ret = lzma_vli_decode(
        &raw mut (*filter).id,
        ::core::ptr::null_mut::<size_t>(),
        in_0,
        in_pos,
        in_size,
    ) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    if (*filter).id >= LZMA_FILTER_RESERVED_START as lzma_vli {
        return LZMA_DATA_ERROR;
    }
    let mut props_size: lzma_vli = 0;
    let ret__0: lzma_ret = lzma_vli_decode(
        &raw mut props_size,
        ::core::ptr::null_mut::<size_t>(),
        in_0,
        in_pos,
        in_size,
    ) as lzma_ret;
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
    ) as lzma_ret;
    *in_pos = (*in_pos as lzma_vli).wrapping_add(props_size) as size_t as size_t;
    return ret;
}
