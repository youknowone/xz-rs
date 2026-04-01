use crate::types::*;
pub unsafe fn lzma_easy_preset(opt_easy: *mut lzma_options_easy, preset: u32) -> bool {
    if lzma_lzma_preset(::core::ptr::addr_of_mut!((*opt_easy).opt_lzma), preset) != 0 {
        return true;
    }
    (*opt_easy).filters[0].id = LZMA_FILTER_LZMA2;
    (*opt_easy).filters[0].options = ::core::ptr::addr_of_mut!((*opt_easy).opt_lzma) as *mut c_void;
    (*opt_easy).filters[1].id = LZMA_VLI_UNKNOWN;
    false
}
