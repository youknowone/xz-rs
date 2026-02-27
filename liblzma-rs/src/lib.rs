#![allow(
    clashing_extern_declarations,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    clippy::all
)]

pub mod types;

pub mod check;
pub mod common;
pub mod delta;
pub mod lz;
pub mod lzma;
pub mod rangecoder;
pub mod simple;
pub mod tuklib;
