use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulonglong, c_void};
extern "C" {
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
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
    fn lzma_lzma_decoder_memusage_nocheck(options: *const c_void) -> u64;
    fn lzma_lzma_lclppb_decode(options: *mut lzma_options_lzma, byte: u8) -> bool;
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
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;
pub const LZMA_MODE_FAST: lzma_mode = 1;
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
    pub picky: bool,
    pub pos: size_t,
    pub uncompressed_size: lzma_vli,
    pub memlimit: u64,
    pub memusage: u64,
    pub options: lzma_options_lzma,
}
pub type C2RustUnnamed_0 = c_uint;
pub const SEQ_CODE: C2RustUnnamed_0 = 4;
pub const SEQ_CODER_INIT: C2RustUnnamed_0 = 3;
pub const SEQ_UNCOMPRESSED_SIZE: C2RustUnnamed_0 = 2;
pub const SEQ_DICTIONARY_SIZE: C2RustUnnamed_0 = 1;
pub const SEQ_PROPERTIES: C2RustUnnamed_0 = 0;
pub type lzma_filter_info = lzma_filter_info_s;
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
pub const UINT32_MAX: c_uint = 4294967295;
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_FILTER_LZMA1EXT: c_ulonglong = 0x4000000000000002;
pub const LZMA_LZMA1EXT_ALLOW_EOPM: c_uint = 0x1;
pub const LZMA_MEMUSAGE_BASE: c_ulonglong = 1 << 15;
unsafe extern "C" fn alone_decode(
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
    while *out_pos < out_size && ((*coder).sequence == SEQ_CODE || *in_pos < in_size) {
        let current_block_42: u64;
        match (*coder).sequence {
            0 => {
                if lzma_lzma_lclppb_decode(
                    &raw mut (*coder).options,
                    *in_0.offset(*in_pos as isize),
                ) {
                    return LZMA_FORMAT_ERROR;
                }
                (*coder).sequence = SEQ_DICTIONARY_SIZE;
                *in_pos = (*in_pos).wrapping_add(1);
                current_block_42 = 11048769245176032998;
            }
            1 => {
                (*coder).options.dict_size = ((*coder).options.dict_size as size_t
                    | (*in_0.offset(*in_pos as isize) as size_t) << (*coder).pos.wrapping_mul(8))
                    as u32;
                (*coder).pos = (*coder).pos.wrapping_add(1);
                if (*coder).pos == 4 {
                    if (*coder).picky as c_int != 0
                        && (*coder).options.dict_size != UINT32_MAX as u32
                    {
                        let mut d: u32 = (*coder).options.dict_size.wrapping_sub(1);
                        d |= d >> 2;
                        d |= d >> 3;
                        d |= d >> 4;
                        d |= d >> 8;
                        d |= d >> 16;
                        d = d.wrapping_add(1);
                        if d != (*coder).options.dict_size {
                            return LZMA_FORMAT_ERROR;
                        }
                    }
                    (*coder).pos = 0;
                    (*coder).sequence = SEQ_UNCOMPRESSED_SIZE;
                }
                *in_pos = (*in_pos).wrapping_add(1);
                current_block_42 = 11048769245176032998;
            }
            2 => {
                (*coder).uncompressed_size |=
                    (*in_0.offset(*in_pos as isize) as lzma_vli) << (*coder).pos.wrapping_mul(8);
                *in_pos = (*in_pos).wrapping_add(1);
                (*coder).pos = (*coder).pos.wrapping_add(1);
                if (*coder).pos < 8 {
                    current_block_42 = 11048769245176032998;
                } else {
                    if (*coder).picky as c_int != 0
                        && (*coder).uncompressed_size != LZMA_VLI_UNKNOWN as lzma_vli
                        && (*coder).uncompressed_size >= (1 as lzma_vli) << 38
                    {
                        return LZMA_FORMAT_ERROR;
                    }
                    (*coder).options.ext_flags = LZMA_LZMA1EXT_ALLOW_EOPM as u32;
                    (*coder).options.ext_size_low = (*coder).uncompressed_size as u32;
                    (*coder).options.ext_size_high = ((*coder).uncompressed_size >> 32) as u32;
                    (*coder).memusage = lzma_lzma_decoder_memusage_nocheck(
                        &raw mut (*coder).options as *const c_void,
                    )
                    .wrapping_add(LZMA_MEMUSAGE_BASE as u64);
                    (*coder).pos = 0;
                    (*coder).sequence = SEQ_CODER_INIT;
                    current_block_42 = 14763689060501151050;
                }
            }
            3 => {
                current_block_42 = 14763689060501151050;
            }
            4 => {
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
        match current_block_42 {
            14763689060501151050 => {
                if (*coder).memusage > (*coder).memlimit {
                    return LZMA_MEMLIMIT_ERROR;
                }
                let mut filters: [lzma_filter_info; 2] = [
                    lzma_filter_info_s {
                        id: LZMA_FILTER_LZMA1EXT as lzma_vli,
                        init: Some(
                            lzma_lzma_decoder_init
                                as unsafe extern "C" fn(
                                    *mut lzma_next_coder,
                                    *const lzma_allocator,
                                    *const lzma_filter_info,
                                )
                                    -> lzma_ret,
                        ),
                        options: &raw mut (*coder).options as *mut c_void,
                    },
                    lzma_filter_info_s {
                        id: 0,
                        init: None,
                        options: core::ptr::null_mut(),
                    },
                ];
                let ret_: lzma_ret = lzma_next_filter_init(
                    &raw mut (*coder).next,
                    allocator,
                    &raw mut filters as *mut lzma_filter_info,
                ) as lzma_ret;
                if ret_ != LZMA_OK {
                    return ret_;
                }
                (*coder).sequence = SEQ_CODE;
            }
            _ => {}
        }
    }
    return LZMA_OK;
}
unsafe extern "C" fn alone_decoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_alone_coder = coder_ptr as *mut lzma_alone_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn alone_decoder_memconfig(
    coder_ptr: *mut c_void,
    memusage: *mut u64,
    old_memlimit: *mut u64,
    new_memlimit: u64,
) -> lzma_ret {
    let coder: *mut lzma_alone_coder = coder_ptr as *mut lzma_alone_coder;
    *memusage = (*coder).memusage;
    *old_memlimit = (*coder).memlimit;
    if new_memlimit != 0 {
        if new_memlimit < (*coder).memusage {
            return LZMA_MEMLIMIT_ERROR;
        }
        (*coder).memlimit = new_memlimit;
    }
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_alone_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    memlimit: u64,
    picky: bool,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                bool,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_alone_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                bool,
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
                bool,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_alone_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                u64,
                bool,
            ) -> lzma_ret,
    ));
    let mut coder: *mut lzma_alone_coder = (*next).coder as *mut lzma_alone_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            core::mem::size_of::<lzma_alone_coder>() as size_t,
            allocator,
        ) as *mut lzma_alone_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            alone_decode
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
            alone_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).memconfig = Some(
            alone_decoder_memconfig
                as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret,
        )
            as Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret>;
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
    (*coder).sequence = SEQ_PROPERTIES;
    (*coder).picky = picky;
    (*coder).pos = 0;
    (*coder).options.dict_size = 0;
    (*coder).options.preset_dict = ::core::ptr::null::<u8>();
    (*coder).options.preset_dict_size = 0;
    (*coder).uncompressed_size = 0 as lzma_vli;
    (*coder).memlimit = if 1 > memlimit { 1 } else { memlimit };
    (*coder).memusage = LZMA_MEMUSAGE_BASE as u64;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_alone_decoder(strm: *mut lzma_stream, memlimit: u64) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = lzma_alone_decoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        memlimit,
        0 as c_int != 0,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    return LZMA_OK;
}
