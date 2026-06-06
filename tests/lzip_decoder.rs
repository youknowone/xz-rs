#![cfg(not(target_family = "wasm"))]

use xz::stream::{Action, Error, Status, Stream, CONCATENATED, IGNORE_CHECK, TELL_ANY_CHECK};

const MEMLIMIT: u64 = 1 << 20;
const HELLO_WORLD: &[u8] = b"Hello\nWorld!\n";
const TRAILING_GARBAGE: &[u8] = b"Trailing garbage\n";

const GOOD_1_V0: &[u8] = include_bytes!("../vendor/xz/tests/files/good-1-v0.lz");
const GOOD_1_V0_TRAILING_1: &[u8] =
    include_bytes!("../vendor/xz/tests/files/good-1-v0-trailing-1.lz");
const GOOD_1_V1: &[u8] = include_bytes!("../vendor/xz/tests/files/good-1-v1.lz");
const GOOD_1_V1_TRAILING_1: &[u8] =
    include_bytes!("../vendor/xz/tests/files/good-1-v1-trailing-1.lz");
const GOOD_1_V1_TRAILING_2: &[u8] =
    include_bytes!("../vendor/xz/tests/files/good-1-v1-trailing-2.lz");
const GOOD_2_V0_V1: &[u8] = include_bytes!("../vendor/xz/tests/files/good-2-v0-v1.lz");
const GOOD_2_V1_V0: &[u8] = include_bytes!("../vendor/xz/tests/files/good-2-v1-v0.lz");
const GOOD_2_V1_V1: &[u8] = include_bytes!("../vendor/xz/tests/files/good-2-v1-v1.lz");

const BAD_1_V0_UNCOMP_SIZE: &[u8] =
    include_bytes!("../vendor/xz/tests/files/bad-1-v0-uncomp-size.lz");
const BAD_1_V1_CRC32: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-v1-crc32.lz");
const BAD_1_V1_DICT_1: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-v1-dict-1.lz");
const BAD_1_V1_DICT_2: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-v1-dict-2.lz");
const BAD_1_V1_MAGIC_1: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-v1-magic-1.lz");
const BAD_1_V1_MAGIC_2: &[u8] = include_bytes!("../vendor/xz/tests/files/bad-1-v1-magic-2.lz");
const BAD_1_V1_MEMBER_SIZE: &[u8] =
    include_bytes!("../vendor/xz/tests/files/bad-1-v1-member-size.lz");
const BAD_1_V1_TRAILING_MAGIC: &[u8] =
    include_bytes!("../vendor/xz/tests/files/bad-1-v1-trailing-magic.lz");
const BAD_1_V1_UNCOMP_SIZE: &[u8] =
    include_bytes!("../vendor/xz/tests/files/bad-1-v1-uncomp-size.lz");
const UNSUPPORTED_1_V234: &[u8] = include_bytes!("../vendor/xz/tests/files/unsupported-1-v234.lz");

