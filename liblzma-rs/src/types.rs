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

// lzma_internal sequence enum (C2RustUnnamed in lzma_internal_s)
pub type lzma_internal_sequence = c_uint;
pub const ISEQ_RUN: lzma_internal_sequence = 0;
pub const ISEQ_SYNC_FLUSH: lzma_internal_sequence = 1;
pub const ISEQ_FULL_FLUSH: lzma_internal_sequence = 2;
pub const ISEQ_FINISH: lzma_internal_sequence = 3;
pub const ISEQ_FULL_BARRIER: lzma_internal_sequence = 4;
pub const ISEQ_END: lzma_internal_sequence = 5;
pub const ISEQ_ERROR: lzma_internal_sequence = 6;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_internal_s {
    pub next: lzma_next_coder,
    pub sequence: lzma_internal_sequence,
    pub avail_in: size_t,
    pub supported_actions: [bool; 5],
    pub allow_buf_error: bool,
}
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
pub const LZMA_STREAM_HEADER_SIZE: u32 = 12;
pub const LZMA_BLOCK_HEADER_SIZE_MAX: u32 = 1024;
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
pub const LZMA_VLI_BYTES_MAX: u32 = 9;

// Filter chain limit
pub const LZMA_FILTERS_MAX: u32 = 4;

// LZMA option limits
pub const LZMA_LCLP_MAX: u32 = 4;
pub const LZMA_PB_MAX: u32 = 4;

// Delta filter
pub const LZMA_DELTA_DIST_MAX: u32 = 256;

// Stream flags
pub const LZMA_BACKWARD_SIZE_MIN: u32 = 4;
pub const LZMA_BACKWARD_SIZE_MAX: u64 = 1 << 34;

// Decoder supported flags
pub const LZMA_SUPPORTED_FLAGS: c_uint = LZMA_TELL_NO_CHECK
    | LZMA_TELL_UNSUPPORTED_CHECK
    | LZMA_TELL_ANY_CHECK
    | LZMA_IGNORE_CHECK
    | LZMA_CONCATENATED
    | LZMA_FAIL_FAST;

// lzma_block struct (shared across 12 modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_block {
    pub version: u32,
    pub header_size: u32,
    pub check: lzma_check,
    pub compressed_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub filters: *mut lzma_filter,
    pub raw_check: [u8; 64],
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
    pub reserved_ptr3: *mut c_void,
    pub reserved_int1: u32,
    pub reserved_int2: u32,
    pub reserved_int3: lzma_vli,
    pub reserved_int4: lzma_vli,
    pub reserved_int5: lzma_vli,
    pub reserved_int6: lzma_vli,
    pub reserved_int7: lzma_vli,
    pub reserved_int8: lzma_vli,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub ignore_check: lzma_bool,
    pub reserved_bool2: lzma_bool,
    pub reserved_bool3: lzma_bool,
    pub reserved_bool4: lzma_bool,
    pub reserved_bool5: lzma_bool,
    pub reserved_bool6: lzma_bool,
    pub reserved_bool7: lzma_bool,
    pub reserved_bool8: lzma_bool,
}

// lzma_stream_flags struct (shared across 11 modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream_flags {
    pub version: u32,
    pub backward_size: lzma_vli,
    pub check: lzma_check,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_bool1: lzma_bool,
    pub reserved_bool2: lzma_bool,
    pub reserved_bool3: lzma_bool,
    pub reserved_bool4: lzma_bool,
    pub reserved_bool5: lzma_bool,
    pub reserved_bool6: lzma_bool,
    pub reserved_bool7: lzma_bool,
    pub reserved_bool8: lzma_bool,
    pub reserved_int1: u32,
    pub reserved_int2: u32,
}

// lzma_sha256_state struct (shared across check modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_sha256_state {
    pub state: [u32; 8],
    pub size: u64,
}

// lzma_check_state unions and struct (shared across check/block modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub union lzma_check_state_buffer {
    pub u8_0: [u8; 64],
    pub u32_0: [u32; 16],
    pub u64_0: [u64; 8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union lzma_check_state_inner {
    pub crc32: u32,
    pub crc64: u64,
    pub sha256: lzma_sha256_state,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_check_state {
    pub buffer: lzma_check_state_buffer,
    pub state: lzma_check_state_inner,
}

// lzma_options_delta struct (shared across delta modules)
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

// lzma_options_easy struct (shared across easy modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_easy {
    pub filters: [lzma_filter; 5],
    pub opt_lzma: lzma_options_lzma,
}

// lzma_dict struct (shared across decoder modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_dict {
    pub buf: *mut u8,
    pub pos: size_t,
    pub full: size_t,
    pub limit: size_t,
    pub size: size_t,
    pub has_wrapped: bool,
    pub need_reset: bool,
}

