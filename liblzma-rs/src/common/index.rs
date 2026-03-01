use crate::types::*;
use core::ffi::{c_int, c_uint, c_ulong, c_ulonglong, c_void};
extern "C" {
    fn lzma_vli_size(vli: lzma_vli) -> u32;
    fn lzma_stream_flags_compare(
        a: *const lzma_stream_flags,
        b: *const lzma_stream_flags,
    ) -> lzma_ret;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_index_s {
    pub streams: index_tree,
    pub uncompressed_size: lzma_vli,
    pub total_size: lzma_vli,
    pub record_count: lzma_vli,
    pub index_list_size: lzma_vli,
    pub prealloc: size_t,
    pub checks: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct index_tree {
    pub root: *mut index_tree_node,
    pub leftmost: *mut index_tree_node,
    pub rightmost: *mut index_tree_node,
    pub count: u32,
}
pub type index_tree_node = index_tree_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct index_tree_node_s {
    pub uncompressed_base: lzma_vli,
    pub compressed_base: lzma_vli,
    pub parent: *mut index_tree_node,
    pub left: *mut index_tree_node,
    pub right: *mut index_tree_node,
}
pub type lzma_index = lzma_index_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_index_iter {
    pub stream: C2RustUnnamed_1,
    pub block: C2RustUnnamed_0,
    pub internal: [C2RustUnnamed; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub p: *const c_void,
    pub s: size_t,
    pub v: lzma_vli,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub number_in_file: lzma_vli,
    pub compressed_file_offset: lzma_vli,
    pub uncompressed_file_offset: lzma_vli,
    pub number_in_stream: lzma_vli,
    pub compressed_stream_offset: lzma_vli,
    pub uncompressed_stream_offset: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub unpadded_size: lzma_vli,
    pub total_size: lzma_vli,
    pub reserved_vli1: lzma_vli,
    pub reserved_vli2: lzma_vli,
    pub reserved_vli3: lzma_vli,
    pub reserved_vli4: lzma_vli,
    pub reserved_ptr1: *const c_void,
    pub reserved_ptr2: *const c_void,
    pub reserved_ptr3: *const c_void,
    pub reserved_ptr4: *const c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub flags: *const lzma_stream_flags,
    pub reserved_ptr1: *const c_void,
    pub reserved_ptr2: *const c_void,
    pub reserved_ptr3: *const c_void,
    pub number: lzma_vli,
    pub block_count: lzma_vli,
    pub compressed_offset: lzma_vli,
    pub uncompressed_offset: lzma_vli,
    pub compressed_size: lzma_vli,
    pub uncompressed_size: lzma_vli,
    pub padding: lzma_vli,
    pub reserved_vli1: lzma_vli,
    pub reserved_vli2: lzma_vli,
    pub reserved_vli3: lzma_vli,
    pub reserved_vli4: lzma_vli,
}
pub type lzma_index_iter_mode = c_uint;
pub const LZMA_INDEX_ITER_NONEMPTY_BLOCK: lzma_index_iter_mode = 3;
pub const LZMA_INDEX_ITER_BLOCK: lzma_index_iter_mode = 2;
pub const LZMA_INDEX_ITER_STREAM: lzma_index_iter_mode = 1;
pub const LZMA_INDEX_ITER_ANY: lzma_index_iter_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct index_record {
    pub uncompressed_sum: lzma_vli,
    pub unpadded_sum: lzma_vli,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct index_group {
    pub node: index_tree_node,
    pub number_base: lzma_vli,
    pub allocated: size_t,
    pub last: size_t,
    pub records: [index_record; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct index_stream {
    pub node: index_tree_node,
    pub number: u32,
    pub block_number_base: lzma_vli,
    pub groups: index_tree,
    pub record_count: lzma_vli,
    pub index_list_size: lzma_vli,
    pub stream_flags: lzma_stream_flags,
    pub stream_padding: lzma_vli,
}
pub const ITER_METHOD_NORMAL: C2RustUnnamed_3 = 0;
pub const ITER_METHOD: C2RustUnnamed_2 = 4;
pub const ITER_RECORD: C2RustUnnamed_2 = 3;
pub const ITER_GROUP: C2RustUnnamed_2 = 2;
pub const ITER_STREAM: C2RustUnnamed_2 = 1;
pub const ITER_INDEX: C2RustUnnamed_2 = 0;
pub const ITER_METHOD_LEFTMOST: C2RustUnnamed_3 = 2;
pub const ITER_METHOD_NEXT: C2RustUnnamed_3 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct index_cat_info {
    pub uncompressed_size: lzma_vli,
    pub file_size: lzma_vli,
    pub block_number_add: lzma_vli,
    pub stream_number_add: u32,
    pub streams: *mut index_tree,
}
pub type C2RustUnnamed_2 = c_uint;
pub type C2RustUnnamed_3 = c_uint;
pub const UINTPTR_MAX: c_ulong = uintptr_t::MAX as c_ulong;
pub const SIZE_MAX: c_ulong = UINTPTR_MAX;
#[inline]
extern "C" fn bsr32(n: u32) -> u32 {
    return n.leading_zeros() as i32 as u32 ^ 31;
}
#[inline]
extern "C" fn ctz32(n: u32) -> u32 {
    return n.trailing_zeros() as i32 as u32;
}
pub const UNPADDED_SIZE_MIN: c_ulonglong = 5;
pub const UNPADDED_SIZE_MAX: c_ulonglong = LZMA_VLI_MAX & !3;
#[inline]
extern "C" fn vli_ceil4(vli: lzma_vli) -> lzma_vli {
    return vli.wrapping_add(3) & !(3);
}
#[inline]
extern "C" fn index_size_unpadded(count: lzma_vli, index_list_size: lzma_vli) -> lzma_vli {
    return (1u32.wrapping_add(unsafe { lzma_vli_size(count) }) as lzma_vli)
        .wrapping_add(index_list_size)
        .wrapping_add(4);
}
#[inline]
extern "C" fn index_size(count: lzma_vli, index_list_size: lzma_vli) -> lzma_vli {
    return vli_ceil4(index_size_unpadded(count, index_list_size));
}
pub const INDEX_GROUP_SIZE: c_int = 512;
pub const PREALLOC_MAX: usize = (SIZE_MAX as usize)
    .wrapping_sub(core::mem::size_of::<index_group>() as usize)
    .wrapping_div(core::mem::size_of::<index_record>() as usize);
unsafe extern "C" fn index_tree_init(tree: *mut index_tree) {
    (*tree).root = core::ptr::null_mut();
    (*tree).leftmost = core::ptr::null_mut();
    (*tree).rightmost = core::ptr::null_mut();
    (*tree).count = 0;
}
unsafe extern "C" fn index_tree_node_end(
    node: *mut index_tree_node,
    allocator: *const lzma_allocator,
    free_func: Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>,
) {
    if !(*node).left.is_null() {
        index_tree_node_end((*node).left, allocator, free_func);
    }
    if !(*node).right.is_null() {
        index_tree_node_end((*node).right, allocator, free_func);
    }
    free_func.expect("non-null function pointer")(node as *mut c_void, allocator);
}
unsafe extern "C" fn index_tree_end(
    tree: *mut index_tree,
    allocator: *const lzma_allocator,
    free_func: Option<unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()>,
) {
    if !(*tree).root.is_null() {
        index_tree_node_end((*tree).root, allocator, free_func);
    }
}
unsafe extern "C" fn index_tree_append(tree: *mut index_tree, mut node: *mut index_tree_node) {
    (*node).parent = (*tree).rightmost;
    (*node).left = core::ptr::null_mut();
    (*node).right = core::ptr::null_mut();
    (*tree).count = (*tree).count.wrapping_add(1);
    if (*tree).root.is_null() {
        (*tree).root = node;
        (*tree).leftmost = node;
        (*tree).rightmost = node;
        return;
    }
    (*(*tree).rightmost).right = node;
    (*tree).rightmost = node;
    let mut up: u32 = (*tree).count ^ (1) << bsr32((*tree).count);
    if up != 0 {
        up = ctz32((*tree).count).wrapping_add(2);
        loop {
            node = (*node).parent;
            up = up.wrapping_sub(1);
            if !(up > 0) {
                break;
            }
        }
        let pivot: *mut index_tree_node = (*node).right;
        if (*node).parent.is_null() {
            (*tree).root = pivot;
        } else {
            (*(*node).parent).right = pivot;
        }
        (*pivot).parent = (*node).parent;
        (*node).right = (*pivot).left;
        if !(*node).right.is_null() {
            (*(*node).right).parent = node;
        }
        (*pivot).left = node;
        (*node).parent = pivot;
    }
}
unsafe extern "C" fn index_tree_next(mut node: *const index_tree_node) -> *mut c_void {
    if !(*node).right.is_null() {
        node = (*node).right;
        while !(*node).left.is_null() {
            node = (*node).left;
        }
        return node as *mut c_void;
    }
    while !(*node).parent.is_null() && (*(*node).parent).right == node as *mut index_tree_node {
        node = (*node).parent;
    }
    return (*node).parent as *mut c_void;
}
unsafe extern "C" fn index_tree_locate(tree: *const index_tree, target: lzma_vli) -> *mut c_void {
    let mut result: *const index_tree_node = ::core::ptr::null::<index_tree_node>();
    let mut node: *const index_tree_node = (*tree).root;
    while !node.is_null() {
        if (*node).uncompressed_base > target {
            node = (*node).left;
        } else {
            result = node;
            node = (*node).right;
        }
    }
    return result as *mut c_void;
}
unsafe extern "C" fn index_stream_init(
    compressed_base: lzma_vli,
    uncompressed_base: lzma_vli,
    stream_number: u32,
    block_number_base: lzma_vli,
    allocator: *const lzma_allocator,
) -> *mut index_stream {
    let s: *mut index_stream =
        lzma_alloc(core::mem::size_of::<index_stream>(), allocator) as *mut index_stream;
    if s.is_null() {
        return core::ptr::null_mut();
    }
    (*s).node.uncompressed_base = uncompressed_base;
    (*s).node.compressed_base = compressed_base;
    (*s).node.parent = core::ptr::null_mut();
    (*s).node.left = core::ptr::null_mut();
    (*s).node.right = core::ptr::null_mut();
    (*s).number = stream_number;
    (*s).block_number_base = block_number_base;
    index_tree_init(&raw mut (*s).groups);
    (*s).record_count = 0;
    (*s).index_list_size = 0;
    (*s).stream_flags.version = UINT32_MAX;
    (*s).stream_padding = 0;
    return s;
}
unsafe extern "C" fn index_stream_end(node: *mut c_void, allocator: *const lzma_allocator) {
    let s: *mut index_stream = node as *mut index_stream;
    index_tree_end(
        &raw mut (*s).groups,
        allocator,
        Some(lzma_free as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> ()),
    );
    lzma_free(s as *mut c_void, allocator);
}
unsafe extern "C" fn index_init_plain(allocator: *const lzma_allocator) -> *mut lzma_index {
    let i: *mut lzma_index =
        lzma_alloc(core::mem::size_of::<lzma_index>(), allocator) as *mut lzma_index;
    if !i.is_null() {
        index_tree_init(&raw mut (*i).streams);
        (*i).uncompressed_size = 0;
        (*i).total_size = 0;
        (*i).record_count = 0;
        (*i).index_list_size = 0;
        (*i).prealloc = INDEX_GROUP_SIZE as size_t;
        (*i).checks = 0;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_init(allocator: *const lzma_allocator) -> *mut lzma_index {
    let i: *mut lzma_index = index_init_plain(allocator);
    if i.is_null() {
        return core::ptr::null_mut();
    }
    let s: *mut index_stream = index_stream_init(0, 0, 1, 0, allocator);
    if s.is_null() {
        lzma_free(i as *mut c_void, allocator);
        return core::ptr::null_mut();
    }
    index_tree_append(&raw mut (*i).streams, &raw mut (*s).node);
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_end(i: *mut lzma_index, allocator: *const lzma_allocator) {
    if !i.is_null() {
        index_tree_end(
            &raw mut (*i).streams,
            allocator,
            Some(
                index_stream_end as unsafe extern "C" fn(*mut c_void, *const lzma_allocator) -> (),
            ),
        );
        lzma_free(i as *mut c_void, allocator);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_prealloc(i: *mut lzma_index, mut records: lzma_vli) {
    if records > PREALLOC_MAX as lzma_vli {
        records = PREALLOC_MAX as lzma_vli;
    }
    (*i).prealloc = records as size_t;
}
#[no_mangle]
pub extern "C" fn lzma_index_memusage(streams: lzma_vli, blocks: lzma_vli) -> u64 {
    let alloc_overhead: size_t = (4_usize).wrapping_mul(core::mem::size_of::<*mut c_void>());
    let stream_base: size_t = (core::mem::size_of::<index_stream>())
        .wrapping_add(core::mem::size_of::<index_group>())
        .wrapping_add((2_usize).wrapping_mul(alloc_overhead));
    let group_base: size_t = (core::mem::size_of::<index_group>())
        .wrapping_add(
            (INDEX_GROUP_SIZE as size_t).wrapping_mul(core::mem::size_of::<index_record>()),
        )
        .wrapping_add(alloc_overhead);
    let groups: lzma_vli = blocks
        .wrapping_add(INDEX_GROUP_SIZE as lzma_vli)
        .wrapping_sub(1)
        .wrapping_div(INDEX_GROUP_SIZE as lzma_vli);
    let streams_mem: u64 = (streams as u64).wrapping_mul(stream_base as u64);
    let groups_mem: u64 = (groups as u64).wrapping_mul(group_base as u64);
    let index_base: u64 =
        (core::mem::size_of::<lzma_index>() as usize).wrapping_add(alloc_overhead as usize) as u64;
    let limit: u64 = (UINT64_MAX).wrapping_sub(index_base);
    if streams == 0
        || streams > UINT32_MAX as lzma_vli
        || blocks > LZMA_VLI_MAX
        || streams > limit.wrapping_div(stream_base as u64)
        || groups > limit.wrapping_div(group_base as u64)
        || limit.wrapping_sub(streams_mem) < groups_mem
    {
        return UINT64_MAX;
    }
    return index_base
        .wrapping_add(streams_mem)
        .wrapping_add(groups_mem);
}
#[no_mangle]
pub extern "C" fn lzma_index_memused(i: *const lzma_index) -> u64 {
    return unsafe { lzma_index_memusage((*i).streams.count as lzma_vli, (*i).record_count) };
}
#[no_mangle]
pub extern "C" fn lzma_index_block_count(i: *const lzma_index) -> lzma_vli {
    return unsafe { (*i).record_count };
}
#[no_mangle]
pub extern "C" fn lzma_index_stream_count(i: *const lzma_index) -> lzma_vli {
    return unsafe { (*i).streams.count as lzma_vli };
}
#[no_mangle]
pub extern "C" fn lzma_index_size(i: *const lzma_index) -> lzma_vli {
    return unsafe { index_size((*i).record_count, (*i).index_list_size) };
}
#[no_mangle]
pub extern "C" fn lzma_index_total_size(i: *const lzma_index) -> lzma_vli {
    return unsafe { (*i).total_size };
}
#[no_mangle]
pub extern "C" fn lzma_index_stream_size(i: *const lzma_index) -> lzma_vli {
    return unsafe {
        (LZMA_STREAM_HEADER_SIZE as lzma_vli)
            .wrapping_add((*i).total_size)
            .wrapping_add(index_size((*i).record_count, (*i).index_list_size))
            .wrapping_add(LZMA_STREAM_HEADER_SIZE as lzma_vli)
    };
}
unsafe extern "C" fn index_file_size(
    compressed_base: lzma_vli,
    unpadded_sum: lzma_vli,
    record_count: lzma_vli,
    index_list_size: lzma_vli,
    stream_padding: lzma_vli,
) -> lzma_vli {
    let mut file_size: lzma_vli = compressed_base
        .wrapping_add((2 * LZMA_STREAM_HEADER_SIZE) as lzma_vli)
        .wrapping_add(stream_padding)
        .wrapping_add(vli_ceil4(unpadded_sum));
    if file_size > LZMA_VLI_MAX {
        return LZMA_VLI_UNKNOWN;
    }
    file_size = file_size.wrapping_add(index_size(record_count, index_list_size));
    if file_size > LZMA_VLI_MAX {
        return LZMA_VLI_UNKNOWN;
    }
    return file_size;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_file_size(i: *const lzma_index) -> lzma_vli {
    let s: *const index_stream = (*i).streams.rightmost as *const index_stream;
    let g: *const index_group = (*s).groups.rightmost as *const index_group;
    return index_file_size(
        (*s).node.compressed_base,
        if g.is_null() {
            0
        } else {
            (*(&raw const (*g).records as *const index_record).offset((*g).last as isize))
                .unpadded_sum
        },
        (*s).record_count,
        (*s).index_list_size,
        (*s).stream_padding,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_uncompressed_size(i: *const lzma_index) -> lzma_vli {
    return (*i).uncompressed_size;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_checks(i: *const lzma_index) -> u32 {
    let mut checks: u32 = (*i).checks;
    let s: *const index_stream = (*i).streams.rightmost as *const index_stream;
    if (*s).stream_flags.version != UINT32_MAX {
        checks = (checks | 1u32 << (*s).stream_flags.check) as u32;
    }
    return checks;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_padding_size(i: *const lzma_index) -> u32 {
    return ((4_u64).wrapping_sub(index_size_unpadded((*i).record_count, (*i).index_list_size)) & 3)
        as u32;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_stream_flags(
    i: *mut lzma_index,
    stream_flags: *const lzma_stream_flags,
) -> lzma_ret {
    if i.is_null() || stream_flags.is_null() {
        return LZMA_PROG_ERROR;
    }
    let ret_: lzma_ret = lzma_stream_flags_compare(stream_flags, stream_flags);
    if ret_ != LZMA_OK {
        return ret_;
    }
    let s: *mut index_stream = (*i).streams.rightmost as *mut index_stream;
    (*s).stream_flags = *stream_flags;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_stream_padding(
    i: *mut lzma_index,
    stream_padding: lzma_vli,
) -> lzma_ret {
    if i.is_null() || stream_padding > LZMA_VLI_MAX || stream_padding & 3 != 0 {
        return LZMA_PROG_ERROR;
    }
    let s: *mut index_stream = (*i).streams.rightmost as *mut index_stream;
    let old_stream_padding: lzma_vli = (*s).stream_padding;
    (*s).stream_padding = 0;
    if lzma_index_file_size(i).wrapping_add(stream_padding) > LZMA_VLI_MAX {
        (*s).stream_padding = old_stream_padding;
        return LZMA_DATA_ERROR;
    }
    (*s).stream_padding = stream_padding;
    return LZMA_OK;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_append(
    i: *mut lzma_index,
    allocator: *const lzma_allocator,
    unpadded_size: lzma_vli,
    uncompressed_size: lzma_vli,
) -> lzma_ret {
    if i.is_null()
        || unpadded_size < UNPADDED_SIZE_MIN as lzma_vli
        || unpadded_size > UNPADDED_SIZE_MAX as lzma_vli
        || uncompressed_size > LZMA_VLI_MAX
    {
        return LZMA_PROG_ERROR;
    }
    let s: *mut index_stream = (*i).streams.rightmost as *mut index_stream;
    let mut g: *mut index_group = (*s).groups.rightmost as *mut index_group;
    let compressed_base: lzma_vli = if g.is_null() {
        0
    } else {
        vli_ceil4(
            (*(&raw mut (*g).records as *mut index_record).offset((*g).last as isize)).unpadded_sum,
        ) as lzma_vli
    };
    let uncompressed_base: lzma_vli = if g.is_null() {
        0
    } else {
        (*(&raw mut (*g).records as *mut index_record).offset((*g).last as isize)).uncompressed_sum
    };
    let index_list_size_add: u32 =
        (lzma_vli_size(unpadded_size) as u32).wrapping_add(lzma_vli_size(uncompressed_size) as u32);
    if uncompressed_base.wrapping_add(uncompressed_size) > LZMA_VLI_MAX {
        return LZMA_DATA_ERROR;
    }
    if compressed_base.wrapping_add(unpadded_size) > UNPADDED_SIZE_MAX as lzma_vli {
        return LZMA_DATA_ERROR;
    }
    if index_file_size(
        (*s).node.compressed_base,
        compressed_base.wrapping_add(unpadded_size),
        (*s).record_count.wrapping_add(1),
        (*s).index_list_size
            .wrapping_add(index_list_size_add as lzma_vli),
        (*s).stream_padding,
    ) == LZMA_VLI_UNKNOWN
    {
        return LZMA_DATA_ERROR;
    }
    if index_size(
        (*i).record_count.wrapping_add(1),
        (*i).index_list_size
            .wrapping_add(index_list_size_add as lzma_vli),
    ) > LZMA_BACKWARD_SIZE_MAX as lzma_vli
    {
        return LZMA_DATA_ERROR;
    }
    if !g.is_null() && (*g).last.wrapping_add(1) < (*g).allocated {
        (*g).last = (*g).last.wrapping_add(1);
    } else {
        g = lzma_alloc(
            (core::mem::size_of::<index_group>()).wrapping_add(
                (*i).prealloc
                    .wrapping_mul(core::mem::size_of::<index_record>()),
            ),
            allocator,
        ) as *mut index_group;
        if g.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*g).last = 0;
        (*g).allocated = (*i).prealloc;
        (*i).prealloc = INDEX_GROUP_SIZE as size_t;
        (*g).node.uncompressed_base = uncompressed_base;
        (*g).node.compressed_base = compressed_base;
        (*g).number_base = (*s).record_count.wrapping_add(1);
        index_tree_append(&raw mut (*s).groups, &raw mut (*g).node);
    }
    (*(&raw mut (*g).records as *mut index_record).offset((*g).last as isize)).uncompressed_sum =
        uncompressed_base.wrapping_add(uncompressed_size);
    (*(&raw mut (*g).records as *mut index_record).offset((*g).last as isize)).unpadded_sum =
        compressed_base.wrapping_add(unpadded_size);
    (*s).record_count = (*s).record_count.wrapping_add(1);
    (*s).index_list_size = (*s)
        .index_list_size
        .wrapping_add(index_list_size_add as lzma_vli);
    (*i).total_size = (*i).total_size.wrapping_add(vli_ceil4(unpadded_size));
    (*i).uncompressed_size = (*i).uncompressed_size.wrapping_add(uncompressed_size);
    (*i).record_count = (*i).record_count.wrapping_add(1);
    (*i).index_list_size = (*i)
        .index_list_size
        .wrapping_add(index_list_size_add as lzma_vli);
    return LZMA_OK;
}
unsafe extern "C" fn index_cat_helper(info: *const index_cat_info, this: *mut index_stream) {
    let left: *mut index_stream = (*this).node.left as *mut index_stream;
    let right: *mut index_stream = (*this).node.right as *mut index_stream;
    if !left.is_null() {
        index_cat_helper(info, left);
    }
    (*this).node.uncompressed_base = (*this)
        .node
        .uncompressed_base
        .wrapping_add((*info).uncompressed_size);
    (*this).node.compressed_base = (*this).node.compressed_base.wrapping_add((*info).file_size);
    (*this).number = (*this).number.wrapping_add((*info).stream_number_add);
    (*this).block_number_base = (*this)
        .block_number_base
        .wrapping_add((*info).block_number_add);
    index_tree_append((*info).streams, &raw mut (*this).node);
    if !right.is_null() {
        index_cat_helper(info, right);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_cat(
    dest: *mut lzma_index,
    src: *mut lzma_index,
    allocator: *const lzma_allocator,
) -> lzma_ret {
    if dest.is_null() || src.is_null() {
        return LZMA_PROG_ERROR;
    }
    let dest_file_size: lzma_vli = lzma_index_file_size(dest) as lzma_vli;
    if dest_file_size.wrapping_add(lzma_index_file_size(src)) > LZMA_VLI_MAX
        || (*dest)
            .uncompressed_size
            .wrapping_add((*src).uncompressed_size)
            > LZMA_VLI_MAX
    {
        return LZMA_DATA_ERROR;
    }
    let dest_size: lzma_vli =
        index_size_unpadded((*dest).record_count, (*dest).index_list_size) as lzma_vli;
    let src_size: lzma_vli =
        index_size_unpadded((*src).record_count, (*src).index_list_size) as lzma_vli;
    if vli_ceil4(dest_size.wrapping_add(src_size)) > LZMA_BACKWARD_SIZE_MAX as lzma_vli {
        return LZMA_DATA_ERROR;
    }
    let s: *mut index_stream = (*dest).streams.rightmost as *mut index_stream;
    let g: *mut index_group = (*s).groups.rightmost as *mut index_group;
    if !g.is_null() && (*g).last.wrapping_add(1) < (*g).allocated {
        let newg: *mut index_group = lzma_alloc(
            (core::mem::size_of::<index_group>()).wrapping_add(
                (*g).last
                    .wrapping_add(1)
                    .wrapping_mul(core::mem::size_of::<index_record>()),
            ),
            allocator,
        ) as *mut index_group;
        if newg.is_null() {
            return LZMA_MEM_ERROR;
        }
        (*newg).node = (*g).node;
        (*newg).allocated = (*g).last.wrapping_add(1);
        (*newg).last = (*g).last;
        (*newg).number_base = (*g).number_base;
        memcpy(
            &raw mut (*newg).records as *mut index_record as *mut c_void,
            &raw mut (*g).records as *mut index_record as *const c_void,
            (*newg)
                .allocated
                .wrapping_mul(core::mem::size_of::<index_record>()),
        );
        if !(*g).node.parent.is_null() {
            (*(*g).node.parent).right = &raw mut (*newg).node;
        }
        if (*s).groups.leftmost == &raw mut (*g).node {
            (*s).groups.leftmost = &raw mut (*newg).node;
            (*s).groups.root = &raw mut (*newg).node;
        }
        (*s).groups.rightmost = &raw mut (*newg).node;
        lzma_free(g as *mut c_void, allocator);
    }
    (*dest).checks = lzma_index_checks(dest);
    let info: index_cat_info = index_cat_info {
        uncompressed_size: (*dest).uncompressed_size,
        file_size: dest_file_size,
        block_number_add: (*dest).record_count,
        stream_number_add: (*dest).streams.count,
        streams: &raw mut (*dest).streams,
    };
    index_cat_helper(&raw const info, (*src).streams.root as *mut index_stream);
    (*dest).uncompressed_size = (*dest)
        .uncompressed_size
        .wrapping_add((*src).uncompressed_size);
    (*dest).total_size = (*dest).total_size.wrapping_add((*src).total_size);
    (*dest).record_count = (*dest).record_count.wrapping_add((*src).record_count);
    (*dest).index_list_size = (*dest).index_list_size.wrapping_add((*src).index_list_size);
    (*dest).checks |= (*src).checks;
    lzma_free(src as *mut c_void, allocator);
    return LZMA_OK;
}
unsafe extern "C" fn index_dup_stream(
    src: *const index_stream,
    allocator: *const lzma_allocator,
) -> *mut index_stream {
    if (*src).record_count > PREALLOC_MAX as lzma_vli {
        return core::ptr::null_mut();
    }
    let dest: *mut index_stream = index_stream_init(
        (*src).node.compressed_base,
        (*src).node.uncompressed_base,
        (*src).number,
        (*src).block_number_base,
        allocator,
    );
    if dest.is_null() {
        return core::ptr::null_mut();
    }
    (*dest).record_count = (*src).record_count;
    (*dest).index_list_size = (*src).index_list_size;
    (*dest).stream_flags = (*src).stream_flags;
    (*dest).stream_padding = (*src).stream_padding;
    if (*src).groups.leftmost.is_null() {
        return dest;
    }
    let destg: *mut index_group = lzma_alloc(
        (core::mem::size_of::<index_group>() as lzma_vli).wrapping_add(
            (*src)
                .record_count
                .wrapping_mul(core::mem::size_of::<index_record>() as lzma_vli),
        ) as size_t,
        allocator,
    ) as *mut index_group;
    if destg.is_null() {
        index_stream_end(dest as *mut c_void, allocator);
        return core::ptr::null_mut();
    }
    (*destg).node.uncompressed_base = 0;
    (*destg).node.compressed_base = 0;
    (*destg).number_base = 1;
    (*destg).allocated = (*src).record_count as size_t;
    (*destg).last = (*src).record_count.wrapping_sub(1) as size_t;
    let mut srcg: *const index_group = (*src).groups.leftmost as *const index_group;
    let mut i: size_t = 0;
    loop {
        memcpy(
            (&raw mut (*destg).records as *mut index_record).offset(i as isize) as *mut c_void,
            &raw const (*srcg).records as *const index_record as *const c_void,
            (*srcg)
                .last
                .wrapping_add(1)
                .wrapping_mul(core::mem::size_of::<index_record>()),
        );
        i = i.wrapping_add((*srcg).last.wrapping_add(1));
        srcg = index_tree_next(&raw const (*srcg).node) as *const index_group;
        if srcg.is_null() {
            break;
        }
    }
    index_tree_append(&raw mut (*dest).groups, &raw mut (*destg).node);
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_dup(
    src: *const lzma_index,
    allocator: *const lzma_allocator,
) -> *mut lzma_index {
    let dest: *mut lzma_index = index_init_plain(allocator);
    if dest.is_null() {
        return core::ptr::null_mut();
    }
    (*dest).uncompressed_size = (*src).uncompressed_size;
    (*dest).total_size = (*src).total_size;
    (*dest).record_count = (*src).record_count;
    (*dest).index_list_size = (*src).index_list_size;
    let mut srcstream: *const index_stream = (*src).streams.leftmost as *const index_stream;
    loop {
        let deststream: *mut index_stream = index_dup_stream(srcstream, allocator);
        if deststream.is_null() {
            lzma_index_end(dest, allocator);
            return core::ptr::null_mut();
        }
        index_tree_append(&raw mut (*dest).streams, &raw mut (*deststream).node);
        srcstream = index_tree_next(&raw const (*srcstream).node) as *const index_stream;
        if srcstream.is_null() {
            break;
        }
    }
    return dest;
}
unsafe extern "C" fn iter_set_info(iter: *mut lzma_index_iter) {
    let i: *const lzma_index = (*iter).internal[ITER_INDEX as usize].p as *const lzma_index;
    let stream: *const index_stream =
        (*iter).internal[ITER_STREAM as usize].p as *const index_stream;
    let group: *const index_group = (*iter).internal[ITER_GROUP as usize].p as *const index_group;
    let record: size_t = (*iter).internal[ITER_RECORD as usize].s;
    if group.is_null() {
        (*iter).internal[ITER_METHOD as usize].s = ITER_METHOD_LEFTMOST as size_t;
    } else if (*i).streams.rightmost != &raw const (*stream).node as *mut index_tree_node
        || (*stream).groups.rightmost != &raw const (*group).node as *mut index_tree_node
    {
        (*iter).internal[ITER_METHOD as usize].s = ITER_METHOD_NORMAL as size_t;
    } else if (*stream).groups.leftmost != &raw const (*group).node as *mut index_tree_node {
        (*iter).internal[ITER_METHOD as usize].s = ITER_METHOD_NEXT as size_t;
        (*iter).internal[ITER_GROUP as usize].p = (*group).node.parent as *const c_void;
    } else {
        (*iter).internal[ITER_METHOD as usize].s = ITER_METHOD_LEFTMOST as size_t;
        (*iter).internal[ITER_GROUP as usize].p = ::core::ptr::null::<c_void>();
    }
    (*iter).stream.number = (*stream).number as lzma_vli;
    (*iter).stream.block_count = (*stream).record_count;
    (*iter).stream.compressed_offset = (*stream).node.compressed_base;
    (*iter).stream.uncompressed_offset = (*stream).node.uncompressed_base;
    (*iter).stream.flags = if (*stream).stream_flags.version == UINT32_MAX {
        ::core::ptr::null::<lzma_stream_flags>()
    } else {
        &raw const (*stream).stream_flags
    };
    (*iter).stream.padding = (*stream).stream_padding;
    if (*stream).groups.rightmost.is_null() {
        (*iter).stream.compressed_size =
            index_size(0, 0).wrapping_add((2 * LZMA_STREAM_HEADER_SIZE) as lzma_vli);
        (*iter).stream.uncompressed_size = 0;
    } else {
        let g: *const index_group = (*stream).groups.rightmost as *const index_group;
        (*iter).stream.compressed_size = ((2 * LZMA_STREAM_HEADER_SIZE) as lzma_vli)
            .wrapping_add(index_size(
                (*stream).record_count,
                (*stream).index_list_size,
            ))
            .wrapping_add(vli_ceil4(
                (*(&raw const (*g).records as *const index_record).offset((*g).last as isize))
                    .unpadded_sum,
            ));
        (*iter).stream.uncompressed_size = (*(&raw const (*g).records as *const index_record)
            .offset((*g).last as isize))
        .uncompressed_sum;
    }
    if !group.is_null() {
        (*iter).block.number_in_stream = (*group).number_base.wrapping_add(record as lzma_vli);
        (*iter).block.number_in_file = (*iter)
            .block
            .number_in_stream
            .wrapping_add((*stream).block_number_base);
        (*iter).block.compressed_stream_offset = if record == 0 {
            (*group).node.compressed_base
        } else {
            vli_ceil4(
                (*(&raw const (*group).records as *const index_record)
                    .offset(record.wrapping_sub(1) as isize))
                .unpadded_sum,
            )
        };
        (*iter).block.uncompressed_stream_offset = if record == 0 {
            (*group).node.uncompressed_base
        } else {
            (*(&raw const (*group).records as *const index_record)
                .offset(record.wrapping_sub(1) as isize))
            .uncompressed_sum
        };
        (*iter).block.uncompressed_size = (*(&raw const (*group).records as *const index_record)
            .offset(record as isize))
        .uncompressed_sum
        .wrapping_sub((*iter).block.uncompressed_stream_offset);
        (*iter).block.unpadded_size = (*(&raw const (*group).records as *const index_record)
            .offset(record as isize))
        .unpadded_sum
        .wrapping_sub((*iter).block.compressed_stream_offset);
        (*iter).block.total_size = vli_ceil4((*iter).block.unpadded_size);
        (*iter).block.compressed_stream_offset = (*iter)
            .block
            .compressed_stream_offset
            .wrapping_add(LZMA_STREAM_HEADER_SIZE as lzma_vli);
        (*iter).block.compressed_file_offset = (*iter)
            .block
            .compressed_stream_offset
            .wrapping_add((*iter).stream.compressed_offset);
        (*iter).block.uncompressed_file_offset = (*iter)
            .block
            .uncompressed_stream_offset
            .wrapping_add((*iter).stream.uncompressed_offset);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_init(iter: *mut lzma_index_iter, i: *const lzma_index) {
    (*iter).internal[ITER_INDEX as usize].p = i as *const c_void;
    lzma_index_iter_rewind(iter);
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_rewind(iter: *mut lzma_index_iter) {
    (*iter).internal[ITER_STREAM as usize].p = ::core::ptr::null::<c_void>();
    (*iter).internal[ITER_GROUP as usize].p = ::core::ptr::null::<c_void>();
    (*iter).internal[ITER_RECORD as usize].s = 0;
    (*iter).internal[ITER_METHOD as usize].s = ITER_METHOD_NORMAL as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_next(
    iter: *mut lzma_index_iter,
    mode: lzma_index_iter_mode,
) -> lzma_bool {
    if mode > LZMA_INDEX_ITER_NONEMPTY_BLOCK {
        return true as lzma_bool;
    }
    let i: *const lzma_index = (*iter).internal[ITER_INDEX as usize].p as *const lzma_index;
    let mut stream: *const index_stream =
        (*iter).internal[ITER_STREAM as usize].p as *const index_stream;
    let mut group: *const index_group = ::core::ptr::null::<index_group>();
    let mut record: size_t = (*iter).internal[ITER_RECORD as usize].s;
    if mode != LZMA_INDEX_ITER_STREAM {
        match (*iter).internal[ITER_METHOD as usize].s {
            0 => {
                group = (*iter).internal[ITER_GROUP as usize].p as *const index_group;
            }
            1 => {
                group = index_tree_next(
                    (*iter).internal[ITER_GROUP as usize].p as *const index_tree_node,
                ) as *const index_group;
            }
            2 => {
                group = (*stream).groups.leftmost as *const index_group;
            }
            _ => {}
        }
    }
    loop {
        if stream.is_null() {
            stream = (*i).streams.leftmost as *const index_stream;
            if mode >= LZMA_INDEX_ITER_BLOCK {
                while (*stream).groups.leftmost.is_null() {
                    stream = index_tree_next(&raw const (*stream).node) as *const index_stream;
                    if stream.is_null() {
                        return true as lzma_bool;
                    }
                }
            }
            group = (*stream).groups.leftmost as *const index_group;
            record = 0;
        } else if !group.is_null() && record < (*group).last {
            record = record.wrapping_add(1);
        } else {
            record = 0;
            if !group.is_null() {
                group = index_tree_next(&raw const (*group).node) as *const index_group;
            }
            if group.is_null() {
                loop {
                    stream = index_tree_next(&raw const (*stream).node) as *const index_stream;
                    if stream.is_null() {
                        return true as lzma_bool;
                    }
                    if !(mode >= LZMA_INDEX_ITER_BLOCK && (*stream).groups.leftmost.is_null()) {
                        break;
                    }
                }
                group = (*stream).groups.leftmost as *const index_group;
            }
        }
        if !(mode == LZMA_INDEX_ITER_NONEMPTY_BLOCK) {
            break;
        }
        if record == 0 {
            if !((*group).node.uncompressed_base
                == (*(&raw const (*group).records as *const index_record).offset(0))
                    .uncompressed_sum)
            {
                break;
            }
        } else if !((*(&raw const (*group).records as *const index_record)
            .offset(record.wrapping_sub(1) as isize))
        .uncompressed_sum
            == (*(&raw const (*group).records as *const index_record).offset(record as isize))
                .uncompressed_sum)
        {
            break;
        }
    }
    (*iter).internal[ITER_STREAM as usize].p = stream as *const c_void;
    (*iter).internal[ITER_GROUP as usize].p = group as *const c_void;
    (*iter).internal[ITER_RECORD as usize].s = record;
    iter_set_info(iter);
    return false as lzma_bool;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_index_iter_locate(
    iter: *mut lzma_index_iter,
    mut target: lzma_vli,
) -> lzma_bool {
    let i: *const lzma_index = (*iter).internal[ITER_INDEX as usize].p as *const lzma_index;
    if (*i).uncompressed_size <= target {
        return true as lzma_bool;
    }
    let stream: *const index_stream =
        index_tree_locate(&raw const (*i).streams, target) as *const index_stream;
    target = target.wrapping_sub((*stream).node.uncompressed_base);
    let group: *const index_group =
        index_tree_locate(&raw const (*stream).groups, target) as *const index_group;
    let mut left: size_t = 0;
    let mut right: size_t = (*group).last;
    while left < right {
        let pos: size_t = left.wrapping_add(right.wrapping_sub(left).wrapping_div(2));
        if (*(&raw const (*group).records as *const index_record).offset(pos as isize))
            .uncompressed_sum
            <= target
        {
            left = pos.wrapping_add(1);
        } else {
            right = pos;
        }
    }
    (*iter).internal[ITER_STREAM as usize].p = stream as *const c_void;
    (*iter).internal[ITER_GROUP as usize].p = group as *const c_void;
    (*iter).internal[ITER_RECORD as usize].s = left;
    iter_set_info(iter);
    return false as lzma_bool;
}
