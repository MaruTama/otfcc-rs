#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};


use crate::support::json_funcs::{json_obj_get_type, json_obj_getint, json_obj_getint_fallback, preserialize};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_8u, read_16u, read_32u};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{ColorId, FontFilePointer, TableId};
use crate::vendor::json::{json_array, json_object, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{b16, b32, b8, BkBlock, bk_int, bk_new_Block, bk_ptr, bk_push, p32};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::support::{ComparFn};
use crate::bk::bkgraph::{bk_build_Block};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsempty};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpalColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    pub label: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpalColorElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CpalColor) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CpalColor, *const CpalColor) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut CpalColor, *mut CpalColor) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CpalColor) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut CpalColor, CpalColor) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut CpalColor, CpalColor) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpalColorSet {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut CpalColor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpalColorSetVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut CpalColorSet) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CpalColorSet, *const CpalColorSet) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut CpalColorSet, *mut CpalColorSet) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CpalColorSet) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut CpalColorSet, CpalColorSet) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut CpalColorSet, CpalColorSet) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut CpalColorSet>,
    pub free: Option<unsafe extern "C" fn(*mut CpalColorSet) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut CpalColorSet, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut CpalColorSet, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut CpalColorSet>,
    pub fill: Option<unsafe extern "C" fn(*mut CpalColorSet, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut CpalColorSet) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut CpalColorSet, CpalColor) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut CpalColorSet) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut CpalColorSet) -> CpalColor>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut CpalColorSet, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut CpalColorSet,
            Option<unsafe extern "C" fn(*const CpalColor, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut CpalColorSet,
            Option<unsafe extern "C" fn(*const CpalColor, *const CpalColor) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpalPalette {
    pub colorset: CpalColorSet,
    pub type_0: u32,
    pub label: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpalPaletteElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CpalPalette) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CpalPalette, *const CpalPalette) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut CpalPalette, *mut CpalPalette) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CpalPalette) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut CpalPalette, CpalPalette) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut CpalPalette, CpalPalette) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpalPaletteSet {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut CpalPalette,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpalPaletteSetVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut CpalPaletteSet) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CpalPaletteSet, *const CpalPaletteSet) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut CpalPaletteSet, *mut CpalPaletteSet) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CpalPaletteSet) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut CpalPaletteSet, CpalPaletteSet) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut CpalPaletteSet, CpalPaletteSet) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut CpalPaletteSet>,
    pub free: Option<unsafe extern "C" fn(*mut CpalPaletteSet) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut CpalPaletteSet, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut CpalPaletteSet, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut CpalPaletteSet>,
    pub fill: Option<unsafe extern "C" fn(*mut CpalPaletteSet, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut CpalPaletteSet) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut CpalPaletteSet, CpalPalette) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut CpalPaletteSet) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut CpalPaletteSet) -> CpalPalette>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut CpalPaletteSet, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut CpalPaletteSet,
            Option<unsafe extern "C" fn(*const CpalPalette, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut CpalPaletteSet,
            Option<
                unsafe extern "C" fn(
                    *const CpalPalette,
                    *const CpalPalette,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpalTable {
    pub version: u16,
    pub palettes: CpalPaletteSet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpalTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CpalTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CpalTable, *const CpalTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut CpalTable, *mut CpalTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CpalTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut CpalTable, CpalTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut CpalTable, CpalTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut CpalTable>,
    pub free: Option<unsafe extern "C" fn(*mut CpalTable) -> ()>,
}
#[inline]
unsafe extern "C" fn cpal_Color_move(mut dst: *mut CpalColor, mut src: *mut CpalColor) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CpalColor>() as usize,
    );
    cpal_Color_init(src);
}
#[inline]
unsafe extern "C" fn cpal_Color_replace(mut dst: *mut CpalColor, src: CpalColor) {
    cpal_Color_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CpalColor>() as usize,
    );
}
pub static cpal_iColor: CpalColorElementInterface = {
    CpalColorElementInterface {
        init: Some(cpal_Color_init as unsafe extern "C" fn(*mut CpalColor) -> ()),
        copy: Some(
            cpal_Color_copy as unsafe extern "C" fn(*mut CpalColor, *const CpalColor) -> (),
        ),
        move_0: Some(
            cpal_Color_move as unsafe extern "C" fn(*mut CpalColor, *mut CpalColor) -> (),
        ),
        dispose: Some(cpal_Color_dispose as unsafe extern "C" fn(*mut CpalColor) -> ()),
        replace: Some(
            cpal_Color_replace as unsafe extern "C" fn(*mut CpalColor, CpalColor) -> (),
        ),
        copyReplace: Some(
            cpal_Color_copyReplace as unsafe extern "C" fn(*mut CpalColor, CpalColor) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn cpal_Color_init(mut x: *mut CpalColor) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<CpalColor>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cpal_Color_copyReplace(mut dst: *mut CpalColor, src: CpalColor) {
    cpal_Color_dispose(dst);
    cpal_Color_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cpal_Color_copy(mut dst: *mut CpalColor, mut src: *const CpalColor) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CpalColor>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cpal_Color_dispose(mut _x: *mut CpalColor) {}
#[inline]
unsafe extern "C" fn cpal_ColorSet_disposeItem(mut arr: *mut CpalColorSet, mut n: usize) {
    if cpal_iColor.dispose.is_some() {
        cpal_iColor.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut CpalColor
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_sort(
    mut arr: *mut CpalColorSet,
    mut fn_0: Option<
        unsafe extern "C" fn(*const CpalColor, *const CpalColor) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<CpalColor>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const CpalColor, *const CpalColor) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_copy(
    mut dst: *mut CpalColorSet,
    mut src: *const CpalColorSet,
) {
    cpal_ColorSet_init(dst);
    cpal_ColorSet_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if cpal_iColor.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            cpal_iColor.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut CpalColor,
                (*src).items.offset(j as isize) as *mut CpalColor as *const CpalColor,
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
unsafe extern "C" fn cpal_ColorSet_dispose(mut arr: *mut CpalColorSet) {
    if arr.is_null() {
        return;
    }
    if cpal_iColor.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            cpal_iColor.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut CpalColor,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<CpalColor>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_replace(mut dst: *mut CpalColorSet, src: CpalColorSet) {
    cpal_ColorSet_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CpalColorSet>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_initCapN(mut arr: *mut CpalColorSet, mut n: usize) {
    cpal_ColorSet_init(arr);
    cpal_ColorSet_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_growToN(arr: *mut CpalColorSet, target: usize) {
    cvec_grow_to_n(cpal_ColorSet_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_initN(mut arr: *mut CpalColorSet, mut n: usize) {
    cpal_ColorSet_init(arr);
    cpal_ColorSet_growToN(arr, n);
    cpal_ColorSet_fill(arr, n);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_free(mut x: *mut CpalColorSet) {
    if x.is_null() {
        return;
    }
    cpal_ColorSet_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_createN(mut n: usize) -> *mut CpalColorSet {
    let mut t: *mut CpalColorSet =
        malloc(::core::mem::size_of::<CpalColorSet>() as usize) as *mut CpalColorSet;
    cpal_ColorSet_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_create() -> *mut CpalColorSet {
    let mut x: *mut CpalColorSet =
        malloc(::core::mem::size_of::<CpalColorSet>() as usize) as *mut CpalColorSet;
    cpal_ColorSet_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_fill(mut arr: *mut CpalColorSet, mut n: usize) {
    while (*arr).length < n {
        let mut x: CpalColor = CpalColor {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
            label: 0,
        };
        if cpal_iColor.init.is_some() {
            cpal_iColor.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<CpalColor>() as usize,
            );
        }
        cpal_ColorSet_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_push(arr: *mut CpalColorSet, elem: CpalColor) {
    cvec_push(cpal_ColorSet_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_grow(arr: *mut CpalColorSet) {
    cvec_grow(cpal_ColorSet_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_move(dst: *mut CpalColorSet, src: *mut CpalColorSet) {
    cvec_move(cpal_ColorSet_as_cvec(dst), cpal_ColorSet_as_cvec(src));
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_shrinkToFit(mut arr: *mut CpalColorSet) {
    cpal_ColorSet_resizeTo(arr, (*arr).length);
}
pub static cpal_iColorSet: CpalColorSetVectorInterface = {
    CpalColorSetVectorInterface {
        init: Some(cpal_ColorSet_init as unsafe extern "C" fn(*mut CpalColorSet) -> ()),
        copy: Some(
            cpal_ColorSet_copy
                as unsafe extern "C" fn(*mut CpalColorSet, *const CpalColorSet) -> (),
        ),
        move_0: Some(
            cpal_ColorSet_move
                as unsafe extern "C" fn(*mut CpalColorSet, *mut CpalColorSet) -> (),
        ),
        dispose: Some(cpal_ColorSet_dispose as unsafe extern "C" fn(*mut CpalColorSet) -> ()),
        replace: Some(
            cpal_ColorSet_replace as unsafe extern "C" fn(*mut CpalColorSet, CpalColorSet) -> (),
        ),
        copyReplace: Some(
            cpal_ColorSet_copyReplace
                as unsafe extern "C" fn(*mut CpalColorSet, CpalColorSet) -> (),
        ),
        create: Some(cpal_ColorSet_create),
        free: Some(cpal_ColorSet_free as unsafe extern "C" fn(*mut CpalColorSet) -> ()),
        initN: Some(cpal_ColorSet_initN as unsafe extern "C" fn(*mut CpalColorSet, usize) -> ()),
        initCapN: Some(
            cpal_ColorSet_initCapN as unsafe extern "C" fn(*mut CpalColorSet, usize) -> (),
        ),
        createN: Some(cpal_ColorSet_createN as unsafe extern "C" fn(usize) -> *mut CpalColorSet),
        fill: Some(cpal_ColorSet_fill as unsafe extern "C" fn(*mut CpalColorSet, usize) -> ()),
        clear: Some(cpal_ColorSet_dispose as unsafe extern "C" fn(*mut CpalColorSet) -> ()),
        push: Some(
            cpal_ColorSet_push as unsafe extern "C" fn(*mut CpalColorSet, CpalColor) -> (),
        ),
        shrinkToFit: Some(
            cpal_ColorSet_shrinkToFit as unsafe extern "C" fn(*mut CpalColorSet) -> (),
        ),
        pop: Some(cpal_ColorSet_pop as unsafe extern "C" fn(*mut CpalColorSet) -> CpalColor),
        disposeItem: Some(
            cpal_ColorSet_disposeItem as unsafe extern "C" fn(*mut CpalColorSet, usize) -> (),
        ),
        filterEnv: Some(
            cpal_ColorSet_filterEnv
                as unsafe extern "C" fn(
                    *mut CpalColorSet,
                    Option<
                        unsafe extern "C" fn(*const CpalColor, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            cpal_ColorSet_sort
                as unsafe extern "C" fn(
                    *mut CpalColorSet,
                    Option<
                        unsafe extern "C" fn(
                            *const CpalColor,
                            *const CpalColor,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn cpal_ColorSet_growTo(arr: *mut CpalColorSet, target: usize) {
    cvec_grow_to(cpal_ColorSet_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_pop(arr: *mut CpalColorSet) -> CpalColor {
    cvec_pop(cpal_ColorSet_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_resizeTo(arr: *mut CpalColorSet, target: usize) {
    cvec_resize_to(cpal_ColorSet_as_cvec(arr), target);
}
#[inline]
unsafe fn cpal_ColorSet_as_cvec(arr: *mut CpalColorSet) -> *mut CVecRaw<CpalColor> {
    arr as *mut CVecRaw<CpalColor>
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_init(arr: *mut CpalColorSet) {
    cvec_init(cpal_ColorSet_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_copyReplace(mut dst: *mut CpalColorSet, src: CpalColorSet) {
    cpal_ColorSet_dispose(dst);
    cpal_ColorSet_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_filterEnv(
    mut arr: *mut CpalColorSet,
    mut fn_0: Option<unsafe extern "C" fn(*const CpalColor, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut CpalColor,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if cpal_iColor.dispose.is_some() {
                cpal_iColor.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut CpalColor,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn initPalette(mut p: *mut CpalPalette) {
    cpal_iColorSet.init.expect("non-null function pointer")(&raw mut (*p).colorset);
    (*p).type_0 = 0 as u32;
    (*p).label = 0xffff as u32;
}
#[inline]
unsafe extern "C" fn disposePalette(mut p: *mut CpalPalette) {
    cpal_iColorSet.dispose.expect("non-null function pointer")(&raw mut (*p).colorset);
}
pub static cpal_iPalette: CpalPaletteElementInterface = {
    CpalPaletteElementInterface {
        init: Some(cpal_Palette_init as unsafe extern "C" fn(*mut CpalPalette) -> ()),
        copy: Some(
            cpal_Palette_copy as unsafe extern "C" fn(*mut CpalPalette, *const CpalPalette) -> (),
        ),
        move_0: Some(
            cpal_Palette_move as unsafe extern "C" fn(*mut CpalPalette, *mut CpalPalette) -> (),
        ),
        dispose: Some(cpal_Palette_dispose as unsafe extern "C" fn(*mut CpalPalette) -> ()),
        replace: Some(
            cpal_Palette_replace as unsafe extern "C" fn(*mut CpalPalette, CpalPalette) -> (),
        ),
        copyReplace: Some(
            cpal_Palette_copyReplace as unsafe extern "C" fn(*mut CpalPalette, CpalPalette) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn cpal_Palette_copyReplace(mut dst: *mut CpalPalette, src: CpalPalette) {
    cpal_Palette_dispose(dst);
    cpal_Palette_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cpal_Palette_move(mut dst: *mut CpalPalette, mut src: *mut CpalPalette) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CpalPalette>() as usize,
    );
    cpal_Palette_init(src);
}
#[inline]
unsafe extern "C" fn cpal_Palette_replace(mut dst: *mut CpalPalette, src: CpalPalette) {
    cpal_Palette_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CpalPalette>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cpal_Palette_dispose(mut x: *mut CpalPalette) {
    disposePalette(x);
}
#[inline]
unsafe extern "C" fn cpal_Palette_init(mut x: *mut CpalPalette) {
    initPalette(x);
}
#[inline]
unsafe extern "C" fn cpal_Palette_copy(mut dst: *mut CpalPalette, mut src: *const CpalPalette) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CpalPalette>() as usize,
    );
}
#[inline]
unsafe fn cpal_PaletteSet_as_cvec(arr: *mut CpalPaletteSet) -> *mut CVecRaw<CpalPalette> {
    arr as *mut CVecRaw<CpalPalette>
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_init(arr: *mut CpalPaletteSet) {
    cvec_init(cpal_PaletteSet_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_move(dst: *mut CpalPaletteSet, src: *mut CpalPaletteSet) {
    cvec_move(cpal_PaletteSet_as_cvec(dst), cpal_PaletteSet_as_cvec(src));
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_filterEnv(
    mut arr: *mut CpalPaletteSet,
    mut fn_0: Option<unsafe extern "C" fn(*const CpalPalette, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut CpalPalette,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if cpal_iPalette.dispose.is_some() {
                cpal_iPalette.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut CpalPalette,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_disposeItem(mut arr: *mut CpalPaletteSet, mut n: usize) {
    if cpal_iPalette.dispose.is_some() {
        cpal_iPalette.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut CpalPalette
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_sort(
    mut arr: *mut CpalPaletteSet,
    mut fn_0: Option<
        unsafe extern "C" fn(*const CpalPalette, *const CpalPalette) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<CpalPalette>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const CpalPalette,
                    *const CpalPalette,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_fill(mut arr: *mut CpalPaletteSet, mut n: usize) {
    while (*arr).length < n {
        let mut x: CpalPalette = CpalPalette {
            colorset: CpalColorSet {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<CpalColor>(),
            },
            type_0: 0,
            label: 0,
        };
        if cpal_iPalette.init.is_some() {
            cpal_iPalette.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<CpalPalette>() as usize,
            );
        }
        cpal_PaletteSet_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_push(arr: *mut CpalPaletteSet, elem: CpalPalette) {
    cvec_push(cpal_PaletteSet_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_grow(arr: *mut CpalPaletteSet) {
    cvec_grow(cpal_PaletteSet_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_growTo(arr: *mut CpalPaletteSet, target: usize) {
    cvec_grow_to(cpal_PaletteSet_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_pop(arr: *mut CpalPaletteSet) -> CpalPalette {
    cvec_pop(cpal_PaletteSet_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_copyReplace(
    mut dst: *mut CpalPaletteSet,
    src: CpalPaletteSet,
) {
    cpal_PaletteSet_dispose(dst);
    cpal_PaletteSet_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_copy(
    mut dst: *mut CpalPaletteSet,
    mut src: *const CpalPaletteSet,
) {
    cpal_PaletteSet_init(dst);
    cpal_PaletteSet_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if cpal_iPalette.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            cpal_iPalette.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut CpalPalette,
                (*src).items.offset(j as isize) as *mut CpalPalette as *const CpalPalette,
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
unsafe extern "C" fn cpal_PaletteSet_dispose(mut arr: *mut CpalPaletteSet) {
    if arr.is_null() {
        return;
    }
    if cpal_iPalette.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh3 = j;
            j = j.wrapping_sub(1);
            if !(fresh3 != 0) {
                break;
            }
            cpal_iPalette.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut CpalPalette,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<CpalPalette>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_replace(mut dst: *mut CpalPaletteSet, src: CpalPaletteSet) {
    cpal_PaletteSet_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CpalPaletteSet>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_initCapN(mut arr: *mut CpalPaletteSet, mut n: usize) {
    cpal_PaletteSet_init(arr);
    cpal_PaletteSet_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_growToN(arr: *mut CpalPaletteSet, target: usize) {
    cvec_grow_to_n(cpal_PaletteSet_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_initN(mut arr: *mut CpalPaletteSet, mut n: usize) {
    cpal_PaletteSet_init(arr);
    cpal_PaletteSet_growToN(arr, n);
    cpal_PaletteSet_fill(arr, n);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_free(mut x: *mut CpalPaletteSet) {
    if x.is_null() {
        return;
    }
    cpal_PaletteSet_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_createN(mut n: usize) -> *mut CpalPaletteSet {
    let mut t: *mut CpalPaletteSet =
        malloc(::core::mem::size_of::<CpalPaletteSet>() as usize) as *mut CpalPaletteSet;
    cpal_PaletteSet_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_create() -> *mut CpalPaletteSet {
    let mut x: *mut CpalPaletteSet =
        malloc(::core::mem::size_of::<CpalPaletteSet>() as usize) as *mut CpalPaletteSet;
    cpal_PaletteSet_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_shrinkToFit(mut arr: *mut CpalPaletteSet) {
    cpal_PaletteSet_resizeTo(arr, (*arr).length);
}
pub static cpal_iPaletteSet: CpalPaletteSetVectorInterface = {
    CpalPaletteSetVectorInterface {
        init: Some(cpal_PaletteSet_init as unsafe extern "C" fn(*mut CpalPaletteSet) -> ()),
        copy: Some(
            cpal_PaletteSet_copy
                as unsafe extern "C" fn(*mut CpalPaletteSet, *const CpalPaletteSet) -> (),
        ),
        move_0: Some(
            cpal_PaletteSet_move
                as unsafe extern "C" fn(*mut CpalPaletteSet, *mut CpalPaletteSet) -> (),
        ),
        dispose: Some(cpal_PaletteSet_dispose as unsafe extern "C" fn(*mut CpalPaletteSet) -> ()),
        replace: Some(
            cpal_PaletteSet_replace
                as unsafe extern "C" fn(*mut CpalPaletteSet, CpalPaletteSet) -> (),
        ),
        copyReplace: Some(
            cpal_PaletteSet_copyReplace
                as unsafe extern "C" fn(*mut CpalPaletteSet, CpalPaletteSet) -> (),
        ),
        create: Some(cpal_PaletteSet_create),
        free: Some(cpal_PaletteSet_free as unsafe extern "C" fn(*mut CpalPaletteSet) -> ()),
        initN: Some(
            cpal_PaletteSet_initN as unsafe extern "C" fn(*mut CpalPaletteSet, usize) -> (),
        ),
        initCapN: Some(
            cpal_PaletteSet_initCapN as unsafe extern "C" fn(*mut CpalPaletteSet, usize) -> (),
        ),
        createN: Some(
            cpal_PaletteSet_createN as unsafe extern "C" fn(usize) -> *mut CpalPaletteSet,
        ),
        fill: Some(
            cpal_PaletteSet_fill as unsafe extern "C" fn(*mut CpalPaletteSet, usize) -> (),
        ),
        clear: Some(cpal_PaletteSet_dispose as unsafe extern "C" fn(*mut CpalPaletteSet) -> ()),
        push: Some(
            cpal_PaletteSet_push as unsafe extern "C" fn(*mut CpalPaletteSet, CpalPalette) -> (),
        ),
        shrinkToFit: Some(
            cpal_PaletteSet_shrinkToFit as unsafe extern "C" fn(*mut CpalPaletteSet) -> (),
        ),
        pop: Some(
            cpal_PaletteSet_pop as unsafe extern "C" fn(*mut CpalPaletteSet) -> CpalPalette,
        ),
        disposeItem: Some(
            cpal_PaletteSet_disposeItem as unsafe extern "C" fn(*mut CpalPaletteSet, usize) -> (),
        ),
        filterEnv: Some(
            cpal_PaletteSet_filterEnv
                as unsafe extern "C" fn(
                    *mut CpalPaletteSet,
                    Option<
                        unsafe extern "C" fn(*const CpalPalette, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            cpal_PaletteSet_sort
                as unsafe extern "C" fn(
                    *mut CpalPaletteSet,
                    Option<
                        unsafe extern "C" fn(
                            *const CpalPalette,
                            *const CpalPalette,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn cpal_PaletteSet_resizeTo(arr: *mut CpalPaletteSet, target: usize) {
    cvec_resize_to(cpal_PaletteSet_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn initCPAL(mut cpal: *mut CpalTable) {
    (*cpal).version = 1 as u16;
    cpal_iPaletteSet.init.expect("non-null function pointer")(&raw mut (*cpal).palettes);
}
#[inline]
unsafe extern "C" fn disposeCPAL(mut cpal: *mut CpalTable) {
    cpal_iPaletteSet.dispose.expect("non-null function pointer")(&raw mut (*cpal).palettes);
}
#[inline]
unsafe extern "C" fn table_CPAL_dispose(mut x: *mut CpalTable) {
    disposeCPAL(x);
}
#[inline]
unsafe extern "C" fn table_CPAL_init(mut x: *mut CpalTable) {
    initCPAL(x);
}
#[inline]
unsafe extern "C" fn table_CPAL_create() -> *mut CpalTable {
    let mut x: *mut CpalTable =
        malloc(::core::mem::size_of::<CpalTable>() as usize) as *mut CpalTable;
    table_CPAL_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_CPAL_copyReplace(mut dst: *mut CpalTable, src: CpalTable) {
    table_CPAL_dispose(dst);
    table_CPAL_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_CPAL_copy(mut dst: *mut CpalTable, mut src: *const CpalTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CpalTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_CPAL_replace(mut dst: *mut CpalTable, src: CpalTable) {
    table_CPAL_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CpalTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_CPAL_move(mut dst: *mut CpalTable, mut src: *mut CpalTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CpalTable>() as usize,
    );
    table_CPAL_init(src);
}
pub static table_iCPAL: CpalTableElementInterface = {
    CpalTableElementInterface {
        init: Some(table_CPAL_init as unsafe extern "C" fn(*mut CpalTable) -> ()),
        copy: Some(
            table_CPAL_copy as unsafe extern "C" fn(*mut CpalTable, *const CpalTable) -> (),
        ),
        move_0: Some(
            table_CPAL_move as unsafe extern "C" fn(*mut CpalTable, *mut CpalTable) -> (),
        ),
        dispose: Some(table_CPAL_dispose as unsafe extern "C" fn(*mut CpalTable) -> ()),
        replace: Some(
            table_CPAL_replace as unsafe extern "C" fn(*mut CpalTable, CpalTable) -> (),
        ),
        copyReplace: Some(
            table_CPAL_copyReplace as unsafe extern "C" fn(*mut CpalTable, CpalTable) -> (),
        ),
        create: Some(table_CPAL_create),
        free: Some(table_CPAL_free as unsafe extern "C" fn(*mut CpalTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_CPAL_free(mut x: *mut CpalTable) {
    if x.is_null() {
        return;
    }
    table_CPAL_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub static white: CpalColor = CpalColor {
    red: 0xff as u8,
    green: 0xff as u8,
    blue: 0xff as u8,
    alpha: 0xff as u8,
    label: 0xffff as u16,
};
pub unsafe extern "C" fn otfcc_readCPAL(
    packet: Packet,
    mut _options: *const Options,
) -> *mut CpalTable {
    let mut version: u16 = 0;
    let mut tableHeaderLength: u32 = 0;
    let mut numPalettesEntries: u16 = 0;
    let mut numPalettes: u16 = 0;
    let mut numColorRecords: u16 = 0;
    let mut offsetFirstColorRecord: u32 = 0;
    let mut colorList: *mut CpalColor = ::core::ptr::null_mut::<CpalColor>();
    let mut t: *mut CpalTable = ::core::ptr::null_mut::<CpalTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1129333068i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if !(length < 2 as u32) {
                        t = (
                            table_iCPAL.create.expect("non-null function pointer"))();
                        version = read_16u(data as *const u8);
                        tableHeaderLength =
                            (if version as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                14 as ::core::ffi::c_int
                            } else {
                                26 as ::core::ffi::c_int
                            }) as u32;
                        if !(length < tableHeaderLength) {
                            (*t).version = version;
                            numPalettesEntries = read_16u(
                                data.offset(2 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            numPalettes = read_16u(
                                data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                            );
                            numColorRecords = read_16u(
                                data.offset(6 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            offsetFirstColorRecord = read_32u(
                                data.offset(8 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            if !(length
                                < offsetFirstColorRecord.wrapping_add(
                                    (numColorRecords as ::core::ffi::c_int
                                        * 4 as ::core::ffi::c_int)
                                        as u32,
                                ))
                            {
                                if !(length
                                    < tableHeaderLength.wrapping_add(
                                        (2 as ::core::ffi::c_int
                                            * numPalettes as ::core::ffi::c_int)
                                            as u32,
                                    ))
                                {
                                    colorList = ::core::ptr::null_mut::<CpalColor>();
                                    colorList = __caryll_allocate_clean(
                                        (::core::mem::size_of::<CpalColor>() as usize)
                                            .wrapping_mul(numColorRecords as usize),
                                        55 as ::core::ffi::c_ulong,
                                    )
                                        as *mut CpalColor;
                                    let mut j: u16 = 0 as u16;
                                    while (j as ::core::ffi::c_int)
                                        < numColorRecords as ::core::ffi::c_int
                                    {
                                        *colorList.offset(j as isize) = CpalColor {
                                            red: read_8u(
                                                data.offset(offsetFirstColorRecord as isize)
                                                    .offset(
                                                        (j as ::core::ffi::c_int
                                                            * 4 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    .offset(2 as ::core::ffi::c_int as isize)
                                                    as *const u8,
                                            ),
                                            green: read_8u(
                                                data.offset(offsetFirstColorRecord as isize)
                                                    .offset(
                                                        (j as ::core::ffi::c_int
                                                            * 4 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    .offset(1 as ::core::ffi::c_int as isize)
                                                    as *const u8,
                                            ),
                                            blue: read_8u(
                                                data.offset(offsetFirstColorRecord as isize).offset(
                                                    (j as ::core::ffi::c_int
                                                        * 4 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as *const u8,
                                            ),
                                            alpha: read_8u(
                                                data.offset(offsetFirstColorRecord as isize)
                                                    .offset(
                                                        (j as ::core::ffi::c_int
                                                            * 4 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    .offset(3 as ::core::ffi::c_int as isize)
                                                    as *const u8,
                                            ),
                                            label: 0xffff as u16,
                                        };
                                        j = j.wrapping_add(1);
                                    }
                                    let mut j_0: TableId = 0 as TableId;
                                    while (j_0 as ::core::ffi::c_int)
                                        < numPalettes as ::core::ffi::c_int
                                    {
                                        let mut palette: CpalPalette = CpalPalette {
                                            colorset: CpalColorSet {
                                                length: 0,
                                                capacity: 0,
                                                items: ::core::ptr::null_mut::<CpalColor>(),
                                            },
                                            type_0: 0,
                                            label: 0,
                                        };
                                        cpal_iPalette.init.expect("non-null function pointer")(
                                            &raw mut palette,
                                        );
                                        let mut paletteStartIndex: TableId = read_16u(
                                            data.offset(12 as ::core::ffi::c_int as isize).offset(
                                                (j_0 as ::core::ffi::c_int
                                                    * 2 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as *const u8,
                                        )
                                            as TableId;
                                        let mut j_1: ColorId = 0 as ColorId;
                                        while (j_1 as ::core::ffi::c_int)
                                            < numPalettesEntries as ::core::ffi::c_int
                                        {
                                            if (paletteStartIndex as ::core::ffi::c_int
                                                + j_1 as ::core::ffi::c_int)
                                                < numColorRecords as ::core::ffi::c_int
                                            {
                                                cpal_iColorSet
                                                    .push
                                                    .expect("non-null function pointer")(
                                                    &raw mut palette.colorset,
                                                    *colorList.offset(
                                                        (j_1 as ::core::ffi::c_int
                                                            + paletteStartIndex
                                                                as ::core::ffi::c_int)
                                                            as isize,
                                                    ),
                                                );
                                            } else {
                                                cpal_iColorSet
                                                    .push
                                                    .expect("non-null function pointer")(
                                                    &raw mut palette.colorset,
                                                    white,
                                                );
                                            }
                                            j_1 = j_1.wrapping_add(1);
                                        }
                                        cpal_iPaletteSet.push.expect("non-null function pointer")(
                                            &raw mut (*t).palettes,
                                            palette,
                                        );
                                        j_0 = j_0.wrapping_add(1);
                                    }
                                    if version as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                        let mut offsetPaletteTypeArray: u32 = read_32u(
                                            data.offset(16 as ::core::ffi::c_int as isize).offset(
                                                (2 as ::core::ffi::c_int
                                                    * numPalettes as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as *const u8,
                                        );
                                        if offsetPaletteTypeArray != 0
                                            && length
                                                >= offsetPaletteTypeArray.wrapping_add(
                                                    (4 as ::core::ffi::c_int
                                                        * numPalettes as ::core::ffi::c_int)
                                                        as u32,
                                                )
                                        {
                                            let mut j_2: TableId = 0 as TableId;
                                            while (j_2 as ::core::ffi::c_int)
                                                < numPalettes as ::core::ffi::c_int
                                            {
                                                let mut type_0: u32 = read_32u(
                                                    data.offset(
                                                        (j_2 as ::core::ffi::c_int
                                                            * 4 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    .offset(offsetPaletteTypeArray as isize)
                                                        as *const u8,
                                                );
                                                (*(*t).palettes.items.offset(j_2 as isize))
                                                    .type_0 = type_0;
                                                j_2 = j_2.wrapping_add(1);
                                            }
                                        }
                                        let mut offsetPaletteLabelArray: u32 = read_32u(
                                            data.offset(20 as ::core::ffi::c_int as isize).offset(
                                                (2 as ::core::ffi::c_int
                                                    * numPalettes as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as *const u8,
                                        );
                                        if offsetPaletteLabelArray != 0
                                            && length
                                                >= offsetPaletteLabelArray.wrapping_add(
                                                    (2 as ::core::ffi::c_int
                                                        * numPalettes as ::core::ffi::c_int)
                                                        as u32,
                                                )
                                        {
                                            let mut j_3: TableId = 0 as TableId;
                                            while (j_3 as ::core::ffi::c_int)
                                                < numPalettes as ::core::ffi::c_int
                                            {
                                                let mut label: u16 = read_16u(
                                                    data.offset(
                                                        (j_3 as ::core::ffi::c_int
                                                            * 2 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    .offset(offsetPaletteLabelArray as isize)
                                                        as *const u8,
                                                );
                                                (*(*t).palettes.items.offset(j_3 as isize)).label =
                                                    label as u32;
                                                j_3 = j_3.wrapping_add(1);
                                            }
                                        }
                                        if version as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                            let mut offsetPaletteEntryLabelArray: u32 =
                                                read_32u(
                                                    data.offset(24 as ::core::ffi::c_int as isize)
                                                        .offset(
                                                            (2 as ::core::ffi::c_int
                                                                * numPalettes as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        as *const u8,
                                                );
                                            if offsetPaletteEntryLabelArray != 0
                                                && length
                                                    >= offsetPaletteEntryLabelArray.wrapping_add(
                                                        (4 as ::core::ffi::c_int
                                                            * numPalettesEntries
                                                                as ::core::ffi::c_int)
                                                            as u32,
                                                    )
                                            {
                                                let mut j_4: ColorId = 0 as ColorId;
                                                while (j_4 as ::core::ffi::c_int)
                                                    < numPalettesEntries as ::core::ffi::c_int
                                                {
                                                    let mut label_0: u16 = read_16u(
                                                        data.offset(
                                                            (j_4 as ::core::ffi::c_int
                                                                * 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        .offset(
                                                            offsetPaletteEntryLabelArray as isize,
                                                        )
                                                            as *const u8,
                                                    );
                                                    let mut k: TableId = 0 as TableId;
                                                    while (k as ::core::ffi::c_int)
                                                        < numPalettes as ::core::ffi::c_int
                                                    {
                                                        (*(*(*t)
                                                            .palettes
                                                            .items
                                                            .offset(k as isize))
                                                        .colorset
                                                        .items
                                                        .offset(j_4 as isize))
                                                        .label = label_0;
                                                        k = k.wrapping_add(1);
                                                    }
                                                    j_4 = j_4.wrapping_add(1);
                                                }
                                            }
                                        }
                                    }
                                    free(colorList as *mut ::core::ffi::c_void);
                                    colorList = ::core::ptr::null_mut::<CpalColor>();
                                    return t;
                                }
                            }
                        }
                    }
                    table_iCPAL.free.expect("non-null function pointer")(t);
                    t = ::core::ptr::null_mut::<CpalTable>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<CpalTable>();
}
#[inline]
unsafe extern "C" fn dumpColor(mut color: *mut CpalColor) -> *mut JsonValue {
    let mut _color: *mut JsonValue = json_object_new(5 as usize);
    json_object_push(
        _color,
        b"red\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*color).red as i64),
    );
    json_object_push(
        _color,
        b"green\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*color).green as i64),
    );
    json_object_push(
        _color,
        b"blue\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*color).blue as i64),
    );
    if (*color).alpha as ::core::ffi::c_int != 0xff as ::core::ffi::c_int {
        json_object_push(
            _color,
            b"alpha\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*color).alpha as i64),
        );
    }
    if (*color).label as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int {
        json_object_push(
            _color,
            b"label\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*color).label as i64),
        );
    }
    return preserialize(_color);
}
#[inline]
unsafe extern "C" fn dumpPalette(mut palette: *mut CpalPalette) -> *mut JsonValue {
    let mut _palette: *mut JsonValue = json_object_new(3 as usize);
    if (*palette).type_0 != 0 {
        json_object_push(
            _palette,
            b"type\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*palette).type_0 as i64),
        );
    }
    if (*palette).label != 0xffff as u32 {
        json_object_push(
            _palette,
            b"label\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*palette).label as i64),
        );
    }
    let mut a: *mut JsonValue = json_array_new((*palette).colorset.length);
    let mut j: ColorId = 0 as ColorId;
    while (j as usize) < (*palette).colorset.length {
        json_array_push(
            a,
            dumpColor((*palette).colorset.items.offset(j as isize) as *mut CpalColor),
        );
        j = j.wrapping_add(1);
    }
    json_object_push(
        _palette,
        b"colors\0" as *const u8 as *const ::core::ffi::c_char,
        a,
    );
    return _palette;
}
pub unsafe extern "C" fn otfcc_dumpCPAL(
    mut table: *const CpalTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"CPAL"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _t: *mut JsonValue = json_object_new(2 as usize);
        json_object_push(
            _t,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).version as i64),
        );
        let mut _a: *mut JsonValue = json_array_new((*table).palettes.length);
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*table).palettes.length {
            json_array_push(
                _a,
                dumpPalette((*table).palettes.items.offset(j as isize) as *mut CpalPalette),
            );
            j = j.wrapping_add(1);
        }
        json_object_push(
            _t,
            b"palettes\0" as *const u8 as *const ::core::ffi::c_char,
            _a,
        );
        json_object_push(
            root,
            b"CPAL\0" as *const u8 as *const ::core::ffi::c_char,
            _t,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
#[inline]
unsafe extern "C" fn parseColor(mut _color: *const JsonValue) -> CpalColor {
    let mut color: CpalColor = white;
    if _color.is_null()
        || (*_color).type_0 != json_object
    {
        return color;
    }
    color.red = json_obj_getint_fallback(
        _color,
        b"red\0" as *const u8 as *const ::core::ffi::c_char,
        0 as i32,
    ) as u8;
    color.green = json_obj_getint_fallback(
        _color,
        b"green\0" as *const u8 as *const ::core::ffi::c_char,
        0 as i32,
    ) as u8;
    color.blue = json_obj_getint_fallback(
        _color,
        b"blue\0" as *const u8 as *const ::core::ffi::c_char,
        0 as i32,
    ) as u8;
    color.alpha = json_obj_getint_fallback(
        _color,
        b"alpha\0" as *const u8 as *const ::core::ffi::c_char,
        0xff as i32,
    ) as u8;
    color.label = json_obj_getint_fallback(
        _color,
        b"label\0" as *const u8 as *const ::core::ffi::c_char,
        0xffff as i32,
    ) as u16;
    return color;
}
pub unsafe extern "C" fn otfcc_parseCPAL(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut CpalTable {
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"CPAL\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if table.is_null() {
        return ::core::ptr::null_mut::<CpalTable>();
    }
    let mut cpal: *mut CpalTable = ::core::ptr::null_mut::<CpalTable>();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"CPAL"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _palettes: *mut JsonValue = json_obj_get_type(
            table,
            b"palettes\0" as *const u8 as *const ::core::ffi::c_char,
            json_array,
        );
        if _palettes.is_null() || (*_palettes).u.array.length == 0 {
            return ::core::ptr::null_mut::<CpalTable>();
        }
        cpal = (
            table_iCPAL.create.expect("non-null function pointer"))();
        (*cpal).version = json_obj_getint(
            table,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u16;
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_uint) < (*_palettes).u.array.length {
            let mut _palette: *mut JsonValue =
                *(*_palettes).u.array.values.offset(j as isize) as *mut JsonValue;
            if !(_palette.is_null()
                || (*_palette).type_0 != json_object)
            {
                let mut _colors: *mut JsonValue = json_obj_get_type(
                    _palette,
                    b"colors\0" as *const u8 as *const ::core::ffi::c_char,
                    json_array,
                );
                if !_colors.is_null() {
                    let mut palette: CpalPalette = CpalPalette {
                        colorset: CpalColorSet {
                            length: 0,
                            capacity: 0,
                            items: ::core::ptr::null_mut::<CpalColor>(),
                        },
                        type_0: 0,
                        label: 0,
                    };
                    cpal_iPalette.init.expect("non-null function pointer")(&raw mut palette);
                    palette.type_0 = json_obj_getint(
                        _palette,
                        b"type\0" as *const u8 as *const ::core::ffi::c_char,
                    ) as u32;
                    palette.label = json_obj_getint_fallback(
                        _palette,
                        b"type\0" as *const u8 as *const ::core::ffi::c_char,
                        0xffff as i32,
                    ) as u32;
                    let mut k: ColorId = 0 as ColorId;
                    while (k as ::core::ffi::c_uint) < (*_colors).u.array.length {
                        cpal_iColorSet.push.expect("non-null function pointer")(
                            &raw mut palette.colorset,
                            parseColor(*(*_colors).u.array.values.offset(k as isize)),
                        );
                        k = k.wrapping_add(1);
                    }
                    cpal_iPaletteSet.push.expect("non-null function pointer")(
                        &raw mut (*cpal).palettes,
                        palette,
                    );
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return cpal;
}
#[inline]
unsafe extern "C" fn buildPaletteType(mut cpal: *const CpalTable) -> *mut BkBlock {
    let mut needsPaletteType: bool = false;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*cpal).palettes.length {
        if (*(*cpal).palettes.items.offset(j as isize)).type_0 != 0 {
            needsPaletteType = true;
        }
        j = j.wrapping_add(1);
    }
    if !needsPaletteType {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let mut block: *mut BkBlock = bk_new_Block(&[]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < (*cpal).palettes.length {
        bk_push(block, &[bk_int(b32, ((*(*cpal).palettes.items.offset(j_0 as isize)).type_0) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
#[inline]
unsafe extern "C" fn buildPaletteLabel(mut cpal: *const CpalTable) -> *mut BkBlock {
    let mut needsPaletteLabel: bool = false;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*cpal).palettes.length {
        if (*(*cpal).palettes.items.offset(j as isize)).label != 0xffff as u32 {
            needsPaletteLabel = true;
        }
        j = j.wrapping_add(1);
    }
    if !needsPaletteLabel {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let mut block: *mut BkBlock = bk_new_Block(&[]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < (*cpal).palettes.length {
        bk_push(block, &[bk_int(b16, ((*(*cpal).palettes.items.offset(j_0 as isize)).label) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
#[inline]
unsafe extern "C" fn buildPaletteEntryLabel(mut cpal: *const CpalTable) -> *mut BkBlock {
    let mut needsPaletteEntryLabel: bool = false;
    let mut palette: *mut CpalPalette = (*cpal)
        .palettes
        .items
        .offset(0 as ::core::ffi::c_int as isize)
        as *mut CpalPalette;
    let mut j: ColorId = 0 as ColorId;
    while (j as usize) < (*palette).colorset.length {
        if (*(*palette).colorset.items.offset(j as isize)).label as ::core::ffi::c_int
            != 0xffff as ::core::ffi::c_int
        {
            needsPaletteEntryLabel = true;
        }
        j = j.wrapping_add(1);
    }
    if !needsPaletteEntryLabel {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let mut block: *mut BkBlock = bk_new_Block(&[]);
    let mut j_0: ColorId = 0 as ColorId;
    while (j_0 as usize) < (*palette).colorset.length {
        bk_push(block, &[bk_int(b16, ((*(*palette).colorset.items.offset(j_0 as isize)).label as ::core::ffi::c_int) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
pub unsafe extern "C" fn otfcc_buildCPAL(
    mut cpal: *const CpalTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if cpal.is_null() || (*cpal).palettes.length == 0 {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut numPalettes: u16 = (*cpal).palettes.length as u16;
    let mut numPalettesEntries: u16 = (*(*cpal)
        .palettes
        .items
        .offset(0 as ::core::ffi::c_int as isize))
    .colorset
    .length as u16;
    let mut numColorRecords: u16 =
        (numPalettes as ::core::ffi::c_int * numPalettesEntries as ::core::ffi::c_int) as u16;
    let mut colorRecords: *mut BkBlock = bk_new_Block(&[]);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < numPalettes as ::core::ffi::c_int {
        let mut palette: *mut CpalPalette =
            (*cpal).palettes.items.offset(j as isize) as *mut CpalPalette;
        let mut totalColors: ColorId = (*palette).colorset.length as ColorId;
        let mut k: ColorId = 0 as ColorId;
        while (k as ::core::ffi::c_int) < numPalettesEntries as ::core::ffi::c_int {
            let mut color: *const CpalColor = ::core::ptr::null::<CpalColor>();
            if (k as ::core::ffi::c_int) < totalColors as ::core::ffi::c_int {
                color = (*palette).colorset.items.offset(k as isize) as *mut CpalColor;
            } else {
                color = &raw const white;
            }
            bk_push(colorRecords, &[bk_int(b8, ((*color).blue as ::core::ffi::c_int) as u32), bk_int(b8, ((*color).green as ::core::ffi::c_int) as u32), bk_int(b8, ((*color).red as ::core::ffi::c_int) as u32), bk_int(b8, ((*color).alpha as ::core::ffi::c_int) as u32)]);
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    let mut root: *mut BkBlock = bk_new_Block(&[bk_int(b16, ((*cpal).version as ::core::ffi::c_int) as u32), bk_int(b16, (numPalettesEntries as ::core::ffi::c_int) as u32), bk_int(b16, (numPalettes as ::core::ffi::c_int) as u32), bk_int(b16, (numColorRecords as ::core::ffi::c_int) as u32), bk_ptr(p32, colorRecords)]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int) < numPalettes as ::core::ffi::c_int {
        bk_push(root, &[bk_int(b16, (numPalettesEntries as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    if (*cpal).version as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(p32, buildPaletteType(cpal)), bk_ptr(p32, buildPaletteLabel(cpal)), bk_ptr(p32, buildPaletteEntryLabel(cpal))]);
    }
    return bk_build_Block(root);
}
