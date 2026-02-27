pub type uint8_t = u8;
pub type uint32_t = u32;
pub type lzma_bool = ::core::ffi::c_uchar;
pub type lzma_reserved_enum = ::core::ffi::c_uint;
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
pub type lzma_match_finder = ::core::ffi::c_uint;
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub type lzma_mode = ::core::ffi::c_uint;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;
pub const LZMA_MODE_FAST: lzma_mode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_lzma {
    pub dict_size: uint32_t,
    pub preset_dict: *const uint8_t,
    pub preset_dict_size: uint32_t,
    pub lc: uint32_t,
    pub lp: uint32_t,
    pub pb: uint32_t,
    pub mode: lzma_mode,
    pub nice_len: uint32_t,
    pub mf: lzma_match_finder,
    pub depth: uint32_t,
    pub ext_flags: uint32_t,
    pub ext_size_low: uint32_t,
    pub ext_size_high: uint32_t,
    pub reserved_int4: uint32_t,
    pub reserved_int5: uint32_t,
    pub reserved_int6: uint32_t,
    pub reserved_int7: uint32_t,
    pub reserved_int8: uint32_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_ptr1: *mut ::core::ffi::c_void,
    pub reserved_ptr2: *mut ::core::ffi::c_void,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LZMA_LC_DEFAULT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LZMA_LP_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LZMA_PB_DEFAULT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LZMA_PRESET_LEVEL_MASK: ::core::ffi::c_uint = 0x1f as ::core::ffi::c_uint;
pub const LZMA_PRESET_EXTREME: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn lzma_lzma_preset(
    mut options: *mut lzma_options_lzma,
    mut preset: uint32_t,
) -> lzma_bool {
    let level: uint32_t = preset & LZMA_PRESET_LEVEL_MASK as uint32_t;
    let flags: uint32_t = preset & !(LZMA_PRESET_LEVEL_MASK as uint32_t);
    let supported_flags: uint32_t = LZMA_PRESET_EXTREME as uint32_t;
    if level > 9 as uint32_t || flags & !supported_flags != 0 {
        return true_0 as lzma_bool;
    }
    (*options).preset_dict = ::core::ptr::null::<uint8_t>();
    (*options).preset_dict_size = 0 as uint32_t;
    (*options).lc = LZMA_LC_DEFAULT as uint32_t;
    (*options).lp = LZMA_LP_DEFAULT as uint32_t;
    (*options).pb = LZMA_PB_DEFAULT as uint32_t;
    static mut dict_pow2: [uint8_t; 10] = [
        18 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
    ];
    (*options).dict_size =
        ((1 as ::core::ffi::c_uint) << dict_pow2[level as usize] as ::core::ffi::c_int) as uint32_t;
    if level <= 3 as uint32_t {
        (*options).mode = LZMA_MODE_FAST;
        (*options).mf = (if level == 0 as uint32_t {
            LZMA_MF_HC3 as ::core::ffi::c_int
        } else {
            LZMA_MF_HC4 as ::core::ffi::c_int
        }) as lzma_match_finder;
        (*options).nice_len = (if level <= 1 as uint32_t {
            128 as ::core::ffi::c_int
        } else {
            273 as ::core::ffi::c_int
        }) as uint32_t;
        static mut depths: [uint8_t; 4] = [
            4 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            48 as ::core::ffi::c_int as uint8_t,
        ];
        (*options).depth = depths[level as usize] as uint32_t;
    } else {
        (*options).mode = LZMA_MODE_NORMAL;
        (*options).mf = LZMA_MF_BT4;
        (*options).nice_len = (if level == 4 as uint32_t {
            16 as ::core::ffi::c_int
        } else if level == 5 as uint32_t {
            32 as ::core::ffi::c_int
        } else {
            64 as ::core::ffi::c_int
        }) as uint32_t;
        (*options).depth = 0 as uint32_t;
    }
    if flags & LZMA_PRESET_EXTREME as uint32_t != 0 {
        (*options).mode = LZMA_MODE_NORMAL;
        (*options).mf = LZMA_MF_BT4;
        if level == 3 as uint32_t || level == 5 as uint32_t {
            (*options).nice_len = 192 as uint32_t;
            (*options).depth = 0 as uint32_t;
        } else {
            (*options).nice_len = 273 as uint32_t;
            (*options).depth = 512 as uint32_t;
        }
    }
    return false_0 as lzma_bool;
}
