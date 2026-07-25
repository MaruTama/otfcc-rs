#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort, strcmp};
unsafe extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn bufwrite_bytes(buf: *mut caryll_Buffer, size: usize, str: *const u8);
    fn bufwrite_buf(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new(_: *const ::core::ffi::c_char) -> *mut json_value;
    fn json_string_new_length(
        length: ::core::ffi::c_uint,
        _: *const ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn base64_encode(src: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
    fn bk_newBlockFromBufferCopy(buf: *const caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}
use crate::support::binio::{read_16u, read_32u};
use crate::logger::{otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphid_t};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, sds, sdshdr16, sdshdr32, sdshdr64, sdshdr8};
use crate::vendor::json::{json_array, json_double, json_integer, json_object, json_string, json_type, json_value};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{b16, b32, bk_Block, bk_int, bk_new_Block, bk_ptr, bk_push, p32};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};
use crate::support::{__compar_fn_t};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct svg_Assignment {
    pub start: glyphid_t,
    pub end: glyphid_t,
    pub document: *mut caryll_Buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_svg_Assignment {
    pub init: Option<unsafe extern "C" fn(*mut svg_Assignment) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut svg_Assignment, *const svg_Assignment) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut svg_Assignment, *mut svg_Assignment) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut svg_Assignment) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut svg_Assignment, svg_Assignment) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut svg_Assignment, svg_Assignment) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> svg_Assignment>,
    pub dup: Option<unsafe extern "C" fn(svg_Assignment) -> svg_Assignment>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_SVG {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut svg_Assignment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_table_SVG {
    pub init: Option<unsafe extern "C" fn(*mut table_SVG) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_SVG, *const table_SVG) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_SVG, *mut table_SVG) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_SVG) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_SVG, table_SVG) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_SVG, table_SVG) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_SVG>,
    pub free: Option<unsafe extern "C" fn(*mut table_SVG) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut table_SVG, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_SVG, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut table_SVG>,
    pub fill: Option<unsafe extern "C" fn(*mut table_SVG, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_SVG) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_SVG, svg_Assignment) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_SVG) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_SVG) -> svg_Assignment>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_SVG, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut table_SVG,
            Option<unsafe extern "C" fn(*const svg_Assignment, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut table_SVG,
            Option<
                unsafe extern "C" fn(
                    *const svg_Assignment,
                    *const svg_Assignment,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
#[inline]
unsafe extern "C" fn initSVGAssigment(mut a: *mut svg_Assignment) {
    memset(
        a as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<svg_Assignment>() as usize,
    );
}
#[inline]
unsafe extern "C" fn copySVGAssigment(
    mut dst: *mut svg_Assignment,
    mut src: *const svg_Assignment,
) {
    (*dst).start = (*src).start;
    (*dst).end = (*src).end;
    (*dst).document = bufnew();
    bufwrite_buf((*dst).document, (*src).document);
}
#[inline]
unsafe extern "C" fn disposeSVGAssignment(mut a: *mut svg_Assignment) {
    buffree((*a).document);
}
#[inline]
unsafe extern "C" fn svg_Assignment_move(
    mut dst: *mut svg_Assignment,
    mut src: *mut svg_Assignment,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<svg_Assignment>() as usize,
    );
    svg_Assignment_init(src);
}
#[inline]
unsafe extern "C" fn svg_Assignment_replace(mut dst: *mut svg_Assignment, src: svg_Assignment) {
    svg_Assignment_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<svg_Assignment>() as usize,
    );
}
#[unsafe(no_mangle)]
pub static svg_iAssignment: __caryll_elementinterface_svg_Assignment = {
    __caryll_elementinterface_svg_Assignment {
        init: Some(svg_Assignment_init as unsafe extern "C" fn(*mut svg_Assignment) -> ()),
        copy: Some(
            svg_Assignment_copy
                as unsafe extern "C" fn(*mut svg_Assignment, *const svg_Assignment) -> (),
        ),
        move_0: Some(
            svg_Assignment_move
                as unsafe extern "C" fn(*mut svg_Assignment, *mut svg_Assignment) -> (),
        ),
        dispose: Some(svg_Assignment_dispose as unsafe extern "C" fn(*mut svg_Assignment) -> ()),
        replace: Some(
            svg_Assignment_replace
                as unsafe extern "C" fn(*mut svg_Assignment, svg_Assignment) -> (),
        ),
        copyReplace: Some(
            svg_Assignment_copyReplace
                as unsafe extern "C" fn(*mut svg_Assignment, svg_Assignment) -> (),
        ),
        empty: Some(svg_Assignment_empty),
        dup: Some(svg_Assignment_dup as unsafe extern "C" fn(svg_Assignment) -> svg_Assignment),
    }
};
#[inline]
unsafe extern "C" fn svg_Assignment_empty() -> svg_Assignment {
    let mut x: svg_Assignment = svg_Assignment {
        start: 0,
        end: 0,
        document: ::core::ptr::null_mut::<caryll_Buffer>(),
    };
    svg_Assignment_init(&raw mut x);
    return x;
}
#[inline]
unsafe extern "C" fn svg_Assignment_init(mut x: *mut svg_Assignment) {
    initSVGAssigment(x);
}
#[inline]
unsafe extern "C" fn svg_Assignment_dup(src: svg_Assignment) -> svg_Assignment {
    let mut dst: svg_Assignment = svg_Assignment {
        start: 0,
        end: 0,
        document: ::core::ptr::null_mut::<caryll_Buffer>(),
    };
    svg_Assignment_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
unsafe extern "C" fn svg_Assignment_copyReplace(mut dst: *mut svg_Assignment, src: svg_Assignment) {
    svg_Assignment_dispose(dst);
    svg_Assignment_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn svg_Assignment_copy(
    mut dst: *mut svg_Assignment,
    mut src: *const svg_Assignment,
) {
    copySVGAssigment(dst, src);
}
#[inline]
unsafe extern "C" fn svg_Assignment_dispose(mut x: *mut svg_Assignment) {
    disposeSVGAssignment(x);
}
#[inline]
unsafe extern "C" fn table_SVG_createN(mut n: usize) -> *mut table_SVG {
    let mut t: *mut table_SVG =
        malloc(::core::mem::size_of::<table_SVG>() as usize) as *mut table_SVG;
    table_SVG_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_SVG_move(dst: *mut table_SVG, src: *mut table_SVG) {
    cvec_move(table_SVG_as_cvec(dst), table_SVG_as_cvec(src));
}
#[inline]
unsafe fn table_SVG_as_cvec(arr: *mut table_SVG) -> *mut CVecRaw<svg_Assignment> {
    arr as *mut CVecRaw<svg_Assignment>
}
#[inline]
unsafe extern "C" fn table_SVG_init(arr: *mut table_SVG) {
    cvec_init(table_SVG_as_cvec(arr));
}
#[unsafe(no_mangle)]
pub static table_iSVG: __caryll_vectorinterface_table_SVG = {
    __caryll_vectorinterface_table_SVG {
        init: Some(table_SVG_init as unsafe extern "C" fn(*mut table_SVG) -> ()),
        copy: Some(table_SVG_copy as unsafe extern "C" fn(*mut table_SVG, *const table_SVG) -> ()),
        move_0: Some(table_SVG_move as unsafe extern "C" fn(*mut table_SVG, *mut table_SVG) -> ()),
        dispose: Some(table_SVG_dispose as unsafe extern "C" fn(*mut table_SVG) -> ()),
        replace: Some(table_SVG_replace as unsafe extern "C" fn(*mut table_SVG, table_SVG) -> ()),
        copyReplace: Some(
            table_SVG_copyReplace as unsafe extern "C" fn(*mut table_SVG, table_SVG) -> (),
        ),
        create: Some(table_SVG_create),
        free: Some(table_SVG_free as unsafe extern "C" fn(*mut table_SVG) -> ()),
        initN: Some(table_SVG_initN as unsafe extern "C" fn(*mut table_SVG, usize) -> ()),
        initCapN: Some(table_SVG_initCapN as unsafe extern "C" fn(*mut table_SVG, usize) -> ()),
        createN: Some(table_SVG_createN as unsafe extern "C" fn(usize) -> *mut table_SVG),
        fill: Some(table_SVG_fill as unsafe extern "C" fn(*mut table_SVG, usize) -> ()),
        clear: Some(table_SVG_dispose as unsafe extern "C" fn(*mut table_SVG) -> ()),
        push: Some(table_SVG_push as unsafe extern "C" fn(*mut table_SVG, svg_Assignment) -> ()),
        shrinkToFit: Some(table_SVG_shrinkToFit as unsafe extern "C" fn(*mut table_SVG) -> ()),
        pop: Some(table_SVG_pop as unsafe extern "C" fn(*mut table_SVG) -> svg_Assignment),
        disposeItem: Some(
            table_SVG_disposeItem as unsafe extern "C" fn(*mut table_SVG, usize) -> (),
        ),
        filterEnv: Some(
            table_SVG_filterEnv
                as unsafe extern "C" fn(
                    *mut table_SVG,
                    Option<
                        unsafe extern "C" fn(
                            *const svg_Assignment,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_SVG_sort
                as unsafe extern "C" fn(
                    *mut table_SVG,
                    Option<
                        unsafe extern "C" fn(
                            *const svg_Assignment,
                            *const svg_Assignment,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn table_SVG_filterEnv(
    mut arr: *mut table_SVG,
    mut fn_0: Option<unsafe extern "C" fn(*const svg_Assignment, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut svg_Assignment,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if svg_iAssignment.dispose.is_some() {
                svg_iAssignment.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut svg_Assignment,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn table_SVG_disposeItem(mut arr: *mut table_SVG, mut n: usize) {
    if svg_iAssignment.dispose.is_some() {
        svg_iAssignment.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut svg_Assignment,
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_SVG_sort(
    mut arr: *mut table_SVG,
    mut fn_0: Option<
        unsafe extern "C" fn(*const svg_Assignment, *const svg_Assignment) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<svg_Assignment>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const svg_Assignment,
                    *const svg_Assignment,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn table_SVG_fill(mut arr: *mut table_SVG, mut n: usize) {
    while (*arr).length < n {
        let mut x: svg_Assignment = svg_Assignment {
            start: 0,
            end: 0,
            document: ::core::ptr::null_mut::<caryll_Buffer>(),
        };
        if svg_iAssignment.init.is_some() {
            svg_iAssignment.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<svg_Assignment>() as usize,
            );
        }
        table_SVG_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_SVG_push(arr: *mut table_SVG, elem: svg_Assignment) {
    cvec_push(table_SVG_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_SVG_grow(arr: *mut table_SVG) {
    cvec_grow(table_SVG_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_SVG_growTo(arr: *mut table_SVG, target: usize) {
    cvec_grow_to(table_SVG_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_SVG_pop(arr: *mut table_SVG) -> svg_Assignment {
    cvec_pop(table_SVG_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_SVG_copyReplace(mut dst: *mut table_SVG, src: table_SVG) {
    table_SVG_dispose(dst);
    table_SVG_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_SVG_copy(mut dst: *mut table_SVG, mut src: *const table_SVG) {
    table_SVG_init(dst);
    table_SVG_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if svg_iAssignment.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            svg_iAssignment.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut svg_Assignment,
                (*src).items.offset(j as isize) as *mut svg_Assignment as *const svg_Assignment,
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
unsafe extern "C" fn table_SVG_dispose(mut arr: *mut table_SVG) {
    if arr.is_null() {
        return;
    }
    if svg_iAssignment.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            svg_iAssignment.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut svg_Assignment,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<svg_Assignment>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn table_SVG_replace(mut dst: *mut table_SVG, src: table_SVG) {
    table_SVG_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_SVG>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_SVG_initCapN(mut arr: *mut table_SVG, mut n: usize) {
    table_SVG_init(arr);
    table_SVG_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn table_SVG_growToN(arr: *mut table_SVG, target: usize) {
    cvec_grow_to_n(table_SVG_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_SVG_initN(mut arr: *mut table_SVG, mut n: usize) {
    table_SVG_init(arr);
    table_SVG_growToN(arr, n);
    table_SVG_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_SVG_free(mut x: *mut table_SVG) {
    if x.is_null() {
        return;
    }
    table_SVG_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_SVG_shrinkToFit(mut arr: *mut table_SVG) {
    table_SVG_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_SVG_create() -> *mut table_SVG {
    let mut x: *mut table_SVG =
        malloc(::core::mem::size_of::<table_SVG>() as usize) as *mut table_SVG;
    table_SVG_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_SVG_resizeTo(arr: *mut table_SVG, target: usize) {
    cvec_resize_to(table_SVG_as_cvec(arr), target);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_readSVG(
    packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
) -> *mut table_SVG {
    let mut offsetToSVGDocIndex: u32 = 0;
    let mut numEntries: u16 = 0;
    let mut svg: *mut table_SVG = ::core::ptr::null_mut::<table_SVG>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1398163232i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    if !(table.length < 10 as u32) {
                        offsetToSVGDocIndex =
                            read_32u(table.data.offset(2 as ::core::ffi::c_int as isize));
                        if !(table.length < offsetToSVGDocIndex.wrapping_add(2 as u32)) {
                            numEntries = read_16u(table.data.offset(offsetToSVGDocIndex as isize));
                            if !(table.length
                                < offsetToSVGDocIndex
                                    .wrapping_add(2 as u32)
                                    .wrapping_add(
                                        (12 as ::core::ffi::c_int
                                            * numEntries as ::core::ffi::c_int)
                                            as u32,
                                    ))
                            {
                                svg = (
                                    table_iSVG.create.expect("non-null function pointer"))();
                                let mut j: glyphid_t = 0 as glyphid_t;
                                while (j as ::core::ffi::c_int) < numEntries as ::core::ffi::c_int {
                                    let mut record: font_file_pointer = table
                                        .data
                                        .offset(offsetToSVGDocIndex as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (12 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                as isize,
                                        );
                                    let mut asg: svg_Assignment =
                                        (
                                            svg_iAssignment
                                                .empty
                                                .expect("non-null function pointer"))();
                                    asg.start = read_16u(record as *const u8) as glyphid_t;
                                    asg.end =
                                        read_16u(record.offset(2 as ::core::ffi::c_int as isize)
                                            as *const u8)
                                            as glyphid_t;
                                    let mut docstart: u32 =
                                        read_32u(record.offset(4 as ::core::ffi::c_int as isize)
                                            as *const u8);
                                    let mut doclen: u32 =
                                        read_32u(record.offset(8 as ::core::ffi::c_int as isize)
                                            as *const u8);
                                    if offsetToSVGDocIndex
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
                                                .offset(offsetToSVGDocIndex as isize)
                                                .offset(docstart as isize),
                                        );
                                    } else {
                                        asg.document = bufnew();
                                    }
                                    table_iSVG.push.expect("non-null function pointer")(svg, asg);
                                    j = j.wrapping_add(1);
                                }
                                return svg;
                            }
                        }
                    }
                    table_iSVG.dispose.expect("non-null function pointer")(svg);
                    svg = ::core::ptr::null_mut::<table_SVG>();
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
unsafe extern "C" fn canUsePlainFormat(mut buf: *const caryll_Buffer) -> bool {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_dumpSVG(
    mut svg: *const table_SVG,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if svg.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"SVG "),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _svg: *mut json_value = json_array_new((*svg).length);
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < (*svg).length {
            let mut a: *mut svg_Assignment = (*svg).items.offset(__caryll_index as isize);
            while keep != 0 {
                let mut _a: *mut json_value = json_object_new(4 as usize);
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
                if canUsePlainFormat((*a).document) {
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
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_parseSVG(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_SVG {
    let mut _svg: *mut json_value = ::core::ptr::null_mut::<json_value>();
    _svg = json_obj_get_type(
        root,
        b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    if _svg.is_null() {
        return ::core::ptr::null_mut::<table_SVG>();
    }
    let mut svg: *mut table_SVG = (
        table_iSVG.create.expect("non-null function pointer"))();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"SVG "),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as ::core::ffi::c_uint) < (*_svg).u.array.length {
            let mut _a: *mut json_value =
                *(*_svg).u.array.values.offset(j as isize) as *mut json_value;
            if !(_a.is_null()
                || (*_a).type_0 != json_object)
            {
                let mut format: *const ::core::ffi::c_char = json_obj_getstr_share(
                    _a,
                    b"format\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let mut doc: sds =
                    json_obj_getsds(_a, b"document\0" as *const u8 as *const ::core::ffi::c_char);
                if !(format.is_null() || doc.is_null()) {
                    let mut asg: svg_Assignment = (
                        svg_iAssignment.empty.expect("non-null function pointer"))();
                    asg.start =
                        json_obj_getint(_a, b"start\0" as *const u8 as *const ::core::ffi::c_char)
                            as glyphid_t;
                    asg.end =
                        json_obj_getint(_a, b"end\0" as *const u8 as *const ::core::ffi::c_char)
                            as glyphid_t;
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
                    table_iSVG.push.expect("non-null function pointer")(svg, asg);
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    return svg;
}
unsafe extern "C" fn byStartGID(
    mut a: *const svg_Assignment,
    mut b: *const svg_Assignment,
) -> ::core::ffi::c_int {
    return (*a).start as ::core::ffi::c_int - (*b).start as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_buildSVG(
    mut _svg: *const table_SVG,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if _svg.is_null() || (*_svg).length == 0 {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut svg: table_SVG = table_SVG {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<svg_Assignment>(),
    };
    table_iSVG.copy.expect("non-null function pointer")(&raw mut svg, _svg);
    table_iSVG.sort.expect("non-null function pointer")(
        &raw mut svg,
        Some(
            byStartGID
                as unsafe extern "C" fn(
                    *const svg_Assignment,
                    *const svg_Assignment,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut major: *mut bk_Block = bk_new_Block(&[bk_int(b16, (svg.length) as u32)]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < svg.length {
        let mut a: *mut svg_Assignment = svg.items.offset(__caryll_index as isize);
        while keep != 0 {
            bk_push(major, &[bk_int(b16, ((*a).start as ::core::ffi::c_int) as u32), bk_int(b16, ((*a).end as ::core::ffi::c_int) as u32), bk_ptr(p32, bk_newBlockFromBufferCopy((*a).document)), bk_int(b32, ((*(*a).document).size) as u32)]);
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut root: *mut bk_Block = bk_new_Block(&[bk_int(b16, 0 as u32), bk_ptr(p32, major), bk_int(b32, 0 as u32)]);
    table_iSVG.dispose.expect("non-null function pointer")(&raw mut svg);
    return bk_build_Block(root);
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 != json_object
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_getsds(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> sds {
    let mut v: *mut json_value = json_obj_get_type(obj, key, json_string);
    if v.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        return sdsnewlen(
            (*v).u.string.ptr as *const ::core::ffi::c_void,
            (*v).u.string.length as usize,
        );
    };
}
#[inline]
unsafe extern "C" fn json_obj_getstr_share(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut v: *mut json_value = json_obj_get_type(obj, key, json_string);
    if v.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        return (*v).u.string.ptr;
    };
}
#[inline]
unsafe extern "C" fn json_obj_getint(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> i32 {
    if obj.is_null()
        || (*obj).type_0 != json_object
    {
        return 0 as i32;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 == json_integer
            {
                return (*cv).u.integer as i32;
            }
            if !cv.is_null()
                && (*cv).type_0 == json_double
            {
                return (*cv).u.dbl as i32;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0 as i32;
}
