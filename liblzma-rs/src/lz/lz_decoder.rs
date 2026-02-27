use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulong, c_ulonglong, c_void};
extern "C" {
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    fn lzma_next_filter_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_info_s {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub options: *mut c_void,
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
pub struct lzma_dict {
    pub buf: *mut u8,
    pub pos: size_t,
    pub full: size_t,
    pub limit: size_t,
    pub size: size_t,
    pub has_wrapped: bool,
    pub need_reset: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_options {
    pub dict_size: size_t,
    pub preset_dict: *const u8,
    pub preset_dict_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_decoder {
    pub coder: *mut c_void,
    pub code: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut lzma_dict,
            *const u8,
            *mut size_t,
            size_t,
        ) -> lzma_ret,
    >,
    pub reset: Option<unsafe extern "C" fn(*mut c_void, *const c_void) -> ()>,
    pub set_uncompressed: Option<unsafe extern "C" fn(*mut c_void, lzma_vli, bool) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_coder {
    pub dict: lzma_dict,
    pub lz: lzma_lz_decoder,
    pub next: lzma_next_coder,
    pub next_finished: bool,
    pub this_finished: bool,
    pub temp: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub pos: size_t,
    pub size: size_t,
    pub buffer: [u8; LZMA_BUFFER_SIZE as usize],
}
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const UINTPTR_MAX: c_ulong = uintptr_t::MAX as c_ulong;
pub const SIZE_MAX: c_ulong = UINTPTR_MAX;
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_BUFFER_SIZE: c_int = 4096 as c_int;
pub const LZ_DICT_EXTRA: c_int = 0 as c_int;
pub const LZ_DICT_REPEAT_MAX: c_int = 288 as c_int;
pub const LZ_DICT_INIT_POS: c_int = 2 as c_int * LZ_DICT_REPEAT_MAX;
pub const LZMA_LZ_DECODER_INIT: lzma_lz_decoder = lzma_lz_decoder {
    coder: NULL,
    code: None,
    reset: None,
    set_uncompressed: None,
    end: None,
};
unsafe extern "C" fn lz_decoder_reset(mut coder: *mut lzma_coder) {
    (*coder).dict.pos = LZ_DICT_INIT_POS as size_t;
    (*coder).dict.full = 0 as size_t;
    *(*coder)
        .dict
        .buf
        .offset((LZ_DICT_INIT_POS - 1 as c_int) as isize) = '\0' as i32 as u8;
    (*coder).dict.has_wrapped = false_0 != 0;
    (*coder).dict.need_reset = false_0 != 0;
}
unsafe extern "C" fn decode_buffer(
    mut coder: *mut lzma_coder,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut out: *mut u8,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
) -> lzma_ret {
    loop {
        if (*coder).dict.pos == (*coder).dict.size {
            (*coder).dict.pos = LZ_DICT_REPEAT_MAX as size_t;
            (*coder).dict.has_wrapped = true_0 != 0;
            memcpy(
                (*coder).dict.buf as *mut c_void,
                (*coder)
                    .dict
                    .buf
                    .offset((*coder).dict.size as isize)
                    .offset(-(LZ_DICT_REPEAT_MAX as isize)) as *const c_void,
                LZ_DICT_REPEAT_MAX as size_t,
            );
        }
        let dict_start: size_t = (*coder).dict.pos;
        (*coder).dict.limit = (*coder).dict.pos.wrapping_add(
            (if out_size.wrapping_sub(*out_pos) < (*coder).dict.size.wrapping_sub((*coder).dict.pos)
            {
                out_size.wrapping_sub(*out_pos)
            } else {
                (*coder).dict.size.wrapping_sub((*coder).dict.pos)
            }),
        );
        let ret: lzma_ret = (*coder).lz.code.expect("non-null function pointer")(
            (*coder).lz.coder,
            &raw mut (*coder).dict,
            in_0,
            in_pos,
            in_size,
        ) as lzma_ret;
        let copy_size: size_t = (*coder).dict.pos.wrapping_sub(dict_start);
        if copy_size > 0 as size_t {
            memcpy(
                out.offset(*out_pos as isize) as *mut c_void,
                (*coder).dict.buf.offset(dict_start as isize) as *const c_void,
                copy_size,
            );
        }
        *out_pos = (*out_pos).wrapping_add(copy_size);
        if (*coder).dict.need_reset {
            lz_decoder_reset(coder);
            if ret != LZMA_OK || *out_pos == out_size {
                return ret;
            }
        } else if ret != LZMA_OK || *out_pos == out_size || (*coder).dict.pos < (*coder).dict.size {
            return ret;
        }
    }
}
unsafe extern "C" fn lz_decode(
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
    let mut coder: *mut lzma_coder = coder_ptr as *mut lzma_coder;
    if (*coder).next.code.is_none() {
        return decode_buffer(coder, in_0, in_pos, in_size, out, out_pos, out_size);
    }
    while *out_pos < out_size {
        if !(*coder).next_finished && (*coder).temp.pos == (*coder).temp.size {
            (*coder).temp.pos = 0 as size_t;
            (*coder).temp.size = 0 as size_t;
            let ret: lzma_ret = (*coder).next.code.expect("non-null function pointer")(
                (*coder).next.coder,
                allocator,
                in_0,
                in_pos,
                in_size,
                &raw mut (*coder).temp.buffer as *mut u8,
                &raw mut (*coder).temp.size,
                LZMA_BUFFER_SIZE as size_t,
                action,
            ) as lzma_ret;
            if ret == LZMA_STREAM_END {
                (*coder).next_finished = true_0 != 0;
            } else if ret != LZMA_OK || (*coder).temp.size == 0 as size_t {
                return ret;
            }
        }
        if (*coder).this_finished {
            if (*coder).temp.size != 0 as size_t {
                return LZMA_DATA_ERROR;
            }
            if (*coder).next_finished {
                return LZMA_STREAM_END;
            }
            return LZMA_OK;
        }
        let ret_0: lzma_ret = decode_buffer(
            coder,
            &raw mut (*coder).temp.buffer as *mut u8,
            &raw mut (*coder).temp.pos,
            (*coder).temp.size,
            out,
            out_pos,
            out_size,
        ) as lzma_ret;
        if ret_0 == LZMA_STREAM_END {
            (*coder).this_finished = true_0 != 0;
        } else if ret_0 != LZMA_OK {
            return ret_0;
        } else if (*coder).next_finished as c_int != 0 && *out_pos < out_size {
            return LZMA_DATA_ERROR;
        }
    }
    return LZMA_OK;
}
unsafe extern "C" fn lz_decoder_end(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_coder = coder_ptr as *mut lzma_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free((*coder).dict.buf as *mut c_void, allocator);
    if (*coder).lz.end.is_some() {
        (*coder).lz.end.expect("non-null function pointer")((*coder).lz.coder, allocator);
    } else {
        lzma_free((*coder).lz.coder, allocator);
    }
    lzma_free(coder as *mut c_void, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lz_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
    mut lz_init: Option<
        unsafe extern "C" fn(
            *mut lzma_lz_decoder,
            *const lzma_allocator,
            lzma_vli,
            *const c_void,
            *mut lzma_lz_options,
        ) -> lzma_ret,
    >,
) -> lzma_ret {
    let mut coder: *mut lzma_coder = (*next).coder as *mut lzma_coder;
    if coder.is_null() {
        coder = lzma_alloc(::core::mem::size_of::<lzma_coder>() as size_t, allocator)
            as *mut lzma_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            lz_decode
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
        (*next).end =
            Some(lz_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ())
                as lzma_end_function;
        (*coder).dict.buf = ::core::ptr::null_mut::<u8>();
        (*coder).dict.size = 0 as size_t;
        (*coder).lz = LZMA_LZ_DECODER_INIT;
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
    let mut lz_options: lzma_lz_options = lzma_lz_options {
        dict_size: 0,
        preset_dict: ::core::ptr::null::<u8>(),
        preset_dict_size: 0,
    };
    let ret_: lzma_ret = lz_init.expect("non-null function pointer")(
        &raw mut (*coder).lz,
        allocator,
        (*filters.offset(0)).id,
        (*filters.offset(0)).options,
        &raw mut lz_options,
    ) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    if lz_options.dict_size < 4096 as size_t {
        lz_options.dict_size = 4096 as size_t;
    }
    if lz_options.dict_size
        > (SIZE_MAX as size_t)
            .wrapping_sub(15 as size_t)
            .wrapping_sub((2 as c_int * LZ_DICT_REPEAT_MAX) as size_t)
            .wrapping_sub(LZ_DICT_EXTRA as size_t)
    {
        return LZMA_MEM_ERROR;
    }
    lz_options.dict_size = lz_options.dict_size.wrapping_add(15 as size_t) & !(15 as size_t);
    let alloc_size: size_t = lz_options
        .dict_size
        .wrapping_add((2 as c_int * LZ_DICT_REPEAT_MAX) as size_t);
    if (*coder).dict.size != alloc_size {
        lzma_free((*coder).dict.buf as *mut c_void, allocator);
        (*coder).dict.buf =
            lzma_alloc(alloc_size.wrapping_add(LZ_DICT_EXTRA as size_t), allocator) as *mut u8;
        if (*coder).dict.buf.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*coder).dict.size = alloc_size;
    }
    lz_decoder_reset((*next).coder as *mut lzma_coder);
    if !lz_options.preset_dict.is_null() && lz_options.preset_dict_size > 0 as size_t {
        let copy_size: size_t = if lz_options.preset_dict_size < lz_options.dict_size {
            lz_options.preset_dict_size
        } else {
            lz_options.dict_size
        };
        let offset: size_t = lz_options.preset_dict_size.wrapping_sub(copy_size);
        memcpy(
            (*coder).dict.buf.offset((*coder).dict.pos as isize) as *mut c_void,
            lz_options.preset_dict.offset(offset as isize) as *const c_void,
            copy_size,
        );
        (*coder).dict.pos = (*coder).dict.pos.wrapping_add(copy_size);
        (*coder).dict.full = copy_size;
    }
    (*coder).next_finished = false_0 != 0;
    (*coder).this_finished = false_0 != 0;
    (*coder).temp.pos = 0 as size_t;
    (*coder).temp.size = 0 as size_t;
    return lzma_next_filter_init(&raw mut (*coder).next, allocator, filters.offset(1));
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lz_decoder_memusage(mut dictionary_size: size_t) -> u64 {
    return (::core::mem::size_of::<lzma_coder>() as u64)
        .wrapping_add(dictionary_size as u64)
        .wrapping_add((2 as c_int * LZ_DICT_REPEAT_MAX) as u64)
        .wrapping_add(LZ_DICT_EXTRA as u64);
}
