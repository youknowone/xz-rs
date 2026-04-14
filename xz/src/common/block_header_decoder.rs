use crate::common::filter_flags_decoder::lzma_filter_flags_decode;
use crate::types::*;
pub unsafe fn lzma_block_header_decode(
    block: *mut lzma_block,
    allocator: *const lzma_allocator,
    input: *const u8,
) -> lzma_ret {
    if block.is_null() || (*block).filters.is_null() || input.is_null() {
        return LZMA_PROG_ERROR;
    }
    let mut i: size_t = 0;
    while i <= LZMA_FILTERS_MAX as size_t {
        (*(*block).filters.offset(i as isize)).id = LZMA_VLI_UNKNOWN;
        (*(*block).filters.offset(i as isize)).options = core::ptr::null_mut();
        i += 1;
    }
    if (*block).version > 1 {
        (*block).version = 1;
    }
    (*block).ignore_check = false as lzma_bool;
    if (*input as u32 + 1) * 4 != (*block).header_size || (*block).check > LZMA_CHECK_ID_MAX {
        return LZMA_PROG_ERROR;
    }
    let in_size: size_t = ((*block).header_size - 4) as size_t;
    if lzma_crc32(input, in_size, 0) != read32le(&*input.add(in_size).cast::<[u8; 4]>()) {
        return LZMA_DATA_ERROR;
    }
    if *input.offset(1) & 0x3c != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut in_pos: size_t = 2;
    if *input.offset(1) & 0x40 != 0 {
        let ret: lzma_ret = lzma_vli_decode(
            ::core::ptr::addr_of_mut!((*block).compressed_size),
            core::ptr::null_mut(),
            input,
            ::core::ptr::addr_of_mut!(in_pos),
            in_size,
        );
        if ret != LZMA_OK {
            return ret;
        }
        if lzma_block_unpadded_size(block) == 0 {
            return LZMA_DATA_ERROR;
        }
    } else {
        (*block).compressed_size = LZMA_VLI_UNKNOWN;
    }
    if *input.offset(1) & 0x80 != 0 {
        let ret: lzma_ret = lzma_vli_decode(
            ::core::ptr::addr_of_mut!((*block).uncompressed_size),
            core::ptr::null_mut(),
            input,
            ::core::ptr::addr_of_mut!(in_pos),
            in_size,
        );
        if ret != LZMA_OK {
            return ret;
        }
    } else {
        (*block).uncompressed_size = LZMA_VLI_UNKNOWN;
    }
    let filter_count: size_t = ((u32::from(*input.offset(1)) & 3) + 1) as size_t;
    let mut i_0: size_t = 0;
    while i_0 < filter_count {
        let ret: lzma_ret = lzma_filter_flags_decode(
            (*block).filters.offset(i_0 as isize) as *mut lzma_filter,
            allocator,
            input,
            ::core::ptr::addr_of_mut!(in_pos),
            in_size,
        );
        if ret != LZMA_OK {
            lzma_filters_free((*block).filters, allocator);
            return ret;
        }
        i_0 += 1;
    }
    while in_pos < in_size {
        if *input.offset(in_pos as isize) != 0 {
            lzma_filters_free((*block).filters, allocator);
            return LZMA_OPTIONS_ERROR;
        }
        in_pos += 1;
    }
    LZMA_OK
}
