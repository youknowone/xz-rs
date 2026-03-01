use crate::types::*;
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
        *vli = 0;
        if *in_pos >= in_size {
            return LZMA_DATA_ERROR;
        }
    } else {
        if *vli_pos == 0 {
            *vli = 0;
        }
        if *vli_pos >= LZMA_VLI_BYTES_MAX as size_t || *vli >> (*vli_pos).wrapping_mul(7) != 0 {
            return LZMA_PROG_ERROR;
        }
        if *in_pos >= in_size {
            return LZMA_BUF_ERROR;
        }
    }
    loop {
        let byte: u8 = *in_0.offset(*in_pos as isize);
        *in_pos = (*in_pos).wrapping_add(1);
        *vli = (*vli).wrapping_add(((byte & 0x7f) as lzma_vli) << (*vli_pos).wrapping_mul(7));
        *vli_pos = (*vli_pos).wrapping_add(1);
        if byte & 0x80 == 0 {
            if byte == 0 && *vli_pos > 1 {
                return LZMA_DATA_ERROR;
            }
            return if vli_pos == &raw mut vli_pos_internal {
                LZMA_OK
            } else {
                LZMA_STREAM_END
            };
        }
        if *vli_pos == LZMA_VLI_BYTES_MAX as size_t {
            return LZMA_DATA_ERROR;
        }
        if *in_pos >= in_size {
            break;
        }
    }
    if vli_pos == &raw mut vli_pos_internal {
        LZMA_DATA_ERROR
    } else {
        LZMA_OK
    }
}