// lzma_lz_decoder struct (shared across decoder modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_decoder {
    pub coder: *mut c_void,
    pub code: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut lzma_dict,
            *const u8,
            *mut size_t,
            size_t,
        ) -> lzma_ret,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut c_void, *const c_void) -> ()>,
    pub set_uncompressed: Option<unsafe extern "C" fn(*mut c_void, lzma_vli, bool) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>,
}

// lzma_match struct (shared across encoder modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_match {
    pub len: u32,
    pub dist: u32,
}

// lzma_mf_s struct (shared across encoder modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_mf_s {
    pub buffer: *mut u8,
    pub size: u32,
    pub keep_size_before: u32,
    pub keep_size_after: u32,
    pub offset: u32,
    pub read_pos: u32,
    pub read_ahead: u32,
    pub read_limit: u32,
    pub write_pos: u32,
    pub pending: u32,
    pub find: Option<unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> u32>,
    pub skip: Option<unsafe extern "C" fn(*mut lzma_mf, u32) -> ()>,
    pub hash: *mut u32,
    pub son: *mut u32,
    pub cyclic_pos: u32,
    pub cyclic_size: u32,
    pub hash_mask: u32,
    pub depth: u32,
    pub nice_len: u32,
    pub match_len_max: u32,
    pub action: lzma_action,
    pub hash_count: u32,
    pub sons_count: u32,
}
pub type lzma_mf = lzma_mf_s;

// lzma_delta_coder struct (shared across delta modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_delta_coder {
    pub next: lzma_next_coder,
    pub distance: size_t,
    pub pos: u8,
    pub history: [u8; LZMA_DELTA_DIST_MAX as usize],
}

// lzma_lz_encoder struct (shared across encoder modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_encoder {
    pub coder: *mut c_void,
    pub code: Option<
        unsafe extern "C" fn(*mut c_void, *mut lzma_mf, *mut u8, *mut size_t, size_t) -> lzma_ret,
    >,
    pub end: Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>,
    pub options_update: Option<unsafe extern "C" fn(*mut c_void, *const lzma_filter) -> lzma_ret>,
    pub set_out_limit: Option<unsafe extern "C" fn(*mut c_void, *mut u64, u64) -> lzma_ret>,
}

// lzma_outbuf_s struct (shared across mt modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_outbuf_s {
    pub next: *mut lzma_outbuf,
    pub worker: *mut c_void,
    pub allocated: size_t,
    pub pos: size_t,
    pub decoder_in_pos: size_t,
    pub finished: bool,
    pub finish_ret: lzma_ret,
    pub unpadded_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub buf: [u8; 0],
}
pub type lzma_outbuf = lzma_outbuf_s;

// lzma_outq struct (shared across mt modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_outq {
    pub head: *mut lzma_outbuf,
    pub tail: *mut lzma_outbuf,
    pub read_pos: size_t,
    pub cache: *mut lzma_outbuf,
    pub mem_allocated: u64,
    pub mem_in_use: u64,
    pub bufs_in_use: u32,
    pub bufs_allocated: u32,
    pub bufs_limit: u32,
}

// lzma_options_bcj struct (shared across simple/filter modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_bcj {
    pub start_offset: u32,
}

// lzma_mt struct (shared across mt modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_mt {
    pub flags: u32,
    pub threads: u32,
    pub block_size: u64,
    pub timeout: u32,
    pub preset: u32,
    pub filters: *const lzma_filter,
    pub check: lzma_check,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_int1: u32,
    pub reserved_int2: u32,
    pub reserved_int3: u32,
    pub reserved_int4: u32,
    pub memlimit_threading: u64,
    pub memlimit_stop: u64,
    pub reserved_int7: u64,
    pub reserved_int8: u64,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
    pub reserved_ptr3: *mut c_void,
    pub reserved_ptr4: *mut c_void,
}

// lzma_filter_coder struct (shared across filter modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_coder {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub memusage: Option<unsafe extern "C" fn(*const c_void) -> u64>,
}
pub type lzma_filter_find = Option<unsafe extern "C" fn(lzma_vli) -> *const lzma_filter_coder>;

// lzma_simple_coder struct (shared across simple modules)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_simple_coder {
    pub next: lzma_next_coder,
    pub end_was_reached: bool,
    pub is_encoder: bool,
    pub filter: Option<unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t>,
    pub simple: *mut c_void,
    pub now_pos: u32,
    pub allocated: size_t,
    pub pos: size_t,
    pub filtered: size_t,
    pub size: size_t,
    pub buffer: [u8; 0],
}

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
