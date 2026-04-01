use crate::common::stream_buffer_encoder::lzma_stream_buffer_encode;
use crate::types::*;
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
    let mut opt_easy: lzma_options_easy = lzma_options_easy {
        filters: [lzma_filter {
            id: 0,
            options: core::ptr::null_mut(),
        }; 5],
        opt_lzma: lzma_options_lzma {
            dict_size: 0,
            preset_dict: core::ptr::null(),
            preset_dict_size: 0,
            lc: 0,
            lp: 0,
            pb: 0,
            mode: 0,
            nice_len: 0,
            mf: 0,
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
    if lzma_easy_preset(::core::ptr::addr_of_mut!(opt_easy), preset) {
        return LZMA_OPTIONS_ERROR;
    }
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
