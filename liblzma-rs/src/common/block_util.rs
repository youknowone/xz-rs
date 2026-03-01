use crate::types::*;
use core::ffi::c_ulonglong;
extern "C" {
    fn lzma_check_size(check: lzma_check) -> u32;
}
pub const LZMA_BLOCK_HEADER_SIZE_MIN: u32 = 8;
pub const UNPADDED_SIZE_MAX: c_ulonglong = LZMA_VLI_MAX & !3;
#[inline]
extern "C" fn vli_ceil4(vli: lzma_vli) -> lzma_vli {
    return vli.wrapping_add(3) & !(3);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_compressed_size(
    block: *mut lzma_block,
    unpadded_size: lzma_vli,
) -> lzma_ret {
    if lzma_block_unpadded_size(block) == 0 {
        return LZMA_PROG_ERROR;
    }
    let container_size: u32 = (*block)
        .header_size
        .wrapping_add(lzma_check_size((*block).check) as u32);
    if unpadded_size <= container_size as lzma_vli {
        return LZMA_DATA_ERROR;
    }
    let compressed_size: lzma_vli = unpadded_size.wrapping_sub(container_size as lzma_vli);
    if (*block).compressed_size != LZMA_VLI_UNKNOWN && (*block).compressed_size != compressed_size {
        return LZMA_DATA_ERROR;
    }
    (*block).compressed_size = compressed_size;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_unpadded_size(block: *const lzma_block) -> lzma_vli {
    if block.is_null()
        || (*block).version > 1
        || (*block).header_size < LZMA_BLOCK_HEADER_SIZE_MIN
        || (*block).header_size > LZMA_BLOCK_HEADER_SIZE_MAX
        || (*block).header_size & 3 != 0
        || !((*block).compressed_size <= LZMA_VLI_MAX
            || (*block).compressed_size == LZMA_VLI_UNKNOWN)
        || (*block).compressed_size == 0
        || (*block).check > LZMA_CHECK_ID_MAX
    {
        return 0;
    }
    if (*block).compressed_size == LZMA_VLI_UNKNOWN {
        return LZMA_VLI_UNKNOWN;
    }
    let unpadded_size: lzma_vli = (*block)
        .compressed_size
        .wrapping_add((*block).header_size as lzma_vli)
        .wrapping_add(lzma_check_size((*block).check) as lzma_vli);
    if unpadded_size > UNPADDED_SIZE_MAX as lzma_vli {
        return 0;
    }
    return unpadded_size;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_total_size(block: *const lzma_block) -> lzma_vli {
    let mut unpadded_size: lzma_vli = lzma_block_unpadded_size(block);
    if unpadded_size != LZMA_VLI_UNKNOWN {
        unpadded_size = vli_ceil4(unpadded_size);
    }
    return unpadded_size;
}
