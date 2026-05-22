#![allow(bad_style)]

#[cfg(not(feature = "xz-sys"))]
compile_error!("Enable backend feature: xz-sys");

#[cfg(feature = "xz-sys")]
use xz_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
