use crate::types::*;
use core::ffi::c_void;
extern "C" {
    fn lzma_next_filter_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_coder {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub memusage: Option<unsafe extern "C" fn(*const c_void) -> u64>,
}
pub type lzma_filter_find = Option<unsafe extern "C" fn(lzma_vli) -> *const lzma_filter_coder>;
static mut features: [C2RustUnnamed; 13] = [
    C2RustUnnamed {
        id: LZMA_FILTER_LZMA1 as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_lzma>(),
        non_last_ok: false,
        last_ok: true,
        changes_size: true,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_LZMA1EXT as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_lzma>(),
        non_last_ok: false,
        last_ok: true,
        changes_size: true,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_LZMA2 as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_lzma>(),
        non_last_ok: false,
        last_ok: true,
        changes_size: true,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_X86 as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_bcj>(),
        non_last_ok: true,
        last_ok: false,
        changes_size: false,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_POWERPC as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_bcj>(),
        non_last_ok: true,
        last_ok: false,
        changes_size: false,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_IA64 as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_bcj>(),
        non_last_ok: true,
        last_ok: false,
        changes_size: false,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_ARM as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_bcj>(),
        non_last_ok: true,
        last_ok: false,
        changes_size: false,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_ARMTHUMB as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_bcj>(),
        non_last_ok: true,
        last_ok: false,
        changes_size: false,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_ARM64 as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_bcj>(),
        non_last_ok: true,
        last_ok: false,
        changes_size: false,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_SPARC as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_bcj>(),
        non_last_ok: true,
        last_ok: false,
        changes_size: false,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_RISCV as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_bcj>(),
        non_last_ok: true,
        last_ok: false,
        changes_size: false,
    },
    C2RustUnnamed {
        id: LZMA_FILTER_DELTA as lzma_vli,
        options_size: core::mem::size_of::<lzma_options_delta>(),
        non_last_ok: true,
        last_ok: false,
        changes_size: false,
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
    src: *const lzma_filter,
    real_dest: *mut lzma_filter,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    let current_block: u64;
    if src.is_null() || real_dest.is_null() {
        return LZMA_PROG_ERROR;
    }
    let mut dest: [lzma_filter; 5] = [lzma_filter {
        id: 0,
        options: core::ptr::null_mut(),
    }; 5];
    let mut ret: lzma_ret = LZMA_OK;
    let mut i: size_t = 0;
    i = 0;
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
                dest[i as usize].options = core::ptr::null_mut();
            } else {
                let mut j: size_t = 0;
                j = 0;
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
            while i > 0 {
                i = i.wrapping_sub(1);
                lzma_free(dest[i as usize].options, allocator);
            }
            return ret;
        }
        _ => {
            dest[i as usize].id = LZMA_VLI_UNKNOWN as lzma_vli;
            dest[i as usize].options = core::ptr::null_mut();
            memcpy(
                real_dest as *mut c_void,
                &raw mut dest as *mut lzma_filter as *const c_void,
                i.wrapping_add(1)
                    .wrapping_mul(core::mem::size_of::<lzma_filter>()),
            );
            return LZMA_OK;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lzma_filters_free(
    filters: *mut lzma_filter,
    allocator: *const lzma_allocator,
) {
    if filters.is_null() {
        return;
    }
    let mut i: size_t = 0;
    while (*filters.offset(i as isize)).id != LZMA_VLI_UNKNOWN as lzma_vli {
        if i == LZMA_FILTERS_MAX as size_t {
            break;
        }
        lzma_free((*filters.offset(i as isize)).options, allocator);
        let ref mut fresh0 = (*filters.offset(i as isize)).options;
        *fresh0 = core::ptr::null_mut();
        (*filters.offset(i as isize)).id = LZMA_VLI_UNKNOWN as lzma_vli;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_validate_chain(
    filters: *const lzma_filter,
    count: *mut size_t,
) -> lzma_ret {
    if filters.is_null() || (*filters.offset(0)).id == LZMA_VLI_UNKNOWN as lzma_vli {
        return LZMA_PROG_ERROR;
    }
    let mut changes_size_count: size_t = 0;
    let mut non_last_ok: bool = true;
    let mut last_ok: bool = false;
    let mut i: size_t = 0;
    loop {
        let mut j: size_t = 0;
        j = 0;
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
    if i > LZMA_FILTERS_MAX as size_t || !last_ok || changes_size_count > 3 {
        return LZMA_OPTIONS_ERROR;
    }
    *count = i;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    options: *const lzma_filter,
    coder_find: lzma_filter_find,
    is_encoder: bool,
) -> lzma_ret {
    let mut count: size_t = 0;
    let ret_: lzma_ret = lzma_validate_chain(options, &raw mut count) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let mut filters: [lzma_filter_info; 5] = [lzma_filter_info_s {
        id: 0,
        init: None,
        options: core::ptr::null_mut(),
    }; 5];
    if is_encoder {
        let mut i: size_t = 0;
        while i < count {
            let j: size_t = count.wrapping_sub(i).wrapping_sub(1);
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
        let mut i_0: size_t = 0;
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
    coder_find: lzma_filter_find,
    filters: *const lzma_filter,
) -> u64 {
    let mut tmp: size_t = 0;
    if lzma_validate_chain(filters, &raw mut tmp) != LZMA_OK {
        return UINT64_MAX as u64;
    }
    let mut total: u64 = 0;
    let mut i: size_t = 0;
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
