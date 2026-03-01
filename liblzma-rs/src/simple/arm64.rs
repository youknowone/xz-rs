use crate::types::*;
use core::ffi::c_void;
extern "C" {
    fn lzma_simple_coder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
        filter: Option<unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t>,
        simple_size: size_t,
        unfiltered_max: size_t,
        alignment: u32,
        is_encoder: bool,
    ) -> lzma_ret;
}
#[inline]
extern "C" fn read32le(buf: *const u8) -> u32 {
    return unsafe {
        let mut num: u32 = *buf as u32;
        num |= (*buf.offset(1) as u32) << 8;
        num |= (*buf.offset(2) as u32) << 16;
        num |= (*buf.offset(3) as u32) << 24;
        num
    };
}
#[inline]
extern "C" fn write32le(buf: *mut u8, num: u32) {
    unsafe {
        *buf = num as u8;
        *buf.offset(1) = (num >> 8) as u8;
        *buf.offset(2) = (num >> 16) as u8;
        *buf.offset(3) = (num >> 24) as u8;
    }
}
unsafe extern "C" fn arm64_code(
    _simple: *mut c_void,
    now_pos: u32,
    is_encoder: bool,
    buffer: *mut u8,
    mut size: size_t,
) -> size_t {
    size &= !(3);
    let mut i: size_t = 0;
    i = 0;
    while i < size {
        let mut pc: u32 = (now_pos as size_t).wrapping_add(i) as u32;
        let mut instr: u32 = read32le(buffer.offset(i as isize));
        if instr >> 26 == 0x25 {
            let src: u32 = instr;
            instr = 0x94000000;
            pc >>= 2;
            if !is_encoder {
                pc = 0u32.wrapping_sub(pc);
            }
            instr |= src.wrapping_add(pc) & 0x3ffffff;
            write32le(buffer.offset(i as isize), instr);
        } else if instr & 0x9f000000 == 0x90000000 {
            let src_0: u32 = instr >> 29 & 3 | instr >> 3 & 0x1ffffc;
            if src_0.wrapping_add(0x20000) & 0x1c0000 == 0 {
                instr = (instr & 0x9000001f) as u32;
                pc >>= 12;
                if !is_encoder {
                    pc = 0u32.wrapping_sub(pc);
                }
                let dest: u32 = src_0.wrapping_add(pc);
                instr |= (dest & 3) << 29;
                instr |= (dest & 0x3fffc) << 3;
                instr = (instr | (0u32.wrapping_sub(dest & 0x20000) & 0xe00000)) as u32;
                write32le(buffer.offset(i as isize), instr);
            }
        }
        i = i.wrapping_add(4);
    }
    i
}
extern "C" fn arm64_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    is_encoder: bool,
) -> lzma_ret {
    unsafe {
        lzma_simple_coder_init(
            next,
            allocator,
            filters,
            Some(
                arm64_code
                    as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t,
            ),
            0,
            4,
            4,
            is_encoder,
        )
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_arm64_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    arm64_coder_init(next, allocator, filters, true)
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_arm64_encode(
    mut start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    start_offset = (start_offset & !3u32) as u32;
    arm64_code(core::ptr::null_mut(), start_offset, true, buf, size)
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_arm64_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    arm64_coder_init(next, allocator, filters, false)
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_arm64_decode(
    mut start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    start_offset = (start_offset & !3u32) as u32;
    arm64_code(core::ptr::null_mut(), start_offset, false, buf, size)
}
