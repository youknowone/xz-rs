use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
extern "C" {
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_stream_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        memlimit: u64,
        flags: u32,
    ) -> lzma_ret;
}
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
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = 18446744073709551615;
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_TELL_ANY_CHECK: c_uint = 0x4;
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_buffer_decode(
    mut memlimit: *mut u64,
    mut flags: u32,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    if in_pos.is_null()
        || in_0.is_null() && *in_pos != in_size
        || *in_pos > in_size
        || out_pos.is_null()
        || out.is_null() && *out_pos != out_size
        || *out_pos > out_size
    {
        return LZMA_PROG_ERROR;
    }
    if flags & LZMA_TELL_ANY_CHECK as u32 != 0 {
        return LZMA_PROG_ERROR;
    }
    let mut stream_decoder: lzma_next_coder = lzma_next_coder_s {
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
    let mut ret: lzma_ret =
        lzma_stream_decoder_init(&raw mut stream_decoder, allocator, *memlimit, flags);
    if ret == LZMA_OK {
        let in_start: size_t = *in_pos;
        let out_start: size_t = *out_pos;
        ret = stream_decoder.code.expect("non-null function pointer")(
            stream_decoder.coder,
            allocator,
            in_0,
            in_pos,
            in_size,
            out,
            out_pos,
            out_size,
            LZMA_FINISH,
        );
        if ret == LZMA_STREAM_END {
            ret = LZMA_OK;
        } else {
            *in_pos = in_start;
            *out_pos = out_start;
            if ret == LZMA_OK {
                if *in_pos == in_size {
                    ret = LZMA_DATA_ERROR;
                } else {
                    ret = LZMA_BUF_ERROR;
                }
            } else if ret == LZMA_MEMLIMIT_ERROR {
                let mut memusage: u64 = 0;
                stream_decoder.memconfig.expect("non-null function pointer")(
                    stream_decoder.coder,
                    memlimit,
                    &raw mut memusage,
                    0 as u64,
                );
            }
        }
    }
    lzma_next_end(&raw mut stream_decoder, allocator);
    return ret;
}
