use crate::types::*;
use core::ffi::c_void;
#[no_mangle]
pub extern "C" fn lzma_outq_memusage(buf_size_max: u64, threads: u32) -> u64 {
    let limit: u64 = (UINT64_MAX)
        .wrapping_div((2 * 16384) as u64)
        .wrapping_div(2);
    if threads > LZMA_THREADS_MAX || buf_size_max > limit {
        return UINT64_MAX;
    }
    ((2u32).wrapping_mul(threads) as u64)
        .wrapping_mul(lzma_outq_outbuf_memusage(buf_size_max as size_t))
}
unsafe extern "C" fn move_head_to_cache(outq: *mut lzma_outq, allocator: *const lzma_allocator) {
    let buf: *mut lzma_outbuf = (*outq).head;
    (*outq).head = (*buf).next;
    if (*outq).head.is_null() {
        (*outq).tail = core::ptr::null_mut();
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
    outq: *mut lzma_outq,
    allocator: *const lzma_allocator,
) {
    let buf: *mut lzma_outbuf = (*outq).cache;
    (*outq).cache = (*buf).next;
    (*outq).bufs_allocated = (*outq).bufs_allocated.wrapping_sub(1);
    (*outq).mem_allocated = (*outq)
        .mem_allocated
        .wrapping_sub(lzma_outq_outbuf_memusage((*buf).allocated));
    lzma_free(buf as *mut c_void, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_clear_cache(
    outq: *mut lzma_outq,
    allocator: *const lzma_allocator,
) {
    while !(*outq).cache.is_null() {
        free_one_cached_buffer(outq, allocator);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_clear_cache2(
    outq: *mut lzma_outq,
    allocator: *const lzma_allocator,
    keep_size: size_t,
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
    outq: *mut lzma_outq,
    allocator: *const lzma_allocator,
    threads: u32,
) -> lzma_ret {
    if threads > LZMA_THREADS_MAX {
        return LZMA_OPTIONS_ERROR;
    }
    let bufs_limit: u32 = (2u32).wrapping_mul(threads);
    while !(*outq).head.is_null() {
        move_head_to_cache(outq, allocator);
    }
    while bufs_limit < (*outq).bufs_allocated {
        free_one_cached_buffer(outq, allocator);
    }
    (*outq).bufs_limit = bufs_limit;
    (*outq).read_pos = 0;
    LZMA_OK
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_end(outq: *mut lzma_outq, allocator: *const lzma_allocator) {
    while !(*outq).head.is_null() {
        move_head_to_cache(outq, allocator);
    }
    lzma_outq_clear_cache(outq, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_prealloc_buf(
    outq: *mut lzma_outq,
    allocator: *const lzma_allocator,
    size: size_t,
) -> lzma_ret {
    if !(*outq).cache.is_null() && (*(*outq).cache).allocated == size {
        return LZMA_OK;
    }
    if size > (SIZE_MAX as usize).wrapping_sub(core::mem::size_of::<lzma_outbuf>()) {
        return LZMA_MEM_ERROR;
    }
    let alloc_size: size_t = lzma_outq_outbuf_memusage(size) as size_t;
    lzma_outq_clear_cache(outq, allocator);
    (*outq).cache = lzma_alloc(alloc_size, allocator) as *mut lzma_outbuf;
    if (*outq).cache.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*(*outq).cache).next = core::ptr::null_mut();
    (*(*outq).cache).allocated = size;
    (*outq).bufs_allocated = (*outq).bufs_allocated.wrapping_add(1);
    (*outq).mem_allocated = (*outq).mem_allocated.wrapping_add(alloc_size as u64);
    LZMA_OK
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_get_buf(
    outq: *mut lzma_outq,
    worker: *mut c_void,
) -> *mut lzma_outbuf {
    let buf: *mut lzma_outbuf = (*outq).cache;
    (*outq).cache = (*buf).next;
    (*buf).next = core::ptr::null_mut();
    if !(*outq).tail.is_null() {
        (*(*outq).tail).next = buf;
    } else {
        (*outq).head = buf;
    }
    (*outq).tail = buf;
    (*buf).worker = worker;
    (*buf).finished = false;
    (*buf).finish_ret = LZMA_STREAM_END;
    (*buf).pos = 0;
    (*buf).decoder_in_pos = 0;
    (*buf).unpadded_size = 0;
    (*buf).uncompressed_size = 0;
    (*outq).bufs_in_use = (*outq).bufs_in_use.wrapping_add(1);
    (*outq).mem_in_use = (*outq)
        .mem_in_use
        .wrapping_add(lzma_outq_outbuf_memusage((*buf).allocated));
    buf
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_is_readable(outq: *const lzma_outq) -> bool {
    if (*outq).head.is_null() {
        return false;
    }
    (*outq).read_pos < (*(*outq).head).pos || (*(*outq).head).finished
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_read(
    outq: *mut lzma_outq,
    allocator: *const lzma_allocator,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
    unpadded_size: *mut lzma_vli,
    uncompressed_size: *mut lzma_vli,
) -> lzma_ret {
    if (*outq).bufs_in_use == 0 {
        return LZMA_OK;
    }
    let buf: *mut lzma_outbuf = (*outq).head;
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
    (*outq).read_pos = 0;
    finish_ret
}
#[no_mangle]
pub unsafe extern "C" fn lzma_outq_enable_partial_output(
    outq: *mut lzma_outq,
    enable_partial_output: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
) {
    if !(*outq).head.is_null() && !(*(*outq).head).finished && !(*(*outq).head).worker.is_null() {
        enable_partial_output.unwrap()((*(*outq).head).worker);
        (*(*outq).head).worker = core::ptr::null_mut();
    }
}
