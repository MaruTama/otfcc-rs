use libc::{free, malloc, memcpy, memset, qsort};
use crate::vendor::sds::{sds};
extern "C" {
    fn sdsfree(s: sds);
}
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::support::{__compar_fn_t};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct meta_Entry {
    pub tag: u32,
    pub data: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_meta_Entry {
    pub init: Option<unsafe extern "C" fn(*mut meta_Entry) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut meta_Entry, *const meta_Entry) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut meta_Entry, *mut meta_Entry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut meta_Entry) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut meta_Entry, meta_Entry) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut meta_Entry, meta_Entry) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct meta_Entries {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut meta_Entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_meta_Entries {
    pub init: Option<unsafe extern "C" fn(*mut meta_Entries) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut meta_Entries, *const meta_Entries) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut meta_Entries, *mut meta_Entries) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut meta_Entries) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut meta_Entries, meta_Entries) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut meta_Entries, meta_Entries) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut meta_Entries>,
    pub free: Option<unsafe extern "C" fn(*mut meta_Entries) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut meta_Entries, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut meta_Entries, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut meta_Entries>,
    pub fill: Option<unsafe extern "C" fn(*mut meta_Entries, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut meta_Entries) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut meta_Entries, meta_Entry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut meta_Entries) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut meta_Entries) -> meta_Entry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut meta_Entries, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut meta_Entries,
            Option<unsafe extern "C" fn(*const meta_Entry, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut meta_Entries,
            Option<unsafe extern "C" fn(*const meta_Entry, *const meta_Entry) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_meta {
    pub version: u32,
    pub flags: u32,
    pub entries: meta_Entries,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_meta {
    pub init: Option<unsafe extern "C" fn(*mut table_meta) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_meta, *const table_meta) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_meta, *mut table_meta) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_meta) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_meta, table_meta) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_meta, table_meta) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_meta>,
    pub free: Option<unsafe extern "C" fn(*mut table_meta) -> ()>,
}
unsafe extern "C" fn initMetaEntry(mut e: *mut meta_Entry) {
    (*e).tag = 1 as u32;
    (*e).data = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn disposeMetaEntry(mut e: *mut meta_Entry) {
    sdsfree((*e).data);
}
#[no_mangle]
pub static mut meta_iEntry: __caryll_elementinterface_meta_Entry = {
    __caryll_elementinterface_meta_Entry {
        init: Some(meta_Entry_init as unsafe extern "C" fn(*mut meta_Entry) -> ()),
        copy: Some(
            meta_Entry_copy as unsafe extern "C" fn(*mut meta_Entry, *const meta_Entry) -> (),
        ),
        move_0: Some(
            meta_Entry_move as unsafe extern "C" fn(*mut meta_Entry, *mut meta_Entry) -> (),
        ),
        dispose: Some(meta_Entry_dispose as unsafe extern "C" fn(*mut meta_Entry) -> ()),
        replace: Some(
            meta_Entry_replace as unsafe extern "C" fn(*mut meta_Entry, meta_Entry) -> (),
        ),
        copyReplace: Some(
            meta_Entry_copyReplace as unsafe extern "C" fn(*mut meta_Entry, meta_Entry) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn meta_Entry_replace(mut dst: *mut meta_Entry, src: meta_Entry) {
    meta_Entry_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<meta_Entry>() as usize,
    );
}
#[inline]
unsafe extern "C" fn meta_Entry_init(mut x: *mut meta_Entry) {
    initMetaEntry(x);
}
#[inline]
unsafe extern "C" fn meta_Entry_dispose(mut x: *mut meta_Entry) {
    disposeMetaEntry(x);
}
#[inline]
unsafe extern "C" fn meta_Entry_copy(mut dst: *mut meta_Entry, mut src: *const meta_Entry) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<meta_Entry>() as usize,
    );
}
#[inline]
unsafe extern "C" fn meta_Entry_move(mut dst: *mut meta_Entry, mut src: *mut meta_Entry) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<meta_Entry>() as usize,
    );
    meta_Entry_init(src);
}
#[inline]
unsafe extern "C" fn meta_Entry_copyReplace(mut dst: *mut meta_Entry, src: meta_Entry) {
    meta_Entry_dispose(dst);
    meta_Entry_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn meta_Entries_filterEnv(
    mut arr: *mut meta_Entries,
    mut fn_0: Option<unsafe extern "C" fn(*const meta_Entry, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut meta_Entry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if meta_iEntry.dispose.is_some() {
                meta_iEntry.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut meta_Entry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn meta_Entries_move(dst: *mut meta_Entries, src: *mut meta_Entries) {
    cvec_move(meta_Entries_as_cvec(dst), meta_Entries_as_cvec(src));
}
#[inline]
unsafe fn meta_Entries_as_cvec(arr: *mut meta_Entries) -> *mut CVecRaw<meta_Entry> {
    arr as *mut CVecRaw<meta_Entry>
}
#[inline]
unsafe extern "C" fn meta_Entries_init(arr: *mut meta_Entries) {
    cvec_init(meta_Entries_as_cvec(arr));
}
#[no_mangle]
pub static mut meta_iEntries: __caryll_vectorinterface_meta_Entries = {
    __caryll_vectorinterface_meta_Entries {
        init: Some(meta_Entries_init as unsafe extern "C" fn(*mut meta_Entries) -> ()),
        copy: Some(
            meta_Entries_copy as unsafe extern "C" fn(*mut meta_Entries, *const meta_Entries) -> (),
        ),
        move_0: Some(
            meta_Entries_move as unsafe extern "C" fn(*mut meta_Entries, *mut meta_Entries) -> (),
        ),
        dispose: Some(meta_Entries_dispose as unsafe extern "C" fn(*mut meta_Entries) -> ()),
        replace: Some(
            meta_Entries_replace as unsafe extern "C" fn(*mut meta_Entries, meta_Entries) -> (),
        ),
        copyReplace: Some(
            meta_Entries_copyReplace as unsafe extern "C" fn(*mut meta_Entries, meta_Entries) -> (),
        ),
        create: Some(meta_Entries_create),
        free: Some(meta_Entries_free as unsafe extern "C" fn(*mut meta_Entries) -> ()),
        initN: Some(meta_Entries_initN as unsafe extern "C" fn(*mut meta_Entries, usize) -> ()),
        initCapN: Some(
            meta_Entries_initCapN as unsafe extern "C" fn(*mut meta_Entries, usize) -> (),
        ),
        createN: Some(meta_Entries_createN as unsafe extern "C" fn(usize) -> *mut meta_Entries),
        fill: Some(meta_Entries_fill as unsafe extern "C" fn(*mut meta_Entries, usize) -> ()),
        clear: Some(meta_Entries_dispose as unsafe extern "C" fn(*mut meta_Entries) -> ()),
        push: Some(meta_Entries_push as unsafe extern "C" fn(*mut meta_Entries, meta_Entry) -> ()),
        shrinkToFit: Some(
            meta_Entries_shrinkToFit as unsafe extern "C" fn(*mut meta_Entries) -> (),
        ),
        pop: Some(meta_Entries_pop as unsafe extern "C" fn(*mut meta_Entries) -> meta_Entry),
        disposeItem: Some(
            meta_Entries_disposeItem as unsafe extern "C" fn(*mut meta_Entries, usize) -> (),
        ),
        filterEnv: Some(
            meta_Entries_filterEnv
                as unsafe extern "C" fn(
                    *mut meta_Entries,
                    Option<
                        unsafe extern "C" fn(*const meta_Entry, *mut ::core::ffi::c_void) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            meta_Entries_sort
                as unsafe extern "C" fn(
                    *mut meta_Entries,
                    Option<
                        unsafe extern "C" fn(
                            *const meta_Entry,
                            *const meta_Entry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn meta_Entries_disposeItem(mut arr: *mut meta_Entries, mut n: usize) {
    if meta_iEntry.dispose.is_some() {
        meta_iEntry.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut meta_Entry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn meta_Entries_sort(
    mut arr: *mut meta_Entries,
    mut fn_0: Option<
        unsafe extern "C" fn(*const meta_Entry, *const meta_Entry) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<meta_Entry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*const meta_Entry, *const meta_Entry) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn meta_Entries_fill(mut arr: *mut meta_Entries, mut n: usize) {
    while (*arr).length < n {
        let mut x: meta_Entry = meta_Entry {
            tag: 0,
            data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        if meta_iEntry.init.is_some() {
            meta_iEntry.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<meta_Entry>() as usize,
            );
        }
        meta_Entries_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn meta_Entries_push(arr: *mut meta_Entries, elem: meta_Entry) {
    cvec_push(meta_Entries_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn meta_Entries_grow(arr: *mut meta_Entries) {
    cvec_grow(meta_Entries_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn meta_Entries_growTo(arr: *mut meta_Entries, target: usize) {
    cvec_grow_to(meta_Entries_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn meta_Entries_pop(arr: *mut meta_Entries) -> meta_Entry {
    cvec_pop(meta_Entries_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn meta_Entries_copyReplace(mut dst: *mut meta_Entries, src: meta_Entries) {
    meta_Entries_dispose(dst);
    meta_Entries_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn meta_Entries_copy(mut dst: *mut meta_Entries, mut src: *const meta_Entries) {
    meta_Entries_init(dst);
    meta_Entries_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if meta_iEntry.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            meta_iEntry.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut meta_Entry,
                (*src).items.offset(j as isize) as *mut meta_Entry as *const meta_Entry,
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
unsafe extern "C" fn meta_Entries_dispose(mut arr: *mut meta_Entries) {
    if arr.is_null() {
        return;
    }
    if meta_iEntry.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            meta_iEntry.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut meta_Entry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<meta_Entry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn meta_Entries_replace(mut dst: *mut meta_Entries, src: meta_Entries) {
    meta_Entries_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<meta_Entries>() as usize,
    );
}
#[inline]
unsafe extern "C" fn meta_Entries_initCapN(mut arr: *mut meta_Entries, mut n: usize) {
    meta_Entries_init(arr);
    meta_Entries_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn meta_Entries_growToN(arr: *mut meta_Entries, target: usize) {
    cvec_grow_to_n(meta_Entries_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn meta_Entries_initN(mut arr: *mut meta_Entries, mut n: usize) {
    meta_Entries_init(arr);
    meta_Entries_growToN(arr, n);
    meta_Entries_fill(arr, n);
}
#[inline]
unsafe extern "C" fn meta_Entries_free(mut x: *mut meta_Entries) {
    if x.is_null() {
        return;
    }
    meta_Entries_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn meta_Entries_createN(mut n: usize) -> *mut meta_Entries {
    let mut t: *mut meta_Entries =
        malloc(::core::mem::size_of::<meta_Entries>() as usize) as *mut meta_Entries;
    meta_Entries_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn meta_Entries_create() -> *mut meta_Entries {
    let mut x: *mut meta_Entries =
        malloc(::core::mem::size_of::<meta_Entries>() as usize) as *mut meta_Entries;
    meta_Entries_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn meta_Entries_shrinkToFit(mut arr: *mut meta_Entries) {
    meta_Entries_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn meta_Entries_resizeTo(arr: *mut meta_Entries, target: usize) {
    cvec_resize_to(meta_Entries_as_cvec(arr), target);
}
unsafe extern "C" fn initMetaTable(mut t: *mut table_meta) {
    (*t).version = 1 as u32;
    (*t).flags = 0 as u32;
    meta_iEntries.init.expect("non-null function pointer")(&raw mut (*t).entries);
}
unsafe extern "C" fn disposeMetaTable(mut t: *mut table_meta) {
    meta_iEntries.dispose.expect("non-null function pointer")(&raw mut (*t).entries);
}
#[inline]
unsafe extern "C" fn table_meta_free(mut x: *mut table_meta) {
    if x.is_null() {
        return;
    }
    table_meta_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_meta_move(mut dst: *mut table_meta, mut src: *mut table_meta) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_meta>() as usize,
    );
    table_meta_init(src);
}
#[inline]
unsafe extern "C" fn table_meta_copy(mut dst: *mut table_meta, mut src: *const table_meta) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_meta>() as usize,
    );
}
#[no_mangle]
pub static mut table_iMeta: __caryll_elementinterface_table_meta = {
    __caryll_elementinterface_table_meta {
        init: Some(table_meta_init as unsafe extern "C" fn(*mut table_meta) -> ()),
        copy: Some(
            table_meta_copy as unsafe extern "C" fn(*mut table_meta, *const table_meta) -> (),
        ),
        move_0: Some(
            table_meta_move as unsafe extern "C" fn(*mut table_meta, *mut table_meta) -> (),
        ),
        dispose: Some(table_meta_dispose as unsafe extern "C" fn(*mut table_meta) -> ()),
        replace: Some(
            table_meta_replace as unsafe extern "C" fn(*mut table_meta, table_meta) -> (),
        ),
        copyReplace: Some(
            table_meta_copyReplace as unsafe extern "C" fn(*mut table_meta, table_meta) -> (),
        ),
        create: Some(table_meta_create),
        free: Some(table_meta_free as unsafe extern "C" fn(*mut table_meta) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_meta_init(mut x: *mut table_meta) {
    initMetaTable(x);
}
#[inline]
unsafe extern "C" fn table_meta_dispose(mut x: *mut table_meta) {
    disposeMetaTable(x);
}
#[inline]
unsafe extern "C" fn table_meta_replace(mut dst: *mut table_meta, src: table_meta) {
    table_meta_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_meta>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_meta_create() -> *mut table_meta {
    let mut x: *mut table_meta =
        malloc(::core::mem::size_of::<table_meta>() as usize) as *mut table_meta;
    table_meta_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_meta_copyReplace(mut dst: *mut table_meta, src: table_meta) {
    table_meta_dispose(dst);
    table_meta_copy(dst, &raw const src);
}
