#![no_main]

mod common;

use libfuzzer_sys::fuzz_target;
use xz::stream::{Check, Stream};

fuzz_target!(|data: &[u8]| {
    let controls = common::controls(data);
    let input = common::encode_input(data);
    let preset = common::preset(data.first().copied().unwrap_or(0));
    let stream = Stream::new_easy_encoder(preset, Check::Crc64).unwrap();
    let encoded = common::encode_with_actions(stream, input, controls, true);
    common::assert_xz_decodes_to(&encoded, input);
});
