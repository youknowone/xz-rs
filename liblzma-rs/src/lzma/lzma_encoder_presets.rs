use crate::types::*;
use core::ffi::{c_int, c_uint, c_void};
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
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
pub const LZMA_LC_DEFAULT: c_int = 3;
pub const LZMA_LP_DEFAULT: c_int = 0;
pub const LZMA_PB_DEFAULT: c_int = 2;
pub const LZMA_PRESET_LEVEL_MASK: c_uint = 0x1f;
pub const LZMA_PRESET_EXTREME: c_uint = 1u32 << 31;
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_preset(
    options: *mut lzma_options_lzma,
    preset: u32,
) -> lzma_bool {
    let level: u32 = preset & LZMA_PRESET_LEVEL_MASK as u32;
    let flags: u32 = preset & !(LZMA_PRESET_LEVEL_MASK as u32);
    let supported_flags: u32 = LZMA_PRESET_EXTREME as u32;
    if level > 9 || flags & !supported_flags != 0 {
        return true as lzma_bool;
    }
    (*options).preset_dict = ::core::ptr::null::<u8>();
    (*options).preset_dict_size = 0;
    (*options).lc = LZMA_LC_DEFAULT as u32;
    (*options).lp = LZMA_LP_DEFAULT as u32;
    (*options).pb = LZMA_PB_DEFAULT as u32;
    static mut dict_pow2: [u8; 10] = [18, 20, 21, 22, 22, 23, 23, 24, 25, 26];
    (*options).dict_size = (1u32 << dict_pow2[level as usize] as c_int) as u32;
    if level <= 3 {
        (*options).mode = LZMA_MODE_FAST;
        (*options).mf = (if level == 0 {
            LZMA_MF_HC3 as c_int
        } else {
            LZMA_MF_HC4 as c_int
        }) as lzma_match_finder;
        (*options).nice_len = (if level <= 1 {
            128 as c_int
        } else {
            273 as c_int
        }) as u32;
        static mut depths: [u8; 4] = [4, 8, 24, 48];
        (*options).depth = depths[level as usize] as u32;
    } else {
        (*options).mode = LZMA_MODE_NORMAL;
        (*options).mf = LZMA_MF_BT4;
        (*options).nice_len = (if level == 4 {
            16 as c_int
        } else if level == 5 {
            32 as c_int
        } else {
            64 as c_int
        }) as u32;
        (*options).depth = 0;
    }
    if flags & LZMA_PRESET_EXTREME as u32 != 0 {
        (*options).mode = LZMA_MODE_NORMAL;
        (*options).mf = LZMA_MF_BT4;
        if level == 3 || level == 5 {
            (*options).nice_len = 192;
            (*options).depth = 0;
        } else {
            (*options).nice_len = 273;
            (*options).depth = 512;
        }
    }
    return false as lzma_bool;
}
