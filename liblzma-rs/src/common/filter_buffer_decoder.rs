use crate::types::*;
extern "C" {
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_raw_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        options: *const lzma_filter,
    ) -> lzma_ret;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_buffer_decode(
    filters: *const lzma_filter,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    if in_0.is_null()
        || in_pos.is_null()
        || *in_pos > in_size
        || out.is_null()
        || out_pos.is_null()
        || *out_pos > out_size
    {
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
    let ret_: lzma_ret = lzma_raw_decoder_init(&raw mut next, allocator, filters);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let in_start: size_t = *in_pos;
    let out_start: size_t = *out_pos;
    let mut ret: lzma_ret = next.code.unwrap()(
        next.coder,
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
            if *in_pos != in_size {
                ret = LZMA_BUF_ERROR;
            } else if *out_pos != out_size {
                ret = LZMA_DATA_ERROR;
            } else {
                let mut tmp: [u8; 1] = [0; 1];
                let mut tmp_pos: size_t = 0;
                next.code.unwrap()(
                    next.coder,
                    allocator,
                    in_0,
                    in_pos,
                    in_size,
                    &raw mut tmp as *mut u8,
                    &raw mut tmp_pos,
                    1,
                    LZMA_FINISH,
                );
                if tmp_pos == 1 {
                    ret = LZMA_BUF_ERROR;
                } else {
                    ret = LZMA_DATA_ERROR;
                }
            }
        }
        *in_pos = in_start;
        *out_pos = out_start;
    }
    lzma_next_end(&raw mut next, allocator);
    ret
}
