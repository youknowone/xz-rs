use crate::common::stream_buffer_encoder::lzma_stream_buffer_encode;
use crate::types::*;
use core::mem::MaybeUninit;
pub unsafe fn lzma_easy_buffer_encode(
    preset: u32,
    check: lzma_check,
    allocator: *const lzma_allocator,
    in_0: *const u8,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
) -> lzma_ret {
    let mut opt_easy = MaybeUninit::<lzma_options_easy>::uninit();
    if lzma_easy_preset(opt_easy.as_mut_ptr(), preset) {
        return LZMA_OPTIONS_ERROR;
    }
    let opt_easy = opt_easy.assume_init_mut();
    lzma_stream_buffer_encode(
        ::core::ptr::addr_of_mut!(opt_easy.filters) as *mut lzma_filter,
        check,
        allocator,
        in_0,
        in_size,
        out,
        out_pos,
        out_size,
    )
}
