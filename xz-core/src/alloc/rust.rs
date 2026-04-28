use crate::raw_alloc::{RUST_ALLOC_ALIGN, alloc_impl, free_impl};
use crate::types::*;

#[repr(transparent)]
pub(super) struct StaticAllocator(lzma_allocator);

unsafe impl Sync for StaticAllocator {}

static RUST_ALLOCATOR: StaticAllocator = StaticAllocator(lzma_allocator {
    alloc: Some(lzma_rust_alloc),
    free: Some(lzma_rust_free),
    opaque: core::ptr::null_mut(),
});

pub(super) unsafe fn lzma_rust_alloc(
    _opaque: *mut c_void,
    nmemb: size_t,
    size: size_t,
) -> *mut c_void {
    let Some(size) = (nmemb as usize).checked_mul(size as usize) else {
        return core::ptr::null_mut();
    };
    alloc_impl(size, RUST_ALLOC_ALIGN, false)
}

pub(super) unsafe fn lzma_rust_free(_opaque: *mut c_void, ptr: *mut c_void) {
    unsafe { free_impl(ptr) };
}

pub fn rust_allocator() -> lzma_allocator {
    RUST_ALLOCATOR.0
}

pub(super) fn rust_allocator_ptr() -> *const lzma_allocator {
    &raw const RUST_ALLOCATOR.0
}
