use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};

// Platform-dependent type aliases
pub type size_t = libc::size_t;
pub type uintptr_t = libc::uintptr_t;

// lzma type aliases
pub type lzma_bool = c_uchar;
pub type lzma_ret = c_uint;
pub type lzma_action = c_uint;
pub type lzma_check = c_uint;
pub type lzma_vli = u64;
pub type lzma_reserved_enum = c_uint;
pub type lzma_mode = c_uint;
pub type lzma_match_finder = c_uint;
pub type lzma_lzma_state = c_uint;
pub type lzma_delta_type = c_uint;
pub type probability = u16;

// Common struct shared across all modules
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, size_t, size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    pub opaque: *mut c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut c_void,
}

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

pub type lzma_init_function = Option<
    unsafe extern "C" fn(
        *mut lzma_next_coder,
        *const lzma_allocator,
        *const lzma_filter_info,
    ) -> lzma_ret,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_info_s {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub options: *mut c_void,
}
pub type lzma_filter_info = lzma_filter_info_s;

// lzma_ret constants
pub const LZMA_OK: lzma_ret = 0;
pub const LZMA_STREAM_END: lzma_ret = 1;
pub const LZMA_NO_CHECK: lzma_ret = 2;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret = 3;
pub const LZMA_GET_CHECK: lzma_ret = 4;
pub const LZMA_MEM_ERROR: lzma_ret = 5;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret = 6;
pub const LZMA_FORMAT_ERROR: lzma_ret = 7;
pub const LZMA_OPTIONS_ERROR: lzma_ret = 8;
pub const LZMA_DATA_ERROR: lzma_ret = 9;
pub const LZMA_BUF_ERROR: lzma_ret = 10;
pub const LZMA_PROG_ERROR: lzma_ret = 11;
pub const LZMA_SEEK_NEEDED: lzma_ret = 12;
pub const LZMA_RET_INTERNAL1: lzma_ret = 101;
pub const LZMA_RET_INTERNAL2: lzma_ret = 102;
pub const LZMA_RET_INTERNAL3: lzma_ret = 103;
pub const LZMA_RET_INTERNAL4: lzma_ret = 104;
pub const LZMA_RET_INTERNAL5: lzma_ret = 105;
pub const LZMA_RET_INTERNAL6: lzma_ret = 106;
pub const LZMA_RET_INTERNAL7: lzma_ret = 107;
pub const LZMA_RET_INTERNAL8: lzma_ret = 108;

// lzma_action constants
pub const LZMA_RUN: lzma_action = 0;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;

// lzma_check constants
pub const LZMA_CHECK_NONE: lzma_check = 0;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_SHA256: lzma_check = 10;

// lzma_reserved_enum constant
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;

// Integer max constants
pub const UINT32_MAX: c_uint = u32::MAX;
pub const UINT64_MAX: u64 = u64::MAX;

// VLI constants
pub const LZMA_VLI_MAX: lzma_vli = u64::MAX / 2;
pub const LZMA_VLI_UNKNOWN: lzma_vli = u64::MAX;

// lzma_match_finder constants
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT4: lzma_match_finder = 20;

// lzma_mode constants
pub const LZMA_MODE_FAST: lzma_mode = 1;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;

// lzma_check additional constants
pub const LZMA_CHECK_ID_MAX: lzma_check = 15;

// lzma_delta_type constants
pub const LZMA_DELTA_TYPE_BYTE: lzma_delta_type = 0;

// LZMA filter IDs
pub const LZMA_FILTER_DELTA: lzma_vli = 0x3;
pub const LZMA_FILTER_X86: lzma_vli = 0x4;
pub const LZMA_FILTER_POWERPC: lzma_vli = 0x5;
pub const LZMA_FILTER_IA64: lzma_vli = 0x6;
pub const LZMA_FILTER_ARM: lzma_vli = 0x7;
pub const LZMA_FILTER_ARMTHUMB: lzma_vli = 0x8;
pub const LZMA_FILTER_SPARC: lzma_vli = 0x9;
pub const LZMA_FILTER_ARM64: lzma_vli = 0xa;
pub const LZMA_FILTER_RISCV: lzma_vli = 0xb;
pub const LZMA_FILTER_LZMA1: lzma_vli = 0x4000000000000001;
pub const LZMA_FILTER_LZMA1EXT: lzma_vli = 0x4000000000000002;
pub const LZMA_FILTER_LZMA2: lzma_vli = 0x21;
pub const LZMA_FILTER_RESERVED_START: lzma_vli = 1 << 62;

// Decoder flags
pub const LZMA_TELL_NO_CHECK: c_uint = 0x1;
pub const LZMA_TELL_UNSUPPORTED_CHECK: c_uint = 0x2;
pub const LZMA_TELL_ANY_CHECK: c_uint = 0x4;
pub const LZMA_CONCATENATED: c_uint = 0x8;
pub const LZMA_IGNORE_CHECK: c_uint = 0x10;
pub const LZMA_FAIL_FAST: c_uint = 0x20;

// Stream/block size constants
pub const LZMA_STREAM_HEADER_SIZE: c_int = 12;
pub const LZMA_BLOCK_HEADER_SIZE_MAX: c_int = 1024;
pub const LZMA_MEMUSAGE_BASE: u64 = 1 << 15;
pub const LZMA_DICT_SIZE_MIN: c_uint = 4096;

// lzma_lzma_state constants
pub const STATE_LIT_LIT: lzma_lzma_state = 0;
pub const STATE_MATCH_LIT_LIT: lzma_lzma_state = 1;
pub const STATE_REP_LIT_LIT: lzma_lzma_state = 2;
pub const STATE_SHORTREP_LIT_LIT: lzma_lzma_state = 3;
pub const STATE_MATCH_LIT: lzma_lzma_state = 4;
pub const STATE_REP_LIT: lzma_lzma_state = 5;
pub const STATE_SHORTREP_LIT: lzma_lzma_state = 6;
pub const STATE_LIT_MATCH: lzma_lzma_state = 7;
pub const STATE_LIT_LONGREP: lzma_lzma_state = 8;
pub const STATE_LIT_SHORTREP: lzma_lzma_state = 9;
pub const STATE_NONLIT_MATCH: lzma_lzma_state = 10;
pub const STATE_NONLIT_REP: lzma_lzma_state = 11;

// VLI encoding constants
pub const LZMA_VLI_BYTES_MAX: c_int = 9;

// Filter chain limit
pub const LZMA_FILTERS_MAX: c_int = 4;

// LZMA option limits
pub const LZMA_LCLP_MAX: c_int = 4;
pub const LZMA_PB_MAX: c_int = 4;

// Delta filter
pub const LZMA_DELTA_DIST_MAX: c_int = 256;

// Stream flags
pub const LZMA_BACKWARD_SIZE_MIN: c_int = 4;
pub const LZMA_BACKWARD_SIZE_MAX: u64 = 1 << 34;

// Decoder supported flags
pub const LZMA_SUPPORTED_FLAGS: c_uint = LZMA_TELL_NO_CHECK
    | LZMA_TELL_UNSUPPORTED_CHECK
    | LZMA_TELL_ANY_CHECK
    | LZMA_IGNORE_CHECK
    | LZMA_CONCATENATED
    | LZMA_FAIL_FAST;

// Common extern functions used across many modules
extern "C" {
    pub fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    pub fn lzma_alloc_zero(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    pub fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    pub fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memmove(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    pub fn memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn strlen(s: *const c_char) -> size_t;
}
