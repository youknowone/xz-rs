use crate::types::*;
use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ulonglong, c_void};
pub enum lzma_index_hash_s {}
extern "C" {
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
    fn lzma_check_size(check: lzma_check) -> u32;
    fn lzma_filters_free(filters: *mut lzma_filter, allocator: *const lzma_allocator);
    fn lzma_raw_decoder_memusage(filters: *const lzma_filter) -> u64;
    fn lzma_stream_header_decode(options: *mut lzma_stream_flags, in_0: *const u8) -> lzma_ret;
    fn lzma_stream_footer_decode(options: *mut lzma_stream_flags, in_0: *const u8) -> lzma_ret;
    fn lzma_stream_flags_compare(
        a: *const lzma_stream_flags,
        b: *const lzma_stream_flags,
    ) -> lzma_ret;
    fn lzma_block_header_decode(
        block: *mut lzma_block,
        allocator: *const lzma_allocator,
        in_0: *const u8,
    ) -> lzma_ret;
    fn lzma_block_unpadded_size(block: *const lzma_block) -> lzma_vli;
    fn lzma_index_hash_init(
        index_hash: *mut lzma_index_hash,
        allocator: *const lzma_allocator,
    ) -> *mut lzma_index_hash;
    fn lzma_index_hash_end(index_hash: *mut lzma_index_hash, allocator: *const lzma_allocator);
    fn lzma_index_hash_append(
        index_hash: *mut lzma_index_hash,
        unpadded_size: lzma_vli,
        uncompressed_size: lzma_vli,
    ) -> lzma_ret;
    fn lzma_index_hash_decode(
        index_hash: *mut lzma_index_hash,
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
    ) -> lzma_ret;
    fn lzma_index_hash_size(index_hash: *const lzma_index_hash) -> lzma_vli;
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
    fn lzma_block_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        block: *mut lzma_block,
    ) -> lzma_ret;
    fn lzma_outq_init(
        outq: *mut lzma_outq,
        allocator: *const lzma_allocator,
        threads: u32,
    ) -> lzma_ret;
    fn lzma_outq_end(outq: *mut lzma_outq, allocator: *const lzma_allocator);
    fn lzma_outq_clear_cache(outq: *mut lzma_outq, allocator: *const lzma_allocator);
    fn lzma_outq_clear_cache2(
        outq: *mut lzma_outq,
        allocator: *const lzma_allocator,
        keep_size: size_t,
    );
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
    fn lzma_outq_enable_partial_output(
        outq: *mut lzma_outq,
        enable_partial_output: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream_coder {
    pub sequence: C2RustUnnamed_0,
    pub block_decoder: lzma_next_coder,
    pub block_options: lzma_block,
    pub filters: [lzma_filter; 5],
    pub stream_flags: lzma_stream_flags,
    pub index_hash: *mut lzma_index_hash,
    pub timeout: u32,
    pub thread_error: lzma_ret,
    pub pending_error: lzma_ret,
    pub threads_max: u32,
    pub threads_initialized: u32,
    pub threads: *mut worker_thread,
    pub threads_free: *mut worker_thread,
    pub thr: *mut worker_thread,
    pub outq: lzma_outq,
    pub mutex: mythread_mutex,
    pub cond: mythread_cond,
    pub memlimit_threading: u64,
    pub memlimit_stop: u64,
    pub mem_direct_mode: u64,
    pub mem_in_use: u64,
    pub mem_cached: u64,
    pub mem_next_filters: u64,
    pub mem_next_in: u64,
    pub mem_next_block: u64,
    pub progress_in: u64,
    pub progress_out: u64,
    pub tell_no_check: bool,
    pub tell_unsupported_check: bool,
    pub tell_any_check: bool,
    pub ignore_check: bool,
    pub concatenated: bool,
    pub fail_fast: bool,
    pub first_stream: bool,
    pub out_was_filled: bool,
    pub pos: size_t,
    pub buffer: [u8; LZMA_BLOCK_HEADER_SIZE_MAX as usize],
}
pub const LZMA_BLOCK_HEADER_SIZE_MAX: c_int = 1024 as c_int;
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
pub struct worker_thread {
    pub state: worker_state,
    pub in_0: *mut u8,
    pub in_size: size_t,
    pub in_filled: size_t,
    pub in_pos: size_t,
    pub out_pos: size_t,
    pub coder: *mut lzma_stream_coder,
    pub allocator: *const lzma_allocator,
    pub outbuf: *mut lzma_outbuf,
    pub progress_in: size_t,
    pub progress_out: size_t,
    pub partial_update_enabled: bool,
    pub partial_update_started: bool,
    pub block_decoder: lzma_next_coder,
    pub block_options: lzma_block,
    pub mem_filters: u64,
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
pub type worker_state = c_uint;
pub const THR_EXIT: worker_state = 2;
pub const THR_RUN: worker_state = 1;
pub const THR_IDLE: worker_state = 0;
pub type lzma_index_hash = lzma_index_hash_s;
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
pub type C2RustUnnamed_0 = c_uint;
pub const SEQ_ERROR: C2RustUnnamed_0 = 11;
pub const SEQ_STREAM_PADDING: C2RustUnnamed_0 = 10;
pub const SEQ_STREAM_FOOTER: C2RustUnnamed_0 = 9;
pub const SEQ_INDEX_DECODE: C2RustUnnamed_0 = 8;
pub const SEQ_INDEX_WAIT_OUTPUT: C2RustUnnamed_0 = 7;
pub const SEQ_BLOCK_DIRECT_RUN: C2RustUnnamed_0 = 6;
pub const SEQ_BLOCK_DIRECT_INIT: C2RustUnnamed_0 = 5;
pub const SEQ_BLOCK_THR_RUN: C2RustUnnamed_0 = 4;
pub const SEQ_BLOCK_THR_INIT: C2RustUnnamed_0 = 3;
pub const SEQ_BLOCK_INIT: C2RustUnnamed_0 = 2;
pub const SEQ_BLOCK_HEADER: C2RustUnnamed_0 = 1;
pub const SEQ_STREAM_HEADER: C2RustUnnamed_0 = 0;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const UINTPTR_MAX: c_ulong = uintptr_t::MAX as c_ulong;
pub const SIZE_MAX: c_ulong = UINTPTR_MAX;
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
pub const LZMA_TELL_NO_CHECK: c_uint = 0x1;
pub const LZMA_TELL_UNSUPPORTED_CHECK: c_uint = 0x2;
pub const LZMA_TELL_ANY_CHECK: c_uint = 0x4;
pub const LZMA_IGNORE_CHECK: c_uint = 0x10;
pub const LZMA_CONCATENATED: c_uint = 0x8;
pub const LZMA_FAIL_FAST: c_uint = 0x20;
pub const LZMA_STREAM_HEADER_SIZE: c_int = 12 as c_int;
pub const LZMA_THREADS_MAX: c_int = 16384 as c_int;
pub const LZMA_MEMUSAGE_BASE: c_ulonglong = 1 << 15;
pub const LZMA_SUPPORTED_FLAGS: c_uint = LZMA_TELL_NO_CHECK
    | LZMA_TELL_UNSUPPORTED_CHECK
    | LZMA_TELL_ANY_CHECK
    | LZMA_IGNORE_CHECK
    | LZMA_CONCATENATED
    | LZMA_FAIL_FAST;
pub const INDEX_INDICATOR: c_int = 0 as c_int;
#[inline]
unsafe extern "C" fn vli_ceil4(mut vli: lzma_vli) -> lzma_vli {
    return vli.wrapping_add(3 as lzma_vli) & !(3 as lzma_vli);
}
#[inline]
unsafe extern "C" fn lzma_outq_has_buf(mut outq: *const lzma_outq) -> bool {
    return (*outq).bufs_in_use < (*outq).bufs_limit;
}
#[inline]
unsafe extern "C" fn lzma_outq_is_empty(mut outq: *const lzma_outq) -> bool {
    return (*outq).bufs_in_use == 0 as u32;
}
#[inline]
unsafe extern "C" fn lzma_outq_outbuf_memusage(mut buf_size: size_t) -> u64 {
    return (::core::mem::size_of::<lzma_outbuf>() as usize).wrapping_add(buf_size as usize) as u64;
}
unsafe extern "C" fn worker_enable_partial_update(mut thr_ptr: *mut c_void) {
    let mut thr: *mut worker_thread = thr_ptr as *mut worker_thread;
    let mut mythread_i_325: c_uint = 0;
    while if mythread_i_325 != 0 {
        mythread_mutex_unlock(&raw mut (*thr).mutex);
        0 as c_int
    } else {
        mythread_mutex_lock(&raw mut (*thr).mutex);
        1 as c_int
    } != 0
    {
        let mut mythread_j_325: c_uint = 0;
        while mythread_j_325 == 0 {
            (*thr).partial_update_enabled = true_0 != 0;
            mythread_cond_signal(&raw mut (*thr).cond);
            mythread_j_325 = 1;
        }
        mythread_i_325 = 1;
    }
}
unsafe extern "C" fn worker_decoder(mut thr_ptr: *mut c_void) -> *mut c_void {
    let mut thr: *mut worker_thread = thr_ptr as *mut worker_thread;
    let mut in_filled: size_t = 0;
    let mut partial_update_enabled: bool = false;
    let mut ret: lzma_ret = LZMA_OK;
    loop {
        mythread_mutex_lock(&raw mut (*thr).mutex);
        loop {
            if (*thr).state == THR_IDLE {
                mythread_cond_wait(&raw mut (*thr).cond, &raw mut (*thr).mutex);
            } else {
                if (*thr).state == THR_EXIT {
                    mythread_mutex_unlock(&raw mut (*thr).mutex);
                    lzma_free((*thr).in_0 as *mut c_void, (*thr).allocator);
                    lzma_next_end(&raw mut (*thr).block_decoder, (*thr).allocator);
                    mythread_mutex_destroy(&raw mut (*thr).mutex);
                    mythread_cond_destroy(&raw mut (*thr).cond);
                    return MYTHREAD_RET_VALUE;
                }
                (*thr).progress_in = (*thr).in_pos;
                (*thr).progress_out = (*thr).out_pos;
                in_filled = (*thr).in_filled;
                partial_update_enabled = (*thr).partial_update_enabled;
                if !(in_filled == (*thr).in_pos
                    && !(partial_update_enabled as c_int != 0 && !(*thr).partial_update_started))
                {
                    break;
                }
                mythread_cond_wait(&raw mut (*thr).cond, &raw mut (*thr).mutex);
            }
        }
        mythread_mutex_unlock(&raw mut (*thr).mutex);
        let chunk_size: size_t = 16384 as size_t;
        if in_filled.wrapping_sub((*thr).in_pos) > chunk_size {
            in_filled = (*thr).in_pos.wrapping_add(chunk_size);
        }
        ret = (*thr)
            .block_decoder
            .code
            .expect("non-null function pointer")(
            (*thr).block_decoder.coder,
            (*thr).allocator,
            (*thr).in_0,
            &raw mut (*thr).in_pos,
            in_filled,
            &raw mut (*(*thr).outbuf).buf as *mut u8,
            &raw mut (*thr).out_pos,
            (*(*thr).outbuf).allocated,
            LZMA_RUN,
        );
        if ret == LZMA_OK {
            if partial_update_enabled {
                (*thr).partial_update_started = true_0 != 0;
                let mut mythread_i_415: c_uint = 0;
                while if mythread_i_415 != 0 {
                    mythread_mutex_unlock(&raw mut (*(*thr).coder).mutex);
                    0 as c_int
                } else {
                    mythread_mutex_lock(&raw mut (*(*thr).coder).mutex);
                    1 as c_int
                } != 0
                {
                    let mut mythread_j_415: c_uint = 0;
                    while mythread_j_415 == 0 {
                        (*(*thr).outbuf).pos = (*thr).out_pos;
                        (*(*thr).outbuf).decoder_in_pos = (*thr).in_pos;
                        mythread_cond_signal(&raw mut (*(*thr).coder).cond);
                        mythread_j_415 = 1;
                    }
                    mythread_i_415 = 1;
                }
            }
        } else {
            let mut mythread_i_434: c_uint = 0;
            while if mythread_i_434 != 0 {
                mythread_mutex_unlock(&raw mut (*thr).mutex);
                0 as c_int
            } else {
                mythread_mutex_lock(&raw mut (*thr).mutex);
                1 as c_int
            } != 0
            {
                let mut mythread_j_434: c_uint = 0;
                while mythread_j_434 == 0 {
                    if ret == LZMA_STREAM_END && (*thr).in_filled != (*thr).in_size {
                        ret = LZMA_PROG_ERROR;
                    }
                    if (*thr).state != THR_EXIT {
                        (*thr).state = THR_IDLE;
                    }
                    mythread_j_434 = 1;
                }
                mythread_i_434 = 1;
            }
            if ret == LZMA_STREAM_END {
                lzma_free((*thr).in_0 as *mut c_void, (*thr).allocator);
                (*thr).in_0 = ::core::ptr::null_mut::<u8>();
            }
            let mut mythread_i_458: c_uint = 0;
            while if mythread_i_458 != 0 {
                mythread_mutex_unlock(&raw mut (*(*thr).coder).mutex);
                0 as c_int
            } else {
                mythread_mutex_lock(&raw mut (*(*thr).coder).mutex);
                1 as c_int
            } != 0
            {
                let mut mythread_j_458: c_uint = 0;
                while mythread_j_458 == 0 {
                    (*(*thr).coder).progress_in = (*(*thr).coder)
                        .progress_in
                        .wrapping_add((*thr).in_pos as u64);
                    (*(*thr).coder).progress_out = (*(*thr).coder)
                        .progress_out
                        .wrapping_add((*thr).out_pos as u64);
                    (*thr).progress_in = 0 as size_t;
                    (*thr).progress_out = 0 as size_t;
                    (*(*thr).outbuf).pos = (*thr).out_pos;
                    (*(*thr).outbuf).decoder_in_pos = (*thr).in_pos;
                    (*(*thr).outbuf).finished = true_0 != 0;
                    (*(*thr).outbuf).finish_ret = ret;
                    (*thr).outbuf = ::core::ptr::null_mut::<lzma_outbuf>();
                    if ret != LZMA_STREAM_END && (*(*thr).coder).thread_error == LZMA_OK {
                        (*(*thr).coder).thread_error = ret;
                    }
                    if ret == LZMA_STREAM_END {
                        (*(*thr).coder).mem_in_use = (*(*thr).coder)
                            .mem_in_use
                            .wrapping_sub((*thr).in_size as u64);
                        (*(*thr).coder).mem_in_use =
                            (*(*thr).coder).mem_in_use.wrapping_sub((*thr).mem_filters);
                        (*(*thr).coder).mem_cached =
                            (*(*thr).coder).mem_cached.wrapping_add((*thr).mem_filters);
                        (*thr).next = (*(*thr).coder).threads_free;
                        (*(*thr).coder).threads_free = thr;
                    }
                    mythread_cond_signal(&raw mut (*(*thr).coder).cond);
                    mythread_j_458 = 1;
                }
                mythread_i_458 = 1;
            }
        }
    }
}
unsafe extern "C" fn threads_end(
    mut coder: *mut lzma_stream_coder,
    mut allocator: *const lzma_allocator,
) {
    let mut i: u32 = 0 as u32;
    while i < (*coder).threads_initialized {
        let mut mythread_i_502: c_uint = 0;
        while if mythread_i_502 != 0 {
            mythread_mutex_unlock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
            0 as c_int
        } else {
            mythread_mutex_lock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
            1 as c_int
        } != 0
        {
            let mut mythread_j_502: c_uint = 0;
            while mythread_j_502 == 0 {
                (*(*coder).threads.offset(i as isize)).state = THR_EXIT;
                mythread_cond_signal(&raw mut (*(*coder).threads.offset(i as isize)).cond);
                mythread_j_502 = 1;
            }
            mythread_i_502 = 1;
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: u32 = 0 as u32;
    while i_0 < (*coder).threads_initialized {
        mythread_join((*(*coder).threads.offset(i_0 as isize)).thread_id);
        i_0 = i_0.wrapping_add(1);
    }
    lzma_free((*coder).threads as *mut c_void, allocator);
    (*coder).threads_initialized = 0 as u32;
    (*coder).threads = ::core::ptr::null_mut::<worker_thread>();
    (*coder).threads_free = ::core::ptr::null_mut::<worker_thread>();
    (*coder).mem_in_use = 0 as u64;
    (*coder).mem_cached = 0 as u64;
}
unsafe extern "C" fn threads_stop(mut coder: *mut lzma_stream_coder) {
    let mut i: u32 = 0 as u32;
    while i < (*coder).threads_initialized {
        let mut mythread_i_538: c_uint = 0;
        while if mythread_i_538 != 0 {
            mythread_mutex_unlock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
            0 as c_int
        } else {
            mythread_mutex_lock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
            1 as c_int
        } != 0
        {
            let mut mythread_j_538: c_uint = 0;
            while mythread_j_538 == 0 {
                (*(*coder).threads.offset(i as isize)).state = THR_IDLE;
                mythread_j_538 = 1;
            }
            mythread_i_538 = 1;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn initialize_new_thread(
    mut coder: *mut lzma_stream_coder,
    mut allocator: *const lzma_allocator,
) -> lzma_ret {
    if (*coder).threads.is_null() {
        (*coder).threads = lzma_alloc(
            ((*coder).threads_max as size_t)
                .wrapping_mul(::core::mem::size_of::<worker_thread>() as size_t),
            allocator,
        ) as *mut worker_thread;
        if (*coder).threads.is_null() {
            return LZMA_MEM_ERROR;
        }
    }
    let mut thr: *mut worker_thread = (*coder)
        .threads
        .offset((*coder).threads_initialized as isize)
        as *mut worker_thread;
    if !(mythread_mutex_init(&raw mut (*thr).mutex) != 0) {
        if !(mythread_cond_init(&raw mut (*thr).cond) != 0) {
            (*thr).state = THR_IDLE;
            (*thr).in_0 = ::core::ptr::null_mut::<u8>();
            (*thr).in_size = 0 as size_t;
            (*thr).allocator = allocator;
            (*thr).coder = coder as *mut lzma_stream_coder;
            (*thr).outbuf = ::core::ptr::null_mut::<lzma_outbuf>();
            (*thr).block_decoder = lzma_next_coder_s {
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
            (*thr).mem_filters = 0 as u64;
            if mythread_create(
                &raw mut (*thr).thread_id,
                Some(worker_decoder as unsafe extern "C" fn(*mut c_void) -> *mut c_void),
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
    return LZMA_MEM_ERROR;
}
unsafe extern "C" fn get_thread(
    mut coder: *mut lzma_stream_coder,
    mut allocator: *const lzma_allocator,
) -> lzma_ret {
    let mut mythread_i_608: c_uint = 0;
    while if mythread_i_608 != 0 {
        mythread_mutex_unlock(&raw mut (*coder).mutex);
        0 as c_int
    } else {
        mythread_mutex_lock(&raw mut (*coder).mutex);
        1 as c_int
    } != 0
    {
        let mut mythread_j_608: c_uint = 0;
        while mythread_j_608 == 0 {
            if !(*coder).threads_free.is_null() {
                (*coder).thr = (*coder).threads_free;
                (*coder).threads_free = (*(*coder).threads_free).next;
                (*coder).mem_cached = (*coder)
                    .mem_cached
                    .wrapping_sub((*(*coder).thr).mem_filters);
            }
            mythread_j_608 = 1;
        }
        mythread_i_608 = 1;
    }
    if (*coder).thr.is_null() {
        let ret_: lzma_ret = initialize_new_thread(coder, allocator) as lzma_ret;
        if ret_ != LZMA_OK {
            return ret_;
        }
    }
    (*(*coder).thr).in_filled = 0 as size_t;
    (*(*coder).thr).in_pos = 0 as size_t;
    (*(*coder).thr).out_pos = 0 as size_t;
    (*(*coder).thr).progress_in = 0 as size_t;
    (*(*coder).thr).progress_out = 0 as size_t;
    (*(*coder).thr).partial_update_enabled = false_0 != 0;
    (*(*coder).thr).partial_update_started = false_0 != 0;
    return LZMA_OK;
}
unsafe extern "C" fn read_output_and_wait(
    mut coder: *mut lzma_stream_coder,
    mut allocator: *const lzma_allocator,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut input_is_possible: *mut bool,
    mut waiting_allowed: bool,
    mut wait_abs: *mut mythread_condtime,
    mut has_blocked: *mut bool,
) -> lzma_ret {
    let mut ret: lzma_ret = LZMA_OK;
    let mut mythread_i_654: c_uint = 0;
    while if mythread_i_654 != 0 {
        mythread_mutex_unlock(&raw mut (*coder).mutex);
        0 as c_int
    } else {
        mythread_mutex_lock(&raw mut (*coder).mutex);
        1 as c_int
    } != 0
    {
        let mut mythread_j_654: c_uint = 0;
        while mythread_j_654 == 0 {
            loop {
                let out_start: size_t = *out_pos;
                loop {
                    ret = lzma_outq_read(
                        &raw mut (*coder).outq,
                        allocator,
                        out,
                        out_pos,
                        out_size,
                        ::core::ptr::null_mut::<lzma_vli>(),
                        ::core::ptr::null_mut::<lzma_vli>(),
                    );
                    if ret == LZMA_STREAM_END {
                        lzma_outq_enable_partial_output(
                            &raw mut (*coder).outq,
                            Some(
                                worker_enable_partial_update
                                    as unsafe extern "C" fn(*mut c_void) -> (),
                            ),
                        );
                    }
                    if !(ret == LZMA_STREAM_END) {
                        break;
                    }
                }
                if ret != LZMA_OK {
                    break;
                }
                if *out_pos == out_size && *out_pos != out_start {
                    (*coder).out_was_filled = true_0 != 0;
                }
                if (*coder).thread_error != LZMA_OK {
                    if (*coder).fail_fast {
                        ret = (*coder).thread_error;
                        break;
                    } else {
                        (*coder).pending_error = LZMA_PROG_ERROR;
                    }
                }
                if !input_is_possible.is_null()
                    && (*coder)
                        .memlimit_threading
                        .wrapping_sub((*coder).mem_in_use)
                        .wrapping_sub((*coder).outq.mem_in_use)
                        >= (*coder).mem_next_block
                    && lzma_outq_has_buf(&raw mut (*coder).outq) as c_int != 0
                    && ((*coder).threads_initialized < (*coder).threads_max
                        || !(*coder).threads_free.is_null())
                {
                    *input_is_possible = true_0 != 0;
                    break;
                } else {
                    if !waiting_allowed {
                        break;
                    }
                    if lzma_outq_is_empty(&raw mut (*coder).outq) {
                        break;
                    }
                    if lzma_outq_is_readable(&raw mut (*coder).outq) {
                        break;
                    }
                    if !(*coder).thr.is_null()
                        && (*(*coder).thr).partial_update_enabled as c_int != 0
                    {
                        if (*(*(*coder).thr).outbuf).decoder_in_pos == (*(*coder).thr).in_filled {
                            break;
                        }
                    }
                    if (*coder).timeout != 0 as u32 {
                        if !*has_blocked {
                            *has_blocked = true_0 != 0;
                            mythread_condtime_set(
                                wait_abs,
                                &raw mut (*coder).cond,
                                (*coder).timeout,
                            );
                        }
                        if mythread_cond_timedwait(
                            &raw mut (*coder).cond,
                            &raw mut (*coder).mutex,
                            wait_abs,
                        ) != 0 as c_int
                        {
                            ret = LZMA_RET_INTERNAL1;
                            break;
                        }
                    } else {
                        mythread_cond_wait(&raw mut (*coder).cond, &raw mut (*coder).mutex);
                    }
                    if !(ret == LZMA_OK) {
                        break;
                    }
                }
            }
            mythread_j_654 = 1;
        }
        mythread_i_654 = 1;
    }
    if ret != LZMA_OK && ret != LZMA_RET_INTERNAL1 {
        threads_stop(coder);
    }
    return ret;
}
unsafe extern "C" fn decode_block_header(
    mut coder: *mut lzma_stream_coder,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
) -> lzma_ret {
    if *in_pos >= in_size {
        return LZMA_OK;
    }
    if (*coder).pos == 0 as size_t {
        if *in_0.offset(*in_pos as isize) as c_int == INDEX_INDICATOR {
            return LZMA_RET_INTERNAL2;
        }
        (*coder).block_options.header_size = (*in_0.offset(*in_pos as isize) as u32)
            .wrapping_add(1 as u32)
            .wrapping_mul(4 as u32);
    }
    lzma_bufcpy(
        in_0,
        in_pos,
        in_size,
        &raw mut (*coder).buffer as *mut u8,
        &raw mut (*coder).pos,
        (*coder).block_options.header_size as size_t,
    );
    if (*coder).pos < (*coder).block_options.header_size as size_t {
        return LZMA_OK;
    }
    (*coder).pos = 0 as size_t;
    (*coder).block_options.version = 1 as u32;
    (*coder).block_options.filters = &raw mut (*coder).filters as *mut lzma_filter;
    let ret_: lzma_ret = lzma_block_header_decode(
        &raw mut (*coder).block_options,
        allocator,
        &raw mut (*coder).buffer as *mut u8,
    ) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    (*coder).block_options.ignore_check = (*coder).ignore_check as lzma_bool;
    return LZMA_STREAM_END;
}
unsafe extern "C" fn comp_blk_size(mut coder: *const lzma_stream_coder) -> size_t {
    return vli_ceil4((*coder).block_options.compressed_size)
        .wrapping_add(lzma_check_size((*coder).stream_flags.check) as lzma_vli)
        as size_t;
}
unsafe extern "C" fn is_direct_mode_needed(mut size: lzma_vli) -> bool {
    return size == LZMA_VLI_UNKNOWN as lzma_vli
        || size > SIZE_MAX.wrapping_div(3 as c_ulong) as lzma_vli;
}
unsafe extern "C" fn stream_decoder_reset(
    mut coder: *mut lzma_stream_coder,
    mut allocator: *const lzma_allocator,
) -> lzma_ret {
    (*coder).index_hash = lzma_index_hash_init((*coder).index_hash, allocator);
    if (*coder).index_hash.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*coder).sequence = SEQ_STREAM_HEADER;
    (*coder).pos = 0 as size_t;
    return LZMA_OK;
}
unsafe extern "C" fn stream_decode_mt(
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
    let mut wait_abs: mythread_condtime = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut has_blocked: bool = false_0 != 0;
    let waiting_allowed: bool =
        action == LZMA_FINISH || *in_pos == in_size && !(*coder).out_was_filled;
    (*coder).out_was_filled = false_0 != 0;
    loop {
        let mut current_block_239: u64;
        match (*coder).sequence {
            0 => {
                let in_old: size_t = *in_pos;
                lzma_bufcpy(
                    in_0,
                    in_pos,
                    in_size,
                    &raw mut (*coder).buffer as *mut u8,
                    &raw mut (*coder).pos,
                    LZMA_STREAM_HEADER_SIZE as size_t,
                );
                (*coder).progress_in = (*coder)
                    .progress_in
                    .wrapping_add((*in_pos).wrapping_sub(in_old) as u64);
                if (*coder).pos < LZMA_STREAM_HEADER_SIZE as size_t {
                    return LZMA_OK;
                }
                (*coder).pos = 0 as size_t;
                let ret: lzma_ret = lzma_stream_header_decode(
                    &raw mut (*coder).stream_flags,
                    &raw mut (*coder).buffer as *mut u8,
                ) as lzma_ret;
                if ret != LZMA_OK {
                    return (if ret == LZMA_FORMAT_ERROR && !(*coder).first_stream {
                        LZMA_DATA_ERROR
                    } else {
                        ret
                    }) as lzma_ret;
                }
                (*coder).first_stream = false_0 != 0;
                (*coder).block_options.check = (*coder).stream_flags.check;
                (*coder).sequence = SEQ_BLOCK_HEADER;
                if (*coder).tell_no_check as c_int != 0
                    && (*coder).stream_flags.check == LZMA_CHECK_NONE
                {
                    return LZMA_NO_CHECK;
                }
                if (*coder).tell_unsupported_check as c_int != 0
                    && lzma_check_is_supported((*coder).stream_flags.check) == 0
                {
                    return LZMA_UNSUPPORTED_CHECK;
                }
                if (*coder).tell_any_check {
                    return LZMA_GET_CHECK;
                }
                current_block_239 = 7149356873433890176;
            }
            1 => {
                current_block_239 = 7149356873433890176;
            }
            2 => {
                current_block_239 = 3123434771885419771;
            }
            3 => {
                current_block_239 = 11441799814184323368;
            }
            4 => {
                current_block_239 = 7728257318064351663;
            }
            5 => {
                let ret__3: lzma_ret = read_output_and_wait(
                    coder,
                    allocator,
                    out,
                    out_pos,
                    out_size,
                    ::core::ptr::null_mut::<bool>(),
                    1 as c_int != 0,
                    &raw mut wait_abs,
                    &raw mut has_blocked,
                ) as lzma_ret;
                if ret__3 != LZMA_OK {
                    return ret__3;
                }
                if !lzma_outq_is_empty(&raw mut (*coder).outq) {
                    return LZMA_OK;
                }
                lzma_outq_clear_cache(&raw mut (*coder).outq, allocator);
                threads_end(coder, allocator);
                let ret_3: lzma_ret = lzma_block_decoder_init(
                    &raw mut (*coder).block_decoder,
                    allocator,
                    &raw mut (*coder).block_options,
                ) as lzma_ret;
                lzma_filters_free(&raw mut (*coder).filters as *mut lzma_filter, allocator);
                (*coder).block_options.filters = ::core::ptr::null_mut::<lzma_filter>();
                if ret_3 != LZMA_OK {
                    return ret_3;
                }
                (*coder).mem_direct_mode = (*coder).mem_next_filters;
                (*coder).sequence = SEQ_BLOCK_DIRECT_RUN;
                current_block_239 = 7173345243791314703;
            }
            6 => {
                current_block_239 = 7173345243791314703;
            }
            7 => {
                let ret__5: lzma_ret = read_output_and_wait(
                    coder,
                    allocator,
                    out,
                    out_pos,
                    out_size,
                    ::core::ptr::null_mut::<bool>(),
                    1 as c_int != 0,
                    &raw mut wait_abs,
                    &raw mut has_blocked,
                ) as lzma_ret;
                if ret__5 != LZMA_OK {
                    return ret__5;
                }
                if !lzma_outq_is_empty(&raw mut (*coder).outq) {
                    return LZMA_OK;
                }
                (*coder).sequence = SEQ_INDEX_DECODE;
                current_block_239 = 13812071707085482240;
            }
            8 => {
                current_block_239 = 13812071707085482240;
            }
            9 => {
                current_block_239 = 15174413556390356007;
            }
            10 => {
                current_block_239 = 17073193239823527980;
            }
            11 => {
                if !(*coder).fail_fast {
                    let ret__8: lzma_ret = read_output_and_wait(
                        coder,
                        allocator,
                        out,
                        out_pos,
                        out_size,
                        ::core::ptr::null_mut::<bool>(),
                        1 as c_int != 0,
                        &raw mut wait_abs,
                        &raw mut has_blocked,
                    ) as lzma_ret;
                    if ret__8 != LZMA_OK {
                        return ret__8;
                    }
                    if !lzma_outq_is_empty(&raw mut (*coder).outq) {
                        return LZMA_OK;
                    }
                }
                return (*coder).pending_error;
            }
            _ => return LZMA_PROG_ERROR,
        }
        match current_block_239 {
            7173345243791314703 => {
                let in_old_1: size_t = *in_pos;
                let out_old: size_t = *out_pos;
                let ret_4: lzma_ret = (*coder)
                    .block_decoder
                    .code
                    .expect("non-null function pointer")(
                    (*coder).block_decoder.coder,
                    allocator,
                    in_0,
                    in_pos,
                    in_size,
                    out,
                    out_pos,
                    out_size,
                    action,
                ) as lzma_ret;
                (*coder).progress_in = (*coder)
                    .progress_in
                    .wrapping_add((*in_pos).wrapping_sub(in_old_1) as u64);
                (*coder).progress_out = (*coder)
                    .progress_out
                    .wrapping_add((*out_pos).wrapping_sub(out_old) as u64);
                if ret_4 != LZMA_STREAM_END {
                    return ret_4;
                }
                let ret__4: lzma_ret = lzma_index_hash_append(
                    (*coder).index_hash,
                    lzma_block_unpadded_size(&raw mut (*coder).block_options),
                    (*coder).block_options.uncompressed_size,
                ) as lzma_ret;
                if ret__4 != LZMA_OK {
                    return ret__4;
                }
                (*coder).sequence = SEQ_BLOCK_HEADER;
                current_block_239 = 11639917216603986996;
            }
            7149356873433890176 => {
                let in_old_0: size_t = *in_pos;
                let ret_0: lzma_ret =
                    decode_block_header(coder, allocator, in_0, in_pos, in_size) as lzma_ret;
                (*coder).progress_in = (*coder)
                    .progress_in
                    .wrapping_add((*in_pos).wrapping_sub(in_old_0) as u64);
                if ret_0 == LZMA_OK {
                    if action == LZMA_FINISH && (*coder).fail_fast as c_int != 0 {
                        threads_stop(coder);
                        return LZMA_DATA_ERROR;
                    }
                    let ret_: lzma_ret = read_output_and_wait(
                        coder,
                        allocator,
                        out,
                        out_pos,
                        out_size,
                        ::core::ptr::null_mut::<bool>(),
                        waiting_allowed,
                        &raw mut wait_abs,
                        &raw mut has_blocked,
                    ) as lzma_ret;
                    if ret_ != LZMA_OK {
                        return ret_;
                    }
                    if (*coder).pending_error != LZMA_OK {
                        (*coder).sequence = SEQ_ERROR;
                    } else {
                        return LZMA_OK;
                    }
                    current_block_239 = 11639917216603986996;
                } else if ret_0 == LZMA_RET_INTERNAL2 {
                    (*coder).sequence = SEQ_INDEX_WAIT_OUTPUT;
                    current_block_239 = 11639917216603986996;
                } else if ret_0 != LZMA_STREAM_END {
                    (*coder).pending_error = ret_0;
                    (*coder).sequence = SEQ_ERROR;
                    current_block_239 = 11639917216603986996;
                } else {
                    (*coder).mem_next_filters =
                        lzma_raw_decoder_memusage(&raw mut (*coder).filters as *mut lzma_filter);
                    if (*coder).mem_next_filters == UINT64_MAX as u64 {
                        (*coder).pending_error = LZMA_OPTIONS_ERROR;
                        (*coder).sequence = SEQ_ERROR;
                        current_block_239 = 11639917216603986996;
                    } else {
                        (*coder).sequence = SEQ_BLOCK_INIT;
                        current_block_239 = 3123434771885419771;
                    }
                }
            }
            13812071707085482240 => {
                if *in_pos >= in_size {
                    return LZMA_OK;
                }
                let in_old_2: size_t = *in_pos;
                let ret_5: lzma_ret =
                    lzma_index_hash_decode((*coder).index_hash, in_0, in_pos, in_size) as lzma_ret;
                (*coder).progress_in = (*coder)
                    .progress_in
                    .wrapping_add((*in_pos).wrapping_sub(in_old_2) as u64);
                if ret_5 != LZMA_STREAM_END {
                    return ret_5;
                }
                (*coder).sequence = SEQ_STREAM_FOOTER;
                current_block_239 = 15174413556390356007;
            }
            _ => {}
        }
        match current_block_239 {
            3123434771885419771 => {
                if (*coder).mem_next_filters > (*coder).memlimit_stop {
                    let ret__0: lzma_ret = read_output_and_wait(
                        coder,
                        allocator,
                        out,
                        out_pos,
                        out_size,
                        ::core::ptr::null_mut::<bool>(),
                        1 as c_int != 0,
                        &raw mut wait_abs,
                        &raw mut has_blocked,
                    ) as lzma_ret;
                    if ret__0 != LZMA_OK {
                        return ret__0;
                    }
                    if !lzma_outq_is_empty(&raw mut (*coder).outq) {
                        return LZMA_OK;
                    }
                    return LZMA_MEMLIMIT_ERROR;
                }
                if is_direct_mode_needed((*coder).block_options.compressed_size) as c_int != 0
                    || is_direct_mode_needed((*coder).block_options.uncompressed_size) as c_int != 0
                {
                    (*coder).sequence = SEQ_BLOCK_DIRECT_INIT;
                    current_block_239 = 11639917216603986996;
                } else {
                    (*coder).mem_next_in = comp_blk_size(coder) as u64;
                    let mem_buffers: u64 =
                        (*coder).mem_next_in.wrapping_add(lzma_outq_outbuf_memusage(
                            (*coder).block_options.uncompressed_size as size_t,
                        ) as u64);
                    if (UINT64_MAX as u64).wrapping_sub(mem_buffers) < (*coder).mem_next_filters {
                        (*coder).sequence = SEQ_BLOCK_DIRECT_INIT;
                        current_block_239 = 11639917216603986996;
                    } else {
                        (*coder).mem_next_block =
                            (*coder).mem_next_filters.wrapping_add(mem_buffers);
                        if (*coder).mem_next_block > (*coder).memlimit_threading {
                            (*coder).sequence = SEQ_BLOCK_DIRECT_INIT;
                            current_block_239 = 11639917216603986996;
                        } else {
                            lzma_next_end(&raw mut (*coder).block_decoder, allocator);
                            (*coder).mem_direct_mode = 0 as u64;
                            let ret_1: lzma_ret = lzma_index_hash_append(
                                (*coder).index_hash,
                                lzma_block_unpadded_size(&raw mut (*coder).block_options),
                                (*coder).block_options.uncompressed_size,
                            ) as lzma_ret;
                            if ret_1 != LZMA_OK {
                                (*coder).pending_error = ret_1;
                                (*coder).sequence = SEQ_ERROR;
                                current_block_239 = 11639917216603986996;
                            } else {
                                (*coder).sequence = SEQ_BLOCK_THR_INIT;
                                current_block_239 = 11441799814184323368;
                            }
                        }
                    }
                }
            }
            15174413556390356007 => {
                let in_old_3: size_t = *in_pos;
                lzma_bufcpy(
                    in_0,
                    in_pos,
                    in_size,
                    &raw mut (*coder).buffer as *mut u8,
                    &raw mut (*coder).pos,
                    LZMA_STREAM_HEADER_SIZE as size_t,
                );
                (*coder).progress_in = (*coder)
                    .progress_in
                    .wrapping_add((*in_pos).wrapping_sub(in_old_3) as u64);
                if (*coder).pos < LZMA_STREAM_HEADER_SIZE as size_t {
                    return LZMA_OK;
                }
                (*coder).pos = 0 as size_t;
                let mut footer_flags: lzma_stream_flags = lzma_stream_flags {
                    version: 0,
                    backward_size: 0,
                    check: LZMA_CHECK_NONE,
                    reserved_enum1: LZMA_RESERVED_ENUM,
                    reserved_enum2: LZMA_RESERVED_ENUM,
                    reserved_enum3: LZMA_RESERVED_ENUM,
                    reserved_enum4: LZMA_RESERVED_ENUM,
                    reserved_bool1: 0,
                    reserved_bool2: 0,
                    reserved_bool3: 0,
                    reserved_bool4: 0,
                    reserved_bool5: 0,
                    reserved_bool6: 0,
                    reserved_bool7: 0,
                    reserved_bool8: 0,
                    reserved_int1: 0,
                    reserved_int2: 0,
                };
                let ret_6: lzma_ret = lzma_stream_footer_decode(
                    &raw mut footer_flags,
                    &raw mut (*coder).buffer as *mut u8,
                ) as lzma_ret;
                if ret_6 != LZMA_OK {
                    return (if ret_6 == LZMA_FORMAT_ERROR {
                        LZMA_DATA_ERROR
                    } else {
                        ret_6
                    }) as lzma_ret;
                }
                if lzma_index_hash_size((*coder).index_hash) != footer_flags.backward_size {
                    return LZMA_DATA_ERROR;
                }
                let ret__6: lzma_ret = lzma_stream_flags_compare(
                    &raw mut (*coder).stream_flags,
                    &raw mut footer_flags,
                ) as lzma_ret;
                if ret__6 != LZMA_OK {
                    return ret__6;
                }
                if !(*coder).concatenated {
                    return LZMA_STREAM_END;
                }
                (*coder).sequence = SEQ_STREAM_PADDING;
                current_block_239 = 17073193239823527980;
            }
            _ => {}
        }
        match current_block_239 {
            17073193239823527980 => {
                loop {
                    if *in_pos >= in_size {
                        if action != LZMA_FINISH {
                            return LZMA_OK;
                        }
                        return (if (*coder).pos == 0 as size_t {
                            LZMA_STREAM_END as c_int
                        } else {
                            LZMA_DATA_ERROR as c_int
                        }) as lzma_ret;
                    }
                    if *in_0.offset(*in_pos as isize) as c_int != 0 as c_int {
                        break;
                    }
                    *in_pos = (*in_pos).wrapping_add(1);
                    (*coder).progress_in = (*coder).progress_in.wrapping_add(1);
                    (*coder).pos = (*coder).pos.wrapping_add(1 as size_t) & 3 as size_t;
                }
                if (*coder).pos != 0 as size_t {
                    *in_pos = (*in_pos).wrapping_add(1);
                    (*coder).progress_in = (*coder).progress_in.wrapping_add(1);
                    return LZMA_DATA_ERROR;
                }
                let ret__7: lzma_ret = stream_decoder_reset(coder, allocator) as lzma_ret;
                if ret__7 != LZMA_OK {
                    return ret__7;
                }
                current_block_239 = 11639917216603986996;
            }
            11441799814184323368 => {
                let mut block_can_start: bool = false_0 != 0;
                let ret__1: lzma_ret = read_output_and_wait(
                    coder,
                    allocator,
                    out,
                    out_pos,
                    out_size,
                    &raw mut block_can_start,
                    1 as c_int != 0,
                    &raw mut wait_abs,
                    &raw mut has_blocked,
                ) as lzma_ret;
                if ret__1 != LZMA_OK {
                    return ret__1;
                }
                if (*coder).pending_error != LZMA_OK {
                    (*coder).sequence = SEQ_ERROR;
                    current_block_239 = 11639917216603986996;
                } else {
                    if !block_can_start {
                        return LZMA_OK;
                    }
                    let mut mem_in_use: u64 = 0 as u64;
                    let mut mem_cached: u64 = 0 as u64;
                    let mut thr: *mut worker_thread = ::core::ptr::null_mut::<worker_thread>();
                    let mut mythread_i_1347: c_uint = 0;
                    while if mythread_i_1347 != 0 {
                        mythread_mutex_unlock(&raw mut (*coder).mutex);
                        0 as c_int
                    } else {
                        mythread_mutex_lock(&raw mut (*coder).mutex);
                        1 as c_int
                    } != 0
                    {
                        let mut mythread_j_1347: c_uint = 0;
                        while mythread_j_1347 == 0 {
                            mem_in_use = (*coder).mem_in_use;
                            mem_cached = (*coder).mem_cached;
                            thr = (*coder).threads_free;
                            mythread_j_1347 = 1;
                        }
                        mythread_i_1347 = 1;
                    }
                    let mem_max: u64 = (*coder)
                        .memlimit_threading
                        .wrapping_sub((*coder).mem_next_block);
                    if mem_in_use
                        .wrapping_add(mem_cached)
                        .wrapping_add((*coder).outq.mem_allocated)
                        > mem_max
                    {
                        lzma_outq_clear_cache2(
                            &raw mut (*coder).outq,
                            allocator,
                            (*coder).block_options.uncompressed_size as size_t,
                        );
                    }
                    let mut mem_freed: u64 = 0 as u64;
                    if !thr.is_null()
                        && mem_in_use
                            .wrapping_add(mem_cached)
                            .wrapping_add((*coder).outq.mem_in_use)
                            > mem_max
                    {
                        if (*thr).mem_filters <= (*coder).mem_next_filters {
                            thr = (*thr).next;
                        }
                        while !thr.is_null() {
                            lzma_next_end(&raw mut (*thr).block_decoder, allocator);
                            mem_freed = mem_freed.wrapping_add((*thr).mem_filters);
                            (*thr).mem_filters = 0 as u64;
                            thr = (*thr).next;
                        }
                    }
                    let mut mythread_i_1410: c_uint = 0;
                    while if mythread_i_1410 != 0 {
                        mythread_mutex_unlock(&raw mut (*coder).mutex);
                        0 as c_int
                    } else {
                        mythread_mutex_lock(&raw mut (*coder).mutex);
                        1 as c_int
                    } != 0
                    {
                        let mut mythread_j_1410: c_uint = 0;
                        while mythread_j_1410 == 0 {
                            (*coder).mem_cached = (*coder).mem_cached.wrapping_sub(mem_freed);
                            (*coder).mem_in_use = (*coder).mem_in_use.wrapping_add(
                                (*coder).mem_next_in.wrapping_add((*coder).mem_next_filters),
                            );
                            mythread_j_1410 = 1;
                        }
                        mythread_i_1410 = 1;
                    }
                    let mut ret_2: lzma_ret = lzma_outq_prealloc_buf(
                        &raw mut (*coder).outq,
                        allocator,
                        (*coder).block_options.uncompressed_size as size_t,
                    );
                    if ret_2 != LZMA_OK {
                        threads_stop(coder);
                        return ret_2;
                    }
                    ret_2 = get_thread(coder, allocator);
                    if ret_2 != LZMA_OK {
                        threads_stop(coder);
                        return ret_2;
                    }
                    (*(*coder).thr).mem_filters = (*coder).mem_next_filters;
                    (*(*coder).thr).block_options = (*coder).block_options;
                    ret_2 = lzma_block_decoder_init(
                        &raw mut (*(*coder).thr).block_decoder,
                        allocator,
                        &raw mut (*(*coder).thr).block_options,
                    );
                    lzma_filters_free(&raw mut (*coder).filters as *mut lzma_filter, allocator);
                    (*(*coder).thr).block_options.filters = ::core::ptr::null_mut::<lzma_filter>();
                    if ret_2 != LZMA_OK {
                        (*coder).pending_error = ret_2;
                        (*coder).sequence = SEQ_ERROR;
                        current_block_239 = 11639917216603986996;
                    } else {
                        (*(*coder).thr).in_size = (*coder).mem_next_in as size_t;
                        (*(*coder).thr).in_0 =
                            lzma_alloc((*(*coder).thr).in_size, allocator) as *mut u8;
                        if (*(*coder).thr).in_0.is_null() {
                            threads_stop(coder);
                            return LZMA_MEM_ERROR;
                        }
                        (*(*coder).thr).outbuf =
                            lzma_outq_get_buf(&raw mut (*coder).outq, (*coder).thr as *mut c_void);
                        let mut mythread_i_1478: c_uint = 0;
                        while if mythread_i_1478 != 0 {
                            mythread_mutex_unlock(&raw mut (*(*coder).thr).mutex);
                            0 as c_int
                        } else {
                            mythread_mutex_lock(&raw mut (*(*coder).thr).mutex);
                            1 as c_int
                        } != 0
                        {
                            let mut mythread_j_1478: c_uint = 0;
                            while mythread_j_1478 == 0 {
                                (*(*coder).thr).state = THR_RUN;
                                mythread_cond_signal(&raw mut (*(*coder).thr).cond);
                                mythread_j_1478 = 1;
                            }
                            mythread_i_1478 = 1;
                        }
                        let mut mythread_i_1486: c_uint = 0;
                        while if mythread_i_1486 != 0 {
                            mythread_mutex_unlock(&raw mut (*coder).mutex);
                            0 as c_int
                        } else {
                            mythread_mutex_lock(&raw mut (*coder).mutex);
                            1 as c_int
                        } != 0
                        {
                            let mut mythread_j_1486: c_uint = 0;
                            while mythread_j_1486 == 0 {
                                lzma_outq_enable_partial_output(
                                    &raw mut (*coder).outq,
                                    Some(
                                        worker_enable_partial_update
                                            as unsafe extern "C" fn(*mut c_void) -> (),
                                    ),
                                );
                                mythread_j_1486 = 1;
                            }
                            mythread_i_1486 = 1;
                        }
                        (*coder).sequence = SEQ_BLOCK_THR_RUN;
                        current_block_239 = 7728257318064351663;
                    }
                }
            }
            _ => {}
        }
        match current_block_239 {
            7728257318064351663 => {
                if action == LZMA_FINISH && (*coder).fail_fast as c_int != 0 {
                    let in_avail: size_t = in_size.wrapping_sub(*in_pos);
                    let in_needed: size_t = (*(*coder).thr)
                        .in_size
                        .wrapping_sub((*(*coder).thr).in_filled);
                    if in_avail < in_needed {
                        threads_stop(coder);
                        return LZMA_DATA_ERROR;
                    }
                }
                let mut cur_in_filled: size_t = (*(*coder).thr).in_filled;
                lzma_bufcpy(
                    in_0,
                    in_pos,
                    in_size,
                    (*(*coder).thr).in_0,
                    &raw mut cur_in_filled,
                    (*(*coder).thr).in_size,
                );
                let mut mythread_i_1517: c_uint = 0;
                while if mythread_i_1517 != 0 {
                    mythread_mutex_unlock(&raw mut (*(*coder).thr).mutex);
                    0 as c_int
                } else {
                    mythread_mutex_lock(&raw mut (*(*coder).thr).mutex);
                    1 as c_int
                } != 0
                {
                    let mut mythread_j_1517: c_uint = 0;
                    while mythread_j_1517 == 0 {
                        (*(*coder).thr).in_filled = cur_in_filled;
                        mythread_cond_signal(&raw mut (*(*coder).thr).cond);
                        mythread_j_1517 = 1;
                    }
                    mythread_i_1517 = 1;
                }
                let ret__2: lzma_ret = read_output_and_wait(
                    coder,
                    allocator,
                    out,
                    out_pos,
                    out_size,
                    ::core::ptr::null_mut::<bool>(),
                    waiting_allowed as c_int != 0 && *in_pos == in_size,
                    &raw mut wait_abs,
                    &raw mut has_blocked,
                ) as lzma_ret;
                if ret__2 != LZMA_OK {
                    return ret__2;
                }
                if (*coder).pending_error != LZMA_OK {
                    (*coder).sequence = SEQ_ERROR;
                } else {
                    if (*(*coder).thr).in_filled < (*(*coder).thr).in_size {
                        return LZMA_OK;
                    }
                    (*coder).thr = ::core::ptr::null_mut::<worker_thread>();
                    (*coder).sequence = SEQ_BLOCK_HEADER;
                }
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn stream_decoder_mt_end(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    threads_end(coder, allocator);
    lzma_outq_end(&raw mut (*coder).outq, allocator);
    lzma_next_end(&raw mut (*coder).block_decoder, allocator);
    lzma_filters_free(&raw mut (*coder).filters as *mut lzma_filter, allocator);
    lzma_index_hash_end((*coder).index_hash, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn stream_decoder_mt_get_check(mut coder_ptr: *const c_void) -> lzma_check {
    let mut coder: *const lzma_stream_coder = coder_ptr as *const lzma_stream_coder;
    return (*coder).stream_flags.check;
}
unsafe extern "C" fn stream_decoder_mt_memconfig(
    mut coder_ptr: *mut c_void,
    mut memusage: *mut u64,
    mut old_memlimit: *mut u64,
    mut new_memlimit: u64,
) -> lzma_ret {
    let mut coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    let mut mythread_i_1829: c_uint = 0;
    while if mythread_i_1829 != 0 {
        mythread_mutex_unlock(&raw mut (*coder).mutex);
        0 as c_int
    } else {
        mythread_mutex_lock(&raw mut (*coder).mutex);
        1 as c_int
    } != 0
    {
        let mut mythread_j_1829: c_uint = 0;
        while mythread_j_1829 == 0 {
            *memusage = (*coder)
                .mem_direct_mode
                .wrapping_add((*coder).mem_in_use)
                .wrapping_add((*coder).mem_cached)
                .wrapping_add((*coder).outq.mem_allocated);
            mythread_j_1829 = 1;
        }
        mythread_i_1829 = 1;
    }
    if *memusage < LZMA_MEMUSAGE_BASE as u64 {
        *memusage = LZMA_MEMUSAGE_BASE as u64;
    }
    *old_memlimit = (*coder).memlimit_stop;
    if new_memlimit != 0 as u64 {
        if new_memlimit < *memusage {
            return LZMA_MEMLIMIT_ERROR;
        }
        (*coder).memlimit_stop = new_memlimit;
    }
    return LZMA_OK;
}
unsafe extern "C" fn stream_decoder_mt_get_progress(
    mut coder_ptr: *mut c_void,
    mut progress_in: *mut u64,
    mut progress_out: *mut u64,
) {
    let mut coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    let mut mythread_i_1862: c_uint = 0;
    while if mythread_i_1862 != 0 {
        mythread_mutex_unlock(&raw mut (*coder).mutex);
        0 as c_int
    } else {
        mythread_mutex_lock(&raw mut (*coder).mutex);
        1 as c_int
    } != 0
    {
        let mut mythread_j_1862: c_uint = 0;
        while mythread_j_1862 == 0 {
            *progress_in = (*coder).progress_in;
            *progress_out = (*coder).progress_out;
            let mut i: size_t = 0 as size_t;
            while i < (*coder).threads_initialized as size_t {
                let mut mythread_i_1867: c_uint = 0;
                while if mythread_i_1867 != 0 {
                    mythread_mutex_unlock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
                    0 as c_int
                } else {
                    mythread_mutex_lock(&raw mut (*(*coder).threads.offset(i as isize)).mutex);
                    1 as c_int
                } != 0
                {
                    let mut mythread_j_1867: c_uint = 0;
                    while mythread_j_1867 == 0 {
                        *progress_in = (*progress_in).wrapping_add(
                            (*(*coder).threads.offset(i as isize)).progress_in as u64,
                        );
                        *progress_out = (*progress_out).wrapping_add(
                            (*(*coder).threads.offset(i as isize)).progress_out as u64,
                        );
                        mythread_j_1867 = 1;
                    }
                    mythread_i_1867 = 1;
                }
                i = i.wrapping_add(1);
            }
            mythread_j_1862 = 1;
        }
        mythread_i_1862 = 1;
    }
}
unsafe extern "C" fn stream_decoder_mt_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut options: *const lzma_mt,
) -> lzma_ret {
    let mut coder: *mut lzma_stream_coder = ::core::ptr::null_mut::<lzma_stream_coder>();
    if (*options).threads == 0 as u32 || (*options).threads > LZMA_THREADS_MAX as u32 {
        return LZMA_OPTIONS_ERROR;
    }
    if (*options).flags & !(LZMA_SUPPORTED_FLAGS as u32) != 0 {
        return LZMA_OPTIONS_ERROR;
    }
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
        stream_decoder_mt_init
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
        stream_decoder_mt_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_mt,
            ) -> lzma_ret,
    ));
    coder = (*next).coder as *mut lzma_stream_coder;
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
            return LZMA_MEM_ERROR;
        }
        if mythread_cond_init(&raw mut (*coder).cond) != 0 {
            mythread_mutex_destroy(&raw mut (*coder).mutex);
            lzma_free(coder as *mut c_void, allocator);
            return LZMA_MEM_ERROR;
        }
        (*next).code = Some(
            stream_decode_mt
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
            stream_decoder_mt_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).get_check =
            Some(stream_decoder_mt_get_check as unsafe extern "C" fn(*const c_void) -> lzma_check)
                as Option<unsafe extern "C" fn(*const c_void) -> lzma_check>;
        (*next).memconfig = Some(
            stream_decoder_mt_memconfig
                as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret,
        )
            as Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret>;
        (*next).get_progress = Some(
            stream_decoder_mt_get_progress
                as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64) -> (),
        )
            as Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64) -> ()>;
        (*coder).filters[0].id = LZMA_VLI_UNKNOWN as lzma_vli;
        memset(
            &raw mut (*coder).outq as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<lzma_outq>() as size_t,
        );
        (*coder).block_decoder = lzma_next_coder_s {
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
        (*coder).mem_direct_mode = 0 as u64;
        (*coder).index_hash = ::core::ptr::null_mut::<lzma_index_hash>();
        (*coder).threads = ::core::ptr::null_mut::<worker_thread>();
        (*coder).threads_free = ::core::ptr::null_mut::<worker_thread>();
        (*coder).threads_initialized = 0 as u32;
    }
    lzma_filters_free(&raw mut (*coder).filters as *mut lzma_filter, allocator);
    threads_end(coder, allocator);
    (*coder).mem_in_use = 0 as u64;
    (*coder).mem_cached = 0 as u64;
    (*coder).mem_next_block = 0 as u64;
    (*coder).progress_in = 0 as u64;
    (*coder).progress_out = 0 as u64;
    (*coder).sequence = SEQ_STREAM_HEADER;
    (*coder).thread_error = LZMA_OK;
    (*coder).pending_error = LZMA_OK;
    (*coder).thr = ::core::ptr::null_mut::<worker_thread>();
    (*coder).timeout = (*options).timeout;
    (*coder).memlimit_threading = if 1 as u64 > (*options).memlimit_threading {
        1 as u64
    } else {
        (*options).memlimit_threading
    };
    (*coder).memlimit_stop = if 1 as u64 > (*options).memlimit_stop {
        1 as u64
    } else {
        (*options).memlimit_stop
    };
    if (*coder).memlimit_threading > (*coder).memlimit_stop {
        (*coder).memlimit_threading = (*coder).memlimit_stop;
    }
    (*coder).tell_no_check = (*options).flags & LZMA_TELL_NO_CHECK as u32 != 0 as u32;
    (*coder).tell_unsupported_check =
        (*options).flags & LZMA_TELL_UNSUPPORTED_CHECK as u32 != 0 as u32;
    (*coder).tell_any_check = (*options).flags & LZMA_TELL_ANY_CHECK as u32 != 0 as u32;
    (*coder).ignore_check = (*options).flags & LZMA_IGNORE_CHECK as u32 != 0 as u32;
    (*coder).concatenated = (*options).flags & LZMA_CONCATENATED as u32 != 0 as u32;
    (*coder).fail_fast = (*options).flags & LZMA_FAIL_FAST as u32 != 0 as u32;
    (*coder).first_stream = true_0 != 0;
    (*coder).out_was_filled = false_0 != 0;
    (*coder).pos = 0 as size_t;
    (*coder).threads_max = (*options).threads;
    let ret_: lzma_ret =
        lzma_outq_init(&raw mut (*coder).outq, allocator, (*coder).threads_max) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    return stream_decoder_reset(coder, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_decoder_mt(
    mut strm: *mut lzma_stream,
    mut options: *const lzma_mt,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = stream_decoder_mt_init(
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
