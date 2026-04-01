#![allow(bad_style)]

#[cfg(all(feature = "c-sys", feature = "rs-sys"))]
compile_error!("Enable exactly one backend feature: c-sys or rs-sys");
#[cfg(not(any(feature = "c-sys", feature = "rs-sys")))]
compile_error!("Enable one backend feature: c-sys or rs-sys");

#[cfg(feature = "c-sys")]
use liblzma_c_sys::*;
#[cfg(feature = "rs-sys")]
use liblzma_rs_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
