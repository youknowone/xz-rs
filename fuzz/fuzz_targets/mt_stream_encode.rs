#![no_main]

mod common;

use libfuzzer_sys::fuzz_target;
use xz::stream::{Check, MtStreamBuilder};

fuzz_target!(|data: &[u8]| {
    let controls = common::controls(data);
    let input = common::encode_input(data);
    let preset = common::preset(data.first().copied().unwrap_or(0));
    let stream = MtStreamBuilder::new()
        .preset(preset)
        .check(Check::Crc64)
        .threads(2)
        .encoder()
        .unwrap();
    let encoded = common::encode_with_actions(stream, input, controls, false);
    common::assert_xz_decodes_to(&encoded, input);
});
