extern "C" {
    static lzma_crc32_table: [[uint32_t; 256]; 8];
}
pub type uint8_t = u8;
pub type uint32_t = u32;
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
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mf_ptr(mut mf: *const lzma_mf) -> *const uint8_t {
    return (*mf).buffer.offset((*mf).read_pos as isize);
}
#[inline]
unsafe extern "C" fn mf_avail(mut mf: *const lzma_mf) -> uint32_t {
    return (*mf).write_pos.wrapping_sub((*mf).read_pos);
}
pub const HASH_2_SIZE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << 10 as ::core::ffi::c_int;
pub const HASH_3_SIZE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << 16 as ::core::ffi::c_int;
pub const HASH_2_MASK: ::core::ffi::c_uint = HASH_2_SIZE.wrapping_sub(1 as ::core::ffi::c_uint);
pub const HASH_3_MASK: ::core::ffi::c_uint = HASH_3_SIZE.wrapping_sub(1 as ::core::ffi::c_uint);
pub const FIX_3_HASH_SIZE: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 10 as ::core::ffi::c_int;
pub const FIX_4_HASH_SIZE: ::core::ffi::c_uint = HASH_2_SIZE.wrapping_add(HASH_3_SIZE);
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
pub unsafe extern "C" fn lzma_mf_find(
    mut mf: *mut lzma_mf,
    mut count_ptr: *mut uint32_t,
    mut matches: *mut lzma_match,
) -> uint32_t {
    let count: uint32_t = (*mf).find.expect("non-null function pointer")(mf, matches) as uint32_t;
    let mut len_best: uint32_t = 0 as uint32_t;
    if count > 0 as uint32_t {
        len_best = (*matches.offset(count.wrapping_sub(1 as uint32_t) as isize)).len;
        if len_best == (*mf).nice_len {
            let mut limit: uint32_t = mf_avail(mf).wrapping_add(1 as uint32_t);
            if limit > (*mf).match_len_max {
                limit = (*mf).match_len_max;
            }
            let mut p1: *const uint8_t = mf_ptr(mf).offset(-(1 as ::core::ffi::c_int as isize));
            let mut p2: *const uint8_t = p1
                .offset(
                    -((*matches.offset(count.wrapping_sub(1 as uint32_t) as isize)).dist as isize),
                )
                .offset(-(1 as ::core::ffi::c_int as isize));
            len_best = lzma_memcmplen(p1, p2, len_best, limit);
        }
    }
    *count_ptr = count;
    (*mf).read_ahead = (*mf).read_ahead.wrapping_add(1);
    return len_best;
}
pub const EMPTY_HASH_VALUE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MUST_NORMALIZE_POS: ::core::ffi::c_uint = UINT32_MAX;
unsafe extern "C" fn normalize(mut mf: *mut lzma_mf) {
    let subvalue: uint32_t = (MUST_NORMALIZE_POS as uint32_t).wrapping_sub((*mf).cyclic_size);
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*mf).hash_count {
        if *(*mf).hash.offset(i as isize) <= subvalue {
            *(*mf).hash.offset(i as isize) = EMPTY_HASH_VALUE as uint32_t;
        } else {
            let ref mut fresh0 = *(*mf).hash.offset(i as isize);
            *fresh0 = (*fresh0).wrapping_sub(subvalue);
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < (*mf).sons_count {
        if *(*mf).son.offset(i_0 as isize) <= subvalue {
            *(*mf).son.offset(i_0 as isize) = EMPTY_HASH_VALUE as uint32_t;
        } else {
            let ref mut fresh1 = *(*mf).son.offset(i_0 as isize);
            *fresh1 = (*fresh1).wrapping_sub(subvalue);
        }
        i_0 = i_0.wrapping_add(1);
    }
    (*mf).offset = (*mf).offset.wrapping_sub(subvalue);
}
unsafe extern "C" fn move_pos(mut mf: *mut lzma_mf) {
    (*mf).cyclic_pos = (*mf).cyclic_pos.wrapping_add(1);
    if (*mf).cyclic_pos == (*mf).cyclic_size {
        (*mf).cyclic_pos = 0 as uint32_t;
    }
    (*mf).read_pos = (*mf).read_pos.wrapping_add(1);
    if ((*mf).read_pos.wrapping_add((*mf).offset) == 4294967295 as uint32_t) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        normalize(mf);
    }
}
unsafe extern "C" fn move_pending(mut mf: *mut lzma_mf) {
    (*mf).read_pos = (*mf).read_pos.wrapping_add(1);
    (*mf).pending = (*mf).pending.wrapping_add(1);
}
unsafe extern "C" fn hc_find_func(
    len_limit: uint32_t,
    pos: uint32_t,
    cur: *const uint8_t,
    mut cur_match: uint32_t,
    mut depth: uint32_t,
    son: *mut uint32_t,
    cyclic_pos: uint32_t,
    cyclic_size: uint32_t,
    mut matches: *mut lzma_match,
    mut len_best: uint32_t,
) -> *mut lzma_match {
    *son.offset(cyclic_pos as isize) = cur_match;
    loop {
        let delta: uint32_t = pos.wrapping_sub(cur_match);
        let fresh2 = depth;
        depth = depth.wrapping_sub(1);
        if fresh2 == 0 as uint32_t || delta >= cyclic_size {
            return matches;
        }
        let pb: *const uint8_t = cur.offset(-(delta as isize));
        cur_match = *son.offset(cyclic_pos.wrapping_sub(delta).wrapping_add(
            (if delta > cyclic_pos {
                cyclic_size
            } else {
                0 as uint32_t
            }),
        ) as isize);
        if *pb.offset(len_best as isize) as ::core::ffi::c_int
            == *cur.offset(len_best as isize) as ::core::ffi::c_int
            && *pb.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        {
            let mut len: uint32_t = lzma_memcmplen(pb, cur, 1 as uint32_t, len_limit);
            if len_best < len {
                len_best = len;
                (*matches).len = len;
                (*matches).dist = delta.wrapping_sub(1 as uint32_t);
                matches = matches.offset(1);
                if len == len_limit {
                    return matches;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_hc3_find(
    mut mf: *mut lzma_mf,
    mut matches: *mut lzma_match,
) -> uint32_t {
    let mut len_limit: uint32_t = mf_avail(mf);
    if (*mf).nice_len <= len_limit {
        len_limit = (*mf).nice_len;
    } else if len_limit < 3 as uint32_t
        || 0 as ::core::ffi::c_int != 0
            && (*mf).action as ::core::ffi::c_uint
                == LZMA_SYNC_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        move_pending(mf);
        return 0 as uint32_t;
    }
    let mut cur: *const uint8_t = mf_ptr(mf);
    let pos: uint32_t = (*mf).read_pos.wrapping_add((*mf).offset);
    let mut matches_count: uint32_t = 0 as uint32_t;
    let temp: uint32_t = lzma_crc32_table[0 as ::core::ffi::c_int as usize]
        [*cur.offset(0 as ::core::ffi::c_int as isize) as usize]
        ^ *cur.offset(1 as ::core::ffi::c_int as isize) as uint32_t;
    let hash_2_value: uint32_t = temp & HASH_2_MASK as uint32_t;
    let hash_value: uint32_t = (temp
        ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int)
        & (*mf).hash_mask;
    let delta2: uint32_t = pos.wrapping_sub(*(*mf).hash.offset(hash_2_value as isize));
    let cur_match: uint32_t = *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize);
    *(*mf).hash.offset(hash_2_value as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize) = pos;
    let mut len_best: uint32_t = 2 as uint32_t;
    if delta2 < (*mf).cyclic_size
        && *cur.offset(-(delta2 as isize)) as ::core::ffi::c_int == *cur as ::core::ffi::c_int
    {
        len_best = lzma_memcmplen(cur.offset(-(delta2 as isize)), cur, len_best, len_limit);
        (*matches.offset(0 as ::core::ffi::c_int as isize)).len = len_best;
        (*matches.offset(0 as ::core::ffi::c_int as isize)).dist =
            delta2.wrapping_sub(1 as uint32_t);
        matches_count = 1 as uint32_t;
        if len_best == len_limit {
            *(*mf).son.offset((*mf).cyclic_pos as isize) = cur_match;
            move_pos(mf);
            return 1 as uint32_t;
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
    .offset_from(matches) as ::core::ffi::c_long as uint32_t;
    move_pos(mf);
    return matches_count;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_hc3_skip(mut mf: *mut lzma_mf, mut amount: uint32_t) {
    loop {
        if mf_avail(mf) < 3 as uint32_t {
            move_pending(mf);
        } else {
            let mut cur: *const uint8_t = mf_ptr(mf);
            let pos: uint32_t = (*mf).read_pos.wrapping_add((*mf).offset);
            let temp: uint32_t = lzma_crc32_table[0 as ::core::ffi::c_int as usize]
                [*cur.offset(0 as ::core::ffi::c_int as isize) as usize]
                ^ *cur.offset(1 as ::core::ffi::c_int as isize) as uint32_t;
            let hash_2_value: uint32_t = temp & HASH_2_MASK as uint32_t;
            let hash_value: uint32_t = (temp
                ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
                    << 8 as ::core::ffi::c_int)
                & (*mf).hash_mask;
            let cur_match: uint32_t = *(*mf)
                .hash
                .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize);
            *(*mf).hash.offset(hash_2_value as isize) = pos;
            *(*mf)
                .hash
                .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize) = pos;
            *(*mf).son.offset((*mf).cyclic_pos as isize) = cur_match;
            move_pos(mf);
        }
        amount = amount.wrapping_sub(1);
        if !(amount != 0 as uint32_t) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_hc4_find(
    mut mf: *mut lzma_mf,
    mut matches: *mut lzma_match,
) -> uint32_t {
    let mut len_limit: uint32_t = mf_avail(mf);
    if (*mf).nice_len <= len_limit {
        len_limit = (*mf).nice_len;
    } else if len_limit < 4 as uint32_t
        || 0 as ::core::ffi::c_int != 0
            && (*mf).action as ::core::ffi::c_uint
                == LZMA_SYNC_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        move_pending(mf);
        return 0 as uint32_t;
    }
    let mut cur: *const uint8_t = mf_ptr(mf);
    let pos: uint32_t = (*mf).read_pos.wrapping_add((*mf).offset);
    let mut matches_count: uint32_t = 0 as uint32_t;
    let temp: uint32_t = lzma_crc32_table[0 as ::core::ffi::c_int as usize]
        [*cur.offset(0 as ::core::ffi::c_int as isize) as usize]
        ^ *cur.offset(1 as ::core::ffi::c_int as isize) as uint32_t;
    let hash_2_value: uint32_t = temp & HASH_2_MASK as uint32_t;
    let hash_3_value: uint32_t = (temp
        ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int)
        & HASH_3_MASK as uint32_t;
    let hash_value: uint32_t = (temp
        ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        ^ lzma_crc32_table[0 as ::core::ffi::c_int as usize]
            [*cur.offset(3 as ::core::ffi::c_int as isize) as usize]
            << 5 as ::core::ffi::c_int)
        & (*mf).hash_mask;
    let mut delta2: uint32_t = pos.wrapping_sub(*(*mf).hash.offset(hash_2_value as isize));
    let delta3: uint32_t = pos.wrapping_sub(
        *(*mf)
            .hash
            .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_3_value) as isize),
    );
    let cur_match: uint32_t = *(*mf)
        .hash
        .offset((FIX_4_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize);
    *(*mf).hash.offset(hash_2_value as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_3_value) as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_4_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize) = pos;
    let mut len_best: uint32_t = 1 as uint32_t;
    if delta2 < (*mf).cyclic_size
        && *cur.offset(-(delta2 as isize)) as ::core::ffi::c_int == *cur as ::core::ffi::c_int
    {
        len_best = 2 as uint32_t;
        (*matches.offset(0 as ::core::ffi::c_int as isize)).len = 2 as uint32_t;
        (*matches.offset(0 as ::core::ffi::c_int as isize)).dist =
            delta2.wrapping_sub(1 as uint32_t);
        matches_count = 1 as uint32_t;
    }
    if delta2 != delta3
        && delta3 < (*mf).cyclic_size
        && *cur.offset(-(delta3 as isize)) as ::core::ffi::c_int == *cur as ::core::ffi::c_int
    {
        len_best = 3 as uint32_t;
        let fresh3 = matches_count;
        matches_count = matches_count.wrapping_add(1);
        (*matches.offset(fresh3 as isize)).dist = delta3.wrapping_sub(1 as uint32_t);
        delta2 = delta3;
    }
    if matches_count != 0 as uint32_t {
        len_best = lzma_memcmplen(cur.offset(-(delta2 as isize)), cur, len_best, len_limit);
        (*matches.offset(matches_count.wrapping_sub(1 as uint32_t) as isize)).len = len_best;
        if len_best == len_limit {
            *(*mf).son.offset((*mf).cyclic_pos as isize) = cur_match;
            move_pos(mf);
            return matches_count;
        }
    }
    if len_best < 3 as uint32_t {
        len_best = 3 as uint32_t;
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
    .offset_from(matches) as ::core::ffi::c_long as uint32_t;
    move_pos(mf);
    return matches_count;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_hc4_skip(mut mf: *mut lzma_mf, mut amount: uint32_t) {
    loop {
        if mf_avail(mf) < 4 as uint32_t {
            move_pending(mf);
        } else {
            let mut cur: *const uint8_t = mf_ptr(mf);
            let pos: uint32_t = (*mf).read_pos.wrapping_add((*mf).offset);
            let temp: uint32_t = lzma_crc32_table[0 as ::core::ffi::c_int as usize]
                [*cur.offset(0 as ::core::ffi::c_int as isize) as usize]
                ^ *cur.offset(1 as ::core::ffi::c_int as isize) as uint32_t;
            let hash_2_value: uint32_t = temp & HASH_2_MASK as uint32_t;
            let hash_3_value: uint32_t = (temp
                ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
                    << 8 as ::core::ffi::c_int)
                & HASH_3_MASK as uint32_t;
            let hash_value: uint32_t = (temp
                ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
                    << 8 as ::core::ffi::c_int
                ^ lzma_crc32_table[0 as ::core::ffi::c_int as usize]
                    [*cur.offset(3 as ::core::ffi::c_int as isize) as usize]
                    << 5 as ::core::ffi::c_int)
                & (*mf).hash_mask;
            let cur_match: uint32_t = *(*mf)
                .hash
                .offset((FIX_4_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize);
            *(*mf).hash.offset(hash_2_value as isize) = pos;
            *(*mf)
                .hash
                .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_3_value) as isize) = pos;
            *(*mf)
                .hash
                .offset((FIX_4_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize) = pos;
            *(*mf).son.offset((*mf).cyclic_pos as isize) = cur_match;
            move_pos(mf);
        }
        amount = amount.wrapping_sub(1);
        if !(amount != 0 as uint32_t) {
            break;
        }
    }
}
unsafe extern "C" fn bt_find_func(
    len_limit: uint32_t,
    pos: uint32_t,
    cur: *const uint8_t,
    mut cur_match: uint32_t,
    mut depth: uint32_t,
    son: *mut uint32_t,
    cyclic_pos: uint32_t,
    cyclic_size: uint32_t,
    mut matches: *mut lzma_match,
    mut len_best: uint32_t,
) -> *mut lzma_match {
    let mut ptr0: *mut uint32_t = son
        .offset((cyclic_pos << 1 as ::core::ffi::c_int) as isize)
        .offset(1 as ::core::ffi::c_int as isize);
    let mut ptr1: *mut uint32_t = son.offset((cyclic_pos << 1 as ::core::ffi::c_int) as isize);
    let mut len0: uint32_t = 0 as uint32_t;
    let mut len1: uint32_t = 0 as uint32_t;
    loop {
        let delta: uint32_t = pos.wrapping_sub(cur_match);
        let fresh4 = depth;
        depth = depth.wrapping_sub(1);
        if fresh4 == 0 as uint32_t || delta >= cyclic_size {
            *ptr0 = EMPTY_HASH_VALUE as uint32_t;
            *ptr1 = EMPTY_HASH_VALUE as uint32_t;
            return matches;
        }
        let pair: *mut uint32_t = son.offset(
            (cyclic_pos.wrapping_sub(delta).wrapping_add(
                (if delta > cyclic_pos {
                    cyclic_size
                } else {
                    0 as uint32_t
                }),
            ) << 1 as ::core::ffi::c_int) as isize,
        );
        let pb: *const uint8_t = cur.offset(-(delta as isize));
        let mut len: uint32_t = if len0 < len1 { len0 } else { len1 };
        if *pb.offset(len as isize) as ::core::ffi::c_int
            == *cur.offset(len as isize) as ::core::ffi::c_int
        {
            len = lzma_memcmplen(pb, cur, len.wrapping_add(1 as uint32_t), len_limit);
            if len_best < len {
                len_best = len;
                (*matches).len = len;
                (*matches).dist = delta.wrapping_sub(1 as uint32_t);
                matches = matches.offset(1);
                if len == len_limit {
                    *ptr1 = *pair.offset(0 as ::core::ffi::c_int as isize);
                    *ptr0 = *pair.offset(1 as ::core::ffi::c_int as isize);
                    return matches;
                }
            }
        }
        if (*pb.offset(len as isize) as ::core::ffi::c_int)
            < *cur.offset(len as isize) as ::core::ffi::c_int
        {
            *ptr1 = cur_match;
            ptr1 = pair.offset(1 as ::core::ffi::c_int as isize);
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
    len_limit: uint32_t,
    pos: uint32_t,
    cur: *const uint8_t,
    mut cur_match: uint32_t,
    mut depth: uint32_t,
    son: *mut uint32_t,
    cyclic_pos: uint32_t,
    cyclic_size: uint32_t,
) {
    let mut ptr0: *mut uint32_t = son
        .offset((cyclic_pos << 1 as ::core::ffi::c_int) as isize)
        .offset(1 as ::core::ffi::c_int as isize);
    let mut ptr1: *mut uint32_t = son.offset((cyclic_pos << 1 as ::core::ffi::c_int) as isize);
    let mut len0: uint32_t = 0 as uint32_t;
    let mut len1: uint32_t = 0 as uint32_t;
    loop {
        let delta: uint32_t = pos.wrapping_sub(cur_match);
        let fresh5 = depth;
        depth = depth.wrapping_sub(1);
        if fresh5 == 0 as uint32_t || delta >= cyclic_size {
            *ptr0 = EMPTY_HASH_VALUE as uint32_t;
            *ptr1 = EMPTY_HASH_VALUE as uint32_t;
            return;
        }
        let mut pair: *mut uint32_t = son.offset(
            (cyclic_pos.wrapping_sub(delta).wrapping_add(
                (if delta > cyclic_pos {
                    cyclic_size
                } else {
                    0 as uint32_t
                }),
            ) << 1 as ::core::ffi::c_int) as isize,
        );
        let mut pb: *const uint8_t = cur.offset(-(delta as isize));
        let mut len: uint32_t = if len0 < len1 { len0 } else { len1 };
        if *pb.offset(len as isize) as ::core::ffi::c_int
            == *cur.offset(len as isize) as ::core::ffi::c_int
        {
            len = lzma_memcmplen(pb, cur, len.wrapping_add(1 as uint32_t), len_limit);
            if len == len_limit {
                *ptr1 = *pair.offset(0 as ::core::ffi::c_int as isize);
                *ptr0 = *pair.offset(1 as ::core::ffi::c_int as isize);
                return;
            }
        }
        if (*pb.offset(len as isize) as ::core::ffi::c_int)
            < *cur.offset(len as isize) as ::core::ffi::c_int
        {
            *ptr1 = cur_match;
            ptr1 = pair.offset(1 as ::core::ffi::c_int as isize);
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
pub unsafe extern "C" fn lzma_mf_bt2_find(
    mut mf: *mut lzma_mf,
    mut matches: *mut lzma_match,
) -> uint32_t {
    let mut len_limit: uint32_t = mf_avail(mf);
    if (*mf).nice_len <= len_limit {
        len_limit = (*mf).nice_len;
    } else if len_limit < 2 as uint32_t
        || 1 as ::core::ffi::c_int != 0
            && (*mf).action as ::core::ffi::c_uint
                == LZMA_SYNC_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        move_pending(mf);
        return 0 as uint32_t;
    }
    let mut cur: *const uint8_t = mf_ptr(mf);
    let pos: uint32_t = (*mf).read_pos.wrapping_add((*mf).offset);
    let mut matches_count: uint32_t = 0 as uint32_t;
    let hash_value: uint32_t = *cur.offset(0 as ::core::ffi::c_int as isize) as uint32_t
        | (*cur.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int;
    let cur_match: uint32_t = *(*mf).hash.offset(hash_value as isize);
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
        1 as uint32_t,
    )
    .offset_from(matches) as ::core::ffi::c_long as uint32_t;
    move_pos(mf);
    return matches_count;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_bt2_skip(mut mf: *mut lzma_mf, mut amount: uint32_t) {
    let mut current_block_8: u64;
    loop {
        let mut len_limit: uint32_t = mf_avail(mf);
        if (*mf).nice_len <= len_limit {
            len_limit = (*mf).nice_len;
            current_block_8 = 11875828834189669668;
        } else if len_limit < 2 as uint32_t
            || 1 as ::core::ffi::c_int != 0
                && (*mf).action as ::core::ffi::c_uint
                    == LZMA_SYNC_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            move_pending(mf);
            current_block_8 = 18088007599891946824;
        } else {
            current_block_8 = 11875828834189669668;
        }
        match current_block_8 {
            11875828834189669668 => {
                let mut cur: *const uint8_t = mf_ptr(mf);
                let pos: uint32_t = (*mf).read_pos.wrapping_add((*mf).offset);
                let hash_value: uint32_t = *cur.offset(0 as ::core::ffi::c_int as isize)
                    as uint32_t
                    | (*cur.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                        << 8 as ::core::ffi::c_int;
                let cur_match: uint32_t = *(*mf).hash.offset(hash_value as isize);
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
        amount = amount.wrapping_sub(1);
        if !(amount != 0 as uint32_t) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_bt3_find(
    mut mf: *mut lzma_mf,
    mut matches: *mut lzma_match,
) -> uint32_t {
    let mut len_limit: uint32_t = mf_avail(mf);
    if (*mf).nice_len <= len_limit {
        len_limit = (*mf).nice_len;
    } else if len_limit < 3 as uint32_t
        || 1 as ::core::ffi::c_int != 0
            && (*mf).action as ::core::ffi::c_uint
                == LZMA_SYNC_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        move_pending(mf);
        return 0 as uint32_t;
    }
    let mut cur: *const uint8_t = mf_ptr(mf);
    let pos: uint32_t = (*mf).read_pos.wrapping_add((*mf).offset);
    let mut matches_count: uint32_t = 0 as uint32_t;
    let temp: uint32_t = lzma_crc32_table[0 as ::core::ffi::c_int as usize]
        [*cur.offset(0 as ::core::ffi::c_int as isize) as usize]
        ^ *cur.offset(1 as ::core::ffi::c_int as isize) as uint32_t;
    let hash_2_value: uint32_t = temp & HASH_2_MASK as uint32_t;
    let hash_value: uint32_t = (temp
        ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int)
        & (*mf).hash_mask;
    let delta2: uint32_t = pos.wrapping_sub(*(*mf).hash.offset(hash_2_value as isize));
    let cur_match: uint32_t = *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize);
    *(*mf).hash.offset(hash_2_value as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize) = pos;
    let mut len_best: uint32_t = 2 as uint32_t;
    if delta2 < (*mf).cyclic_size
        && *cur.offset(-(delta2 as isize)) as ::core::ffi::c_int == *cur as ::core::ffi::c_int
    {
        len_best = lzma_memcmplen(cur, cur.offset(-(delta2 as isize)), len_best, len_limit);
        (*matches.offset(0 as ::core::ffi::c_int as isize)).len = len_best;
        (*matches.offset(0 as ::core::ffi::c_int as isize)).dist =
            delta2.wrapping_sub(1 as uint32_t);
        matches_count = 1 as uint32_t;
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
            return 1 as uint32_t;
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
    .offset_from(matches) as ::core::ffi::c_long as uint32_t;
    move_pos(mf);
    return matches_count;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_bt3_skip(mut mf: *mut lzma_mf, mut amount: uint32_t) {
    let mut current_block_9: u64;
    loop {
        let mut len_limit: uint32_t = mf_avail(mf);
        if (*mf).nice_len <= len_limit {
            len_limit = (*mf).nice_len;
            current_block_9 = 11875828834189669668;
        } else if len_limit < 3 as uint32_t
            || 1 as ::core::ffi::c_int != 0
                && (*mf).action as ::core::ffi::c_uint
                    == LZMA_SYNC_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            move_pending(mf);
            current_block_9 = 18088007599891946824;
        } else {
            current_block_9 = 11875828834189669668;
        }
        match current_block_9 {
            11875828834189669668 => {
                let mut cur: *const uint8_t = mf_ptr(mf);
                let pos: uint32_t = (*mf).read_pos.wrapping_add((*mf).offset);
                let temp: uint32_t = lzma_crc32_table[0 as ::core::ffi::c_int as usize]
                    [*cur.offset(0 as ::core::ffi::c_int as isize) as usize]
                    ^ *cur.offset(1 as ::core::ffi::c_int as isize) as uint32_t;
                let hash_2_value: uint32_t = temp & HASH_2_MASK as uint32_t;
                let hash_value: uint32_t = (temp
                    ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
                        << 8 as ::core::ffi::c_int)
                    & (*mf).hash_mask;
                let cur_match: uint32_t = *(*mf)
                    .hash
                    .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize);
                *(*mf).hash.offset(hash_2_value as isize) = pos;
                *(*mf)
                    .hash
                    .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize) = pos;
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
        amount = amount.wrapping_sub(1);
        if !(amount != 0 as uint32_t) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_bt4_find(
    mut mf: *mut lzma_mf,
    mut matches: *mut lzma_match,
) -> uint32_t {
    let mut len_limit: uint32_t = mf_avail(mf);
    if (*mf).nice_len <= len_limit {
        len_limit = (*mf).nice_len;
    } else if len_limit < 4 as uint32_t
        || 1 as ::core::ffi::c_int != 0
            && (*mf).action as ::core::ffi::c_uint
                == LZMA_SYNC_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        move_pending(mf);
        return 0 as uint32_t;
    }
    let mut cur: *const uint8_t = mf_ptr(mf);
    let pos: uint32_t = (*mf).read_pos.wrapping_add((*mf).offset);
    let mut matches_count: uint32_t = 0 as uint32_t;
    let temp: uint32_t = lzma_crc32_table[0 as ::core::ffi::c_int as usize]
        [*cur.offset(0 as ::core::ffi::c_int as isize) as usize]
        ^ *cur.offset(1 as ::core::ffi::c_int as isize) as uint32_t;
    let hash_2_value: uint32_t = temp & HASH_2_MASK as uint32_t;
    let hash_3_value: uint32_t = (temp
        ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int)
        & HASH_3_MASK as uint32_t;
    let hash_value: uint32_t = (temp
        ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        ^ lzma_crc32_table[0 as ::core::ffi::c_int as usize]
            [*cur.offset(3 as ::core::ffi::c_int as isize) as usize]
            << 5 as ::core::ffi::c_int)
        & (*mf).hash_mask;
    let mut delta2: uint32_t = pos.wrapping_sub(*(*mf).hash.offset(hash_2_value as isize));
    let delta3: uint32_t = pos.wrapping_sub(
        *(*mf)
            .hash
            .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_3_value) as isize),
    );
    let cur_match: uint32_t = *(*mf)
        .hash
        .offset((FIX_4_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize);
    *(*mf).hash.offset(hash_2_value as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_3_value) as isize) = pos;
    *(*mf)
        .hash
        .offset((FIX_4_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize) = pos;
    let mut len_best: uint32_t = 1 as uint32_t;
    if delta2 < (*mf).cyclic_size
        && *cur.offset(-(delta2 as isize)) as ::core::ffi::c_int == *cur as ::core::ffi::c_int
    {
        len_best = 2 as uint32_t;
        (*matches.offset(0 as ::core::ffi::c_int as isize)).len = 2 as uint32_t;
        (*matches.offset(0 as ::core::ffi::c_int as isize)).dist =
            delta2.wrapping_sub(1 as uint32_t);
        matches_count = 1 as uint32_t;
    }
    if delta2 != delta3
        && delta3 < (*mf).cyclic_size
        && *cur.offset(-(delta3 as isize)) as ::core::ffi::c_int == *cur as ::core::ffi::c_int
    {
        len_best = 3 as uint32_t;
        let fresh6 = matches_count;
        matches_count = matches_count.wrapping_add(1);
        (*matches.offset(fresh6 as isize)).dist = delta3.wrapping_sub(1 as uint32_t);
        delta2 = delta3;
    }
    if matches_count != 0 as uint32_t {
        len_best = lzma_memcmplen(cur, cur.offset(-(delta2 as isize)), len_best, len_limit);
        (*matches.offset(matches_count.wrapping_sub(1 as uint32_t) as isize)).len = len_best;
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
    if len_best < 3 as uint32_t {
        len_best = 3 as uint32_t;
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
    .offset_from(matches) as ::core::ffi::c_long as uint32_t;
    move_pos(mf);
    return matches_count;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_bt4_skip(mut mf: *mut lzma_mf, mut amount: uint32_t) {
    let mut current_block_10: u64;
    loop {
        let mut len_limit: uint32_t = mf_avail(mf);
        if (*mf).nice_len <= len_limit {
            len_limit = (*mf).nice_len;
            current_block_10 = 11875828834189669668;
        } else if len_limit < 4 as uint32_t
            || 1 as ::core::ffi::c_int != 0
                && (*mf).action as ::core::ffi::c_uint
                    == LZMA_SYNC_FLUSH as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            move_pending(mf);
            current_block_10 = 18088007599891946824;
        } else {
            current_block_10 = 11875828834189669668;
        }
        match current_block_10 {
            11875828834189669668 => {
                let mut cur: *const uint8_t = mf_ptr(mf);
                let pos: uint32_t = (*mf).read_pos.wrapping_add((*mf).offset);
                let temp: uint32_t = lzma_crc32_table[0 as ::core::ffi::c_int as usize]
                    [*cur.offset(0 as ::core::ffi::c_int as isize) as usize]
                    ^ *cur.offset(1 as ::core::ffi::c_int as isize) as uint32_t;
                let hash_2_value: uint32_t = temp & HASH_2_MASK as uint32_t;
                let hash_3_value: uint32_t = (temp
                    ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
                        << 8 as ::core::ffi::c_int)
                    & HASH_3_MASK as uint32_t;
                let hash_value: uint32_t = (temp
                    ^ (*cur.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
                        << 8 as ::core::ffi::c_int
                    ^ lzma_crc32_table[0 as ::core::ffi::c_int as usize]
                        [*cur.offset(3 as ::core::ffi::c_int as isize) as usize]
                        << 5 as ::core::ffi::c_int)
                    & (*mf).hash_mask;
                let cur_match: uint32_t = *(*mf)
                    .hash
                    .offset((FIX_4_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize);
                *(*mf).hash.offset(hash_2_value as isize) = pos;
                *(*mf)
                    .hash
                    .offset((FIX_3_HASH_SIZE as uint32_t).wrapping_add(hash_3_value) as isize) =
                    pos;
                *(*mf)
                    .hash
                    .offset((FIX_4_HASH_SIZE as uint32_t).wrapping_add(hash_value) as isize) = pos;
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
        amount = amount.wrapping_sub(1);
        if !(amount != 0 as uint32_t) {
            break;
        }
    }
}
