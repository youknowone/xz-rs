extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn lzma_check_is_supported(check: lzma_check) -> lzma_bool;
    fn lzma_check_size(check: lzma_check) -> uint32_t;
    fn lzma_block_header_size(block: *mut lzma_block) -> lzma_ret;
    fn lzma_block_header_encode(block: *const lzma_block, out: *mut uint8_t) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_raw_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter,
    ) -> lzma_ret;
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
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type lzma_bool = ::core::ffi::c_uchar;
pub type lzma_reserved_enum = ::core::ffi::c_uint;
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
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
pub type lzma_action = ::core::ffi::c_uint;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
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
pub type lzma_next_coder = lzma_next_coder_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_next_coder_s {
    pub coder: *mut ::core::ffi::c_void,
    pub id: lzma_vli,
    pub init: uintptr_t,
    pub code: lzma_code_function,
    pub end: lzma_end_function,
    pub get_progress:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, *mut uint64_t) -> ()>,
    pub get_check: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> lzma_check>,
    pub memconfig: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut uint64_t,
            *mut uint64_t,
            uint64_t,
        ) -> lzma_ret,
    >,
    pub update: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const lzma_allocator,
            *const lzma_filter,
            *const lzma_filter,
        ) -> lzma_ret,
    >,
    pub set_out_limit:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, uint64_t) -> lzma_ret>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut ::core::ffi::c_void,
}
pub type lzma_vli = uint64_t;
pub type lzma_check = ::core::ffi::c_uint;
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
pub type lzma_end_function =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_allocator) -> ()>;
pub type lzma_code_function = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const lzma_allocator,
        *const uint8_t,
        *mut size_t,
        size_t,
        *mut uint8_t,
        *mut size_t,
        size_t,
        lzma_action,
    ) -> lzma_ret,
