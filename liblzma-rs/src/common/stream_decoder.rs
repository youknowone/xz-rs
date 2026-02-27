use crate::types::*;
use core::ffi::{c_int, c_uchar, c_uint, c_ulonglong, c_void};
#[repr(C)]
pub struct lzma_index_hash_s {
    _opaque: [u8; 0],
}
extern "C" {
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_check_is_supported(check: lzma_check) -> lzma_bool;
    fn lzma_filters_free(filters: *mut lzma_filter, allocator: *const lzma_allocator);
    fn lzma_raw_decoder_memusage(filters: *const lzma_filter) -> u64;
    fn lzma_stream_header_decode(options: *mut lzma_stream_flags, in_0: *const u8) -> lzma_ret;
    fn lzma_stream_footer_decode(options: *mut lzma_stream_flags, in_0: *const u8) -> lzma_ret;
    fn lzma_stream_flags_compare(
        a: *const lzma_stream_flags,
        b: *const lzma_stream_flags,
    ) -> lzma_ret;
    fn lzma_block_header_decode(
        block: *mut lzma_block,
        allocator: *const lzma_allocator,
        in_0: *const u8,
    ) -> lzma_ret;
    fn lzma_block_unpadded_size(block: *const lzma_block) -> lzma_vli;
    fn lzma_index_hash_init(
        index_hash: *mut lzma_index_hash,
        allocator: *const lzma_allocator,
    ) -> *mut lzma_index_hash;
    fn lzma_index_hash_end(index_hash: *mut lzma_index_hash, allocator: *const lzma_allocator);
    fn lzma_index_hash_append(
        index_hash: *mut lzma_index_hash,
        unpadded_size: lzma_vli,
        uncompressed_size: lzma_vli,
    ) -> lzma_ret;
    fn lzma_index_hash_decode(
        index_hash: *mut lzma_index_hash,
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
    ) -> lzma_ret;
    fn lzma_index_hash_size(index_hash: *const lzma_index_hash) -> lzma_vli;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_bufcpy(
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> size_t;
    fn lzma_block_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        block: *mut lzma_block,
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
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, size_t, size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    pub opaque: *mut c_void,
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
pub type C2RustUnnamed = c_uint;
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
    pub coder: *mut c_void,
    pub id: lzma_vli,
    pub init: uintptr_t,
    pub code: lzma_code_function,
    pub end: lzma_end_function,
    pub get_progress: Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64) -> ()>,
    pub get_check: Option<unsafe extern "C" fn(*const c_void) -> lzma_check>,
    pub memconfig: Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret>,
    pub update: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const lzma_allocator,
            *const lzma_filter,
            *const lzma_filter,
        ) -> lzma_ret,
    >,
    pub set_out_limit: Option<unsafe extern "C" fn(*mut c_void, *mut u64, u64) -> lzma_ret>,
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
pub type lzma_end_function = Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>;
pub type lzma_code_function = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const lzma_allocator,
        *const u8,
        *mut size_t,
        size_t,
        *mut u8,
        *mut size_t,
        size_t,
        lzma_action,
    ) -> lzma_ret,
