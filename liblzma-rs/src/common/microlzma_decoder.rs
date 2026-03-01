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
    fn lzma_lzma_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_lzma_lclppb_decode(options: *mut lzma_options_lzma, byte: u8) -> bool;
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
    pub comp_size: u64,
    pub uncomp_size: lzma_vli,
    pub dict_size: u32,
    pub uncomp_size_is_exact: bool,
    pub props_decoded: bool,
}
unsafe extern "C" fn microlzma_decode(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_pos: *mut size_t,
    mut in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    mut out_size: size_t,
    action: lzma_action,
) -> lzma_ret {
    let coder: *mut lzma_microlzma_coder = coder_ptr as *mut lzma_microlzma_coder;
    let in_start: size_t = *in_pos;
    let out_start: size_t = *out_pos;
    if in_size.wrapping_sub(*in_pos) as u64 > (*coder).comp_size {
        in_size = (*in_pos).wrapping_add((*coder).comp_size as size_t);
    }
    if !(*coder).uncomp_size_is_exact
        && out_size.wrapping_sub(*out_pos) as lzma_vli > (*coder).uncomp_size
    {
        out_size = (*out_pos).wrapping_add((*coder).uncomp_size as size_t);
    }
    if !(*coder).props_decoded {
        if *in_pos >= in_size {
            return LZMA_OK;
        }
        let mut options: lzma_options_lzma = lzma_options_lzma {
            dict_size: (*coder).dict_size,
            preset_dict: ::core::ptr::null::<u8>(),
            preset_dict_size: 0,
            lc: 0,
            lp: 0,
            pb: 0,
            mode: 0 as lzma_mode,
            nice_len: 0,
            mf: 0 as lzma_match_finder,
            depth: 0,
            ext_flags: 0,
            ext_size_low: UINT32_MAX as u32,
            ext_size_high: UINT32_MAX as u32,
            reserved_int4: 0,
            reserved_int5: 0,
            reserved_int6: 0,
            reserved_int7: 0,
            reserved_int8: 0,
            reserved_enum1: LZMA_RESERVED_ENUM,
            reserved_enum2: LZMA_RESERVED_ENUM,
            reserved_enum3: LZMA_RESERVED_ENUM,
            reserved_enum4: LZMA_RESERVED_ENUM,
            reserved_ptr1: core::ptr::null_mut(),
            reserved_ptr2: core::ptr::null_mut(),
        };
        if (*coder).uncomp_size_is_exact {
            options.ext_size_low = (*coder).uncomp_size as u32;
            options.ext_size_high = ((*coder).uncomp_size >> 32) as u32;
        }
        if lzma_lzma_lclppb_decode(
            &raw mut options,
            !(*in_0.offset(*in_pos as isize) as c_int) as u8,
        ) {
            return LZMA_OPTIONS_ERROR;
        }
        *in_pos = (*in_pos).wrapping_add(1);
        let mut filters: [lzma_filter_info; 2] = [
            lzma_filter_info_s {
                id: LZMA_FILTER_LZMA1EXT as lzma_vli,
                init: Some(
                    lzma_lzma_decoder_init
                        as unsafe extern "C" fn(
                            *mut lzma_next_coder,
                            *const lzma_allocator,
                            *const lzma_filter_info,
                        ) -> lzma_ret,
                ),
                options: &raw mut options as *mut c_void,
            },
            lzma_filter_info_s {
                id: 0,
                init: None,
                options: core::ptr::null_mut(),
            },
        ];
        let ret_: lzma_ret = lzma_next_filter_init(
            &raw mut (*coder).lzma,
            allocator,
            &raw mut filters as *mut lzma_filter_info,
        ) as lzma_ret;
        if ret_ != LZMA_OK {
            return ret_;
        }
        let dummy_in: u8 = 0;
        let mut dummy_in_pos: size_t = 0;
        if (*coder).lzma.code.expect("non-null function pointer")(
            (*coder).lzma.coder,
            allocator,
            &raw const dummy_in,
            &raw mut dummy_in_pos,
            1,
            out,
            out_pos,
            out_size,
            LZMA_RUN,
        ) != LZMA_OK
        {
            return LZMA_PROG_ERROR;
        }
        (*coder).props_decoded = true;
    }
    let mut ret: lzma_ret = (*coder).lzma.code.expect("non-null function pointer")(
        (*coder).lzma.coder,
        allocator,
        in_0,
        in_pos,
        in_size,
        out,
        out_pos,
        out_size,
        action,
    );
    (*coder).comp_size = (*coder)
        .comp_size
        .wrapping_sub((*in_pos).wrapping_sub(in_start) as u64);
    if (*coder).uncomp_size_is_exact {
        if ret == LZMA_STREAM_END && (*coder).comp_size != 0 {
            ret = LZMA_DATA_ERROR;
        }
    } else {
        (*coder).uncomp_size = (*coder)
            .uncomp_size
            .wrapping_sub((*out_pos).wrapping_sub(out_start) as lzma_vli);
        if ret == LZMA_STREAM_END {
            ret = LZMA_DATA_ERROR;
        } else if (*coder).uncomp_size == 0 as lzma_vli {
            ret = LZMA_STREAM_END;
        }
    }
    return ret;
}
unsafe extern "C" fn microlzma_decoder_end(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
) {
    let coder: *mut lzma_microlzma_coder = coder_ptr as *mut lzma_microlzma_coder;
    lzma_next_end(&raw mut (*coder).lzma, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn microlzma_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    comp_size: u64,
    uncomp_size: u64,
    uncomp_size_is_exact: bool,
    dict_size: u32,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                u64,
                bool,
                u32,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        microlzma_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                u64,
                bool,
                u32,
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
                u64,
                u64,
                bool,
                u32,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        microlzma_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                u64,
                bool,
                u32,
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
            microlzma_decode
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
            microlzma_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
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
    if uncomp_size > LZMA_VLI_MAX as u64 {
        return LZMA_OPTIONS_ERROR;
    }
    (*coder).comp_size = comp_size;
    (*coder).uncomp_size = uncomp_size as lzma_vli;
    (*coder).uncomp_size_is_exact = uncomp_size_is_exact;
    (*coder).dict_size = dict_size;
    (*coder).props_decoded = false;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_microlzma_decoder(
    strm: *mut lzma_stream,
    comp_size: u64,
    uncomp_size: u64,
    uncomp_size_is_exact: lzma_bool,
    dict_size: u32,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = microlzma_decoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        comp_size,
        uncomp_size,
        uncomp_size_is_exact != 0,
        dict_size,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    return LZMA_OK;
}
