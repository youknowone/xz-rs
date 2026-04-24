use crate::types::*;

use super::rust::{rust_alloc_bytes, rust_alloc_impl, rust_alloc_zeroed_bytes, rust_free_ptr};

pub unsafe fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void {
    let _ = allocator;
    unsafe { rust_alloc_bytes(size) }
}

pub unsafe fn lzma_alloc_zero(size: size_t, allocator: *const lzma_allocator) -> *mut c_void {
    let _ = allocator;
    unsafe { rust_alloc_zeroed_bytes(size) }
}

pub unsafe fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator) {
    let _ = allocator;
    unsafe { rust_free_ptr(ptr) };
}

pub(crate) unsafe fn internal_alloc_bytes(
    size: size_t,
    allocator: *const lzma_allocator,
) -> *mut c_void {
    let _ = allocator;
    unsafe { rust_alloc_bytes(size) }
}

pub(crate) unsafe fn internal_alloc_object<T>(allocator: *const lzma_allocator) -> *mut T {
    let _ = allocator;
    rust_alloc_impl(core::mem::size_of::<T>(), core::mem::align_of::<T>(), false) as *mut T
}

pub(crate) unsafe fn internal_alloc_array<T>(
    count: size_t,
    allocator: *const lzma_allocator,
) -> *mut T {
    let _ = allocator;
    let Some(size) = (count as usize).checked_mul(core::mem::size_of::<T>()) else {
        return core::ptr::null_mut();
    };
    rust_alloc_impl(size, core::mem::align_of::<T>(), false) as *mut T
}

pub(crate) unsafe fn internal_alloc_zeroed_array<T>(
    count: size_t,
    allocator: *const lzma_allocator,
) -> *mut T {
    let _ = allocator;
    let Some(size) = (count as usize).checked_mul(core::mem::size_of::<T>()) else {
        return core::ptr::null_mut();
    };
    rust_alloc_impl(size, core::mem::align_of::<T>(), true) as *mut T
}

pub(crate) unsafe fn internal_free(ptr: *mut c_void, allocator: *const lzma_allocator) {
    let _ = allocator;
    unsafe { rust_free_ptr(ptr) };
}
