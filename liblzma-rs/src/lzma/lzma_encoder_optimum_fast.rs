use crate::types::*;
use core::ffi::{c_int, c_uint, c_void};
extern "C" {
    fn memcmp(__s1: *const c_void, __s2: *const c_void, __n: size_t) -> c_int;
    fn lzma_mf_find(mf: *mut lzma_mf, count: *mut u32, matches: *mut lzma_match) -> u32;
}
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
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
pub const OPTS: c_int = (1) << 12;
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
#[inline]
unsafe extern "C" fn mf_ptr(mf: *const lzma_mf) -> *const u8 {
    return (*mf).buffer.offset((*mf).read_pos as isize);
}
#[inline]
unsafe extern "C" fn mf_avail(mf: *const lzma_mf) -> u32 {
    return (*mf).write_pos.wrapping_sub((*mf).read_pos);
}
#[inline]
unsafe extern "C" fn mf_skip(mf: *mut lzma_mf, amount: u32) {
    if amount != 0 {
        (*mf).skip.expect("non-null function pointer")(mf, amount);
        (*mf).read_ahead = (*mf).read_ahead.wrapping_add(amount);
    }
}
pub const REPS: c_int = 4;
#[inline(always)]
unsafe extern "C" fn lzma_memcmplen(
    buf1: *const u8,
    buf2: *const u8,
    mut len: u32,
    limit: u32,
) -> u32 {
    while len < limit && *buf1.offset(len as isize) as c_int == *buf2.offset(len as isize) as c_int
    {
        len = len.wrapping_add(1);
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_optimum_fast(
    coder: *mut lzma_lzma1_encoder,
    mf: *mut lzma_mf,
    back_res: *mut u32,
    len_res: *mut u32,
) {
    let nice_len: u32 = (*mf).nice_len;
    let mut len_main: u32 = 0;
    let mut matches_count: u32 = 0;
    if (*mf).read_ahead == 0 {
        len_main = lzma_mf_find(
            mf,
            &raw mut matches_count,
            &raw mut (*coder).matches as *mut lzma_match,
        );
    } else {
        len_main = (*coder).longest_match_length;
        matches_count = (*coder).matches_count;
    }
    let mut buf: *const u8 = mf_ptr(mf).offset(-1);
    let buf_avail: u32 = if mf_avail(mf).wrapping_add(1)
        < (2 as c_int + (((1 as c_int) << 3) + ((1 as c_int) << 3) + ((1 as c_int) << 8))
            - 1 as c_int) as u32
    {
        (mf_avail(mf) as u32).wrapping_add(1)
    } else {
        (2 as c_int + (((1 as c_int) << 3) + ((1 as c_int) << 3) + ((1 as c_int) << 8))
            - 1 as c_int) as u32
    };
    if buf_avail < 2 {
        *back_res = UINT32_MAX as u32;
        *len_res = 1;
        return;
    }
    let mut rep_len: u32 = 0;
    let mut rep_index: u32 = 0;
    let mut i: u32 = 0;
    while i < REPS as u32 {
        let buf_back: *const u8 = buf.offset(-((*coder).reps[i as usize] as isize)).offset(-1);
        if !(*buf.offset(0) as c_int != *buf_back.offset(0) as c_int
            || *buf.offset(1) as c_int != *buf_back.offset(1) as c_int)
        {
            let len: u32 = lzma_memcmplen(buf, buf_back, 2, buf_avail) as u32;
            if len >= nice_len {
                *back_res = i;
                *len_res = len;
                mf_skip(mf, len.wrapping_sub(1));
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
        *back_res = (*coder).matches[matches_count.wrapping_sub(1) as usize]
            .dist
            .wrapping_add(REPS as u32);
        *len_res = len_main;
        mf_skip(mf, len_main.wrapping_sub(1));
        return;
    }
    let mut back_main: u32 = 0;
    if len_main >= 2 {
        back_main = (*coder).matches[matches_count.wrapping_sub(1) as usize].dist;
        while matches_count > 1
            && len_main
                == (*coder).matches[matches_count.wrapping_sub(2) as usize]
                    .len
                    .wrapping_add(1)
        {
            if !(back_main >> 7 > (*coder).matches[matches_count.wrapping_sub(2) as usize].dist) {
                break;
            }
            matches_count = matches_count.wrapping_sub(1);
            len_main = (*coder).matches[matches_count.wrapping_sub(1) as usize].len;
            back_main = (*coder).matches[matches_count.wrapping_sub(1) as usize].dist;
        }
        if len_main == 2 && back_main >= 0x80 as u32 {
            len_main = 1;
        }
    }
    if rep_len >= 2 {
        if rep_len.wrapping_add(1) >= len_main
            || rep_len.wrapping_add(2) >= len_main && back_main > (1) << 9
            || rep_len.wrapping_add(3) >= len_main && back_main > (1) << 15
        {
            *back_res = rep_index;
            *len_res = rep_len;
            mf_skip(mf, rep_len.wrapping_sub(1));
            return;
        }
    }
    if len_main < 2 || buf_avail <= 2 {
        *back_res = UINT32_MAX as u32;
        *len_res = 1;
        return;
    }
    (*coder).longest_match_length = lzma_mf_find(
        mf,
        &raw mut (*coder).matches_count,
        &raw mut (*coder).matches as *mut lzma_match,
    );
    if (*coder).longest_match_length >= 2 {
        let new_dist: u32 = (*coder).matches[(*coder).matches_count.wrapping_sub(1) as usize].dist;
        if (*coder).longest_match_length >= len_main && new_dist < back_main
            || (*coder).longest_match_length == len_main.wrapping_add(1)
                && !(new_dist >> 7 > back_main)
            || (*coder).longest_match_length > len_main.wrapping_add(1)
            || (*coder).longest_match_length.wrapping_add(1) >= len_main
                && len_main >= 3
                && back_main >> 7 > new_dist
        {
            *back_res = UINT32_MAX as u32;
            *len_res = 1;
            return;
        }
    }
    buf = buf.offset(1);
    let limit: u32 = if 2 > len_main.wrapping_sub(1) {
        2
    } else {
        len_main.wrapping_sub(1)
    };
    let mut i_0: u32 = 0;
    while i_0 < REPS as u32 {
        if memcmp(
            buf as *const c_void,
            buf.offset(-((*coder).reps[i_0 as usize] as isize))
                .offset(-1) as *const c_void,
            limit as size_t,
        ) == 0 as c_int
        {
            *back_res = UINT32_MAX as u32;
            *len_res = 1;
            return;
        }
        i_0 = i_0.wrapping_add(1);
    }
    *back_res = back_main.wrapping_add(REPS as u32);
    *len_res = len_main;
    mf_skip(mf, len_main.wrapping_sub(2));
}
