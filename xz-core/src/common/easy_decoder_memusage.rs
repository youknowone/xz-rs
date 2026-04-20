use crate::types::*;
use core::mem::MaybeUninit;
pub fn lzma_easy_decoder_memusage(preset: u32) -> u64 {
    let mut opt_easy = MaybeUninit::<lzma_options_easy>::uninit();
    if unsafe { lzma_easy_preset(opt_easy.as_mut_ptr(), preset) } {
        return UINT32_MAX as u64;
    }
    let opt_easy = unsafe { opt_easy.assume_init_mut() };
    unsafe {
        lzma_raw_decoder_memusage(::core::ptr::addr_of_mut!(opt_easy.filters) as *mut lzma_filter)
    }
}
