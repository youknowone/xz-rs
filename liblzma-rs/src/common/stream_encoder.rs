use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
#[repr(C)]
pub struct lzma_index_s {
    _opaque: [u8; 0],
}
extern "C" {
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_filters_copy(
        src: *const lzma_filter,
        dest: *mut lzma_filter,
        allocator: *const lzma_allocator,
    ) -> lzma_ret;
    fn lzma_filters_free(filters: *mut lzma_filter, allocator: *const lzma_allocator);
    fn lzma_stream_header_encode(options: *const lzma_stream_flags, out: *mut u8) -> lzma_ret;
    fn lzma_stream_footer_encode(options: *const lzma_stream_flags, out: *mut u8) -> lzma_ret;
    fn lzma_block_header_size(block: *mut lzma_block) -> lzma_ret;
    fn lzma_block_header_encode(block: *const lzma_block, out: *mut u8) -> lzma_ret;
    fn lzma_block_unpadded_size(block: *const lzma_block) -> lzma_vli;
    fn lzma_index_init(allocator: *const lzma_allocator) -> *mut lzma_index;
    fn lzma_index_end(i: *mut lzma_index, allocator: *const lzma_allocator);
    fn lzma_index_append(
        i: *mut lzma_index,
        allocator: *const lzma_allocator,
        unpadded_size: lzma_vli,
        uncompressed_size: lzma_vli,
    ) -> lzma_ret;
    fn lzma_index_size(i: *const lzma_index) -> lzma_vli;
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
    fn lzma_block_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        block: *mut lzma_block,
    ) -> lzma_ret;
    fn lzma_index_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        i: *const lzma_index,
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
    pub block_encoder_is_initialized: bool,
    pub block_encoder: lzma_next_coder,
    pub block_options: lzma_block,
    pub filters: [lzma_filter; 5],
    pub index_encoder: lzma_next_coder,
    pub index: *mut lzma_index,
    pub buffer_pos: size_t,
    pub buffer_size: size_t,
    pub buffer: [u8; LZMA_BLOCK_HEADER_SIZE_MAX as usize],
}
pub const LZMA_BLOCK_HEADER_SIZE_MAX: c_int = 1024;
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
pub type C2RustUnnamed_0 = c_uint;
pub const SEQ_STREAM_FOOTER: C2RustUnnamed_0 = 5;
pub const SEQ_INDEX_ENCODE: C2RustUnnamed_0 = 4;
pub const SEQ_BLOCK_ENCODE: C2RustUnnamed_0 = 3;
pub const SEQ_BLOCK_HEADER: C2RustUnnamed_0 = 2;
pub const SEQ_BLOCK_INIT: C2RustUnnamed_0 = 1;
pub const SEQ_STREAM_HEADER: C2RustUnnamed_0 = 0;
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
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_STREAM_HEADER_SIZE: c_int = 12;
unsafe extern "C" fn block_encoder_init(
    coder: *mut lzma_stream_coder,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    (*coder).block_options.compressed_size = LZMA_VLI_UNKNOWN as lzma_vli;
    (*coder).block_options.uncompressed_size = LZMA_VLI_UNKNOWN as lzma_vli;
    let ret_: lzma_ret = lzma_block_header_size(&raw mut (*coder).block_options) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    return lzma_block_encoder_init(
        &raw mut (*coder).block_encoder,
        allocator,
        &raw mut (*coder).block_options,
    );
}
unsafe extern "C" fn stream_encode(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
    action: lzma_action,
) -> lzma_ret {
    let coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    while *out_pos < out_size {
        match (*coder).sequence {
            0 | 2 | 5 => {
                lzma_bufcpy(
                    &raw mut (*coder).buffer as *mut u8,
                    &raw mut (*coder).buffer_pos,
                    (*coder).buffer_size,
                    out,
                    out_pos,
                    out_size,
                );
                if (*coder).buffer_pos < (*coder).buffer_size {
                    return LZMA_OK;
                }
                if (*coder).sequence == SEQ_STREAM_FOOTER {
                    return LZMA_STREAM_END;
                }
                (*coder).buffer_pos = 0;
                (*coder).sequence += 1;
            }
            1 => {
                if *in_pos == in_size {
                    if action != LZMA_FINISH {
                        return (if action == LZMA_RUN {
                            LZMA_OK as c_int
                        } else {
                            LZMA_STREAM_END as c_int
                        }) as lzma_ret;
                    }
                    let ret_: lzma_ret = lzma_index_encoder_init(
                        &raw mut (*coder).index_encoder,
                        allocator,
                        (*coder).index,
                    ) as lzma_ret;
                    if ret_ != LZMA_OK {
                        return ret_;
                    }
                    (*coder).sequence = SEQ_INDEX_ENCODE;
                } else {
                    if !(*coder).block_encoder_is_initialized {
                        let ret__0: lzma_ret = block_encoder_init(coder, allocator) as lzma_ret;
                        if ret__0 != LZMA_OK {
                            return ret__0;
                        }
                    }
                    (*coder).block_encoder_is_initialized = false;
                    if lzma_block_header_encode(
                        &raw mut (*coder).block_options,
                        &raw mut (*coder).buffer as *mut u8,
                    ) != LZMA_OK
                    {
                        return LZMA_PROG_ERROR;
                    }
                    (*coder).buffer_size = (*coder).block_options.header_size as size_t;
                    (*coder).sequence = SEQ_BLOCK_HEADER;
                }
            }
            3 => {
                static mut convert: [lzma_action; 5] = [
                    LZMA_RUN,
                    LZMA_SYNC_FLUSH,
                    LZMA_FINISH,
                    LZMA_FINISH,
                    LZMA_FINISH,
                ];
                let ret: lzma_ret = (*coder)
                    .block_encoder
                    .code
                    .expect("non-null function pointer")(
                    (*coder).block_encoder.coder,
                    allocator,
                    in_0,
                    in_pos,
                    in_size,
                    out,
                    out_pos,
                    out_size,
                    convert[action as usize],
                ) as lzma_ret;
                if ret != LZMA_STREAM_END || action == LZMA_SYNC_FLUSH {
                    return ret;
                }
                let unpadded_size: lzma_vli =
                    lzma_block_unpadded_size(&raw mut (*coder).block_options) as lzma_vli;
                let ret__1: lzma_ret = lzma_index_append(
                    (*coder).index,
                    allocator,
                    unpadded_size,
                    (*coder).block_options.uncompressed_size,
                ) as lzma_ret;
                if ret__1 != LZMA_OK {
                    return ret__1;
                }
                (*coder).sequence = SEQ_BLOCK_INIT;
            }
            4 => {
                let ret_0: lzma_ret = (*coder)
                    .index_encoder
                    .code
                    .expect("non-null function pointer")(
                    (*coder).index_encoder.coder,
                    allocator,
                    ::core::ptr::null::<u8>(),
                    core::ptr::null_mut(),
                    0,
                    out,
                    out_pos,
                    out_size,
                    LZMA_RUN,
                ) as lzma_ret;
                if ret_0 != LZMA_STREAM_END {
                    return ret_0;
                }
                let stream_flags: lzma_stream_flags = lzma_stream_flags {
                    version: 0,
                    backward_size: lzma_index_size((*coder).index),
                    check: (*coder).block_options.check,
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
                if lzma_stream_footer_encode(
                    &raw const stream_flags,
                    &raw mut (*coder).buffer as *mut u8,
                ) != LZMA_OK
                {
                    return LZMA_PROG_ERROR;
                }
                (*coder).buffer_size = LZMA_STREAM_HEADER_SIZE as size_t;
                (*coder).sequence = SEQ_STREAM_FOOTER;
            }
            _ => return LZMA_PROG_ERROR,
        }
    }
    return LZMA_OK;
}
unsafe extern "C" fn stream_encoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    lzma_next_end(&raw mut (*coder).block_encoder, allocator);
    lzma_next_end(&raw mut (*coder).index_encoder, allocator);
    lzma_index_end((*coder).index, allocator);
    lzma_filters_free(&raw mut (*coder).filters as *mut lzma_filter, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn stream_encoder_update(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter,
    reversed_filters: *const lzma_filter,
) -> lzma_ret {
    let current_block: u64;
    let coder: *mut lzma_stream_coder = coder_ptr as *mut lzma_stream_coder;
    let mut ret: lzma_ret = LZMA_OK;
    let mut temp: [lzma_filter; 5] = [lzma_filter {
        id: 0,
        options: core::ptr::null_mut(),
    }; 5];
    let ret_: lzma_ret =
        lzma_filters_copy(filters, &raw mut temp as *mut lzma_filter, allocator) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    if (*coder).sequence <= SEQ_BLOCK_INIT {
        (*coder).block_encoder_is_initialized = false;
        (*coder).block_options.filters = &raw mut temp as *mut lzma_filter;
        ret = block_encoder_init(coder, allocator);
        (*coder).block_options.filters = &raw mut (*coder).filters as *mut lzma_filter;
        if ret != LZMA_OK {
            current_block = 9913398440939854562;
        } else {
            (*coder).block_encoder_is_initialized = true;
            current_block = 8236137900636309791;
        }
    } else if (*coder).sequence <= SEQ_BLOCK_ENCODE {
        ret = (*coder)
            .block_encoder
            .update
            .expect("non-null function pointer")(
            (*coder).block_encoder.coder,
            allocator,
            filters,
            reversed_filters,
        );
        if ret != LZMA_OK {
            current_block = 9913398440939854562;
        } else {
            current_block = 8236137900636309791;
        }
    } else {
        ret = LZMA_PROG_ERROR;
        current_block = 9913398440939854562;
    }
    match current_block {
        9913398440939854562 => {
            lzma_filters_free(&raw mut temp as *mut lzma_filter, allocator);
            return ret;
        }
        _ => {
            lzma_filters_free(&raw mut (*coder).filters as *mut lzma_filter, allocator);
            memcpy(
                &raw mut (*coder).filters as *mut lzma_filter as *mut c_void,
                &raw mut temp as *mut lzma_filter as *const c_void,
                core::mem::size_of::<[lzma_filter; 5]>() as size_t,
            );
            return LZMA_OK;
        }
    };
}
unsafe extern "C" fn stream_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter,
    check: lzma_check,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_filter,
                lzma_check,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        stream_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_filter,
                lzma_check,
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
                *const lzma_filter,
                lzma_check,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        stream_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_filter,
                lzma_check,
            ) -> lzma_ret,
    ));
    if filters.is_null() {
        return LZMA_PROG_ERROR;
    }
    let mut coder: *mut lzma_stream_coder = (*next).coder as *mut lzma_stream_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            core::mem::size_of::<lzma_stream_coder>() as size_t,
            allocator,
        ) as *mut lzma_stream_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            stream_encode
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
            stream_encoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).update = Some(
            stream_encoder_update
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const lzma_allocator,
                    *const lzma_filter,
                    *const lzma_filter,
                ) -> lzma_ret,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut c_void,
                    *const lzma_allocator,
                    *const lzma_filter,
                    *const lzma_filter,
                ) -> lzma_ret,
            >;
        (*coder).filters[0].id = LZMA_VLI_UNKNOWN as lzma_vli;
        (*coder).block_encoder = lzma_next_coder_s {
            coder: core::ptr::null_mut(),
            id: LZMA_VLI_UNKNOWN as lzma_vli,
            init: 0,
            code: None,
            end: None,
            get_progress: None,
            get_check: None,
            memconfig: None,
            update: None,
            set_out_limit: None,
        };
        (*coder).index_encoder = lzma_next_coder_s {
            coder: core::ptr::null_mut(),
            id: LZMA_VLI_UNKNOWN as lzma_vli,
            init: 0,
            code: None,
            end: None,
            get_progress: None,
            get_check: None,
            memconfig: None,
            update: None,
            set_out_limit: None,
        };
        (*coder).index = core::ptr::null_mut();
    }
    (*coder).sequence = SEQ_STREAM_HEADER;
    (*coder).block_options.version = 0;
    (*coder).block_options.check = check;
    lzma_index_end((*coder).index, allocator);
    (*coder).index = lzma_index_init(allocator);
    if (*coder).index.is_null() {
        return LZMA_MEM_ERROR;
    }
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
    let ret_: lzma_ret =
        lzma_stream_header_encode(&raw mut stream_flags, &raw mut (*coder).buffer as *mut u8)
            as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    (*coder).buffer_pos = 0;
    (*coder).buffer_size = LZMA_STREAM_HEADER_SIZE as size_t;
    return stream_encoder_update(
        coder as *mut c_void,
        allocator,
        filters,
        ::core::ptr::null::<lzma_filter>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_stream_encoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
    check: lzma_check,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = stream_encoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        filters,
        check,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_SYNC_FLUSH as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FULL_FLUSH as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FULL_BARRIER as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    return LZMA_OK;
}
