#![no_main]

mod common;

use libfuzzer_sys::fuzz_target;
use xz::stream::{Stream, CONCATENATED};

fuzz_target!(|data: &[u8]| {
    let stream = Stream::new_stream_decoder(common::MEM_LIMIT, CONCATENATED).unwrap();
    common::feed_decoder(stream, data, common::controls(data));
});
