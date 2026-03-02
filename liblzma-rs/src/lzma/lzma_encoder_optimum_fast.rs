use crate::types::*;
use core::ffi::c_void;
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
    let buf_avail: u32 =
        if mf_avail(mf).wrapping_add(1) < (2 + ((1 << 3) + (1 << 3) + (1 << 8)) - 1) as u32 {
            (mf_avail(mf) as u32).wrapping_add(1)
        } else {
            (2 + ((1 << 3) + (1 << 3) + (1 << 8)) - 1) as u32
        };
    if buf_avail < 2 {
        *back_res = UINT32_MAX;
        *len_res = 1;
        return;
    }
    let mut rep_len: u32 = 0;
    let mut rep_index: u32 = 0;
    let mut i: u32 = 0;
    while i < REPS {
        let buf_back: *const u8 = buf.offset(-((*coder).reps[i as usize] as isize)).offset(-1);
        if *buf == *buf_back && *buf.offset(1) == *buf_back.offset(1) {
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
        i += 1;
    }
    if len_main >= nice_len {
        *back_res = (*coder).matches[matches_count.wrapping_sub(1) as usize]
            .dist
            .wrapping_add(REPS);
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
            if back_main >> 7 <= (*coder).matches[matches_count.wrapping_sub(2) as usize].dist {
                break;
            }
            matches_count -= 1;
            len_main = (*coder).matches[matches_count.wrapping_sub(1) as usize].len;
            back_main = (*coder).matches[matches_count.wrapping_sub(1) as usize].dist;
        }
        if len_main == 2 && back_main >= 0x80 {
            len_main = 1;
        }
    }
    if rep_len >= 2 {
        if rep_len.wrapping_add(1) >= len_main
            || rep_len.wrapping_add(2) >= len_main && back_main > 1 << 9
            || rep_len.wrapping_add(3) >= len_main && back_main > 1 << 15
        {
            *back_res = rep_index;
            *len_res = rep_len;
            mf_skip(mf, rep_len.wrapping_sub(1));
            return;
        }
    }
    if len_main < 2 || buf_avail <= 2 {
        *back_res = UINT32_MAX;
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
            *back_res = UINT32_MAX;
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
    while i_0 < REPS {
        if memcmp(
            buf as *const c_void,
            buf.offset(-((*coder).reps[i_0 as usize] as isize))
                .offset(-1) as *const c_void,
            limit as size_t,
        ) == 0
        {
            *back_res = UINT32_MAX;
            *len_res = 1;
            return;
        }
        i_0 += 1;
    }
    *back_res = back_main.wrapping_add(REPS);
    *len_res = len_main;
    mf_skip(mf, len_main.wrapping_sub(2));
}
