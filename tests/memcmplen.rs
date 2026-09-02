//! Differential tests for lzma_memcmplen.
//!
//! Only one of the compare branches is compiled for any given target, so
//! these compare whichever branch was selected against a byte-at-a-time
//! reference. The buffers carry LZMA_MEMCMPLEN_EXTRA zeroed tail bytes,
//! exactly like the match finder buffer the function is used on, so a
//! branch reading further than its EXTRA allows shows up under a
//! sanitizer as well.

#![cfg(not(target_family = "wasm"))]

use xz_core::types::{lzma_memcmplen, LZMA_MEMCMPLEN_EXTRA};

/// Byte-at-a-time equivalent of lzma_memcmplen.
fn reference(buf1: &[u8], buf2: &[u8], mut len: u32, limit: u32) -> u32 {
    while len < limit && buf1[len as usize] == buf2[len as usize] {
        len += 1;
    }
    len
}

/// Allocates `len` bytes plus the zeroed tail the fast paths may read.
fn padded(bytes: &[u8]) -> Vec<u8> {
    let mut v = bytes.to_vec();
    v.resize(bytes.len() + LZMA_MEMCMPLEN_EXTRA as usize, 0);
    v
}

fn check(a: &[u8], b: &[u8], len: u32, limit: u32) {
    assert_eq!(a.len(), b.len());
    let pa = padded(a);
    let pb = padded(b);
    let got = unsafe { lzma_memcmplen(pa.as_ptr(), pb.as_ptr(), len, limit) };
    let want = reference(a, b, len, limit);
    assert_eq!(
        got, want,
        "len={len} limit={limit} a={a:?} b={b:?} (EXTRA={LZMA_MEMCMPLEN_EXTRA})"
    );
}

#[test]
fn equal_buffers_return_limit() {
    // Cover every length across the 4/8/16-byte step boundaries.
    for limit in 0..=64u32 {
        let buf: Vec<u8> = (0..limit).map(|i| i as u8).collect();
        check(&buf, &buf.clone(), 0, limit);
    }
}

#[test]
fn first_difference_at_every_position() {
    const LIMIT: u32 = 64;
    let base: Vec<u8> = (0..LIMIT).map(|i| i as u8).collect();
    for diff_at in 0..LIMIT {
        let mut other = base.clone();
        other[diff_at as usize] ^= 0xFF;
        check(&base, &other, 0, LIMIT);
    }
}

#[test]
fn difference_is_never_reported_past_limit() {
    // A mismatch beyond `limit` must not shorten the result: the fast
    // paths read past `limit` and have to clamp.
    const SIZE: u32 = 64;
    let base: Vec<u8> = (0..SIZE).map(|i| i as u8).collect();
    for limit in 0..=SIZE {
        for diff_at in limit..SIZE {
            let mut other = base.clone();
            other[diff_at as usize] ^= 0xFF;
            check(&base, &other, 0, limit);
        }
    }
}

#[test]
fn resuming_from_a_nonzero_len() {
    const LIMIT: u32 = 48;
    let base: Vec<u8> = (0..LIMIT).map(|i| i as u8).collect();
    for start in 0..LIMIT {
        for diff_at in start..LIMIT {
            let mut other = base.clone();
            other[diff_at as usize] ^= 0xFF;
            check(&base, &other, start, LIMIT);
        }
    }
}

#[test]
fn single_bit_differences() {
    // Exercises the byte-index extraction: a difference in only the low or
    // only the high bit of a byte must still resolve to that byte.
    const LIMIT: u32 = 32;
    let base = vec![0x55u8; LIMIT as usize];
    for diff_at in 0..LIMIT {
        for bit in 0..8 {
            let mut other = base.clone();
            other[diff_at as usize] ^= 1 << bit;
            check(&base, &other, 0, LIMIT);
        }
    }
}
