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
extern "C" fn rotr_32(num: u32, amount: c_uint) -> u32 {
    return num >> amount | num << 32u32.wrapping_sub(amount);
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
unsafe extern "C" fn transform(state: *mut u32, data: *const u32) {
    let mut W: [u32; 16] = [0; 16];
    let mut T: [u32; 8] = [0; 8];
    memcpy(
        &raw mut T as *mut u32 as *mut c_void,
        state as *const c_void,
        core::mem::size_of::<[u32; 8]>() as size_t,
    );
    W[0] = (*data.offset(0) & 0xff) << 24
        | (*data.offset(0) & 0xff00) << 8
        | (*data.offset(0) & 0xff0000) >> 8
        | (*data.offset(0) & 0xff000000) >> 24;
    T[7] = T[7].wrapping_add(
        rotr_32(T[4] ^ rotr_32(T[4] ^ rotr_32(T[4], 14), 5), 6)
            .wrapping_add(T[6] ^ T[4] & (T[5] ^ T[6]))
            .wrapping_add(SHA256_K[0])
            .wrapping_add(W[0]),
    );
    T[3] = T[3].wrapping_add(T[7]);
    T[7] = T[7].wrapping_add(
        rotr_32(T[0] ^ rotr_32(T[0] ^ rotr_32(T[0], 9), 11), 2)
            .wrapping_add((T[0] & (T[1] ^ T[2])).wrapping_add(T[1] & T[2])),
    );
    W[1] = (*data.offset(1) & 0xff) << 24
        | (*data.offset(1) & 0xff00) << 8
        | (*data.offset(1) & 0xff0000) >> 8
        | (*data.offset(1) & 0xff000000) >> 24;
    T[6] = T[6].wrapping_add(
        rotr_32(T[3] ^ rotr_32(T[3] ^ rotr_32(T[3], 14), 5), 6)
            .wrapping_add(T[5] ^ T[3] & (T[4] ^ T[5]))
            .wrapping_add(SHA256_K[1])
            .wrapping_add(W[1]),
    );
    T[2] = T[2].wrapping_add(T[6]);
    T[6] = T[6].wrapping_add(
        rotr_32(T[7] ^ rotr_32(T[7] ^ rotr_32(T[7], 9), 11), 2)
            .wrapping_add((T[7] & (T[0] ^ T[1])).wrapping_add(T[0] & T[1])),
    );
    W[2] = (*data.offset(2) & 0xff) << 24
        | (*data.offset(2) & 0xff00) << 8
        | (*data.offset(2) & 0xff0000) >> 8
        | (*data.offset(2) & 0xff000000) >> 24;
    T[5] = T[5].wrapping_add(
        rotr_32(T[2] ^ rotr_32(T[2] ^ rotr_32(T[2], 14), 5), 6)
            .wrapping_add(T[4] ^ T[2] & (T[3] ^ T[4]))
            .wrapping_add(SHA256_K[2])
            .wrapping_add(W[2]),
    );
    T[1] = T[1].wrapping_add(T[5]);
    T[5] = T[5].wrapping_add(
        rotr_32(T[6] ^ rotr_32(T[6] ^ rotr_32(T[6], 9), 11), 2)
            .wrapping_add((T[6] & (T[7] ^ T[0])).wrapping_add(T[7] & T[0])),
    );
    W[3] = (*data.offset(3) & 0xff) << 24
        | (*data.offset(3) & 0xff00) << 8
        | (*data.offset(3) & 0xff0000) >> 8
        | (*data.offset(3) & 0xff000000) >> 24;
    T[4] = T[4].wrapping_add(
        rotr_32(T[1] ^ rotr_32(T[1] ^ rotr_32(T[1], 14), 5), 6)
            .wrapping_add(T[3] ^ T[1] & (T[2] ^ T[3]))
            .wrapping_add(SHA256_K[3])
            .wrapping_add(W[3]),
    );
    T[0] = T[0].wrapping_add(T[4]);
    T[4] = T[4].wrapping_add(
        rotr_32(T[5] ^ rotr_32(T[5] ^ rotr_32(T[5], 9), 11), 2)
            .wrapping_add((T[5] & (T[6] ^ T[7])).wrapping_add(T[6] & T[7])),
    );
    W[4] = (*data.offset(4) & 0xff) << 24
        | (*data.offset(4) & 0xff00) << 8
        | (*data.offset(4) & 0xff0000) >> 8
        | (*data.offset(4) & 0xff000000) >> 24;
    T[3] = T[3].wrapping_add(
        rotr_32(T[0] ^ rotr_32(T[0] ^ rotr_32(T[0], 14), 5), 6)
            .wrapping_add(T[2] ^ T[0] & (T[1] ^ T[2]))
            .wrapping_add(SHA256_K[4])
            .wrapping_add(W[4]),
    );
    T[7] = T[7].wrapping_add(T[3]);
    T[3] = T[3].wrapping_add(
        rotr_32(T[4] ^ rotr_32(T[4] ^ rotr_32(T[4], 9), 11), 2)
            .wrapping_add((T[4] & (T[5] ^ T[6])).wrapping_add(T[5] & T[6])),
    );
    W[5] = (*data.offset(5) & 0xff) << 24
        | (*data.offset(5) & 0xff00) << 8
        | (*data.offset(5) & 0xff0000) >> 8
        | (*data.offset(5) & 0xff000000) >> 24;
    T[2] = T[2].wrapping_add(
        rotr_32(T[7] ^ rotr_32(T[7] ^ rotr_32(T[7], 14), 5), 6)
            .wrapping_add(T[1] ^ T[7] & (T[0] ^ T[1]))
            .wrapping_add(SHA256_K[5])
            .wrapping_add(W[5]),
    );
    T[6] = T[6].wrapping_add(T[2]);
    T[2] = T[2].wrapping_add(
        rotr_32(T[3] ^ rotr_32(T[3] ^ rotr_32(T[3], 9), 11), 2)
            .wrapping_add((T[3] & (T[4] ^ T[5])).wrapping_add(T[4] & T[5])),
    );
    W[6] = (*data.offset(6) & 0xff) << 24
        | (*data.offset(6) & 0xff00) << 8
        | (*data.offset(6) & 0xff0000) >> 8
        | (*data.offset(6) & 0xff000000) >> 24;
    T[1] = T[1].wrapping_add(
        rotr_32(T[6] ^ rotr_32(T[6] ^ rotr_32(T[6], 14), 5), 6)
            .wrapping_add(T[0] ^ T[6] & (T[7] ^ T[0]))
            .wrapping_add(SHA256_K[6])
            .wrapping_add(W[6]),
    );
    T[5] = T[5].wrapping_add(T[1]);
    T[1] = T[1].wrapping_add(
        rotr_32(T[2] ^ rotr_32(T[2] ^ rotr_32(T[2], 9), 11), 2)
            .wrapping_add((T[2] & (T[3] ^ T[4])).wrapping_add(T[3] & T[4])),
    );
    W[7] = (*data.offset(7) & 0xff) << 24
        | (*data.offset(7) & 0xff00) << 8
        | (*data.offset(7) & 0xff0000) >> 8
        | (*data.offset(7) & 0xff000000) >> 24;
    T[0] = T[0].wrapping_add(
        rotr_32(T[5] ^ rotr_32(T[5] ^ rotr_32(T[5], 14), 5), 6)
            .wrapping_add(T[7] ^ T[5] & (T[6] ^ T[7]))
            .wrapping_add(SHA256_K[7])
            .wrapping_add(W[7]),
    );
    T[4] = T[4].wrapping_add(T[0]);
    T[0] = T[0].wrapping_add(
        rotr_32(T[1] ^ rotr_32(T[1] ^ rotr_32(T[1], 9), 11), 2)
            .wrapping_add((T[1] & (T[2] ^ T[3])).wrapping_add(T[2] & T[3])),
    );
    W[8] = (*data.offset(8) & 0xff) << 24
        | (*data.offset(8) & 0xff00) << 8
        | (*data.offset(8) & 0xff0000) >> 8
        | (*data.offset(8) & 0xff000000) >> 24;
    T[7] = T[7].wrapping_add(
        rotr_32(T[4] ^ rotr_32(T[4] ^ rotr_32(T[4], 14), 5), 6)
            .wrapping_add(T[6] ^ T[4] & (T[5] ^ T[6]))
            .wrapping_add(SHA256_K[8])
            .wrapping_add(W[8]),
    );
    T[3] = T[3].wrapping_add(T[7]);
    T[7] = T[7].wrapping_add(
        rotr_32(T[0] ^ rotr_32(T[0] ^ rotr_32(T[0], 9), 11), 2)
            .wrapping_add((T[0] & (T[1] ^ T[2])).wrapping_add(T[1] & T[2])),
    );
    W[9] = (*data.offset(9) & 0xff) << 24
        | (*data.offset(9) & 0xff00) << 8
        | (*data.offset(9) & 0xff0000) >> 8
        | (*data.offset(9) & 0xff000000) >> 24;
    T[6] = T[6].wrapping_add(
        rotr_32(T[3] ^ rotr_32(T[3] ^ rotr_32(T[3], 14), 5), 6)
            .wrapping_add(T[5] ^ T[3] & (T[4] ^ T[5]))
            .wrapping_add(SHA256_K[9])
            .wrapping_add(W[9]),
    );
    T[2] = T[2].wrapping_add(T[6]);
    T[6] = T[6].wrapping_add(
        rotr_32(T[7] ^ rotr_32(T[7] ^ rotr_32(T[7], 9), 11), 2)
            .wrapping_add((T[7] & (T[0] ^ T[1])).wrapping_add(T[0] & T[1])),
    );
    W[10] = (*data.offset(10) & 0xff) << 24
        | (*data.offset(10) & 0xff00) << 8
        | (*data.offset(10) & 0xff0000) >> 8
        | (*data.offset(10) & 0xff000000) >> 24;
    T[5] = T[5].wrapping_add(
        rotr_32(T[2] ^ rotr_32(T[2] ^ rotr_32(T[2], 14), 5), 6)
            .wrapping_add(T[4] ^ T[2] & (T[3] ^ T[4]))
            .wrapping_add(SHA256_K[10])
            .wrapping_add(W[10]),
    );
    T[1] = T[1].wrapping_add(T[5]);
    T[5] = T[5].wrapping_add(
        rotr_32(T[6] ^ rotr_32(T[6] ^ rotr_32(T[6], 9), 11), 2)
            .wrapping_add((T[6] & (T[7] ^ T[0])).wrapping_add(T[7] & T[0])),
    );
    W[11] = (*data.offset(11) & 0xff) << 24
        | (*data.offset(11) & 0xff00) << 8
        | (*data.offset(11) & 0xff0000) >> 8
        | (*data.offset(11) & 0xff000000) >> 24;
    T[4] = T[4].wrapping_add(
        rotr_32(T[1] ^ rotr_32(T[1] ^ rotr_32(T[1], 14), 5), 6)
            .wrapping_add(T[3] ^ T[1] & (T[2] ^ T[3]))
            .wrapping_add(SHA256_K[11])
            .wrapping_add(W[11]),
    );
    T[0] = T[0].wrapping_add(T[4]);
    T[4] = T[4].wrapping_add(
        rotr_32(T[5] ^ rotr_32(T[5] ^ rotr_32(T[5], 9), 11), 2)
            .wrapping_add((T[5] & (T[6] ^ T[7])).wrapping_add(T[6] & T[7])),
    );
    W[12] = (*data.offset(12) & 0xff) << 24
        | (*data.offset(12) & 0xff00) << 8
        | (*data.offset(12) & 0xff0000) >> 8
        | (*data.offset(12) & 0xff000000) >> 24;
    T[3] = T[3].wrapping_add(
        rotr_32(T[0] ^ rotr_32(T[0] ^ rotr_32(T[0], 14), 5), 6)
            .wrapping_add(T[2] ^ T[0] & (T[1] ^ T[2]))
            .wrapping_add(SHA256_K[12])
            .wrapping_add(W[12]),
    );
    T[7] = T[7].wrapping_add(T[3]);
    T[3] = T[3].wrapping_add(
        rotr_32(T[4] ^ rotr_32(T[4] ^ rotr_32(T[4], 9), 11), 2)
            .wrapping_add((T[4] & (T[5] ^ T[6])).wrapping_add(T[5] & T[6])),
    );
    W[13] = (*data.offset(13) & 0xff) << 24
        | (*data.offset(13) & 0xff00) << 8
        | (*data.offset(13) & 0xff0000) >> 8
        | (*data.offset(13) & 0xff000000) >> 24;
    T[2] = T[2].wrapping_add(
        rotr_32(T[7] ^ rotr_32(T[7] ^ rotr_32(T[7], 14), 5), 6)
            .wrapping_add(T[1] ^ T[7] & (T[0] ^ T[1]))
            .wrapping_add(SHA256_K[13])
            .wrapping_add(W[13]),
    );
    T[6] = T[6].wrapping_add(T[2]);
    T[2] = T[2].wrapping_add(
        rotr_32(T[3] ^ rotr_32(T[3] ^ rotr_32(T[3], 9), 11), 2)
            .wrapping_add((T[3] & (T[4] ^ T[5])).wrapping_add(T[4] & T[5])),
    );
    W[14] = (*data.offset(14) & 0xff) << 24
        | (*data.offset(14) & 0xff00) << 8
        | (*data.offset(14) & 0xff0000) >> 8
        | (*data.offset(14) & 0xff000000) >> 24;
    T[1] = T[1].wrapping_add(
        rotr_32(T[6] ^ rotr_32(T[6] ^ rotr_32(T[6], 14), 5), 6)
            .wrapping_add(T[0] ^ T[6] & (T[7] ^ T[0]))
            .wrapping_add(SHA256_K[14])
            .wrapping_add(W[14]),
    );
    T[5] = T[5].wrapping_add(T[1]);
    T[1] = T[1].wrapping_add(
        rotr_32(T[2] ^ rotr_32(T[2] ^ rotr_32(T[2], 9), 11), 2)
            .wrapping_add((T[2] & (T[3] ^ T[4])).wrapping_add(T[3] & T[4])),
    );
    W[15] = (*data.offset(15) & 0xff) << 24
        | (*data.offset(15) & 0xff00) << 8
        | (*data.offset(15) & 0xff0000) >> 8
        | (*data.offset(15) & 0xff000000) >> 24;
    T[0] = T[0].wrapping_add(
        rotr_32(T[5] ^ rotr_32(T[5] ^ rotr_32(T[5], 14), 5), 6)
            .wrapping_add(T[7] ^ T[5] & (T[6] ^ T[7]))
            .wrapping_add(SHA256_K[15])
            .wrapping_add(W[15]),
    );
    T[4] = T[4].wrapping_add(T[0]);
    T[0] = T[0].wrapping_add(
        rotr_32(T[1] ^ rotr_32(T[1] ^ rotr_32(T[1], 9), 11), 2)
            .wrapping_add((T[1] & (T[2] ^ T[3])).wrapping_add(T[2] & T[3])),
    );
    let mut j: c_uint = 16;
    while j < 64 {
        W[0] = W[0].wrapping_add(
            (rotr_32(W[14] ^ rotr_32(W[14], 2), 17) ^ W[14] >> 10)
                .wrapping_add(W[9])
                .wrapping_add(rotr_32(W[1] ^ rotr_32(W[1], 11), 7) ^ W[1] >> 3),
        );
        T[7] = T[7].wrapping_add(
            rotr_32(T[4] ^ rotr_32(T[4] ^ rotr_32(T[4], 14), 5), 6)
                .wrapping_add(T[6] ^ T[4] & (T[5] ^ T[6]))
                .wrapping_add(SHA256_K[0u32.wrapping_add(j) as usize])
                .wrapping_add(W[0]),
        );
        T[3] = T[3].wrapping_add(T[7]);
        T[7] = T[7].wrapping_add(
            rotr_32(T[0] ^ rotr_32(T[0] ^ rotr_32(T[0], 9), 11), 2)
                .wrapping_add((T[0] & (T[1] ^ T[2])).wrapping_add(T[1] & T[2])),
        );
        W[1] = W[1].wrapping_add(
            (rotr_32(W[15] ^ rotr_32(W[15], 2), 17) ^ W[15] >> 10)
                .wrapping_add(W[10])
                .wrapping_add(rotr_32(W[2] ^ rotr_32(W[2], 11), 7) ^ W[2] >> 3),
        );
        T[6] = T[6].wrapping_add(
            rotr_32(T[3] ^ rotr_32(T[3] ^ rotr_32(T[3], 14), 5), 6)
                .wrapping_add(T[5] ^ T[3] & (T[4] ^ T[5]))
                .wrapping_add(SHA256_K[1u32.wrapping_add(j) as usize])
                .wrapping_add(W[1]),
        );
        T[2] = T[2].wrapping_add(T[6]);
        T[6] = T[6].wrapping_add(
            rotr_32(T[7] ^ rotr_32(T[7] ^ rotr_32(T[7], 9), 11), 2)
                .wrapping_add((T[7] & (T[0] ^ T[1])).wrapping_add(T[0] & T[1])),
        );
        W[2] = W[2].wrapping_add(
            (rotr_32(W[0] ^ rotr_32(W[0], 2), 17) ^ W[0] >> 10)
                .wrapping_add(W[11])
                .wrapping_add(rotr_32(W[3] ^ rotr_32(W[3], 11), 7) ^ W[3] >> 3),
        );
        T[5] = T[5].wrapping_add(
            rotr_32(T[2] ^ rotr_32(T[2] ^ rotr_32(T[2], 14), 5), 6)
                .wrapping_add(T[4] ^ T[2] & (T[3] ^ T[4]))
                .wrapping_add(SHA256_K[2u32.wrapping_add(j) as usize])
                .wrapping_add(W[2]),
        );
        T[1] = T[1].wrapping_add(T[5]);
        T[5] = T[5].wrapping_add(
            rotr_32(T[6] ^ rotr_32(T[6] ^ rotr_32(T[6], 9), 11), 2)
                .wrapping_add((T[6] & (T[7] ^ T[0])).wrapping_add(T[7] & T[0])),
        );
        W[3] = W[3].wrapping_add(
            (rotr_32(W[1] ^ rotr_32(W[1], 2), 17) ^ W[1] >> 10)
                .wrapping_add(W[12])
                .wrapping_add(rotr_32(W[4] ^ rotr_32(W[4], 11), 7) ^ W[4] >> 3),
        );
        T[4] = T[4].wrapping_add(
            rotr_32(T[1] ^ rotr_32(T[1] ^ rotr_32(T[1], 14), 5), 6)
                .wrapping_add(T[3] ^ T[1] & (T[2] ^ T[3]))
                .wrapping_add(SHA256_K[3u32.wrapping_add(j) as usize])
                .wrapping_add(W[3]),
        );
        T[0] = T[0].wrapping_add(T[4]);
        T[4] = T[4].wrapping_add(
            rotr_32(T[5] ^ rotr_32(T[5] ^ rotr_32(T[5], 9), 11), 2)
                .wrapping_add((T[5] & (T[6] ^ T[7])).wrapping_add(T[6] & T[7])),
        );
        W[4] = W[4].wrapping_add(
            (rotr_32(W[2] ^ rotr_32(W[2], 2), 17) ^ W[2] >> 10)
                .wrapping_add(W[13])
                .wrapping_add(rotr_32(W[5] ^ rotr_32(W[5], 11), 7) ^ W[5] >> 3),
        );
        T[3] = T[3].wrapping_add(
            rotr_32(T[0] ^ rotr_32(T[0] ^ rotr_32(T[0], 14), 5), 6)
                .wrapping_add(T[2] ^ T[0] & (T[1] ^ T[2]))
                .wrapping_add(SHA256_K[4u32.wrapping_add(j) as usize])
                .wrapping_add(W[4]),
        );
        T[7] = T[7].wrapping_add(T[3]);
        T[3] = T[3].wrapping_add(
            rotr_32(T[4] ^ rotr_32(T[4] ^ rotr_32(T[4], 9), 11), 2)
                .wrapping_add((T[4] & (T[5] ^ T[6])).wrapping_add(T[5] & T[6])),
        );
        W[5] = W[5].wrapping_add(
            (rotr_32(W[3] ^ rotr_32(W[3], 2), 17) ^ W[3] >> 10)
                .wrapping_add(W[14])
                .wrapping_add(rotr_32(W[6] ^ rotr_32(W[6], 11), 7) ^ W[6] >> 3),
        );
        T[2] = T[2].wrapping_add(
            rotr_32(T[7] ^ rotr_32(T[7] ^ rotr_32(T[7], 14), 5), 6)
                .wrapping_add(T[1] ^ T[7] & (T[0] ^ T[1]))
                .wrapping_add(SHA256_K[5u32.wrapping_add(j) as usize])
                .wrapping_add(W[5]),
        );
        T[6] = T[6].wrapping_add(T[2]);
        T[2] = T[2].wrapping_add(
            rotr_32(T[3] ^ rotr_32(T[3] ^ rotr_32(T[3], 9), 11), 2)
                .wrapping_add((T[3] & (T[4] ^ T[5])).wrapping_add(T[4] & T[5])),
        );
        W[6] = W[6].wrapping_add(
            (rotr_32(W[4] ^ rotr_32(W[4], 2), 17) ^ W[4] >> 10)
                .wrapping_add(W[15])
                .wrapping_add(rotr_32(W[7] ^ rotr_32(W[7], 11), 7) ^ W[7] >> 3),
        );
        T[1] = T[1].wrapping_add(
            rotr_32(T[6] ^ rotr_32(T[6] ^ rotr_32(T[6], 14), 5), 6)
                .wrapping_add(T[0] ^ T[6] & (T[7] ^ T[0]))
                .wrapping_add(SHA256_K[6u32.wrapping_add(j) as usize])
                .wrapping_add(W[6]),
        );
        T[5] = T[5].wrapping_add(T[1]);
        T[1] = T[1].wrapping_add(
            rotr_32(T[2] ^ rotr_32(T[2] ^ rotr_32(T[2], 9), 11), 2)
                .wrapping_add((T[2] & (T[3] ^ T[4])).wrapping_add(T[3] & T[4])),
        );
        W[7] = W[7].wrapping_add(
            (rotr_32(W[5] ^ rotr_32(W[5], 2), 17) ^ W[5] >> 10)
                .wrapping_add(W[0])
                .wrapping_add(rotr_32(W[8] ^ rotr_32(W[8], 11), 7) ^ W[8] >> 3),
        );
        T[0] = T[0].wrapping_add(
            rotr_32(T[5] ^ rotr_32(T[5] ^ rotr_32(T[5], 14), 5), 6)
                .wrapping_add(T[7] ^ T[5] & (T[6] ^ T[7]))
                .wrapping_add(SHA256_K[7u32.wrapping_add(j) as usize])
                .wrapping_add(W[7]),
        );
        T[4] = T[4].wrapping_add(T[0]);
        T[0] = T[0].wrapping_add(
            rotr_32(T[1] ^ rotr_32(T[1] ^ rotr_32(T[1], 9), 11), 2)
                .wrapping_add((T[1] & (T[2] ^ T[3])).wrapping_add(T[2] & T[3])),
        );
        W[8] = W[8].wrapping_add(
            (rotr_32(W[6] ^ rotr_32(W[6], 2), 17) ^ W[6] >> 10)
                .wrapping_add(W[1])
                .wrapping_add(rotr_32(W[9] ^ rotr_32(W[9], 11), 7) ^ W[9] >> 3),
        );
        T[7] = T[7].wrapping_add(
            rotr_32(T[4] ^ rotr_32(T[4] ^ rotr_32(T[4], 14), 5), 6)
                .wrapping_add(T[6] ^ T[4] & (T[5] ^ T[6]))
                .wrapping_add(SHA256_K[8u32.wrapping_add(j) as usize])
                .wrapping_add(W[8]),
        );
        T[3] = T[3].wrapping_add(T[7]);
        T[7] = T[7].wrapping_add(
            rotr_32(T[0] ^ rotr_32(T[0] ^ rotr_32(T[0], 9), 11), 2)
                .wrapping_add((T[0] & (T[1] ^ T[2])).wrapping_add(T[1] & T[2])),
        );
        W[9] = W[9].wrapping_add(
            (rotr_32(W[7] ^ rotr_32(W[7], 2), 17) ^ W[7] >> 10)
                .wrapping_add(W[2])
                .wrapping_add(rotr_32(W[10] ^ rotr_32(W[10], 11), 7) ^ W[10] >> 3),
        );
        T[6] = T[6].wrapping_add(
            rotr_32(T[3] ^ rotr_32(T[3] ^ rotr_32(T[3], 14), 5), 6)
                .wrapping_add(T[5] ^ T[3] & (T[4] ^ T[5]))
                .wrapping_add(SHA256_K[9u32.wrapping_add(j) as usize])
                .wrapping_add(W[9]),
        );
        T[2] = T[2].wrapping_add(T[6]);
        T[6] = T[6].wrapping_add(
            rotr_32(T[7] ^ rotr_32(T[7] ^ rotr_32(T[7], 9), 11), 2)
                .wrapping_add((T[7] & (T[0] ^ T[1])).wrapping_add(T[0] & T[1])),
        );
        W[10] = W[10].wrapping_add(
            (rotr_32(W[8] ^ rotr_32(W[8], 2), 17) ^ W[8] >> 10)
                .wrapping_add(W[3])
                .wrapping_add(rotr_32(W[11] ^ rotr_32(W[11], 11), 7) ^ W[11] >> 3),
        );
        T[5] = T[5].wrapping_add(
            rotr_32(T[2] ^ rotr_32(T[2] ^ rotr_32(T[2], 14), 5), 6)
                .wrapping_add(T[4] ^ T[2] & (T[3] ^ T[4]))
                .wrapping_add(SHA256_K[10u32.wrapping_add(j) as usize])
                .wrapping_add(W[10]),
        );
        T[1] = T[1].wrapping_add(T[5]);
        T[5] = T[5].wrapping_add(
            rotr_32(T[6] ^ rotr_32(T[6] ^ rotr_32(T[6], 9), 11), 2)
                .wrapping_add((T[6] & (T[7] ^ T[0])).wrapping_add(T[7] & T[0])),
        );
        W[11] = W[11].wrapping_add(
            (rotr_32(W[9] ^ rotr_32(W[9], 2), 17) ^ W[9] >> 10)
                .wrapping_add(W[4])
                .wrapping_add(rotr_32(W[12] ^ rotr_32(W[12], 11), 7) ^ W[12] >> 3),
        );
        T[4] = T[4].wrapping_add(
            rotr_32(T[1] ^ rotr_32(T[1] ^ rotr_32(T[1], 14), 5), 6)
                .wrapping_add(T[3] ^ T[1] & (T[2] ^ T[3]))
                .wrapping_add(SHA256_K[11u32.wrapping_add(j) as usize])
                .wrapping_add(W[11]),
        );
        T[0] = T[0].wrapping_add(T[4]);
        T[4] = T[4].wrapping_add(
            rotr_32(T[5] ^ rotr_32(T[5] ^ rotr_32(T[5], 9), 11), 2)
                .wrapping_add((T[5] & (T[6] ^ T[7])).wrapping_add(T[6] & T[7])),
        );
        W[12] = W[12].wrapping_add(
            (rotr_32(W[10] ^ rotr_32(W[10], 2), 17) ^ W[10] >> 10)
                .wrapping_add(W[5])
                .wrapping_add(rotr_32(W[13] ^ rotr_32(W[13], 11), 7) ^ W[13] >> 3),
        );
        T[3] = T[3].wrapping_add(
            rotr_32(T[0] ^ rotr_32(T[0] ^ rotr_32(T[0], 14), 5), 6)
                .wrapping_add(T[2] ^ T[0] & (T[1] ^ T[2]))
                .wrapping_add(SHA256_K[12u32.wrapping_add(j) as usize])
                .wrapping_add(W[12]),
        );
        T[7] = T[7].wrapping_add(T[3]);
        T[3] = T[3].wrapping_add(
            rotr_32(T[4] ^ rotr_32(T[4] ^ rotr_32(T[4], 9), 11), 2)
                .wrapping_add((T[4] & (T[5] ^ T[6])).wrapping_add(T[5] & T[6])),
        );
        W[13] = W[13].wrapping_add(
            (rotr_32(W[11] ^ rotr_32(W[11], 2), 17) ^ W[11] >> 10)
                .wrapping_add(W[6])
                .wrapping_add(rotr_32(W[14] ^ rotr_32(W[14], 11), 7) ^ W[14] >> 3),
        );
        T[2] = T[2].wrapping_add(
            rotr_32(T[7] ^ rotr_32(T[7] ^ rotr_32(T[7], 14), 5), 6)
                .wrapping_add(T[1] ^ T[7] & (T[0] ^ T[1]))
                .wrapping_add(SHA256_K[13u32.wrapping_add(j) as usize])
                .wrapping_add(W[13]),
        );
        T[6] = T[6].wrapping_add(T[2]);
        T[2] = T[2].wrapping_add(
            rotr_32(T[3] ^ rotr_32(T[3] ^ rotr_32(T[3], 9), 11), 2)
                .wrapping_add((T[3] & (T[4] ^ T[5])).wrapping_add(T[4] & T[5])),
        );
        W[14] = W[14].wrapping_add(
            (rotr_32(W[12] ^ rotr_32(W[12], 2), 17) ^ W[12] >> 10)
                .wrapping_add(W[7])
                .wrapping_add(rotr_32(W[15] ^ rotr_32(W[15], 11), 7) ^ W[15] >> 3),
        );
        T[1] = T[1].wrapping_add(
            rotr_32(T[6] ^ rotr_32(T[6] ^ rotr_32(T[6], 14), 5), 6)
                .wrapping_add(T[0] ^ T[6] & (T[7] ^ T[0]))
                .wrapping_add(SHA256_K[14u32.wrapping_add(j) as usize])
                .wrapping_add(W[14]),
        );
        T[5] = T[5].wrapping_add(T[1]);
        T[1] = T[1].wrapping_add(
            rotr_32(T[2] ^ rotr_32(T[2] ^ rotr_32(T[2], 9), 11), 2)
                .wrapping_add((T[2] & (T[3] ^ T[4])).wrapping_add(T[3] & T[4])),
        );
        W[15] = W[15].wrapping_add(
            (rotr_32(W[13] ^ rotr_32(W[13], 2), 17) ^ W[13] >> 10)
                .wrapping_add(W[8])
                .wrapping_add(rotr_32(W[0] ^ rotr_32(W[0], 11), 7) ^ W[0] >> 3),
        );
        T[0] = T[0].wrapping_add(
            rotr_32(T[5] ^ rotr_32(T[5] ^ rotr_32(T[5], 14), 5), 6)
                .wrapping_add(T[7] ^ T[5] & (T[6] ^ T[7]))
                .wrapping_add(SHA256_K[15u32.wrapping_add(j) as usize])
                .wrapping_add(W[15]),
        );
        T[4] = T[4].wrapping_add(T[0]);
        T[0] = T[0].wrapping_add(
            rotr_32(T[1] ^ rotr_32(T[1] ^ rotr_32(T[1], 9), 11), 2)
                .wrapping_add((T[1] & (T[2] ^ T[3])).wrapping_add(T[2] & T[3])),
        );
        j = j.wrapping_add(16);
    }
    let ref mut fresh0 = *state.offset(0);
    *fresh0 = (*fresh0).wrapping_add(T[0]);
    let ref mut fresh1 = *state.offset(1);
    *fresh1 = (*fresh1).wrapping_add(T[1]);
    let ref mut fresh2 = *state.offset(2);
    *fresh2 = (*fresh2).wrapping_add(T[2]);
    let ref mut fresh3 = *state.offset(3);
    *fresh3 = (*fresh3).wrapping_add(T[3]);
    let ref mut fresh4 = *state.offset(4);
    *fresh4 = (*fresh4).wrapping_add(T[4]);
    let ref mut fresh5 = *state.offset(5);
    *fresh5 = (*fresh5).wrapping_add(T[5]);
    let ref mut fresh6 = *state.offset(6);
    *fresh6 = (*fresh6).wrapping_add(T[6]);
    let ref mut fresh7 = *state.offset(7);
    *fresh7 = (*fresh7).wrapping_add(T[7]);
}
unsafe extern "C" fn process(check: *mut lzma_check_state) {
    transform(
        &raw mut (*check).state.sha256.state as *mut u32,
        &raw mut (*check).buffer.u32_0 as *mut u32 as *const u32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_sha256_init(check: *mut lzma_check_state) {
    static mut s: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    memcpy(
        &raw mut (*check).state.sha256.state as *mut u32 as *mut c_void,
        &raw const s as *const u32 as *const c_void,
        core::mem::size_of::<[u32; 8]>() as size_t,
    );
    (*check).state.sha256.size = 0;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_sha256_update(
    mut buf: *const u8,
    mut size: size_t,
    check: *mut lzma_check_state,
) {
    while size > 0 {
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
        if (*check).state.sha256.size & 0x3f as u64 == 0 {
            process(check);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_sha256_finish(check: *mut lzma_check_state) {
    let mut pos: size_t = ((*check).state.sha256.size & 0x3f as u64) as size_t;
    let fresh8 = pos;
    pos = pos.wrapping_add(1);
    (*check).buffer.u8_0[fresh8 as usize] = 0x80 as u8;
    while pos != (64 as c_int - 8 as c_int) as size_t {
        if pos == 64 {
            process(check);
            pos = 0;
        }
        let fresh9 = pos;
        pos = pos.wrapping_add(1);
        (*check).buffer.u8_0[fresh9 as usize] = 0;
    }
    (*check).state.sha256.size = (*check).state.sha256.size.wrapping_mul(8);
    (*check).buffer.u64_0[7] = ((*check).state.sha256.size & 0xff as u64) << 56
        | ((*check).state.sha256.size & 0xff00 as u64) << 40
        | ((*check).state.sha256.size & 0xff0000 as u64) << 24
        | ((*check).state.sha256.size & 0xff000000 as u64) << 8
        | ((*check).state.sha256.size & 0xff00000000 as u64) >> 8
        | ((*check).state.sha256.size & 0xff0000000000 as u64) >> 24
        | ((*check).state.sha256.size & 0xff000000000000 as u64) >> 40
        | ((*check).state.sha256.size & 0xff00000000000000 as u64) >> 56;
    process(check);
    let mut i: size_t = 0;
    while i < 8 {
        (*check).buffer.u32_0[i as usize] = ((*check).state.sha256.state[i as usize] & 0xff) << 24
            | ((*check).state.sha256.state[i as usize] & 0xff00) << 8
            | ((*check).state.sha256.state[i as usize] & 0xff0000) >> 8
            | ((*check).state.sha256.state[i as usize] & 0xff000000) >> 24;
        i = i.wrapping_add(1);
    }
}
