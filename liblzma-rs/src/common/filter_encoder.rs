use crate::types::*;
use core::ffi::c_void;
extern "C" {
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret;
    fn lzma_raw_coder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter,
        coder_find_0: lzma_filter_find,
        is_encoder: bool,
    ) -> lzma_ret;
    fn lzma_raw_coder_memusage(coder_find_0: lzma_filter_find, filters: *const lzma_filter) -> u64;
    fn lzma_lzma_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_lzma_encoder_memusage(options: *const c_void) -> u64;
    fn lzma_lzma_props_encode(options: *const c_void, out: *mut u8) -> lzma_ret;
    fn lzma_lzma2_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_lzma2_encoder_memusage(options: *const c_void) -> u64;
    fn lzma_lzma2_props_encode(options: *const c_void, out: *mut u8) -> lzma_ret;
    fn lzma_lzma2_block_size(options: *const c_void) -> u64;
    fn lzma_simple_x86_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_powerpc_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_ia64_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_arm_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_armthumb_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_arm64_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_sparc_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_riscv_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_props_size(size: *mut u32, options: *const c_void) -> lzma_ret;
    fn lzma_simple_props_encode(options: *const c_void, out: *mut u8) -> lzma_ret;
    fn lzma_delta_coder_memusage(options: *const c_void) -> u64;
    fn lzma_delta_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_delta_props_encode(options: *const c_void, out: *mut u8) -> lzma_ret;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_encoder {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub memusage: Option<unsafe extern "C" fn(*const c_void) -> u64>,
    pub block_size: Option<unsafe extern "C" fn(*const c_void) -> u64>,
    pub props_size_get: Option<unsafe extern "C" fn(*mut u32, *const c_void) -> lzma_ret>,
    pub props_size_fixed: u32,
    pub props_encode: Option<unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret>,
}
static encoders: [lzma_filter_encoder; 12] = [
    lzma_filter_encoder {
        id: LZMA_FILTER_LZMA1,
        init: Some(
            lzma_lzma_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: Some(lzma_lzma_encoder_memusage as unsafe extern "C" fn(*const c_void) -> u64),
        block_size: None,
        props_size_get: None,
        props_size_fixed: 5,
        props_encode: Some(
            lzma_lzma_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
    lzma_filter_encoder {
        id: LZMA_FILTER_LZMA1EXT,
        init: Some(
            lzma_lzma_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: Some(lzma_lzma_encoder_memusage as unsafe extern "C" fn(*const c_void) -> u64),
        block_size: None,
        props_size_get: None,
        props_size_fixed: 5,
        props_encode: Some(
            lzma_lzma_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
    lzma_filter_encoder {
        id: LZMA_FILTER_LZMA2,
        init: Some(
            lzma_lzma2_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: Some(lzma_lzma2_encoder_memusage as unsafe extern "C" fn(*const c_void) -> u64),
        block_size: Some(lzma_lzma2_block_size as unsafe extern "C" fn(*const c_void) -> u64),
        props_size_get: None,
        props_size_fixed: 1,
        props_encode: Some(
            lzma_lzma2_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
    lzma_filter_encoder {
        id: LZMA_FILTER_X86,
        init: Some(
            lzma_simple_x86_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        block_size: None,
        props_size_get: Some(
            lzma_simple_props_size as unsafe extern "C" fn(*mut u32, *const c_void) -> lzma_ret,
        ),
        props_size_fixed: 0,
        props_encode: Some(
            lzma_simple_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
    lzma_filter_encoder {
        id: LZMA_FILTER_POWERPC,
        init: Some(
            lzma_simple_powerpc_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        block_size: None,
        props_size_get: Some(
            lzma_simple_props_size as unsafe extern "C" fn(*mut u32, *const c_void) -> lzma_ret,
        ),
        props_size_fixed: 0,
        props_encode: Some(
            lzma_simple_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
    lzma_filter_encoder {
        id: LZMA_FILTER_IA64,
        init: Some(
            lzma_simple_ia64_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        block_size: None,
        props_size_get: Some(
            lzma_simple_props_size as unsafe extern "C" fn(*mut u32, *const c_void) -> lzma_ret,
        ),
        props_size_fixed: 0,
        props_encode: Some(
            lzma_simple_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
    lzma_filter_encoder {
        id: LZMA_FILTER_ARM,
        init: Some(
            lzma_simple_arm_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        block_size: None,
        props_size_get: Some(
            lzma_simple_props_size as unsafe extern "C" fn(*mut u32, *const c_void) -> lzma_ret,
        ),
        props_size_fixed: 0,
        props_encode: Some(
            lzma_simple_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
    lzma_filter_encoder {
        id: LZMA_FILTER_ARMTHUMB,
        init: Some(
            lzma_simple_armthumb_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        block_size: None,
        props_size_get: Some(
            lzma_simple_props_size as unsafe extern "C" fn(*mut u32, *const c_void) -> lzma_ret,
        ),
        props_size_fixed: 0,
        props_encode: Some(
            lzma_simple_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
    lzma_filter_encoder {
        id: LZMA_FILTER_ARM64,
        init: Some(
            lzma_simple_arm64_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        block_size: None,
        props_size_get: Some(
            lzma_simple_props_size as unsafe extern "C" fn(*mut u32, *const c_void) -> lzma_ret,
        ),
        props_size_fixed: 0,
        props_encode: Some(
            lzma_simple_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
    lzma_filter_encoder {
        id: LZMA_FILTER_SPARC,
        init: Some(
            lzma_simple_sparc_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        block_size: None,
        props_size_get: Some(
            lzma_simple_props_size as unsafe extern "C" fn(*mut u32, *const c_void) -> lzma_ret,
        ),
        props_size_fixed: 0,
        props_encode: Some(
            lzma_simple_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
    lzma_filter_encoder {
        id: LZMA_FILTER_RISCV,
        init: Some(
            lzma_simple_riscv_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        block_size: None,
        props_size_get: Some(
            lzma_simple_props_size as unsafe extern "C" fn(*mut u32, *const c_void) -> lzma_ret,
        ),
        props_size_fixed: 0,
        props_encode: Some(
            lzma_simple_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
    lzma_filter_encoder {
        id: LZMA_FILTER_DELTA,
        init: Some(
            lzma_delta_encoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: Some(lzma_delta_coder_memusage as unsafe extern "C" fn(*const c_void) -> u64),
        block_size: None,
        props_size_get: None,
        props_size_fixed: 1,
        props_encode: Some(
            lzma_delta_props_encode as unsafe extern "C" fn(*const c_void, *mut u8) -> lzma_ret,
        ),
    },
];
extern "C" fn encoder_find(id: lzma_vli) -> *const lzma_filter_encoder {
    let mut i: size_t = 0;
    while i
        < (core::mem::size_of::<[lzma_filter_encoder; 12]>())
            .wrapping_div(core::mem::size_of::<lzma_filter_encoder>())
    {
        if encoders[i as usize].id == id {
            return encoders.as_ptr().wrapping_add(i as usize);
        }
        i += 1;
    }
    core::ptr::null()
}
extern "C" fn coder_find(id: lzma_vli) -> *const lzma_filter_coder {
    encoder_find(id) as *const lzma_filter_coder
}
#[no_mangle]
pub extern "C" fn lzma_filter_encoder_is_supported(id: lzma_vli) -> lzma_bool {
    !encoder_find(id).is_null() as lzma_bool
}
#[no_mangle]
pub unsafe extern "C" fn lzma_filters_update(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
) -> lzma_ret {
    if (*(*strm).internal).next.update.is_none() {
        return LZMA_PROG_ERROR;
    }
    if lzma_raw_encoder_memusage(filters) == UINT64_MAX {
        return LZMA_OPTIONS_ERROR;
    }
    let mut count: size_t = 1;
    while (*filters.offset(count as isize)).id != LZMA_VLI_UNKNOWN {
        count += 1;
    }
    let mut reversed_filters: [lzma_filter; 5] = [lzma_filter {
        id: 0,
        options: core::ptr::null_mut(),
    }; 5];
    let mut i: size_t = 0;
    while i < count {
        reversed_filters[count.wrapping_sub(i).wrapping_sub(1) as usize] =
            *filters.offset(i as isize);
        i += 1;
    }
    reversed_filters[count as usize].id = LZMA_VLI_UNKNOWN;
    (*(*strm).internal).next.update.unwrap()(
        (*(*strm).internal).next.coder,
        (*strm).allocator,
        filters,
        &raw mut reversed_filters as *mut lzma_filter,
    )
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter,
) -> lzma_ret {
    lzma_raw_coder_init(
        next,
        allocator,
        filters,
        Some(coder_find as unsafe extern "C" fn(lzma_vli) -> *const lzma_filter_coder),
        true,
    )
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_encoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = lzma_raw_coder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        filters,
        Some(coder_find as unsafe extern "C" fn(lzma_vli) -> *const lzma_filter_coder),
        true,
    );
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_SYNC_FLUSH as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    LZMA_OK
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_encoder_memusage(filters: *const lzma_filter) -> u64 {
    lzma_raw_coder_memusage(
        Some(coder_find as unsafe extern "C" fn(lzma_vli) -> *const lzma_filter_coder),
        filters,
    )
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mt_block_size(filters: *const lzma_filter) -> u64 {
    if filters.is_null() {
        return UINT64_MAX;
    }
    let mut max: u64 = 0;
    let mut i: size_t = 0;
    while (*filters.offset(i as isize)).id != LZMA_VLI_UNKNOWN {
        let fe: *const lzma_filter_encoder =
            encoder_find((*filters.offset(i as isize)).id) as *const lzma_filter_encoder;
        if fe.is_null() {
            return UINT64_MAX;
        }
        if (*fe).block_size.is_some() {
            let size: u64 = (*fe).block_size.unwrap()((*filters.offset(i as isize)).options) as u64;
            if size > max {
                max = size;
            }
        }
        i += 1;
    }
    if max == 0 {
        UINT64_MAX
    } else {
        max
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_properties_size(
    size: *mut u32,
    filter: *const lzma_filter,
) -> lzma_ret {
    let fe: *const lzma_filter_encoder = encoder_find((*filter).id) as *const lzma_filter_encoder;
    if fe.is_null() {
        return if (*filter).id <= LZMA_VLI_MAX {
            LZMA_OPTIONS_ERROR
        } else {
            LZMA_PROG_ERROR
        };
    }
    if (*fe).props_size_get.is_none() {
        *size = (*fe).props_size_fixed;
        return LZMA_OK;
    }
    (*fe).props_size_get.unwrap()(size, (*filter).options)
}
#[no_mangle]
pub unsafe extern "C" fn lzma_properties_encode(
    filter: *const lzma_filter,
    props: *mut u8,
) -> lzma_ret {
    let fe: *const lzma_filter_encoder = encoder_find((*filter).id) as *const lzma_filter_encoder;
    if fe.is_null() {
        return LZMA_PROG_ERROR;
    }
    if (*fe).props_encode.is_none() {
        return LZMA_OK;
    }
    (*fe).props_encode.unwrap()((*filter).options, props)
}
