use crate::types::*;
#[no_mangle]
pub extern "C" fn lzma_vli_size(mut vli: lzma_vli) -> u32 {
    if vli > LZMA_VLI_MAX {
        return 0;
    }
    let mut i: u32 = 0;
    loop {
        vli >>= 7;
        i += 1;
        if vli == 0 {
            break;
        }
    }
    i
}
