use crate::tuklib::tuklib_physmem::tuklib_physmem;

pub extern "C" fn lzma_physmem() -> u64 {
    tuklib_physmem()
}
