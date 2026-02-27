use crate::types::*;
use core::ffi::{c_int, c_uchar, c_uint};
extern "C" {
    fn lzma_crc32(buf: *const u8, size: size_t, crc: u32) -> u32;
    fn lzma_crc64(buf: *const u8, size: size_t, crc: u64) -> u64;
    fn lzma_sha256_init(check: *mut lzma_check_state);
    fn lzma_sha256_update(buf: *const u8, size: size_t, check: *mut lzma_check_state);
    fn lzma_sha256_finish(check: *mut lzma_check_state);
}
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_sha256_state {
    pub state: [u32; 8],
    pub size: u64,
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
    pub crc32: u32,
    pub crc64: u64,
    pub sha256: lzma_sha256_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub u8_0: [u8; 64],
    pub u32_0: [u32; 16],
    pub u64_0: [u64; 8],
}
pub const UINT32_MAX: c_uint = 4294967295;
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
pub const LZMA_CHECK_ID_MAX: lzma_check = 15;
#[no_mangle]
pub unsafe extern "C" fn lzma_check_is_supported(mut type_0: lzma_check) -> lzma_bool {
    if type_0 > LZMA_CHECK_ID_MAX {
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
    return available_checks[type_0 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn lzma_check_size(mut type_0: lzma_check) -> u32 {
    if type_0 > LZMA_CHECK_ID_MAX {
        return UINT32_MAX as u32;
    }
    static mut check_sizes: [u8; 16] = [
        0 as u8, 4 as u8, 4 as u8, 4 as u8, 8 as u8, 8 as u8, 8 as u8, 16 as u8, 16 as u8,
        16 as u8, 32 as u8, 32 as u8, 32 as u8, 64 as u8, 64 as u8, 64 as u8,
    ];
    return check_sizes[type_0 as usize] as u32;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_check_init(mut check: *mut lzma_check_state, mut type_0: lzma_check) {
    match type_0 {
        1 => {
            (*check).state.crc32 = 0 as u32;
        }
        4 => {
            (*check).state.crc64 = 0 as u64;
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
    mut buf: *const u8,
    mut size: size_t,
) {
    match type_0 {
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
    match type_0 {
        1 => {
            (*check).buffer.u32_0[0 as usize] = (*check).state.crc32;
        }
        4 => {
            (*check).buffer.u64_0[0 as usize] = (*check).state.crc64;
        }
        10 => {
            lzma_sha256_finish(check);
        }
        _ => {}
    };
}
