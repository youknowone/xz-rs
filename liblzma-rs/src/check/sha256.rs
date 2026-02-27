use crate::types::*;
use core::ffi::{c_int, c_uint, c_void};
extern "C" {
    fn memcpy(__dst: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_sha256_state {
    pub state: [u32; 8],
    pub size: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_check_state {
    pub buffer: C2RustUnnamed_0,
    pub state: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub crc32: u32,
    pub crc64: u64,
    pub sha256: lzma_sha256_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub u8_0: [u8; 64],
    pub u32_0: [u32; 16],
    pub u64_0: [u64; 8],
}
#[inline]
unsafe extern "C" fn rotr_32(mut num: u32, mut amount: c_uint) -> u32 {
    return num >> amount | num << (32 as c_uint).wrapping_sub(amount);
}
static mut SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0xfc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x6ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];
unsafe extern "C" fn transform(mut state: *mut u32, mut data: *const u32) {
    let mut W: [u32; 16] = [0; 16];
    let mut T: [u32; 8] = [0; 8];
    memcpy(
        &raw mut T as *mut u32 as *mut c_void,
        state as *const c_void,
        ::core::mem::size_of::<[u32; 8]>() as size_t,
    );
    W[0 as usize] = (*data.offset(0 as isize) & 0xff) << 24 as c_int
        | (*data.offset(0 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(0 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(0 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 0 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 0 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 0 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 0 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 0 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 0 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 0 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 0 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 0 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(0 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[0 as usize]),
        );
    T[(3 as c_int - 0 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 0 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 0 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 0 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 0 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 0 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 0 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 0 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 0 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 0 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 0 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 0 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 0 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[1 as usize] = (*data.offset(1 as isize) & 0xff) << 24 as c_int
        | (*data.offset(1 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(1 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(1 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 1 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 1 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 1 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 1 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 1 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 1 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 1 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 1 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 1 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(1 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[1 as usize]),
        );
    T[(3 as c_int - 1 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 1 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 1 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 1 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 1 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 1 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 1 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 1 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 1 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 1 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 1 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 1 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 1 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[2 as usize] = (*data.offset(2 as isize) & 0xff) << 24 as c_int
        | (*data.offset(2 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(2 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(2 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 2 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 2 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 2 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 2 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 2 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 2 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 2 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 2 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 2 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(2 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[2 as usize]),
        );
    T[(3 as c_int - 2 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 2 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 2 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 2 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 2 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 2 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 2 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 2 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 2 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 2 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 2 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 2 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 2 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[3 as usize] = (*data.offset(3 as isize) & 0xff) << 24 as c_int
        | (*data.offset(3 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(3 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(3 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 3 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 3 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 3 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 3 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 3 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 3 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 3 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 3 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 3 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(3 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[3 as usize]),
        );
    T[(3 as c_int - 3 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 3 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 3 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 3 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 3 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 3 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 3 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 3 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 3 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 3 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 3 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 3 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 3 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[4 as usize] = (*data.offset(4 as isize) & 0xff) << 24 as c_int
        | (*data.offset(4 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(4 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(4 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 4 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 4 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 4 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 4 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 4 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 4 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 4 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 4 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 4 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(4 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[4 as usize]),
        );
    T[(3 as c_int - 4 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 4 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 4 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 4 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 4 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 4 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 4 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 4 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 4 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 4 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 4 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 4 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 4 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[5 as usize] = (*data.offset(5 as isize) & 0xff) << 24 as c_int
        | (*data.offset(5 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(5 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(5 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 5 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 5 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 5 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 5 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 5 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 5 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 5 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 5 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 5 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(5 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[5 as usize]),
        );
    T[(3 as c_int - 5 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 5 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 5 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 5 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 5 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 5 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 5 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 5 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 5 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 5 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 5 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 5 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 5 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[6 as usize] = (*data.offset(6 as isize) & 0xff) << 24 as c_int
        | (*data.offset(6 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(6 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(6 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 6 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 6 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 6 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 6 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 6 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 6 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 6 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 6 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 6 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(6 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[6 as usize]),
        );
    T[(3 as c_int - 6 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 6 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 6 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 6 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 6 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 6 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 6 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 6 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 6 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 6 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 6 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 6 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 6 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[7 as usize] = (*data.offset(7 as isize) & 0xff) << 24 as c_int
        | (*data.offset(7 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(7 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(7 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 7 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 7 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 7 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 7 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 7 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 7 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 7 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 7 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 7 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(7 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[7 as usize]),
        );
    T[(3 as c_int - 7 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 7 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 7 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 7 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 7 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 7 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 7 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 7 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 7 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 7 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 7 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 7 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 7 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[8 as usize] = (*data.offset(8 as isize) & 0xff) << 24 as c_int
        | (*data.offset(8 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(8 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(8 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 8 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 8 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 8 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 8 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 8 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 8 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 8 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 8 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 8 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(8 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[8 as usize]),
        );
    T[(3 as c_int - 8 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 8 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 8 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 8 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 8 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 8 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 8 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 8 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 8 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 8 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 8 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 8 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 8 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[9 as usize] = (*data.offset(9 as isize) & 0xff) << 24 as c_int
        | (*data.offset(9 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(9 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(9 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 9 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 9 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 9 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 9 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 9 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 9 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 9 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 9 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 9 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(9 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[9 as usize]),
        );
    T[(3 as c_int - 9 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 9 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 9 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 9 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 9 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 9 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 9 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 9 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 9 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 9 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 9 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 9 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 9 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[10 as usize] = (*data.offset(10 as isize) & 0xff) << 24 as c_int
        | (*data.offset(10 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(10 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(10 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 10 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 10 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 10 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 10 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 10 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 10 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 10 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 10 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 10 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(10 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[10 as usize]),
        );
    T[(3 as c_int - 10 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 10 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 10 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 10 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 10 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 10 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 10 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 10 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 10 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 10 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 10 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 10 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 10 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[11 as usize] = (*data.offset(11 as isize) & 0xff) << 24 as c_int
        | (*data.offset(11 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(11 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(11 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 11 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 11 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 11 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 11 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 11 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 11 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 11 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 11 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 11 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(11 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[11 as usize]),
        );
    T[(3 as c_int - 11 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 11 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 11 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 11 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 11 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 11 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 11 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 11 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 11 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 11 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 11 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 11 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 11 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[12 as usize] = (*data.offset(12 as isize) & 0xff) << 24 as c_int
        | (*data.offset(12 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(12 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(12 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 12 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 12 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 12 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 12 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 12 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 12 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 12 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 12 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 12 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(12 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[12 as usize]),
        );
    T[(3 as c_int - 12 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 12 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 12 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 12 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 12 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 12 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 12 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 12 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 12 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 12 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 12 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 12 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 12 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[13 as usize] = (*data.offset(13 as isize) & 0xff) << 24 as c_int
        | (*data.offset(13 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(13 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(13 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 13 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 13 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 13 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 13 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 13 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 13 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 13 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 13 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 13 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(13 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[13 as usize]),
        );
    T[(3 as c_int - 13 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 13 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 13 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 13 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 13 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 13 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 13 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 13 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 13 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 13 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 13 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 13 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 13 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[14 as usize] = (*data.offset(14 as isize) & 0xff) << 24 as c_int
        | (*data.offset(14 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(14 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(14 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 14 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 14 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 14 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 14 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 14 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 14 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 14 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 14 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 14 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(14 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[14 as usize]),
        );
    T[(3 as c_int - 14 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 14 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 14 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 14 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 14 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 14 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 14 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 14 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 14 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 14 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 14 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 14 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 14 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    W[15 as usize] = (*data.offset(15 as isize) & 0xff) << 24 as c_int
        | (*data.offset(15 as isize) & 0xff00) << 8 as c_int
        | (*data.offset(15 as isize) & 0xff0000) >> 8 as c_int
        | (*data.offset(15 as isize) & 0xff000000) >> 24 as c_int;
    T[(7 as c_int - 15 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 15 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(4 as c_int - 15 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(4 as c_int - 15 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(4 as c_int - 15 as c_int & 7 as c_int) as usize], 14),
                        5,
                    ),
                6,
            )
            .wrapping_add(
                T[(6 as c_int - 15 as c_int & 7 as c_int) as usize]
                    ^ T[(4 as c_int - 15 as c_int & 7 as c_int) as usize]
                        & (T[(5 as c_int - 15 as c_int & 7 as c_int) as usize]
                            ^ T[(6 as c_int - 15 as c_int & 7 as c_int) as usize]),
            )
            .wrapping_add(SHA256_K[(15 as c_int + 0 as c_int) as usize])
            .wrapping_add(W[15 as usize]),
        );
    T[(3 as c_int - 15 as c_int & 7 as c_int) as usize] = T
        [(3 as c_int - 15 as c_int & 7 as c_int) as usize]
        .wrapping_add(T[(7 as c_int - 15 as c_int & 7 as c_int) as usize]);
    T[(7 as c_int - 15 as c_int & 7 as c_int) as usize] =
        T[(7 as c_int - 15 as c_int & 7 as c_int) as usize].wrapping_add(
            rotr_32(
                T[(0 as c_int - 15 as c_int & 7 as c_int) as usize]
                    ^ rotr_32(
                        T[(0 as c_int - 15 as c_int & 7 as c_int) as usize]
                            ^ rotr_32(T[(0 as c_int - 15 as c_int & 7 as c_int) as usize], 9),
                        11,
                    ),
                2,
            )
            .wrapping_add(
                (T[(0 as c_int - 15 as c_int & 7 as c_int) as usize]
                    & (T[(1 as c_int - 15 as c_int & 7 as c_int) as usize]
                        ^ T[(2 as c_int - 15 as c_int & 7 as c_int) as usize]))
                    .wrapping_add(
                        T[(1 as c_int - 15 as c_int & 7 as c_int) as usize]
                            & T[(2 as c_int - 15 as c_int & 7 as c_int) as usize],
                    ),
            ),
        );
    let mut j: c_uint = 16;
    while j < 64 {
        W[(0 as c_int & 15 as c_int) as usize] = W[(0 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(0 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(0 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(0 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(0 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(0 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(W[(0 as c_int - 15 as c_int & 15 as c_int) as usize], 11),
                            7,
                        ) ^ W[(0 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 0 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 0 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 0 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 0 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 0 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 0 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 0 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 0 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 0 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(0 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(0 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 0 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 0 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 0 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 0 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 0 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 0 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 0 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 0 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 0 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 0 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 0 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 0 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 0 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(1 as c_int & 15 as c_int) as usize] = W[(1 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(1 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(1 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(1 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(1 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(1 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(W[(1 as c_int - 15 as c_int & 15 as c_int) as usize], 11),
                            7,
                        ) ^ W[(1 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 1 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 1 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 1 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 1 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 1 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 1 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 1 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 1 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 1 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[1u32.wrapping_add(j) as usize])
                .wrapping_add(W[(1 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 1 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 1 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 1 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 1 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 1 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 1 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 1 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 1 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 1 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 1 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 1 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 1 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 1 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(2 as c_int & 15 as c_int) as usize] = W[(2 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(2 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(2 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(2 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(2 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(2 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(W[(2 as c_int - 15 as c_int & 15 as c_int) as usize], 11),
                            7,
                        ) ^ W[(2 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 2 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 2 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 2 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 2 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 2 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 2 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 2 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 2 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 2 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(2 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(2 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 2 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 2 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 2 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 2 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 2 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 2 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 2 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 2 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 2 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 2 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 2 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 2 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 2 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(3 as c_int & 15 as c_int) as usize] = W[(3 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(3 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(3 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(3 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(3 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(3 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(W[(3 as c_int - 15 as c_int & 15 as c_int) as usize], 11),
                            7,
                        ) ^ W[(3 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 3 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 3 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 3 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 3 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 3 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 3 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 3 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 3 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 3 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(3 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(3 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 3 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 3 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 3 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 3 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 3 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 3 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 3 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 3 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 3 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 3 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 3 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 3 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 3 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(4 as c_int & 15 as c_int) as usize] = W[(4 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(4 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(4 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(4 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(4 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(4 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(W[(4 as c_int - 15 as c_int & 15 as c_int) as usize], 11),
                            7,
                        ) ^ W[(4 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 4 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 4 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 4 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 4 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 4 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 4 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 4 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 4 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 4 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(4 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(4 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 4 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 4 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 4 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 4 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 4 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 4 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 4 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 4 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 4 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 4 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 4 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 4 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 4 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(5 as c_int & 15 as c_int) as usize] = W[(5 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(5 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(5 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(5 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(5 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(5 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(W[(5 as c_int - 15 as c_int & 15 as c_int) as usize], 11),
                            7,
                        ) ^ W[(5 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 5 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 5 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 5 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 5 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 5 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 5 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 5 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 5 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 5 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(5 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(5 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 5 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 5 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 5 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 5 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 5 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 5 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 5 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 5 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 5 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 5 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 5 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 5 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 5 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(6 as c_int & 15 as c_int) as usize] = W[(6 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(6 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(6 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(6 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(6 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(6 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(W[(6 as c_int - 15 as c_int & 15 as c_int) as usize], 11),
                            7,
                        ) ^ W[(6 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 6 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 6 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 6 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 6 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 6 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 6 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 6 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 6 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 6 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(6 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(6 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 6 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 6 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 6 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 6 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 6 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 6 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 6 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 6 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 6 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 6 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 6 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 6 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 6 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(7 as c_int & 15 as c_int) as usize] = W[(7 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(7 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(7 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(7 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(7 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(7 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(W[(7 as c_int - 15 as c_int & 15 as c_int) as usize], 11),
                            7,
                        ) ^ W[(7 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 7 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 7 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 7 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 7 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 7 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 7 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 7 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 7 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 7 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(7 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(7 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 7 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 7 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 7 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 7 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 7 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 7 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 7 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 7 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 7 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 7 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 7 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 7 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 7 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(8 as c_int & 15 as c_int) as usize] = W[(8 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(8 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(8 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(8 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(8 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(8 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(W[(8 as c_int - 15 as c_int & 15 as c_int) as usize], 11),
                            7,
                        ) ^ W[(8 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 8 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 8 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 8 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 8 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 8 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 8 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 8 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 8 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 8 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(8 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(8 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 8 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 8 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 8 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 8 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 8 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 8 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 8 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 8 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 8 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 8 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 8 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 8 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 8 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(9 as c_int & 15 as c_int) as usize] = W[(9 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(9 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(9 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(9 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(9 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(9 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(W[(9 as c_int - 15 as c_int & 15 as c_int) as usize], 11),
                            7,
                        ) ^ W[(9 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 9 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 9 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 9 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 9 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 9 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 9 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 9 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 9 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 9 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(9 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(9 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 9 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 9 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 9 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 9 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 9 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 9 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 9 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 9 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 9 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 9 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 9 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 9 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 9 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(10 as c_int & 15 as c_int) as usize] = W[(10 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(10 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(10 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(10 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(10 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(10 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(
                                    W[(10 as c_int - 15 as c_int & 15 as c_int) as usize],
                                    11,
                                ),
                            7,
                        ) ^ W[(10 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 10 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 10 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 10 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 10 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 10 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 10 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 10 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 10 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 10 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(10 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(10 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 10 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 10 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 10 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 10 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 10 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 10 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 10 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 10 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 10 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 10 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 10 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 10 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 10 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(11 as c_int & 15 as c_int) as usize] = W[(11 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(11 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(11 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(11 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(11 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(11 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(
                                    W[(11 as c_int - 15 as c_int & 15 as c_int) as usize],
                                    11,
                                ),
                            7,
                        ) ^ W[(11 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 11 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 11 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 11 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 11 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 11 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 11 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 11 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 11 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 11 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(11 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(11 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 11 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 11 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 11 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 11 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 11 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 11 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 11 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 11 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 11 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 11 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 11 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 11 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 11 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(12 as c_int & 15 as c_int) as usize] = W[(12 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(12 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(12 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(12 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(12 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(12 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(
                                    W[(12 as c_int - 15 as c_int & 15 as c_int) as usize],
                                    11,
                                ),
                            7,
                        ) ^ W[(12 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 12 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 12 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 12 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 12 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 12 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 12 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 12 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 12 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 12 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(12 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(12 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 12 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 12 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 12 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 12 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 12 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 12 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 12 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 12 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 12 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 12 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 12 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 12 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 12 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(13 as c_int & 15 as c_int) as usize] = W[(13 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(13 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(13 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(13 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(13 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(13 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(
                                    W[(13 as c_int - 15 as c_int & 15 as c_int) as usize],
                                    11,
                                ),
                            7,
                        ) ^ W[(13 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 13 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 13 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 13 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 13 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 13 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 13 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 13 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 13 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 13 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(13 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(13 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 13 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 13 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 13 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 13 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 13 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 13 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 13 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 13 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 13 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 13 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 13 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 13 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 13 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(14 as c_int & 15 as c_int) as usize] = W[(14 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(14 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(14 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(14 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(14 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(14 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(
                                    W[(14 as c_int - 15 as c_int & 15 as c_int) as usize],
                                    11,
                                ),
                            7,
                        ) ^ W[(14 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 14 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 14 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 14 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 14 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 14 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 14 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 14 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 14 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 14 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(14 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(14 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 14 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 14 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 14 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 14 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 14 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 14 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 14 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 14 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 14 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 14 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 14 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 14 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 14 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        W[(15 as c_int & 15 as c_int) as usize] = W[(15 as c_int & 15 as c_int) as usize]
            .wrapping_add(
                (rotr_32(
                    W[(15 as c_int - 2 as c_int & 15 as c_int) as usize]
                        ^ rotr_32(W[(15 as c_int - 2 as c_int & 15 as c_int) as usize], 2),
                    17,
                ) ^ W[(15 as c_int - 2 as c_int & 15 as c_int) as usize] >> 10 as c_int)
                    .wrapping_add(W[(15 as c_int - 7 as c_int & 15 as c_int) as usize])
                    .wrapping_add(
                        rotr_32(
                            W[(15 as c_int - 15 as c_int & 15 as c_int) as usize]
                                ^ rotr_32(
                                    W[(15 as c_int - 15 as c_int & 15 as c_int) as usize],
                                    11,
                                ),
                            7,
                        ) ^ W[(15 as c_int - 15 as c_int & 15 as c_int) as usize] >> 3 as c_int,
                    ),
            );
        T[(7 as c_int - 15 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 15 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(4 as c_int - 15 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(4 as c_int - 15 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(4 as c_int - 15 as c_int & 7 as c_int) as usize], 14),
                            5,
                        ),
                    6,
                )
                .wrapping_add(
                    T[(6 as c_int - 15 as c_int & 7 as c_int) as usize]
                        ^ T[(4 as c_int - 15 as c_int & 7 as c_int) as usize]
                            & (T[(5 as c_int - 15 as c_int & 7 as c_int) as usize]
                                ^ T[(6 as c_int - 15 as c_int & 7 as c_int) as usize]),
                )
                .wrapping_add(SHA256_K[(15 as c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(15 as c_int & 15 as c_int) as usize]),
            );
        T[(3 as c_int - 15 as c_int & 7 as c_int) as usize] = T
            [(3 as c_int - 15 as c_int & 7 as c_int) as usize]
            .wrapping_add(T[(7 as c_int - 15 as c_int & 7 as c_int) as usize]);
        T[(7 as c_int - 15 as c_int & 7 as c_int) as usize] =
            T[(7 as c_int - 15 as c_int & 7 as c_int) as usize].wrapping_add(
                rotr_32(
                    T[(0 as c_int - 15 as c_int & 7 as c_int) as usize]
                        ^ rotr_32(
                            T[(0 as c_int - 15 as c_int & 7 as c_int) as usize]
                                ^ rotr_32(T[(0 as c_int - 15 as c_int & 7 as c_int) as usize], 9),
                            11,
                        ),
                    2,
                )
                .wrapping_add(
                    (T[(0 as c_int - 15 as c_int & 7 as c_int) as usize]
                        & (T[(1 as c_int - 15 as c_int & 7 as c_int) as usize]
                            ^ T[(2 as c_int - 15 as c_int & 7 as c_int) as usize]))
                        .wrapping_add(
                            T[(1 as c_int - 15 as c_int & 7 as c_int) as usize]
                                & T[(2 as c_int - 15 as c_int & 7 as c_int) as usize],
                        ),
                ),
            );
        j = j.wrapping_add(16);
    }
    let ref mut fresh0 = *state.offset(0 as isize);
    *fresh0 = (*fresh0).wrapping_add(T[(0 as c_int - 0 as c_int & 7 as c_int) as usize]);
    let ref mut fresh1 = *state.offset(1 as isize);
    *fresh1 = (*fresh1).wrapping_add(T[(1 as c_int - 0 as c_int & 7 as c_int) as usize]);
    let ref mut fresh2 = *state.offset(2 as isize);
    *fresh2 = (*fresh2).wrapping_add(T[(2 as c_int - 0 as c_int & 7 as c_int) as usize]);
    let ref mut fresh3 = *state.offset(3 as isize);
    *fresh3 = (*fresh3).wrapping_add(T[(3 as c_int - 0 as c_int & 7 as c_int) as usize]);
    let ref mut fresh4 = *state.offset(4 as isize);
    *fresh4 = (*fresh4).wrapping_add(T[(4 as c_int - 0 as c_int & 7 as c_int) as usize]);
    let ref mut fresh5 = *state.offset(5 as isize);
    *fresh5 = (*fresh5).wrapping_add(T[(5 as c_int - 0 as c_int & 7 as c_int) as usize]);
    let ref mut fresh6 = *state.offset(6 as isize);
    *fresh6 = (*fresh6).wrapping_add(T[(6 as c_int - 0 as c_int & 7 as c_int) as usize]);
    let ref mut fresh7 = *state.offset(7 as isize);
    *fresh7 = (*fresh7).wrapping_add(T[(7 as c_int - 0 as c_int & 7 as c_int) as usize]);
}
unsafe extern "C" fn process(mut check: *mut lzma_check_state) {
    transform(
        &raw mut (*check).state.sha256.state as *mut u32,
        &raw mut (*check).buffer.u32_0 as *mut u32 as *const u32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_sha256_init(mut check: *mut lzma_check_state) {
    static mut s: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    memcpy(
        &raw mut (*check).state.sha256.state as *mut u32 as *mut c_void,
        &raw const s as *const u32 as *const c_void,
        ::core::mem::size_of::<[u32; 8]>() as size_t,
    );
    (*check).state.sha256.size = 0 as u64;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_sha256_update(
    mut buf: *const u8,
    mut size: size_t,
    mut check: *mut lzma_check_state,
) {
    while size > 0 as size_t {
        let copy_start: size_t = ((*check).state.sha256.size & 0x3f as u64) as size_t;
        let mut copy_size: size_t = (64 as size_t).wrapping_sub(copy_start);
        if copy_size > size {
            copy_size = size;
        }
        memcpy(
            (&raw mut (*check).buffer.u8_0 as *mut u8).offset(copy_start as isize) as *mut c_void,
            buf as *const c_void,
            copy_size,
        );
        buf = buf.offset(copy_size as isize);
        size = size.wrapping_sub(copy_size);
        (*check).state.sha256.size = (*check).state.sha256.size.wrapping_add(copy_size as u64);
        if (*check).state.sha256.size & 0x3f as u64 == 0 as u64 {
            process(check);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_sha256_finish(mut check: *mut lzma_check_state) {
    let mut pos: size_t = ((*check).state.sha256.size & 0x3f as u64) as size_t;
    let fresh8 = pos;
    pos = pos.wrapping_add(1);
    (*check).buffer.u8_0[fresh8 as usize] = 0x80 as u8;
    while pos != (64 as c_int - 8 as c_int) as size_t {
        if pos == 64 as size_t {
            process(check);
            pos = 0 as size_t;
        }
        let fresh9 = pos;
        pos = pos.wrapping_add(1);
        (*check).buffer.u8_0[fresh9 as usize] = 0 as u8;
    }
    (*check).state.sha256.size = (*check).state.sha256.size.wrapping_mul(8 as u64);
    (*check).buffer.u64_0[((64 as c_int - 8 as c_int) / 8 as c_int) as usize] =
        ((*check).state.sha256.size & 0xff as u64) << 56 as c_int
            | ((*check).state.sha256.size & 0xff00 as u64) << 40 as c_int
            | ((*check).state.sha256.size & 0xff0000 as u64) << 24 as c_int
            | ((*check).state.sha256.size & 0xff000000 as u64) << 8 as c_int
            | ((*check).state.sha256.size & 0xff00000000 as u64) >> 8 as c_int
            | ((*check).state.sha256.size & 0xff0000000000 as u64) >> 24 as c_int
            | ((*check).state.sha256.size & 0xff000000000000 as u64) >> 40 as c_int
            | ((*check).state.sha256.size & 0xff00000000000000 as u64) >> 56 as c_int;
    process(check);
    let mut i: size_t = 0 as size_t;
    while i < 8 as size_t {
        (*check).buffer.u32_0[i as usize] = ((*check).state.sha256.state[i as usize] & 0xff)
            << 24 as c_int
            | ((*check).state.sha256.state[i as usize] & 0xff00) << 8 as c_int
            | ((*check).state.sha256.state[i as usize] & 0xff0000) >> 8 as c_int
            | ((*check).state.sha256.state[i as usize] & 0xff000000) >> 24 as c_int;
        i = i.wrapping_add(1);
    }
}
