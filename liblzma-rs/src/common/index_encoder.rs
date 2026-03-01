use crate::types::*;
use core::ffi::{c_uint, c_void};
extern "C" {
    fn lzma_index_block_count(i: *const lzma_index) -> lzma_vli;
    fn lzma_index_iter_init(iter: *mut lzma_index_iter, i: *const lzma_index);
    fn lzma_index_iter_next(iter: *mut lzma_index_iter, mode: lzma_index_iter_mode) -> lzma_bool;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_index_iter {
    pub stream: C2RustUnnamed_2,
    pub block: C2RustUnnamed_1,
    pub internal: [C2RustUnnamed_0; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub p: *const c_void,
    pub s: size_t,
    pub v: lzma_vli,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub number_in_file: lzma_vli,
    pub compressed_file_offset: lzma_vli,
    pub uncompressed_file_offset: lzma_vli,
    pub number_in_stream: lzma_vli,
    pub compressed_stream_offset: lzma_vli,
    pub uncompressed_stream_offset: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub unpadded_size: lzma_vli,
    pub total_size: lzma_vli,
    pub reserved_vli1: lzma_vli,
    pub reserved_vli2: lzma_vli,
    pub reserved_vli3: lzma_vli,
    pub reserved_vli4: lzma_vli,
    pub reserved_ptr1: *const c_void,
    pub reserved_ptr2: *const c_void,
    pub reserved_ptr3: *const c_void,
    pub reserved_ptr4: *const c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub flags: *const lzma_stream_flags,
    pub reserved_ptr1: *const c_void,
    pub reserved_ptr2: *const c_void,
    pub reserved_ptr3: *const c_void,
    pub number: lzma_vli,
    pub block_count: lzma_vli,
    pub compressed_offset: lzma_vli,
    pub uncompressed_offset: lzma_vli,
    pub compressed_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub padding: lzma_vli,
    pub reserved_vli1: lzma_vli,
    pub reserved_vli2: lzma_vli,
    pub reserved_vli3: lzma_vli,
    pub reserved_vli4: lzma_vli,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_index_coder {
    pub sequence: C2RustUnnamed_3,
    pub index: *const lzma_index,
    pub iter: lzma_index_iter,
    pub pos: size_t,
    pub crc32: u32,
}
pub type C2RustUnnamed_3 = c_uint;
pub const SEQ_CRC32: C2RustUnnamed_3 = 6;
pub const SEQ_PADDING: C2RustUnnamed_3 = 5;
pub const SEQ_NEXT: C2RustUnnamed_3 = 4;
pub const SEQ_UNCOMPRESSED: C2RustUnnamed_3 = 3;
pub const SEQ_UNPADDED: C2RustUnnamed_3 = 2;
pub const SEQ_COUNT: C2RustUnnamed_3 = 1;
pub const SEQ_INDICATOR: C2RustUnnamed_3 = 0;
unsafe extern "C" fn index_encode(
    coder_ptr: *mut c_void,
    _allocator: *const lzma_allocator,
    _in_0: *const u8,
    _in_pos: *mut size_t,
    _in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
    _action: lzma_action,
) -> lzma_ret {
    let mut current_block: u64;
    let coder: *mut lzma_index_coder = coder_ptr as *mut lzma_index_coder;
    let out_start: size_t = *out_pos;
    let mut ret: lzma_ret = LZMA_OK;
    while *out_pos < out_size {
        match (*coder).sequence {
            0 => {
                *out.offset(*out_pos as isize) = INDEX_INDICATOR;
                *out_pos = (*out_pos).wrapping_add(1);
                (*coder).sequence = SEQ_COUNT;
                continue;
            }
            1 => {
                let count: lzma_vli = lzma_index_block_count((*coder).index) as lzma_vli;
                ret = lzma_vli_encode(count, &raw mut (*coder).pos, out, out_pos, out_size);
                if ret != LZMA_STREAM_END {
                    break;
                }
                ret = LZMA_OK;
                (*coder).pos = 0;
                (*coder).sequence = SEQ_NEXT;
                continue;
            }
            4 => {
                if lzma_index_iter_next(&raw mut (*coder).iter, LZMA_INDEX_ITER_BLOCK) != 0 {
                    (*coder).pos = lzma_index_padding_size((*coder).index) as size_t;
                    (*coder).sequence = SEQ_PADDING;
                    continue;
                } else {
                    (*coder).sequence = SEQ_UNPADDED;
                }
                current_block = 10048703153582371463;
            }
            2 | 3 => {
                current_block = 10048703153582371463;
            }
            5 => {
                if (*coder).pos > 0 {
                    (*coder).pos = (*coder).pos.wrapping_sub(1);
                    *out.offset(*out_pos as isize) = 0;
                    *out_pos += 1;
                    continue;
                } else {
                    (*coder).crc32 = lzma_crc32(
                        out.offset(out_start as isize),
                        (*out_pos).wrapping_sub(out_start),
                        (*coder).crc32,
                    );
                    (*coder).sequence = SEQ_CRC32;
                }
                current_block = 10175200006830010844;
            }
            6 => {
                current_block = 10175200006830010844;
            }
            _ => return LZMA_PROG_ERROR,
        }
        match current_block {
            10048703153582371463 => {
                let size: lzma_vli = if (*coder).sequence == SEQ_UNPADDED {
                    (*coder).iter.block.unpadded_size
                } else {
                    (*coder).iter.block.uncompressed_size
                };
                ret = lzma_vli_encode(size, &raw mut (*coder).pos, out, out_pos, out_size);
                if ret != LZMA_STREAM_END {
                    break;
                }
                ret = LZMA_OK;
                (*coder).pos = 0;
                (*coder).sequence += 1;
            }
            _ => {
                loop {
                    if *out_pos == out_size {
                        return LZMA_OK;
                    }
                    *out.offset(*out_pos as isize) =
                        ((*coder).crc32 >> (*coder).pos.wrapping_mul(8) & 0xff) as u8;
                    *out_pos = (*out_pos).wrapping_add(1);
                    (*coder).pos = (*coder).pos.wrapping_add(1);
                    if (*coder).pos >= 4 {
                        break;
                    }
                }
                return LZMA_STREAM_END;
            }
        }
    }
    let out_used: size_t = (*out_pos).wrapping_sub(out_start);
    if out_used > 0 {
        (*coder).crc32 = lzma_crc32(out.offset(out_start as isize), out_used, (*coder).crc32);
    }
    ret
}
unsafe extern "C" fn index_encoder_end(coder: *mut c_void, allocator: *const lzma_allocator) {
    lzma_free(coder, allocator);
}
unsafe extern "C" fn index_encoder_reset(coder: *mut lzma_index_coder, i: *const lzma_index) {
    lzma_index_iter_init(&raw mut (*coder).iter, i);
    (*coder).sequence = SEQ_INDICATOR;
    (*coder).index = i;
    (*coder).pos = 0;
    (*coder).crc32 = 0;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    i: *const lzma_index,
) -> lzma_ret {
    if core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_index,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_index_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_index,
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
                *const lzma_index,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_index_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_index,
            ) -> lzma_ret,
    ));
    if i.is_null() {
        return LZMA_PROG_ERROR;
    }
    if (*next).coder.is_null() {
        (*next).coder = lzma_alloc(core::mem::size_of::<lzma_index_coder>(), allocator);
        if (*next).coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).code = Some(
            index_encode
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
            index_encoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        );
    }
    index_encoder_reset((*next).coder as *mut lzma_index_coder, i);
    LZMA_OK
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_encoder(
    strm: *mut lzma_stream,
    i: *const lzma_index,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret =
        lzma_index_encoder_init(&raw mut (*(*strm).internal).next, (*strm).allocator, i);
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    LZMA_OK
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_buffer_encode(
    i: *const lzma_index,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    if i.is_null() || out.is_null() || out_pos.is_null() || *out_pos > out_size {
        return LZMA_PROG_ERROR;
    }
    if (out_size.wrapping_sub(*out_pos) as lzma_vli) < lzma_index_size(i) {
        return LZMA_BUF_ERROR;
    }
    let mut coder: lzma_index_coder = lzma_index_coder {
        sequence: SEQ_INDICATOR,
        index: core::ptr::null(),
        iter: lzma_index_iter {
            stream: C2RustUnnamed_2 {
                flags: core::ptr::null(),
                reserved_ptr1: core::ptr::null(),
                reserved_ptr2: core::ptr::null(),
                reserved_ptr3: core::ptr::null(),
                number: 0,
                block_count: 0,
                compressed_offset: 0,
                uncompressed_offset: 0,
                compressed_size: 0,
                uncompressed_size: 0,
                padding: 0,
                reserved_vli1: 0,
                reserved_vli2: 0,
                reserved_vli3: 0,
                reserved_vli4: 0,
            },
            block: C2RustUnnamed_1 {
                number_in_file: 0,
                compressed_file_offset: 0,
                uncompressed_file_offset: 0,
                number_in_stream: 0,
                compressed_stream_offset: 0,
                uncompressed_stream_offset: 0,
                uncompressed_size: 0,
                unpadded_size: 0,
                total_size: 0,
                reserved_vli1: 0,
                reserved_vli2: 0,
                reserved_vli3: 0,
                reserved_vli4: 0,
                reserved_ptr1: core::ptr::null(),
                reserved_ptr2: core::ptr::null(),
                reserved_ptr3: core::ptr::null(),
                reserved_ptr4: core::ptr::null(),
            },
            internal: [C2RustUnnamed_0 {
                p: core::ptr::null(),
            }; 6],
        },
        pos: 0,
        crc32: 0,
    };
    index_encoder_reset(&raw mut coder, i);
    let out_start: size_t = *out_pos;
    let mut ret: lzma_ret = index_encode(
        &raw mut coder as *mut c_void,
        core::ptr::null(),
        core::ptr::null(),
        core::ptr::null_mut(),
        0,
        out,
        out_pos,
        out_size,
        LZMA_RUN,
    );
    if ret == LZMA_STREAM_END {
        ret = LZMA_OK;
    } else {
        *out_pos = out_start;
        ret = LZMA_PROG_ERROR;
    }
    ret
}
