use crate::common::filter_flags_encoder::{lzma_filter_flags_encode, lzma_filter_flags_size};
use crate::types::*;
pub unsafe fn lzma_block_header_size(block: *mut lzma_block) -> lzma_ret {
    if (*block).version > 1 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut size: u32 = (1 + 1 + 4) as u32;
    if (*block).compressed_size != LZMA_VLI_UNKNOWN {
        let add: u32 = lzma_vli_size((*block).compressed_size) as u32;
        if add == 0 || (*block).compressed_size == 0 {
            return LZMA_PROG_ERROR;
        }
        size += add;
    }
    if (*block).uncompressed_size != LZMA_VLI_UNKNOWN {
        let add_0: u32 = lzma_vli_size((*block).uncompressed_size) as u32;
        if add_0 == 0 {
            return LZMA_PROG_ERROR;
        }
        size += add_0;
    }
    if (*block).filters.is_null() || (*(*block).filters).id == LZMA_VLI_UNKNOWN {
        return LZMA_PROG_ERROR;
    }
    let mut i: size_t = 0;
    while (*(*block).filters.offset(i as isize)).id != LZMA_VLI_UNKNOWN {
        if i == LZMA_FILTERS_MAX as size_t {
            return LZMA_PROG_ERROR;
        }
        let mut add_1: u32 = 0;
        let ret_: lzma_ret = lzma_filter_flags_size(
            ::core::ptr::addr_of_mut!(add_1),
            (*block).filters.offset(i as isize),
        );
        if ret_ != LZMA_OK {
            return ret_;
        }
        size += add_1;
        i += 1;
    }
    (*block).header_size = (size + 3) & !(3);
    LZMA_OK
}
pub unsafe fn lzma_block_header_encode(block: *const lzma_block, out: *mut u8) -> lzma_ret {
    if lzma_block_unpadded_size(block) == 0
        || !((*block).uncompressed_size <= LZMA_VLI_MAX
            || (*block).uncompressed_size == LZMA_VLI_UNKNOWN)
    {
        return LZMA_PROG_ERROR;
    }
    let out_size: size_t = ((*block).header_size - 4) as size_t;
    *out = (out_size / 4) as u8;
    *out.offset(1) = 0;
    let mut out_pos: size_t = 2;
    if (*block).compressed_size != LZMA_VLI_UNKNOWN {
        let ret_: lzma_ret = lzma_vli_encode(
            (*block).compressed_size,
            core::ptr::null_mut(),
            out,
            ::core::ptr::addr_of_mut!(out_pos),
            out_size,
        );
        if ret_ != LZMA_OK {
            return ret_;
        }
        *out.offset(1) |= 0x40;
    }
    if (*block).uncompressed_size != LZMA_VLI_UNKNOWN {
        let ret__0: lzma_ret = lzma_vli_encode(
            (*block).uncompressed_size,
            core::ptr::null_mut(),
            out,
            ::core::ptr::addr_of_mut!(out_pos),
            out_size,
        );
        if ret__0 != LZMA_OK {
            return ret__0;
        }
        *out.offset(1) |= 0x80;
    }
    if (*block).filters.is_null() || (*(*block).filters).id == LZMA_VLI_UNKNOWN {
        return LZMA_PROG_ERROR;
    }
    let mut filter_count: size_t = 0;
    loop {
        if filter_count == LZMA_FILTERS_MAX as size_t {
            return LZMA_PROG_ERROR;
        }
        let ret__1: lzma_ret = lzma_filter_flags_encode(
            (*block).filters.offset(filter_count as isize),
            out,
            ::core::ptr::addr_of_mut!(out_pos),
            out_size,
        );
        if ret__1 != LZMA_OK {
            return ret__1;
        }
        filter_count += 1;
        if (*(*block).filters.offset(filter_count as isize)).id == LZMA_VLI_UNKNOWN {
            break;
        }
    }
    *out.offset(1) |= (filter_count - 1) as u8;
    core::ptr::write_bytes(
        out.offset(out_pos as isize) as *mut u8,
        0 as u8,
        out_size - out_pos,
    );
    write32le(out.offset(out_size as isize), lzma_crc32(out, out_size, 0));
    LZMA_OK
}
