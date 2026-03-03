use crate::types::*;
use core::ffi::{c_uint, c_void};
extern "C" {
    fn lzma_alone_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        memlimit: u64,
        picky: bool,
    ) -> lzma_ret;
}
pub type auto_decoder_seq = c_uint;
pub const SEQ_FINISH: auto_decoder_seq = 2;
pub const SEQ_CODE: auto_decoder_seq = 1;
pub const SEQ_INIT: auto_decoder_seq = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_auto_coder {
    pub next: lzma_next_coder,
    pub memlimit: u64,
    pub flags: u32,
    pub sequence: auto_decoder_seq,
}
unsafe extern "C" fn auto_decode(
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
    let coder: *mut lzma_auto_coder = coder_ptr as *mut lzma_auto_coder;
    let current_block_28: u64;
    match (*coder).sequence {
        0 => {
            if *in_pos >= in_size {
                return LZMA_OK;
            }
            (*coder).sequence = SEQ_CODE;
            if *in_0.offset(*in_pos as isize) == 0xfd {
                let ret_: lzma_ret = lzma_stream_decoder_init(
                    &raw mut (*coder).next,
                    allocator,
                    (*coder).memlimit,
                    (*coder).flags,
                );
                if ret_ != LZMA_OK {
                    return ret_;
                }
            } else {
                let ret__0: lzma_ret = lzma_alone_decoder_init(
                    &raw mut (*coder).next,
                    allocator,
                    (*coder).memlimit,
                    true,
                );
                if ret__0 != LZMA_OK {
                    return ret__0;
                }
                if (*coder).flags & LZMA_TELL_NO_CHECK as u32 != 0 {
                    return LZMA_NO_CHECK;
                }
                if (*coder).flags & LZMA_TELL_ANY_CHECK as u32 != 0 {
                    return LZMA_GET_CHECK;
                }
            }
            current_block_28 = 13935781298497728377;
        }
        1 => {
            current_block_28 = 13935781298497728377;
        }
        2 => {
            current_block_28 = 4647193646042868866;
        }
        _ => return LZMA_PROG_ERROR,
    }
    match current_block_28 {
        13935781298497728377 => {
            let ret: lzma_ret = (*coder).next.code.unwrap()(
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
            if ret != LZMA_STREAM_END || (*coder).flags & LZMA_CONCATENATED as u32 == 0 {
                return ret;
            }
            (*coder).sequence = SEQ_FINISH;
        }
        _ => {}
    }
    if *in_pos < in_size {
        return LZMA_DATA_ERROR;
    }
    if action == LZMA_FINISH {
        LZMA_STREAM_END
    } else {
        LZMA_OK
    }
}
unsafe extern "C" fn auto_decoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_auto_coder = coder_ptr as *mut lzma_auto_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
extern "C" fn auto_decoder_get_check(coder_ptr: *const c_void) -> lzma_check {
    return unsafe {
        let coder: *const lzma_auto_coder = coder_ptr as *const lzma_auto_coder;
        (if (*coder).next.get_check.is_none() {
            LZMA_CHECK_NONE
        } else {
            (*coder).next.get_check.unwrap()((*coder).next.coder)
        }) as lzma_check
    };
}
unsafe extern "C" fn auto_decoder_memconfig(
    coder_ptr: *mut c_void,
    memusage: *mut u64,
    old_memlimit: *mut u64,
    new_memlimit: u64,
) -> lzma_ret {
    let coder: *mut lzma_auto_coder = coder_ptr as *mut lzma_auto_coder;
    let mut ret: lzma_ret = LZMA_OK;
    if (*coder).next.memconfig.is_some() {
        ret = (*coder).next.memconfig.unwrap()(
            (*coder).next.coder,
            memusage,
            old_memlimit,
            new_memlimit,
        );
    } else {
        *memusage = LZMA_MEMUSAGE_BASE;
        *old_memlimit = (*coder).memlimit;
        ret = LZMA_OK;
        if new_memlimit != 0 && new_memlimit < *memusage {
            ret = LZMA_MEMLIMIT_ERROR;
        }
    }
    if ret == LZMA_OK && new_memlimit != 0 {
        (*coder).memlimit = new_memlimit;
    }
    ret
}
unsafe extern "C" fn auto_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    if core::mem::transmute::<
        Option<
            unsafe extern "C" fn(*mut lzma_next_coder, *const lzma_allocator, u64, u32) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        auto_decoder_init
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
    (*next).init = core::mem::transmute::<
        Option<
            unsafe extern "C" fn(*mut lzma_next_coder, *const lzma_allocator, u64, u32) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        auto_decoder_init
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
    let mut coder: *mut lzma_auto_coder = (*next).coder as *mut lzma_auto_coder;
    if coder.is_null() {
        coder =
            lzma_alloc(core::mem::size_of::<lzma_auto_coder>(), allocator) as *mut lzma_auto_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            auto_decode
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
        );
        (*next).end = Some(
            auto_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        );
        (*next).get_check =
            Some(auto_decoder_get_check as unsafe extern "C" fn(*const c_void) -> lzma_check);
        (*next).memconfig = Some(
            auto_decoder_memconfig
                as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret,
        );
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
    (*coder).memlimit = if 1 > memlimit { 1 } else { memlimit };
    (*coder).flags = flags;
    (*coder).sequence = SEQ_INIT;
    LZMA_OK
}
pub unsafe extern "C" fn lzma_auto_decoder(
    strm: *mut lzma_stream,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = auto_decoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        memlimit,
        flags,
    );
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true;
    LZMA_OK
}
