#![allow(
    clashing_extern_declarations,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unsafe_op_in_unsafe_fn,
    unused_assignments,
    clippy::all
)]
#[macro_export]
macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
}
pub mod alloc;
pub mod check;
pub mod common;
pub mod delta;
pub mod lz;
pub mod lzma;
pub mod rangecoder;
mod raw_alloc;
pub mod simple;
pub mod tuklib;
pub mod types;