fn drive_until_event(
    stream: &mut Stream,
    input: &[u8],
    input_pos: &mut usize,
    action: Action,
    chunk_size: usize,
) -> Result<(Vec<u8>, Status), Error> {
    let mut decoded = Vec::new();
    let mut stalled_once = false;

    for _ in 0..10_000 {
        let in_before = stream.total_in();
        let out_before = stream.total_out();
        let mut output = [0; 64];

        let available = input.len().saturating_sub(*input_pos);
        let take = available.min(chunk_size);
        let chunk = &input[*input_pos..*input_pos + take];
        let ret = stream.process(chunk, &mut output, action);

        *input_pos += (stream.total_in() - in_before) as usize;
        let output_used = (stream.total_out() - out_before) as usize;
        decoded.extend_from_slice(&output[..output_used]);

        let status = ret?;

        if status != Status::Ok {
            return Ok((decoded, status));
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

fn drive_new_lzip(
    input: &[u8],
    flags: u32,
    action: Action,
    chunk_size: usize,
) -> Result<(Vec<u8>, usize, Status), Error> {
    let mut stream = Stream::new_lzip_decoder(MEMLIMIT, flags)?;
    let mut input_pos = 0;
    let (decoded, status) =
        drive_until_event(&mut stream, input, &mut input_pos, action, chunk_size)?;
    Ok((decoded, input_pos, status))
}

#[test]
fn lzip_single_member_files_decode_one_byte_at_a_time() {
    for (name, input) in [("good-1-v0.lz", GOOD_1_V0), ("good-1-v1.lz", GOOD_1_V1)] {
        let (decoded, consumed, status) = drive_new_lzip(input, 0, Action::Run, 1).unwrap();

        assert_eq!(status, Status::StreamEnd, "{name}");
        assert_eq!(consumed, input.len(), "{name}");
        assert_eq!(decoded, HELLO_WORLD, "{name}");
    }
}

#[test]
fn lzip_concatenated_members_decode_to_one_payload() {
    for (name, input) in [
        ("good-2-v0-v1.lz", GOOD_2_V0_V1),
        ("good-2-v1-v0.lz", GOOD_2_V1_V0),
        ("good-2-v1-v1.lz", GOOD_2_V1_V1),
    ] {
        let (decoded, consumed, status) =
            drive_new_lzip(input, CONCATENATED, Action::Finish, usize::MAX).unwrap();

        assert_eq!(status, Status::StreamEnd, "{name}");
        assert_eq!(consumed, input.len(), "{name}");
        assert_eq!(decoded, HELLO_WORLD, "{name}");
    }
}

#[test]
fn lzip_concatenated_trailing_data_remains_unconsumed() {
    for (name, input) in [
        ("good-1-v0-trailing-1.lz", GOOD_1_V0_TRAILING_1),
        ("good-1-v1-trailing-1.lz", GOOD_1_V1_TRAILING_1),
        ("good-1-v1-trailing-2.lz", GOOD_1_V1_TRAILING_2),
    ] {
        let (decoded, consumed, status) =
            drive_new_lzip(input, CONCATENATED, Action::Run, usize::MAX).unwrap();

        assert_eq!(status, Status::StreamEnd, "{name}");
        assert!(consumed < input.len(), "{name}");
        assert_eq!(decoded, HELLO_WORLD, "{name}");
        assert_eq!(&input[consumed..], TRAILING_GARBAGE, "{name}");
    }
}

#[test]
fn lzip_bad_files_report_expected_errors() {
    for (name, input, expected) in [
        ("bad-1-v1-magic-1.lz", BAD_1_V1_MAGIC_1, Error::Format),
        ("bad-1-v1-magic-2.lz", BAD_1_V1_MAGIC_2, Error::Format),
        ("unsupported-1-v234.lz", UNSUPPORTED_1_V234, Error::Options),
        ("bad-1-v1-dict-1.lz", BAD_1_V1_DICT_1, Error::Data),
        ("bad-1-v1-dict-2.lz", BAD_1_V1_DICT_2, Error::Data),
        ("bad-1-v1-crc32.lz", BAD_1_V1_CRC32, Error::Data),
        ("bad-1-v0-uncomp-size.lz", BAD_1_V0_UNCOMP_SIZE, Error::Data),
        ("bad-1-v1-uncomp-size.lz", BAD_1_V1_UNCOMP_SIZE, Error::Data),
        ("bad-1-v1-member-size.lz", BAD_1_V1_MEMBER_SIZE, Error::Data),
    ] {
        let err = drive_new_lzip(input, CONCATENATED, Action::Finish, usize::MAX).expect_err(name);
        assert_eq!(err, expected, "{name}");
    }
}

#[test]
fn lzip_trailing_magic_reports_truncated_input() {
    let (_decoded, _consumed, status) = drive_new_lzip(
        BAD_1_V1_TRAILING_MAGIC,
        CONCATENATED,
        Action::Finish,
        usize::MAX,
    )
    .unwrap();

    assert_eq!(status, Status::MemNeeded);
}

#[test]
fn lzip_check_flags_match_upstream_behavior() {
    let err = drive_new_lzip(BAD_1_V1_CRC32, CONCATENATED, Action::Finish, usize::MAX)
        .expect_err("bad CRC32 should fail");
    assert_eq!(err, Error::Data);

    let (decoded, consumed, status) = drive_new_lzip(
        BAD_1_V1_CRC32,
        CONCATENATED | IGNORE_CHECK,
        Action::Finish,
        usize::MAX,
    )
    .unwrap();
    assert_eq!(status, Status::StreamEnd);
    assert_eq!(consumed, BAD_1_V1_CRC32.len());
    assert_eq!(decoded, HELLO_WORLD);

    let mut stream = Stream::new_lzip_decoder(MEMLIMIT, CONCATENATED | TELL_ANY_CHECK).unwrap();
    let mut input_pos = 0;
    let (decoded_before_check, status) = drive_until_event(
        &mut stream,
        BAD_1_V1_CRC32,
        &mut input_pos,
        Action::Finish,
        usize::MAX,
    )
    .unwrap();
    assert_eq!(status, Status::GetCheck);
    assert!(decoded_before_check.is_empty());

    let err = drive_until_event(
        &mut stream,
        BAD_1_V1_CRC32,
        &mut input_pos,
        Action::Finish,
        usize::MAX,
    )
    .expect_err("CRC32 should still be verified after GetCheck");
    assert_eq!(err, Error::Data);
}

#[test]
fn lzip_memlimit_can_be_raised_after_error() {
    let mut stream = Stream::new_lzip_decoder(1, 0).unwrap();
    let mut input_pos = 0;

    let err = drive_until_event(
        &mut stream,
        GOOD_1_V1,
        &mut input_pos,
        Action::Run,
        usize::MAX,
    )
    .expect_err("low memlimit should fail");
    assert_eq!(err, Error::MemLimit);

    assert_eq!(stream.set_memlimit(100).unwrap_err(), Error::MemLimit);
    stream.set_memlimit(MEMLIMIT).unwrap();

    let (decoded, status) = drive_until_event(
        &mut stream,
        GOOD_1_V1,
        &mut input_pos,
        Action::Finish,
        usize::MAX,
    )
    .unwrap();
    assert_eq!(status, Status::StreamEnd);
    assert_eq!(input_pos, GOOD_1_V1.len());
    assert_eq!(decoded, HELLO_WORLD);
}

#[test]
fn lzip_invalid_flags_are_rejected() {
    match Stream::new_lzip_decoder(MEMLIMIT, u32::MAX) {
        Err(err) => assert_eq!(err, Error::Options),
        Ok(_) => panic!("invalid lzip flags should be rejected"),
    }
}