>;
pub type lzma_internal = lzma_internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream {
    pub next_in: *const u8,
    pub avail_in: size_t,
    pub total_in: u64,
    pub next_out: *mut u8,
    pub avail_out: size_t,
    pub total_out: u64,
    pub allocator: *const lzma_allocator,
    pub internal: *mut lzma_internal,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
    pub reserved_ptr3: *mut c_void,
    pub reserved_ptr4: *mut c_void,
    pub seek_pos: u64,
    pub reserved_int2: u64,
    pub reserved_int3: size_t,
    pub reserved_int4: size_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream_coder {
    pub sequence: C2RustUnnamed_0,
    pub block_decoder: lzma_next_coder,
    pub block_options: lzma_block,
    pub stream_flags: lzma_stream_flags,
    pub index_hash: *mut lzma_index_hash,
    pub memlimit: u64,
    pub memusage: u64,
    pub tell_no_check: bool,
    pub tell_unsupported_check: bool,
    pub tell_any_check: bool,
    pub ignore_check: bool,
    pub concatenated: bool,
    pub first_stream: bool,
    pub pos: size_t,
    pub buffer: [u8; LZMA_BLOCK_HEADER_SIZE_MAX as usize],
}
pub const LZMA_BLOCK_HEADER_SIZE_MAX: c_int = 1024 as c_int;
pub type lzma_index_hash = lzma_index_hash_s;
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
pub type C2RustUnnamed_0 = c_uint;
pub const SEQ_STREAM_PADDING: C2RustUnnamed_0 = 6;
pub const SEQ_STREAM_FOOTER: C2RustUnnamed_0 = 5;
pub const SEQ_INDEX: C2RustUnnamed_0 = 4;
pub const SEQ_BLOCK_RUN: C2RustUnnamed_0 = 3;
pub const SEQ_BLOCK_INIT: C2RustUnnamed_0 = 2;
pub const SEQ_BLOCK_HEADER: C2RustUnnamed_0 = 1;
pub const SEQ_STREAM_HEADER: C2RustUnnamed_0 = 0;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_TELL_NO_CHECK: c_uint = 0x1;
pub const LZMA_TELL_UNSUPPORTED_CHECK: c_uint = 0x2;
pub const LZMA_TELL_ANY_CHECK: c_uint = 0x4;
pub const LZMA_IGNORE_CHECK: c_uint = 0x10;
pub const LZMA_CONCATENATED: c_uint = 0x8;
pub const LZMA_FAIL_FAST: c_uint = 0x20;
pub const LZMA_STREAM_HEADER_SIZE: c_int = 12 as c_int;
pub const LZMA_MEMUSAGE_BASE: c_ulonglong = 1 << 15;
pub const LZMA_SUPPORTED_FLAGS: c_uint = LZMA_TELL_NO_CHECK
    | LZMA_TELL_UNSUPPORTED_CHECK
    | LZMA_TELL_ANY_CHECK
    | LZMA_IGNORE_CHECK
    | LZMA_CONCATENATED
    | LZMA_FAIL_FAST;
pub const INDEX_INDICATOR: c_int = 0 as c_int;
unsafe extern "C" fn stream_decoder_reset(
    mut coder: *mut lzma_stream_coder,
    mut allocator: *const lzma_allocator,
) -> lzma_ret {
    (*coder).index_hash = lzma_index_hash_init((*coder).index_hash, allocator);
    if (*coder).index_hash.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*coder).sequence = SEQ_STREAM_HEADER;
    (*coder).pos = 0 as size_t;
    return LZMA_OK;
}
unsafe extern "C" fn stream_decode(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut action: lzma_action,
) -> lzma_ret {
    let mut coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    loop {
        let mut current_block_100: u64;
        match (*coder).sequence {
            0 => {
                lzma_bufcpy(
                    in_0,
                    in_pos,
                    in_size,
                    &raw mut (*coder).buffer as *mut u8,
                    &raw mut (*coder).pos,
                    LZMA_STREAM_HEADER_SIZE as size_t,
                );
                if (*coder).pos < LZMA_STREAM_HEADER_SIZE as size_t {
                    return LZMA_OK;
                }
                (*coder).pos = 0 as size_t;
                let ret: lzma_ret = lzma_stream_header_decode(
                    &raw mut (*coder).stream_flags,
                    &raw mut (*coder).buffer as *mut u8,
                ) as lzma_ret;
                if ret != LZMA_OK {
                    return (if ret == LZMA_FORMAT_ERROR && !(*coder).first_stream {
                        LZMA_DATA_ERROR
                    } else {
                        ret
                    }) as lzma_ret;
                }
                (*coder).first_stream = false_0 != 0;
                (*coder).block_options.check = (*coder).stream_flags.check;
                (*coder).sequence = SEQ_BLOCK_HEADER;
                if (*coder).tell_no_check as c_int != 0
                    && (*coder).stream_flags.check == LZMA_CHECK_NONE
                {
                    return LZMA_NO_CHECK;
                }
                if (*coder).tell_unsupported_check as c_int != 0
                    && lzma_check_is_supported((*coder).stream_flags.check) == 0
                {
                    return LZMA_UNSUPPORTED_CHECK;
                }
                if (*coder).tell_any_check {
                    return LZMA_GET_CHECK;
                }
                current_block_100 = 4166486009154926805;
            }
            1 => {
                current_block_100 = 4166486009154926805;
            }
            2 => {
                current_block_100 = 3500765272169221397;
            }
            3 => {
                current_block_100 = 721385680381463314;
            }
            4 => {
                if *in_pos >= in_size {
                    return LZMA_OK;
                }
                let ret_2: lzma_ret =
                    lzma_index_hash_decode((*coder).index_hash, in_0, in_pos, in_size) as lzma_ret;
                if ret_2 != LZMA_STREAM_END {
                    return ret_2;
                }
                (*coder).sequence = SEQ_STREAM_FOOTER;
                current_block_100 = 17861496924281778896;
            }
            5 => {
                current_block_100 = 17861496924281778896;
            }
            6 => {
                current_block_100 = 15462640364611497761;
            }
            _ => return LZMA_PROG_ERROR,
        }
        match current_block_100 {
            4166486009154926805 => {
                if *in_pos >= in_size {
                    return LZMA_OK;
                }
                if (*coder).pos == 0 as size_t {
                    if *in_0.offset(*in_pos as isize) as c_int == INDEX_INDICATOR {
                        (*coder).sequence = SEQ_INDEX;
                        current_block_100 = 16789764818708874114;
                    } else {
                        (*coder).block_options.header_size = (*in_0.offset(*in_pos as isize)
                            as u32)
                            .wrapping_add(1 as u32)
                            .wrapping_mul(4 as u32);
                        current_block_100 = 13242334135786603907;
                    }
                } else {
                    current_block_100 = 13242334135786603907;
                }
                match current_block_100 {
                    16789764818708874114 => {}
                    _ => {
                        lzma_bufcpy(
                            in_0,
                            in_pos,
                            in_size,
                            &raw mut (*coder).buffer as *mut u8,
                            &raw mut (*coder).pos,
                            (*coder).block_options.header_size as size_t,
                        );
                        if (*coder).pos < (*coder).block_options.header_size as size_t {
                            return LZMA_OK;
                        }
                        (*coder).pos = 0 as size_t;
                        (*coder).sequence = SEQ_BLOCK_INIT;
                        current_block_100 = 3500765272169221397;
                    }
                }
            }
            17861496924281778896 => {
                lzma_bufcpy(
                    in_0,
                    in_pos,
                    in_size,
                    &raw mut (*coder).buffer as *mut u8,
                    &raw mut (*coder).pos,
                    LZMA_STREAM_HEADER_SIZE as size_t,
                );
                if (*coder).pos < LZMA_STREAM_HEADER_SIZE as size_t {
                    return LZMA_OK;
                }
                (*coder).pos = 0 as size_t;
                let mut footer_flags: lzma_stream_flags = lzma_stream_flags {
                    version: 0,
                    backward_size: 0,
                    check: LZMA_CHECK_NONE,
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
                let ret_3: lzma_ret = lzma_stream_footer_decode(
                    &raw mut footer_flags,
                    &raw mut (*coder).buffer as *mut u8,
                ) as lzma_ret;
                if ret_3 != LZMA_OK {
                    return (if ret_3 == LZMA_FORMAT_ERROR {
                        LZMA_DATA_ERROR
                    } else {
                        ret_3
                    }) as lzma_ret;
                }
                if lzma_index_hash_size((*coder).index_hash) != footer_flags.backward_size {
                    return LZMA_DATA_ERROR;
                }
                let ret__1: lzma_ret = lzma_stream_flags_compare(
                    &raw mut (*coder).stream_flags,
                    &raw mut footer_flags,
                ) as lzma_ret;
                if ret__1 != LZMA_OK {
                    return ret__1;
                }
                if !(*coder).concatenated {
                    return LZMA_STREAM_END;
                }
                (*coder).sequence = SEQ_STREAM_PADDING;
                current_block_100 = 15462640364611497761;
            }
            _ => {}
        }
        match current_block_100 {
            15462640364611497761 => {
                loop {
                    if *in_pos >= in_size {
                        if action != LZMA_FINISH {
                            return LZMA_OK;
                        }
                        return (if (*coder).pos == 0 as size_t {
                            LZMA_STREAM_END as c_int
                        } else {
                            LZMA_DATA_ERROR as c_int
                        }) as lzma_ret;
                    }
                    if *in_0.offset(*in_pos as isize) as c_int != 0 as c_int {
                        break;
                    }
                    *in_pos = (*in_pos).wrapping_add(1);
                    (*coder).pos = (*coder).pos.wrapping_add(1 as size_t) & 3 as size_t;
                }
                if (*coder).pos != 0 as size_t {
                    *in_pos = (*in_pos).wrapping_add(1);
                    return LZMA_DATA_ERROR;
                }
                let ret__2: lzma_ret = stream_decoder_reset(coder, allocator) as lzma_ret;
                if ret__2 != LZMA_OK {
                    return ret__2;
                }
                current_block_100 = 16789764818708874114;
            }
            3500765272169221397 => {
                (*coder).block_options.version = 1 as u32;
                let mut filters: [lzma_filter; 5] = [lzma_filter {
                    id: 0,
                    options: ::core::ptr::null_mut::<c_void>(),
                }; 5];
                (*coder).block_options.filters = &raw mut filters as *mut lzma_filter;
                let ret_: lzma_ret = lzma_block_header_decode(
                    &raw mut (*coder).block_options,
                    allocator,
                    &raw mut (*coder).buffer as *mut u8,
                ) as lzma_ret;
                if ret_ != LZMA_OK {
                    return ret_;
                }
                (*coder).block_options.ignore_check = (*coder).ignore_check as lzma_bool;
                let memusage: u64 =
                    lzma_raw_decoder_memusage(&raw mut filters as *mut lzma_filter) as u64;
                let mut ret_0: lzma_ret = LZMA_OK;
                if memusage == UINT64_MAX as u64 {
                    ret_0 = LZMA_OPTIONS_ERROR;
                } else {
                    (*coder).memusage = memusage;
                    if memusage > (*coder).memlimit {
                        ret_0 = LZMA_MEMLIMIT_ERROR;
                    } else {
                        ret_0 = lzma_block_decoder_init(
                            &raw mut (*coder).block_decoder,
                            allocator,
                            &raw mut (*coder).block_options,
                        );
                    }
                }
                lzma_filters_free(&raw mut filters as *mut lzma_filter, allocator);
                (*coder).block_options.filters = ::core::ptr::null_mut::<lzma_filter>();
                if ret_0 != LZMA_OK {
                    return ret_0;
                }
                (*coder).sequence = SEQ_BLOCK_RUN;
                current_block_100 = 721385680381463314;
            }
            _ => {}
        }
        match current_block_100 {
            721385680381463314 => {
                let ret_1: lzma_ret = (*coder)
                    .block_decoder
                    .code
                    .expect("non-null function pointer")(
                    (*coder).block_decoder.coder,
                    allocator,
                    in_0,
                    in_pos,
                    in_size,
                    out,
                    out_pos,
                    out_size,
                    action,
                ) as lzma_ret;
                if ret_1 != LZMA_STREAM_END {
                    return ret_1;
                }
                let ret__0: lzma_ret = lzma_index_hash_append(
                    (*coder).index_hash,
                    lzma_block_unpadded_size(&raw mut (*coder).block_options),
                    (*coder).block_options.uncompressed_size,
                ) as lzma_ret;
                if ret__0 != LZMA_OK {
                    return ret__0;
                }
                (*coder).sequence = SEQ_BLOCK_HEADER;
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn stream_decoder_end(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    lzma_next_end(&raw mut (*coder).block_decoder, allocator);
    lzma_index_hash_end((*coder).index_hash, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn stream_decoder_get_check(mut coder_ptr: *const c_void) -> lzma_check {
    let mut coder: *const lzma_stream_coder = coder_ptr as *const lzma_stream_coder;
    return (*coder).stream_flags.check;
}
unsafe extern "C" fn stream_decoder_memconfig(
    mut coder_ptr: *mut c_void,
    mut memusage: *mut u64,
    mut old_memlimit: *mut u64,
    mut new_memlimit: u64,
) -> lzma_ret {
    let mut coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    *memusage = (*coder).memusage;
    *old_memlimit = (*coder).memlimit;
    if new_memlimit != 0 as u64 {
        if new_memlimit < (*coder).memusage {
            return LZMA_MEMLIMIT_ERROR;
        }
        (*coder).memlimit = new_memlimit;
    }
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut memlimit: u64,
    mut flags: u32,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(*mut lzma_next_coder, *const lzma_allocator, u64, u32) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_stream_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                u32,
            ) -> lzma_ret,
    )) != (*next).init
    {
        lzma_next_end(next, allocator);
    }
    (*next).init = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(*mut lzma_next_coder, *const lzma_allocator, u64, u32) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_stream_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                u32,
            ) -> lzma_ret,
    ));
    if flags & !(LZMA_SUPPORTED_FLAGS as u32) != 0 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut coder: *mut lzma_stream_coder = (*next).coder as *mut lzma_stream_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            ::core::mem::size_of::<lzma_stream_coder>() as size_t,
            allocator,
        ) as *mut lzma_stream_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            stream_decode
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const lzma_allocator,
                    *const u8,
                    *mut size_t,
                    size_t,
                    *mut u8,
                    *mut size_t,
                    size_t,
                    lzma_action,
                ) -> lzma_ret,
        ) as lzma_code_function;
        (*next).end = Some(
            stream_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).get_check =
            Some(stream_decoder_get_check as unsafe extern "C" fn(*const c_void) -> lzma_check)
                as Option<unsafe extern "C" fn(*const c_void) -> lzma_check>;
        (*next).memconfig = Some(
            stream_decoder_memconfig
                as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret,
        )
            as Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret>;
        (*coder).block_decoder = lzma_next_coder_s {
            coder: NULL,
            id: LZMA_VLI_UNKNOWN as lzma_vli,
            init: ::core::ptr::null_mut::<c_void>() as uintptr_t,
            code: None,
            end: None,
            get_progress: None,
            get_check: None,
            memconfig: None,
            update: None,
            set_out_limit: None,
        };
        (*coder).index_hash = ::core::ptr::null_mut::<lzma_index_hash>();
    }
    (*coder).memlimit = if 1 as u64 > memlimit {
        1 as u64
    } else {
        memlimit
    };
    (*coder).memusage = LZMA_MEMUSAGE_BASE as u64;
    (*coder).tell_no_check = flags & LZMA_TELL_NO_CHECK as u32 != 0 as u32;
    (*coder).tell_unsupported_check = flags & LZMA_TELL_UNSUPPORTED_CHECK as u32 != 0 as u32;
    (*coder).tell_any_check = flags & LZMA_TELL_ANY_CHECK as u32 != 0 as u32;
    (*coder).ignore_check = flags & LZMA_IGNORE_CHECK as u32 != 0 as u32;
    (*coder).concatenated = flags & LZMA_CONCATENATED as u32 != 0 as u32;
    (*coder).first_stream = true_0 != 0;
    return stream_decoder_reset(coder, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_decoder(
    mut strm: *mut lzma_stream,
    mut memlimit: u64,
    mut flags: u32,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = lzma_stream_decoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        memlimit,
        flags,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true_0 != 0;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true_0 != 0;
    return LZMA_OK;
}
