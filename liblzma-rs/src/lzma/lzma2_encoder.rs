#[repr(C)]
pub struct lzma_lzma1_encoder_s {
    _opaque: [u8; 0],
}
extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut ::core::ffi::c_void;
    fn lzma_free(ptr: *mut ::core::ffi::c_void, allocator: *const lzma_allocator);
    fn lzma_bufcpy(
        in_0: *const uint8_t,
        in_pos: *mut size_t,
        in_size: size_t,
        out: *mut uint8_t,
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
                *const ::core::ffi::c_void,
                *mut lzma_lz_options,
            ) -> lzma_ret,
        >,
    ) -> lzma_ret;
    fn lzma_lzma_encoder_memusage(options: *const ::core::ffi::c_void) -> uint64_t;
    fn lzma_lzma_lclppb_encode(options: *const lzma_options_lzma, byte: *mut uint8_t) -> bool;
    fn lzma_lzma_encoder_create(
        coder_ptr: *mut *mut ::core::ffi::c_void,
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
        out: *mut uint8_t,
        out_pos: *mut size_t,
        out_size: size_t,
        read_limit: uint32_t,
    ) -> lzma_ret;
    static lzma_fastpos: [uint8_t; 8192];
}
pub type __darwin_size_t = usize;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type lzma_reserved_enum = ::core::ffi::c_uint;
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
pub type lzma_ret = ::core::ffi::c_uint;
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
pub type lzma_action = ::core::ffi::c_uint;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, size_t) -> *mut ::core::ffi::c_void,
    >,
    pub free:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}
pub type lzma_next_coder = lzma_next_coder_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_next_coder_s {
    pub coder: *mut ::core::ffi::c_void,
    pub id: lzma_vli,
    pub init: uintptr_t,
    pub code: lzma_code_function,
    pub end: lzma_end_function,
    pub get_progress:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, *mut uint64_t) -> ()>,
    pub get_check: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> lzma_check>,
    pub memconfig: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut uint64_t,
            *mut uint64_t,
            uint64_t,
        ) -> lzma_ret,
    >,
    pub update: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const lzma_allocator,
            *const lzma_filter,
            *const lzma_filter,
        ) -> lzma_ret,
    >,
    pub set_out_limit:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, uint64_t) -> lzma_ret>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut ::core::ffi::c_void,
}
pub type lzma_vli = uint64_t;
pub type lzma_check = ::core::ffi::c_uint;
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
pub type lzma_end_function =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_allocator) -> ()>;
pub type lzma_code_function = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const lzma_allocator,
        *const uint8_t,
        *mut size_t,
        size_t,
        *mut uint8_t,
        *mut size_t,
        size_t,
        lzma_action,
    ) -> lzma_ret,
