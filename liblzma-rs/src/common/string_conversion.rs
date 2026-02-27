extern "C" {
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn lzma_lzma_preset(options: *mut lzma_options_lzma, preset: uint32_t) -> lzma_bool;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut ::core::ffi::c_void;
    fn lzma_alloc_zero(size: size_t, allocator: *const lzma_allocator) -> *mut ::core::ffi::c_void;
    fn lzma_free(ptr: *mut ::core::ffi::c_void, allocator: *const lzma_allocator);
    fn lzma_validate_chain(filters: *const lzma_filter, count: *mut size_t) -> lzma_ret;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type lzma_bool = ::core::ffi::c_uchar;
pub type lzma_reserved_enum = ::core::ffi::c_uint;
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
pub type lzma_ret = ::core::ffi::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, size_t) -> *mut ::core::ffi::c_void,
    >,
    pub free:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut ::core::ffi::c_void,
}
pub type lzma_vli = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub name: [::core::ffi::c_char; 12],
    pub opts_size: uint32_t,
    pub id: lzma_vli,
    pub parse: Option<
        unsafe extern "C" fn(
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> *const ::core::ffi::c_char,
    >,
    pub optmap: *const option_map,
    pub strfy_encoder: uint8_t,
    pub strfy_decoder: uint8_t,
    pub allow_null: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_map {
    pub name: [::core::ffi::c_char; 12],
    pub type_0: uint8_t,
    pub flags: uint8_t,
    pub offset: uint16_t,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub map: *const name_value_map,
    pub range: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub min: uint32_t,
    pub max: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_value_map {
    pub name: [::core::ffi::c_char; 12],
    pub value: uint32_t,
}
pub type lzma_match_finder = ::core::ffi::c_uint;
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub const OPTMAP_TYPE_LZMA_MATCH_FINDER: C2RustUnnamed_2 = 2;
pub type lzma_mode = ::core::ffi::c_uint;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;
pub const LZMA_MODE_FAST: lzma_mode = 1;
pub const OPTMAP_TYPE_LZMA_MODE: C2RustUnnamed_2 = 1;
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
pub const OPTMAP_TYPE_LZMA_PRESET: C2RustUnnamed_2 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_delta {
    pub type_0: lzma_delta_type,
    pub dist: uint32_t,
    pub reserved_int1: uint32_t,
    pub reserved_int2: uint32_t,
    pub reserved_int3: uint32_t,
    pub reserved_int4: uint32_t,
    pub reserved_ptr1: *mut ::core::ffi::c_void,
    pub reserved_ptr2: *mut ::core::ffi::c_void,
}
pub type lzma_delta_type = ::core::ffi::c_uint;
pub const LZMA_DELTA_TYPE_BYTE: lzma_delta_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_bcj {
    pub start_offset: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_str {
    pub buf: *mut ::core::ffi::c_char,
    pub pos: size_t,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const OPTMAP_TYPE_UINT32: C2RustUnnamed_2 = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulonglong = 18446744073709551615 as ::core::ffi::c_ulonglong;
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LZMA_VLI_UNKNOWN: ::core::ffi::c_ulonglong = UINT64_MAX;
pub const LZMA_FILTERS_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LZMA_STR_ALL_FILTERS: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const LZMA_STR_NO_VALIDATION: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const LZMA_STR_ENCODER: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const LZMA_STR_DECODER: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const LZMA_STR_GETOPT_LONG: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const LZMA_STR_NO_SPACES: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const LZMA_FILTER_X86: ::core::ffi::c_ulonglong = 0x4 as ::core::ffi::c_ulonglong;
pub const LZMA_FILTER_POWERPC: ::core::ffi::c_ulonglong = 0x5 as ::core::ffi::c_ulonglong;
pub const LZMA_FILTER_IA64: ::core::ffi::c_ulonglong = 0x6 as ::core::ffi::c_ulonglong;
pub const LZMA_FILTER_ARM: ::core::ffi::c_ulonglong = 0x7 as ::core::ffi::c_ulonglong;
pub const LZMA_FILTER_ARMTHUMB: ::core::ffi::c_ulonglong = 0x8 as ::core::ffi::c_ulonglong;
pub const LZMA_FILTER_SPARC: ::core::ffi::c_ulonglong = 0x9 as ::core::ffi::c_ulonglong;
pub const LZMA_FILTER_ARM64: ::core::ffi::c_ulonglong = 0xa as ::core::ffi::c_ulonglong;
pub const LZMA_FILTER_RISCV: ::core::ffi::c_ulonglong = 0xb as ::core::ffi::c_ulonglong;
pub const LZMA_FILTER_DELTA: ::core::ffi::c_ulonglong = 0x3 as ::core::ffi::c_ulonglong;
pub const LZMA_DELTA_DIST_MIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LZMA_DELTA_DIST_MAX: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const LZMA_FILTER_LZMA1: ::core::ffi::c_ulonglong =
    0x4000000000000001 as ::core::ffi::c_ulonglong;
pub const LZMA_FILTER_LZMA2: ::core::ffi::c_ulonglong = 0x21 as ::core::ffi::c_ulonglong;
pub const LZMA_DICT_SIZE_MIN: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
pub const LZMA_DICT_SIZE_DEFAULT: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 23 as ::core::ffi::c_int;
pub const LZMA_LCLP_MIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LZMA_LCLP_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LZMA_PB_MIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LZMA_PB_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LZMA_PRESET_DEFAULT: ::core::ffi::c_uint = 6 as ::core::ffi::c_uint;
pub const LZMA_PRESET_EXTREME: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int;
pub const LZMA_FILTER_RESERVED_START: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int;
pub const STR_ALLOC_SIZE: ::core::ffi::c_int = 800 as ::core::ffi::c_int;
unsafe extern "C" fn str_init(
    mut str: *mut lzma_str,
    mut allocator: *const lzma_allocator,
) -> lzma_ret {
    (*str).buf = lzma_alloc(STR_ALLOC_SIZE as size_t, allocator) as *mut ::core::ffi::c_char;
    if (*str).buf.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*str).pos = 0 as size_t;
    return LZMA_OK;
}
unsafe extern "C" fn str_free(mut str: *mut lzma_str, mut allocator: *const lzma_allocator) {
    lzma_free((*str).buf as *mut ::core::ffi::c_void, allocator);
}
unsafe extern "C" fn str_is_full(mut str: *const lzma_str) -> bool {
    return (*str).pos == (STR_ALLOC_SIZE - 1 as ::core::ffi::c_int) as size_t;
}
unsafe extern "C" fn str_finish(
    mut dest: *mut *mut ::core::ffi::c_char,
    mut str: *mut lzma_str,
    mut allocator: *const lzma_allocator,
) -> lzma_ret {
    if str_is_full(str) {
        lzma_free((*str).buf as *mut ::core::ffi::c_void, allocator);
        *dest = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return LZMA_PROG_ERROR;
    }
    *(*str).buf.offset((*str).pos as isize) = '\0' as i32 as ::core::ffi::c_char;
    *dest = (*str).buf;
    return LZMA_OK;
}
unsafe extern "C" fn str_append_str(mut str: *mut lzma_str, mut s: *const ::core::ffi::c_char) {
    let len: size_t = strlen(s) as size_t;
    let limit: size_t =
        ((STR_ALLOC_SIZE - 1 as ::core::ffi::c_int) as size_t).wrapping_sub((*str).pos);
    let copy_size: size_t = if len < limit { len } else { limit };
    memcpy(
        (*str).buf.offset((*str).pos as isize) as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        copy_size,
    );
    (*str).pos = (*str).pos.wrapping_add(copy_size);
}
unsafe extern "C" fn str_append_u32(
    mut str: *mut lzma_str,
    mut v: uint32_t,
    mut use_byte_suffix: bool,
) {
    if v == 0 as uint32_t {
        str_append_str(str, b"0\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        static mut suffixes: [[::core::ffi::c_char; 4]; 4] = unsafe {
            [
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"\0\0\0\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"KiB\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"MiB\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"GiB\0"),
            ]
        };
        let mut suf: size_t = 0 as size_t;
        if use_byte_suffix {
            while v & 1023 as uint32_t == 0 as uint32_t
                && suf
                    < (::core::mem::size_of::<[[::core::ffi::c_char; 4]; 4]>() as usize)
                        .wrapping_div(::core::mem::size_of::<[::core::ffi::c_char; 4]>() as usize)
                        .wrapping_sub(1 as usize)
            {
                v >>= 10 as ::core::ffi::c_int;
                suf = suf.wrapping_add(1);
            }
        }
        let mut buf: [::core::ffi::c_char; 16] = ::core::mem::transmute::<
            [u8; 16],
            [::core::ffi::c_char; 16],
        >(*b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
        let mut pos: size_t = (::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t)
            .wrapping_sub(1 as size_t);
        loop {
            pos = pos.wrapping_sub(1);
            buf[pos as usize] = ('0' as i32 as uint32_t)
                .wrapping_add(v.wrapping_rem(10 as uint32_t))
                as ::core::ffi::c_char;
            v = v.wrapping_div(10 as uint32_t);
            if !(v != 0 as uint32_t) {
                break;
            }
        }
        str_append_str(
            str,
            (&raw mut buf as *mut ::core::ffi::c_char).offset(pos as isize),
        );
        str_append_str(
            str,
            &raw const *(&raw const suffixes as *const [::core::ffi::c_char; 4])
                .offset(suf as isize) as *const ::core::ffi::c_char,
        );
    };
}
pub const NAME_LEN_MAX: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const OPTMAP_USE_NAME_VALUE_MAP: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPTMAP_USE_BYTE_SUFFIX: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPTMAP_NO_STRFY_ZERO: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
static mut bcj_optmap: [option_map; 1] = unsafe {
    [option_map {
        name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
            *b"start\0\0\0\0\0\0\0",
        ),
        type_0: 0,
        flags: (OPTMAP_NO_STRFY_ZERO | OPTMAP_USE_BYTE_SUFFIX) as uint8_t,
        offset: 0 as ::core::ffi::c_ulong as uint16_t,
        u: C2RustUnnamed_0 {
            range: C2RustUnnamed_1 {
                min: 0 as uint32_t,
                max: UINT32_MAX as uint32_t,
            },
        },
    }]
};
unsafe extern "C" fn parse_bcj(
    str: *mut *const ::core::ffi::c_char,
    mut str_end: *const ::core::ffi::c_char,
    mut filter_options: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    return parse_options(
        str,
        str_end,
        filter_options,
        &raw const bcj_optmap as *const option_map,
        (::core::mem::size_of::<[option_map; 1]>() as size_t)
            .wrapping_div(::core::mem::size_of::<option_map>() as size_t),
    );
}
static mut delta_optmap: [option_map; 1] = unsafe {
    [option_map {
        name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
            *b"dist\0\0\0\0\0\0\0\0",
        ),
        type_0: 0,
        flags: 0,
        offset: 4 as ::core::ffi::c_ulong as uint16_t,
        u: C2RustUnnamed_0 {
            range: C2RustUnnamed_1 {
                min: LZMA_DELTA_DIST_MIN as uint32_t,
                max: LZMA_DELTA_DIST_MAX as uint32_t,
            },
        },
    }]
};
unsafe extern "C" fn parse_delta(
    str: *mut *const ::core::ffi::c_char,
    mut str_end: *const ::core::ffi::c_char,
    mut filter_options: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    let mut opts: *mut lzma_options_delta = filter_options as *mut lzma_options_delta;
    (*opts).type_0 = LZMA_DELTA_TYPE_BYTE;
    (*opts).dist = LZMA_DELTA_DIST_MIN as uint32_t;
    return parse_options(
        str,
        str_end,
        filter_options,
        &raw const delta_optmap as *const option_map,
        (::core::mem::size_of::<[option_map; 1]>() as size_t)
            .wrapping_div(::core::mem::size_of::<option_map>() as size_t),
    );
}
pub const LZMA12_PRESET_STR: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"0-9[e]\0") };
unsafe extern "C" fn parse_lzma12_preset(
    str: *mut *const ::core::ffi::c_char,
    mut str_end: *const ::core::ffi::c_char,
    mut preset: *mut uint32_t,
) -> *const ::core::ffi::c_char {
    if !(**str as ::core::ffi::c_int >= '0' as i32 && **str as ::core::ffi::c_int <= '9' as i32) {
        return b"Unsupported preset\0" as *const u8 as *const ::core::ffi::c_char;
    }
    *preset = (**str as ::core::ffi::c_int - '0' as i32) as uint32_t;
    loop {
        *str = (*str).offset(1);
        if !(*str < str_end) {
            break;
        }
        match **str as ::core::ffi::c_int {
            101 => {
                *preset = (*preset as ::core::ffi::c_uint | LZMA_PRESET_EXTREME) as uint32_t;
            }
            _ => {
                return b"Unsupported flag in the preset\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
        }
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn set_lzma12_preset(
    str: *mut *const ::core::ffi::c_char,
    mut str_end: *const ::core::ffi::c_char,
    mut filter_options: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    let mut preset: uint32_t = 0;
    let mut errmsg: *const ::core::ffi::c_char = parse_lzma12_preset(str, str_end, &raw mut preset);
    if !errmsg.is_null() {
        return errmsg;
    }
    let mut opts: *mut lzma_options_lzma = filter_options as *mut lzma_options_lzma;
    if lzma_lzma_preset(opts, preset) != 0 {
        return b"Unsupported preset\0" as *const u8 as *const ::core::ffi::c_char;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
static mut lzma12_mode_map: [name_value_map; 3] = unsafe {
    [
        name_value_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"fast\0\0\0\0\0\0\0\0",
            ),
            value: LZMA_MODE_FAST as ::core::ffi::c_int as uint32_t,
        },
        name_value_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"normal\0\0\0\0\0\0",
            ),
            value: LZMA_MODE_NORMAL as ::core::ffi::c_int as uint32_t,
        },
        name_value_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            value: 0 as uint32_t,
        },
    ]
};
static mut lzma12_mf_map: [name_value_map; 6] = unsafe {
    [
        name_value_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"hc3\0\0\0\0\0\0\0\0\0",
            ),
            value: LZMA_MF_HC3 as ::core::ffi::c_int as uint32_t,
        },
        name_value_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"hc4\0\0\0\0\0\0\0\0\0",
            ),
            value: LZMA_MF_HC4 as ::core::ffi::c_int as uint32_t,
        },
        name_value_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"bt2\0\0\0\0\0\0\0\0\0",
            ),
            value: LZMA_MF_BT2 as ::core::ffi::c_int as uint32_t,
        },
        name_value_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"bt3\0\0\0\0\0\0\0\0\0",
            ),
            value: LZMA_MF_BT3 as ::core::ffi::c_int as uint32_t,
        },
        name_value_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"bt4\0\0\0\0\0\0\0\0\0",
            ),
            value: LZMA_MF_BT4 as ::core::ffi::c_int as uint32_t,
        },
        name_value_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            value: 0 as uint32_t,
        },
    ]
};
static mut lzma12_optmap: [option_map; 9] = [option_map {
    name: [0; 12],
    type_0: 0,
    flags: 0,
    offset: 0,
    u: C2RustUnnamed_0 {
        map: ::core::ptr::null::<name_value_map>(),
    },
}; 9];
unsafe extern "C" fn parse_lzma12(
    str: *mut *const ::core::ffi::c_char,
    mut str_end: *const ::core::ffi::c_char,
    mut filter_options: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    let mut opts: *mut lzma_options_lzma = filter_options as *mut lzma_options_lzma;
    let preset_ret: bool = lzma_lzma_preset(opts, LZMA_PRESET_DEFAULT as uint32_t) != 0;
    let mut errmsg: *const ::core::ffi::c_char = parse_options(
        str,
        str_end,
        filter_options,
        &raw const lzma12_optmap as *const option_map,
        (::core::mem::size_of::<[option_map; 9]>() as size_t)
            .wrapping_div(::core::mem::size_of::<option_map>() as size_t),
    );
    if !errmsg.is_null() {
        return errmsg;
    }
    if (*opts).lc.wrapping_add((*opts).lp) > LZMA_LCLP_MAX as uint32_t {
        return b"The sum of lc and lp must not exceed 4\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
static mut filter_name_map: [C2RustUnnamed; 11] = unsafe {
    [
        C2RustUnnamed {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"lzma1\0\0\0\0\0\0\0",
            ),
            opts_size: ::core::mem::size_of::<lzma_options_lzma>() as uint32_t,
            id: LZMA_FILTER_LZMA1 as lzma_vli,
            parse: Some(
                parse_lzma12
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            optmap: &raw const lzma12_optmap as *const option_map,
            strfy_encoder: 9 as uint8_t,
            strfy_decoder: 5 as uint8_t,
            allow_null: false_0 != 0,
        },
        C2RustUnnamed {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"lzma2\0\0\0\0\0\0\0",
            ),
            opts_size: ::core::mem::size_of::<lzma_options_lzma>() as uint32_t,
            id: LZMA_FILTER_LZMA2 as lzma_vli,
            parse: Some(
                parse_lzma12
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            optmap: &raw const lzma12_optmap as *const option_map,
            strfy_encoder: 9 as uint8_t,
            strfy_decoder: 2 as uint8_t,
            allow_null: false_0 != 0,
        },
        C2RustUnnamed {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"x86\0\0\0\0\0\0\0\0\0",
            ),
            opts_size: ::core::mem::size_of::<lzma_options_bcj>() as uint32_t,
            id: LZMA_FILTER_X86 as lzma_vli,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            optmap: &raw const bcj_optmap as *const option_map,
            strfy_encoder: 1 as uint8_t,
            strfy_decoder: 1 as uint8_t,
            allow_null: true_0 != 0,
        },
        C2RustUnnamed {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"arm\0\0\0\0\0\0\0\0\0",
            ),
            opts_size: ::core::mem::size_of::<lzma_options_bcj>() as uint32_t,
            id: LZMA_FILTER_ARM as lzma_vli,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            optmap: &raw const bcj_optmap as *const option_map,
            strfy_encoder: 1 as uint8_t,
            strfy_decoder: 1 as uint8_t,
            allow_null: true_0 != 0,
        },
        C2RustUnnamed {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"armthumb\0\0\0\0",
            ),
            opts_size: ::core::mem::size_of::<lzma_options_bcj>() as uint32_t,
            id: LZMA_FILTER_ARMTHUMB as lzma_vli,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            optmap: &raw const bcj_optmap as *const option_map,
            strfy_encoder: 1 as uint8_t,
            strfy_decoder: 1 as uint8_t,
            allow_null: true_0 != 0,
        },
        C2RustUnnamed {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"arm64\0\0\0\0\0\0\0",
            ),
            opts_size: ::core::mem::size_of::<lzma_options_bcj>() as uint32_t,
            id: LZMA_FILTER_ARM64 as lzma_vli,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            optmap: &raw const bcj_optmap as *const option_map,
            strfy_encoder: 1 as uint8_t,
            strfy_decoder: 1 as uint8_t,
            allow_null: true_0 != 0,
        },
        C2RustUnnamed {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"riscv\0\0\0\0\0\0\0",
            ),
            opts_size: ::core::mem::size_of::<lzma_options_bcj>() as uint32_t,
            id: LZMA_FILTER_RISCV as lzma_vli,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            optmap: &raw const bcj_optmap as *const option_map,
            strfy_encoder: 1 as uint8_t,
            strfy_decoder: 1 as uint8_t,
            allow_null: true_0 != 0,
        },
        C2RustUnnamed {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"powerpc\0\0\0\0\0",
            ),
            opts_size: ::core::mem::size_of::<lzma_options_bcj>() as uint32_t,
            id: LZMA_FILTER_POWERPC as lzma_vli,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            optmap: &raw const bcj_optmap as *const option_map,
            strfy_encoder: 1 as uint8_t,
            strfy_decoder: 1 as uint8_t,
            allow_null: true_0 != 0,
        },
        C2RustUnnamed {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"ia64\0\0\0\0\0\0\0\0",
            ),
            opts_size: ::core::mem::size_of::<lzma_options_bcj>() as uint32_t,
            id: LZMA_FILTER_IA64 as lzma_vli,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            optmap: &raw const bcj_optmap as *const option_map,
            strfy_encoder: 1 as uint8_t,
            strfy_decoder: 1 as uint8_t,
            allow_null: true_0 != 0,
        },
        C2RustUnnamed {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"sparc\0\0\0\0\0\0\0",
            ),
            opts_size: ::core::mem::size_of::<lzma_options_bcj>() as uint32_t,
            id: LZMA_FILTER_SPARC as lzma_vli,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            optmap: &raw const bcj_optmap as *const option_map,
            strfy_encoder: 1 as uint8_t,
            strfy_decoder: 1 as uint8_t,
            allow_null: true_0 != 0,
        },
        C2RustUnnamed {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"delta\0\0\0\0\0\0\0",
            ),
            opts_size: ::core::mem::size_of::<lzma_options_delta>() as uint32_t,
            id: LZMA_FILTER_DELTA as lzma_vli,
            parse: Some(
                parse_delta
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *const ::core::ffi::c_char,
            ),
            optmap: &raw const delta_optmap as *const option_map,
            strfy_encoder: 1 as uint8_t,
            strfy_decoder: 1 as uint8_t,
            allow_null: false_0 != 0,
        },
    ]
};
unsafe extern "C" fn parse_options(
    str: *mut *const ::core::ffi::c_char,
    mut str_end: *const ::core::ffi::c_char,
    mut filter_options: *mut ::core::ffi::c_void,
    optmap: *const option_map,
    optmap_size: size_t,
) -> *const ::core::ffi::c_char {
    while *str < str_end && **str as ::core::ffi::c_int != '\0' as i32 {
        if **str as ::core::ffi::c_int == ',' as i32 {
            *str = (*str).offset(1);
        } else {
            let str_len: size_t = str_end.offset_from(*str) as ::core::ffi::c_long as size_t;
            let mut name_eq_value_end: *const ::core::ffi::c_char =
                memchr(*str as *const ::core::ffi::c_void, ',' as i32, str_len)
                    as *const ::core::ffi::c_char;
            if name_eq_value_end.is_null() {
                name_eq_value_end = str_end;
            }
            let mut equals_sign: *const ::core::ffi::c_char = memchr(
                *str as *const ::core::ffi::c_void,
                '=' as i32,
                name_eq_value_end.offset_from(*str) as ::core::ffi::c_long as size_t,
            )
                as *const ::core::ffi::c_char;
            if equals_sign.is_null() || **str as ::core::ffi::c_int == '=' as i32 {
                return b"Options must be 'name=value' pairs separated with commas\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            let name_len: size_t = equals_sign.offset_from(*str) as ::core::ffi::c_long as size_t;
            if name_len > NAME_LEN_MAX as size_t {
                return b"Unknown option name\0" as *const u8 as *const ::core::ffi::c_char;
            }
            let mut i: size_t = 0 as size_t;
            loop {
                if i == optmap_size {
                    return b"Unknown option name\0" as *const u8 as *const ::core::ffi::c_char;
                }
                if memcmp(
                    *str as *const ::core::ffi::c_void,
                    &raw const (*optmap.offset(i as isize)).name as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    name_len,
                ) == 0 as ::core::ffi::c_int
                    && (*optmap.offset(i as isize)).name[name_len as usize] as ::core::ffi::c_int
                        == '\0' as i32
                {
                    break;
                }
                i = i.wrapping_add(1);
            }
            *str = equals_sign.offset(1 as ::core::ffi::c_int as isize);
            let value_len: size_t =
                name_eq_value_end.offset_from(*str) as ::core::ffi::c_long as size_t;
            if value_len == 0 as size_t {
                return b"Option value cannot be empty\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            if (*optmap.offset(i as isize)).type_0 as ::core::ffi::c_int
                == OPTMAP_TYPE_LZMA_PRESET as ::core::ffi::c_int
            {
                let mut errmsg: *const ::core::ffi::c_char =
                    set_lzma12_preset(str, name_eq_value_end, filter_options);
                if !errmsg.is_null() {
                    return errmsg;
                }
            } else {
                let mut v: uint32_t = 0;
                if (*optmap.offset(i as isize)).flags as ::core::ffi::c_int
                    & OPTMAP_USE_NAME_VALUE_MAP
                    != 0
                {
                    if value_len > NAME_LEN_MAX as size_t {
                        return b"Invalid option value\0" as *const u8
                            as *const ::core::ffi::c_char;
                    }
                    let mut map: *const name_value_map = (*optmap.offset(i as isize)).u.map;
                    let mut j: size_t = 0 as size_t;
                    loop {
                        if (*map.offset(j as isize)).name[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            == '\0' as i32
                        {
                            return b"Invalid option value\0" as *const u8
                                as *const ::core::ffi::c_char;
                        }
                        if memcmp(
                            *str as *const ::core::ffi::c_void,
                            &raw const (*map.offset(j as isize)).name as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            value_len,
                        ) == 0 as ::core::ffi::c_int
                            && (*map.offset(j as isize)).name[value_len as usize]
                                as ::core::ffi::c_int
                                == '\0' as i32
                        {
                            v = (*map.offset(j as isize)).value;
                            break;
                        } else {
                            j = j.wrapping_add(1);
                        }
                    }
                } else if (**str as ::core::ffi::c_int) < '0' as i32
                    || **str as ::core::ffi::c_int > '9' as i32
                {
                    return b"Value is not a non-negative decimal integer\0" as *const u8
                        as *const ::core::ffi::c_char;
                } else {
                    let mut p: *const ::core::ffi::c_char = *str;
                    v = 0 as uint32_t;
                    loop {
                        if v > (UINT32_MAX as uint32_t).wrapping_div(10 as uint32_t) {
                            return b"Value out of range\0" as *const u8
                                as *const ::core::ffi::c_char;
                        }
                        v = v.wrapping_mul(10 as uint32_t);
                        let add: uint32_t = (*p as ::core::ffi::c_int - '0' as i32) as uint32_t;
                        if (UINT32_MAX as uint32_t).wrapping_sub(add) < v {
                            return b"Value out of range\0" as *const u8
                                as *const ::core::ffi::c_char;
                        }
                        v = v.wrapping_add(add);
                        p = p.offset(1);
                        if !(p < name_eq_value_end
                            && *p as ::core::ffi::c_int >= '0' as i32
                            && *p as ::core::ffi::c_int <= '9' as i32)
                        {
                            break;
                        }
                    }
                    if p < name_eq_value_end {
                        let mut multiplier_start: *const ::core::ffi::c_char = p;
                        if (*optmap.offset(i as isize)).flags as ::core::ffi::c_int
                            & OPTMAP_USE_BYTE_SUFFIX
                            == 0 as ::core::ffi::c_int
                        {
                            *str = multiplier_start;
                            return b"This option does not support any multiplier suffixes\0"
                                as *const u8
                                as *const ::core::ffi::c_char;
                        }
                        let mut shift: uint32_t = 0;
                        match *p as ::core::ffi::c_int {
                            107 | 75 => {
                                shift = 10 as uint32_t;
                            }
                            109 | 77 => {
                                shift = 20 as uint32_t;
                            }
                            103 | 71 => {
                                shift = 30 as uint32_t;
                            }
                            _ => {
                                *str = multiplier_start;
                                return b"Invalid multiplier suffix (KiB, MiB, or GiB)\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char;
                            }
                        }
                        p = p.offset(1);
                        if p < name_eq_value_end && *p as ::core::ffi::c_int == 'i' as i32 {
                            p = p.offset(1);
                        }
                        if p < name_eq_value_end && *p as ::core::ffi::c_int == 'B' as i32 {
                            p = p.offset(1);
                        }
                        if p < name_eq_value_end {
                            *str = multiplier_start;
                            return b"Invalid multiplier suffix (KiB, MiB, or GiB)\0" as *const u8
                                as *const ::core::ffi::c_char;
                        }
                        if v > UINT32_MAX as uint32_t >> shift {
                            return b"Value out of range\0" as *const u8
                                as *const ::core::ffi::c_char;
                        }
                        v <<= shift;
                    }
                    if v < (*optmap.offset(i as isize)).u.range.min
                        || v > (*optmap.offset(i as isize)).u.range.max
                    {
                        return b"Value out of range\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                }
                let mut ptr: *mut ::core::ffi::c_void = (filter_options as *mut ::core::ffi::c_char)
                    .offset((*optmap.offset(i as isize)).offset as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void;
                match (*optmap.offset(i as isize)).type_0 as ::core::ffi::c_int {
                    1 => {
                        *(ptr as *mut lzma_mode) = v as lzma_mode;
                    }
                    2 => {
                        *(ptr as *mut lzma_match_finder) = v as lzma_match_finder;
                    }
                    _ => {
                        *(ptr as *mut uint32_t) = v;
                    }
                }
                *str = name_eq_value_end;
            }
        }
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn parse_filter(
    str: *mut *const ::core::ffi::c_char,
    mut str_end: *const ::core::ffi::c_char,
    mut filter: *mut lzma_filter,
    mut allocator: *const lzma_allocator,
    mut only_xz: bool,
) -> *const ::core::ffi::c_char {
    let mut name_end: *const ::core::ffi::c_char = str_end;
    let mut opts_start: *const ::core::ffi::c_char = str_end;
    let mut p: *const ::core::ffi::c_char = *str;
    while p < str_end {
        if *p as ::core::ffi::c_int == ':' as i32 || *p as ::core::ffi::c_int == '=' as i32 {
            name_end = p;
            opts_start = p.offset(1 as ::core::ffi::c_int as isize);
            break;
        } else {
            p = p.offset(1);
        }
    }
    let name_len: size_t = name_end.offset_from(*str) as ::core::ffi::c_long as size_t;
    if name_len > NAME_LEN_MAX as size_t {
        return b"Unknown filter name\0" as *const u8 as *const ::core::ffi::c_char;
    }
    let mut i: size_t = 0 as size_t;
    while i
        < (::core::mem::size_of::<[C2RustUnnamed; 11]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
    {
        if memcmp(
            *str as *const ::core::ffi::c_void,
            &raw const (*(&raw const filter_name_map as *const C2RustUnnamed).offset(i as isize))
                .name as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            name_len,
        ) == 0 as ::core::ffi::c_int
            && filter_name_map[i as usize].name[name_len as usize] as ::core::ffi::c_int
                == '\0' as i32
        {
            if only_xz as ::core::ffi::c_int != 0
                && filter_name_map[i as usize].id >= LZMA_FILTER_RESERVED_START as lzma_vli
            {
                return b"This filter cannot be used in the .xz format\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            let mut options: *mut ::core::ffi::c_void =
                lzma_alloc_zero(filter_name_map[i as usize].opts_size as size_t, allocator);
            if options.is_null() {
                return b"Memory allocation failed\0" as *const u8 as *const ::core::ffi::c_char;
            }
            *str = opts_start;
            let mut errmsg: *const ::core::ffi::c_char =
                filter_name_map[i as usize]
                    .parse
                    .expect("non-null function pointer")(str, str_end, options);
            if !errmsg.is_null() {
                lzma_free(options, allocator);
                return errmsg;
            }
            (*filter).id = filter_name_map[i as usize].id;
            (*filter).options = options;
            return ::core::ptr::null::<::core::ffi::c_char>();
        }
        i = i.wrapping_add(1);
    }
    return b"Unknown filter name\0" as *const u8 as *const ::core::ffi::c_char;
}
unsafe extern "C" fn str_to_filters(
    str: *mut *const ::core::ffi::c_char,
    mut filters: *mut lzma_filter,
    mut flags: uint32_t,
    mut allocator: *const lzma_allocator,
) -> *const ::core::ffi::c_char {
    let mut current_block: u64;
    let mut errmsg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    while **str as ::core::ffi::c_int == ' ' as i32 {
        *str = (*str).offset(1);
    }
    if **str as ::core::ffi::c_int == '\0' as i32 {
        return b"Empty string is not allowed, try '6' if a default value is needed\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    if **str as ::core::ffi::c_int >= '0' as i32 && **str as ::core::ffi::c_int <= '9' as i32
        || **str as ::core::ffi::c_int == '-' as i32
            && (*(*str).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= '0' as i32
                && *(*str).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= '9' as i32)
    {
        if **str as ::core::ffi::c_int == '-' as i32 {
            *str = (*str).offset(1);
        }
        let str_len: size_t = strlen(*str) as size_t;
        let mut str_end: *const ::core::ffi::c_char =
            memchr(*str as *const ::core::ffi::c_void, ' ' as i32, str_len)
                as *const ::core::ffi::c_char;
        if !str_end.is_null() {
            let mut i: size_t = 1 as size_t;
            while *str_end.offset(i as isize) as ::core::ffi::c_int != '\0' as i32 {
                if *str_end.offset(i as isize) as ::core::ffi::c_int != ' ' as i32 {
                    return b"Unsupported preset\0" as *const u8 as *const ::core::ffi::c_char;
                }
                i = i.wrapping_add(1);
            }
        } else {
            str_end = (*str).offset(str_len as isize);
        }
        let mut preset: uint32_t = 0;
        errmsg = parse_lzma12_preset(str, str_end, &raw mut preset);
        if !errmsg.is_null() {
            return errmsg;
        }
        let mut opts: *mut lzma_options_lzma = lzma_alloc(
            ::core::mem::size_of::<lzma_options_lzma>() as size_t,
            allocator,
        ) as *mut lzma_options_lzma;
        if opts.is_null() {
            return b"Memory allocation failed\0" as *const u8 as *const ::core::ffi::c_char;
        }
        if lzma_lzma_preset(opts, preset) != 0 {
            lzma_free(opts as *mut ::core::ffi::c_void, allocator);
            return b"Unsupported preset\0" as *const u8 as *const ::core::ffi::c_char;
        }
        (*filters.offset(0 as ::core::ffi::c_int as isize)).id = LZMA_FILTER_LZMA2 as lzma_vli;
        let ref mut fresh0 = (*filters.offset(0 as ::core::ffi::c_int as isize)).options;
        *fresh0 = opts as *mut ::core::ffi::c_void;
        (*filters.offset(1 as ::core::ffi::c_int as isize)).id = LZMA_VLI_UNKNOWN as lzma_vli;
        let ref mut fresh1 = (*filters.offset(1 as ::core::ffi::c_int as isize)).options;
        *fresh1 = NULL;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let only_xz: bool = flags & LZMA_STR_ALL_FILTERS as uint32_t == 0 as uint32_t;
    let mut temp_filters: [lzma_filter; 5] = [lzma_filter {
        id: 0,
        options: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    }; 5];
    let mut i_0: size_t = 0 as size_t;
    loop {
        if i_0 == LZMA_FILTERS_MAX as size_t {
            errmsg = b"The maximum number of filters is four\0" as *const u8
                as *const ::core::ffi::c_char;
            current_block = 6100283484465977373;
            break;
        } else {
            if *(*str).offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
                && *(*str).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '-' as i32
            {
                *str = (*str).offset(2 as ::core::ffi::c_int as isize);
            }
            let mut filter_end: *const ::core::ffi::c_char = *str;
            while *filter_end.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '\0' as i32
            {
                if *filter_end.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '-' as i32
                    && *filter_end.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '-' as i32
                    || *filter_end.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ' ' as i32
                {
                    break;
                }
                filter_end = filter_end.offset(1);
            }
            if filter_end == *str {
                errmsg = b"Filter name is missing\0" as *const u8 as *const ::core::ffi::c_char;
                current_block = 6100283484465977373;
                break;
            } else {
                errmsg = parse_filter(
                    str,
                    filter_end,
                    (&raw mut temp_filters as *mut lzma_filter).offset(i_0 as isize)
                        as *mut lzma_filter,
                    allocator,
                    only_xz,
                );
                if !errmsg.is_null() {
                    current_block = 6100283484465977373;
                    break;
                }
                while **str as ::core::ffi::c_int == ' ' as i32 {
                    *str = (*str).offset(1);
                }
                i_0 = i_0.wrapping_add(1);
                if !(**str as ::core::ffi::c_int != '\0' as i32) {
                    current_block = 15090052786889560393;
                    break;
                }
            }
        }
    }
    match current_block {
        15090052786889560393 => {
            temp_filters[i_0 as usize].id = LZMA_VLI_UNKNOWN as lzma_vli;
            temp_filters[i_0 as usize].options = NULL;
            if flags & LZMA_STR_NO_VALIDATION as uint32_t == 0 as uint32_t {
                let mut dummy: size_t = 0;
                let ret: lzma_ret =
                    lzma_validate_chain(&raw mut temp_filters as *mut lzma_filter, &raw mut dummy)
                        as lzma_ret;
                if ret as ::core::ffi::c_uint
                    != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    errmsg = b"Invalid filter chain ('lzma2' missing at the end?)\0" as *const u8
                        as *const ::core::ffi::c_char;
                    current_block = 6100283484465977373;
                } else {
                    current_block = 12381812505308290051;
                }
            } else {
                current_block = 12381812505308290051;
            }
            match current_block {
                6100283484465977373 => {}
                _ => {
                    memcpy(
                        filters as *mut ::core::ffi::c_void,
                        &raw mut temp_filters as *mut lzma_filter as *const ::core::ffi::c_void,
                        i_0.wrapping_add(1 as size_t)
                            .wrapping_mul(::core::mem::size_of::<lzma_filter>() as size_t),
                    );
                    return ::core::ptr::null::<::core::ffi::c_char>();
                }
            }
        }
        _ => {}
    }
    loop {
        let fresh2 = i_0;
        i_0 = i_0.wrapping_sub(1);
        if !(fresh2 > 0 as size_t) {
            break;
        }
        lzma_free(temp_filters[i_0 as usize].options, allocator);
    }
    return errmsg;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_str_to_filters(
    mut str: *const ::core::ffi::c_char,
    mut error_pos: *mut ::core::ffi::c_int,
    mut filters: *mut lzma_filter,
    mut flags: uint32_t,
    mut allocator: *const lzma_allocator,
) -> *const ::core::ffi::c_char {
    if !error_pos.is_null() {
        *error_pos = 0 as ::core::ffi::c_int;
    }
    if str.is_null() || filters.is_null() {
        return b"Unexpected NULL pointer argument(s) to lzma_str_to_filters()\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    let supported_flags: uint32_t =
        LZMA_STR_ALL_FILTERS as uint32_t | LZMA_STR_NO_VALIDATION as uint32_t;
    if flags & !supported_flags != 0 {
        return b"Unsupported flags to lzma_str_to_filters()\0" as *const u8
            as *const ::core::ffi::c_char;
    }
    let mut used: *const ::core::ffi::c_char = str;
    let mut errmsg: *const ::core::ffi::c_char =
        str_to_filters(&raw mut used, filters, flags, allocator);
    if !error_pos.is_null() {
        let n: size_t = used.offset_from(str) as ::core::ffi::c_long as size_t;
        *error_pos = if n > INT_MAX as size_t {
            INT_MAX
        } else {
            n as ::core::ffi::c_int
        };
    }
    return errmsg;
}
unsafe extern "C" fn strfy_filter(
    mut dest: *mut lzma_str,
    mut delimiter: *const ::core::ffi::c_char,
    mut optmap: *const option_map,
    mut optmap_count: size_t,
    mut filter_options: *const ::core::ffi::c_void,
) {
    let mut i: size_t = 0 as size_t;
    while i < optmap_count {
        if !((*optmap.offset(i as isize)).type_0 as ::core::ffi::c_int
            == OPTMAP_TYPE_LZMA_PRESET as ::core::ffi::c_int)
        {
            let mut v: uint32_t = 0;
            let mut ptr: *const ::core::ffi::c_void = (filter_options as *const ::core::ffi::c_char)
                .offset((*optmap.offset(i as isize)).offset as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void;
            match (*optmap.offset(i as isize)).type_0 as ::core::ffi::c_int {
                1 => {
                    v = *(ptr as *const lzma_mode) as uint32_t;
                }
                2 => {
                    v = *(ptr as *const lzma_match_finder) as uint32_t;
                }
                _ => {
                    v = *(ptr as *const uint32_t);
                }
            }
            if !(v == 0 as uint32_t
                && (*optmap.offset(i as isize)).flags as ::core::ffi::c_int & OPTMAP_NO_STRFY_ZERO
                    != 0)
            {
                str_append_str(dest, delimiter);
                delimiter = b",\0" as *const u8 as *const ::core::ffi::c_char;
                str_append_str(
                    dest,
                    &raw const (*optmap.offset(i as isize)).name as *const ::core::ffi::c_char,
                );
                str_append_str(dest, b"=\0" as *const u8 as *const ::core::ffi::c_char);
                if (*optmap.offset(i as isize)).flags as ::core::ffi::c_int
                    & OPTMAP_USE_NAME_VALUE_MAP
                    != 0
                {
                    let mut map: *const name_value_map = (*optmap.offset(i as isize)).u.map;
                    let mut j: size_t = 0 as size_t;
                    loop {
                        if (*map.offset(j as isize)).name[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            == '\0' as i32
                        {
                            str_append_str(
                                dest,
                                b"UNKNOWN\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            break;
                        } else if (*map.offset(j as isize)).value == v {
                            str_append_str(
                                dest,
                                &raw const (*map.offset(j as isize)).name
                                    as *const ::core::ffi::c_char,
                            );
                            break;
                        } else {
                            j = j.wrapping_add(1);
                        }
                    }
                } else {
                    str_append_u32(
                        dest,
                        v,
                        (*optmap.offset(i as isize)).flags as ::core::ffi::c_int
                            & OPTMAP_USE_BYTE_SUFFIX
                            != 0,
                    );
                }
            }
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_str_from_filters(
    mut output_str: *mut *mut ::core::ffi::c_char,
    mut filters: *const lzma_filter,
    mut flags: uint32_t,
    mut allocator: *const lzma_allocator,
) -> lzma_ret {
    if output_str.is_null() {
        return LZMA_PROG_ERROR;
    }
    *output_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if filters.is_null() {
        return LZMA_PROG_ERROR;
    }
    let supported_flags: uint32_t = LZMA_STR_ENCODER as uint32_t
        | LZMA_STR_DECODER as uint32_t
        | LZMA_STR_GETOPT_LONG as uint32_t
        | LZMA_STR_NO_SPACES as uint32_t;
    if flags & !supported_flags != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    if (*filters.offset(0 as ::core::ffi::c_int as isize)).id == LZMA_VLI_UNKNOWN as lzma_vli {
        return LZMA_OPTIONS_ERROR;
    }
    let mut dest: lzma_str = lzma_str {
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pos: 0,
    };
    let ret_: lzma_ret = str_init(&raw mut dest, allocator) as lzma_ret;
    if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret_;
    }
    let show_opts: bool =
        flags & (LZMA_STR_ENCODER as uint32_t | LZMA_STR_DECODER as uint32_t) != 0;
    let mut opt_delim: *const ::core::ffi::c_char = if flags & LZMA_STR_GETOPT_LONG as uint32_t != 0
    {
        b"=\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b":\0" as *const u8 as *const ::core::ffi::c_char
    };
    let mut i: size_t = 0 as size_t;
    while (*filters.offset(i as isize)).id != LZMA_VLI_UNKNOWN as lzma_vli {
        if i == LZMA_FILTERS_MAX as size_t {
            str_free(&raw mut dest, allocator);
            return LZMA_OPTIONS_ERROR;
        }
        if i > 0 as size_t && flags & LZMA_STR_NO_SPACES as uint32_t == 0 {
            str_append_str(
                &raw mut dest,
                b" \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if flags & LZMA_STR_GETOPT_LONG as uint32_t != 0
            || i > 0 as size_t && flags & LZMA_STR_NO_SPACES as uint32_t != 0
        {
            str_append_str(
                &raw mut dest,
                b"--\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let mut j: size_t = 0 as size_t;
        loop {
            if j == (::core::mem::size_of::<[C2RustUnnamed; 11]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
            {
                str_free(&raw mut dest, allocator);
                return LZMA_OPTIONS_ERROR;
            }
            if filter_name_map[j as usize].id == (*filters.offset(i as isize)).id {
                str_append_str(
                    &raw mut dest,
                    &raw const (*(&raw const filter_name_map as *const C2RustUnnamed)
                        .offset(j as isize))
                    .name as *const ::core::ffi::c_char,
                );
                if !show_opts {
                    break;
                }
                if (*filters.offset(i as isize)).options.is_null() {
                    if !filter_name_map[j as usize].allow_null {
                        str_free(&raw mut dest, allocator);
                        return LZMA_OPTIONS_ERROR;
                    }
                    break;
                } else {
                    let optmap_count: size_t = (if flags & LZMA_STR_ENCODER as uint32_t != 0 {
                        filter_name_map[j as usize].strfy_encoder as ::core::ffi::c_int
                    } else {
                        filter_name_map[j as usize].strfy_decoder as ::core::ffi::c_int
                    }) as size_t;
                    strfy_filter(
                        &raw mut dest,
                        opt_delim,
                        filter_name_map[j as usize].optmap,
                        optmap_count,
                        (*filters.offset(i as isize)).options,
                    );
                    break;
                }
            } else {
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    return str_finish(output_str, &raw mut dest, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_str_list_filters(
    mut output_str: *mut *mut ::core::ffi::c_char,
    mut filter_id: lzma_vli,
    mut flags: uint32_t,
    mut allocator: *const lzma_allocator,
) -> lzma_ret {
    if output_str.is_null() {
        return LZMA_PROG_ERROR;
    }
    *output_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let supported_flags: uint32_t = LZMA_STR_ALL_FILTERS as uint32_t
        | LZMA_STR_ENCODER as uint32_t
        | LZMA_STR_DECODER as uint32_t
        | LZMA_STR_GETOPT_LONG as uint32_t;
    if flags & !supported_flags != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut dest: lzma_str = lzma_str {
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pos: 0,
    };
    let ret_: lzma_ret = str_init(&raw mut dest, allocator) as lzma_ret;
    if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret_;
    }
    let show_opts: bool =
        flags & (LZMA_STR_ENCODER as uint32_t | LZMA_STR_DECODER as uint32_t) != 0;
    let mut filter_delim: *const ::core::ffi::c_char = if show_opts as ::core::ffi::c_int != 0 {
        b"\n\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b" \0" as *const u8 as *const ::core::ffi::c_char
    };
    let mut opt_delim: *const ::core::ffi::c_char = if flags & LZMA_STR_GETOPT_LONG as uint32_t != 0
    {
        b"=\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b":\0" as *const u8 as *const ::core::ffi::c_char
    };
    let mut first_filter_printed: bool = false_0 != 0;
    let mut i: size_t = 0 as size_t;
    while i
        < (::core::mem::size_of::<[C2RustUnnamed; 11]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
    {
        if !(filter_id != LZMA_VLI_UNKNOWN as lzma_vli
            && filter_id != filter_name_map[i as usize].id)
        {
            if !(filter_name_map[i as usize].id >= LZMA_FILTER_RESERVED_START as lzma_vli
                && flags & LZMA_STR_ALL_FILTERS as uint32_t == 0 as uint32_t
                && filter_id == LZMA_VLI_UNKNOWN as lzma_vli)
            {
                if first_filter_printed {
                    str_append_str(&raw mut dest, filter_delim);
                }
                first_filter_printed = true_0 != 0;
                if flags & LZMA_STR_GETOPT_LONG as uint32_t != 0 {
                    str_append_str(
                        &raw mut dest,
                        b"--\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                str_append_str(
                    &raw mut dest,
                    &raw const (*(&raw const filter_name_map as *const C2RustUnnamed)
                        .offset(i as isize))
                    .name as *const ::core::ffi::c_char,
                );
                if show_opts {
                    let mut optmap: *const option_map = filter_name_map[i as usize].optmap;
                    let mut d: *const ::core::ffi::c_char = opt_delim;
                    let end: size_t = (if flags & LZMA_STR_ENCODER as uint32_t != 0 {
                        filter_name_map[i as usize].strfy_encoder as ::core::ffi::c_int
                    } else {
                        filter_name_map[i as usize].strfy_decoder as ::core::ffi::c_int
                    }) as size_t;
                    let mut j: size_t = 0 as size_t;
                    while j < end {
                        str_append_str(&raw mut dest, d);
                        d = b",\0" as *const u8 as *const ::core::ffi::c_char;
                        str_append_str(
                            &raw mut dest,
                            &raw const (*optmap.offset(j as isize)).name
                                as *const ::core::ffi::c_char,
                        );
                        str_append_str(
                            &raw mut dest,
                            b"=<\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        if (*optmap.offset(j as isize)).type_0 as ::core::ffi::c_int
                            == OPTMAP_TYPE_LZMA_PRESET as ::core::ffi::c_int
                        {
                            str_append_str(&raw mut dest, LZMA12_PRESET_STR.as_ptr());
                        } else if (*optmap.offset(j as isize)).flags as ::core::ffi::c_int
                            & OPTMAP_USE_NAME_VALUE_MAP
                            != 0
                        {
                            let mut m: *const name_value_map = (*optmap.offset(j as isize)).u.map;
                            let mut k: size_t = 0 as size_t;
                            while (*m.offset(k as isize)).name[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                != '\0' as i32
                            {
                                if k > 0 as size_t {
                                    str_append_str(
                                        &raw mut dest,
                                        b"|\0" as *const u8 as *const ::core::ffi::c_char,
                                    );
                                }
                                str_append_str(
                                    &raw mut dest,
                                    &raw const (*m.offset(k as isize)).name
                                        as *const ::core::ffi::c_char,
                                );
                                k = k.wrapping_add(1);
                            }
                        } else {
                            let use_byte_suffix: bool = (*optmap.offset(j as isize)).flags
                                as ::core::ffi::c_int
                                & OPTMAP_USE_BYTE_SUFFIX
                                != 0;
                            str_append_u32(
                                &raw mut dest,
                                (*optmap.offset(j as isize)).u.range.min,
                                use_byte_suffix,
                            );
                            str_append_str(
                                &raw mut dest,
                                b"-\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            str_append_u32(
                                &raw mut dest,
                                (*optmap.offset(j as isize)).u.range.max,
                                use_byte_suffix,
                            );
                        }
                        str_append_str(
                            &raw mut dest,
                            b">\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        j = j.wrapping_add(1);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if !first_filter_printed {
        str_free(&raw mut dest, allocator);
        return LZMA_OPTIONS_ERROR;
    }
    return str_finish(output_str, &raw mut dest, allocator);
}
unsafe extern "C" fn run_static_initializers() {
    lzma12_optmap = [
        option_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"preset\0\0\0\0\0\0",
            ),
            type_0: OPTMAP_TYPE_LZMA_PRESET as ::core::ffi::c_int as uint8_t,
            flags: 0,
            offset: 0,
            u: C2RustUnnamed_0 {
                map: ::core::ptr::null::<name_value_map>(),
            },
        },
        option_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"dict\0\0\0\0\0\0\0\0",
            ),
            type_0: 0,
            flags: OPTMAP_USE_BYTE_SUFFIX as uint8_t,
            offset: 0 as ::core::ffi::c_ulong as uint16_t,
            u: C2RustUnnamed_0 {
                range: C2RustUnnamed_1 {
                    min: LZMA_DICT_SIZE_MIN as uint32_t,
                    max: ((1 as uint32_t) << 30 as ::core::ffi::c_int)
                        .wrapping_add((1 as uint32_t) << 29 as ::core::ffi::c_int),
                },
            },
        },
        option_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"lc\0\0\0\0\0\0\0\0\0\0",
            ),
            type_0: 0,
            flags: 0,
            offset: 20 as ::core::ffi::c_ulong as uint16_t,
            u: C2RustUnnamed_0 {
                range: C2RustUnnamed_1 {
                    min: LZMA_LCLP_MIN as uint32_t,
                    max: LZMA_LCLP_MAX as uint32_t,
                },
            },
        },
        option_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"lp\0\0\0\0\0\0\0\0\0\0",
            ),
            type_0: 0,
            flags: 0,
            offset: 24 as ::core::ffi::c_ulong as uint16_t,
            u: C2RustUnnamed_0 {
                range: C2RustUnnamed_1 {
                    min: LZMA_LCLP_MIN as uint32_t,
                    max: LZMA_LCLP_MAX as uint32_t,
                },
            },
        },
        option_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"pb\0\0\0\0\0\0\0\0\0\0",
            ),
            type_0: 0,
            flags: 0,
            offset: 28 as ::core::ffi::c_ulong as uint16_t,
            u: C2RustUnnamed_0 {
                range: C2RustUnnamed_1 {
                    min: LZMA_PB_MIN as uint32_t,
                    max: LZMA_PB_MAX as uint32_t,
                },
            },
        },
        option_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"mode\0\0\0\0\0\0\0\0",
            ),
            type_0: OPTMAP_TYPE_LZMA_MODE as ::core::ffi::c_int as uint8_t,
            flags: OPTMAP_USE_NAME_VALUE_MAP as uint8_t,
            offset: 32 as ::core::ffi::c_ulong as uint16_t,
            u: C2RustUnnamed_0 {
                map: &raw const lzma12_mode_map as *const name_value_map,
            },
        },
        option_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"nice\0\0\0\0\0\0\0\0",
            ),
            type_0: 0,
            flags: 0,
            offset: 36 as ::core::ffi::c_ulong as uint16_t,
            u: C2RustUnnamed_0 {
                range: C2RustUnnamed_1 {
                    min: 2 as uint32_t,
                    max: 273 as uint32_t,
                },
            },
        },
        option_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"mf\0\0\0\0\0\0\0\0\0\0",
            ),
            type_0: OPTMAP_TYPE_LZMA_MATCH_FINDER as ::core::ffi::c_int as uint8_t,
            flags: OPTMAP_USE_NAME_VALUE_MAP as uint8_t,
            offset: 40 as ::core::ffi::c_ulong as uint16_t,
            u: C2RustUnnamed_0 {
                map: &raw const lzma12_mf_map as *const name_value_map,
            },
        },
        option_map {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"depth\0\0\0\0\0\0\0",
            ),
            type_0: 0,
            flags: 0,
            offset: 44 as ::core::ffi::c_ulong as uint16_t,
            u: C2RustUnnamed_0 {
                range: C2RustUnnamed_1 {
                    min: 0 as uint32_t,
                    max: UINT32_MAX as uint32_t,
                },
            },
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
