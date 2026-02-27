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
pub struct lzma_simple_x86 {
    pub prev_mask: uint32_t,
    pub prev_pos: uint32_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_simple_coder {
    pub next: lzma_next_coder,
    pub end_was_reached: bool,
    pub is_encoder: bool,
    pub filter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            uint32_t,
            bool,
            *mut uint8_t,
            size_t,
        ) -> size_t,
    >,
    pub simple: *mut ::core::ffi::c_void,
    pub now_pos: uint32_t,
    pub allocated: size_t,
    pub pos: size_t,
    pub filtered: size_t,
    pub size: size_t,
    pub buffer: [uint8_t; 0],
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn x86_code(
    mut simple_ptr: *mut ::core::ffi::c_void,
    mut now_pos: uint32_t,
    mut is_encoder: bool,
    mut buffer: *mut uint8_t,
    mut size: size_t,
) -> size_t {
    static mut MASK_TO_BIT_NUMBER: [uint32_t; 5] = [
        0 as ::core::ffi::c_int as uint32_t,
        1 as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int as uint32_t,
        2 as ::core::ffi::c_int as uint32_t,
        3 as ::core::ffi::c_int as uint32_t,
    ];
    let mut simple: *mut lzma_simple_x86 = simple_ptr as *mut lzma_simple_x86;
    let mut prev_mask: uint32_t = (*simple).prev_mask;
    let mut prev_pos: uint32_t = (*simple).prev_pos;
    if size < 5 as size_t {
        return 0 as size_t;
    }
    if now_pos.wrapping_sub(prev_pos) > 5 as uint32_t {
        prev_pos = now_pos.wrapping_sub(5 as uint32_t);
    }
    let limit: size_t = size.wrapping_sub(5 as size_t);
    let mut buffer_pos: size_t = 0 as size_t;
    while buffer_pos <= limit {
        let mut b: uint8_t = *buffer.offset(buffer_pos as isize);
        if b as ::core::ffi::c_int != 0xe8 as ::core::ffi::c_int
            && b as ::core::ffi::c_int != 0xe9 as ::core::ffi::c_int
        {
            buffer_pos = buffer_pos.wrapping_add(1);
        } else {
            let offset: uint32_t = now_pos
                .wrapping_add(buffer_pos as uint32_t)
                .wrapping_sub(prev_pos);
            prev_pos = now_pos.wrapping_add(buffer_pos as uint32_t);
            if offset > 5 as uint32_t {
                prev_mask = 0 as uint32_t;
            } else {
                let mut i: uint32_t = 0 as uint32_t;
                while i < offset {
                    prev_mask &= 0x77 as uint32_t;
                    prev_mask <<= 1 as ::core::ffi::c_int;
                    i = i.wrapping_add(1);
                }
            }
            b = *buffer.offset(buffer_pos.wrapping_add(4 as size_t) as isize);
            if (b as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || b as ::core::ffi::c_int == 0xff as ::core::ffi::c_int)
                && prev_mask >> 1 as ::core::ffi::c_int <= 4 as uint32_t
                && prev_mask >> 1 as ::core::ffi::c_int != 3 as uint32_t
            {
                let mut src: uint32_t = (b as uint32_t) << 24 as ::core::ffi::c_int
                    | (*buffer.offset(buffer_pos.wrapping_add(3 as size_t) as isize) as uint32_t)
                        << 16 as ::core::ffi::c_int
                    | (*buffer.offset(buffer_pos.wrapping_add(2 as size_t) as isize) as uint32_t)
                        << 8 as ::core::ffi::c_int
                    | *buffer.offset(buffer_pos.wrapping_add(1 as size_t) as isize) as uint32_t;
                let mut dest: uint32_t = 0;
                loop {
                    if is_encoder {
                        dest = src.wrapping_add(
                            now_pos
                                .wrapping_add(buffer_pos as uint32_t)
                                .wrapping_add(5 as uint32_t),
                        );
                    } else {
                        dest = src.wrapping_sub(
                            now_pos
                                .wrapping_add(buffer_pos as uint32_t)
                                .wrapping_add(5 as uint32_t),
                        );
                    }
                    if prev_mask == 0 as uint32_t {
                        break;
                    }
                    let i_0: uint32_t =
                        MASK_TO_BIT_NUMBER[(prev_mask >> 1 as ::core::ffi::c_int) as usize];
                    b = (dest >> (24 as uint32_t).wrapping_sub(i_0.wrapping_mul(8 as uint32_t)))
                        as uint8_t;
                    if !(b as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        || b as ::core::ffi::c_int == 0xff as ::core::ffi::c_int)
                    {
                        break;
                    }
                    src = dest
                        ^ ((1 as uint32_t)
                            << (32 as uint32_t).wrapping_sub(i_0.wrapping_mul(8 as uint32_t)))
                        .wrapping_sub(1 as uint32_t);
                }
                *buffer.offset(buffer_pos.wrapping_add(4 as size_t) as isize) =
                    !(dest >> 24 as ::core::ffi::c_int & 1 as uint32_t).wrapping_sub(1 as uint32_t)
                        as uint8_t;
                *buffer.offset(buffer_pos.wrapping_add(3 as size_t) as isize) =
                    (dest >> 16 as ::core::ffi::c_int) as uint8_t;
                *buffer.offset(buffer_pos.wrapping_add(2 as size_t) as isize) =
                    (dest >> 8 as ::core::ffi::c_int) as uint8_t;
                *buffer.offset(buffer_pos.wrapping_add(1 as size_t) as isize) = dest as uint8_t;
                buffer_pos = buffer_pos.wrapping_add(5 as size_t);
                prev_mask = 0 as uint32_t;
            } else {
                buffer_pos = buffer_pos.wrapping_add(1);
                prev_mask |= 1 as uint32_t;
                if b as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || b as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
                {
                    prev_mask |= 0x10 as uint32_t;
                }
            }
        }
    }
    (*simple).prev_mask = prev_mask;
    (*simple).prev_pos = prev_pos;
    return buffer_pos;
}
unsafe extern "C" fn x86_coder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
    mut is_encoder: bool,
) -> lzma_ret {
    let ret: lzma_ret = lzma_simple_coder_init(
        next,
        allocator,
        filters,
        Some(
            x86_code
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    uint32_t,
                    bool,
                    *mut uint8_t,
                    size_t,
                ) -> size_t,
        ),
        ::core::mem::size_of::<lzma_simple_x86>() as size_t,
        5 as size_t,
        1 as uint32_t,
        is_encoder,
    ) as lzma_ret;
    if ret as ::core::ffi::c_uint == LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut coder: *mut lzma_simple_coder = (*next).coder as *mut lzma_simple_coder;
        let mut simple: *mut lzma_simple_x86 = (*coder).simple as *mut lzma_simple_x86;
        (*simple).prev_mask = 0 as uint32_t;
        (*simple).prev_pos = -(5 as ::core::ffi::c_int) as uint32_t;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_x86_encoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
) -> lzma_ret {
    return x86_coder_init(next, allocator, filters, true_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_x86_encode(
    mut start_offset: uint32_t,
    mut buf: *mut uint8_t,
    mut size: size_t,
) -> size_t {
    let mut simple: lzma_simple_x86 = lzma_simple_x86 {
        prev_mask: 0 as uint32_t,
        prev_pos: -(5 as ::core::ffi::c_int) as uint32_t,
    };
    return x86_code(
        &raw mut simple as *mut ::core::ffi::c_void,
        start_offset,
        true_0 != 0,
        buf,
        size,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_x86_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
) -> lzma_ret {
    return x86_coder_init(next, allocator, filters, false_0 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_x86_decode(
    mut start_offset: uint32_t,
    mut buf: *mut uint8_t,
    mut size: size_t,
) -> size_t {
    let mut simple: lzma_simple_x86 = lzma_simple_x86 {
        prev_mask: 0 as uint32_t,
        prev_pos: -(5 as ::core::ffi::c_int) as uint32_t,
    };
    return x86_code(
        &raw mut simple as *mut ::core::ffi::c_void,
        start_offset,
        false_0 != 0,
        buf,
        size,
    );
}
