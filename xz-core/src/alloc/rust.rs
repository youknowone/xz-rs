use crate::types::*;
use std::alloc::{Layout, alloc, alloc_zeroed, dealloc};

pub(super) const RUST_ALLOC_ALIGN: usize = 16;
const ZERO_SIZE_PTR: *mut c_void = core::ptr::without_provenance_mut(RUST_ALLOC_ALIGN);

#[derive(Copy, Clone)]
#[repr(C)]
struct RustAllocHeader {
    total_size: usize,
    align: usize,
    offset: usize,
}

#[repr(transparent)]
struct StaticAllocator(lzma_allocator);

unsafe impl Sync for StaticAllocator {}

static RUST_ALLOCATOR: StaticAllocator = StaticAllocator(lzma_allocator {
    alloc: Some(lzma_rust_alloc),
    free: Some(lzma_rust_free),
    opaque: core::ptr::null_mut(),
});

const fn round_up(value: usize, align: usize) -> usize {
    (value + (align - 1)) & !(align - 1)
}

pub(super) fn rust_alloc_impl(size: usize, align: usize, zeroed: bool) -> *mut c_void {
    if size == 0 {
        return ZERO_SIZE_PTR;
    }
    let align = align
        .max(RUST_ALLOC_ALIGN)
        .max(core::mem::align_of::<RustAllocHeader>());
    let header_size = core::mem::size_of::<RustAllocHeader>();
    let Some(value) = header_size.checked_add(align - 1) else {
        return core::ptr::null_mut();
    };
    let offset = round_up(value, align);
    let Some(total_size) = offset.checked_add(size) else {
        return core::ptr::null_mut();
    };
    let Ok(layout) = Layout::from_size_align(total_size, align) else {
        return core::ptr::null_mut();
    };
    let base = unsafe {
        if zeroed {
            alloc_zeroed(layout)
        } else {
            alloc(layout)
        }
    };
    if base.is_null() {
        return core::ptr::null_mut();
    }
    let user_ptr = unsafe { base.add(offset) };
    let header_ptr = unsafe { user_ptr.sub(header_size) as *mut RustAllocHeader };
    unsafe {
        header_ptr.write(RustAllocHeader {
            total_size,
            align,
            offset,
        });
    }
    user_ptr as *mut c_void
}

pub(super) unsafe fn rust_free_impl(ptr: *mut c_void) {
    if ptr.is_null() || ptr == ZERO_SIZE_PTR {
        return;
    }
    let header_size = core::mem::size_of::<RustAllocHeader>();
    let user_ptr = ptr as *mut u8;
    let header_ptr = unsafe { user_ptr.sub(header_size) as *const RustAllocHeader };
    let header = unsafe { header_ptr.read() };
    let base = unsafe { user_ptr.sub(header.offset) };
    let layout = unsafe { Layout::from_size_align_unchecked(header.total_size, header.align) };
    unsafe { dealloc(base, layout) };
}

pub(crate) unsafe fn lzma_rust_alloc(
    _opaque: *mut c_void,
    nmemb: size_t,
    size: size_t,
) -> *mut c_void {
    let Some(size) = (nmemb as usize).checked_mul(size as usize) else {
        return core::ptr::null_mut();
    };
    rust_alloc_impl(size, RUST_ALLOC_ALIGN, false)
}

pub(crate) unsafe fn lzma_rust_free(_opaque: *mut c_void, ptr: *mut c_void) {
    unsafe { rust_free_impl(ptr) };
}

pub(super) unsafe fn rust_alloc_bytes(size: size_t) -> *mut c_void {
    rust_alloc_impl(size as usize, RUST_ALLOC_ALIGN, false)
}

pub(super) unsafe fn rust_alloc_zeroed_bytes(size: size_t) -> *mut c_void {
    rust_alloc_impl(size as usize, RUST_ALLOC_ALIGN, true)
}

pub(super) unsafe fn rust_free_ptr(ptr: *mut c_void) {
    unsafe { rust_free_impl(ptr) };
}

pub fn rust_allocator() -> lzma_allocator {
    RUST_ALLOCATOR.0
}

pub(super) fn rust_allocator_ptr() -> *const lzma_allocator {
    &raw const RUST_ALLOCATOR.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_allocator_round_trip() {
        unsafe {
            let ptr = lzma_rust_alloc(core::ptr::null_mut(), 4, 8);
            assert!(!ptr.is_null());
            lzma_rust_free(core::ptr::null_mut(), ptr);
        }
    }

    #[test]
    fn rust_allocation_respects_alignment() {
        #[repr(align(32))]
        struct Align32([u8; 32]);

        unsafe {
            let ptr = rust_alloc_impl(
                core::mem::size_of::<Align32>(),
                core::mem::align_of::<Align32>(),
                false,
            );
            assert!(!ptr.is_null());
            assert_eq!((ptr as usize) % core::mem::align_of::<Align32>(), 0);
            rust_free_impl(ptr);
        }
    }
}
