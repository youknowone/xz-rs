use crate::types::*;
use core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn lzma_vli_encode(
    mut vli: lzma_vli,
    mut vli_pos: *mut size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    let mut vli_pos_internal: size_t = 0;
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
    vli >>= (*vli_pos).wrapping_mul(7);
    while vli >= 0x80 as lzma_vli {
        *vli_pos = (*vli_pos).wrapping_add(1);
        *out.offset(*out_pos as isize) = (vli as u8 as c_int | 0x80 as c_int) as u8;
        vli >>= 7 as c_int;
        *out_pos = (*out_pos).wrapping_add(1);
        if *out_pos == out_size {
            return (if vli_pos == &raw mut vli_pos_internal {
                LZMA_PROG_ERROR as c_int
            } else {
                LZMA_OK as c_int
            }) as lzma_ret;
        }
    }
    *out.offset(*out_pos as isize) = vli as u8;
    *out_pos = (*out_pos).wrapping_add(1);
    *vli_pos = (*vli_pos).wrapping_add(1);
    return (if vli_pos == &raw mut vli_pos_internal {
        LZMA_OK as c_int
    } else {
        LZMA_STREAM_END as c_int
    }) as lzma_ret;
}
