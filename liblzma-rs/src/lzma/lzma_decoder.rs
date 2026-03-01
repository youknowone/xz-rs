use crate::types::*;
use core::ffi::{c_long, c_uint, c_void};
extern "C" {
    fn lzma_lz_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
        lz_init: Option<
            unsafe extern "C" fn(
                *mut lzma_lz_decoder,
                *const lzma_allocator,
                lzma_vli,
                *const c_void,
                *mut lzma_lz_options,
            ) -> lzma_ret,
        >,
    ) -> lzma_ret;
    fn lzma_lz_decoder_memusage(dictionary_size: size_t) -> u64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_options {
    pub dict_size: size_t,
    pub preset_dict: *const u8,
    pub preset_dict_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lzma1_decoder {
    pub literal: [probability; 12288],
    pub is_match: [[probability; 16]; 12],
    pub is_rep: [probability; 12],
    pub is_rep0: [probability; 12],
    pub is_rep1: [probability; 12],
    pub is_rep2: [probability; 12],
    pub is_rep0_long: [[probability; 16]; 12],
    pub dist_slot: [[probability; 64]; 4],
    pub pos_special: [probability; 114],
    pub pos_align: [probability; 16],
    pub match_len_decoder: lzma_length_decoder,
    pub rep_len_decoder: lzma_length_decoder,
    pub rc: lzma_range_decoder,
    pub state: lzma_lzma_state,
    pub rep0: u32,
    pub rep1: u32,
    pub rep2: u32,
    pub rep3: u32,
    pub pos_mask: u32,
    pub literal_context_bits: u32,
    pub literal_mask: u32,
    pub uncompressed_size: lzma_vli,
    pub allow_eopm: bool,
    pub sequence: C2RustUnnamed,
    pub probs: *mut probability,
    pub symbol: u32,
    pub limit: u32,
    pub offset: u32,
    pub len: u32,
}
pub type C2RustUnnamed = c_uint;
pub const SEQ_COPY: C2RustUnnamed = 22;
pub const SEQ_REP_LEN_BITTREE: C2RustUnnamed = 21;
pub const SEQ_REP_LEN_CHOICE2: C2RustUnnamed = 20;
pub const SEQ_REP_LEN_CHOICE: C2RustUnnamed = 19;
pub const SEQ_IS_REP2: C2RustUnnamed = 18;
pub const SEQ_IS_REP1: C2RustUnnamed = 17;
pub const SEQ_IS_REP0_LONG: C2RustUnnamed = 16;
pub const SEQ_SHORTREP: C2RustUnnamed = 15;
pub const SEQ_IS_REP0: C2RustUnnamed = 14;
pub const SEQ_EOPM: C2RustUnnamed = 13;
pub const SEQ_ALIGN: C2RustUnnamed = 12;
pub const SEQ_DIRECT: C2RustUnnamed = 11;
pub const SEQ_DIST_MODEL: C2RustUnnamed = 10;
pub const SEQ_DIST_SLOT: C2RustUnnamed = 9;
pub const SEQ_MATCH_LEN_BITTREE: C2RustUnnamed = 8;
pub const SEQ_MATCH_LEN_CHOICE2: C2RustUnnamed = 7;
pub const SEQ_MATCH_LEN_CHOICE: C2RustUnnamed = 6;
pub const SEQ_IS_REP: C2RustUnnamed = 5;
pub const SEQ_LITERAL_WRITE: C2RustUnnamed = 4;
pub const SEQ_LITERAL_MATCHED: C2RustUnnamed = 3;
pub const SEQ_LITERAL: C2RustUnnamed = 2;
pub const SEQ_IS_MATCH: C2RustUnnamed = 1;
pub const SEQ_NORMALIZE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_range_decoder {
    pub range: u32,
    pub code: u32,
    pub init_bytes_left: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_length_decoder {
    pub choice: probability,
    pub choice2: probability,
    pub low: [[probability; 8]; 16],
    pub mid: [[probability; 8]; 16],
    pub high: [probability; 256],
}
#[inline]
extern "C" fn read32le(buf: *const u8) -> u32 {
    return unsafe {
        let mut num: u32 = *buf.offset(0) as u32;
        num |= (*buf.offset(1) as u32) << 8;
        num |= (*buf.offset(2) as u32) << 16;
        num |= (*buf.offset(3) as u32) << 24;
        num
    };
}
pub const LZMA_LZMA1EXT_ALLOW_EOPM: c_uint = 0x1;
pub const LZ_DICT_REPEAT_MAX: u32 = 288;
pub const LZ_DICT_INIT_POS: u32 = 2 * LZ_DICT_REPEAT_MAX;
#[inline]
unsafe extern "C" fn dict_get(dict: *const lzma_dict, distance: u32) -> u8 {
    return *(*dict).buf.offset(
        (*dict)
            .pos
            .wrapping_sub(distance as size_t)
            .wrapping_sub(1)
            .wrapping_add(if (distance as size_t) < (*dict).pos {
                0
            } else {
                (*dict).size.wrapping_sub(LZ_DICT_REPEAT_MAX as size_t)
            }) as isize,
    );
}
#[inline]
unsafe extern "C" fn dict_get0(dict: *const lzma_dict) -> u8 {
    return *(*dict).buf.offset((*dict).pos.wrapping_sub(1) as isize);
}
#[inline]
unsafe extern "C" fn dict_is_distance_valid(dict: *const lzma_dict, distance: size_t) -> bool {
    return (*dict).full > distance;
}
#[inline]
unsafe extern "C" fn dict_repeat(dict: *mut lzma_dict, distance: u32, len: *mut u32) -> bool {
    let dict_avail: size_t = (*dict).limit.wrapping_sub((*dict).pos);
    let mut left: u32 = (if dict_avail < *len as size_t {
        dict_avail
    } else {
        *len as size_t
    }) as u32;
    *len = (*len).wrapping_sub(left);
    let mut back: size_t = (*dict).pos.wrapping_sub(distance as size_t).wrapping_sub(1);
    if distance as size_t >= (*dict).pos {
        back = back.wrapping_add((*dict).size.wrapping_sub(LZ_DICT_REPEAT_MAX as size_t));
    }
    if distance < left {
        loop {
            let fresh0 = back;
            back += 1;
            let fresh1 = (*dict).pos;
            (*dict).pos = (*dict).pos.wrapping_add(1);
            *(*dict).buf.offset(fresh1 as isize) = *(*dict).buf.offset(fresh0 as isize);
            left -= 1;
            if !(left > 0) {
                break;
            }
        }
    } else {
        memcpy(
            (*dict).buf.offset((*dict).pos as isize) as *mut c_void,
            (*dict).buf.offset(back as isize) as *const c_void,
            left as size_t,
        );
        (*dict).pos = (*dict).pos.wrapping_add(left as size_t);
    }
    if !(*dict).has_wrapped {
        (*dict).full = (*dict).pos.wrapping_sub(LZ_DICT_INIT_POS as size_t);
    }
    return *len != 0;
}
#[inline]
unsafe extern "C" fn dict_put(dict: *mut lzma_dict, byte: u8) {
    let fresh2 = (*dict).pos;
    (*dict).pos = (*dict).pos.wrapping_add(1);
    *(*dict).buf.offset(fresh2 as isize) = byte;
    if !(*dict).has_wrapped {
        (*dict).full = (*dict).pos.wrapping_sub(LZ_DICT_INIT_POS as size_t);
    }
}
#[inline]
unsafe extern "C" fn dict_put_safe(dict: *mut lzma_dict, byte: u8) -> bool {
    if ((*dict).pos == (*dict).limit) as c_long != 0 {
        return true;
    }
    dict_put(dict, byte);
    return false;
}
pub const RC_SHIFT_BITS: u32 = 8;
pub const RC_TOP_BITS: u32 = 24;
pub const RC_TOP_VALUE: c_uint = 1u32 << RC_TOP_BITS;
pub const RC_BIT_MODEL_TOTAL_BITS: u32 = 11;
pub const RC_BIT_MODEL_TOTAL: c_uint = 1u32 << RC_BIT_MODEL_TOTAL_BITS;
pub const RC_MOVE_BITS: u32 = 5;
#[inline]
unsafe extern "C" fn is_lclppb_valid(options: *const lzma_options_lzma) -> bool {
    return (*options).lc <= LZMA_LCLP_MAX
        && (*options).lp <= LZMA_LCLP_MAX
        && (*options).lc.wrapping_add((*options).lp) <= LZMA_LCLP_MAX
        && (*options).pb <= LZMA_PB_MAX;
}
pub const STATES: u32 = 12;
pub const LIT_STATES: u32 = 7;
pub const LITERAL_CODER_SIZE: c_uint = 0x300;
#[inline]
unsafe extern "C" fn literal_init(probs: *mut probability, lc: u32, lp: u32) {
    let coders: size_t = (LITERAL_CODER_SIZE << lc.wrapping_add(lp)) as size_t;
    let mut i: size_t = 0;
    while i < coders {
        *probs.offset(i as isize) = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        i += 1;
    }
}
pub const MATCH_LEN_MIN: u32 = 2;
pub const LEN_LOW_BITS: u32 = 3;
pub const LEN_LOW_SYMBOLS: u32 = (1) << LEN_LOW_BITS;
pub const LEN_MID_BITS: u32 = 3;
pub const LEN_MID_SYMBOLS: u32 = (1) << LEN_MID_BITS;
pub const LEN_HIGH_BITS: u32 = 8;
pub const LEN_HIGH_SYMBOLS: u32 = (1) << LEN_HIGH_BITS;
pub const DIST_STATES: u32 = 4;
pub const DIST_SLOT_BITS: u32 = 6;
pub const DIST_SLOTS: u32 = (1) << DIST_SLOT_BITS;
pub const DIST_MODEL_START: u32 = 4;
pub const DIST_MODEL_END: u32 = 14;
pub const FULL_DISTANCES_BITS: u32 = DIST_MODEL_END / 2;
pub const FULL_DISTANCES: u32 = 1 << FULL_DISTANCES_BITS;
pub const ALIGN_BITS: u32 = 4;
pub const ALIGN_SIZE: u32 = (1) << ALIGN_BITS;
#[inline]
unsafe extern "C" fn rc_read_init(
    rc: *mut lzma_range_decoder,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    while (*rc).init_bytes_left > 0 {
        if *in_pos == in_size {
            return LZMA_OK;
        }
        if (*rc).init_bytes_left == 5 && *in_0.offset(*in_pos as isize) != 0 {
            return LZMA_DATA_ERROR;
        }
        (*rc).code = (*rc).code << 8 | *in_0.offset(*in_pos as isize) as u32;
        *in_pos = (*in_pos).wrapping_add(1);
        (*rc).init_bytes_left = (*rc).init_bytes_left.wrapping_sub(1);
    }
    return LZMA_STREAM_END;
}
unsafe extern "C" fn lzma_decode(
    coder_ptr: *mut c_void,
    dictptr: *mut lzma_dict,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    let mut current_block: u64;
    let coder: *mut lzma_lzma1_decoder = coder_ptr as *mut lzma_lzma1_decoder;
    let ret: lzma_ret = rc_read_init(&raw mut (*coder).rc, in_0, in_pos, in_size);
    if ret != LZMA_STREAM_END {
        return ret;
    }
    let mut dict: lzma_dict = *dictptr;
    let dict_start: size_t = dict.pos;
    let mut rc: lzma_range_decoder = (*coder).rc;
    let mut rc_in_ptr: *const u8 = in_0.offset(*in_pos as isize);
    let rc_in_end: *const u8 = in_0.offset(in_size as isize);
    let rc_in_fast_end: *const u8 = if rc_in_end.offset_from(rc_in_ptr) as c_long <= 20 as c_long {
        rc_in_ptr
    } else {
        rc_in_end.offset(-20)
    };
    let mut rc_bound: u32 = 0;
    let mut state: u32 = (*coder).state as u32;
    let mut rep0: u32 = (*coder).rep0;
    let mut rep1: u32 = (*coder).rep1;
    let mut rep2: u32 = (*coder).rep2;
    let mut rep3: u32 = (*coder).rep3;
    let pos_mask: u32 = (*coder).pos_mask;
    let mut probs: *mut probability = (*coder).probs;
    let mut symbol: u32 = (*coder).symbol;
    let mut limit: u32 = (*coder).limit;
    let mut offset: u32 = (*coder).offset;
    let mut len: u32 = (*coder).len;
    let literal_mask: u32 = (*coder).literal_mask;
    let literal_context_bits: u32 = (*coder).literal_context_bits;
    let mut pos_state: u32 = (dict.pos & pos_mask as size_t) as u32;
    let mut ret_0: lzma_ret = LZMA_OK;
    let mut eopm_is_valid: bool = (*coder).uncompressed_size == LZMA_VLI_UNKNOWN;
    let mut might_finish_without_eopm: bool = false;
    if (*coder).uncompressed_size != LZMA_VLI_UNKNOWN
        && (*coder).uncompressed_size <= dict.limit.wrapping_sub(dict.pos) as lzma_vli
    {
        dict.limit = dict.pos.wrapping_add((*coder).uncompressed_size as size_t);
        might_finish_without_eopm = true;
    }
    match (*coder).sequence {
        0 | 1 => {
            current_block = 5979571030476392895;
        }
        2 => {
            current_block = 13844743919235296534;
        }
        3 => {
            current_block = 18125716024132132232;
        }
        4 => {
            current_block = 10535798129821001304;
        }
        5 => {
            current_block = 3469750012682708893;
        }
        6 => {
            current_block = 1138292997408115650;
        }
        7 => {
            current_block = 13912927785247575907;
        }
        8 => {
            current_block = 592696588731961849;
        }
        9 => {
            current_block = 4174862988780014241;
        }
        10 => {
            current_block = 617447976488552541;
        }
        11 => {
            current_block = 15418612220330286504;
        }
        12 => {
            current_block = 10510472849010538284;
        }
        13 => {
            current_block = 7073645523065812117;
        }
        14 => {
            current_block = 4420799852307653083;
        }
        16 => {
            current_block = 1698084742280242340;
        }
        15 => {
            current_block = 5341942013764523046;
        }
        17 => {
            current_block = 11808118301119257848;
        }
        18 => {
            current_block = 3996983927318648760;
        }
        19 => {
            current_block = 12043352250568755004;
        }
        20 => {
            current_block = 6834592846991627977;
        }
        21 => {
            current_block = 2467942631393454738;
        }
        22 => {
            current_block = 17340485688450593529;
        }
        _ => {
            current_block = 4609795085482299213;
        }
    }
    'c_9380: loop {
        match current_block {
            4609795085482299213 => {
                (*dictptr).pos = dict.pos;
                break;
            }
            12043352250568755004 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_REP_LEN_CHOICE;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh142 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh142 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).rep_len_decoder.choice as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).rep_len_decoder.choice = ((*coder).rep_len_decoder.choice as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub((*coder).rep_len_decoder.choice as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    probs = &raw mut *(&raw mut (*coder).rep_len_decoder.low
                        as *mut [probability; 8])
                        .offset(pos_state as isize) as *mut probability;
                    limit = LEN_LOW_SYMBOLS;
                    len = MATCH_LEN_MIN;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).rep_len_decoder.choice = (*coder).rep_len_decoder.choice
                        - ((*coder).rep_len_decoder.choice >> RC_MOVE_BITS);
                    current_block = 6834592846991627977;
                    continue;
                }
                current_block = 16690975975023747857;
            }
            3996983927318648760 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_REP2;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh141 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh141 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).is_rep2[state as usize] as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_rep2[state as usize] = ((*coder).is_rep2[state as usize] as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL
                                .wrapping_sub((*coder).is_rep2[state as usize] as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    let distance_3: u32 = rep2;
                    rep2 = rep1;
                    rep1 = rep0;
                    rep0 = distance_3;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_rep2[state as usize] = (*coder).is_rep2[state as usize]
                        - ((*coder).is_rep2[state as usize] >> RC_MOVE_BITS);
                    let distance_4: u32 = rep3;
                    rep3 = rep2;
                    rep2 = rep1;
                    rep1 = rep0;
                    rep0 = distance_4;
                }
                current_block = 15498320742470848828;
            }
            11808118301119257848 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_REP1;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh140 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh140 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).is_rep1[state as usize] as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_rep1[state as usize] = ((*coder).is_rep1[state as usize] as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL
                                .wrapping_sub((*coder).is_rep1[state as usize] as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    let distance_2: u32 = rep1;
                    rep1 = rep0;
                    rep0 = distance_2;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_rep1[state as usize] = (*coder).is_rep1[state as usize]
                        - ((*coder).is_rep1[state as usize] >> RC_MOVE_BITS);
                    current_block = 3996983927318648760;
                    continue;
                }
                current_block = 15498320742470848828;
            }
            5341942013764523046 => {
                if dict_put_safe(&raw mut dict, dict_get(&raw mut dict, rep0)) {
                    (*coder).sequence = SEQ_SHORTREP;
                    current_block = 4609795085482299213;
                    continue;
                } else {
                    current_block = 4956146061682418353;
                }
            }
            1698084742280242340 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_REP0_LONG;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh139 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh139 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).is_rep0_long[state as usize][pos_state as usize] as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_rep0_long[state as usize][pos_state as usize] =
                        ((*coder).is_rep0_long[state as usize][pos_state as usize] as u32)
                            .wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    (*coder).is_rep0_long[state as usize][pos_state as usize]
                                        as u32,
                                ) >> RC_MOVE_BITS,
                            ) as probability;
                    state = (if state < LIT_STATES {
                        STATE_LIT_SHORTREP
                    } else {
                        STATE_NONLIT_REP
                    }) as u32;
                    current_block = 5341942013764523046;
                    continue;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_rep0_long[state as usize][pos_state as usize] = (*coder)
                        .is_rep0_long[state as usize][pos_state as usize]
                        - ((*coder).is_rep0_long[state as usize][pos_state as usize]
                            >> RC_MOVE_BITS);
                }
                current_block = 15498320742470848828;
            }
            4420799852307653083 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_REP0;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh138 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh138 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).is_rep0[state as usize] as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_rep0[state as usize] = ((*coder).is_rep0[state as usize] as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL
                                .wrapping_sub((*coder).is_rep0[state as usize] as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    current_block = 1698084742280242340;
                    continue;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_rep0[state as usize] = (*coder).is_rep0[state as usize]
                        - ((*coder).is_rep0[state as usize] >> RC_MOVE_BITS);
                    current_block = 11808118301119257848;
                    continue;
                }
            }
            3469750012682708893 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_REP;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh123 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh123 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).is_rep[state as usize] as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_rep[state as usize] = ((*coder).is_rep[state as usize] as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub((*coder).is_rep[state as usize] as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    state = (if state < LIT_STATES {
                        STATE_LIT_MATCH
                    } else {
                        STATE_NONLIT_MATCH
                    }) as u32;
                    rep3 = rep2;
                    rep2 = rep1;
                    rep1 = rep0;
                    current_block = 1138292997408115650;
                    continue;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_rep[state as usize] = (*coder).is_rep[state as usize]
                        - ((*coder).is_rep[state as usize] >> RC_MOVE_BITS);
                    if !(!dict_is_distance_valid(&raw mut dict, 0)) {
                        current_block = 4420799852307653083;
                        continue;
                    }
                    ret_0 = LZMA_DATA_ERROR;
                    current_block = 4609795085482299213;
                    continue;
                }
            }
            7073645523065812117 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_EOPM;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh137 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh137 as u32;
                    }
                }
                ret_0 = if rc.code == 0 {
                    LZMA_STREAM_END
                } else {
                    LZMA_DATA_ERROR
                };
                current_block = 4609795085482299213;
                continue;
            }
            10510472849010538284 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_ALIGN;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh136 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh136 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).pos_align[offset.wrapping_add(symbol) as usize] as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).pos_align[offset.wrapping_add(symbol) as usize] =
                        ((*coder).pos_align[offset.wrapping_add(symbol) as usize] as u32)
                            .wrapping_add(
                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                    (*coder).pos_align[offset.wrapping_add(symbol) as usize] as u32,
                                ) >> RC_MOVE_BITS,
                            ) as probability;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).pos_align[offset.wrapping_add(symbol) as usize] = (*coder).pos_align
                        [offset.wrapping_add(symbol) as usize]
                        - ((*coder).pos_align[offset.wrapping_add(symbol) as usize]
                            >> RC_MOVE_BITS);
                    symbol = symbol.wrapping_add(offset);
                }
                offset <<= 1;
                if offset < ALIGN_SIZE {
                    current_block = 10510472849010538284;
                    continue;
                }
                rep0 = rep0.wrapping_add(symbol);
                if rep0 == UINT32_MAX {
                    current_block = 12043253436139097694;
                } else {
                    current_block = 13383302701878543647;
                }
            }
            15418612220330286504 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_DIRECT;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh135 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh135 as u32;
                    }
                }
                rc.range >>= 1;
                rc.code = rc.code.wrapping_sub(rc.range);
                rc_bound = 0u32.wrapping_sub(rc.code >> 31);
                rc.code = rc.code.wrapping_add(rc.range & rc_bound);
                rep0 = (rep0 << 1).wrapping_add(rc_bound.wrapping_add(1));
                limit -= 1;
                if limit > 0 {
                    current_block = 15418612220330286504;
                    continue;
                }
                rep0 <<= ALIGN_BITS;
                symbol = 0;
                offset = 1;
                current_block = 10510472849010538284;
                continue;
            }
            617447976488552541 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_DIST_MODEL;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh132 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh132 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(symbol as isize) as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    *probs.offset(symbol as isize) = (*probs.offset(symbol as isize) as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(*probs.offset(symbol as isize) as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    symbol <<= 1;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    *probs.offset(symbol as isize) -=
                        *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                    symbol = (symbol << 1).wrapping_add(1);
                    rep0 = (rep0 as u32).wrapping_add(1u32 << offset) as u32;
                }
                offset += 1;
                if offset < limit {
                    current_block = 617447976488552541;
                    continue;
                }
                current_block = 13383302701878543647;
            }
            4174862988780014241 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_DIST_SLOT;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh129 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh129 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(symbol as isize) as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    *probs.offset(symbol as isize) = (*probs.offset(symbol as isize) as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(*probs.offset(symbol as isize) as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    symbol <<= 1;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    *probs.offset(symbol as isize) -=
                        *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                    symbol = (symbol << 1).wrapping_add(1);
                }
                if symbol < DIST_SLOTS {
                    current_block = 4174862988780014241;
                    continue;
                }
                symbol = symbol.wrapping_sub(DIST_SLOTS);
                if symbol < DIST_MODEL_START {
                    rep0 = symbol;
                } else {
                    limit = (symbol >> 1).wrapping_sub(1);
                    rep0 = (2u32).wrapping_add(symbol & 1);
                    if symbol < DIST_MODEL_END {
                        rep0 <<= limit;
                        probs = (&raw mut (*coder).pos_special as *mut probability)
                            .offset(rep0 as isize)
                            .offset(-(symbol as isize))
                            .offset(-1);
                        symbol = 1;
                        offset = 0;
                        current_block = 617447976488552541;
                        continue;
                    } else {
                        limit = limit.wrapping_sub(ALIGN_BITS);
                        current_block = 15418612220330286504;
                        continue;
                    }
                }
                current_block = 13383302701878543647;
            }
            592696588731961849 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_MATCH_LEN_BITTREE;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh126 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh126 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(symbol as isize) as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    *probs.offset(symbol as isize) = (*probs.offset(symbol as isize) as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(*probs.offset(symbol as isize) as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    symbol <<= 1;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    *probs.offset(symbol as isize) -=
                        *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                    symbol = (symbol << 1).wrapping_add(1);
                }
                if symbol < limit {
                    current_block = 592696588731961849;
                    continue;
                }
                len = len.wrapping_add(symbol.wrapping_sub(limit));
                probs = &raw mut *(&raw mut (*coder).dist_slot as *mut [probability; 64]).offset(
                    (if len < (DIST_STATES + MATCH_LEN_MIN) as u32 {
                        len.wrapping_sub(MATCH_LEN_MIN)
                    } else {
                        (DIST_STATES - 1) as u32
                    }) as isize,
                ) as *mut probability;
                symbol = 1;
                current_block = 4174862988780014241;
                continue;
            }
            13912927785247575907 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_MATCH_LEN_CHOICE2;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh125 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh125 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).match_len_decoder.choice2 as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).match_len_decoder.choice2 = ((*coder).match_len_decoder.choice2 as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL
                                .wrapping_sub((*coder).match_len_decoder.choice2 as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    probs = &raw mut *(&raw mut (*coder).match_len_decoder.mid
                        as *mut [probability; 8])
                        .offset(pos_state as isize) as *mut probability;
                    limit = LEN_MID_SYMBOLS;
                    len = (MATCH_LEN_MIN + LEN_LOW_SYMBOLS) as u32;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).match_len_decoder.choice2 = (*coder).match_len_decoder.choice2
                        - ((*coder).match_len_decoder.choice2 >> RC_MOVE_BITS);
                    probs = &raw mut (*coder).match_len_decoder.high as *mut probability;
                    limit = LEN_HIGH_SYMBOLS;
                    len = (MATCH_LEN_MIN + LEN_LOW_SYMBOLS + LEN_MID_SYMBOLS) as u32;
                }
                current_block = 8485842003490715114;
            }
            1138292997408115650 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_MATCH_LEN_CHOICE;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh124 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh124 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).match_len_decoder.choice as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).match_len_decoder.choice = ((*coder).match_len_decoder.choice as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL
                                .wrapping_sub((*coder).match_len_decoder.choice as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    probs = &raw mut *(&raw mut (*coder).match_len_decoder.low
                        as *mut [probability; 8])
                        .offset(pos_state as isize) as *mut probability;
                    limit = LEN_LOW_SYMBOLS;
                    len = MATCH_LEN_MIN;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).match_len_decoder.choice = (*coder).match_len_decoder.choice
                        - ((*coder).match_len_decoder.choice >> RC_MOVE_BITS);
                    current_block = 13912927785247575907;
                    continue;
                }
                current_block = 8485842003490715114;
            }
            10535798129821001304 => {
                if dict_put_safe(&raw mut dict, symbol as u8) {
                    (*coder).sequence = SEQ_LITERAL_WRITE;
                    current_block = 4609795085482299213;
                    continue;
                } else {
                    current_block = 4956146061682418353;
                }
            }
            18125716024132132232 => {
                let match_bit: u32 = len & offset;
                let subcoder_index: u32 = offset.wrapping_add(match_bit).wrapping_add(symbol);
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_LITERAL_MATCHED;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh120 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh120 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(subcoder_index as isize) as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    *probs.offset(subcoder_index as isize) =
                        (*probs.offset(subcoder_index as isize) as u32).wrapping_add(
                            RC_BIT_MODEL_TOTAL
                                .wrapping_sub(*probs.offset(subcoder_index as isize) as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    symbol <<= 1;
                    offset &= !match_bit;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    *probs.offset(subcoder_index as isize) -=
                        *probs.offset(subcoder_index as isize) >> RC_MOVE_BITS;
                    symbol = (symbol << 1).wrapping_add(1);
                    offset &= match_bit;
                }
                len <<= 1;
                if symbol < ((1) << 8) as u32 {
                    current_block = 18125716024132132232;
                    continue;
                } else {
                    current_block = 10535798129821001304;
                    continue;
                }
            }
            5979571030476392895 => {
                if (might_finish_without_eopm && dict.pos == dict.limit) as c_long != 0 {
                    if rc.range < RC_TOP_VALUE as u32 {
                        if rc_in_ptr == rc_in_end {
                            (*coder).sequence = SEQ_NORMALIZE;
                            current_block = 4609795085482299213;
                            continue;
                        } else {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh115 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh115 as u32;
                        }
                    }
                    if rc.code == 0 {
                        ret_0 = LZMA_STREAM_END;
                        current_block = 4609795085482299213;
                        continue;
                    } else if !(*coder).allow_eopm {
                        ret_0 = LZMA_DATA_ERROR;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        eopm_is_valid = true;
                    }
                }
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_IS_MATCH;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh116 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh116 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).is_match[state as usize][pos_state as usize] as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_match[state as usize][pos_state as usize] = ((*coder).is_match
                        [state as usize][pos_state as usize]
                        as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                (*coder).is_match[state as usize][pos_state as usize] as u32,
                            ) >> RC_MOVE_BITS,
                        )
                        as probability
                        as probability;
                    probs = (&raw mut (*coder).literal as *mut probability).offset(
                        (3_usize).wrapping_mul(
                            ((dict.pos << 8).wrapping_add(dict_get0(&raw mut dict) as size_t)
                                & literal_mask as size_t)
                                << literal_context_bits,
                        ) as isize,
                    );
                    symbol = 1;
                    if state < LIT_STATES {
                        state = if state <= STATE_SHORTREP_LIT_LIT {
                            STATE_LIT_LIT
                        } else {
                            state.wrapping_sub(3)
                        };
                        current_block = 13844743919235296534;
                        continue;
                    } else {
                        state = if state <= STATE_LIT_SHORTREP {
                            state.wrapping_sub(3)
                        } else {
                            state.wrapping_sub(6)
                        };
                        len = (dict_get(&raw mut dict, rep0) as u32) << 1;
                        offset = 0x100;
                        current_block = 18125716024132132232;
                        continue;
                    }
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_match[state as usize][pos_state as usize] = (*coder).is_match
                        [state as usize][pos_state as usize]
                        - ((*coder).is_match[state as usize][pos_state as usize] >> RC_MOVE_BITS);
                    current_block = 3469750012682708893;
                    continue;
                }
            }
            13844743919235296534 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_LITERAL;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh117 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh117 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(symbol as isize) as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    *probs.offset(symbol as isize) = (*probs.offset(symbol as isize) as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(*probs.offset(symbol as isize) as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    symbol <<= 1;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    *probs.offset(symbol as isize) -=
                        *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                    symbol = (symbol << 1).wrapping_add(1);
                }
                if symbol < ((1) << 8) as u32 {
                    current_block = 13844743919235296534;
                    continue;
                } else {
                    current_block = 10535798129821001304;
                    continue;
                }
            }
            2467942631393454738 => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_REP_LEN_BITTREE;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh144 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh144 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul(*probs.offset(symbol as isize) as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    *probs.offset(symbol as isize) = (*probs.offset(symbol as isize) as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(*probs.offset(symbol as isize) as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    symbol <<= 1;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    *probs.offset(symbol as isize) -=
                        *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                    symbol = (symbol << 1).wrapping_add(1);
                }
                if symbol < limit {
                    current_block = 2467942631393454738;
                    continue;
                }
                len = len.wrapping_add(symbol.wrapping_sub(limit));
                current_block = 17340485688450593529;
                continue;
            }
            17340485688450593529 => {
                if dict_repeat(&raw mut dict, rep0, &raw mut len) as c_long != 0 {
                    (*coder).sequence = SEQ_COPY;
                    current_block = 4609795085482299213;
                    continue;
                } else {
                    current_block = 4956146061682418353;
                }
            }
            _ => {
                if rc.range < RC_TOP_VALUE as u32 {
                    if rc_in_ptr == rc_in_end {
                        (*coder).sequence = SEQ_REP_LEN_CHOICE2;
                        current_block = 4609795085482299213;
                        continue;
                    } else {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh143 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh143 as u32;
                    }
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).rep_len_decoder.choice2 as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).rep_len_decoder.choice2 = ((*coder).rep_len_decoder.choice2 as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL
                                .wrapping_sub((*coder).rep_len_decoder.choice2 as u32)
                                >> RC_MOVE_BITS,
                        ) as probability;
                    probs = &raw mut *(&raw mut (*coder).rep_len_decoder.mid
                        as *mut [probability; 8])
                        .offset(pos_state as isize) as *mut probability;
                    limit = LEN_MID_SYMBOLS;
                    len = (MATCH_LEN_MIN + LEN_LOW_SYMBOLS) as u32;
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).rep_len_decoder.choice2 = (*coder).rep_len_decoder.choice2
                        - ((*coder).rep_len_decoder.choice2 >> RC_MOVE_BITS);
                    probs = &raw mut (*coder).rep_len_decoder.high as *mut probability;
                    limit = LEN_HIGH_SYMBOLS;
                    len = (MATCH_LEN_MIN + LEN_LOW_SYMBOLS + LEN_MID_SYMBOLS) as u32;
                }
                current_block = 16690975975023747857;
            }
        }
        match current_block {
            13383302701878543647 => {
                if !(!dict_is_distance_valid(&raw mut dict, rep0 as size_t)) {
                    current_block = 17340485688450593529;
                    continue;
                }
                ret_0 = LZMA_DATA_ERROR;
                current_block = 4609795085482299213;
                continue;
            }
            4956146061682418353 => loop {
                pos_state = (dict.pos & pos_mask as size_t) as u32;
                if !(rc_in_ptr < rc_in_fast_end) || dict.pos == dict.limit {
                    current_block = 5979571030476392895;
                    continue 'c_9380;
                }
                if rc.range < RC_TOP_VALUE as u32 {
                    rc.range <<= RC_SHIFT_BITS;
                    let fresh3 = rc_in_ptr;
                    rc_in_ptr = rc_in_ptr.offset(1);
                    rc.code = rc.code << RC_SHIFT_BITS | *fresh3 as u32;
                }
                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                    .wrapping_mul((*coder).is_match[state as usize][pos_state as usize] as u32);
                if rc.code < rc_bound {
                    rc.range = rc_bound;
                    (*coder).is_match[state as usize][pos_state as usize] = ((*coder).is_match
                        [state as usize][pos_state as usize]
                        as u32)
                        .wrapping_add(
                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                (*coder).is_match[state as usize][pos_state as usize] as u32,
                            ) >> RC_MOVE_BITS,
                        )
                        as probability
                        as probability;
                    probs = (&raw mut (*coder).literal as *mut probability).offset(
                        (3_usize).wrapping_mul(
                            ((dict.pos << 8).wrapping_add(dict_get0(&raw mut dict) as size_t)
                                & literal_mask as size_t)
                                << literal_context_bits,
                        ) as isize,
                    );
                    if state < LIT_STATES {
                        state = if state <= STATE_SHORTREP_LIT_LIT {
                            STATE_LIT_LIT
                        } else {
                            state.wrapping_sub(3)
                        };
                        symbol = 1;
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh4 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh4 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh7 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh7 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh10 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh10 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh13 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh13 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh16 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh16 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh19 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh19 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh22 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh22 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh25 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh25 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                    } else {
                        state = if state <= STATE_LIT_SHORTREP {
                            state.wrapping_sub(3)
                        } else {
                            state.wrapping_sub(6)
                        };
                        let mut t_match_byte: u32 = dict_get(&raw mut dict, rep0) as u32;
                        let mut t_match_bit: u32 = 0;
                        let mut t_subcoder_index: u32 = 0;
                        let mut t_offset: u32 = 0x100;
                        symbol = 1;
                        t_match_byte <<= 1;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh28 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh28 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(t_subcoder_index as isize) =
                                (*probs.offset(t_subcoder_index as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                        *probs.offset(t_subcoder_index as isize) as u32,
                                    ) >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(t_subcoder_index as isize) -=
                                *probs.offset(t_subcoder_index as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh31 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh31 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(t_subcoder_index as isize) =
                                (*probs.offset(t_subcoder_index as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                        *probs.offset(t_subcoder_index as isize) as u32,
                                    ) >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(t_subcoder_index as isize) -=
                                *probs.offset(t_subcoder_index as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh34 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh34 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(t_subcoder_index as isize) =
                                (*probs.offset(t_subcoder_index as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                        *probs.offset(t_subcoder_index as isize) as u32,
                                    ) >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(t_subcoder_index as isize) -=
                                *probs.offset(t_subcoder_index as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh37 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh37 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(t_subcoder_index as isize) =
                                (*probs.offset(t_subcoder_index as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                        *probs.offset(t_subcoder_index as isize) as u32,
                                    ) >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(t_subcoder_index as isize) -=
                                *probs.offset(t_subcoder_index as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh40 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh40 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(t_subcoder_index as isize) =
                                (*probs.offset(t_subcoder_index as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                        *probs.offset(t_subcoder_index as isize) as u32,
                                    ) >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(t_subcoder_index as isize) -=
                                *probs.offset(t_subcoder_index as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh43 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh43 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(t_subcoder_index as isize) =
                                (*probs.offset(t_subcoder_index as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                        *probs.offset(t_subcoder_index as isize) as u32,
                                    ) >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(t_subcoder_index as isize) -=
                                *probs.offset(t_subcoder_index as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh46 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh46 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(t_subcoder_index as isize) =
                                (*probs.offset(t_subcoder_index as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                        *probs.offset(t_subcoder_index as isize) as u32,
                                    ) >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(t_subcoder_index as isize) -=
                                *probs.offset(t_subcoder_index as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                            t_offset &= t_match_bit;
                        }
                        t_match_byte <<= 1;
                        t_match_bit = t_match_byte & t_offset;
                        t_subcoder_index = t_offset.wrapping_add(t_match_bit).wrapping_add(symbol);
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh49 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh49 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(t_subcoder_index as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(t_subcoder_index as isize) =
                                (*probs.offset(t_subcoder_index as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                        *probs.offset(t_subcoder_index as isize) as u32,
                                    ) >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                            t_offset &= !t_match_bit;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(t_subcoder_index as isize) -=
                                *probs.offset(t_subcoder_index as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                            t_offset &= t_match_bit;
                        }
                    }
                    dict_put(&raw mut dict, symbol as u8);
                } else {
                    rc.range = rc.range.wrapping_sub(rc_bound);
                    rc.code = rc.code.wrapping_sub(rc_bound);
                    (*coder).is_match[state as usize][pos_state as usize] = (*coder).is_match
                        [state as usize][pos_state as usize]
                        - ((*coder).is_match[state as usize][pos_state as usize] >> RC_MOVE_BITS);
                    if rc.range < RC_TOP_VALUE as u32 {
                        rc.range <<= RC_SHIFT_BITS;
                        let fresh52 = rc_in_ptr;
                        rc_in_ptr = rc_in_ptr.offset(1);
                        rc.code = rc.code << RC_SHIFT_BITS | *fresh52 as u32;
                    }
                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                        .wrapping_mul((*coder).is_rep[state as usize] as u32);
                    if rc.code < rc_bound {
                        rc.range = rc_bound;
                        (*coder).is_rep[state as usize] =
                            ((*coder).is_rep[state as usize] as u32).wrapping_add(
                                RC_BIT_MODEL_TOTAL
                                    .wrapping_sub((*coder).is_rep[state as usize] as u32)
                                    >> RC_MOVE_BITS,
                            ) as probability;
                        state = (if state < LIT_STATES {
                            STATE_LIT_MATCH
                        } else {
                            STATE_NONLIT_MATCH
                        }) as u32;
                        rep3 = rep2;
                        rep2 = rep1;
                        rep1 = rep0;
                        symbol = 1;
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh53 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh53 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul((*coder).match_len_decoder.choice as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            (*coder).match_len_decoder.choice =
                                ((*coder).match_len_decoder.choice as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub((*coder).match_len_decoder.choice as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol = 1;
                            if rc.range < RC_TOP_VALUE as u32 {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh54 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh54 as u32;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                (*coder).match_len_decoder.low[pos_state as usize][symbol as usize]
                                    as u32,
                            );
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = ((*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    as u32)
                                    .wrapping_add(
                                        RC_BIT_MODEL_TOTAL.wrapping_sub(
                                            (*coder).match_len_decoder.low[pos_state as usize]
                                                [symbol as usize]
                                                as u32,
                                        ) >> RC_MOVE_BITS,
                                    )
                                    as probability
                                    as probability;
                                symbol <<= 1;
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = (*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    - ((*coder).match_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        >> RC_MOVE_BITS);
                                symbol = (symbol << 1).wrapping_add(1);
                            }
                            if rc.range < RC_TOP_VALUE as u32 {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh55 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh55 as u32;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                (*coder).match_len_decoder.low[pos_state as usize][symbol as usize]
                                    as u32,
                            );
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = ((*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    as u32)
                                    .wrapping_add(
                                        RC_BIT_MODEL_TOTAL.wrapping_sub(
                                            (*coder).match_len_decoder.low[pos_state as usize]
                                                [symbol as usize]
                                                as u32,
                                        ) >> RC_MOVE_BITS,
                                    )
                                    as probability
                                    as probability;
                                symbol <<= 1;
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = (*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    - ((*coder).match_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        >> RC_MOVE_BITS);
                                symbol = (symbol << 1).wrapping_add(1);
                            }
                            if rc.range < RC_TOP_VALUE as u32 {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh56 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh56 as u32;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                (*coder).match_len_decoder.low[pos_state as usize][symbol as usize]
                                    as u32,
                            );
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = ((*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    as u32)
                                    .wrapping_add(
                                        RC_BIT_MODEL_TOTAL.wrapping_sub(
                                            (*coder).match_len_decoder.low[pos_state as usize]
                                                [symbol as usize]
                                                as u32,
                                        ) >> RC_MOVE_BITS,
                                    )
                                    as probability
                                    as probability;
                                symbol <<= 1;
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).match_len_decoder.low[pos_state as usize]
                                    [symbol as usize] = (*coder).match_len_decoder.low
                                    [pos_state as usize][symbol as usize]
                                    - ((*coder).match_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        >> RC_MOVE_BITS);
                                symbol = (symbol << 1).wrapping_add(1);
                            }
                            symbol = symbol.wrapping_add((-((1_i32) << 3) + 2) as u32);
                            len = symbol;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            (*coder).match_len_decoder.choice = (*coder).match_len_decoder.choice
                                - ((*coder).match_len_decoder.choice >> RC_MOVE_BITS);
                            if rc.range < RC_TOP_VALUE as u32 {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh57 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh57 as u32;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                .wrapping_mul((*coder).match_len_decoder.choice2 as u32);
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).match_len_decoder.choice2 =
                                    ((*coder).match_len_decoder.choice2 as u32).wrapping_add(
                                        RC_BIT_MODEL_TOTAL.wrapping_sub(
                                            (*coder).match_len_decoder.choice2 as u32,
                                        ) >> RC_MOVE_BITS,
                                    ) as probability
                                        as probability;
                                symbol = 1;
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh58 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh58 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize]
                                        as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = ((*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as u32)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).match_len_decoder.mid[pos_state as usize]
                                                    [symbol as usize]
                                                    as u32,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = (*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        - ((*coder).match_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh59 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh59 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize]
                                        as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = ((*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as u32)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).match_len_decoder.mid[pos_state as usize]
                                                    [symbol as usize]
                                                    as u32,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = (*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        - ((*coder).match_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh60 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh60 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize]
                                        as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = ((*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        as u32)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).match_len_decoder.mid[pos_state as usize]
                                                    [symbol as usize]
                                                    as u32,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.mid[pos_state as usize]
                                        [symbol as usize] = (*coder).match_len_decoder.mid
                                        [pos_state as usize]
                                        [symbol as usize]
                                        - ((*coder).match_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                symbol =
                                    symbol.wrapping_add((-((1_i32) << 3) + 2 + ((1) << 3)) as u32);
                                len = symbol;
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).match_len_decoder.choice2 =
                                    (*coder).match_len_decoder.choice2
                                        - ((*coder).match_len_decoder.choice2 >> RC_MOVE_BITS);
                                symbol = 1;
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh61 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh61 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize] as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        (*coder).match_len_decoder.high[symbol as usize]
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh62 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh62 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize] as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        (*coder).match_len_decoder.high[symbol as usize]
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh63 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh63 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize] as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        (*coder).match_len_decoder.high[symbol as usize]
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh64 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh64 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize] as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        (*coder).match_len_decoder.high[symbol as usize]
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh65 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh65 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize] as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        (*coder).match_len_decoder.high[symbol as usize]
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh66 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh66 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize] as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        (*coder).match_len_decoder.high[symbol as usize]
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh67 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh67 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize] as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        (*coder).match_len_decoder.high[symbol as usize]
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh68 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh68 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).match_len_decoder.high[symbol as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        ((*coder).match_len_decoder.high[symbol as usize] as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).match_len_decoder.high[symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).match_len_decoder.high[symbol as usize] =
                                        (*coder).match_len_decoder.high[symbol as usize]
                                            - ((*coder).match_len_decoder.high[symbol as usize]
                                                >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                symbol = symbol.wrapping_add(
                                    (-((1_i32) << 8) + 2 + ((1) << 3) + ((1) << 3)) as u32,
                                );
                                len = symbol;
                            }
                        }
                        probs = &raw mut *(&raw mut (*coder).dist_slot as *mut [probability; 64])
                            .offset(
                                (if len < (DIST_STATES + MATCH_LEN_MIN) as u32 {
                                    len.wrapping_sub(MATCH_LEN_MIN)
                                } else {
                                    (DIST_STATES - 1) as u32
                                }) as isize,
                            ) as *mut probability;
                        symbol = 1;
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh69 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh69 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh72 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh72 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh75 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh75 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh78 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh78 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh81 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh81 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        if rc.range < RC_TOP_VALUE as u32 {
                            rc.range <<= RC_SHIFT_BITS;
                            let fresh84 = rc_in_ptr;
                            rc_in_ptr = rc_in_ptr.offset(1);
                            rc.code = rc.code << RC_SHIFT_BITS | *fresh84 as u32;
                        }
                        rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                            .wrapping_mul(*probs.offset(symbol as isize) as u32);
                        if rc.code < rc_bound {
                            rc.range = rc_bound;
                            *probs.offset(symbol as isize) =
                                (*probs.offset(symbol as isize) as u32).wrapping_add(
                                    RC_BIT_MODEL_TOTAL
                                        .wrapping_sub(*probs.offset(symbol as isize) as u32)
                                        >> RC_MOVE_BITS,
                                ) as probability;
                            symbol <<= 1;
                        } else {
                            rc.range = rc.range.wrapping_sub(rc_bound);
                            rc.code = rc.code.wrapping_sub(rc_bound);
                            *probs.offset(symbol as isize) -=
                                *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                            symbol = (symbol << 1).wrapping_add(1);
                        }
                        symbol = symbol.wrapping_add(-((1_i32) << 6) as u32);
                        if symbol < DIST_MODEL_START {
                            rep0 = symbol;
                        } else {
                            limit = (symbol >> 1).wrapping_sub(1);
                            rep0 = (2u32).wrapping_add(symbol & 1);
                            if symbol < DIST_MODEL_END {
                                rep0 <<= limit;
                                probs = (&raw mut (*coder).pos_special as *mut probability)
                                    .offset(rep0 as isize)
                                    .offset(-(symbol as isize))
                                    .offset(-1);
                                symbol = 1;
                                offset = 1;
                                loop {
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh87 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh87 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                        .wrapping_mul(*probs.offset(symbol as isize) as u32);
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        *probs.offset(symbol as isize) =
                                            (*probs.offset(symbol as isize) as u32).wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    *probs.offset(symbol as isize) as u32,
                                                ) >> RC_MOVE_BITS,
                                            )
                                                as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        *probs.offset(symbol as isize) -=
                                            *probs.offset(symbol as isize) >> RC_MOVE_BITS;
                                        symbol = (symbol << 1).wrapping_add(1);
                                        rep0 = rep0.wrapping_add(offset);
                                    }
                                    offset <<= 1;
                                    limit -= 1;
                                    if !(limit > 0) {
                                        break;
                                    }
                                }
                            } else {
                                limit = limit.wrapping_sub(ALIGN_BITS);
                                loop {
                                    rep0 = (rep0 << 1).wrapping_add(1);
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh90 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh90 as u32;
                                    }
                                    rc.range >>= 1;
                                    rc.code = rc.code.wrapping_sub(rc.range);
                                    rc_bound = 0u32.wrapping_sub(rc.code >> 31);
                                    rep0 = rep0.wrapping_add(rc_bound);
                                    rc.code = rc.code.wrapping_add(rc.range & rc_bound);
                                    limit -= 1;
                                    if !(limit > 0) {
                                        break;
                                    }
                                }
                                rep0 <<= ALIGN_BITS;
                                symbol = 0;
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh91 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh91 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).pos_align[symbol.wrapping_add(1) as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).pos_align[symbol.wrapping_add(1) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(1) as usize]
                                        as u32)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).pos_align[symbol.wrapping_add(1) as usize]
                                                    as u32,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).pos_align[symbol.wrapping_add(1) as usize] =
                                        (*coder).pos_align[symbol.wrapping_add(1) as usize]
                                            - ((*coder).pos_align[symbol.wrapping_add(1) as usize]
                                                >> RC_MOVE_BITS);
                                    symbol += 1;
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh92 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh92 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).pos_align[symbol.wrapping_add(2) as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).pos_align[symbol.wrapping_add(2) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(2) as usize]
                                        as u32)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).pos_align[symbol.wrapping_add(2) as usize]
                                                    as u32,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).pos_align[symbol.wrapping_add(2) as usize] =
                                        (*coder).pos_align[symbol.wrapping_add(2) as usize]
                                            - ((*coder).pos_align[symbol.wrapping_add(2) as usize]
                                                >> RC_MOVE_BITS);
                                    symbol = symbol.wrapping_add(2);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh93 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh93 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).pos_align[symbol.wrapping_add(4) as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).pos_align[symbol.wrapping_add(4) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(4) as usize]
                                        as u32)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).pos_align[symbol.wrapping_add(4) as usize]
                                                    as u32,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).pos_align[symbol.wrapping_add(4) as usize] =
                                        (*coder).pos_align[symbol.wrapping_add(4) as usize]
                                            - ((*coder).pos_align[symbol.wrapping_add(4) as usize]
                                                >> RC_MOVE_BITS);
                                    symbol = symbol.wrapping_add(4);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh94 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh94 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).pos_align[symbol.wrapping_add(8) as usize] as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).pos_align[symbol.wrapping_add(8) as usize] = ((*coder)
                                        .pos_align
                                        [symbol.wrapping_add(8) as usize]
                                        as u32)
                                        .wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).pos_align[symbol.wrapping_add(8) as usize]
                                                    as u32,
                                            ) >> RC_MOVE_BITS,
                                        )
                                        as probability
                                        as probability;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).pos_align[symbol.wrapping_add(8) as usize] =
                                        (*coder).pos_align[symbol.wrapping_add(8) as usize]
                                            - ((*coder).pos_align[symbol.wrapping_add(8) as usize]
                                                >> RC_MOVE_BITS);
                                    symbol = symbol.wrapping_add(8);
                                }
                                rep0 = rep0.wrapping_add(symbol);
                                if rep0 == UINT32_MAX {
                                    break;
                                }
                            }
                        }
                        if !dict_is_distance_valid(&raw mut dict, rep0 as size_t) as c_long != 0 {
                            ret_0 = LZMA_DATA_ERROR;
                            current_block = 4609795085482299213;
                            continue 'c_9380;
                        }
                    } else {
                        rc.range = rc.range.wrapping_sub(rc_bound);
                        rc.code = rc.code.wrapping_sub(rc_bound);
                        (*coder).is_rep[state as usize] = (*coder).is_rep[state as usize]
                            - ((*coder).is_rep[state as usize] >> RC_MOVE_BITS);
                        if !dict_is_distance_valid(&raw mut dict, 0) {
                            ret_0 = LZMA_DATA_ERROR;
                            current_block = 4609795085482299213;
                            continue 'c_9380;
                        } else {
                            if rc.range < RC_TOP_VALUE as u32 {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh95 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh95 as u32;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                .wrapping_mul((*coder).is_rep0[state as usize] as u32);
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).is_rep0[state as usize] =
                                    ((*coder).is_rep0[state as usize] as u32).wrapping_add(
                                        RC_BIT_MODEL_TOTAL
                                            .wrapping_sub((*coder).is_rep0[state as usize] as u32)
                                            >> RC_MOVE_BITS,
                                    ) as probability
                                        as probability;
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh96 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh96 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).is_rep0_long[state as usize][pos_state as usize]
                                        as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).is_rep0_long[state as usize][pos_state as usize] =
                                        ((*coder).is_rep0_long[state as usize][pos_state as usize]
                                            as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).is_rep0_long[state as usize]
                                                        [pos_state as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    state = (if state < LIT_STATES {
                                        STATE_LIT_SHORTREP
                                    } else {
                                        STATE_NONLIT_REP
                                    }) as u32;
                                    dict_put(&raw mut dict, dict_get(&raw mut dict, rep0));
                                    continue;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).is_rep0_long[state as usize][pos_state as usize] =
                                        (*coder).is_rep0_long[state as usize][pos_state as usize]
                                            - ((*coder).is_rep0_long[state as usize]
                                                [pos_state as usize]
                                                >> RC_MOVE_BITS);
                                }
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).is_rep0[state as usize] = (*coder).is_rep0[state as usize]
                                    - ((*coder).is_rep0[state as usize] >> RC_MOVE_BITS);
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh97 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh97 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                    .wrapping_mul((*coder).is_rep1[state as usize] as u32);
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).is_rep1[state as usize] =
                                        ((*coder).is_rep1[state as usize] as u32).wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).is_rep1[state as usize] as u32,
                                            ) >> RC_MOVE_BITS,
                                        ) as probability
                                            as probability;
                                    let distance: u32 = rep1;
                                    rep1 = rep0;
                                    rep0 = distance;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).is_rep1[state as usize] = (*coder).is_rep1
                                        [state as usize]
                                        - ((*coder).is_rep1[state as usize] >> RC_MOVE_BITS);
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh98 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh98 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                        .wrapping_mul((*coder).is_rep2[state as usize] as u32);
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).is_rep2[state as usize] =
                                            ((*coder).is_rep2[state as usize] as u32).wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).is_rep2[state as usize] as u32,
                                                ) >> RC_MOVE_BITS,
                                            )
                                                as probability
                                                as probability;
                                        let distance_0: u32 = rep2;
                                        rep2 = rep1;
                                        rep1 = rep0;
                                        rep0 = distance_0;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).is_rep2[state as usize] = (*coder).is_rep2
                                            [state as usize]
                                            - ((*coder).is_rep2[state as usize] >> RC_MOVE_BITS);
                                        let distance_1: u32 = rep3;
                                        rep3 = rep2;
                                        rep2 = rep1;
                                        rep1 = rep0;
                                        rep0 = distance_1;
                                    }
                                }
                            }
                            state = (if state < LIT_STATES {
                                STATE_LIT_LONGREP
                            } else {
                                STATE_NONLIT_REP
                            }) as u32;
                            symbol = 1;
                            if rc.range < RC_TOP_VALUE as u32 {
                                rc.range <<= RC_SHIFT_BITS;
                                let fresh99 = rc_in_ptr;
                                rc_in_ptr = rc_in_ptr.offset(1);
                                rc.code = rc.code << RC_SHIFT_BITS | *fresh99 as u32;
                            }
                            rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                .wrapping_mul((*coder).rep_len_decoder.choice as u32);
                            if rc.code < rc_bound {
                                rc.range = rc_bound;
                                (*coder).rep_len_decoder.choice =
                                    ((*coder).rep_len_decoder.choice as u32).wrapping_add(
                                        RC_BIT_MODEL_TOTAL
                                            .wrapping_sub((*coder).rep_len_decoder.choice as u32)
                                            >> RC_MOVE_BITS,
                                    ) as probability
                                        as probability;
                                symbol = 1;
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh100 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh100 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] =
                                        ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                                        [symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] = (*coder).rep_len_decoder.low
                                        [pos_state as usize]
                                        [symbol as usize]
                                        - ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh101 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh101 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] =
                                        ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                                        [symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] = (*coder).rep_len_decoder.low
                                        [pos_state as usize]
                                        [symbol as usize]
                                        - ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh102 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh102 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize]
                                        as u32,
                                );
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] =
                                        ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                                        [symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            ) as probability
                                            as probability;
                                    symbol <<= 1;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).rep_len_decoder.low[pos_state as usize]
                                        [symbol as usize] = (*coder).rep_len_decoder.low
                                        [pos_state as usize]
                                        [symbol as usize]
                                        - ((*coder).rep_len_decoder.low[pos_state as usize]
                                            [symbol as usize]
                                            >> RC_MOVE_BITS);
                                    symbol = (symbol << 1).wrapping_add(1);
                                }
                                symbol = symbol.wrapping_add((-((1_i32) << 3) + 2) as u32);
                                len = symbol;
                            } else {
                                rc.range = rc.range.wrapping_sub(rc_bound);
                                rc.code = rc.code.wrapping_sub(rc_bound);
                                (*coder).rep_len_decoder.choice = (*coder).rep_len_decoder.choice
                                    - ((*coder).rep_len_decoder.choice >> RC_MOVE_BITS);
                                if rc.range < RC_TOP_VALUE as u32 {
                                    rc.range <<= RC_SHIFT_BITS;
                                    let fresh103 = rc_in_ptr;
                                    rc_in_ptr = rc_in_ptr.offset(1);
                                    rc.code = rc.code << RC_SHIFT_BITS | *fresh103 as u32;
                                }
                                rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS)
                                    .wrapping_mul((*coder).rep_len_decoder.choice2 as u32);
                                if rc.code < rc_bound {
                                    rc.range = rc_bound;
                                    (*coder).rep_len_decoder.choice2 =
                                        ((*coder).rep_len_decoder.choice2 as u32).wrapping_add(
                                            RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                (*coder).rep_len_decoder.choice2 as u32,
                                            ) >> RC_MOVE_BITS,
                                        ) as probability
                                            as probability;
                                    symbol = 1;
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh104 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh104 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            as u32,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = ((*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.mid[pos_state as usize]
                                                        [symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = (*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            - ((*coder).rep_len_decoder.mid[pos_state as usize]
                                                [symbol as usize]
                                                >> RC_MOVE_BITS);
                                        symbol = (symbol << 1).wrapping_add(1);
                                    }
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh105 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh105 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            as u32,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = ((*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.mid[pos_state as usize]
                                                        [symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = (*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            - ((*coder).rep_len_decoder.mid[pos_state as usize]
                                                [symbol as usize]
                                                >> RC_MOVE_BITS);
                                        symbol = (symbol << 1).wrapping_add(1);
                                    }
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh106 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh106 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize]
                                            as u32,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = ((*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            as u32)
                                            .wrapping_add(
                                                RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                    (*coder).rep_len_decoder.mid[pos_state as usize]
                                                        [symbol as usize]
                                                        as u32,
                                                ) >> RC_MOVE_BITS,
                                            )
                                            as probability
                                            as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.mid[pos_state as usize]
                                            [symbol as usize] = (*coder).rep_len_decoder.mid
                                            [pos_state as usize]
                                            [symbol as usize]
                                            - ((*coder).rep_len_decoder.mid[pos_state as usize]
                                                [symbol as usize]
                                                >> RC_MOVE_BITS);
                                        symbol = (symbol << 1).wrapping_add(1);
                                    }
                                    symbol = symbol
                                        .wrapping_add((-((1_i32) << 3) + 2 + ((1) << 3)) as u32);
                                    len = symbol;
                                } else {
                                    rc.range = rc.range.wrapping_sub(rc_bound);
                                    rc.code = rc.code.wrapping_sub(rc_bound);
                                    (*coder).rep_len_decoder.choice2 =
                                        (*coder).rep_len_decoder.choice2
                                            - ((*coder).rep_len_decoder.choice2 >> RC_MOVE_BITS);
                                    symbol = 1;
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh107 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh107 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as u32,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize] as u32)
                                                .wrapping_add(
                                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                        (*coder).rep_len_decoder.high
                                                            [symbol as usize]
                                                            as u32,
                                                    ) >> RC_MOVE_BITS,
                                                )
                                                as probability
                                                as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            (*coder).rep_len_decoder.high[symbol as usize]
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    >> RC_MOVE_BITS);
                                        symbol = (symbol << 1).wrapping_add(1);
                                    }
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh108 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh108 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as u32,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize] as u32)
                                                .wrapping_add(
                                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                        (*coder).rep_len_decoder.high
                                                            [symbol as usize]
                                                            as u32,
                                                    ) >> RC_MOVE_BITS,
                                                )
                                                as probability
                                                as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            (*coder).rep_len_decoder.high[symbol as usize]
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    >> RC_MOVE_BITS);
                                        symbol = (symbol << 1).wrapping_add(1);
                                    }
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh109 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh109 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as u32,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize] as u32)
                                                .wrapping_add(
                                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                        (*coder).rep_len_decoder.high
                                                            [symbol as usize]
                                                            as u32,
                                                    ) >> RC_MOVE_BITS,
                                                )
                                                as probability
                                                as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            (*coder).rep_len_decoder.high[symbol as usize]
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    >> RC_MOVE_BITS);
                                        symbol = (symbol << 1).wrapping_add(1);
                                    }
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh110 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh110 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as u32,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize] as u32)
                                                .wrapping_add(
                                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                        (*coder).rep_len_decoder.high
                                                            [symbol as usize]
                                                            as u32,
                                                    ) >> RC_MOVE_BITS,
                                                )
                                                as probability
                                                as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            (*coder).rep_len_decoder.high[symbol as usize]
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    >> RC_MOVE_BITS);
                                        symbol = (symbol << 1).wrapping_add(1);
                                    }
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh111 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh111 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as u32,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize] as u32)
                                                .wrapping_add(
                                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                        (*coder).rep_len_decoder.high
                                                            [symbol as usize]
                                                            as u32,
                                                    ) >> RC_MOVE_BITS,
                                                )
                                                as probability
                                                as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            (*coder).rep_len_decoder.high[symbol as usize]
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    >> RC_MOVE_BITS);
                                        symbol = (symbol << 1).wrapping_add(1);
                                    }
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh112 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh112 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as u32,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize] as u32)
                                                .wrapping_add(
                                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                        (*coder).rep_len_decoder.high
                                                            [symbol as usize]
                                                            as u32,
                                                    ) >> RC_MOVE_BITS,
                                                )
                                                as probability
                                                as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            (*coder).rep_len_decoder.high[symbol as usize]
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    >> RC_MOVE_BITS);
                                        symbol = (symbol << 1).wrapping_add(1);
                                    }
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh113 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh113 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as u32,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize] as u32)
                                                .wrapping_add(
                                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                        (*coder).rep_len_decoder.high
                                                            [symbol as usize]
                                                            as u32,
                                                    ) >> RC_MOVE_BITS,
                                                )
                                                as probability
                                                as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            (*coder).rep_len_decoder.high[symbol as usize]
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    >> RC_MOVE_BITS);
                                        symbol = (symbol << 1).wrapping_add(1);
                                    }
                                    if rc.range < RC_TOP_VALUE as u32 {
                                        rc.range <<= RC_SHIFT_BITS;
                                        let fresh114 = rc_in_ptr;
                                        rc_in_ptr = rc_in_ptr.offset(1);
                                        rc.code = rc.code << RC_SHIFT_BITS | *fresh114 as u32;
                                    }
                                    rc_bound = (rc.range >> RC_BIT_MODEL_TOTAL_BITS).wrapping_mul(
                                        (*coder).rep_len_decoder.high[symbol as usize] as u32,
                                    );
                                    if rc.code < rc_bound {
                                        rc.range = rc_bound;
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            ((*coder).rep_len_decoder.high[symbol as usize] as u32)
                                                .wrapping_add(
                                                    RC_BIT_MODEL_TOTAL.wrapping_sub(
                                                        (*coder).rep_len_decoder.high
                                                            [symbol as usize]
                                                            as u32,
                                                    ) >> RC_MOVE_BITS,
                                                )
                                                as probability
                                                as probability;
                                        symbol <<= 1;
                                    } else {
                                        rc.range = rc.range.wrapping_sub(rc_bound);
                                        rc.code = rc.code.wrapping_sub(rc_bound);
                                        (*coder).rep_len_decoder.high[symbol as usize] =
                                            (*coder).rep_len_decoder.high[symbol as usize]
                                                - ((*coder).rep_len_decoder.high[symbol as usize]
                                                    >> RC_MOVE_BITS);
                                        symbol = (symbol << 1).wrapping_add(1);
                                    }
                                    symbol = symbol.wrapping_add(
                                        (-((1_i32) << 8) + 2 + ((1) << 3) + ((1) << 3)) as u32,
                                    );
                                    len = symbol;
                                }
                            }
                        }
                    }
                    if !(dict_repeat(&raw mut dict, rep0, &raw mut len) as c_long != 0) {
                        continue;
                    }
                    (*coder).sequence = SEQ_COPY;
                    current_block = 4609795085482299213;
                    continue 'c_9380;
                }
            },
            16690975975023747857 => {
                symbol = 1;
                current_block = 2467942631393454738;
                continue;
            }
            15498320742470848828 => {
                state = (if state < LIT_STATES {
                    STATE_LIT_LONGREP
                } else {
                    STATE_NONLIT_REP
                }) as u32;
                current_block = 12043352250568755004;
                continue;
            }
            8485842003490715114 => {
                symbol = 1;
                current_block = 592696588731961849;
                continue;
            }
            _ => {}
        }
        if eopm_is_valid {
            current_block = 7073645523065812117;
            continue;
        }
        ret_0 = LZMA_DATA_ERROR;
        current_block = 4609795085482299213;
    }
    (*dictptr).full = dict.full;
    (*coder).rc = rc;
    *in_pos = rc_in_ptr.offset_from(in_0) as size_t;
    (*coder).state = state as lzma_lzma_state;
    (*coder).rep0 = rep0;
    (*coder).rep1 = rep1;
    (*coder).rep2 = rep2;
    (*coder).rep3 = rep3;
    (*coder).probs = probs;
    (*coder).symbol = symbol;
    (*coder).limit = limit;
    (*coder).offset = offset;
    (*coder).len = len;
    if (*coder).uncompressed_size != LZMA_VLI_UNKNOWN {
        (*coder).uncompressed_size = (*coder)
            .uncompressed_size
            .wrapping_sub(dict.pos.wrapping_sub(dict_start) as lzma_vli);
        if (*coder).uncompressed_size == 0
            && ret_0 == LZMA_OK
            && ((*coder).sequence == SEQ_LITERAL_WRITE
                || (*coder).sequence == SEQ_SHORTREP
                || (*coder).sequence == SEQ_COPY)
        {
            ret_0 = LZMA_DATA_ERROR;
        }
    }
    if ret_0 == LZMA_STREAM_END {
        (*coder).rc.range = UINT32_MAX;
        (*coder).rc.code = 0;
        (*coder).rc.init_bytes_left = 5;
        (*coder).sequence = SEQ_IS_MATCH;
    }
    return ret_0;
}
unsafe extern "C" fn lzma_decoder_uncompressed(
    coder_ptr: *mut c_void,
    uncompressed_size: lzma_vli,
    allow_eopm: bool,
) {
    let coder: *mut lzma_lzma1_decoder = coder_ptr as *mut lzma_lzma1_decoder;
    (*coder).uncompressed_size = uncompressed_size;
    (*coder).allow_eopm = allow_eopm;
}
unsafe extern "C" fn lzma_decoder_reset(coder_ptr: *mut c_void, opt: *const c_void) {
    let coder: *mut lzma_lzma1_decoder = coder_ptr as *mut lzma_lzma1_decoder;
    let options: *const lzma_options_lzma = opt as *const lzma_options_lzma;
    (*coder).pos_mask = (1u32 << (*options).pb).wrapping_sub(1) as u32;
    literal_init(
        &raw mut (*coder).literal as *mut probability,
        (*options).lc,
        (*options).lp,
    );
    (*coder).literal_context_bits = (*options).lc;
    (*coder).literal_mask = (0x100u32 << (*options).lp).wrapping_sub(0x100 >> (*options).lc);
    (*coder).state = STATE_LIT_LIT;
    (*coder).rep0 = 0;
    (*coder).rep1 = 0;
    (*coder).rep2 = 0;
    (*coder).rep3 = 0;
    (*coder).pos_mask = (1u32 << (*options).pb).wrapping_sub(1) as u32;
    (*coder).rc.range = UINT32_MAX;
    (*coder).rc.code = 0;
    (*coder).rc.init_bytes_left = 5;
    let mut i: u32 = 0;
    while i < STATES {
        let mut j: u32 = 0;
        while j <= (*coder).pos_mask {
            (*coder).is_match[i as usize][j as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
            (*coder).is_rep0_long[i as usize][j as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            j += 1;
        }
        (*coder).is_rep[i as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        (*coder).is_rep0[i as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        (*coder).is_rep1[i as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        (*coder).is_rep2[i as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        i += 1;
    }
    let mut i_0: u32 = 0;
    while i_0 < DIST_STATES {
        let mut bt_i: u32 = 0;
        while bt_i < ((1) << 6) as u32 {
            (*coder).dist_slot[i_0 as usize][bt_i as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            bt_i += 1;
        }
        i_0 += 1;
    }
    let mut i_1: u32 = 0;
    while i_1 < (FULL_DISTANCES - DIST_MODEL_END) as u32 {
        (*coder).pos_special[i_1 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        i_1 += 1;
    }
    let mut bt_i_0: u32 = 0;
    while bt_i_0 < ((1) << 4) as u32 {
        (*coder).pos_align[bt_i_0 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        bt_i_0 += 1;
    }
    let num_pos_states: u32 = (1) << (*options).pb;
    (*coder).match_len_decoder.choice = (RC_BIT_MODEL_TOTAL >> 1) as probability;
    (*coder).match_len_decoder.choice2 = (RC_BIT_MODEL_TOTAL >> 1) as probability;
    (*coder).rep_len_decoder.choice = (RC_BIT_MODEL_TOTAL >> 1) as probability;
    (*coder).rep_len_decoder.choice2 = (RC_BIT_MODEL_TOTAL >> 1) as probability;
    let mut pos_state: u32 = 0;
    while pos_state < num_pos_states {
        let mut bt_i_1: u32 = 0;
        while bt_i_1 < ((1) << 3) as u32 {
            (*coder).match_len_decoder.low[pos_state as usize][bt_i_1 as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            bt_i_1 += 1;
        }
        let mut bt_i_2: u32 = 0;
        while bt_i_2 < ((1) << 3) as u32 {
            (*coder).match_len_decoder.mid[pos_state as usize][bt_i_2 as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            bt_i_2 += 1;
        }
        let mut bt_i_3: u32 = 0;
        while bt_i_3 < ((1) << 3) as u32 {
            (*coder).rep_len_decoder.low[pos_state as usize][bt_i_3 as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            bt_i_3 += 1;
        }
        let mut bt_i_4: u32 = 0;
        while bt_i_4 < ((1) << 3) as u32 {
            (*coder).rep_len_decoder.mid[pos_state as usize][bt_i_4 as usize] =
                (RC_BIT_MODEL_TOTAL >> 1) as probability;
            bt_i_4 += 1;
        }
        pos_state += 1;
    }
    let mut bt_i_5: u32 = 0;
    while bt_i_5 < ((1) << 8) as u32 {
        (*coder).match_len_decoder.high[bt_i_5 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        bt_i_5 += 1;
    }
    let mut bt_i_6: u32 = 0;
    while bt_i_6 < ((1) << 8) as u32 {
        (*coder).rep_len_decoder.high[bt_i_6 as usize] = (RC_BIT_MODEL_TOTAL >> 1) as probability;
        bt_i_6 += 1;
    }
    (*coder).sequence = SEQ_IS_MATCH;
    (*coder).probs = core::ptr::null_mut();
    (*coder).symbol = 0;
    (*coder).limit = 0;
    (*coder).offset = 0;
    (*coder).len = 0;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_decoder_create(
    lz: *mut lzma_lz_decoder,
    allocator: *const lzma_allocator,
    options: *const lzma_options_lzma,
    lz_options: *mut lzma_lz_options,
) -> lzma_ret {
    if (*lz).coder.is_null() {
        (*lz).coder = lzma_alloc(core::mem::size_of::<lzma_lzma1_decoder>(), allocator);
        if (*lz).coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*lz).code = Some(
            lzma_decode
                as unsafe extern "C" fn(
                    *mut c_void,
                    *mut lzma_dict,
                    *const u8,
                    *mut size_t,
                    size_t,
                ) -> lzma_ret,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut c_void,
                    *mut lzma_dict,
                    *const u8,
                    *mut size_t,
                    size_t,
                ) -> lzma_ret,
            >;
        (*lz).reset =
            Some(lzma_decoder_reset as unsafe extern "C" fn(*mut c_void, *const c_void) -> ())
                as Option<unsafe extern "C" fn(*mut c_void, *const c_void) -> ()>;
        (*lz).set_uncompressed = Some(
            lzma_decoder_uncompressed as unsafe extern "C" fn(*mut c_void, lzma_vli, bool) -> (),
        )
            as Option<unsafe extern "C" fn(*mut c_void, lzma_vli, bool) -> ()>;
    }
    (*lz_options).dict_size = (*options).dict_size as size_t;
    (*lz_options).preset_dict = (*options).preset_dict;
    (*lz_options).preset_dict_size = (*options).preset_dict_size as size_t;
    return LZMA_OK;
}
unsafe extern "C" fn lzma_decoder_init(
    lz: *mut lzma_lz_decoder,
    allocator: *const lzma_allocator,
    id: lzma_vli,
    options: *const c_void,
    lz_options: *mut lzma_lz_options,
) -> lzma_ret {
    if !is_lclppb_valid(options as *const lzma_options_lzma) {
        return LZMA_PROG_ERROR;
    }
    let mut uncomp_size: lzma_vli = LZMA_VLI_UNKNOWN;
    let mut allow_eopm: bool = true;
    if id == LZMA_FILTER_LZMA1EXT {
        let opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
        if (*opt).ext_flags & !(LZMA_LZMA1EXT_ALLOW_EOPM as u32) != 0 {
            return LZMA_OPTIONS_ERROR;
        }
        uncomp_size = ((*opt).ext_size_low as u64).wrapping_add(((*opt).ext_size_high as u64) << 32)
            as lzma_vli;
        allow_eopm = (*opt).ext_flags & LZMA_LZMA1EXT_ALLOW_EOPM as u32 != 0
            || uncomp_size == LZMA_VLI_UNKNOWN;
    }
    let ret_: lzma_ret = lzma_lzma_decoder_create(
        lz,
        allocator,
        options as *const lzma_options_lzma,
        lz_options,
    );
    if ret_ != LZMA_OK {
        return ret_;
    }
    lzma_decoder_reset((*lz).coder, options);
    lzma_decoder_uncompressed((*lz).coder, uncomp_size, allow_eopm);
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    return lzma_lz_decoder_init(
        next,
        allocator,
        filters,
        Some(
            lzma_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_lz_decoder,
                    *const lzma_allocator,
                    lzma_vli,
                    *const c_void,
                    *mut lzma_lz_options,
                ) -> lzma_ret,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_lclppb_decode(
    options: *mut lzma_options_lzma,
    mut byte: u8,
) -> bool {
    if byte > (4 * 5 + 4) * 9 + 8 {
        return true;
    }
    (*options).pb = (byte / (9 * 5)) as u32;
    byte = (byte as u32).wrapping_sub((*options).pb.wrapping_mul(9u32).wrapping_mul(5)) as u8;
    (*options).lp = (byte / 9) as u32;
    (*options).lc = (byte as u32).wrapping_sub((*options).lp.wrapping_mul(9));
    return (*options).lc.wrapping_add((*options).lp) > LZMA_LCLP_MAX;
}
#[no_mangle]
pub extern "C" fn lzma_lzma_decoder_memusage_nocheck(options: *const c_void) -> u64 {
    return unsafe {
        let opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
        (core::mem::size_of::<lzma_lzma1_decoder>() as u64)
            .wrapping_add(lzma_lz_decoder_memusage((*opt).dict_size as size_t))
    };
}
#[no_mangle]
pub extern "C" fn lzma_lzma_decoder_memusage(options: *const c_void) -> u64 {
    if !unsafe { is_lclppb_valid(options as *const lzma_options_lzma) } {
        return UINT64_MAX;
    }
    return lzma_lzma_decoder_memusage_nocheck(options);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_props_decode(
    options: *mut *mut c_void,
    allocator: *const lzma_allocator,
    props: *const u8,
    props_size: size_t,
) -> lzma_ret {
    if props_size != 5 {
        return LZMA_OPTIONS_ERROR;
    }
    let opt: *mut lzma_options_lzma =
        lzma_alloc(core::mem::size_of::<lzma_options_lzma>(), allocator) as *mut lzma_options_lzma;
    if opt.is_null() {
        return LZMA_MEM_ERROR;
    }
    if lzma_lzma_lclppb_decode(opt, *props.offset(0)) {
        lzma_free(opt as *mut c_void, allocator);
        return LZMA_OPTIONS_ERROR;
    } else {
        (*opt).dict_size = read32le(props.offset(1));
        (*opt).preset_dict = ::core::ptr::null::<u8>();
        (*opt).preset_dict_size = 0;
        *options = opt as *mut c_void;
        return LZMA_OK;
    };
}
