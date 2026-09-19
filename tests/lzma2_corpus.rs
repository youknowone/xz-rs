#![cfg(not(target_family = "wasm"))]

use xz::stream::{Action, Error, Status, Stream};

const GOOD_LZMA2_1: &[u8] = include_bytes!("../vendor/xz/tests/files/good-1-lzma2-1.xz");
const GOOD_LZMA2_2: &[u8] = include_bytes!("../vendor/xz/tests/files/good-1-lzma2-2.xz");
const GOOD_LZMA2_3: &[u8] = include_bytes!("../vendor/xz/tests/files/good-1-lzma2-3.xz");
const GOOD_LZMA2_4: &[u8] = include_bytes!("../vendor/xz/tests/files/good-1-lzma2-4.xz");
const GOOD_LZMA2_5: &[u8] = include_bytes!("../vendor/xz/tests/files/good-1-lzma2-5.xz");
const GOOD_2_LZMA2: &[u8] = include_bytes!("../vendor/xz/tests/files/good-2-lzma2.xz");

const BAD_LZMA2_1: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-lzma2-1.xz");
const BAD_LZMA2_2: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-lzma2-2.xz");
const BAD_LZMA2_3: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-lzma2-3.xz");
const BAD_LZMA2_4: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-lzma2-4.xz");
const BAD_LZMA2_5: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-lzma2-5.xz");
const BAD_LZMA2_6: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-lzma2-6.xz");
const BAD_LZMA2_7: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-lzma2-7.xz");
const BAD_LZMA2_8: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-lzma2-8.xz");
const BAD_LZMA2_9: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-lzma2-9.xz");
const BAD_LZMA2_10: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-lzma2-10.xz");
const BAD_LZMA2_11: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-lzma2-11.xz");

fn decode_stream(input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut stream = Stream::new_stream_decoder(u64::MAX, 0).unwrap();
    let mut input_pos = 0usize;
    let mut decoded = Vec::new();
    let mut stalled_once = false;

    for _ in 0..10_000 {
        let in_before = stream.total_in();
        let out_before = stream.total_out();
        let mut output = [0; 128];

        let status = stream.process(&input[input_pos..], &mut output, Action::Finish)?;

        input_pos += (stream.total_in() - in_before) as usize;
        let output_used = (stream.total_out() - out_before) as usize;
        decoded.extend_from_slice(&output[..output_used]);

        if status == Status::StreamEnd {
            assert_eq!(input_pos, input.len());
            return Ok(decoded);
        }

        if stream.total_in() == in_before && stream.total_out() == out_before {
            if stalled_once {
                panic!("decoder made no progress");
            }
            stalled_once = true;
        } else {
            stalled_once = false;
        }
    }

    panic!("decoder did not converge");
}

#[test]
fn lzma2_good_chunk_variants_decode() {
    let expected = decode_stream(GOOD_LZMA2_1).unwrap();
    assert!(expected.starts_with(b"Lorem ipsum dolor sit amet"));
    assert!(expected.ends_with(b"laborum. \n"));

    for (name, input) in [
        ("good-1-lzma2-2.xz", GOOD_LZMA2_2),
        ("good-1-lzma2-3.xz", GOOD_LZMA2_3),
        ("good-1-lzma2-4.xz", GOOD_LZMA2_4),
    ] {
        assert_eq!(decode_stream(input).unwrap(), expected, "{name}");
    }

    assert!(decode_stream(GOOD_LZMA2_5).unwrap().is_empty());
    assert_eq!(decode_stream(GOOD_2_LZMA2).unwrap(), b"Hello\nWorld!\n");
}

#[test]
fn lzma2_bad_chunk_variants_are_rejected() {
    for (name, input) in [
        ("bad-1-lzma2-1.xz", BAD_LZMA2_1),
        ("bad-1-lzma2-2.xz", BAD_LZMA2_2),
        ("bad-1-lzma2-3.xz", BAD_LZMA2_3),
        ("bad-1-lzma2-4.xz", BAD_LZMA2_4),
        ("bad-1-lzma2-5.xz", BAD_LZMA2_5),
        ("bad-1-lzma2-6.xz", BAD_LZMA2_6),
        ("bad-1-lzma2-7.xz", BAD_LZMA2_7),
        ("bad-1-lzma2-8.xz", BAD_LZMA2_8),
        ("bad-1-lzma2-9.xz", BAD_LZMA2_9),
        ("bad-1-lzma2-10.xz", BAD_LZMA2_10),
        ("bad-1-lzma2-11.xz", BAD_LZMA2_11),
    ] {
        assert_eq!(decode_stream(input).unwrap_err(), Error::Data, "{name}");
    }
}
