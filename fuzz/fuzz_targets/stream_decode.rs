#![no_main]

mod common;

use libfuzzer_sys::fuzz_target;
use xz::stream::{CONCATENATED, IGNORE_CHECK, Stream};

fuzz_target!(|data: &[u8]| {
    let stream =
        Stream::new_stream_decoder(common::MEM_LIMIT, CONCATENATED | IGNORE_CHECK).unwrap();
    common::feed_decoder_like_upstream(stream, data);

    let stream =
        Stream::new_stream_decoder(common::MEM_LIMIT, CONCATENATED | IGNORE_CHECK).unwrap();
    common::feed_decoder(stream, data, common::controls(data));
});
