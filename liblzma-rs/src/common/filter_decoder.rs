use crate::types::*;
use core::ffi::{c_int, c_uchar, c_uint, c_ulonglong, c_void};
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_internal_s {
    pub next: lzma_next_coder,
    pub sequence: C2RustUnnamed,
    pub avail_in: size_t,
    pub supported_actions: [bool; 5],
    pub allow_buf_error: bool,
}
pub type C2RustUnnamed = c_uint;
pub const ISEQ_ERROR: C2RustUnnamed = 6;
pub const ISEQ_END: C2RustUnnamed = 5;
pub const ISEQ_FULL_BARRIER: C2RustUnnamed = 4;
pub const ISEQ_FINISH: C2RustUnnamed = 3;
pub const ISEQ_FULL_FLUSH: C2RustUnnamed = 2;
pub const ISEQ_SYNC_FLUSH: C2RustUnnamed = 1;
pub const ISEQ_RUN: C2RustUnnamed = 0;
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
pub type lzma_internal = lzma_internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream {
    pub next_in: *const u8,
    pub avail_in: size_t,
    pub total_in: u64,
    pub next_out: *mut u8,
    pub avail_out: size_t,
    pub total_out: u64,
    pub allocator: *const lzma_allocator,
    pub internal: *mut lzma_internal,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
    pub reserved_ptr3: *mut c_void,
    pub reserved_ptr4: *mut c_void,
    pub seek_pos: u64,
    pub reserved_int2: u64,
    pub reserved_int3: size_t,
    pub reserved_int4: size_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
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
pub struct lzma_filter_info_s {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub options: *mut c_void,
}
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
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
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
static mut decoders: [lzma_filter_decoder; 12] = [
    lzma_filter_decoder {
        id: LZMA_FILTER_LZMA1 as lzma_vli,
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
        id: LZMA_FILTER_LZMA1EXT as lzma_vli,
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
        id: LZMA_FILTER_LZMA2 as lzma_vli,
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
        id: LZMA_FILTER_X86 as lzma_vli,
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
        id: LZMA_FILTER_POWERPC as lzma_vli,
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
        id: LZMA_FILTER_IA64 as lzma_vli,
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
        id: LZMA_FILTER_ARM as lzma_vli,
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
        id: LZMA_FILTER_ARMTHUMB as lzma_vli,
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
        id: LZMA_FILTER_ARM64 as lzma_vli,
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
        id: LZMA_FILTER_SPARC as lzma_vli,
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
        id: LZMA_FILTER_RISCV as lzma_vli,
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
        id: LZMA_FILTER_DELTA as lzma_vli,
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
unsafe extern "C" fn decoder_find(mut id: lzma_vli) -> *const lzma_filter_decoder {
    let mut i: size_t = 0 as size_t;
    while i
        < (::core::mem::size_of::<[lzma_filter_decoder; 12]>() as usize)
            .wrapping_div(::core::mem::size_of::<lzma_filter_decoder>() as usize)
    {
        if decoders[i as usize].id == id {
            return (&raw const decoders as *const lzma_filter_decoder).offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null::<lzma_filter_decoder>();
}
unsafe extern "C" fn coder_find(mut id: lzma_vli) -> *const lzma_filter_coder {
    return decoder_find(id) as *const lzma_filter_coder;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_filter_decoder_is_supported(mut id: lzma_vli) -> lzma_bool {
    return (decoder_find(id) != NULL as *const lzma_filter_decoder) as c_int as lzma_bool;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut options: *const lzma_filter,
) -> lzma_ret {
    return lzma_raw_coder_init(
        next,
        allocator,
        options,
        Some(coder_find as unsafe extern "C" fn(lzma_vli) -> *const lzma_filter_coder),
        false_0 != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_decoder(
    mut strm: *mut lzma_stream,
    mut options: *const lzma_filter,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = lzma_raw_decoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        options,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true_0 != 0;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true_0 != 0;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_raw_decoder_memusage(mut filters: *const lzma_filter) -> u64 {
    return lzma_raw_coder_memusage(
        Some(coder_find as unsafe extern "C" fn(lzma_vli) -> *const lzma_filter_coder),
        filters,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_properties_decode(
    mut filter: *mut lzma_filter,
    mut allocator: *const lzma_allocator,
    mut props: *const u8,
    mut props_size: size_t,
) -> lzma_ret {
    (*filter).options = NULL;
    let fd: *const lzma_filter_decoder = decoder_find((*filter).id) as *const lzma_filter_decoder;
    if fd.is_null() {
        return LZMA_OPTIONS_ERROR;
    }
    if (*fd).props_decode.is_none() {
        return (if props_size == 0 as size_t {
            LZMA_OK as c_int
        } else {
            LZMA_OPTIONS_ERROR as c_int
        }) as lzma_ret;
    }
    return (*fd).props_decode.expect("non-null function pointer")(
        &raw mut (*filter).options,
        allocator,
        props,
        props_size,
    );
}
