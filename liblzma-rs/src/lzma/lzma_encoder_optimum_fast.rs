extern "C" {
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn lzma_mf_find(mf: *mut lzma_mf, count: *mut uint32_t, matches: *mut lzma_match) -> uint32_t;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type lzma_action = ::core::ffi::c_uint;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
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
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn mf_ptr(mut mf: *const lzma_mf) -> *const uint8_t {
    return (*mf).buffer.offset((*mf).read_pos as isize);
}
#[inline]
unsafe extern "C" fn mf_avail(mut mf: *const lzma_mf) -> uint32_t {
    return (*mf).write_pos.wrapping_sub((*mf).read_pos);
}
#[inline]
unsafe extern "C" fn mf_skip(mut mf: *mut lzma_mf, mut amount: uint32_t) {
    if amount != 0 as uint32_t {
        (*mf).skip.expect("non-null function pointer")(mf, amount);
        (*mf).read_ahead = (*mf).read_ahead.wrapping_add(amount);
    }
}
pub const REPS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn lzma_memcmplen(
    mut buf1: *const uint8_t,
    mut buf2: *const uint8_t,
    mut len: uint32_t,
    mut limit: uint32_t,
) -> uint32_t {
    while len < limit
        && *buf1.offset(len as isize) as ::core::ffi::c_int
            == *buf2.offset(len as isize) as ::core::ffi::c_int
    {
        len = len.wrapping_add(1);
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_optimum_fast(
    mut coder: *mut lzma_lzma1_encoder,
    mut mf: *mut lzma_mf,
    mut back_res: *mut uint32_t,
    mut len_res: *mut uint32_t,
) {
    let nice_len: uint32_t = (*mf).nice_len;
    let mut len_main: uint32_t = 0;
    let mut matches_count: uint32_t = 0;
    if (*mf).read_ahead == 0 as uint32_t {
        len_main = lzma_mf_find(
            mf,
            &raw mut matches_count,
            &raw mut (*coder).matches as *mut lzma_match,
        );
    } else {
        len_main = (*coder).longest_match_length;
        matches_count = (*coder).matches_count;
    }
    let mut buf: *const uint8_t = mf_ptr(mf).offset(-(1 as ::core::ffi::c_int as isize));
    let buf_avail: uint32_t = if mf_avail(mf).wrapping_add(1 as uint32_t)
        < (2 as ::core::ffi::c_int
            + (((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
            - 1 as ::core::ffi::c_int) as uint32_t
    {
        (mf_avail(mf) as uint32_t).wrapping_add(1 as uint32_t)
    } else {
        (2 as ::core::ffi::c_int
            + (((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                + ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int))
            - 1 as ::core::ffi::c_int) as uint32_t
    };
    if buf_avail < 2 as uint32_t {
        *back_res = UINT32_MAX as uint32_t;
        *len_res = 1 as uint32_t;
        return;
    }
    let mut rep_len: uint32_t = 0 as uint32_t;
    let mut rep_index: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < REPS as uint32_t {
        let buf_back: *const uint8_t = buf
            .offset(-((*coder).reps[i as usize] as isize))
            .offset(-(1 as ::core::ffi::c_int as isize));
        if !(*buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != *buf_back.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            || *buf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != *buf_back.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        {
            let len: uint32_t = lzma_memcmplen(buf, buf_back, 2 as uint32_t, buf_avail) as uint32_t;
            if len >= nice_len {
                *back_res = i;
                *len_res = len;
                mf_skip(mf, len.wrapping_sub(1 as uint32_t));
                return;
            }
            if len > rep_len {
                rep_index = i;
                rep_len = len;
            }
        }
        i = i.wrapping_add(1);
    }
    if len_main >= nice_len {
        *back_res = (*coder).matches[matches_count.wrapping_sub(1 as uint32_t) as usize]
            .dist
            .wrapping_add(REPS as uint32_t);
        *len_res = len_main;
        mf_skip(mf, len_main.wrapping_sub(1 as uint32_t));
        return;
    }
    let mut back_main: uint32_t = 0 as uint32_t;
    if len_main >= 2 as uint32_t {
        back_main = (*coder).matches[matches_count.wrapping_sub(1 as uint32_t) as usize].dist;
        while matches_count > 1 as uint32_t
            && len_main
                == (*coder).matches[matches_count.wrapping_sub(2 as uint32_t) as usize]
                    .len
                    .wrapping_add(1 as uint32_t)
        {
            if !(back_main >> 7 as ::core::ffi::c_int
                > (*coder).matches[matches_count.wrapping_sub(2 as uint32_t) as usize].dist)
            {
                break;
            }
            matches_count = matches_count.wrapping_sub(1);
            len_main = (*coder).matches[matches_count.wrapping_sub(1 as uint32_t) as usize].len;
            back_main = (*coder).matches[matches_count.wrapping_sub(1 as uint32_t) as usize].dist;
        }
        if len_main == 2 as uint32_t && back_main >= 0x80 as uint32_t {
            len_main = 1 as uint32_t;
        }
    }
    if rep_len >= 2 as uint32_t {
        if rep_len.wrapping_add(1 as uint32_t) >= len_main
            || rep_len.wrapping_add(2 as uint32_t) >= len_main
                && back_main > (1 as uint32_t) << 9 as ::core::ffi::c_int
            || rep_len.wrapping_add(3 as uint32_t) >= len_main
                && back_main > (1 as uint32_t) << 15 as ::core::ffi::c_int
        {
            *back_res = rep_index;
            *len_res = rep_len;
            mf_skip(mf, rep_len.wrapping_sub(1 as uint32_t));
            return;
        }
    }
    if len_main < 2 as uint32_t || buf_avail <= 2 as uint32_t {
        *back_res = UINT32_MAX as uint32_t;
        *len_res = 1 as uint32_t;
        return;
    }
    (*coder).longest_match_length = lzma_mf_find(
        mf,
        &raw mut (*coder).matches_count,
        &raw mut (*coder).matches as *mut lzma_match,
    );
    if (*coder).longest_match_length >= 2 as uint32_t {
        let new_dist: uint32_t =
            (*coder).matches[(*coder).matches_count.wrapping_sub(1 as uint32_t) as usize].dist;
        if (*coder).longest_match_length >= len_main && new_dist < back_main
            || (*coder).longest_match_length == len_main.wrapping_add(1 as uint32_t)
                && !(new_dist >> 7 as ::core::ffi::c_int > back_main)
            || (*coder).longest_match_length > len_main.wrapping_add(1 as uint32_t)
            || (*coder).longest_match_length.wrapping_add(1 as uint32_t) >= len_main
                && len_main >= 3 as uint32_t
                && back_main >> 7 as ::core::ffi::c_int > new_dist
        {
            *back_res = UINT32_MAX as uint32_t;
            *len_res = 1 as uint32_t;
            return;
        }
    }
    buf = buf.offset(1);
    let limit: uint32_t = if 2 as uint32_t > len_main.wrapping_sub(1 as uint32_t) {
        2 as uint32_t
    } else {
        len_main.wrapping_sub(1 as uint32_t)
    };
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < REPS as uint32_t {
        if memcmp(
            buf as *const ::core::ffi::c_void,
            buf.offset(-((*coder).reps[i_0 as usize] as isize))
                .offset(-(1 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_void,
            limit as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            *back_res = UINT32_MAX as uint32_t;
            *len_res = 1 as uint32_t;
            return;
        }
        i_0 = i_0.wrapping_add(1);
    }
    *back_res = back_main.wrapping_add(REPS as uint32_t);
    *len_res = len_main;
    mf_skip(mf, len_main.wrapping_sub(2 as uint32_t));
}
