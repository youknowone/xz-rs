extern "C" {
    fn lzma_vli_encode(
        vli: lzma_vli,
        vli_pos: *mut size_t,
        out: *mut uint8_t,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> lzma_ret;
    fn lzma_vli_size(vli: lzma_vli) -> uint32_t;
    fn lzma_properties_size(size: *mut uint32_t, filter: *const lzma_filter) -> lzma_ret;
    fn lzma_properties_encode(filter: *const lzma_filter, props: *mut uint8_t) -> lzma_ret;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut ::core::ffi::c_void,
}
pub type lzma_vli = uint64_t;
pub const LZMA_FILTER_RESERVED_START: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_size(
    mut size: *mut uint32_t,
    mut filter: *const lzma_filter,
) -> lzma_ret {
    if (*filter).id >= LZMA_FILTER_RESERVED_START as lzma_vli {
        return LZMA_PROG_ERROR;
    }
    let ret_: lzma_ret = lzma_properties_size(size, filter) as lzma_ret;
    if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret_;
    }
    *size = (*size)
        .wrapping_add(lzma_vli_size((*filter).id).wrapping_add(lzma_vli_size(*size as lzma_vli)));
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_encode(
    mut filter: *const lzma_filter,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    if (*filter).id >= LZMA_FILTER_RESERVED_START as lzma_vli {
        return LZMA_PROG_ERROR;
    }
    let ret_: lzma_ret = lzma_vli_encode(
        (*filter).id,
        ::core::ptr::null_mut::<size_t>(),
        out,
        out_pos,
        out_size,
    ) as lzma_ret;
    if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret_;
    }
    let mut props_size: uint32_t = 0;
    let ret__0: lzma_ret = lzma_properties_size(&raw mut props_size, filter) as lzma_ret;
    if ret__0 as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret__0;
    }
    let ret__1: lzma_ret = lzma_vli_encode(
        props_size as lzma_vli,
        ::core::ptr::null_mut::<size_t>(),
        out,
        out_pos,
        out_size,
    ) as lzma_ret;
    if ret__1 as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret__1;
    }
    if out_size.wrapping_sub(*out_pos) < props_size as size_t {
        return LZMA_PROG_ERROR;
    }
    let ret__2: lzma_ret =
        lzma_properties_encode(filter, out.offset(*out_pos as isize)) as lzma_ret;
    if ret__2 as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret__2;
    }
    *out_pos = (*out_pos).wrapping_add(props_size as size_t);
    return LZMA_OK;
}
