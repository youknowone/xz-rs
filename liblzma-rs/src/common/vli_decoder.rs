use crate::types::*;
use core::ffi::c_int;
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
pub const LZMA_VLI_BYTES_MAX: c_int = 9;
#[no_mangle]
pub unsafe extern "C" fn lzma_vli_decode(
    vli: *mut lzma_vli,
    mut vli_pos: *mut size_t,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    let mut vli_pos_internal: size_t = 0;
    if vli_pos.is_null() {
        vli_pos = &raw mut vli_pos_internal;
        *vli = 0 as lzma_vli;
        if *in_pos >= in_size {
            return LZMA_DATA_ERROR;
        }
    } else {
        if *vli_pos == 0 {
            *vli = 0 as lzma_vli;
        }
        if *vli_pos >= LZMA_VLI_BYTES_MAX as size_t
            || *vli >> (*vli_pos).wrapping_mul(7) != 0 as lzma_vli
        {
            return LZMA_PROG_ERROR;
        }
        if *in_pos >= in_size {
            return LZMA_BUF_ERROR;
        }
    }
    loop {
        let byte: u8 = *in_0.offset(*in_pos as isize);
        *in_pos = (*in_pos).wrapping_add(1);
        *vli = (*vli).wrapping_add(
            ((byte as c_int & 0x7f as c_int) as lzma_vli) << (*vli_pos).wrapping_mul(7),
        );
        *vli_pos = (*vli_pos).wrapping_add(1);
        if byte as c_int & 0x80 as c_int == 0 as c_int {
            if byte as c_int == 0 as c_int && *vli_pos > 1 {
                return LZMA_DATA_ERROR;
            }
            return (if vli_pos == &raw mut vli_pos_internal {
                LZMA_OK as c_int
            } else {
                LZMA_STREAM_END as c_int
            }) as lzma_ret;
        }
        if *vli_pos == LZMA_VLI_BYTES_MAX as size_t {
            return LZMA_DATA_ERROR;
        }
        if !(*in_pos < in_size) {
            break;
        }
    }
    return (if vli_pos == &raw mut vli_pos_internal {
        LZMA_DATA_ERROR as c_int
    } else {
        LZMA_OK as c_int
    }) as lzma_ret;
}
