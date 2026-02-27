use crate::types::*;
use core::ffi::{c_int, c_uchar, c_uint, c_ulonglong, c_void};
extern "C" {
    fn lzma_check_size(check: lzma_check) -> u32;
}
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut c_void,
}
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_block {
    pub version: u32,
    pub header_size: u32,
    pub check: lzma_check,
    pub compressed_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub filters: *mut lzma_filter,
    pub raw_check: [u8; 64],
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
    pub reserved_ptr3: *mut c_void,
    pub reserved_int1: u32,
    pub reserved_int2: u32,
    pub reserved_int3: lzma_vli,
    pub reserved_int4: lzma_vli,
    pub reserved_int5: lzma_vli,
    pub reserved_int6: lzma_vli,
    pub reserved_int7: lzma_vli,
    pub reserved_int8: lzma_vli,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub ignore_check: lzma_bool,
    pub reserved_bool2: lzma_bool,
    pub reserved_bool3: lzma_bool,
    pub reserved_bool4: lzma_bool,
    pub reserved_bool5: lzma_bool,
    pub reserved_bool6: lzma_bool,
    pub reserved_bool7: lzma_bool,
    pub reserved_bool8: lzma_bool,
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = 18446744073709551615;
pub const LZMA_VLI_MAX: c_ulonglong = UINT64_MAX.wrapping_div(2);
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_CHECK_ID_MAX: lzma_check = 15;
pub const LZMA_BLOCK_HEADER_SIZE_MIN: c_int = 8 as c_int;
pub const LZMA_BLOCK_HEADER_SIZE_MAX: c_int = 1024 as c_int;
pub const UNPADDED_SIZE_MAX: c_ulonglong = LZMA_VLI_MAX & !3;
#[inline]
unsafe extern "C" fn vli_ceil4(mut vli: lzma_vli) -> lzma_vli {
    return vli.wrapping_add(3 as lzma_vli) & !(3 as lzma_vli);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_compressed_size(
    mut block: *mut lzma_block,
    mut unpadded_size: lzma_vli,
) -> lzma_ret {
    if lzma_block_unpadded_size(block) == 0 as lzma_vli {
        return LZMA_PROG_ERROR;
    }
    let container_size: u32 = (*block)
        .header_size
        .wrapping_add(lzma_check_size((*block).check) as u32);
    if unpadded_size <= container_size as lzma_vli {
        return LZMA_DATA_ERROR;
    }
    let compressed_size: lzma_vli = unpadded_size.wrapping_sub(container_size as lzma_vli);
    if (*block).compressed_size != LZMA_VLI_UNKNOWN as lzma_vli
        && (*block).compressed_size != compressed_size
    {
        return LZMA_DATA_ERROR;
    }
    (*block).compressed_size = compressed_size;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_unpadded_size(mut block: *const lzma_block) -> lzma_vli {
    if block.is_null()
        || (*block).version > 1 as u32
        || (*block).header_size < LZMA_BLOCK_HEADER_SIZE_MIN as u32
        || (*block).header_size > LZMA_BLOCK_HEADER_SIZE_MAX as u32
        || (*block).header_size & 3 as u32 != 0
        || !((*block).compressed_size <= LZMA_VLI_MAX as lzma_vli
            || (*block).compressed_size == LZMA_VLI_UNKNOWN as lzma_vli)
        || (*block).compressed_size == 0 as lzma_vli
        || (*block).check > LZMA_CHECK_ID_MAX
    {
        return 0 as lzma_vli;
    }
    if (*block).compressed_size == LZMA_VLI_UNKNOWN as lzma_vli {
        return LZMA_VLI_UNKNOWN as lzma_vli;
    }
    let unpadded_size: lzma_vli = (*block)
        .compressed_size
        .wrapping_add((*block).header_size as lzma_vli)
        .wrapping_add(lzma_check_size((*block).check) as lzma_vli);
    if unpadded_size > UNPADDED_SIZE_MAX as lzma_vli {
        return 0 as lzma_vli;
    }
    return unpadded_size;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_total_size(mut block: *const lzma_block) -> lzma_vli {
    let mut unpadded_size: lzma_vli = lzma_block_unpadded_size(block);
    if unpadded_size != LZMA_VLI_UNKNOWN as lzma_vli {
        unpadded_size = vli_ceil4(unpadded_size);
    }
    return unpadded_size;
}
