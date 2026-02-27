extern "C" {
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut ::core::ffi::c_void;
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
    fn lzma_lz_encoder_memusage(lz_options: *const lzma_lz_options) -> uint64_t;
    static lzma_rc_prices: [uint8_t; 128];
    fn lzma_lzma_optimum_fast(
        coder: *mut lzma_lzma1_encoder,
        mf: *mut lzma_mf,
        back_res: *mut uint32_t,
        len_res: *mut uint32_t,
    );
    fn lzma_lzma_optimum_normal(
        coder: *mut lzma_lzma1_encoder,
        mf: *mut lzma_mf,
        back_res: *mut uint32_t,
        len_res: *mut uint32_t,
        position: uint32_t,
    );
    static lzma_fastpos: [uint8_t; 8192];
}
pub type __darwin_size_t = usize;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type lzma_bool = ::core::ffi::c_uchar;
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
pub type probability = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_range_encoder {
    pub low: uint64_t,
    pub cache_size: uint64_t,
    pub range: uint32_t,
    pub cache: uint8_t,
    pub out_total: uint64_t,
    pub count: size_t,
    pub pos: size_t,
    pub symbols: [C2RustUnnamed; 53],
    pub probs: [*mut probability; 53],
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const RC_FLUSH: C2RustUnnamed = 4;
pub const RC_DIRECT_1: C2RustUnnamed = 3;
pub const RC_DIRECT_0: C2RustUnnamed = 2;
pub const RC_BIT_1: C2RustUnnamed = 1;
pub const RC_BIT_0: C2RustUnnamed = 0;
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
pub struct lzma_lzma1_encoder_s {
    pub rc: lzma_range_encoder,
    pub uncomp_size: uint64_t,
    pub out_limit: uint64_t,
    pub uncomp_size_ptr: *mut uint64_t,
    pub state: lzma_lzma_state,
    pub reps: [uint32_t; 4],
    pub matches: [lzma_match; 274],
    pub matches_count: uint32_t,
    pub longest_match_length: uint32_t,
    pub fast_mode: bool,
    pub is_initialized: bool,
    pub is_flushed: bool,
    pub use_eopm: bool,
    pub pos_mask: uint32_t,
    pub literal_context_bits: uint32_t,
    pub literal_mask: uint32_t,
    pub literal: [probability; 12288],
    pub is_match: [[probability; 16]; 12],
    pub is_rep: [probability; 12],
    pub is_rep0: [probability; 12],
    pub is_rep1: [probability; 12],
    pub is_rep2: [probability; 12],
    pub is_rep0_long: [[probability; 16]; 12],
    pub dist_slot: [[probability; 64]; 4],
    pub dist_special: [probability; 114],
    pub dist_align: [probability; 16],
    pub match_len_encoder: lzma_length_encoder,
    pub rep_len_encoder: lzma_length_encoder,
    pub dist_slot_prices: [[uint32_t; 64]; 4],
    pub dist_prices: [[uint32_t; 128]; 4],
    pub dist_table_size: uint32_t,
    pub match_price_count: uint32_t,
    pub align_prices: [uint32_t; 16],
    pub align_price_count: uint32_t,
    pub opts_end_index: uint32_t,
    pub opts_current_index: uint32_t,
    pub opts: [lzma_optimal; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_optimal {
    pub state: lzma_lzma_state,
    pub prev_1_is_literal: bool,
    pub prev_2: bool,
    pub pos_prev_2: uint32_t,
    pub back_prev_2: uint32_t,
    pub price: uint32_t,
    pub pos_prev: uint32_t,
    pub back_prev: uint32_t,
    pub backs: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_length_encoder {
    pub choice: probability,
    pub choice2: probability,
    pub low: [[probability; 8]; 16],
    pub mid: [[probability; 8]; 16],
    pub high: [probability; 256],
    pub prices: [[uint32_t; 272]; 16],
    pub table_size: uint32_t,
    pub counters: [uint32_t; 16],
}
pub type lzma_lzma1_encoder = lzma_lzma1_encoder_s;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulonglong = 18446744073709551615 as ::core::ffi::c_ulonglong;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn write32le(mut buf: *mut uint8_t, mut num: uint32_t) {
    *buf.offset(0 as ::core::ffi::c_int as isize) = num as uint8_t;
    *buf.offset(1 as ::core::ffi::c_int as isize) = (num >> 8 as ::core::ffi::c_int) as uint8_t;
    *buf.offset(2 as ::core::ffi::c_int as isize) = (num >> 16 as ::core::ffi::c_int) as uint8_t;
    *buf.offset(3 as ::core::ffi::c_int as isize) = (num >> 24 as ::core::ffi::c_int) as uint8_t;
}
pub const LZMA_FILTER_LZMA1: ::core::ffi::c_ulonglong =
    0x4000000000000001 as ::core::ffi::c_ulonglong;
pub const LZMA_FILTER_LZMA1EXT: ::core::ffi::c_ulonglong =
    0x4000000000000002 as ::core::ffi::c_ulonglong;
pub const LZMA_LCLP_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LZMA_PB_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LZMA_LZMA1EXT_ALLOW_EOPM: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const LZMA2_CHUNK_MAX: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 16 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mf_get_hash_bytes(mut match_finder: lzma_match_finder) -> uint32_t {
    return match_finder as uint32_t & 0xf as uint32_t;
}
#[inline]
unsafe extern "C" fn mf_skip(mut mf: *mut lzma_mf, mut amount: uint32_t) {
    if amount != 0 as uint32_t {
        (*mf).skip.expect("non-null function pointer")(mf, amount);
        (*mf).read_ahead = (*mf).read_ahead.wrapping_add(amount);
    }
}
pub const RC_SHIFT_BITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const RC_TOP_BITS: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const RC_TOP_VALUE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << RC_TOP_BITS;
pub const RC_BIT_MODEL_TOTAL_BITS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const RC_BIT_MODEL_TOTAL: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << RC_BIT_MODEL_TOTAL_BITS;
pub const RC_MOVE_BITS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const RC_MOVE_REDUCING_BITS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn rc_bit_price(prob: probability, bit: uint32_t) -> uint32_t {
    return lzma_rc_prices[((prob as uint32_t
        ^ (0 as uint32_t).wrapping_sub(bit)
            & (RC_BIT_MODEL_TOTAL as uint32_t).wrapping_sub(1 as uint32_t))
        >> RC_MOVE_REDUCING_BITS) as usize] as uint32_t;
}
#[inline]
unsafe extern "C" fn rc_bit_0_price(prob: probability) -> uint32_t {
    return lzma_rc_prices[(prob as ::core::ffi::c_int >> RC_MOVE_REDUCING_BITS) as usize]
        as uint32_t;
}
#[inline]
unsafe extern "C" fn rc_bit_1_price(prob: probability) -> uint32_t {
    return lzma_rc_prices[((prob as ::core::ffi::c_uint
        ^ RC_BIT_MODEL_TOTAL.wrapping_sub(1 as ::core::ffi::c_uint))
        >> RC_MOVE_REDUCING_BITS) as usize] as uint32_t;
}
#[inline]
unsafe extern "C" fn rc_bittree_price(
    probs: *const probability,
    bit_levels: uint32_t,
    mut symbol: uint32_t,
) -> uint32_t {
    let mut price: uint32_t = 0 as uint32_t;
    symbol = (symbol as ::core::ffi::c_uint).wrapping_add((1 as ::core::ffi::c_uint) << bit_levels)
        as uint32_t as uint32_t;
    loop {
        let bit: uint32_t = symbol & 1 as uint32_t;
        symbol >>= 1 as ::core::ffi::c_int;
        price = price.wrapping_add(rc_bit_price(*probs.offset(symbol as isize), bit));
        if !(symbol != 1 as uint32_t) {
            break;
        }
    }
    return price;
}
#[inline]
unsafe extern "C" fn rc_reset(mut rc: *mut lzma_range_encoder) {
    (*rc).low = 0 as uint64_t;
    (*rc).cache_size = 1 as uint64_t;
    (*rc).range = UINT32_MAX as uint32_t;
    (*rc).cache = 0 as uint8_t;
    (*rc).out_total = 0 as uint64_t;
    (*rc).count = 0 as size_t;
    (*rc).pos = 0 as size_t;
}
#[inline]
unsafe extern "C" fn rc_forget(mut rc: *mut lzma_range_encoder) {
    (*rc).count = 0 as size_t;
}
#[inline]
unsafe extern "C" fn rc_bit(
    mut rc: *mut lzma_range_encoder,
    mut prob: *mut probability,
    mut bit: uint32_t,
) {
    (*rc).symbols[(*rc).count as usize] = bit as C2RustUnnamed;
    (*rc).probs[(*rc).count as usize] = prob;
    (*rc).count = (*rc).count.wrapping_add(1);
}
#[inline]
unsafe extern "C" fn rc_bittree(
    mut rc: *mut lzma_range_encoder,
    mut probs: *mut probability,
    mut bit_count: uint32_t,
    mut symbol: uint32_t,
) {
    let mut model_index: uint32_t = 1 as uint32_t;
    loop {
        bit_count = bit_count.wrapping_sub(1);
        let bit: uint32_t = symbol >> bit_count & 1 as uint32_t;
        rc_bit(
            rc,
            probs.offset(model_index as isize) as *mut probability,
            bit,
        );
        model_index = (model_index << 1 as ::core::ffi::c_int).wrapping_add(bit);
        if !(bit_count != 0 as uint32_t) {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn rc_bittree_reverse(
    mut rc: *mut lzma_range_encoder,
    mut probs: *mut probability,
    mut bit_count: uint32_t,
    mut symbol: uint32_t,
) {
    let mut model_index: uint32_t = 1 as uint32_t;
    loop {
        let bit: uint32_t = symbol & 1 as uint32_t;
        symbol >>= 1 as ::core::ffi::c_int;
        rc_bit(
            rc,
            probs.offset(model_index as isize) as *mut probability,
            bit,
        );
        model_index = (model_index << 1 as ::core::ffi::c_int).wrapping_add(bit);
        bit_count = bit_count.wrapping_sub(1);
        if !(bit_count != 0 as uint32_t) {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn rc_direct(
    mut rc: *mut lzma_range_encoder,
    mut value: uint32_t,
    mut bit_count: uint32_t,
) {
    loop {
        bit_count = bit_count.wrapping_sub(1);
        let fresh0 = (*rc).count;
        (*rc).count = (*rc).count.wrapping_add(1);
        (*rc).symbols[fresh0 as usize] = (RC_DIRECT_0 as ::core::ffi::c_int as uint32_t)
            .wrapping_add(value >> bit_count & 1 as uint32_t)
            as C2RustUnnamed;
        if !(bit_count != 0 as uint32_t) {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn rc_flush(mut rc: *mut lzma_range_encoder) {
    let mut i: size_t = 0 as size_t;
    while i < 5 as size_t {
        let fresh1 = (*rc).count;
        (*rc).count = (*rc).count.wrapping_add(1);
        (*rc).symbols[fresh1 as usize] = RC_FLUSH;
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn rc_shift_low(
    mut rc: *mut lzma_range_encoder,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> bool {
    if ((*rc).low as uint32_t) < 0xff000000 as ::core::ffi::c_uint as uint32_t
        || ((*rc).low >> 32 as ::core::ffi::c_int) as uint32_t != 0 as uint32_t
    {
        loop {
            if *out_pos == out_size {
                return true_0 != 0;
            }
            *out.offset(*out_pos as isize) = ((*rc).cache as ::core::ffi::c_int
                + ((*rc).low >> 32 as ::core::ffi::c_int) as uint8_t as ::core::ffi::c_int)
                as uint8_t;
            *out_pos = (*out_pos).wrapping_add(1);
            (*rc).out_total = (*rc).out_total.wrapping_add(1);
            (*rc).cache = 0xff as uint8_t;
            (*rc).cache_size = (*rc).cache_size.wrapping_sub(1);
            if !((*rc).cache_size != 0 as uint64_t) {
                break;
            }
        }
        (*rc).cache = ((*rc).low >> 24 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    }
    (*rc).cache_size = (*rc).cache_size.wrapping_add(1);
    (*rc).low = ((*rc).low & 0xffffff as uint64_t) << RC_SHIFT_BITS;
    return false_0 != 0;
}
#[inline]
unsafe extern "C" fn rc_shift_low_dummy(
    mut low: *mut uint64_t,
    mut cache_size: *mut uint64_t,
    mut cache: *mut uint8_t,
    mut out_pos: *mut uint64_t,
    mut out_size: uint64_t,
) -> bool {
    if (*low as uint32_t) < 0xff000000 as ::core::ffi::c_uint as uint32_t
        || (*low >> 32 as ::core::ffi::c_int) as uint32_t != 0 as uint32_t
    {
        loop {
            if *out_pos == out_size {
                return true_0 != 0;
            }
            *out_pos = (*out_pos).wrapping_add(1);
            *cache = 0xff as uint8_t;
            *cache_size = (*cache_size).wrapping_sub(1);
            if !(*cache_size != 0 as uint64_t) {
                break;
            }
        }
        *cache = (*low >> 24 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    }
    *cache_size = (*cache_size).wrapping_add(1);
    *low = (*low & 0xffffff as uint64_t) << RC_SHIFT_BITS;
    return false_0 != 0;
}
#[inline]
unsafe extern "C" fn rc_encode(
    mut rc: *mut lzma_range_encoder,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> bool {
    while (*rc).pos < (*rc).count {
        if (*rc).range < RC_TOP_VALUE as uint32_t {
            if rc_shift_low(rc, out, out_pos, out_size) {
                return true_0 != 0;
            }
            (*rc).range <<= RC_SHIFT_BITS;
        }
        match (*rc).symbols[(*rc).pos as usize] as ::core::ffi::c_uint {
            0 => {
                let mut prob: probability = *(*rc).probs[(*rc).pos as usize];
                (*rc).range =
                    ((*rc).range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(prob as uint32_t);
                prob = (prob as ::core::ffi::c_uint).wrapping_add(
                    RC_BIT_MODEL_TOTAL.wrapping_sub(prob as ::core::ffi::c_uint) >> RC_MOVE_BITS,
                ) as probability as probability;
                *(*rc).probs[(*rc).pos as usize] = prob;
            }
            1 => {
                let mut prob_0: probability = *(*rc).probs[(*rc).pos as usize];
                let bound: uint32_t =
                    (prob_0 as uint32_t).wrapping_mul((*rc).range >> RC_BIT_MODEL_TOTAL_BITS);
                (*rc).low = (*rc).low.wrapping_add(bound as uint64_t);
                (*rc).range = (*rc).range.wrapping_sub(bound);
                prob_0 = (prob_0 as ::core::ffi::c_int
                    - (prob_0 as ::core::ffi::c_int >> RC_MOVE_BITS))
                    as probability;
                *(*rc).probs[(*rc).pos as usize] = prob_0;
            }
            2 => {
                (*rc).range >>= 1 as ::core::ffi::c_int;
            }
            3 => {
                (*rc).range >>= 1 as ::core::ffi::c_int;
                (*rc).low = (*rc).low.wrapping_add((*rc).range as uint64_t);
            }
            4 => {
                (*rc).range = UINT32_MAX as uint32_t;
                loop {
                    if rc_shift_low(rc, out, out_pos, out_size) {
                        return true_0 != 0;
                    }
                    (*rc).pos = (*rc).pos.wrapping_add(1);
                    if !((*rc).pos < (*rc).count) {
                        break;
                    }
                }
                rc_reset(rc);
                return false_0 != 0;
            }
            _ => {}
        }
        (*rc).pos = (*rc).pos.wrapping_add(1);
    }
    (*rc).count = 0 as size_t;
    (*rc).pos = 0 as size_t;
    return false_0 != 0;
}
#[inline]
unsafe extern "C" fn rc_encode_dummy(
    mut rc: *const lzma_range_encoder,
    mut out_limit: uint64_t,
) -> bool {
    let mut low: uint64_t = (*rc).low;
    let mut cache_size: uint64_t = (*rc).cache_size;
    let mut range: uint32_t = (*rc).range;
    let mut cache: uint8_t = (*rc).cache;
    let mut out_pos: uint64_t = (*rc).out_total;
    let mut pos: size_t = (*rc).pos;
    loop {
        if range < RC_TOP_VALUE as uint32_t {
            if rc_shift_low_dummy(
                &raw mut low,
                &raw mut cache_size,
                &raw mut cache,
                &raw mut out_pos,
                out_limit,
            ) {
                return true_0 != 0;
            }
            range <<= RC_SHIFT_BITS;
        }
        if pos == (*rc).count {
            break;
        }
        match (*rc).symbols[pos as usize] as ::core::ffi::c_uint {
            0 => {
                let mut prob: probability = *(*rc).probs[pos as usize];
                range = (range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(prob as uint32_t);
            }
            1 => {
                let mut prob_0: probability = *(*rc).probs[pos as usize];
                let bound: uint32_t =
                    (prob_0 as uint32_t).wrapping_mul(range >> RC_BIT_MODEL_TOTAL_BITS);
                low = low.wrapping_add(bound as uint64_t);
                range = range.wrapping_sub(bound);
            }
            2 => {
                range >>= 1 as ::core::ffi::c_int;
            }
            3 => {
                range >>= 1 as ::core::ffi::c_int;
                low = low.wrapping_add(range as uint64_t);
            }
            4 | _ => {}
        }
        pos = pos.wrapping_add(1);
    }
    pos = 0 as size_t;
    while pos < 5 as size_t {
        if rc_shift_low_dummy(
            &raw mut low,
            &raw mut cache_size,
            &raw mut cache,
            &raw mut out_pos,
            out_limit,
        ) {
            return true_0 != 0;
        }
        pos = pos.wrapping_add(1);
    }
    return false_0 != 0;
}
#[inline]
unsafe extern "C" fn rc_pending(mut rc: *const lzma_range_encoder) -> uint64_t {
    return (*rc)
        .cache_size
        .wrapping_add(5 as uint64_t)
        .wrapping_sub(1 as uint64_t);
}
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
pub const LEN_SYMBOLS: ::core::ffi::c_int = LEN_LOW_SYMBOLS + LEN_MID_SYMBOLS + LEN_HIGH_SYMBOLS;
pub const MATCH_LEN_MAX: ::core::ffi::c_int = MATCH_LEN_MIN + LEN_SYMBOLS - 1 as ::core::ffi::c_int;
pub const DIST_STATES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DIST_SLOT_BITS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const DIST_MODEL_START: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DIST_MODEL_END: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const FULL_DISTANCES_BITS: ::core::ffi::c_int = DIST_MODEL_END / 2 as ::core::ffi::c_int;
pub const FULL_DISTANCES: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << FULL_DISTANCES_BITS;
pub const ALIGN_BITS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ALIGN_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << ALIGN_BITS;
pub const ALIGN_MASK: ::core::ffi::c_int = ALIGN_SIZE - 1 as ::core::ffi::c_int;
pub const REPS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OPTS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn literal_matched(
    mut rc: *mut lzma_range_encoder,
    mut subcoder: *mut probability,
    mut match_byte: uint32_t,
    mut symbol: uint32_t,
) {
    let mut offset: uint32_t = 0x100 as uint32_t;
    symbol = (symbol as ::core::ffi::c_uint)
        .wrapping_add((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
        as uint32_t as uint32_t;
    loop {
        match_byte <<= 1 as ::core::ffi::c_int;
        let match_bit: uint32_t = match_byte & offset;
        let subcoder_index: uint32_t = offset
            .wrapping_add(match_bit)
            .wrapping_add(symbol >> 8 as ::core::ffi::c_int);
        let bit: uint32_t = symbol >> 7 as ::core::ffi::c_int & 1 as uint32_t;
        rc_bit(
            rc,
            subcoder.offset(subcoder_index as isize) as *mut probability,
            bit,
        );
        symbol <<= 1 as ::core::ffi::c_int;
        offset &= !(match_byte ^ symbol);
        if !(symbol < (1 as uint32_t) << 16 as ::core::ffi::c_int) {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn literal(
    mut coder: *mut lzma_lzma1_encoder,
    mut mf: *mut lzma_mf,
    mut position: uint32_t,
) {
    let cur_byte: uint8_t = *(*mf)
        .buffer
        .offset((*mf).read_pos.wrapping_sub((*mf).read_ahead) as isize);
    let mut subcoder: *mut probability = (&raw mut (*coder).literal as *mut probability).offset(
        (3 as uint32_t).wrapping_mul(
            ((position << 8 as ::core::ffi::c_int).wrapping_add(
                *(*mf).buffer.offset(
                    (*mf)
                        .read_pos
                        .wrapping_sub((*mf).read_ahead)
                        .wrapping_sub(1 as uint32_t) as isize,
                ) as uint32_t,
            ) & (*coder).literal_mask)
                << (*coder).literal_context_bits,
        ) as isize,
    );
    if ((*coder).state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint {
        (*coder).state = (if (*coder).state as ::core::ffi::c_uint
            <= STATE_SHORTREP_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            STATE_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
        } else {
            ((*coder).state as ::core::ffi::c_uint).wrapping_sub(3 as ::core::ffi::c_uint)
        }) as lzma_lzma_state;
        rc_bittree(
            &raw mut (*coder).rc,
            subcoder,
            8 as uint32_t,
            cur_byte as uint32_t,
        );
    } else {
        (*coder).state = (if (*coder).state as ::core::ffi::c_uint
            <= STATE_LIT_SHORTREP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            ((*coder).state as ::core::ffi::c_uint).wrapping_sub(3 as ::core::ffi::c_uint)
        } else {
            ((*coder).state as ::core::ffi::c_uint).wrapping_sub(6 as ::core::ffi::c_uint)
        }) as lzma_lzma_state;
        let match_byte: uint8_t = *(*mf).buffer.offset(
            (*mf)
                .read_pos
                .wrapping_sub((*coder).reps[0 as ::core::ffi::c_int as usize])
                .wrapping_sub(1 as uint32_t)
                .wrapping_sub((*mf).read_ahead) as isize,
        );
        literal_matched(
            &raw mut (*coder).rc,
            subcoder,
            match_byte as uint32_t,
            cur_byte as uint32_t,
        );
    };
}
unsafe extern "C" fn length_update_prices(mut lc: *mut lzma_length_encoder, pos_state: uint32_t) {
    let table_size: uint32_t = (*lc).table_size;
    (*lc).counters[pos_state as usize] = table_size;
    let a0: uint32_t = rc_bit_0_price((*lc).choice) as uint32_t;
    let a1: uint32_t = rc_bit_1_price((*lc).choice) as uint32_t;
    let b0: uint32_t = a1.wrapping_add(rc_bit_0_price((*lc).choice2) as uint32_t);
    let b1: uint32_t = a1.wrapping_add(rc_bit_1_price((*lc).choice2) as uint32_t);
    let prices: *mut uint32_t = &raw mut *(&raw mut (*lc).prices as *mut [uint32_t; 272])
        .offset(pos_state as isize) as *mut uint32_t;
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < table_size && i < LEN_LOW_SYMBOLS as uint32_t {
        *prices.offset(i as isize) = a0.wrapping_add(rc_bittree_price(
            &raw mut *(&raw mut (*lc).low as *mut [probability; 8]).offset(pos_state as isize)
                as *mut probability,
            LEN_LOW_BITS as uint32_t,
            i,
        ));
        i = i.wrapping_add(1);
    }
    while i < table_size && i < (LEN_LOW_SYMBOLS + LEN_MID_SYMBOLS) as uint32_t {
        *prices.offset(i as isize) = b0.wrapping_add(rc_bittree_price(
            &raw mut *(&raw mut (*lc).mid as *mut [probability; 8]).offset(pos_state as isize)
                as *mut probability,
            LEN_MID_BITS as uint32_t,
            i.wrapping_sub(LEN_LOW_SYMBOLS as uint32_t),
        ));
        i = i.wrapping_add(1);
    }
    while i < table_size {
        *prices.offset(i as isize) = b1.wrapping_add(rc_bittree_price(
            &raw mut (*lc).high as *mut probability,
            LEN_HIGH_BITS as uint32_t,
            i.wrapping_sub(LEN_LOW_SYMBOLS as uint32_t)
                .wrapping_sub(LEN_MID_SYMBOLS as uint32_t),
        ));
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn length(
    mut rc: *mut lzma_range_encoder,
    mut lc: *mut lzma_length_encoder,
    pos_state: uint32_t,
    mut len: uint32_t,
    fast_mode: bool,
) {
    len = len.wrapping_sub(MATCH_LEN_MIN as uint32_t);
    if len < LEN_LOW_SYMBOLS as uint32_t {
        rc_bit(rc, &raw mut (*lc).choice, 0 as uint32_t);
        rc_bittree(
            rc,
            &raw mut *(&raw mut (*lc).low as *mut [probability; 8]).offset(pos_state as isize)
                as *mut probability,
            LEN_LOW_BITS as uint32_t,
            len,
        );
    } else {
        rc_bit(rc, &raw mut (*lc).choice, 1 as uint32_t);
        len = len.wrapping_sub(LEN_LOW_SYMBOLS as uint32_t);
        if len < LEN_MID_SYMBOLS as uint32_t {
            rc_bit(rc, &raw mut (*lc).choice2, 0 as uint32_t);
            rc_bittree(
                rc,
                &raw mut *(&raw mut (*lc).mid as *mut [probability; 8]).offset(pos_state as isize)
                    as *mut probability,
                LEN_MID_BITS as uint32_t,
                len,
            );
        } else {
            rc_bit(rc, &raw mut (*lc).choice2, 1 as uint32_t);
            len = len.wrapping_sub(LEN_MID_SYMBOLS as uint32_t);
            rc_bittree(
                rc,
                &raw mut (*lc).high as *mut probability,
                LEN_HIGH_BITS as uint32_t,
                len,
            );
        }
    }
    if !fast_mode {
        (*lc).counters[pos_state as usize] = (*lc).counters[pos_state as usize].wrapping_sub(1);
        if (*lc).counters[pos_state as usize] == 0 as uint32_t {
            length_update_prices(lc, pos_state);
        }
    }
}
#[inline]
unsafe extern "C" fn match_0(
    mut coder: *mut lzma_lzma1_encoder,
    pos_state: uint32_t,
    distance: uint32_t,
    len: uint32_t,
) {
    (*coder).state = (if ((*coder).state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint
    {
        STATE_LIT_MATCH as ::core::ffi::c_int
    } else {
        STATE_NONLIT_MATCH as ::core::ffi::c_int
    }) as lzma_lzma_state;
    length(
        &raw mut (*coder).rc,
        &raw mut (*coder).match_len_encoder,
        pos_state,
        len,
        (*coder).fast_mode,
    );
    let dist_slot: uint32_t = get_dist_slot(distance) as uint32_t;
    let dist_state: uint32_t = if len < (DIST_STATES + MATCH_LEN_MIN) as uint32_t {
        len.wrapping_sub(MATCH_LEN_MIN as uint32_t)
    } else {
        (DIST_STATES - 1 as ::core::ffi::c_int) as uint32_t
    };
    rc_bittree(
        &raw mut (*coder).rc,
        &raw mut *(&raw mut (*coder).dist_slot as *mut [probability; 64])
            .offset(dist_state as isize) as *mut probability,
        DIST_SLOT_BITS as uint32_t,
        dist_slot,
    );
    if dist_slot >= DIST_MODEL_START as uint32_t {
        let footer_bits: uint32_t =
            (dist_slot >> 1 as ::core::ffi::c_int).wrapping_sub(1 as uint32_t);
        let base: uint32_t = (2 as uint32_t | dist_slot & 1 as uint32_t) << footer_bits;
        let dist_reduced: uint32_t = distance.wrapping_sub(base);
        if dist_slot < DIST_MODEL_END as uint32_t {
            rc_bittree_reverse(
                &raw mut (*coder).rc,
                (&raw mut (*coder).dist_special as *mut probability)
                    .offset(base as isize)
                    .offset(-(dist_slot as isize))
                    .offset(-(1 as ::core::ffi::c_int as isize)),
                footer_bits,
                dist_reduced,
            );
        } else {
            rc_direct(
                &raw mut (*coder).rc,
                dist_reduced >> ALIGN_BITS,
                footer_bits.wrapping_sub(ALIGN_BITS as uint32_t),
            );
            rc_bittree_reverse(
                &raw mut (*coder).rc,
                &raw mut (*coder).dist_align as *mut probability,
                ALIGN_BITS as uint32_t,
                dist_reduced & ALIGN_MASK as uint32_t,
            );
            (*coder).align_price_count = (*coder).align_price_count.wrapping_add(1);
        }
    }
    (*coder).reps[3 as ::core::ffi::c_int as usize] =
        (*coder).reps[2 as ::core::ffi::c_int as usize];
    (*coder).reps[2 as ::core::ffi::c_int as usize] =
        (*coder).reps[1 as ::core::ffi::c_int as usize];
    (*coder).reps[1 as ::core::ffi::c_int as usize] =
        (*coder).reps[0 as ::core::ffi::c_int as usize];
    (*coder).reps[0 as ::core::ffi::c_int as usize] = distance;
    (*coder).match_price_count = (*coder).match_price_count.wrapping_add(1);
}
#[inline]
unsafe extern "C" fn rep_match(
    mut coder: *mut lzma_lzma1_encoder,
    pos_state: uint32_t,
    rep: uint32_t,
    len: uint32_t,
) {
    if rep == 0 as uint32_t {
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut (*coder).is_rep0 as *mut probability).offset((*coder).state as isize)
                as *mut probability,
            0 as uint32_t,
        );
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut *(&raw mut (*coder).is_rep0_long as *mut [probability; 16])
                .offset((*coder).state as isize) as *mut probability)
                .offset(pos_state as isize) as *mut probability,
            (len != 1 as uint32_t) as ::core::ffi::c_int as uint32_t,
        );
    } else {
        let distance: uint32_t = (*coder).reps[rep as usize];
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut (*coder).is_rep0 as *mut probability).offset((*coder).state as isize)
                as *mut probability,
            1 as uint32_t,
        );
        if rep == 1 as uint32_t {
            rc_bit(
                &raw mut (*coder).rc,
                (&raw mut (*coder).is_rep1 as *mut probability).offset((*coder).state as isize)
                    as *mut probability,
                0 as uint32_t,
            );
        } else {
            rc_bit(
                &raw mut (*coder).rc,
                (&raw mut (*coder).is_rep1 as *mut probability).offset((*coder).state as isize)
                    as *mut probability,
                1 as uint32_t,
            );
            rc_bit(
                &raw mut (*coder).rc,
                (&raw mut (*coder).is_rep2 as *mut probability).offset((*coder).state as isize)
                    as *mut probability,
                rep.wrapping_sub(2 as uint32_t),
            );
            if rep == 3 as uint32_t {
                (*coder).reps[3 as ::core::ffi::c_int as usize] =
                    (*coder).reps[2 as ::core::ffi::c_int as usize];
            }
            (*coder).reps[2 as ::core::ffi::c_int as usize] =
                (*coder).reps[1 as ::core::ffi::c_int as usize];
        }
        (*coder).reps[1 as ::core::ffi::c_int as usize] =
            (*coder).reps[0 as ::core::ffi::c_int as usize];
        (*coder).reps[0 as ::core::ffi::c_int as usize] = distance;
    }
    if len == 1 as uint32_t {
        (*coder).state =
            (if ((*coder).state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint {
                STATE_LIT_SHORTREP as ::core::ffi::c_int
            } else {
                STATE_NONLIT_REP as ::core::ffi::c_int
            }) as lzma_lzma_state;
    } else {
        length(
            &raw mut (*coder).rc,
            &raw mut (*coder).rep_len_encoder,
            pos_state,
            len,
            (*coder).fast_mode,
        );
        (*coder).state =
            (if ((*coder).state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint {
                STATE_LIT_LONGREP as ::core::ffi::c_int
            } else {
                STATE_NONLIT_REP as ::core::ffi::c_int
            }) as lzma_lzma_state;
    };
}
unsafe extern "C" fn encode_symbol(
    mut coder: *mut lzma_lzma1_encoder,
    mut mf: *mut lzma_mf,
    mut back: uint32_t,
    mut len: uint32_t,
    mut position: uint32_t,
) {
    let pos_state: uint32_t = position & (*coder).pos_mask;
    if back == UINT32_MAX as uint32_t {
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut *(&raw mut (*coder).is_match as *mut [probability; 16])
                .offset((*coder).state as isize) as *mut probability)
                .offset(pos_state as isize) as *mut probability,
            0 as uint32_t,
        );
        literal(coder, mf, position);
    } else {
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut *(&raw mut (*coder).is_match as *mut [probability; 16])
                .offset((*coder).state as isize) as *mut probability)
                .offset(pos_state as isize) as *mut probability,
            1 as uint32_t,
        );
        if back < REPS as uint32_t {
            rc_bit(
                &raw mut (*coder).rc,
                (&raw mut (*coder).is_rep as *mut probability).offset((*coder).state as isize)
                    as *mut probability,
                1 as uint32_t,
            );
            rep_match(coder, pos_state, back, len);
        } else {
            rc_bit(
                &raw mut (*coder).rc,
                (&raw mut (*coder).is_rep as *mut probability).offset((*coder).state as isize)
                    as *mut probability,
                0 as uint32_t,
            );
            match_0(coder, pos_state, back.wrapping_sub(REPS as uint32_t), len);
        }
    }
    (*mf).read_ahead = (*mf).read_ahead.wrapping_sub(len);
}
unsafe extern "C" fn encode_init(mut coder: *mut lzma_lzma1_encoder, mut mf: *mut lzma_mf) -> bool {
    if (*mf).read_pos == (*mf).read_limit {
        if (*mf).action as ::core::ffi::c_uint
            == LZMA_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return false_0 != 0;
        }
    } else {
        mf_skip(mf, 1 as uint32_t);
        (*mf).read_ahead = 0 as uint32_t;
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut *(&raw mut (*coder).is_match as *mut [probability; 16])
                .offset(0 as ::core::ffi::c_int as isize) as *mut probability)
                .offset(0 as ::core::ffi::c_int as isize) as *mut probability,
            0 as uint32_t,
        );
        rc_bittree(
            &raw mut (*coder).rc,
            (&raw mut (*coder).literal as *mut probability)
                .offset(0 as ::core::ffi::c_int as isize),
            8 as uint32_t,
            *(*mf).buffer.offset(0 as ::core::ffi::c_int as isize) as uint32_t,
        );
        (*coder).uncomp_size = (*coder).uncomp_size.wrapping_add(1);
    }
    (*coder).is_initialized = true_0 != 0;
    return true_0 != 0;
}
unsafe extern "C" fn encode_eopm(mut coder: *mut lzma_lzma1_encoder, mut position: uint32_t) {
    let pos_state: uint32_t = position & (*coder).pos_mask;
    rc_bit(
        &raw mut (*coder).rc,
        (&raw mut *(&raw mut (*coder).is_match as *mut [probability; 16])
            .offset((*coder).state as isize) as *mut probability)
            .offset(pos_state as isize) as *mut probability,
        1 as uint32_t,
    );
    rc_bit(
        &raw mut (*coder).rc,
        (&raw mut (*coder).is_rep as *mut probability).offset((*coder).state as isize)
            as *mut probability,
        0 as uint32_t,
    );
    match_0(
        coder,
        pos_state,
        UINT32_MAX as uint32_t,
        MATCH_LEN_MIN as uint32_t,
    );
}
pub const LOOP_INPUT_MAX: ::core::ffi::c_int = OPTS + 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_encode(
    mut coder: *mut lzma_lzma1_encoder,
    mut mf: *mut lzma_mf,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut limit: uint32_t,
) -> lzma_ret {
    if !(*coder).is_initialized && !encode_init(coder, mf) {
        return LZMA_OK;
    }
    if rc_encode(&raw mut (*coder).rc, out, out_pos, out_size) {
        return LZMA_OK;
    }
    if (*coder).is_flushed {
        return LZMA_STREAM_END;
    }
    while !(limit != UINT32_MAX as uint32_t
        && ((*mf).read_pos.wrapping_sub((*mf).read_ahead) >= limit
            || (*out_pos as uint64_t).wrapping_add(rc_pending(&raw mut (*coder).rc))
                >= LZMA2_CHUNK_MAX.wrapping_sub(LOOP_INPUT_MAX as ::core::ffi::c_uint) as uint64_t))
    {
        if (*mf).read_pos >= (*mf).read_limit {
            if (*mf).action as ::core::ffi::c_uint
                == LZMA_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return LZMA_OK;
            }
            if (*mf).read_ahead == 0 as uint32_t {
                break;
            }
        }
        let mut len: uint32_t = 0;
        let mut back: uint32_t = 0;
        if (*coder).fast_mode {
            lzma_lzma_optimum_fast(coder, mf, &raw mut back, &raw mut len);
        } else {
            lzma_lzma_optimum_normal(
                coder,
                mf,
                &raw mut back,
                &raw mut len,
                (*coder).uncomp_size as uint32_t,
            );
        }
        encode_symbol(coder, mf, back, len, (*coder).uncomp_size as uint32_t);
        if (*coder).out_limit != 0 as uint64_t
            && rc_encode_dummy(&raw mut (*coder).rc, (*coder).out_limit) as ::core::ffi::c_int != 0
        {
            rc_forget(&raw mut (*coder).rc);
            break;
        } else {
            (*coder).uncomp_size = (*coder).uncomp_size.wrapping_add(len as uint64_t);
            if rc_encode(&raw mut (*coder).rc, out, out_pos, out_size) {
                return LZMA_OK;
            }
        }
    }
    if !(*coder).uncomp_size_ptr.is_null() {
        *(*coder).uncomp_size_ptr = (*coder).uncomp_size;
    }
    if (*coder).use_eopm {
        encode_eopm(coder, (*coder).uncomp_size as uint32_t);
    }
    rc_flush(&raw mut (*coder).rc);
    if rc_encode(&raw mut (*coder).rc, out, out_pos, out_size) {
        (*coder).is_flushed = true_0 != 0;
        return LZMA_OK;
    }
    return LZMA_STREAM_END;
}
unsafe extern "C" fn lzma_encode(
    mut coder: *mut ::core::ffi::c_void,
    mut mf: *mut lzma_mf,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    if ((*mf).action as ::core::ffi::c_uint
        == LZMA_SYNC_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        return LZMA_OPTIONS_ERROR;
    }
    return lzma_lzma_encode(
        coder as *mut lzma_lzma1_encoder,
        mf,
        out,
        out_pos,
        out_size,
        UINT32_MAX as uint32_t,
    );
}
unsafe extern "C" fn lzma_lzma_set_out_limit(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut uncomp_size: *mut uint64_t,
    mut out_limit: uint64_t,
) -> lzma_ret {
    if out_limit < 6 as uint64_t {
        return LZMA_BUF_ERROR;
    }
    let mut coder: *mut lzma_lzma1_encoder = coder_ptr as *mut lzma_lzma1_encoder;
    (*coder).out_limit = out_limit;
    (*coder).uncomp_size_ptr = uncomp_size;
    (*coder).use_eopm = false_0 != 0;
    return LZMA_OK;
}
unsafe extern "C" fn is_options_valid(mut options: *const lzma_options_lzma) -> bool {
    return is_lclppb_valid(options) as ::core::ffi::c_int != 0
        && (*options).nice_len >= MATCH_LEN_MIN as uint32_t
        && (*options).nice_len <= MATCH_LEN_MAX as uint32_t
        && ((*options).mode as ::core::ffi::c_uint
            == LZMA_MODE_FAST as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*options).mode as ::core::ffi::c_uint
                == LZMA_MODE_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint);
}
unsafe extern "C" fn set_lz_options(
    mut lz_options: *mut lzma_lz_options,
    mut options: *const lzma_options_lzma,
) {
    (*lz_options).before_size = OPTS as size_t;
    (*lz_options).dict_size = (*options).dict_size as size_t;
    (*lz_options).after_size = LOOP_INPUT_MAX as size_t;
    (*lz_options).match_len_max = MATCH_LEN_MAX as size_t;
    (*lz_options).nice_len = (if mf_get_hash_bytes((*options).mf) > (*options).nice_len {
        mf_get_hash_bytes((*options).mf)
    } else {
        (*options).nice_len
    }) as size_t;
    (*lz_options).match_finder = (*options).mf;
    (*lz_options).depth = (*options).depth;
    (*lz_options).preset_dict = (*options).preset_dict;
    (*lz_options).preset_dict_size = (*options).preset_dict_size;
}
unsafe extern "C" fn length_encoder_reset(
    mut lencoder: *mut lzma_length_encoder,
    num_pos_states: uint32_t,
    fast_mode: bool,
) {
    (*lencoder).choice = (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
    (*lencoder).choice2 = (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
    let mut pos_state: size_t = 0 as size_t;
    while pos_state < num_pos_states as size_t {
        let mut bt_i: uint32_t = 0 as uint32_t;
        while bt_i < ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t {
            (*lencoder).low[pos_state as usize][bt_i as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            bt_i = bt_i.wrapping_add(1);
        }
        let mut bt_i_0: uint32_t = 0 as uint32_t;
        while bt_i_0 < ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as uint32_t {
            (*lencoder).mid[pos_state as usize][bt_i_0 as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            bt_i_0 = bt_i_0.wrapping_add(1);
        }
        pos_state = pos_state.wrapping_add(1);
    }
    let mut bt_i_1: uint32_t = 0 as uint32_t;
    while bt_i_1 < ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t {
        (*lencoder).high[bt_i_1 as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        bt_i_1 = bt_i_1.wrapping_add(1);
    }
    if !fast_mode {
        let mut pos_state_0: uint32_t = 0 as uint32_t;
        while pos_state_0 < num_pos_states {
            length_update_prices(lencoder, pos_state_0);
            pos_state_0 = pos_state_0.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_encoder_reset(
    mut coder: *mut lzma_lzma1_encoder,
    mut options: *const lzma_options_lzma,
) -> lzma_ret {
    if !is_options_valid(options) {
        return LZMA_OPTIONS_ERROR;
    }
    (*coder).pos_mask = ((1 as ::core::ffi::c_uint) << (*options).pb)
        .wrapping_sub(1 as ::core::ffi::c_uint) as uint32_t;
    (*coder).literal_context_bits = (*options).lc;
    (*coder).literal_mask = ((0x100 as ::core::ffi::c_uint) << (*options).lp)
        .wrapping_sub(0x100 as ::core::ffi::c_uint >> (*options).lc)
        as uint32_t;
    rc_reset(&raw mut (*coder).rc);
    (*coder).state = STATE_LIT_LIT;
    let mut i: size_t = 0 as size_t;
    while i < REPS as size_t {
        (*coder).reps[i as usize] = 0 as uint32_t;
        i = i.wrapping_add(1);
    }
    literal_init(
        &raw mut (*coder).literal as *mut probability,
        (*options).lc,
        (*options).lp,
    );
    let mut i_0: size_t = 0 as size_t;
    while i_0 < STATES as size_t {
        let mut j: size_t = 0 as size_t;
        while j <= (*coder).pos_mask as size_t {
            (*coder).is_match[i_0 as usize][j as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            (*coder).is_rep0_long[i_0 as usize][j as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            j = j.wrapping_add(1);
        }
        (*coder).is_rep[i_0 as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        (*coder).is_rep0[i_0 as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        (*coder).is_rep1[i_0 as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        (*coder).is_rep2[i_0 as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: size_t = 0 as size_t;
    while i_1 < (FULL_DISTANCES - DIST_MODEL_END) as size_t {
        (*coder).dist_special[i_1 as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        i_1 = i_1.wrapping_add(1);
    }
    let mut i_2: size_t = 0 as size_t;
    while i_2 < DIST_STATES as size_t {
        let mut bt_i: uint32_t = 0 as uint32_t;
        while bt_i < ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t {
            (*coder).dist_slot[i_2 as usize][bt_i as usize] =
                (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
            bt_i = bt_i.wrapping_add(1);
        }
        i_2 = i_2.wrapping_add(1);
    }
    let mut bt_i_0: uint32_t = 0 as uint32_t;
    while bt_i_0 < ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint32_t {
        (*coder).dist_align[bt_i_0 as usize] =
            (RC_BIT_MODEL_TOTAL >> 1 as ::core::ffi::c_int) as probability;
        bt_i_0 = bt_i_0.wrapping_add(1);
    }
    length_encoder_reset(
        &raw mut (*coder).match_len_encoder,
        (1 as uint32_t) << (*options).pb,
        (*coder).fast_mode,
    );
    length_encoder_reset(
        &raw mut (*coder).rep_len_encoder,
        (1 as uint32_t) << (*options).pb,
        (*coder).fast_mode,
    );
    (*coder).match_price_count = UINT32_MAX.wrapping_div(2 as ::core::ffi::c_uint) as uint32_t;
    (*coder).align_price_count = UINT32_MAX.wrapping_div(2 as ::core::ffi::c_uint) as uint32_t;
    (*coder).opts_end_index = 0 as uint32_t;
    (*coder).opts_current_index = 0 as uint32_t;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_encoder_create(
    mut coder_ptr: *mut *mut ::core::ffi::c_void,
    mut allocator: *const lzma_allocator,
    mut id: lzma_vli,
    mut options: *const lzma_options_lzma,
    mut lz_options: *mut lzma_lz_options,
) -> lzma_ret {
    if (*coder_ptr).is_null() {
        *coder_ptr = lzma_alloc(
            ::core::mem::size_of::<lzma_lzma1_encoder>() as size_t,
            allocator,
        );
        if (*coder_ptr).is_null() {
            return LZMA_MEM_ERROR;
        }
    }
    let mut coder: *mut lzma_lzma1_encoder = *coder_ptr as *mut lzma_lzma1_encoder;
    match (*options).mode as ::core::ffi::c_uint {
        1 => {
            (*coder).fast_mode = true_0 != 0;
        }
        2 => {
            (*coder).fast_mode = false_0 != 0;
            if (*options).dict_size
                > ((1 as uint32_t) << 30 as ::core::ffi::c_int)
                    .wrapping_add((1 as uint32_t) << 29 as ::core::ffi::c_int)
            {
                return LZMA_OPTIONS_ERROR;
            }
            let mut log_size: uint32_t = 0 as uint32_t;
            while (1 as uint32_t) << log_size < (*options).dict_size {
                log_size = log_size.wrapping_add(1);
            }
            (*coder).dist_table_size = log_size.wrapping_mul(2 as uint32_t);
            let nice_len: uint32_t = if mf_get_hash_bytes((*options).mf) > (*options).nice_len {
                mf_get_hash_bytes((*options).mf) as uint32_t
            } else {
                (*options).nice_len
            };
            (*coder).match_len_encoder.table_size = nice_len
                .wrapping_add(1 as uint32_t)
                .wrapping_sub(MATCH_LEN_MIN as uint32_t);
            (*coder).rep_len_encoder.table_size = nice_len
                .wrapping_add(1 as uint32_t)
                .wrapping_sub(MATCH_LEN_MIN as uint32_t);
        }
        _ => return LZMA_OPTIONS_ERROR,
    }
    (*coder).is_initialized =
        !(*options).preset_dict.is_null() && (*options).preset_dict_size > 0 as uint32_t;
    (*coder).is_flushed = false_0 != 0;
    (*coder).uncomp_size = 0 as uint64_t;
    (*coder).uncomp_size_ptr = ::core::ptr::null_mut::<uint64_t>();
    (*coder).out_limit = 0 as uint64_t;
    (*coder).use_eopm = id == LZMA_FILTER_LZMA1 as lzma_vli;
    if id == LZMA_FILTER_LZMA1EXT as lzma_vli {
        if (*options).ext_flags & !(LZMA_LZMA1EXT_ALLOW_EOPM as uint32_t) != 0 {
            return LZMA_OPTIONS_ERROR;
        }
        (*coder).use_eopm =
            (*options).ext_flags & LZMA_LZMA1EXT_ALLOW_EOPM as uint32_t != 0 as uint32_t;
    }
    set_lz_options(lz_options, options);
    return lzma_lzma_encoder_reset(coder, options);
}
unsafe extern "C" fn lzma_encoder_init(
    mut lz: *mut lzma_lz_encoder,
    mut allocator: *const lzma_allocator,
    mut id: lzma_vli,
    mut options: *const ::core::ffi::c_void,
    mut lz_options: *mut lzma_lz_options,
) -> lzma_ret {
    if options.is_null() {
        return LZMA_PROG_ERROR;
    }
    (*lz).code = Some(
        lzma_encode
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
    (*lz).set_out_limit = Some(
        lzma_lzma_set_out_limit
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, uint64_t) -> lzma_ret,
    )
        as Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, uint64_t) -> lzma_ret,
        >;
    return lzma_lzma_encoder_create(
        &raw mut (*lz).coder,
        allocator,
        id,
        options as *const lzma_options_lzma,
        lz_options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_encoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
) -> lzma_ret {
    return lzma_lz_encoder_init(
        next,
        allocator,
        filters,
        Some(
            lzma_encoder_init
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
pub unsafe extern "C" fn lzma_lzma_encoder_memusage(
    mut options: *const ::core::ffi::c_void,
) -> uint64_t {
    if !is_options_valid(options as *const lzma_options_lzma) {
        return UINT64_MAX as uint64_t;
    }
    let mut lz_options: lzma_lz_options = lzma_lz_options {
        before_size: 0,
        dict_size: 0,
        after_size: 0,
        match_len_max: 0,
        nice_len: 0,
        match_finder: 0 as lzma_match_finder,
        depth: 0,
        preset_dict: ::core::ptr::null::<uint8_t>(),
        preset_dict_size: 0,
    };
    set_lz_options(&raw mut lz_options, options as *const lzma_options_lzma);
    let lz_memusage: uint64_t = lzma_lz_encoder_memusage(&raw mut lz_options) as uint64_t;
    if lz_memusage == UINT64_MAX as uint64_t {
        return UINT64_MAX as uint64_t;
    }
    return (::core::mem::size_of::<lzma_lzma1_encoder>() as uint64_t).wrapping_add(lz_memusage);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_lclppb_encode(
    mut options: *const lzma_options_lzma,
    mut byte: *mut uint8_t,
) -> bool {
    if !is_lclppb_valid(options) {
        return true_0 != 0;
    }
    *byte = (*options)
        .pb
        .wrapping_mul(5 as uint32_t)
        .wrapping_add((*options).lp)
        .wrapping_mul(9 as uint32_t)
        .wrapping_add((*options).lc) as uint8_t;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_props_encode(
    mut options: *const ::core::ffi::c_void,
    mut out: *mut uint8_t,
) -> lzma_ret {
    if options.is_null() {
        return LZMA_PROG_ERROR;
    }
    let opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
    if lzma_lzma_lclppb_encode(opt, out) {
        return LZMA_PROG_ERROR;
    }
    write32le(
        out.offset(1 as ::core::ffi::c_int as isize),
        (*opt).dict_size,
    );
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mode_is_supported(mut mode: lzma_mode) -> lzma_bool {
    return (mode as ::core::ffi::c_uint
        == LZMA_MODE_FAST as ::core::ffi::c_int as ::core::ffi::c_uint
        || mode as ::core::ffi::c_uint
            == LZMA_MODE_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int as lzma_bool;
}
