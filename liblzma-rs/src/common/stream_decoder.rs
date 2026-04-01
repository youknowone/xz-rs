use crate::types::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream_coder {
    pub sequence: stream_decoder_seq,
    pub block_decoder: lzma_next_coder,
    pub block_options: lzma_block,
    pub stream_flags: lzma_stream_flags,
    pub index_hash: *mut lzma_index_hash,
    pub memlimit: u64,
    pub memusage: u64,
    pub tell_no_check: bool,
    pub tell_unsupported_check: bool,
    pub tell_any_check: bool,
    pub ignore_check: bool,
    pub concatenated: bool,
    pub first_stream: bool,
    pub pos: size_t,
    pub buffer: [u8; LZMA_BLOCK_HEADER_SIZE_MAX as usize],
}
pub type stream_decoder_seq = c_uint;
pub const SEQ_STREAM_PADDING: stream_decoder_seq = 6;
pub const SEQ_STREAM_FOOTER: stream_decoder_seq = 5;
pub const SEQ_INDEX: stream_decoder_seq = 4;
pub const SEQ_BLOCK_RUN: stream_decoder_seq = 3;
pub const SEQ_BLOCK_INIT: stream_decoder_seq = 2;
pub const SEQ_BLOCK_HEADER: stream_decoder_seq = 1;
pub const SEQ_STREAM_HEADER: stream_decoder_seq = 0;
unsafe fn stream_decoder_reset(
    coder: *mut lzma_stream_coder,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    (*coder).index_hash = lzma_index_hash_init((*coder).index_hash, allocator);
    if (*coder).index_hash.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*coder).sequence = SEQ_STREAM_HEADER;
    (*coder).pos = 0;
    LZMA_OK
}
unsafe extern "C" fn stream_decode(
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
    loop {
        let mut current_block_100: u64;
        match (*coder).sequence {
            0 => {
                lzma_bufcpy(
                    in_0,
                    in_pos,
                    in_size,
                    ::core::ptr::addr_of_mut!((*coder).buffer) as *mut u8,
                    ::core::ptr::addr_of_mut!((*coder).pos),
                    LZMA_STREAM_HEADER_SIZE as size_t,
                );
                if (*coder).pos < LZMA_STREAM_HEADER_SIZE as size_t {
                    return LZMA_OK;
                }
                (*coder).pos = 0;
                let ret: lzma_ret = lzma_stream_header_decode(
                    ::core::ptr::addr_of_mut!((*coder).stream_flags),
                    ::core::ptr::addr_of_mut!((*coder).buffer) as *mut u8,
                );
                if ret != LZMA_OK {
                    return if ret == LZMA_FORMAT_ERROR && !(*coder).first_stream {
                        LZMA_DATA_ERROR
                    } else {
                        ret
                    };
                }
                (*coder).first_stream = false;
                (*coder).block_options.check = (*coder).stream_flags.check;
                (*coder).sequence = SEQ_BLOCK_HEADER;
                if (*coder).tell_no_check && (*coder).stream_flags.check == LZMA_CHECK_NONE {
                    return LZMA_NO_CHECK;
                }
                if (*coder).tell_unsupported_check
                    && lzma_check_is_supported((*coder).stream_flags.check) == 0
                {
                    return LZMA_UNSUPPORTED_CHECK;
                }
                if (*coder).tell_any_check {
                    return LZMA_GET_CHECK;
                }
                current_block_100 = 4166486009154926805;
            }
            1 => {
                current_block_100 = 4166486009154926805;
            }
            2 => {
                current_block_100 = 3500765272169221397;
            }
            3 => {
                current_block_100 = 721385680381463314;
            }
            4 => {
                if *in_pos >= in_size {
                    return LZMA_OK;
                }
                let ret_2: lzma_ret =
                    lzma_index_hash_decode((*coder).index_hash, in_0, in_pos, in_size);
                if ret_2 != LZMA_STREAM_END {
                    return ret_2;
                }
                (*coder).sequence = SEQ_STREAM_FOOTER;
                current_block_100 = 17861496924281778896;
            }
            5 => {
                current_block_100 = 17861496924281778896;
            }
            6 => {
                current_block_100 = 15462640364611497761;
            }
            _ => return LZMA_PROG_ERROR,
        }
        match current_block_100 {
            4166486009154926805 => {
                if *in_pos >= in_size {
                    return LZMA_OK;
                }
                if (*coder).pos == 0 {
                    if *in_0.offset(*in_pos as isize) == INDEX_INDICATOR {
                        (*coder).sequence = SEQ_INDEX;
                        current_block_100 = 16789764818708874114;
                    } else {
                        (*coder).block_options.header_size =
                            ((*in_0.offset(*in_pos as isize) as u32) + 1) * 4;
                        current_block_100 = 13242334135786603907;
                    }
                } else {
                    current_block_100 = 13242334135786603907;
                }
                match current_block_100 {
                    16789764818708874114 => {}
                    _ => {
                        lzma_bufcpy(
                            in_0,
                            in_pos,
                            in_size,
                            ::core::ptr::addr_of_mut!((*coder).buffer) as *mut u8,
                            ::core::ptr::addr_of_mut!((*coder).pos),
                            (*coder).block_options.header_size as size_t,
                        );
                        if (*coder).pos < (*coder).block_options.header_size as size_t {
                            return LZMA_OK;
                        }
                        (*coder).pos = 0;
                        (*coder).sequence = SEQ_BLOCK_INIT;
                        current_block_100 = 3500765272169221397;
                    }
                }
            }
            17861496924281778896 => {
                lzma_bufcpy(
                    in_0,
                    in_pos,
                    in_size,
                    ::core::ptr::addr_of_mut!((*coder).buffer) as *mut u8,
                    ::core::ptr::addr_of_mut!((*coder).pos),
                    LZMA_STREAM_HEADER_SIZE as size_t,
                );
                if (*coder).pos < LZMA_STREAM_HEADER_SIZE as size_t {
                    return LZMA_OK;
                }
                (*coder).pos = 0;
                let mut footer_flags: lzma_stream_flags = lzma_stream_flags {
                    version: 0,
                    backward_size: 0,
                    check: LZMA_CHECK_NONE,
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
                let ret_3: lzma_ret = lzma_stream_footer_decode(
                    ::core::ptr::addr_of_mut!(footer_flags),
                    ::core::ptr::addr_of_mut!((*coder).buffer) as *mut u8,
                );
                if ret_3 != LZMA_OK {
                    return if ret_3 == LZMA_FORMAT_ERROR {
                        LZMA_DATA_ERROR
                    } else {
                        ret_3
                    };
                }
                if lzma_index_hash_size((*coder).index_hash) != footer_flags.backward_size {
                    return LZMA_DATA_ERROR;
                }
                let ret__1: lzma_ret = lzma_stream_flags_compare(
                    ::core::ptr::addr_of_mut!((*coder).stream_flags),
                    ::core::ptr::addr_of_mut!(footer_flags),
                );
                if ret__1 != LZMA_OK {
                    return ret__1;
                }
                if !(*coder).concatenated {
                    return LZMA_STREAM_END;
                }
                (*coder).sequence = SEQ_STREAM_PADDING;
                current_block_100 = 15462640364611497761;
            }
            _ => {}
        }
        match current_block_100 {
            15462640364611497761 => {
                loop {
                    if *in_pos >= in_size {
                        if action != LZMA_FINISH {
                            return LZMA_OK;
                        }
                        return if (*coder).pos == 0 {
                            LZMA_STREAM_END
                        } else {
                            LZMA_DATA_ERROR
                        };
                    }
                    if *in_0.offset(*in_pos as isize) != 0 {
                        break;
                    }
                    *in_pos += 1;
                    (*coder).pos = ((*coder).pos + 1) & 3;
                }
                if (*coder).pos != 0 {
                    *in_pos += 1;
                    return LZMA_DATA_ERROR;
                }
                let ret__2: lzma_ret = stream_decoder_reset(coder, allocator);
                if ret__2 != LZMA_OK {
                    return ret__2;
                }
                current_block_100 = 16789764818708874114;
            }
            3500765272169221397 => {
                (*coder).block_options.version = 1;
                let mut filters: [lzma_filter; 5] = [lzma_filter {
                    id: 0,
                    options: core::ptr::null_mut(),
                }; 5];
                (*coder).block_options.filters =
                    ::core::ptr::addr_of_mut!(filters) as *mut lzma_filter;
                let ret_: lzma_ret = lzma_block_header_decode(
                    ::core::ptr::addr_of_mut!((*coder).block_options),
                    allocator,
                    ::core::ptr::addr_of_mut!((*coder).buffer) as *mut u8,
                );
                if ret_ != LZMA_OK {
                    return ret_;
                }
                (*coder).block_options.ignore_check = (*coder).ignore_check as lzma_bool;
                let memusage: u64 = lzma_raw_decoder_memusage(
                    ::core::ptr::addr_of_mut!(filters) as *mut lzma_filter
                ) as u64;
                let mut ret_0: lzma_ret = LZMA_OK;
                if memusage == UINT64_MAX {
                    ret_0 = LZMA_OPTIONS_ERROR;
                } else {
                    (*coder).memusage = memusage;
                    if memusage > (*coder).memlimit {
                        ret_0 = LZMA_MEMLIMIT_ERROR;
                    } else {
                        ret_0 = lzma_block_decoder_init(
                            ::core::ptr::addr_of_mut!((*coder).block_decoder),
                            allocator,
                            ::core::ptr::addr_of_mut!((*coder).block_options),
                        );
                    }
                }
                lzma_filters_free(
                    ::core::ptr::addr_of_mut!(filters) as *mut lzma_filter,
                    allocator,
                );
                (*coder).block_options.filters = core::ptr::null_mut();
                if ret_0 != LZMA_OK {
                    return ret_0;
                }
                (*coder).sequence = SEQ_BLOCK_RUN;
                current_block_100 = 721385680381463314;
            }
            _ => {}
        }
        match current_block_100 {
            721385680381463314 => {
                let ret_1: lzma_ret = (*coder).block_decoder.code.unwrap()(
                    (*coder).block_decoder.coder,
                    allocator,
                    in_0,
                    in_pos,
                    in_size,
                    out,
                    out_pos,
                    out_size,
                    action,
                );
                if ret_1 != LZMA_STREAM_END {
                    return ret_1;
                }
                let ret__0: lzma_ret = lzma_index_hash_append(
                    (*coder).index_hash,
                    lzma_block_unpadded_size(::core::ptr::addr_of_mut!((*coder).block_options)),
                    (*coder).block_options.uncompressed_size,
                );
                if ret__0 != LZMA_OK {
                    return ret__0;
                }
                (*coder).sequence = SEQ_BLOCK_HEADER;
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn stream_decoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    lzma_next_end(::core::ptr::addr_of_mut!((*coder).block_decoder), allocator);
    lzma_index_hash_end((*coder).index_hash, allocator);
    crate::alloc::internal_free(coder as *mut c_void, allocator);
}
extern "C" fn stream_decoder_get_check(coder_ptr: *const c_void) -> lzma_check {
    return unsafe {
        let coder: *const lzma_stream_coder = coder_ptr as *const lzma_stream_coder;
        (*coder).stream_flags.check
    };
}
unsafe extern "C" fn stream_decoder_memconfig(
    coder_ptr: *mut c_void,
    memusage: *mut u64,
    old_memlimit: *mut u64,
    new_memlimit: u64,
) -> lzma_ret {
    let coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    *memusage = (*coder).memusage;
    *old_memlimit = (*coder).memlimit;
    if new_memlimit != 0 {
        if new_memlimit < (*coder).memusage {
            return LZMA_MEMLIMIT_ERROR;
        }
        (*coder).memlimit = new_memlimit;
    }
    LZMA_OK
}
pub unsafe extern "C" fn lzma_stream_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    if core::mem::transmute::<
        Option<
            unsafe extern "C" fn(*mut lzma_next_coder, *const lzma_allocator, u64, u32) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_stream_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                u32,
            ) -> lzma_ret,
    )) != (*next).init
    {
        lzma_next_end(next, allocator);
    }
    (*next).init = core::mem::transmute::<
        Option<
            unsafe extern "C" fn(*mut lzma_next_coder, *const lzma_allocator, u64, u32) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_stream_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                u32,
            ) -> lzma_ret,
    ));
    if flags & !(LZMA_SUPPORTED_FLAGS as u32) != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut coder: *mut lzma_stream_coder = (*next).coder as *mut lzma_stream_coder;
    if coder.is_null() {
        coder = crate::alloc::internal_alloc_object::<lzma_stream_coder>(allocator);
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            stream_decode
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
            stream_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        );
        (*next).get_check =
            Some(stream_decoder_get_check as unsafe extern "C" fn(*const c_void) -> lzma_check);
        (*next).memconfig = Some(
            stream_decoder_memconfig
                as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret,
        );
        (*coder).block_decoder = lzma_next_coder_s {
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
        (*coder).index_hash = core::ptr::null_mut();
    }
    (*coder).memlimit = if 1 > memlimit { 1 } else { memlimit };
    (*coder).memusage = LZMA_MEMUSAGE_BASE;
    (*coder).tell_no_check = flags & LZMA_TELL_NO_CHECK as u32 != 0;
    (*coder).tell_unsupported_check = flags & LZMA_TELL_UNSUPPORTED_CHECK as u32 != 0;
    (*coder).tell_any_check = flags & LZMA_TELL_ANY_CHECK as u32 != 0;
    (*coder).ignore_check = flags & LZMA_IGNORE_CHECK as u32 != 0;
    (*coder).concatenated = flags & LZMA_CONCATENATED as u32 != 0;
    (*coder).first_stream = true;
    stream_decoder_reset(coder, allocator)
}
pub unsafe fn lzma_stream_decoder(strm: *mut lzma_stream, memlimit: u64, flags: u32) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = lzma_stream_decoder_init(
        ::core::ptr::addr_of_mut!((*(*strm).internal).next),
        (*strm).allocator,
        memlimit,
        flags,
    );
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    LZMA_OK
}
