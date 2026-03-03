use crate::types::*;
use core::ffi::{c_uint, c_void};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream_coder {
    pub sequence: stream_encoder_seq,
    pub block_encoder_is_initialized: bool,
    pub block_encoder: lzma_next_coder,
    pub block_options: lzma_block,
    pub filters: [lzma_filter; 5],
    pub index_encoder: lzma_next_coder,
    pub index: *mut lzma_index,
    pub buffer_pos: size_t,
    pub buffer_size: size_t,
    pub buffer: [u8; LZMA_BLOCK_HEADER_SIZE_MAX as usize],
}
pub type stream_encoder_seq = c_uint;
pub const SEQ_STREAM_FOOTER: stream_encoder_seq = 5;
pub const SEQ_INDEX_ENCODE: stream_encoder_seq = 4;
pub const SEQ_BLOCK_ENCODE: stream_encoder_seq = 3;
pub const SEQ_BLOCK_HEADER: stream_encoder_seq = 2;
pub const SEQ_BLOCK_INIT: stream_encoder_seq = 1;
pub const SEQ_STREAM_HEADER: stream_encoder_seq = 0;
unsafe extern "C" fn block_encoder_init(
    coder: *mut lzma_stream_coder,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    (*coder).block_options.compressed_size = LZMA_VLI_UNKNOWN;
    (*coder).block_options.uncompressed_size = LZMA_VLI_UNKNOWN;
    let ret_: lzma_ret = lzma_block_header_size(&raw mut (*coder).block_options);
    if ret_ != LZMA_OK {
        return ret_;
    }
    lzma_block_encoder_init(
        &raw mut (*coder).block_encoder,
        allocator,
        &raw mut (*coder).block_options,
    )
}
unsafe extern "C" fn stream_encode(
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
    let coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    while *out_pos < out_size {
        match (*coder).sequence {
            0 | 2 | 5 => {
                lzma_bufcpy(
                    &raw mut (*coder).buffer as *mut u8,
                    &raw mut (*coder).buffer_pos,
                    (*coder).buffer_size,
                    out,
                    out_pos,
                    out_size,
                );
                if (*coder).buffer_pos < (*coder).buffer_size {
                    return LZMA_OK;
                }
                if (*coder).sequence == SEQ_STREAM_FOOTER {
                    return LZMA_STREAM_END;
                }
                (*coder).buffer_pos = 0;
                (*coder).sequence += 1;
            }
            1 => {
                if *in_pos == in_size {
                    if action != LZMA_FINISH {
                        return if action == LZMA_RUN {
                            LZMA_OK
                        } else {
                            LZMA_STREAM_END
                        };
                    }
                    let ret_: lzma_ret = lzma_index_encoder_init(
                        &raw mut (*coder).index_encoder,
                        allocator,
                        (*coder).index,
                    );
                    if ret_ != LZMA_OK {
                        return ret_;
                    }
                    (*coder).sequence = SEQ_INDEX_ENCODE;
                } else {
                    if !(*coder).block_encoder_is_initialized {
                        let ret__0: lzma_ret = block_encoder_init(coder, allocator);
                        if ret__0 != LZMA_OK {
                            return ret__0;
                        }
                    }
                    (*coder).block_encoder_is_initialized = false;
                    if lzma_block_header_encode(
                        &raw mut (*coder).block_options,
                        &raw mut (*coder).buffer as *mut u8,
                    ) != LZMA_OK
                    {
                        return LZMA_PROG_ERROR;
                    }
                    (*coder).buffer_size = (*coder).block_options.header_size as size_t;
                    (*coder).sequence = SEQ_BLOCK_HEADER;
                }
            }
            3 => {
                static mut convert: [lzma_action; 5] = [
                    LZMA_RUN,
                    LZMA_SYNC_FLUSH,
                    LZMA_FINISH,
                    LZMA_FINISH,
                    LZMA_FINISH,
                ];
                let ret: lzma_ret = (*coder).block_encoder.code.unwrap()(
                    (*coder).block_encoder.coder,
                    allocator,
                    in_0,
                    in_pos,
                    in_size,
                    out,
                    out_pos,
                    out_size,
                    convert[action as usize],
                );
                if ret != LZMA_STREAM_END || action == LZMA_SYNC_FLUSH {
                    return ret;
                }
                let unpadded_size: lzma_vli =
                    lzma_block_unpadded_size(&raw mut (*coder).block_options) as lzma_vli;
                let ret__1: lzma_ret = lzma_index_append(
                    (*coder).index,
                    allocator,
                    unpadded_size,
                    (*coder).block_options.uncompressed_size,
                );
                if ret__1 != LZMA_OK {
                    return ret__1;
                }
                (*coder).sequence = SEQ_BLOCK_INIT;
            }
            4 => {
                let ret_0: lzma_ret = (*coder).index_encoder.code.unwrap()(
                    (*coder).index_encoder.coder,
                    allocator,
                    core::ptr::null(),
                    core::ptr::null_mut(),
                    0,
                    out,
                    out_pos,
                    out_size,
                    LZMA_RUN,
                );
                if ret_0 != LZMA_STREAM_END {
                    return ret_0;
                }
                let stream_flags: lzma_stream_flags = lzma_stream_flags {
                    version: 0,
                    backward_size: lzma_index_size((*coder).index),
                    check: (*coder).block_options.check,
                    reserved_enum1: LZMA_RESERVED_ENUM,
                    reserved_enum2: LZMA_RESERVED_ENUM,
                    reserved_enum3: LZMA_RESERVED_ENUM,
                    reserved_enum4: LZMA_RESERVED_ENUM,
                    reserved_bool1: 0,
                    reserved_bool2: 0,
                    reserved_bool3: 0,
                    reserved_bool4: 0,
                    reserved_bool5: 0,
                    reserved_bool6: 0,
                    reserved_bool7: 0,
                    reserved_bool8: 0,
                    reserved_int1: 0,
                    reserved_int2: 0,
                };
                if lzma_stream_footer_encode(
                    &raw const stream_flags,
                    &raw mut (*coder).buffer as *mut u8,
                ) != LZMA_OK
                {
                    return LZMA_PROG_ERROR;
                }
                (*coder).buffer_size = LZMA_STREAM_HEADER_SIZE as size_t;
                (*coder).sequence = SEQ_STREAM_FOOTER;
            }
            _ => return LZMA_PROG_ERROR,
        }
    }
    LZMA_OK
}
unsafe extern "C" fn stream_encoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    lzma_next_end(&raw mut (*coder).block_encoder, allocator);
    lzma_next_end(&raw mut (*coder).index_encoder, allocator);
    lzma_index_end((*coder).index, allocator);
    lzma_filters_free(&raw mut (*coder).filters as *mut lzma_filter, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn stream_encoder_update(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter,
    reversed_filters: *const lzma_filter,
) -> lzma_ret {
    let current_block: u64;
    let coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    let mut ret: lzma_ret = LZMA_OK;
    let mut temp: [lzma_filter; 5] = [lzma_filter {
        id: 0,
        options: core::ptr::null_mut(),
    }; 5];
    let ret_: lzma_ret = lzma_filters_copy(filters, &raw mut temp as *mut lzma_filter, allocator);
    if ret_ != LZMA_OK {
        return ret_;
    }
    if (*coder).sequence <= SEQ_BLOCK_INIT {
        (*coder).block_encoder_is_initialized = false;
        (*coder).block_options.filters = &raw mut temp as *mut lzma_filter;
        ret = block_encoder_init(coder, allocator);
        (*coder).block_options.filters = &raw mut (*coder).filters as *mut lzma_filter;
        if ret != LZMA_OK {
            current_block = 9913398440939854562;
        } else {
            (*coder).block_encoder_is_initialized = true;
            current_block = 8236137900636309791;
        }
    } else if (*coder).sequence <= SEQ_BLOCK_ENCODE {
        ret = (*coder).block_encoder.update.unwrap()(
            (*coder).block_encoder.coder,
            allocator,
            filters,
            reversed_filters,
        );
        if ret != LZMA_OK {
            current_block = 9913398440939854562;
        } else {
            current_block = 8236137900636309791;
        }
    } else {
        ret = LZMA_PROG_ERROR;
        current_block = 9913398440939854562;
    }
    match current_block {
        9913398440939854562 => {
            lzma_filters_free(&raw mut temp as *mut lzma_filter, allocator);
            return ret;
        }
        _ => {
            lzma_filters_free(&raw mut (*coder).filters as *mut lzma_filter, allocator);
            core::ptr::copy_nonoverlapping(
                &raw mut temp as *const u8,
                &raw mut (*coder).filters as *mut u8,
                core::mem::size_of::<[lzma_filter; 5]>(),
            );
            return LZMA_OK;
        }
    };
}
unsafe extern "C" fn stream_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter,
    check: lzma_check,
) -> lzma_ret {
    if core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_filter,
                lzma_check,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        stream_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_filter,
                lzma_check,
            ) -> lzma_ret,
    )) != (*next).init
    {
        lzma_next_end(next, allocator);
    }
    (*next).init = core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_filter,
                lzma_check,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        stream_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_filter,
                lzma_check,
            ) -> lzma_ret,
    ));
    if filters.is_null() {
        return LZMA_PROG_ERROR;
    }
    let mut coder: *mut lzma_stream_coder = (*next).coder as *mut lzma_stream_coder;
    if coder.is_null() {
        coder = lzma_alloc(core::mem::size_of::<lzma_stream_coder>(), allocator)
            as *mut lzma_stream_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            stream_encode
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
        );
        (*next).end = Some(
            stream_encoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        );
        (*next).update = Some(
            stream_encoder_update
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const lzma_allocator,
                    *const lzma_filter,
                    *const lzma_filter,
                ) -> lzma_ret,
        );
        (*coder).filters[0].id = LZMA_VLI_UNKNOWN;
        (*coder).block_encoder = lzma_next_coder_s {
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
        (*coder).index_encoder = lzma_next_coder_s {
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
        (*coder).index = core::ptr::null_mut();
    }
    (*coder).sequence = SEQ_STREAM_HEADER;
    (*coder).block_options.version = 0;
    (*coder).block_options.check = check;
    lzma_index_end((*coder).index, allocator);
    (*coder).index = lzma_index_init(allocator);
    if (*coder).index.is_null() {
        return LZMA_MEM_ERROR;
    }
    let mut stream_flags: lzma_stream_flags = lzma_stream_flags {
        version: 0,
        backward_size: 0,
        check: check,
        reserved_enum1: LZMA_RESERVED_ENUM,
        reserved_enum2: LZMA_RESERVED_ENUM,
        reserved_enum3: LZMA_RESERVED_ENUM,
        reserved_enum4: LZMA_RESERVED_ENUM,
        reserved_bool1: 0,
        reserved_bool2: 0,
        reserved_bool3: 0,
        reserved_bool4: 0,
        reserved_bool5: 0,
        reserved_bool6: 0,
        reserved_bool7: 0,
        reserved_bool8: 0,
        reserved_int1: 0,
        reserved_int2: 0,
    };
    let ret_: lzma_ret =
        lzma_stream_header_encode(&raw mut stream_flags, &raw mut (*coder).buffer as *mut u8);
    if ret_ != LZMA_OK {
        return ret_;
    }
    (*coder).buffer_pos = 0;
    (*coder).buffer_size = LZMA_STREAM_HEADER_SIZE as size_t;
    stream_encoder_update(coder as *mut c_void, allocator, filters, core::ptr::null())
}
pub unsafe extern "C" fn lzma_stream_encoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
    check: lzma_check,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = stream_encoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        filters,
        check,
    );
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_SYNC_FLUSH as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FULL_FLUSH as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FULL_BARRIER as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    LZMA_OK
}
