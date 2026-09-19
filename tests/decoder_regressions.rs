#![cfg(not(target_family = "wasm"))]

use xz::stream::{Action, Error, Status, Stream, IGNORE_CHECK};

const BAD_CRC32_XZ: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-check-crc32.xz");
const GOOD_LZIP: &[u8] = include_bytes!("../vendor/xz/tests/files/good-1-v1.lz");

fn decode_with(mut stream: Stream, input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut input_pos = 0usize;
    let mut decoded = Vec::new();

    for _ in 0..10_000 {
        let in_before = stream.total_in();
        let out_before = stream.total_out();
        let mut output = [0; 1024];

        let status = stream.process(&input[input_pos..], &mut output, Action::Finish)?;

        input_pos += (stream.total_in() - in_before) as usize;
        let output_used = (stream.total_out() - out_before) as usize;
        decoded.extend_from_slice(&output[..output_used]);

        if status == Status::StreamEnd {
            return Ok(decoded);
        }

        if stream.total_in() == in_before && stream.total_out() == out_before {
            panic!("decoder made no progress");
        }
    }

    panic!("decoder did not converge");
}

fn decode_stream(input: &[u8], flags: u32) -> Result<Vec<u8>, Error> {
    decode_with(Stream::new_stream_decoder(u64::MAX, flags).unwrap(), input)
}

#[test]
fn alone_decoder_rejects_wrapping_dictionary_size() {
    let input = [
        0x5D, // common valid LZMA properties
        0x00, 0x00, 0x00, 0xE0, // dictionary size that overflows the picky check's rounding
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // uncompressed size
    ];
    let mut stream = Stream::new_auto_decoder(u64::MAX, 0).unwrap();
    let mut output = [0; 1];

    let err = stream
        .process(&input, &mut output, Action::Run)
        .expect_err("invalid .lzma header should be rejected");
    assert_eq!(err, Error::Format);
}

#[test]
fn stream_decoder_ignore_check_skips_integrity_check_verification() {
    let err = decode_stream(BAD_CRC32_XZ, 0).unwrap_err();
    assert_eq!(err, Error::Data);

    let decoded = decode_stream(BAD_CRC32_XZ, IGNORE_CHECK).unwrap();
    assert!(!decoded.is_empty());
}

#[test]
fn auto_decoder_accepts_lzip_streams() {
    let direct = decode_with(Stream::new_lzip_decoder(u64::MAX, 0).unwrap(), GOOD_LZIP).unwrap();
    let auto = decode_with(Stream::new_auto_decoder(u64::MAX, 0).unwrap(), GOOD_LZIP);

    // liblzma-sys may link a build whose auto decoder was compiled without
    // HAVE_LZIP_DECODER (its bundled static config.h omits it), so the lzip
    // decoder works directly but the auto decoder cannot detect lzip and
    // returns Format. Accept that only on the liblzma-sys backend; verify
    // correctness whenever the auto decoder does handle the stream.
    #[cfg(feature = "liblzma-sys")]
    if matches!(auto, Err(Error::Format)) {
        return;
    }

    let auto = auto.unwrap();
    assert_eq!(auto, direct);
    assert!(!auto.is_empty());
}
