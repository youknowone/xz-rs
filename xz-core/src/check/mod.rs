pub mod check;
pub mod crc32_fast;
pub mod crc64_fast;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod crc_x86_clmul;
pub mod sha256;