>;
pub type lzma_match_finder = ::core::ffi::c_uint;
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub type lzma_mode = ::core::ffi::c_uint;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;
pub const LZMA_MODE_FAST: lzma_mode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_lzma {
    pub dict_size: uint32_t,
    pub preset_dict: *const uint8_t,
    pub preset_dict_size: uint32_t,
    pub lc: uint32_t,
    pub lp: uint32_t,
    pub pb: uint32_t,
    pub mode: lzma_mode,
    pub nice_len: uint32_t,
    pub mf: lzma_match_finder,
    pub depth: uint32_t,
    pub ext_flags: uint32_t,
    pub ext_size_low: uint32_t,
    pub ext_size_high: uint32_t,
    pub reserved_int4: uint32_t,
    pub reserved_int5: uint32_t,
    pub reserved_int6: uint32_t,
    pub reserved_int7: uint32_t,
    pub reserved_int8: uint32_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_ptr1: *mut ::core::ffi::c_void,
    pub reserved_ptr2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_block {
    pub version: uint32_t,
    pub header_size: uint32_t,
    pub check: lzma_check,
    pub compressed_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub filters: *mut lzma_filter,
    pub raw_check: [uint8_t; 64],
    pub reserved_ptr1: *mut ::core::ffi::c_void,
    pub reserved_ptr2: *mut ::core::ffi::c_void,
    pub reserved_ptr3: *mut ::core::ffi::c_void,
    pub reserved_int1: uint32_t,
    pub reserved_int2: uint32_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u8_0: [uint8_t; 64],
    pub u32_0: [uint32_t; 16],
    pub u64_0: [uint64_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_check_state {
    pub buffer: C2RustUnnamed,
    pub state: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const UINT64_MAX: ::core::ffi::c_ulonglong = 18446744073709551615 as ::core::ffi::c_ulonglong;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LZMA_VLI_MAX: ::core::ffi::c_ulonglong =
    UINT64_MAX.wrapping_div(2 as ::core::ffi::c_ulonglong);
pub const LZMA_VLI_UNKNOWN: ::core::ffi::c_ulonglong = UINT64_MAX;
pub const LZMA_VLI_BYTES_MAX: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const LZMA_CHECK_ID_MAX: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const LZMA_CHECK_SIZE_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const LZMA_FILTER_LZMA2: ::core::ffi::c_ulonglong = 0x21 as ::core::ffi::c_ulonglong;
pub const LZMA_DICT_SIZE_MIN: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
pub const LZMA_BLOCK_HEADER_SIZE_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const COMPRESSED_SIZE_MAX: ::core::ffi::c_ulonglong = LZMA_VLI_MAX
    .wrapping_sub(LZMA_BLOCK_HEADER_SIZE_MAX as ::core::ffi::c_ulonglong)
    .wrapping_sub(LZMA_CHECK_SIZE_MAX as ::core::ffi::c_ulonglong)
    & !(3 as ::core::ffi::c_ulonglong);
pub const LZMA2_CHUNK_MAX: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 16 as ::core::ffi::c_int;
pub const LZMA2_HEADER_UNCOMPRESSED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const HEADERS_BOUND: ::core::ffi::c_int = 1 as ::core::ffi::c_int
    + 1 as ::core::ffi::c_int
    + 2 as ::core::ffi::c_int * LZMA_VLI_BYTES_MAX
    + 3 as ::core::ffi::c_int
    + 4 as ::core::ffi::c_int
    + LZMA_CHECK_SIZE_MAX
    + 3 as ::core::ffi::c_int
    & !(3 as ::core::ffi::c_int);
unsafe extern "C" fn lzma2_bound(mut uncompressed_size: uint64_t) -> uint64_t {
    if uncompressed_size > COMPRESSED_SIZE_MAX as uint64_t {
        return 0 as uint64_t;
    }
    let overhead: uint64_t = uncompressed_size
        .wrapping_add(LZMA2_CHUNK_MAX as uint64_t)
        .wrapping_sub(1 as uint64_t)
        .wrapping_div(LZMA2_CHUNK_MAX as uint64_t)
        .wrapping_mul(LZMA2_HEADER_UNCOMPRESSED as uint64_t)
        .wrapping_add(1 as uint64_t);
    if (COMPRESSED_SIZE_MAX as uint64_t).wrapping_sub(overhead) < uncompressed_size {
        return 0 as uint64_t;
    }
    return uncompressed_size.wrapping_add(overhead);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_buffer_bound64(mut uncompressed_size: uint64_t) -> uint64_t {
    let mut lzma2_size: uint64_t = lzma2_bound(uncompressed_size);
    if lzma2_size == 0 as uint64_t {
        return 0 as uint64_t;
    }
    lzma2_size = lzma2_size.wrapping_add(3 as uint64_t) & !(3 as uint64_t);
    return (HEADERS_BOUND as uint64_t).wrapping_add(lzma2_size);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_buffer_bound(mut uncompressed_size: size_t) -> size_t {
    let mut ret: uint64_t = lzma_block_buffer_bound64(uncompressed_size as uint64_t);
    return ret as size_t;
}
unsafe extern "C" fn block_encode_uncompressed(
    mut block: *mut lzma_block,
    mut in_0: *const uint8_t,
    mut in_size: size_t,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    let mut lzma2: lzma_options_lzma = lzma_options_lzma {
        dict_size: LZMA_DICT_SIZE_MIN as uint32_t,
        preset_dict: ::core::ptr::null::<uint8_t>(),
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
        reserved_ptr1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        reserved_ptr2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut filters: [lzma_filter; 2] = [lzma_filter {
        id: 0,
        options: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    }; 2];
    filters[0 as ::core::ffi::c_int as usize].id = LZMA_FILTER_LZMA2 as lzma_vli;
    filters[0 as ::core::ffi::c_int as usize].options = &raw mut lzma2 as *mut ::core::ffi::c_void;
    filters[1 as ::core::ffi::c_int as usize].id = LZMA_VLI_UNKNOWN as lzma_vli;
    let mut filters_orig: *mut lzma_filter = (*block).filters;
    (*block).filters = &raw mut filters as *mut lzma_filter;
    if lzma_block_header_size(block) as ::core::ffi::c_uint
        != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*block).filters = filters_orig;
        return LZMA_PROG_ERROR;
    }
    if (out_size.wrapping_sub(*out_pos) as lzma_vli)
        < ((*block).header_size as lzma_vli).wrapping_add((*block).compressed_size)
    {
        (*block).filters = filters_orig;
        return LZMA_BUF_ERROR;
    }
    if lzma_block_header_encode(block, out.offset(*out_pos as isize)) as ::core::ffi::c_uint
        != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*block).filters = filters_orig;
        return LZMA_PROG_ERROR;
    }
    (*block).filters = filters_orig;
    *out_pos = (*out_pos).wrapping_add((*block).header_size as size_t);
    let mut in_pos: size_t = 0 as size_t;
    let mut control: uint8_t = 0x1 as uint8_t;
    while in_pos < in_size {
        let fresh1 = *out_pos;
        *out_pos = (*out_pos).wrapping_add(1);
        *out.offset(fresh1 as isize) = control;
        control = 0x2 as uint8_t;
        let copy_size: size_t = if in_size.wrapping_sub(in_pos)
            < ((1 as ::core::ffi::c_uint) << 16 as ::core::ffi::c_int) as size_t
        {
            in_size.wrapping_sub(in_pos)
        } else {
            ((1 as ::core::ffi::c_uint) << 16 as ::core::ffi::c_int) as size_t
        };
        let fresh2 = *out_pos;
        *out_pos = (*out_pos).wrapping_add(1);
        *out.offset(fresh2 as isize) =
            (copy_size.wrapping_sub(1 as size_t) >> 8 as ::core::ffi::c_int) as uint8_t;
        let fresh3 = *out_pos;
        *out_pos = (*out_pos).wrapping_add(1);
        *out.offset(fresh3 as isize) =
            (copy_size.wrapping_sub(1 as size_t) & 0xff as size_t) as uint8_t;
        memcpy(
            out.offset(*out_pos as isize) as *mut ::core::ffi::c_void,
            in_0.offset(in_pos as isize) as *const ::core::ffi::c_void,
            copy_size,
        );
        in_pos = in_pos.wrapping_add(copy_size);
        *out_pos = (*out_pos).wrapping_add(copy_size);
    }
    let fresh4 = *out_pos;
    *out_pos = (*out_pos).wrapping_add(1);
    *out.offset(fresh4 as isize) = 0 as uint8_t;
    return LZMA_OK;
}
unsafe extern "C" fn block_encode_normal(
    mut block: *mut lzma_block,
    mut allocator: *const lzma_allocator,
    mut in_0: *const uint8_t,
    mut in_size: size_t,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_block_header_size(block) as lzma_ret;
    if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret_;
    }
    if out_size.wrapping_sub(*out_pos) <= (*block).header_size as size_t {
        return LZMA_BUF_ERROR;
    }
    let out_start: size_t = *out_pos;
    *out_pos = (*out_pos).wrapping_add((*block).header_size as size_t);
    if out_size.wrapping_sub(*out_pos) as lzma_vli > (*block).compressed_size {
        out_size = (*out_pos as lzma_vli).wrapping_add((*block).compressed_size) as size_t;
    }
    let mut raw_encoder: lzma_next_coder = lzma_next_coder_s {
        coder: NULL,
        id: LZMA_VLI_UNKNOWN as lzma_vli,
        init: ::core::ptr::null_mut::<::core::ffi::c_void>() as uintptr_t,
        code: None,
        end: None,
        get_progress: None,
        get_check: None,
        memconfig: None,
        update: None,
        set_out_limit: None,
    };
    let mut ret: lzma_ret =
        lzma_raw_encoder_init(&raw mut raw_encoder, allocator, (*block).filters);
    if ret as ::core::ffi::c_uint == LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut in_pos: size_t = 0 as size_t;
        ret = raw_encoder.code.expect("non-null function pointer")(
            raw_encoder.coder,
            allocator,
            in_0,
            &raw mut in_pos,
            in_size,
            out,
            out_pos,
            out_size,
            LZMA_FINISH,
        );
    }
    lzma_next_end(&raw mut raw_encoder, allocator);
    if ret as ::core::ffi::c_uint == LZMA_STREAM_END as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*block).compressed_size = (*out_pos)
            .wrapping_sub(out_start.wrapping_add((*block).header_size as size_t))
            as lzma_vli;
        ret = lzma_block_header_encode(block, out.offset(out_start as isize));
        if ret as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
            ret = LZMA_PROG_ERROR;
        }
    } else if ret as ::core::ffi::c_uint == LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        ret = LZMA_BUF_ERROR;
    }
    if ret as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        *out_pos = out_start;
    }
    return ret;
}
unsafe extern "C" fn block_buffer_encode(
    mut block: *mut lzma_block,
    mut allocator: *const lzma_allocator,
    mut in_0: *const uint8_t,
    mut in_size: size_t,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut try_to_compress: bool,
) -> lzma_ret {
    if block.is_null()
        || in_0.is_null() && in_size != 0 as size_t
        || out.is_null()
        || out_pos.is_null()
        || *out_pos > out_size
    {
        return LZMA_PROG_ERROR;
    }
    if (*block).version > 1 as uint32_t {
        return LZMA_OPTIONS_ERROR;
    }
    if (*block).check as ::core::ffi::c_uint > LZMA_CHECK_ID_MAX as ::core::ffi::c_uint
        || try_to_compress as ::core::ffi::c_int != 0 && (*block).filters.is_null()
    {
        return LZMA_PROG_ERROR;
    }
    if lzma_check_is_supported((*block).check) == 0 {
        return LZMA_UNSUPPORTED_CHECK;
    }
    out_size = out_size.wrapping_sub(out_size.wrapping_sub(*out_pos) & 3 as size_t);
    let check_size: size_t = lzma_check_size((*block).check) as size_t;
    if out_size.wrapping_sub(*out_pos) <= check_size {
        return LZMA_BUF_ERROR;
    }
    out_size = out_size.wrapping_sub(check_size);
    (*block).uncompressed_size = in_size as lzma_vli;
    (*block).compressed_size = lzma2_bound(in_size as uint64_t) as lzma_vli;
    if (*block).compressed_size == 0 as lzma_vli {
        return LZMA_DATA_ERROR;
    }
    let mut ret: lzma_ret = LZMA_BUF_ERROR;
    if try_to_compress {
        ret = block_encode_normal(block, allocator, in_0, in_size, out, out_pos, out_size);
    }
    if ret as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        if ret as ::core::ffi::c_uint != LZMA_BUF_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return ret;
        }
        let ret_: lzma_ret =
            block_encode_uncompressed(block, in_0, in_size, out, out_pos, out_size) as lzma_ret;
        if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
            return ret_;
        }
    }
    let mut i: size_t = (*block).compressed_size as size_t;
    while i & 3 as size_t != 0 {
        let fresh0 = *out_pos;
        *out_pos = (*out_pos).wrapping_add(1);
        *out.offset(fresh0 as isize) = 0 as uint8_t;
        i = i.wrapping_add(1);
    }
    if check_size > 0 as size_t {
        let mut check: lzma_check_state = lzma_check_state {
            buffer: C2RustUnnamed { u8_0: [0; 64] },
            state: C2RustUnnamed_0 { crc32: 0 },
        };
        lzma_check_init(&raw mut check, (*block).check);
        lzma_check_update(&raw mut check, (*block).check, in_0, in_size);
        lzma_check_finish(&raw mut check, (*block).check);
        memcpy(
            &raw mut (*block).raw_check as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut check.buffer.u8_0 as *mut uint8_t as *const ::core::ffi::c_void,
            check_size,
        );
        memcpy(
            out.offset(*out_pos as isize) as *mut ::core::ffi::c_void,
            &raw mut check.buffer.u8_0 as *mut uint8_t as *const ::core::ffi::c_void,
            check_size,
        );
        *out_pos = (*out_pos).wrapping_add(check_size);
    }
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_buffer_encode(
    mut block: *mut lzma_block,
    mut allocator: *const lzma_allocator,
    mut in_0: *const uint8_t,
    mut in_size: size_t,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    return block_buffer_encode(
        block,
        allocator,
        in_0,
        in_size,
        out,
        out_pos,
        out_size,
        true_0 != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_uncomp_encode(
    mut block: *mut lzma_block,
    mut in_0: *const uint8_t,
    mut in_size: size_t,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    return block_buffer_encode(
        block,
        ::core::ptr::null::<lzma_allocator>(),
        in_0,
        in_size,
        out,
        out_pos,
        out_size,
        false_0 != 0,
    );
}
