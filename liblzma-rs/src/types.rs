use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};

// Platform-dependent type aliases
pub type size_t = libc::size_t;
pub type uintptr_t = libc::uintptr_t;

// lzma type aliases
pub type lzma_bool = c_uchar;
pub type lzma_ret = c_uint;
pub type lzma_action = c_uint;
pub type lzma_check = c_uint;
pub type lzma_vli = u64;
pub type lzma_reserved_enum = c_uint;
pub type lzma_mode = c_uint;
pub type lzma_match_finder = c_uint;
pub type lzma_lzma_state = c_uint;
pub type lzma_delta_type = c_uint;
pub type probability = u16;

// Common struct shared across all modules
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, size_t, size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    pub opaque: *mut c_void,
}

// Common extern functions used across many modules
extern "C" {
    pub fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    pub fn lzma_alloc_zero(size: size_t, allocator: *const lzma_allocator) -> *mut c_void;
    pub fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator);
    pub fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memmove(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    pub fn memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn strlen(s: *const c_char) -> size_t;
}
