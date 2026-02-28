use crate::types::*;
use core::ffi::{c_int, c_void};
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
pub type lzma_filter_info = lzma_filter_info_s;
#[inline]
extern "C" fn read32be(buf: *const u8) -> u32 {
    return unsafe {
        let mut num: u32 = (*buf.offset(0) as u32) << 24;
        num |= (*buf.offset(1) as u32) << 16;
        num |= (*buf.offset(2) as u32) << 8;
        num |= *buf.offset(3) as u32;
        num
    };
}
#[inline]
extern "C" fn read32le(buf: *const u8) -> u32 {
    return unsafe {
        let mut num: u32 = *buf.offset(0) as u32;
        num |= (*buf.offset(1) as u32) << 8;
        num |= (*buf.offset(2) as u32) << 16;
        num |= (*buf.offset(3) as u32) << 24;
        num
    };
}
#[inline]
extern "C" fn write32be(buf: *mut u8, num: u32) {
    unsafe {
        *buf.offset(0) = (num >> 24) as u8;
        *buf.offset(1) = (num >> 16) as u8;
        *buf.offset(2) = (num >> 8) as u8;
        *buf.offset(3) = num as u8;
    }
}
#[inline]
extern "C" fn write32le(buf: *mut u8, num: u32) {
    unsafe {
        *buf.offset(0) = num as u8;
        *buf.offset(1) = (num >> 8) as u8;
        *buf.offset(2) = (num >> 16) as u8;
        *buf.offset(3) = (num >> 24) as u8;
    }
}
unsafe extern "C" fn riscv_encode(
    _simple: *mut c_void,
    now_pos: u32,
    _is_encoder: bool,
    buffer: *mut u8,
    mut size: size_t,
) -> size_t {
    if size < 8 {
        return 0;
    }
    size = size.wrapping_sub(8);
    let mut i: size_t = 0;
    let mut current_block_22: u64;
    i = 0;
    while i <= size {
        let mut inst: u32 = *buffer.offset(i as isize) as u32;
        if inst == 0xef as u32 {
            let b1: u32 = *buffer.offset(i.wrapping_add(1) as isize) as u32;
            if !(b1 & 0xd as u32 != 0) {
                let b2: u32 = *buffer.offset(i.wrapping_add(2) as isize) as u32;
                let b3: u32 = *buffer.offset(i.wrapping_add(3) as isize) as u32;
                let pc: u32 = now_pos.wrapping_add(i as u32);
                let mut addr: u32 = (b1 & 0xf0 as u32) << 8
                    | (b2 & 0xf as u32) << 16
                    | (b2 & 0x10 as u32) << 7
                    | (b2 & 0xe0 as u32) >> 4
                    | (b3 & 0x7f as u32) << 4
                    | (b3 & 0x80 as u32) << 13;
                addr = addr.wrapping_add(pc);
                *buffer.offset(i.wrapping_add(1) as isize) =
                    (b1 & 0xf as u32 | addr >> 13 & 0xf0 as u32) as u8;
                *buffer.offset(i.wrapping_add(2) as isize) = (addr >> 9) as u8;
                *buffer.offset(i.wrapping_add(3) as isize) = (addr >> 1) as u8;
                i = i.wrapping_add((4 as c_int - 2 as c_int) as size_t);
            }
        } else if inst & 0x7f as u32 == 0x17 as u32 {
            inst |= (*buffer.offset(i.wrapping_add(1) as isize) as u32) << 8;
            inst |= (*buffer.offset(i.wrapping_add(2) as isize) as u32) << 16;
            inst |= (*buffer.offset(i.wrapping_add(3) as isize) as u32) << 24;
            if inst & 0xe80 as u32 != 0 {
                let inst2: u32 = read32le(buffer.offset(i as isize).offset(4));
                if (inst << 8 ^ inst2.wrapping_sub(3)) & 0xf8003 as u32 != 0 {
                    i = i.wrapping_add((6 as c_int - 2 as c_int) as size_t);
                    current_block_22 = 12517898123489920830;
                } else {
                    let mut addr_0: u32 = inst & 0xfffff000 as u32;
                    addr_0 = addr_0
                        .wrapping_add((inst2 >> 20).wrapping_sub(inst2 >> 19 & 0x1000 as u32));
                    addr_0 = addr_0.wrapping_add(now_pos.wrapping_add(i as u32));
                    inst = (0x17 as c_int | (2 as c_int) << 7) as u32 | inst2 << 12;
                    write32le(buffer.offset(i as isize), inst);
                    write32be(buffer.offset(i as isize).offset(4), addr_0);
                    current_block_22 = 15125582407903384992;
                }
            } else {
                let fake_rs1: u32 = inst >> 27;
                if inst.wrapping_sub(0x3117 as u32) << 18 >= fake_rs1 & 0x1d as u32 {
                    i = i.wrapping_add((4 as c_int - 2 as c_int) as size_t);
                    current_block_22 = 12517898123489920830;
                } else {
                    let fake_addr: u32 = read32le(buffer.offset(i as isize).offset(4)) as u32;
                    let fake_inst2: u32 = inst >> 12 | fake_addr << 20;
                    inst = 0x17 as u32 | fake_rs1 << 7 | fake_addr & 0xfffff000 as u32;
                    write32le(buffer.offset(i as isize), inst);
                    write32le(buffer.offset(i as isize).offset(4), fake_inst2);
                    current_block_22 = 15125582407903384992;
                }
            }
            match current_block_22 {
                12517898123489920830 => {}
                _ => {
                    i = i.wrapping_add((8 as c_int - 2 as c_int) as size_t);
                }
            }
        }
        i = i.wrapping_add(2);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_riscv_encoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    return lzma_simple_coder_init(
        next,
        allocator,
        filters,
        Some(
            riscv_encode as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t,
        ),
        0,
        8,
        2,
        true,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_riscv_encode(
    mut start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    start_offset = (start_offset & !1u32) as u32;
    return riscv_encode(core::ptr::null_mut(), start_offset, true, buf, size);
}
unsafe extern "C" fn riscv_decode(
    _simple: *mut c_void,
    now_pos: u32,
    _is_encoder: bool,
    buffer: *mut u8,
    mut size: size_t,
) -> size_t {
    if size < 8 {
        return 0;
    }
    size = size.wrapping_sub(8);
    let mut i: size_t = 0;
    let mut current_block_23: u64;
    i = 0;
    while i <= size {
        let mut inst: u32 = *buffer.offset(i as isize) as u32;
        if inst == 0xef as u32 {
            let b1: u32 = *buffer.offset(i.wrapping_add(1) as isize) as u32;
            if !(b1 & 0xd as u32 != 0) {
                let b2: u32 = *buffer.offset(i.wrapping_add(2) as isize) as u32;
                let b3: u32 = *buffer.offset(i.wrapping_add(3) as isize) as u32;
                let pc: u32 = now_pos.wrapping_add(i as u32);
                let mut addr: u32 = (b1 & 0xf0 as u32) << 13 | b2 << 9 | b3 << 1;
                addr = addr.wrapping_sub(pc);
                *buffer.offset(i.wrapping_add(1) as isize) =
                    (b1 & 0xf as u32 | addr >> 8 & 0xf0 as u32) as u8;
                *buffer.offset(i.wrapping_add(2) as isize) =
                    (addr >> 16 & 0xf as u32 | addr >> 7 & 0x10 as u32 | addr << 4 & 0xe0 as u32)
                        as u8;
                *buffer.offset(i.wrapping_add(3) as isize) =
                    (addr >> 4 & 0x7f as u32 | addr >> 13 & 0x80 as u32) as u8;
                i = i.wrapping_add((4 as c_int - 2 as c_int) as size_t);
            }
        } else if inst & 0x7f as u32 == 0x17 as u32 {
            let mut inst2: u32 = 0;
            inst |= (*buffer.offset(i.wrapping_add(1) as isize) as u32) << 8;
            inst |= (*buffer.offset(i.wrapping_add(2) as isize) as u32) << 16;
            inst |= (*buffer.offset(i.wrapping_add(3) as isize) as u32) << 24;
            if inst & 0xe80 as u32 != 0 {
                inst2 = read32le(buffer.offset(i as isize).offset(4));
                if (inst << 8 ^ inst2.wrapping_sub(3)) & 0xf8003 as u32 != 0 {
                    i = i.wrapping_add((6 as c_int - 2 as c_int) as size_t);
                    current_block_23 = 12517898123489920830;
                } else {
                    let mut addr_0: u32 = inst & 0xfffff000 as u32;
                    addr_0 = addr_0.wrapping_add(inst2 >> 20);
                    inst = (0x17 as c_int | (2 as c_int) << 7) as u32 | inst2 << 12;
                    inst2 = addr_0;
                    current_block_23 = 6669252993407410313;
                }
            } else {
                let inst2_rs1: u32 = inst >> 27;
                if inst.wrapping_sub(0x3117 as u32) << 18 >= inst2_rs1 & 0x1d as u32 {
                    i = i.wrapping_add((4 as c_int - 2 as c_int) as size_t);
                    current_block_23 = 12517898123489920830;
                } else {
                    let mut addr_1: u32 = read32be(buffer.offset(i as isize).offset(4));
                    addr_1 = addr_1.wrapping_sub(now_pos.wrapping_add(i as u32));
                    inst2 = inst >> 12 | addr_1 << 20;
                    inst = 0x17 as u32
                        | inst2_rs1 << 7
                        | addr_1.wrapping_add(0x800 as u32) & 0xfffff000 as u32;
                    current_block_23 = 6669252993407410313;
                }
            }
            match current_block_23 {
                12517898123489920830 => {}
                _ => {
                    write32le(buffer.offset(i as isize), inst);
                    write32le(buffer.offset(i as isize).offset(4), inst2);
                    i = i.wrapping_add((8 as c_int - 2 as c_int) as size_t);
                }
            }
        }
        i = i.wrapping_add(2);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_riscv_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    return lzma_simple_coder_init(
        next,
        allocator,
        filters,
        Some(
            riscv_decode as unsafe extern "C" fn(*mut c_void, u32, bool, *mut u8, size_t) -> size_t,
        ),
        0,
        8,
        2,
        false,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_riscv_decode(
    mut start_offset: u32,
    buf: *mut u8,
    size: size_t,
) -> size_t {
    start_offset = (start_offset & !1u32) as u32;
    return riscv_decode(core::ptr::null_mut(), start_offset, false, buf, size);
}
