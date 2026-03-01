use crate::types::*;
use core::ffi::{c_int, c_uint, c_void};
extern "C" {
    fn lzma_mf_find(mf: *mut lzma_mf, count: *mut u32, matches: *mut lzma_match) -> u32;
    static lzma_rc_prices: [u8; 128];
    static lzma_fastpos: [u8; 8192];
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
pub const RC_BIT_MODEL_TOTAL_BITS: c_int = 11;
pub const RC_BIT_MODEL_TOTAL: c_uint = 1u32 << RC_BIT_MODEL_TOTAL_BITS;
pub const RC_MOVE_REDUCING_BITS: c_int = 4;
pub const RC_BIT_PRICE_SHIFT_BITS: c_int = 4;
pub const RC_INFINITY_PRICE: c_uint = 1u32 << 30;
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
unsafe extern "C" fn rc_bittree_reverse_price(
    probs: *const probability,
    mut bit_levels: u32,
    mut symbol: u32,
) -> u32 {
    let mut price: u32 = 0;
    let mut model_index: u32 = 1;
    loop {
        let bit: u32 = symbol & 1;
        symbol >>= 1 as c_int;
        price = price.wrapping_add(rc_bit_price(*probs.offset(model_index as isize), bit));
        model_index = (model_index << 1).wrapping_add(bit);
        bit_levels = bit_levels.wrapping_sub(1);
        if !(bit_levels != 0) {
            break;
        }
    }
    return price;
}
#[inline]
extern "C" fn rc_direct_price(bits: u32) -> u32 {
    return bits << RC_BIT_PRICE_SHIFT_BITS;
}
pub const LIT_STATES: c_int = 7;
pub const MATCH_LEN_MIN: c_int = 2;
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
unsafe extern "C" fn get_dist_slot_2(dist: u32) -> u32 {
    if dist
        < (1)
            << FASTPOS_BITS
                + (14 as c_int / 2 as c_int - 1 as c_int + 0 as c_int * (FASTPOS_BITS - 1 as c_int))
    {
        return (lzma_fastpos[(dist
            >> 14 / 2 as c_int - 1 as c_int + 0 as c_int * (FASTPOS_BITS - 1 as c_int))
            as usize] as u32)
            .wrapping_add(
                (2 as c_int
                    * (14 as c_int / 2 as c_int - 1 as c_int
                        + 0 as c_int * (FASTPOS_BITS - 1 as c_int))) as u32,
            );
    }
    if dist
        < (1)
            << FASTPOS_BITS
                + (14 as c_int / 2 as c_int - 1 as c_int + 1 as c_int * (FASTPOS_BITS - 1 as c_int))
    {
        return (lzma_fastpos[(dist
            >> 14 / 2 as c_int - 1 as c_int + 1 as c_int * (FASTPOS_BITS - 1 as c_int))
            as usize] as u32)
            .wrapping_add(
                (2 as c_int
                    * (14 as c_int / 2 as c_int - 1 as c_int
                        + 1 as c_int * (FASTPOS_BITS - 1 as c_int))) as u32,
            );
    }
    return (lzma_fastpos
        [(dist >> 14 / 2 as c_int - 1 as c_int + 2 as c_int * (FASTPOS_BITS - 1 as c_int)) as usize]
        as u32)
        .wrapping_add(
            (2 as c_int
                * (14 as c_int / 2 as c_int - 1 as c_int
                    + 2 as c_int * (FASTPOS_BITS - 1 as c_int))) as u32,
        );
}
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
unsafe extern "C" fn get_literal_price(
    coder: *const lzma_lzma1_encoder,
    pos: u32,
    prev_byte: u32,
    match_mode: bool,
    mut match_byte: u32,
    mut symbol: u32,
) -> u32 {
    let subcoder: *const probability =
        (&raw const (*coder).literal as *const probability).offset((3u32).wrapping_mul(
            ((pos << 8).wrapping_add(prev_byte) & (*coder).literal_mask)
                << (*coder).literal_context_bits,
        ) as isize);
    let mut price: u32 = 0;
    if !match_mode {
        price = rc_bittree_price(subcoder, 8, symbol);
    } else {
        let mut offset: u32 = 0x100 as u32;
        symbol = (symbol as u32).wrapping_add(1u32 << 8) as u32;
        loop {
            match_byte <<= 1 as c_int;
            let match_bit: u32 = match_byte & offset;
            let subcoder_index: u32 = offset.wrapping_add(match_bit).wrapping_add(symbol >> 8);
            let bit: u32 = symbol >> 7 & 1;
            price =
                price.wrapping_add(rc_bit_price(*subcoder.offset(subcoder_index as isize), bit));
            symbol <<= 1 as c_int;
            offset &= !(match_byte ^ symbol);
            if !(symbol < (1) << 16) {
                break;
            }
        }
    }
    return price;
}
#[inline]
extern "C" fn get_len_price(lencoder: *const lzma_length_encoder, len: u32, pos_state: u32) -> u32 {
    return unsafe {
        (*lencoder).prices[pos_state as usize][len.wrapping_sub(MATCH_LEN_MIN as u32) as usize]
    };
}
#[inline]
extern "C" fn get_short_rep_price(
    coder: *const lzma_lzma1_encoder,
    state: lzma_lzma_state,
    pos_state: u32,
) -> u32 {
    return unsafe {
        rc_bit_0_price((*coder).is_rep0[state as usize]).wrapping_add(rc_bit_0_price(
            (*coder).is_rep0_long[state as usize][pos_state as usize],
        ))
    };
}
#[inline]
extern "C" fn get_pure_rep_price(
    coder: *const lzma_lzma1_encoder,
    rep_index: u32,
    state: lzma_lzma_state,
    pos_state: u32,
) -> u32 {
    return unsafe {
        let mut price: u32 = 0;
        if rep_index == 0 {
            price = rc_bit_0_price((*coder).is_rep0[state as usize]);
            price = price.wrapping_add(rc_bit_1_price(
                (*coder).is_rep0_long[state as usize][pos_state as usize],
            ));
        } else {
            price = rc_bit_1_price((*coder).is_rep0[state as usize]);
            if rep_index == 1 {
                price = price.wrapping_add(rc_bit_0_price((*coder).is_rep1[state as usize]));
            } else {
                price = price.wrapping_add(rc_bit_1_price((*coder).is_rep1[state as usize]));
                price = price.wrapping_add(rc_bit_price(
                    (*coder).is_rep2[state as usize],
                    rep_index.wrapping_sub(2),
                ));
            }
        }
        price
    };
}
#[inline]
extern "C" fn get_rep_price(
    coder: *const lzma_lzma1_encoder,
    rep_index: u32,
    len: u32,
    state: lzma_lzma_state,
    pos_state: u32,
) -> u32 {
    return unsafe {
        get_len_price(&raw const (*coder).rep_len_encoder, len, pos_state)
            .wrapping_add(get_pure_rep_price(coder, rep_index, state, pos_state))
    };
}
#[inline]
extern "C" fn get_dist_len_price(
    coder: *const lzma_lzma1_encoder,
    dist: u32,
    len: u32,
    pos_state: u32,
) -> u32 {
    return unsafe {
        let dist_state: u32 = if len < (DIST_STATES + MATCH_LEN_MIN) as u32 {
            len.wrapping_sub(MATCH_LEN_MIN as u32)
        } else {
            (DIST_STATES - 1 as c_int) as u32
        };
        let mut price: u32 = 0;
        if dist < FULL_DISTANCES as u32 {
            price = (*coder).dist_prices[dist_state as usize][dist as usize];
        } else {
            let dist_slot: u32 = get_dist_slot_2(dist) as u32;
            price = (*coder).dist_slot_prices[dist_state as usize][dist_slot as usize]
                .wrapping_add((*coder).align_prices[(dist & ALIGN_MASK as u32) as usize]);
        }
        price = price.wrapping_add(get_len_price(
            &raw const (*coder).match_len_encoder,
            len,
            pos_state,
        ));
        price
    };
}
unsafe extern "C" fn fill_dist_prices(coder: *mut lzma_lzma1_encoder) {
    let mut dist_state: u32 = 0;
    while dist_state < DIST_STATES as u32 {
        let dist_slot_prices: *mut u32 = &raw mut *(&raw mut (*coder).dist_slot_prices
            as *mut [u32; 64])
            .offset(dist_state as isize) as *mut u32;
        let mut dist_slot: u32 = 0;
        while dist_slot < (*coder).dist_table_size {
            *dist_slot_prices.offset(dist_slot as isize) = rc_bittree_price(
                &raw mut *(&raw mut (*coder).dist_slot as *mut [probability; 64])
                    .offset(dist_state as isize) as *mut probability,
                DIST_SLOT_BITS as u32,
                dist_slot,
            );
            dist_slot = dist_slot.wrapping_add(1);
        }
        let mut dist_slot_0: u32 = DIST_MODEL_END as u32;
        while dist_slot_0 < (*coder).dist_table_size {
            let ref mut fresh1 = *dist_slot_prices.offset(dist_slot_0 as isize);
            *fresh1 = (*fresh1).wrapping_add(rc_direct_price(
                (dist_slot_0 >> 1)
                    .wrapping_sub(1)
                    .wrapping_sub(ALIGN_BITS as u32),
            ));
            dist_slot_0 = dist_slot_0.wrapping_add(1);
        }
        let mut i: u32 = 0;
        while i < DIST_MODEL_START as u32 {
            (*coder).dist_prices[dist_state as usize][i as usize] =
                *dist_slot_prices.offset(i as isize);
            i = i.wrapping_add(1);
        }
        dist_state = dist_state.wrapping_add(1);
    }
    let mut i_0: u32 = DIST_MODEL_START as u32;
    while i_0 < FULL_DISTANCES as u32 {
        let dist_slot_1: u32 = get_dist_slot(i_0) as u32;
        let footer_bits: u32 = (dist_slot_1 >> 1).wrapping_sub(1);
        let base: u32 = (2 | dist_slot_1 & 1) << footer_bits;
        let price: u32 = rc_bittree_reverse_price(
            (&raw mut (*coder).dist_special as *mut probability)
                .offset(base as isize)
                .offset(-(dist_slot_1 as isize))
                .offset(-1),
            footer_bits,
            i_0.wrapping_sub(base),
        ) as u32;
        let mut dist_state_0: u32 = 0;
        while dist_state_0 < DIST_STATES as u32 {
            (*coder).dist_prices[dist_state_0 as usize][i_0 as usize] = price.wrapping_add(
                (*coder).dist_slot_prices[dist_state_0 as usize][dist_slot_1 as usize],
            );
            dist_state_0 = dist_state_0.wrapping_add(1);
        }
        i_0 = i_0.wrapping_add(1);
    }
    (*coder).match_price_count = 0;
}
unsafe extern "C" fn fill_align_prices(coder: *mut lzma_lzma1_encoder) {
    let mut i: u32 = 0;
    while i < ALIGN_SIZE as u32 {
        (*coder).align_prices[i as usize] = rc_bittree_reverse_price(
            &raw mut (*coder).dist_align as *mut probability,
            ALIGN_BITS as u32,
            i,
        );
        i = i.wrapping_add(1);
    }
    (*coder).align_price_count = 0;
}
#[inline]
unsafe extern "C" fn make_literal(optimal: *mut lzma_optimal) {
    (*optimal).back_prev = UINT32_MAX as u32;
    (*optimal).prev_1_is_literal = false;
}
#[inline]
unsafe extern "C" fn make_short_rep(optimal: *mut lzma_optimal) {
    (*optimal).back_prev = 0;
    (*optimal).prev_1_is_literal = false;
}
unsafe extern "C" fn backward(
    coder: *mut lzma_lzma1_encoder,
    len_res: *mut u32,
    back_res: *mut u32,
    mut cur: u32,
) {
    (*coder).opts_end_index = cur;
    let mut pos_mem: u32 = (*coder).opts[cur as usize].pos_prev;
    let mut back_mem: u32 = (*coder).opts[cur as usize].back_prev;
    loop {
        if (*coder).opts[cur as usize].prev_1_is_literal {
            make_literal(
                (&raw mut (*coder).opts as *mut lzma_optimal).offset(pos_mem as isize)
                    as *mut lzma_optimal,
            );
            (*coder).opts[pos_mem as usize].pos_prev = pos_mem.wrapping_sub(1);
            if (*coder).opts[cur as usize].prev_2 {
                (*coder).opts[pos_mem.wrapping_sub(1) as usize].prev_1_is_literal = false;
                (*coder).opts[pos_mem.wrapping_sub(1) as usize].pos_prev =
                    (*coder).opts[cur as usize].pos_prev_2;
                (*coder).opts[pos_mem.wrapping_sub(1) as usize].back_prev =
                    (*coder).opts[cur as usize].back_prev_2;
            }
        }
        let pos_prev: u32 = pos_mem;
        let back_cur: u32 = back_mem;
        back_mem = (*coder).opts[pos_prev as usize].back_prev;
        pos_mem = (*coder).opts[pos_prev as usize].pos_prev;
        (*coder).opts[pos_prev as usize].back_prev = back_cur;
        (*coder).opts[pos_prev as usize].pos_prev = cur;
        cur = pos_prev;
        if !(cur != 0) {
            break;
        }
    }
    (*coder).opts_current_index = (*coder).opts[0].pos_prev;
    *len_res = (*coder).opts[0].pos_prev;
    *back_res = (*coder).opts[0].back_prev;
}
#[inline]
unsafe extern "C" fn helper1(
    coder: *mut lzma_lzma1_encoder,
    mf: *mut lzma_mf,
    back_res: *mut u32,
    len_res: *mut u32,
    position: u32,
) -> u32 {
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
        return UINT32_MAX as u32;
    }
    let buf: *const u8 = mf_ptr(mf).offset(-1);
    let mut rep_lens: [u32; 4] = [0; 4];
    let mut rep_max_index: u32 = 0;
    let mut i: u32 = 0;
    while i < REPS as u32 {
        let buf_back: *const u8 = buf.offset(-((*coder).reps[i as usize] as isize)).offset(-1);
        if *buf.offset(0) as c_int != *buf_back.offset(0) as c_int
            || *buf.offset(1) as c_int != *buf_back.offset(1) as c_int
        {
            rep_lens[i as usize] = 0;
        } else {
            rep_lens[i as usize] = lzma_memcmplen(buf, buf_back, 2, buf_avail);
            if rep_lens[i as usize] > rep_lens[rep_max_index as usize] {
                rep_max_index = i;
            }
        }
        i = i.wrapping_add(1);
    }
    if rep_lens[rep_max_index as usize] >= nice_len {
        *back_res = rep_max_index;
        *len_res = rep_lens[rep_max_index as usize];
        mf_skip(mf, (*len_res).wrapping_sub(1));
        return UINT32_MAX as u32;
    }
    if len_main >= nice_len {
        *back_res = (*coder).matches[matches_count.wrapping_sub(1) as usize]
            .dist
            .wrapping_add(REPS as u32);
        *len_res = len_main;
        mf_skip(mf, len_main.wrapping_sub(1));
        return UINT32_MAX as u32;
    }
    let current_byte: u8 = *buf;
    let match_byte: u8 = *buf.offset(-((*coder).reps[0] as isize)).offset(-1);
    if len_main < 2
        && current_byte as c_int != match_byte as c_int
        && rep_lens[rep_max_index as usize] < 2
    {
        *back_res = UINT32_MAX as u32;
        *len_res = 1;
        return UINT32_MAX as u32;
    }
    (*coder).opts[0].state = (*coder).state;
    let pos_state: u32 = position & (*coder).pos_mask;
    (*coder).opts[1].price =
        rc_bit_0_price((*coder).is_match[(*coder).state as usize][pos_state as usize])
            .wrapping_add(get_literal_price(
                coder,
                position,
                *buf.offset(-(1 as c_int) as isize) as u32,
                !(((*coder).state as u32) < LIT_STATES as u32),
                match_byte as u32,
                current_byte as u32,
            ));
    make_literal((&raw mut (*coder).opts as *mut lzma_optimal).offset(1) as *mut lzma_optimal);
    let match_price: u32 =
        rc_bit_1_price((*coder).is_match[(*coder).state as usize][pos_state as usize]) as u32;
    let rep_match_price: u32 =
        match_price.wrapping_add(rc_bit_1_price((*coder).is_rep[(*coder).state as usize]) as u32);
    if match_byte as c_int == current_byte as c_int {
        let short_rep_price: u32 =
            rep_match_price
                .wrapping_add(get_short_rep_price(coder, (*coder).state, pos_state) as u32);
        if short_rep_price < (*coder).opts[1].price {
            (*coder).opts[1].price = short_rep_price;
            make_short_rep(
                (&raw mut (*coder).opts as *mut lzma_optimal).offset(1) as *mut lzma_optimal
            );
        }
    }
    let len_end: u32 = if len_main > rep_lens[rep_max_index as usize] {
        len_main
    } else {
        rep_lens[rep_max_index as usize]
    };
    if len_end < 2 {
        *back_res = (*coder).opts[1].back_prev;
        *len_res = 1;
        return UINT32_MAX as u32;
    }
    (*coder).opts[1].pos_prev = 0;
    let mut i_0: u32 = 0;
    while i_0 < REPS as u32 {
        (*coder).opts[0].backs[i_0 as usize] = (*coder).reps[i_0 as usize];
        i_0 = i_0.wrapping_add(1);
    }
    let mut len: u32 = len_end;
    loop {
        (*coder).opts[len as usize].price = RC_INFINITY_PRICE as u32;
        len = len.wrapping_sub(1);
        if !(len >= 2) {
            break;
        }
    }
    let mut i_1: u32 = 0;
    while i_1 < REPS as u32 {
        let mut rep_len: u32 = rep_lens[i_1 as usize];
        if !(rep_len < 2) {
            let price: u32 = rep_match_price.wrapping_add(get_pure_rep_price(
                coder,
                i_1,
                (*coder).state,
                pos_state,
            ) as u32);
            loop {
                let cur_and_len_price: u32 = price.wrapping_add(get_len_price(
                    &raw mut (*coder).rep_len_encoder,
                    rep_len,
                    pos_state,
                ) as u32);
                if cur_and_len_price < (*coder).opts[rep_len as usize].price {
                    (*coder).opts[rep_len as usize].price = cur_and_len_price;
                    (*coder).opts[rep_len as usize].pos_prev = 0;
                    (*coder).opts[rep_len as usize].back_prev = i_1;
                    (*coder).opts[rep_len as usize].prev_1_is_literal = false;
                }
                rep_len = rep_len.wrapping_sub(1);
                if !(rep_len >= 2) {
                    break;
                }
            }
        }
        i_1 = i_1.wrapping_add(1);
    }
    let normal_match_price: u32 =
        match_price.wrapping_add(rc_bit_0_price((*coder).is_rep[(*coder).state as usize]) as u32);
    len = if rep_lens[0] >= 2 {
        rep_lens[0].wrapping_add(1)
    } else {
        2
    };
    if len <= len_main {
        let mut i_2: u32 = 0;
        while len > (*coder).matches[i_2 as usize].len {
            i_2 = i_2.wrapping_add(1);
        }
        loop {
            let dist: u32 = (*coder).matches[i_2 as usize].dist;
            let cur_and_len_price_0: u32 = normal_match_price
                .wrapping_add(get_dist_len_price(coder, dist, len, pos_state) as u32);
            if cur_and_len_price_0 < (*coder).opts[len as usize].price {
                (*coder).opts[len as usize].price = cur_and_len_price_0;
                (*coder).opts[len as usize].pos_prev = 0;
                (*coder).opts[len as usize].back_prev = dist.wrapping_add(REPS as u32);
                (*coder).opts[len as usize].prev_1_is_literal = false;
            }
            if len == (*coder).matches[i_2 as usize].len {
                i_2 = i_2.wrapping_add(1);
                if i_2 == matches_count {
                    break;
                }
            }
            len = len.wrapping_add(1);
        }
    }
    return len_end;
}
#[inline]
unsafe extern "C" fn helper2(
    coder: *mut lzma_lzma1_encoder,
    reps: *mut u32,
    buf: *const u8,
    mut len_end: u32,
    position: u32,
    cur: u32,
    nice_len: u32,
    buf_avail_full: u32,
) -> u32 {
    let mut matches_count: u32 = (*coder).matches_count;
    let mut new_len: u32 = (*coder).longest_match_length;
    let mut pos_prev: u32 = (*coder).opts[cur as usize].pos_prev;
    let mut state: lzma_lzma_state = STATE_LIT_LIT;
    if (*coder).opts[cur as usize].prev_1_is_literal {
        pos_prev = pos_prev.wrapping_sub(1);
        if (*coder).opts[cur as usize].prev_2 {
            state = (*coder).opts[(*coder).opts[cur as usize].pos_prev_2 as usize].state;
            if (*coder).opts[cur as usize].back_prev_2 < REPS as u32 {
                state = (if (state as u32) < LIT_STATES as u32 {
                    STATE_LIT_LONGREP as c_int
                } else {
                    STATE_NONLIT_REP as c_int
                }) as lzma_lzma_state;
            } else {
                state = (if (state as u32) < LIT_STATES as u32 {
                    STATE_LIT_MATCH as c_int
                } else {
                    STATE_NONLIT_MATCH as c_int
                }) as lzma_lzma_state;
            }
        } else {
            state = (*coder).opts[pos_prev as usize].state;
        }
        state = (if state <= STATE_SHORTREP_LIT_LIT {
            STATE_LIT_LIT
        } else if state <= STATE_LIT_SHORTREP {
            (state as u32).wrapping_sub(3)
        } else {
            (state as u32).wrapping_sub(6)
        }) as lzma_lzma_state;
    } else {
        state = (*coder).opts[pos_prev as usize].state;
    }
    if pos_prev == cur.wrapping_sub(1) {
        if (*coder).opts[cur as usize].back_prev == 0 {
            state = (if (state as u32) < LIT_STATES as u32 {
                STATE_LIT_SHORTREP as c_int
            } else {
                STATE_NONLIT_REP as c_int
            }) as lzma_lzma_state;
        } else {
            state = (if state <= STATE_SHORTREP_LIT_LIT {
                STATE_LIT_LIT
            } else if state <= STATE_LIT_SHORTREP {
                (state as u32).wrapping_sub(3)
            } else {
                (state as u32).wrapping_sub(6)
            }) as lzma_lzma_state;
        }
    } else {
        let mut pos: u32 = 0;
        if (*coder).opts[cur as usize].prev_1_is_literal && (*coder).opts[cur as usize].prev_2 {
            pos_prev = (*coder).opts[cur as usize].pos_prev_2;
            pos = (*coder).opts[cur as usize].back_prev_2;
            state = (if (state as u32) < LIT_STATES as u32 {
                STATE_LIT_LONGREP as c_int
            } else {
                STATE_NONLIT_REP as c_int
            }) as lzma_lzma_state;
        } else {
            pos = (*coder).opts[cur as usize].back_prev;
            if pos < REPS as u32 {
                state = (if (state as u32) < LIT_STATES as u32 {
                    STATE_LIT_LONGREP as c_int
                } else {
                    STATE_NONLIT_REP as c_int
                }) as lzma_lzma_state;
            } else {
                state = (if (state as u32) < LIT_STATES as u32 {
                    STATE_LIT_MATCH as c_int
                } else {
                    STATE_NONLIT_MATCH as c_int
                }) as lzma_lzma_state;
            }
        }
        if pos < REPS as u32 {
            *reps.offset(0) = (*coder).opts[pos_prev as usize].backs[pos as usize];
            let mut i: u32 = 0;
            i = 1;
            while i <= pos {
                *reps.offset(i as isize) =
                    (*coder).opts[pos_prev as usize].backs[i.wrapping_sub(1) as usize];
                i = i.wrapping_add(1);
            }
            while i < REPS as u32 {
                *reps.offset(i as isize) = (*coder).opts[pos_prev as usize].backs[i as usize];
                i = i.wrapping_add(1);
            }
        } else {
            *reps.offset(0) = pos.wrapping_sub(REPS as u32);
            let mut i_0: u32 = 1;
            while i_0 < REPS as u32 {
                *reps.offset(i_0 as isize) =
                    (*coder).opts[pos_prev as usize].backs[i_0.wrapping_sub(1) as usize];
                i_0 = i_0.wrapping_add(1);
            }
        }
    }
    (*coder).opts[cur as usize].state = state;
    let mut i_1: u32 = 0;
    while i_1 < REPS as u32 {
        (*coder).opts[cur as usize].backs[i_1 as usize] = *reps.offset(i_1 as isize);
        i_1 = i_1.wrapping_add(1);
    }
    let cur_price: u32 = (*coder).opts[cur as usize].price;
    let current_byte: u8 = *buf;
    let match_byte: u8 = *buf.offset(-(*reps.offset(0) as isize)).offset(-1);
    let pos_state: u32 = position & (*coder).pos_mask;
    let cur_and_1_price: u32 = cur_price
        .wrapping_add(rc_bit_0_price((*coder).is_match[state as usize][pos_state as usize]) as u32)
        .wrapping_add(get_literal_price(
            coder,
            position,
            *buf.offset(-(1 as c_int) as isize) as u32,
            !((state as u32) < LIT_STATES as u32),
            match_byte as u32,
            current_byte as u32,
        ) as u32);
    let mut next_is_literal: bool = false;
    if cur_and_1_price < (*coder).opts[cur.wrapping_add(1) as usize].price {
        (*coder).opts[cur.wrapping_add(1) as usize].price = cur_and_1_price;
        (*coder).opts[cur.wrapping_add(1) as usize].pos_prev = cur;
        make_literal(
            (&raw mut (*coder).opts as *mut lzma_optimal).offset(cur.wrapping_add(1) as isize)
                as *mut lzma_optimal,
        );
        next_is_literal = true;
    }
    let match_price: u32 = cur_price
        .wrapping_add(rc_bit_1_price((*coder).is_match[state as usize][pos_state as usize]) as u32);
    let rep_match_price: u32 =
        match_price.wrapping_add(rc_bit_1_price((*coder).is_rep[state as usize]) as u32);
    if match_byte as c_int == current_byte as c_int
        && !((*coder).opts[cur.wrapping_add(1) as usize].pos_prev < cur
            && (*coder).opts[cur.wrapping_add(1) as usize].back_prev == 0)
    {
        let short_rep_price: u32 =
            rep_match_price.wrapping_add(get_short_rep_price(coder, state, pos_state) as u32);
        if short_rep_price <= (*coder).opts[cur.wrapping_add(1) as usize].price {
            (*coder).opts[cur.wrapping_add(1) as usize].price = short_rep_price;
            (*coder).opts[cur.wrapping_add(1) as usize].pos_prev = cur;
            make_short_rep(
                (&raw mut (*coder).opts as *mut lzma_optimal).offset(cur.wrapping_add(1) as isize)
                    as *mut lzma_optimal,
            );
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
    if !next_is_literal && match_byte as c_int != current_byte as c_int {
        let buf_back: *const u8 = buf.offset(-(*reps.offset(0) as isize)).offset(-1);
        let limit: u32 = if buf_avail_full < nice_len.wrapping_add(1) {
            buf_avail_full
        } else {
            nice_len.wrapping_add(1)
        };
        let len_test: u32 = (lzma_memcmplen(buf, buf_back, 1, limit) as u32).wrapping_sub(1);
        if len_test >= 2 {
            let mut state_2: lzma_lzma_state = state;
            state_2 = (if state_2 <= STATE_SHORTREP_LIT_LIT {
                STATE_LIT_LIT
            } else if state_2 <= STATE_LIT_SHORTREP {
                (state_2 as u32).wrapping_sub(3)
            } else {
                (state_2 as u32).wrapping_sub(6)
            }) as lzma_lzma_state;
            let pos_state_next: u32 = position.wrapping_add(1) & (*coder).pos_mask;
            let next_rep_match_price: u32 = cur_and_1_price
                .wrapping_add(rc_bit_1_price(
                    (*coder).is_match[state_2 as usize][pos_state_next as usize],
                ) as u32)
                .wrapping_add(rc_bit_1_price((*coder).is_rep[state_2 as usize]) as u32);
            let offset: u32 = cur.wrapping_add(1u32).wrapping_add(len_test);
            while len_end < offset {
                len_end = len_end.wrapping_add(1);
                (*coder).opts[len_end as usize].price = RC_INFINITY_PRICE as u32;
            }
            let cur_and_len_price: u32 = next_rep_match_price.wrapping_add(get_rep_price(
                coder,
                0,
                len_test,
                state_2,
                pos_state_next,
            ) as u32);
            if cur_and_len_price < (*coder).opts[offset as usize].price {
                (*coder).opts[offset as usize].price = cur_and_len_price;
                (*coder).opts[offset as usize].pos_prev = cur.wrapping_add(1);
                (*coder).opts[offset as usize].back_prev = 0;
                (*coder).opts[offset as usize].prev_1_is_literal = true;
                (*coder).opts[offset as usize].prev_2 = false;
            }
        }
    }
    let mut start_len: u32 = 2;
    let mut rep_index: u32 = 0;
    while rep_index < REPS as u32 {
        let buf_back_0: *const u8 = buf
            .offset(-(*reps.offset(rep_index as isize) as isize))
            .offset(-1);
        if !(*buf.offset(0) as c_int != *buf_back_0.offset(0) as c_int
            || *buf.offset(1) as c_int != *buf_back_0.offset(1) as c_int)
        {
            let mut len_test_0: u32 = lzma_memcmplen(buf, buf_back_0, 2, buf_avail);
            while len_end < cur.wrapping_add(len_test_0) {
                len_end = len_end.wrapping_add(1);
                (*coder).opts[len_end as usize].price = RC_INFINITY_PRICE as u32;
            }
            let len_test_temp: u32 = len_test_0;
            let price: u32 = rep_match_price
                .wrapping_add(get_pure_rep_price(coder, rep_index, state, pos_state) as u32);
            loop {
                let cur_and_len_price_0: u32 = price.wrapping_add(get_len_price(
                    &raw mut (*coder).rep_len_encoder,
                    len_test_0,
                    pos_state,
                ) as u32);
                if cur_and_len_price_0 < (*coder).opts[cur.wrapping_add(len_test_0) as usize].price
                {
                    (*coder).opts[cur.wrapping_add(len_test_0) as usize].price =
                        cur_and_len_price_0;
                    (*coder).opts[cur.wrapping_add(len_test_0) as usize].pos_prev = cur;
                    (*coder).opts[cur.wrapping_add(len_test_0) as usize].back_prev = rep_index;
                    (*coder).opts[cur.wrapping_add(len_test_0) as usize].prev_1_is_literal = false;
                }
                len_test_0 = len_test_0.wrapping_sub(1);
                if !(len_test_0 >= 2) {
                    break;
                }
            }
            len_test_0 = len_test_temp;
            if rep_index == 0 {
                start_len = len_test_0.wrapping_add(1);
            }
            let mut len_test_2: u32 = len_test_0.wrapping_add(1);
            let limit_0: u32 = if buf_avail_full < len_test_2.wrapping_add(nice_len) {
                buf_avail_full
            } else {
                len_test_2.wrapping_add(nice_len)
            };
            if len_test_2 < limit_0 {
                len_test_2 = lzma_memcmplen(buf, buf_back_0, len_test_2, limit_0);
            }
            len_test_2 = len_test_2.wrapping_sub(len_test_0.wrapping_add(1));
            if len_test_2 >= 2 {
                let mut state_2_0: lzma_lzma_state = state;
                state_2_0 = (if (state_2_0 as u32) < LIT_STATES as u32 {
                    STATE_LIT_LONGREP as c_int
                } else {
                    STATE_NONLIT_REP as c_int
                }) as lzma_lzma_state;
                let mut pos_state_next_0: u32 =
                    position.wrapping_add(len_test_0) & (*coder).pos_mask;
                let cur_and_len_literal_price: u32 = price
                    .wrapping_add(get_len_price(
                        &raw mut (*coder).rep_len_encoder,
                        len_test_0,
                        pos_state,
                    ) as u32)
                    .wrapping_add(rc_bit_0_price(
                        (*coder).is_match[state_2_0 as usize][pos_state_next_0 as usize],
                    ) as u32)
                    .wrapping_add(get_literal_price(
                        coder,
                        position.wrapping_add(len_test_0),
                        *buf.offset(len_test_0.wrapping_sub(1) as isize) as u32,
                        true,
                        *buf_back_0.offset(len_test_0 as isize) as u32,
                        *buf.offset(len_test_0 as isize) as u32,
                    ) as u32);
                state_2_0 = (if state_2_0 <= STATE_SHORTREP_LIT_LIT {
                    STATE_LIT_LIT
                } else if state_2_0 <= STATE_LIT_SHORTREP {
                    (state_2_0 as u32).wrapping_sub(3)
                } else {
                    (state_2_0 as u32).wrapping_sub(6)
                }) as lzma_lzma_state;
                pos_state_next_0 =
                    position.wrapping_add(len_test_0).wrapping_add(1) & (*coder).pos_mask;
                let next_rep_match_price_0: u32 = cur_and_len_literal_price
                    .wrapping_add(rc_bit_1_price(
                        (*coder).is_match[state_2_0 as usize][pos_state_next_0 as usize],
                    ) as u32)
                    .wrapping_add(rc_bit_1_price((*coder).is_rep[state_2_0 as usize]) as u32);
                let offset_0: u32 = cur
                    .wrapping_add(len_test_0)
                    .wrapping_add(1)
                    .wrapping_add(len_test_2);
                while len_end < offset_0 {
                    len_end = len_end.wrapping_add(1);
                    (*coder).opts[len_end as usize].price = RC_INFINITY_PRICE as u32;
                }
                let cur_and_len_price_1: u32 = next_rep_match_price_0.wrapping_add(get_rep_price(
                    coder,
                    0,
                    len_test_2,
                    state_2_0,
                    pos_state_next_0,
                )
                    as u32);
                if cur_and_len_price_1 < (*coder).opts[offset_0 as usize].price {
                    (*coder).opts[offset_0 as usize].price = cur_and_len_price_1;
                    (*coder).opts[offset_0 as usize].pos_prev =
                        cur.wrapping_add(len_test_0).wrapping_add(1);
                    (*coder).opts[offset_0 as usize].back_prev = 0;
                    (*coder).opts[offset_0 as usize].prev_1_is_literal = true;
                    (*coder).opts[offset_0 as usize].prev_2 = true;
                    (*coder).opts[offset_0 as usize].pos_prev_2 = cur;
                    (*coder).opts[offset_0 as usize].back_prev_2 = rep_index;
                }
            }
        }
        rep_index = rep_index.wrapping_add(1);
    }
    if new_len > buf_avail {
        new_len = buf_avail;
        matches_count = 0;
        while new_len > (*coder).matches[matches_count as usize].len {
            matches_count = matches_count.wrapping_add(1);
        }
        let fresh0 = matches_count;
        matches_count = matches_count.wrapping_add(1);
        (*coder).matches[fresh0 as usize].len = new_len;
    }
    if new_len >= start_len {
        let normal_match_price: u32 =
            match_price.wrapping_add(rc_bit_0_price((*coder).is_rep[state as usize]) as u32);
        while len_end < cur.wrapping_add(new_len) {
            len_end = len_end.wrapping_add(1);
            (*coder).opts[len_end as usize].price = RC_INFINITY_PRICE as u32;
        }
        let mut i_2: u32 = 0;
        while start_len > (*coder).matches[i_2 as usize].len {
            i_2 = i_2.wrapping_add(1);
        }
        let mut len_test_1: u32 = start_len;
        loop {
            let cur_back: u32 = (*coder).matches[i_2 as usize].dist;
            let mut cur_and_len_price_2: u32 = normal_match_price
                .wrapping_add(get_dist_len_price(coder, cur_back, len_test_1, pos_state));
            if cur_and_len_price_2 < (*coder).opts[cur.wrapping_add(len_test_1) as usize].price {
                (*coder).opts[cur.wrapping_add(len_test_1) as usize].price = cur_and_len_price_2;
                (*coder).opts[cur.wrapping_add(len_test_1) as usize].pos_prev = cur;
                (*coder).opts[cur.wrapping_add(len_test_1) as usize].back_prev =
                    cur_back.wrapping_add(REPS as u32);
                (*coder).opts[cur.wrapping_add(len_test_1) as usize].prev_1_is_literal = false;
            }
            if len_test_1 == (*coder).matches[i_2 as usize].len {
                let buf_back_1: *const u8 = buf.offset(-(cur_back as isize)).offset(-1);
                let mut len_test_2_0: u32 = len_test_1.wrapping_add(1);
                let limit_1: u32 = if buf_avail_full < len_test_2_0.wrapping_add(nice_len) {
                    buf_avail_full
                } else {
                    len_test_2_0.wrapping_add(nice_len)
                };
                if len_test_2_0 < limit_1 {
                    len_test_2_0 = lzma_memcmplen(buf, buf_back_1, len_test_2_0, limit_1);
                }
                len_test_2_0 = len_test_2_0.wrapping_sub(len_test_1.wrapping_add(1));
                if len_test_2_0 >= 2 {
                    let mut state_2_1: lzma_lzma_state = state;
                    state_2_1 = (if (state_2_1 as u32) < LIT_STATES as u32 {
                        STATE_LIT_MATCH as c_int
                    } else {
                        STATE_NONLIT_MATCH as c_int
                    }) as lzma_lzma_state;
                    let mut pos_state_next_1: u32 =
                        position.wrapping_add(len_test_1) & (*coder).pos_mask;
                    let cur_and_len_literal_price_0: u32 = cur_and_len_price_2
                        .wrapping_add(rc_bit_0_price(
                            (*coder).is_match[state_2_1 as usize][pos_state_next_1 as usize],
                        ) as u32)
                        .wrapping_add(get_literal_price(
                            coder,
                            position.wrapping_add(len_test_1),
                            *buf.offset(len_test_1.wrapping_sub(1) as isize) as u32,
                            true,
                            *buf_back_1.offset(len_test_1 as isize) as u32,
                            *buf.offset(len_test_1 as isize) as u32,
                        ) as u32);
                    state_2_1 = (if state_2_1 <= STATE_SHORTREP_LIT_LIT {
                        STATE_LIT_LIT
                    } else if state_2_1 <= STATE_LIT_SHORTREP {
                        (state_2_1 as u32).wrapping_sub(3)
                    } else {
                        (state_2_1 as u32).wrapping_sub(6)
                    }) as lzma_lzma_state;
                    pos_state_next_1 = pos_state_next_1.wrapping_add(1) & (*coder).pos_mask;
                    let next_rep_match_price_1: u32 = cur_and_len_literal_price_0
                        .wrapping_add(rc_bit_1_price(
                            (*coder).is_match[state_2_1 as usize][pos_state_next_1 as usize],
                        ) as u32)
                        .wrapping_add(rc_bit_1_price((*coder).is_rep[state_2_1 as usize]) as u32);
                    let offset_1: u32 = cur
                        .wrapping_add(len_test_1)
                        .wrapping_add(1)
                        .wrapping_add(len_test_2_0);
                    while len_end < offset_1 {
                        len_end = len_end.wrapping_add(1);
                        (*coder).opts[len_end as usize].price = RC_INFINITY_PRICE as u32;
                    }
                    cur_and_len_price_2 = next_rep_match_price_1.wrapping_add(get_rep_price(
                        coder,
                        0,
                        len_test_2_0,
                        state_2_1,
                        pos_state_next_1,
                    ));
                    if cur_and_len_price_2 < (*coder).opts[offset_1 as usize].price {
                        (*coder).opts[offset_1 as usize].price = cur_and_len_price_2;
                        (*coder).opts[offset_1 as usize].pos_prev =
                            cur.wrapping_add(len_test_1).wrapping_add(1);
                        (*coder).opts[offset_1 as usize].back_prev = 0;
                        (*coder).opts[offset_1 as usize].prev_1_is_literal = true;
                        (*coder).opts[offset_1 as usize].prev_2 = true;
                        (*coder).opts[offset_1 as usize].pos_prev_2 = cur;
                        (*coder).opts[offset_1 as usize].back_prev_2 =
                            cur_back.wrapping_add(REPS as u32);
                    }
                }
                i_2 = i_2.wrapping_add(1);
                if i_2 == matches_count {
                    break;
                }
            }
            len_test_1 = len_test_1.wrapping_add(1);
        }
    }
    return len_end;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_optimum_normal(
    coder: *mut lzma_lzma1_encoder,
    mf: *mut lzma_mf,
    back_res: *mut u32,
    len_res: *mut u32,
    position: u32,
) {
    if (*coder).opts_end_index != (*coder).opts_current_index {
        *len_res = (*coder).opts[(*coder).opts_current_index as usize]
            .pos_prev
            .wrapping_sub((*coder).opts_current_index);
        *back_res = (*coder).opts[(*coder).opts_current_index as usize].back_prev;
        (*coder).opts_current_index = (*coder).opts[(*coder).opts_current_index as usize].pos_prev;
        return;
    }
    if (*mf).read_ahead == 0 {
        if (*coder).match_price_count >= ((1 as c_int) << 7) as u32 {
            fill_dist_prices(coder);
        }
        if (*coder).align_price_count >= ALIGN_SIZE as u32 {
            fill_align_prices(coder);
        }
    }
    let mut len_end: u32 = helper1(coder, mf, back_res, len_res, position);
    if len_end == UINT32_MAX as u32 {
        return;
    }
    let mut reps: [u32; 4] = [0; 4];
    memcpy(
        &raw mut reps as *mut u32 as *mut c_void,
        &raw mut (*coder).reps as *mut u32 as *const c_void,
        core::mem::size_of::<[u32; 4]>(),
    );
    let mut cur: u32 = 0;
    cur = 1;
    while cur < len_end {
        (*coder).longest_match_length = lzma_mf_find(
            mf,
            &raw mut (*coder).matches_count,
            &raw mut (*coder).matches as *mut lzma_match,
        );
        if (*coder).longest_match_length >= (*mf).nice_len {
            break;
        }
        len_end = helper2(
            coder,
            &raw mut reps as *mut u32,
            mf_ptr(mf).offset(-1),
            len_end,
            position.wrapping_add(cur),
            cur,
            (*mf).nice_len,
            if mf_avail(mf).wrapping_add(1)
                < ((((1 as c_int) << 12) - 1 as c_int) as u32).wrapping_sub(cur)
            {
                mf_avail(mf).wrapping_add(1)
            } else {
                ((((1 as c_int) << 12) - 1 as c_int) as u32).wrapping_sub(cur)
            },
        );
        cur = cur.wrapping_add(1);
    }
    backward(coder, len_res, back_res, cur);
}
