use crate::types::*;
use core::ffi::c_void;
extern "C" {
    fn lzma_next_filter_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
    ) -> lzma_ret;
    fn lzma_next_end(next: *mut lzma_next_coder, allocator: *const lzma_allocator);
}
pub const LZMA_DELTA_DIST_MIN: u32 = 1;
unsafe extern "C" fn delta_coder_end(coder_ptr: *mut c_void, allocator: *const lzma_allocator) {
    let coder: *mut lzma_delta_coder = coder_ptr as *mut lzma_delta_coder;
    lzma_next_end(&raw mut (*coder).next, allocator);
    lzma_free(coder as *mut c_void, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_delta_coder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    let mut coder: *mut lzma_delta_coder = (*next).coder as *mut lzma_delta_coder;
    if coder.is_null() {
        coder = lzma_alloc(core::mem::size_of::<lzma_delta_coder>(), allocator)
            as *mut lzma_delta_coder;
        if coder.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*next).coder = coder as *mut c_void;
        (*next).end =
            Some(delta_coder_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ())
                as lzma_end_function;
        (*coder).next = lzma_next_coder_s {
            coder: core::ptr::null_mut(),
            id: LZMA_VLI_UNKNOWN,
            init: 0,
            code: None,
            end: None,
            get_progress: None,
            get_check: None,
            memconfig: None,
            update: None,
            set_out_limit: None,
        };
    }
    if lzma_delta_coder_memusage((*filters.offset(0)).options) == UINT64_MAX {
        return LZMA_OPTIONS_ERROR;
    }
    let opt: *const lzma_options_delta = (*filters.offset(0)).options as *const lzma_options_delta;
    (*coder).distance = (*opt).dist as size_t;
    (*coder).pos = 0;
    memset(&raw mut (*coder).history as *mut u8 as *mut c_void, 0, 256);
    return lzma_next_filter_init(&raw mut (*coder).next, allocator, filters.offset(1));
}
#[no_mangle]
pub unsafe extern "C" fn lzma_delta_coder_memusage(options: *const c_void) -> u64 {
    let opt: *const lzma_options_delta = options as *const lzma_options_delta;
    if opt.is_null()
        || (*opt).type_0 != LZMA_DELTA_TYPE_BYTE
        || (*opt).dist < LZMA_DELTA_DIST_MIN
        || (*opt).dist > LZMA_DELTA_DIST_MAX
    {
        return UINT64_MAX;
    }
    return core::mem::size_of::<lzma_delta_coder>() as u64;
}
