#![no_main]

mod common;

use libfuzzer_sys::fuzz_target;
use xz::stream::{CONCATENATED, IGNORE_CHECK, MtStreamBuilder};

fuzz_target!(|data: &[u8]| {
    let stream = MtStreamBuilder::new()
        .flags(CONCATENATED | IGNORE_CHECK)
        .threads(2)
        .memlimit_threading(common::MEM_LIMIT / 2)
        .memlimit_stop(common::MEM_LIMIT)
        .decoder()
        .unwrap();
    common::feed_decoder_like_upstream(stream, data);

    let stream = MtStreamBuilder::new()
        .flags(CONCATENATED | IGNORE_CHECK)
        .threads(2)
        .memlimit_threading(common::MEM_LIMIT / 2)
        .memlimit_stop(common::MEM_LIMIT)
        .decoder()
        .unwrap();
    common::feed_decoder(stream, data, common::controls(data));
});
