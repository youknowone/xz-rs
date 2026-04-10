use crate::types::*;
fn ia64_code_impl(now_pos: u32, is_encoder: bool, buffer: &mut [u8]) -> size_t {
    const BRANCH_TABLE: [u32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 6, 6, 0, 0, 7, 7, 4, 4, 0, 0, 4, 4,
        0, 0,
    ];
    let size = buffer.len() & !15;
    let mut i: size_t = 0;
    while i < size {
        let instr_template: u32 = (buffer[i] & 0x1f) as u32;
        let mask: u32 = BRANCH_TABLE[instr_template as usize];
        let mut bit_pos: u32 = 5;
        let mut slot: size_t = 0;
        while slot < 3 {
            if mask >> slot & 1 != 0 {
                let byte_pos: size_t = (bit_pos >> 3) as size_t;
                let bit_res: u32 = bit_pos & 0x7;
                let mut instruction: u64 = 0;
                let mut j: size_t = 0;
                while j < 6 {
                    instruction += (buffer[i + j + byte_pos] as u64) << (8 * j);
                    j += 1;
                }
                let mut inst_norm: u64 = instruction >> bit_res;
                if inst_norm >> 37 & 0xf == 0x5 && inst_norm >> 9 & 0x7 == 0 {
                    let mut src: u32 = (inst_norm >> 13 & 0xfffff) as u32;
                    src = (src as u64 | (inst_norm >> 36 & 1) << 20) as u32;
                    src <<= 4;
                    let dest = if is_encoder {
                        now_pos.wrapping_add(i as u32).wrapping_add(src)
                    } else {
                        src.wrapping_sub(now_pos.wrapping_add(i as u32))
                    };
                    let dest = dest >> 4;
                    inst_norm &= !(0x8fffff_u64 << 13);
                    inst_norm |= ((dest & 0xfffff) as u64) << 13;
                    inst_norm |= ((dest & 0x100000) as u64) << 16;
                    instruction &= ((1u32 << bit_res) - 1) as u64;
                    instruction |= inst_norm << bit_res;
                    let mut j: size_t = 0;
                    while j < 6 {
                        buffer[i + j + byte_pos] = (instruction >> (8 * j)) as u8;
                        j += 1;
                    }
                }
            }
            slot += 1;
            bit_pos += 41;
        }
        i += 16;
    }
    i
}
unsafe extern "C" fn ia64_code(
    _simple: *mut c_void,
    now_pos: u32,
    is_encoder: bool,
    buffer: *mut u8,
    size: size_t,
) -> size_t {
    if size == 0 {
        return 0;
    }
    ia64_code_impl(
        now_pos,
        is_encoder,
        core::slice::from_raw_parts_mut(buffer, size),
    )
}
unsafe fn ia64_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
    is_encoder: bool,
) -> lzma_ret {
    lzma_simple_coder_init(
        next,
        allocator,
        filters,
        Some(ia64_code as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t),
        0,
        16,
        16,
        is_encoder,
    )
}
pub(crate) unsafe extern "C" fn lzma_simple_ia64_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    ia64_coder_init(next, allocator, filters, true)
}
pub(crate) unsafe extern "C" fn lzma_simple_ia64_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    ia64_coder_init(next, allocator, filters, false)
}
