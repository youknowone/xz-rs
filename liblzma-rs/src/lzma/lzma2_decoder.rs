use crate::types::*;
use core::ffi::{c_uint, c_void};
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
    fn lzma_lzma_decoder_create(
        lz: *mut lzma_lz_decoder,
        allocator: *const lzma_allocator,
        opt: *const lzma_options_lzma,
        lz_options: *mut lzma_lz_options,
    ) -> lzma_ret;
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
pub struct lzma_lzma2_coder {
    pub sequence: sequence,
    pub next_sequence: sequence,
    pub lzma: lzma_lz_decoder,
    pub uncompressed_size: size_t,
    pub compressed_size: size_t,
    pub need_properties: bool,
    pub need_dictionary_reset: bool,
    pub options: lzma_options_lzma,
}
pub type sequence = c_uint;
pub const SEQ_COPY: sequence = 7;
pub const SEQ_LZMA: sequence = 6;
pub const SEQ_PROPERTIES: sequence = 5;
pub const SEQ_COMPRESSED_1: sequence = 4;
pub const SEQ_COMPRESSED_0: sequence = 3;
pub const SEQ_UNCOMPRESSED_2: sequence = 2;
pub const SEQ_UNCOMPRESSED_1: sequence = 1;
pub const SEQ_CONTROL: sequence = 0;
pub const LZMA_LZ_DECODER_INIT: lzma_lz_decoder = lzma_lz_decoder {
    coder: core::ptr::null_mut(),
    code: None,
    reset: None,
    set_uncompressed: None,
    end: None,
};
#[inline]
unsafe extern "C" fn dict_write(
    dict: *mut lzma_dict,
    in_0: *const u8,
    in_pos: *mut size_t,
    mut in_size: size_t,
    left: *mut size_t,
) {
    if in_size.wrapping_sub(*in_pos) > *left {
        in_size = (*in_pos).wrapping_add(*left);
    }
    *left = (*left).wrapping_sub(lzma_bufcpy(
        in_0,
        in_pos,
        in_size,
        (*dict).buf,
        &raw mut (*dict).pos,
        (*dict).limit,
    ));
    if !(*dict).has_wrapped {
        (*dict).full = (*dict).pos.wrapping_sub(LZ_DICT_INIT_POS as size_t);
    }
}
#[inline]
unsafe extern "C" fn dict_reset(dict: *mut lzma_dict) {
    (*dict).need_reset = true;
}
unsafe extern "C" fn lzma2_decode(
    coder_ptr: *mut c_void,
    dict: *mut lzma_dict,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
) -> lzma_ret {
    let coder: *mut lzma_lzma2_coder = coder_ptr as *mut lzma_lzma2_coder;
    while *in_pos < in_size || (*coder).sequence == SEQ_LZMA {
        match (*coder).sequence {
            0 => {
                let control: u32 = *in_0.offset(*in_pos as isize) as u32;
                *in_pos = (*in_pos).wrapping_add(1);
                if control == 0 {
                    return LZMA_STREAM_END;
                }
                if control >= 0xe0 || control == 1 {
                    (*coder).need_properties = true;
                    (*coder).need_dictionary_reset = true;
                } else if (*coder).need_dictionary_reset {
                    return LZMA_DATA_ERROR;
                }
                if control >= 0x80 {
                    (*coder).uncompressed_size = ((control & 0x1f) << 16) as size_t;
                    (*coder).sequence = SEQ_UNCOMPRESSED_1;
                    if control >= 0xc0 {
                        (*coder).need_properties = false;
                        (*coder).next_sequence = SEQ_PROPERTIES;
                    } else if (*coder).need_properties {
                        return LZMA_DATA_ERROR;
                    } else {
                        (*coder).next_sequence = SEQ_LZMA;
                        if control >= 0xa0 {
                            (*coder).lzma.reset.unwrap()(
                                (*coder).lzma.coder,
                                &raw mut (*coder).options as *const c_void,
                            );
                        }
                    }
                } else {
                    if control > 2 {
                        return LZMA_DATA_ERROR;
                    }
                    (*coder).sequence = SEQ_COMPRESSED_0;
                    (*coder).next_sequence = SEQ_COPY;
                }
                if (*coder).need_dictionary_reset {
                    (*coder).need_dictionary_reset = false;
                    dict_reset(dict);
                    return LZMA_OK;
                }
            }
            1 => {
                (*coder).uncompressed_size = (*coder)
                    .uncompressed_size
                    .wrapping_add(((*in_0.offset(*in_pos as isize) as u32) << 8) as size_t);
                *in_pos += 1;
                (*coder).sequence = SEQ_UNCOMPRESSED_2;
            }
            2 => {
                (*coder).uncompressed_size = (*coder).uncompressed_size.wrapping_add(
                    u32::from(*in_0.offset(*in_pos as isize)).wrapping_add(1) as size_t,
                );
                *in_pos += 1;
                (*coder).sequence = SEQ_COMPRESSED_0;
                (*coder).lzma.set_uncompressed.unwrap()(
                    (*coder).lzma.coder,
                    (*coder).uncompressed_size as lzma_vli,
                    false,
                );
            }
            3 => {
                (*coder).compressed_size = ((*in_0.offset(*in_pos as isize) as u32) << 8) as size_t;
                *in_pos += 1;
                (*coder).sequence = SEQ_COMPRESSED_1;
            }
            4 => {
                (*coder).compressed_size = (*coder).compressed_size.wrapping_add(
                    u32::from(*in_0.offset(*in_pos as isize)).wrapping_add(1) as size_t,
                );
                *in_pos += 1;
                (*coder).sequence = (*coder).next_sequence as sequence;
            }
            5 => {
                let prop_byte = *in_0.offset(*in_pos as isize);
                *in_pos += 1;
                if lzma_lzma_lclppb_decode(&raw mut (*coder).options, prop_byte) {
                    return LZMA_DATA_ERROR;
                }
                (*coder).lzma.reset.unwrap()(
                    (*coder).lzma.coder,
                    &raw mut (*coder).options as *const c_void,
                );
                (*coder).sequence = SEQ_LZMA;
            }
            6 => {
                let in_start: size_t = *in_pos;
                let ret: lzma_ret =
                    (*coder).lzma.code.unwrap()((*coder).lzma.coder, dict, in_0, in_pos, in_size);
                let in_used: size_t = (*in_pos).wrapping_sub(in_start);
                if in_used > (*coder).compressed_size {
                    return LZMA_DATA_ERROR;
                }
                (*coder).compressed_size = (*coder).compressed_size.wrapping_sub(in_used);
                if ret != LZMA_STREAM_END {
                    return ret;
                }
                if (*coder).compressed_size != 0 {
                    return LZMA_DATA_ERROR;
                }
                (*coder).sequence = SEQ_CONTROL;
            }
            7 => {
                dict_write(
                    dict,
                    in_0,
                    in_pos,
                    in_size,
                    &raw mut (*coder).compressed_size,
                );
                if (*coder).compressed_size != 0 {
                    return LZMA_OK;
                }
                (*coder).sequence = SEQ_CONTROL;
            }
            _ => return LZMA_PROG_ERROR,
        }
    }
    LZMA_OK
}
unsafe extern "C" fn lzma2_decoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_lzma2_coder = coder_ptr as *mut lzma_lzma2_coder;
    lzma_free((*coder).lzma.coder, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn lzma2_decoder_init(
    lz: *mut lzma_lz_decoder,
    allocator: *const lzma_allocator,
    _id: lzma_vli,
    opt: *const c_void,
    lz_options: *mut lzma_lz_options,
) -> lzma_ret {
    let mut coder: *mut lzma_lzma2_coder = (*lz).coder as *mut lzma_lzma2_coder;
    if coder.is_null() {
        coder = lzma_alloc(core::mem::size_of::<lzma_lzma2_coder>(), allocator)
            as *mut lzma_lzma2_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*lz).coder = coder as *mut c_void;
        (*lz).code = Some(
            lzma2_decode
                as unsafe extern "C" fn(
                    *mut c_void,
                    *mut lzma_dict,
                    *const u8,
                    *mut size_t,
                    size_t,
                ) -> lzma_ret,
        );
        (*lz).end = Some(
            lzma2_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        );
        (*coder).lzma = LZMA_LZ_DECODER_INIT;
    }
    let options: *const lzma_options_lzma = opt as *const lzma_options_lzma;
    (*coder).sequence = SEQ_CONTROL;
    (*coder).need_properties = true;
    (*coder).need_dictionary_reset =
        (*options).preset_dict.is_null() || (*options).preset_dict_size == 0;
    lzma_lzma_decoder_create(&raw mut (*coder).lzma, allocator, options, lz_options)
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    lzma_lz_decoder_init(
        next,
        allocator,
        filters,
        Some(
            lzma2_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_lz_decoder,
                    *const lzma_allocator,
                    lzma_vli,
                    *const c_void,
                    *mut lzma_lz_options,
                ) -> lzma_ret,
        ),
    )
}
#[no_mangle]
pub extern "C" fn lzma_lzma2_decoder_memusage(options: *const c_void) -> u64 {
    (core::mem::size_of::<lzma_lzma2_coder>() as u64)
        .wrapping_add(unsafe { lzma_lzma_decoder_memusage_nocheck(options) })
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_props_decode(
    options: *mut *mut c_void,
    allocator: *const lzma_allocator,
    props: *const u8,
    props_size: size_t,
) -> lzma_ret {
    if props_size != 1 {
        return LZMA_OPTIONS_ERROR;
    }
    if *props & 0xc0 != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    if *props > 40 {
        return LZMA_OPTIONS_ERROR;
    }
    let opt: *mut lzma_options_lzma =
        lzma_alloc(core::mem::size_of::<lzma_options_lzma>(), allocator) as *mut lzma_options_lzma;
    if opt.is_null() {
        return LZMA_MEM_ERROR;
    }
    if *props == 40 {
        (*opt).dict_size = UINT32_MAX;
    } else {
        (*opt).dict_size = 2u32 | (u32::from(*props) & 1);
        (*opt).dict_size <<= u32::from(*props).wrapping_div(2).wrapping_add(11);
    }
    (*opt).preset_dict = core::ptr::null();
    (*opt).preset_dict_size = 0;
    *options = opt as *mut c_void;
    LZMA_OK
}
