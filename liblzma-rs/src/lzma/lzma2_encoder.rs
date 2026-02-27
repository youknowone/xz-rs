use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
#[repr(C)]
pub struct lzma_lzma1_encoder_s {
    _opaque: [u8; 0],
}
extern "C" {
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_bufcpy(
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> size_t;
    fn lzma_lz_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
        lz_init: Option<
            unsafe extern "C" fn(
                *mut lzma_lz_encoder,
                *const lzma_allocator,
                lzma_vli,
                *const c_void,
                *mut lzma_lz_options,
            ) -> lzma_ret,
        >,
    ) -> lzma_ret;
    fn lzma_lzma_encoder_memusage(options: *const c_void) -> u64;
    fn lzma_lzma_lclppb_encode(options: *const lzma_options_lzma, byte: *mut u8) -> bool;
    fn lzma_lzma_encoder_create(
        coder_ptr: *mut *mut c_void,
        allocator: *const lzma_allocator,
        id: lzma_vli,
        options: *const lzma_options_lzma,
        lz_options: *mut lzma_lz_options,
    ) -> lzma_ret;
    fn lzma_lzma_encoder_reset(
        coder: *mut lzma_lzma1_encoder,
        options: *const lzma_options_lzma,
    ) -> lzma_ret;
    fn lzma_lzma_encode(
        coder: *mut lzma_lzma1_encoder,
        mf: *mut lzma_mf,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
        read_limit: u32,
    ) -> lzma_ret;
    static lzma_fastpos: [u8; 8192];
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
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;
pub const LZMA_MODE_FAST: lzma_mode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_lzma {
    pub dict_size: u32,
    pub preset_dict: *const u8,
    pub preset_dict_size: u32,
    pub lc: u32,
    pub lp: u32,
    pub pb: u32,
    pub mode: lzma_mode,
    pub nice_len: u32,
    pub mf: lzma_match_finder,
    pub depth: u32,
    pub ext_flags: u32,
    pub ext_size_low: u32,
    pub ext_size_high: u32,
    pub reserved_int4: u32,
    pub reserved_int5: u32,
    pub reserved_int6: u32,
    pub reserved_int7: u32,
    pub reserved_int8: u32,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_info_s {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub options: *mut c_void,
}
pub type lzma_init_function = Option<
    unsafe extern "C" fn(
        *mut lzma_next_coder,
        *const lzma_allocator,
        *const lzma_filter_info,
    ) -> lzma_ret,
>;
pub type lzma_filter_info = lzma_filter_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_match {
    pub len: u32,
    pub dist: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_mf_s {
    pub buffer: *mut u8,
    pub size: u32,
    pub keep_size_before: u32,
    pub keep_size_after: u32,
    pub offset: u32,
    pub read_pos: u32,
    pub read_ahead: u32,
    pub read_limit: u32,
    pub write_pos: u32,
    pub pending: u32,
    pub find: Option<unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> u32>,
    pub skip: Option<unsafe extern "C" fn(*mut lzma_mf, u32) -> ()>,
    pub hash: *mut u32,
    pub son: *mut u32,
    pub cyclic_pos: u32,
    pub cyclic_size: u32,
    pub hash_mask: u32,
    pub depth: u32,
    pub nice_len: u32,
    pub match_len_max: u32,
    pub action: lzma_action,
    pub hash_count: u32,
    pub sons_count: u32,
}
pub type lzma_mf = lzma_mf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_options {
    pub before_size: size_t,
    pub dict_size: size_t,
    pub after_size: size_t,
    pub match_len_max: size_t,
    pub nice_len: size_t,
    pub match_finder: lzma_match_finder,
    pub depth: u32,
    pub preset_dict: *const u8,
    pub preset_dict_size: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_encoder {
    pub coder: *mut c_void,
    pub code: Option<
        unsafe extern "C" fn(*mut c_void, *mut lzma_mf, *mut u8, *mut size_t, size_t) -> lzma_ret,
    >,
    pub end: Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>,
    pub options_update: Option<unsafe extern "C" fn(*mut c_void, *const lzma_filter) -> lzma_ret>,
    pub set_out_limit: Option<unsafe extern "C" fn(*mut c_void, *mut u64, u64) -> lzma_ret>,
}
pub type lzma_lzma1_encoder = lzma_lzma1_encoder_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lzma2_coder {
    pub sequence: C2RustUnnamed,
    pub lzma: *mut c_void,
    pub opt_cur: lzma_options_lzma,
    pub need_properties: bool,
    pub need_state_reset: bool,
    pub need_dictionary_reset: bool,
    pub uncompressed_size: size_t,
    pub compressed_size: size_t,
    pub buf_pos: size_t,
    pub buf: [u8; 65542],
}
pub type C2RustUnnamed = c_uint;
pub const SEQ_UNCOMPRESSED_COPY: C2RustUnnamed = 4;
pub const SEQ_UNCOMPRESSED_HEADER: C2RustUnnamed = 3;
pub const SEQ_LZMA_COPY: C2RustUnnamed = 2;
pub const SEQ_LZMA_ENCODE: C2RustUnnamed = 1;
pub const SEQ_INIT: C2RustUnnamed = 0;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT32_MAX: c_uint = 4294967295;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
pub const LZMA_DICT_SIZE_MIN: c_uint = 4096;
pub const LZMA_LCLP_MAX: c_int = 4 as c_int;
pub const LZMA_PB_MAX: c_int = 4 as c_int;
#[inline]
unsafe extern "C" fn mf_unencoded(mut mf: *const lzma_mf) -> u32 {
    return (*mf)
        .write_pos
        .wrapping_sub((*mf).read_pos)
        .wrapping_add((*mf).read_ahead);
}
#[inline]
unsafe extern "C" fn mf_read(
    mut mf: *mut lzma_mf,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut left: *mut size_t,
) {
    let out_avail: size_t = out_size.wrapping_sub(*out_pos);
    let copy_size: size_t = if out_avail < *left { out_avail } else { *left };
    memcpy(
        out.offset(*out_pos as isize) as *mut c_void,
        (*mf)
            .buffer
            .offset((*mf).read_pos as isize)
            .offset(-(*left as isize)) as *const c_void,
        copy_size,
    );
    *out_pos = (*out_pos).wrapping_add(copy_size);
    *left = (*left).wrapping_sub(copy_size);
}
pub const FASTPOS_BITS: c_int = 13 as c_int;
#[inline]
unsafe extern "C" fn get_dist_slot(mut dist: u32) -> u32 {
    if dist < (1 as u32) << FASTPOS_BITS + (0 as c_int + 0 as c_int * (FASTPOS_BITS - 1 as c_int)) {
        return lzma_fastpos[dist as usize] as u32;
    }
    if dist < (1 as u32) << FASTPOS_BITS + (0 as c_int + 1 as c_int * (FASTPOS_BITS - 1 as c_int)) {
        return (lzma_fastpos[(dist >> 0 + 1 as c_int * (FASTPOS_BITS - 1 as c_int)) as usize]
            as u32)
            .wrapping_add(
                (2 as c_int * (0 as c_int + 1 as c_int * (FASTPOS_BITS - 1 as c_int))) as u32,
            );
    }
    return (lzma_fastpos[(dist >> 0 + 2 as c_int * (FASTPOS_BITS - 1 as c_int)) as usize] as u32)
        .wrapping_add(
            (2 as c_int * (0 as c_int + 2 as c_int * (FASTPOS_BITS - 1 as c_int))) as u32,
        );
}
pub const LZMA2_CHUNK_MAX: c_uint = 1u32 << 16;
pub const LZMA2_UNCOMPRESSED_MAX: c_uint = 1u32 << 21;
pub const LZMA2_HEADER_MAX: c_int = 6 as c_int;
pub const LZMA2_HEADER_UNCOMPRESSED: c_int = 3 as c_int;
unsafe extern "C" fn lzma2_header_lzma(mut coder: *mut lzma_lzma2_coder) {
    let mut pos: size_t = 0;
    if (*coder).need_properties {
        pos = 0 as size_t;
        if (*coder).need_dictionary_reset {
            (*coder).buf[pos as usize] = (0x80 as c_int + ((3 as c_int) << 5)) as u8;
        } else {
            (*coder).buf[pos as usize] = (0x80 as c_int + ((2 as c_int) << 5)) as u8;
        }
    } else {
        pos = 1 as size_t;
        if (*coder).need_state_reset {
            (*coder).buf[pos as usize] = (0x80 as c_int + ((1 as c_int) << 5)) as u8;
        } else {
            (*coder).buf[pos as usize] = 0x80 as u8;
        }
    }
    (*coder).buf_pos = pos;
    let mut size: size_t = (*coder).uncompressed_size.wrapping_sub(1 as size_t);
    let fresh1 = pos;
    pos = pos.wrapping_add(1);
    (*coder).buf[fresh1 as usize] =
        ((*coder).buf[fresh1 as usize] as size_t).wrapping_add(size >> 16) as u8 as u8;
    let fresh2 = pos;
    pos = pos.wrapping_add(1);
    (*coder).buf[fresh2 as usize] = (size >> 8 & 0xff as size_t) as u8;
    let fresh3 = pos;
    pos = pos.wrapping_add(1);
    (*coder).buf[fresh3 as usize] = (size & 0xff as size_t) as u8;
    size = (*coder).compressed_size.wrapping_sub(1 as size_t);
    let fresh4 = pos;
    pos = pos.wrapping_add(1);
    (*coder).buf[fresh4 as usize] = (size >> 8) as u8;
    let fresh5 = pos;
    pos = pos.wrapping_add(1);
    (*coder).buf[fresh5 as usize] = (size & 0xff as size_t) as u8;
    if (*coder).need_properties {
        lzma_lzma_lclppb_encode(
            &raw mut (*coder).opt_cur,
            (&raw mut (*coder).buf as *mut u8).offset(pos as isize),
        );
    }
    (*coder).need_properties = false_0 != 0;
    (*coder).need_state_reset = false_0 != 0;
    (*coder).need_dictionary_reset = false_0 != 0;
    (*coder).compressed_size = (*coder)
        .compressed_size
        .wrapping_add(LZMA2_HEADER_MAX as size_t);
}
unsafe extern "C" fn lzma2_header_uncompressed(mut coder: *mut lzma_lzma2_coder) {
    if (*coder).need_dictionary_reset {
        (*coder).buf[0] = 1 as u8;
    } else {
        (*coder).buf[0] = 2 as u8;
    }
    (*coder).need_dictionary_reset = false_0 != 0;
    (*coder).buf[1] = ((*coder).uncompressed_size.wrapping_sub(1 as size_t) >> 8) as u8;
    (*coder).buf[2] = ((*coder).uncompressed_size.wrapping_sub(1 as size_t) & 0xff as size_t) as u8;
    (*coder).buf_pos = 0 as size_t;
}
unsafe extern "C" fn lzma2_encode(
    mut coder_ptr: *mut c_void,
    mut mf: *mut lzma_mf,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    let mut coder: *mut lzma_lzma2_coder = coder_ptr as *mut lzma_lzma2_coder;
    while *out_pos < out_size {
        let mut current_block_45: u64;
        match (*coder).sequence {
            0 => {
                if mf_unencoded(mf) == 0 as u32 {
                    if (*mf).action == LZMA_FINISH {
                        let fresh0 = *out_pos;
                        *out_pos = (*out_pos).wrapping_add(1);
                        *out.offset(fresh0 as isize) = 0 as u8;
                    }
                    return (if (*mf).action == LZMA_RUN {
                        LZMA_OK as c_int
                    } else {
                        LZMA_STREAM_END as c_int
                    }) as lzma_ret;
                }
                if (*coder).need_state_reset {
                    let ret_: lzma_ret = lzma_lzma_encoder_reset(
                        (*coder).lzma as *mut lzma_lzma1_encoder,
                        &raw mut (*coder).opt_cur,
                    ) as lzma_ret;
                    if ret_ != LZMA_OK {
                        return ret_;
                    }
                }
                (*coder).uncompressed_size = 0 as size_t;
                (*coder).compressed_size = 0 as size_t;
                (*coder).sequence = SEQ_LZMA_ENCODE;
                current_block_45 = 2979737022853876585;
            }
            1 => {
                current_block_45 = 2979737022853876585;
            }
            2 => {
                current_block_45 = 13410404938545238636;
            }
            3 => {
                lzma_bufcpy(
                    &raw mut (*coder).buf as *mut u8,
                    &raw mut (*coder).buf_pos,
                    LZMA2_HEADER_UNCOMPRESSED as size_t,
                    out,
                    out_pos,
                    out_size,
                );
                if (*coder).buf_pos != LZMA2_HEADER_UNCOMPRESSED as size_t {
                    return LZMA_OK;
                }
                (*coder).sequence = SEQ_UNCOMPRESSED_COPY;
                current_block_45 = 10903800704467975402;
            }
            4 => {
                current_block_45 = 10903800704467975402;
            }
            _ => {
                current_block_45 = 11743904203796629665;
            }
        }
        match current_block_45 {
            10903800704467975402 => {
                mf_read(
                    mf,
                    out,
                    out_pos,
                    out_size,
                    &raw mut (*coder).uncompressed_size,
                );
                if (*coder).uncompressed_size != 0 as size_t {
                    return LZMA_OK;
                }
                (*coder).sequence = SEQ_INIT;
                current_block_45 = 11743904203796629665;
            }
            2979737022853876585 => {
                let left: u32 = (LZMA2_UNCOMPRESSED_MAX as size_t)
                    .wrapping_sub((*coder).uncompressed_size)
                    as u32;
                let mut limit: u32 = 0;
                if left < (*mf).match_len_max {
                    limit = 0 as u32;
                } else {
                    limit = (*mf)
                        .read_pos
                        .wrapping_sub((*mf).read_ahead)
                        .wrapping_add(left)
                        .wrapping_sub((*mf).match_len_max);
                }
                let read_start: u32 = (*mf).read_pos.wrapping_sub((*mf).read_ahead);
                let ret: lzma_ret = lzma_lzma_encode(
                    (*coder).lzma as *mut lzma_lzma1_encoder,
                    mf,
                    (&raw mut (*coder).buf as *mut u8).offset(LZMA2_HEADER_MAX as isize),
                    &raw mut (*coder).compressed_size,
                    LZMA2_CHUNK_MAX as size_t,
                    limit,
                ) as lzma_ret;
                (*coder).uncompressed_size = (*coder).uncompressed_size.wrapping_add(
                    (*mf)
                        .read_pos
                        .wrapping_sub((*mf).read_ahead)
                        .wrapping_sub(read_start) as size_t,
                );
                if ret != LZMA_STREAM_END {
                    return LZMA_OK;
                }
                if (*coder).compressed_size >= (*coder).uncompressed_size {
                    (*coder).uncompressed_size = (*coder)
                        .uncompressed_size
                        .wrapping_add((*mf).read_ahead as size_t);
                    (*mf).read_ahead = 0 as u32;
                    lzma2_header_uncompressed(coder);
                    (*coder).need_state_reset = true_0 != 0;
                    (*coder).sequence = SEQ_UNCOMPRESSED_HEADER;
                    current_block_45 = 11743904203796629665;
                } else {
                    lzma2_header_lzma(coder);
                    (*coder).sequence = SEQ_LZMA_COPY;
                    current_block_45 = 13410404938545238636;
                }
            }
            _ => {}
        }
        match current_block_45 {
            13410404938545238636 => {
                lzma_bufcpy(
                    &raw mut (*coder).buf as *mut u8,
                    &raw mut (*coder).buf_pos,
                    (*coder).compressed_size,
                    out,
                    out_pos,
                    out_size,
                );
                if (*coder).buf_pos != (*coder).compressed_size {
                    return LZMA_OK;
                }
                (*coder).sequence = SEQ_INIT;
            }
            _ => {}
        }
    }
    return LZMA_OK;
}
unsafe extern "C" fn lzma2_encoder_end(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_lzma2_coder = coder_ptr as *mut lzma_lzma2_coder;
    lzma_free((*coder).lzma, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn lzma2_encoder_options_update(
    mut coder_ptr: *mut c_void,
    mut filter: *const lzma_filter,
) -> lzma_ret {
    let mut coder: *mut lzma_lzma2_coder = coder_ptr as *mut lzma_lzma2_coder;
    if (*filter).options.is_null() || (*coder).sequence != SEQ_INIT {
        return LZMA_PROG_ERROR;
    }
    let mut opt: *const lzma_options_lzma = (*filter).options as *const lzma_options_lzma;
    if (*coder).opt_cur.lc != (*opt).lc
        || (*coder).opt_cur.lp != (*opt).lp
        || (*coder).opt_cur.pb != (*opt).pb
    {
        if (*opt).lc > LZMA_LCLP_MAX as u32
            || (*opt).lp > LZMA_LCLP_MAX as u32
            || (*opt).lc.wrapping_add((*opt).lp) > LZMA_LCLP_MAX as u32
            || (*opt).pb > LZMA_PB_MAX as u32
        {
            return LZMA_OPTIONS_ERROR;
        }
        (*coder).opt_cur.lc = (*opt).lc;
        (*coder).opt_cur.lp = (*opt).lp;
        (*coder).opt_cur.pb = (*opt).pb;
        (*coder).need_properties = true_0 != 0;
        (*coder).need_state_reset = true_0 != 0;
    }
    return LZMA_OK;
}
unsafe extern "C" fn lzma2_encoder_init(
    mut lz: *mut lzma_lz_encoder,
    mut allocator: *const lzma_allocator,
    mut id: lzma_vli,
    mut options: *const c_void,
    mut lz_options: *mut lzma_lz_options,
) -> lzma_ret {
    if options.is_null() {
        return LZMA_PROG_ERROR;
    }
    let mut coder: *mut lzma_lzma2_coder = (*lz).coder as *mut lzma_lzma2_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            ::core::mem::size_of::<lzma_lzma2_coder>() as size_t,
            allocator,
        ) as *mut lzma_lzma2_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*lz).coder = coder as *mut c_void;
        (*lz).code = Some(
            lzma2_encode
                as unsafe extern "C" fn(
                    *mut c_void,
                    *mut lzma_mf,
                    *mut u8,
                    *mut size_t,
                    size_t,
                ) -> lzma_ret,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut c_void,
                    *mut lzma_mf,
                    *mut u8,
                    *mut size_t,
                    size_t,
                ) -> lzma_ret,
            >;
        (*lz).end = Some(
            lzma2_encoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        )
            as Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>;
        (*lz).options_update = Some(
            lzma2_encoder_options_update
                as unsafe extern "C" fn(*mut c_void, *const lzma_filter) -> lzma_ret,
        )
            as Option<unsafe extern "C" fn(*mut c_void, *const lzma_filter) -> lzma_ret>;
        (*coder).lzma = NULL;
    }
    (*coder).opt_cur = *(options as *const lzma_options_lzma);
    (*coder).sequence = SEQ_INIT;
    (*coder).need_properties = true_0 != 0;
    (*coder).need_state_reset = false_0 != 0;
    (*coder).need_dictionary_reset =
        (*coder).opt_cur.preset_dict.is_null() || (*coder).opt_cur.preset_dict_size == 0 as u32;
    let ret_: lzma_ret = lzma_lzma_encoder_create(
        &raw mut (*coder).lzma,
        allocator,
        0x21 as lzma_vli,
        &raw mut (*coder).opt_cur,
        lz_options,
    ) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    if (*lz_options)
        .before_size
        .wrapping_add((*lz_options).dict_size)
        < LZMA2_CHUNK_MAX as size_t
    {
        (*lz_options).before_size =
            (LZMA2_CHUNK_MAX as size_t).wrapping_sub((*lz_options).dict_size);
    }
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_encoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
) -> lzma_ret {
    return lzma_lz_encoder_init(
        next,
        allocator,
        filters,
        Some(
            lzma2_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_lz_encoder,
                    *const lzma_allocator,
                    lzma_vli,
                    *const c_void,
                    *mut lzma_lz_options,
                ) -> lzma_ret,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_encoder_memusage(mut options: *const c_void) -> u64 {
    let lzma_mem: u64 = lzma_lzma_encoder_memusage(options) as u64;
    if lzma_mem == UINT64_MAX as u64 {
        return UINT64_MAX as u64;
    }
    return (::core::mem::size_of::<lzma_lzma2_coder>() as u64).wrapping_add(lzma_mem);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_props_encode(
    mut options: *const c_void,
    mut out: *mut u8,
) -> lzma_ret {
    if options.is_null() {
        return LZMA_PROG_ERROR;
    }
    let opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
    let mut d: u32 = if (*opt).dict_size > 4096 as u32 {
        (*opt).dict_size
    } else {
        4096 as u32
    };
    d = d.wrapping_sub(1);
    d |= d >> 2;
    d |= d >> 3;
    d |= d >> 4;
    d |= d >> 8;
    d |= d >> 16;
    if d == UINT32_MAX as u32 {
        *out.offset(0) = 40 as u8;
    } else {
        *out.offset(0) = get_dist_slot(d.wrapping_add(1 as u32)).wrapping_sub(24 as u32) as u8;
    }
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_block_size(mut options: *const c_void) -> u64 {
    let opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
    if !((*opt).dict_size >= LZMA_DICT_SIZE_MIN as u32
        && (*opt).dict_size <= ((1 as u32) << 30).wrapping_add((1 as u32) << 29))
    {
        return UINT64_MAX as u64;
    }
    return if ((*opt).dict_size as u64).wrapping_mul(3 as u64) > (1 as u64) << 20 {
        ((*opt).dict_size as u64).wrapping_mul(3 as u64)
    } else {
        (1 as u64) << 20
    };
}
