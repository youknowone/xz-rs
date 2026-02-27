use crate::types::*;
use core::ffi::{c_int, c_uint, c_void};
extern "C" {
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_bufcpy(
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> size_t;
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
    fn lzma_lzma_decoder_memusage_nocheck(options: *const c_void) -> u64;
    fn lzma_lzma_lclppb_decode(options: *mut lzma_options_lzma, byte: u8) -> bool;
    fn lzma_lzma_decoder_create(
        lz: *mut lzma_lz_decoder,
        allocator: *const lzma_allocator,
        opt: *const lzma_options_lzma,
        lz_options: *mut lzma_lz_options,
    ) -> lzma_ret;
}
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
pub const LZMA_RET_INTERNAL8: lzma_ret = 108;
pub const LZMA_RET_INTERNAL7: lzma_ret = 107;
pub const LZMA_RET_INTERNAL6: lzma_ret = 106;
pub const LZMA_RET_INTERNAL5: lzma_ret = 105;
pub const LZMA_RET_INTERNAL4: lzma_ret = 104;
pub const LZMA_RET_INTERNAL3: lzma_ret = 103;
pub const LZMA_RET_INTERNAL2: lzma_ret = 102;
pub const LZMA_RET_INTERNAL1: lzma_ret = 101;
pub const LZMA_SEEK_NEEDED: lzma_ret = 12;
pub const LZMA_PROG_ERROR: lzma_ret = 11;
pub const LZMA_BUF_ERROR: lzma_ret = 10;
pub const LZMA_DATA_ERROR: lzma_ret = 9;
pub const LZMA_OPTIONS_ERROR: lzma_ret = 8;
pub const LZMA_FORMAT_ERROR: lzma_ret = 7;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret = 6;
pub const LZMA_MEM_ERROR: lzma_ret = 5;
pub const LZMA_GET_CHECK: lzma_ret = 4;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret = 3;
pub const LZMA_NO_CHECK: lzma_ret = 2;
pub const LZMA_STREAM_END: lzma_ret = 1;
pub const LZMA_OK: lzma_ret = 0;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, size_t, size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    pub opaque: *mut c_void,
}
pub type lzma_next_coder = lzma_next_coder_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_next_coder_s {
    pub coder: *mut c_void,
    pub id: lzma_vli,
    pub init: uintptr_t,
    pub code: lzma_code_function,
    pub end: lzma_end_function,
    pub get_progress: Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64) -> ()>,
    pub get_check: Option<unsafe extern "C" fn(*const c_void) -> lzma_check>,
    pub memconfig: Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret>,
    pub update: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const lzma_allocator,
            *const lzma_filter,
            *const lzma_filter,
        ) -> lzma_ret,
    >,
    pub set_out_limit: Option<unsafe extern "C" fn(*mut c_void, *mut u64, u64) -> lzma_ret>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut c_void,
}
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
pub type lzma_end_function = Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>;
pub type lzma_code_function = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const lzma_allocator,
        *const u8,
        *mut size_t,
        size_t,
        *mut u8,
        *mut size_t,
        size_t,
        lzma_action,
    ) -> lzma_ret,
