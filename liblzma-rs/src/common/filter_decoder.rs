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
    fn lzma_lzma_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_lzma_decoder_memusage(options: *const c_void) -> u64;
    fn lzma_lzma_props_decode(
        options: *mut *mut c_void,
        allocator: *const lzma_allocator,
        props: *const u8,
        props_size: size_t,
    ) -> lzma_ret;
    fn lzma_lzma2_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_lzma2_decoder_memusage(options: *const c_void) -> u64;
    fn lzma_lzma2_props_decode(
        options: *mut *mut c_void,
        allocator: *const lzma_allocator,
        props: *const u8,
        props_size: size_t,
    ) -> lzma_ret;
    fn lzma_simple_x86_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_powerpc_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_ia64_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_arm_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_armthumb_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_arm64_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_sparc_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_riscv_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_simple_props_decode(
        options: *mut *mut c_void,
        allocator: *const lzma_allocator,
        props: *const u8,
        props_size: size_t,
    ) -> lzma_ret;
    fn lzma_delta_coder_memusage(options: *const c_void) -> u64;
    fn lzma_delta_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_delta_props_decode(
        options: *mut *mut c_void,
        allocator: *const lzma_allocator,
        props: *const u8,
        props_size: size_t,
    ) -> lzma_ret;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_decoder {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub memusage: Option<unsafe extern "C" fn(*const c_void) -> u64>,
    pub props_decode: Option<
        unsafe extern "C" fn(
            *mut *mut c_void,
            *const lzma_allocator,
            *const u8,
            size_t,
        ) -> lzma_ret,
    >,
}
static decoders: [lzma_filter_decoder; 12] = [
    lzma_filter_decoder {
        id: LZMA_FILTER_LZMA1,
        init: Some(
            lzma_lzma_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: Some(lzma_lzma_decoder_memusage as unsafe extern "C" fn(*const c_void) -> u64),
        props_decode: Some(
            lzma_lzma_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
    lzma_filter_decoder {
        id: LZMA_FILTER_LZMA1EXT,
        init: Some(
            lzma_lzma_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: Some(lzma_lzma_decoder_memusage as unsafe extern "C" fn(*const c_void) -> u64),
        props_decode: Some(
            lzma_lzma_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
    lzma_filter_decoder {
        id: LZMA_FILTER_LZMA2,
        init: Some(
            lzma_lzma2_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: Some(lzma_lzma2_decoder_memusage as unsafe extern "C" fn(*const c_void) -> u64),
        props_decode: Some(
            lzma_lzma2_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
    lzma_filter_decoder {
        id: LZMA_FILTER_X86,
        init: Some(
            lzma_simple_x86_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        props_decode: Some(
            lzma_simple_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
    lzma_filter_decoder {
        id: LZMA_FILTER_POWERPC,
        init: Some(
            lzma_simple_powerpc_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        props_decode: Some(
            lzma_simple_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
    lzma_filter_decoder {
        id: LZMA_FILTER_IA64,
        init: Some(
            lzma_simple_ia64_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        props_decode: Some(
            lzma_simple_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
    lzma_filter_decoder {
        id: LZMA_FILTER_ARM,
        init: Some(
            lzma_simple_arm_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        props_decode: Some(
            lzma_simple_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
    lzma_filter_decoder {
        id: LZMA_FILTER_ARMTHUMB,
        init: Some(
            lzma_simple_armthumb_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        props_decode: Some(
            lzma_simple_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
    lzma_filter_decoder {
        id: LZMA_FILTER_ARM64,
        init: Some(
            lzma_simple_arm64_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        props_decode: Some(
            lzma_simple_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
    lzma_filter_decoder {
        id: LZMA_FILTER_SPARC,
        init: Some(
            lzma_simple_sparc_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        props_decode: Some(
            lzma_simple_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
    lzma_filter_decoder {
        id: LZMA_FILTER_RISCV,
        init: Some(
            lzma_simple_riscv_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: None,
        props_decode: Some(
            lzma_simple_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
    lzma_filter_decoder {
        id: LZMA_FILTER_DELTA,
        init: Some(
            lzma_delta_decoder_init
                as unsafe extern "C" fn(
                    *mut lzma_next_coder,
                    *const lzma_allocator,
                    *const lzma_filter_info,
                ) -> lzma_ret,
        ),
        memusage: Some(lzma_delta_coder_memusage as unsafe extern "C" fn(*const c_void) -> u64),
        props_decode: Some(
            lzma_delta_props_decode
                as unsafe extern "C" fn(
                    *mut *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    size_t,
                ) -> lzma_ret,
        ),
    },
];
extern "C" fn decoder_find(id: lzma_vli) -> *const lzma_filter_decoder {
    let mut i: size_t = 0;
    while i
        < (core::mem::size_of::<[lzma_filter_decoder; 12]>() as usize)
            .wrapping_div(core::mem::size_of::<lzma_filter_decoder>() as usize)
    {
        if decoders[i as usize].id == id {
            return decoders.as_ptr().wrapping_add(i as usize);
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null::<lzma_filter_decoder>();
}
extern "C" fn coder_find(id: lzma_vli) -> *const lzma_filter_coder {
    return decoder_find(id) as *const lzma_filter_coder;
}
#[no_mangle]
pub extern "C" fn lzma_filter_decoder_is_supported(id: lzma_vli) -> lzma_bool {
    return !decoder_find(id).is_null() as lzma_bool;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    options: *const lzma_filter,
) -> lzma_ret {
    return lzma_raw_coder_init(
        next,
        allocator,
        options,
        Some(coder_find as unsafe extern "C" fn(lzma_vli) -> *const lzma_filter_coder),
        false,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_decoder(
    strm: *mut lzma_stream,
    options: *const lzma_filter,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = lzma_raw_decoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        options,
    );
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_decoder_memusage(filters: *const lzma_filter) -> u64 {
    return lzma_raw_coder_memusage(
        Some(coder_find as unsafe extern "C" fn(lzma_vli) -> *const lzma_filter_coder),
        filters,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_properties_decode(
    filter: *mut lzma_filter,
    allocator: *const lzma_allocator,
    props: *const u8,
    props_size: size_t,
) -> lzma_ret {
    (*filter).options = core::ptr::null_mut();
    let fd: *const lzma_filter_decoder = decoder_find((*filter).id) as *const lzma_filter_decoder;
    if fd.is_null() {
        return LZMA_OPTIONS_ERROR;
    }
    if (*fd).props_decode.is_none() {
        return if props_size == 0 {
            LZMA_OK
        } else {
            LZMA_OPTIONS_ERROR
        };
    }
    return (*fd).props_decode.expect("non-null function pointer")(
        &raw mut (*filter).options,
        allocator,
        props,
        props_size,
    );
}
