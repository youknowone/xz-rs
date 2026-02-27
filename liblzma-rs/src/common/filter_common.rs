use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
extern "C" {
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_next_filter_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub id: lzma_vli,
    pub options_size: size_t,
    pub non_last_ok: bool,
    pub last_ok: bool,
    pub changes_size: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_delta {
    pub type_0: lzma_delta_type,
    pub dist: u32,
    pub reserved_int1: u32,
    pub reserved_int2: u32,
    pub reserved_int3: u32,
    pub reserved_int4: u32,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
}
pub const LZMA_DELTA_TYPE_BYTE: lzma_delta_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_bcj {
    pub start_offset: u32,
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
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;
pub const LZMA_MODE_FAST: lzma_mode = 1;
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
pub struct lzma_filter_coder {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub memusage: Option<unsafe extern "C" fn(*const c_void) -> u64>,
}
pub type lzma_filter_find = Option<unsafe extern "C" fn(lzma_vli) -> *const lzma_filter_coder>;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_FILTERS_MAX: c_int = 4 as c_int;
pub const LZMA_FILTER_X86: c_ulonglong = 0x4;
pub const LZMA_FILTER_POWERPC: c_ulonglong = 0x5;
pub const LZMA_FILTER_IA64: c_ulonglong = 0x6;
pub const LZMA_FILTER_ARM: c_ulonglong = 0x7;
pub const LZMA_FILTER_ARMTHUMB: c_ulonglong = 0x8;
pub const LZMA_FILTER_SPARC: c_ulonglong = 0x9;
pub const LZMA_FILTER_ARM64: c_ulonglong = 0xa;
pub const LZMA_FILTER_RISCV: c_ulonglong = 0xb;
pub const LZMA_FILTER_DELTA: c_ulonglong = 0x3;
pub const LZMA_FILTER_LZMA1: c_ulonglong = 0x4000000000000001;
pub const LZMA_FILTER_LZMA1EXT: c_ulonglong = 0x4000000000000002;
pub const LZMA_FILTER_LZMA2: c_ulonglong = 0x21;
pub const LZMA_MEMUSAGE_BASE: c_ulonglong = 1 << 15;
static mut features: [C2RustUnnamed; 13] = [
    C2RustUnnamed {
        id: LZMA_FILTER_LZMA1 as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_lzma>() as size_t,
        non_last_ok: false_0 != 0,
        last_ok: true_0 != 0,
        changes_size: true_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_LZMA1EXT as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_lzma>() as size_t,
        non_last_ok: false_0 != 0,
        last_ok: true_0 != 0,
        changes_size: true_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_LZMA2 as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_lzma>() as size_t,
        non_last_ok: false_0 != 0,
        last_ok: true_0 != 0,
        changes_size: true_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_X86 as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_bcj>() as size_t,
        non_last_ok: true_0 != 0,
        last_ok: false_0 != 0,
        changes_size: false_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_POWERPC as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_bcj>() as size_t,
        non_last_ok: true_0 != 0,
        last_ok: false_0 != 0,
        changes_size: false_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_IA64 as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_bcj>() as size_t,
        non_last_ok: true_0 != 0,
        last_ok: false_0 != 0,
        changes_size: false_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_ARM as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_bcj>() as size_t,
        non_last_ok: true_0 != 0,
        last_ok: false_0 != 0,
        changes_size: false_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_ARMTHUMB as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_bcj>() as size_t,
        non_last_ok: true_0 != 0,
        last_ok: false_0 != 0,
        changes_size: false_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_ARM64 as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_bcj>() as size_t,
        non_last_ok: true_0 != 0,
        last_ok: false_0 != 0,
        changes_size: false_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_SPARC as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_bcj>() as size_t,
        non_last_ok: true_0 != 0,
        last_ok: false_0 != 0,
        changes_size: false_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_RISCV as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_bcj>() as size_t,
        non_last_ok: true_0 != 0,
        last_ok: false_0 != 0,
        changes_size: false_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_DELTA as lzma_vli,
        options_size: ::core::mem::size_of::<lzma_options_delta>() as size_t,
        non_last_ok: true_0 != 0,
        last_ok: false_0 != 0,
        changes_size: false_0 != 0,
    },
    C2RustUnnamed {
        id: LZMA_VLI_UNKNOWN as lzma_vli,
        options_size: 0,
        non_last_ok: false,
        last_ok: false,
        changes_size: false,
    },
];
#[no_mangle]
pub unsafe extern "C" fn lzma_filters_copy(
    mut src: *const lzma_filter,
    mut real_dest: *mut lzma_filter,
    mut allocator: *const lzma_allocator,
) -> lzma_ret {
    let mut current_block: u64;
    if src.is_null() || real_dest.is_null() {
        return LZMA_PROG_ERROR;
    }
    let mut dest: [lzma_filter; 5] = [lzma_filter {
        id: 0,
        options: ::core::ptr::null_mut::<c_void>(),
    }; 5];
    let mut ret: lzma_ret = LZMA_OK;
    let mut i: size_t = 0;
    i = 0 as size_t;
    's_15: loop {
        if !((*src.offset(i as isize)).id != LZMA_VLI_UNKNOWN as lzma_vli) {
            current_block = 7175849428784450219;
            break;
        }
        if i == LZMA_FILTERS_MAX as size_t {
            ret = LZMA_OPTIONS_ERROR;
            current_block = 6392083060350426025;
            break;
        } else {
            dest[i as usize].id = (*src.offset(i as isize)).id;
            if (*src.offset(i as isize)).options.is_null() {
                dest[i as usize].options = NULL;
            } else {
                let mut j: size_t = 0;
                j = 0 as size_t;
                while (*src.offset(i as isize)).id != features[j as usize].id {
                    if features[j as usize].id == LZMA_VLI_UNKNOWN as lzma_vli {
                        ret = LZMA_OPTIONS_ERROR;
                        current_block = 6392083060350426025;
                        break 's_15;
                    } else {
                        j = j.wrapping_add(1);
                    }
                }
                dest[i as usize].options = lzma_alloc(features[j as usize].options_size, allocator);
                if dest[i as usize].options.is_null() {
                    ret = LZMA_MEM_ERROR;
                    current_block = 6392083060350426025;
                    break;
                } else {
                    memcpy(
                        dest[i as usize].options,
                        (*src.offset(i as isize)).options,
                        features[j as usize].options_size,
                    );
                }
            }
            i = i.wrapping_add(1);
        }
    }
    match current_block {
        6392083060350426025 => {
            while i > 0 as size_t {
                i = i.wrapping_sub(1);
                lzma_free(dest[i as usize].options, allocator);
            }
            return ret;
        }
        _ => {
            dest[i as usize].id = LZMA_VLI_UNKNOWN as lzma_vli;
            dest[i as usize].options = NULL;
            memcpy(
                real_dest as *mut c_void,
                &raw mut dest as *mut lzma_filter as *const c_void,
                i.wrapping_add(1 as size_t)
                    .wrapping_mul(::core::mem::size_of::<lzma_filter>() as size_t),
            );
            return LZMA_OK;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_filters_free(
    mut filters: *mut lzma_filter,
    mut allocator: *const lzma_allocator,
) {
    if filters.is_null() {
        return;
    }
    let mut i: size_t = 0 as size_t;
    while (*filters.offset(i as isize)).id != LZMA_VLI_UNKNOWN as lzma_vli {
        if i == LZMA_FILTERS_MAX as size_t {
            break;
        }
        lzma_free((*filters.offset(i as isize)).options, allocator);
        let ref mut fresh0 = (*filters.offset(i as isize)).options;
        *fresh0 = NULL;
        (*filters.offset(i as isize)).id = LZMA_VLI_UNKNOWN as lzma_vli;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_validate_chain(
    mut filters: *const lzma_filter,
    mut count: *mut size_t,
) -> lzma_ret {
    if filters.is_null() || (*filters.offset(0)).id == LZMA_VLI_UNKNOWN as lzma_vli {
        return LZMA_PROG_ERROR;
    }
    let mut changes_size_count: size_t = 0 as size_t;
    let mut non_last_ok: bool = true_0 != 0;
    let mut last_ok: bool = false_0 != 0;
    let mut i: size_t = 0 as size_t;
    loop {
        let mut j: size_t = 0;
        j = 0 as size_t;
        while (*filters.offset(i as isize)).id != features[j as usize].id {
            if features[j as usize].id == LZMA_VLI_UNKNOWN as lzma_vli {
                return LZMA_OPTIONS_ERROR;
            }
            j = j.wrapping_add(1);
        }
        if !non_last_ok {
            return LZMA_OPTIONS_ERROR;
        }
        non_last_ok = features[j as usize].non_last_ok;
        last_ok = features[j as usize].last_ok;
        changes_size_count =
            changes_size_count.wrapping_add(features[j as usize].changes_size as size_t);
        i = i.wrapping_add(1);
        if !((*filters.offset(i as isize)).id != LZMA_VLI_UNKNOWN as lzma_vli) {
            break;
        }
    }
    if i > LZMA_FILTERS_MAX as size_t || !last_ok || changes_size_count > 3 as size_t {
        return LZMA_OPTIONS_ERROR;
    }
    *count = i;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_coder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut options: *const lzma_filter,
    mut coder_find: lzma_filter_find,
    mut is_encoder: bool,
) -> lzma_ret {
    let mut count: size_t = 0;
    let ret_: lzma_ret = lzma_validate_chain(options, &raw mut count) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let mut filters: [lzma_filter_info; 5] = [lzma_filter_info_s {
        id: 0,
        init: None,
        options: ::core::ptr::null_mut::<c_void>(),
    }; 5];
    if is_encoder {
        let mut i: size_t = 0 as size_t;
        while i < count {
            let j: size_t = count.wrapping_sub(i).wrapping_sub(1 as size_t);
            let fc: *const lzma_filter_coder =
                coder_find.expect("non-null function pointer")((*options.offset(i as isize)).id)
                    as *const lzma_filter_coder;
            if fc.is_null() || (*fc).init.is_none() {
                return LZMA_OPTIONS_ERROR;
            }
            filters[j as usize].id = (*options.offset(i as isize)).id;
            filters[j as usize].init = (*fc).init;
            filters[j as usize].options = (*options.offset(i as isize)).options;
            i = i.wrapping_add(1);
        }
    } else {
        let mut i_0: size_t = 0 as size_t;
        while i_0 < count {
            let fc_0: *const lzma_filter_coder =
                coder_find.expect("non-null function pointer")((*options.offset(i_0 as isize)).id)
                    as *const lzma_filter_coder;
            if fc_0.is_null() || (*fc_0).init.is_none() {
                return LZMA_OPTIONS_ERROR;
            }
            filters[i_0 as usize].id = (*options.offset(i_0 as isize)).id;
            filters[i_0 as usize].init = (*fc_0).init;
            filters[i_0 as usize].options = (*options.offset(i_0 as isize)).options;
            i_0 = i_0.wrapping_add(1);
        }
    }
    filters[count as usize].id = LZMA_VLI_UNKNOWN as lzma_vli;
    filters[count as usize].init = None;
    let ret: lzma_ret =
        lzma_next_filter_init(next, allocator, &raw mut filters as *mut lzma_filter_info)
            as lzma_ret;
    if ret != LZMA_OK {
        lzma_next_end(next, allocator);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_coder_memusage(
    mut coder_find: lzma_filter_find,
    mut filters: *const lzma_filter,
) -> u64 {
    let mut tmp: size_t = 0;
    if lzma_validate_chain(filters, &raw mut tmp) != LZMA_OK {
        return UINT64_MAX as u64;
    }
    let mut total: u64 = 0 as u64;
    let mut i: size_t = 0 as size_t;
    loop {
        let fc: *const lzma_filter_coder =
            coder_find.expect("non-null function pointer")((*filters.offset(i as isize)).id)
                as *const lzma_filter_coder;
        if fc.is_null() {
            return UINT64_MAX as u64;
        }
        if (*fc).memusage.is_none() {
            total = total.wrapping_add(1024 as u64);
        } else {
            let usage: u64 = (*fc).memusage.expect("non-null function pointer")(
                (*filters.offset(i as isize)).options,
            ) as u64;
            if usage == UINT64_MAX as u64 {
                return UINT64_MAX as u64;
            }
            total = total.wrapping_add(usage);
        }
        i = i.wrapping_add(1);
        if !((*filters.offset(i as isize)).id != LZMA_VLI_UNKNOWN as lzma_vli) {
            break;
        }
    }
    return total.wrapping_add(LZMA_MEMUSAGE_BASE as u64);
}
