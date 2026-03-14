use crate::tuklib::tuklib_cpucores::tuklib_cpucores;
pub extern "C" fn lzma_cputhreads() -> u32 {
    tuklib_cpucores()
}
