use crate::types::*;
use core::ffi::{c_int, c_uchar, c_uint, c_ulonglong, c_void};
extern "C" {
    fn memcmp(__s1: *const c_void, __s2: *const c_void, __n: size_t) -> c_int;
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_check_is_supported(check: lzma_check) -> lzma_bool;
    fn lzma_check_size(check: lzma_check) -> u32;
    fn lzma_block_unpadded_size(block: *const lzma_block) -> lzma_vli;
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
    fn lzma_raw_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        options: *const lzma_filter,
    ) -> lzma_ret;
    fn lzma_check_init(check: *mut lzma_check_state, type_0: lzma_check);
    fn lzma_check_update(
        check: *mut lzma_check_state,
        type_0: lzma_check,
        buf: *const u8,
        size: size_t,
    );
    fn lzma_check_finish(check: *mut lzma_check_state, type_0: lzma_check);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_block_coder {
    pub sequence: C2RustUnnamed_2,
    pub next: lzma_next_coder,
    pub block: *mut lzma_block,
    pub compressed_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub compressed_limit: lzma_vli,
    pub uncompressed_limit: lzma_vli,
    pub check_pos: size_t,
    pub check: lzma_check_state,
    pub ignore_check: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_check_state {
    pub buffer: C2RustUnnamed_1,
    pub state: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub crc32: u32,
    pub crc64: u64,
    pub sha256: lzma_sha256_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_sha256_state {
    pub state: [u32; 8],
    pub size: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub u8_0: [u8; 64],
    pub u32_0: [u32; 16],
    pub u64_0: [u64; 8],
}
pub type C2RustUnnamed_2 = c_uint;
pub const SEQ_CHECK: C2RustUnnamed_2 = 2;
pub const SEQ_PADDING: C2RustUnnamed_2 = 1;
pub const SEQ_CODE: C2RustUnnamed_2 = 0;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = 18446744073709551615;
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
pub const LZMA_VLI_MAX: c_ulonglong = UINT64_MAX.wrapping_div(2);
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
#[inline]
unsafe extern "C" fn is_size_valid(mut size: lzma_vli, mut reference: lzma_vli) -> bool {
    return reference == LZMA_VLI_UNKNOWN as lzma_vli || reference == size;
}
unsafe extern "C" fn block_decode(
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
    let mut coder: *mut lzma_block_coder = coder_ptr as *mut lzma_block_coder;
    's_177: {
        let mut current_block_40: u64;
        match (*coder).sequence {
            0 => {
                let in_start: size_t = *in_pos;
                let out_start: size_t = *out_pos;
                let in_stop: size_t = (*in_pos).wrapping_add(
                    (if (in_size.wrapping_sub(*in_pos) as lzma_vli)
                        < (*coder)
                            .compressed_limit
                            .wrapping_sub((*coder).compressed_size)
                    {
                        in_size.wrapping_sub(*in_pos) as lzma_vli
                    } else {
                        (*coder)
                            .compressed_limit
                            .wrapping_sub((*coder).compressed_size)
                    }) as size_t,
                );
                let out_stop: size_t = (*out_pos).wrapping_add(
                    (if (out_size.wrapping_sub(*out_pos) as lzma_vli)
                        < (*coder)
                            .uncompressed_limit
                            .wrapping_sub((*coder).uncompressed_size)
                    {
                        out_size.wrapping_sub(*out_pos) as lzma_vli
                    } else {
                        (*coder)
                            .uncompressed_limit
                            .wrapping_sub((*coder).uncompressed_size)
                    }) as size_t,
                );
                let ret: lzma_ret = (*coder).next.code.expect("non-null function pointer")(
                    (*coder).next.coder,
                    allocator,
                    in_0,
                    in_pos,
                    in_stop,
                    out,
                    out_pos,
                    out_stop,
                    action,
                ) as lzma_ret;
                let in_used: size_t = (*in_pos).wrapping_sub(in_start);
                let out_used: size_t = (*out_pos).wrapping_sub(out_start);
                (*coder).compressed_size =
                    (*coder).compressed_size.wrapping_add(in_used as lzma_vli);
                (*coder).uncompressed_size = (*coder)
                    .uncompressed_size
                    .wrapping_add(out_used as lzma_vli);
                if ret == LZMA_OK {
                    let comp_done: bool =
                        (*coder).compressed_size == (*(*coder).block).compressed_size;
                    let uncomp_done: bool =
                        (*coder).uncompressed_size == (*(*coder).block).uncompressed_size;
                    if comp_done as c_int != 0 && uncomp_done as c_int != 0 {
                        return LZMA_DATA_ERROR;
                    }
                    if comp_done as c_int != 0 && *out_pos < out_size {
                        return LZMA_DATA_ERROR;
                    }
                    if uncomp_done as c_int != 0 && *in_pos < in_size {
                        return LZMA_DATA_ERROR;
                    }
                }
                if !(*coder).ignore_check && out_used > 0 as size_t {
                    lzma_check_update(
                        &raw mut (*coder).check,
                        (*(*coder).block).check,
                        out.offset(out_start as isize),
                        out_used,
                    );
                }
                if ret != LZMA_STREAM_END {
                    return ret;
                }
                if !is_size_valid((*coder).compressed_size, (*(*coder).block).compressed_size)
                    || !is_size_valid(
                        (*coder).uncompressed_size,
                        (*(*coder).block).uncompressed_size,
                    )
                {
                    return LZMA_DATA_ERROR;
                }
                (*(*coder).block).compressed_size = (*coder).compressed_size;
                (*(*coder).block).uncompressed_size = (*coder).uncompressed_size;
                (*coder).sequence = SEQ_PADDING;
                current_block_40 = 17473121293339793080;
            }
            1 => {
                current_block_40 = 17473121293339793080;
            }
            2 => {
                current_block_40 = 9393557385011460022;
            }
            _ => {
                break 's_177;
            }
        }
        match current_block_40 {
            17473121293339793080 => {
                while (*coder).compressed_size & 3 as lzma_vli != 0 {
                    if *in_pos >= in_size {
                        return LZMA_OK;
                    }
                    (*coder).compressed_size = (*coder).compressed_size.wrapping_add(1);
                    let fresh0 = *in_pos;
                    *in_pos = (*in_pos).wrapping_add(1);
                    if *in_0.offset(fresh0 as isize) as c_int != 0 as c_int {
                        return LZMA_DATA_ERROR;
                    }
                }
                if (*(*coder).block).check == LZMA_CHECK_NONE {
                    return LZMA_STREAM_END;
                }
                if !(*coder).ignore_check {
                    lzma_check_finish(&raw mut (*coder).check, (*(*coder).block).check);
                }
                (*coder).sequence = SEQ_CHECK;
            }
            _ => {}
        }
        let check_size: size_t = lzma_check_size((*(*coder).block).check) as size_t;
        lzma_bufcpy(
            in_0,
            in_pos,
            in_size,
            &raw mut (*(*coder).block).raw_check as *mut u8,
            &raw mut (*coder).check_pos,
            check_size,
        );
        if (*coder).check_pos < check_size {
            return LZMA_OK;
        }
        if !(*coder).ignore_check
            && lzma_check_is_supported((*(*coder).block).check) as c_int != 0
            && memcmp(
                &raw mut (*(*coder).block).raw_check as *mut u8 as *const c_void,
                &raw mut (*coder).check.buffer.u8_0 as *mut u8 as *const c_void,
                check_size,
            ) != 0 as c_int
        {
            return LZMA_DATA_ERROR;
        }
        return LZMA_STREAM_END;
    }
    return LZMA_PROG_ERROR;
}
unsafe extern "C" fn block_decoder_end(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_block_coder = coder_ptr as *mut lzma_block_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut block: *mut lzma_block,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut lzma_block,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_block_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut lzma_block,
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
                *mut lzma_block,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_block_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut lzma_block,
            ) -> lzma_ret,
    ));
    if lzma_block_unpadded_size(block) == 0 as lzma_vli
        || !((*block).uncompressed_size <= LZMA_VLI_MAX as lzma_vli
            || (*block).uncompressed_size == LZMA_VLI_UNKNOWN as lzma_vli)
    {
        return LZMA_PROG_ERROR;
    }
    let mut coder: *mut lzma_block_coder = (*next).coder as *mut lzma_block_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            ::core::mem::size_of::<lzma_block_coder>() as size_t,
            allocator,
        ) as *mut lzma_block_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            block_decode
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
            block_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*coder).next = lzma_next_coder_s {
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
    }
    (*coder).sequence = SEQ_CODE;
    (*coder).block = block;
    (*coder).compressed_size = 0 as lzma_vli;
    (*coder).uncompressed_size = 0 as lzma_vli;
    (*coder).compressed_limit = if (*block).compressed_size == LZMA_VLI_UNKNOWN as lzma_vli {
        (LZMA_VLI_MAX as lzma_vli & !(3 as lzma_vli))
            .wrapping_sub((*block).header_size as lzma_vli)
            .wrapping_sub(lzma_check_size((*block).check) as lzma_vli)
    } else {
        (*block).compressed_size
    };
    (*coder).uncompressed_limit = if (*block).uncompressed_size == LZMA_VLI_UNKNOWN as lzma_vli {
        LZMA_VLI_MAX as lzma_vli
    } else {
        (*block).uncompressed_size
    };
    (*coder).check_pos = 0 as size_t;
    lzma_check_init(&raw mut (*coder).check, (*block).check);
    (*coder).ignore_check = if (*block).version >= 1 as u32 {
        (*block).ignore_check as c_int
    } else {
        false_0
    } != 0;
    return lzma_raw_decoder_init(&raw mut (*coder).next, allocator, (*block).filters);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_block_decoder(
    mut strm: *mut lzma_stream,
    mut block: *mut lzma_block,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret =
        lzma_block_decoder_init(&raw mut (*(*strm).internal).next, (*strm).allocator, block)
            as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true_0 != 0;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true_0 != 0;
    return LZMA_OK;
}
