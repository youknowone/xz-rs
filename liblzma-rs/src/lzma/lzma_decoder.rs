extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut ::core::ffi::c_void;
    fn lzma_free(ptr: *mut ::core::ffi::c_void, allocator: *const lzma_allocator);
    fn lzma_lz_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
        lz_init: Option<
            unsafe extern "C" fn(
                *mut lzma_lz_decoder,
                *const lzma_allocator,
                lzma_vli,
                *const ::core::ffi::c_void,
                *mut lzma_lz_options,
            ) -> lzma_ret,
        >,
    ) -> lzma_ret;
    fn lzma_lz_decoder_memusage(dictionary_size: size_t) -> uint64_t;
}
pub type __darwin_size_t = usize;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint16_t = u16;
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
pub struct lzma_dict {
    pub buf: *mut uint8_t,
    pub pos: size_t,
    pub full: size_t,
    pub limit: size_t,
    pub size: size_t,
    pub has_wrapped: bool,
    pub need_reset: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_options {
    pub dict_size: size_t,
    pub preset_dict: *const uint8_t,
    pub preset_dict_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_decoder {
    pub coder: *mut ::core::ffi::c_void,
    pub code: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut lzma_dict,
            *const uint8_t,
            *mut size_t,
            size_t,
        ) -> lzma_ret,
    >,
    pub reset:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> ()>,
    pub set_uncompressed:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, lzma_vli, bool) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_allocator) -> ()>,
}
pub type probability = uint16_t;
pub type lzma_lzma_state = ::core::ffi::c_uint;
pub const STATE_NONLIT_REP: lzma_lzma_state = 11;
pub const STATE_NONLIT_MATCH: lzma_lzma_state = 10;
pub const STATE_LIT_SHORTREP: lzma_lzma_state = 9;
pub const STATE_LIT_LONGREP: lzma_lzma_state = 8;
pub const STATE_LIT_MATCH: lzma_lzma_state = 7;
pub const STATE_SHORTREP_LIT: lzma_lzma_state = 6;
pub const STATE_REP_LIT: lzma_lzma_state = 5;
pub const STATE_MATCH_LIT: lzma_lzma_state = 4;
pub const STATE_SHORTREP_LIT_LIT: lzma_lzma_state = 3;
pub const STATE_REP_LIT_LIT: lzma_lzma_state = 2;
pub const STATE_MATCH_LIT_LIT: lzma_lzma_state = 1;
pub const STATE_LIT_LIT: lzma_lzma_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lzma1_decoder {
    pub literal: [probability; 12288],
    pub is_match: [[probability; 16]; 12],
    pub is_rep: [probability; 12],
    pub is_rep0: [probability; 12],
    pub is_rep1: [probability; 12],
    pub is_rep2: [probability; 12],
    pub is_rep0_long: [[probability; 16]; 12],
    pub dist_slot: [[probability; 64]; 4],
    pub pos_special: [probability; 114],
    pub pos_align: [probability; 16],
    pub match_len_decoder: lzma_length_decoder,
    pub rep_len_decoder: lzma_length_decoder,
    pub rc: lzma_range_decoder,
    pub state: lzma_lzma_state,
    pub rep0: uint32_t,
    pub rep1: uint32_t,
    pub rep2: uint32_t,
    pub rep3: uint32_t,
    pub pos_mask: uint32_t,
    pub literal_context_bits: uint32_t,
    pub literal_mask: uint32_t,
    pub uncompressed_size: lzma_vli,
    pub allow_eopm: bool,
    pub sequence: C2RustUnnamed,
    pub probs: *mut probability,
    pub symbol: uint32_t,
    pub limit: uint32_t,
    pub offset: uint32_t,
    pub len: uint32_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const SEQ_COPY: C2RustUnnamed = 22;
pub const SEQ_REP_LEN_BITTREE: C2RustUnnamed = 21;
pub const SEQ_REP_LEN_CHOICE2: C2RustUnnamed = 20;
pub const SEQ_REP_LEN_CHOICE: C2RustUnnamed = 19;
pub const SEQ_IS_REP2: C2RustUnnamed = 18;
pub const SEQ_IS_REP1: C2RustUnnamed = 17;
pub const SEQ_IS_REP0_LONG: C2RustUnnamed = 16;
pub const SEQ_SHORTREP: C2RustUnnamed = 15;
pub const SEQ_IS_REP0: C2RustUnnamed = 14;
pub const SEQ_EOPM: C2RustUnnamed = 13;
pub const SEQ_ALIGN: C2RustUnnamed = 12;
pub const SEQ_DIRECT: C2RustUnnamed = 11;
pub const SEQ_DIST_MODEL: C2RustUnnamed = 10;
pub const SEQ_DIST_SLOT: C2RustUnnamed = 9;
pub const SEQ_MATCH_LEN_BITTREE: C2RustUnnamed = 8;
pub const SEQ_MATCH_LEN_CHOICE2: C2RustUnnamed = 7;
pub const SEQ_MATCH_LEN_CHOICE: C2RustUnnamed = 6;
pub const SEQ_IS_REP: C2RustUnnamed = 5;
pub const SEQ_LITERAL_WRITE: C2RustUnnamed = 4;
pub const SEQ_LITERAL_MATCHED: C2RustUnnamed = 3;
pub const SEQ_LITERAL: C2RustUnnamed = 2;
pub const SEQ_IS_MATCH: C2RustUnnamed = 1;
pub const SEQ_NORMALIZE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_range_decoder {
    pub range: uint32_t,
    pub code: uint32_t,
    pub init_bytes_left: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_length_decoder {
    pub choice: probability,
    pub choice2: probability,
    pub low: [[probability; 8]; 16],
    pub mid: [[probability; 8]; 16],
    pub high: [probability; 256],
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulonglong = 18446744073709551615 as ::core::ffi::c_ulonglong;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn read32le(mut buf: *const uint8_t) -> uint32_t {
    let mut num: uint32_t = *buf.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
    num |= (*buf.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int;
    num |= (*buf.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int;
    num |= (*buf.offset(3 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int;
    return num;
}
pub const LZMA_VLI_UNKNOWN: ::core::ffi::c_ulonglong = UINT64_MAX;
pub const LZMA_FILTER_LZMA1EXT: ::core::ffi::c_ulonglong =
    0x4000000000000002 as ::core::ffi::c_ulonglong;
pub const LZMA_LCLP_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LZMA_PB_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LZMA_LZMA1EXT_ALLOW_EOPM: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const LZ_DICT_REPEAT_MAX: ::core::ffi::c_int = 288 as ::core::ffi::c_int;
pub const LZ_DICT_INIT_POS: ::core::ffi::c_int = 2 as ::core::ffi::c_int * LZ_DICT_REPEAT_MAX;
#[inline]
unsafe extern "C" fn dict_get(dict: *const lzma_dict, distance: uint32_t) -> uint8_t {
    return *(*dict).buf.offset(
        (*dict)
            .pos
            .wrapping_sub(distance as size_t)
            .wrapping_sub(1 as size_t)
            .wrapping_add(
                (if (distance as size_t) < (*dict).pos {
                    0 as size_t
                } else {
                    (*dict).size.wrapping_sub(LZ_DICT_REPEAT_MAX as size_t)
                }),
            ) as isize,
    );
}
#[inline]
unsafe extern "C" fn dict_get0(dict: *const lzma_dict) -> uint8_t {
    return *(*dict)
        .buf
        .offset((*dict).pos.wrapping_sub(1 as size_t) as isize);
}
#[inline]
unsafe extern "C" fn dict_is_distance_valid(dict: *const lzma_dict, distance: size_t) -> bool {
    return (*dict).full > distance;
}
#[inline]
unsafe extern "C" fn dict_repeat(
    mut dict: *mut lzma_dict,
    mut distance: uint32_t,
    mut len: *mut uint32_t,
) -> bool {
    let dict_avail: size_t = (*dict).limit.wrapping_sub((*dict).pos);
    let mut left: uint32_t = (if dict_avail < *len as size_t {
        dict_avail
    } else {
        *len as size_t
    }) as uint32_t;
    *len = (*len).wrapping_sub(left);
    let mut back: size_t = (*dict)
        .pos
        .wrapping_sub(distance as size_t)
        .wrapping_sub(1 as size_t);
    if distance as size_t >= (*dict).pos {
        back = back.wrapping_add((*dict).size.wrapping_sub(LZ_DICT_REPEAT_MAX as size_t));
    }
    if distance < left {
        loop {
            let fresh0 = back;
            back = back.wrapping_add(1);
            let fresh1 = (*dict).pos;
            (*dict).pos = (*dict).pos.wrapping_add(1);
            *(*dict).buf.offset(fresh1 as isize) = *(*dict).buf.offset(fresh0 as isize);
            left = left.wrapping_sub(1);
            if !(left > 0 as uint32_t) {
                break;
            }
        }
    } else {
        memcpy(
            (*dict).buf.offset((*dict).pos as isize) as *mut ::core::ffi::c_void,
            (*dict).buf.offset(back as isize) as *const ::core::ffi::c_void,
            left as size_t,
        );
        (*dict).pos = (*dict).pos.wrapping_add(left as size_t);
    }
    if !(*dict).has_wrapped {
        (*dict).full = (*dict).pos.wrapping_sub(LZ_DICT_INIT_POS as size_t);
    }
    return *len != 0 as uint32_t;
}
#[inline]
unsafe extern "C" fn dict_put(mut dict: *mut lzma_dict, mut byte: uint8_t) {
    let fresh2 = (*dict).pos;
    (*dict).pos = (*dict).pos.wrapping_add(1);
    *(*dict).buf.offset(fresh2 as isize) = byte;
    if !(*dict).has_wrapped {
        (*dict).full = (*dict).pos.wrapping_sub(LZ_DICT_INIT_POS as size_t);
    }
}
#[inline]
unsafe extern "C" fn dict_put_safe(mut dict: *mut lzma_dict, mut byte: uint8_t) -> bool {
    if ((*dict).pos == (*dict).limit) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        return true_0 != 0;
    }
    dict_put(dict, byte);
    return false_0 != 0;
}
pub const RC_SHIFT_BITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RC_TOP_BITS: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const RC_TOP_VALUE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << RC_TOP_BITS;
pub const RC_BIT_MODEL_TOTAL_BITS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const RC_BIT_MODEL_TOTAL: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << RC_BIT_MODEL_TOTAL_BITS;
pub const RC_MOVE_BITS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn is_lclppb_valid(mut options: *const lzma_options_lzma) -> bool {
    return (*options).lc <= LZMA_LCLP_MAX as uint32_t
        && (*options).lp <= LZMA_LCLP_MAX as uint32_t
        && (*options).lc.wrapping_add((*options).lp) <= LZMA_LCLP_MAX as uint32_t
        && (*options).pb <= LZMA_PB_MAX as uint32_t;
}
pub const STATES: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const LIT_STATES: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LITERAL_CODER_SIZE: ::core::ffi::c_uint = 0x300 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn literal_init(mut probs: *mut probability, mut lc: uint32_t, mut lp: uint32_t) {
    let coders: size_t = (LITERAL_CODER_SIZE << lc.wrapping_add(lp)) as size_t;
    let mut i: size_t = 0 as size_t;
    while i < coders {
        *probs.offset(i as isize) = (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        i = i.wrapping_add(1);
    }
}
pub const MATCH_LEN_MIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LEN_LOW_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LEN_LOW_SYMBOLS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << LEN_LOW_BITS;
pub const LEN_MID_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LEN_MID_SYMBOLS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << LEN_MID_BITS;
pub const LEN_HIGH_BITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LEN_HIGH_SYMBOLS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << LEN_HIGH_BITS;
pub const DIST_STATES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DIST_SLOT_BITS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const DIST_SLOTS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << DIST_SLOT_BITS;
pub const DIST_MODEL_START: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DIST_MODEL_END: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const FULL_DISTANCES_BITS: ::core::ffi::c_int = DIST_MODEL_END / 2 as ::core::ffi::c_int;
pub const FULL_DISTANCES: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << FULL_DISTANCES_BITS;
pub const ALIGN_BITS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ALIGN_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << ALIGN_BITS;
#[inline]
unsafe extern "C" fn rc_read_init(
    mut rc: *mut lzma_range_decoder,
    mut in_0: *const uint8_t,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
) -> lzma_ret {
    while (*rc).init_bytes_left > 0 as uint32_t {
        if *in_pos == in_size {
            return LZMA_OK;
        }
        if (*rc).init_bytes_left == 5 as uint32_t
            && *in_0.offset(*in_pos as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            return LZMA_DATA_ERROR;
        }
        (*rc).code =
            (*rc).code << 8 as ::core::ffi::c_int | *in_0.offset(*in_pos as isize) as uint32_t;
        *in_pos = (*in_pos).wrapping_add(1);
        (*rc).init_bytes_left = (*rc).init_bytes_left.wrapping_sub(1);
    }
    return LZMA_STREAM_END;
}
unsafe extern "C" fn lzma_decode(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut dictptr: *mut lzma_dict,
    mut in_0: *const uint8_t,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
) -> lzma_ret {
    let mut current_block: u64;
    let mut coder: *mut lzma_lzma1_decoder = coder_ptr as *mut lzma_lzma1_decoder;
    let ret: lzma_ret = rc_read_init(&raw mut (*coder).rc, in_0, in_pos, in_size) as lzma_ret;
    if ret as ::core::ffi::c_uint != LZMA_STREAM_END as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret;
    }
    let mut dict: lzma_dict = *dictptr;
    let dict_start: size_t = dict.pos;
    let mut rc: lzma_range_decoder = (*coder).rc;
    let mut rc_in_ptr: *const uint8_t = in_0.offset(*in_pos as isize);
    let mut rc_in_end: *const uint8_t = in_0.offset(in_size as isize);
    let mut rc_in_fast_end: *const uint8_t =
        if rc_in_end.offset_from(rc_in_ptr) as ::core::ffi::c_long <= 20 as ::core::ffi::c_long {
            rc_in_ptr
        } else {
            rc_in_end.offset(-(20 as ::core::ffi::c_int as isize))
        };
    let mut rc_bound: uint32_t = 0;
    let mut state: uint32_t = (*coder).state as uint32_t;
    let mut rep0: uint32_t = (*coder).rep0;
    let mut rep1: uint32_t = (*coder).rep1;
    let mut rep2: uint32_t = (*coder).rep2;
    let mut rep3: uint32_t = (*coder).rep3;
    let pos_mask: uint32_t = (*coder).pos_mask;
    let mut probs: *mut probability = (*coder).probs;
    let mut symbol: uint32_t = (*coder).symbol;
    let mut limit: uint32_t = (*coder).limit;
    let mut offset: uint32_t = (*coder).offset;
    let mut len: uint32_t = (*coder).len;
    let literal_mask: uint32_t = (*coder).literal_mask;
    let literal_context_bits: uint32_t = (*coder).literal_context_bits;
    let mut pos_state: uint32_t = (dict.pos & pos_mask as size_t) as uint32_t;
    let mut ret_0: lzma_ret = LZMA_OK;
    let mut eopm_is_valid: bool = (*coder).uncompressed_size == LZMA_VLI_UNKNOWN as lzma_vli;
    let mut might_finish_without_eopm: bool = false_0 != 0;
    if (*coder).uncompressed_size != LZMA_VLI_UNKNOWN as lzma_vli
        && (*coder).uncompressed_size <= dict.limit.wrapping_sub(dict.pos) as lzma_vli
    {
        dict.limit = dict.pos.wrapping_add((*coder).uncompressed_size as size_t);
        might_finish_without_eopm = true_0 != 0;
    }
    match (*coder).sequence as ::core::ffi::c_uint {
        0 | 1 => {
            current_block = 5979571030476392895;
        }
        2 => {
            current_block = 13844743919235296534;
        }
        3 => {
            current_block = 18125716024132132232;
        }
        4 => {
            current_block = 10535798129821001304;
        }
        5 => {
            current_block = 3469750012682708893;
        }
        6 => {
            current_block = 1138292997408115650;
        }
        7 => {
            current_block = 13912927785247575907;
        }
        8 => {
            current_block = 592696588731961849;
        }
        9 => {
            current_block = 4174862988780014241;
        }
        10 => {
            current_block = 617447976488552541;
        }
        11 => {
            current_block = 15418612220330286504;
        }
        12 => {
            current_block = 10510472849010538284;
        }
        13 => {
            current_block = 7073645523065812117;
        }
        14 => {
            current_block = 4420799852307653083;
        }
        16 => {
            current_block = 1698084742280242340;
        }
        15 => {
            current_block = 5341942013764523046;
        }
        17 => {
            current_block = 11808118301119257848;
        }
        18 => {
            current_block = 3996983927318648760;
        }
        19 => {
            current_block = 12043352250568755004;
        }
        20 => {
            current_block = 6834592846991627977;
        }
        21 => {
            current_block = 2467942631393454738;
        }
        22 => {
            current_block = 17340485688450593529;
        }
        _ => {
            current_block = 4609795085482299213;
        }
    }
    'c_9380: loop {
        match current_block {
            4609795085482299213 => {
                (*dictptr).pos = dict.pos;
                break;
            }
            12043352250568755004 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_REP_LEN_CHOICE;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh142 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh142 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).rep_len_decoder.choice as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).rep_len_decoder.choice =
                        ((*coder).rep_len_decoder.choice as ::core::ffi::c_uint).wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                (*coder).rep_len_decoder.choice as ::core::ffi::c_uint,
                            ) >> RC_MOVE_BITS,
                        ) as probability as probability;
                    probs = &raw mut *(&raw mut (*coder).rep_len_decoder.low
                        as *mut [probability; 8])
                        .offset(pos_state as isize) as *mut probability;
                    limit = LEN_LOW_SYMBOLS as uint32_t;
                    len = MATCH_LEN_MIN as uint32_t;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).rep_len_decoder.choice = ((*coder).rep_len_decoder.choice
                        as ::core::ffi::c_int
                        - ((*coder).rep_len_decoder.choice as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    current_block = 6834592846991627977;
                    continue;
                }
                current_block = 16690975975023747857;
            }
            3996983927318648760 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_REP2;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh141 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh141 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).is_rep2[state as usize] as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_rep2[state as usize] =
                        ((*coder).is_rep2[state as usize] as ::core::ffi::c_uint).wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                (*coder).is_rep2[state as usize] as ::core::ffi::c_uint,
                            ) >> RC_MOVE_BITS,
                        ) as probability as probability;
                    let distance_3: uint32_t = rep2;
                    rep2 = rep1;
                    rep1 = rep0;
                    rep0 = distance_3;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_rep2[state as usize] = ((*coder).is_rep2[state as usize]
                        as ::core::ffi::c_int
                        - ((*coder).is_rep2[state as usize] as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    let distance_4: uint32_t = rep3;
                    rep3 = rep2;
                    rep2 = rep1;
                    rep1 = rep0;
                    rep0 = distance_4;
                }
                current_block = 15498320742470848828;
            }
            11808118301119257848 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_REP1;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh140 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh140 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).is_rep1[state as usize] as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_rep1[state as usize] =
                        ((*coder).is_rep1[state as usize] as ::core::ffi::c_uint).wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                (*coder).is_rep1[state as usize] as ::core::ffi::c_uint,
                            ) >> RC_MOVE_BITS,
                        ) as probability as probability;
                    let distance_2: uint32_t = rep1;
                    rep1 = rep0;
                    rep0 = distance_2;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_rep1[state as usize] = ((*coder).is_rep1[state as usize]
                        as ::core::ffi::c_int
                        - ((*coder).is_rep1[state as usize] as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    current_block = 3996983927318648760;
                    continue;
                }
                current_block = 15498320742470848828;
            }
            5341942013764523046 => {
                if dict_put_safe(&raw mut dict, dict_get(&raw mut dict, rep0)) {
                    (*coder).sequence = SEQ_SHORTREP;
                    current_block = 4609795085482299213;
                    continue;
                } else {
                    current_block = 4956146061682418353;
                }
            }
            1698084742280242340 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_REP0_LONG;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh139 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh139 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                    (*coder).is_rep0_long[state as usize][pos_state as usize] as uint32_t,
                );
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_rep0_long[state as usize][pos_state as usize] =
                        ((*coder).is_rep0_long[state as usize][pos_state as usize]
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    (*coder).is_rep0_long[state as usize][pos_state as usize]
                                        as ::core::ffi::c_uint,
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                    state = (if state < LIT_STATES as uint32_t {
                        STATE_LIT_SHORTREP as ::core::ffi::c_int
                    } else {
                        STATE_NONLIT_REP as ::core::ffi::c_int
                    }) as uint32_t;
                    current_block = 5341942013764523046;
                    continue;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_rep0_long[state as usize][pos_state as usize] =
                        ((*coder).is_rep0_long[state as usize][pos_state as usize]
                            as ::core::ffi::c_int
                            - ((*coder).is_rep0_long[state as usize][pos_state as usize]
                                as ::core::ffi::c_int
                                >> RC_MOVE_BITS)) as probability;
                }
                current_block = 15498320742470848828;
            }
            4420799852307653083 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_REP0;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh138 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh138 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).is_rep0[state as usize] as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_rep0[state as usize] =
                        ((*coder).is_rep0[state as usize] as ::core::ffi::c_uint).wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                (*coder).is_rep0[state as usize] as ::core::ffi::c_uint,
                            ) >> RC_MOVE_BITS,
                        ) as probability as probability;
                    current_block = 1698084742280242340;
                    continue;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_rep0[state as usize] = ((*coder).is_rep0[state as usize]
                        as ::core::ffi::c_int
                        - ((*coder).is_rep0[state as usize] as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    current_block = 11808118301119257848;
                    continue;
                }
            }
            3469750012682708893 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_REP;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh123 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh123 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).is_rep[state as usize] as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_rep[state as usize] =
                        ((*coder).is_rep[state as usize] as ::core::ffi::c_uint).wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                (*coder).is_rep[state as usize] as ::core::ffi::c_uint,
                            ) >> RC_MOVE_BITS,
                        ) as probability as probability;
                    state = (if state < LIT_STATES as uint32_t {
                        STATE_LIT_MATCH as ::core::ffi::c_int
                    } else {
                        STATE_NONLIT_MATCH as ::core::ffi::c_int
                    }) as uint32_t;
                    rep3 = rep2;
                    rep2 = rep1;
                    rep1 = rep0;
                    current_block = 1138292997408115650;
                    continue;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_rep[state as usize] = ((*coder).is_rep[state as usize]
                        as ::core::ffi::c_int
                        - ((*coder).is_rep[state as usize] as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    if !(!dict_is_distance_valid(&raw mut dict, 0 as size_t) as ::core::ffi::c_int
                        as ::core::ffi::c_long
                        != 0)
                    {
                        current_block = 4420799852307653083;
                        continue;
                    }
                    ret_0 = LZMA_DATA_ERROR;
                    current_block = 4609795085482299213;
                    continue;
                }
            }
            7073645523065812117 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_EOPM;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh137 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh137 as uint32_t;
                    }
                }
                ret_0 = (if rc.code == 0 as uint32_t {
                    LZMA_STREAM_END as ::core::ffi::c_int
                } else {
                    LZMA_DATA_ERROR as ::core::ffi::c_int
                }) as lzma_ret;
                current_block = 4609795085482299213;
                continue;
            }
            10510472849010538284 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_ALIGN;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh136 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh136 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                    (*coder).pos_align[offset.wrapping_add(symbol) as usize] as uint32_t,
                );
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).pos_align[offset.wrapping_add(symbol) as usize] =
                        ((*coder).pos_align[offset.wrapping_add(symbol) as usize]
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    (*coder).pos_align[offset.wrapping_add(symbol) as usize]
                                        as ::core::ffi::c_uint,
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).pos_align[offset.wrapping_add(symbol) as usize] =
                        ((*coder).pos_align[offset.wrapping_add(symbol) as usize]
                            as ::core::ffi::c_int
                            - ((*coder).pos_align[offset.wrapping_add(symbol) as usize]
                                as ::core::ffi::c_int
                                >> RC_MOVE_BITS)) as probability;
                    symbol = symbol.wrapping_add(offset);
                }
                offset <<= 1 as ::core::ffi::c_int;
                if offset < ALIGN_SIZE as uint32_t {
                    current_block = 10510472849010538284;
                    continue;
                }
                rep0 = rep0.wrapping_add(symbol);
                if rep0 == UINT32_MAX as uint32_t {
                    current_block = 12043253436139097694;
                } else {
                    current_block = 13383302701878543647;
                }
            }
            15418612220330286504 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_DIRECT;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh135 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh135 as uint32_t;
                    }
                }
                rc.range >>= 1 as ::core::ffi::c_int;
                rc.code = rc.code.wrapping_sub(rc.range);
                rc_bound = (0 as uint32_t).wrapping_sub(rc.code >> 31 as ::core::ffi::c_int);
                rc.code = rc.code.wrapping_add(rc.range & rc_bound);
                rep0 = (rep0 << 1 as ::core::ffi::c_int)
                    .wrapping_add(rc_bound.wrapping_add(1 as uint32_t));
                limit = limit.wrapping_sub(1);
                if limit > 0 as uint32_t {
                    current_block = 15418612220330286504;
                    continue;
                }
                rep0 <<= ALIGN_BITS;
                symbol = 0 as uint32_t;
                offset = 1 as uint32_t;
                current_block = 10510472849010538284;
                continue;
            }
            617447976488552541 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_DIST_MODEL;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh132 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh132 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    let ref mut fresh133 = *probs.offset(symbol as isize);
                    *fresh133 = (*fresh133 as ::core::ffi::c_uint).wrapping_add(
                        RC_BIT_MODEL_TOTAL
                            .wrapping_sub(*probs.offset(symbol as isize) as ::core::ffi::c_uint)
                            >> RC_MOVE_BITS,
                    ) as probability as probability;
                    symbol <<= 1 as ::core::ffi::c_int;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    let ref mut fresh134 = *probs.offset(symbol as isize);
                    *fresh134 = (*fresh134 as ::core::ffi::c_int
                        - (*probs.offset(symbol as isize) as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    symbol = (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                    rep0 = (rep0 as ::core::ffi::c_uint)
                        .wrapping_add((1 as ::core::ffi::c_uint) << offset)
                        as uint32_t as uint32_t;
                }
                offset = offset.wrapping_add(1);
                if offset < limit {
                    current_block = 617447976488552541;
                    continue;
                }
                current_block = 13383302701878543647;
            }
            4174862988780014241 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_DIST_SLOT;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh129 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh129 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    let ref mut fresh130 = *probs.offset(symbol as isize);
                    *fresh130 = (*fresh130 as ::core::ffi::c_uint).wrapping_add(
                        RC_BIT_MODEL_TOTAL
                            .wrapping_sub(*probs.offset(symbol as isize) as ::core::ffi::c_uint)
                            >> RC_MOVE_BITS,
                    ) as probability as probability;
                    symbol <<= 1 as ::core::ffi::c_int;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    let ref mut fresh131 = *probs.offset(symbol as isize);
                    *fresh131 = (*fresh131 as ::core::ffi::c_int
                        - (*probs.offset(symbol as isize) as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    symbol = (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                }
                if symbol < DIST_SLOTS as uint32_t {
                    current_block = 4174862988780014241;
                    continue;
                }
                symbol = symbol.wrapping_sub(DIST_SLOTS as uint32_t);
                if symbol < DIST_MODEL_START as uint32_t {
                    rep0 = symbol;
                } else {
                    limit = (symbol >> 1 as ::core::ffi::c_int).wrapping_sub(1 as uint32_t);
                    rep0 = (2 as uint32_t).wrapping_add(symbol & 1 as uint32_t);
                    if symbol < DIST_MODEL_END as uint32_t {
                        rep0 <<= limit;
                        probs = (&raw mut (*coder).pos_special as *mut probability)
                            .offset(rep0 as isize)
                            .offset(-(symbol as isize))
                            .offset(-(1 as ::core::ffi::c_int as isize));
                        symbol = 1 as uint32_t;
                        offset = 0 as uint32_t;
                        current_block = 617447976488552541;
                        continue;
                    } else {
                        limit = limit.wrapping_sub(ALIGN_BITS as uint32_t);
                        current_block = 15418612220330286504;
                        continue;
                    }
                }
                current_block = 13383302701878543647;
            }
            592696588731961849 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_MATCH_LEN_BITTREE;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh126 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh126 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    let ref mut fresh127 = *probs.offset(symbol as isize);
                    *fresh127 = (*fresh127 as ::core::ffi::c_uint).wrapping_add(
                        RC_BIT_MODEL_TOTAL
                            .wrapping_sub(*probs.offset(symbol as isize) as ::core::ffi::c_uint)
                            >> RC_MOVE_BITS,
                    ) as probability as probability;
                    symbol <<= 1 as ::core::ffi::c_int;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    let ref mut fresh128 = *probs.offset(symbol as isize);
                    *fresh128 = (*fresh128 as ::core::ffi::c_int
                        - (*probs.offset(symbol as isize) as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    symbol = (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                }
                if symbol < limit {
                    current_block = 592696588731961849;
                    continue;
                }
                len = len.wrapping_add(symbol.wrapping_sub(limit));
                probs = &raw mut *(&raw mut (*coder).dist_slot as *mut [probability; 64]).offset(
                    (if len < (DIST_STATES + MATCH_LEN_MIN) as uint32_t {
                        len.wrapping_sub(MATCH_LEN_MIN as uint32_t)
                    } else {
                        (DIST_STATES - 1 as ::core::ffi::c_int) as uint32_t
                    }) as isize,
                ) as *mut probability;
                symbol = 1 as uint32_t;
                current_block = 4174862988780014241;
                continue;
            }
            13912927785247575907 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_MATCH_LEN_CHOICE2;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh125 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh125 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).match_len_decoder.choice2 as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).match_len_decoder.choice2 =
                        ((*coder).match_len_decoder.choice2 as ::core::ffi::c_uint).wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                (*coder).match_len_decoder.choice2 as ::core::ffi::c_uint,
                            ) >> RC_MOVE_BITS,
                        ) as probability as probability;
                    probs = &raw mut *(&raw mut (*coder).match_len_decoder.mid
                        as *mut [probability; 8])
                        .offset(pos_state as isize) as *mut probability;
                    limit = LEN_MID_SYMBOLS as uint32_t;
                    len = (MATCH_LEN_MIN + LEN_LOW_SYMBOLS) as uint32_t;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).match_len_decoder.choice2 =
                        ((*coder).match_len_decoder.choice2 as ::core::ffi::c_int
                            - ((*coder).match_len_decoder.choice2 as ::core::ffi::c_int
                                >> RC_MOVE_BITS)) as probability;
                    probs = &raw mut (*coder).match_len_decoder.high as *mut probability;
                    limit = LEN_HIGH_SYMBOLS as uint32_t;
                    len = (MATCH_LEN_MIN + LEN_LOW_SYMBOLS + LEN_MID_SYMBOLS) as uint32_t;
                }
                current_block = 8485842003490715114;
            }
            1138292997408115650 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_MATCH_LEN_CHOICE;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh124 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh124 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).match_len_decoder.choice as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).match_len_decoder.choice =
                        ((*coder).match_len_decoder.choice as ::core::ffi::c_uint).wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                (*coder).match_len_decoder.choice as ::core::ffi::c_uint,
                            ) >> RC_MOVE_BITS,
                        ) as probability as probability;
                    probs = &raw mut *(&raw mut (*coder).match_len_decoder.low
                        as *mut [probability; 8])
                        .offset(pos_state as isize) as *mut probability;
                    limit = LEN_LOW_SYMBOLS as uint32_t;
                    len = MATCH_LEN_MIN as uint32_t;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).match_len_decoder.choice = ((*coder).match_len_decoder.choice
                        as ::core::ffi::c_int
                        - ((*coder).match_len_decoder.choice as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    current_block = 13912927785247575907;
                    continue;
                }
                current_block = 8485842003490715114;
            }
            10535798129821001304 => {
                if dict_put_safe(&raw mut dict, symbol as uint8_t) {
                    (*coder).sequence = SEQ_LITERAL_WRITE;
                    current_block = 4609795085482299213;
                    continue;
                } else {
                    current_block = 4956146061682418353;
                }
            }
            18125716024132132232 => {
                let match_bit: uint32_t = len & offset;
                let subcoder_index: uint32_t = offset.wrapping_add(match_bit).wrapping_add(symbol);
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_LITERAL_MATCHED;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh120 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh120 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(subcoder_index as isize) as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    let ref mut fresh121 = *probs.offset(subcoder_index as isize);
                    *fresh121 = (*fresh121 as ::core::ffi::c_uint).wrapping_add(
                        RC_BIT_MODEL_TOTAL.wrapping_sub(
                            *probs.offset(subcoder_index as isize) as ::core::ffi::c_uint
                        ) >> RC_MOVE_BITS,
                    ) as probability as probability;
                    symbol <<= 1 as ::core::ffi::c_int;
                    offset &= !match_bit;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    let ref mut fresh122 = *probs.offset(subcoder_index as isize);
                    *fresh122 = (*fresh122 as ::core::ffi::c_int
                        - (*probs.offset(subcoder_index as isize) as ::core::ffi::c_int
                            >> RC_MOVE_BITS)) as probability;
                    symbol = (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                    offset &= match_bit;
                }
                len <<= 1 as ::core::ffi::c_int;
                if symbol < ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t {
                    current_block = 18125716024132132232;
                    continue;
                } else {
                    current_block = 10535798129821001304;
                    continue;
                }
            }
            5979571030476392895 => {
                if (might_finish_without_eopm as ::core::ffi::c_int != 0 && dict.pos == dict.limit)
                    as ::core::ffi::c_int as ::core::ffi::c_long
                    != 0
                {
                    if rc.range < RC_TOP_VALUE as uint32_t {
                        if rc_in_ptr == rc_in_end {
                            (*coder).sequence = SEQ_NORMALIZE;
                            current_block = 4609795085482299213;
                            continue;
                        } else {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh115 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh115 as uint32_t;
                        }
                    }
                    if rc.code == 0 as uint32_t {
                        ret_0 = LZMA_STREAM_END;
                        current_block = 4609795085482299213;
                        continue;
                    } else if !(*coder).allow_eopm {
                        ret_0 = LZMA_DATA_ERROR;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        eopm_is_valid = true_0 != 0;
                    }
                }
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_MATCH;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh116 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh116 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                    (*coder).is_match[state as usize][pos_state as usize] as uint32_t,
                );
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_match[state as usize][pos_state as usize] =
                        ((*coder).is_match[state as usize][pos_state as usize]
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    (*coder).is_match[state as usize][pos_state as usize]
                                        as ::core::ffi::c_uint,
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                    probs = (&raw mut (*coder).literal as *mut probability).offset(
                        (3 as size_t).wrapping_mul(
                            ((dict.pos << 8 as ::core::ffi::c_int)
                                .wrapping_add(dict_get0(&raw mut dict) as size_t)
                                & literal_mask as size_t)
                                << literal_context_bits,
                        ) as isize,
                    );
                    symbol = 1 as uint32_t;
                    if state < LIT_STATES as uint32_t {
                        state = if state <= STATE_SHORTREP_LIT_LIT as ::core::ffi::c_int as uint32_t
                        {
                            STATE_LIT_LIT as ::core::ffi::c_int as uint32_t
                        } else {
                            state.wrapping_sub(3 as uint32_t)
                        };
                        current_block = 13844743919235296534;
                        continue;
                    } else {
                        state = if state <= STATE_LIT_SHORTREP as ::core::ffi::c_int as uint32_t {
                            state.wrapping_sub(3 as uint32_t)
                        } else {
                            state.wrapping_sub(6 as uint32_t)
                        };
                        len =
                            (dict_get(&raw mut dict, rep0) as uint32_t) << 1 as ::core::ffi::c_int;
                        offset = 0x100 as uint32_t;
                        current_block = 18125716024132132232;
                        continue;
                    }
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_match[state as usize][pos_state as usize] =
                        ((*coder).is_match[state as usize][pos_state as usize]
                            as ::core::ffi::c_int
                            - ((*coder).is_match[state as usize][pos_state as usize]
                                as ::core::ffi::c_int
                                >> RC_MOVE_BITS)) as probability;
                    current_block = 3469750012682708893;
                    continue;
                }
            }
            13844743919235296534 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_LITERAL;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh117 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh117 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    let ref mut fresh118 = *probs.offset(symbol as isize);
                    *fresh118 = (*fresh118 as ::core::ffi::c_uint).wrapping_add(
                        RC_BIT_MODEL_TOTAL
                            .wrapping_sub(*probs.offset(symbol as isize) as ::core::ffi::c_uint)
                            >> RC_MOVE_BITS,
                    ) as probability as probability;
                    symbol <<= 1 as ::core::ffi::c_int;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    let ref mut fresh119 = *probs.offset(symbol as isize);
                    *fresh119 = (*fresh119 as ::core::ffi::c_int
                        - (*probs.offset(symbol as isize) as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    symbol = (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                }
                if symbol < ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t {
                    current_block = 13844743919235296534;
                    continue;
                } else {
                    current_block = 10535798129821001304;
                    continue;
                }
            }
            2467942631393454738 => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_REP_LEN_BITTREE;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh144 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh144 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    let ref mut fresh145 = *probs.offset(symbol as isize);
                    *fresh145 = (*fresh145 as ::core::ffi::c_uint).wrapping_add(
                        RC_BIT_MODEL_TOTAL
                            .wrapping_sub(*probs.offset(symbol as isize) as ::core::ffi::c_uint)
                            >> RC_MOVE_BITS,
                    ) as probability as probability;
                    symbol <<= 1 as ::core::ffi::c_int;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    let ref mut fresh146 = *probs.offset(symbol as isize);
                    *fresh146 = (*fresh146 as ::core::ffi::c_int
                        - (*probs.offset(symbol as isize) as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    symbol = (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                }
                if symbol < limit {
                    current_block = 2467942631393454738;
                    continue;
                }
                len = len.wrapping_add(symbol.wrapping_sub(limit));
                current_block = 17340485688450593529;
                continue;
            }
            17340485688450593529 => {
                if dict_repeat(&raw mut dict, rep0, &raw mut len) as ::core::ffi::c_long != 0 {
                    (*coder).sequence = SEQ_COPY;
                    current_block = 4609795085482299213;
                    continue;
                } else {
                    current_block = 4956146061682418353;
                }
            }
            _ => {
                if rc.range < RC_TOP_VALUE as uint32_t {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_REP_LEN_CHOICE2;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh143 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh143 as uint32_t;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).rep_len_decoder.choice2 as uint32_t);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).rep_len_decoder.choice2 =
                        ((*coder).rep_len_decoder.choice2 as ::core::ffi::c_uint).wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                (*coder).rep_len_decoder.choice2 as ::core::ffi::c_uint,
                            ) >> RC_MOVE_BITS,
                        ) as probability as probability;
                    probs = &raw mut *(&raw mut (*coder).rep_len_decoder.mid
                        as *mut [probability; 8])
                        .offset(pos_state as isize) as *mut probability;
                    limit = LEN_MID_SYMBOLS as uint32_t;
                    len = (MATCH_LEN_MIN + LEN_LOW_SYMBOLS) as uint32_t;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).rep_len_decoder.choice2 = ((*coder).rep_len_decoder.choice2
                        as ::core::ffi::c_int
                        - ((*coder).rep_len_decoder.choice2 as ::core::ffi::c_int >> RC_MOVE_BITS))
                        as probability;
                    probs = &raw mut (*coder).rep_len_decoder.high as *mut probability;
                    limit = LEN_HIGH_SYMBOLS as uint32_t;
                    len = (MATCH_LEN_MIN + LEN_LOW_SYMBOLS + LEN_MID_SYMBOLS) as uint32_t;
                }
                current_block = 16690975975023747857;
            }
        }
        match current_block {
            13383302701878543647 => {
                if !(!dict_is_distance_valid(&raw mut dict, rep0 as size_t) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0)
                {
                    current_block = 17340485688450593529;
                    continue;
                }
                ret_0 = LZMA_DATA_ERROR;
                current_block = 4609795085482299213;
                continue;
            }
            4956146061682418353 => loop {
                pos_state = (dict.pos & pos_mask as size_t) as uint32_t;
                if (!(rc_in_ptr < rc_in_fast_end) || dict.pos == dict.limit) as ::core::ffi::c_int
                    as ::core::ffi::c_long
                    != 0
                {
                    current_block = 5979571030476392895;
                    continue 'c_9380;
                }
                if rc.range < RC_TOP_VALUE as uint32_t {
                    rc.range <<= RC_SHIFT_BITS;
                    let fresh3 = rc_in_ptr;
                    rc_in_ptr = rc_in_ptr.offset(1);
                    rc.code = rc.code << RC_SHIFT_BITS | *fresh3 as uint32_t;
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                    (*coder).is_match[state as usize][pos_state as usize] as uint32_t,
                );
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_match[state as usize][pos_state as usize] =
                        ((*coder).is_match[state as usize][pos_state as usize]
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    (*coder).is_match[state as usize][pos_state as usize]
                                        as ::core::ffi::c_uint,
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                    probs = (&raw mut (*coder).literal as *mut probability).offset(
                        (3 as size_t).wrapping_mul(
                            ((dict.pos << 8 as ::core::ffi::c_int)
                                .wrapping_add(dict_get0(&raw mut dict) as size_t)
                                & literal_mask as size_t)
                                << literal_context_bits,
                        ) as isize,
                    );
                    if state < LIT_STATES as uint32_t {
                        state = if state <= STATE_SHORTREP_LIT_LIT as ::core::ffi::c_int as uint32_t
                        {
                            STATE_LIT_LIT as ::core::ffi::c_int as uint32_t
                        } else {
                            state.wrapping_sub(3 as uint32_t)
                        };
                        symbol = 1 as uint32_t;
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh4 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh4 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh5 = *probs.offset(symbol as isize);
                            *fresh5 = (*fresh5 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh6 = *probs.offset(symbol as isize);
                            *fresh6 = (*fresh6 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh7 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh7 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh8 = *probs.offset(symbol as isize);
                            *fresh8 = (*fresh8 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh9 = *probs.offset(symbol as isize);
                            *fresh9 = (*fresh9 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh10 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh10 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh11 = *probs.offset(symbol as isize);
                            *fresh11 = (*fresh11 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh12 = *probs.offset(symbol as isize);
                            *fresh12 = (*fresh12 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh13 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh13 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh14 = *probs.offset(symbol as isize);
                            *fresh14 = (*fresh14 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh15 = *probs.offset(symbol as isize);
                            *fresh15 = (*fresh15 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh16 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh16 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh17 = *probs.offset(symbol as isize);
                            *fresh17 = (*fresh17 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh18 = *probs.offset(symbol as isize);
                            *fresh18 = (*fresh18 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh19 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh19 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh20 = *probs.offset(symbol as isize);
                            *fresh20 = (*fresh20 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh21 = *probs.offset(symbol as isize);
                            *fresh21 = (*fresh21 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh22 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh22 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh23 = *probs.offset(symbol as isize);
                            *fresh23 = (*fresh23 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh24 = *probs.offset(symbol as isize);
                            *fresh24 = (*fresh24 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh25 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh25 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh26 = *probs.offset(symbol as isize);
                            *fresh26 = (*fresh26 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh27 = *probs.offset(symbol as isize);
                            *fresh27 = (*fresh27 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        symbol = symbol.wrapping_add(0 as ::core::ffi::c_int as uint32_t);
                    } else {
                        state = if state <= STATE_LIT_SHORTREP as ::core::ffi::c_int as uint32_t {
                            state.wrapping_sub(3 as uint32_t)
                        } else {
                            state.wrapping_sub(6 as uint32_t)
                        };
                        let mut t_match_byte: uint32_t = dict_get(&raw mut dict, rep0) as uint32_t;
                        let mut t_match_bit: uint32_t = 0;
                        let mut t_subcoder_index: uint32_t = 0;
                        let mut t_offset: uint32_t = 0x100 as uint32_t;
                        symbol = 1 as uint32_t;
                        t_match_byte <<= 1 as ::core::ffi::c_int;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh28 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh28 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh29 = *probs.offset(t_subcoder_index as isize);
                            *fresh29 = (*fresh29 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL
                                    .wrapping_sub(*probs.offset(t_subcoder_index as isize)
                                        as ::core::ffi::c_uint)
                                    >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh30 = *probs.offset(t_subcoder_index as isize);
                            *fresh30 = (*fresh30 as ::core::ffi::c_int
                                - (*probs.offset(t_subcoder_index as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1 as ::core::ffi::c_int;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh31 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh31 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh32 = *probs.offset(t_subcoder_index as isize);
                            *fresh32 = (*fresh32 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL
                                    .wrapping_sub(*probs.offset(t_subcoder_index as isize)
                                        as ::core::ffi::c_uint)
                                    >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh33 = *probs.offset(t_subcoder_index as isize);
                            *fresh33 = (*fresh33 as ::core::ffi::c_int
                                - (*probs.offset(t_subcoder_index as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1 as ::core::ffi::c_int;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh34 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh34 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh35 = *probs.offset(t_subcoder_index as isize);
                            *fresh35 = (*fresh35 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL
                                    .wrapping_sub(*probs.offset(t_subcoder_index as isize)
                                        as ::core::ffi::c_uint)
                                    >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh36 = *probs.offset(t_subcoder_index as isize);
                            *fresh36 = (*fresh36 as ::core::ffi::c_int
                                - (*probs.offset(t_subcoder_index as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1 as ::core::ffi::c_int;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh37 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh37 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh38 = *probs.offset(t_subcoder_index as isize);
                            *fresh38 = (*fresh38 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL
                                    .wrapping_sub(*probs.offset(t_subcoder_index as isize)
                                        as ::core::ffi::c_uint)
                                    >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh39 = *probs.offset(t_subcoder_index as isize);
                            *fresh39 = (*fresh39 as ::core::ffi::c_int
                                - (*probs.offset(t_subcoder_index as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1 as ::core::ffi::c_int;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh40 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh40 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh41 = *probs.offset(t_subcoder_index as isize);
                            *fresh41 = (*fresh41 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL
                                    .wrapping_sub(*probs.offset(t_subcoder_index as isize)
                                        as ::core::ffi::c_uint)
                                    >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh42 = *probs.offset(t_subcoder_index as isize);
                            *fresh42 = (*fresh42 as ::core::ffi::c_int
                                - (*probs.offset(t_subcoder_index as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1 as ::core::ffi::c_int;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh43 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh43 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh44 = *probs.offset(t_subcoder_index as isize);
                            *fresh44 = (*fresh44 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL
                                    .wrapping_sub(*probs.offset(t_subcoder_index as isize)
                                        as ::core::ffi::c_uint)
                                    >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh45 = *probs.offset(t_subcoder_index as isize);
                            *fresh45 = (*fresh45 as ::core::ffi::c_int
                                - (*probs.offset(t_subcoder_index as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1 as ::core::ffi::c_int;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh46 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh46 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh47 = *probs.offset(t_subcoder_index as isize);
                            *fresh47 = (*fresh47 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL
                                    .wrapping_sub(*probs.offset(t_subcoder_index as isize)
                                        as ::core::ffi::c_uint)
                                    >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh48 = *probs.offset(t_subcoder_index as isize);
                            *fresh48 = (*fresh48 as ::core::ffi::c_int
                                - (*probs.offset(t_subcoder_index as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1 as ::core::ffi::c_int;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh49 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh49 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh50 = *probs.offset(t_subcoder_index as isize);
                            *fresh50 = (*fresh50 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL
                                    .wrapping_sub(*probs.offset(t_subcoder_index as isize)
                                        as ::core::ffi::c_uint)
                                    >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh51 = *probs.offset(t_subcoder_index as isize);
                            *fresh51 = (*fresh51 as ::core::ffi::c_int
                                - (*probs.offset(t_subcoder_index as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                            t_offset &= t_match_bit;
                        }
                    }
                    dict_put(&raw mut dict, symbol as uint8_t);
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_match[state as usize][pos_state as usize] =
                        ((*coder).is_match[state as usize][pos_state as usize]
                            as ::core::ffi::c_int
                            - ((*coder).is_match[state as usize][pos_state as usize]
                                as ::core::ffi::c_int
                                >> RC_MOVE_BITS)) as probability;
                    if rc.range < RC_TOP_VALUE as uint32_t {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh52 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh52 as uint32_t;
                    }
                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                        .wrapping_mul((*coder).is_rep[state as usize] as uint32_t);
                    if rc.code < rc_bound {
                        rc.range = rc_bound;
                        (*coder).is_rep[state as usize] =
                            ((*coder).is_rep[state as usize] as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    (*coder).is_rep[state as usize] as ::core::ffi::c_uint,
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                        state = (if state < LIT_STATES as uint32_t {
                            STATE_LIT_MATCH as ::core::ffi::c_int
                        } else {
                            STATE_NONLIT_MATCH as ::core::ffi::c_int
                        }) as uint32_t;
                        rep3 = rep2;
                        rep2 = rep1;
                        rep1 = rep0;
                        symbol = 1 as uint32_t;
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh53 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh53 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul((*coder).match_len_decoder.choice as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            (*coder).match_len_decoder.choice =
                                ((*coder).match_len_decoder.choice as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        RC_BIT_MODEL_TOTAL.wrapping_sub(
                                            (*coder).match_len_decoder.choice
                                                as ::core::ffi::c_uint,
                                        ) >> RC_MOVE_BITS,
                                    ) as probability as probability;
                            symbol = 1 as uint32_t;
                            if rc.range < RC_TOP_VALUE as uint32_t {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh54 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh54 as uint32_t;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                (*coder).match_len_decoder.low[pos_state as usize][symbol as usize]
                                    as uint32_t,
                            );
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = ((*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        RC_BIT_MODEL_TOTAL.wrapping_sub(
                                            (*coder).match_len_decoder.low[pos_state as usize]
                                                [symbol as usize]
                                                as ::core::ffi::c_uint,
                                        ) >> RC_MOVE_BITS,
                                    )
                                    as probability
                                    as probability;
                                symbol <<= 1 as ::core::ffi::c_int;
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = ((*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    as ::core::ffi::c_int
                                    - ((*coder).match_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_int
                                        >> RC_MOVE_BITS))
                                    as probability;
                                symbol =
                                    (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                            }
                            if rc.range < RC_TOP_VALUE as uint32_t {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh55 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh55 as uint32_t;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                (*coder).match_len_decoder.low[pos_state as usize][symbol as usize]
                                    as uint32_t,
                            );
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = ((*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        RC_BIT_MODEL_TOTAL.wrapping_sub(
                                            (*coder).match_len_decoder.low[pos_state as usize]
                                                [symbol as usize]
                                                as ::core::ffi::c_uint,
                                        ) >> RC_MOVE_BITS,
                                    )
                                    as probability
                                    as probability;
                                symbol <<= 1 as ::core::ffi::c_int;
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = ((*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    as ::core::ffi::c_int
                                    - ((*coder).match_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_int
                                        >> RC_MOVE_BITS))
                                    as probability;
                                symbol =
                                    (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                            }
                            if rc.range < RC_TOP_VALUE as uint32_t {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh56 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh56 as uint32_t;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                (*coder).match_len_decoder.low[pos_state as usize][symbol as usize]
                                    as uint32_t,
                            );
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = ((*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        RC_BIT_MODEL_TOTAL.wrapping_sub(
                                            (*coder).match_len_decoder.low[pos_state as usize]
                                                [symbol as usize]
                                                as ::core::ffi::c_uint,
                                        ) >> RC_MOVE_BITS,
                                    )
                                    as probability
                                    as probability;
                                symbol <<= 1 as ::core::ffi::c_int;
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = ((*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    as ::core::ffi::c_int
                                    - ((*coder).match_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_int
                                        >> RC_MOVE_BITS))
                                    as probability;
                                symbol =
                                    (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                            }
                            symbol = symbol.wrapping_add(
                                (-((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                    + 2 as ::core::ffi::c_int)
                                    as uint32_t,
                            );
                            len = symbol;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            (*coder).match_len_decoder.choice = ((*coder).match_len_decoder.choice
                                as ::core::ffi::c_int
                                - ((*coder).match_len_decoder.choice as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            if rc.range < RC_TOP_VALUE as uint32_t {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh57 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh57 as uint32_t;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                .wrapping_mul((*coder).match_len_decoder.choice2 as uint32_t);
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).match_len_decoder.choice2 =
                                    ((*coder).match_len_decoder.choice2 as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).match_len_decoder.choice2
                                                    as ::core::ffi::c_uint,
                                            ) >> RC_MOVE_BITS,
                                        ) as probability
                                        as probability;
                                symbol = 1 as uint32_t;
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh58 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh58 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize]
                                        as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = ((*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).match_len_decoder.mid[pos_state as usize]
                                                    [symbol as usize]
                                                    as ::core::ffi::c_uint,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = ((*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_int
                                        - ((*coder).match_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh59 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh59 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize]
                                        as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = ((*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).match_len_decoder.mid[pos_state as usize]
                                                    [symbol as usize]
                                                    as ::core::ffi::c_uint,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = ((*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_int
                                        - ((*coder).match_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh60 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh60 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize]
                                        as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = ((*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).match_len_decoder.mid[pos_state as usize]
                                                    [symbol as usize]
                                                    as ::core::ffi::c_uint,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = ((*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_int
                                        - ((*coder).match_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                symbol = symbol.wrapping_add(
                                    (-((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                        + 2 as ::core::ffi::c_int
                                        + ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int))
                                        as uint32_t,
                                );
                                len = symbol;
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).match_len_decoder.choice2 =
                                    ((*coder).match_len_decoder.choice2 as ::core::ffi::c_int
                                        - ((*coder).match_len_decoder.choice2
                                            as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                symbol = 1 as uint32_t;
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh61 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh61 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh62 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh62 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh63 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh63 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh64 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh64 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh65 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh65 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh66 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh66 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh67 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh67 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh68 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh68 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                symbol = symbol.wrapping_add(
                                    (-((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                        + 2 as ::core::ffi::c_int
                                        + ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                        + ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int))
                                        as uint32_t,
                                );
                                len = symbol;
                            }
                        }
                        probs = &raw mut *(&raw mut (*coder).dist_slot as *mut [probability; 64])
                            .offset(
                                (if len < (DIST_STATES + MATCH_LEN_MIN) as uint32_t {
                                    len.wrapping_sub(MATCH_LEN_MIN as uint32_t)
                                } else {
                                    (DIST_STATES - 1 as ::core::ffi::c_int) as uint32_t
                                }) as isize,
                            ) as *mut probability;
                        symbol = 1 as uint32_t;
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh69 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh69 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh70 = *probs.offset(symbol as isize);
                            *fresh70 = (*fresh70 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh71 = *probs.offset(symbol as isize);
                            *fresh71 = (*fresh71 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh72 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh72 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh73 = *probs.offset(symbol as isize);
                            *fresh73 = (*fresh73 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh74 = *probs.offset(symbol as isize);
                            *fresh74 = (*fresh74 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh75 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh75 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh76 = *probs.offset(symbol as isize);
                            *fresh76 = (*fresh76 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh77 = *probs.offset(symbol as isize);
                            *fresh77 = (*fresh77 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh78 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh78 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh79 = *probs.offset(symbol as isize);
                            *fresh79 = (*fresh79 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh80 = *probs.offset(symbol as isize);
                            *fresh80 = (*fresh80 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh81 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh81 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh82 = *probs.offset(symbol as isize);
                            *fresh82 = (*fresh82 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh83 = *probs.offset(symbol as isize);
                            *fresh83 = (*fresh83 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        if rc.range < RC_TOP_VALUE as uint32_t {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh84 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh84 as uint32_t;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            let ref mut fresh85 = *probs.offset(symbol as isize);
                            *fresh85 = (*fresh85 as ::core::ffi::c_uint).wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    *probs.offset(symbol as isize) as ::core::ffi::c_uint
                                ) >> RC_MOVE_BITS,
                            ) as probability as probability;
                            symbol <<= 1 as ::core::ffi::c_int;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            let ref mut fresh86 = *probs.offset(symbol as isize);
                            *fresh86 = (*fresh86 as ::core::ffi::c_int
                                - (*probs.offset(symbol as isize) as ::core::ffi::c_int
                                    >> RC_MOVE_BITS))
                                as probability;
                            symbol =
                                (symbol << 1 as ::core::ffi::c_int).wrapping_add(1 as uint32_t);
                        }
                        symbol = symbol.wrapping_add(
                            -((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t,
                        );
                        if symbol < DIST_MODEL_START as uint32_t {
                            rep0 = symbol;
                        } else {
                            limit = (symbol >> 1 as ::core::ffi::c_int).wrapping_sub(1 as uint32_t);
                            rep0 = (2 as uint32_t).wrapping_add(symbol & 1 as uint32_t);
                            if symbol < DIST_MODEL_END as uint32_t {
                                rep0 <<= limit;
                                probs = (&raw mut (*coder).pos_special as *mut probability)
                                    .offset(rep0 as isize)
                                    .offset(-(symbol as isize))
                                    .offset(-(1 as ::core::ffi::c_int as isize));
                                symbol = 1 as uint32_t;
                                offset = 1 as uint32_t;
                                loop {
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh87 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh87 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                        .wrapping_mul(*probs.offset(symbol as isize) as uint32_t);
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        let ref mut fresh88 = *probs.offset(symbol as isize);
                                        *fresh88 = (*fresh88 as ::core::ffi::c_uint).wrapping_add(
                                            RC_BIT_MODEL_TOTAL
                                                .wrapping_sub(*probs.offset(symbol as isize)
                                                    as ::core::ffi::c_uint)
                                                >> RC_MOVE_BITS,
                                        )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        let ref mut fresh89 = *probs.offset(symbol as isize);
                                        *fresh89 = (*fresh89 as ::core::ffi::c_int
                                            - (*probs.offset(symbol as isize)
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                        rep0 = rep0.wrapping_add(offset);
                                    }
                                    offset <<= 1 as ::core::ffi::c_int;
                                    limit = limit.wrapping_sub(1);
                                    if !(limit > 0 as uint32_t) {
                                        break;
                                    }
                                }
                            } else {
                                limit = limit.wrapping_sub(ALIGN_BITS as uint32_t);
                                loop {
                                    rep0 = (rep0 << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh90 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh90 as uint32_t;
                                    }
                                    rc.range >>= 1 as ::core::ffi::c_int;
                                    rc.code = rc.code.wrapping_sub(rc.range);
                                    rc_bound = (0 as uint32_t)
                                        .wrapping_sub(rc.code >> 31 as ::core::ffi::c_int);
                                    rep0 = rep0.wrapping_add(rc_bound);
                                    rc.code = rc.code.wrapping_add(rc.range & rc_bound);
                                    limit = limit.wrapping_sub(1);
                                    if !(limit > 0 as uint32_t) {
                                        break;
                                    }
                                }
                                rep0 <<= ALIGN_BITS;
                                symbol = 0 as uint32_t;
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh91 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh91 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).pos_align[symbol.wrapping_add(1 as uint32_t) as usize]
                                        as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).pos_align
                                        [symbol.wrapping_add(1 as uint32_t) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(1 as uint32_t) as usize]
                                        as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).pos_align
                                                    [symbol.wrapping_add(1 as uint32_t) as usize]
                                                    as ::core::ffi::c_uint,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).pos_align
                                        [symbol.wrapping_add(1 as uint32_t) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(1 as uint32_t) as usize]
                                        as ::core::ffi::c_int
                                        - ((*coder).pos_align
                                            [symbol.wrapping_add(1 as uint32_t) as usize]
                                            as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                    symbol = symbol.wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh92 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh92 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).pos_align[symbol.wrapping_add(2 as uint32_t) as usize]
                                        as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).pos_align
                                        [symbol.wrapping_add(2 as uint32_t) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(2 as uint32_t) as usize]
                                        as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).pos_align
                                                    [symbol.wrapping_add(2 as uint32_t) as usize]
                                                    as ::core::ffi::c_uint,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).pos_align
                                        [symbol.wrapping_add(2 as uint32_t) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(2 as uint32_t) as usize]
                                        as ::core::ffi::c_int
                                        - ((*coder).pos_align
                                            [symbol.wrapping_add(2 as uint32_t) as usize]
                                            as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                    symbol = symbol.wrapping_add(2 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh93 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh93 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).pos_align[symbol.wrapping_add(4 as uint32_t) as usize]
                                        as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).pos_align
                                        [symbol.wrapping_add(4 as uint32_t) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(4 as uint32_t) as usize]
                                        as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).pos_align
                                                    [symbol.wrapping_add(4 as uint32_t) as usize]
                                                    as ::core::ffi::c_uint,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).pos_align
                                        [symbol.wrapping_add(4 as uint32_t) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(4 as uint32_t) as usize]
                                        as ::core::ffi::c_int
                                        - ((*coder).pos_align
                                            [symbol.wrapping_add(4 as uint32_t) as usize]
                                            as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                    symbol = symbol.wrapping_add(4 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh94 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh94 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).pos_align[symbol.wrapping_add(8 as uint32_t) as usize]
                                        as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).pos_align
                                        [symbol.wrapping_add(8 as uint32_t) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(8 as uint32_t) as usize]
                                        as ::core::ffi::c_uint)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).pos_align
                                                    [symbol.wrapping_add(8 as uint32_t) as usize]
                                                    as ::core::ffi::c_uint,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).pos_align
                                        [symbol.wrapping_add(8 as uint32_t) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(8 as uint32_t) as usize]
                                        as ::core::ffi::c_int
                                        - ((*coder).pos_align
                                            [symbol.wrapping_add(8 as uint32_t) as usize]
                                            as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                    symbol = symbol.wrapping_add(8 as uint32_t);
                                }
                                rep0 = rep0.wrapping_add(symbol);
                                if rep0 == UINT32_MAX as uint32_t {
                                    break;
                                }
                            }
                        }
                        if !dict_is_distance_valid(&raw mut dict, rep0 as size_t)
                            as ::core::ffi::c_int as ::core::ffi::c_long
                            != 0
                        {
                            ret_0 = LZMA_DATA_ERROR;
                            current_block = 4609795085482299213;
                            continue 'c_9380;
                        }
                    } else {
                        rc.range = rc.range.wrapping_sub(rc_bound);
                        rc.code = rc.code.wrapping_sub(rc_bound);
                        (*coder).is_rep[state as usize] =
                            ((*coder).is_rep[state as usize] as ::core::ffi::c_int
                                - ((*coder).is_rep[state as usize] as ::core::ffi::c_int
                                    >> RC_MOVE_BITS)) as probability;
                        if !dict_is_distance_valid(&raw mut dict, 0 as size_t) as ::core::ffi::c_int
                            as ::core::ffi::c_long
                            != 0
                        {
                            ret_0 = LZMA_DATA_ERROR;
                            current_block = 4609795085482299213;
                            continue 'c_9380;
                        } else {
                            if rc.range < RC_TOP_VALUE as uint32_t {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh95 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh95 as uint32_t;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                .wrapping_mul((*coder).is_rep0[state as usize] as uint32_t);
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).is_rep0[state as usize] = ((*coder).is_rep0[state as usize]
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        RC_BIT_MODEL_TOTAL.wrapping_sub(
                                            (*coder).is_rep0[state as usize] as ::core::ffi::c_uint,
                                        ) >> RC_MOVE_BITS,
                                    )
                                    as probability
                                    as probability;
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh96 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh96 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).is_rep0_long[state as usize][pos_state as usize]
                                        as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).is_rep0_long[state as usize][pos_state as usize] =
                                        ((*coder).is_rep0_long[state as usize][pos_state as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).is_rep0_long[state as usize]
                                                        [pos_state as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    state = (if state < LIT_STATES as uint32_t {
                                        STATE_LIT_SHORTREP as ::core::ffi::c_int
                                    } else {
                                        STATE_NONLIT_REP as ::core::ffi::c_int
                                    }) as uint32_t;
                                    dict_put(&raw mut dict, dict_get(&raw mut dict, rep0));
                                    continue;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).is_rep0_long[state as usize][pos_state as usize] =
                                        ((*coder).is_rep0_long[state as usize][pos_state as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).is_rep0_long[state as usize]
                                                [pos_state as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                }
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).is_rep0[state as usize] = ((*coder).is_rep0[state as usize]
                                    as ::core::ffi::c_int
                                    - ((*coder).is_rep0[state as usize] as ::core::ffi::c_int
                                        >> RC_MOVE_BITS))
                                    as probability;
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh97 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh97 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                    .wrapping_mul((*coder).is_rep1[state as usize] as uint32_t);
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).is_rep1[state as usize] =
                                        ((*coder).is_rep1[state as usize] as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).is_rep1[state as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    let distance: uint32_t = rep1;
                                    rep1 = rep0;
                                    rep0 = distance;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).is_rep1[state as usize] = ((*coder).is_rep1
                                        [state as usize]
                                        as ::core::ffi::c_int
                                        - ((*coder).is_rep1[state as usize] as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh98 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh98 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                        .wrapping_mul((*coder).is_rep2[state as usize] as uint32_t);
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).is_rep2[state as usize] = ((*coder).is_rep2
                                            [state as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).is_rep2[state as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        let distance_0: uint32_t = rep2;
                                        rep2 = rep1;
                                        rep1 = rep0;
                                        rep0 = distance_0;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).is_rep2[state as usize] =
                                            ((*coder).is_rep2[state as usize] as ::core::ffi::c_int
                                                - ((*coder).is_rep2[state as usize]
                                                    as ::core::ffi::c_int
                                                    >> RC_MOVE_BITS))
                                                as probability;
                                        let distance_1: uint32_t = rep3;
                                        rep3 = rep2;
                                        rep2 = rep1;
                                        rep1 = rep0;
                                        rep0 = distance_1;
                                    }
                                }
                            }
                            state = (if state < LIT_STATES as uint32_t {
                                STATE_LIT_LONGREP as ::core::ffi::c_int
                            } else {
                                STATE_NONLIT_REP as ::core::ffi::c_int
                            }) as uint32_t;
                            symbol = 1 as uint32_t;
                            if rc.range < RC_TOP_VALUE as uint32_t {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh99 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh99 as uint32_t;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                .wrapping_mul((*coder).rep_len_decoder.choice as uint32_t);
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).rep_len_decoder.choice = ((*coder).rep_len_decoder.choice
                                    as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        RC_BIT_MODEL_TOTAL.wrapping_sub(
                                            (*coder).rep_len_decoder.choice as ::core::ffi::c_uint,
                                        ) >> RC_MOVE_BITS,
                                    )
                                    as probability
                                    as probability;
                                symbol = 1 as uint32_t;
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh100 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh100 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] =
                                        ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                                        [symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] = ((*coder).rep_len_decoder.low
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_int
                                        - ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh101 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh101 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] =
                                        ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                                        [symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] = ((*coder).rep_len_decoder.low
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_int
                                        - ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh102 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh102 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        as uint32_t,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] =
                                        ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                                        [symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1 as ::core::ffi::c_int;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] = ((*coder).rep_len_decoder.low
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as ::core::ffi::c_int
                                        - ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_int
                                            >> RC_MOVE_BITS))
                                        as probability;
                                    symbol = (symbol << 1 as ::core::ffi::c_int)
                                        .wrapping_add(1 as uint32_t);
                                }
                                symbol = symbol.wrapping_add(
                                    (-((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                        + 2 as ::core::ffi::c_int)
                                        as uint32_t,
                                );
                                len = symbol;
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).rep_len_decoder.choice = ((*coder).rep_len_decoder.choice
                                    as ::core::ffi::c_int
                                    - ((*coder).rep_len_decoder.choice as ::core::ffi::c_int
                                        >> RC_MOVE_BITS))
                                    as probability;
                                if rc.range < RC_TOP_VALUE as uint32_t {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh103 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh103 as uint32_t;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                    .wrapping_mul((*coder).rep_len_decoder.choice2 as uint32_t);
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).rep_len_decoder.choice2 =
                                        ((*coder).rep_len_decoder.choice2 as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.choice2
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol = 1 as uint32_t;
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh104 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh104 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            as uint32_t,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = ((*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.mid[pos_state as usize]
                                                        [symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = ((*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).rep_len_decoder.mid[pos_state as usize]
                                                [symbol as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                    }
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh105 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh105 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            as uint32_t,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = ((*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.mid[pos_state as usize]
                                                        [symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = ((*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).rep_len_decoder.mid[pos_state as usize]
                                                [symbol as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                    }
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh106 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh106 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            as uint32_t,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = ((*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.mid[pos_state as usize]
                                                        [symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = ((*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            as ::core::ffi::c_int
                                            - ((*coder).rep_len_decoder.mid[pos_state as usize]
                                                [symbol as usize]
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                    }
                                    symbol = symbol.wrapping_add(
                                        (-((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                                            + 2 as ::core::ffi::c_int
                                            + ((1 as ::core::ffi::c_int)
                                                << 3 as ::core::ffi::c_int))
                                            as uint32_t,
                                    );
                                    len = symbol;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).rep_len_decoder.choice2 =
                                        ((*coder).rep_len_decoder.choice2 as ::core::ffi::c_int
                                            - ((*coder).rep_len_decoder.choice2
                                                as ::core::ffi::c_int
                                                >> RC_MOVE_BITS))
                                            as probability;
                                    symbol = 1 as uint32_t;
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh107 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh107 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as uint32_t,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] = ((*coder)
                                            .rep_len_decoder
                                            .high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    as ::core::ffi::c_int
                                                    >> RC_MOVE_BITS))
                                                as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                    }
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh108 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh108 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as uint32_t,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] = ((*coder)
                                            .rep_len_decoder
                                            .high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    as ::core::ffi::c_int
                                                    >> RC_MOVE_BITS))
                                                as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                    }
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh109 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh109 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as uint32_t,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] = ((*coder)
                                            .rep_len_decoder
                                            .high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    as ::core::ffi::c_int
                                                    >> RC_MOVE_BITS))
                                                as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                    }
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh110 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh110 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as uint32_t,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] = ((*coder)
                                            .rep_len_decoder
                                            .high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    as ::core::ffi::c_int
                                                    >> RC_MOVE_BITS))
                                                as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                    }
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh111 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh111 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as uint32_t,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] = ((*coder)
                                            .rep_len_decoder
                                            .high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    as ::core::ffi::c_int
                                                    >> RC_MOVE_BITS))
                                                as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                    }
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh112 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh112 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as uint32_t,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] = ((*coder)
                                            .rep_len_decoder
                                            .high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    as ::core::ffi::c_int
                                                    >> RC_MOVE_BITS))
                                                as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                    }
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh113 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh113 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as uint32_t,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] = ((*coder)
                                            .rep_len_decoder
                                            .high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    as ::core::ffi::c_int
                                                    >> RC_MOVE_BITS))
                                                as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                    }
                                    if rc.range < RC_TOP_VALUE as uint32_t {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh114 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh114 as uint32_t;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as uint32_t,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] = ((*coder)
                                            .rep_len_decoder
                                            .high[symbol as usize]
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.high[symbol as usize]
                                                        as ::core::ffi::c_uint,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1 as ::core::ffi::c_int;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize]
                                                as ::core::ffi::c_int
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    as ::core::ffi::c_int
                                                    >> RC_MOVE_BITS))
                                                as probability;
                                        symbol = (symbol << 1 as ::core::ffi::c_int)
                                            .wrapping_add(1 as uint32_t);
                                    }
                                    symbol = symbol.wrapping_add(
                                        (-((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                            + 2 as ::core::ffi::c_int
                                            + ((1 as ::core::ffi::c_int)
                                                << 3 as ::core::ffi::c_int)
                                            + ((1 as ::core::ffi::c_int)
                                                << 3 as ::core::ffi::c_int))
                                            as uint32_t,
                                    );
                                    len = symbol;
                                }
                            }
                        }
                    }
                    if !(dict_repeat(&raw mut dict, rep0, &raw mut len) as ::core::ffi::c_long != 0)
                    {
                        continue;
                    }
                    (*coder).sequence = SEQ_COPY;
                    current_block = 4609795085482299213;
                    continue 'c_9380;
                }
            },
            16690975975023747857 => {
                symbol = 1 as uint32_t;
                current_block = 2467942631393454738;
                continue;
            }
            15498320742470848828 => {
                state = (if state < LIT_STATES as uint32_t {
                    STATE_LIT_LONGREP as ::core::ffi::c_int
                } else {
                    STATE_NONLIT_REP as ::core::ffi::c_int
                }) as uint32_t;
                current_block = 12043352250568755004;
                continue;
            }
            8485842003490715114 => {
                symbol = 1 as uint32_t;
                current_block = 592696588731961849;
                continue;
            }
            _ => {}
        }
        if eopm_is_valid {
            current_block = 7073645523065812117;
            continue;
        }
        ret_0 = LZMA_DATA_ERROR;
        current_block = 4609795085482299213;
    }
    (*dictptr).full = dict.full;
    (*coder).rc = rc;
    *in_pos = rc_in_ptr.offset_from(in_0) as ::core::ffi::c_long as size_t;
    (*coder).state = state as lzma_lzma_state;
    (*coder).rep0 = rep0;
    (*coder).rep1 = rep1;
    (*coder).rep2 = rep2;
    (*coder).rep3 = rep3;
    (*coder).probs = probs;
    (*coder).symbol = symbol;
    (*coder).limit = limit;
    (*coder).offset = offset;
    (*coder).len = len;
    if (*coder).uncompressed_size != LZMA_VLI_UNKNOWN as lzma_vli {
        (*coder).uncompressed_size = (*coder)
            .uncompressed_size
            .wrapping_sub(dict.pos.wrapping_sub(dict_start) as lzma_vli);
        if (*coder).uncompressed_size == 0 as lzma_vli
            && ret_0 as ::core::ffi::c_uint == LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            && ((*coder).sequence as ::core::ffi::c_uint
                == SEQ_LITERAL_WRITE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*coder).sequence as ::core::ffi::c_uint
                    == SEQ_SHORTREP as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*coder).sequence as ::core::ffi::c_uint
                    == SEQ_COPY as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            ret_0 = LZMA_DATA_ERROR;
        }
    }
    if ret_0 as ::core::ffi::c_uint == LZMA_STREAM_END as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*coder).rc.range = UINT32_MAX as uint32_t;
        (*coder).rc.code = 0 as uint32_t;
        (*coder).rc.init_bytes_left = 5 as uint32_t;
        (*coder).sequence = SEQ_IS_MATCH;
    }
    return ret_0;
}
unsafe extern "C" fn lzma_decoder_uncompressed(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut uncompressed_size: lzma_vli,
    mut allow_eopm: bool,
) {
    let mut coder: *mut lzma_lzma1_decoder = coder_ptr as *mut lzma_lzma1_decoder;
    (*coder).uncompressed_size = uncompressed_size;
    (*coder).allow_eopm = allow_eopm;
}
unsafe extern "C" fn lzma_decoder_reset(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut opt: *const ::core::ffi::c_void,
) {
    let mut coder: *mut lzma_lzma1_decoder = coder_ptr as *mut lzma_lzma1_decoder;
    let mut options: *const lzma_options_lzma = opt as *const lzma_options_lzma;
    (*coder).pos_mask = ((1 as ::core::ffi::c_uint) << (*options).pb)
        .wrapping_sub(1 as ::core::ffi::c_uint) as uint32_t;
    literal_init(
        &raw mut (*coder).literal as *mut probability,
        (*options).lc,
        (*options).lp,
    );
    (*coder).literal_context_bits = (*options).lc;
    (*coder).literal_mask = ((0x100 as ::core::ffi::c_uint) << (*options).lp)
        .wrapping_sub(0x100 as ::core::ffi::c_uint >> (*options).lc)
        as uint32_t;
    (*coder).state = STATE_LIT_LIT;
    (*coder).rep0 = 0 as uint32_t;
    (*coder).rep1 = 0 as uint32_t;
    (*coder).rep2 = 0 as uint32_t;
    (*coder).rep3 = 0 as uint32_t;
    (*coder).pos_mask = ((1 as ::core::ffi::c_uint) << (*options).pb)
        .wrapping_sub(1 as ::core::ffi::c_uint) as uint32_t;
    (*coder).rc.range = UINT32_MAX as uint32_t;
    (*coder).rc.code = 0 as uint32_t;
    (*coder).rc.init_bytes_left = 5 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < STATES as uint32_t {
        let mut j: uint32_t = 0 as uint32_t;
        while j <= (*coder).pos_mask {
            (*coder).is_match[i as usize][j as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            (*coder).is_rep0_long[i as usize][j as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            j = j.wrapping_add(1);
        }
        (*coder).is_rep[i as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        (*coder).is_rep0[i as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        (*coder).is_rep1[i as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        (*coder).is_rep2[i as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        i = i.wrapping_add(1);
    }
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < DIST_STATES as uint32_t {
        let mut bt_i: uint32_t = 0 as uint32_t;
        while bt_i < ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t {
            (*coder).dist_slot[i_0 as usize][bt_i as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            bt_i = bt_i.wrapping_add(1);
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: uint32_t = 0 as uint32_t;
    while i_1 < (FULL_DISTANCES - DIST_MODEL_END) as uint32_t {
        (*coder).pos_special[i_1 as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        i_1 = i_1.wrapping_add(1);
    }
    let mut bt_i_0: uint32_t = 0 as uint32_t;
    while bt_i_0 < ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint32_t {
        (*coder).pos_align[bt_i_0 as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        bt_i_0 = bt_i_0.wrapping_add(1);
    }
    let num_pos_states: uint32_t = (1 as uint32_t) << (*options).pb;
    (*coder).match_len_decoder.choice =
        (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
    (*coder).match_len_decoder.choice2 =
        (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
    (*coder).rep_len_decoder.choice =
        (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
    (*coder).rep_len_decoder.choice2 =
        (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
    let mut pos_state: uint32_t = 0 as uint32_t;
    while pos_state < num_pos_states {
        let mut bt_i_1: uint32_t = 0 as uint32_t;
        while bt_i_1 < ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t {
            (*coder).match_len_decoder.low[pos_state as usize][bt_i_1 as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            bt_i_1 = bt_i_1.wrapping_add(1);
        }
        let mut bt_i_2: uint32_t = 0 as uint32_t;
        while bt_i_2 < ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t {
            (*coder).match_len_decoder.mid[pos_state as usize][bt_i_2 as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            bt_i_2 = bt_i_2.wrapping_add(1);
        }
        let mut bt_i_3: uint32_t = 0 as uint32_t;
        while bt_i_3 < ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t {
            (*coder).rep_len_decoder.low[pos_state as usize][bt_i_3 as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            bt_i_3 = bt_i_3.wrapping_add(1);
        }
        let mut bt_i_4: uint32_t = 0 as uint32_t;
        while bt_i_4 < ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t {
            (*coder).rep_len_decoder.mid[pos_state as usize][bt_i_4 as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            bt_i_4 = bt_i_4.wrapping_add(1);
        }
        pos_state = pos_state.wrapping_add(1);
    }
    let mut bt_i_5: uint32_t = 0 as uint32_t;
    while bt_i_5 < ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t {
        (*coder).match_len_decoder.high[bt_i_5 as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        bt_i_5 = bt_i_5.wrapping_add(1);
    }
    let mut bt_i_6: uint32_t = 0 as uint32_t;
    while bt_i_6 < ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t {
        (*coder).rep_len_decoder.high[bt_i_6 as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        bt_i_6 = bt_i_6.wrapping_add(1);
    }
    (*coder).sequence = SEQ_IS_MATCH;
    (*coder).probs = ::core::ptr::null_mut::<probability>();
    (*coder).symbol = 0 as uint32_t;
    (*coder).limit = 0 as uint32_t;
    (*coder).offset = 0 as uint32_t;
    (*coder).len = 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_decoder_create(
    mut lz: *mut lzma_lz_decoder,
    mut allocator: *const lzma_allocator,
    mut options: *const lzma_options_lzma,
    mut lz_options: *mut lzma_lz_options,
) -> lzma_ret {
    if (*lz).coder.is_null() {
        (*lz).coder = lzma_alloc(
            ::core::mem::size_of::<lzma_lzma1_decoder>() as size_t,
            allocator,
        );
        if (*lz).coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*lz).code = Some(
            lzma_decode
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut lzma_dict,
                    *const uint8_t,
                    *mut size_t,
                    size_t,
                ) -> lzma_ret,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut lzma_dict,
                    *const uint8_t,
                    *mut size_t,
                    size_t,
                ) -> lzma_ret,
            >;
        (*lz).reset = Some(
            lzma_decoder_reset
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> (),
        )
            as Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void) -> (),
            >;
        (*lz).set_uncompressed = Some(
            lzma_decoder_uncompressed
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, lzma_vli, bool) -> (),
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, lzma_vli, bool) -> ()>;
    }
    (*lz_options).dict_size = (*options).dict_size as size_t;
    (*lz_options).preset_dict = (*options).preset_dict;
    (*lz_options).preset_dict_size = (*options).preset_dict_size as size_t;
    return LZMA_OK;
}
unsafe extern "C" fn lzma_decoder_init(
    mut lz: *mut lzma_lz_decoder,
    mut allocator: *const lzma_allocator,
    mut id: lzma_vli,
    mut options: *const ::core::ffi::c_void,
    mut lz_options: *mut lzma_lz_options,
) -> lzma_ret {
    if !is_lclppb_valid(options as *const lzma_options_lzma) {
        return LZMA_PROG_ERROR;
    }
    let mut uncomp_size: lzma_vli = LZMA_VLI_UNKNOWN as lzma_vli;
    let mut allow_eopm: bool = true_0 != 0;
    if id == LZMA_FILTER_LZMA1EXT as lzma_vli {
        let mut opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
        if (*opt).ext_flags & !(LZMA_LZMA1EXT_ALLOW_EOPM as uint32_t) != 0 {
            return LZMA_OPTIONS_ERROR;
        }
        uncomp_size = ((*opt).ext_size_low as uint64_t)
            .wrapping_add(((*opt).ext_size_high as uint64_t) << 32 as ::core::ffi::c_int)
            as lzma_vli;
        allow_eopm = (*opt).ext_flags & LZMA_LZMA1EXT_ALLOW_EOPM as uint32_t != 0 as uint32_t
            || uncomp_size == LZMA_VLI_UNKNOWN as lzma_vli;
    }
    let ret_: lzma_ret = lzma_lzma_decoder_create(
        lz,
        allocator,
        options as *const lzma_options_lzma,
        lz_options,
    ) as lzma_ret;
    if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret_;
    }
    lzma_decoder_reset((*lz).coder, options);
    lzma_decoder_uncompressed((*lz).coder, uncomp_size, allow_eopm);
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
) -> lzma_ret {
    return lzma_lz_decoder_init(
        next,
        allocator,
        filters,
        Some(
            lzma_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_lz_decoder,
                    *const lzma_allocator,
                    lzma_vli,
                    *const ::core::ffi::c_void,
                    *mut lzma_lz_options,
                ) -> lzma_ret,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_lclppb_decode(
    mut options: *mut lzma_options_lzma,
    mut byte: uint8_t,
) -> bool {
    if byte as ::core::ffi::c_int
        > (4 as ::core::ffi::c_int * 5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            * 9 as ::core::ffi::c_int
            + 8 as ::core::ffi::c_int
    {
        return true_0 != 0;
    }
    (*options).pb = (byte as ::core::ffi::c_int
        / (9 as ::core::ffi::c_int * 5 as ::core::ffi::c_int)) as uint32_t;
    byte = (byte as uint32_t).wrapping_sub(
        (*options)
            .pb
            .wrapping_mul(9 as uint32_t)
            .wrapping_mul(5 as uint32_t),
    ) as uint8_t as uint8_t;
    (*options).lp = (byte as ::core::ffi::c_int / 9 as ::core::ffi::c_int) as uint32_t;
    (*options).lc = (byte as uint32_t).wrapping_sub((*options).lp.wrapping_mul(9 as uint32_t));
    return (*options).lc.wrapping_add((*options).lp) > LZMA_LCLP_MAX as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_decoder_memusage_nocheck(
    mut options: *const ::core::ffi::c_void,
) -> uint64_t {
    let opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
    return (::core::mem::size_of::<lzma_lzma1_decoder>() as uint64_t)
        .wrapping_add(lzma_lz_decoder_memusage((*opt).dict_size as size_t));
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_decoder_memusage(
    mut options: *const ::core::ffi::c_void,
) -> uint64_t {
    if !is_lclppb_valid(options as *const lzma_options_lzma) {
        return UINT64_MAX as uint64_t;
    }
    return lzma_lzma_decoder_memusage_nocheck(options);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_props_decode(
    mut options: *mut *mut ::core::ffi::c_void,
    mut allocator: *const lzma_allocator,
    mut props: *const uint8_t,
    mut props_size: size_t,
) -> lzma_ret {
    if props_size != 5 as size_t {
        return LZMA_OPTIONS_ERROR;
    }
    let mut opt: *mut lzma_options_lzma = lzma_alloc(
        ::core::mem::size_of::<lzma_options_lzma>() as size_t,
        allocator,
    ) as *mut lzma_options_lzma;
    if opt.is_null() {
        return LZMA_MEM_ERROR;
    }
    if lzma_lzma_lclppb_decode(opt, *props.offset(0 as ::core::ffi::c_int as isize)) {
        lzma_free(opt as *mut ::core::ffi::c_void, allocator);
        return LZMA_OPTIONS_ERROR;
    } else {
        (*opt).dict_size = read32le(props.offset(1 as ::core::ffi::c_int as isize));
        (*opt).preset_dict = ::core::ptr::null::<uint8_t>();
        (*opt).preset_dict_size = 0 as uint32_t;
        *options = opt as *mut ::core::ffi::c_void;
        return LZMA_OK;
    };
}
