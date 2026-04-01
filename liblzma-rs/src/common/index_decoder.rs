use crate::common::index::lzma_index_prealloc;
use crate::types::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_index_coder {
    pub sequence: index_decoder_seq,
    pub memlimit: u64,
    pub index: *mut lzma_index,
    pub index_ptr: *mut *mut lzma_index,
    pub count: lzma_vli,
    pub unpadded_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub pos: size_t,
    pub crc32: u32,
}
pub type index_decoder_seq = c_uint;
pub const SEQ_CRC32: index_decoder_seq = 7;
pub const SEQ_PADDING: index_decoder_seq = 6;
pub const SEQ_PADDING_INIT: index_decoder_seq = 5;
pub const SEQ_UNCOMPRESSED: index_decoder_seq = 4;
pub const SEQ_UNPADDED: index_decoder_seq = 3;
pub const SEQ_MEMUSAGE: index_decoder_seq = 2;
pub const SEQ_COUNT: index_decoder_seq = 1;
pub const SEQ_INDICATOR: index_decoder_seq = 0;
unsafe extern "C" fn index_decode(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    _out: *mut u8,
    _out_pos: *mut size_t,
    _out_size: size_t,
    _action: lzma_action,
) -> lzma_ret {
    let mut current_block: u64;
    let coder: *mut lzma_index_coder = coder_ptr as *mut lzma_index_coder;
    let in_start: size_t = *in_pos;
    let mut ret: lzma_ret = LZMA_OK;
    while *in_pos < in_size {
        match (*coder).sequence {
            0 => {
                let byte = *in_0.offset(*in_pos as isize);
                *in_pos += 1;
                if byte != INDEX_INDICATOR {
                    return LZMA_DATA_ERROR;
                }
                (*coder).sequence = SEQ_COUNT;
                continue;
            }
            1 => {
                ret = lzma_vli_decode(
                    ::core::ptr::addr_of_mut!((*coder).count),
                    ::core::ptr::addr_of_mut!((*coder).pos),
                    in_0,
                    in_pos,
                    in_size,
                );
                if ret != LZMA_STREAM_END {
                    break;
                }
                (*coder).pos = 0;
                (*coder).sequence = SEQ_MEMUSAGE;
                current_block = 7642845755631126846;
            }
            2 => {
                current_block = 7642845755631126846;
            }
            3 | 4 => {
                let size: *mut lzma_vli = if (*coder).sequence == SEQ_UNPADDED {
                    ::core::ptr::addr_of_mut!((*coder).unpadded_size)
                } else {
                    ::core::ptr::addr_of_mut!((*coder).uncompressed_size)
                };
                ret = lzma_vli_decode(
                    size,
                    ::core::ptr::addr_of_mut!((*coder).pos),
                    in_0,
                    in_pos,
                    in_size,
                );
                if ret != LZMA_STREAM_END {
                    break;
                }
                ret = LZMA_OK;
                (*coder).pos = 0;
                if (*coder).sequence == SEQ_UNPADDED {
                    if (*coder).unpadded_size < UNPADDED_SIZE_MIN
                        || (*coder).unpadded_size > UNPADDED_SIZE_MAX
                    {
                        return LZMA_DATA_ERROR;
                    }
                    (*coder).sequence = SEQ_UNCOMPRESSED;
                } else {
                    let ret_: lzma_ret = lzma_index_append(
                        (*coder).index,
                        allocator,
                        (*coder).unpadded_size,
                        (*coder).uncompressed_size,
                    );
                    if ret_ != LZMA_OK {
                        return ret_;
                    }
                    (*coder).count -= 1;
                    (*coder).sequence = (if (*coder).count == 0 {
                        SEQ_PADDING_INIT
                    } else {
                        SEQ_UNPADDED
                    }) as index_decoder_seq;
                }
                continue;
            }
            5 => {
                (*coder).pos = lzma_index_padding_size((*coder).index) as size_t;
                (*coder).sequence = SEQ_PADDING;
                current_block = 8340016495055110192;
            }
            6 => {
                current_block = 8340016495055110192;
            }
            7 => {
                current_block = 9471676622948044094;
            }
            _ => return LZMA_PROG_ERROR,
        }
        match current_block {
            8340016495055110192 => {
                if (*coder).pos > 0 {
                    (*coder).pos -= 1;
                    let byte = *in_0.offset(*in_pos as isize);
                    *in_pos += 1;
                    if byte != 0 {
                        return LZMA_DATA_ERROR;
                    }
                    continue;
                } else {
                    (*coder).crc32 = lzma_crc32(
                        in_0.offset(in_start as isize),
                        *in_pos - in_start,
                        (*coder).crc32,
                    );
                    (*coder).sequence = SEQ_CRC32;
                }
            }
            7642845755631126846 => {
                if lzma_index_memusage(1, (*coder).count) > (*coder).memlimit {
                    ret = LZMA_MEMLIMIT_ERROR;
                    break;
                } else {
                    lzma_index_prealloc((*coder).index, (*coder).count);
                    ret = LZMA_OK;
                    (*coder).sequence = (if (*coder).count == 0 {
                        SEQ_PADDING_INIT
                    } else {
                        SEQ_UNPADDED
                    }) as index_decoder_seq;
                    continue;
                }
            }
            _ => {}
        }
        loop {
            if *in_pos == in_size {
                return LZMA_OK;
            }
            let val = *in_0.offset(*in_pos as isize);
            *in_pos += 1;
            if (*coder).crc32 >> ((*coder).pos * 8) & 0xff != val as u32 {
                return LZMA_DATA_ERROR;
            }
            (*coder).pos += 1;
            if (*coder).pos >= 4 {
                break;
            }
        }
        *(*coder).index_ptr = (*coder).index;
        (*coder).index = core::ptr::null_mut();
        return LZMA_STREAM_END;
    }
    let in_used: size_t = *in_pos - in_start;
    if in_used > 0 {
        (*coder).crc32 = lzma_crc32(in_0.offset(in_start as isize), in_used, (*coder).crc32);
    }
    ret
}
unsafe extern "C" fn index_decoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_index_coder = coder_ptr as *mut lzma_index_coder;
    lzma_index_end((*coder).index, allocator);
    crate::alloc::internal_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn index_decoder_memconfig(
    coder_ptr: *mut c_void,
    memusage: *mut u64,
    old_memlimit: *mut u64,
    new_memlimit: u64,
) -> lzma_ret {
    let coder: *mut lzma_index_coder = coder_ptr as *mut lzma_index_coder;
    *memusage = lzma_index_memusage(1, (*coder).count);
    *old_memlimit = (*coder).memlimit;
    if new_memlimit != 0 {
        if new_memlimit < *memusage {
            return LZMA_MEMLIMIT_ERROR;
        }
        (*coder).memlimit = new_memlimit;
    }
    LZMA_OK
}
unsafe fn index_decoder_reset(
    coder: *mut lzma_index_coder,
    allocator: *const lzma_allocator,
    i: *mut *mut lzma_index,
    memlimit: u64,
) -> lzma_ret {
    (*coder).index_ptr = i;
    *i = core::ptr::null_mut();
    (*coder).index = lzma_index_init(allocator);
    if (*coder).index.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*coder).sequence = SEQ_INDICATOR;
    (*coder).memlimit = if 1 > memlimit { 1 } else { memlimit };
    (*coder).count = 0;
    (*coder).pos = 0;
    (*coder).crc32 = 0;
    LZMA_OK
}
pub unsafe extern "C" fn lzma_index_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    i: *mut *mut lzma_index,
    memlimit: u64,
) -> lzma_ret {
    if core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut *mut lzma_index,
                u64,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_index_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut *mut lzma_index,
                u64,
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
                *mut *mut lzma_index,
                u64,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_index_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut *mut lzma_index,
                u64,
            ) -> lzma_ret,
    ));
    if i.is_null() {
        return LZMA_PROG_ERROR;
    }
    let mut coder: *mut lzma_index_coder = (*next).coder as *mut lzma_index_coder;
    if coder.is_null() {
        coder = crate::alloc::internal_alloc_object::<lzma_index_coder>(allocator);
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            index_decode
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
            index_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        );
        (*next).memconfig = Some(
            index_decoder_memconfig
                as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret,
        );
        (*coder).index = core::ptr::null_mut();
    } else {
        lzma_index_end((*coder).index, allocator);
    }
    index_decoder_reset(coder, allocator, i, memlimit)
}
pub unsafe fn lzma_index_decoder(
    strm: *mut lzma_stream,
    i: *mut *mut lzma_index,
    memlimit: u64,
) -> lzma_ret {
    if !i.is_null() {
        *i = core::ptr::null_mut();
    }
    let ret_: lzma_ret = lzma_strm_init(strm);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = lzma_index_decoder_init(
        ::core::ptr::addr_of_mut!((*(*strm).internal).next),
        (*strm).allocator,
        i,
        memlimit,
    );
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    LZMA_OK
}
pub unsafe fn lzma_index_buffer_decode(
    i: *mut *mut lzma_index,
    memlimit: *mut u64,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    if !i.is_null() {
        *i = core::ptr::null_mut();
    }
    if i.is_null() || memlimit.is_null() || in_0.is_null() || in_pos.is_null() || *in_pos > in_size
    {
        return LZMA_PROG_ERROR;
    }
    let mut coder: lzma_index_coder = lzma_index_coder {
        sequence: SEQ_INDICATOR,
        memlimit: 0,
        index: core::ptr::null_mut(),
        index_ptr: core::ptr::null_mut(),
        count: 0,
        unpadded_size: 0,
        uncompressed_size: 0,
        pos: 0,
        crc32: 0,
    };
    let ret_: lzma_ret =
        index_decoder_reset(::core::ptr::addr_of_mut!(coder), allocator, i, *memlimit);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let in_start: size_t = *in_pos;
    let mut ret: lzma_ret = index_decode(
        ::core::ptr::addr_of_mut!(coder) as *mut c_void,
        allocator,
        in_0,
        in_pos,
        in_size,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        0,
        LZMA_RUN,
    );
    if ret == LZMA_STREAM_END {
        ret = LZMA_OK;
    } else {
        lzma_index_end(coder.index, allocator);
        *in_pos = in_start;
        if ret == LZMA_OK {
            ret = LZMA_DATA_ERROR;
        } else if ret == LZMA_MEMLIMIT_ERROR {
            *memlimit = lzma_index_memusage(1, coder.count);
        }
    }
    ret
}
