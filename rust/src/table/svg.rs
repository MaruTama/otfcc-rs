#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort, strcmp};
use crate::support::json_funcs::{json_obj_get_type, json_obj_getint, json_obj_getsds, json_obj_getstr_share};
use crate::support::binio::{read_16u, read_32u};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::{ComparFn};
use crate::bk::bkblock::{bk_new_block_from_buffer_copy};
use crate::bk::bkgraph::{bk_build_block};
use crate::support::base64::{base64_encode};
use crate::support::buffer::{buffree, bufnew, bufwrite_buf, bufwrite_bytes};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_string_new, json_string_new_length};
use crate::vendor::sds::{sdsempty, sdsfree};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SvgAssignment {
    pub start: GlyphId,
    pub end: GlyphId,
    pub document: *mut Buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SvgAssignmentElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut SvgAssignment) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut SvgAssignment, *const SvgAssignment) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut SvgAssignment, *mut SvgAssignment) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut SvgAssignment) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut SvgAssignment, SvgAssignment) -> ()>,
    pub copy_replace: Option<unsafe extern "C" fn(*mut SvgAssignment, SvgAssignment) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> SvgAssignment>,
    pub dup: Option<unsafe extern "C" fn(SvgAssignment) -> SvgAssignment>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SvgTable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut SvgAssignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SvgTableVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut SvgTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut SvgTable, *const SvgTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut SvgTable, *mut SvgTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut SvgTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut SvgTable, SvgTable) -> ()>,
    pub copy_replace: Option<unsafe extern "C" fn(*mut SvgTable, SvgTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut SvgTable>,
    pub free: Option<unsafe extern "C" fn(*mut SvgTable) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut SvgTable, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut SvgTable, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut SvgTable>,
    pub fill: Option<unsafe extern "C" fn(*mut SvgTable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut SvgTable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut SvgTable, SvgAssignment) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut SvgTable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut SvgTable) -> SvgAssignment>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut SvgTable, usize) -> ()>,
    pub filter_env: Option<
        unsafe extern "C" fn(
            *mut SvgTable,
            Option<unsafe extern "C" fn(*const SvgAssignment, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut SvgTable,
            Option<
                unsafe extern "C" fn(
                    *const SvgAssignment,
                    *const SvgAssignment,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
#[inline]
unsafe extern "C" fn init_svg_assigment(mut a: *mut SvgAssignment) {
    memset(
        a as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<SvgAssignment>() as usize,
    );
}
#[inline]
unsafe extern "C" fn copy_svg_assigment(
    mut dst: *mut SvgAssignment,
    mut src: *const SvgAssignment,
) {
    (*dst).start = (*src).start;
    (*dst).end = (*src).end;
    (*dst).document = bufnew();
    bufwrite_buf((*dst).document, (*src).document);
}
#[inline]
unsafe extern "C" fn dispose_svg_assignment(mut a: *mut SvgAssignment) {
    buffree((*a).document);
}
#[inline]
unsafe extern "C" fn svg_assignment_move(
    mut dst: *mut SvgAssignment,
    mut src: *mut SvgAssignment,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<SvgAssignment>() as usize,
    );
    svg_assignment_init(src);
}
#[inline]
unsafe extern "C" fn svg_assignment_replace(mut dst: *mut SvgAssignment, src: SvgAssignment) {
    svg_assignment_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<SvgAssignment>() as usize,
    );
}
pub static SVG_I_ASSIGNMENT: SvgAssignmentElementInterface = {
    SvgAssignmentElementInterface {
        init: Some(svg_assignment_init as unsafe extern "C" fn(*mut SvgAssignment) -> ()),
        copy: Some(
            svg_assignment_copy
                as unsafe extern "C" fn(*mut SvgAssignment, *const SvgAssignment) -> (),
        ),
        move_0: Some(
            svg_assignment_move
                as unsafe extern "C" fn(*mut SvgAssignment, *mut SvgAssignment) -> (),
        ),
        dispose: Some(svg_assignment_dispose as unsafe extern "C" fn(*mut SvgAssignment) -> ()),
        replace: Some(
            svg_assignment_replace
                as unsafe extern "C" fn(*mut SvgAssignment, SvgAssignment) -> (),
        ),
        copy_replace: Some(
            svg_assignment_copy_replace
                as unsafe extern "C" fn(*mut SvgAssignment, SvgAssignment) -> (),
        ),
        empty: Some(svg_assignment_empty),
        dup: Some(svg_assignment_dup as unsafe extern "C" fn(SvgAssignment) -> SvgAssignment),
    }
};
#[inline]
unsafe extern "C" fn svg_assignment_empty() -> SvgAssignment {
    let mut x: SvgAssignment = SvgAssignment {
        start: 0,
        end: 0,
        document: ::core::ptr::null_mut::<Buffer>(),
    };
    svg_assignment_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn svg_assignment_init(mut x: *mut SvgAssignment) {
    init_svg_assigment(x);
}
#[inline]
unsafe extern "C" fn svg_assignment_dup(src: SvgAssignment) -> SvgAssignment {
    let mut dst: SvgAssignment = SvgAssignment {
        start: 0,
        end: 0,
        document: ::core::ptr::null_mut::<Buffer>(),
    };
    svg_assignment_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn svg_assignment_copy_replace(mut dst: *mut SvgAssignment, src: SvgAssignment) {
    svg_assignment_dispose(dst);
    svg_assignment_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn svg_assignment_copy(
    mut dst: *mut SvgAssignment,
    mut src: *const SvgAssignment,
) {
    copy_svg_assigment(dst, src);
}
#[inline]
unsafe extern "C" fn svg_assignment_dispose(mut x: *mut SvgAssignment) {
    dispose_svg_assignment(x);
}
#[inline]
unsafe extern "C" fn table_svg_create_n(mut n: usize) -> *mut SvgTable {
    let mut t: *mut SvgTable =
        malloc(::core::mem::size_of::<SvgTable>() as usize) as *mut SvgTable;
    table_svg_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_svg_move(dst: *mut SvgTable, src: *mut SvgTable) {
    cvec_move(table_svg_as_cvec(dst), table_svg_as_cvec(src));
}
#[inline]
unsafe fn table_svg_as_cvec(arr: *mut SvgTable) -> *mut CVecRaw<SvgAssignment> {
    arr as *mut CVecRaw<SvgAssignment>
}
#[inline]
unsafe extern "C" fn table_svg_init(arr: *mut SvgTable) {
    cvec_init(table_svg_as_cvec(arr));
}
pub static TABLE_I_SVG: SvgTableVectorInterface = {
    SvgTableVectorInterface {
        init: Some(table_svg_init as unsafe extern "C" fn(*mut SvgTable) -> ()),
        copy: Some(table_svg_copy as unsafe extern "C" fn(*mut SvgTable, *const SvgTable) -> ()),
        move_0: Some(table_svg_move as unsafe extern "C" fn(*mut SvgTable, *mut SvgTable) -> ()),
        dispose: Some(table_svg_dispose as unsafe extern "C" fn(*mut SvgTable) -> ()),
        replace: Some(table_svg_replace as unsafe extern "C" fn(*mut SvgTable, SvgTable) -> ()),
        copy_replace: Some(
            table_svg_copy_replace as unsafe extern "C" fn(*mut SvgTable, SvgTable) -> (),
        ),
        create: Some(table_svg_create),
        free: Some(table_svg_free as unsafe extern "C" fn(*mut SvgTable) -> ()),
        init_n: Some(table_svg_init_n as unsafe extern "C" fn(*mut SvgTable, usize) -> ()),
        init_cap_n: Some(table_svg_init_cap_n as unsafe extern "C" fn(*mut SvgTable, usize) -> ()),
        create_n: Some(table_svg_create_n as unsafe extern "C" fn(usize) -> *mut SvgTable),
        fill: Some(table_svg_fill as unsafe extern "C" fn(*mut SvgTable, usize) -> ()),
        clear: Some(table_svg_dispose as unsafe extern "C" fn(*mut SvgTable) -> ()),
        push: Some(table_svg_push as unsafe extern "C" fn(*mut SvgTable, SvgAssignment) -> ()),
        shrink_to_fit: Some(table_svg_shrink_to_fit as unsafe extern "C" fn(*mut SvgTable) -> ()),
        pop: Some(table_svg_pop as unsafe extern "C" fn(*mut SvgTable) -> SvgAssignment),
        dispose_item: Some(
            table_svg_dispose_item as unsafe extern "C" fn(*mut SvgTable, usize) -> (),
        ),
        filter_env: Some(
            table_svg_filter_env
                as unsafe extern "C" fn(
                    *mut SvgTable,
                    Option<
                        unsafe extern "C" fn(
                            *const SvgAssignment,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_svg_sort
                as unsafe extern "C" fn(
                    *mut SvgTable,
                    Option<
                        unsafe extern "C" fn(
                            *const SvgAssignment,
                            *const SvgAssignment,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn table_svg_filter_env(
    mut arr: *mut SvgTable,
    mut fn_0: Option<unsafe extern "C" fn(*const SvgAssignment, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut SvgAssignment,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if SVG_I_ASSIGNMENT.dispose.is_some() {
                SVG_I_ASSIGNMENT.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut SvgAssignment,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn table_svg_dispose_item(mut arr: *mut SvgTable, mut n: usize) {
    if SVG_I_ASSIGNMENT.dispose.is_some() {
        SVG_I_ASSIGNMENT.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut SvgAssignment,
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_svg_sort(
    mut arr: *mut SvgTable,
    mut fn_0: Option<
        unsafe extern "C" fn(*const SvgAssignment, *const SvgAssignment) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<SvgAssignment>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const SvgAssignment,
                    *const SvgAssignment,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn table_svg_fill(mut arr: *mut SvgTable, mut n: usize) {
    while (*arr).length < n {
        let mut x: SvgAssignment = SvgAssignment {
            start: 0,
            end: 0,
            document: ::core::ptr::null_mut::<Buffer>(),
        };
        if SVG_I_ASSIGNMENT.init.is_some() {
            SVG_I_ASSIGNMENT.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<SvgAssignment>() as usize,
            );
        }
        table_svg_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_svg_push(arr: *mut SvgTable, elem: SvgAssignment) {
    cvec_push(table_svg_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_svg_grow(arr: *mut SvgTable) {
    cvec_grow(table_svg_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_svg_grow_to(arr: *mut SvgTable, target: usize) {
    cvec_grow_to(table_svg_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_svg_pop(arr: *mut SvgTable) -> SvgAssignment {
    cvec_pop(table_svg_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_svg_copy_replace(mut dst: *mut SvgTable, src: SvgTable) {
    table_svg_dispose(dst);
    table_svg_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_svg_copy(mut dst: *mut SvgTable, mut src: *const SvgTable) {
    table_svg_init(dst);
    table_svg_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if SVG_I_ASSIGNMENT.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            SVG_I_ASSIGNMENT.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut SvgAssignment,
                (*src).items.offset(j as isize) as *mut SvgAssignment as *const SvgAssignment,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn table_svg_dispose(mut arr: *mut SvgTable) {
    if arr.is_null() {
        return;
    }
    if SVG_I_ASSIGNMENT.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            SVG_I_ASSIGNMENT.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut SvgAssignment,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<SvgAssignment>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn table_svg_replace(mut dst: *mut SvgTable, src: SvgTable) {
    table_svg_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<SvgTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_svg_init_cap_n(mut arr: *mut SvgTable, mut n: usize) {
    table_svg_init(arr);
    table_svg_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn table_svg_grow_to_n(arr: *mut SvgTable, target: usize) {
    cvec_grow_to_n(table_svg_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_svg_init_n(mut arr: *mut SvgTable, mut n: usize) {
    table_svg_init(arr);
    table_svg_grow_to_n(arr, n);
    table_svg_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_svg_free(mut x: *mut SvgTable) {
    if x.is_null() {
        return;
    }
    table_svg_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_svg_shrink_to_fit(mut arr: *mut SvgTable) {
    table_svg_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_svg_create() -> *mut SvgTable {
    let mut x: *mut SvgTable =
        malloc(::core::mem::size_of::<SvgTable>() as usize) as *mut SvgTable;
    table_svg_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_svg_resize_to(arr: *mut SvgTable, target: usize) {
    cvec_resize_to(table_svg_as_cvec(arr), target);
}
pub unsafe extern "C" fn otfcc_read_svg(
    packet: Packet,
    mut _options: *const Options,
) -> *mut SvgTable {
    let mut offset_to_svg_doc_index: u32 = 0;
    let mut num_entries: u16 = 0;
    let mut svg: *mut SvgTable = ::core::ptr::null_mut::<SvgTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1398163232i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    if !(table.length < 10 as u32) {
                        offset_to_svg_doc_index =
                            read_32u(table.data.offset(2 as ::core::ffi::c_int as isize));
                        if !(table.length < offset_to_svg_doc_index.wrapping_add(2 as u32)) {
                            num_entries = read_16u(table.data.offset(offset_to_svg_doc_index as isize));
                            if !(table.length
                                < offset_to_svg_doc_index
                                    .wrapping_add(2 as u32)
                                    .wrapping_add(
                                        (12 as ::core::ffi::c_int
                                            * num_entries as ::core::ffi::c_int)
                                            as u32,
                                    ))
                            {
                                svg = (
                                    TABLE_I_SVG.create.expect("non-null function pointer"))();
                                let mut j: GlyphId = 0 as GlyphId;
                                while (j as ::core::ffi::c_int) < num_entries as ::core::ffi::c_int {
                                    let mut record: FontFilePointer = table
                                        .data
                                        .offset(offset_to_svg_doc_index as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (12 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                as isize,
                                        );
                                    let mut asg: SvgAssignment =
                                        (
                                            SVG_I_ASSIGNMENT
                                                .empty
                                                .expect("non-null function pointer"))();
                                    asg.start = read_16u(record as *const u8) as GlyphId;
                                    asg.end =
                                        read_16u(record.offset(2 as ::core::ffi::c_int as isize)
                                            as *const u8)
                                            as GlyphId;
                                    let mut docstart: u32 =
                                        read_32u(record.offset(4 as ::core::ffi::c_int as isize)
                                            as *const u8);
                                    let mut doclen: u32 =
                                        read_32u(record.offset(8 as ::core::ffi::c_int as isize)
                                            as *const u8);
                                    if offset_to_svg_doc_index
                                        .wrapping_add(docstart)
                                        .wrapping_add(doclen)
                                        <= table.length
                                    {
                                        asg.document = bufnew();
                                        bufwrite_bytes(
                                            asg.document,
                                            doclen as usize,
                                            table
                                                .data
                                                .offset(offset_to_svg_doc_index as isize)
                                                .offset(docstart as isize),
                                        );
                                    } else {
                                        asg.document = bufnew();
                                    }
                                    TABLE_I_SVG.push.expect("non-null function pointer")(svg, asg);
                                    j = j.wrapping_add(1);
                                }
                                return svg;
                            }
                        }
                    }
                    TABLE_I_SVG.dispose.expect("non-null function pointer")(svg);
                    svg = ::core::ptr::null_mut::<SvgTable>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return svg;
}
unsafe extern "C" fn can_use_plain_format(mut buf: *const Buffer) -> bool {
    return (*buf).size > 4 as usize
        && *(*buf).data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '<' as i32
        && *(*buf).data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 's' as i32
        && *(*buf).data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'v' as i32
        && *(*buf).data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'g' as i32
        || (*buf).size > 5 as usize
            && *(*buf).data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '<' as i32
            && *(*buf).data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '?' as i32
            && *(*buf).data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'x' as i32
            && *(*buf).data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'm' as i32
            && *(*buf).data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'l' as i32;
}
pub unsafe extern "C" fn otfcc_dump_svg(
    mut svg: *const SvgTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if svg.is_null() {
        return;
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"SVG "),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _svg: *mut JsonValue = json_array_new((*svg).length);
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < (*svg).length {
            let mut a: *mut SvgAssignment = (*svg).items.offset(__caryll_index as isize);
            while keep != 0 {
                let mut _a: *mut JsonValue = json_object_new(4 as usize);
                json_object_push(
                    _a,
                    b"start\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*a).start as i64),
                );
                json_object_push(
                    _a,
                    b"end\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*a).end as i64),
                );
                if can_use_plain_format((*a).document) {
                    json_object_push(
                        _a,
                        b"format\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new(b"plain\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                    json_object_push(
                        _a,
                        b"document\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            (*(*a).document).size as ::core::ffi::c_uint,
                            (*(*a).document).data as *mut ::core::ffi::c_char,
                        ),
                    );
                } else {
                    let mut len: usize = 0 as usize;
                    let mut buf: *mut u8 =
                        base64_encode((*(*a).document).data, (*(*a).document).size, &raw mut len);
                    json_object_push(
                        _a,
                        b"format\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new(b"base64\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                    json_object_push(
                        _a,
                        b"document\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            len as ::core::ffi::c_uint,
                            buf as *mut ::core::ffi::c_char,
                        ),
                    );
                    free(buf as *mut ::core::ffi::c_void);
                    buf = ::core::ptr::null_mut::<u8>();
                }
                json_array_push(_svg, _a);
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        json_object_push(
            root,
            b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
            _svg,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_svg(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut SvgTable {
    let mut _svg: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    _svg = json_obj_get_type(
        root,
        b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _svg.is_null() {
        return ::core::ptr::null_mut::<SvgTable>();
    }
    let mut svg: *mut SvgTable = (
        TABLE_I_SVG.create.expect("non-null function pointer"))();
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"SVG "),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: GlyphId = 0 as GlyphId;
        while (j as ::core::ffi::c_uint) < (*_svg).u.array.length {
            let mut _a: *mut JsonValue =
                *(*_svg).u.array.values.offset(j as isize) as *mut JsonValue;
            if !(_a.is_null()
                || (*_a).type_0 != JsonType::Object)
            {
                let mut format: *const ::core::ffi::c_char = json_obj_getstr_share(
                    _a,
                    b"format\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let mut doc: SdsRaw =
                    json_obj_getsds(_a, b"document\0" as *const u8 as *const ::core::ffi::c_char);
                if !(format.is_null() || doc.is_null()) {
                    let mut asg: SvgAssignment = (
                        SVG_I_ASSIGNMENT.empty.expect("non-null function pointer"))();
                    asg.start =
                        json_obj_getint(_a, b"start\0" as *const u8 as *const ::core::ffi::c_char)
                            as GlyphId;
                    asg.end =
                        json_obj_getint(_a, b"end\0" as *const u8 as *const ::core::ffi::c_char)
                            as GlyphId;
                    if strcmp(
                        format,
                        b"plain\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        asg.document = bufnew();
                        bufwrite_bytes(asg.document, sdslen(doc), doc as *mut u8);
                        sdsfree(doc);
                    } else {
                        asg.document = bufnew();
                        let mut len: usize = 0 as usize;
                        let mut buf: *mut u8 =
                            base64_encode(doc as *mut u8, sdslen(doc), &raw mut len);
                        bufwrite_bytes(asg.document, len, buf);
                        free(buf as *mut ::core::ffi::c_void);
                        buf = ::core::ptr::null_mut::<u8>();
                        sdsfree(doc);
                    }
                    TABLE_I_SVG.push.expect("non-null function pointer")(svg, asg);
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return svg;
}
unsafe extern "C" fn by_start_gid(
    mut a: *const SvgAssignment,
    mut b: *const SvgAssignment,
) -> ::core::ffi::c_int {
    return (*a).start as ::core::ffi::c_int - (*b).start as ::core::ffi::c_int;
}
pub unsafe extern "C" fn otfcc_build_svg(
    mut _svg: *const SvgTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if _svg.is_null() || (*_svg).length == 0 {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut svg: SvgTable = SvgTable {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<SvgAssignment>(),
    };
    TABLE_I_SVG.copy.expect("non-null function pointer")(&raw mut svg, _svg);
    TABLE_I_SVG.sort.expect("non-null function pointer")(
        &raw mut svg,
        Some(
            by_start_gid
                as unsafe extern "C" fn(
                    *const SvgAssignment,
                    *const SvgAssignment,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut major: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (svg.length) as u32)]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < svg.length {
        let mut a: *mut SvgAssignment = svg.items.offset(__caryll_index as isize);
        while keep != 0 {
            bk_push(major, &[bk_int(BkCellType::B16, ((*a).start as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*a).end as ::core::ffi::c_int) as u32), bk_ptr(BkCellType::P32, bk_new_block_from_buffer_copy((*a).document)), bk_int(BkCellType::B32, ((*(*a).document).size) as u32)]);
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 0 as u32), bk_ptr(BkCellType::P32, major), bk_int(BkCellType::B32, 0 as u32)]);
    TABLE_I_SVG.dispose.expect("non-null function pointer")(&raw mut svg);
    return bk_build_block(root);
}
