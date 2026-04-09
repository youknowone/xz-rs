use crate::common::stream_encoder::lzma_stream_encoder;
use crate::types::*;
use core::mem::MaybeUninit;
pub unsafe fn lzma_easy_encoder(
    strm: *mut lzma_stream,
    preset: u32,
    check: lzma_check,
) -> lzma_ret {
    let mut opt_easy = MaybeUninit::<lzma_options_easy>::uninit();
    if lzma_easy_preset(opt_easy.as_mut_ptr(), preset) {
        return LZMA_OPTIONS_ERROR;
    }
    let opt_easy = opt_easy.assume_init_mut();
    lzma_stream_encoder(
        strm,
        ::core::ptr::addr_of_mut!(opt_easy.filters) as *mut lzma_filter,
        check,
    )
}