>;
pub type lzma_match_finder = ::core::ffi::c_uint;
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub type lzma_mode = ::core::ffi::c_uint;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;
pub const LZMA_MODE_FAST: lzma_mode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_lzma {
    pub dict_size: uint32_t,
    pub preset_dict: *const uint8_t,
    pub preset_dict_size: uint32_t,
    pub lc: uint32_t,
    pub lp: uint32_t,
    pub pb: uint32_t,
    pub mode: lzma_mode,
    pub nice_len: uint32_t,
    pub mf: lzma_match_finder,
    pub depth: uint32_t,
    pub ext_flags: uint32_t,
    pub ext_size_low: uint32_t,
    pub ext_size_high: uint32_t,
    pub reserved_int4: uint32_t,
    pub reserved_int5: uint32_t,
    pub reserved_int6: uint32_t,
    pub reserved_int7: uint32_t,
    pub reserved_int8: uint32_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_ptr1: *mut ::core::ffi::c_void,
    pub reserved_ptr2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_info_s {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub options: *mut ::core::ffi::c_void,
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
    pub len: uint32_t,
    pub dist: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_mf_s {
    pub buffer: *mut uint8_t,
    pub size: uint32_t,
    pub keep_size_before: uint32_t,
    pub keep_size_after: uint32_t,
    pub offset: uint32_t,
    pub read_pos: uint32_t,
    pub read_ahead: uint32_t,
    pub read_limit: uint32_t,
    pub write_pos: uint32_t,
    pub pending: uint32_t,
    pub find: Option<unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t>,
    pub skip: Option<unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ()>,
    pub hash: *mut uint32_t,
    pub son: *mut uint32_t,
    pub cyclic_pos: uint32_t,
    pub cyclic_size: uint32_t,
    pub hash_mask: uint32_t,
    pub depth: uint32_t,
    pub nice_len: uint32_t,
    pub match_len_max: uint32_t,
    pub action: lzma_action,
    pub hash_count: uint32_t,
    pub sons_count: uint32_t,
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
    pub depth: uint32_t,
    pub preset_dict: *const uint8_t,
    pub preset_dict_size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_encoder {
    pub coder: *mut ::core::ffi::c_void,
    pub code: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut lzma_mf,
            *mut uint8_t,
            *mut size_t,
            size_t,
        ) -> lzma_ret,
    >,
    pub end: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_allocator) -> ()>,
    pub options_update:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_filter) -> lzma_ret>,
    pub set_out_limit:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, uint64_t) -> lzma_ret>,
}
pub type lzma_lzma1_encoder = lzma_lzma1_encoder_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lzma2_coder {
    pub sequence: C2RustUnnamed,
    pub lzma: *mut ::core::ffi::c_void,
    pub opt_cur: lzma_options_lzma,
    pub need_properties: bool,
    pub need_state_reset: bool,
    pub need_dictionary_reset: bool,
    pub uncompressed_size: size_t,
    pub compressed_size: size_t,
    pub buf_pos: size_t,
    pub buf: [uint8_t; 65542],
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const SEQ_UNCOMPRESSED_COPY: C2RustUnnamed = 4;
pub const SEQ_UNCOMPRESSED_HEADER: C2RustUnnamed = 3;
pub const SEQ_LZMA_COPY: C2RustUnnamed = 2;
pub const SEQ_LZMA_ENCODE: C2RustUnnamed = 1;
pub const SEQ_INIT: C2RustUnnamed = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulonglong = 18446744073709551615 as ::core::ffi::c_ulonglong;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LZMA_DICT_SIZE_MIN: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
pub const LZMA_LCLP_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LZMA_PB_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mf_unencoded(mut mf: *const lzma_mf) -> uint32_t {
    return (*mf)
        .write_pos
        .wrapping_sub((*mf).read_pos)
        .wrapping_add((*mf).read_ahead);
}
#[inline]
unsafe extern "C" fn mf_read(
    mut mf: *mut lzma_mf,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut left: *mut size_t,
) {
    let out_avail: size_t = out_size.wrapping_sub(*out_pos);
    let copy_size: size_t = if out_avail < *left { out_avail } else { *left };
    memcpy(
        out.offset(*out_pos as isize) as *mut ::core::ffi::c_void,
        (*mf)
            .buffer
            .offset((*mf).read_pos as isize)
            .offset(-(*left as isize)) as *const ::core::ffi::c_void,
        copy_size,
    );
    *out_pos = (*out_pos).wrapping_add(copy_size);
    *left = (*left).wrapping_sub(copy_size);
}
pub const FASTPOS_BITS: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn get_dist_slot(mut dist: uint32_t) -> uint32_t {
    if dist
        < (1 as uint32_t)
            << FASTPOS_BITS
                + (0 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
    {
        return lzma_fastpos[dist as usize] as uint32_t;
    }
    if dist
        < (1 as uint32_t)
            << FASTPOS_BITS
                + (0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
    {
        return (lzma_fastpos[(dist
            >> 0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
            as usize] as uint32_t)
            .wrapping_add(
                (2 as ::core::ffi::c_int
                    * (0 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int)))
                    as uint32_t,
            );
    }
    return (lzma_fastpos[(dist
        >> 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
        as usize] as uint32_t)
        .wrapping_add(
            (2 as ::core::ffi::c_int
                * (0 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int)))
                as uint32_t,
        );
}
pub const LZMA2_CHUNK_MAX: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 16 as ::core::ffi::c_int;
pub const LZMA2_UNCOMPRESSED_MAX: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 21 as ::core::ffi::c_int;
pub const LZMA2_HEADER_MAX: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LZMA2_HEADER_UNCOMPRESSED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
unsafe extern "C" fn lzma2_header_lzma(mut coder: *mut lzma_lzma2_coder) {
    let mut pos: size_t = 0;
    if (*coder).need_properties {
        pos = 0 as size_t;
        if (*coder).need_dictionary_reset {
            (*coder).buf[pos as usize] = (0x80 as ::core::ffi::c_int
                + ((3 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int))
                as uint8_t;
        } else {
            (*coder).buf[pos as usize] = (0x80 as ::core::ffi::c_int
                + ((2 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int))
                as uint8_t;
        }
    } else {
        pos = 1 as size_t;
        if (*coder).need_state_reset {
            (*coder).buf[pos as usize] = (0x80 as ::core::ffi::c_int
                + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int))
                as uint8_t;
        } else {
            (*coder).buf[pos as usize] = 0x80 as uint8_t;
        }
    }
    (*coder).buf_pos = pos;
    let mut size: size_t = (*coder).uncompressed_size.wrapping_sub(1 as size_t);
    let fresh1 = pos;
    pos = pos.wrapping_add(1);
    (*coder).buf[fresh1 as usize] = ((*coder).buf[fresh1 as usize] as size_t)
        .wrapping_add(size >> 16 as ::core::ffi::c_int)
        as uint8_t as uint8_t;
    let fresh2 = pos;
    pos = pos.wrapping_add(1);
    (*coder).buf[fresh2 as usize] = (size >> 8 as ::core::ffi::c_int & 0xff as size_t) as uint8_t;
    let fresh3 = pos;
    pos = pos.wrapping_add(1);
    (*coder).buf[fresh3 as usize] = (size & 0xff as size_t) as uint8_t;
    size = (*coder).compressed_size.wrapping_sub(1 as size_t);
    let fresh4 = pos;
    pos = pos.wrapping_add(1);
    (*coder).buf[fresh4 as usize] = (size >> 8 as ::core::ffi::c_int) as uint8_t;
    let fresh5 = pos;
    pos = pos.wrapping_add(1);
    (*coder).buf[fresh5 as usize] = (size & 0xff as size_t) as uint8_t;
    if (*coder).need_properties {
        lzma_lzma_lclppb_encode(
            &raw mut (*coder).opt_cur,
            (&raw mut (*coder).buf as *mut uint8_t).offset(pos as isize),
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
        (*coder).buf[0 as ::core::ffi::c_int as usize] = 1 as uint8_t;
    } else {
        (*coder).buf[0 as ::core::ffi::c_int as usize] = 2 as uint8_t;
    }
    (*coder).need_dictionary_reset = false_0 != 0;
    (*coder).buf[1 as ::core::ffi::c_int as usize] =
        ((*coder).uncompressed_size.wrapping_sub(1 as size_t) >> 8 as ::core::ffi::c_int)
            as uint8_t;
    (*coder).buf[2 as ::core::ffi::c_int as usize] =
        ((*coder).uncompressed_size.wrapping_sub(1 as size_t) & 0xff as size_t) as uint8_t;
    (*coder).buf_pos = 0 as size_t;
}
unsafe extern "C" fn lzma2_encode(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut mf: *mut lzma_mf,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    let mut coder: *mut lzma_lzma2_coder = coder_ptr as *mut lzma_lzma2_coder;
    while *out_pos < out_size {
        let mut current_block_45: u64;
        match (*coder).sequence as ::core::ffi::c_uint {
            0 => {
                if mf_unencoded(mf) == 0 as uint32_t {
                    if (*mf).action as ::core::ffi::c_uint
                        == LZMA_FINISH as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        let fresh0 = *out_pos;
                        *out_pos = (*out_pos).wrapping_add(1);
                        *out.offset(fresh0 as isize) = 0 as uint8_t;
                    }
                    return (if (*mf).action as ::core::ffi::c_uint
                        == LZMA_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        LZMA_OK as ::core::ffi::c_int
                    } else {
                        LZMA_STREAM_END as ::core::ffi::c_int
                    }) as lzma_ret;
                }
                if (*coder).need_state_reset {
                    let ret_: lzma_ret = lzma_lzma_encoder_reset(
                        (*coder).lzma as *mut lzma_lzma1_encoder,
                        &raw mut (*coder).opt_cur,
                    ) as lzma_ret;
                    if ret_ as ::core::ffi::c_uint
                        != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
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
                    &raw mut (*coder).buf as *mut uint8_t,
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
                let left: uint32_t = (LZMA2_UNCOMPRESSED_MAX as size_t)
                    .wrapping_sub((*coder).uncompressed_size)
                    as uint32_t;
                let mut limit: uint32_t = 0;
                if left < (*mf).match_len_max {
                    limit = 0 as uint32_t;
                } else {
                    limit = (*mf)
                        .read_pos
                        .wrapping_sub((*mf).read_ahead)
                        .wrapping_add(left)
                        .wrapping_sub((*mf).match_len_max);
                }
                let read_start: uint32_t = (*mf).read_pos.wrapping_sub((*mf).read_ahead);
                let ret: lzma_ret = lzma_lzma_encode(
                    (*coder).lzma as *mut lzma_lzma1_encoder,
                    mf,
                    (&raw mut (*coder).buf as *mut uint8_t).offset(LZMA2_HEADER_MAX as isize),
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
                if ret as ::core::ffi::c_uint
                    != LZMA_STREAM_END as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return LZMA_OK;
                }
                if (*coder).compressed_size >= (*coder).uncompressed_size {
                    (*coder).uncompressed_size = (*coder)
                        .uncompressed_size
                        .wrapping_add((*mf).read_ahead as size_t);
                    (*mf).read_ahead = 0 as uint32_t;
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
                    &raw mut (*coder).buf as *mut uint8_t,
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
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_lzma2_coder = coder_ptr as *mut lzma_lzma2_coder;
    lzma_free((*coder).lzma, allocator);
    lzma_free(coder as *mut ::core::ffi::c_void, allocator);
}
unsafe extern "C" fn lzma2_encoder_options_update(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut filter: *const lzma_filter,
) -> lzma_ret {
    let mut coder: *mut lzma_lzma2_coder = coder_ptr as *mut lzma_lzma2_coder;
    if (*filter).options.is_null()
        || (*coder).sequence as ::core::ffi::c_uint
            != SEQ_INIT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return LZMA_PROG_ERROR;
    }
    let mut opt: *const lzma_options_lzma = (*filter).options as *const lzma_options_lzma;
    if (*coder).opt_cur.lc != (*opt).lc
        || (*coder).opt_cur.lp != (*opt).lp
        || (*coder).opt_cur.pb != (*opt).pb
    {
        if (*opt).lc > LZMA_LCLP_MAX as uint32_t
            || (*opt).lp > LZMA_LCLP_MAX as uint32_t
            || (*opt).lc.wrapping_add((*opt).lp) > LZMA_LCLP_MAX as uint32_t
            || (*opt).pb > LZMA_PB_MAX as uint32_t
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
    mut options: *const ::core::ffi::c_void,
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
        (*lz).coder = coder as *mut ::core::ffi::c_void;
        (*lz).code = Some(
            lzma2_encode
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut lzma_mf,
                    *mut uint8_t,
                    *mut size_t,
                    size_t,
                ) -> lzma_ret,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut lzma_mf,
                    *mut uint8_t,
                    *mut size_t,
                    size_t,
                ) -> lzma_ret,
            >;
        (*lz).end = Some(
            lzma2_encoder_end
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_allocator) -> (),
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_allocator) -> ()>;
        (*lz).options_update = Some(
            lzma2_encoder_options_update
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_filter) -> lzma_ret,
        )
            as Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_filter) -> lzma_ret,
            >;
        (*coder).lzma = NULL;
    }
    (*coder).opt_cur = *(options as *const lzma_options_lzma);
    (*coder).sequence = SEQ_INIT;
    (*coder).need_properties = true_0 != 0;
    (*coder).need_state_reset = false_0 != 0;
    (*coder).need_dictionary_reset = (*coder).opt_cur.preset_dict.is_null()
        || (*coder).opt_cur.preset_dict_size == 0 as uint32_t;
    let ret_: lzma_ret = lzma_lzma_encoder_create(
        &raw mut (*coder).lzma,
        allocator,
        0x21 as lzma_vli,
        &raw mut (*coder).opt_cur,
        lz_options,
    ) as lzma_ret;
    if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
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
                    *const ::core::ffi::c_void,
                    *mut lzma_lz_options,
                ) -> lzma_ret,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_encoder_memusage(
    mut options: *const ::core::ffi::c_void,
) -> uint64_t {
    let lzma_mem: uint64_t = lzma_lzma_encoder_memusage(options) as uint64_t;
    if lzma_mem == UINT64_MAX as uint64_t {
        return UINT64_MAX as uint64_t;
    }
    return (::core::mem::size_of::<lzma_lzma2_coder>() as uint64_t).wrapping_add(lzma_mem);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_props_encode(
    mut options: *const ::core::ffi::c_void,
    mut out: *mut uint8_t,
) -> lzma_ret {
    if options.is_null() {
        return LZMA_PROG_ERROR;
    }
    let opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
    let mut d: uint32_t = if (*opt).dict_size > 4096 as uint32_t {
        (*opt).dict_size
    } else {
        4096 as uint32_t
    };
    d = d.wrapping_sub(1);
    d |= d >> 2 as ::core::ffi::c_int;
    d |= d >> 3 as ::core::ffi::c_int;
    d |= d >> 4 as ::core::ffi::c_int;
    d |= d >> 8 as ::core::ffi::c_int;
    d |= d >> 16 as ::core::ffi::c_int;
    if d == UINT32_MAX as uint32_t {
        *out.offset(0 as ::core::ffi::c_int as isize) = 40 as uint8_t;
    } else {
        *out.offset(0 as ::core::ffi::c_int as isize) =
            get_dist_slot(d.wrapping_add(1 as uint32_t)).wrapping_sub(24 as uint32_t) as uint8_t;
    }
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_block_size(
    mut options: *const ::core::ffi::c_void,
) -> uint64_t {
    let opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
    if !((*opt).dict_size >= LZMA_DICT_SIZE_MIN as uint32_t
        && (*opt).dict_size
            <= ((1 as uint32_t) << 30 as ::core::ffi::c_int)
                .wrapping_add((1 as uint32_t) << 29 as ::core::ffi::c_int))
    {
        return UINT64_MAX as uint64_t;
    }
    return if ((*opt).dict_size as uint64_t).wrapping_mul(3 as uint64_t)
        > (1 as uint64_t) << 20 as ::core::ffi::c_int
    {
        ((*opt).dict_size as uint64_t).wrapping_mul(3 as uint64_t)
    } else {
        (1 as uint64_t) << 20 as ::core::ffi::c_int
    };
}
