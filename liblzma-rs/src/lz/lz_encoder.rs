use crate::types::*;
use core::ffi::c_void;
extern "C" {
    fn lzma_mf_hc3_find(dict: *mut lzma_mf, matches: *mut lzma_match) -> u32;
    fn lzma_mf_hc3_skip(dict: *mut lzma_mf, amount: u32);
    fn lzma_mf_hc4_find(dict: *mut lzma_mf, matches: *mut lzma_match) -> u32;
    fn lzma_mf_hc4_skip(dict: *mut lzma_mf, amount: u32);
    fn lzma_mf_bt2_find(dict: *mut lzma_mf, matches: *mut lzma_match) -> u32;
    fn lzma_mf_bt2_skip(dict: *mut lzma_mf, amount: u32);
    fn lzma_mf_bt3_find(dict: *mut lzma_mf, matches: *mut lzma_match) -> u32;
    fn lzma_mf_bt3_skip(dict: *mut lzma_mf, amount: u32);
    fn lzma_mf_bt4_find(dict: *mut lzma_mf, matches: *mut lzma_match) -> u32;
    fn lzma_mf_bt4_skip(dict: *mut lzma_mf, amount: u32);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_options {
    pub before_size: size_t,
    pub dict_size: size_t,
    pub after_size: size_t,
    pub match_len_max: size_t,
    pub nice_len: size_t,
    pub match_finder: lzma_match_finder,
    pub depth: u32,
    pub preset_dict: *const u8,
    pub preset_dict_size: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_coder {
    pub lz: lzma_lz_encoder,
    pub mf: lzma_mf,
    pub next: lzma_next_coder,
}
pub const LZMA_MEMCMPLEN_EXTRA: u32 = 0;
unsafe extern "C" fn move_window(mf: *mut lzma_mf) {
    let move_offset: u32 = (*mf).read_pos.wrapping_sub((*mf).keep_size_before) & !(15);
    let move_size: size_t = (*mf).write_pos.wrapping_sub(move_offset) as size_t;
    core::ptr::copy(
        (*mf).buffer.offset(move_offset as isize) as *const u8,
        (*mf).buffer as *mut u8,
        move_size,
    );
    (*mf).offset = (*mf).offset.wrapping_add(move_offset);
    (*mf).read_pos = (*mf).read_pos.wrapping_sub(move_offset);
    (*mf).read_limit = (*mf).read_limit.wrapping_sub(move_offset);
    (*mf).write_pos = (*mf).write_pos.wrapping_sub(move_offset);
}
unsafe extern "C" fn fill_window(
    coder: *mut lzma_coder,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    action: lzma_action,
) -> lzma_ret {
    if (*coder).mf.read_pos >= (*coder).mf.size.wrapping_sub((*coder).mf.keep_size_after) {
        move_window(&raw mut (*coder).mf);
    }
    let mut write_pos: size_t = (*coder).mf.write_pos as size_t;
    let mut ret: lzma_ret = LZMA_OK;
    if (*coder).next.code.is_none() {
        lzma_bufcpy(
            in_0,
            in_pos,
            in_size,
            (*coder).mf.buffer,
            &raw mut write_pos,
            (*coder).mf.size as size_t,
        );
        ret = if action != LZMA_RUN && *in_pos == in_size {
            LZMA_STREAM_END
        } else {
            LZMA_OK
        };
    } else {
        ret = (*coder).next.code.unwrap()(
            (*coder).next.coder,
            allocator,
            in_0,
            in_pos,
            in_size,
            (*coder).mf.buffer,
            &raw mut write_pos,
            (*coder).mf.size as size_t,
            action,
        );
    }
    (*coder).mf.write_pos = write_pos as u32;
    core::ptr::write_bytes(
        (*coder).mf.buffer.offset(write_pos as isize) as *mut u8,
        0 as u8,
        0,
    );
    if ret == LZMA_STREAM_END {
        ret = LZMA_OK;
        (*coder).mf.action = action;
        (*coder).mf.read_limit = (*coder).mf.write_pos;
    } else if (*coder).mf.write_pos > (*coder).mf.keep_size_after {
        (*coder).mf.read_limit = (*coder)
            .mf
            .write_pos
            .wrapping_sub((*coder).mf.keep_size_after);
    }
    if (*coder).mf.pending > 0 && (*coder).mf.read_pos < (*coder).mf.read_limit {
        let pending: u32 = (*coder).mf.pending;
        (*coder).mf.pending = 0;
        (*coder).mf.read_pos = (*coder).mf.read_pos.wrapping_sub(pending);
        (*coder).mf.skip.unwrap()(&raw mut (*coder).mf, pending);
    }
    ret
}
unsafe extern "C" fn lz_encode(
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
    while *out_pos < out_size && (*in_pos < in_size || action != LZMA_RUN) {
        if (*coder).mf.action == LZMA_RUN && (*coder).mf.read_pos >= (*coder).mf.read_limit {
            let ret_: lzma_ret = fill_window(coder, allocator, in_0, in_pos, in_size, action);
            if ret_ != LZMA_OK {
                return ret_;
            }
        }
        let ret: lzma_ret = (*coder).lz.code.unwrap()(
            (*coder).lz.coder,
            &raw mut (*coder).mf,
            out,
            out_pos,
            out_size,
        );
        if ret != LZMA_OK {
            (*coder).mf.action = LZMA_RUN;
            return ret;
        }
    }
    LZMA_OK
}
unsafe extern "C" fn lz_encoder_prepare(
    mf: *mut lzma_mf,
    allocator: *const lzma_allocator,
    lz_options: *const lzma_lz_options,
) -> bool {
    if (*lz_options).dict_size < LZMA_DICT_SIZE_MIN as size_t
        || (*lz_options).dict_size > (1u32 << 30).wrapping_add(1u32 << 29) as size_t
        || (*lz_options).nice_len > (*lz_options).match_len_max
    {
        return true;
    }
    (*mf).keep_size_before = (*lz_options)
        .before_size
        .wrapping_add((*lz_options).dict_size) as u32;
    (*mf).keep_size_after = (*lz_options)
        .after_size
        .wrapping_add((*lz_options).match_len_max) as u32;
    let mut reserve: u32 = (*lz_options).dict_size.wrapping_div(2) as u32;
    if reserve > 1 << 30 {
        reserve = reserve.wrapping_div(2);
    }
    reserve = (reserve as size_t).wrapping_add(
        (*lz_options)
            .before_size
            .wrapping_add((*lz_options).match_len_max)
            .wrapping_add((*lz_options).after_size)
            .wrapping_div(2)
            .wrapping_add((1u32 << 19) as size_t),
    ) as u32;
    let old_size: u32 = (*mf).size;
    (*mf).size = (*mf)
        .keep_size_before
        .wrapping_add(reserve)
        .wrapping_add((*mf).keep_size_after);
    if !(*mf).buffer.is_null() && old_size != (*mf).size {
        lzma_free((*mf).buffer as *mut c_void, allocator);
        (*mf).buffer = core::ptr::null_mut();
    }
    (*mf).match_len_max = (*lz_options).match_len_max as u32;
    (*mf).nice_len = (*lz_options).nice_len as u32;
    (*mf).cyclic_size = (*lz_options).dict_size.wrapping_add(1) as u32;
    match (*lz_options).match_finder {
        3 => {
            (*mf).find = Some(
                lzma_mf_hc3_find as unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> u32,
            );
            (*mf).skip = Some(lzma_mf_hc3_skip as unsafe extern "C" fn(*mut lzma_mf, u32) -> ());
        }
        4 => {
            (*mf).find = Some(
                lzma_mf_hc4_find as unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> u32,
            );
            (*mf).skip = Some(lzma_mf_hc4_skip as unsafe extern "C" fn(*mut lzma_mf, u32) -> ());
        }
        18 => {
            (*mf).find = Some(
                lzma_mf_bt2_find as unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> u32,
            );
            (*mf).skip = Some(lzma_mf_bt2_skip as unsafe extern "C" fn(*mut lzma_mf, u32) -> ());
        }
        19 => {
            (*mf).find = Some(
                lzma_mf_bt3_find as unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> u32,
            );
            (*mf).skip = Some(lzma_mf_bt3_skip as unsafe extern "C" fn(*mut lzma_mf, u32) -> ());
        }
        20 => {
            (*mf).find = Some(
                lzma_mf_bt4_find as unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> u32,
            );
            (*mf).skip = Some(lzma_mf_bt4_skip as unsafe extern "C" fn(*mut lzma_mf, u32) -> ());
        }
        _ => return true,
    }
    let hash_bytes: u32 = mf_get_hash_bytes((*lz_options).match_finder) as u32;
    let is_bt: bool = (*lz_options).match_finder & 0x10 != 0;
    let mut hs: u32 = 0;
    if hash_bytes == 2 {
        hs = 0xffff;
    } else {
        hs = (*lz_options).dict_size.wrapping_sub(1) as u32;
        hs |= hs >> 1;
        hs |= hs >> 2;
        hs |= hs >> 4;
        hs |= hs >> 8;
        hs >>= 1;
        hs |= 0xffff;
        if hs > 1 << 24 {
            if hash_bytes == 3 {
                hs = (1u32 << 24).wrapping_sub(1) as u32;
            } else {
                hs >>= 1;
            }
        }
    }
    (*mf).hash_mask = hs;
    hs += 1;
    if hash_bytes > 2 {
        hs = hs.wrapping_add(HASH_2_SIZE);
    }
    if hash_bytes > 3 {
        hs = hs.wrapping_add(HASH_3_SIZE);
    }
    let old_hash_count: u32 = (*mf).hash_count;
    let old_sons_count: u32 = (*mf).sons_count;
    (*mf).hash_count = hs;
    (*mf).sons_count = (*mf).cyclic_size;
    if is_bt {
        (*mf).sons_count = (*mf).sons_count.wrapping_mul(2);
    }
    if old_hash_count != (*mf).hash_count || old_sons_count != (*mf).sons_count {
        lzma_free((*mf).hash as *mut c_void, allocator);
        (*mf).hash = core::ptr::null_mut();
        lzma_free((*mf).son as *mut c_void, allocator);
        (*mf).son = core::ptr::null_mut();
    }
    (*mf).depth = (*lz_options).depth;
    if (*mf).depth == 0 {
        if is_bt {
            (*mf).depth = (16u32).wrapping_add((*mf).nice_len.wrapping_div(2));
        } else {
            (*mf).depth = (4u32).wrapping_add((*mf).nice_len.wrapping_div(4));
        }
    }
    false
}
unsafe extern "C" fn lz_encoder_init(
    mf: *mut lzma_mf,
    allocator: *const lzma_allocator,
    lz_options: *const lzma_lz_options,
) -> bool {
    if (*mf).buffer.is_null() {
        (*mf).buffer = lzma_alloc(
            (*mf).size.wrapping_add(LZMA_MEMCMPLEN_EXTRA) as size_t,
            allocator,
        ) as *mut u8;
        if (*mf).buffer.is_null() {
            return true;
        }
        core::ptr::write_bytes(
            (*mf).buffer.offset((*mf).size as isize) as *mut u8,
            0 as u8,
            0,
        );
    }
    (*mf).offset = (*mf).cyclic_size;
    (*mf).read_pos = 0;
    (*mf).read_ahead = 0;
    (*mf).read_limit = 0;
    (*mf).write_pos = 0;
    (*mf).pending = 0;
    if (*mf).hash.is_null() {
        (*mf).hash = lzma_alloc_zero(
            ((*mf).hash_count as size_t).wrapping_mul(core::mem::size_of::<u32>()),
            allocator,
        ) as *mut u32;
        (*mf).son = lzma_alloc(
            ((*mf).sons_count as size_t).wrapping_mul(core::mem::size_of::<u32>()),
            allocator,
        ) as *mut u32;
        if (*mf).hash.is_null() || (*mf).son.is_null() {
            lzma_free((*mf).hash as *mut c_void, allocator);
            (*mf).hash = core::ptr::null_mut();
            lzma_free((*mf).son as *mut c_void, allocator);
            (*mf).son = core::ptr::null_mut();
            return true;
        }
    } else {
        core::ptr::write_bytes(
            (*mf).hash as *mut u8,
            0 as u8,
            ((*mf).hash_count as size_t).wrapping_mul(core::mem::size_of::<u32>()),
        );
    }
    (*mf).cyclic_pos = 0;
    if !(*lz_options).preset_dict.is_null() && (*lz_options).preset_dict_size > 0 {
        (*mf).write_pos = if (*lz_options).preset_dict_size < (*mf).size {
            (*lz_options).preset_dict_size
        } else {
            (*mf).size
        };
        core::ptr::copy_nonoverlapping(
            (*lz_options)
                .preset_dict
                .offset((*lz_options).preset_dict_size as isize)
                .offset(-((*mf).write_pos as isize)) as *const u8,
            (*mf).buffer as *mut u8,
            (*mf).write_pos as size_t,
        );
        (*mf).action = LZMA_SYNC_FLUSH;
        (*mf).skip.unwrap()(mf, (*mf).write_pos);
    }
    (*mf).action = LZMA_RUN;
    false
}
#[no_mangle]
pub extern "C" fn lzma_lz_encoder_memusage(lz_options: *const lzma_lz_options) -> u64 {
    let mut mf: lzma_mf = lzma_mf_s {
        buffer: core::ptr::null_mut(),
        size: 0,
        keep_size_before: 0,
        keep_size_after: 0,
        offset: 0,
        read_pos: 0,
        read_ahead: 0,
        read_limit: 0,
        write_pos: 0,
        pending: 0,
        find: None,
        skip: None,
        hash: core::ptr::null_mut(),
        son: core::ptr::null_mut(),
        cyclic_pos: 0,
        cyclic_size: 0,
        hash_mask: 0,
        depth: 0,
        nice_len: 0,
        match_len_max: 0,
        action: LZMA_RUN,
        hash_count: 0,
        sons_count: 0,
    };
    if unsafe { lz_encoder_prepare(&raw mut mf, core::ptr::null(), lz_options) } {
        return UINT64_MAX;
    }
    (mf.hash_count as u64)
        .wrapping_add(mf.sons_count as u64)
        .wrapping_mul(core::mem::size_of::<u32>() as u64)
        .wrapping_add(mf.size as u64)
        .wrapping_add(core::mem::size_of::<lzma_coder>() as u64)
}
unsafe extern "C" fn lz_encoder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_coder = coder_ptr as *mut lzma_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free((*coder).mf.son as *mut c_void, allocator);
    lzma_free((*coder).mf.hash as *mut c_void, allocator);
    lzma_free((*coder).mf.buffer as *mut c_void, allocator);
    if (*coder).lz.end.is_some() {
        (*coder).lz.end.unwrap()((*coder).lz.coder, allocator);
    } else {
        lzma_free((*coder).lz.coder, allocator);
    }
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn lz_encoder_update(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    _filters_null: *const lzma_filter,
    reversed_filters: *const lzma_filter,
) -> lzma_ret {
    let coder: *mut lzma_coder = coder_ptr as *mut lzma_coder;
    if (*coder).lz.options_update.is_none() {
        return LZMA_PROG_ERROR;
    }
    let ret_: lzma_ret = (*coder).lz.options_update.unwrap()((*coder).lz.coder, reversed_filters);
    if ret_ != LZMA_OK {
        return ret_;
    }
    lzma_next_filter_update(
        &raw mut (*coder).next,
        allocator,
        reversed_filters.offset(1),
    )
}
unsafe extern "C" fn lz_encoder_set_out_limit(
    coder_ptr: *mut c_void,
    uncomp_size: *mut u64,
    out_limit: u64,
) -> lzma_ret {
    let coder: *mut lzma_coder = coder_ptr as *mut lzma_coder;
    if (*coder).next.code.is_none() && (*coder).lz.set_out_limit.is_some() {
        return (*coder).lz.set_out_limit.unwrap()((*coder).lz.coder, uncomp_size, out_limit);
    }
    LZMA_OPTIONS_ERROR
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lz_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    lz_init: Option<
        unsafe extern "C" fn(
            *mut lzma_lz_encoder,
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
            lz_encode
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
        (*next).end =
            Some(lz_encoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ());
        (*next).update = Some(
            lz_encoder_update
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const lzma_allocator,
                    *const lzma_filter,
                    *const lzma_filter,
                ) -> lzma_ret,
        );
        (*next).set_out_limit = Some(
            lz_encoder_set_out_limit
                as unsafe extern "C" fn(*mut c_void, *mut u64, u64) -> lzma_ret,
        );
        (*coder).lz.coder = core::ptr::null_mut();
        (*coder).lz.code = None;
        (*coder).lz.end = None;
        (*coder).lz.options_update = None;
        (*coder).lz.set_out_limit = None;
        (*coder).mf.buffer = core::ptr::null_mut();
        (*coder).mf.size = 0;
        (*coder).mf.hash = core::ptr::null_mut();
        (*coder).mf.son = core::ptr::null_mut();
        (*coder).mf.hash_count = 0;
        (*coder).mf.sons_count = 0;
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
        before_size: 0,
        dict_size: 0,
        after_size: 0,
        match_len_max: 0,
        nice_len: 0,
        match_finder: 0,
        depth: 0,
        preset_dict: core::ptr::null(),
        preset_dict_size: 0,
    };
    let ret_: lzma_ret = lz_init.unwrap()(
        &raw mut (*coder).lz,
        allocator,
        (*filters).id,
        (*filters).options,
        &raw mut lz_options,
    );
    if ret_ != LZMA_OK {
        return ret_;
    }
    if lz_encoder_prepare(&raw mut (*coder).mf, allocator, &raw mut lz_options) {
        return LZMA_OPTIONS_ERROR;
    }
    if lz_encoder_init(&raw mut (*coder).mf, allocator, &raw mut lz_options) {
        return LZMA_MEM_ERROR;
    }
    lzma_next_filter_init(&raw mut (*coder).next, allocator, filters.offset(1))
}
pub extern "C" fn lzma_mf_is_supported(mf: lzma_match_finder) -> lzma_bool {
    match mf {
        3 => return true as lzma_bool,
        4 => return true as lzma_bool,
        18 => return true as lzma_bool,
        19 => return true as lzma_bool,
        20 => return true as lzma_bool,
        _ => return false as lzma_bool,
    };
}
