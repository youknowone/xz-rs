use crate::types::*;
use core::ffi::{c_int, c_uchar, c_uint, c_ulonglong, c_void};
#[repr(C)]
pub struct lzma_index_s {
    _opaque: [u8; 0],
}
extern "C" {
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_stream_header_decode(options: *mut lzma_stream_flags, in_0: *const u8) -> lzma_ret;
    fn lzma_stream_footer_decode(options: *mut lzma_stream_flags, in_0: *const u8) -> lzma_ret;
    fn lzma_stream_flags_compare(
        a: *const lzma_stream_flags,
        b: *const lzma_stream_flags,
    ) -> lzma_ret;
    fn lzma_index_memusage(streams: lzma_vli, blocks: lzma_vli) -> u64;
    fn lzma_index_memused(i: *const lzma_index) -> u64;
    fn lzma_index_end(i: *mut lzma_index, allocator: *const lzma_allocator);
    fn lzma_index_stream_flags(
        i: *mut lzma_index,
        stream_flags: *const lzma_stream_flags,
    ) -> lzma_ret;
    fn lzma_index_stream_padding(i: *mut lzma_index, stream_padding: lzma_vli) -> lzma_ret;
    fn lzma_index_total_size(i: *const lzma_index) -> lzma_vli;
    fn lzma_index_cat(
        dest: *mut lzma_index,
        src: *mut lzma_index,
        allocator: *const lzma_allocator,
    ) -> lzma_ret;
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
    fn lzma_index_decoder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        i: *mut *mut lzma_index,
        memlimit: u64,
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
pub type lzma_index = lzma_index_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_file_info_coder {
    pub sequence: C2RustUnnamed_0,
    pub file_cur_pos: u64,
    pub file_target_pos: u64,
    pub file_size: u64,
    pub index_decoder: lzma_next_coder,
    pub index_remaining: lzma_vli,
    pub this_index: *mut lzma_index,
    pub stream_padding: lzma_vli,
    pub combined_index: *mut lzma_index,
    pub dest_index: *mut *mut lzma_index,
    pub external_seek_pos: *mut u64,
    pub memlimit: u64,
    pub first_header_flags: lzma_stream_flags,
    pub header_flags: lzma_stream_flags,
    pub footer_flags: lzma_stream_flags,
    pub temp_pos: size_t,
    pub temp_size: size_t,
    pub temp: [u8; 8192],
}
pub type C2RustUnnamed_0 = c_uint;
pub const SEQ_HEADER_COMPARE: C2RustUnnamed_0 = 7;
pub const SEQ_HEADER_DECODE: C2RustUnnamed_0 = 6;
pub const SEQ_INDEX_DECODE: C2RustUnnamed_0 = 5;
pub const SEQ_INDEX_INIT: C2RustUnnamed_0 = 4;
pub const SEQ_FOOTER: C2RustUnnamed_0 = 3;
pub const SEQ_PADDING_DECODE: C2RustUnnamed_0 = 2;
pub const SEQ_PADDING_SEEK: C2RustUnnamed_0 = 1;
pub const SEQ_MAGIC_BYTES: C2RustUnnamed_0 = 0;
pub const __DARWIN_NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const NULL: *mut c_void = __DARWIN_NULL;
pub const UINT64_MAX: c_ulonglong = 18446744073709551615;
pub const true_0: c_int = 1 as c_int;
pub const false_0: c_int = 0 as c_int;
pub const LZMA_VLI_MAX: c_ulonglong = UINT64_MAX.wrapping_div(2);
pub const LZMA_VLI_UNKNOWN: c_ulonglong = UINT64_MAX;
pub const LZMA_STREAM_HEADER_SIZE: c_int = 12 as c_int;
unsafe extern "C" fn fill_temp(
    mut coder: *mut lzma_file_info_coder,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
) -> bool {
    (*coder).file_cur_pos = (*coder).file_cur_pos.wrapping_add(lzma_bufcpy(
        in_0,
        in_pos,
        in_size,
        &raw mut (*coder).temp as *mut u8,
        &raw mut (*coder).temp_pos,
        (*coder).temp_size,
    ) as u64);
    return (*coder).temp_pos < (*coder).temp_size;
}
unsafe extern "C" fn seek_to_pos(
    mut coder: *mut lzma_file_info_coder,
    mut target_pos: u64,
    mut in_start: size_t,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
) -> bool {
    let pos_min: u64 = (*coder)
        .file_cur_pos
        .wrapping_sub((*in_pos).wrapping_sub(in_start) as u64);
    let pos_max: u64 = (*coder)
        .file_cur_pos
        .wrapping_add(in_size.wrapping_sub(*in_pos) as u64);
    let mut external_seek_needed: bool = false;
    if target_pos >= pos_min && target_pos <= pos_max {
        *in_pos = (*in_pos).wrapping_add(target_pos.wrapping_sub((*coder).file_cur_pos) as size_t);
        external_seek_needed = false_0 != 0;
    } else {
        *(*coder).external_seek_pos = target_pos;
        external_seek_needed = true_0 != 0;
        *in_pos = in_size;
    }
    (*coder).file_cur_pos = target_pos;
    return external_seek_needed;
}
unsafe extern "C" fn reverse_seek(
    mut coder: *mut lzma_file_info_coder,
    mut in_start: size_t,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
) -> lzma_ret {
    if (*coder).file_target_pos < (2 as c_int * LZMA_STREAM_HEADER_SIZE) as u64 {
        return LZMA_DATA_ERROR;
    }
    (*coder).temp_pos = 0 as size_t;
    if (*coder)
        .file_target_pos
        .wrapping_sub(LZMA_STREAM_HEADER_SIZE as u64)
        < ::core::mem::size_of::<[u8; 8192]>() as u64
    {
        (*coder).temp_size = (*coder)
            .file_target_pos
            .wrapping_sub(LZMA_STREAM_HEADER_SIZE as u64) as size_t;
    } else {
        (*coder).temp_size = ::core::mem::size_of::<[u8; 8192]>() as usize as size_t;
    }
    if seek_to_pos(
        coder,
        (*coder)
            .file_target_pos
            .wrapping_sub((*coder).temp_size as u64),
        in_start,
        in_pos,
        in_size,
    ) {
        return LZMA_SEEK_NEEDED;
    }
    return LZMA_OK;
}
unsafe extern "C" fn get_padding_size(mut buf: *const u8, mut buf_size: size_t) -> size_t {
    let mut padding: size_t = 0 as size_t;
    while buf_size > 0 as size_t && {
        buf_size = buf_size.wrapping_sub(1);
        *buf.offset(buf_size as isize) as c_int == 0 as c_int
    } {
        padding = padding.wrapping_add(1);
    }
    return padding;
}
unsafe extern "C" fn hide_format_error(mut ret: lzma_ret) -> lzma_ret {
    if ret == LZMA_FORMAT_ERROR {
        ret = LZMA_DATA_ERROR;
    }
    return ret;
}
unsafe extern "C" fn decode_index(
    mut coder: *mut lzma_file_info_coder,
    mut allocator: *const lzma_allocator,
    mut in_0: *const u8,
    mut in_pos: *mut size_t,
    mut in_size: size_t,
    mut update_file_cur_pos: bool,
) -> lzma_ret {
    let in_start: size_t = *in_pos;
    let ret: lzma_ret = (*coder)
        .index_decoder
        .code
        .expect("non-null function pointer")(
        (*coder).index_decoder.coder,
        allocator,
        in_0,
        in_pos,
        in_size,
        ::core::ptr::null_mut::<u8>(),
        ::core::ptr::null_mut::<size_t>(),
        0 as size_t,
        LZMA_RUN,
    ) as lzma_ret;
    (*coder).index_remaining = (*coder)
        .index_remaining
        .wrapping_sub((*in_pos).wrapping_sub(in_start) as lzma_vli);
    if update_file_cur_pos {
        (*coder).file_cur_pos = (*coder)
            .file_cur_pos
            .wrapping_add((*in_pos).wrapping_sub(in_start) as u64);
    }
    return ret;
}
unsafe extern "C" fn file_info_decode(
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
    let mut coder: *mut lzma_file_info_coder = coder_ptr as *mut lzma_file_info_coder;
    let in_start: size_t = *in_pos;
    if (*coder).file_size.wrapping_sub((*coder).file_cur_pos)
        < in_size.wrapping_sub(in_start) as u64
    {
        in_size =
            in_start.wrapping_add((*coder).file_size.wrapping_sub((*coder).file_cur_pos) as size_t);
    }
    loop {
        let mut current_block_142: u64;
        match (*coder).sequence {
            0 => {
                if (*coder).file_size < LZMA_STREAM_HEADER_SIZE as u64 {
                    return LZMA_FORMAT_ERROR;
                }
                if fill_temp(coder, in_0, in_pos, in_size) {
                    return LZMA_OK;
                }
                let ret_: lzma_ret = lzma_stream_header_decode(
                    &raw mut (*coder).first_header_flags,
                    &raw mut (*coder).temp as *mut u8,
                ) as lzma_ret;
                if ret_ != LZMA_OK {
                    return ret_;
                }
                if (*coder).file_size > LZMA_VLI_MAX as u64 || (*coder).file_size & 3 as u64 != 0 {
                    return LZMA_DATA_ERROR;
                }
                (*coder).file_target_pos = (*coder).file_size;
                current_block_142 = 10445208204442080639;
            }
            1 => {
                current_block_142 = 10445208204442080639;
            }
            2 => {
                current_block_142 = 13242334135786603907;
            }
            3 => {
                current_block_142 = 9626344630975045425;
            }
            4 => {
                current_block_142 = 9376024032952078885;
            }
            5 => {
                current_block_142 = 16203797167131938757;
            }
            6 => {
                current_block_142 = 1317013834825322123;
            }
            7 => {
                current_block_142 = 6010056518000876263;
            }
            _ => return LZMA_PROG_ERROR,
        }
        match current_block_142 {
            10445208204442080639 => {
                (*coder).sequence = SEQ_PADDING_DECODE;
                let ret__0: lzma_ret = reverse_seek(coder, in_start, in_pos, in_size) as lzma_ret;
                if ret__0 != LZMA_OK {
                    return ret__0;
                }
                current_block_142 = 13242334135786603907;
            }
            _ => {}
        }
        match current_block_142 {
            13242334135786603907 => {
                if fill_temp(coder, in_0, in_pos, in_size) {
                    return LZMA_OK;
                }
                let new_padding: size_t =
                    get_padding_size(&raw mut (*coder).temp as *mut u8, (*coder).temp_size)
                        as size_t;
                (*coder).stream_padding = (*coder)
                    .stream_padding
                    .wrapping_add(new_padding as lzma_vli);
                (*coder).file_target_pos =
                    (*coder).file_target_pos.wrapping_sub(new_padding as u64);
                if new_padding == (*coder).temp_size {
                    (*coder).sequence = SEQ_PADDING_SEEK;
                    current_block_142 = 13014351284863956202;
                } else {
                    if (*coder).stream_padding & 3 as lzma_vli != 0 {
                        return LZMA_DATA_ERROR;
                    }
                    (*coder).sequence = SEQ_FOOTER;
                    (*coder).temp_size = (*coder).temp_size.wrapping_sub(new_padding);
                    (*coder).temp_pos = (*coder).temp_size;
                    if (*coder).temp_size < LZMA_STREAM_HEADER_SIZE as size_t {
                        let ret__1: lzma_ret =
                            reverse_seek(coder, in_start, in_pos, in_size) as lzma_ret;
                        if ret__1 != LZMA_OK {
                            return ret__1;
                        }
                    }
                    current_block_142 = 9626344630975045425;
                }
            }
            _ => {}
        }
        match current_block_142 {
            9626344630975045425 => {
                if fill_temp(coder, in_0, in_pos, in_size) {
                    return LZMA_OK;
                }
                (*coder).file_target_pos = (*coder)
                    .file_target_pos
                    .wrapping_sub(LZMA_STREAM_HEADER_SIZE as u64);
                (*coder).temp_size = (*coder)
                    .temp_size
                    .wrapping_sub(LZMA_STREAM_HEADER_SIZE as size_t);
                let ret__2: lzma_ret = hide_format_error(lzma_stream_footer_decode(
                    &raw mut (*coder).footer_flags,
                    (&raw mut (*coder).temp as *mut u8).offset((*coder).temp_size as isize),
                )) as lzma_ret;
                if ret__2 != LZMA_OK {
                    return ret__2;
                }
                if (*coder).file_target_pos
                    < (*coder)
                        .footer_flags
                        .backward_size
                        .wrapping_add(LZMA_STREAM_HEADER_SIZE as lzma_vli)
                {
                    return LZMA_DATA_ERROR;
                }
                (*coder).file_target_pos = (*coder)
                    .file_target_pos
                    .wrapping_sub((*coder).footer_flags.backward_size as u64);
                (*coder).sequence = SEQ_INDEX_INIT;
                if (*coder).temp_size as lzma_vli >= (*coder).footer_flags.backward_size {
                    (*coder).temp_pos = ((*coder).temp_size as lzma_vli)
                        .wrapping_sub((*coder).footer_flags.backward_size)
                        as size_t;
                } else {
                    (*coder).temp_pos = 0 as size_t;
                    (*coder).temp_size = 0 as size_t;
                    if seek_to_pos(coder, (*coder).file_target_pos, in_start, in_pos, in_size) {
                        return LZMA_SEEK_NEEDED;
                    }
                }
                current_block_142 = 9376024032952078885;
            }
            _ => {}
        }
        match current_block_142 {
            9376024032952078885 => {
                let mut memused: u64 = 0 as u64;
                if !(*coder).combined_index.is_null() {
                    memused = lzma_index_memused((*coder).combined_index);
                    if memused > (*coder).memlimit {
                        return LZMA_PROG_ERROR;
                    }
                }
                let ret__3: lzma_ret = lzma_index_decoder_init(
                    &raw mut (*coder).index_decoder,
                    allocator,
                    &raw mut (*coder).this_index,
                    (*coder).memlimit.wrapping_sub(memused),
                ) as lzma_ret;
                if ret__3 != LZMA_OK {
                    return ret__3;
                }
                (*coder).index_remaining = (*coder).footer_flags.backward_size;
                (*coder).sequence = SEQ_INDEX_DECODE;
                current_block_142 = 16203797167131938757;
            }
            _ => {}
        }
        match current_block_142 {
            16203797167131938757 => {
                let mut ret: lzma_ret = LZMA_OK;
                if (*coder).temp_size != 0 as size_t {
                    ret = decode_index(
                        coder,
                        allocator,
                        &raw mut (*coder).temp as *mut u8,
                        &raw mut (*coder).temp_pos,
                        (*coder).temp_size,
                        false_0 != 0,
                    );
                } else {
                    let mut in_stop: size_t = in_size;
                    if in_size.wrapping_sub(*in_pos) as lzma_vli > (*coder).index_remaining {
                        in_stop = (*in_pos).wrapping_add((*coder).index_remaining as size_t);
                    }
                    ret = decode_index(coder, allocator, in_0, in_pos, in_stop, true_0 != 0);
                }
                match ret {
                    0 => {
                        if (*coder).index_remaining == 0 as lzma_vli {
                            return LZMA_DATA_ERROR;
                        }
                        return LZMA_OK;
                    }
                    1 => {
                        if (*coder).index_remaining != 0 as lzma_vli {
                            return LZMA_DATA_ERROR;
                        }
                    }
                    _ => return ret,
                }
                let seek_amount: u64 = (lzma_index_total_size((*coder).this_index) as u64)
                    .wrapping_add(LZMA_STREAM_HEADER_SIZE as u64);
                if (*coder).file_target_pos < seek_amount {
                    return LZMA_DATA_ERROR;
                }
                (*coder).file_target_pos = (*coder).file_target_pos.wrapping_sub(seek_amount);
                if (*coder).file_target_pos == 0 as u64 {
                    (*coder).header_flags = (*coder).first_header_flags;
                    (*coder).sequence = SEQ_HEADER_COMPARE;
                    current_block_142 = 13014351284863956202;
                } else {
                    (*coder).sequence = SEQ_HEADER_DECODE;
                    (*coder).file_target_pos = (*coder)
                        .file_target_pos
                        .wrapping_add(LZMA_STREAM_HEADER_SIZE as u64);
                    if (*coder).temp_size != 0 as size_t
                        && ((*coder).temp_size as lzma_vli)
                            .wrapping_sub((*coder).footer_flags.backward_size)
                            >= seek_amount
                    {
                        (*coder).temp_pos = ((*coder).temp_size as lzma_vli)
                            .wrapping_sub((*coder).footer_flags.backward_size)
                            .wrapping_sub(seek_amount as lzma_vli)
                            .wrapping_add(LZMA_STREAM_HEADER_SIZE as lzma_vli)
                            as size_t;
                        (*coder).temp_size = (*coder).temp_pos;
                    } else {
                        let ret__4: lzma_ret =
                            reverse_seek(coder, in_start, in_pos, in_size) as lzma_ret;
                        if ret__4 != LZMA_OK {
                            return ret__4;
                        }
                    }
                    current_block_142 = 1317013834825322123;
                }
            }
            _ => {}
        }
        match current_block_142 {
            1317013834825322123 => {
                if fill_temp(coder, in_0, in_pos, in_size) {
                    return LZMA_OK;
                }
                (*coder).file_target_pos = (*coder)
                    .file_target_pos
                    .wrapping_sub(LZMA_STREAM_HEADER_SIZE as u64);
                (*coder).temp_size = (*coder)
                    .temp_size
                    .wrapping_sub(LZMA_STREAM_HEADER_SIZE as size_t);
                (*coder).temp_pos = (*coder).temp_size;
                let ret__5: lzma_ret = hide_format_error(lzma_stream_header_decode(
                    &raw mut (*coder).header_flags,
                    (&raw mut (*coder).temp as *mut u8).offset((*coder).temp_size as isize),
                )) as lzma_ret;
                if ret__5 != LZMA_OK {
                    return ret__5;
                }
                (*coder).sequence = SEQ_HEADER_COMPARE;
                current_block_142 = 6010056518000876263;
            }
            _ => {}
        }
        match current_block_142 {
            6010056518000876263 => {
                let ret__6: lzma_ret = lzma_stream_flags_compare(
                    &raw mut (*coder).header_flags,
                    &raw mut (*coder).footer_flags,
                ) as lzma_ret;
                if ret__6 != LZMA_OK {
                    return ret__6;
                }
                if lzma_index_stream_flags((*coder).this_index, &raw mut (*coder).footer_flags)
                    != LZMA_OK
                {
                    return LZMA_PROG_ERROR;
                }
                if lzma_index_stream_padding((*coder).this_index, (*coder).stream_padding)
                    != LZMA_OK
                {
                    return LZMA_PROG_ERROR;
                }
                (*coder).stream_padding = 0 as lzma_vli;
                if !(*coder).combined_index.is_null() {
                    let ret__7: lzma_ret =
                        lzma_index_cat((*coder).this_index, (*coder).combined_index, allocator)
                            as lzma_ret;
                    if ret__7 != LZMA_OK {
                        return ret__7;
                    }
                }
                (*coder).combined_index = (*coder).this_index;
                (*coder).this_index = ::core::ptr::null_mut::<lzma_index>();
                if (*coder).file_target_pos == 0 as u64 {
                    *(*coder).dest_index = (*coder).combined_index;
                    (*coder).combined_index = ::core::ptr::null_mut::<lzma_index>();
                    *in_pos = in_size;
                    return LZMA_STREAM_END;
                }
                (*coder).sequence = (if (*coder).temp_size > 0 as size_t {
                    SEQ_PADDING_DECODE as c_int
                } else {
                    SEQ_PADDING_SEEK as c_int
                }) as C2RustUnnamed_0;
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn file_info_decoder_memconfig(
    mut coder_ptr: *mut c_void,
    mut memusage: *mut u64,
    mut old_memlimit: *mut u64,
    mut new_memlimit: u64,
) -> lzma_ret {
    let mut coder: *mut lzma_file_info_coder = coder_ptr as *mut lzma_file_info_coder;
    let mut combined_index_memusage: u64 = 0 as u64;
    let mut this_index_memusage: u64 = 0 as u64;
    if !(*coder).combined_index.is_null() {
        combined_index_memusage = lzma_index_memused((*coder).combined_index);
    }
    if !(*coder).this_index.is_null() {
        this_index_memusage = lzma_index_memused((*coder).this_index);
    } else if (*coder).sequence == SEQ_INDEX_DECODE {
        let mut dummy: u64 = 0;
        if (*coder)
            .index_decoder
            .memconfig
            .expect("non-null function pointer")(
            (*coder).index_decoder.coder,
            &raw mut this_index_memusage,
            &raw mut dummy,
            0 as u64,
        ) != LZMA_OK
        {
            return LZMA_PROG_ERROR;
        }
    }
    *memusage = combined_index_memusage.wrapping_add(this_index_memusage);
    if *memusage == 0 as u64 {
        *memusage = lzma_index_memusage(1 as lzma_vli, 0 as lzma_vli);
    }
    *old_memlimit = (*coder).memlimit;
    if new_memlimit != 0 as u64 {
        if new_memlimit < *memusage {
            return LZMA_MEMLIMIT_ERROR;
        }
        if (*coder).this_index.is_null() && (*coder).sequence == SEQ_INDEX_DECODE {
            let idec_new_memlimit: u64 = new_memlimit.wrapping_sub(combined_index_memusage);
            let mut dummy1: u64 = 0;
            let mut dummy2: u64 = 0;
            if (*coder)
                .index_decoder
                .memconfig
                .expect("non-null function pointer")(
                (*coder).index_decoder.coder,
                &raw mut dummy1,
                &raw mut dummy2,
                idec_new_memlimit,
            ) != LZMA_OK
            {
                return LZMA_PROG_ERROR;
            }
        }
        (*coder).memlimit = new_memlimit;
    }
    return LZMA_OK;
}
unsafe extern "C" fn file_info_decoder_end(
    mut coder_ptr: *mut c_void,
    mut allocator: *const lzma_allocator,
) {
    let mut coder: *mut lzma_file_info_coder = coder_ptr as *mut lzma_file_info_coder;
    lzma_next_end(&raw mut (*coder).index_decoder, allocator);
    lzma_index_end((*coder).this_index, allocator);
    lzma_index_end((*coder).combined_index, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
unsafe extern "C" fn lzma_file_info_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut seek_pos: *mut u64,
    mut dest_index: *mut *mut lzma_index,
    mut memlimit: u64,
    mut file_size: u64,
) -> lzma_ret {
    if ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut u64,
                *mut *mut lzma_index,
                u64,
                u64,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_file_info_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut u64,
                *mut *mut lzma_index,
                u64,
                u64,
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
                *mut u64,
                *mut *mut lzma_index,
                u64,
                u64,
            ) -> lzma_ret,
        >,
        uintptr_t,
    >(Some(
        lzma_file_info_decoder_init
            as unsafe extern "C" fn(
                *mut lzma_next_coder,
                *const lzma_allocator,
                *mut u64,
                *mut *mut lzma_index,
                u64,
                u64,
            ) -> lzma_ret,
    ));
    if dest_index.is_null() {
        return LZMA_PROG_ERROR;
    }
    let mut coder: *mut lzma_file_info_coder = (*next).coder as *mut lzma_file_info_coder;
    if coder.is_null() {
        coder = lzma_alloc(
            ::core::mem::size_of::<lzma_file_info_coder>() as size_t,
            allocator,
        ) as *mut lzma_file_info_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).code = Some(
            file_info_decode
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
            file_info_decoder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
        ) as lzma_end_function;
        (*next).memconfig = Some(
            file_info_decoder_memconfig
                as unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret,
        )
            as Option<unsafe extern "C" fn(*mut c_void, *mut u64, *mut u64, u64) -> lzma_ret>;
        (*coder).index_decoder = lzma_next_coder_s {
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
        (*coder).this_index = ::core::ptr::null_mut::<lzma_index>();
        (*coder).combined_index = ::core::ptr::null_mut::<lzma_index>();
    }
    (*coder).sequence = SEQ_MAGIC_BYTES;
    (*coder).file_cur_pos = 0 as u64;
    (*coder).file_target_pos = 0 as u64;
    (*coder).file_size = file_size;
    lzma_index_end((*coder).this_index, allocator);
    (*coder).this_index = ::core::ptr::null_mut::<lzma_index>();
    lzma_index_end((*coder).combined_index, allocator);
    (*coder).combined_index = ::core::ptr::null_mut::<lzma_index>();
    (*coder).stream_padding = 0 as lzma_vli;
    (*coder).dest_index = dest_index;
    (*coder).external_seek_pos = seek_pos;
    (*coder).memlimit = if 1 as u64 > memlimit {
        1 as u64
    } else {
        memlimit
    };
    (*coder).temp_pos = 0 as size_t;
    (*coder).temp_size = LZMA_STREAM_HEADER_SIZE as size_t;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_file_info_decoder(
    mut strm: *mut lzma_stream,
    mut dest_index: *mut *mut lzma_index,
    mut memlimit: u64,
    mut file_size: u64,
) -> lzma_ret {
    let ret_: lzma_ret = lzma_strm_init(strm) as lzma_ret;
    if ret_ != LZMA_OK {
        return ret_;
    }
    let ret__0: lzma_ret = lzma_file_info_decoder_init(
        &raw mut (*(*strm).internal).next,
        (*strm).allocator,
        &raw mut (*strm).seek_pos,
        dest_index,
        memlimit,
        file_size,
    ) as lzma_ret;
    if ret__0 != LZMA_OK {
        lzma_end(strm);
        return ret__0;
    }
    (*(*strm).internal).supported_actions[LZMA_RUN as usize] = true_0 != 0;
    (*(*strm).internal).supported_actions[LZMA_FINISH as usize] = true_0 != 0;
    return LZMA_OK;
}
