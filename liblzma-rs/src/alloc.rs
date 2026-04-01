use crate::types::*;
use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};

const RUST_ALLOC_ALIGN: usize = 16;

#[derive(Copy, Clone)]
#[repr(C)]
struct RustAllocHeader {
    total_size: usize,
    align: usize,
    offset: usize,
}

const fn round_up(value: usize, align: usize) -> usize {
    (value + (align - 1)) & !(align - 1)
}

unsafe fn rust_alloc_impl(size: usize, align: usize, zeroed: bool) -> *mut c_void {
    let size = if size == 0 { 1 } else { size };
    let align = align
        .max(RUST_ALLOC_ALIGN)
        .max(core::mem::align_of::<RustAllocHeader>());
    let header_size = core::mem::size_of::<RustAllocHeader>();
    let offset = match header_size.checked_add(align - 1) {
        Some(value) => round_up(value, align),
        None => return core::ptr::null_mut(),
    };
    let total_size = match offset.checked_add(size) {
        Some(total_size) => total_size,
        None => return core::ptr::null_mut(),
    };
    let layout = match Layout::from_size_align(total_size, align) {
        Ok(layout) => layout,
        Err(_) => return core::ptr::null_mut(),
    };
    let base = if zeroed {
        alloc_zeroed(layout)
    } else {
        alloc(layout)
    };
    if base.is_null() {
        return core::ptr::null_mut();
    }
    let user_ptr = base.add(offset);
    let header_ptr = user_ptr.sub(header_size) as *mut RustAllocHeader;
    header_ptr.write(RustAllocHeader {
        total_size,
        align,
        offset,
    });
    user_ptr as *mut c_void
}

unsafe fn rust_free_impl(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let header_size = core::mem::size_of::<RustAllocHeader>();
    let user_ptr = ptr as *mut u8;
    let header_ptr = user_ptr.sub(header_size) as *const RustAllocHeader;
    let header = header_ptr.read();
    let base = user_ptr.sub(header.offset);
    let layout = Layout::from_size_align_unchecked(header.total_size, header.align);
    dealloc(base, layout);
}

pub unsafe extern "C" fn lzma_rust_alloc(
    _opaque: *mut c_void,
    nmemb: size_t,
    size: size_t,
) -> *mut c_void {
    let size = match (nmemb as usize).checked_mul(size as usize) {
        Some(size) => size,
        None => return core::ptr::null_mut(),
    };
    rust_alloc_impl(size, RUST_ALLOC_ALIGN, false)
}

pub unsafe extern "C" fn lzma_rust_free(_opaque: *mut c_void, ptr: *mut c_void) {
    rust_free_impl(ptr);
}

pub fn rust_allocator() -> lzma_allocator {
    lzma_allocator {
        alloc: Some(lzma_rust_alloc),
        free: Some(lzma_rust_free),
        opaque: core::ptr::null_mut(),
    }
}

pub(crate) unsafe fn internal_alloc_bytes(
    size: size_t,
    allocator: *const lzma_allocator,
) -> *mut c_void {
    let size = if size == 0 { 1 } else { size };
    if !allocator.is_null() && (*allocator).alloc.is_some() {
        (*allocator).alloc.unwrap()((*allocator).opaque, 1, size)
    } else {
        rust_alloc_impl(size as usize, RUST_ALLOC_ALIGN, false)
    }
}

pub(crate) unsafe fn internal_alloc_zeroed_bytes(
    size: size_t,
    allocator: *const lzma_allocator,
) -> *mut c_void {
    let size = if size == 0 { 1 } else { size };
    if !allocator.is_null() && (*allocator).alloc.is_some() {
        let ptr = (*allocator).alloc.unwrap()((*allocator).opaque, 1, size);
        if !ptr.is_null() {
            core::ptr::write_bytes(ptr as *mut u8, 0, size);
        }
        ptr
    } else {
        rust_alloc_impl(size as usize, RUST_ALLOC_ALIGN, true)
    }
}

pub(crate) unsafe fn internal_alloc_object<T>(allocator: *const lzma_allocator) -> *mut T {
    if !allocator.is_null() && (*allocator).alloc.is_some() {
        return (*allocator).alloc.unwrap()(
            (*allocator).opaque,
            1,
            core::mem::size_of::<T>() as size_t,
        ) as *mut T;
    }
    rust_alloc_impl(core::mem::size_of::<T>(), core::mem::align_of::<T>(), false) as *mut T
}

pub(crate) unsafe fn internal_alloc_array<T>(
    count: size_t,
    allocator: *const lzma_allocator,
) -> *mut T {
    let size = match (count as usize).checked_mul(core::mem::size_of::<T>()) {
        Some(size) => size,
        None => return core::ptr::null_mut(),
    };
    if !allocator.is_null() && (*allocator).alloc.is_some() {
        return (*allocator).alloc.unwrap()((*allocator).opaque, 1, size as size_t) as *mut T;
    }
    rust_alloc_impl(size, core::mem::align_of::<T>(), false) as *mut T
}

pub(crate) unsafe fn internal_alloc_zeroed_array<T>(
    count: size_t,
    allocator: *const lzma_allocator,
) -> *mut T {
    let size = match (count as usize).checked_mul(core::mem::size_of::<T>()) {
        Some(size) => size,
        None => return core::ptr::null_mut(),
    };
    if !allocator.is_null() && (*allocator).alloc.is_some() {
        let ptr = (*allocator).alloc.unwrap()((*allocator).opaque, 1, size as size_t) as *mut T;
        if !ptr.is_null() {
            core::ptr::write_bytes(ptr as *mut u8, 0, size);
        }
        return ptr;
    }
    rust_alloc_impl(size, core::mem::align_of::<T>(), true) as *mut T
}

pub(crate) unsafe fn internal_free(ptr: *mut c_void, allocator: *const lzma_allocator) {
    if !allocator.is_null() && (*allocator).free.is_some() {
        (*allocator).free.unwrap()((*allocator).opaque, ptr);
    } else {
        rust_free_impl(ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_allocator_round_trip() {
        unsafe {
            let allocator = rust_allocator();
            let ptr = allocator.alloc.unwrap()(allocator.opaque, 4, 8);
            assert!(!ptr.is_null());
            allocator.free.unwrap()(allocator.opaque, ptr);
        }
    }

    #[test]
    fn internal_object_allocation_respects_alignment() {
        #[repr(align(32))]
        struct Align32([u8; 32]);

        unsafe {
            let ptr = internal_alloc_object::<Align32>(core::ptr::null());
            assert!(!ptr.is_null());
            assert_eq!((ptr as usize) % core::mem::align_of::<Align32>(), 0);
            internal_free(ptr.cast(), core::ptr::null());
        }
    }
}
