use crate::types::*;
use core::ffi::c_void;
extern "C" {
    fn lzma_lzma_preset(options: *mut lzma_options_lzma, preset: u32) -> lzma_bool;
}
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
#[no_mangle]
pub unsafe extern "C" fn lzma_easy_preset(opt_easy: *mut lzma_options_easy, preset: u32) -> bool {
    if lzma_lzma_preset(&raw mut (*opt_easy).opt_lzma, preset) != 0 {
        return true;
    }
    (*opt_easy).filters[0].id = LZMA_FILTER_LZMA2 as lzma_vli;
    (*opt_easy).filters[0].options = &raw mut (*opt_easy).opt_lzma as *mut c_void;
    (*opt_easy).filters[1].id = LZMA_VLI_UNKNOWN as lzma_vli;
    return false;
}
