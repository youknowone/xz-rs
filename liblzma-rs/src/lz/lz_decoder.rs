use crate::types::*;
use core::ffi::{c_ulong, c_void};
extern "C" {
    fn lzma_next_filter_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
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
pub const UINTPTR_MAX: c_ulong = uintptr_t::MAX as c_ulong;
pub const SIZE_MAX: c_ulong = UINTPTR_MAX;
pub const LZMA_BUFFER_SIZE: u32 = 4096;
pub const LZ_DICT_EXTRA: u32 = 0;
pub const LZ_DICT_REPEAT_MAX: u32 = 288;
pub const LZ_DICT_INIT_POS: u32 = 2 * LZ_DICT_REPEAT_MAX;
pub const LZMA_LZ_DECODER_INIT: lzma_lz_decoder = lzma_lz_decoder {
    coder: core::ptr::null_mut(),
    code: None,
    reset: None,
    set_uncompressed: None,
    end: None,
};
unsafe extern "C" fn lz_decoder_reset(coder: *mut lzma_coder) {
    (*coder).dict.pos = LZ_DICT_INIT_POS as size_t;
    (*coder).dict.full = 0;
    *(*coder).dict.buf.offset((LZ_DICT_INIT_POS - 1) as isize) = '\0' as i32 as u8;
    (*coder).dict.has_wrapped = false;
    (*coder).dict.need_reset = false;
}
unsafe extern "C" fn decode_buffer(
    coder: *mut lzma_coder,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    loop {
        if (*coder).dict.pos == (*coder).dict.size {
            (*coder).dict.pos = LZ_DICT_REPEAT_MAX as size_t;
            (*coder).dict.has_wrapped = true;
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
            if out_size.wrapping_sub(*out_pos) < (*coder).dict.size.wrapping_sub((*coder).dict.pos)
            {
                out_size.wrapping_sub(*out_pos)
            } else {
                (*coder).dict.size.wrapping_sub((*coder).dict.pos)
            },
        );
        let ret: lzma_ret = (*coder).lz.code.expect("non-null function pointer")(
            (*coder).lz.coder,
            &raw mut (*coder).dict,
            in_0,
            in_pos,
            in_size,
        );
        let copy_size: size_t = (*coder).dict.pos.wrapping_sub(dict_start);
        if copy_size > 0 {
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
    let coder: *mut lzma_coder = coder_ptr as *mut lzma_coder;
    if (*coder).next.code.is_none() {
        return decode_buffer(coder, in_0, in_pos, in_size, out, out_pos, out_size);
    }
    while *out_pos < out_size {
        if !(*coder).next_finished && (*coder).temp.pos == (*coder).temp.size {
            (*coder).temp.pos = 0;
            (*coder).temp.size = 0;
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
            );
            if ret == LZMA_STREAM_END {
                (*coder).next_finished = true;
            } else if ret != LZMA_OK || (*coder).temp.size == 0 {
                return ret;
            }
        }
        if (*coder).this_finished {
            if (*coder).temp.size != 0 {
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
        );
        if ret_0 == LZMA_STREAM_END {
            (*coder).this_finished = true;
        } else if ret_0 != LZMA_OK {
            return ret_0;
        } else if (*coder).next_finished && *out_pos < out_size {
            return LZMA_DATA_ERROR;
        }
    }
    return LZMA_OK;
}
unsafe extern "C" fn lz_decoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_coder = coder_ptr as *mut lzma_coder;
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
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    lz_init: Option<
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
        coder = lzma_alloc(core::mem::size_of::<lzma_coder>(), allocator) as *mut lzma_coder;
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
        (*coder).dict.buf = core::ptr::null_mut();
        (*coder).dict.size = 0;
        (*coder).lz = LZMA_LZ_DECODER_INIT;
        (*coder).next = lzma_next_coder_s {
            coder: core::ptr::null_mut(),
            id: LZMA_VLI_UNKNOWN,
            init: 0,
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
    );
    if ret_ != LZMA_OK {
        return ret_;
    }
    if lz_options.dict_size < 4096 {
        lz_options.dict_size = 4096;
    }
    if lz_options.dict_size
        > (SIZE_MAX as size_t)
            .wrapping_sub(15)
            .wrapping_sub((2 * LZ_DICT_REPEAT_MAX) as size_t)
            .wrapping_sub(LZ_DICT_EXTRA as size_t)
    {
        return LZMA_MEM_ERROR;
    }
    lz_options.dict_size = lz_options.dict_size.wrapping_add(15) & !(15);
    let alloc_size: size_t = lz_options
        .dict_size
        .wrapping_add((2 * LZ_DICT_REPEAT_MAX) as size_t);
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
    if !lz_options.preset_dict.is_null() && lz_options.preset_dict_size > 0 {
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
    (*coder).next_finished = false;
    (*coder).this_finished = false;
    (*coder).temp.pos = 0;
    (*coder).temp.size = 0;
    return lzma_next_filter_init(&raw mut (*coder).next, allocator, filters.offset(1));
}
#[no_mangle]
pub extern "C" fn lzma_lz_decoder_memusage(dictionary_size: size_t) -> u64 {
    return (core::mem::size_of::<lzma_coder>() as u64)
        .wrapping_add(dictionary_size as u64)
        .wrapping_add((2 * LZ_DICT_REPEAT_MAX) as u64)
        .wrapping_add(LZ_DICT_EXTRA as u64);
}
