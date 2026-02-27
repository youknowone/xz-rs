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
pub type lzma_vli = uint64_t;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const UINT64_MAX: ::core::ffi::c_ulonglong = 18446744073709551615 as ::core::ffi::c_ulonglong;
pub const LZMA_VLI_MAX: ::core::ffi::c_ulonglong =
    UINT64_MAX.wrapping_div(2 as ::core::ffi::c_ulonglong);
pub const LZMA_VLI_BYTES_MAX: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn lzma_vli_encode(
    mut vli: lzma_vli,
    mut vli_pos: *mut size_t,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    let mut vli_pos_internal: size_t = 0 as size_t;
    if vli_pos.is_null() {
        vli_pos = &raw mut vli_pos_internal;
        if *out_pos >= out_size {
            return LZMA_PROG_ERROR;
        }
    } else if *out_pos >= out_size {
        return LZMA_BUF_ERROR;
    }
    if *vli_pos >= LZMA_VLI_BYTES_MAX as size_t || vli > LZMA_VLI_MAX as lzma_vli {
        return LZMA_PROG_ERROR;
    }
    vli >>= (*vli_pos).wrapping_mul(7 as size_t);
    while vli >= 0x80 as lzma_vli {
        *vli_pos = (*vli_pos).wrapping_add(1);
        *out.offset(*out_pos as isize) =
            (vli as uint8_t as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as uint8_t;
        vli >>= 7 as ::core::ffi::c_int;
        *out_pos = (*out_pos).wrapping_add(1);
        if *out_pos == out_size {
            return (if vli_pos == &raw mut vli_pos_internal {
                LZMA_PROG_ERROR as ::core::ffi::c_int
            } else {
                LZMA_OK as ::core::ffi::c_int
            }) as lzma_ret;
        }
    }
    *out.offset(*out_pos as isize) = vli as uint8_t;
    *out_pos = (*out_pos).wrapping_add(1);
    *vli_pos = (*vli_pos).wrapping_add(1);
    return (if vli_pos == &raw mut vli_pos_internal {
        LZMA_OK as ::core::ffi::c_int
    } else {
        LZMA_STREAM_END as ::core::ffi::c_int
    }) as lzma_ret;
}