>;
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;
pub const LZMA_MODE_FAST: lzma_mode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_lzma {
    pub dict_size: u32,
    pub preset_dict: *const u8,
    pub preset_dict_size: u32,
    pub lc: u32,
    pub lp: u32,
    pub pb: u32,
    pub mode: lzma_mode,
    pub nice_len: u32,
    pub mf: lzma_match_finder,
    pub depth: u32,
    pub ext_flags: u32,
    pub ext_size_low: u32,
    pub ext_size_high: u32,
    pub reserved_int4: u32,
    pub reserved_int5: u32,
    pub reserved_int6: u32,
    pub reserved_int7: u32,
    pub reserved_int8: u32,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_info_s {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub options: *mut c_void,
}
pub type lzma_init_function = Option<
    unsafe extern "C" fn(
        *mut lzma_next_coder,
        *const lzma_allocator,
        *const lzma_filter_info,
    ) -> lzma_ret,
>;
pub type lzma_filter_info = lzma_filter_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_options {
    pub dict_size: size_t,
    pub preset_dict: *const u8,
    pub preset_dict_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_decoder {
    pub coder: *mut c_void,
    pub code: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut lzma_dict,
            *const u8,
            *mut size_t,
            size_t,
        ) -> lzma_ret,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut c_void, *const c_void) -> ()>,
    pub set_uncompressed: Option<unsafe extern "C" fn(*mut c_void, lzma_vli, bool) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_dict {
    pub buf: *mut u8,
    pub pos: size_t,
    pub full: size_t,
    pub limit: size_t,
    pub size: size_t,
    pub has_wrapped: bool,
    pub need_reset: bool,
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
pub const UINT32_MAX: c_uint = 4294967295;
pub const LZ_DICT_REPEAT_MAX: c_int = 288;
pub const LZ_DICT_INIT_POS: c_int = 2 * LZ_DICT_REPEAT_MAX;
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
                if control >= 0xe0 as u32 || control == 1 {
                    (*coder).need_properties = true;
                    (*coder).need_dictionary_reset = true;
                } else if (*coder).need_dictionary_reset {
                    return LZMA_DATA_ERROR;
                }
                if control >= 0x80 as u32 {
                    (*coder).uncompressed_size = ((control & 0x1f as u32) << 16) as size_t;
                    (*coder).sequence = SEQ_UNCOMPRESSED_1;
                    if control >= 0xc0 as u32 {
                        (*coder).need_properties = false;
                        (*coder).next_sequence = SEQ_PROPERTIES;
                    } else if (*coder).need_properties {
                        return LZMA_DATA_ERROR;
                    } else {
                        (*coder).next_sequence = SEQ_LZMA;
                        if control >= 0xa0 as u32 {
                            (*coder).lzma.reset.expect("non-null function pointer")(
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
                let fresh0 = *in_pos;
                *in_pos = (*in_pos).wrapping_add(1);
                (*coder).uncompressed_size = (*coder)
                    .uncompressed_size
                    .wrapping_add(((*in_0.offset(fresh0 as isize) as u32) << 8) as size_t);
                (*coder).sequence = SEQ_UNCOMPRESSED_2;
            }
            2 => {
                let fresh1 = *in_pos;
                *in_pos = (*in_pos).wrapping_add(1);
                (*coder).uncompressed_size = (*coder).uncompressed_size.wrapping_add(
                    u32::from(*in_0.offset(fresh1 as isize)).wrapping_add(1) as size_t,
                );
                (*coder).sequence = SEQ_COMPRESSED_0;
                (*coder)
                    .lzma
                    .set_uncompressed
                    .expect("non-null function pointer")(
                    (*coder).lzma.coder,
                    (*coder).uncompressed_size as lzma_vli,
                    false,
                );
            }
            3 => {
                let fresh2 = *in_pos;
                *in_pos = (*in_pos).wrapping_add(1);
                (*coder).compressed_size = ((*in_0.offset(fresh2 as isize) as u32) << 8) as size_t;
                (*coder).sequence = SEQ_COMPRESSED_1;
            }
            4 => {
                let fresh3 = *in_pos;
                *in_pos = (*in_pos).wrapping_add(1);
                (*coder).compressed_size = (*coder).compressed_size.wrapping_add(
                    u32::from(*in_0.offset(fresh3 as isize)).wrapping_add(1) as size_t,
                );
                (*coder).sequence = (*coder).next_sequence as sequence;
            }
            5 => {
                let fresh4 = *in_pos;
                *in_pos = (*in_pos).wrapping_add(1);
                if lzma_lzma_lclppb_decode(&raw mut (*coder).options, *in_0.offset(fresh4 as isize))
                {
                    return LZMA_DATA_ERROR;
                }
                (*coder).lzma.reset.expect("non-null function pointer")(
                    (*coder).lzma.coder,
                    &raw mut (*coder).options as *const c_void,
                );
                (*coder).sequence = SEQ_LZMA;
            }
            6 => {
                let in_start: size_t = *in_pos;
                let ret: lzma_ret = (*coder).lzma.code.expect("non-null function pointer")(
                    (*coder).lzma.coder,
                    dict,
                    in_0,
                    in_pos,
                    in_size,
                ) as lzma_ret;
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
    return LZMA_OK;
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
        coder = lzma_alloc(
            core::mem::size_of::<lzma_lzma2_coder>() as size_t,
            allocator,
        ) as *mut lzma_lzma2_coder;
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
        (*lz).end = Some(
            lzma2_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        )
            as Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>;
        (*coder).lzma = LZMA_LZ_DECODER_INIT;
    }
    let options: *const lzma_options_lzma = opt as *const lzma_options_lzma;
    (*coder).sequence = SEQ_CONTROL;
    (*coder).need_properties = true;
    (*coder).need_dictionary_reset =
        (*options).preset_dict.is_null() || (*options).preset_dict_size == 0;
    return lzma_lzma_decoder_create(&raw mut (*coder).lzma, allocator, options, lz_options);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma2_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    return lzma_lz_decoder_init(
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
    );
}
#[no_mangle]
pub extern "C" fn lzma_lzma2_decoder_memusage(options: *const c_void) -> u64 {
    return (core::mem::size_of::<lzma_lzma2_coder>() as u64)
        .wrapping_add(unsafe { lzma_lzma_decoder_memusage_nocheck(options) });
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
    if *props.offset(0) as c_int & 0xc0 as c_int != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    if *props.offset(0) as c_int > 40 as c_int {
        return LZMA_OPTIONS_ERROR;
    }
    let opt: *mut lzma_options_lzma = lzma_alloc(
        core::mem::size_of::<lzma_options_lzma>() as size_t,
        allocator,
    ) as *mut lzma_options_lzma;
    if opt.is_null() {
        return LZMA_MEM_ERROR;
    }
    if *props.offset(0) as c_int == 40 as c_int {
        (*opt).dict_size = UINT32_MAX as u32;
    } else {
        (*opt).dict_size = 2u32 | (u32::from(*props.offset(0)) & 1);
        (*opt).dict_size <<= u32::from(*props.offset(0)).wrapping_div(2).wrapping_add(11);
    }
    (*opt).preset_dict = ::core::ptr::null::<u8>();
    (*opt).preset_dict_size = 0;
    *options = opt as *mut c_void;
    return LZMA_OK;
}
