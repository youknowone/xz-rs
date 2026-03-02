use crate::types::*;
use core::ffi::{c_uint, c_void};
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
    fn lzma_lzma_encoder_create(
        coder_ptr: *mut *mut c_void,
        allocator: *const lzma_allocator,
        id: lzma_vli,
        options: *const lzma_options_lzma,
        lz_options: *mut lzma_lz_options,
    ) -> lzma_ret;
    fn lzma_lzma_encoder_reset(
        coder: *mut lzma_lzma1_encoder,
        options: *const lzma_options_lzma,
    ) -> lzma_ret;
    fn lzma_lzma_encode(
        coder: *mut lzma_lzma1_encoder,
        mf: *mut lzma_mf,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
        read_limit: u32,
    ) -> lzma_ret;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lzma2_coder {
    pub sequence: C2RustUnnamed,
    pub lzma: *mut c_void,
    pub opt_cur: lzma_options_lzma,
    pub need_properties: bool,
    pub need_state_reset: bool,
    pub need_dictionary_reset: bool,
    pub uncompressed_size: size_t,
    pub compressed_size: size_t,
    pub buf_pos: size_t,
    pub buf: [u8; 65542],
}
pub type C2RustUnnamed = c_uint;
pub const SEQ_UNCOMPRESSED_COPY: C2RustUnnamed = 4;
pub const SEQ_UNCOMPRESSED_HEADER: C2RustUnnamed = 3;
pub const SEQ_LZMA_COPY: C2RustUnnamed = 2;
pub const SEQ_LZMA_ENCODE: C2RustUnnamed = 1;
pub const SEQ_INIT: C2RustUnnamed = 0;
#[inline]
unsafe extern "C" fn mf_unencoded(mf: *const lzma_mf) -> u32 {
    (*mf)
        .write_pos
        .wrapping_sub((*mf).read_pos)
        .wrapping_add((*mf).read_ahead)
}
#[inline]
unsafe extern "C" fn mf_read(
    mf: *mut lzma_mf,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
    left: *mut size_t,
) {
    let out_avail: size_t = out_size.wrapping_sub(*out_pos);
    let copy_size: size_t = if out_avail < *left { out_avail } else { *left };
    core::ptr::copy_nonoverlapping(
        (*mf)
            .buffer
            .offset((*mf).read_pos as isize)
            .offset(-(*left as isize)) as *const u8,
        out.offset(*out_pos as isize) as *mut u8,
        copy_size,
    );
    *out_pos = (*out_pos).wrapping_add(copy_size);
    *left = (*left).wrapping_sub(copy_size);
}
pub const LZMA2_UNCOMPRESSED_MAX: c_uint = 1u32 << 21;
pub const LZMA2_HEADER_MAX: u32 = 6;
unsafe extern "C" fn lzma2_header_lzma(coder: *mut lzma_lzma2_coder) {
    let mut pos: size_t = 0;
    if (*coder).need_properties {
        pos = 0;
        if (*coder).need_dictionary_reset {
            (*coder).buf[pos as usize] = (0x80 + ((3) << 5)) as u8;
        } else {
            (*coder).buf[pos as usize] = (0x80 + ((2) << 5)) as u8;
        }
    } else {
        pos = 1;
        if (*coder).need_state_reset {
            (*coder).buf[pos as usize] = (0x80 + (1 << 5)) as u8;
        } else {
            (*coder).buf[pos as usize] = 0x80 as u8;
        }
    }
    (*coder).buf_pos = pos;
    let mut size: size_t = (*coder).uncompressed_size.wrapping_sub(1);
    (*coder).buf[pos as usize] =
        ((*coder).buf[pos as usize] as size_t).wrapping_add(size >> 16) as u8;
    pos += 1;
    (*coder).buf[pos as usize] = (size >> 8 & 0xff) as u8;
    pos += 1;
    (*coder).buf[pos as usize] = (size & 0xff) as u8;
    pos += 1;
    size = (*coder).compressed_size.wrapping_sub(1);
    (*coder).buf[pos as usize] = (size >> 8) as u8;
    pos += 1;
    (*coder).buf[pos as usize] = (size & 0xff) as u8;
    pos += 1;
    if (*coder).need_properties {
        lzma_lzma_lclppb_encode(
            &raw mut (*coder).opt_cur,
            (&raw mut (*coder).buf as *mut u8).offset(pos as isize),
        );
    }
    (*coder).need_properties = false;
    (*coder).need_state_reset = false;
    (*coder).need_dictionary_reset = false;
    (*coder).compressed_size = (*coder)
        .compressed_size
        .wrapping_add(LZMA2_HEADER_MAX as size_t);
}
unsafe extern "C" fn lzma2_header_uncompressed(coder: *mut lzma_lzma2_coder) {
    if (*coder).need_dictionary_reset {
        (*coder).buf[0] = 1;
    } else {
        (*coder).buf[0] = 2;
    }
    (*coder).need_dictionary_reset = false;
    (*coder).buf[1] = ((*coder).uncompressed_size.wrapping_sub(1) >> 8) as u8;
    (*coder).buf[2] = ((*coder).uncompressed_size.wrapping_sub(1) & 0xff) as u8;
    (*coder).buf_pos = 0;
}
unsafe extern "C" fn lzma2_encode(
    coder_ptr: *mut c_void,
    mf: *mut lzma_mf,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    let coder: *mut lzma_lzma2_coder = coder_ptr as *mut lzma_lzma2_coder;
    while *out_pos < out_size {
        let mut current_block_45: u64;
        match (*coder).sequence {
            0 => {
                if mf_unencoded(mf) == 0 {
                    if (*mf).action == LZMA_FINISH {
                        *out.offset(*out_pos as isize) = 0;
                        *out_pos += 1;
                    }
                    return if (*mf).action == LZMA_RUN {
                        LZMA_OK
                    } else {
                        LZMA_STREAM_END
                    };
                }
                if (*coder).need_state_reset {
                    let ret_: lzma_ret = lzma_lzma_encoder_reset(
                        (*coder).lzma as *mut lzma_lzma1_encoder,
                        &raw mut (*coder).opt_cur,
                    );
                    if ret_ != LZMA_OK {
                        return ret_;
                    }
                }
                (*coder).uncompressed_size = 0;
                (*coder).compressed_size = 0;
                (*coder).sequence = SEQ_LZMA_ENCODE;
                current_block_45 = 2979737022853876585;
            }
            1 => {
                current_block_45 = 2979737022853876585;
            }
            2 => {
                current_block_45 = 13410404938545238636;
            }
            3 => {
                lzma_bufcpy(
                    &raw mut (*coder).buf as *mut u8,
                    &raw mut (*coder).buf_pos,
                    LZMA2_HEADER_UNCOMPRESSED as size_t,
                    out,
                    out_pos,
                    out_size,
                );
                if (*coder).buf_pos != LZMA2_HEADER_UNCOMPRESSED as size_t {
                    return LZMA_OK;
                }
                (*coder).sequence = SEQ_UNCOMPRESSED_COPY;
                current_block_45 = 10903800704467975402;
            }
            4 => {
                current_block_45 = 10903800704467975402;
            }
            _ => {
                current_block_45 = 11743904203796629665;
            }
        }
        match current_block_45 {
            10903800704467975402 => {
                mf_read(
                    mf,
                    out,
                    out_pos,
                    out_size,
                    &raw mut (*coder).uncompressed_size,
                );
                if (*coder).uncompressed_size != 0 {
                    return LZMA_OK;
                }
                (*coder).sequence = SEQ_INIT;
                current_block_45 = 11743904203796629665;
            }
            2979737022853876585 => {
                let left: u32 = (LZMA2_UNCOMPRESSED_MAX as size_t)
                    .wrapping_sub((*coder).uncompressed_size)
                    as u32;
                let mut limit: u32 = 0;
                if left < (*mf).match_len_max {
                    limit = 0;
                } else {
                    limit = (*mf)
                        .read_pos
                        .wrapping_sub((*mf).read_ahead)
                        .wrapping_add(left)
                        .wrapping_sub((*mf).match_len_max);
                }
                let read_start: u32 = (*mf).read_pos.wrapping_sub((*mf).read_ahead);
                let ret: lzma_ret = lzma_lzma_encode(
                    (*coder).lzma as *mut lzma_lzma1_encoder,
                    mf,
                    (&raw mut (*coder).buf as *mut u8).offset(LZMA2_HEADER_MAX as isize),
                    &raw mut (*coder).compressed_size,
                    LZMA2_CHUNK_MAX as size_t,
                    limit,
                );
                (*coder).uncompressed_size = (*coder).uncompressed_size.wrapping_add(
                    (*mf)
                        .read_pos
                        .wrapping_sub((*mf).read_ahead)
                        .wrapping_sub(read_start) as size_t,
                );
                if ret != LZMA_STREAM_END {
                    return LZMA_OK;
                }
                if (*coder).compressed_size >= (*coder).uncompressed_size {
                    (*coder).uncompressed_size = (*coder)
                        .uncompressed_size
                        .wrapping_add((*mf).read_ahead as size_t);
                    (*mf).read_ahead = 0;
                    lzma2_header_uncompressed(coder);
                    (*coder).need_state_reset = true;
                    (*coder).sequence = SEQ_UNCOMPRESSED_HEADER;
                    current_block_45 = 11743904203796629665;
                } else {
                    lzma2_header_lzma(coder);
                    (*coder).sequence = SEQ_LZMA_COPY;
                    current_block_45 = 13410404938545238636;
                }
            }
            _ => {}
        }
        match current_block_45 {
            13410404938545238636 => {
                lzma_bufcpy(
                    &raw mut (*coder).buf as *mut u8,
                    &raw mut (*coder).buf_pos,
                    (*coder).compressed_size,
                    out,
                    out_pos,
                    out_size,
                );
                if (*coder).buf_pos != (*coder).compressed_size {
                    return LZMA_OK;
                }
                (*coder).sequence = SEQ_INIT;
            }
            _ => {}
        }
    }
    LZMA_OK
}
unsafe extern "C" fn lzma2_encoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_lzma2_coder = coder_ptr as *mut lzma_lzma2_coder;
    lzma_free((*coder).lzma, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn lzma2_encoder_options_update(
    coder_ptr: *mut c_void,
    filter: *const lzma_filter,
) -> lzma_ret {
    let coder: *mut lzma_lzma2_coder = coder_ptr as *mut lzma_lzma2_coder;
    if (*filter).options.is_null() || (*coder).sequence != SEQ_INIT {
        return LZMA_PROG_ERROR;
    }
    let opt: *const lzma_options_lzma = (*filter).options as *const lzma_options_lzma;
    if (*coder).opt_cur.lc != (*opt).lc
        || (*coder).opt_cur.lp != (*opt).lp
        || (*coder).opt_cur.pb != (*opt).pb
    {
        if (*opt).lc > LZMA_LCLP_MAX
            || (*opt).lp > LZMA_LCLP_MAX
            || (*opt).lc.wrapping_add((*opt).lp) > LZMA_LCLP_MAX
            || (*opt).pb > LZMA_PB_MAX
        {
            return LZMA_OPTIONS_ERROR;
        }
        (*coder).opt_cur.lc = (*opt).lc;
        (*coder).opt_cur.lp = (*opt).lp;
        (*coder).opt_cur.pb = (*opt).pb;
        (*coder).need_properties = true;
        (*coder).need_state_reset = true;
    }
    LZMA_OK
}
unsafe extern "C" fn lzma2_encoder_init(
    lz: *mut lzma_lz_encoder,
    allocator: *const lzma_allocator,
    _id: lzma_vli,
    options: *const c_void,
    lz_options: *mut lzma_lz_options,
) -> lzma_ret {
    if options.is_null() {
        return LZMA_PROG_ERROR;
    }
    let mut coder: *mut lzma_lzma2_coder = (*lz).coder as *mut lzma_lzma2_coder;
    if coder.is_null() {
        coder = lzma_alloc(core::mem::size_of::<lzma_lzma2_coder>(), allocator)
            as *mut lzma_lzma2_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*lz).coder = coder as *mut c_void;
        (*lz).code = Some(
            lzma2_encode
                as unsafe extern "C" fn(
                    *mut c_void,
                    *mut lzma_mf,
                    *mut u8,
                    *mut size_t,
                    size_t,
                ) -> lzma_ret,
        );
        (*lz).end = Some(
            lzma2_encoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        );
        (*lz).options_update = Some(
            lzma2_encoder_options_update
                as unsafe extern "C" fn(*mut c_void, *const lzma_filter) -> lzma_ret,
        );
        (*coder).lzma = core::ptr::null_mut();
    }
    (*coder).opt_cur = *(options as *const lzma_options_lzma);
    (*coder).sequence = SEQ_INIT;
    (*coder).need_properties = true;
    (*coder).need_state_reset = false;
    (*coder).need_dictionary_reset =
        (*coder).opt_cur.preset_dict.is_null() || (*coder).opt_cur.preset_dict_size == 0;
    let ret_: lzma_ret = lzma_lzma_encoder_create(
        &raw mut (*coder).lzma,
        allocator,
        0x21,
        &raw mut (*coder).opt_cur,
        lz_options,
    );
    if ret_ != LZMA_OK {
        return ret_;
    }
    if (*lz_options)
        .before_size
        .wrapping_add((*lz_options).dict_size)
        < LZMA2_CHUNK_MAX as size_t
    {
        (*lz_options).before_size =
            (LZMA2_CHUNK_MAX as size_t).wrapping_sub((*lz_options).dict_size);
    }
    LZMA_OK
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    lzma_lz_encoder_init(
        next,
        allocator,
        filters,
        Some(
            lzma2_encoder_init
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
pub extern "C" fn lzma_lzma2_encoder_memusage(options: *const c_void) -> u64 {
    let lzma_mem: u64 = unsafe { lzma_lzma_encoder_memusage(options) } as u64;
    if lzma_mem == UINT64_MAX {
        return UINT64_MAX;
    }
    (core::mem::size_of::<lzma_lzma2_coder>() as u64).wrapping_add(lzma_mem)
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_props_encode(options: *const c_void, out: *mut u8) -> lzma_ret {
    if options.is_null() {
        return LZMA_PROG_ERROR;
    }
    let opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
    let mut d: u32 = if (*opt).dict_size > 4096 {
        (*opt).dict_size
    } else {
        4096
    };
    d -= 1;
    d |= d >> 2;
    d |= d >> 3;
    d |= d >> 4;
    d |= d >> 8;
    d |= d >> 16;
    if d == UINT32_MAX {
        *out = 40;
    } else {
        *out = get_dist_slot(d.wrapping_add(1)).wrapping_sub(24) as u8;
    }
    LZMA_OK
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_block_size(options: *const c_void) -> u64 {
    let opt: *const lzma_options_lzma = options as *const lzma_options_lzma;
    if (*opt).dict_size < LZMA_DICT_SIZE_MIN as u32
        || (*opt).dict_size > (1u32 << 30).wrapping_add(1 << 29)
    {
        return UINT64_MAX;
    }
    if ((*opt).dict_size as u64).wrapping_mul(3) > 1 << 20 {
        ((*opt).dict_size as u64).wrapping_mul(3)
    } else {
        1 << 20
    }
}
