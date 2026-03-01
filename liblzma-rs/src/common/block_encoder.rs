use crate::types::*;
use core::ffi::{c_uint, c_ulonglong, c_void};
extern "C" {
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_check_is_supported(check: lzma_check) -> lzma_bool;
    fn lzma_check_size(check: lzma_check) -> u32;
    fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret;
    fn lzma_next_filter_update(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        reversed_filters: *const lzma_filter,
    ) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_bufcpy(
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> size_t;
    fn lzma_raw_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter,
    ) -> lzma_ret;
    fn lzma_check_init(check: *mut lzma_check_state, type_0: lzma_check);
    fn lzma_check_update(
        check: *mut lzma_check_state,
        type_0: lzma_check,
        buf: *const u8,
        size: size_t,
    );
    fn lzma_check_finish(check: *mut lzma_check_state, type_0: lzma_check);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_block_coder {
    pub next: lzma_next_coder,
    pub block: *mut lzma_block,
    pub sequence: C2RustUnnamed_2,
    pub compressed_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub pos: size_t,
    pub check: lzma_check_state,
}
pub type C2RustUnnamed_2 = c_uint;
pub const SEQ_CHECK: C2RustUnnamed_2 = 2;
pub const SEQ_PADDING: C2RustUnnamed_2 = 1;
pub const SEQ_CODE: C2RustUnnamed_2 = 0;
pub const LZMA_CHECK_SIZE_MAX: u32 = 64;
pub const COMPRESSED_SIZE_MAX: c_ulonglong = LZMA_VLI_MAX
    .wrapping_sub(LZMA_BLOCK_HEADER_SIZE_MAX as u64)
    .wrapping_sub(LZMA_CHECK_SIZE_MAX as u64)
    & !3;
unsafe extern "C" fn block_encode(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
    action: lzma_action,
) -> lzma_ret {
    let coder: *mut lzma_block_coder = coder_ptr as *mut lzma_block_coder;
    if (LZMA_VLI_MAX).wrapping_sub((*coder).uncompressed_size)
        < in_size.wrapping_sub(*in_pos) as lzma_vli
    {
        return LZMA_DATA_ERROR;
    }
    's_142: {
        let current_block_34: u64;
        match (*coder).sequence {
            0 => {
                let in_start: size_t = *in_pos;
                let out_start: size_t = *out_pos;
                let ret: lzma_ret = (*coder).next.code.expect("non-null function pointer")(
                    (*coder).next.coder,
                    allocator,
                    in_0,
                    in_pos,
                    in_size,
                    out,
                    out_pos,
                    out_size,
                    action,
                );
                let in_used: size_t = (*in_pos).wrapping_sub(in_start);
                let out_used: size_t = (*out_pos).wrapping_sub(out_start);
                if (COMPRESSED_SIZE_MAX as lzma_vli).wrapping_sub((*coder).compressed_size)
                    < out_used as lzma_vli
                {
                    return LZMA_DATA_ERROR;
                }
                (*coder).compressed_size =
                    (*coder).compressed_size.wrapping_add(out_used as lzma_vli);
                (*coder).uncompressed_size =
                    (*coder).uncompressed_size.wrapping_add(in_used as lzma_vli);
                if in_used > 0 {
                    lzma_check_update(
                        &raw mut (*coder).check,
                        (*(*coder).block).check,
                        in_0.offset(in_start as isize),
                        in_used,
                    );
                }
                if ret != LZMA_STREAM_END || action == LZMA_SYNC_FLUSH {
                    return ret;
                }
                (*(*coder).block).compressed_size = (*coder).compressed_size;
                (*(*coder).block).uncompressed_size = (*coder).uncompressed_size;
                (*coder).sequence = SEQ_PADDING;
                current_block_34 = 6470892831169497455;
            }
            1 => {
                current_block_34 = 6470892831169497455;
            }
            2 => {
                current_block_34 = 47327340716975230;
            }
            _ => {
                break 's_142;
            }
        }
        match current_block_34 {
            6470892831169497455 => {
                while (*coder).compressed_size & 3 != 0 {
                    if *out_pos >= out_size {
                        return LZMA_OK;
                    }
                    *out.offset(*out_pos as isize) = 0;
                    *out_pos = (*out_pos).wrapping_add(1);
                    (*coder).compressed_size = (*coder).compressed_size.wrapping_add(1);
                }
                if (*(*coder).block).check == LZMA_CHECK_NONE {
                    return LZMA_STREAM_END;
                }
                lzma_check_finish(&raw mut (*coder).check, (*(*coder).block).check);
                (*coder).sequence = SEQ_CHECK;
            }
            _ => {}
        }
        let check_size: size_t = lzma_check_size((*(*coder).block).check) as size_t;
        lzma_bufcpy(
            &raw mut (*coder).check.buffer.u8_0 as *mut u8,
            &raw mut (*coder).pos,
            check_size,
            out,
            out_pos,
            out_size,
        );
        if (*coder).pos < check_size {
            return LZMA_OK;
        }
        memcpy(
            &raw mut (*(*coder).block).raw_check as *mut u8 as *mut c_void,
            &raw mut (*coder).check.buffer.u8_0 as *mut u8 as *const c_void,
            check_size,
        );
        return LZMA_STREAM_END;
    }
    return LZMA_PROG_ERROR;
}
unsafe extern "C" fn block_encoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_block_coder = coder_ptr as *mut lzma_block_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn block_encoder_update(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    _filters: *const lzma_filter,
    reversed_filters: *const lzma_filter,
) -> lzma_ret {
    let coder: *mut lzma_block_coder = coder_ptr as *mut lzma_block_coder;
    if (*coder).sequence != SEQ_CODE {
        return LZMA_PROG_ERROR;
    }
    return lzma_next_filter_update(&raw mut (*coder).next, allocator, reversed_filters);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    block: *mut lzma_block,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut lzma_block,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_block_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut lzma_block,
            ) -> lzma_ret,
    )) != (*next).init
    {
        lzma_next_end(next, allocator);
    }
    (*next).init = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut lzma_block,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_block_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut lzma_block,
            ) -> lzma_ret,
    ));
    if block.is_null() {
        return LZMA_PROG_ERROR;
    }
    if (*block).version > 1 {
        return LZMA_OPTIONS_ERROR;
    }
    if (*block).check > LZMA_CHECK_ID_MAX {
        return LZMA_PROG_ERROR;
    }
    if lzma_check_is_supported((*block).check) == 0 {
        return LZMA_UNSUPPORTED_CHECK;
    }
    let mut coder: *mut lzma_block_coder = (*next).coder as *mut lzma_block_coder;
    if coder.is_null() {
        coder = lzma_alloc(core::mem::size_of::<lzma_block_coder>(), allocator)
            as *mut lzma_block_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            block_encode
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    *mut size_t,
                    size_t,
                    *mut u8,
                    *mut size_t,
                    size_t,
                    lzma_action,
                ) -> lzma_ret,
        ) as lzma_code_function;
        (*next).end = Some(
            block_encoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).update = Some(
            block_encoder_update
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const lzma_allocator,
                    *const lzma_filter,
                    *const lzma_filter,
                ) -> lzma_ret,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut c_void,
                    *const lzma_allocator,
                    *const lzma_filter,
                    *const lzma_filter,
                ) -> lzma_ret,
            >;
        (*coder).next = lzma_next_coder_s {
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
    }
    (*coder).sequence = SEQ_CODE;
    (*coder).block = block;
    (*coder).compressed_size = 0;
    (*coder).uncompressed_size = 0;
    (*coder).pos = 0;
    lzma_check_init(&raw mut (*coder).check, (*block).check);
    return lzma_raw_encoder_init(&raw mut (*coder).next, allocator, (*block).filters);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_encoder(
    strm: *mut lzma_stream,
    block: *mut lzma_block,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret =
        lzma_block_encoder_init(&raw mut (*(*strm).internal).next, (*strm).allocator, block);
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_SYNC_FLUSH as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    return LZMA_OK;
}
