#![no_main]

mod common;

use libfuzzer_sys::fuzz_target;
use xz::stream::MtStreamBuilder;

fuzz_target!(|data: &[u8]| {
    let stream = MtStreamBuilder::new()
        .threads(2)
        .memlimit_threading(common::MEM_LIMIT / 2)
        .memlimit_stop(common::MEM_LIMIT)
        .decoder()
        .unwrap();
    common::feed_decoder(stream, data, common::controls(data));
});
