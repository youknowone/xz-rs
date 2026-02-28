use crate::types::*;
use core::ffi::{c_int, c_long, c_uint, c_ulonglong, c_void};
extern "C" {
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
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
    fn lzma_lz_encoder_memusage(lz_options: *const lzma_lz_options) -> u64;
    static lzma_rc_prices: [u8; 128];
    fn lzma_lzma_optimum_fast(
        coder: *mut lzma_lzma1_encoder,
        mf: *mut lzma_mf,
        back_res: *mut u32,
        len_res: *mut u32,
    );
    fn lzma_lzma_optimum_normal(
        coder: *mut lzma_lzma1_encoder,
        mf: *mut lzma_mf,
        back_res: *mut u32,
        len_res: *mut u32,
        position: u32,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_range_encoder {
    pub low: u64,
    pub cache_size: u64,
    pub range: u32,
    pub cache: u8,
    pub out_total: u64,
    pub count: size_t,
    pub pos: size_t,
    pub symbols: [C2RustUnnamed; 53],
    pub probs: [*mut probability; 53],
}
pub type C2RustUnnamed = c_uint;
pub const RC_FLUSH: C2RustUnnamed = 4;
pub const RC_DIRECT_1: C2RustUnnamed = 3;
pub const RC_DIRECT_0: C2RustUnnamed = 2;
pub const RC_BIT_1: C2RustUnnamed = 1;
pub const RC_BIT_0: C2RustUnnamed = 0;
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
    pub uncomp_size: u64,
    pub out_limit: u64,
    pub uncomp_size_ptr: *mut u64,
    pub state: lzma_lzma_state,
    pub reps: [u32; 4],
    pub matches: [lzma_match; 274],
    pub matches_count: u32,
    pub longest_match_length: u32,
    pub fast_mode: bool,
    pub is_initialized: bool,
    pub is_flushed: bool,
    pub use_eopm: bool,
    pub pos_mask: u32,
    pub literal_context_bits: u32,
    pub literal_mask: u32,
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
    pub dist_slot_prices: [[u32; 64]; 4],
    pub dist_prices: [[u32; 128]; 4],
    pub dist_table_size: u32,
    pub match_price_count: u32,
    pub align_prices: [u32; 16],
    pub align_price_count: u32,
    pub opts_end_index: u32,
    pub opts_current_index: u32,
    pub opts: [lzma_optimal; OPTS as usize],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_optimal {
    pub state: lzma_lzma_state,
    pub prev_1_is_literal: bool,
    pub prev_2: bool,
    pub pos_prev_2: u32,
    pub back_prev_2: u32,
    pub price: u32,
    pub pos_prev: u32,
    pub back_prev: u32,
    pub backs: [u32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_length_encoder {
    pub choice: probability,
    pub choice2: probability,
    pub low: [[probability; 8]; 16],
    pub mid: [[probability; 8]; 16],
    pub high: [probability; 256],
    pub prices: [[u32; 272]; 16],
    pub table_size: u32,
    pub counters: [u32; 16],
}
pub type lzma_lzma1_encoder = lzma_lzma1_encoder_s;
pub const UINT32_MAX: c_uint = 4294967295;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
#[inline]
extern "C" fn write32le(buf: *mut u8, num: u32) {
    unsafe {
        *buf.offset(0) = num as u8;
        *buf.offset(1) = (num >> 8) as u8;
        *buf.offset(2) = (num >> 16) as u8;
        *buf.offset(3) = (num >> 24) as u8;
    }
}
pub const LZMA_FILTER_LZMA1: c_ulonglong = 0x4000000000000001;
pub const LZMA_FILTER_LZMA1EXT: c_ulonglong = 0x4000000000000002;
pub const LZMA_LCLP_MAX: c_int = 4;
pub const LZMA_PB_MAX: c_int = 4;
pub const LZMA_LZMA1EXT_ALLOW_EOPM: c_uint = 0x1;
pub const LZMA2_CHUNK_MAX: c_uint = 1u32 << 16;
#[inline]
extern "C" fn mf_get_hash_bytes(match_finder: lzma_match_finder) -> u32 {
    return match_finder as u32 & 0xf as u32;
}
#[inline]
unsafe extern "C" fn mf_skip(mf: *mut lzma_mf, amount: u32) {
    if amount != 0 {
        (*mf).skip.expect("non-null function pointer")(mf, amount);
        (*mf).read_ahead = (*mf).read_ahead.wrapping_add(amount);
    }
}
pub const RC_SHIFT_BITS: c_int = 8;
pub const RC_TOP_BITS: c_int = 24;
pub const RC_TOP_VALUE: c_uint = 1u32 << RC_TOP_BITS;
pub const RC_BIT_MODEL_TOTAL_BITS: c_int = 11;
pub const RC_BIT_MODEL_TOTAL: c_uint = 1u32 << RC_BIT_MODEL_TOTAL_BITS;
pub const RC_MOVE_BITS: c_int = 5;
pub const RC_MOVE_REDUCING_BITS: c_int = 4;
#[inline]
extern "C" fn rc_bit_price(prob: probability, bit: u32) -> u32 {
    return unsafe {
        lzma_rc_prices[((prob as u32
            ^ 0u32.wrapping_sub(bit) & (RC_BIT_MODEL_TOTAL as u32).wrapping_sub(1))
            >> RC_MOVE_REDUCING_BITS) as usize] as u32
    };
}
#[inline]
extern "C" fn rc_bit_0_price(prob: probability) -> u32 {
    return unsafe { lzma_rc_prices[(prob as c_int >> RC_MOVE_REDUCING_BITS) as usize] as u32 };
}
#[inline]
extern "C" fn rc_bit_1_price(prob: probability) -> u32 {
    return unsafe {
        lzma_rc_prices
            [((prob as u32 ^ RC_BIT_MODEL_TOTAL.wrapping_sub(1)) >> RC_MOVE_REDUCING_BITS) as usize]
            as u32
    };
}
#[inline]
unsafe extern "C" fn rc_bittree_price(
    probs: *const probability,
    bit_levels: u32,
    mut symbol: u32,
) -> u32 {
    let mut price: u32 = 0;
    symbol = (symbol as u32).wrapping_add(1u32 << bit_levels) as u32;
    loop {
        let bit: u32 = symbol & 1;
        symbol >>= 1 as c_int;
        price = price.wrapping_add(rc_bit_price(*probs.offset(symbol as isize), bit));
        if !(symbol != 1) {
            break;
        }
    }
    return price;
}
#[inline]
unsafe extern "C" fn rc_reset(rc: *mut lzma_range_encoder) {
    (*rc).low = 0;
    (*rc).cache_size = 1;
    (*rc).range = UINT32_MAX as u32;
    (*rc).cache = 0;
    (*rc).out_total = 0;
    (*rc).count = 0;
    (*rc).pos = 0;
}
#[inline]
unsafe extern "C" fn rc_forget(rc: *mut lzma_range_encoder) {
    (*rc).count = 0;
}
#[inline]
unsafe extern "C" fn rc_bit(rc: *mut lzma_range_encoder, prob: *mut probability, bit: u32) {
    (*rc).symbols[(*rc).count as usize] = bit as C2RustUnnamed;
    (*rc).probs[(*rc).count as usize] = prob;
    (*rc).count = (*rc).count.wrapping_add(1);
}
#[inline]
unsafe extern "C" fn rc_bittree(
    rc: *mut lzma_range_encoder,
    probs: *mut probability,
    mut bit_count: u32,
    symbol: u32,
) {
    let mut model_index: u32 = 1;
    loop {
        bit_count = bit_count.wrapping_sub(1);
        let bit: u32 = symbol >> bit_count & 1;
        rc_bit(
            rc,
            probs.offset(model_index as isize) as *mut probability,
            bit,
        );
        model_index = (model_index << 1).wrapping_add(bit);
        if !(bit_count != 0) {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn rc_bittree_reverse(
    rc: *mut lzma_range_encoder,
    probs: *mut probability,
    mut bit_count: u32,
    mut symbol: u32,
) {
    let mut model_index: u32 = 1;
    loop {
        let bit: u32 = symbol & 1;
        symbol >>= 1 as c_int;
        rc_bit(
            rc,
            probs.offset(model_index as isize) as *mut probability,
            bit,
        );
        model_index = (model_index << 1).wrapping_add(bit);
        bit_count = bit_count.wrapping_sub(1);
        if !(bit_count != 0) {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn rc_direct(rc: *mut lzma_range_encoder, value: u32, mut bit_count: u32) {
    loop {
        bit_count = bit_count.wrapping_sub(1);
        let fresh0 = (*rc).count;
        (*rc).count = (*rc).count.wrapping_add(1);
        (*rc).symbols[fresh0 as usize] =
            (RC_DIRECT_0 as u32).wrapping_add(value >> bit_count & 1) as C2RustUnnamed;
        if !(bit_count != 0) {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn rc_flush(rc: *mut lzma_range_encoder) {
    let mut i: size_t = 0;
    while i < 5 {
        let fresh1 = (*rc).count;
        (*rc).count = (*rc).count.wrapping_add(1);
        (*rc).symbols[fresh1 as usize] = RC_FLUSH;
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn rc_shift_low(
    rc: *mut lzma_range_encoder,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> bool {
    if ((*rc).low as u32) < 0xff000000 as u32 || ((*rc).low >> 32) as u32 != 0 {
        loop {
            if *out_pos == out_size {
                return true;
            }
            *out.offset(*out_pos as isize) =
                ((*rc).cache as c_int + ((*rc).low >> 32) as u8 as c_int) as u8;
            *out_pos = (*out_pos).wrapping_add(1);
            (*rc).out_total = (*rc).out_total.wrapping_add(1);
            (*rc).cache = 0xff as u8;
            (*rc).cache_size = (*rc).cache_size.wrapping_sub(1);
            if !((*rc).cache_size != 0) {
                break;
            }
        }
        (*rc).cache = ((*rc).low >> 24 & 0xff as u64) as u8;
    }
    (*rc).cache_size = (*rc).cache_size.wrapping_add(1);
    (*rc).low = ((*rc).low & 0xffffff as u64) << RC_SHIFT_BITS;
    return false;
}
#[inline]
unsafe extern "C" fn rc_shift_low_dummy(
    low: *mut u64,
    cache_size: *mut u64,
    cache: *mut u8,
    out_pos: *mut u64,
    out_size: u64,
) -> bool {
    if (*low as u32) < 0xff000000 as u32 || (*low >> 32) as u32 != 0 {
        loop {
            if *out_pos == out_size {
                return true;
            }
            *out_pos = (*out_pos).wrapping_add(1);
            *cache = 0xff as u8;
            *cache_size = (*cache_size).wrapping_sub(1);
            if !(*cache_size != 0) {
                break;
            }
        }
        *cache = (*low >> 24 & 0xff as u64) as u8;
    }
    *cache_size = (*cache_size).wrapping_add(1);
    *low = (*low & 0xffffff as u64) << RC_SHIFT_BITS;
    return false;
}
#[inline]
unsafe extern "C" fn rc_encode(
    rc: *mut lzma_range_encoder,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> bool {
    while (*rc).pos < (*rc).count {
        if (*rc).range < RC_TOP_VALUE as u32 {
            if rc_shift_low(rc, out, out_pos, out_size) {
                return true;
            }
            (*rc).range <<= RC_SHIFT_BITS;
        }
        match (*rc).symbols[(*rc).pos as usize] {
            0 => {
                let mut prob: probability = *(*rc).probs[(*rc).pos as usize];
                (*rc).range = ((*rc).range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(prob as u32);
                prob = (prob as u32)
                    .wrapping_add(RC_BIT_MODEL_TOTAL.wrapping_sub(prob as u32) >> RC_MOVE_BITS)
                    as probability as probability;
                *(*rc).probs[(*rc).pos as usize] = prob;
            }
            1 => {
                let mut prob_0: probability = *(*rc).probs[(*rc).pos as usize];
                let bound: u32 =
                    (prob_0 as u32).wrapping_mul((*rc).range >> RC_BIT_MODEL_TOTAL_BITS);
                (*rc).low = (*rc).low.wrapping_add(bound as u64);
                (*rc).range = (*rc).range.wrapping_sub(bound);
                prob_0 = (prob_0 as c_int - (prob_0 as c_int >> RC_MOVE_BITS)) as probability;
                *(*rc).probs[(*rc).pos as usize] = prob_0;
            }
            2 => {
                (*rc).range >>= 1 as c_int;
            }
            3 => {
                (*rc).range >>= 1 as c_int;
                (*rc).low = (*rc).low.wrapping_add((*rc).range as u64);
            }
            4 => {
                (*rc).range = UINT32_MAX as u32;
                loop {
                    if rc_shift_low(rc, out, out_pos, out_size) {
                        return true;
                    }
                    (*rc).pos = (*rc).pos.wrapping_add(1);
                    if !((*rc).pos < (*rc).count) {
                        break;
                    }
                }
                rc_reset(rc);
                return false;
            }
            _ => {}
        }
        (*rc).pos = (*rc).pos.wrapping_add(1);
    }
    (*rc).count = 0;
    (*rc).pos = 0;
    return false;
}
#[inline]
unsafe extern "C" fn rc_encode_dummy(rc: *const lzma_range_encoder, out_limit: u64) -> bool {
    let mut low: u64 = (*rc).low;
    let mut cache_size: u64 = (*rc).cache_size;
    let mut range: u32 = (*rc).range;
    let mut cache: u8 = (*rc).cache;
    let mut out_pos: u64 = (*rc).out_total;
    let mut pos: size_t = (*rc).pos;
    loop {
        if range < RC_TOP_VALUE as u32 {
            if rc_shift_low_dummy(
                &raw mut low,
                &raw mut cache_size,
                &raw mut cache,
                &raw mut out_pos,
                out_limit,
            ) {
                return true;
            }
            range <<= RC_SHIFT_BITS;
        }
        if pos == (*rc).count {
            break;
        }
        match (*rc).symbols[pos as usize] {
            0 => {
                let prob: probability = *(*rc).probs[pos as usize];
                range = (range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(prob as u32);
            }
            1 => {
                let prob_0: probability = *(*rc).probs[pos as usize];
                let bound: u32 = (prob_0 as u32).wrapping_mul(range >> RC_BIT_MODEL_TOTAL_BITS);
                low = low.wrapping_add(bound as u64);
                range = range.wrapping_sub(bound);
            }
            2 => {
                range >>= 1 as c_int;
            }
            3 => {
                range >>= 1 as c_int;
                low = low.wrapping_add(range as u64);
            }
            4 | _ => {}
        }
        pos = pos.wrapping_add(1);
    }
    pos = 0;
    while pos < 5 {
        if rc_shift_low_dummy(
            &raw mut low,
            &raw mut cache_size,
            &raw mut cache,
            &raw mut out_pos,
            out_limit,
        ) {
            return true;
        }
        pos = pos.wrapping_add(1);
    }
    return false;
}
#[inline]
unsafe extern "C" fn rc_pending(rc: *const lzma_range_encoder) -> u64 {
    return (*rc).cache_size.wrapping_add(5).wrapping_sub(1);
}
#[inline]
unsafe extern "C" fn is_lclppb_valid(options: *const lzma_options_lzma) -> bool {
    return (*options).lc <= LZMA_LCLP_MAX as u32
        && (*options).lp <= LZMA_LCLP_MAX as u32
        && (*options).lc.wrapping_add((*options).lp) <= LZMA_LCLP_MAX as u32
        && (*options).pb <= LZMA_PB_MAX as u32;
}
pub const STATES: c_int = 12;
pub const LIT_STATES: c_int = 7;
pub const LITERAL_CODER_SIZE: c_uint = 0x300;
#[inline]
unsafe extern "C" fn literal_init(probs: *mut probability, lc: u32, lp: u32) {
    let coders: size_t = (LITERAL_CODER_SIZE << lc.wrapping_add(lp)) as size_t;
    let mut i: size_t = 0;
    while i < coders {
        *probs.offset(i as isize) = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        i = i.wrapping_add(1);
    }
}
pub const MATCH_LEN_MIN: c_int = 2;
pub const LEN_LOW_BITS: c_int = 3;
pub const LEN_LOW_SYMBOLS: c_int = (1) << LEN_LOW_BITS;
pub const LEN_MID_BITS: c_int = 3;
pub const LEN_MID_SYMBOLS: c_int = (1) << LEN_MID_BITS;
pub const LEN_HIGH_BITS: c_int = 8;
pub const LEN_HIGH_SYMBOLS: c_int = (1) << LEN_HIGH_BITS;
pub const LEN_SYMBOLS: c_int = LEN_LOW_SYMBOLS + LEN_MID_SYMBOLS + LEN_HIGH_SYMBOLS;
pub const MATCH_LEN_MAX: c_int = MATCH_LEN_MIN + LEN_SYMBOLS - 1;
pub const DIST_STATES: c_int = 4;
pub const DIST_SLOT_BITS: c_int = 6;
pub const DIST_MODEL_START: c_int = 4;
pub const DIST_MODEL_END: c_int = 14;
pub const FULL_DISTANCES_BITS: c_int = DIST_MODEL_END / 2;
pub const FULL_DISTANCES: c_int = (1) << FULL_DISTANCES_BITS;
pub const ALIGN_BITS: c_int = 4;
pub const ALIGN_SIZE: c_int = (1) << ALIGN_BITS;
pub const ALIGN_MASK: c_int = ALIGN_SIZE - 1;
pub const REPS: c_int = 4;
pub const OPTS: c_int = (1) << 12;
pub const FASTPOS_BITS: c_int = 13;
#[inline]
unsafe extern "C" fn get_dist_slot(dist: u32) -> u32 {
    if dist < (1) << FASTPOS_BITS + (0 as c_int + 0 as c_int * (FASTPOS_BITS - 1 as c_int)) {
        return lzma_fastpos[dist as usize] as u32;
    }
    if dist < (1) << FASTPOS_BITS + (0 as c_int + 1 as c_int * (FASTPOS_BITS - 1 as c_int)) {
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
#[inline]
unsafe extern "C" fn literal_matched(
    rc: *mut lzma_range_encoder,
    subcoder: *mut probability,
    mut match_byte: u32,
    mut symbol: u32,
) {
    let mut offset: u32 = 0x100 as u32;
    symbol = (symbol as u32).wrapping_add(1u32 << 8) as u32;
    loop {
        match_byte <<= 1 as c_int;
        let match_bit: u32 = match_byte & offset;
        let subcoder_index: u32 = offset.wrapping_add(match_bit).wrapping_add(symbol >> 8);
        let bit: u32 = symbol >> 7 & 1;
        rc_bit(
            rc,
            subcoder.offset(subcoder_index as isize) as *mut probability,
            bit,
        );
        symbol <<= 1 as c_int;
        offset &= !(match_byte ^ symbol);
        if !(symbol < (1) << 16) {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn literal(coder: *mut lzma_lzma1_encoder, mf: *mut lzma_mf, position: u32) {
    let cur_byte: u8 = *(*mf)
        .buffer
        .offset((*mf).read_pos.wrapping_sub((*mf).read_ahead) as isize);
    let subcoder: *mut probability = (&raw mut (*coder).literal as *mut probability).offset(
        (3u32).wrapping_mul(
            ((position << 8).wrapping_add(
                *(*mf).buffer.offset(
                    (*mf)
                        .read_pos
                        .wrapping_sub((*mf).read_ahead)
                        .wrapping_sub(1) as isize,
                ) as u32,
            ) & (*coder).literal_mask)
                << (*coder).literal_context_bits,
        ) as isize,
    );
    if ((*coder).state as u32) < LIT_STATES as u32 {
        (*coder).state = (if (*coder).state <= STATE_SHORTREP_LIT_LIT {
            STATE_LIT_LIT
        } else {
            ((*coder).state as u32).wrapping_sub(3)
        }) as lzma_lzma_state;
        rc_bittree(&raw mut (*coder).rc, subcoder, 8, cur_byte as u32);
    } else {
        (*coder).state = (if (*coder).state <= STATE_LIT_SHORTREP {
            ((*coder).state as u32).wrapping_sub(3)
        } else {
            ((*coder).state as u32).wrapping_sub(6)
        }) as lzma_lzma_state;
        let match_byte: u8 = *(*mf).buffer.offset(
            (*mf)
                .read_pos
                .wrapping_sub((*coder).reps[0])
                .wrapping_sub(1)
                .wrapping_sub((*mf).read_ahead) as isize,
        );
        literal_matched(
            &raw mut (*coder).rc,
            subcoder,
            match_byte as u32,
            cur_byte as u32,
        );
    };
}
unsafe extern "C" fn length_update_prices(lc: *mut lzma_length_encoder, pos_state: u32) {
    let table_size: u32 = (*lc).table_size;
    (*lc).counters[pos_state as usize] = table_size;
    let a0: u32 = rc_bit_0_price((*lc).choice) as u32;
    let a1: u32 = rc_bit_1_price((*lc).choice) as u32;
    let b0: u32 = a1.wrapping_add(rc_bit_0_price((*lc).choice2) as u32);
    let b1: u32 = a1.wrapping_add(rc_bit_1_price((*lc).choice2) as u32);
    let prices: *mut u32 =
        &raw mut *(&raw mut (*lc).prices as *mut [u32; 272]).offset(pos_state as isize) as *mut u32;
    let mut i: u32 = 0;
    i = 0;
    while i < table_size && i < LEN_LOW_SYMBOLS as u32 {
        *prices.offset(i as isize) = a0.wrapping_add(rc_bittree_price(
            &raw mut *(&raw mut (*lc).low as *mut [probability; 8]).offset(pos_state as isize)
                as *mut probability,
            LEN_LOW_BITS as u32,
            i,
        ));
        i = i.wrapping_add(1);
    }
    while i < table_size && i < (LEN_LOW_SYMBOLS + LEN_MID_SYMBOLS) as u32 {
        *prices.offset(i as isize) = b0.wrapping_add(rc_bittree_price(
            &raw mut *(&raw mut (*lc).mid as *mut [probability; 8]).offset(pos_state as isize)
                as *mut probability,
            LEN_MID_BITS as u32,
            i.wrapping_sub(LEN_LOW_SYMBOLS as u32),
        ));
        i = i.wrapping_add(1);
    }
    while i < table_size {
        *prices.offset(i as isize) = b1.wrapping_add(rc_bittree_price(
            &raw mut (*lc).high as *mut probability,
            LEN_HIGH_BITS as u32,
            i.wrapping_sub(LEN_LOW_SYMBOLS as u32)
                .wrapping_sub(LEN_MID_SYMBOLS as u32),
        ));
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn length(
    rc: *mut lzma_range_encoder,
    lc: *mut lzma_length_encoder,
    pos_state: u32,
    mut len: u32,
    fast_mode: bool,
) {
    len = len.wrapping_sub(MATCH_LEN_MIN as u32);
    if len < LEN_LOW_SYMBOLS as u32 {
        rc_bit(rc, &raw mut (*lc).choice, 0);
        rc_bittree(
            rc,
            &raw mut *(&raw mut (*lc).low as *mut [probability; 8]).offset(pos_state as isize)
                as *mut probability,
            LEN_LOW_BITS as u32,
            len,
        );
    } else {
        rc_bit(rc, &raw mut (*lc).choice, 1);
        len = len.wrapping_sub(LEN_LOW_SYMBOLS as u32);
        if len < LEN_MID_SYMBOLS as u32 {
            rc_bit(rc, &raw mut (*lc).choice2, 0);
            rc_bittree(
                rc,
                &raw mut *(&raw mut (*lc).mid as *mut [probability; 8]).offset(pos_state as isize)
                    as *mut probability,
                LEN_MID_BITS as u32,
                len,
            );
        } else {
            rc_bit(rc, &raw mut (*lc).choice2, 1);
            len = len.wrapping_sub(LEN_MID_SYMBOLS as u32);
            rc_bittree(
                rc,
                &raw mut (*lc).high as *mut probability,
                LEN_HIGH_BITS as u32,
                len,
            );
        }
    }
    if !fast_mode {
        (*lc).counters[pos_state as usize] = (*lc).counters[pos_state as usize].wrapping_sub(1);
        if (*lc).counters[pos_state as usize] == 0 {
            length_update_prices(lc, pos_state);
        }
    }
}
#[inline]
unsafe extern "C" fn match_0(
    coder: *mut lzma_lzma1_encoder,
    pos_state: u32,
    distance: u32,
    len: u32,
) {
    (*coder).state = (if ((*coder).state as u32) < LIT_STATES as u32 {
        STATE_LIT_MATCH as c_int
    } else {
        STATE_NONLIT_MATCH as c_int
    }) as lzma_lzma_state;
    length(
        &raw mut (*coder).rc,
        &raw mut (*coder).match_len_encoder,
        pos_state,
        len,
        (*coder).fast_mode,
    );
    let dist_slot: u32 = get_dist_slot(distance) as u32;
    let dist_state: u32 = if len < (DIST_STATES + MATCH_LEN_MIN) as u32 {
        len.wrapping_sub(MATCH_LEN_MIN as u32)
    } else {
        (DIST_STATES - 1 as c_int) as u32
    };
    rc_bittree(
        &raw mut (*coder).rc,
        &raw mut *(&raw mut (*coder).dist_slot as *mut [probability; 64])
            .offset(dist_state as isize) as *mut probability,
        DIST_SLOT_BITS as u32,
        dist_slot,
    );
    if dist_slot >= DIST_MODEL_START as u32 {
        let footer_bits: u32 = (dist_slot >> 1).wrapping_sub(1);
        let base: u32 = (2 | dist_slot & 1) << footer_bits;
        let dist_reduced: u32 = distance.wrapping_sub(base);
        if dist_slot < DIST_MODEL_END as u32 {
            rc_bittree_reverse(
                &raw mut (*coder).rc,
                (&raw mut (*coder).dist_special as *mut probability)
                    .offset(base as isize)
                    .offset(-(dist_slot as isize))
                    .offset(-1),
                footer_bits,
                dist_reduced,
            );
        } else {
            rc_direct(
                &raw mut (*coder).rc,
                dist_reduced >> ALIGN_BITS,
                footer_bits.wrapping_sub(ALIGN_BITS as u32),
            );
            rc_bittree_reverse(
                &raw mut (*coder).rc,
                &raw mut (*coder).dist_align as *mut probability,
                ALIGN_BITS as u32,
                dist_reduced & ALIGN_MASK as u32,
            );
            (*coder).align_price_count = (*coder).align_price_count.wrapping_add(1);
        }
    }
    (*coder).reps[3] = (*coder).reps[2];
    (*coder).reps[2] = (*coder).reps[1];
    (*coder).reps[1] = (*coder).reps[0];
    (*coder).reps[0] = distance;
    (*coder).match_price_count = (*coder).match_price_count.wrapping_add(1);
}
#[inline]
unsafe extern "C" fn rep_match(coder: *mut lzma_lzma1_encoder, pos_state: u32, rep: u32, len: u32) {
    if rep == 0 {
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut (*coder).is_rep0 as *mut probability).offset((*coder).state as isize)
                as *mut probability,
            0,
        );
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut *(&raw mut (*coder).is_rep0_long as *mut [probability; 16])
                .offset((*coder).state as isize) as *mut probability)
                .offset(pos_state as isize) as *mut probability,
            (len != 1) as u32,
        );
    } else {
        let distance: u32 = (*coder).reps[rep as usize];
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut (*coder).is_rep0 as *mut probability).offset((*coder).state as isize)
                as *mut probability,
            1,
        );
        if rep == 1 {
            rc_bit(
                &raw mut (*coder).rc,
                (&raw mut (*coder).is_rep1 as *mut probability).offset((*coder).state as isize)
                    as *mut probability,
                0,
            );
        } else {
            rc_bit(
                &raw mut (*coder).rc,
                (&raw mut (*coder).is_rep1 as *mut probability).offset((*coder).state as isize)
                    as *mut probability,
                1,
            );
            rc_bit(
                &raw mut (*coder).rc,
                (&raw mut (*coder).is_rep2 as *mut probability).offset((*coder).state as isize)
                    as *mut probability,
                rep.wrapping_sub(2),
            );
            if rep == 3 {
                (*coder).reps[3] = (*coder).reps[2];
            }
            (*coder).reps[2] = (*coder).reps[1];
        }
        (*coder).reps[1] = (*coder).reps[0];
        (*coder).reps[0] = distance;
    }
    if len == 1 {
        (*coder).state = (if ((*coder).state as u32) < LIT_STATES as u32 {
            STATE_LIT_SHORTREP as c_int
        } else {
            STATE_NONLIT_REP as c_int
        }) as lzma_lzma_state;
    } else {
        length(
            &raw mut (*coder).rc,
            &raw mut (*coder).rep_len_encoder,
            pos_state,
            len,
            (*coder).fast_mode,
        );
        (*coder).state = (if ((*coder).state as u32) < LIT_STATES as u32 {
            STATE_LIT_LONGREP as c_int
        } else {
            STATE_NONLIT_REP as c_int
        }) as lzma_lzma_state;
    };
}
unsafe extern "C" fn encode_symbol(
    coder: *mut lzma_lzma1_encoder,
    mf: *mut lzma_mf,
    back: u32,
    len: u32,
    position: u32,
) {
    let pos_state: u32 = position & (*coder).pos_mask;
    if back == UINT32_MAX as u32 {
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut *(&raw mut (*coder).is_match as *mut [probability; 16])
                .offset((*coder).state as isize) as *mut probability)
                .offset(pos_state as isize) as *mut probability,
            0,
        );
        literal(coder, mf, position);
    } else {
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut *(&raw mut (*coder).is_match as *mut [probability; 16])
                .offset((*coder).state as isize) as *mut probability)
                .offset(pos_state as isize) as *mut probability,
            1,
        );
        if back < REPS as u32 {
            rc_bit(
                &raw mut (*coder).rc,
                (&raw mut (*coder).is_rep as *mut probability).offset((*coder).state as isize)
                    as *mut probability,
                1,
            );
            rep_match(coder, pos_state, back, len);
        } else {
            rc_bit(
                &raw mut (*coder).rc,
                (&raw mut (*coder).is_rep as *mut probability).offset((*coder).state as isize)
                    as *mut probability,
                0,
            );
            match_0(coder, pos_state, back.wrapping_sub(REPS as u32), len);
        }
    }
    (*mf).read_ahead = (*mf).read_ahead.wrapping_sub(len);
}
unsafe extern "C" fn encode_init(coder: *mut lzma_lzma1_encoder, mf: *mut lzma_mf) -> bool {
    if (*mf).read_pos == (*mf).read_limit {
        if (*mf).action == LZMA_RUN {
            return false;
        }
    } else {
        mf_skip(mf, 1);
        (*mf).read_ahead = 0;
        rc_bit(
            &raw mut (*coder).rc,
            (&raw mut *(&raw mut (*coder).is_match as *mut [probability; 16]).offset(0)
                as *mut probability)
                .offset(0) as *mut probability,
            0,
        );
        rc_bittree(
            &raw mut (*coder).rc,
            (&raw mut (*coder).literal as *mut probability).offset(0),
            8,
            *(*mf).buffer.offset(0) as u32,
        );
        (*coder).uncomp_size = (*coder).uncomp_size.wrapping_add(1);
    }
    (*coder).is_initialized = true;
    return true;
}
unsafe extern "C" fn encode_eopm(coder: *mut lzma_lzma1_encoder, position: u32) {
    let pos_state: u32 = position & (*coder).pos_mask;
    rc_bit(
        &raw mut (*coder).rc,
        (&raw mut *(&raw mut (*coder).is_match as *mut [probability; 16])
            .offset((*coder).state as isize) as *mut probability)
            .offset(pos_state as isize) as *mut probability,
        1,
    );
    rc_bit(
        &raw mut (*coder).rc,
        (&raw mut (*coder).is_rep as *mut probability).offset((*coder).state as isize)
            as *mut probability,
        0,
    );
    match_0(coder, pos_state, UINT32_MAX as u32, MATCH_LEN_MIN as u32);
}
pub const LOOP_INPUT_MAX: c_int = OPTS + 1;
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_encode(
    coder: *mut lzma_lzma1_encoder,
    mf: *mut lzma_mf,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
    limit: u32,
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
    while !(limit != UINT32_MAX as u32
        && ((*mf).read_pos.wrapping_sub((*mf).read_ahead) >= limit
            || (*out_pos as u64).wrapping_add(rc_pending(&raw mut (*coder).rc))
                >= LZMA2_CHUNK_MAX.wrapping_sub(LOOP_INPUT_MAX as u32) as u64))
    {
        if (*mf).read_pos >= (*mf).read_limit {
            if (*mf).action == LZMA_RUN {
                return LZMA_OK;
            }
            if (*mf).read_ahead == 0 {
                break;
            }
        }
        let mut len: u32 = 0;
        let mut back: u32 = 0;
        if (*coder).fast_mode {
            lzma_lzma_optimum_fast(coder, mf, &raw mut back, &raw mut len);
        } else {
            lzma_lzma_optimum_normal(
                coder,
                mf,
                &raw mut back,
                &raw mut len,
                (*coder).uncomp_size as u32,
            );
        }
        encode_symbol(coder, mf, back, len, (*coder).uncomp_size as u32);
        if (*coder).out_limit != 0
            && rc_encode_dummy(&raw mut (*coder).rc, (*coder).out_limit) as c_int != 0
        {
            rc_forget(&raw mut (*coder).rc);
            break;
        } else {
            (*coder).uncomp_size = (*coder).uncomp_size.wrapping_add(len as u64);
            if rc_encode(&raw mut (*coder).rc, out, out_pos, out_size) {
                return LZMA_OK;
            }
        }
    }
    if !(*coder).uncomp_size_ptr.is_null() {
        *(*coder).uncomp_size_ptr = (*coder).uncomp_size;
    }
    if (*coder).use_eopm {
        encode_eopm(coder, (*coder).uncomp_size as u32);
    }
    rc_flush(&raw mut (*coder).rc);
    if rc_encode(&raw mut (*coder).rc, out, out_pos, out_size) {
        (*coder).is_flushed = true;
        return LZMA_OK;
    }
    return LZMA_STREAM_END;
}
unsafe extern "C" fn lzma_encode(
    coder: *mut c_void,
    mf: *mut lzma_mf,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    if ((*mf).action == LZMA_SYNC_FLUSH) as c_long != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    return lzma_lzma_encode(
        coder as *mut lzma_lzma1_encoder,
        mf,
        out,
        out_pos,
        out_size,
        UINT32_MAX as u32,
    );
}
unsafe extern "C" fn lzma_lzma_set_out_limit(
    coder_ptr: *mut c_void,
    uncomp_size: *mut u64,
    out_limit: u64,
) -> lzma_ret {
    if out_limit < 6 {
        return LZMA_BUF_ERROR;
    }
    let coder: *mut lzma_lzma1_encoder = coder_ptr as *mut lzma_lzma1_encoder;
    (*coder).out_limit = out_limit;
    (*coder).uncomp_size_ptr = uncomp_size;
    (*coder).use_eopm = false;
    return LZMA_OK;
}
extern "C" fn is_options_valid(options: *const lzma_options_lzma) -> bool {
    return unsafe {
        is_lclppb_valid(options) as c_int != 0
            && (*options).nice_len >= MATCH_LEN_MIN as u32
            && (*options).nice_len <= MATCH_LEN_MAX as u32
            && ((*options).mode == LZMA_MODE_FAST || (*options).mode == LZMA_MODE_NORMAL)
    };
}
extern "C" fn set_lz_options(
    lz_options: *mut lzma_lz_options,
    options: *const lzma_options_lzma,
) {
    unsafe {
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
}
unsafe extern "C" fn length_encoder_reset(
    lencoder: *mut lzma_length_encoder,
    num_pos_states: u32,
    fast_mode: bool,
) {
    (*lencoder).choice = (RC_BIT_MODEL_TOTAL >> 1) as probability;
    (*lencoder).choice2 = (RC_BIT_MODEL_TOTAL >> 1) as probability;
    let mut pos_state: size_t = 0;
    while pos_state < num_pos_states as size_t {
        let mut bt_i: u32 = 0;
        while bt_i < ((1 as c_int) << 3) as u32 {
            (*lencoder).low[pos_state as usize][bt_i as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            bt_i = bt_i.wrapping_add(1);
        }
        let mut bt_i_0: u32 = 0;
        while bt_i_0 < ((1 as c_int) << 3) as u32 {
            (*lencoder).mid[pos_state as usize][bt_i_0 as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            bt_i_0 = bt_i_0.wrapping_add(1);
        }
        pos_state = pos_state.wrapping_add(1);
    }
    let mut bt_i_1: u32 = 0;
    while bt_i_1 < ((1 as c_int) << 8) as u32 {
        (*lencoder).high[bt_i_1 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        bt_i_1 = bt_i_1.wrapping_add(1);
    }
    if !fast_mode {
        let mut pos_state_0: u32 = 0;
        while pos_state_0 < num_pos_states {
            length_update_prices(lencoder, pos_state_0);
            pos_state_0 = pos_state_0.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_encoder_reset(
    coder: *mut lzma_lzma1_encoder,
    options: *const lzma_options_lzma,
) -> lzma_ret {
    if !is_options_valid(options) {
        return LZMA_OPTIONS_ERROR;
    }
    (*coder).pos_mask = (1u32 << (*options).pb).wrapping_sub(1) as u32;
    (*coder).literal_context_bits = (*options).lc;
    (*coder).literal_mask =
        ((0x100 as u32) << (*options).lp).wrapping_sub(0x100 >> (*options).lc) as u32;
    rc_reset(&raw mut (*coder).rc);
    (*coder).state = STATE_LIT_LIT;
    let mut i: size_t = 0;
    while i < REPS as size_t {
        (*coder).reps[i as usize] = 0;
        i = i.wrapping_add(1);
    }
    literal_init(
        &raw mut (*coder).literal as *mut probability,
        (*options).lc,
        (*options).lp,
    );
    let mut i_0: size_t = 0;
    while i_0 < STATES as size_t {
        let mut j: size_t = 0;
        while j <= (*coder).pos_mask as size_t {
            (*coder).is_match[i_0 as usize][j as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
            (*coder).is_rep0_long[i_0 as usize][j as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            j = j.wrapping_add(1);
        }
        (*coder).is_rep[i_0 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        (*coder).is_rep0[i_0 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        (*coder).is_rep1[i_0 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        (*coder).is_rep2[i_0 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: size_t = 0;
    while i_1 < (FULL_DISTANCES - DIST_MODEL_END) as size_t {
        (*coder).dist_special[i_1 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        i_1 = i_1.wrapping_add(1);
    }
    let mut i_2: size_t = 0;
    while i_2 < DIST_STATES as size_t {
        let mut bt_i: u32 = 0;
        while bt_i < ((1 as c_int) << 6) as u32 {
            (*coder).dist_slot[i_2 as usize][bt_i as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            bt_i = bt_i.wrapping_add(1);
        }
        i_2 = i_2.wrapping_add(1);
    }
    let mut bt_i_0: u32 = 0;
    while bt_i_0 < ((1 as c_int) << 4) as u32 {
        (*coder).dist_align[bt_i_0 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        bt_i_0 = bt_i_0.wrapping_add(1);
    }
    length_encoder_reset(
        &raw mut (*coder).match_len_encoder,
        (1) << (*options).pb,
        (*coder).fast_mode,
    );
    length_encoder_reset(
        &raw mut (*coder).rep_len_encoder,
        (1) << (*options).pb,
        (*coder).fast_mode,
    );
    (*coder).match_price_count = UINT32_MAX.wrapping_div(2) as u32;
    (*coder).align_price_count = UINT32_MAX.wrapping_div(2) as u32;
    (*coder).opts_end_index = 0;
    (*coder).opts_current_index = 0;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_encoder_create(
    coder_ptr: *mut *mut c_void,
    allocator: *const lzma_allocator,
    id: lzma_vli,
    options: *const lzma_options_lzma,
    lz_options: *mut lzma_lz_options,
) -> lzma_ret {
    if (*coder_ptr).is_null() {
        *coder_ptr = lzma_alloc(
            core::mem::size_of::<lzma_lzma1_encoder>() as size_t,
            allocator,
        );
        if (*coder_ptr).is_null() {
            return LZMA_MEM_ERROR;
        }
    }
    let coder: *mut lzma_lzma1_encoder = *coder_ptr as *mut lzma_lzma1_encoder;
    match (*options).mode {
        1 => {
            (*coder).fast_mode = true;
        }
        2 => {
            (*coder).fast_mode = false;
            if (*options).dict_size > (1u32 << 30).wrapping_add((1) << 29) {
                return LZMA_OPTIONS_ERROR;
            }
            let mut log_size: u32 = 0;
            while (1) << log_size < (*options).dict_size {
                log_size = log_size.wrapping_add(1);
            }
            (*coder).dist_table_size = log_size.wrapping_mul(2);
            let nice_len: u32 = if mf_get_hash_bytes((*options).mf) > (*options).nice_len {
                mf_get_hash_bytes((*options).mf) as u32
            } else {
                (*options).nice_len
            };
            (*coder).match_len_encoder.table_size = nice_len
                .wrapping_add(1u32)
                .wrapping_sub(MATCH_LEN_MIN as u32);
            (*coder).rep_len_encoder.table_size = nice_len
                .wrapping_add(1u32)
                .wrapping_sub(MATCH_LEN_MIN as u32);
        }
        _ => return LZMA_OPTIONS_ERROR,
    }
    (*coder).is_initialized = !(*options).preset_dict.is_null() && (*options).preset_dict_size > 0;
    (*coder).is_flushed = false;
    (*coder).uncomp_size = 0;
    (*coder).uncomp_size_ptr = core::ptr::null_mut();
    (*coder).out_limit = 0;
    (*coder).use_eopm = id == LZMA_FILTER_LZMA1 as lzma_vli;
    if id == LZMA_FILTER_LZMA1EXT as lzma_vli {
        if (*options).ext_flags & !(LZMA_LZMA1EXT_ALLOW_EOPM as u32) != 0 {
            return LZMA_OPTIONS_ERROR;
        }
        (*coder).use_eopm = (*options).ext_flags & LZMA_LZMA1EXT_ALLOW_EOPM as u32 != 0;
    }
    set_lz_options(lz_options, options);
    return lzma_lzma_encoder_reset(coder, options);
}
unsafe extern "C" fn lzma_encoder_init(
    lz: *mut lzma_lz_encoder,
    allocator: *const lzma_allocator,
    id: lzma_vli,
    options: *const c_void,
    lz_options: *mut lzma_lz_options,
) -> lzma_ret {
    if options.is_null() {
        return LZMA_PROG_ERROR;
    }
    (*lz).code = Some(
        lzma_encode
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
    (*lz).set_out_limit = Some(
        lzma_lzma_set_out_limit as unsafe extern "C" fn(*mut c_void, *mut u64, u64) -> lzma_ret,
    )
        as Option<unsafe extern "C" fn(*mut c_void, *mut u64, u64) -> lzma_ret>;
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
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
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
                    *const c_void,
                    *mut lzma_lz_options,
                ) -> lzma_ret,
        ),
    );
}
#[no_mangle]
pub extern "C" fn lzma_lzma_encoder_memusage(options: *const c_void) -> u64 {
    if !is_options_valid(options as *const lzma_options_lzma) {
        return UINT64_MAX as u64;
    }
    let mut lz_options: lzma_lz_options = lzma_lz_options {
        before_size: 0,
        dict_size: 0,
        after_size: 0,
        match_len_max: 0,
        nice_len: 0,
        match_finder: 0 as lzma_match_finder,
        depth: 0,
        preset_dict: ::core::ptr::null::<u8>(),
        preset_dict_size: 0,
    };
    set_lz_options(&raw mut lz_options, options as *const lzma_options_lzma);
    let lz_memusage: u64 = unsafe { lzma_lz_encoder_memusage(&raw mut lz_options) } as u64;
    if lz_memusage == UINT64_MAX as u64 {
        return UINT64_MAX as u64;
    }
    return (core::mem::size_of::<lzma_lzma1_encoder>() as u64).wrapping_add(lz_memusage);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_lclppb_encode(
    options: *const lzma_options_lzma,
    byte: *mut u8,
) -> bool {
    if !is_lclppb_valid(options) {
        return true;
    }
    *byte = (*options)
        .pb
        .wrapping_mul(5)
        .wrapping_add((*options).lp)
        .wrapping_mul(9)
        .wrapping_add((*options).lc) as u8;
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_props_encode(options: *const c_void, out: *mut u8) -> lzma_ret {
    if options.is_null() {
        return LZMA_PROG_ERROR;
    }
    let opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
    if lzma_lzma_lclppb_encode(opt, out) {
        return LZMA_PROG_ERROR;
    }
    write32le(out.offset(1), (*opt).dict_size);
    return LZMA_OK;
}
#[no_mangle]
pub extern "C" fn lzma_mode_is_supported(mode: lzma_mode) -> lzma_bool {
    return (mode == LZMA_MODE_FAST || mode == LZMA_MODE_NORMAL) as lzma_bool;
}
