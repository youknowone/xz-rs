pub type uint8_t = u8;
pub type uint32_t = u32;
pub type lzma_ret = ::core::ffi::c_uint;
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
pub struct lzma_options_bcj {
    pub start_offset: uint32_t,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[inline]
unsafe extern "C" fn write32le(mut buf: *mut uint8_t, mut num: uint32_t) {
    *buf.offset(0 as ::core::ffi::c_int as isize) = num as uint8_t;
    *buf.offset(1 as ::core::ffi::c_int as isize) = (num >> 8 as ::core::ffi::c_int) as uint8_t;
    *buf.offset(2 as ::core::ffi::c_int as isize) = (num >> 16 as ::core::ffi::c_int) as uint8_t;
    *buf.offset(3 as ::core::ffi::c_int as isize) = (num >> 24 as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_props_size(
    mut size: *mut uint32_t,
    mut options: *const ::core::ffi::c_void,
) -> lzma_ret {
    let opt: *const lzma_options_bcj = options as *const lzma_options_bcj;
    *size = (if opt.is_null() || (*opt).start_offset == 0 as uint32_t {
        0 as ::core::ffi::c_int
    } else {
        4 as ::core::ffi::c_int
    }) as uint32_t;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_props_encode(
    mut options: *const ::core::ffi::c_void,
    mut out: *mut uint8_t,
) -> lzma_ret {
    let opt: *const lzma_options_bcj = options as *const lzma_options_bcj;
    if opt.is_null() || (*opt).start_offset == 0 as uint32_t {
        return LZMA_OK;
    }
    write32le(out, (*opt).start_offset);
    return LZMA_OK;
}
