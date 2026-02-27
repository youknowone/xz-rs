extern "C" {
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_crc32(buf: *const uint8_t, size: size_t, crc: uint32_t) -> uint32_t;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut ::core::ffi::c_void;
    fn lzma_free(ptr: *mut ::core::ffi::c_void, allocator: *const lzma_allocator);
    fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret;
    fn lzma_next_filter_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_bufcpy(
        in_0: *const uint8_t,
        in_pos: *mut size_t,
        in_size: size_t,
        out: *mut uint8_t,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> size_t;
    fn lzma_lzma_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_lzma_decoder_memusage_nocheck(options: *const ::core::ffi::c_void) -> uint64_t;
}
pub type __darwin_size_t = usize;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_internal_s {
    pub next: lzma_next_coder,
    pub sequence: C2RustUnnamed,
    pub avail_in: size_t,
    pub supported_actions: [bool; 5],
    pub allow_buf_error: bool,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
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
pub type lzma_internal = lzma_internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream {
    pub next_in: *const uint8_t,
    pub avail_in: size_t,
    pub total_in: uint64_t,
    pub next_out: *mut uint8_t,
    pub avail_out: size_t,
    pub total_out: uint64_t,
    pub allocator: *const lzma_allocator,
    pub internal: *mut lzma_internal,
    pub reserved_ptr1: *mut ::core::ffi::c_void,
    pub reserved_ptr2: *mut ::core::ffi::c_void,
    pub reserved_ptr3: *mut ::core::ffi::c_void,
    pub reserved_ptr4: *mut ::core::ffi::c_void,
    pub seek_pos: uint64_t,
    pub reserved_int2: uint64_t,
    pub reserved_int3: size_t,
    pub reserved_int4: size_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
}
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
pub struct lzma_lzip_coder {
    pub sequence: C2RustUnnamed_0,
    pub version: uint32_t,
    pub crc32: uint32_t,
    pub uncompressed_size: uint64_t,
    pub member_size: uint64_t,
    pub memlimit: uint64_t,
    pub memusage: uint64_t,
    pub tell_any_check: bool,
    pub ignore_check: bool,
    pub concatenated: bool,
    pub first_member: bool,
    pub pos: size_t,
    pub buffer: [uint8_t; 20],
    pub options: lzma_options_lzma,
    pub lzma_decoder: lzma_next_coder,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const SEQ_MEMBER_FOOTER: C2RustUnnamed_0 = 5;
pub const SEQ_LZMA_STREAM: C2RustUnnamed_0 = 4;
pub const SEQ_CODER_INIT: C2RustUnnamed_0 = 3;
pub const SEQ_DICT_SIZE: C2RustUnnamed_0 = 2;
pub const SEQ_VERSION: C2RustUnnamed_0 = 1;
pub const SEQ_ID_STRING: C2RustUnnamed_0 = 0;
pub type lzma_filter_info = lzma_filter_info_s;
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
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
#[inline]
unsafe extern "C" fn read64le(mut buf: *const uint8_t) -> uint64_t {
    let mut num: uint64_t = *buf.offset(0 as ::core::ffi::c_int as isize) as uint64_t;
    num |= (*buf.offset(1 as ::core::ffi::c_int as isize) as uint64_t) << 8 as ::core::ffi::c_int;
    num |= (*buf.offset(2 as ::core::ffi::c_int as isize) as uint64_t) << 16 as ::core::ffi::c_int;
    num |= (*buf.offset(3 as ::core::ffi::c_int as isize) as uint64_t) << 24 as ::core::ffi::c_int;
    num |= (*buf.offset(4 as ::core::ffi::c_int as isize) as uint64_t) << 32 as ::core::ffi::c_int;
    num |= (*buf.offset(5 as ::core::ffi::c_int as isize) as uint64_t) << 40 as ::core::ffi::c_int;
    num |= (*buf.offset(6 as ::core::ffi::c_int as isize) as uint64_t) << 48 as ::core::ffi::c_int;
    num |= (*buf.offset(7 as ::core::ffi::c_int as isize) as uint64_t) << 56 as ::core::ffi::c_int;
    return num;
}
pub const LZMA_VLI_UNKNOWN: ::core::ffi::c_ulonglong = UINT64_MAX;
pub const LZMA_FILTER_LZMA1: ::core::ffi::c_ulonglong =
    0x4000000000000001 as ::core::ffi::c_ulonglong;
pub const LZMA_TELL_NO_CHECK: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const LZMA_TELL_UNSUPPORTED_CHECK: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const LZMA_TELL_ANY_CHECK: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const LZMA_IGNORE_CHECK: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const LZMA_CONCATENATED: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const LZMA_FAIL_FAST: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const LZMA_MEMUSAGE_BASE: ::core::ffi::c_ulonglong =
    (1 as ::core::ffi::c_ulonglong) << 15 as ::core::ffi::c_int;
pub const LZMA_SUPPORTED_FLAGS: ::core::ffi::c_uint = LZMA_TELL_NO_CHECK
    | LZMA_TELL_UNSUPPORTED_CHECK
    | LZMA_TELL_ANY_CHECK
    | LZMA_IGNORE_CHECK
    | LZMA_CONCATENATED
    | LZMA_FAIL_FAST;
pub const LZIP_V0_FOOTER_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const LZIP_V1_FOOTER_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const LZIP_LC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LZIP_LP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LZIP_PB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn lzip_decode(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut allocator: *const lzma_allocator,
    mut in_0: *const uint8_t,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut action: lzma_action,
) -> lzma_ret {
    let mut coder: *mut lzma_lzip_coder = coder_ptr as *mut lzma_lzip_coder;
    loop {
        let mut current_block_80: u64;
        match (*coder).sequence as ::core::ffi::c_uint {
            0 => {
                let lzip_id_string: [uint8_t; 4] = [
                    0x4c as ::core::ffi::c_int as uint8_t,
                    0x5a as ::core::ffi::c_int as uint8_t,
                    0x49 as ::core::ffi::c_int as uint8_t,
                    0x50 as ::core::ffi::c_int as uint8_t,
                ];
                while (*coder).pos < ::core::mem::size_of::<[uint8_t; 4]>() as usize {
                    if *in_pos >= in_size {
                        return (if !(*coder).first_member
                            && action as ::core::ffi::c_uint
                                == LZMA_FINISH as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            LZMA_STREAM_END as ::core::ffi::c_int
                        } else {
                            LZMA_OK as ::core::ffi::c_int
                        }) as lzma_ret;
                    }
                    if *in_0.offset(*in_pos as isize) as ::core::ffi::c_int
                        != lzip_id_string[(*coder).pos as usize] as ::core::ffi::c_int
                    {
                        return (if !(*coder).first_member {
                            LZMA_STREAM_END as ::core::ffi::c_int
                        } else {
                            LZMA_FORMAT_ERROR as ::core::ffi::c_int
                        }) as lzma_ret;
                    }
                    *in_pos = (*in_pos).wrapping_add(1);
                    (*coder).pos = (*coder).pos.wrapping_add(1);
                }
                (*coder).pos = 0 as size_t;
                (*coder).crc32 = 0 as uint32_t;
                (*coder).uncompressed_size = 0 as uint64_t;
                (*coder).member_size = ::core::mem::size_of::<[uint8_t; 4]>() as uint64_t;
                (*coder).sequence = SEQ_VERSION;
                current_block_80 = 11220331375136032509;
            }
            1 => {
                current_block_80 = 11220331375136032509;
            }
            2 => {
                current_block_80 = 2770508642018830579;
            }
            3 => {
                current_block_80 = 15476230294461844687;
            }
            4 => {
                current_block_80 = 13394712405657322686;
            }
            5 => {
                current_block_80 = 13619784596304402172;
            }
            _ => return LZMA_PROG_ERROR,
        }
        match current_block_80 {
            11220331375136032509 => {
                if *in_pos >= in_size {
                    return LZMA_OK;
                }
                let fresh0 = *in_pos;
                *in_pos = (*in_pos).wrapping_add(1);
                (*coder).version = *in_0.offset(fresh0 as isize) as uint32_t;
                if (*coder).version > 1 as uint32_t {
                    return LZMA_OPTIONS_ERROR;
                }
                (*coder).member_size = (*coder).member_size.wrapping_add(1);
                (*coder).sequence = SEQ_DICT_SIZE;
                if (*coder).tell_any_check {
                    return LZMA_GET_CHECK;
                }
                current_block_80 = 2770508642018830579;
            }
            _ => {}
        }
        match current_block_80 {
            2770508642018830579 => {
                if *in_pos >= in_size {
                    return LZMA_OK;
                }
                let fresh1 = *in_pos;
                *in_pos = (*in_pos).wrapping_add(1);
                let ds: uint32_t = *in_0.offset(fresh1 as isize) as uint32_t;
                (*coder).member_size = (*coder).member_size.wrapping_add(1);
                let b2log: uint32_t = ds & 0x1f as uint32_t;
                let fracnum: uint32_t = ds >> 5 as ::core::ffi::c_int;
                if b2log < 12 as uint32_t
                    || b2log > 29 as uint32_t
                    || b2log == 12 as uint32_t && fracnum > 0 as uint32_t
                {
                    return LZMA_DATA_ERROR;
                }
                (*coder).options.dict_size = ((1 as uint32_t) << b2log)
                    .wrapping_sub(fracnum << b2log.wrapping_sub(4 as uint32_t));
                (*coder).options.preset_dict = ::core::ptr::null::<uint8_t>();
                (*coder).options.lc = LZIP_LC as uint32_t;
                (*coder).options.lp = LZIP_LP as uint32_t;
                (*coder).options.pb = LZIP_PB as uint32_t;
                (*coder).memusage = lzma_lzma_decoder_memusage_nocheck(
                    &raw mut (*coder).options as *const ::core::ffi::c_void,
                )
                .wrapping_add(LZMA_MEMUSAGE_BASE as uint64_t);
                (*coder).sequence = SEQ_CODER_INIT;
                current_block_80 = 15476230294461844687;
            }
            _ => {}
        }
        match current_block_80 {
            15476230294461844687 => {
                if (*coder).memusage > (*coder).memlimit {
                    return LZMA_MEMLIMIT_ERROR;
                }
                let filters: [lzma_filter_info; 2] = [
                    lzma_filter_info_s {
                        id: LZMA_FILTER_LZMA1 as lzma_vli,
                        init: Some(
                            lzma_lzma_decoder_init
                                as unsafe extern "C" fn(
                                    *mut lzma_next_coder,
                                    *const lzma_allocator,
                                    *const lzma_filter_info,
                                )
                                    -> lzma_ret,
                        ),
                        options: &raw mut (*coder).options as *mut ::core::ffi::c_void,
                    },
                    lzma_filter_info_s {
                        id: 0,
                        init: None,
                        options: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    },
                ];
                let ret_: lzma_ret = lzma_next_filter_init(
                    &raw mut (*coder).lzma_decoder,
                    allocator,
                    &raw const filters as *const lzma_filter_info,
                ) as lzma_ret;
                if ret_ as ::core::ffi::c_uint
                    != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return ret_;
                }
                (*coder).crc32 = 0 as uint32_t;
                (*coder).sequence = SEQ_LZMA_STREAM;
                current_block_80 = 13394712405657322686;
            }
            _ => {}
        }
        match current_block_80 {
            13394712405657322686 => {
                let in_start: size_t = *in_pos;
                let out_start: size_t = *out_pos;
                let ret: lzma_ret = (*coder)
                    .lzma_decoder
                    .code
                    .expect("non-null function pointer")(
                    (*coder).lzma_decoder.coder,
                    allocator,
                    in_0,
                    in_pos,
                    in_size,
                    out,
                    out_pos,
                    out_size,
                    action,
                ) as lzma_ret;
                let out_used: size_t = (*out_pos).wrapping_sub(out_start);
                (*coder).member_size = (*coder)
                    .member_size
                    .wrapping_add((*in_pos).wrapping_sub(in_start) as uint64_t);
                (*coder).uncompressed_size = (*coder)
                    .uncompressed_size
                    .wrapping_add(out_used as uint64_t);
                if !(*coder).ignore_check && out_used > 0 as size_t {
                    (*coder).crc32 =
                        lzma_crc32(out.offset(out_start as isize), out_used, (*coder).crc32);
                }
                if ret as ::core::ffi::c_uint
                    != LZMA_STREAM_END as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return ret;
                }
                (*coder).sequence = SEQ_MEMBER_FOOTER;
            }
            _ => {}
        }
        let footer_size: size_t = (if (*coder).version == 0 as uint32_t {
            LZIP_V0_FOOTER_SIZE
        } else {
            LZIP_V1_FOOTER_SIZE
        }) as size_t;
        lzma_bufcpy(
            in_0,
            in_pos,
            in_size,
            &raw mut (*coder).buffer as *mut uint8_t,
            &raw mut (*coder).pos,
            footer_size,
        );
        if (*coder).pos < footer_size {
            return LZMA_OK;
        }
        (*coder).pos = 0 as size_t;
        (*coder).member_size = (*coder).member_size.wrapping_add(footer_size as uint64_t);
        if !(*coder).ignore_check
            && (*coder).crc32
                != read32le(
                    (&raw mut (*coder).buffer as *mut uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut uint8_t,
                )
        {
            return LZMA_DATA_ERROR;
        }
        if (*coder).uncompressed_size
            != read64le(
                (&raw mut (*coder).buffer as *mut uint8_t).offset(4 as ::core::ffi::c_int as isize)
                    as *mut uint8_t,
            )
        {
            return LZMA_DATA_ERROR;
        }
        if (*coder).version > 0 as uint32_t {
            if (*coder).member_size
                != read64le(
                    (&raw mut (*coder).buffer as *mut uint8_t)
                        .offset(12 as ::core::ffi::c_int as isize)
                        as *mut uint8_t,
                )
            {
                return LZMA_DATA_ERROR;
            }
        }
        if !(*coder).concatenated {
            return LZMA_STREAM_END;
        }
        (*coder).first_member = false_0 != 0;
        (*coder).sequence = SEQ_ID_STRING;
    }
}
unsafe extern "C" fn lzip_decoder_end(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_lzip_coder = coder_ptr as *mut lzma_lzip_coder;
    lzma_next_end(&raw mut (*coder).lzma_decoder, allocator);
    lzma_free(coder as *mut ::core::ffi::c_void, allocator);
}
unsafe extern "C" fn lzip_decoder_get_check(
    mut coder_ptr: *const ::core::ffi::c_void,
) -> lzma_check {
    return LZMA_CHECK_CRC32;
}
unsafe extern "C" fn lzip_decoder_memconfig(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut memusage: *mut uint64_t,
    mut old_memlimit: *mut uint64_t,
    mut new_memlimit: uint64_t,
) -> lzma_ret {
    let mut coder: *mut lzma_lzip_coder = coder_ptr as *mut lzma_lzip_coder;
    *memusage = (*coder).memusage;
    *old_memlimit = (*coder).memlimit;
    if new_memlimit != 0 as uint64_t {
        if new_memlimit < (*coder).memusage {
            return LZMA_MEMLIMIT_ERROR;
        }
        (*coder).memlimit = new_memlimit;
    }
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzip_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut memlimit: uint64_t,
    mut flags: uint32_t,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                uint64_t,
                uint32_t,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_lzip_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                uint64_t,
                uint32_t,
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
                uint64_t,
                uint32_t,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_lzip_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                uint64_t,
                uint32_t,
            ) -> lzma_ret,
    ));
    if flags & !(LZMA_SUPPORTED_FLAGS as uint32_t) != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut coder: *mut lzma_lzip_coder = (*next).coder as *mut lzma_lzip_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            ::core::mem::size_of::<lzma_lzip_coder>() as size_t,
            allocator,
        ) as *mut lzma_lzip_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut ::core::ffi::c_void;
        (*next).code = Some(
            lzip_decode
                as unsafe extern "C" fn(
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
        ) as lzma_code_function;
        (*next).end = Some(
            lzip_decoder_end
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).get_check = Some(
            lzip_decoder_get_check
                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> lzma_check,
        )
            as Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> lzma_check>;
        (*next).memconfig = Some(
            lzip_decoder_memconfig
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut uint64_t,
                    *mut uint64_t,
                    uint64_t,
                ) -> lzma_ret,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut uint64_t,
                    *mut uint64_t,
                    uint64_t,
                ) -> lzma_ret,
            >;
        (*coder).lzma_decoder = lzma_next_coder_s {
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
    }
    (*coder).sequence = SEQ_ID_STRING;
    (*coder).memlimit = if 1 as uint64_t > memlimit {
        1 as uint64_t
    } else {
        memlimit
    };
    (*coder).memusage = LZMA_MEMUSAGE_BASE as uint64_t;
    (*coder).tell_any_check = flags & LZMA_TELL_ANY_CHECK as uint32_t != 0 as uint32_t;
    (*coder).ignore_check = flags & LZMA_IGNORE_CHECK as uint32_t != 0 as uint32_t;
    (*coder).concatenated = flags & LZMA_CONCATENATED as uint32_t != 0 as uint32_t;
    (*coder).first_member = true_0 != 0;
    (*coder).pos = 0 as size_t;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lzip_decoder(
    mut strm: *mut lzma_stream,
    mut memlimit: uint64_t,
    mut flags: uint32_t,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret_;
    }
    let ret__0: lzma_ret = lzma_lzip_decoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        memlimit,
        flags,
    ) as lzma_ret;
    if ret__0 as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as ::core::ffi::c_int as usize] = true_0 != 0;
    (*(*strm).internal).supported_actions[LZMA_FINISH as ::core::ffi::c_int as usize] = true_0 != 0;
    return LZMA_OK;
}
