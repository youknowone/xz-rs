use crate::types::*;
use core::ffi::{c_uint, c_ulonglong, c_void};
#[repr(C)]
pub struct lzma_index_s {
    _opaque: [u8; 0],
}
extern "C" {
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_vli_decode(
        vli: *mut lzma_vli,
        vli_pos: *mut size_t,
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
    ) -> lzma_ret;
    fn lzma_crc32(buf: *const u8, size: size_t, crc: u32) -> u32;
    fn lzma_index_memusage(streams: lzma_vli, blocks: lzma_vli) -> u64;
    fn lzma_index_init(allocator: *const lzma_allocator) -> *mut lzma_index;
    fn lzma_index_end(i: *mut lzma_index, allocator: *const lzma_allocator);
    fn lzma_index_append(
        i: *mut lzma_index,
        allocator: *const lzma_allocator,
        unpadded_size: lzma_vli,
        uncompressed_size: lzma_vli,
    ) -> lzma_ret;
    fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_index_padding_size(i: *const lzma_index) -> u32;
    fn lzma_index_prealloc(i: *mut lzma_index, records: lzma_vli);
}
pub type lzma_index = lzma_index_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_index_coder {
    pub sequence: C2RustUnnamed_0,
    pub memlimit: u64,
    pub index: *mut lzma_index,
    pub index_ptr: *mut *mut lzma_index,
    pub count: lzma_vli,
    pub unpadded_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub pos: size_t,
    pub crc32: u32,
}
pub type C2RustUnnamed_0 = c_uint;
pub const SEQ_CRC32: C2RustUnnamed_0 = 7;
pub const SEQ_PADDING: C2RustUnnamed_0 = 6;
pub const SEQ_PADDING_INIT: C2RustUnnamed_0 = 5;
pub const SEQ_UNCOMPRESSED: C2RustUnnamed_0 = 4;
pub const SEQ_UNPADDED: C2RustUnnamed_0 = 3;
pub const SEQ_MEMUSAGE: C2RustUnnamed_0 = 2;
pub const SEQ_COUNT: C2RustUnnamed_0 = 1;
pub const SEQ_INDICATOR: C2RustUnnamed_0 = 0;
pub const UNPADDED_SIZE_MIN: c_ulonglong = 5;
pub const UNPADDED_SIZE_MAX: c_ulonglong = LZMA_VLI_MAX & !3;
pub const INDEX_INDICATOR: u8 = 0;
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
                    &raw mut (*coder).count,
                    &raw mut (*coder).pos,
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
                    &raw mut (*coder).unpadded_size
                } else {
                    &raw mut (*coder).uncompressed_size
                };
                ret = lzma_vli_decode(size, &raw mut (*coder).pos, in_0, in_pos, in_size);
                if ret != LZMA_STREAM_END {
                    break;
                }
                ret = LZMA_OK;
                (*coder).pos = 0;
                if (*coder).sequence == SEQ_UNPADDED {
                    if (*coder).unpadded_size < UNPADDED_SIZE_MIN as lzma_vli
                        || (*coder).unpadded_size > UNPADDED_SIZE_MAX as lzma_vli
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
                    (*coder).count = (*coder).count.wrapping_sub(1);
                    (*coder).sequence = (if (*coder).count == 0 {
                        SEQ_PADDING_INIT
                    } else {
                        SEQ_UNPADDED
                    }) as C2RustUnnamed_0;
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
                    (*coder).pos = (*coder).pos.wrapping_sub(1);
                    let byte = *in_0.offset(*in_pos as isize);
                    *in_pos += 1;
                    if byte != 0 {
                        return LZMA_DATA_ERROR;
                    }
                    continue;
                } else {
                    (*coder).crc32 = lzma_crc32(
                        in_0.offset(in_start as isize),
                        (*in_pos).wrapping_sub(in_start),
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
                    }) as C2RustUnnamed_0;
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
            if (*coder).crc32 >> (*coder).pos.wrapping_mul(8) & 0xff != val as u32 {
                return LZMA_DATA_ERROR;
            }
            (*coder).pos = (*coder).pos.wrapping_add(1);
            if (*coder).pos >= 4 {
                break;
            }
        }
        *(*coder).index_ptr = (*coder).index;
        (*coder).index = core::ptr::null_mut();
        return LZMA_STREAM_END;
    }
    let in_used: size_t = (*in_pos).wrapping_sub(in_start);
    if in_used > 0 {
        (*coder).crc32 = lzma_crc32(in_0.offset(in_start as isize), in_used, (*coder).crc32);
    }
    ret
}
unsafe extern "C" fn index_decoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_index_coder = coder_ptr as *mut lzma_index_coder;
    lzma_index_end((*coder).index, allocator);
    lzma_free(coder as *mut c_void, allocator);
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
unsafe extern "C" fn index_decoder_reset(
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
#[no_mangle]
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
        coder = lzma_alloc(core::mem::size_of::<lzma_index_coder>(), allocator)
            as *mut lzma_index_coder;
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
        (*next).memconfig = Some(index_decoder_memconfig as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret);
        (*coder).index = core::ptr::null_mut();
    } else {
        lzma_index_end((*coder).index, allocator);
    }
    index_decoder_reset(coder, allocator, i, memlimit)
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_decoder(
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
        &raw mut (*(*strm).internal).next,
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
#[no_mangle]
pub unsafe extern "C" fn lzma_index_buffer_decode(
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
    let ret_: lzma_ret = index_decoder_reset(&raw mut coder, allocator, i, *memlimit);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let in_start: size_t = *in_pos;
    let mut ret: lzma_ret = index_decode(
        &raw mut coder as *mut c_void,
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
