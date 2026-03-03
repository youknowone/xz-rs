use crate::types::*;
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
    if *vli_pos >= LZMA_VLI_BYTES_MAX as size_t || vli > LZMA_VLI_MAX {
        return LZMA_PROG_ERROR;
    }
    vli >>= (*vli_pos).wrapping_mul(7);
    while vli >= 0x80 {
        *vli_pos = (*vli_pos).wrapping_add(1);
        *out.offset(*out_pos as isize) = vli as u8 | 0x80;
        vli >>= 7;
        *out_pos = (*out_pos).wrapping_add(1);
        if *out_pos == out_size {
            return if vli_pos == &raw mut vli_pos_internal {
                LZMA_PROG_ERROR
            } else {
                LZMA_OK
            };
        }
    }
    *out.offset(*out_pos as isize) = vli as u8;
    *out_pos = (*out_pos).wrapping_add(1);
    *vli_pos = (*vli_pos).wrapping_add(1);
    if vli_pos == &raw mut vli_pos_internal {
        LZMA_OK
    } else {
        LZMA_STREAM_END
    }
}
