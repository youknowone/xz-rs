pub type uint32_t = u32;
pub type uint64_t = u64;
pub type lzma_vli = uint64_t;
pub const UINT64_MAX: ::core::ffi::c_ulonglong = 18446744073709551615 as ::core::ffi::c_ulonglong;
pub const LZMA_VLI_MAX: ::core::ffi::c_ulonglong =
    UINT64_MAX.wrapping_div(2 as ::core::ffi::c_ulonglong);
#[no_mangle]
pub unsafe extern "C" fn lzma_vli_size(mut vli: lzma_vli) -> uint32_t {
    if vli > LZMA_VLI_MAX as lzma_vli {
        return 0 as uint32_t;
    }
    let mut i: uint32_t = 0 as uint32_t;
    loop {
        vli >>= 7 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
        if !(vli != 0 as lzma_vli) {
            break;
        }
    }
    return i;
}
