use crate::types::*;
pub unsafe extern "C" fn lzma_raw_buffer_encode(
    filters: *const lzma_filter,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    if in_0.is_null() && in_size != 0 || out.is_null() || out_pos.is_null() || *out_pos > out_size {
        return LZMA_PROG_ERROR;
    }
    let mut next: lzma_next_coder = lzma_next_coder_s {
        coder: core::ptr::null_mut(),
        id: LZMA_VLI_UNKNOWN,
        init: 0,
        code: None,
        end: None,
        get_progress: None,
        get_check: None,
        memconfig: None,
        update: None,
        set_out_limit: None,
    };
    let ret_: lzma_ret = lzma_raw_encoder_init(&raw mut next, allocator, filters);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let out_start: size_t = *out_pos;
    let mut in_pos: size_t = 0;
    let mut ret: lzma_ret = next.code.unwrap()(
        next.coder,
        allocator,
        in_0,
        &raw mut in_pos,
        in_size,
        out,
        out_pos,
        out_size,
        LZMA_FINISH,
    );
    lzma_next_end(&raw mut next, allocator);
    if ret == LZMA_STREAM_END {
        ret = LZMA_OK;
    } else {
        if ret == LZMA_OK {
            ret = LZMA_BUF_ERROR;
        }
        *out_pos = out_start;
    }
    ret
}
