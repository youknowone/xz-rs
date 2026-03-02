use crate::types::*;
use core::ffi::c_void;
extern "C" {
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
}
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
#[inline]
unsafe extern "C" fn rc_reset(rc: *mut lzma_range_encoder) {
    (*rc).low = 0;
    (*rc).cache_size = 1;
    (*rc).range = UINT32_MAX;
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
    (*rc).symbols[(*rc).count as usize] = bit as rc_symbol;
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
        bit_count -= 1;
        let bit: u32 = symbol >> bit_count & 1;
        rc_bit(
            rc,
            probs.offset(model_index as isize) as *mut probability,
            bit,
        );
        model_index = (model_index << 1).wrapping_add(bit);
        if bit_count == 0 {
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
        symbol >>= 1;
        rc_bit(
            rc,
            probs.offset(model_index as isize) as *mut probability,
            bit,
        );
        model_index = (model_index << 1).wrapping_add(bit);
        bit_count -= 1;
        if bit_count == 0 {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn rc_direct(rc: *mut lzma_range_encoder, value: u32, mut bit_count: u32) {
    loop {
        bit_count -= 1;
        (*rc).symbols[(*rc).count as usize] =
            (RC_DIRECT_0 as u32).wrapping_add(value >> bit_count & 1) as rc_symbol;
        (*rc).count += 1;
        if bit_count == 0 {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn rc_flush(rc: *mut lzma_range_encoder) {
    let mut i: size_t = 0;
    while i < 5 {
        (*rc).symbols[(*rc).count as usize] = RC_FLUSH;
        (*rc).count += 1;
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn rc_shift_low(
    rc: *mut lzma_range_encoder,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> bool {
    if ((*rc).low as u32) < 0xff000000 || ((*rc).low >> 32) as u32 != 0 {
        loop {
            if *out_pos == out_size {
                return true;
            }
            *out.offset(*out_pos as isize) = (*rc).cache.wrapping_add(((*rc).low >> 32) as u8);
            *out_pos = (*out_pos).wrapping_add(1);
            (*rc).out_total = (*rc).out_total.wrapping_add(1);
            (*rc).cache = 0xff;
            (*rc).cache_size = (*rc).cache_size.wrapping_sub(1);
            if (*rc).cache_size == 0 {
                break;
            }
        }
        (*rc).cache = ((*rc).low >> 24 & 0xff as u64) as u8;
    }
    (*rc).cache_size = (*rc).cache_size.wrapping_add(1);
    (*rc).low = ((*rc).low & 0xffffff as u64) << RC_SHIFT_BITS;
    false
}
#[inline]
unsafe extern "C" fn rc_shift_low_dummy(
    low: *mut u64,
    cache_size: *mut u64,
    cache: *mut u8,
    out_pos: *mut u64,
    out_size: u64,
) -> bool {
    if (*low as u32) < 0xff000000 || (*low >> 32) as u32 != 0 {
        loop {
            if *out_pos == out_size {
                return true;
            }
            *out_pos = (*out_pos).wrapping_add(1);
            *cache = 0xff;
            *cache_size = (*cache_size).wrapping_sub(1);
            if *cache_size == 0 {
                break;
            }
        }
        *cache = (*low >> 24 & 0xff as u64) as u8;
    }
    *cache_size = (*cache_size).wrapping_add(1);
    *low = (*low & 0xffffff as u64) << RC_SHIFT_BITS;
    false
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
                    as probability;
                *(*rc).probs[(*rc).pos as usize] = prob;
            }
            1 => {
                let mut prob_0: probability = *(*rc).probs[(*rc).pos as usize];
                let bound: u32 =
                    (prob_0 as u32).wrapping_mul((*rc).range >> RC_BIT_MODEL_TOTAL_BITS);
                (*rc).low = (*rc).low.wrapping_add(bound as u64);
                (*rc).range = (*rc).range.wrapping_sub(bound);
                prob_0 -= prob_0 >> RC_MOVE_BITS;
                *(*rc).probs[(*rc).pos as usize] = prob_0;
            }
            2 => {
                (*rc).range >>= 1;
            }
            3 => {
                (*rc).range >>= 1;
                (*rc).low = (*rc).low.wrapping_add((*rc).range as u64);
            }
            4 => {
                (*rc).range = UINT32_MAX;
                loop {
                    if rc_shift_low(rc, out, out_pos, out_size) {
                        return true;
                    }
                    (*rc).pos = (*rc).pos.wrapping_add(1);
                    if (*rc).pos >= (*rc).count {
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
    false
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
                range >>= 1;
            }
            3 => {
                range >>= 1;
                low = low.wrapping_add(range as u64);
            }
            4 | _ => {}
        }
        pos += 1;
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
        pos += 1;
    }
    false
}
#[inline]
unsafe extern "C" fn rc_pending(rc: *const lzma_range_encoder) -> u64 {
    (*rc).cache_size.wrapping_add(5).wrapping_sub(1)
}
pub const LEN_SYMBOLS: u32 = LEN_LOW_SYMBOLS + LEN_MID_SYMBOLS + LEN_HIGH_SYMBOLS;
pub const MATCH_LEN_MAX: u32 = MATCH_LEN_MIN + LEN_SYMBOLS - 1;
#[inline]
unsafe extern "C" fn literal_matched(
    rc: *mut lzma_range_encoder,
    subcoder: *mut probability,
    mut match_byte: u32,
    mut symbol: u32,
) {
    let mut offset: u32 = 0x100;
    symbol = (symbol as u32).wrapping_add(1u32 << 8) as u32;
    loop {
        match_byte <<= 1;
        let match_bit: u32 = match_byte & offset;
        let subcoder_index: u32 = offset.wrapping_add(match_bit).wrapping_add(symbol >> 8);
        let bit: u32 = symbol >> 7 & 1;
        rc_bit(
            rc,
            subcoder.offset(subcoder_index as isize) as *mut probability,
            bit,
        );
        symbol <<= 1;
        offset &= !(match_byte ^ symbol);
        if symbol >= 1 << 16 {
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
    if ((*coder).state as u32) < LIT_STATES {
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
    while i < table_size && i < LEN_LOW_SYMBOLS {
        *prices.offset(i as isize) = a0.wrapping_add(rc_bittree_price(
            &raw mut *(&raw mut (*lc).low as *mut [probability; 8]).offset(pos_state as isize)
                as *mut probability,
            LEN_LOW_BITS,
            i,
        ));
        i += 1;
    }
    while i < table_size && i < (LEN_LOW_SYMBOLS + LEN_MID_SYMBOLS) as u32 {
        *prices.offset(i as isize) = b0.wrapping_add(rc_bittree_price(
            &raw mut *(&raw mut (*lc).mid as *mut [probability; 8]).offset(pos_state as isize)
                as *mut probability,
            LEN_MID_BITS,
            i.wrapping_sub(LEN_LOW_SYMBOLS),
        ));
        i += 1;
    }
    while i < table_size {
        *prices.offset(i as isize) = b1.wrapping_add(rc_bittree_price(
            &raw mut (*lc).high as *mut probability,
            LEN_HIGH_BITS,
            i.wrapping_sub(LEN_LOW_SYMBOLS)
                .wrapping_sub(LEN_MID_SYMBOLS),
        ));
        i += 1;
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
    len = len.wrapping_sub(MATCH_LEN_MIN);
    if len < LEN_LOW_SYMBOLS {
        rc_bit(rc, &raw mut (*lc).choice, 0);
        rc_bittree(
            rc,
            &raw mut *(&raw mut (*lc).low as *mut [probability; 8]).offset(pos_state as isize)
                as *mut probability,
            LEN_LOW_BITS,
            len,
        );
    } else {
        rc_bit(rc, &raw mut (*lc).choice, 1);
        len = len.wrapping_sub(LEN_LOW_SYMBOLS);
        if len < LEN_MID_SYMBOLS {
            rc_bit(rc, &raw mut (*lc).choice2, 0);
            rc_bittree(
                rc,
                &raw mut *(&raw mut (*lc).mid as *mut [probability; 8]).offset(pos_state as isize)
                    as *mut probability,
                LEN_MID_BITS,
                len,
            );
        } else {
            rc_bit(rc, &raw mut (*lc).choice2, 1);
            len = len.wrapping_sub(LEN_MID_SYMBOLS);
            rc_bittree(
                rc,
                &raw mut (*lc).high as *mut probability,
                LEN_HIGH_BITS,
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
    (*coder).state = (if ((*coder).state as u32) < LIT_STATES {
        STATE_LIT_MATCH
    } else {
        STATE_NONLIT_MATCH
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
        len.wrapping_sub(MATCH_LEN_MIN)
    } else {
        (DIST_STATES - 1) as u32
    };
    rc_bittree(
        &raw mut (*coder).rc,
        &raw mut *(&raw mut (*coder).dist_slot as *mut [probability; 64])
            .offset(dist_state as isize) as *mut probability,
        DIST_SLOT_BITS,
        dist_slot,
    );
    if dist_slot >= DIST_MODEL_START {
        let footer_bits: u32 = (dist_slot >> 1).wrapping_sub(1);
        let base: u32 = (2 | dist_slot & 1) << footer_bits;
        let dist_reduced: u32 = distance.wrapping_sub(base);
        if dist_slot < DIST_MODEL_END {
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
                footer_bits.wrapping_sub(ALIGN_BITS),
            );
            rc_bittree_reverse(
                &raw mut (*coder).rc,
                &raw mut (*coder).dist_align as *mut probability,
                ALIGN_BITS,
                dist_reduced & ALIGN_MASK,
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
        (*coder).state = (if ((*coder).state as u32) < LIT_STATES {
            STATE_LIT_SHORTREP
        } else {
            STATE_NONLIT_REP
        }) as lzma_lzma_state;
    } else {
        length(
            &raw mut (*coder).rc,
            &raw mut (*coder).rep_len_encoder,
            pos_state,
            len,
            (*coder).fast_mode,
        );
        (*coder).state = (if ((*coder).state as u32) < LIT_STATES {
            STATE_LIT_LONGREP
        } else {
            STATE_NONLIT_REP
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
    if back == UINT32_MAX {
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
        if back < REPS {
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
            match_0(coder, pos_state, back.wrapping_sub(REPS), len);
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
            (&raw mut *(&raw mut (*coder).is_match as *mut [probability; 16]) as *mut probability)
                as *mut probability,
            0,
        );
        rc_bittree(
            &raw mut (*coder).rc,
            &raw mut (*coder).literal as *mut probability,
            8,
            *(*mf).buffer as u32,
        );
        (*coder).uncomp_size = (*coder).uncomp_size.wrapping_add(1);
    }
    (*coder).is_initialized = true;
    true
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
    match_0(coder, pos_state, UINT32_MAX, MATCH_LEN_MIN);
}
pub const LOOP_INPUT_MAX: u32 = OPTS + 1;
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
    while !(limit != UINT32_MAX
        && ((*mf).read_pos.wrapping_sub((*mf).read_ahead) >= limit
            || (*out_pos as u64).wrapping_add(rc_pending(&raw mut (*coder).rc))
                >= LZMA2_CHUNK_MAX.wrapping_sub(LOOP_INPUT_MAX) as u64))
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
        if (*coder).out_limit != 0 && rc_encode_dummy(&raw mut (*coder).rc, (*coder).out_limit) {
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
    LZMA_STREAM_END
}
unsafe extern "C" fn lzma_encode(
    coder: *mut c_void,
    mf: *mut lzma_mf,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    if (*mf).action == LZMA_SYNC_FLUSH {
        return LZMA_OPTIONS_ERROR;
    }
    lzma_lzma_encode(
        coder as *mut lzma_lzma1_encoder,
        mf,
        out,
        out_pos,
        out_size,
        UINT32_MAX,
    )
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
    LZMA_OK
}
extern "C" fn is_options_valid(options: *const lzma_options_lzma) -> bool {
    unsafe {
        is_lclppb_valid(options)
            && (*options).nice_len >= MATCH_LEN_MIN
            && (*options).nice_len <= MATCH_LEN_MAX
            && ((*options).mode == LZMA_MODE_FAST || (*options).mode == LZMA_MODE_NORMAL)
    }
}
extern "C" fn set_lz_options(lz_options: *mut lzma_lz_options, options: *const lzma_options_lzma) {
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
        while bt_i < (1 << 3) as u32 {
            (*lencoder).low[pos_state as usize][bt_i as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            bt_i += 1;
        }
        let mut bt_i_0: u32 = 0;
        while bt_i_0 < (1 << 3) as u32 {
            (*lencoder).mid[pos_state as usize][bt_i_0 as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            bt_i_0 += 1;
        }
        pos_state += 1;
    }
    let mut bt_i_1: u32 = 0;
    while bt_i_1 < (1 << 8) as u32 {
        (*lencoder).high[bt_i_1 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        bt_i_1 += 1;
    }
    if !fast_mode {
        let mut pos_state_0: u32 = 0;
        while pos_state_0 < num_pos_states {
            length_update_prices(lencoder, pos_state_0);
            pos_state_0 += 1;
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
    (*coder).literal_mask = (0x100u32 << (*options).lp).wrapping_sub(0x100 >> (*options).lc);
    rc_reset(&raw mut (*coder).rc);
    (*coder).state = STATE_LIT_LIT;
    let mut i: size_t = 0;
    while i < REPS as size_t {
        (*coder).reps[i as usize] = 0;
        i += 1;
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
            j += 1;
        }
        (*coder).is_rep[i_0 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        (*coder).is_rep0[i_0 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        (*coder).is_rep1[i_0 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        (*coder).is_rep2[i_0 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        i_0 += 1;
    }
    let mut i_1: size_t = 0;
    while i_1 < (FULL_DISTANCES - DIST_MODEL_END) as size_t {
        (*coder).dist_special[i_1 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        i_1 += 1;
    }
    let mut i_2: size_t = 0;
    while i_2 < DIST_STATES as size_t {
        let mut bt_i: u32 = 0;
        while bt_i < (1 << 6) as u32 {
            (*coder).dist_slot[i_2 as usize][bt_i as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            bt_i += 1;
        }
        i_2 += 1;
    }
    let mut bt_i_0: u32 = 0;
    while bt_i_0 < (1 << 4) as u32 {
        (*coder).dist_align[bt_i_0 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        bt_i_0 += 1;
    }
    length_encoder_reset(
        &raw mut (*coder).match_len_encoder,
        1 << (*options).pb,
        (*coder).fast_mode,
    );
    length_encoder_reset(
        &raw mut (*coder).rep_len_encoder,
        1 << (*options).pb,
        (*coder).fast_mode,
    );
    (*coder).match_price_count = UINT32_MAX.wrapping_div(2);
    (*coder).align_price_count = UINT32_MAX.wrapping_div(2);
    (*coder).opts_end_index = 0;
    (*coder).opts_current_index = 0;
    LZMA_OK
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
        *coder_ptr = lzma_alloc(core::mem::size_of::<lzma_lzma1_encoder>(), allocator);
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
            if (*options).dict_size > (1u32 << 30).wrapping_add(1 << 29) {
                return LZMA_OPTIONS_ERROR;
            }
            let mut log_size: u32 = 0;
            while 1 << log_size < (*options).dict_size {
                log_size += 1;
            }
            (*coder).dist_table_size = log_size.wrapping_mul(2);
            let nice_len: u32 = if mf_get_hash_bytes((*options).mf) > (*options).nice_len {
                mf_get_hash_bytes((*options).mf) as u32
            } else {
                (*options).nice_len
            };
            (*coder).match_len_encoder.table_size =
                nice_len.wrapping_add(1u32).wrapping_sub(MATCH_LEN_MIN);
            (*coder).rep_len_encoder.table_size =
                nice_len.wrapping_add(1u32).wrapping_sub(MATCH_LEN_MIN);
        }
        _ => return LZMA_OPTIONS_ERROR,
    }
    (*coder).is_initialized = !(*options).preset_dict.is_null() && (*options).preset_dict_size > 0;
    (*coder).is_flushed = false;
    (*coder).uncomp_size = 0;
    (*coder).uncomp_size_ptr = core::ptr::null_mut();
    (*coder).out_limit = 0;
    (*coder).use_eopm = id == LZMA_FILTER_LZMA1;
    if id == LZMA_FILTER_LZMA1EXT {
        if (*options).ext_flags & !(LZMA_LZMA1EXT_ALLOW_EOPM as u32) != 0 {
            return LZMA_OPTIONS_ERROR;
        }
        (*coder).use_eopm = (*options).ext_flags & LZMA_LZMA1EXT_ALLOW_EOPM as u32 != 0;
    }
    set_lz_options(lz_options, options);
    lzma_lzma_encoder_reset(coder, options)
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
    );
    (*lz).set_out_limit = Some(
        lzma_lzma_set_out_limit as unsafe extern "C" fn(*mut c_void, *mut u64, u64) -> lzma_ret,
    );
    lzma_lzma_encoder_create(
        &raw mut (*lz).coder,
        allocator,
        id,
        options as *const lzma_options_lzma,
        lz_options,
    )
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    lzma_lz_encoder_init(
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
    )
}
#[no_mangle]
pub extern "C" fn lzma_lzma_encoder_memusage(options: *const c_void) -> u64 {
    if !is_options_valid(options as *const lzma_options_lzma) {
        return UINT64_MAX;
    }
    let mut lz_options: lzma_lz_options = lzma_lz_options {
        before_size: 0,
        dict_size: 0,
        after_size: 0,
        match_len_max: 0,
        nice_len: 0,
        match_finder: 0,
        depth: 0,
        preset_dict: core::ptr::null(),
        preset_dict_size: 0,
    };
    set_lz_options(&raw mut lz_options, options as *const lzma_options_lzma);
    let lz_memusage: u64 = unsafe { lzma_lz_encoder_memusage(&raw mut lz_options) } as u64;
    if lz_memusage == UINT64_MAX {
        return UINT64_MAX;
    }
    (core::mem::size_of::<lzma_lzma1_encoder>() as u64).wrapping_add(lz_memusage)
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
    false
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
    LZMA_OK
}
#[no_mangle]
pub extern "C" fn lzma_mode_is_supported(mode: lzma_mode) -> lzma_bool {
    (mode == LZMA_MODE_FAST || mode == LZMA_MODE_NORMAL) as lzma_bool
}
