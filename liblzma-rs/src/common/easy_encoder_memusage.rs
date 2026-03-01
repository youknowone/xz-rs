use crate::types::*;
extern "C" {
    fn lzma_raw_encoder_memusage(filters: *const lzma_filter) -> u64;
    fn lzma_easy_preset(easy: *mut lzma_options_easy, preset: u32) -> bool;
}
#[no_mangle]
pub extern "C" fn lzma_easy_encoder_memusage(preset: u32) -> u64 {
    let mut opt_easy: lzma_options_easy = lzma_options_easy {
        filters: [lzma_filter {
            id: 0,
            options: core::ptr::null_mut(),
        }; 5],
        opt_lzma: lzma_options_lzma {
            dict_size: 0,
            preset_dict: core::ptr::null::<u8>(),
            preset_dict_size: 0,
            lc: 0,
            lp: 0,
            pb: 0,
            mode: 0 as lzma_mode,
            nice_len: 0,
            mf: 0 as lzma_match_finder,
            depth: 0,
            ext_flags: 0,
            ext_size_low: 0,
            ext_size_high: 0,
            reserved_int4: 0,
            reserved_int5: 0,
            reserved_int6: 0,
            reserved_int7: 0,
            reserved_int8: 0,
            reserved_enum1: LZMA_RESERVED_ENUM,
            reserved_enum2: LZMA_RESERVED_ENUM,
            reserved_enum3: LZMA_RESERVED_ENUM,
            reserved_enum4: LZMA_RESERVED_ENUM,
            reserved_ptr1: core::ptr::null_mut(),
            reserved_ptr2: core::ptr::null_mut(),
        },
    };
    if unsafe { lzma_easy_preset(&raw mut opt_easy, preset) } {
        return UINT32_MAX as u64;
    }
    unsafe { lzma_raw_encoder_memusage(&raw mut opt_easy.filters as *mut lzma_filter) }
}
