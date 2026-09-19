//! CRC32 and CRC64 implementations using x86 CLMUL instructions
//! (crc_x86_clmul.h, transpiled via c2rust and cleaned up).

use crate::types::size_t;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[inline]
pub fn is_arch_extension_supported() -> bool {
    std::arch::is_x86_feature_detected!("ssse3")
        && std::arch::is_x86_feature_detected!("sse4.1")
        && std::arch::is_x86_feature_detected!("pclmulqdq")
}

#[inline(always)]
unsafe fn read16le(buf: *const u8) -> u16 {
    u16::from_le(core::ptr::read_unaligned(buf as *const u16))
}

#[inline(always)]
unsafe fn read32le(buf: *const u8) -> u32 {
    u32::from_le(core::ptr::read_unaligned(buf as *const u32))
}

#[inline(always)]
unsafe fn read64le(buf: *const u8) -> u64 {
    u64::from_le(core::ptr::read_unaligned(buf as *const u64))
}

#[cfg(target_arch = "x86")]
#[inline(always)]
unsafe fn my_set_low64(a: i64) -> __m128i {
    _mm_set_epi64x(0, a)
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn my_set_low64(a: i64) -> __m128i {
    _mm_cvtsi64_si128(a)
}

// Aligned so that the whole array is within the same cache line:
// more than one unaligned load can be done from this during one call.
// The bytes [0..32] are used with AND to clear the low bytes; the bytes
// [16..48] are for left shifts and [32..64] for right shifts.
#[repr(align(64))]
struct VMasks([u8; 64]);
static vmasks: VMasks = VMasks([
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
]);

// *Unaligned* 128-bit load
#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
#[inline]
unsafe fn my_load128(p: *const u8) -> __m128i {
    _mm_loadu_si128(p as *const __m128i)
}

// Keep the highest "count" bytes as is and clear the remaining low bytes.
#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
#[inline]
unsafe fn keep_high_bytes(v: __m128i, count: size_t) -> __m128i {
    // my_load128 reads 16 bytes, so the offset must leave that much room
    // in the 64-byte table.
    debug_assert!(count <= 48);
    _mm_and_si128(my_load128(vmasks.0.as_ptr().add(count)), v)
}

// Shift the 128-bit value left by "amount" bytes (not bits).
#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
#[inline]
unsafe fn shift_left(v: __m128i, amount: size_t) -> __m128i {
    // 32 - amount is an unsigned subtraction, so an amount above 32 would
    // wrap into a huge offset rather than a small one.
    debug_assert!(amount <= 16);
    _mm_shuffle_epi8(v, my_load128(vmasks.0.as_ptr().add(32 - amount)))
}

// Shift the 128-bit value right by "amount" bytes (not bits).
#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
#[inline]
unsafe fn shift_right(v: __m128i, amount: size_t) -> __m128i {
    debug_assert!(amount <= 16);
    _mm_shuffle_epi8(v, my_load128(vmasks.0.as_ptr().add(32 + amount)))
}

#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
#[inline]
unsafe fn fold(v: __m128i, k: __m128i) -> __m128i {
    let a = _mm_clmulepi64_si128::<0x00>(v, k);
    let b = _mm_clmulepi64_si128::<0x11>(v, k);
    _mm_xor_si128(a, b)
}

#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
#[inline]
unsafe fn fold_xor(v: __m128i, k: __m128i, buf: *const u8) -> __m128i {
    _mm_xor_si128(my_load128(buf), fold(v, k))
}

// Load the last 1-7 input bytes into the high half of v0 for the
// 8..15-byte case; the i386 form mirrors the C #if branches.
#[cfg(target_arch = "x86")]
#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
#[inline]
unsafe fn insert_high64(v0: __m128i, high: u64) -> __m128i {
    let v0 = _mm_insert_epi32::<2>(v0, high as i32);
    _mm_insert_epi32::<3>(v0, (high >> 32) as i32)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
#[inline]
unsafe fn insert_high64(v0: __m128i, high: u64) -> __m128i {
    _mm_insert_epi64::<1>(v0, high as i64)
}

#[cfg(target_arch = "x86")]
#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
#[inline]
unsafe fn extract_crc64(v0: __m128i) -> u64 {
    ((_mm_extract_epi32::<3>(v0) as u32 as u64) << 32) | _mm_extract_epi32::<2>(v0) as u32 as u64
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
#[inline]
unsafe fn extract_crc64(v0: __m128i) -> u64 {
    _mm_extract_epi64::<1>(v0) as u64
}

/// # Safety
///
/// `is_arch_extension_supported()` must have returned true, and `buf`
/// must point to at least `size` readable bytes.
///
/// The unguarded contract comes from crc_x86_clmul.h, where the same
/// function is reachable only through the C feature dispatch and performs
/// no check of its own. Calling it on a CPU without those extensions
/// executes an illegal instruction rather than merely being unsound.
#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
pub unsafe fn crc32_arch_optimized(mut buf: *const u8, mut size: size_t, mut crc: u32) -> u32 {
    if size == 0 {
        return crc;
    }

    // See crc_clmul_consts_gen.c.
    let fold512 = _mm_set_epi64x(0x1d9513d7, 0x8f352d95);
    let fold128 = _mm_set_epi64x(0xccaa009e, 0xae689191);
    let mu_p = _mm_set_epi64x(0xb4e5b025f7011641u64 as i64, 0x1db710640);

    let mut v0: __m128i;
    let mut v1: __m128i;
    let mut v2: __m128i;
    let mut v3: __m128i;

    crc = !crc;

    if size < 8 {
        let mut x: u64 = crc as u64;
        let mut i: u32 = 0;

        if size & 4 != 0 {
            x ^= read32le(buf) as u64;
            buf = buf.add(4);
            i = 32;
        }

        if size & 2 != 0 {
            x ^= (read16le(buf) as u64) << i;
            buf = buf.add(2);
            i += 16;
        }

        if size & 1 != 0 {
            x ^= (*buf as u64) << i;
        }

        v0 = my_set_low64(x as i64);
        v0 = shift_left(v0, 8 - size);
    } else if size < 16 {
        v0 = my_set_low64((crc as u64 ^ read64le(buf)) as i64);

        // buf is intentionally left 8 bytes behind so that the last
        // 1-7 bytes can be read with read64le(buf + size).
        size -= 8;

        if size > 0 {
            let padding = 8 - size;
            let high = read64le(buf.add(size)) >> (padding * 8);

            v0 = insert_high64(v0, high);
            v0 = shift_left(v0, padding);

            v1 = _mm_srli_si128::<8>(v0);
            v0 = _mm_clmulepi64_si128::<0x10>(v0, fold128);
            v0 = _mm_xor_si128(v0, v1);
        }
    } else {
        v0 = my_set_low64(crc as i64);

        // Read the first (and possibly the only) full 16 bytes.
        v0 = _mm_xor_si128(v0, my_load128(buf));
        buf = buf.add(16);
        size -= 16;

        if size >= 48 {
            v1 = my_load128(buf);
            v2 = my_load128(buf.add(16));
            v3 = my_load128(buf.add(32));
            buf = buf.add(48);
            size -= 48;

            while size >= 64 {
                v0 = fold_xor(v0, fold512, buf);
                v1 = fold_xor(v1, fold512, buf.add(16));
                v2 = fold_xor(v2, fold512, buf.add(32));
                v3 = fold_xor(v3, fold512, buf.add(48));
                buf = buf.add(64);
                size -= 64;
            }

            v0 = _mm_xor_si128(v1, fold(v0, fold128));
            v0 = _mm_xor_si128(v2, fold(v0, fold128));
            v0 = _mm_xor_si128(v3, fold(v0, fold128));
        }

        while size >= 16 {
            v0 = fold_xor(v0, fold128, buf);
            buf = buf.add(16);
            size -= 16;
        }

        if size > 0 {
            // The last "size" input bytes go to the high bytes of v1:
            // a full 16-byte load whose low bytes are then cleared.
            v1 = my_load128(buf.add(size).sub(16));
            v1 = keep_high_bytes(v1, size);

            // Shift high bytes from v0 to the low bytes of v1, then
            // shift the high bytes of v0 away padding with zeros.
            v1 = _mm_or_si128(v1, shift_right(v0, size));
            v0 = shift_left(v0, 16 - size);

            v0 = _mm_xor_si128(v1, fold(v0, fold128));
        }

        v1 = _mm_srli_si128::<8>(v0);
        v0 = _mm_clmulepi64_si128::<0x10>(v0, fold128);
        v0 = _mm_xor_si128(v0, v1);
    }

    // Barrett reduction
    v1 = _mm_clmulepi64_si128::<0x10>(v0, mu_p); // v0 * mu
    v1 = _mm_clmulepi64_si128::<0x00>(v1, mu_p); // v1 * p
    v0 = _mm_xor_si128(v0, v1);
    !(_mm_extract_epi32::<2>(v0) as u32)
}

/// # Safety
///
/// `is_arch_extension_supported()` must have returned true, and `buf`
/// must point to at least `size` readable bytes.
///
/// The unguarded contract comes from crc_x86_clmul.h, where the same
/// function is reachable only through the C feature dispatch and performs
/// no check of its own. Calling it on a CPU without those extensions
/// executes an illegal instruction rather than merely being unsound.
#[target_feature(enable = "ssse3", enable = "sse4.1", enable = "pclmulqdq")]
pub unsafe fn crc64_arch_optimized(mut buf: *const u8, mut size: size_t, mut crc: u64) -> u64 {
    if size == 0 {
        return crc;
    }

    // See crc_clmul_consts_gen.c.
    let fold512 = _mm_set_epi64x(0x081f6054a7842df4, 0x6ae3efbb9dd441f3);
    let fold128 = _mm_set_epi64x(0xdabe95afc7875f40u64 as i64, 0xe05dd497ca393ae4u64 as i64);
    let mu_p = _mm_set_epi64x(0x9c3e466c172963d5u64 as i64, 0x92d8af2baf0e1e84u64 as i64);

    let mut v0: __m128i;
    let mut v1: __m128i;
    let mut v2: __m128i;
    let mut v3: __m128i;

    crc = !crc;

    if size < 8 {
        let mut x: u64 = crc;
        let mut i: u32 = 0;

        if size & 4 != 0 {
            x ^= read32le(buf) as u64;
            buf = buf.add(4);
            i = 32;
        }

        if size & 2 != 0 {
            x ^= (read16le(buf) as u64) << i;
            buf = buf.add(2);
            i += 16;
        }

        if size & 1 != 0 {
            x ^= (*buf as u64) << i;
        }

        v0 = my_set_low64(x as i64);
        v0 = shift_left(v0, 8 - size);
    } else if size < 16 {
        v0 = my_set_low64((crc ^ read64le(buf)) as i64);

        // buf is intentionally left 8 bytes behind so that the last
        // 1-7 bytes can be read with read64le(buf + size).
        size -= 8;

        if size > 0 {
            let padding = 8 - size;
            let high = read64le(buf.add(size)) >> (padding * 8);

            v0 = insert_high64(v0, high);
            v0 = shift_left(v0, padding);

            v1 = _mm_srli_si128::<8>(v0);
            v0 = _mm_clmulepi64_si128::<0x10>(v0, fold128);
            v0 = _mm_xor_si128(v0, v1);
        }
    } else {
        v0 = my_set_low64(crc as i64);

        // Read the first (and possibly the only) full 16 bytes.
        v0 = _mm_xor_si128(v0, my_load128(buf));
        buf = buf.add(16);
        size -= 16;

        if size >= 48 {
            v1 = my_load128(buf);
            v2 = my_load128(buf.add(16));
            v3 = my_load128(buf.add(32));
            buf = buf.add(48);
            size -= 48;

            while size >= 64 {
                v0 = fold_xor(v0, fold512, buf);
                v1 = fold_xor(v1, fold512, buf.add(16));
                v2 = fold_xor(v2, fold512, buf.add(32));
                v3 = fold_xor(v3, fold512, buf.add(48));
                buf = buf.add(64);
                size -= 64;
            }

            v0 = _mm_xor_si128(v1, fold(v0, fold128));
            v0 = _mm_xor_si128(v2, fold(v0, fold128));
            v0 = _mm_xor_si128(v3, fold(v0, fold128));
        }

        while size >= 16 {
            v0 = fold_xor(v0, fold128, buf);
            buf = buf.add(16);
            size -= 16;
        }

        if size > 0 {
            // The last "size" input bytes go to the high bytes of v1:
            // a full 16-byte load whose low bytes are then cleared.
            v1 = my_load128(buf.add(size).sub(16));
            v1 = keep_high_bytes(v1, size);

            // Shift high bytes from v0 to the low bytes of v1, then
            // shift the high bytes of v0 away padding with zeros.
            v1 = _mm_or_si128(v1, shift_right(v0, size));
            v0 = shift_left(v0, 16 - size);

            v0 = _mm_xor_si128(v1, fold(v0, fold128));
        }

        v1 = _mm_srli_si128::<8>(v0);
        v0 = _mm_clmulepi64_si128::<0x10>(v0, fold128);
        v0 = _mm_xor_si128(v0, v1);
    }

    // Barrett reduction. Because p is 65 bits but one bit doesn't fit
    // into the 64-bit half of __m128i, finish the second clmul by
    // shifting v1 left by 64 bits and xorring it to the final result.
    v1 = _mm_clmulepi64_si128::<0x10>(v0, mu_p); // v0 * mu
    v2 = _mm_slli_si128::<8>(v1);
    v1 = _mm_clmulepi64_si128::<0x00>(v1, mu_p); // v1 * p
    v0 = _mm_xor_si128(v0, v2);
    v0 = _mm_xor_si128(v0, v1);
    !extract_crc64(v0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::crc32_fast::lzma_crc32_generic;
    use crate::check::crc64_fast::lzma_crc64_generic;

    fn pseudo_random(seed: u64, len: usize) -> Vec<u8> {
        let mut state = seed;
        (0..len)
            .map(|_| {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                (state >> 56) as u8
            })
            .collect()
    }

    // Lengths crossing every branch: <8 bit-tests, ==8, every 8..16
    // padding, every 16..32 tail size (each uses a distinct vmasks
    // window), the 48-byte preload, the 64-byte fold loop, and ragged
    // tails.
    const LENGTHS: &[usize] = &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 47, 48, 63, 64, 65, 96, 127, 128, 129, 255, 256, 1000,
        4096,
    ];

    #[test]
    fn clmul_crc32_matches_generic() {
        if !is_arch_extension_supported() {
            eprintln!("skipping: CLMUL not supported on this CPU");
            return;
        }
        let data = pseudo_random(32, 8192);
        for &len in LENGTHS {
            for offset in 0..8 {
                let buf = &data[offset..offset + len];
                for init in [0u32, 0xFFFF_FFFF, 0x1234_5678] {
                    let expected = lzma_crc32_generic(buf, init);
                    let got = unsafe { crc32_arch_optimized(buf.as_ptr(), buf.len(), init) };
                    assert_eq!(got, expected, "len={len} offset={offset} init={init:#x}");
                }
            }
        }
        // Chained updates across an odd split.
        let mid = lzma_crc32_generic(&data[..1234], 0);
        let expected = lzma_crc32_generic(&data[1234..4000], mid);
        let got = unsafe { crc32_arch_optimized(data[1234..4000].as_ptr(), 4000 - 1234, mid) };
        assert_eq!(got, expected);
    }

    #[test]
    fn clmul_crc64_matches_generic() {
        if !is_arch_extension_supported() {
            eprintln!("skipping: CLMUL not supported on this CPU");
            return;
        }
        let data = pseudo_random(64, 8192);
        for &len in LENGTHS {
            for offset in 0..8 {
                let buf = &data[offset..offset + len];
                for init in [0u64, u64::MAX, 0x0123_4567_89AB_CDEF] {
                    let expected = unsafe { lzma_crc64_generic(buf.as_ptr(), buf.len(), init) };
                    let got = unsafe { crc64_arch_optimized(buf.as_ptr(), buf.len(), init) };
                    assert_eq!(got, expected, "len={len} offset={offset} init={init:#x}");
                }
            }
        }
        let mid = unsafe { lzma_crc64_generic(data.as_ptr(), 1234, 0) };
        let expected = unsafe { lzma_crc64_generic(data[1234..4000].as_ptr(), 4000 - 1234, mid) };
        let got = unsafe { crc64_arch_optimized(data[1234..4000].as_ptr(), 4000 - 1234, mid) };
        assert_eq!(got, expected);
    }
}
