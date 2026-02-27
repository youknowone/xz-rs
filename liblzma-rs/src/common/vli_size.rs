use crate::types::*;
use core::ffi::{c_int, c_ulonglong};
pub const UINT64_MAX: c_ulonglong = u64::MAX as c_ulonglong;
pub const LZMA_VLI_MAX: c_ulonglong = UINT64_MAX.wrapping_div(2);
#[no_mangle]
pub extern "C" fn lzma_vli_size(mut vli: lzma_vli) -> u32 {
    if vli > LZMA_VLI_MAX as lzma_vli {
        return 0;
    }
    let mut i: u32 = 0;
    loop {
        vli >>= 7 as c_int;
        i = i.wrapping_add(1);
        if !(vli != 0 as lzma_vli) {
            break;
        }
    }
    return i;
}
