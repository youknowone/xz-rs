use crate::types::*;
use core::ffi::{c_uint, c_void};
extern "C" {
    fn lzma_raw_encoder_memusage(filters: *const lzma_filter) -> u64;
    fn lzma_easy_preset(easy: *mut lzma_options_easy, preset: u32) -> bool;
}
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut c_void,
}
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
pub struct lzma_options_easy {
    pub filters: [lzma_filter; 5],
    pub opt_lzma: lzma_options_lzma,
}
pub const UINT32_MAX: c_uint = 4294967295;
#[no_mangle]
pub unsafe extern "C" fn lzma_easy_encoder_memusage(preset: u32) -> u64 {
    let mut opt_easy: lzma_options_easy = lzma_options_easy {
        filters: [lzma_filter {
            id: 0,
            options: core::ptr::null_mut(),
        }; 5],
        opt_lzma: lzma_options_lzma {
            dict_size: 0,
            preset_dict: ::core::ptr::null::<u8>(),
            preset_dict_size: 0,
            lc: 0,
            lp: 0,
            pb: 0,
            mode: 0 as lzma_mode,
            nice_len: 0,
            mf: 0 as lzma_match_finder,
            depth: 0,
            ext_flags: 0,
            ext_size_low: 0,
            ext_size_high: 0,
            reserved_int4: 0,
            reserved_int5: 0,
            reserved_int6: 0,
            reserved_int7: 0,
            reserved_int8: 0,
            reserved_enum1: LZMA_RESERVED_ENUM,
            reserved_enum2: LZMA_RESERVED_ENUM,
            reserved_enum3: LZMA_RESERVED_ENUM,
            reserved_enum4: LZMA_RESERVED_ENUM,
            reserved_ptr1: core::ptr::null_mut(),
            reserved_ptr2: core::ptr::null_mut(),
        },
    };
    if lzma_easy_preset(&raw mut opt_easy, preset) {
        return UINT32_MAX as u64;
    }
    return lzma_raw_encoder_memusage(&raw mut opt_easy.filters as *mut lzma_filter);
}
