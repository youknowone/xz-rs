#![no_main]

mod common;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let controls = common::controls(data);
    let preset = common::preset(data.first().copied().unwrap_or(0));

    let upstream_input = common::upstream_encode_input(data);
    let stream = common::stream_encoder_from_preset(preset);
    let encoded = common::encode_like_upstream(stream, upstream_input);
    common::assert_xz_decodes_to(&encoded, &common::upstream_encode_expected(upstream_input));

    let input = common::encode_input(data);
    let stream = common::stream_encoder_from_preset(preset);
    let encoded = common::encode_with_actions(stream, input, controls, true);
    common::assert_xz_decodes_to(&encoded, input);
});
