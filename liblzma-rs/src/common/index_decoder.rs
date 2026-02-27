use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
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
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_index_padding_size(i: *const lzma_index) -> u32;
    fn lzma_index_prealloc(i: *mut lzma_index, records: lzma_vli);
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
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const true_0: c_int = 1 as c_int;
pub const LZMA_VLI_MAX: c_ulonglong = UINT64_MAX.wrapping_div(2);
pub const UNPADDED_SIZE_MIN: c_ulonglong = 5;
pub const UNPADDED_SIZE_MAX: c_ulonglong = LZMA_VLI_MAX & !3;
pub const INDEX_INDICATOR: c_int = 0 as c_int;
unsafe extern "C" fn index_decode(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut action: lzma_action,
) -> lzma_ret {
    let mut current_block: u64;
    let mut coder: *mut lzma_index_coder = coder_ptr as *mut lzma_index_coder;
    let in_start: size_t = *in_pos;
    let mut ret: lzma_ret = LZMA_OK;
    while *in_pos < in_size {
        match (*coder).sequence {
            0 => {
                let fresh0 = *in_pos;
                *in_pos = (*in_pos).wrapping_add(1);
                if *in_0.offset(fresh0 as isize) as c_int != INDEX_INDICATOR {
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
                (*coder).pos = 0 as size_t;
                (*coder).sequence = SEQ_MEMUSAGE;
                current_block = 7642845755631126846;
            }
            2 => {
                current_block = 7642845755631126846;
            }
            3 | 4 => {
                let mut size: *mut lzma_vli = if (*coder).sequence == SEQ_UNPADDED {
                    &raw mut (*coder).unpadded_size
                } else {
                    &raw mut (*coder).uncompressed_size
                };
                ret = lzma_vli_decode(size, &raw mut (*coder).pos, in_0, in_pos, in_size);
                if ret != LZMA_STREAM_END {
                    break;
                }
                ret = LZMA_OK;
                (*coder).pos = 0 as size_t;
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
                    ) as lzma_ret;
                    if ret_ != LZMA_OK {
                        return ret_;
                    }
                    (*coder).count = (*coder).count.wrapping_sub(1);
                    (*coder).sequence = (if (*coder).count == 0 as lzma_vli {
                        SEQ_PADDING_INIT as c_int
                    } else {
                        SEQ_UNPADDED as c_int
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
                if (*coder).pos > 0 as size_t {
                    (*coder).pos = (*coder).pos.wrapping_sub(1);
                    let fresh1 = *in_pos;
                    *in_pos = (*in_pos).wrapping_add(1);
                    if *in_0.offset(fresh1 as isize) as c_int != 0 as c_int {
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
                if lzma_index_memusage(1 as lzma_vli, (*coder).count) > (*coder).memlimit {
                    ret = LZMA_MEMLIMIT_ERROR;
                    break;
                } else {
                    lzma_index_prealloc((*coder).index, (*coder).count);
                    ret = LZMA_OK;
                    (*coder).sequence = (if (*coder).count == 0 as lzma_vli {
                        SEQ_PADDING_INIT as c_int
                    } else {
                        SEQ_UNPADDED as c_int
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
            let fresh2 = *in_pos;
            *in_pos = (*in_pos).wrapping_add(1);
            if (*coder).crc32 >> (*coder).pos.wrapping_mul(8 as size_t) & 0xff as u32
                != *in_0.offset(fresh2 as isize) as u32
            {
                return LZMA_DATA_ERROR;
            }
            (*coder).pos = (*coder).pos.wrapping_add(1);
            if !((*coder).pos < 4 as size_t) {
                break;
            }
        }
        *(*coder).index_ptr = (*coder).index;
        (*coder).index = ::core::ptr::null_mut::<lzma_index>();
        return LZMA_STREAM_END;
    }
    let in_used: size_t = (*in_pos).wrapping_sub(in_start);
    if in_used > 0 as size_t {
        (*coder).crc32 = lzma_crc32(in_0.offset(in_start as isize), in_used, (*coder).crc32);
    }
    return ret;
}
unsafe extern "C" fn index_decoder_end(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_index_coder = coder_ptr as *mut lzma_index_coder;
    lzma_index_end((*coder).index, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn index_decoder_memconfig(
    mut coder_ptr: *mut c_void,
    mut memusage: *mut u64,
    mut old_memlimit: *mut u64,
    mut new_memlimit: u64,
) -> lzma_ret {
    let mut coder: *mut lzma_index_coder = coder_ptr as *mut lzma_index_coder;
    *memusage = lzma_index_memusage(1 as lzma_vli, (*coder).count);
    *old_memlimit = (*coder).memlimit;
    if new_memlimit != 0 as u64 {
        if new_memlimit < *memusage {
            return LZMA_MEMLIMIT_ERROR;
        }
        (*coder).memlimit = new_memlimit;
    }
    return LZMA_OK;
}
unsafe extern "C" fn index_decoder_reset(
    mut coder: *mut lzma_index_coder,
    mut allocator: *const lzma_allocator,
    mut i: *mut *mut lzma_index,
    mut memlimit: u64,
) -> lzma_ret {
    (*coder).index_ptr = i;
    *i = ::core::ptr::null_mut::<lzma_index>();
    (*coder).index = lzma_index_init(allocator);
    if (*coder).index.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*coder).sequence = SEQ_INDICATOR;
    (*coder).memlimit = if 1 as u64 > memlimit {
        1 as u64
    } else {
        memlimit
    };
    (*coder).count = 0 as lzma_vli;
    (*coder).pos = 0 as size_t;
    (*coder).crc32 = 0 as u32;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut i: *mut *mut lzma_index,
    mut memlimit: u64,
) -> lzma_ret {
    if ::core::mem::transmute::<
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
    (*next).init = ::core::mem::transmute::<
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
        coder = lzma_alloc(
            ::core::mem::size_of::<lzma_index_coder>() as size_t,
            allocator,
        ) as *mut lzma_index_coder;
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
        ) as lzma_code_function;
        (*next).end = Some(
            index_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).memconfig = Some(
            index_decoder_memconfig
                as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret,
        )
            as Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret>;
        (*coder).index = ::core::ptr::null_mut::<lzma_index>();
    } else {
        lzma_index_end((*coder).index, allocator);
    }
    return index_decoder_reset(coder, allocator, i, memlimit);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_decoder(
    mut strm: *mut lzma_stream,
    mut i: *mut *mut lzma_index,
    mut memlimit: u64,
) -> lzma_ret {
    if !i.is_null() {
        *i = ::core::ptr::null_mut::<lzma_index>();
    }
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = lzma_index_decoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        i,
        memlimit,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true_0 != 0;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true_0 != 0;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_buffer_decode(
    mut i: *mut *mut lzma_index,
    mut memlimit: *mut u64,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
) -> lzma_ret {
    if !i.is_null() {
        *i = ::core::ptr::null_mut::<lzma_index>();
    }
    if i.is_null() || memlimit.is_null() || in_0.is_null() || in_pos.is_null() || *in_pos > in_size
    {
        return LZMA_PROG_ERROR;
    }
    let mut coder: lzma_index_coder = lzma_index_coder {
        sequence: SEQ_INDICATOR,
        memlimit: 0,
        index: ::core::ptr::null_mut::<lzma_index>(),
        index_ptr: ::core::ptr::null_mut::<*mut lzma_index>(),
        count: 0,
        unpadded_size: 0,
        uncompressed_size: 0,
        pos: 0,
        crc32: 0,
    };
    let ret_: lzma_ret = index_decoder_reset(&raw mut coder, allocator, i, *memlimit) as lzma_ret;
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
        ::core::ptr::null_mut::<u8>(),
        ::core::ptr::null_mut::<size_t>(),
        0 as size_t,
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
            *memlimit = lzma_index_memusage(1 as lzma_vli, coder.count);
        }
    }
    return ret;
}
