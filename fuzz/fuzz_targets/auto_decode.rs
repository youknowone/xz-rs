#![no_main]

mod common;

use libfuzzer_sys::fuzz_target;
use xz::stream::Stream;

fuzz_target!(|data: &[u8]| {
    let stream = Stream::new_auto_decoder(common::MEM_LIMIT, 0).unwrap();
    common::feed_decoder(stream, data, common::controls(data));
});
