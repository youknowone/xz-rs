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
pub struct lzma_microlzma_coder {
    pub lzma: lzma_next_coder,
    pub props: u8,
}
unsafe extern "C" fn microlzma_encode(
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
    let coder: *mut lzma_microlzma_coder = coder_ptr as *mut lzma_microlzma_coder;
    let out_start: size_t = *out_pos;
    let in_start: size_t = *in_pos;
    let mut uncomp_size: u64 = 0;
    if (*coder)
        .lzma
        .set_out_limit
        .expect("non-null function pointer")(
        (*coder).lzma.coder,
        &raw mut uncomp_size,
        out_size.wrapping_sub(*out_pos) as u64,
    ) != LZMA_OK
    {
        return LZMA_PROG_ERROR;
    }
    let ret: lzma_ret = (*coder).lzma.code.expect("non-null function pointer")(
        (*coder).lzma.coder,
        allocator,
        in_0,
        in_pos,
        in_size,
        out,
        out_pos,
        out_size,
        action,
    ) as lzma_ret;
    if ret != LZMA_STREAM_END {
        if ret == LZMA_OK {
            return LZMA_PROG_ERROR;
        }
        return ret;
    }
    *out.offset(out_start as isize) = !((*coder).props as c_int) as u8;
    *in_pos = in_start.wrapping_add(uncomp_size as size_t);
    return ret;
}
unsafe extern "C" fn microlzma_encoder_end(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
) {
    let coder: *mut lzma_microlzma_coder = coder_ptr as *mut lzma_microlzma_coder;
    lzma_next_end(&raw mut (*coder).lzma, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn microlzma_encoder_init(
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
        microlzma_encoder_init
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
        microlzma_encoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *const lzma_options_lzma,
            ) -> lzma_ret,
    ));
    let mut coder: *mut lzma_microlzma_coder = (*next).coder as *mut lzma_microlzma_coder;
    if coder.is_null() {
        coder = lzma_alloc(core::mem::size_of::<lzma_microlzma_coder>(), allocator)
            as *mut lzma_microlzma_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            microlzma_encode
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
            microlzma_encoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*coder).lzma = lzma_next_coder_s {
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
    if lzma_lzma_lclppb_encode(options, &raw mut (*coder).props) {
        return LZMA_OPTIONS_ERROR;
    }
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
        &raw mut (*coder).lzma,
        allocator,
        &raw const filters as *const lzma_filter_info,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_microlzma_encoder(
    strm: *mut lzma_stream,
    options: *const lzma_options_lzma,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = microlzma_encoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        options,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    return LZMA_OK;
}
