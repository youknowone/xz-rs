extern "C" {
    fn lzma_crc32(buf: *const uint8_t, size: size_t, crc: uint32_t) -> uint32_t;
    fn lzma_crc64(buf: *const uint8_t, size: size_t, crc: uint64_t) -> uint64_t;
    fn lzma_sha256_init(check: *mut lzma_check_state);
    fn lzma_sha256_update(buf: *const uint8_t, size: size_t, check: *mut lzma_check_state);
    fn lzma_sha256_finish(check: *mut lzma_check_state);
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type lzma_bool = ::core::ffi::c_uchar;
pub type lzma_check = ::core::ffi::c_uint;
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_sha256_state {
    pub state: [uint32_t; 8],
    pub size: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_check_state {
    pub buffer: C2RustUnnamed_0,
    pub state: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub crc32: uint32_t,
    pub crc64: uint64_t,
    pub sha256: lzma_sha256_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub u8_0: [uint8_t; 64],
    pub u32_0: [uint32_t; 16],
    pub u64_0: [uint64_t; 8],
}
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LZMA_CHECK_ID_MAX: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn lzma_check_is_supported(mut type_0: lzma_check) -> lzma_bool {
    if type_0 as ::core::ffi::c_uint > LZMA_CHECK_ID_MAX as ::core::ffi::c_uint {
        return false_0 as lzma_bool;
    }
    static mut available_checks: [lzma_bool; 16] = [
        true_0 as lzma_bool,
        true_0 as lzma_bool,
        false_0 as lzma_bool,
        false_0 as lzma_bool,
        true_0 as lzma_bool,
        false_0 as lzma_bool,
        false_0 as lzma_bool,
        false_0 as lzma_bool,
        false_0 as lzma_bool,
        false_0 as lzma_bool,
        true_0 as lzma_bool,
        false_0 as lzma_bool,
        false_0 as lzma_bool,
        false_0 as lzma_bool,
        false_0 as lzma_bool,
        false_0 as lzma_bool,
    ];
    return available_checks[type_0 as ::core::ffi::c_uint as usize];
}
#[no_mangle]
pub unsafe extern "C" fn lzma_check_size(mut type_0: lzma_check) -> uint32_t {
    if type_0 as ::core::ffi::c_uint > LZMA_CHECK_ID_MAX as ::core::ffi::c_uint {
        return UINT32_MAX as uint32_t;
    }
    static mut check_sizes: [uint8_t; 16] = [
        0 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        64 as ::core::ffi::c_int as uint8_t,
        64 as ::core::ffi::c_int as uint8_t,
        64 as ::core::ffi::c_int as uint8_t,
    ];
    return check_sizes[type_0 as ::core::ffi::c_uint as usize] as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_check_init(mut check: *mut lzma_check_state, mut type_0: lzma_check) {
    match type_0 as ::core::ffi::c_uint {
        1 => {
            (*check).state.crc32 = 0 as uint32_t;
        }
        4 => {
            (*check).state.crc64 = 0 as uint64_t;
        }
        10 => {
            lzma_sha256_init(check);
        }
        0 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_check_update(
    mut check: *mut lzma_check_state,
    mut type_0: lzma_check,
    mut buf: *const uint8_t,
    mut size: size_t,
) {
    match type_0 as ::core::ffi::c_uint {
        1 => {
            (*check).state.crc32 = lzma_crc32(buf, size, (*check).state.crc32);
        }
        4 => {
            (*check).state.crc64 = lzma_crc64(buf, size, (*check).state.crc64);
        }
        10 => {
            lzma_sha256_update(buf, size, check);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_check_finish(
    mut check: *mut lzma_check_state,
    mut type_0: lzma_check,
) {
    match type_0 as ::core::ffi::c_uint {
        1 => {
            (*check).buffer.u32_0[0 as ::core::ffi::c_int as usize] = (*check).state.crc32;
        }
        4 => {
            (*check).buffer.u64_0[0 as ::core::ffi::c_int as usize] = (*check).state.crc64;
        }
        10 => {
            lzma_sha256_finish(check);
        }
        _ => {}
    };
}
