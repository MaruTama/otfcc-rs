use libc::{free, malloc, memcpy, memset, qsort, strcmp};
extern "C" {
    fn sdsempty() -> sds;
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_8u, read_16u, read_32u};
use crate::logger::{otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{colorid_t, font_file_pointer, tableid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_array, json_double, json_integer, json_object, json_pre_serialized, json_type, json_value};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{b16, b32, b8, bk_Block, bk_int, bk_new_Block, bk_ptr, bk_push, p32};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};

use crate::vendor::json_builder::{json_serialize_mode_packed, json_serialize_opts};
use crate::support::{__compar_fn_t};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    pub label: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_cpal_Color {
    pub init: Option<unsafe extern "C" fn(*mut cpal_Color) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut cpal_Color, *const cpal_Color) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut cpal_Color, *mut cpal_Color) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut cpal_Color) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut cpal_Color, cpal_Color) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut cpal_Color, cpal_Color) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_ColorSet {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut cpal_Color,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_cpal_ColorSet {
    pub init: Option<unsafe extern "C" fn(*mut cpal_ColorSet) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut cpal_ColorSet, *const cpal_ColorSet) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut cpal_ColorSet, *mut cpal_ColorSet) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut cpal_ColorSet) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut cpal_ColorSet, cpal_ColorSet) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut cpal_ColorSet, cpal_ColorSet) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut cpal_ColorSet>,
    pub free: Option<unsafe extern "C" fn(*mut cpal_ColorSet) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut cpal_ColorSet, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut cpal_ColorSet, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut cpal_ColorSet>,
    pub fill: Option<unsafe extern "C" fn(*mut cpal_ColorSet, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut cpal_ColorSet) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut cpal_ColorSet, cpal_Color) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut cpal_ColorSet) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut cpal_ColorSet) -> cpal_Color>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut cpal_ColorSet, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut cpal_ColorSet,
            Option<unsafe extern "C" fn(*const cpal_Color, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut cpal_ColorSet,
            Option<unsafe extern "C" fn(*const cpal_Color, *const cpal_Color) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_Palette {
    pub colorset: cpal_ColorSet,
    pub type_0: u32,
    pub label: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_cpal_Palette {
    pub init: Option<unsafe extern "C" fn(*mut cpal_Palette) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut cpal_Palette, *const cpal_Palette) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut cpal_Palette, *mut cpal_Palette) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut cpal_Palette) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut cpal_Palette, cpal_Palette) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut cpal_Palette, cpal_Palette) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpal_PaletteSet {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut cpal_Palette,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_cpal_PaletteSet {
    pub init: Option<unsafe extern "C" fn(*mut cpal_PaletteSet) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut cpal_PaletteSet, *const cpal_PaletteSet) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut cpal_PaletteSet, *mut cpal_PaletteSet) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut cpal_PaletteSet) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut cpal_PaletteSet, cpal_PaletteSet) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut cpal_PaletteSet, cpal_PaletteSet) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut cpal_PaletteSet>,
    pub free: Option<unsafe extern "C" fn(*mut cpal_PaletteSet) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut cpal_PaletteSet, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut cpal_PaletteSet, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut cpal_PaletteSet>,
    pub fill: Option<unsafe extern "C" fn(*mut cpal_PaletteSet, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut cpal_PaletteSet) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut cpal_PaletteSet, cpal_Palette) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut cpal_PaletteSet) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut cpal_PaletteSet) -> cpal_Palette>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut cpal_PaletteSet, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut cpal_PaletteSet,
            Option<unsafe extern "C" fn(*const cpal_Palette, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut cpal_PaletteSet,
            Option<
                unsafe extern "C" fn(
                    *const cpal_Palette,
                    *const cpal_Palette,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_CPAL {
    pub version: u16,
    pub palettes: cpal_PaletteSet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_CPAL {
    pub init: Option<unsafe extern "C" fn(*mut table_CPAL) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_CPAL, *const table_CPAL) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_CPAL, *mut table_CPAL) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_CPAL) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_CPAL, table_CPAL) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_CPAL, table_CPAL) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_CPAL>,
    pub free: Option<unsafe extern "C" fn(*mut table_CPAL) -> ()>,
}
#[inline]
unsafe extern "C" fn cpal_Color_move(mut dst: *mut cpal_Color, mut src: *mut cpal_Color) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cpal_Color>() as usize,
    );
    cpal_Color_init(src);
}
#[inline]
unsafe extern "C" fn cpal_Color_replace(mut dst: *mut cpal_Color, src: cpal_Color) {
    cpal_Color_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cpal_Color>() as usize,
    );
}
#[no_mangle]
pub static cpal_iColor: __caryll_elementinterface_cpal_Color = {
    __caryll_elementinterface_cpal_Color {
        init: Some(cpal_Color_init as unsafe extern "C" fn(*mut cpal_Color) -> ()),
        copy: Some(
            cpal_Color_copy as unsafe extern "C" fn(*mut cpal_Color, *const cpal_Color) -> (),
        ),
        move_0: Some(
            cpal_Color_move as unsafe extern "C" fn(*mut cpal_Color, *mut cpal_Color) -> (),
        ),
        dispose: Some(cpal_Color_dispose as unsafe extern "C" fn(*mut cpal_Color) -> ()),
        replace: Some(
            cpal_Color_replace as unsafe extern "C" fn(*mut cpal_Color, cpal_Color) -> (),
        ),
        copyReplace: Some(
            cpal_Color_copyReplace as unsafe extern "C" fn(*mut cpal_Color, cpal_Color) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn cpal_Color_init(mut x: *mut cpal_Color) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cpal_Color>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cpal_Color_copyReplace(mut dst: *mut cpal_Color, src: cpal_Color) {
    cpal_Color_dispose(dst);
    cpal_Color_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cpal_Color_copy(mut dst: *mut cpal_Color, mut src: *const cpal_Color) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cpal_Color>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cpal_Color_dispose(mut _x: *mut cpal_Color) {}
#[inline]
unsafe extern "C" fn cpal_ColorSet_disposeItem(mut arr: *mut cpal_ColorSet, mut n: usize) {
    if cpal_iColor.dispose.is_some() {
        cpal_iColor.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut cpal_Color
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_sort(
    mut arr: *mut cpal_ColorSet,
    mut fn_0: Option<
        unsafe extern "C" fn(*const cpal_Color, *const cpal_Color) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<cpal_Color>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const cpal_Color, *const cpal_Color) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_copy(
    mut dst: *mut cpal_ColorSet,
    mut src: *const cpal_ColorSet,
) {
    cpal_ColorSet_init(dst);
    cpal_ColorSet_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if cpal_iColor.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            cpal_iColor.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut cpal_Color,
                (*src).items.offset(j as isize) as *mut cpal_Color as *const cpal_Color,
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
unsafe extern "C" fn cpal_ColorSet_dispose(mut arr: *mut cpal_ColorSet) {
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
                (*arr).items.offset(j as isize) as *mut cpal_Color,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<cpal_Color>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_replace(mut dst: *mut cpal_ColorSet, src: cpal_ColorSet) {
    cpal_ColorSet_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cpal_ColorSet>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_initCapN(mut arr: *mut cpal_ColorSet, mut n: usize) {
    cpal_ColorSet_init(arr);
    cpal_ColorSet_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_growToN(arr: *mut cpal_ColorSet, target: usize) {
    cvec_grow_to_n(cpal_ColorSet_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_initN(mut arr: *mut cpal_ColorSet, mut n: usize) {
    cpal_ColorSet_init(arr);
    cpal_ColorSet_growToN(arr, n);
    cpal_ColorSet_fill(arr, n);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_free(mut x: *mut cpal_ColorSet) {
    if x.is_null() {
        return;
    }
    cpal_ColorSet_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_createN(mut n: usize) -> *mut cpal_ColorSet {
    let mut t: *mut cpal_ColorSet =
        malloc(::core::mem::size_of::<cpal_ColorSet>() as usize) as *mut cpal_ColorSet;
    cpal_ColorSet_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_create() -> *mut cpal_ColorSet {
    let mut x: *mut cpal_ColorSet =
        malloc(::core::mem::size_of::<cpal_ColorSet>() as usize) as *mut cpal_ColorSet;
    cpal_ColorSet_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_fill(mut arr: *mut cpal_ColorSet, mut n: usize) {
    while (*arr).length < n {
        let mut x: cpal_Color = cpal_Color {
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
                ::core::mem::size_of::<cpal_Color>() as usize,
            );
        }
        cpal_ColorSet_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_push(arr: *mut cpal_ColorSet, elem: cpal_Color) {
    cvec_push(cpal_ColorSet_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_grow(arr: *mut cpal_ColorSet) {
    cvec_grow(cpal_ColorSet_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_move(dst: *mut cpal_ColorSet, src: *mut cpal_ColorSet) {
    cvec_move(cpal_ColorSet_as_cvec(dst), cpal_ColorSet_as_cvec(src));
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_shrinkToFit(mut arr: *mut cpal_ColorSet) {
    cpal_ColorSet_resizeTo(arr, (*arr).length);
}
#[no_mangle]
pub static cpal_iColorSet: __caryll_vectorinterface_cpal_ColorSet = {
    __caryll_vectorinterface_cpal_ColorSet {
        init: Some(cpal_ColorSet_init as unsafe extern "C" fn(*mut cpal_ColorSet) -> ()),
        copy: Some(
            cpal_ColorSet_copy
                as unsafe extern "C" fn(*mut cpal_ColorSet, *const cpal_ColorSet) -> (),
        ),
        move_0: Some(
            cpal_ColorSet_move
                as unsafe extern "C" fn(*mut cpal_ColorSet, *mut cpal_ColorSet) -> (),
        ),
        dispose: Some(cpal_ColorSet_dispose as unsafe extern "C" fn(*mut cpal_ColorSet) -> ()),
        replace: Some(
            cpal_ColorSet_replace as unsafe extern "C" fn(*mut cpal_ColorSet, cpal_ColorSet) -> (),
        ),
        copyReplace: Some(
            cpal_ColorSet_copyReplace
                as unsafe extern "C" fn(*mut cpal_ColorSet, cpal_ColorSet) -> (),
        ),
        create: Some(cpal_ColorSet_create),
        free: Some(cpal_ColorSet_free as unsafe extern "C" fn(*mut cpal_ColorSet) -> ()),
        initN: Some(cpal_ColorSet_initN as unsafe extern "C" fn(*mut cpal_ColorSet, usize) -> ()),
        initCapN: Some(
            cpal_ColorSet_initCapN as unsafe extern "C" fn(*mut cpal_ColorSet, usize) -> (),
        ),
        createN: Some(cpal_ColorSet_createN as unsafe extern "C" fn(usize) -> *mut cpal_ColorSet),
        fill: Some(cpal_ColorSet_fill as unsafe extern "C" fn(*mut cpal_ColorSet, usize) -> ()),
        clear: Some(cpal_ColorSet_dispose as unsafe extern "C" fn(*mut cpal_ColorSet) -> ()),
        push: Some(
            cpal_ColorSet_push as unsafe extern "C" fn(*mut cpal_ColorSet, cpal_Color) -> (),
        ),
        shrinkToFit: Some(
            cpal_ColorSet_shrinkToFit as unsafe extern "C" fn(*mut cpal_ColorSet) -> (),
        ),
        pop: Some(cpal_ColorSet_pop as unsafe extern "C" fn(*mut cpal_ColorSet) -> cpal_Color),
        disposeItem: Some(
            cpal_ColorSet_disposeItem as unsafe extern "C" fn(*mut cpal_ColorSet, usize) -> (),
        ),
        filterEnv: Some(
            cpal_ColorSet_filterEnv
                as unsafe extern "C" fn(
                    *mut cpal_ColorSet,
                    Option<
                        unsafe extern "C" fn(*const cpal_Color, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            cpal_ColorSet_sort
                as unsafe extern "C" fn(
                    *mut cpal_ColorSet,
                    Option<
                        unsafe extern "C" fn(
                            *const cpal_Color,
                            *const cpal_Color,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn cpal_ColorSet_growTo(arr: *mut cpal_ColorSet, target: usize) {
    cvec_grow_to(cpal_ColorSet_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_pop(arr: *mut cpal_ColorSet) -> cpal_Color {
    cvec_pop(cpal_ColorSet_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_resizeTo(arr: *mut cpal_ColorSet, target: usize) {
    cvec_resize_to(cpal_ColorSet_as_cvec(arr), target);
}
#[inline]
unsafe fn cpal_ColorSet_as_cvec(arr: *mut cpal_ColorSet) -> *mut CVecRaw<cpal_Color> {
    arr as *mut CVecRaw<cpal_Color>
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_init(arr: *mut cpal_ColorSet) {
    cvec_init(cpal_ColorSet_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_copyReplace(mut dst: *mut cpal_ColorSet, src: cpal_ColorSet) {
    cpal_ColorSet_dispose(dst);
    cpal_ColorSet_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cpal_ColorSet_filterEnv(
    mut arr: *mut cpal_ColorSet,
    mut fn_0: Option<unsafe extern "C" fn(*const cpal_Color, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut cpal_Color,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if cpal_iColor.dispose.is_some() {
                cpal_iColor.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut cpal_Color,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn initPalette(mut p: *mut cpal_Palette) {
    cpal_iColorSet.init.expect("non-null function pointer")(&raw mut (*p).colorset);
    (*p).type_0 = 0 as u32;
    (*p).label = 0xffff as u32;
}
#[inline]
unsafe extern "C" fn disposePalette(mut p: *mut cpal_Palette) {
    cpal_iColorSet.dispose.expect("non-null function pointer")(&raw mut (*p).colorset);
}
#[no_mangle]
pub static cpal_iPalette: __caryll_elementinterface_cpal_Palette = {
    __caryll_elementinterface_cpal_Palette {
        init: Some(cpal_Palette_init as unsafe extern "C" fn(*mut cpal_Palette) -> ()),
        copy: Some(
            cpal_Palette_copy as unsafe extern "C" fn(*mut cpal_Palette, *const cpal_Palette) -> (),
        ),
        move_0: Some(
            cpal_Palette_move as unsafe extern "C" fn(*mut cpal_Palette, *mut cpal_Palette) -> (),
        ),
        dispose: Some(cpal_Palette_dispose as unsafe extern "C" fn(*mut cpal_Palette) -> ()),
        replace: Some(
            cpal_Palette_replace as unsafe extern "C" fn(*mut cpal_Palette, cpal_Palette) -> (),
        ),
        copyReplace: Some(
            cpal_Palette_copyReplace as unsafe extern "C" fn(*mut cpal_Palette, cpal_Palette) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn cpal_Palette_copyReplace(mut dst: *mut cpal_Palette, src: cpal_Palette) {
    cpal_Palette_dispose(dst);
    cpal_Palette_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cpal_Palette_move(mut dst: *mut cpal_Palette, mut src: *mut cpal_Palette) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cpal_Palette>() as usize,
    );
    cpal_Palette_init(src);
}
#[inline]
unsafe extern "C" fn cpal_Palette_replace(mut dst: *mut cpal_Palette, src: cpal_Palette) {
    cpal_Palette_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cpal_Palette>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cpal_Palette_dispose(mut x: *mut cpal_Palette) {
    disposePalette(x);
}
#[inline]
unsafe extern "C" fn cpal_Palette_init(mut x: *mut cpal_Palette) {
    initPalette(x);
}
#[inline]
unsafe extern "C" fn cpal_Palette_copy(mut dst: *mut cpal_Palette, mut src: *const cpal_Palette) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cpal_Palette>() as usize,
    );
}
#[inline]
unsafe fn cpal_PaletteSet_as_cvec(arr: *mut cpal_PaletteSet) -> *mut CVecRaw<cpal_Palette> {
    arr as *mut CVecRaw<cpal_Palette>
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_init(arr: *mut cpal_PaletteSet) {
    cvec_init(cpal_PaletteSet_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_move(dst: *mut cpal_PaletteSet, src: *mut cpal_PaletteSet) {
    cvec_move(cpal_PaletteSet_as_cvec(dst), cpal_PaletteSet_as_cvec(src));
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_filterEnv(
    mut arr: *mut cpal_PaletteSet,
    mut fn_0: Option<unsafe extern "C" fn(*const cpal_Palette, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut cpal_Palette,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if cpal_iPalette.dispose.is_some() {
                cpal_iPalette.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut cpal_Palette,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_disposeItem(mut arr: *mut cpal_PaletteSet, mut n: usize) {
    if cpal_iPalette.dispose.is_some() {
        cpal_iPalette.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut cpal_Palette
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_sort(
    mut arr: *mut cpal_PaletteSet,
    mut fn_0: Option<
        unsafe extern "C" fn(*const cpal_Palette, *const cpal_Palette) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<cpal_Palette>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const cpal_Palette,
                    *const cpal_Palette,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_fill(mut arr: *mut cpal_PaletteSet, mut n: usize) {
    while (*arr).length < n {
        let mut x: cpal_Palette = cpal_Palette {
            colorset: cpal_ColorSet {
                length: 0,
                capacity: 0,
                items: ::core::ptr::null_mut::<cpal_Color>(),
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
                ::core::mem::size_of::<cpal_Palette>() as usize,
            );
        }
        cpal_PaletteSet_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_push(arr: *mut cpal_PaletteSet, elem: cpal_Palette) {
    cvec_push(cpal_PaletteSet_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_grow(arr: *mut cpal_PaletteSet) {
    cvec_grow(cpal_PaletteSet_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_growTo(arr: *mut cpal_PaletteSet, target: usize) {
    cvec_grow_to(cpal_PaletteSet_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_pop(arr: *mut cpal_PaletteSet) -> cpal_Palette {
    cvec_pop(cpal_PaletteSet_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_copyReplace(
    mut dst: *mut cpal_PaletteSet,
    src: cpal_PaletteSet,
) {
    cpal_PaletteSet_dispose(dst);
    cpal_PaletteSet_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_copy(
    mut dst: *mut cpal_PaletteSet,
    mut src: *const cpal_PaletteSet,
) {
    cpal_PaletteSet_init(dst);
    cpal_PaletteSet_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if cpal_iPalette.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            cpal_iPalette.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut cpal_Palette,
                (*src).items.offset(j as isize) as *mut cpal_Palette as *const cpal_Palette,
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
unsafe extern "C" fn cpal_PaletteSet_dispose(mut arr: *mut cpal_PaletteSet) {
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
                (*arr).items.offset(j as isize) as *mut cpal_Palette,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<cpal_Palette>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_replace(mut dst: *mut cpal_PaletteSet, src: cpal_PaletteSet) {
    cpal_PaletteSet_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cpal_PaletteSet>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_initCapN(mut arr: *mut cpal_PaletteSet, mut n: usize) {
    cpal_PaletteSet_init(arr);
    cpal_PaletteSet_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_growToN(arr: *mut cpal_PaletteSet, target: usize) {
    cvec_grow_to_n(cpal_PaletteSet_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_initN(mut arr: *mut cpal_PaletteSet, mut n: usize) {
    cpal_PaletteSet_init(arr);
    cpal_PaletteSet_growToN(arr, n);
    cpal_PaletteSet_fill(arr, n);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_free(mut x: *mut cpal_PaletteSet) {
    if x.is_null() {
        return;
    }
    cpal_PaletteSet_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_createN(mut n: usize) -> *mut cpal_PaletteSet {
    let mut t: *mut cpal_PaletteSet =
        malloc(::core::mem::size_of::<cpal_PaletteSet>() as usize) as *mut cpal_PaletteSet;
    cpal_PaletteSet_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_create() -> *mut cpal_PaletteSet {
    let mut x: *mut cpal_PaletteSet =
        malloc(::core::mem::size_of::<cpal_PaletteSet>() as usize) as *mut cpal_PaletteSet;
    cpal_PaletteSet_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn cpal_PaletteSet_shrinkToFit(mut arr: *mut cpal_PaletteSet) {
    cpal_PaletteSet_resizeTo(arr, (*arr).length);
}
#[no_mangle]
pub static cpal_iPaletteSet: __caryll_vectorinterface_cpal_PaletteSet = {
    __caryll_vectorinterface_cpal_PaletteSet {
        init: Some(cpal_PaletteSet_init as unsafe extern "C" fn(*mut cpal_PaletteSet) -> ()),
        copy: Some(
            cpal_PaletteSet_copy
                as unsafe extern "C" fn(*mut cpal_PaletteSet, *const cpal_PaletteSet) -> (),
        ),
        move_0: Some(
            cpal_PaletteSet_move
                as unsafe extern "C" fn(*mut cpal_PaletteSet, *mut cpal_PaletteSet) -> (),
        ),
        dispose: Some(cpal_PaletteSet_dispose as unsafe extern "C" fn(*mut cpal_PaletteSet) -> ()),
        replace: Some(
            cpal_PaletteSet_replace
                as unsafe extern "C" fn(*mut cpal_PaletteSet, cpal_PaletteSet) -> (),
        ),
        copyReplace: Some(
            cpal_PaletteSet_copyReplace
                as unsafe extern "C" fn(*mut cpal_PaletteSet, cpal_PaletteSet) -> (),
        ),
        create: Some(cpal_PaletteSet_create),
        free: Some(cpal_PaletteSet_free as unsafe extern "C" fn(*mut cpal_PaletteSet) -> ()),
        initN: Some(
            cpal_PaletteSet_initN as unsafe extern "C" fn(*mut cpal_PaletteSet, usize) -> (),
        ),
        initCapN: Some(
            cpal_PaletteSet_initCapN as unsafe extern "C" fn(*mut cpal_PaletteSet, usize) -> (),
        ),
        createN: Some(
            cpal_PaletteSet_createN as unsafe extern "C" fn(usize) -> *mut cpal_PaletteSet,
        ),
        fill: Some(
            cpal_PaletteSet_fill as unsafe extern "C" fn(*mut cpal_PaletteSet, usize) -> (),
        ),
        clear: Some(cpal_PaletteSet_dispose as unsafe extern "C" fn(*mut cpal_PaletteSet) -> ()),
        push: Some(
            cpal_PaletteSet_push as unsafe extern "C" fn(*mut cpal_PaletteSet, cpal_Palette) -> (),
        ),
        shrinkToFit: Some(
            cpal_PaletteSet_shrinkToFit as unsafe extern "C" fn(*mut cpal_PaletteSet) -> (),
        ),
        pop: Some(
            cpal_PaletteSet_pop as unsafe extern "C" fn(*mut cpal_PaletteSet) -> cpal_Palette,
        ),
        disposeItem: Some(
            cpal_PaletteSet_disposeItem as unsafe extern "C" fn(*mut cpal_PaletteSet, usize) -> (),
        ),
        filterEnv: Some(
            cpal_PaletteSet_filterEnv
                as unsafe extern "C" fn(
                    *mut cpal_PaletteSet,
                    Option<
                        unsafe extern "C" fn(*const cpal_Palette, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            cpal_PaletteSet_sort
                as unsafe extern "C" fn(
                    *mut cpal_PaletteSet,
                    Option<
                        unsafe extern "C" fn(
                            *const cpal_Palette,
                            *const cpal_Palette,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn cpal_PaletteSet_resizeTo(arr: *mut cpal_PaletteSet, target: usize) {
    cvec_resize_to(cpal_PaletteSet_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn initCPAL(mut cpal: *mut table_CPAL) {
    (*cpal).version = 1 as u16;
    cpal_iPaletteSet.init.expect("non-null function pointer")(&raw mut (*cpal).palettes);
}
#[inline]
unsafe extern "C" fn disposeCPAL(mut cpal: *mut table_CPAL) {
    cpal_iPaletteSet.dispose.expect("non-null function pointer")(&raw mut (*cpal).palettes);
}
#[inline]
unsafe extern "C" fn table_CPAL_dispose(mut x: *mut table_CPAL) {
    disposeCPAL(x);
}
#[inline]
unsafe extern "C" fn table_CPAL_init(mut x: *mut table_CPAL) {
    initCPAL(x);
}
#[inline]
unsafe extern "C" fn table_CPAL_create() -> *mut table_CPAL {
    let mut x: *mut table_CPAL =
        malloc(::core::mem::size_of::<table_CPAL>() as usize) as *mut table_CPAL;
    table_CPAL_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_CPAL_copyReplace(mut dst: *mut table_CPAL, src: table_CPAL) {
    table_CPAL_dispose(dst);
    table_CPAL_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_CPAL_copy(mut dst: *mut table_CPAL, mut src: *const table_CPAL) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_CPAL>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_CPAL_replace(mut dst: *mut table_CPAL, src: table_CPAL) {
    table_CPAL_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_CPAL>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_CPAL_move(mut dst: *mut table_CPAL, mut src: *mut table_CPAL) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_CPAL>() as usize,
    );
    table_CPAL_init(src);
}
#[no_mangle]
pub static table_iCPAL: __caryll_elementinterface_table_CPAL = {
    __caryll_elementinterface_table_CPAL {
        init: Some(table_CPAL_init as unsafe extern "C" fn(*mut table_CPAL) -> ()),
        copy: Some(
            table_CPAL_copy as unsafe extern "C" fn(*mut table_CPAL, *const table_CPAL) -> (),
        ),
        move_0: Some(
            table_CPAL_move as unsafe extern "C" fn(*mut table_CPAL, *mut table_CPAL) -> (),
        ),
        dispose: Some(table_CPAL_dispose as unsafe extern "C" fn(*mut table_CPAL) -> ()),
        replace: Some(
            table_CPAL_replace as unsafe extern "C" fn(*mut table_CPAL, table_CPAL) -> (),
        ),
        copyReplace: Some(
            table_CPAL_copyReplace as unsafe extern "C" fn(*mut table_CPAL, table_CPAL) -> (),
        ),
        create: Some(table_CPAL_create),
        free: Some(table_CPAL_free as unsafe extern "C" fn(*mut table_CPAL) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_CPAL_free(mut x: *mut table_CPAL) {
    if x.is_null() {
        return;
    }
    table_CPAL_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub static white: cpal_Color = cpal_Color {
    red: 0xff as u8,
    green: 0xff as u8,
    blue: 0xff as u8,
    alpha: 0xff as u8,
    label: 0xffff as u16,
};
#[no_mangle]
pub unsafe extern "C" fn otfcc_readCPAL(
    packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
) -> *mut table_CPAL {
    let mut version: u16 = 0;
    let mut tableHeaderLength: u32 = 0;
    let mut numPalettesEntries: u16 = 0;
    let mut numPalettes: u16 = 0;
    let mut numColorRecords: u16 = 0;
    let mut offsetFirstColorRecord: u32 = 0;
    let mut colorList: *mut cpal_Color = ::core::ptr::null_mut::<cpal_Color>();
    let mut t: *mut table_CPAL = ::core::ptr::null_mut::<table_CPAL>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1129333068i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
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
                                    colorList = ::core::ptr::null_mut::<cpal_Color>();
                                    colorList = __caryll_allocate_clean(
                                        (::core::mem::size_of::<cpal_Color>() as usize)
                                            .wrapping_mul(numColorRecords as usize),
                                        55 as ::core::ffi::c_ulong,
                                    )
                                        as *mut cpal_Color;
                                    let mut j: u16 = 0 as u16;
                                    while (j as ::core::ffi::c_int)
                                        < numColorRecords as ::core::ffi::c_int
                                    {
                                        *colorList.offset(j as isize) = cpal_Color {
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
                                    let mut j_0: tableid_t = 0 as tableid_t;
                                    while (j_0 as ::core::ffi::c_int)
                                        < numPalettes as ::core::ffi::c_int
                                    {
                                        let mut palette: cpal_Palette = cpal_Palette {
                                            colorset: cpal_ColorSet {
                                                length: 0,
                                                capacity: 0,
                                                items: ::core::ptr::null_mut::<cpal_Color>(),
                                            },
                                            type_0: 0,
                                            label: 0,
                                        };
                                        cpal_iPalette.init.expect("non-null function pointer")(
                                            &raw mut palette,
                                        );
                                        let mut paletteStartIndex: tableid_t = read_16u(
                                            data.offset(12 as ::core::ffi::c_int as isize).offset(
                                                (j_0 as ::core::ffi::c_int
                                                    * 2 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as *const u8,
                                        )
                                            as tableid_t;
                                        let mut j_1: colorid_t = 0 as colorid_t;
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
                                            let mut j_2: tableid_t = 0 as tableid_t;
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
                                            let mut j_3: tableid_t = 0 as tableid_t;
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
                                                let mut j_4: colorid_t = 0 as colorid_t;
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
                                                    let mut k: tableid_t = 0 as tableid_t;
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
                                    colorList = ::core::ptr::null_mut::<cpal_Color>();
                                    return t;
                                }
                            }
                        }
                    }
                    table_iCPAL.free.expect("non-null function pointer")(t);
                    t = ::core::ptr::null_mut::<table_CPAL>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_CPAL>();
}
#[inline]
unsafe extern "C" fn dumpColor(mut color: *mut cpal_Color) -> *mut json_value {
    let mut _color: *mut json_value = json_object_new(5 as usize);
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
unsafe extern "C" fn dumpPalette(mut palette: *mut cpal_Palette) -> *mut json_value {
    let mut _palette: *mut json_value = json_object_new(3 as usize);
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
    let mut a: *mut json_value = json_array_new((*palette).colorset.length);
    let mut j: colorid_t = 0 as colorid_t;
    while (j as usize) < (*palette).colorset.length {
        json_array_push(
            a,
            dumpColor((*palette).colorset.items.offset(j as isize) as *mut cpal_Color),
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
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpCPAL(
    mut table: *const table_CPAL,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"CPAL"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _t: *mut json_value = json_object_new(2 as usize);
        json_object_push(
            _t,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).version as i64),
        );
        let mut _a: *mut json_value = json_array_new((*table).palettes.length);
        let mut j: tableid_t = 0 as tableid_t;
        while (j as usize) < (*table).palettes.length {
            json_array_push(
                _a,
                dumpPalette((*table).palettes.items.offset(j as isize) as *mut cpal_Palette),
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
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[inline]
unsafe extern "C" fn parseColor(mut _color: *const json_value) -> cpal_Color {
    let mut color: cpal_Color = white;
    if _color.is_null()
        || (*_color).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
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
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseCPAL(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_CPAL {
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"CPAL\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if table.is_null() {
        return ::core::ptr::null_mut::<table_CPAL>();
    }
    let mut cpal: *mut table_CPAL = ::core::ptr::null_mut::<table_CPAL>();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"CPAL"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _palettes: *mut json_value = json_obj_get_type(
            table,
            b"palettes\0" as *const u8 as *const ::core::ffi::c_char,
            json_array,
        );
        if _palettes.is_null() || (*_palettes).u.array.length == 0 {
            return ::core::ptr::null_mut::<table_CPAL>();
        }
        cpal = (
            table_iCPAL.create.expect("non-null function pointer"))();
        (*cpal).version = json_obj_getint(
            table,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u16;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_uint) < (*_palettes).u.array.length {
            let mut _palette: *mut json_value =
                *(*_palettes).u.array.values.offset(j as isize) as *mut json_value;
            if !(_palette.is_null()
                || (*_palette).type_0 as ::core::ffi::c_uint
                    != json_object as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                let mut _colors: *mut json_value = json_obj_get_type(
                    _palette,
                    b"colors\0" as *const u8 as *const ::core::ffi::c_char,
                    json_array,
                );
                if !_colors.is_null() {
                    let mut palette: cpal_Palette = cpal_Palette {
                        colorset: cpal_ColorSet {
                            length: 0,
                            capacity: 0,
                            items: ::core::ptr::null_mut::<cpal_Color>(),
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
                    let mut k: colorid_t = 0 as colorid_t;
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
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    return cpal;
}
#[inline]
unsafe extern "C" fn buildPaletteType(mut cpal: *const table_CPAL) -> *mut bk_Block {
    let mut needsPaletteType: bool = false;
    let mut j: tableid_t = 0 as tableid_t;
    while (j as usize) < (*cpal).palettes.length {
        if (*(*cpal).palettes.items.offset(j as isize)).type_0 != 0 {
            needsPaletteType = true;
        }
        j = j.wrapping_add(1);
    }
    if !needsPaletteType {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let mut block: *mut bk_Block = bk_new_Block(&[]);
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as usize) < (*cpal).palettes.length {
        bk_push(block, &[bk_int(b32, ((*(*cpal).palettes.items.offset(j_0 as isize)).type_0) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
#[inline]
unsafe extern "C" fn buildPaletteLabel(mut cpal: *const table_CPAL) -> *mut bk_Block {
    let mut needsPaletteLabel: bool = false;
    let mut j: tableid_t = 0 as tableid_t;
    while (j as usize) < (*cpal).palettes.length {
        if (*(*cpal).palettes.items.offset(j as isize)).label != 0xffff as u32 {
            needsPaletteLabel = true;
        }
        j = j.wrapping_add(1);
    }
    if !needsPaletteLabel {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let mut block: *mut bk_Block = bk_new_Block(&[]);
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as usize) < (*cpal).palettes.length {
        bk_push(block, &[bk_int(b16, ((*(*cpal).palettes.items.offset(j_0 as isize)).label) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
#[inline]
unsafe extern "C" fn buildPaletteEntryLabel(mut cpal: *const table_CPAL) -> *mut bk_Block {
    let mut needsPaletteEntryLabel: bool = false;
    let mut palette: *mut cpal_Palette = (*cpal)
        .palettes
        .items
        .offset(0 as ::core::ffi::c_int as isize)
        as *mut cpal_Palette;
    let mut j: colorid_t = 0 as colorid_t;
    while (j as usize) < (*palette).colorset.length {
        if (*(*palette).colorset.items.offset(j as isize)).label as ::core::ffi::c_int
            != 0xffff as ::core::ffi::c_int
        {
            needsPaletteEntryLabel = true;
        }
        j = j.wrapping_add(1);
    }
    if !needsPaletteEntryLabel {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let mut block: *mut bk_Block = bk_new_Block(&[]);
    let mut j_0: colorid_t = 0 as colorid_t;
    while (j_0 as usize) < (*palette).colorset.length {
        bk_push(block, &[bk_int(b16, ((*(*palette).colorset.items.offset(j_0 as isize)).label as ::core::ffi::c_int) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildCPAL(
    mut cpal: *const table_CPAL,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if cpal.is_null() || (*cpal).palettes.length == 0 {
        return ::core::ptr::null_mut::<caryll_Buffer>();
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
    let mut colorRecords: *mut bk_Block = bk_new_Block(&[]);
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < numPalettes as ::core::ffi::c_int {
        let mut palette: *mut cpal_Palette =
            (*cpal).palettes.items.offset(j as isize) as *mut cpal_Palette;
        let mut totalColors: colorid_t = (*palette).colorset.length as colorid_t;
        let mut k: colorid_t = 0 as colorid_t;
        while (k as ::core::ffi::c_int) < numPalettesEntries as ::core::ffi::c_int {
            let mut color: *const cpal_Color = ::core::ptr::null::<cpal_Color>();
            if (k as ::core::ffi::c_int) < totalColors as ::core::ffi::c_int {
                color = (*palette).colorset.items.offset(k as isize) as *mut cpal_Color;
            } else {
                color = &raw const white;
            }
            bk_push(colorRecords, &[bk_int(b8, ((*color).blue as ::core::ffi::c_int) as u32), bk_int(b8, ((*color).green as ::core::ffi::c_int) as u32), bk_int(b8, ((*color).red as ::core::ffi::c_int) as u32), bk_int(b8, ((*color).alpha as ::core::ffi::c_int) as u32)]);
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    let mut root: *mut bk_Block = bk_new_Block(&[bk_int(b16, ((*cpal).version as ::core::ffi::c_int) as u32), bk_int(b16, (numPalettesEntries as ::core::ffi::c_int) as u32), bk_int(b16, (numPalettes as ::core::ffi::c_int) as u32), bk_int(b16, (numColorRecords as ::core::ffi::c_int) as u32), bk_ptr(p32, colorRecords)]);
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as ::core::ffi::c_int) < numPalettes as ::core::ffi::c_int {
        bk_push(root, &[bk_int(b16, (numPalettesEntries as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    if (*cpal).version as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        bk_push(root, &[bk_ptr(p32, buildPaletteType(cpal)), bk_ptr(p32, buildPaletteLabel(cpal)), bk_ptr(p32, buildPaletteEntryLabel(cpal))]);
    }
    return bk_build_Block(root);
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
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
unsafe extern "C" fn json_obj_getint(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> i32 {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
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
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as i32;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl as i32;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn json_obj_getint_fallback(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    mut fallback: i32,
) -> i32 {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return fallback;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as i32;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl as i32;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return fallback;
}
#[inline]
unsafe extern "C" fn preserialize(mut x: *mut json_value) -> *mut json_value {
    let mut opts: json_serialize_opts = json_serialize_opts {
        mode: json_serialize_mode_packed,
        opts: 0,
        indent_size: 0,
    };
    let mut preserialize_len: usize = json_measure_ex(x, opts);
    let mut buf: *mut ::core::ffi::c_char = malloc(preserialize_len) as *mut ::core::ffi::c_char;
    json_serialize_ex(buf, x, opts);
    json_builder_free(x);
    let mut xx: *mut json_value = json_string_new_nocopy(
        preserialize_len.wrapping_sub(1 as usize) as ::core::ffi::c_uint,
        buf,
    );
    (*xx).type_0 = json_pre_serialized;
    return xx;
}
