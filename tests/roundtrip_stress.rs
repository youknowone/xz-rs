#![cfg(not(target_family = "wasm"))]

use std::io::Read;

use xz::bufread::{XzDecoder, XzEncoder};

fn pseudo_random(seed: u64, len: usize) -> Vec<u8> {
    let mut state = seed;
    (0..len)
        .map(|_| {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            (state >> 56) as u8
        })
        .collect()
}

// Mixed content that drives every range-decoder operation: random bytes
// (literals and matched literals), repeated text (matches, length trees,
// align bits), and far-apart duplicate blocks (distances above 2^20, so
// many direct bits).
fn stress_payload() -> Vec<u8> {
    let mut data = Vec::with_capacity(2 << 20);
    let unique = pseudo_random(7, 256 << 10);
    let text: Vec<u8> = b"the quick brown fox jumps over the lazy dog 0123456789 "
        .iter()
        .copied()
        .cycle()
        .take(128 << 10)
        .collect();
    data.extend_from_slice(&unique);
    data.extend_from_slice(&text);
    data.extend_from_slice(&vec![0u8; 64 << 10]);
    data.extend_from_slice(&pseudo_random(11, 768 << 10));
    // Duplicates of early blocks, now more than 1 MiB away.
    data.extend_from_slice(&unique);
    data.extend_from_slice(&text);
    data.extend_from_slice(&pseudo_random(13, 256 << 10));
    data
}

#[test]
fn roundtrip_mixed_payload() {
    let data = stress_payload();
    for preset in [0, 1, 6] {
        let mut compressed = Vec::new();
        XzEncoder::new(&data[..], preset)
            .read_to_end(&mut compressed)
            .unwrap();
        let mut decompressed = Vec::new();
        XzDecoder::new(&compressed[..])
            .read_to_end(&mut decompressed)
            .unwrap();
        assert_eq!(decompressed.len(), data.len(), "preset {preset}");
        assert!(decompressed == data, "preset {preset}");
    }
}
