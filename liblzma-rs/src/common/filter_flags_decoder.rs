extern "C" {
    fn lzma_vli_decode(
        vli: *mut lzma_vli,
        vli_pos: *mut size_t,
        in_0: *const uint8_t,
        in_pos: *mut size_t,
        in_size: size_t,
    ) -> lzma_ret;
    fn lzma_properties_decode(
        filter: *mut lzma_filter,
        allocator: *const lzma_allocator,
        props: *const uint8_t,
        props_size: size_t,
    ) -> lzma_ret;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
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
pub struct lzma_allocator {
    pub alloc: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, size_t) -> *mut ::core::ffi::c_void,
    >,
    pub free:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut ::core::ffi::c_void,
}
pub type lzma_vli = uint64_t;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const LZMA_FILTER_RESERVED_START: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn lzma_filter_flags_decode(
    mut filter: *mut lzma_filter,
    mut allocator: *const lzma_allocator,
    mut in_0: *const uint8_t,
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
    if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
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
    if ret__0 as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
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
