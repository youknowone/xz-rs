use crate::types::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream_flags {
    pub version: u32,
    pub backward_size: lzma_vli,
    pub check: lzma_check,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_bool1: lzma_bool,
    pub reserved_bool2: lzma_bool,
    pub reserved_bool3: lzma_bool,
    pub reserved_bool4: lzma_bool,
    pub reserved_bool5: lzma_bool,
    pub reserved_bool6: lzma_bool,
    pub reserved_bool7: lzma_bool,
    pub reserved_bool8: lzma_bool,
    pub reserved_int1: u32,
    pub reserved_int2: u32,
}
#[inline]
extern "C" fn is_backward_size_valid(options: *const lzma_stream_flags) -> bool {
    return unsafe {
        (*options).backward_size >= LZMA_BACKWARD_SIZE_MIN as lzma_vli
            && (*options).backward_size <= LZMA_BACKWARD_SIZE_MAX as lzma_vli
            && (*options).backward_size & 3 as lzma_vli == 0 as lzma_vli
    };
}
#[no_mangle]
pub static mut lzma_header_magic: [u8; 6] = [
    0xfd as u8, 0x37 as u8, 0x7a as u8, 0x58 as u8, 0x5a as u8, 0,
];
#[no_mangle]
pub static mut lzma_footer_magic: [u8; 2] = [0x59 as u8, 0x5a as u8];
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_flags_compare(
    a: *const lzma_stream_flags,
    b: *const lzma_stream_flags,
) -> lzma_ret {
    if (*a).version != 0 || (*b).version != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    if (*a).check > LZMA_CHECK_ID_MAX || (*b).check > LZMA_CHECK_ID_MAX {
        return LZMA_PROG_ERROR;
    }
    if (*a).check != (*b).check {
        return LZMA_DATA_ERROR;
    }
    if (*a).backward_size != LZMA_VLI_UNKNOWN as lzma_vli
        && (*b).backward_size != LZMA_VLI_UNKNOWN as lzma_vli
    {
        if !is_backward_size_valid(a) || !is_backward_size_valid(b) {
            return LZMA_PROG_ERROR;
        }
        if (*a).backward_size != (*b).backward_size {
            return LZMA_DATA_ERROR;
        }
    }
    return LZMA_OK;
}
