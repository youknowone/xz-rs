extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_sha256_state {
    pub state: [uint32_t; 8],
    pub size: uint64_t,
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
    pub crc32: uint32_t,
    pub crc64: uint64_t,
    pub sha256: lzma_sha256_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub u8_0: [uint8_t; 64],
    pub u32_0: [uint32_t; 16],
    pub u64_0: [uint64_t; 8],
}
#[inline]
unsafe extern "C" fn rotr_32(mut num: uint32_t, mut amount: ::core::ffi::c_uint) -> uint32_t {
    return num >> amount | num << (32 as ::core::ffi::c_uint).wrapping_sub(amount);
}
static mut SHA256_K: [uint32_t; 64] = [
    0x428a2f98 as ::core::ffi::c_int as uint32_t,
    0x71374491 as ::core::ffi::c_int as uint32_t,
    0xb5c0fbcf as ::core::ffi::c_uint,
    0xe9b5dba5 as ::core::ffi::c_uint,
    0x3956c25b as ::core::ffi::c_int as uint32_t,
    0x59f111f1 as ::core::ffi::c_int as uint32_t,
    0x923f82a4 as ::core::ffi::c_uint,
    0xab1c5ed5 as ::core::ffi::c_uint,
    0xd807aa98 as ::core::ffi::c_uint,
    0x12835b01 as ::core::ffi::c_int as uint32_t,
    0x243185be as ::core::ffi::c_int as uint32_t,
    0x550c7dc3 as ::core::ffi::c_int as uint32_t,
    0x72be5d74 as ::core::ffi::c_int as uint32_t,
    0x80deb1fe as ::core::ffi::c_uint,
    0x9bdc06a7 as ::core::ffi::c_uint,
    0xc19bf174 as ::core::ffi::c_uint,
    0xe49b69c1 as ::core::ffi::c_uint,
    0xefbe4786 as ::core::ffi::c_uint,
    0xfc19dc6 as ::core::ffi::c_int as uint32_t,
    0x240ca1cc as ::core::ffi::c_int as uint32_t,
    0x2de92c6f as ::core::ffi::c_int as uint32_t,
    0x4a7484aa as ::core::ffi::c_int as uint32_t,
    0x5cb0a9dc as ::core::ffi::c_int as uint32_t,
    0x76f988da as ::core::ffi::c_int as uint32_t,
    0x983e5152 as ::core::ffi::c_uint,
    0xa831c66d as ::core::ffi::c_uint,
    0xb00327c8 as ::core::ffi::c_uint,
    0xbf597fc7 as ::core::ffi::c_uint,
    0xc6e00bf3 as ::core::ffi::c_uint,
    0xd5a79147 as ::core::ffi::c_uint,
    0x6ca6351 as ::core::ffi::c_int as uint32_t,
    0x14292967 as ::core::ffi::c_int as uint32_t,
    0x27b70a85 as ::core::ffi::c_int as uint32_t,
    0x2e1b2138 as ::core::ffi::c_int as uint32_t,
    0x4d2c6dfc as ::core::ffi::c_int as uint32_t,
    0x53380d13 as ::core::ffi::c_int as uint32_t,
    0x650a7354 as ::core::ffi::c_int as uint32_t,
    0x766a0abb as ::core::ffi::c_int as uint32_t,
    0x81c2c92e as ::core::ffi::c_uint,
    0x92722c85 as ::core::ffi::c_uint,
    0xa2bfe8a1 as ::core::ffi::c_uint,
    0xa81a664b as ::core::ffi::c_uint,
    0xc24b8b70 as ::core::ffi::c_uint,
    0xc76c51a3 as ::core::ffi::c_uint,
    0xd192e819 as ::core::ffi::c_uint,
    0xd6990624 as ::core::ffi::c_uint,
    0xf40e3585 as ::core::ffi::c_uint,
    0x106aa070 as ::core::ffi::c_int as uint32_t,
    0x19a4c116 as ::core::ffi::c_int as uint32_t,
    0x1e376c08 as ::core::ffi::c_int as uint32_t,
    0x2748774c as ::core::ffi::c_int as uint32_t,
    0x34b0bcb5 as ::core::ffi::c_int as uint32_t,
    0x391c0cb3 as ::core::ffi::c_int as uint32_t,
    0x4ed8aa4a as ::core::ffi::c_int as uint32_t,
    0x5b9cca4f as ::core::ffi::c_int as uint32_t,
    0x682e6ff3 as ::core::ffi::c_int as uint32_t,
    0x748f82ee as ::core::ffi::c_int as uint32_t,
    0x78a5636f as ::core::ffi::c_int as uint32_t,
    0x84c87814 as ::core::ffi::c_uint,
    0x8cc70208 as ::core::ffi::c_uint,
    0x90befffa as ::core::ffi::c_uint,
    0xa4506ceb as ::core::ffi::c_uint,
    0xbef9a3f7 as ::core::ffi::c_uint,
    0xc67178f2 as ::core::ffi::c_uint,
];
unsafe extern "C" fn transform(mut state: *mut uint32_t, mut data: *const uint32_t) {
    let mut W: [uint32_t; 16] = [0; 16];
    let mut T: [uint32_t; 8] = [0; 8];
    memcpy(
        &raw mut T as *mut uint32_t as *mut ::core::ffi::c_void,
        state as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint32_t; 8]>() as size_t,
    );
    W[0 as ::core::ffi::c_int as usize] = (*data.offset(0 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(0 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(0 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(0 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[0 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[1 as ::core::ffi::c_int as usize] = (*data.offset(1 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(1 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(1 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(1 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[1 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[2 as ::core::ffi::c_int as usize] = (*data.offset(2 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(2 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(2 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(2 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[2 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[3 as ::core::ffi::c_int as usize] = (*data.offset(3 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(3 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(3 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(3 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[3 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[4 as ::core::ffi::c_int as usize] = (*data.offset(4 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(4 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(4 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(4 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[4 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[5 as ::core::ffi::c_int as usize] = (*data.offset(5 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(5 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(5 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(5 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[5 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[6 as ::core::ffi::c_int as usize] = (*data.offset(6 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(6 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(6 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(6 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[6 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[7 as ::core::ffi::c_int as usize] = (*data.offset(7 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(7 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(7 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(7 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[7 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[8 as ::core::ffi::c_int as usize] = (*data.offset(8 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(8 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(8 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(8 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[8 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[9 as ::core::ffi::c_int as usize] = (*data.offset(9 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(9 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(9 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(9 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[9 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[10 as ::core::ffi::c_int as usize] = (*data.offset(10 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(10 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(10 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(10 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(10 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[10 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[11 as ::core::ffi::c_int as usize] = (*data.offset(11 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(11 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(11 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(11 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(11 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[11 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[12 as ::core::ffi::c_int as usize] = (*data.offset(12 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(12 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(12 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(12 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[12 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[13 as ::core::ffi::c_int as usize] = (*data.offset(13 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(13 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(13 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(13 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(13 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[13 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[14 as ::core::ffi::c_int as usize] = (*data.offset(14 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(14 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(14 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(14 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(14 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[14 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    W[15 as ::core::ffi::c_int as usize] = (*data.offset(15 as ::core::ffi::c_int as isize)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*data.offset(15 as ::core::ffi::c_int as isize) & 0xff00 as uint32_t)
            << 8 as ::core::ffi::c_int
        | (*data.offset(15 as ::core::ffi::c_int as isize) & 0xff0000 as uint32_t)
            >> 8 as ::core::ffi::c_int
        | (*data.offset(15 as ::core::ffi::c_int as isize) & 0xff000000 as uint32_t)
            >> 24 as ::core::ffi::c_int;
    T[(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(4 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(4 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                14 as ::core::ffi::c_uint,
                            ),
                        5 as ::core::ffi::c_uint,
                    ),
                6 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                T[(6 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ T[(4 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(5 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(6 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]),
            )
            .wrapping_add(SHA256_K[(15 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize])
            .wrapping_add(W[15 as ::core::ffi::c_int as usize]),
        );
    T[(3 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(3 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            T[(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize],
        );
    T[(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] = T
        [(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
        .wrapping_add(
            rotr_32(
                T[(0 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    ^ rotr_32(
                        T[(0 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                                9 as ::core::ffi::c_uint,
                            ),
                        11 as ::core::ffi::c_uint,
                    ),
                2 as ::core::ffi::c_uint,
            )
            .wrapping_add(
                (T[(0 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize]
                    & (T[(1 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        ^ T[(2 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]))
                    .wrapping_add(
                        T[(1 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & T[(2 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize],
                    ),
            ),
        );
    let mut j: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
    while j < 64 as ::core::ffi::c_uint {
        W[(0 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(0 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(0 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            W[(0 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(0 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(0 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(0 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(0 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(0 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(4 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(4 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        14 as ::core::ffi::c_uint,
                                    ),
                                5 as ::core::ffi::c_uint,
                            ),
                        6 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        T[(6 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(4 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                & (T[(5 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ T[(6 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize]),
                    )
                    .wrapping_add(SHA256_K[(0 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                    .wrapping_add(W[(0 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
                );
        T[(3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    T[(7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize],
                );
        T[(7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(0 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(0 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        9 as ::core::ffi::c_uint,
                                    ),
                                11 as ::core::ffi::c_uint,
                            ),
                        2 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (T[(0 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(1 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(2 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]))
                            .wrapping_add(
                                T[(1 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    & T[(2 as ::core::ffi::c_int - 0 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                            ),
                    ),
                );
        W[(1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(1 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            W[(1 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(1 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(1 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(1 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(1 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(1 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        14 as ::core::ffi::c_uint,
                                    ),
                                5 as ::core::ffi::c_uint,
                            ),
                        6 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        T[(6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                & (T[(5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ T[(6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize]),
                    )
                    .wrapping_add(SHA256_K[(1 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                    .wrapping_add(W[(1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
                );
        T[(3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    T[(7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize],
                );
        T[(7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        9 as ::core::ffi::c_uint,
                                    ),
                                11 as ::core::ffi::c_uint,
                            ),
                        2 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (T[(0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]))
                            .wrapping_add(
                                T[(1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    & T[(2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                            ),
                    ),
                );
        W[(2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            W[(2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(2 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(2 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(2 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(2 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        14 as ::core::ffi::c_uint,
                                    ),
                                5 as ::core::ffi::c_uint,
                            ),
                        6 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        T[(6 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                & (T[(5 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ T[(6 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize]),
                    )
                    .wrapping_add(SHA256_K[(2 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                    .wrapping_add(W[(2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
                );
        T[(3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    T[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize],
                );
        T[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(0 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(0 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        9 as ::core::ffi::c_uint,
                                    ),
                                11 as ::core::ffi::c_uint,
                            ),
                        2 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (T[(0 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(1 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]))
                            .wrapping_add(
                                T[(1 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    & T[(2 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                            ),
                    ),
                );
        W[(3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            W[(3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(3 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(3 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(3 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(3 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(4 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(4 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        14 as ::core::ffi::c_uint,
                                    ),
                                5 as ::core::ffi::c_uint,
                            ),
                        6 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        T[(6 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(4 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                & (T[(5 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ T[(6 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize]),
                    )
                    .wrapping_add(SHA256_K[(3 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                    .wrapping_add(W[(3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
                );
        T[(3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    T[(7 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize],
                );
        T[(7 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(0 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(0 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        9 as ::core::ffi::c_uint,
                                    ),
                                11 as ::core::ffi::c_uint,
                            ),
                        2 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (T[(0 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(1 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(2 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]))
                            .wrapping_add(
                                T[(1 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    & T[(2 as ::core::ffi::c_int - 3 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                            ),
                    ),
                );
        W[(4 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(4 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            W[(4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(4 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(4 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(4 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(4 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(4 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(4 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        14 as ::core::ffi::c_uint,
                                    ),
                                5 as ::core::ffi::c_uint,
                            ),
                        6 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        T[(6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(4 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                & (T[(5 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ T[(6 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize]),
                    )
                    .wrapping_add(SHA256_K[(4 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                    .wrapping_add(W[(4 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
                );
        T[(3 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(3 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    T[(7 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize],
                );
        T[(7 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 4 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(0 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(0 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        9 as ::core::ffi::c_uint,
                                    ),
                                11 as ::core::ffi::c_uint,
                            ),
                        2 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (T[(0 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(1 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(2 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]))
                            .wrapping_add(
                                T[(1 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    & T[(2 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                            ),
                    ),
                );
        W[(5 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(5 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(5 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            W[(5 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(5 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(5 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(5 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(5 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(5 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(4 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(4 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        14 as ::core::ffi::c_uint,
                                    ),
                                5 as ::core::ffi::c_uint,
                            ),
                        6 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        T[(6 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(4 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                & (T[(5 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ T[(6 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize]),
                    )
                    .wrapping_add(SHA256_K[(5 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                    .wrapping_add(W[(5 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
                );
        T[(3 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(3 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    T[(7 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize],
                );
        T[(7 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 5 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(0 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(0 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        9 as ::core::ffi::c_uint,
                                    ),
                                11 as ::core::ffi::c_uint,
                            ),
                        2 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (T[(0 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(1 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(2 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]))
                            .wrapping_add(
                                T[(1 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    & T[(2 as ::core::ffi::c_int - 5 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                            ),
                    ),
                );
        W[(6 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(6 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(6 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            W[(6 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(6 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(6 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(6 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(6 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(6 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(4 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(4 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        14 as ::core::ffi::c_uint,
                                    ),
                                5 as ::core::ffi::c_uint,
                            ),
                        6 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        T[(6 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(4 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                & (T[(5 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ T[(6 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize]),
                    )
                    .wrapping_add(SHA256_K[(6 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                    .wrapping_add(W[(6 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
                );
        T[(3 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(3 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    T[(7 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize],
                );
        T[(7 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(0 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(0 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        9 as ::core::ffi::c_uint,
                                    ),
                                11 as ::core::ffi::c_uint,
                            ),
                        2 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (T[(0 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(1 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(2 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]))
                            .wrapping_add(
                                T[(1 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    & T[(2 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                            ),
                    ),
                );
        W[(7 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(7 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            W[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(7 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(4 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(4 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        14 as ::core::ffi::c_uint,
                                    ),
                                5 as ::core::ffi::c_uint,
                            ),
                        6 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        T[(6 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(4 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                & (T[(5 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ T[(6 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize]),
                    )
                    .wrapping_add(SHA256_K[(7 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                    .wrapping_add(W[(7 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
                );
        T[(3 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(3 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    T[(7 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize],
                );
        T[(7 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 7 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(0 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(0 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        9 as ::core::ffi::c_uint,
                                    ),
                                11 as ::core::ffi::c_uint,
                            ),
                        2 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (T[(0 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(1 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(2 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]))
                            .wrapping_add(
                                T[(1 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    & T[(2 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                            ),
                    ),
                );
        W[(8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(8 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            W[(8 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(8 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(8 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(8 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(8 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(8 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(4 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(4 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        14 as ::core::ffi::c_uint,
                                    ),
                                5 as ::core::ffi::c_uint,
                            ),
                        6 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        T[(6 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(4 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                & (T[(5 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ T[(6 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize]),
                    )
                    .wrapping_add(SHA256_K[(8 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                    .wrapping_add(W[(8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
                );
        T[(3 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(3 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    T[(7 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize],
                );
        T[(7 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(0 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(0 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        9 as ::core::ffi::c_uint,
                                    ),
                                11 as ::core::ffi::c_uint,
                            ),
                        2 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (T[(0 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(1 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(2 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]))
                            .wrapping_add(
                                T[(1 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    & T[(2 as ::core::ffi::c_int - 8 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                            ),
                    ),
                );
        W[(9 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(9 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(9 as ::core::ffi::c_int - 2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            W[(9 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(9 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(9 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(9 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(9 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(9 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(4 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(4 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(4 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        14 as ::core::ffi::c_uint,
                                    ),
                                5 as ::core::ffi::c_uint,
                            ),
                        6 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        T[(6 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(4 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                & (T[(5 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ T[(6 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize]),
                    )
                    .wrapping_add(SHA256_K[(9 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                    .wrapping_add(W[(9 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
                );
        T[(3 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(3 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    T[(7 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize],
                );
        T[(7 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize] =
            T[(7 as ::core::ffi::c_int - 9 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                as usize]
                .wrapping_add(
                    rotr_32(
                        T[(0 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ rotr_32(
                                T[(0 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    ^ rotr_32(
                                        T[(0 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                            & 7 as ::core::ffi::c_int)
                                            as usize],
                                        9 as ::core::ffi::c_uint,
                                    ),
                                11 as ::core::ffi::c_uint,
                            ),
                        2 as ::core::ffi::c_uint,
                    )
                    .wrapping_add(
                        (T[(0 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(1 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(2 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]))
                            .wrapping_add(
                                T[(1 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]
                                    & T[(2 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                            ),
                    ),
                );
        W[(10 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(10 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(10 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                        & 15 as ::core::ffi::c_int) as usize]
                        ^ rotr_32(
                            W[(10 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(10 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(10 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(10 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(10 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(10 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(4 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(4 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(4 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    14 as ::core::ffi::c_uint,
                                ),
                            5 as ::core::ffi::c_uint,
                        ),
                    6 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    T[(6 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ T[(4 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(5 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(6 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]),
                )
                .wrapping_add(SHA256_K[(10 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(10 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
            );
        T[(3 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(3 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                T[(7 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize],
            );
        T[(7 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(0 as ::core::ffi::c_int - 10 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(0 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(0 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    9 as ::core::ffi::c_uint,
                                ),
                            11 as ::core::ffi::c_uint,
                        ),
                    2 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    (T[(0 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(1 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(2 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]))
                        .wrapping_add(
                            T[(1 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                & T[(2 as ::core::ffi::c_int - 10 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                        ),
                ),
            );
        W[(11 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(11 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(11 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                        & 15 as ::core::ffi::c_int) as usize]
                        ^ rotr_32(
                            W[(11 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(11 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(11 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(11 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(11 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(11 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(4 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(4 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(4 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    14 as ::core::ffi::c_uint,
                                ),
                            5 as ::core::ffi::c_uint,
                        ),
                    6 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    T[(6 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ T[(4 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(5 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(6 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]),
                )
                .wrapping_add(SHA256_K[(11 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(11 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
            );
        T[(3 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(3 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                T[(7 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize],
            );
        T[(7 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(0 as ::core::ffi::c_int - 11 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(0 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(0 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    9 as ::core::ffi::c_uint,
                                ),
                            11 as ::core::ffi::c_uint,
                        ),
                    2 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    (T[(0 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(1 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(2 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]))
                        .wrapping_add(
                            T[(1 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                & T[(2 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                        ),
                ),
            );
        W[(12 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(12 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(12 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                        & 15 as ::core::ffi::c_int) as usize]
                        ^ rotr_32(
                            W[(12 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(12 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(12 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(12 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(12 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(12 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(4 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(4 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(4 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    14 as ::core::ffi::c_uint,
                                ),
                            5 as ::core::ffi::c_uint,
                        ),
                    6 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    T[(6 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ T[(4 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(5 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(6 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]),
                )
                .wrapping_add(SHA256_K[(12 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(12 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
            );
        T[(3 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(3 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                T[(7 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize],
            );
        T[(7 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(0 as ::core::ffi::c_int - 12 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(0 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(0 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    9 as ::core::ffi::c_uint,
                                ),
                            11 as ::core::ffi::c_uint,
                        ),
                    2 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    (T[(0 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(1 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(2 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]))
                        .wrapping_add(
                            T[(1 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                & T[(2 as ::core::ffi::c_int - 12 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                        ),
                ),
            );
        W[(13 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(13 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                        & 15 as ::core::ffi::c_int) as usize]
                        ^ rotr_32(
                            W[(13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(13 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(13 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(13 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(13 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(13 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(4 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(4 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(4 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    14 as ::core::ffi::c_uint,
                                ),
                            5 as ::core::ffi::c_uint,
                        ),
                    6 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    T[(6 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ T[(4 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(5 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(6 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]),
                )
                .wrapping_add(SHA256_K[(13 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(13 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
            );
        T[(3 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(3 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                T[(7 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize],
            );
        T[(7 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(0 as ::core::ffi::c_int - 13 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(0 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(0 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    9 as ::core::ffi::c_uint,
                                ),
                            11 as ::core::ffi::c_uint,
                        ),
                    2 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    (T[(0 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(1 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(2 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]))
                        .wrapping_add(
                            T[(1 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                & T[(2 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                        ),
                ),
            );
        W[(14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                        & 15 as ::core::ffi::c_int) as usize]
                        ^ rotr_32(
                            W[(14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(14 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(14 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(14 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(14 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(4 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(4 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(4 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    14 as ::core::ffi::c_uint,
                                ),
                            5 as ::core::ffi::c_uint,
                        ),
                    6 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    T[(6 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ T[(4 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(5 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(6 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]),
                )
                .wrapping_add(SHA256_K[(14 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
            );
        T[(3 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(3 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                T[(7 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize],
            );
        T[(7 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(0 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(0 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(0 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    9 as ::core::ffi::c_uint,
                                ),
                            11 as ::core::ffi::c_uint,
                        ),
                    2 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    (T[(0 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(1 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]))
                        .wrapping_add(
                            T[(1 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                & T[(2 as ::core::ffi::c_int - 14 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                        ),
                ),
            );
        W[(15 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize] =
            W[(15 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize].wrapping_add(
                (rotr_32(
                    W[(15 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                        & 15 as ::core::ffi::c_int) as usize]
                        ^ rotr_32(
                            W[(15 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize],
                            2 as ::core::ffi::c_uint,
                        ),
                    17 as ::core::ffi::c_uint,
                ) ^ W[(15 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize]
                    >> 10 as ::core::ffi::c_int)
                    .wrapping_add(
                        W[(15 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize],
                    )
                    .wrapping_add(
                        rotr_32(
                            W[(15 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 15 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    W[(15 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 15 as ::core::ffi::c_int)
                                        as usize],
                                    11 as ::core::ffi::c_uint,
                                ),
                            7 as ::core::ffi::c_uint,
                        ) ^ W[(15 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 15 as ::core::ffi::c_int) as usize]
                            >> 3 as ::core::ffi::c_int,
                    ),
            );
        T[(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(4 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(4 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(4 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    14 as ::core::ffi::c_uint,
                                ),
                            5 as ::core::ffi::c_uint,
                        ),
                    6 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    T[(6 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ T[(4 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            & (T[(5 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]
                                ^ T[(6 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize]),
                )
                .wrapping_add(SHA256_K[(15 as ::core::ffi::c_uint).wrapping_add(j) as usize])
                .wrapping_add(W[(15 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as usize]),
            );
        T[(3 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(3 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                T[(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                    as usize],
            );
        T[(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as usize] = T[(7 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
            & 7 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                rotr_32(
                    T[(0 as ::core::ffi::c_int - 15 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as usize]
                        ^ rotr_32(
                            T[(0 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                ^ rotr_32(
                                    T[(0 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                        & 7 as ::core::ffi::c_int)
                                        as usize],
                                    9 as ::core::ffi::c_uint,
                                ),
                            11 as ::core::ffi::c_uint,
                        ),
                    2 as ::core::ffi::c_uint,
                )
                .wrapping_add(
                    (T[(0 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                        & 7 as ::core::ffi::c_int) as usize]
                        & (T[(1 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as usize]
                            ^ T[(2 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int)
                                as usize]))
                        .wrapping_add(
                            T[(1 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                & 7 as ::core::ffi::c_int) as usize]
                                & T[(2 as ::core::ffi::c_int - 15 as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    as usize],
                        ),
                ),
            );
        j = j.wrapping_add(16 as ::core::ffi::c_uint);
    }
    let ref mut fresh0 = *state.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = (*fresh0).wrapping_add(
        T[(0 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize],
    );
    let ref mut fresh1 = *state.offset(1 as ::core::ffi::c_int as isize);
    *fresh1 = (*fresh1).wrapping_add(
        T[(1 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize],
    );
    let ref mut fresh2 = *state.offset(2 as ::core::ffi::c_int as isize);
    *fresh2 = (*fresh2).wrapping_add(
        T[(2 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize],
    );
    let ref mut fresh3 = *state.offset(3 as ::core::ffi::c_int as isize);
    *fresh3 = (*fresh3).wrapping_add(
        T[(3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize],
    );
    let ref mut fresh4 = *state.offset(4 as ::core::ffi::c_int as isize);
    *fresh4 = (*fresh4).wrapping_add(
        T[(4 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize],
    );
    let ref mut fresh5 = *state.offset(5 as ::core::ffi::c_int as isize);
    *fresh5 = (*fresh5).wrapping_add(
        T[(5 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize],
    );
    let ref mut fresh6 = *state.offset(6 as ::core::ffi::c_int as isize);
    *fresh6 = (*fresh6).wrapping_add(
        T[(6 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize],
    );
    let ref mut fresh7 = *state.offset(7 as ::core::ffi::c_int as isize);
    *fresh7 = (*fresh7).wrapping_add(
        T[(7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize],
    );
}
unsafe extern "C" fn process(mut check: *mut lzma_check_state) {
    transform(
        &raw mut (*check).state.sha256.state as *mut uint32_t,
        &raw mut (*check).buffer.u32_0 as *mut uint32_t as *const uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_sha256_init(mut check: *mut lzma_check_state) {
    static mut s: [uint32_t; 8] = [
        0x6a09e667 as ::core::ffi::c_int as uint32_t,
        0xbb67ae85 as ::core::ffi::c_uint,
        0x3c6ef372 as ::core::ffi::c_int as uint32_t,
        0xa54ff53a as ::core::ffi::c_uint,
        0x510e527f as ::core::ffi::c_int as uint32_t,
        0x9b05688c as ::core::ffi::c_uint,
        0x1f83d9ab as ::core::ffi::c_int as uint32_t,
        0x5be0cd19 as ::core::ffi::c_int as uint32_t,
    ];
    memcpy(
        &raw mut (*check).state.sha256.state as *mut uint32_t as *mut ::core::ffi::c_void,
        &raw const s as *const uint32_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint32_t; 8]>() as size_t,
    );
    (*check).state.sha256.size = 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_sha256_update(
    mut buf: *const uint8_t,
    mut size: size_t,
    mut check: *mut lzma_check_state,
) {
    while size > 0 as size_t {
        let copy_start: size_t = ((*check).state.sha256.size & 0x3f as uint64_t) as size_t;
        let mut copy_size: size_t = (64 as size_t).wrapping_sub(copy_start);
        if copy_size > size {
            copy_size = size;
        }
        memcpy(
            (&raw mut (*check).buffer.u8_0 as *mut uint8_t).offset(copy_start as isize)
                as *mut ::core::ffi::c_void,
            buf as *const ::core::ffi::c_void,
            copy_size,
        );
        buf = buf.offset(copy_size as isize);
        size = size.wrapping_sub(copy_size);
        (*check).state.sha256.size = (*check)
            .state
            .sha256
            .size
            .wrapping_add(copy_size as uint64_t);
        if (*check).state.sha256.size & 0x3f as uint64_t == 0 as uint64_t {
            process(check);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_sha256_finish(mut check: *mut lzma_check_state) {
    let mut pos: size_t = ((*check).state.sha256.size & 0x3f as uint64_t) as size_t;
    let fresh8 = pos;
    pos = pos.wrapping_add(1);
    (*check).buffer.u8_0[fresh8 as usize] = 0x80 as uint8_t;
    while pos != (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as size_t {
        if pos == 64 as size_t {
            process(check);
            pos = 0 as size_t;
        }
        let fresh9 = pos;
        pos = pos.wrapping_add(1);
        (*check).buffer.u8_0[fresh9 as usize] = 0 as uint8_t;
    }
    (*check).state.sha256.size = (*check).state.sha256.size.wrapping_mul(8 as uint64_t);
    (*check).buffer.u64_0[((64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
        / 8 as ::core::ffi::c_int) as usize] = ((*check).state.sha256.size & 0xff as uint64_t)
        << 56 as ::core::ffi::c_int
        | ((*check).state.sha256.size & 0xff00 as uint64_t) << 40 as ::core::ffi::c_int
        | ((*check).state.sha256.size & 0xff0000 as uint64_t) << 24 as ::core::ffi::c_int
        | ((*check).state.sha256.size & 0xff000000 as uint64_t) << 8 as ::core::ffi::c_int
        | ((*check).state.sha256.size & 0xff00000000 as uint64_t) >> 8 as ::core::ffi::c_int
        | ((*check).state.sha256.size & 0xff0000000000 as uint64_t) >> 24 as ::core::ffi::c_int
        | ((*check).state.sha256.size & 0xff000000000000 as uint64_t) >> 40 as ::core::ffi::c_int
        | ((*check).state.sha256.size & 0xff00000000000000 as uint64_t) >> 56 as ::core::ffi::c_int;
    process(check);
    let mut i: size_t = 0 as size_t;
    while i < 8 as size_t {
        (*check).buffer.u32_0[i as usize] = ((*check).state.sha256.state[i as usize]
            & 0xff as uint32_t)
            << 24 as ::core::ffi::c_int
            | ((*check).state.sha256.state[i as usize] & 0xff00 as uint32_t)
                << 8 as ::core::ffi::c_int
            | ((*check).state.sha256.state[i as usize] & 0xff0000 as uint32_t)
                >> 8 as ::core::ffi::c_int
            | ((*check).state.sha256.state[i as usize] & 0xff000000 as uint32_t)
                >> 24 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
}
