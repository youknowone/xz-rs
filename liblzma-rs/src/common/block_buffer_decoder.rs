use crate::types::*;
use core::ffi::c_void;
extern "C" {
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_block_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        block: *mut lzma_block,
    ) -> lzma_ret;
}
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
#[no_mangle]
pub unsafe extern "C" fn lzma_block_buffer_decode(
    block: *mut lzma_block,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    if in_pos.is_null()
        || in_0.is_null() && *in_pos != in_size
        || *in_pos > in_size
        || out_pos.is_null()
        || out.is_null() && *out_pos != out_size
        || *out_pos > out_size
    {
        return LZMA_PROG_ERROR;
    }
    let mut block_decoder: lzma_next_coder = lzma_next_coder_s {
        coder: core::ptr::null_mut(),
        id: LZMA_VLI_UNKNOWN as lzma_vli,
        init: 0,
        code: None,
        end: None,
        get_progress: None,
        get_check: None,
        memconfig: None,
        update: None,
        set_out_limit: None,
    };
    let mut ret: lzma_ret = lzma_block_decoder_init(&raw mut block_decoder, allocator, block);
    if ret == LZMA_OK {
        let in_start: size_t = *in_pos;
        let out_start: size_t = *out_pos;
        ret = block_decoder.code.expect("non-null function pointer")(
            block_decoder.coder,
            allocator,
            in_0,
            in_pos,
            in_size,
            out,
            out_pos,
            out_size,
            LZMA_FINISH,
        );
        if ret == LZMA_STREAM_END {
            ret = LZMA_OK;
        } else {
            if ret == LZMA_OK {
                if *in_pos == in_size {
                    ret = LZMA_DATA_ERROR;
                } else {
                    ret = LZMA_BUF_ERROR;
                }
            }
            *in_pos = in_start;
            *out_pos = out_start;
        }
    }
    lzma_next_end(&raw mut block_decoder, allocator);
    return ret;
}
