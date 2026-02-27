extern "C" {
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn lzma_crc32(buf: *const uint8_t, size: size_t, crc: uint32_t) -> uint32_t;
    static lzma_header_magic: [uint8_t; 6];
    static lzma_footer_magic: [uint8_t; 2];
}
pub type __darwin_size_t = usize;
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
pub type lzma_vli = uint64_t;
pub type lzma_check = ::core::ffi::c_uint;
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream_flags {
    pub version: uint32_t,
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
    pub reserved_int1: uint32_t,
    pub reserved_int2: uint32_t,
}
pub const UINT64_MAX: ::core::ffi::c_ulonglong = 18446744073709551615 as ::core::ffi::c_ulonglong;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn read32le(mut buf: *const uint8_t) -> uint32_t {
    let mut num: uint32_t = *buf.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
    num |= (*buf.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int;
    num |= (*buf.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int;
    num |= (*buf.offset(3 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int;
    return num;
}
pub const LZMA_VLI_UNKNOWN: ::core::ffi::c_ulonglong = UINT64_MAX;
pub const LZMA_STREAM_FLAGS_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn stream_flags_decode(
    mut options: *mut lzma_stream_flags,
    mut in_0: *const uint8_t,
) -> bool {
    if *in_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        || *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xf0 as ::core::ffi::c_int
            != 0
    {
        return true_0 != 0;
    }
    (*options).version = 0 as uint32_t;
    (*options).check = (*in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int) as lzma_check;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_header_decode(
    mut options: *mut lzma_stream_flags,
    mut in_0: *const uint8_t,
) -> lzma_ret {
    if memcmp(
        in_0 as *const ::core::ffi::c_void,
        &raw const lzma_header_magic as *const uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 6]>() as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        return LZMA_FORMAT_ERROR;
    }
    let crc: uint32_t = lzma_crc32(
        in_0.offset(::core::mem::size_of::<[uint8_t; 6]>() as usize as isize),
        LZMA_STREAM_FLAGS_SIZE as size_t,
        0 as uint32_t,
    ) as uint32_t;
    if crc
        != read32le(
            in_0.offset(::core::mem::size_of::<[uint8_t; 6]>() as usize as isize)
                .offset(LZMA_STREAM_FLAGS_SIZE as isize),
        )
    {
        return LZMA_DATA_ERROR;
    }
    if stream_flags_decode(
        options,
        in_0.offset(::core::mem::size_of::<[uint8_t; 6]>() as usize as isize),
    ) {
        return LZMA_OPTIONS_ERROR;
    }
    (*options).backward_size = LZMA_VLI_UNKNOWN as lzma_vli;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_footer_decode(
    mut options: *mut lzma_stream_flags,
    mut in_0: *const uint8_t,
) -> lzma_ret {
    if memcmp(
        in_0.offset((::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(2 as usize) as isize)
            .offset(LZMA_STREAM_FLAGS_SIZE as isize) as *const ::core::ffi::c_void,
        &raw const lzma_footer_magic as *const uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        return LZMA_FORMAT_ERROR;
    }
    let crc: uint32_t = lzma_crc32(
        in_0.offset(::core::mem::size_of::<uint32_t>() as usize as isize),
        (::core::mem::size_of::<uint32_t>() as size_t)
            .wrapping_add(LZMA_STREAM_FLAGS_SIZE as size_t),
        0 as uint32_t,
    ) as uint32_t;
    if crc != read32le(in_0) {
        return LZMA_DATA_ERROR;
    }
    if stream_flags_decode(
        options,
        in_0.offset(
            (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(2 as usize) as isize,
        ),
    ) {
        return LZMA_OPTIONS_ERROR;
    }
    (*options).backward_size =
        read32le(in_0.offset(::core::mem::size_of::<uint32_t>() as usize as isize)) as lzma_vli;
    (*options).backward_size = (*options)
        .backward_size
        .wrapping_add(1 as lzma_vli)
        .wrapping_mul(4 as lzma_vli);
    return LZMA_OK;
}
