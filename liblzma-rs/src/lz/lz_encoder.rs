extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut ::core::ffi::c_void;
    fn lzma_alloc_zero(size: size_t, allocator: *const lzma_allocator) -> *mut ::core::ffi::c_void;
    fn lzma_free(ptr: *mut ::core::ffi::c_void, allocator: *const lzma_allocator);
    fn lzma_next_filter_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_next_filter_update(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        reversed_filters: *const lzma_filter,
    ) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
    fn lzma_bufcpy(
        in_0: *const uint8_t,
        in_pos: *mut size_t,
        in_size: size_t,
        out: *mut uint8_t,
        out_pos: *mut size_t,
        out_size: size_t,
    ) -> size_t;
    fn lzma_mf_hc3_find(dict: *mut lzma_mf, matches: *mut lzma_match) -> uint32_t;
    fn lzma_mf_hc3_skip(dict: *mut lzma_mf, amount: uint32_t);
    fn lzma_mf_hc4_find(dict: *mut lzma_mf, matches: *mut lzma_match) -> uint32_t;
    fn lzma_mf_hc4_skip(dict: *mut lzma_mf, amount: uint32_t);
    fn lzma_mf_bt2_find(dict: *mut lzma_mf, matches: *mut lzma_match) -> uint32_t;
    fn lzma_mf_bt2_skip(dict: *mut lzma_mf, amount: uint32_t);
    fn lzma_mf_bt3_find(dict: *mut lzma_mf, matches: *mut lzma_match) -> uint32_t;
    fn lzma_mf_bt3_skip(dict: *mut lzma_mf, amount: uint32_t);
    fn lzma_mf_bt4_find(dict: *mut lzma_mf, matches: *mut lzma_match) -> uint32_t;
    fn lzma_mf_bt4_skip(dict: *mut lzma_mf, amount: uint32_t);
}
pub type __darwin_size_t = usize;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type lzma_bool = ::core::ffi::c_uchar;
pub type lzma_ret = ::core::ffi::c_uint;
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
pub type lzma_action = ::core::ffi::c_uint;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, size_t) -> *mut ::core::ffi::c_void,
    >,
    pub free:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}
pub type lzma_next_coder = lzma_next_coder_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_next_coder_s {
    pub coder: *mut ::core::ffi::c_void,
    pub id: lzma_vli,
    pub init: uintptr_t,
    pub code: lzma_code_function,
    pub end: lzma_end_function,
    pub get_progress:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, *mut uint64_t) -> ()>,
    pub get_check: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> lzma_check>,
    pub memconfig: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut uint64_t,
            *mut uint64_t,
            uint64_t,
        ) -> lzma_ret,
    >,
    pub update: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const lzma_allocator,
            *const lzma_filter,
            *const lzma_filter,
        ) -> lzma_ret,
    >,
    pub set_out_limit:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, uint64_t) -> lzma_ret>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut ::core::ffi::c_void,
}
pub type lzma_vli = uint64_t;
pub type lzma_check = ::core::ffi::c_uint;
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
pub type lzma_end_function =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_allocator) -> ()>;
pub type lzma_code_function = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const lzma_allocator,
        *const uint8_t,
        *mut size_t,
        size_t,
        *mut uint8_t,
        *mut size_t,
        size_t,
        lzma_action,
    ) -> lzma_ret,
