extern "C" {
    fn lzma_simple_coder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
        filter: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                uint32_t,
                bool,
                *mut uint8_t,
                size_t,
            ) -> size_t,
        >,
        simple_size: size_t,
        unfiltered_max: size_t,
        alignment: uint32_t,
        is_encoder: bool,
    ) -> lzma_ret;
}
pub type __darwin_size_t = usize;
pub type uintptr_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_info_s {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub options: *mut ::core::ffi::c_void,
}
pub type lzma_init_function = Option<
    unsafe extern "C" fn(
        *mut lzma_next_coder,
        *const lzma_allocator,
        *const lzma_filter_info,
    ) -> lzma_ret,
>;
pub type lzma_filter_info = lzma_filter_info_s;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn powerpc_code(
    mut simple: *mut ::core::ffi::c_void,
    mut now_pos: uint32_t,
    mut is_encoder: bool,
    mut buffer: *mut uint8_t,
    mut size: size_t,
) -> size_t {
    size &= !(3 as ::core::ffi::c_int as size_t);
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < size {
        if *buffer.offset(i as isize) as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
            == 0x12 as ::core::ffi::c_int
            && *buffer.offset(i.wrapping_add(3 as size_t) as isize) as ::core::ffi::c_int
                & 3 as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
        {
            let src: uint32_t = (*buffer.offset(i.wrapping_add(0 as size_t) as isize) as uint32_t
                & 3 as uint32_t)
                << 24 as ::core::ffi::c_int
                | (*buffer.offset(i.wrapping_add(1 as size_t) as isize) as uint32_t)
                    << 16 as ::core::ffi::c_int
                | (*buffer.offset(i.wrapping_add(2 as size_t) as isize) as uint32_t)
                    << 8 as ::core::ffi::c_int
                | *buffer.offset(i.wrapping_add(3 as size_t) as isize) as uint32_t
                    & !(3 as uint32_t);
            let mut dest: uint32_t = 0;
            if is_encoder {
                dest = now_pos.wrapping_add(i as uint32_t).wrapping_add(src);
            } else {
                dest = src.wrapping_sub(now_pos.wrapping_add(i as uint32_t));
            }
            *buffer.offset(i.wrapping_add(0 as size_t) as isize) =
                (0x48 as uint32_t | dest >> 24 as ::core::ffi::c_int & 0x3 as uint32_t) as uint8_t;
            *buffer.offset(i.wrapping_add(1 as size_t) as isize) =
                (dest >> 16 as ::core::ffi::c_int) as uint8_t;
            *buffer.offset(i.wrapping_add(2 as size_t) as isize) =
                (dest >> 8 as ::core::ffi::c_int) as uint8_t;
            let ref mut fresh0 = *buffer.offset(i.wrapping_add(3 as size_t) as isize);
            *fresh0 = (*fresh0 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int) as uint8_t;
            let ref mut fresh1 = *buffer.offset(i.wrapping_add(3 as size_t) as isize);
            *fresh1 = (*fresh1 as uint32_t | dest) as uint8_t;
        }
        i = i.wrapping_add(4 as size_t);
    }
    return i;
}
unsafe extern "C" fn powerpc_coder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
    mut is_encoder: bool,
) -> lzma_ret {
    return lzma_simple_coder_init(
        next,
        allocator,
        filters,
        Some(
            powerpc_code
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    uint32_t,
                    bool,
                    *mut uint8_t,
                    size_t,
                ) -> size_t,
        ),
        0 as size_t,
        4 as size_t,
        4 as uint32_t,
        is_encoder,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_powerpc_encoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
) -> lzma_ret {
    return powerpc_coder_init(next, allocator, filters, true_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_powerpc_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
) -> lzma_ret {
    return powerpc_coder_init(next, allocator, filters, false_0 != 0);
}
