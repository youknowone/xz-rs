use crate::types::*;
use core::ffi::{c_int, c_uint, c_void};
#[repr(C)]
pub struct lzma_index_s {
    _opaque: [u8; 0],
}
extern "C" {
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_vli_encode(
        vli: lzma_vli,
        vli_pos: *mut size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> lzma_ret;
    fn lzma_crc32(buf: *const u8, size: size_t, crc: u32) -> u32;
    fn lzma_index_block_count(i: *const lzma_index) -> lzma_vli;
    fn lzma_index_size(i: *const lzma_index) -> lzma_vli;
    fn lzma_index_iter_init(iter: *mut lzma_index_iter, i: *const lzma_index);
    fn lzma_index_iter_next(iter: *mut lzma_index_iter, mode: lzma_index_iter_mode) -> lzma_bool;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_index_padding_size(i: *const lzma_index) -> u32;
}
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
pub const LZMA_RET_INTERNAL8: lzma_ret = 108;
pub const LZMA_RET_INTERNAL7: lzma_ret = 107;
pub const LZMA_RET_INTERNAL6: lzma_ret = 106;
pub const LZMA_RET_INTERNAL5: lzma_ret = 105;
pub const LZMA_RET_INTERNAL4: lzma_ret = 104;
pub const LZMA_RET_INTERNAL3: lzma_ret = 103;
pub const LZMA_RET_INTERNAL2: lzma_ret = 102;
pub const LZMA_RET_INTERNAL1: lzma_ret = 101;
pub const LZMA_SEEK_NEEDED: lzma_ret = 12;
pub const LZMA_PROG_ERROR: lzma_ret = 11;
pub const LZMA_BUF_ERROR: lzma_ret = 10;
pub const LZMA_DATA_ERROR: lzma_ret = 9;
pub const LZMA_OPTIONS_ERROR: lzma_ret = 8;
pub const LZMA_FORMAT_ERROR: lzma_ret = 7;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret = 6;
pub const LZMA_MEM_ERROR: lzma_ret = 5;
pub const LZMA_GET_CHECK: lzma_ret = 4;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret = 3;
pub const LZMA_NO_CHECK: lzma_ret = 2;
pub const LZMA_STREAM_END: lzma_ret = 1;
pub const LZMA_OK: lzma_ret = 0;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, size_t, size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    pub opaque: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_internal_s {
    pub next: lzma_next_coder,
    pub sequence: C2RustUnnamed,
    pub avail_in: size_t,
    pub supported_actions: [bool; 5],
    pub allow_buf_error: bool,
}
pub type C2RustUnnamed = c_uint;
pub const ISEQ_ERROR: C2RustUnnamed = 6;
pub const ISEQ_END: C2RustUnnamed = 5;
pub const ISEQ_FULL_BARRIER: C2RustUnnamed = 4;
pub const ISEQ_FINISH: C2RustUnnamed = 3;
pub const ISEQ_FULL_FLUSH: C2RustUnnamed = 2;
pub const ISEQ_SYNC_FLUSH: C2RustUnnamed = 1;
pub const ISEQ_RUN: C2RustUnnamed = 0;
pub type lzma_next_coder = lzma_next_coder_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_next_coder_s {
    pub coder: *mut c_void,
    pub id: lzma_vli,
    pub init: uintptr_t,
    pub code: lzma_code_function,
    pub end: lzma_end_function,
    pub get_progress: Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64) -> ()>,
    pub get_check: Option<unsafe extern "C" fn(*const c_void) -> lzma_check>,
    pub memconfig: Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret>,
    pub update: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const lzma_allocator,
            *const lzma_filter,
            *const lzma_filter,
        ) -> lzma_ret,
    >,
    pub set_out_limit: Option<unsafe extern "C" fn(*mut c_void, *mut u64, u64) -> lzma_ret>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut c_void,
}
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
pub type lzma_end_function = Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>;
pub type lzma_code_function = Option<
    unsafe extern "C" fn(
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
>;
pub type lzma_internal = lzma_internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream {
    pub next_in: *const u8,
    pub avail_in: size_t,
    pub total_in: u64,
    pub next_out: *mut u8,
    pub avail_out: size_t,
    pub total_out: u64,
    pub allocator: *const lzma_allocator,
    pub internal: *mut lzma_internal,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
    pub reserved_ptr3: *mut c_void,
    pub reserved_ptr4: *mut c_void,
    pub seek_pos: u64,
    pub reserved_int2: u64,
    pub reserved_int3: size_t,
    pub reserved_int4: size_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
}
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
pub type lzma_index = lzma_index_s;
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
pub type lzma_index_iter_mode = c_uint;
pub const LZMA_INDEX_ITER_NONEMPTY_BLOCK: lzma_index_iter_mode = 3;
pub const LZMA_INDEX_ITER_BLOCK: lzma_index_iter_mode = 2;
pub const LZMA_INDEX_ITER_STREAM: lzma_index_iter_mode = 1;
pub const LZMA_INDEX_ITER_ANY: lzma_index_iter_mode = 0;
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
pub const INDEX_INDICATOR: c_int = 0;
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
                *out.offset(*out_pos as isize) = INDEX_INDICATOR as u8;
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
                    let fresh0 = *out_pos;
                    *out_pos = (*out_pos).wrapping_add(1);
                    *out.offset(fresh0 as isize) = 0;
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
                        ((*coder).crc32 >> (*coder).pos.wrapping_mul(8) & 0xff as u32) as u8;
                    *out_pos = (*out_pos).wrapping_add(1);
                    (*coder).pos = (*coder).pos.wrapping_add(1);
                    if !((*coder).pos < 4) {
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
    return ret;
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
    if ::core::mem::transmute::<
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
    (*next).init = ::core::mem::transmute::<
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
        (*next).coder = lzma_alloc(
            core::mem::size_of::<lzma_index_coder>() as size_t,
            allocator,
        );
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
        ) as lzma_code_function;
        (*next).end = Some(
            index_encoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
    }
    index_encoder_reset((*next).coder as *mut lzma_index_coder, i);
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_encoder(
    strm: *mut lzma_stream,
    i: *const lzma_index,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret =
        lzma_index_encoder_init(&raw mut (*(*strm).internal).next, (*strm).allocator, i)
            as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    return LZMA_OK;
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
        index: ::core::ptr::null::<lzma_index>(),
        iter: lzma_index_iter {
            stream: C2RustUnnamed_2 {
                flags: ::core::ptr::null::<lzma_stream_flags>(),
                reserved_ptr1: ::core::ptr::null::<c_void>(),
                reserved_ptr2: ::core::ptr::null::<c_void>(),
                reserved_ptr3: ::core::ptr::null::<c_void>(),
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
                reserved_ptr1: ::core::ptr::null::<c_void>(),
                reserved_ptr2: ::core::ptr::null::<c_void>(),
                reserved_ptr3: ::core::ptr::null::<c_void>(),
                reserved_ptr4: ::core::ptr::null::<c_void>(),
            },
            internal: [C2RustUnnamed_0 {
                p: ::core::ptr::null::<c_void>(),
            }; 6],
        },
        pos: 0,
        crc32: 0,
    };
    index_encoder_reset(&raw mut coder, i);
    let out_start: size_t = *out_pos;
    let mut ret: lzma_ret = index_encode(
        &raw mut coder as *mut c_void,
        ::core::ptr::null::<lzma_allocator>(),
        ::core::ptr::null::<u8>(),
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
    return ret;
}
