use crate::types::*;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use std::alloc::{Layout, alloc, alloc_zeroed, dealloc};

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
const C_ALLOC_ALIGN: usize = 16;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
const C_ALLOC_HEADER_SIZE: usize = 16;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
unsafe extern "C" {
    fn malloc(__size: size_t) -> *mut c_void;
    fn calloc(__count: size_t, __size: size_t) -> *mut c_void;
    fn free(__ptr: *mut c_void);
}

#[repr(transparent)]
struct StaticAllocator(lzma_allocator);

unsafe impl Sync for StaticAllocator {}

static C_ALLOCATOR: StaticAllocator = StaticAllocator(lzma_allocator {
    alloc: Some(lzma_c_alloc),
    free: Some(lzma_c_free),
    opaque: core::ptr::null_mut(),
});

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
fn c_alloc_layout(size: usize) -> Option<Layout> {
    let total_size = size.checked_add(C_ALLOC_HEADER_SIZE)?;
    Layout::from_size_align(total_size, C_ALLOC_ALIGN).ok()
}

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
unsafe fn c_malloc(size: size_t) -> *mut c_void {
    unsafe { malloc(size) }
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
unsafe fn c_malloc(size: size_t) -> *mut c_void {
    let Some(layout) = c_alloc_layout(size as usize) else {
        return core::ptr::null_mut();
    };
    let base = unsafe { alloc(layout) };
    if base.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        *(base as *mut usize) = layout.size();
        base.add(C_ALLOC_HEADER_SIZE) as *mut c_void
    }
}

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
unsafe fn c_calloc(count: size_t, size: size_t) -> *mut c_void {
    unsafe { calloc(count, size) }
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
unsafe fn c_calloc(count: size_t, size: size_t) -> *mut c_void {
    let Some(size) = (count as usize).checked_mul(size as usize) else {
        return core::ptr::null_mut();
    };
    let Some(layout) = c_alloc_layout(size) else {
        return core::ptr::null_mut();
    };
    let base = unsafe { alloc_zeroed(layout) };
    if base.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        *(base as *mut usize) = layout.size();
        base.add(C_ALLOC_HEADER_SIZE) as *mut c_void
    }
}

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
unsafe fn c_free(ptr: *mut c_void) {
    unsafe { free(ptr) };
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
unsafe fn c_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let base = unsafe { (ptr as *mut u8).sub(C_ALLOC_HEADER_SIZE) };
    let total_size = unsafe { *(base as *const usize) };
    let layout = unsafe { Layout::from_size_align_unchecked(total_size, C_ALLOC_ALIGN) };
    unsafe { dealloc(base, layout) };
}

pub unsafe fn lzma_c_alloc(_opaque: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void {
    let Some(size) = (nmemb as usize).checked_mul(size as usize) else {
        return core::ptr::null_mut();
    };
    let size = if size == 0 { 1 } else { size };
    unsafe { c_malloc(size as size_t) }
}

pub unsafe fn lzma_c_free(_opaque: *mut c_void, ptr: *mut c_void) {
    unsafe { c_free(ptr) };
}

pub fn c_allocator() -> lzma_allocator {
    C_ALLOCATOR.0
}

pub fn c_allocator_ptr() -> *const lzma_allocator {
    &raw const C_ALLOCATOR.0
}

pub(super) unsafe fn c_alloc_bytes(size: size_t) -> *mut c_void {
    let size = if size == 0 { 1 } else { size };
    unsafe { c_malloc(size) }
}

pub(super) unsafe fn c_alloc_zeroed_bytes(size: size_t) -> *mut c_void {
    let size = if size == 0 { 1 } else { size };
    unsafe { c_calloc(1, size) }
}

pub(super) unsafe fn c_free_ptr(ptr: *mut c_void) {
    unsafe { c_free(ptr) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_allocator_round_trip() {
        unsafe {
            let ptr = lzma_c_alloc(core::ptr::null_mut(), 4, 8);
            assert!(!ptr.is_null());
            lzma_c_free(core::ptr::null_mut(), ptr);
        }
    }
}
