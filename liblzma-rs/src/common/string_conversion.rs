use crate::common::filter_common::lzma_validate_chain;
use crate::types::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_codec_def {
    pub name: [c_char; 12],
    pub opts_size: u32,
    pub id: lzma_vli,
    pub parse: Option<
        unsafe extern "C" fn(*mut *const c_char, *const c_char, *mut c_void) -> *const c_char,
    >,
    pub optmap: *const option_map,
    pub strfy_encoder: u8,
    pub strfy_decoder: u8,
    pub allow_null: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_map {
    pub name: [c_char; 12],
    pub type_0: u8,
    pub flags: u8,
    pub offset: u16,
    pub u: option_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union option_value {
    pub map: *const name_value_map,
    pub range: option_value_range,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_value_range {
    pub min: u32,
    pub max: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_value_map {
    pub name: [c_char; 12],
    pub value: u32,
}
pub const OPTMAP_TYPE_LZMA_MATCH_FINDER: option_type = 2;
pub const OPTMAP_TYPE_LZMA_MODE: option_type = 1;
pub const OPTMAP_TYPE_LZMA_PRESET: option_type = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_str {
    pub buf: *mut c_char,
    pub pos: size_t,
}
pub type option_type = u8;
pub const OPTMAP_TYPE_UINT32: option_type = 0;
pub const INT_MAX: c_int = c_int::MAX;
pub const LZMA_STR_ALL_FILTERS: c_uint = 0x1;
pub const LZMA_STR_NO_VALIDATION: c_uint = 0x2;
pub const LZMA_STR_ENCODER: c_uint = 0x10;
pub const LZMA_STR_DECODER: c_uint = 0x20;
pub const LZMA_STR_GETOPT_LONG: c_uint = 0x40;
pub const LZMA_STR_NO_SPACES: c_uint = 0x80;
pub const LZMA_DICT_SIZE_DEFAULT: c_uint = 1u32 << 23;
pub const LZMA_LCLP_MIN: u32 = 0;
pub const LZMA_PB_MIN: u32 = 0;
pub const LZMA_PRESET_DEFAULT: c_uint = 6;
pub const STR_ALLOC_SIZE: u32 = 800;
unsafe fn str_init(str: *mut lzma_str, allocator: *const lzma_allocator) -> lzma_ret {
    (*str).buf = lzma_alloc(STR_ALLOC_SIZE as size_t, allocator) as *mut c_char;
    if (*str).buf.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*str).pos = 0;
    LZMA_OK
}
unsafe fn str_free(str: *mut lzma_str, allocator: *const lzma_allocator) {
    lzma_free((*str).buf as *mut c_void, allocator);
}
unsafe fn str_is_full(str: *const lzma_str) -> bool {
    (*str).pos == (STR_ALLOC_SIZE - 1) as size_t
}
unsafe fn str_finish(
    dest: *mut *mut c_char,
    str: *mut lzma_str,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    if str_is_full(str) {
        lzma_free((*str).buf as *mut c_void, allocator);
        *dest = core::ptr::null_mut();
        return LZMA_PROG_ERROR;
    }
    *(*str).buf.offset((*str).pos as isize) = '\0' as i32 as c_char;
    *dest = (*str).buf;
    LZMA_OK
}
unsafe fn str_append_str(str: *mut lzma_str, s: *const c_char) {
    let len: size_t = strlen(s) as size_t;
    let limit: size_t = (STR_ALLOC_SIZE - 1) as size_t - (*str).pos;
    let copy_size: size_t = if len < limit { len } else { limit };
    core::ptr::copy_nonoverlapping(
        s as *const u8,
        (*str).buf.offset((*str).pos as isize) as *mut u8,
        copy_size,
    );
    (*str).pos += copy_size;
}
unsafe fn str_append_u32(str: *mut lzma_str, mut v: u32, use_byte_suffix: bool) {
    if v == 0 {
        str_append_str(str, crate::c_str!("0"));
    } else {
        static SUFFIXES: [[c_char; 4]; 4] = unsafe {
            [
                core::mem::transmute::<[u8; 4], [c_char; 4]>(*b"\0\0\0\0"),
                core::mem::transmute::<[u8; 4], [c_char; 4]>(*b"KiB\0"),
                core::mem::transmute::<[u8; 4], [c_char; 4]>(*b"MiB\0"),
                core::mem::transmute::<[u8; 4], [c_char; 4]>(*b"GiB\0"),
            ]
        };
        let mut suf: size_t = 0;
        if use_byte_suffix {
            while v & 1023 == 0
                && suf
                    < core::mem::size_of::<[[c_char; 4]; 4]>() / core::mem::size_of::<[c_char; 4]>()
                        - 1
            {
                v >>= 10;
                suf += 1;
            }
        }
        let mut buf: [c_char; 16] =
            core::mem::transmute::<[u8; 16], [c_char; 16]>(*b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
        let mut pos: size_t = core::mem::size_of::<[c_char; 16]>() - 1;
        loop {
            pos -= 1;
            buf[pos as usize] = ('0' as i32 as u32 + v % 10) as c_char;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        str_append_str(
            str,
            (::core::ptr::addr_of_mut!(buf) as *mut c_char).offset(pos as isize),
        );
        str_append_str(str, SUFFIXES[suf as usize].as_ptr());
    };
}
pub const NAME_LEN_MAX: u32 = 11;
pub const OPTMAP_USE_NAME_VALUE_MAP: u8 = 0x1;
pub const OPTMAP_USE_BYTE_SUFFIX: u8 = 0x2;
pub const OPTMAP_NO_STRFY_ZERO: u8 = 0x4;
static mut bcj_optmap: [option_map; 1] = unsafe {
    [option_map {
        name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"start\0\0\0\0\0\0\0"),
        type_0: 0,
        flags: (OPTMAP_NO_STRFY_ZERO | OPTMAP_USE_BYTE_SUFFIX) as u8,
        offset: 0,
        u: option_value {
            range: option_value_range {
                min: 0,
                max: UINT32_MAX,
            },
        },
    }]
};
extern "C" fn parse_bcj(
    str: *mut *const c_char,
    str_end: *const c_char,
    filter_options: *mut c_void,
) -> *const c_char {
    return unsafe {
        parse_options(
            str,
            str_end,
            filter_options,
            ::core::ptr::addr_of!(bcj_optmap) as *const option_map,
            core::mem::size_of::<[option_map; 1]>() / core::mem::size_of::<option_map>(),
        )
    };
}
static mut delta_optmap: [option_map; 1] = unsafe {
    [option_map {
        name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"dist\0\0\0\0\0\0\0\0"),
        type_0: 0,
        flags: 0,
        offset: 4,
        u: option_value {
            range: option_value_range {
                min: LZMA_DELTA_DIST_MIN,
                max: LZMA_DELTA_DIST_MAX,
            },
        },
    }]
};
extern "C" fn parse_delta(
    str: *mut *const c_char,
    str_end: *const c_char,
    filter_options: *mut c_void,
) -> *const c_char {
    return unsafe {
        let opts: *mut lzma_options_delta = filter_options as *mut lzma_options_delta;
        (*opts).type_0 = LZMA_DELTA_TYPE_BYTE;
        (*opts).dist = LZMA_DELTA_DIST_MIN;
        parse_options(
            str,
            str_end,
            filter_options,
            ::core::ptr::addr_of!(delta_optmap) as *const option_map,
            (core::mem::size_of::<[option_map; 1]>())
                .wrapping_div(core::mem::size_of::<option_map>()),
        )
    };
}
pub const LZMA12_PRESET_STR: [c_char; 7] =
    unsafe { core::mem::transmute::<[u8; 7], [c_char; 7]>(*b"0-9[e]\0") };
unsafe fn parse_lzma12_preset(
    str: *mut *const c_char,
    str_end: *const c_char,
    preset: *mut u32,
) -> *const c_char {
    if (**str as u8) < b'0' || (**str as u8) > b'9' {
        return crate::c_str!("Unsupported preset");
    }
    *preset = (**str as u8 - b'0') as u32;
    loop {
        *str = (*str).offset(1);
        if *str >= str_end {
            break;
        }
        match **str {
            101 => {
                *preset = (*preset | LZMA_PRESET_EXTREME) as u32;
            }
            _ => {
                return crate::c_str!("Unsupported flag in the preset");
            }
        }
    }
    core::ptr::null()
}
unsafe fn set_lzma12_preset(
    str: *mut *const c_char,
    str_end: *const c_char,
    filter_options: *mut c_void,
) -> *const c_char {
    let mut preset: u32 = 0;
    let errmsg: *const c_char =
        parse_lzma12_preset(str, str_end, ::core::ptr::addr_of_mut!(preset));
    if !errmsg.is_null() {
        return errmsg;
    }
    let opts: *mut lzma_options_lzma = filter_options as *mut lzma_options_lzma;
    if lzma_lzma_preset(opts, preset) != 0 {
        return crate::c_str!("Unsupported preset");
    }
    core::ptr::null()
}
static lzma12_mode_map: [name_value_map; 3] = unsafe {
    [
        name_value_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"fast\0\0\0\0\0\0\0\0"),
            value: LZMA_MODE_FAST as u32,
        },
        name_value_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"normal\0\0\0\0\0\0"),
            value: LZMA_MODE_NORMAL as u32,
        },
        name_value_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            value: 0,
        },
    ]
};
static lzma12_mf_map: [name_value_map; 6] = unsafe {
    [
        name_value_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"hc3\0\0\0\0\0\0\0\0\0"),
            value: LZMA_MF_HC3 as u32,
        },
        name_value_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"hc4\0\0\0\0\0\0\0\0\0"),
            value: LZMA_MF_HC4 as u32,
        },
        name_value_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"bt2\0\0\0\0\0\0\0\0\0"),
            value: LZMA_MF_BT2 as u32,
        },
        name_value_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"bt3\0\0\0\0\0\0\0\0\0"),
            value: LZMA_MF_BT3 as u32,
        },
        name_value_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"bt4\0\0\0\0\0\0\0\0\0"),
            value: LZMA_MF_BT4 as u32,
        },
        name_value_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"\0\0\0\0\0\0\0\0\0\0\0\0"),
            value: 0,
        },
    ]
};
static mut lzma12_optmap: [option_map; 9] = [option_map {
    name: [0; 12],
    type_0: 0,
    flags: 0,
    offset: 0,
    u: option_value {
        map: core::ptr::null(),
    },
}; 9];
unsafe extern "C" fn parse_lzma12(
    str: *mut *const c_char,
    str_end: *const c_char,
    filter_options: *mut c_void,
) -> *const c_char {
    let opts: *mut lzma_options_lzma = filter_options as *mut lzma_options_lzma;
    let _preset_ret: bool = lzma_lzma_preset(opts, LZMA_PRESET_DEFAULT as u32) != 0;
    let errmsg: *const c_char = parse_options(
        str,
        str_end,
        filter_options,
        ::core::ptr::addr_of!(lzma12_optmap) as *const option_map,
        core::mem::size_of::<[option_map; 9]>() / core::mem::size_of::<option_map>(),
    );
    if !errmsg.is_null() {
        return errmsg;
    }
    if (*opts).lc.wrapping_add((*opts).lp) > LZMA_LCLP_MAX {
        return crate::c_str!("The sum of lc and lp must not exceed 4");
    }
    core::ptr::null()
}
static mut filter_name_map: [filter_codec_def; 11] = unsafe {
    [
        filter_codec_def {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"lzma1\0\0\0\0\0\0\0"),
            opts_size: core::mem::size_of::<lzma_options_lzma>() as u32,
            id: LZMA_FILTER_LZMA1,
            parse: Some(
                parse_lzma12
                    as unsafe extern "C" fn(
                        *mut *const c_char,
                        *const c_char,
                        *mut c_void,
                    ) -> *const c_char,
            ),
            optmap: ::core::ptr::addr_of!(lzma12_optmap) as *const option_map,
            strfy_encoder: 9,
            strfy_decoder: 5,
            allow_null: false,
        },
        filter_codec_def {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"lzma2\0\0\0\0\0\0\0"),
            opts_size: core::mem::size_of::<lzma_options_lzma>() as u32,
            id: LZMA_FILTER_LZMA2,
            parse: Some(
                parse_lzma12
                    as unsafe extern "C" fn(
                        *mut *const c_char,
                        *const c_char,
                        *mut c_void,
                    ) -> *const c_char,
            ),
            optmap: ::core::ptr::addr_of!(lzma12_optmap) as *const option_map,
            strfy_encoder: 9,
            strfy_decoder: 2,
            allow_null: false,
        },
        filter_codec_def {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"x86\0\0\0\0\0\0\0\0\0"),
            opts_size: core::mem::size_of::<lzma_options_bcj>() as u32,
            id: LZMA_FILTER_X86,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const c_char,
                        *const c_char,
                        *mut c_void,
                    ) -> *const c_char,
            ),
            optmap: ::core::ptr::addr_of!(bcj_optmap) as *const option_map,
            strfy_encoder: 1,
            strfy_decoder: 1,
            allow_null: true,
        },
        filter_codec_def {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"arm\0\0\0\0\0\0\0\0\0"),
            opts_size: core::mem::size_of::<lzma_options_bcj>() as u32,
            id: LZMA_FILTER_ARM,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const c_char,
                        *const c_char,
                        *mut c_void,
                    ) -> *const c_char,
            ),
            optmap: ::core::ptr::addr_of!(bcj_optmap) as *const option_map,
            strfy_encoder: 1,
            strfy_decoder: 1,
            allow_null: true,
        },
        filter_codec_def {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"armthumb\0\0\0\0"),
            opts_size: core::mem::size_of::<lzma_options_bcj>() as u32,
            id: LZMA_FILTER_ARMTHUMB,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const c_char,
                        *const c_char,
                        *mut c_void,
                    ) -> *const c_char,
            ),
            optmap: ::core::ptr::addr_of!(bcj_optmap) as *const option_map,
            strfy_encoder: 1,
            strfy_decoder: 1,
            allow_null: true,
        },
        filter_codec_def {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"arm64\0\0\0\0\0\0\0"),
            opts_size: core::mem::size_of::<lzma_options_bcj>() as u32,
            id: LZMA_FILTER_ARM64,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const c_char,
                        *const c_char,
                        *mut c_void,
                    ) -> *const c_char,
            ),
            optmap: ::core::ptr::addr_of!(bcj_optmap) as *const option_map,
            strfy_encoder: 1,
            strfy_decoder: 1,
            allow_null: true,
        },
        filter_codec_def {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"riscv\0\0\0\0\0\0\0"),
            opts_size: core::mem::size_of::<lzma_options_bcj>() as u32,
            id: LZMA_FILTER_RISCV,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const c_char,
                        *const c_char,
                        *mut c_void,
                    ) -> *const c_char,
            ),
            optmap: ::core::ptr::addr_of!(bcj_optmap) as *const option_map,
            strfy_encoder: 1,
            strfy_decoder: 1,
            allow_null: true,
        },
        filter_codec_def {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"powerpc\0\0\0\0\0"),
            opts_size: core::mem::size_of::<lzma_options_bcj>() as u32,
            id: LZMA_FILTER_POWERPC,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const c_char,
                        *const c_char,
                        *mut c_void,
                    ) -> *const c_char,
            ),
            optmap: ::core::ptr::addr_of!(bcj_optmap) as *const option_map,
            strfy_encoder: 1,
            strfy_decoder: 1,
            allow_null: true,
        },
        filter_codec_def {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"ia64\0\0\0\0\0\0\0\0"),
            opts_size: core::mem::size_of::<lzma_options_bcj>() as u32,
            id: LZMA_FILTER_IA64,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const c_char,
                        *const c_char,
                        *mut c_void,
                    ) -> *const c_char,
            ),
            optmap: ::core::ptr::addr_of!(bcj_optmap) as *const option_map,
            strfy_encoder: 1,
            strfy_decoder: 1,
            allow_null: true,
        },
        filter_codec_def {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"sparc\0\0\0\0\0\0\0"),
            opts_size: core::mem::size_of::<lzma_options_bcj>() as u32,
            id: LZMA_FILTER_SPARC,
            parse: Some(
                parse_bcj
                    as unsafe extern "C" fn(
                        *mut *const c_char,
                        *const c_char,
                        *mut c_void,
                    ) -> *const c_char,
            ),
            optmap: ::core::ptr::addr_of!(bcj_optmap) as *const option_map,
            strfy_encoder: 1,
            strfy_decoder: 1,
            allow_null: true,
        },
        filter_codec_def {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"delta\0\0\0\0\0\0\0"),
            opts_size: core::mem::size_of::<lzma_options_delta>() as u32,
            id: LZMA_FILTER_DELTA,
            parse: Some(
                parse_delta
                    as unsafe extern "C" fn(
                        *mut *const c_char,
                        *const c_char,
                        *mut c_void,
                    ) -> *const c_char,
            ),
            optmap: ::core::ptr::addr_of!(delta_optmap) as *const option_map,
            strfy_encoder: 1,
            strfy_decoder: 1,
            allow_null: false,
        },
    ]
};
unsafe fn parse_options(
    str: *mut *const c_char,
    str_end: *const c_char,
    filter_options: *mut c_void,
    optmap: *const option_map,
    optmap_size: size_t,
) -> *const c_char {
    while *str < str_end && **str != 0 {
        if **str as u8 == b',' {
            *str = (*str).offset(1);
        } else {
            let str_len: size_t = str_end.offset_from(*str) as size_t;
            let mut name_eq_value_end: *const c_char =
                memchr(*str as *const c_void, ',' as i32, str_len) as *const c_char;
            if name_eq_value_end.is_null() {
                name_eq_value_end = str_end;
            }
            let equals_sign: *const c_char = memchr(
                *str as *const c_void,
                '=' as i32,
                name_eq_value_end.offset_from(*str) as size_t,
            ) as *const c_char;
            if equals_sign.is_null() || **str as u8 == b'=' {
                return b"Options must be 'name=value' pairs separated with commas\0" as *const u8
                    as *const c_char;
            }
            let name_len: size_t = equals_sign.offset_from(*str) as size_t;
            if name_len > NAME_LEN_MAX as size_t {
                return crate::c_str!("Unknown option name");
            }
            let mut i: size_t = 0;
            loop {
                if i == optmap_size {
                    return crate::c_str!("Unknown option name");
                }
                if memcmp(
                    *str as *const c_void,
                    ::core::ptr::addr_of!((*optmap.offset(i as isize)).name) as *const c_void,
                    name_len,
                ) == 0
                    && (*optmap.offset(i as isize)).name[name_len as usize] == 0
                {
                    break;
                }
                i += 1;
            }
            *str = equals_sign.offset(1);
            let value_len: size_t = name_eq_value_end.offset_from(*str) as size_t;
            if value_len == 0 {
                return crate::c_str!("Option value cannot be empty");
            }
            if (*optmap.offset(i as isize)).type_0 == OPTMAP_TYPE_LZMA_PRESET {
                let errmsg: *const c_char =
                    set_lzma12_preset(str, name_eq_value_end, filter_options);
                if !errmsg.is_null() {
                    return errmsg;
                }
            } else {
                let mut v: u32 = 0;
                if (*optmap.offset(i as isize)).flags & OPTMAP_USE_NAME_VALUE_MAP != 0 {
                    if value_len > NAME_LEN_MAX as size_t {
                        return crate::c_str!("Invalid option value");
                    }
                    let map: *const name_value_map = (*optmap.offset(i as isize)).u.map;
                    let mut j: size_t = 0;
                    loop {
                        if (*map.offset(j as isize)).name[0] == 0 {
                            return crate::c_str!("Invalid option value");
                        }
                        if memcmp(
                            *str as *const c_void,
                            ::core::ptr::addr_of!((*map.offset(j as isize)).name) as *const c_void,
                            value_len,
                        ) == 0
                            && (*map.offset(j as isize)).name[value_len as usize] == 0
                        {
                            v = (*map.offset(j as isize)).value;
                            break;
                        } else {
                            j += 1;
                        }
                    }
                } else if (**str as u8) < b'0' || **str as u8 > b'9' {
                    return b"Value is not a non-negative decimal integer\0" as *const u8
                        as *const c_char;
                } else {
                    let mut p: *const c_char = *str;
                    v = 0;
                    loop {
                        if v > (UINT32_MAX).wrapping_div(10) {
                            return crate::c_str!("Value out of range");
                        }
                        v = v.wrapping_mul(10);
                        let add: u32 = (*p as u8 - b'0') as u32;
                        if (UINT32_MAX).wrapping_sub(add) < v {
                            return crate::c_str!("Value out of range");
                        }
                        v = v.wrapping_add(add);
                        p = p.offset(1);
                        if p >= name_eq_value_end || (*p as u8) < b'0' || (*p as u8) > b'9' {
                            break;
                        }
                    }
                    if p < name_eq_value_end {
                        let multiplier_start: *const c_char = p;
                        if (*optmap.offset(i as isize)).flags & OPTMAP_USE_BYTE_SUFFIX == 0 {
                            *str = multiplier_start;
                            return b"This option does not support any multiplier suffixes\0"
                                as *const u8 as *const c_char;
                        }
                        let mut shift: u32 = 0;
                        match *p {
                            107 | 75 => {
                                shift = 10;
                            }
                            109 | 77 => {
                                shift = 20;
                            }
                            103 | 71 => {
                                shift = 30;
                            }
                            _ => {
                                *str = multiplier_start;
                                return b"Invalid multiplier suffix (KiB, MiB, or GiB)\0"
                                    as *const u8
                                    as *const c_char;
                            }
                        }
                        p = p.offset(1);
                        if p < name_eq_value_end && *p as u8 == b'i' {
                            p = p.offset(1);
                        }
                        if p < name_eq_value_end && *p as u8 == b'B' {
                            p = p.offset(1);
                        }
                        if p < name_eq_value_end {
                            *str = multiplier_start;
                            return b"Invalid multiplier suffix (KiB, MiB, or GiB)\0" as *const u8
                                as *const c_char;
                        }
                        if v > UINT32_MAX >> shift {
                            return crate::c_str!("Value out of range");
                        }
                        v <<= shift;
                    }
                    if v < (*optmap.offset(i as isize)).u.range.min
                        || v > (*optmap.offset(i as isize)).u.range.max
                    {
                        return crate::c_str!("Value out of range");
                    }
                }
                let ptr: *mut c_void = (filter_options as *mut c_char)
                    .offset((*optmap.offset(i as isize)).offset as isize)
                    as *mut c_void;
                match (*optmap.offset(i as isize)).type_0 {
                    1 => {
                        *(ptr as *mut lzma_mode) = v as lzma_mode;
                    }
                    2 => {
                        *(ptr as *mut lzma_match_finder) = v as lzma_match_finder;
                    }
                    _ => {
                        *(ptr as *mut u32) = v;
                    }
                }
                *str = name_eq_value_end;
            }
        }
    }
    core::ptr::null()
}
unsafe fn parse_filter(
    str: *mut *const c_char,
    str_end: *const c_char,
    filter: *mut lzma_filter,
    allocator: *const lzma_allocator,
    only_xz: bool,
) -> *const c_char {
    let mut name_end: *const c_char = str_end;
    let mut opts_start: *const c_char = str_end;
    let mut p: *const c_char = *str;
    while p < str_end {
        if *p as u8 == b':' || *p as u8 == b'=' {
            name_end = p;
            opts_start = p.offset(1);
            break;
        } else {
            p = p.offset(1);
        }
    }
    let name_len: size_t = name_end.offset_from(*str) as size_t;
    if name_len > NAME_LEN_MAX as size_t {
        return crate::c_str!("Unknown filter name");
    }
    let mut i: size_t = 0;
    while i < core::mem::size_of::<[filter_codec_def; 11]>()
        / core::mem::size_of::<filter_codec_def>()
    {
        if memcmp(
            *str as *const c_void,
            ::core::ptr::addr_of!(
                (*(::core::ptr::addr_of!(filter_name_map) as *const filter_codec_def)
                    .offset(i as isize))
                .name
            ) as *const c_void,
            name_len,
        ) == 0
            && filter_name_map[i as usize].name[name_len as usize] == 0
        {
            if only_xz && filter_name_map[i as usize].id >= LZMA_FILTER_RESERVED_START {
                return b"This filter cannot be used in the .xz format\0" as *const u8
                    as *const c_char;
            }
            let options: *mut c_void =
                lzma_alloc_zero(filter_name_map[i as usize].opts_size as size_t, allocator);
            if options.is_null() {
                return crate::c_str!("Memory allocation failed");
            }
            *str = opts_start;
            let errmsg: *const c_char =
                filter_name_map[i as usize].parse.unwrap()(str, str_end, options);
            if !errmsg.is_null() {
                lzma_free(options, allocator);
                return errmsg;
            }
            (*filter).id = filter_name_map[i as usize].id;
            (*filter).options = options;
            return core::ptr::null();
        }
        i += 1;
    }
    crate::c_str!("Unknown filter name")
}
unsafe fn str_to_filters(
    str: *mut *const c_char,
    filters: *mut lzma_filter,
    flags: u32,
    allocator: *const lzma_allocator,
) -> *const c_char {
    let mut current_block: u64;
    let mut errmsg: *const c_char = core::ptr::null();
    while **str as u8 == b' ' {
        *str = (*str).offset(1);
    }
    if **str == 0 {
        return b"Empty string is not allowed, try '6' if a default value is needed\0" as *const u8
            as *const c_char;
    }
    if **str as u8 >= b'0' && **str as u8 <= b'9'
        || **str as u8 == b'-'
            && (*(*str).offset(1) as u8 >= b'0' && *(*str).offset(1) as u8 <= b'9')
    {
        if **str as u8 == b'-' {
            *str = (*str).offset(1);
        }
        let str_len: size_t = strlen(*str) as size_t;
        let mut str_end: *const c_char =
            memchr(*str as *const c_void, ' ' as i32, str_len) as *const c_char;
        if !str_end.is_null() {
            let mut i: size_t = 1;
            while *str_end.offset(i as isize) != 0 {
                if *str_end.offset(i as isize) as u8 != b' ' {
                    return crate::c_str!("Unsupported preset");
                }
                i += 1;
            }
        } else {
            str_end = (*str).offset(str_len as isize);
        }
        let mut preset: u32 = 0;
        errmsg = parse_lzma12_preset(str, str_end, ::core::ptr::addr_of_mut!(preset));
        if !errmsg.is_null() {
            return errmsg;
        }
        let opts: *mut lzma_options_lzma =
            lzma_alloc(core::mem::size_of::<lzma_options_lzma>(), allocator)
                as *mut lzma_options_lzma;
        if opts.is_null() {
            return crate::c_str!("Memory allocation failed");
        }
        if lzma_lzma_preset(opts, preset) != 0 {
            lzma_free(opts as *mut c_void, allocator);
            return crate::c_str!("Unsupported preset");
        }
        (*filters).id = LZMA_FILTER_LZMA2;
        (*filters).options = opts as *mut c_void;
        (*filters.offset(1)).id = LZMA_VLI_UNKNOWN;
        (*filters.offset(1)).options = core::ptr::null_mut();
        return core::ptr::null();
    }
    let only_xz: bool = flags & LZMA_STR_ALL_FILTERS as u32 == 0;
    let mut temp_filters: [lzma_filter; 5] = [lzma_filter {
        id: 0,
        options: core::ptr::null_mut(),
    }; 5];
    let mut i_0: size_t = 0;
    loop {
        if i_0 == LZMA_FILTERS_MAX as size_t {
            errmsg = crate::c_str!("The maximum number of filters is four");
            current_block = 6100283484465977373;
            break;
        } else {
            if *(*str) as u8 == b'-' && *(*str).offset(1) as u8 == b'-' {
                *str = (*str).offset(2);
            }
            let mut filter_end: *const c_char = *str;
            while *filter_end != 0 {
                if *filter_end as u8 == b'-' && *filter_end.offset(1) as u8 == b'-'
                    || *filter_end as u8 == b' '
                {
                    break;
                }
                filter_end = filter_end.offset(1);
            }
            if filter_end == *str {
                errmsg = crate::c_str!("Filter name is missing");
                current_block = 6100283484465977373;
                break;
            } else {
                errmsg = parse_filter(
                    str,
                    filter_end,
                    (::core::ptr::addr_of_mut!(temp_filters) as *mut lzma_filter)
                        .offset(i_0 as isize) as *mut lzma_filter,
                    allocator,
                    only_xz,
                );
                if !errmsg.is_null() {
                    current_block = 6100283484465977373;
                    break;
                }
                while **str as u8 == b' ' {
                    *str = (*str).offset(1);
                }
                i_0 += 1;
                if **str == 0 {
                    current_block = 15090052786889560393;
                    break;
                }
            }
        }
    }
    match current_block {
        15090052786889560393 => {
            temp_filters[i_0 as usize].id = LZMA_VLI_UNKNOWN;
            temp_filters[i_0 as usize].options = core::ptr::null_mut();
            if flags & LZMA_STR_NO_VALIDATION as u32 == 0 {
                let mut dummy: size_t = 0;
                let ret: lzma_ret = lzma_validate_chain(
                    ::core::ptr::addr_of_mut!(temp_filters) as *mut lzma_filter,
                    ::core::ptr::addr_of_mut!(dummy),
                );
                if ret != LZMA_OK {
                    errmsg = b"Invalid filter chain ('lzma2' missing at the end?)\0" as *const u8
                        as *const c_char;
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
                    core::ptr::copy_nonoverlapping(
                        ::core::ptr::addr_of_mut!(temp_filters) as *const u8,
                        filters as *mut u8,
                        (i_0 + 1) * core::mem::size_of::<lzma_filter>(),
                    );
                    return core::ptr::null();
                }
            }
        }
        _ => {}
    }
    loop {
        let old_i = i_0;
        i_0 -= 1;
        if old_i == 0 {
            break;
        }
        lzma_free(temp_filters[i_0 as usize].options, allocator);
    }
    errmsg
}
pub unsafe fn lzma_str_to_filters(
    str: *const c_char,
    error_pos: *mut c_int,
    filters: *mut lzma_filter,
    flags: u32,
    allocator: *const lzma_allocator,
) -> *const c_char {
    if !error_pos.is_null() {
        *error_pos = 0;
    }
    if str.is_null() || filters.is_null() {
        return b"Unexpected core::ptr::null_mut() pointer argument(s) to lzma_str_to_filters()\0"
            as *const u8 as *const c_char;
    }
    let supported_flags: u32 = LZMA_STR_ALL_FILTERS as u32 | LZMA_STR_NO_VALIDATION as u32;
    if flags & !supported_flags != 0 {
        return crate::c_str!("Unsupported flags to lzma_str_to_filters()");
    }
    let mut used: *const c_char = str;
    let errmsg: *const c_char =
        str_to_filters(::core::ptr::addr_of_mut!(used), filters, flags, allocator);
    if !error_pos.is_null() {
        let n: size_t = used.offset_from(str) as size_t;
        *error_pos = if n > INT_MAX as size_t {
            INT_MAX
        } else {
            n as c_int
        };
    }
    errmsg
}
unsafe fn strfy_filter(
    dest: *mut lzma_str,
    mut delimiter: *const c_char,
    optmap: *const option_map,
    optmap_count: size_t,
    filter_options: *const c_void,
) {
    let mut i: size_t = 0;
    while i < optmap_count {
        if (*optmap.offset(i as isize)).type_0 != OPTMAP_TYPE_LZMA_PRESET {
            let mut v: u32 = 0;
            let ptr: *const c_void = (filter_options as *const c_char)
                .offset((*optmap.offset(i as isize)).offset as isize)
                as *const c_void;
            match (*optmap.offset(i as isize)).type_0 {
                1 => {
                    v = *(ptr as *const lzma_mode) as u32;
                }
                2 => {
                    v = *(ptr as *const lzma_match_finder) as u32;
                }
                _ => {
                    v = *(ptr as *const u32);
                }
            }
            if v != 0 || (*optmap.offset(i as isize)).flags & OPTMAP_NO_STRFY_ZERO == 0 {
                str_append_str(dest, delimiter);
                delimiter = crate::c_str!(",");
                str_append_str(
                    dest,
                    ::core::ptr::addr_of!((*optmap.offset(i as isize)).name) as *const c_char,
                );
                str_append_str(dest, crate::c_str!("="));
                if (*optmap.offset(i as isize)).flags & OPTMAP_USE_NAME_VALUE_MAP != 0 {
                    let map: *const name_value_map = (*optmap.offset(i as isize)).u.map;
                    let mut j: size_t = 0;
                    loop {
                        if (*map.offset(j as isize)).name[0] == 0 {
                            str_append_str(dest, crate::c_str!("UNKNOWN"));
                            break;
                        } else if (*map.offset(j as isize)).value == v {
                            str_append_str(
                                dest,
                                ::core::ptr::addr_of!((*map.offset(j as isize)).name)
                                    as *const c_char,
                            );
                            break;
                        } else {
                            j += 1;
                        }
                    }
                } else {
                    str_append_u32(
                        dest,
                        v,
                        (*optmap.offset(i as isize)).flags & OPTMAP_USE_BYTE_SUFFIX != 0,
                    );
                }
            }
        }
        i += 1;
    }
}
pub unsafe fn lzma_str_from_filters(
    output_str: *mut *mut c_char,
    filters: *const lzma_filter,
    flags: u32,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    if output_str.is_null() {
        return LZMA_PROG_ERROR;
    }
    *output_str = core::ptr::null_mut();
    if filters.is_null() {
        return LZMA_PROG_ERROR;
    }
    let supported_flags: u32 = LZMA_STR_ENCODER as u32
        | LZMA_STR_DECODER as u32
        | LZMA_STR_GETOPT_LONG as u32
        | LZMA_STR_NO_SPACES as u32;
    if flags & !supported_flags != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    if (*filters).id == LZMA_VLI_UNKNOWN {
        return LZMA_OPTIONS_ERROR;
    }
    let mut dest: lzma_str = lzma_str {
        buf: core::ptr::null_mut(),
        pos: 0,
    };
    let ret_: lzma_ret = str_init(::core::ptr::addr_of_mut!(dest), allocator);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let show_opts: bool = flags & (LZMA_STR_ENCODER as u32 | LZMA_STR_DECODER as u32) != 0;
    let opt_delim: *const c_char = if flags & LZMA_STR_GETOPT_LONG as u32 != 0 {
        crate::c_str!("=")
    } else {
        crate::c_str!(":")
    };
    let mut i: size_t = 0;
    while (*filters.offset(i as isize)).id != LZMA_VLI_UNKNOWN {
        if i == LZMA_FILTERS_MAX as size_t {
            str_free(::core::ptr::addr_of_mut!(dest), allocator);
            return LZMA_OPTIONS_ERROR;
        }
        if i > 0 && flags & LZMA_STR_NO_SPACES as u32 == 0 {
            str_append_str(::core::ptr::addr_of_mut!(dest), crate::c_str!(" "));
        }
        if flags & LZMA_STR_GETOPT_LONG as u32 != 0
            || i > 0 && flags & LZMA_STR_NO_SPACES as u32 != 0
        {
            str_append_str(::core::ptr::addr_of_mut!(dest), crate::c_str!("--"));
        }
        let mut j: size_t = 0;
        loop {
            if j == (core::mem::size_of::<[filter_codec_def; 11]>())
                .wrapping_div(core::mem::size_of::<filter_codec_def>())
            {
                str_free(::core::ptr::addr_of_mut!(dest), allocator);
                return LZMA_OPTIONS_ERROR;
            }
            if filter_name_map[j as usize].id == (*filters.offset(i as isize)).id {
                str_append_str(
                    ::core::ptr::addr_of_mut!(dest),
                    ::core::ptr::addr_of!(
                        (*(::core::ptr::addr_of!(filter_name_map) as *const filter_codec_def)
                            .offset(j as isize))
                        .name
                    ) as *const c_char,
                );
                if !show_opts {
                    break;
                }
                if (*filters.offset(i as isize)).options.is_null() {
                    if !filter_name_map[j as usize].allow_null {
                        str_free(::core::ptr::addr_of_mut!(dest), allocator);
                        return LZMA_OPTIONS_ERROR;
                    }
                    break;
                } else {
                    let optmap_count: size_t = (if flags & LZMA_STR_ENCODER as u32 != 0 {
                        filter_name_map[j as usize].strfy_encoder
                    } else {
                        filter_name_map[j as usize].strfy_decoder
                    }) as size_t;
                    strfy_filter(
                        ::core::ptr::addr_of_mut!(dest),
                        opt_delim,
                        filter_name_map[j as usize].optmap,
                        optmap_count,
                        (*filters.offset(i as isize)).options,
                    );
                    break;
                }
            } else {
                j += 1;
            }
        }
        i += 1;
    }
    str_finish(output_str, ::core::ptr::addr_of_mut!(dest), allocator)
}
pub unsafe fn lzma_str_list_filters(
    output_str: *mut *mut c_char,
    filter_id: lzma_vli,
    flags: u32,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    if output_str.is_null() {
        return LZMA_PROG_ERROR;
    }
    *output_str = core::ptr::null_mut();
    let supported_flags: u32 = LZMA_STR_ALL_FILTERS as u32
        | LZMA_STR_ENCODER as u32
        | LZMA_STR_DECODER as u32
        | LZMA_STR_GETOPT_LONG as u32;
    if flags & !supported_flags != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut dest: lzma_str = lzma_str {
        buf: core::ptr::null_mut(),
        pos: 0,
    };
    let ret_: lzma_ret = str_init(::core::ptr::addr_of_mut!(dest), allocator);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let show_opts: bool = flags & (LZMA_STR_ENCODER as u32 | LZMA_STR_DECODER as u32) != 0;
    let filter_delim: *const c_char = if show_opts {
        crate::c_str!("\n")
    } else {
        crate::c_str!(" ")
    };
    let opt_delim: *const c_char = if flags & LZMA_STR_GETOPT_LONG as u32 != 0 {
        crate::c_str!("=")
    } else {
        crate::c_str!(":")
    };
    let mut first_filter_printed: bool = false;
    let mut i: size_t = 0;
    while i
        < (core::mem::size_of::<[filter_codec_def; 11]>())
            .wrapping_div(core::mem::size_of::<filter_codec_def>())
    {
        if filter_id == LZMA_VLI_UNKNOWN || filter_id == filter_name_map[i as usize].id {
            if filter_name_map[i as usize].id < LZMA_FILTER_RESERVED_START
                || flags & LZMA_STR_ALL_FILTERS as u32 != 0
                || filter_id != LZMA_VLI_UNKNOWN
            {
                if first_filter_printed {
                    str_append_str(::core::ptr::addr_of_mut!(dest), filter_delim);
                }
                first_filter_printed = true;
                if flags & LZMA_STR_GETOPT_LONG as u32 != 0 {
                    str_append_str(::core::ptr::addr_of_mut!(dest), crate::c_str!("--"));
                }
                str_append_str(
                    ::core::ptr::addr_of_mut!(dest),
                    ::core::ptr::addr_of!(
                        (*(::core::ptr::addr_of!(filter_name_map) as *const filter_codec_def)
                            .offset(i as isize))
                        .name
                    ) as *const c_char,
                );
                if show_opts {
                    let optmap: *const option_map = filter_name_map[i as usize].optmap;
                    let mut d: *const c_char = opt_delim;
                    let end: size_t = (if flags & LZMA_STR_ENCODER as u32 != 0 {
                        filter_name_map[i as usize].strfy_encoder
                    } else {
                        filter_name_map[i as usize].strfy_decoder
                    }) as size_t;
                    let mut j: size_t = 0;
                    while j < end {
                        str_append_str(::core::ptr::addr_of_mut!(dest), d);
                        d = crate::c_str!(",");
                        str_append_str(
                            ::core::ptr::addr_of_mut!(dest),
                            ::core::ptr::addr_of!((*optmap.offset(j as isize)).name)
                                as *const c_char,
                        );
                        str_append_str(::core::ptr::addr_of_mut!(dest), crate::c_str!("=<"));
                        if (*optmap.offset(j as isize)).type_0 == OPTMAP_TYPE_LZMA_PRESET {
                            str_append_str(
                                ::core::ptr::addr_of_mut!(dest),
                                LZMA12_PRESET_STR.as_ptr(),
                            );
                        } else if (*optmap.offset(j as isize)).flags & OPTMAP_USE_NAME_VALUE_MAP
                            != 0
                        {
                            let m: *const name_value_map = (*optmap.offset(j as isize)).u.map;
                            let mut k: size_t = 0;
                            while (*m.offset(k as isize)).name[0] != 0 {
                                if k > 0 {
                                    str_append_str(
                                        ::core::ptr::addr_of_mut!(dest),
                                        crate::c_str!("|"),
                                    );
                                }
                                str_append_str(
                                    ::core::ptr::addr_of_mut!(dest),
                                    ::core::ptr::addr_of!((*m.offset(k as isize)).name)
                                        as *const c_char,
                                );
                                k += 1;
                            }
                        } else {
                            let use_byte_suffix: bool =
                                (*optmap.offset(j as isize)).flags & OPTMAP_USE_BYTE_SUFFIX != 0;
                            str_append_u32(
                                ::core::ptr::addr_of_mut!(dest),
                                (*optmap.offset(j as isize)).u.range.min,
                                use_byte_suffix,
                            );
                            str_append_str(::core::ptr::addr_of_mut!(dest), crate::c_str!("-"));
                            str_append_u32(
                                ::core::ptr::addr_of_mut!(dest),
                                (*optmap.offset(j as isize)).u.range.max,
                                use_byte_suffix,
                            );
                        }
                        str_append_str(::core::ptr::addr_of_mut!(dest), crate::c_str!(">"));
                        j += 1;
                    }
                }
            }
        }
        i += 1;
    }
    if !first_filter_printed {
        str_free(::core::ptr::addr_of_mut!(dest), allocator);
        return LZMA_OPTIONS_ERROR;
    }
    str_finish(output_str, ::core::ptr::addr_of_mut!(dest), allocator)
}
unsafe extern "C" fn run_static_initializers() {
    lzma12_optmap = [
        option_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"preset\0\0\0\0\0\0"),
            type_0: OPTMAP_TYPE_LZMA_PRESET,
            flags: 0,
            offset: 0,
            u: option_value {
                map: core::ptr::null(),
            },
        },
        option_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"dict\0\0\0\0\0\0\0\0"),
            type_0: 0,
            flags: OPTMAP_USE_BYTE_SUFFIX,
            offset: 0,
            u: option_value {
                range: option_value_range {
                    min: LZMA_DICT_SIZE_MIN as u32,
                    max: (1u32 << 30).wrapping_add(1 << 29),
                },
            },
        },
        option_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"lc\0\0\0\0\0\0\0\0\0\0"),
            type_0: 0,
            flags: 0,
            offset: 20,
            u: option_value {
                range: option_value_range {
                    min: LZMA_LCLP_MIN,
                    max: LZMA_LCLP_MAX,
                },
            },
        },
        option_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"lp\0\0\0\0\0\0\0\0\0\0"),
            type_0: 0,
            flags: 0,
            offset: 24,
            u: option_value {
                range: option_value_range {
                    min: LZMA_LCLP_MIN,
                    max: LZMA_LCLP_MAX,
                },
            },
        },
        option_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"pb\0\0\0\0\0\0\0\0\0\0"),
            type_0: 0,
            flags: 0,
            offset: 28,
            u: option_value {
                range: option_value_range {
                    min: LZMA_PB_MIN,
                    max: LZMA_PB_MAX,
                },
            },
        },
        option_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"mode\0\0\0\0\0\0\0\0"),
            type_0: OPTMAP_TYPE_LZMA_MODE,
            flags: OPTMAP_USE_NAME_VALUE_MAP,
            offset: 32,
            u: option_value {
                map: ::core::ptr::addr_of!(lzma12_mode_map) as *const name_value_map,
            },
        },
        option_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"nice\0\0\0\0\0\0\0\0"),
            type_0: 0,
            flags: 0,
            offset: 36,
            u: option_value {
                range: option_value_range { min: 2, max: 273 },
            },
        },
        option_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"mf\0\0\0\0\0\0\0\0\0\0"),
            type_0: OPTMAP_TYPE_LZMA_MATCH_FINDER,
            flags: OPTMAP_USE_NAME_VALUE_MAP,
            offset: 40,
            u: option_value {
                map: ::core::ptr::addr_of!(lzma12_mf_map) as *const name_value_map,
            },
        },
        option_map {
            name: core::mem::transmute::<[u8; 12], [c_char; 12]>(*b"depth\0\0\0\0\0\0\0"),
            type_0: 0,
            flags: 0,
            offset: 44,
            u: option_value {
                range: option_value_range {
                    min: 0,
                    max: UINT32_MAX,
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
