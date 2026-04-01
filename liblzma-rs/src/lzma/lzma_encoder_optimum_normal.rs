use crate::lzma::lzma_encoder::MATCH_LEN_MAX;
use crate::types::*;
pub const RC_BIT_PRICE_SHIFT_BITS: u32 = 4;
pub const RC_INFINITY_PRICE: c_uint = 1u32 << 30;
#[inline]
unsafe fn rc_bittree_reverse_price(
    probs: *const probability,
    mut bit_levels: u32,
    mut symbol: u32,
) -> u32 {
    let mut price: u32 = 0;
    let mut model_index: u32 = 1;
    loop {
        let bit: u32 = symbol & 1;
        symbol >>= 1;
        price += rc_bit_price(*probs.offset(model_index as isize), bit);
        model_index = (model_index << 1) + bit;
        bit_levels -= 1;
        if bit_levels == 0 {
            break;
        }
    }
    price
}
#[inline]
fn rc_direct_price(bits: u32) -> u32 {
    bits << RC_BIT_PRICE_SHIFT_BITS
}
#[inline]
unsafe fn get_dist_slot_2(dist: u32) -> u32 {
    if dist < 1 << FASTPOS_BITS + (14 / 2 - 1 + 0 * (FASTPOS_BITS - 1)) {
        return (lzma_fastpos[(dist >> 14 / 2 - 1 + 0 * (FASTPOS_BITS - 1)) as usize] as u32)
            + (2 * (14 / 2 - 1 + 0 * (FASTPOS_BITS - 1))) as u32;
    }
    if dist < 1 << FASTPOS_BITS + (14 / 2 - 1 + 1 * (FASTPOS_BITS - 1)) {
        return (lzma_fastpos[(dist >> 14 / 2 - 1 + 1 * (FASTPOS_BITS - 1)) as usize] as u32)
            + (2 * (14 / 2 - 1 + 1 * (FASTPOS_BITS - 1))) as u32;
    }
    (lzma_fastpos[(dist >> 14 / 2 - 1 + 2 * (FASTPOS_BITS - 1)) as usize] as u32)
        + (2 * (14 / 2 - 1 + 2 * (FASTPOS_BITS - 1))) as u32
}
unsafe fn get_literal_price(
    coder: *const lzma_lzma1_encoder,
    pos: u32,
    prev_byte: u32,
    match_mode: bool,
    mut match_byte: u32,
    mut symbol: u32,
) -> u32 {
    let subcoder: *const probability =
        (::core::ptr::addr_of!((*coder).literal) as *const probability).offset(
            (3u32 * (((pos << 8) + prev_byte) & (*coder).literal_mask)
                << (*coder).literal_context_bits) as isize,
        );
    let mut price: u32 = 0;
    if !match_mode {
        price = rc_bittree_price(subcoder, 8, symbol);
    } else {
        let mut offset: u32 = 0x100;
        symbol += 1u32 << 8;
        loop {
            match_byte <<= 1;
            let match_bit: u32 = match_byte & offset;
            let subcoder_index: u32 = offset + match_bit + (symbol >> 8);
            let bit: u32 = symbol >> 7 & 1;
            price += rc_bit_price(*subcoder.offset(subcoder_index as isize), bit);
            symbol <<= 1;
            offset &= !(match_byte ^ symbol);
            if symbol >= 1 << 16 {
                break;
            }
        }
    }
    price
}
#[inline]
fn get_len_price(lencoder: *const lzma_length_encoder, len: u32, pos_state: u32) -> u32 {
    debug_assert!(len >= MATCH_LEN_MIN);
    unsafe {
        let prices = ::core::ptr::addr_of!((*lencoder).prices) as *const [u32; 272];
        let row = prices.add(pos_state as usize) as *const u32;
        *row.add((len - MATCH_LEN_MIN) as usize)
    }
}
#[inline]
fn get_short_rep_price(
    coder: *const lzma_lzma1_encoder,
    state: lzma_lzma_state,
    pos_state: u32,
) -> u32 {
    unsafe {
        rc_bit_0_price(rep0_prob(coder, state))
            + rc_bit_0_price(rep0_long_prob(coder, state, pos_state))
    }
}
#[inline]
fn get_pure_rep_price(
    coder: *const lzma_lzma1_encoder,
    rep_index: u32,
    state: lzma_lzma_state,
    pos_state: u32,
) -> u32 {
    return unsafe {
        let mut price: u32 = 0;
        if rep_index == 0 {
            price = rc_bit_0_price(rep0_prob(coder, state));
            price += rc_bit_1_price(rep0_long_prob(coder, state, pos_state));
        } else {
            price = rc_bit_1_price(rep0_prob(coder, state));
            if rep_index == 1 {
                price += rc_bit_0_price(rep1_prob(coder, state));
            } else {
                debug_assert!(rep_index >= 2);
                price += rc_bit_1_price(rep1_prob(coder, state));
                price += rc_bit_price(rep2_prob(coder, state), rep_index - 2);
            }
        }
        price
    };
}
#[inline]
fn get_rep_price(
    coder: *const lzma_lzma1_encoder,
    rep_index: u32,
    len: u32,
    state: lzma_lzma_state,
    pos_state: u32,
) -> u32 {
    unsafe {
        get_len_price(
            ::core::ptr::addr_of!((*coder).rep_len_encoder),
            len,
            pos_state,
        ) + get_pure_rep_price(coder, rep_index, state, pos_state)
    }
}
#[inline]
fn get_dist_len_price(
    coder: *const lzma_lzma1_encoder,
    dist: u32,
    len: u32,
    pos_state: u32,
) -> u32 {
    return unsafe {
        let dist_state: u32 = if len < (DIST_STATES + MATCH_LEN_MIN) as u32 {
            debug_assert!(len >= MATCH_LEN_MIN);
            len - MATCH_LEN_MIN
        } else {
            (DIST_STATES - 1) as u32
        };
        let mut price: u32 = 0;
        if dist < FULL_DISTANCES {
            price = dist_price(coder, dist_state, dist);
        } else {
            let dist_slot: u32 = get_dist_slot_2(dist) as u32;
            price = dist_slot_price(coder, dist_state, dist_slot)
                + align_price(coder, dist & ALIGN_MASK);
        }
        price += get_len_price(
            ::core::ptr::addr_of!((*coder).match_len_encoder),
            len,
            pos_state,
        );
        price
    };
}
#[inline]
fn is_literal_state(state: lzma_lzma_state) -> bool {
    state < LIT_STATES
}
#[inline]
fn update_literal_state(state: lzma_lzma_state) -> lzma_lzma_state {
    if state <= STATE_SHORTREP_LIT_LIT {
        STATE_LIT_LIT
    } else if state <= STATE_LIT_SHORTREP {
        state - 3
    } else {
        state - 6
    }
}
#[inline]
fn update_match_state(state: lzma_lzma_state) -> lzma_lzma_state {
    if is_literal_state(state) {
        STATE_LIT_MATCH
    } else {
        STATE_NONLIT_MATCH
    }
}
#[inline]
fn update_long_rep_state(state: lzma_lzma_state) -> lzma_lzma_state {
    if is_literal_state(state) {
        STATE_LIT_LONGREP
    } else {
        STATE_NONLIT_REP
    }
}
#[inline]
fn update_short_rep_state(state: lzma_lzma_state) -> lzma_lzma_state {
    if is_literal_state(state) {
        STATE_LIT_SHORTREP
    } else {
        STATE_NONLIT_REP
    }
}
#[inline]
unsafe fn not_equal_16(a: *const u8, b: *const u8) -> bool {
    core::ptr::read_unaligned(a as *const u16) != core::ptr::read_unaligned(b as *const u16)
}
unsafe fn fill_dist_prices(coder: *mut lzma_lzma1_encoder) {
    let mut dist_state: u32 = 0;
    while dist_state < DIST_STATES {
        let dist_slot_prices: *mut u32 =
            ::core::ptr::addr_of_mut!(*(::core::ptr::addr_of_mut!((*coder).dist_slot_prices)
                as *mut [u32; 64])
                .offset(dist_state as isize)) as *mut u32;
        let mut dist_slot: u32 = 0;
        while dist_slot < (*coder).dist_table_size {
            *dist_slot_prices.offset(dist_slot as isize) = rc_bittree_price(
                ::core::ptr::addr_of_mut!(*(::core::ptr::addr_of_mut!((*coder).dist_slot)
                    as *mut [probability; 64])
                    .offset(dist_state as isize)) as *mut probability,
                DIST_SLOT_BITS,
                dist_slot,
            );
            dist_slot += 1;
        }
        let mut dist_slot_0: u32 = DIST_MODEL_END;
        while dist_slot_0 < (*coder).dist_table_size {
            *dist_slot_prices.offset(dist_slot_0 as isize) +=
                rc_direct_price(((dist_slot_0 >> 1) - 1) - ALIGN_BITS);
            dist_slot_0 += 1;
        }
        let mut i: u32 = 0;
        while i < DIST_MODEL_START {
            (*coder).dist_prices[dist_state as usize][i as usize] =
                *dist_slot_prices.offset(i as isize);
            i += 1;
        }
        dist_state += 1;
    }
    let mut i_0: u32 = DIST_MODEL_START;
    while i_0 < FULL_DISTANCES {
        let dist_slot_1: u32 = get_dist_slot(i_0) as u32;
        let footer_bits: u32 = (dist_slot_1 >> 1) - 1;
        let base: u32 = (2 | dist_slot_1 & 1) << footer_bits;
        let price: u32 = rc_bittree_reverse_price(
            (::core::ptr::addr_of_mut!((*coder).dist_special) as *mut probability)
                .offset(base as isize)
                .offset(-(dist_slot_1 as isize))
                .offset(-1),
            footer_bits,
            i_0 - base,
        ) as u32;
        let mut dist_state_0: u32 = 0;
        while dist_state_0 < DIST_STATES {
            (*coder).dist_prices[dist_state_0 as usize][i_0 as usize] =
                price + (*coder).dist_slot_prices[dist_state_0 as usize][dist_slot_1 as usize];
            dist_state_0 += 1;
        }
        i_0 += 1;
    }
    (*coder).match_price_count = 0;
}
unsafe fn fill_align_prices(coder: *mut lzma_lzma1_encoder) {
    let mut i: u32 = 0;
    while i < ALIGN_SIZE {
        (*coder).align_prices[i as usize] = rc_bittree_reverse_price(
            ::core::ptr::addr_of_mut!((*coder).dist_align) as *mut probability,
            ALIGN_BITS,
            i,
        );
        i += 1;
    }
    (*coder).align_price_count = 0;
}
#[inline(always)]
unsafe fn opts_ptr(coder: *mut lzma_lzma1_encoder) -> *mut lzma_optimal {
    ::core::ptr::addr_of_mut!((*coder).opts) as *mut lzma_optimal
}
#[inline(always)]
unsafe fn matches_ptr(coder: *mut lzma_lzma1_encoder) -> *mut lzma_match {
    ::core::ptr::addr_of_mut!((*coder).matches) as *mut lzma_match
}
#[inline(always)]
unsafe fn optimal_backs_ptr(optimal: *mut lzma_optimal) -> *mut u32 {
    ::core::ptr::addr_of_mut!((*optimal).backs) as *mut u32
}
#[inline(always)]
unsafe fn match_prob(
    coder: *const lzma_lzma1_encoder,
    state: lzma_lzma_state,
    pos_state: u32,
) -> probability {
    let row = (::core::ptr::addr_of!((*coder).is_match) as *const [probability; 16])
        .add(state as usize) as *const probability;
    *row.add(pos_state as usize)
}
#[inline(always)]
unsafe fn rep0_prob(coder: *const lzma_lzma1_encoder, state: lzma_lzma_state) -> probability {
    *((::core::ptr::addr_of!((*coder).is_rep0) as *const probability).add(state as usize))
}
#[inline(always)]
unsafe fn is_rep_prob(coder: *const lzma_lzma1_encoder, state: lzma_lzma_state) -> probability {
    *((::core::ptr::addr_of!((*coder).is_rep) as *const probability).add(state as usize))
}
#[inline(always)]
unsafe fn rep0_long_prob(
    coder: *const lzma_lzma1_encoder,
    state: lzma_lzma_state,
    pos_state: u32,
) -> probability {
    let row = (::core::ptr::addr_of!((*coder).is_rep0_long) as *const [probability; 16])
        .add(state as usize) as *const probability;
    *row.add(pos_state as usize)
}
#[inline(always)]
unsafe fn rep1_prob(coder: *const lzma_lzma1_encoder, state: lzma_lzma_state) -> probability {
    *((::core::ptr::addr_of!((*coder).is_rep1) as *const probability).add(state as usize))
}
#[inline(always)]
unsafe fn rep2_prob(coder: *const lzma_lzma1_encoder, state: lzma_lzma_state) -> probability {
    *((::core::ptr::addr_of!((*coder).is_rep2) as *const probability).add(state as usize))
}
#[inline(always)]
unsafe fn dist_slot_price(
    coder: *const lzma_lzma1_encoder,
    dist_state: u32,
    dist_slot: u32,
) -> u32 {
    let row = (::core::ptr::addr_of!((*coder).dist_slot_prices) as *const [u32; 64])
        .add(dist_state as usize) as *const u32;
    *row.add(dist_slot as usize)
}
#[inline(always)]
unsafe fn dist_price(coder: *const lzma_lzma1_encoder, dist_state: u32, dist: u32) -> u32 {
    let row = (::core::ptr::addr_of!((*coder).dist_prices) as *const [u32; 128])
        .add(dist_state as usize) as *const u32;
    *row.add(dist as usize)
}
#[inline(always)]
unsafe fn align_price(coder: *const lzma_lzma1_encoder, index: u32) -> u32 {
    *((::core::ptr::addr_of!((*coder).align_prices) as *const u32).add(index as usize))
}
#[inline]
unsafe fn make_literal(optimal: *mut lzma_optimal) {
    (*optimal).back_prev = UINT32_MAX;
    (*optimal).prev_1_is_literal = false;
}
#[inline]
unsafe fn make_short_rep(optimal: *mut lzma_optimal) {
    (*optimal).back_prev = 0;
    (*optimal).prev_1_is_literal = false;
}
unsafe fn backward(
    coder: *mut lzma_lzma1_encoder,
    len_res: *mut u32,
    back_res: *mut u32,
    mut cur: u32,
) {
    let opts = opts_ptr(coder);
    (*coder).opts_end_index = cur;
    let mut pos_mem: u32 = (*opts.add(cur as usize)).pos_prev;
    let mut back_mem: u32 = (*opts.add(cur as usize)).back_prev;
    loop {
        if (*opts.add(cur as usize)).prev_1_is_literal {
            make_literal(opts.add(pos_mem as usize));
            debug_assert!(pos_mem > 0);
            let literal_pos_prev = pos_mem - 1;
            (*opts.add(pos_mem as usize)).pos_prev = literal_pos_prev;
            if (*opts.add(cur as usize)).prev_2 {
                (*opts.add(literal_pos_prev as usize)).prev_1_is_literal = false;
                (*opts.add(literal_pos_prev as usize)).pos_prev =
                    (*opts.add(cur as usize)).pos_prev_2;
                (*opts.add(literal_pos_prev as usize)).back_prev =
                    (*opts.add(cur as usize)).back_prev_2;
            }
        }
        let pos_prev: u32 = pos_mem;
        let back_cur: u32 = back_mem;
        back_mem = (*opts.add(pos_prev as usize)).back_prev;
        pos_mem = (*opts.add(pos_prev as usize)).pos_prev;
        (*opts.add(pos_prev as usize)).back_prev = back_cur;
        (*opts.add(pos_prev as usize)).pos_prev = cur;
        cur = pos_prev;
        if cur == 0 {
            break;
        }
    }
    (*coder).opts_current_index = (*opts).pos_prev;
    *len_res = (*opts).pos_prev;
    *back_res = (*opts).back_prev;
}
#[inline]
unsafe fn helper1(
    coder: *mut lzma_lzma1_encoder,
    mf: *mut lzma_mf,
    back_res: *mut u32,
    len_res: *mut u32,
    position: u32,
) -> u32 {
    let opts = opts_ptr(coder);
    let matches = matches_ptr(coder);
    let nice_len: u32 = (*mf).nice_len;
    let mut len_main: u32 = 0;
    let mut matches_count: u32 = 0;
    if (*mf).read_ahead == 0 {
        len_main = lzma_mf_find(mf, ::core::ptr::addr_of_mut!(matches_count), matches);
    } else {
        debug_assert!((*mf).read_ahead == 1);
        len_main = (*coder).longest_match_length;
        matches_count = (*coder).matches_count;
    }
    let buf_avail: u32 = core::cmp::min(mf_avail(mf) + 1, MATCH_LEN_MAX);
    if buf_avail < 2 {
        *back_res = UINT32_MAX;
        *len_res = 1;
        return UINT32_MAX;
    }
    let buf: *const u8 = mf_ptr(mf).offset(-1);
    let mut rep_lens: [u32; 4] = [0; 4];
    let rep_lens_ptr = rep_lens.as_mut_ptr();
    let mut rep_max_index: u32 = 0;
    let mut i: u32 = 0;
    while i < REPS {
        let buf_back: *const u8 = buf.offset(-((*coder).reps[i as usize] as isize)).offset(-1);
        if not_equal_16(buf, buf_back) {
            *rep_lens_ptr.add(i as usize) = 0;
        } else {
            *rep_lens_ptr.add(i as usize) = lzma_memcmplen(buf, buf_back, 2, buf_avail);
            if *rep_lens_ptr.add(i as usize) > *rep_lens_ptr.add(rep_max_index as usize) {
                rep_max_index = i;
            }
        }
        i += 1;
    }
    if *rep_lens_ptr.add(rep_max_index as usize) >= nice_len {
        *back_res = rep_max_index;
        *len_res = *rep_lens_ptr.add(rep_max_index as usize);
        mf_skip(mf, *len_res - 1);
        return UINT32_MAX;
    }
    if len_main >= nice_len {
        debug_assert!(matches_count > 0);
        *back_res = (*matches.add((matches_count - 1) as usize)).dist + REPS;
        *len_res = len_main;
        mf_skip(mf, len_main - 1);
        return UINT32_MAX;
    }
    let current_byte: u8 = *buf;
    let match_byte: u8 = *buf.offset(-((*coder).reps[0] as isize)).offset(-1);
    if len_main < 2 && current_byte != match_byte && *rep_lens_ptr.add(rep_max_index as usize) < 2 {
        *back_res = UINT32_MAX;
        *len_res = 1;
        return UINT32_MAX;
    }
    (*opts).state = (*coder).state;
    let pos_state: u32 = position & (*coder).pos_mask;
    (*opts.add(1)).price = rc_bit_0_price(match_prob(coder, (*coder).state, pos_state))
        + get_literal_price(
            coder,
            position,
            *buf.offset(-1) as u32,
            !(((*coder).state as u32) < LIT_STATES),
            match_byte as u32,
            current_byte as u32,
        );
    make_literal(opts.add(1));
    let match_price: u32 = rc_bit_1_price(match_prob(coder, (*coder).state, pos_state)) as u32;
    let rep_match_price: u32 =
        match_price + rc_bit_1_price(is_rep_prob(coder, (*coder).state)) as u32;
    if match_byte == current_byte {
        let short_rep_price: u32 =
            rep_match_price + get_short_rep_price(coder, (*coder).state, pos_state) as u32;
        if short_rep_price < (*opts.add(1)).price {
            (*opts.add(1)).price = short_rep_price;
            make_short_rep(opts.add(1));
        }
    }
    let len_end: u32 = core::cmp::max(len_main, *rep_lens_ptr.add(rep_max_index as usize));
    if len_end < 2 {
        *back_res = (*opts.add(1)).back_prev;
        *len_res = 1;
        return UINT32_MAX;
    }
    (*opts.add(1)).pos_prev = 0;
    let mut i_0: u32 = 0;
    while i_0 < REPS {
        (*opts).backs[i_0 as usize] = (*coder).reps[i_0 as usize];
        i_0 += 1;
    }
    let mut len: u32 = len_end;
    loop {
        (*opts.add(len as usize)).price = RC_INFINITY_PRICE as u32;
        len -= 1;
        if len < 2 {
            break;
        }
    }
    let mut i_1: u32 = 0;
    while i_1 < REPS {
        let mut rep_len: u32 = *rep_lens_ptr.add(i_1 as usize);
        if rep_len >= 2 {
            let price: u32 =
                rep_match_price + get_pure_rep_price(coder, i_1, (*coder).state, pos_state) as u32;
            loop {
                let cur_and_len_price: u32 = price
                    + get_len_price(
                        ::core::ptr::addr_of_mut!((*coder).rep_len_encoder),
                        rep_len,
                        pos_state,
                    ) as u32;
                if cur_and_len_price < (*opts.add(rep_len as usize)).price {
                    (*opts.add(rep_len as usize)).price = cur_and_len_price;
                    (*opts.add(rep_len as usize)).pos_prev = 0;
                    (*opts.add(rep_len as usize)).back_prev = i_1;
                    (*opts.add(rep_len as usize)).prev_1_is_literal = false;
                }
                rep_len -= 1;
                if rep_len < 2 {
                    break;
                }
            }
        }
        i_1 += 1;
    }
    let normal_match_price: u32 =
        match_price + rc_bit_0_price(is_rep_prob(coder, (*coder).state)) as u32;
    len = if *rep_lens_ptr >= 2 {
        *rep_lens_ptr + 1
    } else {
        2
    };
    if len <= len_main {
        let mut i_2: u32 = 0;
        while len > (*matches.add(i_2 as usize)).len {
            i_2 += 1;
        }
        loop {
            let dist: u32 = (*matches.add(i_2 as usize)).dist;
            let cur_and_len_price_0: u32 =
                normal_match_price + get_dist_len_price(coder, dist, len, pos_state) as u32;
            if cur_and_len_price_0 < (*opts.add(len as usize)).price {
                (*opts.add(len as usize)).price = cur_and_len_price_0;
                (*opts.add(len as usize)).pos_prev = 0;
                (*opts.add(len as usize)).back_prev = dist + REPS;
                (*opts.add(len as usize)).prev_1_is_literal = false;
            }
            if len == (*matches.add(i_2 as usize)).len {
                i_2 += 1;
                if i_2 == matches_count {
                    break;
                }
            }
            len += 1;
        }
    }
    len_end
}
#[inline(never)]
unsafe fn helper2(
    coder: *mut lzma_lzma1_encoder,
    reps: *mut u32,
    buf: *const u8,
    mut len_end: u32,
    position: u32,
    cur: u32,
    nice_len: u32,
    buf_avail_full: u32,
) -> u32 {
    let opts = opts_ptr(coder);
    let matches = matches_ptr(coder);
    let mut matches_count: u32 = (*coder).matches_count;
    let mut new_len: u32 = (*coder).longest_match_length;
    let mut pos_prev: u32 = (*opts.add(cur as usize)).pos_prev;
    let mut state: lzma_lzma_state = STATE_LIT_LIT;
    if (*opts.add(cur as usize)).prev_1_is_literal {
        pos_prev -= 1;
        if (*opts.add(cur as usize)).prev_2 {
            state = (*opts.add((*opts.add(cur as usize)).pos_prev_2 as usize)).state;
            if (*opts.add(cur as usize)).back_prev_2 < REPS {
                state = update_long_rep_state(state);
            } else {
                state = update_match_state(state);
            }
        } else {
            state = (*opts.add(pos_prev as usize)).state;
        }
        state = update_literal_state(state);
    } else {
        state = (*opts.add(pos_prev as usize)).state;
    }
    if pos_prev == cur - 1 {
        if (*opts.add(cur as usize)).back_prev == 0 {
            state = update_short_rep_state(state);
        } else {
            state = update_literal_state(state);
        }
    } else {
        let mut pos: u32 = 0;
        if (*opts.add(cur as usize)).prev_1_is_literal && (*opts.add(cur as usize)).prev_2 {
            pos_prev = (*opts.add(cur as usize)).pos_prev_2;
            pos = (*opts.add(cur as usize)).back_prev_2;
            state = update_long_rep_state(state);
        } else {
            pos = (*opts.add(cur as usize)).back_prev;
            if pos < REPS {
                state = update_long_rep_state(state);
            } else {
                state = update_match_state(state);
            }
        }
        if pos < REPS {
            let prev_backs = optimal_backs_ptr(opts.add(pos_prev as usize));
            *reps = *prev_backs.add(pos as usize);
            let mut i: u32 = 0;
            i = 1;
            while i <= pos {
                *reps.offset(i as isize) = *prev_backs.add((i - 1) as usize);
                i += 1;
            }
            while i < REPS {
                *reps.offset(i as isize) = *prev_backs.add(i as usize);
                i += 1;
            }
        } else {
            *reps = pos - REPS;
            let prev_backs = optimal_backs_ptr(opts.add(pos_prev as usize));
            let mut i_0: u32 = 1;
            while i_0 < REPS {
                *reps.offset(i_0 as isize) = *prev_backs.add((i_0 - 1) as usize);
                i_0 += 1;
            }
        }
    }
    (*opts.add(cur as usize)).state = state;
    let cur_backs = optimal_backs_ptr(opts.add(cur as usize));
    let mut i_1: u32 = 0;
    while i_1 < REPS {
        *cur_backs.add(i_1 as usize) = *reps.offset(i_1 as isize);
        i_1 += 1;
    }
    let cur_price: u32 = (*opts.add(cur as usize)).price;
    let current_byte: u8 = *buf;
    let match_byte: u8 = *buf.offset(-(*reps as isize)).offset(-1);
    let pos_state: u32 = position & (*coder).pos_mask;
    let cur_and_1_price: u32 = cur_price
        + rc_bit_0_price(match_prob(coder, state, pos_state)) as u32
        + get_literal_price(
            coder,
            position,
            *buf.offset(-1) as u32,
            !is_literal_state(state),
            match_byte as u32,
            current_byte as u32,
        ) as u32;
    let next_opt = &mut *opts.add((cur + 1) as usize);
    let mut next_is_literal: bool = false;
    if cur_and_1_price < next_opt.price {
        next_opt.price = cur_and_1_price;
        next_opt.pos_prev = cur;
        make_literal(next_opt);
        next_is_literal = true;
    }
    let match_price: u32 = cur_price + rc_bit_1_price(match_prob(coder, state, pos_state)) as u32;
    let rep_match_price: u32 = match_price + rc_bit_1_price(is_rep_prob(coder, state)) as u32;
    if match_byte == current_byte && !(next_opt.pos_prev < cur && next_opt.back_prev == 0) {
        let short_rep_price: u32 =
            rep_match_price + get_short_rep_price(coder, state, pos_state) as u32;
        if short_rep_price <= next_opt.price {
            next_opt.price = short_rep_price;
            next_opt.pos_prev = cur;
            make_short_rep(next_opt);
            next_is_literal = true;
        }
    }
    if buf_avail_full < 2 {
        return len_end;
    }
    let buf_avail: u32 = if buf_avail_full < nice_len {
        buf_avail_full
    } else {
        nice_len
    };
    if !next_is_literal && match_byte != current_byte {
        let buf_back: *const u8 = buf.offset(-(*reps as isize)).offset(-1);
        let limit: u32 = if buf_avail_full < nice_len + 1 {
            buf_avail_full
        } else {
            nice_len + 1
        };
        let len_test: u32 = (lzma_memcmplen(buf, buf_back, 1, limit) as u32) - 1;
        if len_test >= 2 {
            let state_2: lzma_lzma_state = update_literal_state(state);
            let pos_state_next: u32 = (position + 1) & (*coder).pos_mask;
            let next_rep_match_price: u32 = cur_and_1_price
                + rc_bit_1_price(match_prob(coder, state_2, pos_state_next)) as u32
                + rc_bit_1_price(is_rep_prob(coder, state_2)) as u32;
            let offset: u32 = cur + 1 + len_test;
            while len_end < offset {
                len_end += 1;
                (*opts.add(len_end as usize)).price = RC_INFINITY_PRICE as u32;
            }
            let cur_and_len_price: u32 = next_rep_match_price
                + get_rep_price(coder, 0, len_test, state_2, pos_state_next) as u32;
            let opt = opts.add(offset as usize);
            if cur_and_len_price < (*opt).price {
                (*opt).price = cur_and_len_price;
                (*opt).pos_prev = cur + 1;
                (*opt).back_prev = 0;
                (*opt).prev_1_is_literal = true;
                (*opt).prev_2 = false;
            }
        }
    }
    let mut start_len: u32 = 2;
    let mut rep_index: u32 = 0;
    while rep_index < REPS {
        let buf_back_0: *const u8 = buf
            .offset(-(*reps.offset(rep_index as isize) as isize))
            .offset(-1);
        if !not_equal_16(buf, buf_back_0) {
            let mut len_test_0: u32 = lzma_memcmplen(buf, buf_back_0, 2, buf_avail);
            while len_end < cur + len_test_0 {
                len_end += 1;
                (*opts.add(len_end as usize)).price = RC_INFINITY_PRICE as u32;
            }
            let len_test_temp: u32 = len_test_0;
            let price: u32 =
                rep_match_price + get_pure_rep_price(coder, rep_index, state, pos_state) as u32;
            loop {
                let cur_and_len_price_0: u32 = price
                    + get_len_price(
                        ::core::ptr::addr_of_mut!((*coder).rep_len_encoder),
                        len_test_0,
                        pos_state,
                    ) as u32;
                let opt = opts.add((cur + len_test_0) as usize);
                if cur_and_len_price_0 < (*opt).price {
                    (*opt).price = cur_and_len_price_0;
                    (*opt).pos_prev = cur;
                    (*opt).back_prev = rep_index;
                    (*opt).prev_1_is_literal = false;
                }
                len_test_0 -= 1;
                if len_test_0 < 2 {
                    break;
                }
            }
            len_test_0 = len_test_temp;
            if rep_index == 0 {
                start_len = len_test_0 + 1;
            }
            let mut len_test_2: u32 = len_test_0 + 1;
            let limit_0: u32 = if buf_avail_full < len_test_2 + nice_len {
                buf_avail_full
            } else {
                len_test_2 + nice_len
            };
            if len_test_2 < limit_0 {
                len_test_2 = lzma_memcmplen(buf, buf_back_0, len_test_2, limit_0);
            }
            len_test_2 -= len_test_0 + 1;
            if len_test_2 >= 2 {
                consider_literal_after_rep(
                    coder,
                    opts,
                    ::core::ptr::addr_of_mut!(len_end),
                    buf,
                    buf_back_0,
                    position,
                    cur,
                    len_test_0,
                    len_test_2,
                    price,
                    state,
                    pos_state,
                    rep_index,
                );
            }
        }
        rep_index += 1;
    }
    if new_len > buf_avail {
        new_len = buf_avail;
        matches_count = 0;
        while new_len > (*matches.add(matches_count as usize)).len {
            matches_count += 1;
        }
        (*matches.add(matches_count as usize)).len = new_len;
        matches_count += 1;
    }
    if new_len >= start_len {
        let normal_match_price: u32 =
            match_price + rc_bit_0_price(is_rep_prob(coder, state)) as u32;
        while len_end < cur + new_len {
            len_end += 1;
            (*opts.add(len_end as usize)).price = RC_INFINITY_PRICE as u32;
        }
        let mut i_2: u32 = 0;
        while start_len > (*matches.add(i_2 as usize)).len {
            i_2 += 1;
        }
        let mut len_test_1: u32 = start_len;
        loop {
            let cur_back: u32 = (*matches.add(i_2 as usize)).dist;
            let cur_and_len_price_2: u32 =
                normal_match_price + get_dist_len_price(coder, cur_back, len_test_1, pos_state);
            let opt = opts.add((cur + len_test_1) as usize);
            if cur_and_len_price_2 < (*opt).price {
                (*opt).price = cur_and_len_price_2;
                (*opt).pos_prev = cur;
                (*opt).back_prev = cur_back + REPS;
                (*opt).prev_1_is_literal = false;
            }
            if len_test_1 == (*matches.add(i_2 as usize)).len {
                let buf_back_1: *const u8 = buf.offset(-(cur_back as isize)).offset(-1);
                let mut len_test_2_0: u32 = len_test_1 + 1;
                let limit_1: u32 = if buf_avail_full < len_test_2_0 + nice_len {
                    buf_avail_full
                } else {
                    len_test_2_0 + nice_len
                };
                if len_test_2_0 < limit_1 {
                    len_test_2_0 = lzma_memcmplen(buf, buf_back_1, len_test_2_0, limit_1);
                }
                len_test_2_0 -= len_test_1 + 1;
                if len_test_2_0 >= 2 {
                    consider_literal_after_match(
                        coder,
                        opts,
                        ::core::ptr::addr_of_mut!(len_end),
                        buf,
                        buf_back_1,
                        position,
                        cur,
                        len_test_1,
                        len_test_2_0,
                        cur_and_len_price_2,
                        state,
                        cur_back,
                    );
                }
                i_2 += 1;
                if i_2 == matches_count {
                    break;
                }
            }
            len_test_1 += 1;
        }
    }
    len_end
}
#[cold]
#[inline(never)]
unsafe fn consider_literal_after_rep(
    coder: *mut lzma_lzma1_encoder,
    opts: *mut lzma_optimal,
    len_end: *mut u32,
    buf: *const u8,
    buf_back: *const u8,
    position: u32,
    cur: u32,
    len_test: u32,
    len_test_2: u32,
    price: u32,
    state: lzma_lzma_state,
    pos_state: u32,
    rep_index: u32,
) {
    let mut state_2: lzma_lzma_state = state;
    state_2 = update_long_rep_state(state_2);
    let mut pos_state_next: u32 = (position + len_test) & (*coder).pos_mask;
    let cur_and_len_literal_price: u32 = price
        + get_len_price(
            ::core::ptr::addr_of_mut!((*coder).rep_len_encoder),
            len_test,
            pos_state,
        ) as u32
        + rc_bit_0_price(match_prob(coder, state_2, pos_state_next)) as u32
        + get_literal_price(
            coder,
            position + len_test,
            *buf.offset((len_test - 1) as isize) as u32,
            true,
            *buf_back.offset(len_test as isize) as u32,
            *buf.offset(len_test as isize) as u32,
        ) as u32;
    state_2 = update_literal_state(state_2);
    pos_state_next = (position + len_test + 1) & (*coder).pos_mask;
    let next_rep_match_price: u32 = cur_and_len_literal_price
        + rc_bit_1_price(match_prob(coder, state_2, pos_state_next)) as u32
        + rc_bit_1_price(is_rep_prob(coder, state_2)) as u32;
    let offset: u32 = cur + len_test + 1 + len_test_2;
    while *len_end < offset {
        *len_end += 1;
        (*opts.add(*len_end as usize)).price = RC_INFINITY_PRICE as u32;
    }
    let cur_and_len_price: u32 =
        next_rep_match_price + get_rep_price(coder, 0, len_test_2, state_2, pos_state_next) as u32;
    let opt = opts.add(offset as usize);
    if cur_and_len_price < (*opt).price {
        (*opt).price = cur_and_len_price;
        (*opt).pos_prev = cur + len_test + 1;
        (*opt).back_prev = 0;
        (*opt).prev_1_is_literal = true;
        (*opt).prev_2 = true;
        (*opt).pos_prev_2 = cur;
        (*opt).back_prev_2 = rep_index;
    }
}
#[cold]
#[inline(never)]
unsafe fn consider_literal_after_match(
    coder: *mut lzma_lzma1_encoder,
    opts: *mut lzma_optimal,
    len_end: *mut u32,
    buf: *const u8,
    buf_back: *const u8,
    position: u32,
    cur: u32,
    len_test: u32,
    len_test_2: u32,
    cur_and_len_price: u32,
    state: lzma_lzma_state,
    cur_back: u32,
) {
    let mut state_2: lzma_lzma_state = state;
    state_2 = update_match_state(state_2);
    let mut pos_state_next: u32 = (position + len_test) & (*coder).pos_mask;
    let cur_and_len_literal_price: u32 = cur_and_len_price
        + rc_bit_0_price(match_prob(coder, state_2, pos_state_next)) as u32
        + get_literal_price(
            coder,
            position + len_test,
            *buf.offset((len_test - 1) as isize) as u32,
            true,
            *buf_back.offset(len_test as isize) as u32,
            *buf.offset(len_test as isize) as u32,
        ) as u32;
    state_2 = update_literal_state(state_2);
    pos_state_next = (pos_state_next + 1) & (*coder).pos_mask;
    let next_rep_match_price: u32 = cur_and_len_literal_price
        + rc_bit_1_price(match_prob(coder, state_2, pos_state_next)) as u32
        + rc_bit_1_price(is_rep_prob(coder, state_2)) as u32;
    let offset: u32 = cur + len_test + 1 + len_test_2;
    while *len_end < offset {
        *len_end += 1;
        (*opts.add(*len_end as usize)).price = RC_INFINITY_PRICE as u32;
    }
    let cur_and_len_price =
        next_rep_match_price + get_rep_price(coder, 0, len_test_2, state_2, pos_state_next);
    let opt = opts.add(offset as usize);
    if cur_and_len_price < (*opt).price {
        (*opt).price = cur_and_len_price;
        (*opt).pos_prev = cur + len_test + 1;
        (*opt).back_prev = 0;
        (*opt).prev_1_is_literal = true;
        (*opt).prev_2 = true;
        (*opt).pos_prev_2 = cur;
        (*opt).back_prev_2 = cur_back + REPS;
    }
}
pub unsafe fn lzma_lzma_optimum_normal(
    coder: *mut lzma_lzma1_encoder,
    mf: *mut lzma_mf,
    back_res: *mut u32,
    len_res: *mut u32,
    position: u32,
) {
    let opts = opts_ptr(coder);
    let matches = matches_ptr(coder);
    if (*coder).opts_end_index != (*coder).opts_current_index {
        let opt = opts.add((*coder).opts_current_index as usize);
        *len_res = (*opt).pos_prev - (*coder).opts_current_index;
        *back_res = (*opt).back_prev;
        (*coder).opts_current_index = (*opt).pos_prev;
        return;
    }
    if (*mf).read_ahead == 0 {
        if (*coder).match_price_count >= (1 << 7) as u32 {
            fill_dist_prices(coder);
        }
        if (*coder).align_price_count >= ALIGN_SIZE {
            fill_align_prices(coder);
        }
    }
    let mut len_end: u32 = helper1(coder, mf, back_res, len_res, position);
    if len_end == UINT32_MAX {
        return;
    }
    let mut reps: [u32; 4] = (*coder).reps;
    let mut cur: u32 = 0;
    cur = 1;
    while cur < len_end {
        (*coder).longest_match_length = lzma_mf_find(
            mf,
            ::core::ptr::addr_of_mut!((*coder).matches_count),
            matches,
        );
        if (*coder).longest_match_length >= (*mf).nice_len {
            break;
        }
        len_end = helper2(
            coder,
            ::core::ptr::addr_of_mut!(reps) as *mut u32,
            mf_ptr(mf).offset(-1),
            len_end,
            position + cur,
            cur,
            (*mf).nice_len,
            if mf_avail(mf) + 1 < ((1 << 12) - 1) as u32 - cur {
                mf_avail(mf) + 1
            } else {
                ((1 << 12) - 1) as u32 - cur
            },
        );
        cur += 1;
    }
    backward(coder, len_res, back_res, cur);
}
