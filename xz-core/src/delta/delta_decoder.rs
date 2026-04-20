use crate::types::*;
fn decode_buffer(coder: &mut lzma_delta_coder, buffer: &mut [u8]) {
    let distance: size_t = coder.distance;
    let history = coder.history.as_mut_ptr();
    let mut pos = coder.pos;
    let mut i: usize = 0;
    while i < buffer.len() {
        let byte = buffer[i].wrapping_add(unsafe {
            *history.add((distance.wrapping_add(pos as size_t) & 0xff) as usize)
        });
        buffer[i] = byte;
        unsafe {
            *history.add((pos & 0xff) as usize) = byte;
        }
        pos = pos.wrapping_sub(1);
        i += 1;
    }
    coder.pos = pos;
}
unsafe fn delta_decode(
    coder_ptr: *mut c_void,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_pos: *mut size_t,
    in_size: size_t,
    out: *mut u8,
    out_pos: *mut size_t,
    out_size: size_t,
    action: lzma_action,
) -> lzma_ret {
    let coder: &mut lzma_delta_coder = &mut *(coder_ptr as *mut lzma_delta_coder);
    let out_start: size_t = *out_pos;
    debug_assert!(coder.next.code.is_some());
    let code = coder.next.code.unwrap_unchecked();
    let ret: lzma_ret = code(
        coder.next.coder,
        allocator,
        input,
        in_pos,
        in_size,
        out,
        out_pos,
        out_size,
        action,
    );
    debug_assert!(*out_pos >= out_start);
    let size: size_t = *out_pos - out_start;
    if size > 0 {
        decode_buffer(
            coder,
            core::slice::from_raw_parts_mut(out.add(out_start), size),
        );
    }
    ret
}
pub(crate) unsafe fn lzma_delta_decoder_init(
    next: *mut lzma_next_coder,
    allocator: *const lzma_allocator,
    filters: *const lzma_filter_info,
) -> lzma_ret {
    (*next).code = Some(
        delta_decode
            as unsafe fn(
                *mut c_void,
                *const lzma_allocator,
                *const u8,
                *mut size_t,
                size_t,
                *mut u8,
                *mut size_t,
                size_t,
                lzma_action,
            ) -> lzma_ret,
    );
    lzma_delta_coder_init(next, allocator, filters)
}
pub(crate) unsafe fn lzma_delta_props_decode(
    options: *mut *mut c_void,
    allocator: *const lzma_allocator,
    props: *const u8,
    props_size: size_t,
) -> lzma_ret {
    if props_size != 1 {
        return LZMA_OPTIONS_ERROR;
    }
    let opt: *mut lzma_options_delta =
        lzma_alloc(core::mem::size_of::<lzma_options_delta>(), allocator)
            as *mut lzma_options_delta;
    if opt.is_null() {
        return LZMA_MEM_ERROR;
    }
    (*opt).type_ = LZMA_DELTA_TYPE_BYTE;
    (*opt).dist = u32::from(*props) + 1;
    *options = opt as *mut c_void;
    LZMA_OK
}
