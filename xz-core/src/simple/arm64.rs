use crate::types::*;
fn arm64_code_impl(now_pos: u32, is_encoder: bool, buffer: &mut [u8]) -> size_t {
    let size = buffer.len() & !3;
    let mut i: size_t = 0;
    while i < size {
        let mut pc: u32 = now_pos.wrapping_add(i as u32);
        let word: &mut [u8; 4] = unsafe { &mut *buffer.as_mut_ptr().add(i).cast::<[u8; 4]>() };
        let mut instr: u32 = read32le(word);
        if instr >> 26 == 0x25 {
            let src: u32 = instr;
            instr = 0x94000000;
            pc >>= 2;
            if !is_encoder {
                pc = 0u32.wrapping_sub(pc);
            }
            instr |= src.wrapping_add(pc) & 0x3ffffff;
            write32le(word, instr);
        } else if instr & 0x9f000000 == 0x90000000 {
            let src: u32 = instr >> 29 & 3 | instr >> 3 & 0x1ffffc;
            if src.wrapping_add(0x20000) & 0x1c0000 == 0 {
                instr &= 0x9000001f;
                pc >>= 12;
                if !is_encoder {
                    pc = 0u32.wrapping_sub(pc);
                }
                let dest: u32 = src.wrapping_add(pc);
                instr |= (dest & 3) << 29;
                instr |= (dest & 0x3fffc) << 3;
                instr |= 0u32.wrapping_sub(dest & 0x20000) & 0xe00000;
                write32le(word, instr);
            }
        }
        i += 4;
    }
    i
}
unsafe fn arm64_code(
    _simple: *mut c_void,
    now_pos: u32,
    is_encoder: bool,
    buffer: *mut u8,
    size: size_t,
) -> size_t {
    if size == 0 {
        return 0;
    }
    arm64_code_impl(
        now_pos,
        is_encoder,
        core::slice::from_raw_parts_mut(buffer, size),
    )
}
unsafe fn arm64_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    is_encoder: bool,
) -> lzma_ret {
    lzma_simple_coder_init(
        next,
        allocator,
        filters,
        arm64_code as lzma_simple_filter_function,
        0,
        4,
        4,
        is_encoder,
    )
}
pub(crate) unsafe fn lzma_simple_arm64_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    arm64_coder_init(next, allocator, filters, true)
}
pub unsafe fn lzma_bcj_arm64_encode(mut start_offset: u32, buf: *mut u8, size: size_t) -> size_t {
    start_offset = (start_offset & !3u32) as u32;
    if size == 0 {
        return 0;
    }
    arm64_code_impl(
        start_offset,
        true,
        core::slice::from_raw_parts_mut(buf, size),
    )
}
pub(crate) unsafe fn lzma_simple_arm64_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    arm64_coder_init(next, allocator, filters, false)
}
pub unsafe fn lzma_bcj_arm64_decode(mut start_offset: u32, buf: *mut u8, size: size_t) -> size_t {
    start_offset = (start_offset & !3u32) as u32;
    if size == 0 {
        return 0;
    }
    arm64_code_impl(
        start_offset,
        false,
        core::slice::from_raw_parts_mut(buf, size),
    )
}
