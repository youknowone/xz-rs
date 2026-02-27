use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulong, c_ulonglong, c_void};
extern "C" {
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_bufcpy(
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> size_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, size_t, size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    pub opaque: *mut c_void,
}
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
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const UINTPTR_MAX: c_ulong = uintptr_t::MAX as c_ulong;
pub const SIZE_MAX: c_ulong = UINTPTR_MAX;
pub const false_0: c_int = 0 as c_int;
pub const LZMA_THREADS_MAX: c_int = 16384 as c_int;
#[inline]
unsafe extern "C" fn lzma_outq_outbuf_memusage(mut buf_size: size_t) -> u64 {
    return (::core::mem::size_of::<lzma_outbuf>() as usize).wrapping_add(buf_size as usize) as u64;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_memusage(mut buf_size_max: u64, mut threads: u32) -> u64 {
    let limit: u64 = (UINT64_MAX as u64)
        .wrapping_div((2 as c_int * 16384 as c_int) as u64)
        .wrapping_div(2 as u64);
    if threads > LZMA_THREADS_MAX as u32 || buf_size_max > limit {
        return UINT64_MAX as u64;
    }
    return ((2 as u32).wrapping_mul(threads) as u64)
        .wrapping_mul(lzma_outq_outbuf_memusage(buf_size_max as size_t));
}
unsafe extern "C" fn move_head_to_cache(
    mut outq: *mut lzma_outq,
    mut allocator: *const lzma_allocator,
) {
    let mut buf: *mut lzma_outbuf = (*outq).head;
    (*outq).head = (*buf).next;
    if (*outq).head.is_null() {
        (*outq).tail = ::core::ptr::null_mut::<lzma_outbuf>();
    }
    if !(*outq).cache.is_null() && (*(*outq).cache).allocated != (*buf).allocated {
        lzma_outq_clear_cache(outq, allocator);
    }
    (*buf).next = (*outq).cache;
    (*outq).cache = buf;
    (*outq).bufs_in_use = (*outq).bufs_in_use.wrapping_sub(1);
    (*outq).mem_in_use = (*outq)
        .mem_in_use
        .wrapping_sub(lzma_outq_outbuf_memusage((*buf).allocated));
}
unsafe extern "C" fn free_one_cached_buffer(
    mut outq: *mut lzma_outq,
    mut allocator: *const lzma_allocator,
) {
    let mut buf: *mut lzma_outbuf = (*outq).cache;
    (*outq).cache = (*buf).next;
    (*outq).bufs_allocated = (*outq).bufs_allocated.wrapping_sub(1);
    (*outq).mem_allocated = (*outq)
        .mem_allocated
        .wrapping_sub(lzma_outq_outbuf_memusage((*buf).allocated));
    lzma_free(buf as *mut c_void, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_clear_cache(
    mut outq: *mut lzma_outq,
    mut allocator: *const lzma_allocator,
) {
    while !(*outq).cache.is_null() {
        free_one_cached_buffer(outq, allocator);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_clear_cache2(
    mut outq: *mut lzma_outq,
    mut allocator: *const lzma_allocator,
    mut keep_size: size_t,
) {
    if (*outq).cache.is_null() {
        return;
    }
    while !(*(*outq).cache).next.is_null() {
        free_one_cached_buffer(outq, allocator);
    }
    if (*(*outq).cache).allocated != keep_size {
        free_one_cached_buffer(outq, allocator);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_init(
    mut outq: *mut lzma_outq,
    mut allocator: *const lzma_allocator,
    mut threads: u32,
) -> lzma_ret {
    if threads > LZMA_THREADS_MAX as u32 {
        return LZMA_OPTIONS_ERROR;
    }
    let bufs_limit: u32 = (2 as u32).wrapping_mul(threads);
    while !(*outq).head.is_null() {
        move_head_to_cache(outq, allocator);
    }
    while bufs_limit < (*outq).bufs_allocated {
        free_one_cached_buffer(outq, allocator);
    }
    (*outq).bufs_limit = bufs_limit;
    (*outq).read_pos = 0 as size_t;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_end(
    mut outq: *mut lzma_outq,
    mut allocator: *const lzma_allocator,
) {
    while !(*outq).head.is_null() {
        move_head_to_cache(outq, allocator);
    }
    lzma_outq_clear_cache(outq, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_prealloc_buf(
    mut outq: *mut lzma_outq,
    mut allocator: *const lzma_allocator,
    mut size: size_t,
) -> lzma_ret {
    if !(*outq).cache.is_null() && (*(*outq).cache).allocated == size {
        return LZMA_OK;
    }
    if size > (SIZE_MAX as usize).wrapping_sub(::core::mem::size_of::<lzma_outbuf>() as usize) {
        return LZMA_MEM_ERROR;
    }
    let alloc_size: size_t = lzma_outq_outbuf_memusage(size) as size_t;
    lzma_outq_clear_cache(outq, allocator);
    (*outq).cache = lzma_alloc(alloc_size, allocator) as *mut lzma_outbuf;
    if (*outq).cache.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*(*outq).cache).next = ::core::ptr::null_mut::<lzma_outbuf>();
    (*(*outq).cache).allocated = size;
    (*outq).bufs_allocated = (*outq).bufs_allocated.wrapping_add(1);
    (*outq).mem_allocated = (*outq).mem_allocated.wrapping_add(alloc_size as u64);
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_get_buf(
    mut outq: *mut lzma_outq,
    mut worker: *mut c_void,
) -> *mut lzma_outbuf {
    let mut buf: *mut lzma_outbuf = (*outq).cache;
    (*outq).cache = (*buf).next;
    (*buf).next = ::core::ptr::null_mut::<lzma_outbuf>();
    if !(*outq).tail.is_null() {
        (*(*outq).tail).next = buf;
    } else {
        (*outq).head = buf;
    }
    (*outq).tail = buf;
    (*buf).worker = worker;
    (*buf).finished = false_0 != 0;
    (*buf).finish_ret = LZMA_STREAM_END;
    (*buf).pos = 0 as size_t;
    (*buf).decoder_in_pos = 0 as size_t;
    (*buf).unpadded_size = 0 as lzma_vli;
    (*buf).uncompressed_size = 0 as lzma_vli;
    (*outq).bufs_in_use = (*outq).bufs_in_use.wrapping_add(1);
    (*outq).mem_in_use = (*outq)
        .mem_in_use
        .wrapping_add(lzma_outq_outbuf_memusage((*buf).allocated));
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_is_readable(mut outq: *const lzma_outq) -> bool {
    if (*outq).head.is_null() {
        return false_0 != 0;
    }
    return (*outq).read_pos < (*(*outq).head).pos || (*(*outq).head).finished as c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_read(
    mut outq: *mut lzma_outq,
    mut allocator: *const lzma_allocator,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut unpadded_size: *mut lzma_vli,
    mut uncompressed_size: *mut lzma_vli,
) -> lzma_ret {
    if (*outq).bufs_in_use == 0 as u32 {
        return LZMA_OK;
    }
    let mut buf: *mut lzma_outbuf = (*outq).head;
    lzma_bufcpy(
        &raw mut (*buf).buf as *mut u8,
        &raw mut (*outq).read_pos,
        (*buf).pos,
        out,
        out_pos,
        out_size,
    );
    if !(*buf).finished || (*outq).read_pos < (*buf).pos {
        return LZMA_OK;
    }
    if !unpadded_size.is_null() {
        *unpadded_size = (*buf).unpadded_size;
    }
    if !uncompressed_size.is_null() {
        *uncompressed_size = (*buf).uncompressed_size;
    }
    let finish_ret: lzma_ret = (*buf).finish_ret;
    move_head_to_cache(outq, allocator);
    (*outq).read_pos = 0 as size_t;
    return finish_ret;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_enable_partial_output(
    mut outq: *mut lzma_outq,
    mut enable_partial_output: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
) {
    if !(*outq).head.is_null() && !(*(*outq).head).finished && !(*(*outq).head).worker.is_null() {
        enable_partial_output.expect("non-null function pointer")((*(*outq).head).worker);
        (*(*outq).head).worker = NULL;
    }
}
