use crate::types::*;
extern "C" {
    fn lzma_block_buffer_bound(uncompressed_size: size_t) -> size_t;
    fn lzma_block_buffer_encode(
        block: *mut lzma_block,
        allocator: *const lzma_allocator,
        in_0: *const u8,
        in_size: size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> lzma_ret;
    fn lzma_index_buffer_encode(
        i: *const lzma_index,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> lzma_ret;
}
pub const INDEX_BOUND: u32 = 1 + 1 + 2 * LZMA_VLI_BYTES_MAX + 4 + 3 & !(3);
pub const HEADERS_BOUND: u32 = 2 * LZMA_STREAM_HEADER_SIZE + INDEX_BOUND;
#[no_mangle]
pub extern "C" fn lzma_stream_buffer_bound(uncompressed_size: size_t) -> size_t {
    let block_bound: size_t = unsafe { lzma_block_buffer_bound(uncompressed_size) } as size_t;
    if block_bound == 0 {
        return 0;
    }

    // Match C: my_min(SIZE_MAX, LZMA_VLI_MAX) - block_bound < HEADERS_BOUND.
    let stream_bound_max: size_t = core::cmp::min(size_t::MAX, LZMA_VLI_MAX as size_t);
    if stream_bound_max.wrapping_sub(block_bound) < HEADERS_BOUND as size_t {
        return 0;
    }
    block_bound.wrapping_add(HEADERS_BOUND as size_t)
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_buffer_encode(
    filters: *mut lzma_filter,
    check: lzma_check,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos_ptr: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    if filters.is_null()
        || check > LZMA_CHECK_ID_MAX
        || in_0.is_null() && in_size != 0
        || out.is_null()
        || out_pos_ptr.is_null()
        || *out_pos_ptr > out_size
    {
        return LZMA_PROG_ERROR;
    }
    if lzma_check_is_supported(check) == 0 {
        return LZMA_UNSUPPORTED_CHECK;
    }
    let mut out_pos: size_t = *out_pos_ptr;
    if out_size.wrapping_sub(out_pos) <= (2 * LZMA_STREAM_HEADER_SIZE) as size_t {
        return LZMA_BUF_ERROR;
    }
    out_size = out_size.wrapping_sub(LZMA_STREAM_HEADER_SIZE as size_t);
    let mut stream_flags: lzma_stream_flags = lzma_stream_flags {
        version: 0,
        backward_size: 0,
        check: check,
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
    if lzma_stream_header_encode(&raw mut stream_flags, out.offset(out_pos as isize)) != LZMA_OK {
        return LZMA_PROG_ERROR;
    }
    out_pos = out_pos.wrapping_add(LZMA_STREAM_HEADER_SIZE as size_t);
    let mut block: lzma_block = lzma_block {
        version: 0,
        header_size: 0,
        check: check,
        compressed_size: 0,
        uncompressed_size: 0,
        filters: filters,
        raw_check: [0; 64],
        reserved_ptr1: core::ptr::null_mut(),
        reserved_ptr2: core::ptr::null_mut(),
        reserved_ptr3: core::ptr::null_mut(),
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
    if in_size > 0 {
        let ret_: lzma_ret = lzma_block_buffer_encode(
            &raw mut block,
            allocator,
            in_0,
            in_size,
            out,
            &raw mut out_pos,
            out_size,
        );
        if ret_ != LZMA_OK {
            return ret_;
        }
    }
    let i: *mut lzma_index = lzma_index_init(allocator);
    if i.is_null() {
        return LZMA_MEM_ERROR;
    }
    let mut ret: lzma_ret = LZMA_OK;
    if in_size > 0 {
        ret = lzma_index_append(
            i,
            allocator,
            lzma_block_unpadded_size(&raw mut block),
            block.uncompressed_size,
        );
    }
    if ret == LZMA_OK {
        ret = lzma_index_buffer_encode(i, out, &raw mut out_pos, out_size);
        stream_flags.backward_size = lzma_index_size(i);
    }
    lzma_index_end(i, allocator);
    if ret != LZMA_OK {
        return ret;
    }
    if lzma_stream_footer_encode(&raw mut stream_flags, out.offset(out_pos as isize)) != LZMA_OK {
        return LZMA_PROG_ERROR;
    }
    out_pos = out_pos.wrapping_add(LZMA_STREAM_HEADER_SIZE as size_t);
    *out_pos_ptr = out_pos;
    LZMA_OK
}
