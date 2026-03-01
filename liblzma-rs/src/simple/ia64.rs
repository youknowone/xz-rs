use crate::types::*;
use core::ffi::c_void;
unsafe extern "C" fn ia64_code(
    _simple: *mut c_void,
    now_pos: u32,
    is_encoder: bool,
    buffer: *mut u8,
    mut size: size_t,
) -> size_t {
    static mut BRANCH_TABLE: [u32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 6, 6, 0, 0, 7, 7, 4, 4, 0, 0, 4, 4,
        0, 0,
    ];
    size &= !(15);
    let mut i: size_t = 0;
    i = 0;
    while i < size {
        let instr_template: u32 = (*buffer.offset(i as isize) & 0x1f) as u32;
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
                    instruction = instruction.wrapping_add(
                        (*buffer.offset(i.wrapping_add(j).wrapping_add(byte_pos) as isize) as u64)
                            << (8_usize).wrapping_mul(j),
                    );
                    j += 1;
                }
                let mut inst_norm: u64 = instruction >> bit_res;
                if inst_norm >> 37 & 0xf as u64 == 0x5 as u64 && inst_norm >> 9 & 0x7 as u64 == 0 {
                    let mut src: u32 = (inst_norm >> 13 & 0xfffff as u64) as u32;
                    src = (src as u64 | (inst_norm >> 36 & 1) << 20) as u32;
                    src <<= 4;
                    let mut dest: u32 = 0;
                    if is_encoder {
                        dest = now_pos.wrapping_add(i as u32).wrapping_add(src);
                    } else {
                        dest = src.wrapping_sub(now_pos.wrapping_add(i as u32));
                    }
                    dest >>= 4;
                    inst_norm &= !((0x8fffff as u64) << 13);
                    inst_norm |= ((dest & 0xfffff) as u64) << 13;
                    inst_norm |= ((dest & 0x100000) as u64) << 36 - 20;
                    instruction &= (1u32 << bit_res).wrapping_sub(1) as u64;
                    instruction |= inst_norm << bit_res;
                    let mut j_0: size_t = 0;
                    while j_0 < 6 {
                        *buffer.offset(i.wrapping_add(j_0).wrapping_add(byte_pos) as isize) =
                            (instruction >> (8_usize).wrapping_mul(j_0)) as u8;
                        j_0 += 1;
                    }
                }
            }
            slot += 1;
            bit_pos = bit_pos.wrapping_add(41);
        }
        i = i.wrapping_add(16);
    }
    i
}
extern "C" fn ia64_coder_init(
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
                ia64_code
                    as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t,
            ),
            0,
            16,
            16,
            is_encoder,
        )
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_ia64_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    ia64_coder_init(next, allocator, filters, true)
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_ia64_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    ia64_coder_init(next, allocator, filters, false)
}
