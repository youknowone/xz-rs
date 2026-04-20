use crate::lzma::lzma_encoder::MATCH_LEN_MAX;
use crate::types::*;

#[inline(always)]
unsafe fn not_equal_16(a: *const u8, b: *const u8) -> bool {
    core::ptr::read_unaligned(a as *const u16) != core::ptr::read_unaligned(b as *const u16)
}

#[inline(always)]
fn change_pair(small_dist: u32, big_dist: u32) -> bool {
    (big_dist >> 7) > small_dist
}

#[inline(always)]
unsafe fn coder_match(coder: *const lzma_lzma1_encoder, index: u32) -> *const lzma_match {
    debug_assert!((index as usize) < (*coder).matches.len());
    (::core::ptr::addr_of!((*coder).matches) as *const lzma_match).add(index as usize)
}
#[inline(always)]
unsafe fn coder_rep(coder: *const lzma_lzma1_encoder, index: u32) -> u32 {
    debug_assert!((index as usize) < REPS as usize);
    *((::core::ptr::addr_of!((*coder).reps) as *const u32).add(index as usize))
}

pub unsafe fn lzma_lzma_optimum_fast(
    coder: *mut lzma_lzma1_encoder,
    mf: *mut lzma_mf,
    back_res: *mut u32,
    len_res: *mut u32,
) {
    let mf_find = (*mf).find;
    let mf_skip = (*mf).skip;
    let nice_len: u32 = (*mf).nice_len;
    let mut len_main: u32 = 0;
    let mut matches_count: u32 = 0;
    if (*mf).read_ahead == 0 {
        len_main = lzma_mf_find_raw(
            mf,
            ::core::ptr::addr_of_mut!(matches_count),
            ::core::ptr::addr_of_mut!((*coder).matches) as *mut lzma_match,
            mf_find,
        );
    } else {
        debug_assert!((*mf).read_ahead == 1);
        len_main = (*coder).longest_match_length;
        matches_count = (*coder).matches_count;
    }
    let mut buf: *const u8 = mf_ptr(mf).offset(-1);
    let buf_avail: u32 = core::cmp::min(mf_avail(mf) + 1, MATCH_LEN_MAX);
    if buf_avail < 2 {
        *back_res = UINT32_MAX;
        *len_res = 1;
        return;
    }
    let mut rep_len: u32 = 0;
    let mut rep_index: u32 = 0;
    let mut i: u32 = 0;
    while i < REPS {
        let buf_back: *const u8 = buf.offset(-(coder_rep(coder, i) as isize)).offset(-1);
        if not_equal_16(buf, buf_back) {
            i += 1;
            continue;
        }
        let len: u32 = lzma_memcmplen(buf, buf_back, 2, buf_avail) as u32;
        if len >= nice_len {
            *back_res = i;
            *len_res = len;
            mf_skip_raw(mf, len - 1, mf_skip);
            return;
        }
        if len > rep_len {
            rep_index = i;
            rep_len = len;
        }
        i += 1;
    }
    if len_main >= nice_len {
        *back_res = (*coder_match(coder, matches_count - 1)).dist + REPS;
        *len_res = len_main;
        mf_skip_raw(mf, len_main - 1, mf_skip);
        return;
    }
    let mut back_main: u32 = 0;
    if len_main >= 2 {
        back_main = (*coder_match(coder, matches_count - 1)).dist;
        while matches_count > 1 && len_main == (*coder_match(coder, matches_count - 2)).len + 1 {
            if !change_pair((*coder_match(coder, matches_count - 2)).dist, back_main) {
                break;
            }
            matches_count -= 1;
            len_main = (*coder_match(coder, matches_count - 1)).len;
            back_main = (*coder_match(coder, matches_count - 1)).dist;
        }
        if len_main == 2 && back_main >= 0x80 {
            len_main = 1;
        }
    }
    if rep_len >= 2 {
        if rep_len + 1 >= len_main
            || rep_len + 2 >= len_main && back_main > 1 << 9
            || rep_len + 3 >= len_main && back_main > 1 << 15
        {
            *back_res = rep_index;
            *len_res = rep_len;
            mf_skip_raw(mf, rep_len - 1, mf_skip);
            return;
        }
    }
    if len_main < 2 || buf_avail <= 2 {
        *back_res = UINT32_MAX;
        *len_res = 1;
        return;
    }
    (*coder).longest_match_length = lzma_mf_find_raw(
        mf,
        ::core::ptr::addr_of_mut!((*coder).matches_count),
        ::core::ptr::addr_of_mut!((*coder).matches) as *mut lzma_match,
        mf_find,
    );
    if (*coder).longest_match_length >= 2 {
        let new_dist: u32 = (*coder_match(coder, (*coder).matches_count - 1)).dist;
        if (*coder).longest_match_length >= len_main && new_dist < back_main
            || (*coder).longest_match_length == len_main + 1 && !change_pair(back_main, new_dist)
            || (*coder).longest_match_length > len_main + 1
            || (*coder).longest_match_length + 1 >= len_main
                && len_main >= 3
                && change_pair(new_dist, back_main)
        {
            *back_res = UINT32_MAX;
            *len_res = 1;
            return;
        }
    }
    buf = buf.offset(1);
    let limit: u32 = core::cmp::max(2, len_main - 1);
    let mut i_0: u32 = 0;
    while i_0 < REPS {
        if memcmp(
            buf as *const c_void,
            buf.offset(-(coder_rep(coder, i_0) as isize)).offset(-1) as *const c_void,
            limit as size_t,
        ) == 0
        {
            *back_res = UINT32_MAX;
            *len_res = 1;
            return;
        }
        i_0 += 1;
    }
    *back_res = back_main + REPS;
    *len_res = len_main;
    mf_skip_raw(mf, len_main - 2, mf_skip);
}