>;
pub type lzma_match_finder = ::core::ffi::c_uint;
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_info_s {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub options: *mut ::core::ffi::c_void,
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
pub struct lzma_match {
    pub len: uint32_t,
    pub dist: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_mf_s {
    pub buffer: *mut uint8_t,
    pub size: uint32_t,
    pub keep_size_before: uint32_t,
    pub keep_size_after: uint32_t,
    pub offset: uint32_t,
    pub read_pos: uint32_t,
    pub read_ahead: uint32_t,
    pub read_limit: uint32_t,
    pub write_pos: uint32_t,
    pub pending: uint32_t,
    pub find: Option<unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t>,
    pub skip: Option<unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ()>,
    pub hash: *mut uint32_t,
    pub son: *mut uint32_t,
    pub cyclic_pos: uint32_t,
    pub cyclic_size: uint32_t,
    pub hash_mask: uint32_t,
    pub depth: uint32_t,
    pub nice_len: uint32_t,
    pub match_len_max: uint32_t,
    pub action: lzma_action,
    pub hash_count: uint32_t,
    pub sons_count: uint32_t,
}
pub type lzma_mf = lzma_mf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_options {
    pub before_size: size_t,
    pub dict_size: size_t,
    pub after_size: size_t,
    pub match_len_max: size_t,
    pub nice_len: size_t,
    pub match_finder: lzma_match_finder,
    pub depth: uint32_t,
    pub preset_dict: *const uint8_t,
    pub preset_dict_size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_lz_encoder {
    pub coder: *mut ::core::ffi::c_void,
    pub code: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut lzma_mf,
            *mut uint8_t,
            *mut size_t,
            size_t,
        ) -> lzma_ret,
    >,
    pub end: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_allocator) -> ()>,
    pub options_update:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_filter) -> lzma_ret>,
    pub set_out_limit:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, uint64_t) -> lzma_ret>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_coder {
    pub lz: lzma_lz_encoder,
    pub mf: lzma_mf,
    pub next: lzma_next_coder,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const UINT64_MAX: ::core::ffi::c_ulonglong = 18446744073709551615 as ::core::ffi::c_ulonglong;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LZMA_VLI_UNKNOWN: ::core::ffi::c_ulonglong = UINT64_MAX;
pub const LZMA_DICT_SIZE_MIN: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn mf_get_hash_bytes(mut match_finder: lzma_match_finder) -> uint32_t {
    return match_finder as uint32_t & 0xf as uint32_t;
}
pub const HASH_2_SIZE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << 10 as ::core::ffi::c_int;
pub const HASH_3_SIZE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << 16 as ::core::ffi::c_int;
pub const LZMA_MEMCMPLEN_EXTRA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn move_window(mut mf: *mut lzma_mf) {
    let move_offset: uint32_t =
        (*mf).read_pos.wrapping_sub((*mf).keep_size_before) & !(15 as uint32_t);
    let move_size: size_t = (*mf).write_pos.wrapping_sub(move_offset) as size_t;
    memmove(
        (*mf).buffer as *mut ::core::ffi::c_void,
        (*mf).buffer.offset(move_offset as isize) as *const ::core::ffi::c_void,
        move_size,
    );
    (*mf).offset = (*mf).offset.wrapping_add(move_offset);
    (*mf).read_pos = (*mf).read_pos.wrapping_sub(move_offset);
    (*mf).read_limit = (*mf).read_limit.wrapping_sub(move_offset);
    (*mf).write_pos = (*mf).write_pos.wrapping_sub(move_offset);
}
unsafe extern "C" fn fill_window(
    mut coder: *mut lzma_coder,
    mut allocator: *const lzma_allocator,
    mut in_0: *const uint8_t,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut action: lzma_action,
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
        ret = (if action as ::core::ffi::c_uint
            != LZMA_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
            && *in_pos == in_size
        {
            LZMA_STREAM_END as ::core::ffi::c_int
        } else {
            LZMA_OK as ::core::ffi::c_int
        }) as lzma_ret;
    } else {
        ret = (*coder).next.code.expect("non-null function pointer")(
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
    (*coder).mf.write_pos = write_pos as uint32_t;
    memset(
        (*coder).mf.buffer.offset(write_pos as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        0 as size_t,
    );
    if ret as ::core::ffi::c_uint == LZMA_STREAM_END as ::core::ffi::c_int as ::core::ffi::c_uint {
        ret = LZMA_OK;
        (*coder).mf.action = action;
        (*coder).mf.read_limit = (*coder).mf.write_pos;
    } else if (*coder).mf.write_pos > (*coder).mf.keep_size_after {
        (*coder).mf.read_limit = (*coder)
            .mf
            .write_pos
            .wrapping_sub((*coder).mf.keep_size_after);
    }
    if (*coder).mf.pending > 0 as uint32_t && (*coder).mf.read_pos < (*coder).mf.read_limit {
        let pending: uint32_t = (*coder).mf.pending;
        (*coder).mf.pending = 0 as uint32_t;
        (*coder).mf.read_pos = (*coder).mf.read_pos.wrapping_sub(pending);
        (*coder).mf.skip.expect("non-null function pointer")(&raw mut (*coder).mf, pending);
    }
    return ret;
}
unsafe extern "C" fn lz_encode(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut allocator: *const lzma_allocator,
    mut in_0: *const uint8_t,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut out: *mut uint8_t,
    mut out_pos: *mut size_t,
    mut out_size: size_t,
    mut action: lzma_action,
) -> lzma_ret {
    let mut coder: *mut lzma_coder = coder_ptr as *mut lzma_coder;
    while *out_pos < out_size
        && (*in_pos < in_size
            || action as ::core::ffi::c_uint
                != LZMA_RUN as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        if (*coder).mf.action as ::core::ffi::c_uint
            == LZMA_RUN as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*coder).mf.read_pos >= (*coder).mf.read_limit
        {
            let ret_: lzma_ret =
                fill_window(coder, allocator, in_0, in_pos, in_size, action) as lzma_ret;
            if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
                return ret_;
            }
        }
        let ret: lzma_ret = (*coder).lz.code.expect("non-null function pointer")(
            (*coder).lz.coder,
            &raw mut (*coder).mf,
            out,
            out_pos,
            out_size,
        ) as lzma_ret;
        if ret as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
            (*coder).mf.action = LZMA_RUN;
            return ret;
        }
    }
    return LZMA_OK;
}
unsafe extern "C" fn lz_encoder_prepare(
    mut mf: *mut lzma_mf,
    mut allocator: *const lzma_allocator,
    mut lz_options: *const lzma_lz_options,
) -> bool {
    if !((*lz_options).dict_size >= LZMA_DICT_SIZE_MIN as size_t
        && (*lz_options).dict_size
            <= ((1 as ::core::ffi::c_uint) << 30 as ::core::ffi::c_int)
                .wrapping_add((1 as ::core::ffi::c_uint) << 29 as ::core::ffi::c_int)
                as size_t)
        || (*lz_options).nice_len > (*lz_options).match_len_max
    {
        return true_0 != 0;
    }
    (*mf).keep_size_before = (*lz_options)
        .before_size
        .wrapping_add((*lz_options).dict_size) as uint32_t;
    (*mf).keep_size_after = (*lz_options)
        .after_size
        .wrapping_add((*lz_options).match_len_max) as uint32_t;
    let mut reserve: uint32_t = (*lz_options).dict_size.wrapping_div(2 as size_t) as uint32_t;
    if reserve > (1 as uint32_t) << 30 as ::core::ffi::c_int {
        reserve = reserve.wrapping_div(2 as uint32_t);
    }
    reserve = (reserve as size_t).wrapping_add(
        (*lz_options)
            .before_size
            .wrapping_add((*lz_options).match_len_max)
            .wrapping_add((*lz_options).after_size)
            .wrapping_div(2 as size_t)
            .wrapping_add(((1 as ::core::ffi::c_uint) << 19 as ::core::ffi::c_int) as size_t),
    ) as uint32_t as uint32_t;
    let old_size: uint32_t = (*mf).size;
    (*mf).size = (*mf)
        .keep_size_before
        .wrapping_add(reserve)
        .wrapping_add((*mf).keep_size_after);
    if !(*mf).buffer.is_null() && old_size != (*mf).size {
        lzma_free((*mf).buffer as *mut ::core::ffi::c_void, allocator);
        (*mf).buffer = ::core::ptr::null_mut::<uint8_t>();
    }
    (*mf).match_len_max = (*lz_options).match_len_max as uint32_t;
    (*mf).nice_len = (*lz_options).nice_len as uint32_t;
    (*mf).cyclic_size = (*lz_options).dict_size.wrapping_add(1 as size_t) as uint32_t;
    match (*lz_options).match_finder as ::core::ffi::c_uint {
        3 => {
            (*mf).find = Some(
                lzma_mf_hc3_find as unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t,
            )
                as Option<unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t>;
            (*mf).skip =
                Some(lzma_mf_hc3_skip as unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ())
                    as Option<unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ()>;
        }
        4 => {
            (*mf).find = Some(
                lzma_mf_hc4_find as unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t,
            )
                as Option<unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t>;
            (*mf).skip =
                Some(lzma_mf_hc4_skip as unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ())
                    as Option<unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ()>;
        }
        18 => {
            (*mf).find = Some(
                lzma_mf_bt2_find as unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t,
            )
                as Option<unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t>;
            (*mf).skip =
                Some(lzma_mf_bt2_skip as unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ())
                    as Option<unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ()>;
        }
        19 => {
            (*mf).find = Some(
                lzma_mf_bt3_find as unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t,
            )
                as Option<unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t>;
            (*mf).skip =
                Some(lzma_mf_bt3_skip as unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ())
                    as Option<unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ()>;
        }
        20 => {
            (*mf).find = Some(
                lzma_mf_bt4_find as unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t,
            )
                as Option<unsafe extern "C" fn(*mut lzma_mf, *mut lzma_match) -> uint32_t>;
            (*mf).skip =
                Some(lzma_mf_bt4_skip as unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ())
                    as Option<unsafe extern "C" fn(*mut lzma_mf, uint32_t) -> ()>;
        }
        _ => return true_0 != 0,
    }
    let hash_bytes: uint32_t = mf_get_hash_bytes((*lz_options).match_finder) as uint32_t;
    let is_bt: bool = (*lz_options).match_finder as ::core::ffi::c_uint
        & 0x10 as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint;
    let mut hs: uint32_t = 0;
    if hash_bytes == 2 as uint32_t {
        hs = 0xffff as uint32_t;
    } else {
        hs = (*lz_options).dict_size.wrapping_sub(1 as size_t) as uint32_t;
        hs |= hs >> 1 as ::core::ffi::c_int;
        hs |= hs >> 2 as ::core::ffi::c_int;
        hs |= hs >> 4 as ::core::ffi::c_int;
        hs |= hs >> 8 as ::core::ffi::c_int;
        hs >>= 1 as ::core::ffi::c_int;
        hs |= 0xffff as uint32_t;
        if hs > (1 as uint32_t) << 24 as ::core::ffi::c_int {
            if hash_bytes == 3 as uint32_t {
                hs = ((1 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    .wrapping_sub(1 as ::core::ffi::c_uint) as uint32_t;
            } else {
                hs >>= 1 as ::core::ffi::c_int;
            }
        }
    }
    (*mf).hash_mask = hs;
    hs = hs.wrapping_add(1);
    if hash_bytes > 2 as uint32_t {
        hs = (hs as ::core::ffi::c_uint).wrapping_add(HASH_2_SIZE) as uint32_t as uint32_t;
    }
    if hash_bytes > 3 as uint32_t {
        hs = (hs as ::core::ffi::c_uint).wrapping_add(HASH_3_SIZE) as uint32_t as uint32_t;
    }
    let old_hash_count: uint32_t = (*mf).hash_count;
    let old_sons_count: uint32_t = (*mf).sons_count;
    (*mf).hash_count = hs;
    (*mf).sons_count = (*mf).cyclic_size;
    if is_bt {
        (*mf).sons_count = (*mf).sons_count.wrapping_mul(2 as uint32_t);
    }
    if old_hash_count != (*mf).hash_count || old_sons_count != (*mf).sons_count {
        lzma_free((*mf).hash as *mut ::core::ffi::c_void, allocator);
        (*mf).hash = ::core::ptr::null_mut::<uint32_t>();
        lzma_free((*mf).son as *mut ::core::ffi::c_void, allocator);
        (*mf).son = ::core::ptr::null_mut::<uint32_t>();
    }
    (*mf).depth = (*lz_options).depth;
    if (*mf).depth == 0 as uint32_t {
        if is_bt {
            (*mf).depth = (16 as uint32_t).wrapping_add((*mf).nice_len.wrapping_div(2 as uint32_t));
        } else {
            (*mf).depth = (4 as uint32_t).wrapping_add((*mf).nice_len.wrapping_div(4 as uint32_t));
        }
    }
    return false_0 != 0;
}
unsafe extern "C" fn lz_encoder_init(
    mut mf: *mut lzma_mf,
    mut allocator: *const lzma_allocator,
    mut lz_options: *const lzma_lz_options,
) -> bool {
    if (*mf).buffer.is_null() {
        (*mf).buffer = lzma_alloc(
            (*mf).size.wrapping_add(LZMA_MEMCMPLEN_EXTRA as uint32_t) as size_t,
            allocator,
        ) as *mut uint8_t;
        if (*mf).buffer.is_null() {
            return true_0 != 0;
        }
        memset(
            (*mf).buffer.offset((*mf).size as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            0 as size_t,
        );
    }
    (*mf).offset = (*mf).cyclic_size;
    (*mf).read_pos = 0 as uint32_t;
    (*mf).read_ahead = 0 as uint32_t;
    (*mf).read_limit = 0 as uint32_t;
    (*mf).write_pos = 0 as uint32_t;
    (*mf).pending = 0 as uint32_t;
    if (*mf).hash.is_null() {
        (*mf).hash = lzma_alloc_zero(
            ((*mf).hash_count as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            allocator,
        ) as *mut uint32_t;
        (*mf).son = lzma_alloc(
            ((*mf).sons_count as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
            allocator,
        ) as *mut uint32_t;
        if (*mf).hash.is_null() || (*mf).son.is_null() {
            lzma_free((*mf).hash as *mut ::core::ffi::c_void, allocator);
            (*mf).hash = ::core::ptr::null_mut::<uint32_t>();
            lzma_free((*mf).son as *mut ::core::ffi::c_void, allocator);
            (*mf).son = ::core::ptr::null_mut::<uint32_t>();
            return true_0 != 0;
        }
    } else {
        memset(
            (*mf).hash as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*mf).hash_count as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t),
        );
    }
    (*mf).cyclic_pos = 0 as uint32_t;
    if !(*lz_options).preset_dict.is_null() && (*lz_options).preset_dict_size > 0 as uint32_t {
        (*mf).write_pos = if (*lz_options).preset_dict_size < (*mf).size {
            (*lz_options).preset_dict_size
        } else {
            (*mf).size
        };
        memcpy(
            (*mf).buffer as *mut ::core::ffi::c_void,
            (*lz_options)
                .preset_dict
                .offset((*lz_options).preset_dict_size as isize)
                .offset(-((*mf).write_pos as isize)) as *const ::core::ffi::c_void,
            (*mf).write_pos as size_t,
        );
        (*mf).action = LZMA_SYNC_FLUSH;
        (*mf).skip.expect("non-null function pointer")(mf, (*mf).write_pos);
    }
    (*mf).action = LZMA_RUN;
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lz_encoder_memusage(
    mut lz_options: *const lzma_lz_options,
) -> uint64_t {
    let mut mf: lzma_mf = lzma_mf_s {
        buffer: ::core::ptr::null_mut::<uint8_t>(),
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
        hash: ::core::ptr::null_mut::<uint32_t>(),
        son: ::core::ptr::null_mut::<uint32_t>(),
        cyclic_pos: 0,
        cyclic_size: 0,
        hash_mask: 0,
        depth: 0,
        nice_len: 0,
        match_len_max: 0,
        action: LZMA_RUN,
        hash_count: 0 as uint32_t,
        sons_count: 0 as uint32_t,
    };
    if lz_encoder_prepare(
        &raw mut mf,
        ::core::ptr::null::<lzma_allocator>(),
        lz_options,
    ) {
        return UINT64_MAX as uint64_t;
    }
    return (mf.hash_count as uint64_t)
        .wrapping_add(mf.sons_count as uint64_t)
        .wrapping_mul(::core::mem::size_of::<uint32_t>() as uint64_t)
        .wrapping_add(mf.size as uint64_t)
        .wrapping_add(::core::mem::size_of::<lzma_coder>() as uint64_t);
}
unsafe extern "C" fn lz_encoder_end(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_coder = coder_ptr as *mut lzma_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free((*coder).mf.son as *mut ::core::ffi::c_void, allocator);
    lzma_free((*coder).mf.hash as *mut ::core::ffi::c_void, allocator);
    lzma_free((*coder).mf.buffer as *mut ::core::ffi::c_void, allocator);
    if (*coder).lz.end.is_some() {
        (*coder).lz.end.expect("non-null function pointer")((*coder).lz.coder, allocator);
    } else {
        lzma_free((*coder).lz.coder, allocator);
    }
    lzma_free(coder as *mut ::core::ffi::c_void, allocator);
}
unsafe extern "C" fn lz_encoder_update(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut allocator: *const lzma_allocator,
    mut filters_null: *const lzma_filter,
    mut reversed_filters: *const lzma_filter,
) -> lzma_ret {
    let mut coder: *mut lzma_coder = coder_ptr as *mut lzma_coder;
    if (*coder).lz.options_update.is_none() {
        return LZMA_PROG_ERROR;
    }
    let ret_: lzma_ret = (*coder)
        .lz
        .options_update
        .expect("non-null function pointer")(
        (*coder).lz.coder, reversed_filters
    ) as lzma_ret;
    if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret_;
    }
    return lzma_next_filter_update(
        &raw mut (*coder).next,
        allocator,
        reversed_filters.offset(1 as ::core::ffi::c_int as isize),
    );
}
unsafe extern "C" fn lz_encoder_set_out_limit(
    mut coder_ptr: *mut ::core::ffi::c_void,
    mut uncomp_size: *mut uint64_t,
    mut out_limit: uint64_t,
) -> lzma_ret {
    let mut coder: *mut lzma_coder = coder_ptr as *mut lzma_coder;
    if (*coder).next.code.is_none() && (*coder).lz.set_out_limit.is_some() {
        return (*coder)
            .lz
            .set_out_limit
            .expect("non-null function pointer")(
            (*coder).lz.coder, uncomp_size, out_limit
        );
    }
    return LZMA_OPTIONS_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_lz_encoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
    mut lz_init: Option<
        unsafe extern "C" fn(
            *mut lzma_lz_encoder,
            *const lzma_allocator,
            lzma_vli,
            *const ::core::ffi::c_void,
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
        (*next).coder = coder as *mut ::core::ffi::c_void;
        (*next).code = Some(
            lz_encode
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const lzma_allocator,
                    *const uint8_t,
                    *mut size_t,
                    size_t,
                    *mut uint8_t,
                    *mut size_t,
                    size_t,
                    lzma_action,
                ) -> lzma_ret,
        ) as lzma_code_function;
        (*next).end = Some(
            lz_encoder_end
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).update = Some(
            lz_encoder_update
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const lzma_allocator,
                    *const lzma_filter,
                    *const lzma_filter,
                ) -> lzma_ret,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const lzma_allocator,
                    *const lzma_filter,
                    *const lzma_filter,
                ) -> lzma_ret,
            >;
        (*next).set_out_limit = Some(
            lz_encoder_set_out_limit
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut uint64_t,
                    uint64_t,
                ) -> lzma_ret,
        )
            as Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, uint64_t) -> lzma_ret,
            >;
        (*coder).lz.coder = NULL;
        (*coder).lz.code = None;
        (*coder).lz.end = None;
        (*coder).lz.options_update = None;
        (*coder).lz.set_out_limit = None;
        (*coder).mf.buffer = ::core::ptr::null_mut::<uint8_t>();
        (*coder).mf.size = 0 as uint32_t;
        (*coder).mf.hash = ::core::ptr::null_mut::<uint32_t>();
        (*coder).mf.son = ::core::ptr::null_mut::<uint32_t>();
        (*coder).mf.hash_count = 0 as uint32_t;
        (*coder).mf.sons_count = 0 as uint32_t;
        (*coder).next = lzma_next_coder_s {
            coder: NULL,
            id: LZMA_VLI_UNKNOWN as lzma_vli,
            init: ::core::ptr::null_mut::<::core::ffi::c_void>() as uintptr_t,
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
        match_finder: 0 as lzma_match_finder,
        depth: 0,
        preset_dict: ::core::ptr::null::<uint8_t>(),
        preset_dict_size: 0,
    };
    let ret_: lzma_ret = lz_init.expect("non-null function pointer")(
        &raw mut (*coder).lz,
        allocator,
        (*filters.offset(0 as ::core::ffi::c_int as isize)).id,
        (*filters.offset(0 as ::core::ffi::c_int as isize)).options,
        &raw mut lz_options,
    ) as lzma_ret;
    if ret_ as ::core::ffi::c_uint != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        return ret_;
    }
    if lz_encoder_prepare(&raw mut (*coder).mf, allocator, &raw mut lz_options) {
        return LZMA_OPTIONS_ERROR;
    }
    if lz_encoder_init(&raw mut (*coder).mf, allocator, &raw mut lz_options) {
        return LZMA_MEM_ERROR;
    }
    return lzma_next_filter_init(
        &raw mut (*coder).next,
        allocator,
        filters.offset(1 as ::core::ffi::c_int as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_mf_is_supported(mut mf: lzma_match_finder) -> lzma_bool {
    match mf as ::core::ffi::c_uint {
        3 => return true_0 as lzma_bool,
        4 => return true_0 as lzma_bool,
        18 => return true_0 as lzma_bool,
        19 => return true_0 as lzma_bool,
        20 => return true_0 as lzma_bool,
        _ => return false_0 as lzma_bool,
    };
}
