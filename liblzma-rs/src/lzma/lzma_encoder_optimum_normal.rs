extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn lzma_mf_find(mf: *mut lzma_mf, count: *mut uint32_t, matches: *mut lzma_match) -> uint32_t;
    static lzma_rc_prices: [uint8_t; 128];
    static lzma_fastpos: [uint8_t; 8192];
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
#[inline]
unsafe extern "C" fn mf_skip(mut mf: *mut lzma_mf, mut amount: uint32_t) {
    if amount != 0 as uint32_t {
        (*mf).skip.expect("non-null function pointer")(mf, amount);
        (*mf).read_ahead = (*mf).read_ahead.wrapping_add(amount);
    }
}
pub const RC_BIT_MODEL_TOTAL_BITS: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const RC_BIT_MODEL_TOTAL: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << RC_BIT_MODEL_TOTAL_BITS;
pub const RC_MOVE_REDUCING_BITS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const RC_BIT_PRICE_SHIFT_BITS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const RC_INFINITY_PRICE: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 30 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn rc_bit_price(prob: probability, bit: uint32_t) -> uint32_t {
    return lzma_rc_prices[((prob as uint32_t
        ^ (0 as uint32_t).wrapping_sub(bit)
            & (RC_BIT_MODEL_TOTAL as uint32_t).wrapping_sub(1 as uint32_t))
        >> RC_MOVE_REDUCING_BITS) as usize] as uint32_t;
}
#[inline]
unsafe extern "C" fn rc_bit_0_price(prob: probability) -> uint32_t {
    return lzma_rc_prices[(prob as ::core::ffi::c_int >> RC_MOVE_REDUCING_BITS) as usize]
        as uint32_t;
}
#[inline]
unsafe extern "C" fn rc_bit_1_price(prob: probability) -> uint32_t {
    return lzma_rc_prices[((prob as ::core::ffi::c_uint
        ^ RC_BIT_MODEL_TOTAL.wrapping_sub(1 as ::core::ffi::c_uint))
        >> RC_MOVE_REDUCING_BITS) as usize] as uint32_t;
}
#[inline]
unsafe extern "C" fn rc_bittree_price(
    probs: *const probability,
    bit_levels: uint32_t,
    mut symbol: uint32_t,
) -> uint32_t {
    let mut price: uint32_t = 0 as uint32_t;
    symbol = (symbol as ::core::ffi::c_uint).wrapping_add((1 as ::core::ffi::c_uint) << bit_levels)
        as uint32_t as uint32_t;
    loop {
        let bit: uint32_t = symbol & 1 as uint32_t;
        symbol >>= 1 as ::core::ffi::c_int;
        price = price.wrapping_add(rc_bit_price(*probs.offset(symbol as isize), bit));
        if !(symbol != 1 as uint32_t) {
            break;
        }
    }
    return price;
}
#[inline]
unsafe extern "C" fn rc_bittree_reverse_price(
    probs: *const probability,
    mut bit_levels: uint32_t,
    mut symbol: uint32_t,
) -> uint32_t {
    let mut price: uint32_t = 0 as uint32_t;
    let mut model_index: uint32_t = 1 as uint32_t;
    loop {
        let bit: uint32_t = symbol & 1 as uint32_t;
        symbol >>= 1 as ::core::ffi::c_int;
        price = price.wrapping_add(rc_bit_price(*probs.offset(model_index as isize), bit));
        model_index = (model_index << 1 as ::core::ffi::c_int).wrapping_add(bit);
        bit_levels = bit_levels.wrapping_sub(1);
        if !(bit_levels != 0 as uint32_t) {
            break;
        }
    }
    return price;
}
#[inline]
unsafe extern "C" fn rc_direct_price(bits: uint32_t) -> uint32_t {
    return bits << RC_BIT_PRICE_SHIFT_BITS;
}
pub const LIT_STATES: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const MATCH_LEN_MIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DIST_STATES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DIST_SLOT_BITS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const DIST_MODEL_START: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DIST_MODEL_END: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const FULL_DISTANCES_BITS: ::core::ffi::c_int = DIST_MODEL_END / 2 as ::core::ffi::c_int;
pub const FULL_DISTANCES: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << FULL_DISTANCES_BITS;
pub const ALIGN_BITS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ALIGN_SIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << ALIGN_BITS;
pub const ALIGN_MASK: ::core::ffi::c_int = ALIGN_SIZE - 1 as ::core::ffi::c_int;
pub const REPS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FASTPOS_BITS: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn get_dist_slot(mut dist: uint32_t) -> uint32_t {
    if dist
        < (1 as uint32_t)
            << FASTPOS_BITS
                + (0 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
    {
        return lzma_fastpos[dist as usize] as uint32_t;
    }
    if dist
        < (1 as uint32_t)
            << FASTPOS_BITS
                + (0 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
    {
        return (lzma_fastpos[(dist
            >> 0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
            as usize] as uint32_t)
            .wrapping_add(
                (2 as ::core::ffi::c_int
                    * (0 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int)))
                    as uint32_t,
            );
    }
    return (lzma_fastpos[(dist
        >> 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
        as usize] as uint32_t)
        .wrapping_add(
            (2 as ::core::ffi::c_int
                * (0 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int)))
                as uint32_t,
        );
}
#[inline]
unsafe extern "C" fn get_dist_slot_2(mut dist: uint32_t) -> uint32_t {
    if dist
        < (1 as uint32_t)
            << FASTPOS_BITS
                + (14 as ::core::ffi::c_int / 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
    {
        return (lzma_fastpos[(dist
            >> 14 as ::core::ffi::c_int / 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
            as usize] as uint32_t)
            .wrapping_add(
                (2 as ::core::ffi::c_int
                    * (14 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int)))
                    as uint32_t,
            );
    }
    if dist
        < (1 as uint32_t)
            << FASTPOS_BITS
                + (14 as ::core::ffi::c_int / 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
    {
        return (lzma_fastpos[(dist
            >> 14 as ::core::ffi::c_int / 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
            as usize] as uint32_t)
            .wrapping_add(
                (2 as ::core::ffi::c_int
                    * (14 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int)))
                    as uint32_t,
            );
    }
    return (lzma_fastpos[(dist
        >> 14 as ::core::ffi::c_int / 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int))
        as usize] as uint32_t)
        .wrapping_add(
            (2 as ::core::ffi::c_int
                * (14 as ::core::ffi::c_int / 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int * (FASTPOS_BITS - 1 as ::core::ffi::c_int)))
                as uint32_t,
        );
}
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
unsafe extern "C" fn get_literal_price(
    coder: *const lzma_lzma1_encoder,
    pos: uint32_t,
    prev_byte: uint32_t,
    match_mode: bool,
    mut match_byte: uint32_t,
    mut symbol: uint32_t,
) -> uint32_t {
    let subcoder: *const probability =
        (&raw const (*coder).literal as *const probability).offset((3 as uint32_t).wrapping_mul(
            ((pos << 8 as ::core::ffi::c_int).wrapping_add(prev_byte) & (*coder).literal_mask)
                << (*coder).literal_context_bits,
        ) as isize);
    let mut price: uint32_t = 0 as uint32_t;
    if !match_mode {
        price = rc_bittree_price(subcoder, 8 as uint32_t, symbol);
    } else {
        let mut offset: uint32_t = 0x100 as uint32_t;
        symbol = (symbol as ::core::ffi::c_uint)
            .wrapping_add((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
            as uint32_t as uint32_t;
        loop {
            match_byte <<= 1 as ::core::ffi::c_int;
            let match_bit: uint32_t = match_byte & offset;
            let subcoder_index: uint32_t = offset
                .wrapping_add(match_bit)
                .wrapping_add(symbol >> 8 as ::core::ffi::c_int);
            let bit: uint32_t = symbol >> 7 as ::core::ffi::c_int & 1 as uint32_t;
            price =
                price.wrapping_add(rc_bit_price(*subcoder.offset(subcoder_index as isize), bit));
            symbol <<= 1 as ::core::ffi::c_int;
            offset &= !(match_byte ^ symbol);
            if !(symbol < (1 as uint32_t) << 16 as ::core::ffi::c_int) {
                break;
            }
        }
    }
    return price;
}
#[inline]
unsafe extern "C" fn get_len_price(
    lencoder: *const lzma_length_encoder,
    len: uint32_t,
    pos_state: uint32_t,
) -> uint32_t {
    return (*lencoder).prices[pos_state as usize]
        [len.wrapping_sub(MATCH_LEN_MIN as uint32_t) as usize];
}
#[inline]
unsafe extern "C" fn get_short_rep_price(
    coder: *const lzma_lzma1_encoder,
    state: lzma_lzma_state,
    pos_state: uint32_t,
) -> uint32_t {
    return rc_bit_0_price((*coder).is_rep0[state as usize]).wrapping_add(rc_bit_0_price(
        (*coder).is_rep0_long[state as usize][pos_state as usize],
    ));
}
#[inline]
unsafe extern "C" fn get_pure_rep_price(
    coder: *const lzma_lzma1_encoder,
    rep_index: uint32_t,
    state: lzma_lzma_state,
    mut pos_state: uint32_t,
) -> uint32_t {
    let mut price: uint32_t = 0;
    if rep_index == 0 as uint32_t {
        price = rc_bit_0_price((*coder).is_rep0[state as usize]);
        price = price.wrapping_add(rc_bit_1_price(
            (*coder).is_rep0_long[state as usize][pos_state as usize],
        ));
    } else {
        price = rc_bit_1_price((*coder).is_rep0[state as usize]);
        if rep_index == 1 as uint32_t {
            price = price.wrapping_add(rc_bit_0_price((*coder).is_rep1[state as usize]));
        } else {
            price = price.wrapping_add(rc_bit_1_price((*coder).is_rep1[state as usize]));
            price = price.wrapping_add(rc_bit_price(
                (*coder).is_rep2[state as usize],
                rep_index.wrapping_sub(2 as uint32_t),
            ));
        }
    }
    return price;
}
#[inline]
unsafe extern "C" fn get_rep_price(
    coder: *const lzma_lzma1_encoder,
    rep_index: uint32_t,
    len: uint32_t,
    state: lzma_lzma_state,
    pos_state: uint32_t,
) -> uint32_t {
    return get_len_price(&raw const (*coder).rep_len_encoder, len, pos_state)
        .wrapping_add(get_pure_rep_price(coder, rep_index, state, pos_state));
}
#[inline]
unsafe extern "C" fn get_dist_len_price(
    coder: *const lzma_lzma1_encoder,
    dist: uint32_t,
    len: uint32_t,
    pos_state: uint32_t,
) -> uint32_t {
    let dist_state: uint32_t = if len < (DIST_STATES + MATCH_LEN_MIN) as uint32_t {
        len.wrapping_sub(MATCH_LEN_MIN as uint32_t)
    } else {
        (DIST_STATES - 1 as ::core::ffi::c_int) as uint32_t
    };
    let mut price: uint32_t = 0;
    if dist < FULL_DISTANCES as uint32_t {
        price = (*coder).dist_prices[dist_state as usize][dist as usize];
    } else {
        let dist_slot: uint32_t = get_dist_slot_2(dist) as uint32_t;
        price = (*coder).dist_slot_prices[dist_state as usize][dist_slot as usize]
            .wrapping_add((*coder).align_prices[(dist & ALIGN_MASK as uint32_t) as usize]);
    }
    price = price.wrapping_add(get_len_price(
        &raw const (*coder).match_len_encoder,
        len,
        pos_state,
    ));
    return price;
}
unsafe extern "C" fn fill_dist_prices(mut coder: *mut lzma_lzma1_encoder) {
    let mut dist_state: uint32_t = 0 as uint32_t;
    while dist_state < DIST_STATES as uint32_t {
        let dist_slot_prices: *mut uint32_t =
            &raw mut *(&raw mut (*coder).dist_slot_prices as *mut [uint32_t; 64])
                .offset(dist_state as isize) as *mut uint32_t;
        let mut dist_slot: uint32_t = 0 as uint32_t;
        while dist_slot < (*coder).dist_table_size {
            *dist_slot_prices.offset(dist_slot as isize) = rc_bittree_price(
                &raw mut *(&raw mut (*coder).dist_slot as *mut [probability; 64])
                    .offset(dist_state as isize) as *mut probability,
                DIST_SLOT_BITS as uint32_t,
                dist_slot,
            );
            dist_slot = dist_slot.wrapping_add(1);
        }
        let mut dist_slot_0: uint32_t = DIST_MODEL_END as uint32_t;
        while dist_slot_0 < (*coder).dist_table_size {
            let ref mut fresh1 = *dist_slot_prices.offset(dist_slot_0 as isize);
            *fresh1 = (*fresh1).wrapping_add(rc_direct_price(
                (dist_slot_0 >> 1 as ::core::ffi::c_int)
                    .wrapping_sub(1 as uint32_t)
                    .wrapping_sub(ALIGN_BITS as uint32_t),
            ));
            dist_slot_0 = dist_slot_0.wrapping_add(1);
        }
        let mut i: uint32_t = 0 as uint32_t;
        while i < DIST_MODEL_START as uint32_t {
            (*coder).dist_prices[dist_state as usize][i as usize] =
                *dist_slot_prices.offset(i as isize);
            i = i.wrapping_add(1);
        }
        dist_state = dist_state.wrapping_add(1);
    }
    let mut i_0: uint32_t = DIST_MODEL_START as uint32_t;
    while i_0 < FULL_DISTANCES as uint32_t {
        let dist_slot_1: uint32_t = get_dist_slot(i_0) as uint32_t;
        let footer_bits: uint32_t =
            (dist_slot_1 >> 1 as ::core::ffi::c_int).wrapping_sub(1 as uint32_t);
        let base: uint32_t = (2 as uint32_t | dist_slot_1 & 1 as uint32_t) << footer_bits;
        let price: uint32_t = rc_bittree_reverse_price(
            (&raw mut (*coder).dist_special as *mut probability)
                .offset(base as isize)
                .offset(-(dist_slot_1 as isize))
                .offset(-(1 as ::core::ffi::c_int as isize)),
            footer_bits,
            i_0.wrapping_sub(base),
        ) as uint32_t;
        let mut dist_state_0: uint32_t = 0 as uint32_t;
        while dist_state_0 < DIST_STATES as uint32_t {
            (*coder).dist_prices[dist_state_0 as usize][i_0 as usize] = price.wrapping_add(
                (*coder).dist_slot_prices[dist_state_0 as usize][dist_slot_1 as usize],
            );
            dist_state_0 = dist_state_0.wrapping_add(1);
        }
        i_0 = i_0.wrapping_add(1);
    }
    (*coder).match_price_count = 0 as uint32_t;
}
unsafe extern "C" fn fill_align_prices(mut coder: *mut lzma_lzma1_encoder) {
    let mut i: uint32_t = 0 as uint32_t;
    while i < ALIGN_SIZE as uint32_t {
        (*coder).align_prices[i as usize] = rc_bittree_reverse_price(
            &raw mut (*coder).dist_align as *mut probability,
            ALIGN_BITS as uint32_t,
            i,
        );
        i = i.wrapping_add(1);
    }
    (*coder).align_price_count = 0 as uint32_t;
}
#[inline]
unsafe extern "C" fn make_literal(mut optimal: *mut lzma_optimal) {
    (*optimal).back_prev = UINT32_MAX as uint32_t;
    (*optimal).prev_1_is_literal = false_0 != 0;
}
#[inline]
unsafe extern "C" fn make_short_rep(mut optimal: *mut lzma_optimal) {
    (*optimal).back_prev = 0 as uint32_t;
    (*optimal).prev_1_is_literal = false_0 != 0;
}
unsafe extern "C" fn backward(
    mut coder: *mut lzma_lzma1_encoder,
    mut len_res: *mut uint32_t,
    mut back_res: *mut uint32_t,
    mut cur: uint32_t,
) {
    (*coder).opts_end_index = cur;
    let mut pos_mem: uint32_t = (*coder).opts[cur as usize].pos_prev;
    let mut back_mem: uint32_t = (*coder).opts[cur as usize].back_prev;
    loop {
        if (*coder).opts[cur as usize].prev_1_is_literal {
            make_literal(
                (&raw mut (*coder).opts as *mut lzma_optimal).offset(pos_mem as isize)
                    as *mut lzma_optimal,
            );
            (*coder).opts[pos_mem as usize].pos_prev = pos_mem.wrapping_sub(1 as uint32_t);
            if (*coder).opts[cur as usize].prev_2 {
                (*coder).opts[pos_mem.wrapping_sub(1 as uint32_t) as usize].prev_1_is_literal =
                    false_0 != 0;
                (*coder).opts[pos_mem.wrapping_sub(1 as uint32_t) as usize].pos_prev =
                    (*coder).opts[cur as usize].pos_prev_2;
                (*coder).opts[pos_mem.wrapping_sub(1 as uint32_t) as usize].back_prev =
                    (*coder).opts[cur as usize].back_prev_2;
            }
        }
        let pos_prev: uint32_t = pos_mem;
        let back_cur: uint32_t = back_mem;
        back_mem = (*coder).opts[pos_prev as usize].back_prev;
        pos_mem = (*coder).opts[pos_prev as usize].pos_prev;
        (*coder).opts[pos_prev as usize].back_prev = back_cur;
        (*coder).opts[pos_prev as usize].pos_prev = cur;
        cur = pos_prev;
        if !(cur != 0 as uint32_t) {
            break;
        }
    }
    (*coder).opts_current_index = (*coder).opts[0 as ::core::ffi::c_int as usize].pos_prev;
    *len_res = (*coder).opts[0 as ::core::ffi::c_int as usize].pos_prev;
    *back_res = (*coder).opts[0 as ::core::ffi::c_int as usize].back_prev;
}
#[inline]
unsafe extern "C" fn helper1(
    mut coder: *mut lzma_lzma1_encoder,
    mut mf: *mut lzma_mf,
    mut back_res: *mut uint32_t,
    mut len_res: *mut uint32_t,
    mut position: uint32_t,
) -> uint32_t {
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
        return UINT32_MAX as uint32_t;
    }
    let buf: *const uint8_t = mf_ptr(mf).offset(-(1 as ::core::ffi::c_int as isize));
    let mut rep_lens: [uint32_t; 4] = [0; 4];
    let mut rep_max_index: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < REPS as uint32_t {
        let buf_back: *const uint8_t = buf
            .offset(-((*coder).reps[i as usize] as isize))
            .offset(-(1 as ::core::ffi::c_int as isize));
        if *buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != *buf_back.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            || *buf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != *buf_back.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        {
            rep_lens[i as usize] = 0 as uint32_t;
        } else {
            rep_lens[i as usize] = lzma_memcmplen(buf, buf_back, 2 as uint32_t, buf_avail);
            if rep_lens[i as usize] > rep_lens[rep_max_index as usize] {
                rep_max_index = i;
            }
        }
        i = i.wrapping_add(1);
    }
    if rep_lens[rep_max_index as usize] >= nice_len {
        *back_res = rep_max_index;
        *len_res = rep_lens[rep_max_index as usize];
        mf_skip(mf, (*len_res).wrapping_sub(1 as uint32_t));
        return UINT32_MAX as uint32_t;
    }
    if len_main >= nice_len {
        *back_res = (*coder).matches[matches_count.wrapping_sub(1 as uint32_t) as usize]
            .dist
            .wrapping_add(REPS as uint32_t);
        *len_res = len_main;
        mf_skip(mf, len_main.wrapping_sub(1 as uint32_t));
        return UINT32_MAX as uint32_t;
    }
    let current_byte: uint8_t = *buf;
    let match_byte: uint8_t = *buf
        .offset(-((*coder).reps[0 as ::core::ffi::c_int as usize] as isize))
        .offset(-(1 as ::core::ffi::c_int as isize));
    if len_main < 2 as uint32_t
        && current_byte as ::core::ffi::c_int != match_byte as ::core::ffi::c_int
        && rep_lens[rep_max_index as usize] < 2 as uint32_t
    {
        *back_res = UINT32_MAX as uint32_t;
        *len_res = 1 as uint32_t;
        return UINT32_MAX as uint32_t;
    }
    (*coder).opts[0 as ::core::ffi::c_int as usize].state = (*coder).state;
    let pos_state: uint32_t = position & (*coder).pos_mask;
    (*coder).opts[1 as ::core::ffi::c_int as usize].price =
        rc_bit_0_price((*coder).is_match[(*coder).state as usize][pos_state as usize])
            .wrapping_add(get_literal_price(
                coder,
                position,
                *buf.offset(-(1 as ::core::ffi::c_int) as isize) as uint32_t,
                !(((*coder).state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint),
                match_byte as uint32_t,
                current_byte as uint32_t,
            ));
    make_literal(
        (&raw mut (*coder).opts as *mut lzma_optimal).offset(1 as ::core::ffi::c_int as isize)
            as *mut lzma_optimal,
    );
    let match_price: uint32_t =
        rc_bit_1_price((*coder).is_match[(*coder).state as usize][pos_state as usize]) as uint32_t;
    let rep_match_price: uint32_t = match_price
        .wrapping_add(rc_bit_1_price((*coder).is_rep[(*coder).state as usize]) as uint32_t);
    if match_byte as ::core::ffi::c_int == current_byte as ::core::ffi::c_int {
        let short_rep_price: uint32_t =
            rep_match_price
                .wrapping_add(get_short_rep_price(coder, (*coder).state, pos_state) as uint32_t);
        if short_rep_price < (*coder).opts[1 as ::core::ffi::c_int as usize].price {
            (*coder).opts[1 as ::core::ffi::c_int as usize].price = short_rep_price;
            make_short_rep(
                (&raw mut (*coder).opts as *mut lzma_optimal)
                    .offset(1 as ::core::ffi::c_int as isize) as *mut lzma_optimal,
            );
        }
    }
    let len_end: uint32_t = if len_main > rep_lens[rep_max_index as usize] {
        len_main
    } else {
        rep_lens[rep_max_index as usize]
    };
    if len_end < 2 as uint32_t {
        *back_res = (*coder).opts[1 as ::core::ffi::c_int as usize].back_prev;
        *len_res = 1 as uint32_t;
        return UINT32_MAX as uint32_t;
    }
    (*coder).opts[1 as ::core::ffi::c_int as usize].pos_prev = 0 as uint32_t;
    let mut i_0: uint32_t = 0 as uint32_t;
    while i_0 < REPS as uint32_t {
        (*coder).opts[0 as ::core::ffi::c_int as usize].backs[i_0 as usize] =
            (*coder).reps[i_0 as usize];
        i_0 = i_0.wrapping_add(1);
    }
    let mut len: uint32_t = len_end;
    loop {
        (*coder).opts[len as usize].price = RC_INFINITY_PRICE as uint32_t;
        len = len.wrapping_sub(1);
        if !(len >= 2 as uint32_t) {
            break;
        }
    }
    let mut i_1: uint32_t = 0 as uint32_t;
    while i_1 < REPS as uint32_t {
        let mut rep_len: uint32_t = rep_lens[i_1 as usize];
        if !(rep_len < 2 as uint32_t) {
            let price: uint32_t = rep_match_price.wrapping_add(get_pure_rep_price(
                coder,
                i_1,
                (*coder).state,
                pos_state,
            ) as uint32_t);
            loop {
                let cur_and_len_price: uint32_t = price.wrapping_add(get_len_price(
                    &raw mut (*coder).rep_len_encoder,
                    rep_len,
                    pos_state,
                ) as uint32_t);
                if cur_and_len_price < (*coder).opts[rep_len as usize].price {
                    (*coder).opts[rep_len as usize].price = cur_and_len_price;
                    (*coder).opts[rep_len as usize].pos_prev = 0 as uint32_t;
                    (*coder).opts[rep_len as usize].back_prev = i_1;
                    (*coder).opts[rep_len as usize].prev_1_is_literal = false_0 != 0;
                }
                rep_len = rep_len.wrapping_sub(1);
                if !(rep_len >= 2 as uint32_t) {
                    break;
                }
            }
        }
        i_1 = i_1.wrapping_add(1);
    }
    let normal_match_price: uint32_t = match_price
        .wrapping_add(rc_bit_0_price((*coder).is_rep[(*coder).state as usize]) as uint32_t);
    len = if rep_lens[0 as ::core::ffi::c_int as usize] >= 2 as uint32_t {
        rep_lens[0 as ::core::ffi::c_int as usize].wrapping_add(1 as uint32_t)
    } else {
        2 as uint32_t
    };
    if len <= len_main {
        let mut i_2: uint32_t = 0 as uint32_t;
        while len > (*coder).matches[i_2 as usize].len {
            i_2 = i_2.wrapping_add(1);
        }
        loop {
            let dist: uint32_t = (*coder).matches[i_2 as usize].dist;
            let cur_and_len_price_0: uint32_t = normal_match_price
                .wrapping_add(get_dist_len_price(coder, dist, len, pos_state) as uint32_t);
            if cur_and_len_price_0 < (*coder).opts[len as usize].price {
                (*coder).opts[len as usize].price = cur_and_len_price_0;
                (*coder).opts[len as usize].pos_prev = 0 as uint32_t;
                (*coder).opts[len as usize].back_prev = dist.wrapping_add(REPS as uint32_t);
                (*coder).opts[len as usize].prev_1_is_literal = false_0 != 0;
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
    mut coder: *mut lzma_lzma1_encoder,
    mut reps: *mut uint32_t,
    mut buf: *const uint8_t,
    mut len_end: uint32_t,
    mut position: uint32_t,
    cur: uint32_t,
    nice_len: uint32_t,
    buf_avail_full: uint32_t,
) -> uint32_t {
    let mut matches_count: uint32_t = (*coder).matches_count;
    let mut new_len: uint32_t = (*coder).longest_match_length;
    let mut pos_prev: uint32_t = (*coder).opts[cur as usize].pos_prev;
    let mut state: lzma_lzma_state = STATE_LIT_LIT;
    if (*coder).opts[cur as usize].prev_1_is_literal {
        pos_prev = pos_prev.wrapping_sub(1);
        if (*coder).opts[cur as usize].prev_2 {
            state = (*coder).opts[(*coder).opts[cur as usize].pos_prev_2 as usize].state;
            if (*coder).opts[cur as usize].back_prev_2 < REPS as uint32_t {
                state = (if (state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint {
                    STATE_LIT_LONGREP as ::core::ffi::c_int
                } else {
                    STATE_NONLIT_REP as ::core::ffi::c_int
                }) as lzma_lzma_state;
            } else {
                state = (if (state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint {
                    STATE_LIT_MATCH as ::core::ffi::c_int
                } else {
                    STATE_NONLIT_MATCH as ::core::ffi::c_int
                }) as lzma_lzma_state;
            }
        } else {
            state = (*coder).opts[pos_prev as usize].state;
        }
        state = (if state as ::core::ffi::c_uint
            <= STATE_SHORTREP_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            STATE_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
        } else if state as ::core::ffi::c_uint
            <= STATE_LIT_SHORTREP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (state as ::core::ffi::c_uint).wrapping_sub(3 as ::core::ffi::c_uint)
        } else {
            (state as ::core::ffi::c_uint).wrapping_sub(6 as ::core::ffi::c_uint)
        }) as lzma_lzma_state;
    } else {
        state = (*coder).opts[pos_prev as usize].state;
    }
    if pos_prev == cur.wrapping_sub(1 as uint32_t) {
        if (*coder).opts[cur as usize].back_prev == 0 as uint32_t {
            state = (if (state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint {
                STATE_LIT_SHORTREP as ::core::ffi::c_int
            } else {
                STATE_NONLIT_REP as ::core::ffi::c_int
            }) as lzma_lzma_state;
        } else {
            state = (if state as ::core::ffi::c_uint
                <= STATE_SHORTREP_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                STATE_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
            } else if state as ::core::ffi::c_uint
                <= STATE_LIT_SHORTREP as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (state as ::core::ffi::c_uint).wrapping_sub(3 as ::core::ffi::c_uint)
            } else {
                (state as ::core::ffi::c_uint).wrapping_sub(6 as ::core::ffi::c_uint)
            }) as lzma_lzma_state;
        }
    } else {
        let mut pos: uint32_t = 0;
        if (*coder).opts[cur as usize].prev_1_is_literal as ::core::ffi::c_int != 0
            && (*coder).opts[cur as usize].prev_2 as ::core::ffi::c_int != 0
        {
            pos_prev = (*coder).opts[cur as usize].pos_prev_2;
            pos = (*coder).opts[cur as usize].back_prev_2;
            state = (if (state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint {
                STATE_LIT_LONGREP as ::core::ffi::c_int
            } else {
                STATE_NONLIT_REP as ::core::ffi::c_int
            }) as lzma_lzma_state;
        } else {
            pos = (*coder).opts[cur as usize].back_prev;
            if pos < REPS as uint32_t {
                state = (if (state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint {
                    STATE_LIT_LONGREP as ::core::ffi::c_int
                } else {
                    STATE_NONLIT_REP as ::core::ffi::c_int
                }) as lzma_lzma_state;
            } else {
                state = (if (state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint {
                    STATE_LIT_MATCH as ::core::ffi::c_int
                } else {
                    STATE_NONLIT_MATCH as ::core::ffi::c_int
                }) as lzma_lzma_state;
            }
        }
        if pos < REPS as uint32_t {
            *reps.offset(0 as ::core::ffi::c_int as isize) =
                (*coder).opts[pos_prev as usize].backs[pos as usize];
            let mut i: uint32_t = 0;
            i = 1 as uint32_t;
            while i <= pos {
                *reps.offset(i as isize) =
                    (*coder).opts[pos_prev as usize].backs[i.wrapping_sub(1 as uint32_t) as usize];
                i = i.wrapping_add(1);
            }
            while i < REPS as uint32_t {
                *reps.offset(i as isize) = (*coder).opts[pos_prev as usize].backs[i as usize];
                i = i.wrapping_add(1);
            }
        } else {
            *reps.offset(0 as ::core::ffi::c_int as isize) = pos.wrapping_sub(REPS as uint32_t);
            let mut i_0: uint32_t = 1 as uint32_t;
            while i_0 < REPS as uint32_t {
                *reps.offset(i_0 as isize) = (*coder).opts[pos_prev as usize].backs
                    [i_0.wrapping_sub(1 as uint32_t) as usize];
                i_0 = i_0.wrapping_add(1);
            }
        }
    }
    (*coder).opts[cur as usize].state = state;
    let mut i_1: uint32_t = 0 as uint32_t;
    while i_1 < REPS as uint32_t {
        (*coder).opts[cur as usize].backs[i_1 as usize] = *reps.offset(i_1 as isize);
        i_1 = i_1.wrapping_add(1);
    }
    let cur_price: uint32_t = (*coder).opts[cur as usize].price;
    let current_byte: uint8_t = *buf;
    let match_byte: uint8_t = *buf
        .offset(-(*reps.offset(0 as ::core::ffi::c_int as isize) as isize))
        .offset(-(1 as ::core::ffi::c_int as isize));
    let pos_state: uint32_t = position & (*coder).pos_mask;
    let cur_and_1_price: uint32_t = cur_price
        .wrapping_add(
            rc_bit_0_price((*coder).is_match[state as usize][pos_state as usize]) as uint32_t,
        )
        .wrapping_add(get_literal_price(
            coder,
            position,
            *buf.offset(-(1 as ::core::ffi::c_int) as isize) as uint32_t,
            !((state as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint),
            match_byte as uint32_t,
            current_byte as uint32_t,
        ) as uint32_t);
    let mut next_is_literal: bool = false_0 != 0;
    if cur_and_1_price < (*coder).opts[cur.wrapping_add(1 as uint32_t) as usize].price {
        (*coder).opts[cur.wrapping_add(1 as uint32_t) as usize].price = cur_and_1_price;
        (*coder).opts[cur.wrapping_add(1 as uint32_t) as usize].pos_prev = cur;
        make_literal(
            (&raw mut (*coder).opts as *mut lzma_optimal)
                .offset(cur.wrapping_add(1 as uint32_t) as isize) as *mut lzma_optimal,
        );
        next_is_literal = true_0 != 0;
    }
    let match_price: uint32_t = cur_price.wrapping_add(rc_bit_1_price(
        (*coder).is_match[state as usize][pos_state as usize],
    ) as uint32_t);
    let rep_match_price: uint32_t =
        match_price.wrapping_add(rc_bit_1_price((*coder).is_rep[state as usize]) as uint32_t);
    if match_byte as ::core::ffi::c_int == current_byte as ::core::ffi::c_int
        && !((*coder).opts[cur.wrapping_add(1 as uint32_t) as usize].pos_prev < cur
            && (*coder).opts[cur.wrapping_add(1 as uint32_t) as usize].back_prev == 0 as uint32_t)
    {
        let short_rep_price: uint32_t =
            rep_match_price.wrapping_add(get_short_rep_price(coder, state, pos_state) as uint32_t);
        if short_rep_price <= (*coder).opts[cur.wrapping_add(1 as uint32_t) as usize].price {
            (*coder).opts[cur.wrapping_add(1 as uint32_t) as usize].price = short_rep_price;
            (*coder).opts[cur.wrapping_add(1 as uint32_t) as usize].pos_prev = cur;
            make_short_rep(
                (&raw mut (*coder).opts as *mut lzma_optimal)
                    .offset(cur.wrapping_add(1 as uint32_t) as isize)
                    as *mut lzma_optimal,
            );
            next_is_literal = true_0 != 0;
        }
    }
    if buf_avail_full < 2 as uint32_t {
        return len_end;
    }
    let buf_avail: uint32_t = if buf_avail_full < nice_len {
        buf_avail_full
    } else {
        nice_len
    };
    if !next_is_literal && match_byte as ::core::ffi::c_int != current_byte as ::core::ffi::c_int {
        let buf_back: *const uint8_t = buf
            .offset(-(*reps.offset(0 as ::core::ffi::c_int as isize) as isize))
            .offset(-(1 as ::core::ffi::c_int as isize));
        let limit: uint32_t = if buf_avail_full < nice_len.wrapping_add(1 as uint32_t) {
            buf_avail_full
        } else {
            nice_len.wrapping_add(1 as uint32_t)
        };
        let len_test: uint32_t = (lzma_memcmplen(buf, buf_back, 1 as uint32_t, limit) as uint32_t)
            .wrapping_sub(1 as uint32_t);
        if len_test >= 2 as uint32_t {
            let mut state_2: lzma_lzma_state = state;
            state_2 = (if state_2 as ::core::ffi::c_uint
                <= STATE_SHORTREP_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                STATE_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
            } else if state_2 as ::core::ffi::c_uint
                <= STATE_LIT_SHORTREP as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (state_2 as ::core::ffi::c_uint).wrapping_sub(3 as ::core::ffi::c_uint)
            } else {
                (state_2 as ::core::ffi::c_uint).wrapping_sub(6 as ::core::ffi::c_uint)
            }) as lzma_lzma_state;
            let pos_state_next: uint32_t = position.wrapping_add(1 as uint32_t) & (*coder).pos_mask;
            let next_rep_match_price: uint32_t = cur_and_1_price
                .wrapping_add(rc_bit_1_price(
                    (*coder).is_match[state_2 as usize][pos_state_next as usize],
                ) as uint32_t)
                .wrapping_add(rc_bit_1_price((*coder).is_rep[state_2 as usize]) as uint32_t);
            let offset: uint32_t = cur.wrapping_add(1 as uint32_t).wrapping_add(len_test);
            while len_end < offset {
                len_end = len_end.wrapping_add(1);
                (*coder).opts[len_end as usize].price = RC_INFINITY_PRICE as uint32_t;
            }
            let cur_and_len_price: uint32_t = next_rep_match_price.wrapping_add(get_rep_price(
                coder,
                0 as uint32_t,
                len_test,
                state_2,
                pos_state_next,
            )
                as uint32_t);
            if cur_and_len_price < (*coder).opts[offset as usize].price {
                (*coder).opts[offset as usize].price = cur_and_len_price;
                (*coder).opts[offset as usize].pos_prev = cur.wrapping_add(1 as uint32_t);
                (*coder).opts[offset as usize].back_prev = 0 as uint32_t;
                (*coder).opts[offset as usize].prev_1_is_literal = true_0 != 0;
                (*coder).opts[offset as usize].prev_2 = false_0 != 0;
            }
        }
    }
    let mut start_len: uint32_t = 2 as uint32_t;
    let mut rep_index: uint32_t = 0 as uint32_t;
    while rep_index < REPS as uint32_t {
        let buf_back_0: *const uint8_t = buf
            .offset(-(*reps.offset(rep_index as isize) as isize))
            .offset(-(1 as ::core::ffi::c_int as isize));
        if !(*buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != *buf_back_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            || *buf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != *buf_back_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        {
            let mut len_test_0: uint32_t =
                lzma_memcmplen(buf, buf_back_0, 2 as uint32_t, buf_avail);
            while len_end < cur.wrapping_add(len_test_0) {
                len_end = len_end.wrapping_add(1);
                (*coder).opts[len_end as usize].price = RC_INFINITY_PRICE as uint32_t;
            }
            let len_test_temp: uint32_t = len_test_0;
            let price: uint32_t = rep_match_price
                .wrapping_add(get_pure_rep_price(coder, rep_index, state, pos_state) as uint32_t);
            loop {
                let cur_and_len_price_0: uint32_t = price.wrapping_add(get_len_price(
                    &raw mut (*coder).rep_len_encoder,
                    len_test_0,
                    pos_state,
                )
                    as uint32_t);
                if cur_and_len_price_0 < (*coder).opts[cur.wrapping_add(len_test_0) as usize].price
                {
                    (*coder).opts[cur.wrapping_add(len_test_0) as usize].price =
                        cur_and_len_price_0;
                    (*coder).opts[cur.wrapping_add(len_test_0) as usize].pos_prev = cur;
                    (*coder).opts[cur.wrapping_add(len_test_0) as usize].back_prev = rep_index;
                    (*coder).opts[cur.wrapping_add(len_test_0) as usize].prev_1_is_literal =
                        false_0 != 0;
                }
                len_test_0 = len_test_0.wrapping_sub(1);
                if !(len_test_0 >= 2 as uint32_t) {
                    break;
                }
            }
            len_test_0 = len_test_temp;
            if rep_index == 0 as uint32_t {
                start_len = len_test_0.wrapping_add(1 as uint32_t);
            }
            let mut len_test_2: uint32_t = len_test_0.wrapping_add(1 as uint32_t);
            let limit_0: uint32_t = if buf_avail_full < len_test_2.wrapping_add(nice_len) {
                buf_avail_full
            } else {
                len_test_2.wrapping_add(nice_len)
            };
            if len_test_2 < limit_0 {
                len_test_2 = lzma_memcmplen(buf, buf_back_0, len_test_2, limit_0);
            }
            len_test_2 = len_test_2.wrapping_sub(len_test_0.wrapping_add(1 as uint32_t));
            if len_test_2 >= 2 as uint32_t {
                let mut state_2_0: lzma_lzma_state = state;
                state_2_0 =
                    (if (state_2_0 as ::core::ffi::c_uint) < LIT_STATES as ::core::ffi::c_uint {
                        STATE_LIT_LONGREP as ::core::ffi::c_int
                    } else {
                        STATE_NONLIT_REP as ::core::ffi::c_int
                    }) as lzma_lzma_state;
                let mut pos_state_next_0: uint32_t =
                    position.wrapping_add(len_test_0) & (*coder).pos_mask;
                let cur_and_len_literal_price: uint32_t = price
                    .wrapping_add(get_len_price(
                        &raw mut (*coder).rep_len_encoder,
                        len_test_0,
                        pos_state,
                    ) as uint32_t)
                    .wrapping_add(rc_bit_0_price(
                        (*coder).is_match[state_2_0 as usize][pos_state_next_0 as usize],
                    ) as uint32_t)
                    .wrapping_add(get_literal_price(
                        coder,
                        position.wrapping_add(len_test_0),
                        *buf.offset(len_test_0.wrapping_sub(1 as uint32_t) as isize) as uint32_t,
                        true_0 != 0,
                        *buf_back_0.offset(len_test_0 as isize) as uint32_t,
                        *buf.offset(len_test_0 as isize) as uint32_t,
                    ) as uint32_t);
                state_2_0 = (if state_2_0 as ::core::ffi::c_uint
                    <= STATE_SHORTREP_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    STATE_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
                } else if state_2_0 as ::core::ffi::c_uint
                    <= STATE_LIT_SHORTREP as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (state_2_0 as ::core::ffi::c_uint).wrapping_sub(3 as ::core::ffi::c_uint)
                } else {
                    (state_2_0 as ::core::ffi::c_uint).wrapping_sub(6 as ::core::ffi::c_uint)
                }) as lzma_lzma_state;
                pos_state_next_0 = position
                    .wrapping_add(len_test_0)
                    .wrapping_add(1 as uint32_t)
                    & (*coder).pos_mask;
                let next_rep_match_price_0: uint32_t = cur_and_len_literal_price
                    .wrapping_add(rc_bit_1_price(
                        (*coder).is_match[state_2_0 as usize][pos_state_next_0 as usize],
                    ) as uint32_t)
                    .wrapping_add(rc_bit_1_price((*coder).is_rep[state_2_0 as usize]) as uint32_t);
                let offset_0: uint32_t = cur
                    .wrapping_add(len_test_0)
                    .wrapping_add(1 as uint32_t)
                    .wrapping_add(len_test_2);
                while len_end < offset_0 {
                    len_end = len_end.wrapping_add(1);
                    (*coder).opts[len_end as usize].price = RC_INFINITY_PRICE as uint32_t;
                }
                let cur_and_len_price_1: uint32_t =
                    next_rep_match_price_0.wrapping_add(get_rep_price(
                        coder,
                        0 as uint32_t,
                        len_test_2,
                        state_2_0,
                        pos_state_next_0,
                    ) as uint32_t);
                if cur_and_len_price_1 < (*coder).opts[offset_0 as usize].price {
                    (*coder).opts[offset_0 as usize].price = cur_and_len_price_1;
                    (*coder).opts[offset_0 as usize].pos_prev =
                        cur.wrapping_add(len_test_0).wrapping_add(1 as uint32_t);
                    (*coder).opts[offset_0 as usize].back_prev = 0 as uint32_t;
                    (*coder).opts[offset_0 as usize].prev_1_is_literal = true_0 != 0;
                    (*coder).opts[offset_0 as usize].prev_2 = true_0 != 0;
                    (*coder).opts[offset_0 as usize].pos_prev_2 = cur;
                    (*coder).opts[offset_0 as usize].back_prev_2 = rep_index;
                }
            }
        }
        rep_index = rep_index.wrapping_add(1);
    }
    if new_len > buf_avail {
        new_len = buf_avail;
        matches_count = 0 as uint32_t;
        while new_len > (*coder).matches[matches_count as usize].len {
            matches_count = matches_count.wrapping_add(1);
        }
        let fresh0 = matches_count;
        matches_count = matches_count.wrapping_add(1);
        (*coder).matches[fresh0 as usize].len = new_len;
    }
    if new_len >= start_len {
        let normal_match_price: uint32_t =
            match_price.wrapping_add(rc_bit_0_price((*coder).is_rep[state as usize]) as uint32_t);
        while len_end < cur.wrapping_add(new_len) {
            len_end = len_end.wrapping_add(1);
            (*coder).opts[len_end as usize].price = RC_INFINITY_PRICE as uint32_t;
        }
        let mut i_2: uint32_t = 0 as uint32_t;
        while start_len > (*coder).matches[i_2 as usize].len {
            i_2 = i_2.wrapping_add(1);
        }
        let mut len_test_1: uint32_t = start_len;
        loop {
            let cur_back: uint32_t = (*coder).matches[i_2 as usize].dist;
            let mut cur_and_len_price_2: uint32_t = normal_match_price
                .wrapping_add(get_dist_len_price(coder, cur_back, len_test_1, pos_state));
            if cur_and_len_price_2 < (*coder).opts[cur.wrapping_add(len_test_1) as usize].price {
                (*coder).opts[cur.wrapping_add(len_test_1) as usize].price = cur_and_len_price_2;
                (*coder).opts[cur.wrapping_add(len_test_1) as usize].pos_prev = cur;
                (*coder).opts[cur.wrapping_add(len_test_1) as usize].back_prev =
                    cur_back.wrapping_add(REPS as uint32_t);
                (*coder).opts[cur.wrapping_add(len_test_1) as usize].prev_1_is_literal =
                    false_0 != 0;
            }
            if len_test_1 == (*coder).matches[i_2 as usize].len {
                let buf_back_1: *const uint8_t = buf
                    .offset(-(cur_back as isize))
                    .offset(-(1 as ::core::ffi::c_int as isize));
                let mut len_test_2_0: uint32_t = len_test_1.wrapping_add(1 as uint32_t);
                let limit_1: uint32_t = if buf_avail_full < len_test_2_0.wrapping_add(nice_len) {
                    buf_avail_full
                } else {
                    len_test_2_0.wrapping_add(nice_len)
                };
                if len_test_2_0 < limit_1 {
                    len_test_2_0 = lzma_memcmplen(buf, buf_back_1, len_test_2_0, limit_1);
                }
                len_test_2_0 = len_test_2_0.wrapping_sub(len_test_1.wrapping_add(1 as uint32_t));
                if len_test_2_0 >= 2 as uint32_t {
                    let mut state_2_1: lzma_lzma_state = state;
                    state_2_1 = (if (state_2_1 as ::core::ffi::c_uint)
                        < LIT_STATES as ::core::ffi::c_uint
                    {
                        STATE_LIT_MATCH as ::core::ffi::c_int
                    } else {
                        STATE_NONLIT_MATCH as ::core::ffi::c_int
                    }) as lzma_lzma_state;
                    let mut pos_state_next_1: uint32_t =
                        position.wrapping_add(len_test_1) & (*coder).pos_mask;
                    let cur_and_len_literal_price_0: uint32_t = cur_and_len_price_2
                        .wrapping_add(rc_bit_0_price(
                            (*coder).is_match[state_2_1 as usize][pos_state_next_1 as usize],
                        ) as uint32_t)
                        .wrapping_add(get_literal_price(
                            coder,
                            position.wrapping_add(len_test_1),
                            *buf.offset(len_test_1.wrapping_sub(1 as uint32_t) as isize)
                                as uint32_t,
                            true_0 != 0,
                            *buf_back_1.offset(len_test_1 as isize) as uint32_t,
                            *buf.offset(len_test_1 as isize) as uint32_t,
                        ) as uint32_t);
                    state_2_1 = (if state_2_1 as ::core::ffi::c_uint
                        <= STATE_SHORTREP_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        STATE_LIT_LIT as ::core::ffi::c_int as ::core::ffi::c_uint
                    } else if state_2_1 as ::core::ffi::c_uint
                        <= STATE_LIT_SHORTREP as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (state_2_1 as ::core::ffi::c_uint).wrapping_sub(3 as ::core::ffi::c_uint)
                    } else {
                        (state_2_1 as ::core::ffi::c_uint).wrapping_sub(6 as ::core::ffi::c_uint)
                    }) as lzma_lzma_state;
                    pos_state_next_1 =
                        pos_state_next_1.wrapping_add(1 as uint32_t) & (*coder).pos_mask;
                    let next_rep_match_price_1: uint32_t = cur_and_len_literal_price_0
                        .wrapping_add(rc_bit_1_price(
                            (*coder).is_match[state_2_1 as usize][pos_state_next_1 as usize],
                        ) as uint32_t)
                        .wrapping_add(
                            rc_bit_1_price((*coder).is_rep[state_2_1 as usize]) as uint32_t
                        );
                    let offset_1: uint32_t = cur
                        .wrapping_add(len_test_1)
                        .wrapping_add(1 as uint32_t)
                        .wrapping_add(len_test_2_0);
                    while len_end < offset_1 {
                        len_end = len_end.wrapping_add(1);
                        (*coder).opts[len_end as usize].price = RC_INFINITY_PRICE as uint32_t;
                    }
                    cur_and_len_price_2 = next_rep_match_price_1.wrapping_add(get_rep_price(
                        coder,
                        0 as uint32_t,
                        len_test_2_0,
                        state_2_1,
                        pos_state_next_1,
                    ));
                    if cur_and_len_price_2 < (*coder).opts[offset_1 as usize].price {
                        (*coder).opts[offset_1 as usize].price = cur_and_len_price_2;
                        (*coder).opts[offset_1 as usize].pos_prev =
                            cur.wrapping_add(len_test_1).wrapping_add(1 as uint32_t);
                        (*coder).opts[offset_1 as usize].back_prev = 0 as uint32_t;
                        (*coder).opts[offset_1 as usize].prev_1_is_literal = true_0 != 0;
                        (*coder).opts[offset_1 as usize].prev_2 = true_0 != 0;
                        (*coder).opts[offset_1 as usize].pos_prev_2 = cur;
                        (*coder).opts[offset_1 as usize].back_prev_2 =
                            cur_back.wrapping_add(REPS as uint32_t);
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
    mut coder: *mut lzma_lzma1_encoder,
    mut mf: *mut lzma_mf,
    mut back_res: *mut uint32_t,
    mut len_res: *mut uint32_t,
    mut position: uint32_t,
) {
    if (*coder).opts_end_index != (*coder).opts_current_index {
        *len_res = (*coder).opts[(*coder).opts_current_index as usize]
            .pos_prev
            .wrapping_sub((*coder).opts_current_index);
        *back_res = (*coder).opts[(*coder).opts_current_index as usize].back_prev;
        (*coder).opts_current_index = (*coder).opts[(*coder).opts_current_index as usize].pos_prev;
        return;
    }
    if (*mf).read_ahead == 0 as uint32_t {
        if (*coder).match_price_count
            >= ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t
        {
            fill_dist_prices(coder);
        }
        if (*coder).align_price_count >= ALIGN_SIZE as uint32_t {
            fill_align_prices(coder);
        }
    }
    let mut len_end: uint32_t = helper1(coder, mf, back_res, len_res, position);
    if len_end == UINT32_MAX as uint32_t {
        return;
    }
    let mut reps: [uint32_t; 4] = [0; 4];
    memcpy(
        &raw mut reps as *mut uint32_t as *mut ::core::ffi::c_void,
        &raw mut (*coder).reps as *mut uint32_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint32_t; 4]>() as size_t,
    );
    let mut cur: uint32_t = 0;
    cur = 1 as uint32_t;
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
            &raw mut reps as *mut uint32_t,
            mf_ptr(mf).offset(-(1 as ::core::ffi::c_int as isize)),
            len_end,
            position.wrapping_add(cur),
            cur,
            (*mf).nice_len,
            if mf_avail(mf).wrapping_add(1 as uint32_t)
                < ((((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_sub(cur)
            {
                mf_avail(mf).wrapping_add(1 as uint32_t)
            } else {
                ((((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                    as uint32_t)
                    .wrapping_sub(cur)
            },
        );
        cur = cur.wrapping_add(1);
    }
    backward(coder, len_res, back_res, cur);
}
