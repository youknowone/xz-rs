use crate::types::*;
extern "C" {
    fn lzma_crc32(buf: *const u8, size: size_t, crc: u32) -> u32;
    fn lzma_crc64(buf: *const u8, size: size_t, crc: u64) -> u64;
    fn lzma_sha256_init(check: *mut lzma_check_state);
    fn lzma_sha256_update(buf: *const u8, size: size_t, check: *mut lzma_check_state);
    fn lzma_sha256_finish(check: *mut lzma_check_state);
}
#[no_mangle]
pub extern "C" fn lzma_check_is_supported(type_0: lzma_check) -> lzma_bool {
    if type_0 > LZMA_CHECK_ID_MAX {
        return false as lzma_bool;
    }
    static available_checks: [lzma_bool; 16] = [
        true as lzma_bool,
        true as lzma_bool,
        false as lzma_bool,
        false as lzma_bool,
        true as lzma_bool,
        false as lzma_bool,
        false as lzma_bool,
        false as lzma_bool,
        false as lzma_bool,
        false as lzma_bool,
        true as lzma_bool,
        false as lzma_bool,
        false as lzma_bool,
        false as lzma_bool,
        false as lzma_bool,
        false as lzma_bool,
    ];
    return available_checks[type_0 as usize];
}
#[no_mangle]
pub extern "C" fn lzma_check_size(type_0: lzma_check) -> u32 {
    if type_0 > LZMA_CHECK_ID_MAX {
        return UINT32_MAX;
    }
    static check_sizes: [u8; 16] = [
        0, 4 as u8, 4 as u8, 4 as u8, 8 as u8, 8 as u8, 8 as u8, 16 as u8, 16 as u8, 16 as u8,
        32 as u8, 32 as u8, 32 as u8, 64 as u8, 64 as u8, 64 as u8,
    ];
    return check_sizes[type_0 as usize] as u32;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_check_init(check: *mut lzma_check_state, type_0: lzma_check) {
    match type_0 {
        1 => {
            (*check).state.crc32 = 0;
        }
        4 => {
            (*check).state.crc64 = 0;
        }
        10 => {
            lzma_sha256_init(check);
        }
        0 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_check_update(
    check: *mut lzma_check_state,
    type_0: lzma_check,
    buf: *const u8,
    size: size_t,
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
pub unsafe extern "C" fn lzma_check_finish(check: *mut lzma_check_state, type_0: lzma_check) {
    match type_0 {
        1 => {
            (*check).buffer.u32_0[0] = (*check).state.crc32;
        }
        4 => {
            (*check).buffer.u64_0[0] = (*check).state.crc64;
        }
        10 => {
            lzma_sha256_finish(check);
        }
        _ => {}
    };
}
