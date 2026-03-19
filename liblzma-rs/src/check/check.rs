use crate::check::crc64_fast::lzma_crc64;
use crate::check::sha256::{lzma_sha256_finish, lzma_sha256_init, lzma_sha256_update};
use crate::types::*;
pub fn lzma_check_is_supported(type_0: lzma_check) -> lzma_bool {
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
    available_checks[type_0 as usize]
}
pub fn lzma_check_size(type_0: lzma_check) -> u32 {
    if type_0 > LZMA_CHECK_ID_MAX {
        return UINT32_MAX;
    }
    static check_sizes: [u8; 16] = [
        0, 4 as u8, 4 as u8, 4 as u8, 8 as u8, 8 as u8, 8 as u8, 16 as u8, 16 as u8, 16 as u8,
        32 as u8, 32 as u8, 32 as u8, 64 as u8, 64 as u8, 64 as u8,
    ];
    check_sizes[type_0 as usize] as u32
}
pub unsafe fn lzma_check_init(check: *mut lzma_check_state, type_0: lzma_check) {
    if check.is_null() {
        return;
    }

    match type_0 {
        LZMA_CHECK_CRC32 => {
            (*check).state.crc32 = 0;
        }
        LZMA_CHECK_CRC64 => {
            (*check).state.crc64 = 0;
        }
        LZMA_CHECK_SHA256 => {
            lzma_sha256_init(check);
        }
        LZMA_CHECK_NONE | _ => {}
    };
}
pub unsafe fn lzma_check_update(
    check: *mut lzma_check_state,
    type_0: lzma_check,
    buf: *const u8,
    size: size_t,
) {
    if check.is_null() {
        return;
    }

    match type_0 {
        LZMA_CHECK_CRC32 => {
            (*check).state.crc32 = lzma_crc32(buf, size, (*check).state.crc32);
        }
        LZMA_CHECK_CRC64 => {
            (*check).state.crc64 = lzma_crc64(buf, size, (*check).state.crc64);
        }
        LZMA_CHECK_SHA256 => {
            lzma_sha256_update(buf, size, check);
        }
        _ => {}
    };
}
pub unsafe fn lzma_check_finish(check: *mut lzma_check_state, type_0: lzma_check) {
    if check.is_null() {
        return;
    }

    match type_0 {
        LZMA_CHECK_CRC32 => {
            (*check).buffer.u32_0[0] = (*check).state.crc32;
        }
        LZMA_CHECK_CRC64 => {
            (*check).buffer.u64_0[0] = (*check).state.crc64;
        }
        LZMA_CHECK_SHA256 => {
            lzma_sha256_finish(check);
        }
        _ => {}
    };
}
