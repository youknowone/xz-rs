use crate::types::*;
use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulonglong, c_void};
pub enum lzma_index_s {}
extern "C" {
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn memset(__b: *mut c_void, __c: c_int, __len: size_t) -> *mut c_void;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> c_int;
    fn pthread_cond_destroy(_: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_init(_: *mut pthread_cond_t, _: *const pthread_condattr_t) -> c_int;
    fn pthread_cond_signal(_: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_timedwait(
        _: *mut pthread_cond_t,
        _: *mut pthread_mutex_t,
        _: *const timespec,
    ) -> c_int;
    fn pthread_cond_wait(_: *mut pthread_cond_t, _: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(
        _: *mut pthread_t,
        _: *const pthread_attr_t,
        _: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        _: *mut c_void,
    ) -> c_int;
    fn pthread_join(_: pthread_t, _: *mut *mut c_void) -> c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_init(_: *mut pthread_mutex_t, _: *const pthread_mutexattr_t) -> c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> c_int;
    fn pthread_sigmask(_: c_int, _: *const sigset_t, _: *mut sigset_t) -> c_int;
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_check_is_supported(check: lzma_check) -> lzma_bool;
    fn lzma_filters_copy(
        src: *const lzma_filter,
        dest: *mut lzma_filter,
        allocator: *const lzma_allocator,
    ) -> lzma_ret;
    fn lzma_filters_free(filters: *mut lzma_filter, allocator: *const lzma_allocator);
    fn lzma_raw_encoder_memusage(filters: *const lzma_filter) -> u64;
    fn lzma_mt_block_size(filters: *const lzma_filter) -> u64;
    fn lzma_stream_header_encode(options: *const lzma_stream_flags, out: *mut u8) -> lzma_ret;
    fn lzma_stream_footer_encode(options: *const lzma_stream_flags, out: *mut u8) -> lzma_ret;
    fn lzma_block_header_size(block: *mut lzma_block) -> lzma_ret;
    fn lzma_block_header_encode(block: *const lzma_block, out: *mut u8) -> lzma_ret;
    fn lzma_block_unpadded_size(block: *const lzma_block) -> lzma_vli;
    fn lzma_block_uncomp_encode(
        block: *mut lzma_block,
        in_0: *const u8,
        in_size: size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> lzma_ret;
    fn lzma_index_init(allocator: *const lzma_allocator) -> *mut lzma_index;
    fn lzma_index_end(i: *mut lzma_index, allocator: *const lzma_allocator);
    fn lzma_index_append(
        i: *mut lzma_index,
        allocator: *const lzma_allocator,
        unpadded_size: lzma_vli,
        uncompressed_size: lzma_vli,
    ) -> lzma_ret;
    fn lzma_index_size(i: *const lzma_index) -> lzma_vli;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_bufcpy(
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> size_t;
    fn lzma_easy_preset(easy: *mut lzma_options_easy, preset: u32) -> bool;
    fn lzma_block_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        block: *mut lzma_block,
    ) -> lzma_ret;
    fn lzma_block_buffer_bound64(uncompressed_size: u64) -> u64;
    fn lzma_index_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        i: *const lzma_index,
    ) -> lzma_ret;
    fn lzma_outq_memusage(buf_size_max: u64, threads: u32) -> u64;
    fn lzma_outq_init(
        outq: *mut lzma_outq,
        allocator: *const lzma_allocator,
        threads: u32,
    ) -> lzma_ret;
    fn lzma_outq_end(outq: *mut lzma_outq, allocator: *const lzma_allocator);
    fn lzma_outq_prealloc_buf(
        outq: *mut lzma_outq,
        allocator: *const lzma_allocator,
        size: size_t,
    ) -> lzma_ret;
    fn lzma_outq_get_buf(outq: *mut lzma_outq, worker: *mut c_void) -> *mut lzma_outbuf;
    fn lzma_outq_is_readable(outq: *const lzma_outq) -> bool;
    fn lzma_outq_read(
        outq: *mut lzma_outq,
        allocator: *const lzma_allocator,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
        unpadded_size: *mut lzma_vli,
        uncompressed_size: *mut lzma_vli,
    ) -> lzma_ret;
}
pub type __uint32_t = u32;
pub type __darwin_time_t = c_long;
pub type __darwin_sigset_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
    pub __arg: *mut c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_attr_t {
    pub __sig: c_long,
    pub __opaque: [c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_cond_t {
    pub __sig: c_long,
    pub __opaque: [c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: c_long,
    pub __opaque: [c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: c_long,
    pub __opaque: [c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: c_long,
    pub __opaque: [c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [c_char; 8176],
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type pthread_attr_t = __darwin_pthread_attr_t;
pub type sigset_t = __darwin_sigset_t;
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: c_long,
}
pub type clockid_t = c_uint;
pub const _CLOCK_THREAD_CPUTIME_ID: clockid_t = 16;
pub const _CLOCK_PROCESS_CPUTIME_ID: clockid_t = 12;
pub const _CLOCK_UPTIME_RAW_APPROX: clockid_t = 9;
pub const _CLOCK_UPTIME_RAW: clockid_t = 8;
pub const _CLOCK_MONOTONIC_RAW_APPROX: clockid_t = 5;
pub const _CLOCK_MONOTONIC_RAW: clockid_t = 4;
pub const _CLOCK_MONOTONIC: clockid_t = 6;
pub const _CLOCK_REALTIME: clockid_t = 0;
pub type pthread_cond_t = __darwin_pthread_cond_t;
pub type pthread_condattr_t = __darwin_pthread_condattr_t;
pub type pthread_mutex_t = __darwin_pthread_mutex_t;
pub type pthread_mutexattr_t = __darwin_pthread_mutexattr_t;
pub type pthread_t = __darwin_pthread_t;
pub type mythread = pthread_t;
pub type mythread_mutex = pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mythread_cond {
    pub cond: pthread_cond_t,
    pub clk_id: clockid_t,
}
pub type mythread_condtime = timespec;
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
pub type worker_thread = worker_thread_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker_thread_s {
    pub state: worker_state,
    pub in_0: *mut u8,
    pub in_size: size_t,
    pub outbuf: *mut lzma_outbuf,
    pub coder: *mut lzma_stream_coder,
    pub allocator: *const lzma_allocator,
    pub progress_in: u64,
    pub progress_out: u64,
    pub block_encoder: lzma_next_coder,
    pub block_options: lzma_block,
    pub filters: [lzma_filter; 5],
    pub next: *mut worker_thread,
    pub mutex: mythread_mutex,
    pub cond: mythread_cond,
    pub thread_id: mythread,
}
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
pub type lzma_stream_coder = lzma_stream_coder_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream_coder_s {
    pub sequence: C2RustUnnamed_0,
    pub block_size: size_t,
    pub filters: [lzma_filter; 5],
    pub filters_cache: [lzma_filter; 5],
    pub index: *mut lzma_index,
    pub index_encoder: lzma_next_coder,
    pub stream_flags: lzma_stream_flags,
    pub header: [u8; 12],
    pub header_pos: size_t,
    pub outq: lzma_outq,
    pub outbuf_alloc_size: size_t,
    pub timeout: u32,
    pub thread_error: lzma_ret,
    pub threads: *mut worker_thread,
    pub threads_max: u32,
    pub threads_initialized: u32,
    pub threads_free: *mut worker_thread,
    pub thr: *mut worker_thread,
    pub progress_in: u64,
    pub progress_out: u64,
    pub mutex: mythread_mutex,
    pub cond: mythread_cond,
}
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
pub type lzma_outbuf = lzma_outbuf_s;
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
pub type lzma_index = lzma_index_s;
pub type C2RustUnnamed_0 = c_uint;
pub const SEQ_STREAM_FOOTER: C2RustUnnamed_0 = 3;
pub const SEQ_INDEX: C2RustUnnamed_0 = 2;
pub const SEQ_BLOCK: C2RustUnnamed_0 = 1;
pub const SEQ_STREAM_HEADER: C2RustUnnamed_0 = 0;
pub type worker_state = c_uint;
pub const THR_EXIT: worker_state = 4;
pub const THR_STOP: worker_state = 3;
pub const THR_FINISH: worker_state = 2;
pub const THR_RUN: worker_state = 1;
pub const THR_IDLE: worker_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_easy {
    pub filters: [lzma_filter; 5],
    pub opt_lzma: lzma_options_lzma,
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = 18446744073709551615;
pub const SIG_SETMASK: c_int = 3 as c_int;
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
pub const MYTHREAD_RET_VALUE: *mut c_void = NULL;
#[inline]
unsafe extern "C" fn mythread_sigmask(
    mut how: c_int,
    mut set: *const sigset_t,
    mut oset: *mut sigset_t,
) {
    let mut ret: c_int = pthread_sigmask(how, set as *const sigset_t, oset as *mut sigset_t);
}
#[inline]
unsafe extern "C" fn mythread_create(
    mut thread: *mut mythread,
    mut func: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    mut arg: *mut c_void,
) -> c_int {
    let mut old: sigset_t = 0;
    let mut all: sigset_t = 0;
    all = !(0 as sigset_t);
    mythread_sigmask(SIG_SETMASK, &raw mut all, &raw mut old);
    let ret: c_int = pthread_create(
        thread as *mut pthread_t,
        ::core::ptr::null::<pthread_attr_t>(),
        func as Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg as *mut c_void,
    ) as c_int;
    mythread_sigmask(
        SIG_SETMASK,
        &raw mut old,
        ::core::ptr::null_mut::<sigset_t>(),
    );
    return ret;
}
#[inline]
unsafe extern "C" fn mythread_join(mut thread: mythread) -> c_int {
    return pthread_join(thread as pthread_t, ::core::ptr::null_mut::<*mut c_void>());
}
#[inline]
unsafe extern "C" fn mythread_mutex_init(mut mutex: *mut mythread_mutex) -> c_int {
    return pthread_mutex_init(
        mutex as *mut pthread_mutex_t,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    );
}
#[inline]
unsafe extern "C" fn mythread_mutex_destroy(mut mutex: *mut mythread_mutex) {
    let mut ret: c_int = pthread_mutex_destroy(mutex as *mut pthread_mutex_t);
}
#[inline]
unsafe extern "C" fn mythread_mutex_lock(mut mutex: *mut mythread_mutex) {
    let mut ret: c_int = pthread_mutex_lock(mutex as *mut pthread_mutex_t);
}
#[inline]
unsafe extern "C" fn mythread_mutex_unlock(mut mutex: *mut mythread_mutex) {
    let mut ret: c_int = pthread_mutex_unlock(mutex as *mut pthread_mutex_t);
}
#[inline]
unsafe extern "C" fn mythread_cond_init(mut mycond: *mut mythread_cond) -> c_int {
    (*mycond).clk_id = _CLOCK_REALTIME;
    return pthread_cond_init(
        &raw mut (*mycond).cond,
        ::core::ptr::null::<pthread_condattr_t>(),
    );
}
#[inline]
unsafe extern "C" fn mythread_cond_destroy(mut cond: *mut mythread_cond) {
    let mut ret: c_int = pthread_cond_destroy(&raw mut (*cond).cond);
}
#[inline]
unsafe extern "C" fn mythread_cond_signal(mut cond: *mut mythread_cond) {
    let mut ret: c_int = pthread_cond_signal(&raw mut (*cond).cond);
}
#[inline]
unsafe extern "C" fn mythread_cond_wait(
    mut cond: *mut mythread_cond,
    mut mutex: *mut mythread_mutex,
) {
    let mut ret: c_int = pthread_cond_wait(&raw mut (*cond).cond, mutex as *mut pthread_mutex_t);
}
#[inline]
unsafe extern "C" fn mythread_cond_timedwait(
    mut cond: *mut mythread_cond,
    mut mutex: *mut mythread_mutex,
    mut condtime: *const mythread_condtime,
) -> c_int {
    let mut ret: c_int = pthread_cond_timedwait(
        &raw mut (*cond).cond,
        mutex as *mut pthread_mutex_t,
        condtime as *const timespec,
    );
    return ret;
}
#[inline]
unsafe extern "C" fn mythread_condtime_set(
    mut condtime: *mut mythread_condtime,
    mut cond: *const mythread_cond,
    mut timeout_ms: u32,
) {
    (*condtime).tv_sec = timeout_ms.wrapping_div(1000 as u32) as time_t as __darwin_time_t;
    (*condtime).tv_nsec = timeout_ms
        .wrapping_rem(1000 as u32)
        .wrapping_mul(1000000 as u32) as c_long;
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ret: c_int = clock_gettime((*cond).clk_id, &raw mut now);
    (*condtime).tv_sec += now.tv_sec;
    (*condtime).tv_nsec += now.tv_nsec;
    if (*condtime).tv_nsec >= 1000000000 as c_long {
        (*condtime).tv_nsec -= 1000000000 as c_long;
        (*condtime).tv_sec += 1;
    }
}
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_CHECK_ID_MAX: lzma_check = 15;
pub const LZMA_STREAM_HEADER_SIZE: c_int = 12 as c_int;
pub const LZMA_THREADS_MAX: c_int = 16384 as c_int;
pub const LZMA_MEMUSAGE_BASE: c_ulonglong = 1 << 15;
#[inline]
unsafe extern "C" fn lzma_outq_has_buf(mut outq: *const lzma_outq) -> bool {
    return (*outq).bufs_in_use < (*outq).bufs_limit;
}
#[inline]
unsafe extern "C" fn lzma_outq_is_empty(mut outq: *const lzma_outq) -> bool {
    return (*outq).bufs_in_use == 0 as u32;
}
pub const BLOCK_SIZE_MAX: c_ulonglong = UINT64_MAX.wrapping_div(LZMA_THREADS_MAX as u64);
unsafe extern "C" fn worker_error(mut thr: *mut worker_thread, mut ret: lzma_ret) {
    let mut mythread_i_207: c_uint = 0;
    while if mythread_i_207 != 0 {
        mythread_mutex_unlock(&raw mut (*(*thr).coder).mutex);
        0 as c_int
    } else {
        mythread_mutex_lock(&raw mut (*(*thr).coder).mutex);
        1 as c_int
    } != 0
    {
        let mut mythread_j_207: c_uint = 0;
        while mythread_j_207 == 0 {
            if (*(*thr).coder).thread_error == LZMA_OK {
                (*(*thr).coder).thread_error = ret;
            }
            mythread_cond_signal(&raw mut (*(*thr).coder).cond);
            mythread_j_207 = 1;
        }
        mythread_i_207 = 1;
    }
}
unsafe extern "C" fn worker_encode(
    mut thr: *mut worker_thread,
    mut out_pos: *mut size_t,
    mut state: worker_state,
) -> worker_state {
    (*thr).block_options = lzma_block {
        version: 0 as u32,
        header_size: 0,
        check: (*(*thr).coder).stream_flags.check,
        compressed_size: (*(*thr).outbuf).allocated as lzma_vli,
        uncompressed_size: (*(*thr).coder).block_size as lzma_vli,
        filters: &raw mut (*thr).filters as *mut lzma_filter,
        raw_check: [0; 64],
        reserved_ptr1: ::core::ptr::null_mut::<c_void>(),
        reserved_ptr2: ::core::ptr::null_mut::<c_void>(),
        reserved_ptr3: ::core::ptr::null_mut::<c_void>(),
        reserved_int1: 0,
        reserved_int2: 0,
        reserved_int3: 0,
        reserved_int4: 0,
        reserved_int5: 0,
        reserved_int6: 0,
        reserved_int7: 0,
        reserved_int8: 0,
        reserved_enum1: LZMA_RESERVED_ENUM,
        reserved_enum2: LZMA_RESERVED_ENUM,
        reserved_enum3: LZMA_RESERVED_ENUM,
        reserved_enum4: LZMA_RESERVED_ENUM,
        ignore_check: 0,
        reserved_bool2: 0,
        reserved_bool3: 0,
        reserved_bool4: 0,
        reserved_bool5: 0,
        reserved_bool6: 0,
        reserved_bool7: 0,
        reserved_bool8: 0,
    };
    let mut ret: lzma_ret = lzma_block_header_size(&raw mut (*thr).block_options);
    if ret != LZMA_OK {
        worker_error(thr, ret);
        return THR_STOP;
    }
    ret = lzma_block_encoder_init(
        &raw mut (*thr).block_encoder,
        (*thr).allocator,
        &raw mut (*thr).block_options,
    );
    if ret != LZMA_OK {
        worker_error(thr, ret);
        return THR_STOP;
    }
    let mut in_pos: size_t = 0 as size_t;
    let mut in_size: size_t = 0 as size_t;
    *out_pos = (*thr).block_options.header_size as size_t;
    let out_size: size_t = (*(*thr).outbuf).allocated;
    loop {
        let mut mythread_i_258: c_uint = 0;
        while if mythread_i_258 != 0 {
            mythread_mutex_unlock(&raw mut (*thr).mutex);
            0 as c_int
        } else {
            mythread_mutex_lock(&raw mut (*thr).mutex);
            1 as c_int
        } != 0
        {
            let mut mythread_j_258: c_uint = 0;
            while mythread_j_258 == 0 {
                (*thr).progress_in = in_pos as u64;
                (*thr).progress_out = *out_pos as u64;
                while in_size == (*thr).in_size && (*thr).state == THR_RUN {
                    mythread_cond_wait(&raw mut (*thr).cond, &raw mut (*thr).mutex);
                }
                state = (*thr).state;
                in_size = (*thr).in_size;
                mythread_j_258 = 1;
            }
            mythread_i_258 = 1;
        }
        if state >= THR_STOP {
            return state;
        }
        let mut action: lzma_action = (if state == THR_FINISH {
            LZMA_FINISH as c_int
        } else {
            LZMA_RUN as c_int
        }) as lzma_action;
        static mut in_chunk_max: size_t = 16384 as size_t;
        let mut in_limit: size_t = in_size;
        if in_size.wrapping_sub(in_pos) > in_chunk_max {
            in_limit = in_pos.wrapping_add(in_chunk_max);
            action = LZMA_RUN;
        }
        ret = (*thr)
            .block_encoder
            .code
            .expect("non-null function pointer")(
            (*thr).block_encoder.coder,
            (*thr).allocator,
            (*thr).in_0,
            &raw mut in_pos,
            in_limit,
            &raw mut (*(*thr).outbuf).buf as *mut u8,
            out_pos,
            out_size,
            action,
        );
        if !(ret == LZMA_OK && *out_pos < out_size) {
            break;
        }
    }
    match ret {
        1 => {
            ret = lzma_block_header_encode(
                &raw mut (*thr).block_options,
                &raw mut (*(*thr).outbuf).buf as *mut u8,
            );
            if ret != LZMA_OK {
                worker_error(thr, ret);
                return THR_STOP;
            }
        }
        0 => {
            let mut mythread_i_321: c_uint = 0;
            while if mythread_i_321 != 0 {
                mythread_mutex_unlock(&raw mut (*thr).mutex);
                0 as c_int
            } else {
                mythread_mutex_lock(&raw mut (*thr).mutex);
                1 as c_int
            } != 0
            {
                let mut mythread_j_321: c_uint = 0;
                while mythread_j_321 == 0 {
                    while (*thr).state == THR_RUN {
                        mythread_cond_wait(&raw mut (*thr).cond, &raw mut (*thr).mutex);
                    }
                    state = (*thr).state;
                    in_size = (*thr).in_size;
                    mythread_j_321 = 1;
                }
                mythread_i_321 = 1;
            }
            if state >= THR_STOP {
                return state;
            }
            *out_pos = 0 as size_t;
            ret = lzma_block_uncomp_encode(
                &raw mut (*thr).block_options,
                (*thr).in_0,
                in_size,
                &raw mut (*(*thr).outbuf).buf as *mut u8,
                out_pos,
                out_size,
            );
            if ret != LZMA_OK {
                worker_error(thr, LZMA_PROG_ERROR);
                return THR_STOP;
            }
        }
        _ => {
            worker_error(thr, ret);
            return THR_STOP;
        }
    }
    (*(*thr).outbuf).unpadded_size = lzma_block_unpadded_size(&raw mut (*thr).block_options);
    (*(*thr).outbuf).uncompressed_size = (*thr).block_options.uncompressed_size;
    return THR_FINISH;
}
unsafe extern "C" fn worker_start(mut thr_ptr: *mut c_void) -> *mut c_void {
    let mut thr: *mut worker_thread = thr_ptr as *mut worker_thread;
    let mut state: worker_state = THR_IDLE;
    loop {
        let mut mythread_i_370: c_uint = 0;
        while if mythread_i_370 != 0 {
            mythread_mutex_unlock(&raw mut (*thr).mutex);
            0 as c_int
        } else {
            mythread_mutex_lock(&raw mut (*thr).mutex);
            1 as c_int
        } != 0
        {
            let mut mythread_j_370: c_uint = 0;
            while mythread_j_370 == 0 {
                loop {
                    if (*thr).state == THR_STOP {
                        (*thr).state = THR_IDLE;
                        mythread_cond_signal(&raw mut (*thr).cond);
                    }
                    state = (*thr).state;
                    if state != THR_IDLE {
                        break;
                    }
                    mythread_cond_wait(&raw mut (*thr).cond, &raw mut (*thr).mutex);
                }
                mythread_j_370 = 1;
            }
            mythread_i_370 = 1;
        }
        let mut out_pos: size_t = 0 as size_t;
        if state <= THR_FINISH {
            state = worker_encode(thr, &raw mut out_pos, state);
        }
        if state == THR_EXIT {
            break;
        }
        let mut mythread_i_401: c_uint = 0;
        while if mythread_i_401 != 0 {
            mythread_mutex_unlock(&raw mut (*thr).mutex);
            0 as c_int
        } else {
            mythread_mutex_lock(&raw mut (*thr).mutex);
            1 as c_int
        } != 0
        {
            let mut mythread_j_401: c_uint = 0;
            while mythread_j_401 == 0 {
                if (*thr).state != THR_EXIT {
                    (*thr).state = THR_IDLE;
                    mythread_cond_signal(&raw mut (*thr).cond);
                }
                mythread_j_401 = 1;
            }
            mythread_i_401 = 1;
        }
        let mut mythread_i_408: c_uint = 0;
        while if mythread_i_408 != 0 {
            mythread_mutex_unlock(&raw mut (*(*thr).coder).mutex);
            0 as c_int
        } else {
            mythread_mutex_lock(&raw mut (*(*thr).coder).mutex);
            1 as c_int
        } != 0
        {
            let mut mythread_j_408: c_uint = 0;
            while mythread_j_408 == 0 {
                if state == THR_FINISH {
                    (*(*thr).outbuf).pos = out_pos;
                    (*(*thr).outbuf).finished = true_0 != 0;
                }
                (*(*thr).coder).progress_in = (*(*thr).coder)
                    .progress_in
                    .wrapping_add((*(*thr).outbuf).uncompressed_size as u64);
                (*(*thr).coder).progress_out =
                    (*(*thr).coder).progress_out.wrapping_add(out_pos as u64);
                (*thr).progress_in = 0 as u64;
                (*thr).progress_out = 0 as u64;
                (*thr).next = (*(*thr).coder).threads_free;
                (*(*thr).coder).threads_free = thr;
                mythread_cond_signal(&raw mut (*(*thr).coder).cond);
                mythread_j_408 = 1;
            }
            mythread_i_408 = 1;
        }
    }
    lzma_filters_free(
        &raw mut (*thr).filters as *mut lzma_filter,
        (*thr).allocator,
    );
    mythread_mutex_destroy(&raw mut (*thr).mutex);
    mythread_cond_destroy(&raw mut (*thr).cond);
    lzma_next_end(&raw mut (*thr).block_encoder, (*thr).allocator);
    lzma_free((*thr).in_0 as *mut c_void, (*thr).allocator);
    return MYTHREAD_RET_VALUE;
}
unsafe extern "C" fn threads_stop(mut coder: *mut lzma_stream_coder, mut wait_for_threads: bool) {
    let mut i: u32 = 0 as u32;
    while i < (*coder).threads_initialized {
        let mut mythread_i_449: c_uint = 0;
        while if mythread_i_449 != 0 {
            mythread_mutex_unlock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
            0 as c_int
        } else {
            mythread_mutex_lock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
            1 as c_int
        } != 0
        {
            let mut mythread_j_449: c_uint = 0;
            while mythread_j_449 == 0 {
                (*(*coder).threads.offset(i as isize)).state = THR_STOP;
                mythread_cond_signal(&raw mut (*(*coder).threads.offset(i as isize)).cond);
                mythread_j_449 = 1;
            }
            mythread_i_449 = 1;
        }
        i = i.wrapping_add(1);
    }
    if !wait_for_threads {
        return;
    }
    let mut i_0: u32 = 0 as u32;
    while i_0 < (*coder).threads_initialized {
        let mut mythread_i_460: c_uint = 0;
        while if mythread_i_460 != 0 {
            mythread_mutex_unlock(&raw mut (*(*coder).threads.offset(i_0 as isize)).mutex);
            0 as c_int
        } else {
            mythread_mutex_lock(&raw mut (*(*coder).threads.offset(i_0 as isize)).mutex);
            1 as c_int
        } != 0
        {
            let mut mythread_j_460: c_uint = 0;
            while mythread_j_460 == 0 {
                while (*(*coder).threads.offset(i_0 as isize)).state != THR_IDLE {
                    mythread_cond_wait(
                        &raw mut (*(*coder).threads.offset(i_0 as isize)).cond,
                        &raw mut (*(*coder).threads.offset(i_0 as isize)).mutex,
                    );
                }
                mythread_j_460 = 1;
            }
            mythread_i_460 = 1;
        }
        i_0 = i_0.wrapping_add(1);
    }
}
unsafe extern "C" fn threads_end(
    mut coder: *mut lzma_stream_coder,
    mut allocator: *const lzma_allocator,
) {
    let mut i: u32 = 0 as u32;
    while i < (*coder).threads_initialized {
        let mut mythread_i_477: c_uint = 0;
        while if mythread_i_477 != 0 {
            mythread_mutex_unlock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
            0 as c_int
        } else {
            mythread_mutex_lock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
            1 as c_int
        } != 0
        {
            let mut mythread_j_477: c_uint = 0;
            while mythread_j_477 == 0 {
                (*(*coder).threads.offset(i as isize)).state = THR_EXIT;
                mythread_cond_signal(&raw mut (*(*coder).threads.offset(i as isize)).cond);
                mythread_j_477 = 1;
            }
            mythread_i_477 = 1;
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: u32 = 0 as u32;
    while i_0 < (*coder).threads_initialized {
        let mut ret: c_int = mythread_join((*(*coder).threads.offset(i_0 as isize)).thread_id);
        i_0 = i_0.wrapping_add(1);
    }
    lzma_free((*coder).threads as *mut c_void, allocator);
}
unsafe extern "C" fn initialize_new_thread(
    mut coder: *mut lzma_stream_coder,
    mut allocator: *const lzma_allocator,
) -> lzma_ret {
    let mut thr: *mut worker_thread = (*coder)
        .threads
        .offset((*coder).threads_initialized as isize)
        as *mut worker_thread;
    (*thr).in_0 = lzma_alloc((*coder).block_size, allocator) as *mut u8;
    if (*thr).in_0.is_null() {
        return LZMA_MEM_ERROR;
    }
    if !(mythread_mutex_init(&raw mut (*thr).mutex) != 0) {
        if !(mythread_cond_init(&raw mut (*thr).cond) != 0) {
            (*thr).state = THR_IDLE;
            (*thr).allocator = allocator;
            (*thr).coder = coder;
            (*thr).progress_in = 0 as u64;
            (*thr).progress_out = 0 as u64;
            (*thr).block_encoder = lzma_next_coder_s {
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
            (*thr).filters[0].id = LZMA_VLI_UNKNOWN as lzma_vli;
            if mythread_create(
                &raw mut (*thr).thread_id,
                Some(worker_start as unsafe extern "C" fn(*mut c_void) -> *mut c_void),
                thr as *mut c_void,
            ) != 0
            {
                mythread_cond_destroy(&raw mut (*thr).cond);
            } else {
                (*coder).threads_initialized = (*coder).threads_initialized.wrapping_add(1);
                (*coder).thr = thr;
                return LZMA_OK;
            }
        }
        mythread_mutex_destroy(&raw mut (*thr).mutex);
    }
    lzma_free((*thr).in_0 as *mut c_void, allocator);
    return LZMA_MEM_ERROR;
}
unsafe extern "C" fn get_thread(
    mut coder: *mut lzma_stream_coder,
    mut allocator: *const lzma_allocator,
) -> lzma_ret {
    if !lzma_outq_has_buf(&raw mut (*coder).outq) {
        return LZMA_OK;
    }
    let ret_: lzma_ret = lzma_outq_prealloc_buf(
        &raw mut (*coder).outq,
        allocator,
        (*coder).outbuf_alloc_size,
    ) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    if (*coder).filters_cache[0].id == LZMA_VLI_UNKNOWN as lzma_vli {
        let ret__0: lzma_ret = lzma_filters_copy(
            &raw mut (*coder).filters as *mut lzma_filter,
            &raw mut (*coder).filters_cache as *mut lzma_filter,
            allocator,
        ) as lzma_ret;
        if ret__0 != LZMA_OK {
            return ret__0;
        }
    }
    let mut mythread_i_560: c_uint = 0;
    while if mythread_i_560 != 0 {
        mythread_mutex_unlock(&raw mut (*coder).mutex);
        0 as c_int
    } else {
        mythread_mutex_lock(&raw mut (*coder).mutex);
        1 as c_int
    } != 0
    {
        let mut mythread_j_560: c_uint = 0;
        while mythread_j_560 == 0 {
            if !(*coder).threads_free.is_null() {
                (*coder).thr = (*coder).threads_free;
                (*coder).threads_free = (*(*coder).threads_free).next;
            }
            mythread_j_560 = 1;
        }
        mythread_i_560 = 1;
    }
    if (*coder).thr.is_null() {
        if (*coder).threads_initialized == (*coder).threads_max {
            return LZMA_OK;
        }
        let ret__1: lzma_ret = initialize_new_thread(coder, allocator) as lzma_ret;
        if ret__1 != LZMA_OK {
            return ret__1;
        }
    }
    let mut mythread_i_578: c_uint = 0;
    while if mythread_i_578 != 0 {
        mythread_mutex_unlock(&raw mut (*(*coder).thr).mutex);
        0 as c_int
    } else {
        mythread_mutex_lock(&raw mut (*(*coder).thr).mutex);
        1 as c_int
    } != 0
    {
        let mut mythread_j_578: c_uint = 0;
        while mythread_j_578 == 0 {
            (*(*coder).thr).state = THR_RUN;
            (*(*coder).thr).in_size = 0 as size_t;
            (*(*coder).thr).outbuf = lzma_outq_get_buf(&raw mut (*coder).outq, NULL);
            lzma_filters_free(
                &raw mut (*(*coder).thr).filters as *mut lzma_filter,
                allocator,
            );
            memcpy(
                &raw mut (*(*coder).thr).filters as *mut lzma_filter as *mut c_void,
                &raw mut (*coder).filters_cache as *mut lzma_filter as *const c_void,
                ::core::mem::size_of::<[lzma_filter; 5]>() as size_t,
            );
            (*coder).filters_cache[0].id = LZMA_VLI_UNKNOWN as lzma_vli;
            mythread_cond_signal(&raw mut (*(*coder).thr).cond);
            mythread_j_578 = 1;
        }
        mythread_i_578 = 1;
    }
    return LZMA_OK;
}
unsafe extern "C" fn stream_encode_in(
    mut coder: *mut lzma_stream_coder,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut action: lzma_action,
) -> lzma_ret {
    while *in_pos < in_size || !(*coder).thr.is_null() && action != LZMA_RUN {
        if (*coder).thr.is_null() {
            let ret: lzma_ret = get_thread(coder, allocator) as lzma_ret;
            if (*coder).thr.is_null() {
                return ret;
            }
        }
        let mut thr_in_size: size_t = (*(*coder).thr).in_size;
        lzma_bufcpy(
            in_0,
            in_pos,
            in_size,
            (*(*coder).thr).in_0,
            &raw mut thr_in_size,
            (*coder).block_size,
        );
        let finish: bool =
            thr_in_size == (*coder).block_size || *in_pos == in_size && action != LZMA_RUN;
        let mut block_error: bool = false_0 != 0;
        let mut mythread_i_628: c_uint = 0;
        while if mythread_i_628 != 0 {
            mythread_mutex_unlock(&raw mut (*(*coder).thr).mutex);
            0 as c_int
        } else {
            mythread_mutex_lock(&raw mut (*(*coder).thr).mutex);
            1 as c_int
        } != 0
        {
            let mut mythread_j_628: c_uint = 0;
            while mythread_j_628 == 0 {
                if (*(*coder).thr).state == THR_IDLE {
                    block_error = true_0 != 0;
                } else {
                    (*(*coder).thr).in_size = thr_in_size;
                    if finish {
                        (*(*coder).thr).state = THR_FINISH;
                    }
                    mythread_cond_signal(&raw mut (*(*coder).thr).cond);
                }
                mythread_j_628 = 1;
            }
            mythread_i_628 = 1;
        }
        if block_error {
            let mut ret_0: lzma_ret = LZMA_OK;
            let mut mythread_i_649: c_uint = 0;
            while if mythread_i_649 != 0 {
                mythread_mutex_unlock(&raw mut (*coder).mutex);
                0 as c_int
            } else {
                mythread_mutex_lock(&raw mut (*coder).mutex);
                1 as c_int
            } != 0
            {
                let mut mythread_j_649: c_uint = 0;
                while mythread_j_649 == 0 {
                    ret_0 = (*coder).thread_error;
                    mythread_j_649 = 1;
                }
                mythread_i_649 = 1;
            }
            return ret_0;
        }
        if finish {
            (*coder).thr = ::core::ptr::null_mut::<worker_thread>();
        }
    }
    return LZMA_OK;
}
unsafe extern "C" fn wait_for_work(
    mut coder: *mut lzma_stream_coder,
    mut wait_abs: *mut mythread_condtime,
    mut has_blocked: *mut bool,
    mut has_input: bool,
) -> bool {
    if (*coder).timeout != 0 as u32 && !*has_blocked {
        *has_blocked = true_0 != 0;
        mythread_condtime_set(wait_abs, &raw mut (*coder).cond, (*coder).timeout);
    }
    let mut timed_out: bool = false_0 != 0;
    let mut mythread_i_689: c_uint = 0;
    while if mythread_i_689 != 0 {
        mythread_mutex_unlock(&raw mut (*coder).mutex);
        0 as c_int
    } else {
        mythread_mutex_lock(&raw mut (*coder).mutex);
        1 as c_int
    } != 0
    {
        let mut mythread_j_689: c_uint = 0;
        while mythread_j_689 == 0 {
            while (!has_input
                || (*coder).threads_free.is_null()
                || !lzma_outq_has_buf(&raw mut (*coder).outq))
                && !lzma_outq_is_readable(&raw mut (*coder).outq)
                && (*coder).thread_error == LZMA_OK
                && !timed_out
            {
                if (*coder).timeout != 0 as u32 {
                    timed_out = mythread_cond_timedwait(
                        &raw mut (*coder).cond,
                        &raw mut (*coder).mutex,
                        wait_abs,
                    ) != 0 as c_int;
                } else {
                    mythread_cond_wait(&raw mut (*coder).cond, &raw mut (*coder).mutex);
                }
            }
            mythread_j_689 = 1;
        }
        mythread_i_689 = 1;
    }
    return timed_out;
}
unsafe extern "C" fn stream_encode_mt(
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
    let mut coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    's_270: {
        let mut current_block_53: u64;
        match (*coder).sequence {
            0 => {
                lzma_bufcpy(
                    &raw mut (*coder).header as *mut u8,
                    &raw mut (*coder).header_pos,
                    ::core::mem::size_of::<[u8; 12]>() as size_t,
                    out,
                    out_pos,
                    out_size,
                );
                if (*coder).header_pos < ::core::mem::size_of::<[u8; 12]>() as usize {
                    return LZMA_OK;
                }
                (*coder).header_pos = 0 as size_t;
                (*coder).sequence = SEQ_BLOCK;
                current_block_53 = 18046538441878631153;
            }
            1 => {
                current_block_53 = 18046538441878631153;
            }
            2 => {
                current_block_53 = 7301844830188010456;
            }
            3 => {
                current_block_53 = 8365064614624041636;
            }
            _ => {
                break 's_270;
            }
        }
        match current_block_53 {
            18046538441878631153 => {
                let mut unpadded_size: lzma_vli = 0 as lzma_vli;
                let mut uncompressed_size: lzma_vli = 0 as lzma_vli;
                let mut ret: lzma_ret = LZMA_OK;
                let mut has_blocked: bool = false_0 != 0;
                let mut wait_abs: mythread_condtime = timespec {
                    tv_sec: 0 as __darwin_time_t,
                    tv_nsec: 0,
                };
                loop {
                    let mut mythread_i_747: c_uint = 0;
                    while if mythread_i_747 != 0 {
                        mythread_mutex_unlock(&raw mut (*coder).mutex);
                        0 as c_int
                    } else {
                        mythread_mutex_lock(&raw mut (*coder).mutex);
                        1 as c_int
                    } != 0
                    {
                        let mut mythread_j_747: c_uint = 0;
                        while mythread_j_747 == 0 {
                            ret = (*coder).thread_error;
                            if ret != LZMA_OK {
                                break;
                            }
                            ret = lzma_outq_read(
                                &raw mut (*coder).outq,
                                allocator,
                                out,
                                out_pos,
                                out_size,
                                &raw mut unpadded_size,
                                &raw mut uncompressed_size,
                            );
                            mythread_j_747 = 1;
                        }
                        mythread_i_747 = 1;
                    }
                    if ret == LZMA_STREAM_END {
                        ret = lzma_index_append(
                            (*coder).index,
                            allocator,
                            unpadded_size,
                            uncompressed_size,
                        );
                        if ret != LZMA_OK {
                            threads_stop(coder, false_0 != 0);
                            return ret;
                        }
                        if *out_pos < out_size {
                            continue;
                        }
                    }
                    if ret != LZMA_OK {
                        threads_stop(coder, false_0 != 0);
                        return ret;
                    }
                    ret = stream_encode_in(coder, allocator, in_0, in_pos, in_size, action);
                    if ret != LZMA_OK {
                        threads_stop(coder, false_0 != 0);
                        return ret;
                    }
                    if *in_pos == in_size {
                        if action == LZMA_RUN {
                            return LZMA_OK;
                        }
                        if action == LZMA_FULL_BARRIER {
                            return LZMA_STREAM_END;
                        }
                        if lzma_outq_is_empty(&raw mut (*coder).outq) {
                            if action == LZMA_FINISH {
                                break;
                            }
                            if action == LZMA_FULL_FLUSH {
                                return LZMA_STREAM_END;
                            }
                        }
                    }
                    if *out_pos == out_size {
                        return LZMA_OK;
                    }
                    if wait_for_work(
                        coder,
                        &raw mut wait_abs,
                        &raw mut has_blocked,
                        *in_pos < in_size,
                    ) {
                        return LZMA_RET_INTERNAL1;
                    }
                }
                let ret_: lzma_ret = lzma_index_encoder_init(
                    &raw mut (*coder).index_encoder,
                    allocator,
                    (*coder).index,
                ) as lzma_ret;
                if ret_ != LZMA_OK {
                    return ret_;
                }
                (*coder).sequence = SEQ_INDEX;
                (*coder).progress_out = (*coder).progress_out.wrapping_add(
                    lzma_index_size((*coder).index)
                        .wrapping_add(LZMA_STREAM_HEADER_SIZE as lzma_vli)
                        as u64,
                );
                current_block_53 = 7301844830188010456;
            }
            _ => {}
        }
        match current_block_53 {
            7301844830188010456 => {
                let ret_0: lzma_ret = (*coder)
                    .index_encoder
                    .code
                    .expect("non-null function pointer")(
                    (*coder).index_encoder.coder,
                    allocator,
                    ::core::ptr::null::<u8>(),
                    ::core::ptr::null_mut::<size_t>(),
                    0 as size_t,
                    out,
                    out_pos,
                    out_size,
                    LZMA_RUN,
                ) as lzma_ret;
                if ret_0 != LZMA_STREAM_END {
                    return ret_0;
                }
                (*coder).stream_flags.backward_size = lzma_index_size((*coder).index);
                if lzma_stream_footer_encode(
                    &raw mut (*coder).stream_flags,
                    &raw mut (*coder).header as *mut u8,
                ) != LZMA_OK
                {
                    return LZMA_PROG_ERROR;
                }
                (*coder).sequence = SEQ_STREAM_FOOTER;
            }
            _ => {}
        }
        lzma_bufcpy(
            &raw mut (*coder).header as *mut u8,
            &raw mut (*coder).header_pos,
            ::core::mem::size_of::<[u8; 12]>() as size_t,
            out,
            out_pos,
            out_size,
        );
        return (if (*coder).header_pos < ::core::mem::size_of::<[u8; 12]>() as usize {
            LZMA_OK as c_int
        } else {
            LZMA_STREAM_END as c_int
        }) as lzma_ret;
    }
    return LZMA_PROG_ERROR;
}
unsafe extern "C" fn stream_encoder_mt_end(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    threads_end(coder, allocator);
    lzma_outq_end(&raw mut (*coder).outq, allocator);
    lzma_filters_free(&raw mut (*coder).filters as *mut lzma_filter, allocator);
    lzma_filters_free(
        &raw mut (*coder).filters_cache as *mut lzma_filter,
        allocator,
    );
    lzma_next_end(&raw mut (*coder).index_encoder, allocator);
    lzma_index_end((*coder).index, allocator);
    mythread_cond_destroy(&raw mut (*coder).cond);
    mythread_mutex_destroy(&raw mut (*coder).mutex);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn stream_encoder_mt_update(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter,
    mut reversed_filters: *const lzma_filter,
) -> lzma_ret {
    let mut coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    if (*coder).sequence > SEQ_BLOCK {
        return LZMA_PROG_ERROR;
    }
    if !(*coder).thr.is_null() {
        return LZMA_PROG_ERROR;
    }
    if lzma_raw_encoder_memusage(filters) == UINT64_MAX as u64 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut temp: [lzma_filter; 5] = [lzma_filter {
        id: 0,
        options: ::core::ptr::null_mut::<c_void>(),
    }; 5];
    let ret_: lzma_ret =
        lzma_filters_copy(filters, &raw mut temp as *mut lzma_filter, allocator) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    lzma_filters_free(&raw mut (*coder).filters as *mut lzma_filter, allocator);
    lzma_filters_free(
        &raw mut (*coder).filters_cache as *mut lzma_filter,
        allocator,
    );
    memcpy(
        &raw mut (*coder).filters as *mut lzma_filter as *mut c_void,
        &raw mut temp as *mut lzma_filter as *const c_void,
        ::core::mem::size_of::<[lzma_filter; 5]>() as size_t,
    );
    return LZMA_OK;
}
unsafe extern "C" fn get_options(
    mut options: *const lzma_mt,
    mut opt_easy: *mut lzma_options_easy,
    mut filters: *mut *const lzma_filter,
    mut block_size: *mut u64,
    mut outbuf_size_max: *mut u64,
) -> lzma_ret {
    if options.is_null() {
        return LZMA_PROG_ERROR;
    }
    if (*options).flags != 0 as u32
        || (*options).threads == 0 as u32
        || (*options).threads > LZMA_THREADS_MAX as u32
    {
        return LZMA_OPTIONS_ERROR;
    }
    if !(*options).filters.is_null() {
        *filters = (*options).filters;
    } else {
        if lzma_easy_preset(opt_easy, (*options).preset) {
            return LZMA_OPTIONS_ERROR;
        }
        *filters = &raw mut (*opt_easy).filters as *mut lzma_filter;
    }
    if (*options).block_size > 0 as u64 {
        *block_size = (*options).block_size;
    } else {
        *block_size = lzma_mt_block_size(*filters);
    }
    if *block_size > BLOCK_SIZE_MAX as u64 || *block_size == UINT64_MAX as u64 {
        return LZMA_OPTIONS_ERROR;
    }
    *outbuf_size_max = lzma_block_buffer_bound64(*block_size);
    if *outbuf_size_max == 0 as u64 {
        return LZMA_MEM_ERROR;
    }
    return LZMA_OK;
}
unsafe extern "C" fn get_progress(
    mut coder_ptr: *mut c_void,
    mut progress_in: *mut u64,
    mut progress_out: *mut u64,
) {
    let mut coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    let mut mythread_i_1010: c_uint = 0;
    while if mythread_i_1010 != 0 {
        mythread_mutex_unlock(&raw mut (*coder).mutex);
        0 as c_int
    } else {
        mythread_mutex_lock(&raw mut (*coder).mutex);
        1 as c_int
    } != 0
    {
        let mut mythread_j_1010: c_uint = 0;
        while mythread_j_1010 == 0 {
            *progress_in = (*coder).progress_in;
            *progress_out = (*coder).progress_out;
            let mut i: size_t = 0 as size_t;
            while i < (*coder).threads_initialized as size_t {
                let mut mythread_i_1015: c_uint = 0;
                while if mythread_i_1015 != 0 {
                    mythread_mutex_unlock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
                    0 as c_int
                } else {
                    mythread_mutex_lock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
                    1 as c_int
                } != 0
                {
                    let mut mythread_j_1015: c_uint = 0;
                    while mythread_j_1015 == 0 {
                        *progress_in = (*progress_in)
                            .wrapping_add((*(*coder).threads.offset(i as isize)).progress_in);
                        *progress_out = (*progress_out)
                            .wrapping_add((*(*coder).threads.offset(i as isize)).progress_out);
                        mythread_j_1015 = 1;
                    }
                    mythread_i_1015 = 1;
                }
                i = i.wrapping_add(1);
            }
            mythread_j_1010 = 1;
        }
        mythread_i_1010 = 1;
    }
}
unsafe extern "C" fn stream_encoder_mt_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut options: *const lzma_mt,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_mt,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        stream_encoder_mt_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_mt,
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
                *const lzma_mt,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        stream_encoder_mt_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_mt,
            ) -> lzma_ret,
    ));
    let mut easy: lzma_options_easy = lzma_options_easy {
        filters: [lzma_filter {
            id: 0,
            options: ::core::ptr::null_mut::<c_void>(),
        }; 5],
        opt_lzma: lzma_options_lzma {
            dict_size: 0,
            preset_dict: ::core::ptr::null::<u8>(),
            preset_dict_size: 0,
            lc: 0,
            lp: 0,
            pb: 0,
            mode: 0 as lzma_mode,
            nice_len: 0,
            mf: 0 as lzma_match_finder,
            depth: 0,
            ext_flags: 0,
            ext_size_low: 0,
            ext_size_high: 0,
            reserved_int4: 0,
            reserved_int5: 0,
            reserved_int6: 0,
            reserved_int7: 0,
            reserved_int8: 0,
            reserved_enum1: LZMA_RESERVED_ENUM,
            reserved_enum2: LZMA_RESERVED_ENUM,
            reserved_enum3: LZMA_RESERVED_ENUM,
            reserved_enum4: LZMA_RESERVED_ENUM,
            reserved_ptr1: ::core::ptr::null_mut::<c_void>(),
            reserved_ptr2: ::core::ptr::null_mut::<c_void>(),
        },
    };
    let mut filters: *const lzma_filter = ::core::ptr::null::<lzma_filter>();
    let mut block_size: u64 = 0;
    let mut outbuf_size_max: u64 = 0;
    let ret_: lzma_ret = get_options(
        options,
        &raw mut easy,
        &raw mut filters,
        &raw mut block_size,
        &raw mut outbuf_size_max,
    ) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    if lzma_raw_encoder_memusage(filters) == UINT64_MAX as u64 {
        return LZMA_OPTIONS_ERROR;
    }
    if (*options).check > LZMA_CHECK_ID_MAX {
        return LZMA_PROG_ERROR;
    }
    if lzma_check_is_supported((*options).check) == 0 {
        return LZMA_UNSUPPORTED_CHECK;
    }
    let mut coder: *mut lzma_stream_coder = (*next).coder as *mut lzma_stream_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            ::core::mem::size_of::<lzma_stream_coder>() as size_t,
            allocator,
        ) as *mut lzma_stream_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        if mythread_mutex_init(&raw mut (*coder).mutex) != 0 {
            lzma_free(coder as *mut c_void, allocator);
            (*next).coder = NULL;
            return LZMA_MEM_ERROR;
        }
        if mythread_cond_init(&raw mut (*coder).cond) != 0 {
            mythread_mutex_destroy(&raw mut (*coder).mutex);
            lzma_free(coder as *mut c_void, allocator);
            (*next).coder = NULL;
            return LZMA_MEM_ERROR;
        }
        (*next).code = Some(
            stream_encode_mt
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
            stream_encoder_mt_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).get_progress =
            Some(get_progress as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64) -> ())
                as Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64) -> ()>;
        (*next).update = Some(
            stream_encoder_mt_update
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const lzma_allocator,
                    *const lzma_filter,
                    *const lzma_filter,
                ) -> lzma_ret,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut c_void,
                    *const lzma_allocator,
                    *const lzma_filter,
                    *const lzma_filter,
                ) -> lzma_ret,
            >;
        (*coder).filters[0].id = LZMA_VLI_UNKNOWN as lzma_vli;
        (*coder).filters_cache[0].id = LZMA_VLI_UNKNOWN as lzma_vli;
        (*coder).index_encoder = lzma_next_coder_s {
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
        (*coder).index = ::core::ptr::null_mut::<lzma_index>();
        memset(
            &raw mut (*coder).outq as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<lzma_outq>() as size_t,
        );
        (*coder).threads = ::core::ptr::null_mut::<worker_thread>();
        (*coder).threads_max = 0 as u32;
        (*coder).threads_initialized = 0 as u32;
    }
    (*coder).sequence = SEQ_STREAM_HEADER;
    (*coder).block_size = block_size as size_t;
    (*coder).outbuf_alloc_size = outbuf_size_max as size_t;
    (*coder).thread_error = LZMA_OK;
    (*coder).thr = ::core::ptr::null_mut::<worker_thread>();
    if (*coder).threads_max != (*options).threads {
        threads_end(coder, allocator);
        (*coder).threads = ::core::ptr::null_mut::<worker_thread>();
        (*coder).threads_max = 0 as u32;
        (*coder).threads_initialized = 0 as u32;
        (*coder).threads_free = ::core::ptr::null_mut::<worker_thread>();
        (*coder).threads = lzma_alloc(
            ((*options).threads as size_t)
                .wrapping_mul(::core::mem::size_of::<worker_thread>() as size_t),
            allocator,
        ) as *mut worker_thread;
        if (*coder).threads.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*coder).threads_max = (*options).threads;
    } else {
        threads_stop(coder, true_0 != 0);
    }
    let ret__0: lzma_ret =
        lzma_outq_init(&raw mut (*coder).outq, allocator, (*options).threads) as lzma_ret;
    if ret__0 != LZMA_OK {
        return ret__0;
    }
    (*coder).timeout = (*options).timeout;
    lzma_filters_free(&raw mut (*coder).filters as *mut lzma_filter, allocator);
    lzma_filters_free(
        &raw mut (*coder).filters_cache as *mut lzma_filter,
        allocator,
    );
    let ret__1: lzma_ret = lzma_filters_copy(
        filters,
        &raw mut (*coder).filters as *mut lzma_filter,
        allocator,
    ) as lzma_ret;
    if ret__1 != LZMA_OK {
        return ret__1;
    }
    lzma_index_end((*coder).index, allocator);
    (*coder).index = lzma_index_init(allocator);
    if (*coder).index.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*coder).stream_flags.version = 0 as u32;
    (*coder).stream_flags.check = (*options).check;
    let ret__2: lzma_ret = lzma_stream_header_encode(
        &raw mut (*coder).stream_flags,
        &raw mut (*coder).header as *mut u8,
    ) as lzma_ret;
    if ret__2 != LZMA_OK {
        return ret__2;
    }
    (*coder).header_pos = 0 as size_t;
    (*coder).progress_in = 0 as u64;
    (*coder).progress_out = LZMA_STREAM_HEADER_SIZE as u64;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_encoder_mt(
    mut strm: *mut lzma_stream,
    mut options: *const lzma_mt,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = stream_encoder_mt_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        options,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true_0 != 0;
    (*(*strm).internal).supported_actions[LZMA_FULL_FLUSH as usize] = true_0 != 0;
    (*(*strm).internal).supported_actions[LZMA_FULL_BARRIER as usize] = true_0 != 0;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true_0 != 0;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_encoder_mt_memusage(mut options: *const lzma_mt) -> u64 {
    let mut easy: lzma_options_easy = lzma_options_easy {
        filters: [lzma_filter {
            id: 0,
            options: ::core::ptr::null_mut::<c_void>(),
        }; 5],
        opt_lzma: lzma_options_lzma {
            dict_size: 0,
            preset_dict: ::core::ptr::null::<u8>(),
            preset_dict_size: 0,
            lc: 0,
            lp: 0,
            pb: 0,
            mode: 0 as lzma_mode,
            nice_len: 0,
            mf: 0 as lzma_match_finder,
            depth: 0,
            ext_flags: 0,
            ext_size_low: 0,
            ext_size_high: 0,
            reserved_int4: 0,
            reserved_int5: 0,
            reserved_int6: 0,
            reserved_int7: 0,
            reserved_int8: 0,
            reserved_enum1: LZMA_RESERVED_ENUM,
            reserved_enum2: LZMA_RESERVED_ENUM,
            reserved_enum3: LZMA_RESERVED_ENUM,
            reserved_enum4: LZMA_RESERVED_ENUM,
            reserved_ptr1: ::core::ptr::null_mut::<c_void>(),
            reserved_ptr2: ::core::ptr::null_mut::<c_void>(),
        },
    };
    let mut filters: *const lzma_filter = ::core::ptr::null::<lzma_filter>();
    let mut block_size: u64 = 0;
    let mut outbuf_size_max: u64 = 0;
    if get_options(
        options,
        &raw mut easy,
        &raw mut filters,
        &raw mut block_size,
        &raw mut outbuf_size_max,
    ) != LZMA_OK
    {
        return UINT64_MAX as u64;
    }
    let inbuf_memusage: u64 = ((*options).threads as u64).wrapping_mul(block_size);
    let mut filters_memusage: u64 = lzma_raw_encoder_memusage(filters);
    if filters_memusage == UINT64_MAX as u64 {
        return UINT64_MAX as u64;
    }
    filters_memusage = filters_memusage.wrapping_mul((*options).threads as u64);
    let outq_memusage: u64 = lzma_outq_memusage(outbuf_size_max, (*options).threads) as u64;
    if outq_memusage == UINT64_MAX as u64 {
        return UINT64_MAX as u64;
    }
    let mut total_memusage: u64 = (LZMA_MEMUSAGE_BASE as u64)
        .wrapping_add(::core::mem::size_of::<lzma_stream_coder>() as u64)
        .wrapping_add(
            ((*options).threads as usize)
                .wrapping_mul(::core::mem::size_of::<worker_thread>() as usize) as u64,
        );
    if (UINT64_MAX as u64).wrapping_sub(total_memusage) < inbuf_memusage {
        return UINT64_MAX as u64;
    }
    total_memusage = total_memusage.wrapping_add(inbuf_memusage);
    if (UINT64_MAX as u64).wrapping_sub(total_memusage) < filters_memusage {
        return UINT64_MAX as u64;
    }
    total_memusage = total_memusage.wrapping_add(filters_memusage);
    if (UINT64_MAX as u64).wrapping_sub(total_memusage) < outq_memusage {
        return UINT64_MAX as u64;
    }
    return total_memusage.wrapping_add(outq_memusage);
}
