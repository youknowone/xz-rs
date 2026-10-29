use std::ffi::CStr;
use xz_core::common::filter_common::lzma_filters_free;
use xz_core::common::string_conversion::lzma_str_to_filters;
use xz_core::types::*;

#[test]
fn initialized_option_table_parses_every_lzma_field() {
    let input = c"lzma2:preset=1,dict=2MiB,lc=2,lp=1,pb=1,mode=normal,nice=64,mf=bt3,depth=14";
    let mut filters = [lzma_filter {
        id: LZMA_VLI_UNKNOWN,
        options: std::ptr::null_mut(),
    }; 5];
    // The API owns each returned option allocation; release the complete chain.
    unsafe {
        let error = lzma_str_to_filters(
            input.as_ptr(),
            std::ptr::null_mut(),
            filters.as_mut_ptr(),
            0,
            std::ptr::null(),
        );
        assert!(
            error.is_null(),
            "{}",
            CStr::from_ptr(error).to_string_lossy()
        );
        assert_eq!(filters[0].id, LZMA_FILTER_LZMA2);
        assert_eq!(filters[1].id, LZMA_VLI_UNKNOWN);
        let option = &*filters[0].options.cast::<lzma_options_lzma>();
        assert_eq!(option.dict_size, 2 << 20);
        assert_eq!((option.lc, option.lp, option.pb), (2, 1, 1));
        assert_eq!(option.mode, LZMA_MODE_NORMAL);
        assert_eq!(option.nice_len, 64);
        assert_eq!(option.mf, LZMA_MF_BT3);
        assert_eq!(option.depth, 14);
        lzma_filters_free(filters.as_mut_ptr(), std::ptr::null());
    }
}

#[test]
fn initialized_option_table_preserves_bounds() {
    for input in [
        c"lzma2:lc=5",
        c"lzma2:lc=4,lp=1",
        c"lzma2:dict=1",
        c"lzma2:pb=5",
        c"lzma2:nice=1",
        c"lzma2:mf=unknown",
        c"lzma2:mode=unknown",
        c"lzma2:depth=4294967296",
        c"lzma2:preset=10",
    ] {
        let mut filters = [lzma_filter {
            id: LZMA_VLI_UNKNOWN,
            options: std::ptr::null_mut(),
        }; 5];
        // Invalid options must return an error and release partial allocations.
        unsafe {
            let error = lzma_str_to_filters(
                input.as_ptr(),
                std::ptr::null_mut(),
                filters.as_mut_ptr(),
                0,
                std::ptr::null(),
            );
            assert!(!error.is_null(), "{input:?}");
            assert_eq!(filters[0].id, LZMA_VLI_UNKNOWN);
        }
    }
}
