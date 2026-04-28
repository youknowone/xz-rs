use crate::types::*;

use super::c::{c_alloc_bytes, c_alloc_zeroed_bytes, c_allocator_ptr, c_free_ptr};
use super::rust::rust_allocator_ptr;
use crate::raw_alloc::{RUST_ALLOC_ALIGN, alloc_impl, free_ptr};

fn c_size(size: size_t) -> size_t {
    if size == 0 { 1 } else { size }
}

pub(crate) fn allocator_or_rust(allocator: *const lzma_allocator) -> *const lzma_allocator {
    if allocator.is_null() {
        rust_allocator_ptr()
    } else {
        allocator
    }
}

pub fn allocator_or_c(allocator: *const lzma_allocator) -> *const lzma_allocator {
    if allocator.is_null() {
        c_allocator_ptr()
    } else {
        allocator
    }
}

pub unsafe fn lzma_alloc(size: size_t, allocator: *const lzma_allocator) -> *mut c_void {
    let size = c_size(size);
    if !allocator.is_null()
        && let Some(alloc) = unsafe { (*allocator).alloc }
    {
        return unsafe { alloc((*allocator).opaque, 1, size) };
    }
    unsafe { c_alloc_bytes(size) }
}

pub unsafe fn lzma_alloc_zero(size: size_t, allocator: *const lzma_allocator) -> *mut c_void {
    let size = c_size(size);
    if !allocator.is_null()
        && let Some(alloc) = unsafe { (*allocator).alloc }
    {
        let ptr = unsafe { alloc((*allocator).opaque, 1, size) };
        if !ptr.is_null() {
            unsafe { core::ptr::write_bytes(ptr as *mut u8, 0, size) };
        }
        return ptr;
    }
    unsafe { c_alloc_zeroed_bytes(size) }
}

pub unsafe fn lzma_free(ptr: *mut c_void, allocator: *const lzma_allocator) {
    if !allocator.is_null()
        && let Some(free) = unsafe { (*allocator).free }
    {
        unsafe { free((*allocator).opaque, ptr) };
        return;
    }
    unsafe { c_free_ptr(ptr) };
}

pub(crate) unsafe fn internal_alloc_bytes(
    size: size_t,
    allocator: *const lzma_allocator,
) -> *mut c_void {
    let size = c_size(size);
    let allocator = allocator_or_rust(allocator);
    if let Some(alloc) = unsafe { (*allocator).alloc } {
        return unsafe { alloc((*allocator).opaque, 1, size) };
    }
    alloc_impl(size as usize, RUST_ALLOC_ALIGN, false)
}

pub(crate) unsafe fn internal_alloc_object<T>(allocator: *const lzma_allocator) -> *mut T {
    if !allocator.is_null()
        && let Some(alloc) = unsafe { (*allocator).alloc }
    {
        return unsafe {
            alloc((*allocator).opaque, 1, core::mem::size_of::<T>() as size_t) as *mut T
        };
    }
    alloc_impl(core::mem::size_of::<T>(), core::mem::align_of::<T>(), false) as *mut T
}

pub(crate) unsafe fn internal_alloc_array<T>(
    count: size_t,
    allocator: *const lzma_allocator,
) -> *mut T {
    let Some(size) = (count as usize).checked_mul(core::mem::size_of::<T>()) else {
        return core::ptr::null_mut();
    };
    if !allocator.is_null()
        && let Some(alloc) = unsafe { (*allocator).alloc }
    {
        return unsafe { alloc((*allocator).opaque, 1, size as size_t) as *mut T };
    }
    alloc_impl(size, core::mem::align_of::<T>(), false) as *mut T
}

pub(crate) unsafe fn internal_alloc_zeroed_array<T>(
    count: size_t,
    allocator: *const lzma_allocator,
) -> *mut T {
    let Some(size) = (count as usize).checked_mul(core::mem::size_of::<T>()) else {
        return core::ptr::null_mut();
    };
    if !allocator.is_null()
        && let Some(alloc) = unsafe { (*allocator).alloc }
    {
        let ptr = unsafe { alloc((*allocator).opaque, 1, size as size_t) as *mut T };
        if !ptr.is_null() {
            unsafe { core::ptr::write_bytes(ptr as *mut u8, 0, size) };
        }
        return ptr;
    }
    alloc_impl(size, core::mem::align_of::<T>(), true) as *mut T
}

pub(crate) unsafe fn internal_free_bytes(ptr: *mut c_void, allocator: *const lzma_allocator) {
    if !allocator.is_null()
        && let Some(free) = unsafe { (*allocator).free }
    {
        unsafe { free((*allocator).opaque, ptr) };
        return;
    }
    unsafe { free_ptr(ptr) };
}

pub(crate) unsafe fn internal_free<T>(ptr: *mut T, allocator: *const lzma_allocator) {
    unsafe { internal_free_bytes(ptr.cast(), allocator) };
}

pub(crate) unsafe fn internal_free_array<T>(
    ptr: *mut T,
    _count: size_t,
    allocator: *const lzma_allocator,
) {
    unsafe { internal_free_bytes(ptr.cast(), allocator) };
}
