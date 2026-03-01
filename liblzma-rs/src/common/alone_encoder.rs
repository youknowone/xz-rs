use crate::types::*;
use core::ffi::{c_int, c_uint, c_void};
extern "C" {
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_strm_init(strm: *mut lzma_stream) -> lzma_ret;
    fn lzma_next_filter_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_bufcpy(
        in_0: *const u8,
        in_pos: *mut size_t,
        in_size: size_t,
        out: *mut u8,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> size_t;
    fn lzma_lzma_encoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_lzma_lclppb_encode(options: *const lzma_options_lzma, byte: *mut u8) -> bool;
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
pub struct lzma_options_lzma {
    pub dict_size: u32,
    pub preset_dict: *const u8,
    pub preset_dict_size: u32,
    pub lc: u32,
    pub lp: u32,
    pub pb: u32,
    pub mode: lzma_mode,
    pub nice_len: u32,
    pub mf: lzma_match_finder,
    pub depth: u32,
    pub ext_flags: u32,
    pub ext_size_low: u32,
    pub ext_size_high: u32,
    pub reserved_int4: u32,
    pub reserved_int5: u32,
    pub reserved_int6: u32,
    pub reserved_int7: u32,
    pub reserved_int8: u32,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_ptr1: *mut c_void,
    pub reserved_ptr2: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_alone_coder {
    pub next: lzma_next_coder,
    pub sequence: C2RustUnnamed_0,
    pub header_pos: size_t,
    pub header: [u8; 13],
}
pub type C2RustUnnamed_0 = c_uint;
pub const SEQ_CODE: C2RustUnnamed_0 = 1;
pub const SEQ_HEADER: C2RustUnnamed_0 = 0;
#[inline]
extern "C" fn write32le(buf: *mut u8, num: u32) {
    unsafe {
        *buf.offset(0) = num as u8;
        *buf.offset(1) = (num >> 8) as u8;
        *buf.offset(2) = (num >> 16) as u8;
        *buf.offset(3) = (num >> 24) as u8;
    }
}
pub const ALONE_HEADER_SIZE: c_int = 1 + 4 as c_int + 8 as c_int;
unsafe extern "C" fn alone_encode(
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
    let coder: *mut lzma_alone_coder = coder_ptr as *mut lzma_alone_coder;
    while *out_pos < out_size {
        match (*coder).sequence {
            0 => {
                lzma_bufcpy(
                    &raw mut (*coder).header as *mut u8,
                    &raw mut (*coder).header_pos,
                    ALONE_HEADER_SIZE as size_t,
                    out,
                    out_pos,
                    out_size,
                );
                if (*coder).header_pos < ALONE_HEADER_SIZE as size_t {
                    return LZMA_OK;
                }
                (*coder).sequence = SEQ_CODE;
            }
            1 => {
                return (*coder).next.code.expect("non-null function pointer")(
                    (*coder).next.coder,
                    allocator,
                    in_0,
                    in_pos,
                    in_size,
                    out,
                    out_pos,
                    out_size,
                    action,
                );
            }
            _ => return LZMA_PROG_ERROR,
        }
    }
    return LZMA_OK;
}
unsafe extern "C" fn alone_encoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_alone_coder = coder_ptr as *mut lzma_alone_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn alone_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    options: *const lzma_options_lzma,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_options_lzma,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        alone_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_options_lzma,
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
                *const lzma_options_lzma,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        alone_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_options_lzma,
            ) -> lzma_ret,
    ));
    let mut coder: *mut lzma_alone_coder = (*next).coder as *mut lzma_alone_coder;
    if coder.is_null() {
        coder = lzma_alloc(core::mem::size_of::<lzma_alone_coder>(), allocator)
            as *mut lzma_alone_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            alone_encode
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
            alone_encoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*coder).next = lzma_next_coder_s {
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
    }
    (*coder).sequence = SEQ_HEADER;
    (*coder).header_pos = 0;
    if lzma_lzma_lclppb_encode(options, &raw mut (*coder).header as *mut u8) {
        return LZMA_OPTIONS_ERROR;
    }
    if (*options).dict_size < LZMA_DICT_SIZE_MIN as u32 {
        return LZMA_OPTIONS_ERROR;
    }
    let mut d: u32 = (*options).dict_size.wrapping_sub(1);
    d |= d >> 2;
    d |= d >> 3;
    d |= d >> 4;
    d |= d >> 8;
    d |= d >> 16;
    if d != UINT32_MAX as u32 {
        d = d.wrapping_add(1);
    }
    write32le((&raw mut (*coder).header as *mut u8).offset(1), d);
    memset(
        (&raw mut (*coder).header as *mut u8).offset(1).offset(4) as *mut c_void,
        0xff as c_int,
        8,
    );
    let filters: [lzma_filter_info; 2] = [
        lzma_filter_info_s {
            id: LZMA_FILTER_LZMA1 as lzma_vli,
            init: Some(
                lzma_lzma_encoder_init
                    as unsafe extern "C" fn(
                        *mut lzma_next_coder,
                        *const lzma_allocator,
                        *const lzma_filter_info,
                    ) -> lzma_ret,
            ),
            options: options as *mut c_void,
        },
        lzma_filter_info_s {
            id: 0,
            init: None,
            options: core::ptr::null_mut(),
        },
    ];
    return lzma_next_filter_init(
        &raw mut (*coder).next,
        allocator,
        &raw const filters as *const lzma_filter_info,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_alone_encoder(
    strm: *mut lzma_stream,
    options: *const lzma_options_lzma,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = alone_encoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        options,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    return LZMA_OK;
}
