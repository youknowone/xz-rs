use crate::types::*;
use core::ffi::c_uint;
extern "C" {
    static lzma_crc32_table: [[u32; 256]; 8];
}
#[inline]
unsafe extern "C" fn mf_ptr(mf: *const lzma_mf) -> *const u8 {
    (*mf).buffer.offset((*mf).read_pos as isize)
}
#[inline]
unsafe extern "C" fn mf_avail(mf: *const lzma_mf) -> u32 {
    (*mf).write_pos.wrapping_sub((*mf).read_pos)
}
pub const HASH_2_SIZE: c_uint = 1u32 << 10;
pub const HASH_3_SIZE: c_uint = 1u32 << 16;
pub const HASH_2_MASK: c_uint = HASH_2_SIZE.wrapping_sub(1);
pub const HASH_3_MASK: c_uint = HASH_3_SIZE.wrapping_sub(1);
pub const FIX_3_HASH_SIZE: c_uint = 1u32 << 10;
pub const FIX_4_HASH_SIZE: c_uint = HASH_2_SIZE.wrapping_add(HASH_3_SIZE);
#[inline(always)]
unsafe extern "C" fn lzma_memcmplen(
    buf1: *const u8,
    buf2: *const u8,
    mut len: u32,
    limit: u32,
) -> u32 {
    while len < limit && *buf1.offset(len as isize) == *buf2.offset(len as isize) {
        len += 1;
    }
    len
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_find(
    mf: *mut lzma_mf,
    count_ptr: *mut u32,
    matches: *mut lzma_match,
) -> u32 {
    let count: u32 = (*mf).find.unwrap()(mf, matches) as u32;
    let mut len_best: u32 = 0;
    if count > 0 {
        len_best = (*matches.offset(count.wrapping_sub(1) as isize)).len;
        if len_best == (*mf).nice_len {
            let mut limit: u32 = mf_avail(mf).wrapping_add(1);
            if limit > (*mf).match_len_max {
                limit = (*mf).match_len_max;
            }
            let p1: *const u8 = mf_ptr(mf).offset(-1);
            let p2: *const u8 = p1
                .offset(-((*matches.offset(count.wrapping_sub(1) as isize)).dist as isize))
                .offset(-1);
            len_best = lzma_memcmplen(p1, p2, len_best, limit);
        }
    }
    *count_ptr = count;
    (*mf).read_ahead = (*mf).read_ahead.wrapping_add(1);
    len_best
}
pub const EMPTY_HASH_VALUE: u32 = 0;
pub const MUST_NORMALIZE_POS: c_uint = UINT32_MAX;
unsafe extern "C" fn normalize(mf: *mut lzma_mf) {
    let subvalue: u32 = MUST_NORMALIZE_POS.wrapping_sub((*mf).cyclic_size);
    let mut i: u32 = 0;
    while i < (*mf).hash_count {
        if *(*mf).hash.offset(i as isize) <= subvalue {
            *(*mf).hash.offset(i as isize) = EMPTY_HASH_VALUE;
        } else {
            *(*mf).hash.offset(i as isize) =
                (*(*mf).hash.offset(i as isize)).wrapping_sub(subvalue);
        }
        i += 1;
    }
    let mut i_0: u32 = 0;
    while i_0 < (*mf).sons_count {
        if *(*mf).son.offset(i_0 as isize) <= subvalue {
            *(*mf).son.offset(i_0 as isize) = EMPTY_HASH_VALUE;
        } else {
            *(*mf).son.offset(i_0 as isize) =
                (*(*mf).son.offset(i_0 as isize)).wrapping_sub(subvalue);
        }
        i_0 += 1;
    }
    (*mf).offset = (*mf).offset.wrapping_sub(subvalue);
}
unsafe extern "C" fn move_pos(mf: *mut lzma_mf) {
    (*mf).cyclic_pos = (*mf).cyclic_pos.wrapping_add(1);
    if (*mf).cyclic_pos == (*mf).cyclic_size {
        (*mf).cyclic_pos = 0;
    }
    (*mf).read_pos = (*mf).read_pos.wrapping_add(1);
    if (*mf).read_pos.wrapping_add((*mf).offset) == 4294967295 {
        normalize(mf);
    }
}
unsafe extern "C" fn move_pending(mf: *mut lzma_mf) {
    (*mf).read_pos = (*mf).read_pos.wrapping_add(1);
    (*mf).pending = (*mf).pending.wrapping_add(1);
}
unsafe extern "C" fn hc_find_func(
    len_limit: u32,
    pos: u32,
    cur: *const u8,
    mut cur_match: u32,
    mut depth: u32,
    son: *mut u32,
    cyclic_pos: u32,
    cyclic_size: u32,
    mut matches: *mut lzma_match,
    mut len_best: u32,
) -> *mut lzma_match {
    *son.offset(cyclic_pos as isize) = cur_match;
    loop {
        let delta: u32 = pos.wrapping_sub(cur_match);
        let old_depth = depth;
        depth = depth.wrapping_sub(1);
        if old_depth == 0 || delta >= cyclic_size {
            return matches;
        }
        let pb: *const u8 = cur.offset(-(delta as isize));
        cur_match = *son.offset(
            cyclic_pos
                .wrapping_sub(delta)
                .wrapping_add(if delta > cyclic_pos { cyclic_size } else { 0 })
                as isize,
        );
        if *pb.offset(len_best as isize) == *cur.offset(len_best as isize)
            && *pb == *cur
        {
            let len: u32 = lzma_memcmplen(pb, cur, 1, len_limit);
            if len_best < len {
                len_best = len;
                (*matches).len = len;
                (*matches).dist = delta.wrapping_sub(1);
                matches = matches.offset(1);
                if len == len_limit {
                    return matches;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_hc3_find(mf: *mut lzma_mf, matches: *mut lzma_match) -> u32 {
    let mut len_limit: u32 = mf_avail(mf);
    if (*mf).nice_len <= len_limit {
        len_limit = (*mf).nice_len;
    } else if len_limit < 3 || false && (*mf).action == LZMA_SYNC_FLUSH {
        move_pending(mf);
        return 0;
    }
    let cur: *const u8 = mf_ptr(mf);
    let pos: u32 = (*mf).read_pos.wrapping_add((*mf).offset);
    let mut matches_count: u32 = 0;
    let temp: u32 = lzma_crc32_table[0][*cur as usize] ^ *cur.offset(1) as u32;
    let hash_2_value: u32 = temp & HASH_2_MASK as u32;
    let hash_value: u32 = (temp ^ (*cur.offset(2) as u32) << 8) & (*mf).hash_mask;
    let delta2: u32 = pos.wrapping_sub(*(*mf).hash.offset(hash_2_value as isize));
    let cur_match: u32 = *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_value) as isize);
    *(*mf).hash.offset(hash_2_value as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_value) as isize) = pos;
    let mut len_best: u32 = 2;
    if delta2 < (*mf).cyclic_size && *cur.offset(-(delta2 as isize)) == *cur {
        len_best = lzma_memcmplen(cur.offset(-(delta2 as isize)), cur, len_best, len_limit);
        (*matches).len = len_best;
        (*matches).dist = delta2.wrapping_sub(1);
        matches_count = 1;
        if len_best == len_limit {
            *(*mf).son.offset((*mf).cyclic_pos as isize) = cur_match;
            move_pos(mf);
            return 1;
        }
    }
    matches_count = hc_find_func(
        len_limit,
        pos,
        cur,
        cur_match,
        (*mf).depth,
        (*mf).son,
        (*mf).cyclic_pos,
        (*mf).cyclic_size,
        matches.offset(matches_count as isize),
        len_best,
    )
    .offset_from(matches) as u32;
    move_pos(mf);
    matches_count
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_hc3_skip(mf: *mut lzma_mf, mut amount: u32) {
    loop {
        if mf_avail(mf) < 3 {
            move_pending(mf);
        } else {
            let cur: *const u8 = mf_ptr(mf);
            let pos: u32 = (*mf).read_pos.wrapping_add((*mf).offset);
            let temp: u32 = lzma_crc32_table[0][*cur as usize] ^ *cur.offset(1) as u32;
            let hash_2_value: u32 = temp & HASH_2_MASK as u32;
            let hash_value: u32 = (temp ^ (*cur.offset(2) as u32) << 8) & (*mf).hash_mask;
            let cur_match: u32 = *(*mf)
                .hash
                .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_value) as isize);
            *(*mf).hash.offset(hash_2_value as isize) = pos;
            *(*mf)
                .hash
                .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_value) as isize) = pos;
            *(*mf).son.offset((*mf).cyclic_pos as isize) = cur_match;
            move_pos(mf);
        }
        amount -= 1;
        if amount == 0 {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_hc4_find(mf: *mut lzma_mf, matches: *mut lzma_match) -> u32 {
    let mut len_limit: u32 = mf_avail(mf);
    if (*mf).nice_len <= len_limit {
        len_limit = (*mf).nice_len;
    } else if len_limit < 4 || false && (*mf).action == LZMA_SYNC_FLUSH {
        move_pending(mf);
        return 0;
    }
    let cur: *const u8 = mf_ptr(mf);
    let pos: u32 = (*mf).read_pos.wrapping_add((*mf).offset);
    let mut matches_count: u32 = 0;
    let temp: u32 = lzma_crc32_table[0][*cur as usize] ^ *cur.offset(1) as u32;
    let hash_2_value: u32 = temp & HASH_2_MASK as u32;
    let hash_3_value: u32 = (temp ^ (*cur.offset(2) as u32) << 8) & HASH_3_MASK as u32;
    let hash_value: u32 =
        (temp ^ (*cur.offset(2) as u32) << 8 ^ lzma_crc32_table[0][*cur.offset(3) as usize] << 5)
            & (*mf).hash_mask;
    let mut delta2: u32 = pos.wrapping_sub(*(*mf).hash.offset(hash_2_value as isize));
    let delta3: u32 = pos.wrapping_sub(
        *(*mf)
            .hash
            .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_3_value) as isize),
    );
    let cur_match: u32 = *(*mf)
        .hash
        .offset((FIX_4_HASH_SIZE as u32).wrapping_add(hash_value) as isize);
    *(*mf).hash.offset(hash_2_value as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_3_value) as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_4_HASH_SIZE as u32).wrapping_add(hash_value) as isize) = pos;
    let mut len_best: u32 = 1;
    if delta2 < (*mf).cyclic_size && *cur.offset(-(delta2 as isize)) == *cur {
        len_best = 2;
        (*matches).len = 2;
        (*matches).dist = delta2.wrapping_sub(1);
        matches_count = 1;
    }
    if delta2 != delta3 && delta3 < (*mf).cyclic_size && *cur.offset(-(delta3 as isize)) == *cur {
        len_best = 3;
        (*matches.offset(matches_count as isize)).dist = delta3.wrapping_sub(1);
        matches_count += 1;
        delta2 = delta3;
    }
    if matches_count != 0 {
        len_best = lzma_memcmplen(cur.offset(-(delta2 as isize)), cur, len_best, len_limit);
        (*matches.offset(matches_count.wrapping_sub(1) as isize)).len = len_best;
        if len_best == len_limit {
            *(*mf).son.offset((*mf).cyclic_pos as isize) = cur_match;
            move_pos(mf);
            return matches_count;
        }
    }
    if len_best < 3 {
        len_best = 3;
    }
    matches_count = hc_find_func(
        len_limit,
        pos,
        cur,
        cur_match,
        (*mf).depth,
        (*mf).son,
        (*mf).cyclic_pos,
        (*mf).cyclic_size,
        matches.offset(matches_count as isize),
        len_best,
    )
    .offset_from(matches) as u32;
    move_pos(mf);
    matches_count
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_hc4_skip(mf: *mut lzma_mf, mut amount: u32) {
    loop {
        if mf_avail(mf) < 4 {
            move_pending(mf);
        } else {
            let cur: *const u8 = mf_ptr(mf);
            let pos: u32 = (*mf).read_pos.wrapping_add((*mf).offset);
            let temp: u32 = lzma_crc32_table[0][*cur as usize] ^ *cur.offset(1) as u32;
            let hash_2_value: u32 = temp & HASH_2_MASK as u32;
            let hash_3_value: u32 = (temp ^ (*cur.offset(2) as u32) << 8) & HASH_3_MASK as u32;
            let hash_value: u32 = (temp
                ^ (*cur.offset(2) as u32) << 8
                ^ lzma_crc32_table[0][*cur.offset(3) as usize] << 5)
                & (*mf).hash_mask;
            let cur_match: u32 = *(*mf)
                .hash
                .offset((FIX_4_HASH_SIZE as u32).wrapping_add(hash_value) as isize);
            *(*mf).hash.offset(hash_2_value as isize) = pos;
            *(*mf)
                .hash
                .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_3_value) as isize) = pos;
            *(*mf)
                .hash
                .offset((FIX_4_HASH_SIZE as u32).wrapping_add(hash_value) as isize) = pos;
            *(*mf).son.offset((*mf).cyclic_pos as isize) = cur_match;
            move_pos(mf);
        }
        amount -= 1;
        if amount == 0 {
            break;
        }
    }
}
unsafe extern "C" fn bt_find_func(
    len_limit: u32,
    pos: u32,
    cur: *const u8,
    mut cur_match: u32,
    mut depth: u32,
    son: *mut u32,
    cyclic_pos: u32,
    cyclic_size: u32,
    mut matches: *mut lzma_match,
    mut len_best: u32,
) -> *mut lzma_match {
    let mut ptr0: *mut u32 = son.offset((cyclic_pos << 1) as isize).offset(1);
    let mut ptr1: *mut u32 = son.offset((cyclic_pos << 1) as isize);
    let mut len0: u32 = 0;
    let mut len1: u32 = 0;
    loop {
        let delta: u32 = pos.wrapping_sub(cur_match);
        let old_depth = depth;
        depth = depth.wrapping_sub(1);
        if old_depth == 0 || delta >= cyclic_size {
            *ptr0 = EMPTY_HASH_VALUE;
            *ptr1 = EMPTY_HASH_VALUE;
            return matches;
        }
        let pair: *mut u32 = son.offset(
            (cyclic_pos
                .wrapping_sub(delta)
                .wrapping_add(if delta > cyclic_pos { cyclic_size } else { 0 })
                << 1) as isize,
        );
        let pb: *const u8 = cur.offset(-(delta as isize));
        let mut len: u32 = if len0 < len1 { len0 } else { len1 };
        if *pb.offset(len as isize) == *cur.offset(len as isize) {
            len = lzma_memcmplen(pb, cur, len.wrapping_add(1), len_limit);
            if len_best < len {
                len_best = len;
                (*matches).len = len;
                (*matches).dist = delta.wrapping_sub(1);
                matches = matches.offset(1);
                if len == len_limit {
                    *ptr1 = *pair;
                    *ptr0 = *pair.offset(1);
                    return matches;
                }
            }
        }
        if (*pb.offset(len as isize)) < *cur.offset(len as isize) {
            *ptr1 = cur_match;
            ptr1 = pair.offset(1);
            cur_match = *ptr1;
            len1 = len;
        } else {
            *ptr0 = cur_match;
            ptr0 = pair;
            cur_match = *ptr0;
            len0 = len;
        }
    }
}
unsafe extern "C" fn bt_skip_func(
    len_limit: u32,
    pos: u32,
    cur: *const u8,
    mut cur_match: u32,
    mut depth: u32,
    son: *mut u32,
    cyclic_pos: u32,
    cyclic_size: u32,
) {
    let mut ptr0: *mut u32 = son.offset((cyclic_pos << 1) as isize).offset(1);
    let mut ptr1: *mut u32 = son.offset((cyclic_pos << 1) as isize);
    let mut len0: u32 = 0;
    let mut len1: u32 = 0;
    loop {
        let delta: u32 = pos.wrapping_sub(cur_match);
        let old_depth = depth;
        depth = depth.wrapping_sub(1);
        if old_depth == 0 || delta >= cyclic_size {
            *ptr0 = EMPTY_HASH_VALUE;
            *ptr1 = EMPTY_HASH_VALUE;
            return;
        }
        let pair: *mut u32 = son.offset(
            (cyclic_pos
                .wrapping_sub(delta)
                .wrapping_add(if delta > cyclic_pos { cyclic_size } else { 0 })
                << 1) as isize,
        );
        let pb: *const u8 = cur.offset(-(delta as isize));
        let mut len: u32 = if len0 < len1 { len0 } else { len1 };
        if *pb.offset(len as isize) == *cur.offset(len as isize) {
            len = lzma_memcmplen(pb, cur, len.wrapping_add(1), len_limit);
            if len == len_limit {
                *ptr1 = *pair;
                *ptr0 = *pair.offset(1);
                return;
            }
        }
        if (*pb.offset(len as isize)) < *cur.offset(len as isize) {
            *ptr1 = cur_match;
            ptr1 = pair.offset(1);
            cur_match = *ptr1;
            len1 = len;
        } else {
            *ptr0 = cur_match;
            ptr0 = pair;
            cur_match = *ptr0;
            len0 = len;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_bt2_find(mf: *mut lzma_mf, matches: *mut lzma_match) -> u32 {
    let mut len_limit: u32 = mf_avail(mf);
    if (*mf).nice_len <= len_limit {
        len_limit = (*mf).nice_len;
    } else if len_limit < 2 || true && (*mf).action == LZMA_SYNC_FLUSH {
        move_pending(mf);
        return 0;
    }
    let cur: *const u8 = mf_ptr(mf);
    let pos: u32 = (*mf).read_pos.wrapping_add((*mf).offset);
    let mut matches_count: u32 = 0;
    let hash_value: u32 = *cur as u32 | (*cur.offset(1) as u32) << 8;
    let cur_match: u32 = *(*mf).hash.offset(hash_value as isize);
    *(*mf).hash.offset(hash_value as isize) = pos;
    matches_count = bt_find_func(
        len_limit,
        pos,
        cur,
        cur_match,
        (*mf).depth,
        (*mf).son,
        (*mf).cyclic_pos,
        (*mf).cyclic_size,
        matches.offset(matches_count as isize),
        1,
    )
    .offset_from(matches) as u32;
    move_pos(mf);
    matches_count
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_bt2_skip(mf: *mut lzma_mf, mut amount: u32) {
    let mut current_block_8: u64;
    loop {
        let mut len_limit: u32 = mf_avail(mf);
        if (*mf).nice_len <= len_limit {
            len_limit = (*mf).nice_len;
            current_block_8 = 11875828834189669668;
        } else if len_limit < 2 || true && (*mf).action == LZMA_SYNC_FLUSH {
            move_pending(mf);
            current_block_8 = 18088007599891946824;
        } else {
            current_block_8 = 11875828834189669668;
        }
        match current_block_8 {
            11875828834189669668 => {
                let cur: *const u8 = mf_ptr(mf);
                let pos: u32 = (*mf).read_pos.wrapping_add((*mf).offset);
                let hash_value: u32 = *cur as u32 | (*cur.offset(1) as u32) << 8;
                let cur_match: u32 = *(*mf).hash.offset(hash_value as isize);
                *(*mf).hash.offset(hash_value as isize) = pos;
                bt_skip_func(
                    len_limit,
                    pos,
                    cur,
                    cur_match,
                    (*mf).depth,
                    (*mf).son,
                    (*mf).cyclic_pos,
                    (*mf).cyclic_size,
                );
                move_pos(mf);
            }
            _ => {}
        }
        amount -= 1;
        if amount == 0 {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_bt3_find(mf: *mut lzma_mf, matches: *mut lzma_match) -> u32 {
    let mut len_limit: u32 = mf_avail(mf);
    if (*mf).nice_len <= len_limit {
        len_limit = (*mf).nice_len;
    } else if len_limit < 3 || true && (*mf).action == LZMA_SYNC_FLUSH {
        move_pending(mf);
        return 0;
    }
    let cur: *const u8 = mf_ptr(mf);
    let pos: u32 = (*mf).read_pos.wrapping_add((*mf).offset);
    let mut matches_count: u32 = 0;
    let temp: u32 = lzma_crc32_table[0][*cur as usize] ^ *cur.offset(1) as u32;
    let hash_2_value: u32 = temp & HASH_2_MASK as u32;
    let hash_value: u32 = (temp ^ (*cur.offset(2) as u32) << 8) & (*mf).hash_mask;
    let delta2: u32 = pos.wrapping_sub(*(*mf).hash.offset(hash_2_value as isize));
    let cur_match: u32 = *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_value) as isize);
    *(*mf).hash.offset(hash_2_value as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_value) as isize) = pos;
    let mut len_best: u32 = 2;
    if delta2 < (*mf).cyclic_size && *cur.offset(-(delta2 as isize)) == *cur {
        len_best = lzma_memcmplen(cur, cur.offset(-(delta2 as isize)), len_best, len_limit);
        (*matches).len = len_best;
        (*matches).dist = delta2.wrapping_sub(1);
        matches_count = 1;
        if len_best == len_limit {
            bt_skip_func(
                len_limit,
                pos,
                cur,
                cur_match,
                (*mf).depth,
                (*mf).son,
                (*mf).cyclic_pos,
                (*mf).cyclic_size,
            );
            move_pos(mf);
            return 1;
        }
    }
    matches_count = bt_find_func(
        len_limit,
        pos,
        cur,
        cur_match,
        (*mf).depth,
        (*mf).son,
        (*mf).cyclic_pos,
        (*mf).cyclic_size,
        matches.offset(matches_count as isize),
        len_best,
    )
    .offset_from(matches) as u32;
    move_pos(mf);
    matches_count
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_bt3_skip(mf: *mut lzma_mf, mut amount: u32) {
    let mut current_block_9: u64;
    loop {
        let mut len_limit: u32 = mf_avail(mf);
        if (*mf).nice_len <= len_limit {
            len_limit = (*mf).nice_len;
            current_block_9 = 11875828834189669668;
        } else if len_limit < 3 || true && (*mf).action == LZMA_SYNC_FLUSH {
            move_pending(mf);
            current_block_9 = 18088007599891946824;
        } else {
            current_block_9 = 11875828834189669668;
        }
        match current_block_9 {
            11875828834189669668 => {
                let cur: *const u8 = mf_ptr(mf);
                let pos: u32 = (*mf).read_pos.wrapping_add((*mf).offset);
                let temp: u32 =
                    lzma_crc32_table[0][*cur as usize] ^ *cur.offset(1) as u32;
                let hash_2_value: u32 = temp & HASH_2_MASK as u32;
                let hash_value: u32 = (temp ^ (*cur.offset(2) as u32) << 8) & (*mf).hash_mask;
                let cur_match: u32 = *(*mf)
                    .hash
                    .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_value) as isize);
                *(*mf).hash.offset(hash_2_value as isize) = pos;
                *(*mf)
                    .hash
                    .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_value) as isize) = pos;
                bt_skip_func(
                    len_limit,
                    pos,
                    cur,
                    cur_match,
                    (*mf).depth,
                    (*mf).son,
                    (*mf).cyclic_pos,
                    (*mf).cyclic_size,
                );
                move_pos(mf);
            }
            _ => {}
        }
        amount -= 1;
        if amount == 0 {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_bt4_find(mf: *mut lzma_mf, matches: *mut lzma_match) -> u32 {
    let mut len_limit: u32 = mf_avail(mf);
    if (*mf).nice_len <= len_limit {
        len_limit = (*mf).nice_len;
    } else if len_limit < 4 || true && (*mf).action == LZMA_SYNC_FLUSH {
        move_pending(mf);
        return 0;
    }
    let cur: *const u8 = mf_ptr(mf);
    let pos: u32 = (*mf).read_pos.wrapping_add((*mf).offset);
    let mut matches_count: u32 = 0;
    let temp: u32 = lzma_crc32_table[0][*cur as usize] ^ *cur.offset(1) as u32;
    let hash_2_value: u32 = temp & HASH_2_MASK as u32;
    let hash_3_value: u32 = (temp ^ (*cur.offset(2) as u32) << 8) & HASH_3_MASK as u32;
    let hash_value: u32 =
        (temp ^ (*cur.offset(2) as u32) << 8 ^ lzma_crc32_table[0][*cur.offset(3) as usize] << 5)
            & (*mf).hash_mask;
    let mut delta2: u32 = pos.wrapping_sub(*(*mf).hash.offset(hash_2_value as isize));
    let delta3: u32 = pos.wrapping_sub(
        *(*mf)
            .hash
            .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_3_value) as isize),
    );
    let cur_match: u32 = *(*mf)
        .hash
        .offset((FIX_4_HASH_SIZE as u32).wrapping_add(hash_value) as isize);
    *(*mf).hash.offset(hash_2_value as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_3_value) as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_4_HASH_SIZE as u32).wrapping_add(hash_value) as isize) = pos;
    let mut len_best: u32 = 1;
    if delta2 < (*mf).cyclic_size && *cur.offset(-(delta2 as isize)) == *cur {
        len_best = 2;
        (*matches).len = 2;
        (*matches).dist = delta2.wrapping_sub(1);
        matches_count = 1;
    }
    if delta2 != delta3 && delta3 < (*mf).cyclic_size && *cur.offset(-(delta3 as isize)) == *cur {
        len_best = 3;
        (*matches.offset(matches_count as isize)).dist = delta3.wrapping_sub(1);
        matches_count += 1;
        delta2 = delta3;
    }
    if matches_count != 0 {
        len_best = lzma_memcmplen(cur, cur.offset(-(delta2 as isize)), len_best, len_limit);
        (*matches.offset(matches_count.wrapping_sub(1) as isize)).len = len_best;
        if len_best == len_limit {
            bt_skip_func(
                len_limit,
                pos,
                cur,
                cur_match,
                (*mf).depth,
                (*mf).son,
                (*mf).cyclic_pos,
                (*mf).cyclic_size,
            );
            move_pos(mf);
            return matches_count;
        }
    }
    if len_best < 3 {
        len_best = 3;
    }
    matches_count = bt_find_func(
        len_limit,
        pos,
        cur,
        cur_match,
        (*mf).depth,
        (*mf).son,
        (*mf).cyclic_pos,
        (*mf).cyclic_size,
        matches.offset(matches_count as isize),
        len_best,
    )
    .offset_from(matches) as u32;
    move_pos(mf);
    matches_count
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_bt4_skip(mf: *mut lzma_mf, mut amount: u32) {
    let mut current_block_10: u64;
    loop {
        let mut len_limit: u32 = mf_avail(mf);
        if (*mf).nice_len <= len_limit {
            len_limit = (*mf).nice_len;
            current_block_10 = 11875828834189669668;
        } else if len_limit < 4 || true && (*mf).action == LZMA_SYNC_FLUSH {
            move_pending(mf);
            current_block_10 = 18088007599891946824;
        } else {
            current_block_10 = 11875828834189669668;
        }
        match current_block_10 {
            11875828834189669668 => {
                let cur: *const u8 = mf_ptr(mf);
                let pos: u32 = (*mf).read_pos.wrapping_add((*mf).offset);
                let temp: u32 =
                    lzma_crc32_table[0][*cur as usize] ^ *cur.offset(1) as u32;
                let hash_2_value: u32 = temp & HASH_2_MASK as u32;
                let hash_3_value: u32 = (temp ^ (*cur.offset(2) as u32) << 8) & HASH_3_MASK as u32;
                let hash_value: u32 = (temp
                    ^ (*cur.offset(2) as u32) << 8
                    ^ lzma_crc32_table[0][*cur.offset(3) as usize] << 5)
                    & (*mf).hash_mask;
                let cur_match: u32 = *(*mf)
                    .hash
                    .offset((FIX_4_HASH_SIZE as u32).wrapping_add(hash_value) as isize);
                *(*mf).hash.offset(hash_2_value as isize) = pos;
                *(*mf)
                    .hash
                    .offset((FIX_3_HASH_SIZE as u32).wrapping_add(hash_3_value) as isize) = pos;
                *(*mf)
                    .hash
                    .offset((FIX_4_HASH_SIZE as u32).wrapping_add(hash_value) as isize) = pos;
                bt_skip_func(
                    len_limit,
                    pos,
                    cur,
                    cur_match,
                    (*mf).depth,
                    (*mf).son,
                    (*mf).cyclic_pos,
                    (*mf).cyclic_size,
                );
                move_pos(mf);
            }
            _ => {}
        }
        amount -= 1;
        if amount == 0 {
            break;
        }
    }
}
