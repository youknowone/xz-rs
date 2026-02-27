use crate::types::*;
use core::ffi::{c_int, c_uchar, c_uint, c_ulonglong, c_void};
#[repr(C)]
pub struct lzma_index_s {
    _opaque: [u8; 0],
}
extern "C" {
    fn lzma_check_is_supported(check: lzma_check) -> lzma_bool;
    fn lzma_stream_header_encode(options: *const lzma_stream_flags, out: *mut u8) -> lzma_ret;
    fn lzma_stream_footer_encode(options: *const lzma_stream_flags, out: *mut u8) -> lzma_ret;
    fn lzma_block_unpadded_size(block: *const lzma_block) -> lzma_vli;
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
    fn lzma_index_init(allocator: *const lzma_allocator) -> *mut lzma_index;
    fn lzma_index_end(i: *mut lzma_index, allocator: *const lzma_allocator);
    fn lzma_index_append(
        i: *mut lzma_index,
        allocator: *const lzma_allocator,
        unpadded_size: lzma_vli,
        uncompressed_size: lzma_vli,
    ) -> lzma_ret;
    fn lzma_index_size(i: *const lzma_index) -> lzma_vli;
    fn lzma_index_buffer_encode(
        i: *const lzma_index,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
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
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
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
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const LZMA_VLI_BYTES_MAX: c_int = 9 as c_int;
pub const UINT64_MAX: c_ulonglong = 18446744073709551615;
pub const LZMA_VLI_MAX: c_ulonglong = UINT64_MAX.wrapping_div(2);
pub const LZMA_CHECK_ID_MAX: lzma_check = 15;
pub const LZMA_STREAM_HEADER_SIZE: c_int = 12 as c_int;
pub const INDEX_BOUND: c_int =
    1 as c_int + 1 as c_int + 2 as c_int * LZMA_VLI_BYTES_MAX + 4 as c_int + 3 as c_int
        & !(3 as c_int);
pub const HEADERS_BOUND: c_int = 2 as c_int * LZMA_STREAM_HEADER_SIZE + INDEX_BOUND;
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_buffer_bound(mut uncompressed_size: size_t) -> size_t {
    let block_bound: size_t = lzma_block_buffer_bound(uncompressed_size) as size_t;
    if block_bound == 0 as size_t {
        return 0 as size_t;
    }

    // Match C: my_min(SIZE_MAX, LZMA_VLI_MAX) - block_bound < HEADERS_BOUND.
    let stream_bound_max: size_t = core::cmp::min(size_t::MAX, LZMA_VLI_MAX as size_t);
    if stream_bound_max.wrapping_sub(block_bound) < HEADERS_BOUND as size_t {
        return 0 as size_t;
    }
    return block_bound.wrapping_add(HEADERS_BOUND as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_buffer_encode(
    mut filters: *mut lzma_filter,
    mut check: lzma_check,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_size: size_t,
    mut out: *mut u8,
    mut out_pos_ptr: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    if filters.is_null()
        || check > LZMA_CHECK_ID_MAX
        || in_0.is_null() && in_size != 0 as size_t
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
    if out_size.wrapping_sub(out_pos) <= (2 as c_int * LZMA_STREAM_HEADER_SIZE) as size_t {
        return LZMA_BUF_ERROR;
    }
    out_size = out_size.wrapping_sub(LZMA_STREAM_HEADER_SIZE as size_t);
    let mut stream_flags: lzma_stream_flags = lzma_stream_flags {
        version: 0 as u32,
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
        version: 0 as u32,
        header_size: 0,
        check: check,
        compressed_size: 0,
        uncompressed_size: 0,
        filters: filters,
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
    if in_size > 0 as size_t {
        let ret_: lzma_ret = lzma_block_buffer_encode(
            &raw mut block,
            allocator,
            in_0,
            in_size,
            out,
            &raw mut out_pos,
            out_size,
        ) as lzma_ret;
        if ret_ != LZMA_OK {
            return ret_;
        }
    }
    let mut i: *mut lzma_index = lzma_index_init(allocator);
    if i.is_null() {
        return LZMA_MEM_ERROR;
    }
    let mut ret: lzma_ret = LZMA_OK;
    if in_size > 0 as size_t {
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
    return LZMA_OK;
}
