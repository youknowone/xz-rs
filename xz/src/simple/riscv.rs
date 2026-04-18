use crate::types::*;
#[inline]
fn read32be(buf: &[u8; 4]) -> u32 {
    let mut num: u32 = (buf[0] as u32) << 24;
    num |= (buf[1] as u32) << 16;
    num |= (buf[2] as u32) << 8;
    num |= buf[3] as u32;
    num
}
#[inline]
fn write32be(buf: &mut [u8; 4], num: u32) {
    buf[0] = (num >> 24) as u8;
    buf[1] = (num >> 16) as u8;
    buf[2] = (num >> 8) as u8;
    buf[3] = num as u8;
}
#[inline]
unsafe fn read32le_at(buffer: &[u8], offset: usize) -> u32 {
    read32le(&*buffer.as_ptr().add(offset).cast::<[u8; 4]>())
}
#[inline]
unsafe fn read32be_at(buffer: &[u8], offset: usize) -> u32 {
    read32be(&*buffer.as_ptr().add(offset).cast::<[u8; 4]>())
}
#[inline]
unsafe fn write32le_at(buffer: &mut [u8], offset: usize, value: u32) {
    write32le(
        &mut *buffer.as_mut_ptr().add(offset).cast::<[u8; 4]>(),
        value,
    );
}
#[inline]
unsafe fn write32be_at(buffer: &mut [u8], offset: usize, value: u32) {
    write32be(
        &mut *buffer.as_mut_ptr().add(offset).cast::<[u8; 4]>(),
        value,
    );
}
fn riscv_encode_impl(now_pos: u32, buffer: &mut [u8]) -> size_t {
    if buffer.len() < 8 {
        return 0;
    }
    let size = buffer.len() - 8;
    let ptr = buffer.as_mut_ptr();
    let mut i: size_t = 0;
    while i <= size {
        let mut inst: u32 = unsafe { *ptr.add(i) as u32 };
        if inst == 0xef {
            let b1: u32 = unsafe { *ptr.add(i + 1) as u32 };
            if b1 & 0xd == 0 {
                let b2: u32 = unsafe { *ptr.add(i + 2) as u32 };
                let b3: u32 = unsafe { *ptr.add(i + 3) as u32 };
                let pc: u32 = now_pos.wrapping_add(i as u32);
                let mut addr: u32 = (b1 & 0xf0) << 8
                    | (b2 & 0xf) << 16
                    | (b2 & 0x10) << 7
                    | (b2 & 0xe0) >> 4
                    | (b3 & 0x7f) << 4
                    | (b3 & 0x80) << 13;
                addr = addr.wrapping_add(pc);
                unsafe {
                    *ptr.add(i + 1) = (b1 & 0xf | addr >> 13 & 0xf0) as u8;
                    *ptr.add(i + 2) = (addr >> 9) as u8;
                    *ptr.add(i + 3) = (addr >> 1) as u8;
                }
                i += 4 - 2;
            }
        } else if inst & 0x7f == 0x17 {
            inst |= unsafe { (*ptr.add(i + 1) as u32) << 8 };
            inst |= unsafe { (*ptr.add(i + 2) as u32) << 16 };
            inst |= unsafe { (*ptr.add(i + 3) as u32) << 24 };
            if inst & 0xe80 != 0 {
                let inst2: u32 = unsafe { read32le_at(buffer, i + 4) };
                if (inst << 8 ^ inst2.wrapping_sub(3)) & 0xf8003 != 0 {
                    i += 6 - 2;
                } else {
                    let mut addr_0: u32 = inst & 0xfffff000;
                    addr_0 = addr_0.wrapping_add((inst2 >> 20).wrapping_sub(inst2 >> 19 & 0x1000));
                    addr_0 = addr_0.wrapping_add(now_pos.wrapping_add(i as u32));
                    inst = (0x17 | (2) << 7) as u32 | inst2 << 12;
                    unsafe {
                        write32le_at(buffer, i, inst);
                        write32be_at(buffer, i + 4, addr_0);
                    }
                    i += 8 - 2;
                }
            } else {
                let fake_rs1: u32 = inst >> 27;
                if inst.wrapping_sub(0x3117) << 18 >= fake_rs1 & 0x1d {
                    i += 4 - 2;
                } else {
                    let fake_addr: u32 = unsafe { read32le_at(buffer, i + 4) };
                    let fake_inst2: u32 = inst >> 12 | fake_addr << 20;
                    inst = 0x17 | fake_rs1 << 7 | fake_addr & 0xfffff000;
                    unsafe {
                        write32le_at(buffer, i, inst);
                        write32le_at(buffer, i + 4, fake_inst2);
                    }
                    i += 8 - 2;
                }
            }
        }
        i += 2;
    }
    i
}
unsafe fn riscv_encode(
    _simple: *mut c_void,
    now_pos: u32,
    _is_encoder: bool,
    buffer: *mut u8,
    size: size_t,
) -> size_t {
    if size == 0 {
        return 0;
    }
    riscv_encode_impl(now_pos, core::slice::from_raw_parts_mut(buffer, size))
}
pub(crate) unsafe fn lzma_simple_riscv_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    lzma_simple_coder_init(
        next,
        allocator,
        filters,
        riscv_encode as lzma_simple_filter_function,
        0,
        8,
        2,
        true,
    )
}
pub unsafe fn lzma_bcj_riscv_encode(mut start_offset: u32, buf: *mut u8, size: size_t) -> size_t {
    start_offset = (start_offset & !1u32) as u32;
    if size == 0 {
        return 0;
    }
    riscv_encode_impl(start_offset, core::slice::from_raw_parts_mut(buf, size))
}
fn riscv_decode_impl(now_pos: u32, buffer: &mut [u8]) -> size_t {
    if buffer.len() < 8 {
        return 0;
    }
    let size = buffer.len() - 8;
    let ptr = buffer.as_mut_ptr();
    let mut i: size_t = 0;
    while i <= size {
        let mut inst: u32 = unsafe { *ptr.add(i) as u32 };
        if inst == 0xef {
            let b1: u32 = unsafe { *ptr.add(i + 1) as u32 };
            if b1 & 0xd == 0 {
                let b2: u32 = unsafe { *ptr.add(i + 2) as u32 };
                let b3: u32 = unsafe { *ptr.add(i + 3) as u32 };
                let pc: u32 = now_pos.wrapping_add(i as u32);
                let mut addr: u32 = (b1 & 0xf0) << 13 | b2 << 9 | b3 << 1;
                addr = addr.wrapping_sub(pc);
                unsafe {
                    *ptr.add(i + 1) = (b1 & 0xf | addr >> 8 & 0xf0) as u8;
                    *ptr.add(i + 2) =
                        (addr >> 16 & 0xf | addr >> 7 & 0x10 | addr << 4 & 0xe0) as u8;
                    *ptr.add(i + 3) = (addr >> 4 & 0x7f | addr >> 13 & 0x80) as u8;
                }
                i += 4 - 2;
            }
        } else if inst & 0x7f == 0x17 {
            let mut inst2: u32 = 0;
            inst |= unsafe { (*ptr.add(i + 1) as u32) << 8 };
            inst |= unsafe { (*ptr.add(i + 2) as u32) << 16 };
            inst |= unsafe { (*ptr.add(i + 3) as u32) << 24 };
            if inst & 0xe80 != 0 {
                inst2 = unsafe { read32le_at(buffer, i + 4) };
                if (inst << 8 ^ inst2.wrapping_sub(3)) & 0xf8003 != 0 {
                    i += 6 - 2;
                } else {
                    let mut addr_0: u32 = inst & 0xfffff000;
                    addr_0 = addr_0.wrapping_add(inst2 >> 20);
                    inst = (0x17 | (2) << 7) as u32 | inst2 << 12;
                    inst2 = addr_0;
                    unsafe {
                        write32le_at(buffer, i, inst);
                        write32le_at(buffer, i + 4, inst2);
                    }
                    i += 8 - 2;
                }
            } else {
                let inst2_rs1: u32 = inst >> 27;
                if inst.wrapping_sub(0x3117) << 18 >= inst2_rs1 & 0x1d {
                    i += 4 - 2;
                } else {
                    let mut addr_1: u32 = unsafe { read32be_at(buffer, i + 4) };
                    addr_1 = addr_1.wrapping_sub(now_pos.wrapping_add(i as u32));
                    inst2 = inst >> 12 | addr_1 << 20;
                    inst = 0x17 | inst2_rs1 << 7 | addr_1.wrapping_add(0x800) & 0xfffff000;
                    unsafe {
                        write32le_at(buffer, i, inst);
                        write32le_at(buffer, i + 4, inst2);
                    }
                    i += 8 - 2;
                }
            }
        }
        i += 2;
    }
    i
}
unsafe fn riscv_decode(
    _simple: *mut c_void,
    now_pos: u32,
    _is_encoder: bool,
    buffer: *mut u8,
    size: size_t,
) -> size_t {
    if size == 0 {
        return 0;
    }
    riscv_decode_impl(now_pos, core::slice::from_raw_parts_mut(buffer, size))
}
pub(crate) unsafe fn lzma_simple_riscv_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    lzma_simple_coder_init(
        next,
        allocator,
        filters,
        riscv_decode as lzma_simple_filter_function,
        0,
        8,
        2,
        false,
    )
}
pub unsafe fn lzma_bcj_riscv_decode(mut start_offset: u32, buf: *mut u8, size: size_t) -> size_t {
    start_offset = (start_offset & !1u32) as u32;
    if size == 0 {
        return 0;
    }
    riscv_decode_impl(start_offset, core::slice::from_raw_parts_mut(buf, size))
}
