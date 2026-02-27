use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
extern "C" {
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_stream_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        memlimit: u64,
        flags: u32,
    ) -> lzma_ret;
    fn lzma_alone_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        memlimit: u64,
        picky: bool,
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
pub type C2RustUnnamed_0 = c_uint;
pub const SEQ_FINISH: C2RustUnnamed_0 = 2;
pub const SEQ_CODE: C2RustUnnamed_0 = 1;
pub const SEQ_INIT: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_auto_coder {
    pub next: lzma_next_coder,
    pub memlimit: u64,
    pub flags: u32,
    pub sequence: C2RustUnnamed_0,
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = 18446744073709551615;
pub const true_0: c_int = 1 as c_int;
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_TELL_NO_CHECK: c_uint = 0x1;
pub const LZMA_TELL_UNSUPPORTED_CHECK: c_uint = 0x2;
pub const LZMA_TELL_ANY_CHECK: c_uint = 0x4;
pub const LZMA_IGNORE_CHECK: c_uint = 0x10;
pub const LZMA_CONCATENATED: c_uint = 0x8;
pub const LZMA_FAIL_FAST: c_uint = 0x20;
pub const LZMA_MEMUSAGE_BASE: c_ulonglong = 1 << 15;
pub const LZMA_SUPPORTED_FLAGS: c_uint = LZMA_TELL_NO_CHECK
    | LZMA_TELL_UNSUPPORTED_CHECK
    | LZMA_TELL_ANY_CHECK
    | LZMA_IGNORE_CHECK
    | LZMA_CONCATENATED
    | LZMA_FAIL_FAST;
unsafe extern "C" fn auto_decode(
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
    let mut coder: *mut lzma_auto_coder = coder_ptr as *mut lzma_auto_coder;
    let mut current_block_28: u64;
    match (*coder).sequence {
        0 => {
            if *in_pos >= in_size {
                return LZMA_OK;
            }
            (*coder).sequence = SEQ_CODE;
            if *in_0.offset(*in_pos as isize) as c_int == 0xfd as c_int {
                let ret_: lzma_ret = lzma_stream_decoder_init(
                    &raw mut (*coder).next,
                    allocator,
                    (*coder).memlimit,
                    (*coder).flags,
                ) as lzma_ret;
                if ret_ != LZMA_OK {
                    return ret_;
                }
            } else {
                let ret__0: lzma_ret = lzma_alone_decoder_init(
                    &raw mut (*coder).next,
                    allocator,
                    (*coder).memlimit,
                    1 as c_int != 0,
                ) as lzma_ret;
                if ret__0 != LZMA_OK {
                    return ret__0;
                }
                if (*coder).flags & LZMA_TELL_NO_CHECK as u32 != 0 {
                    return LZMA_NO_CHECK;
                }
                if (*coder).flags & LZMA_TELL_ANY_CHECK as u32 != 0 {
                    return LZMA_GET_CHECK;
                }
            }
            current_block_28 = 13935781298497728377;
        }
        1 => {
            current_block_28 = 13935781298497728377;
        }
        2 => {
            current_block_28 = 4647193646042868866;
        }
        _ => return LZMA_PROG_ERROR,
    }
    match current_block_28 {
        13935781298497728377 => {
            let ret: lzma_ret = (*coder).next.code.expect("non-null function pointer")(
                (*coder).next.coder,
                allocator,
                in_0,
                in_pos,
                in_size,
                out,
                out_pos,
                out_size,
                action,
            ) as lzma_ret;
            if ret != LZMA_STREAM_END || (*coder).flags & LZMA_CONCATENATED as u32 == 0 as u32 {
                return ret;
            }
            (*coder).sequence = SEQ_FINISH;
        }
        _ => {}
    }
    if *in_pos < in_size {
        return LZMA_DATA_ERROR;
    }
    return (if action == LZMA_FINISH {
        LZMA_STREAM_END as c_int
    } else {
        LZMA_OK as c_int
    }) as lzma_ret;
}
unsafe extern "C" fn auto_decoder_end(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_auto_coder = coder_ptr as *mut lzma_auto_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn auto_decoder_get_check(mut coder_ptr: *const c_void) -> lzma_check {
    let mut coder: *const lzma_auto_coder = coder_ptr as *const lzma_auto_coder;
    return (if (*coder).next.get_check.is_none() {
        LZMA_CHECK_NONE
    } else {
        (*coder).next.get_check.expect("non-null function pointer")((*coder).next.coder)
    }) as lzma_check;
}
unsafe extern "C" fn auto_decoder_memconfig(
    mut coder_ptr: *mut c_void,
    mut memusage: *mut u64,
    mut old_memlimit: *mut u64,
    mut new_memlimit: u64,
) -> lzma_ret {
    let mut coder: *mut lzma_auto_coder = coder_ptr as *mut lzma_auto_coder;
    let mut ret: lzma_ret = LZMA_OK;
    if (*coder).next.memconfig.is_some() {
        ret = (*coder).next.memconfig.expect("non-null function pointer")(
            (*coder).next.coder,
            memusage,
            old_memlimit,
            new_memlimit,
        );
    } else {
        *memusage = LZMA_MEMUSAGE_BASE as u64;
        *old_memlimit = (*coder).memlimit;
        ret = LZMA_OK;
        if new_memlimit != 0 as u64 && new_memlimit < *memusage {
            ret = LZMA_MEMLIMIT_ERROR;
        }
    }
    if ret == LZMA_OK && new_memlimit != 0 as u64 {
        (*coder).memlimit = new_memlimit;
    }
    return ret;
}
unsafe extern "C" fn auto_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut memlimit: u64,
    mut flags: u32,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(*mut lzma_next_coder, *const lzma_allocator, u64, u32) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        auto_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                u32,
            ) -> lzma_ret,
    )) != (*next).init
    {
        lzma_next_end(next, allocator);
    }
    (*next).init = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(*mut lzma_next_coder, *const lzma_allocator, u64, u32) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        auto_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                u32,
            ) -> lzma_ret,
    ));
    if flags & !(LZMA_SUPPORTED_FLAGS as u32) != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut coder: *mut lzma_auto_coder = (*next).coder as *mut lzma_auto_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            ::core::mem::size_of::<lzma_auto_coder>() as size_t,
            allocator,
        ) as *mut lzma_auto_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            auto_decode
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
            auto_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).get_check =
            Some(auto_decoder_get_check as unsafe extern "C" fn(*const c_void) -> lzma_check)
                as Option<unsafe extern "C" fn(*const c_void) -> lzma_check>;
        (*next).memconfig = Some(
            auto_decoder_memconfig
                as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret,
        )
            as Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret>;
        (*coder).next = lzma_next_coder_s {
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
    (*coder).memlimit = if 1 as u64 > memlimit {
        1 as u64
    } else {
        memlimit
    };
    (*coder).flags = flags;
    (*coder).sequence = SEQ_INIT;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_auto_decoder(
    mut strm: *mut lzma_stream,
    mut memlimit: u64,
    mut flags: u32,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = auto_decoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        memlimit,
        flags,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true_0 != 0;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true_0 != 0;
    return LZMA_OK;
}
