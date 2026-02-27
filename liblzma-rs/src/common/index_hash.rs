extern "C" {
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn lzma_vli_decode(
        vli: *mut lzma_vli,
        vli_pos: *mut size_t,
        in_0: *const uint8_t,
        in_pos: *mut size_t,
        in_size: size_t,
    ) -> lzma_ret;
    fn lzma_vli_size(vli: lzma_vli) -> uint32_t;
    fn lzma_check_size(check: lzma_check) -> uint32_t;
    fn lzma_crc32(buf: *const uint8_t, size: size_t, crc: uint32_t) -> uint32_t;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut ::core::ffi::c_void;
    fn lzma_free(ptr: *mut ::core::ffi::c_void, allocator: *const lzma_allocator);
    fn lzma_check_init(check: *mut lzma_check_state, type_0: lzma_check);
    fn lzma_check_update(
        check: *mut lzma_check_state,
        type_0: lzma_check,
        buf: *const uint8_t,
        size: size_t,
    );
    fn lzma_check_finish(check: *mut lzma_check_state, type_0: lzma_check);
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type lzma_ret = ::core::ffi::c_uint;
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
    pub alloc: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, size_t) -> *mut ::core::ffi::c_void,
    >,
    pub free:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}
pub type lzma_vli = uint64_t;
pub type lzma_check = ::core::ffi::c_uint;
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_index_hash_s {
    pub sequence: C2RustUnnamed_1,
    pub blocks: lzma_index_hash_info,
    pub records: lzma_index_hash_info,
    pub remaining: lzma_vli,
    pub unpadded_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub pos: size_t,
    pub crc32: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_index_hash_info {
    pub blocks_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub count: lzma_vli,
    pub index_list_size: lzma_vli,
    pub check: lzma_check_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_check_state {
    pub buffer: C2RustUnnamed_0,
    pub state: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub crc32: uint32_t,
    pub crc64: uint64_t,
    pub sha256: lzma_sha256_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_sha256_state {
    pub state: [uint32_t; 8],
    pub size: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub u8_0: [uint8_t; 64],
    pub u32_0: [uint32_t; 16],
    pub u64_0: [uint64_t; 8],
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const SEQ_CRC32: C2RustUnnamed_1 = 6;
pub const SEQ_PADDING: C2RustUnnamed_1 = 5;
pub const SEQ_PADDING_INIT: C2RustUnnamed_1 = 4;
pub const SEQ_UNCOMPRESSED: C2RustUnnamed_1 = 3;
pub const SEQ_UNPADDED: C2RustUnnamed_1 = 2;
pub const SEQ_COUNT: C2RustUnnamed_1 = 1;
pub const SEQ_BLOCK: C2RustUnnamed_1 = 0;
pub type lzma_index_hash = lzma_index_hash_s;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const UINT64_MAX: ::core::ffi::c_ulonglong = 18446744073709551615 as ::core::ffi::c_ulonglong;
pub const LZMA_VLI_MAX: ::core::ffi::c_ulonglong =
    UINT64_MAX.wrapping_div(2 as ::core::ffi::c_ulonglong);
pub const LZMA_STREAM_HEADER_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const LZMA_BACKWARD_SIZE_MAX: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << 34 as ::core::ffi::c_int;
pub const UNPADDED_SIZE_MIN: ::core::ffi::c_ulonglong = 5 as ::core::ffi::c_ulonglong;
pub const UNPADDED_SIZE_MAX: ::core::ffi::c_ulonglong =
    LZMA_VLI_MAX & !(3 as ::core::ffi::c_ulonglong);
pub const INDEX_INDICATOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn vli_ceil4(mut vli: lzma_vli) -> lzma_vli {
    return vli.wrapping_add(3 as lzma_vli) & !(3 as lzma_vli);
}
#[inline]
unsafe extern "C" fn index_size_unpadded(
    mut count: lzma_vli,
    mut index_list_size: lzma_vli,
) -> lzma_vli {
    return ((1 as uint32_t).wrapping_add(lzma_vli_size(count)) as lzma_vli)
        .wrapping_add(index_list_size)
        .wrapping_add(4 as lzma_vli);
}
#[inline]
unsafe extern "C" fn index_size(mut count: lzma_vli, mut index_list_size: lzma_vli) -> lzma_vli {
    return vli_ceil4(index_size_unpadded(count, index_list_size));
}
#[inline]
unsafe extern "C" fn index_stream_size(
    mut blocks_size: lzma_vli,
    mut count: lzma_vli,
    mut index_list_size: lzma_vli,
) -> lzma_vli {
    return (LZMA_STREAM_HEADER_SIZE as lzma_vli)
        .wrapping_add(blocks_size)
        .wrapping_add(index_size(count, index_list_size))
        .wrapping_add(LZMA_STREAM_HEADER_SIZE as lzma_vli);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_init(
    mut index_hash: *mut lzma_index_hash,
    mut allocator: *const lzma_allocator,
) -> *mut lzma_index_hash {
    if index_hash.is_null() {
        index_hash = lzma_alloc(
            ::core::mem::size_of::<lzma_index_hash>() as size_t,
            allocator,
        ) as *mut lzma_index_hash;
        if index_hash.is_null() {
            return ::core::ptr::null_mut::<lzma_index_hash>();
        }
    }
    (*index_hash).sequence = SEQ_BLOCK;
    (*index_hash).blocks.blocks_size = 0 as lzma_vli;
    (*index_hash).blocks.uncompressed_size = 0 as lzma_vli;
    (*index_hash).blocks.count = 0 as lzma_vli;
    (*index_hash).blocks.index_list_size = 0 as lzma_vli;
    (*index_hash).records.blocks_size = 0 as lzma_vli;
    (*index_hash).records.uncompressed_size = 0 as lzma_vli;
    (*index_hash).records.count = 0 as lzma_vli;
    (*index_hash).records.index_list_size = 0 as lzma_vli;
    (*index_hash).unpadded_size = 0 as lzma_vli;
    (*index_hash).uncompressed_size = 0 as lzma_vli;
    (*index_hash).pos = 0 as size_t;
    (*index_hash).crc32 = 0 as uint32_t;
    lzma_check_init(&raw mut (*index_hash).blocks.check, LZMA_CHECK_SHA256);
    lzma_check_init(&raw mut (*index_hash).records.check, LZMA_CHECK_SHA256);
    return index_hash;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_end(
    mut index_hash: *mut lzma_index_hash,
    mut allocator: *const lzma_allocator,
) {
    lzma_free(index_hash as *mut ::core::ffi::c_void, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_size(mut index_hash: *const lzma_index_hash) -> lzma_vli {
    return index_size(
        (*index_hash).blocks.count,
        (*index_hash).blocks.index_list_size,
    );
}
unsafe extern "C" fn hash_append(
    mut info: *mut lzma_index_hash_info,
    mut unpadded_size: lzma_vli,
    mut uncompressed_size: lzma_vli,
) {
    (*info).blocks_size = (*info).blocks_size.wrapping_add(vli_ceil4(unpadded_size));
    (*info).uncompressed_size = (*info).uncompressed_size.wrapping_add(uncompressed_size);
    (*info).index_list_size = (*info).index_list_size.wrapping_add(
        lzma_vli_size(unpadded_size).wrapping_add(lzma_vli_size(uncompressed_size)) as lzma_vli,
    );
    (*info).count = (*info).count.wrapping_add(1);
    let sizes: [lzma_vli; 2] = [unpadded_size, uncompressed_size];
    lzma_check_update(
        &raw mut (*info).check,
        LZMA_CHECK_SHA256,
        &raw const sizes as *const lzma_vli as *const uint8_t,
        ::core::mem::size_of::<[lzma_vli; 2]>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_append(
    mut index_hash: *mut lzma_index_hash,
    mut unpadded_size: lzma_vli,
    mut uncompressed_size: lzma_vli,
) -> lzma_ret {
    if index_hash.is_null()
        || (*index_hash).sequence as ::core::ffi::c_uint
            != SEQ_BLOCK as ::core::ffi::c_int as ::core::ffi::c_uint
        || unpadded_size < UNPADDED_SIZE_MIN as lzma_vli
        || unpadded_size > UNPADDED_SIZE_MAX as lzma_vli
        || uncompressed_size > LZMA_VLI_MAX as lzma_vli
    {
        return LZMA_PROG_ERROR;
    }
    hash_append(
        &raw mut (*index_hash).blocks,
        unpadded_size,
        uncompressed_size,
    );
    if (*index_hash).blocks.blocks_size > LZMA_VLI_MAX as lzma_vli
        || (*index_hash).blocks.uncompressed_size > LZMA_VLI_MAX as lzma_vli
        || index_size(
            (*index_hash).blocks.count,
            (*index_hash).blocks.index_list_size,
        ) > LZMA_BACKWARD_SIZE_MAX as lzma_vli
        || index_stream_size(
            (*index_hash).blocks.blocks_size,
            (*index_hash).blocks.count,
            (*index_hash).blocks.index_list_size,
        ) > LZMA_VLI_MAX as lzma_vli
    {
        return LZMA_DATA_ERROR;
    }
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_hash_decode(
    mut index_hash: *mut lzma_index_hash,
    mut in_0: *const uint8_t,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
) -> lzma_ret {
    let mut current_block: u64;
    if *in_pos >= in_size {
        return LZMA_BUF_ERROR;
    }
    let in_start: size_t = *in_pos;
    let mut ret: lzma_ret = LZMA_OK;
    while *in_pos < in_size {
        match (*index_hash).sequence as ::core::ffi::c_uint {
            0 => {
                let fresh0 = *in_pos;
                *in_pos = (*in_pos).wrapping_add(1);
                if *in_0.offset(fresh0 as isize) as ::core::ffi::c_int != INDEX_INDICATOR {
                    return LZMA_DATA_ERROR;
                }
                (*index_hash).sequence = SEQ_COUNT;
                continue;
            }
            1 => {
                ret = lzma_vli_decode(
                    &raw mut (*index_hash).remaining,
                    &raw mut (*index_hash).pos,
                    in_0,
                    in_pos,
                    in_size,
                );
                if ret as ::core::ffi::c_uint
                    != LZMA_STREAM_END as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    break;
                }
                if (*index_hash).remaining != (*index_hash).blocks.count {
                    return LZMA_DATA_ERROR;
                }
                ret = LZMA_OK;
                (*index_hash).pos = 0 as size_t;
                (*index_hash).sequence = (if (*index_hash).remaining == 0 as lzma_vli {
                    SEQ_PADDING_INIT as ::core::ffi::c_int
                } else {
                    SEQ_UNPADDED as ::core::ffi::c_int
                }) as C2RustUnnamed_1;
                continue;
            }
            2 | 3 => {
                let mut size: *mut lzma_vli = if (*index_hash).sequence as ::core::ffi::c_uint
                    == SEQ_UNPADDED as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    &raw mut (*index_hash).unpadded_size
                } else {
                    &raw mut (*index_hash).uncompressed_size
                };
                ret = lzma_vli_decode(size, &raw mut (*index_hash).pos, in_0, in_pos, in_size);
                if ret as ::core::ffi::c_uint
                    != LZMA_STREAM_END as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    break;
                }
                ret = LZMA_OK;
                (*index_hash).pos = 0 as size_t;
                if (*index_hash).sequence as ::core::ffi::c_uint
                    == SEQ_UNPADDED as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if (*index_hash).unpadded_size < UNPADDED_SIZE_MIN as lzma_vli
                        || (*index_hash).unpadded_size > UNPADDED_SIZE_MAX as lzma_vli
                    {
                        return LZMA_DATA_ERROR;
                    }
                    (*index_hash).sequence = SEQ_UNCOMPRESSED;
                } else {
                    hash_append(
                        &raw mut (*index_hash).records,
                        (*index_hash).unpadded_size,
                        (*index_hash).uncompressed_size,
                    );
                    if (*index_hash).blocks.blocks_size < (*index_hash).records.blocks_size
                        || (*index_hash).blocks.uncompressed_size
                            < (*index_hash).records.uncompressed_size
                        || (*index_hash).blocks.index_list_size
                            < (*index_hash).records.index_list_size
                    {
                        return LZMA_DATA_ERROR;
                    }
                    (*index_hash).remaining = (*index_hash).remaining.wrapping_sub(1);
                    (*index_hash).sequence = (if (*index_hash).remaining == 0 as lzma_vli {
                        SEQ_PADDING_INIT as ::core::ffi::c_int
                    } else {
                        SEQ_UNPADDED as ::core::ffi::c_int
                    }) as C2RustUnnamed_1;
                }
                continue;
            }
            4 => {
                (*index_hash).pos = ((4 as lzma_vli).wrapping_sub(index_size_unpadded(
                    (*index_hash).records.count,
                    (*index_hash).records.index_list_size,
                )) & 3 as lzma_vli) as size_t;
                (*index_hash).sequence = SEQ_PADDING;
                current_block = 12753679906265593574;
            }
            5 => {
                current_block = 12753679906265593574;
            }
            6 => {
                current_block = 88292361528268742;
            }
            _ => return LZMA_PROG_ERROR,
        }
        match current_block {
            12753679906265593574 => {
                if (*index_hash).pos > 0 as size_t {
                    (*index_hash).pos = (*index_hash).pos.wrapping_sub(1);
                    let fresh1 = *in_pos;
                    *in_pos = (*in_pos).wrapping_add(1);
                    if *in_0.offset(fresh1 as isize) as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        return LZMA_DATA_ERROR;
                    }
                    continue;
                } else {
                    if (*index_hash).blocks.blocks_size != (*index_hash).records.blocks_size
                        || (*index_hash).blocks.uncompressed_size
                            != (*index_hash).records.uncompressed_size
                        || (*index_hash).blocks.index_list_size
                            != (*index_hash).records.index_list_size
                    {
                        return LZMA_DATA_ERROR;
                    }
                    lzma_check_finish(&raw mut (*index_hash).blocks.check, LZMA_CHECK_SHA256);
                    lzma_check_finish(&raw mut (*index_hash).records.check, LZMA_CHECK_SHA256);
                    if memcmp(
                        &raw mut (*index_hash).blocks.check.buffer.u8_0 as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        &raw mut (*index_hash).records.check.buffer.u8_0 as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        lzma_check_size(LZMA_CHECK_SHA256) as size_t,
                    ) != 0 as ::core::ffi::c_int
                    {
                        return LZMA_DATA_ERROR;
                    }
                    (*index_hash).crc32 = lzma_crc32(
                        in_0.offset(in_start as isize),
                        (*in_pos).wrapping_sub(in_start),
                        (*index_hash).crc32,
                    );
                    (*index_hash).sequence = SEQ_CRC32;
                }
            }
            _ => {}
        }
        loop {
            if *in_pos == in_size {
                return LZMA_OK;
            }
            let fresh2 = *in_pos;
            *in_pos = (*in_pos).wrapping_add(1);
            if (*index_hash).crc32 >> (*index_hash).pos.wrapping_mul(8 as size_t) & 0xff as uint32_t
                != *in_0.offset(fresh2 as isize) as uint32_t
            {
                return LZMA_DATA_ERROR;
            }
            (*index_hash).pos = (*index_hash).pos.wrapping_add(1);
            if !((*index_hash).pos < 4 as size_t) {
                break;
            }
        }
        return LZMA_STREAM_END;
    }
    let in_used: size_t = (*in_pos).wrapping_sub(in_start);
    if in_used > 0 as size_t {
        (*index_hash).crc32 =
            lzma_crc32(in_0.offset(in_start as isize), in_used, (*index_hash).crc32);
    }
    return ret;
}
