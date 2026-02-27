use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
extern "C" {
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret;
    fn lzma_next_filter_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_lzma_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_lzma_lclppb_encode(options: *const lzma_options_lzma, byte: *mut u8) -> bool;
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
pub type lzma_filter_info = lzma_filter_info_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_microlzma_coder {
    pub lzma: lzma_next_coder,
    pub props: u8,
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = 18446744073709551615;
pub const true_0: c_int = 1 as c_int;
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_FILTER_LZMA1: c_ulonglong = 0x4000000000000001;
unsafe extern "C" fn microlzma_encode(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut action: lzma_action,
) -> lzma_ret {
    let mut coder: *mut lzma_microlzma_coder = coder_ptr as *mut lzma_microlzma_coder;
    let out_start: size_t = *out_pos;
    let in_start: size_t = *in_pos;
    let mut uncomp_size: u64 = 0;
    if (*coder)
        .lzma
        .set_out_limit
        .expect("non-null function pointer")(
        (*coder).lzma.coder,
        &raw mut uncomp_size,
        out_size.wrapping_sub(*out_pos) as u64,
    ) != LZMA_OK
    {
        return LZMA_PROG_ERROR;
    }
    let ret: lzma_ret = (*coder).lzma.code.expect("non-null function pointer")(
        (*coder).lzma.coder,
        allocator,
        in_0,
        in_pos,
        in_size,
        out,
        out_pos,
        out_size,
        action,
    ) as lzma_ret;
    if ret != LZMA_STREAM_END {
        if ret == LZMA_OK {
            return LZMA_PROG_ERROR;
        }
        return ret;
    }
    *out.offset(out_start as isize) = !((*coder).props as c_int) as u8;
    *in_pos = in_start.wrapping_add(uncomp_size as size_t);
    return ret;
}
unsafe extern "C" fn microlzma_encoder_end(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_microlzma_coder = coder_ptr as *mut lzma_microlzma_coder;
    lzma_next_end(&raw mut (*coder).lzma, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn microlzma_encoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut options: *const lzma_options_lzma,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_options_lzma,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        microlzma_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_options_lzma,
            ) -> lzma_ret,
    )) != (*next).init
    {
        lzma_next_end(next, allocator);
    }
    (*next).init = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_options_lzma,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        microlzma_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_options_lzma,
            ) -> lzma_ret,
    ));
    let mut coder: *mut lzma_microlzma_coder = (*next).coder as *mut lzma_microlzma_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            ::core::mem::size_of::<lzma_microlzma_coder>() as size_t,
            allocator,
        ) as *mut lzma_microlzma_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            microlzma_encode
                as unsafe extern "C" fn(
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
        ) as lzma_code_function;
        (*next).end = Some(
            microlzma_encoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*coder).lzma = lzma_next_coder_s {
            coder: NULL,
            id: LZMA_VLI_UNKNOWN as lzma_vli,
            init: ::core::ptr::null_mut::<c_void>() as uintptr_t,
            code: None,
            end: None,
            get_progress: None,
            get_check: None,
            memconfig: None,
            update: None,
            set_out_limit: None,
        };
    }
    if lzma_lzma_lclppb_encode(options, &raw mut (*coder).props) {
        return LZMA_OPTIONS_ERROR;
    }
    let filters: [lzma_filter_info; 2] = [
        lzma_filter_info_s {
            id: LZMA_FILTER_LZMA1 as lzma_vli,
            init: Some(
                lzma_lzma_encoder_init
                    as unsafe extern "C" fn(
                        *mut lzma_next_coder,
                        *const lzma_allocator,
                        *const lzma_filter_info,
                    ) -> lzma_ret,
            ),
            options: options as *mut c_void,
        },
        lzma_filter_info_s {
            id: 0,
            init: None,
            options: ::core::ptr::null_mut::<c_void>(),
        },
    ];
    return lzma_next_filter_init(
        &raw mut (*coder).lzma,
        allocator,
        &raw const filters as *const lzma_filter_info,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_microlzma_encoder(
    mut strm: *mut lzma_stream,
    mut options: *const lzma_options_lzma,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = microlzma_encoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        options,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true_0 != 0;
    return LZMA_OK;
}
