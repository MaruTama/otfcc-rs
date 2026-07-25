extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn bufwrite8(buf: *mut caryll_Buffer, byte: uint8_t);
}

use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type arity_t = uint32_t;
pub type cff_IndexCountType = ::core::ffi::c_uint;
pub const CFF_INDEX_32: cff_IndexCountType = 1;
pub const CFF_INDEX_16: cff_IndexCountType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Index {
    pub countType: cff_IndexCountType,
    pub count: arity_t,
    pub offSize: uint8_t,
    pub offset: *mut uint32_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_cff_Index {
    pub init: Option<unsafe extern "C" fn(*mut cff_Index) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut cff_Index, *const cff_Index) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut cff_Index, *mut cff_Index) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut cff_Index) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut cff_Index, cff_Index) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut cff_Index, cff_Index) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut cff_Index>,
    pub free: Option<unsafe extern "C" fn(*mut cff_Index) -> ()>,
    pub empty: Option<unsafe extern "C" fn(*mut cff_Index) -> ()>,
    pub getLength: Option<unsafe extern "C" fn(*const cff_Index) -> uint32_t>,
    pub parse: Option<unsafe extern "C" fn(*mut uint8_t, uint32_t, *mut cff_Index) -> ()>,
    pub fromCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            uint32_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> *mut caryll_Buffer>,
        ) -> *mut cff_Index,
    >,
    pub build: Option<unsafe extern "C" fn(*const cff_Index) -> *mut caryll_Buffer>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn gu1(mut s: *mut uint8_t, mut p: uint32_t) -> uint32_t {
    let mut b0: uint32_t = *s.offset(p as isize) as uint32_t;
    return b0;
}
#[inline]
unsafe extern "C" fn gu2(mut s: *mut uint8_t, mut p: uint32_t) -> uint32_t {
    let mut b0: uint32_t =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint32_t;
    let mut b1: uint32_t = *s
        .offset(p as isize)
        .offset(1 as ::core::ffi::c_int as isize) as uint32_t;
    return b0 | b1;
}
#[inline]
unsafe extern "C" fn gu3(mut s: *mut uint8_t, mut p: uint32_t) -> uint32_t {
    let mut b0: uint32_t =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t;
    let mut b1: uint32_t =
        ((*s.offset(p as isize)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as uint32_t;
    let mut b2: uint32_t = *s
        .offset(p as isize)
        .offset(2 as ::core::ffi::c_int as isize) as uint32_t;
    return b0 | b1 | b2;
}
#[inline]
unsafe extern "C" fn gu4(mut s: *mut uint8_t, mut p: uint32_t) -> uint32_t {
    let mut b0: uint32_t =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as uint32_t;
    let mut b1: uint32_t =
        ((*s.offset(p as isize)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as uint32_t;
    let mut b2: uint32_t =
        ((*s.offset(p as isize)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as uint32_t;
    let mut b3: uint32_t = *s
        .offset(p as isize)
        .offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    return b0 | b1 | b2 | b3;
}
#[inline]
unsafe extern "C" fn disposeCffIndex(mut in_0: *mut cff_Index) {
    if !(*in_0).offset.is_null() {
        free((*in_0).offset as *mut ::core::ffi::c_void);
        (*in_0).offset = ::core::ptr::null_mut::<uint32_t>();
    }
    if !(*in_0).data.is_null() {
        free((*in_0).data as *mut ::core::ffi::c_void);
        (*in_0).data = ::core::ptr::null_mut::<uint8_t>();
    }
}
#[inline]
unsafe extern "C" fn cff_Index_copyReplace(mut dst: *mut cff_Index, src: cff_Index) {
    cff_Index_dispose(dst);
    cff_Index_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cff_Index_dispose(mut x: *mut cff_Index) {
    disposeCffIndex(x);
}
#[inline]
unsafe extern "C" fn cff_Index_copy(mut dst: *mut cff_Index, mut src: *const cff_Index) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_Index>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn cff_Index_free(mut x: *mut cff_Index) {
    if x.is_null() {
        return;
    }
    cff_Index_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn cff_Index_create() -> *mut cff_Index {
    let mut x: *mut cff_Index =
        malloc(::core::mem::size_of::<cff_Index>() as size_t) as *mut cff_Index;
    cff_Index_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn cff_Index_init(mut x: *mut cff_Index) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cff_Index>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn cff_Index_replace(mut dst: *mut cff_Index, src: cff_Index) {
    cff_Index_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_Index>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn cff_Index_move(mut dst: *mut cff_Index, mut src: *mut cff_Index) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_Index>() as size_t,
    );
    cff_Index_init(src);
}
unsafe extern "C" fn getIndexLength(mut i: *const cff_Index) -> uint32_t {
    if (*i).count != 0 as arity_t {
        return (3 as uint32_t)
            .wrapping_add((*(*i).offset.offset((*i).count as isize)).wrapping_sub(1 as uint32_t))
            .wrapping_add(
                ((*i).count as uint32_t)
                    .wrapping_add(1 as uint32_t)
                    .wrapping_mul((*i).offSize as uint32_t),
            );
    } else {
        return 3 as uint32_t;
    };
}
unsafe extern "C" fn emptyIndex(mut i: *mut cff_Index) {
    cff_iIndex.dispose.expect("non-null function pointer")(i);
    memset(
        i as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cff_Index>() as size_t,
    );
}
unsafe extern "C" fn extractIndex(
    mut data: *mut uint8_t,
    mut pos: uint32_t,
    mut in_0: *mut cff_Index,
) {
    (*in_0).count = gu2(data, pos) as arity_t;
    (*in_0).offSize = gu1(data, pos.wrapping_add(2 as uint32_t)) as uint8_t;
    if (*in_0).count > 0 as arity_t {
        (*in_0).offset = __caryll_allocate_clean(
            (::core::mem::size_of::<uint32_t>() as size_t)
                .wrapping_mul((*in_0).count.wrapping_add(1 as arity_t) as size_t),
            27 as ::core::ffi::c_ulong,
        ) as *mut uint32_t;
        let mut i: arity_t = 0 as arity_t;
        while i <= (*in_0).count {
            match (*in_0).offSize as ::core::ffi::c_int {
                1 => {
                    *(*in_0).offset.offset(i as isize) = gu1(
                        data,
                        pos.wrapping_add(3 as uint32_t).wrapping_add(
                            (i as uint32_t).wrapping_mul((*in_0).offSize as uint32_t),
                        ),
                    );
                }
                2 => {
                    *(*in_0).offset.offset(i as isize) = gu2(
                        data,
                        pos.wrapping_add(3 as uint32_t).wrapping_add(
                            (i as uint32_t).wrapping_mul((*in_0).offSize as uint32_t),
                        ),
                    );
                }
                3 => {
                    *(*in_0).offset.offset(i as isize) = gu3(
                        data,
                        pos.wrapping_add(3 as uint32_t).wrapping_add(
                            (i as uint32_t).wrapping_mul((*in_0).offSize as uint32_t),
                        ),
                    );
                }
                4 => {
                    *(*in_0).offset.offset(i as isize) = gu4(
                        data,
                        pos.wrapping_add(3 as uint32_t).wrapping_add(
                            (i as uint32_t).wrapping_mul((*in_0).offSize as uint32_t),
                        ),
                    );
                }
                _ => {}
            }
            i = i.wrapping_add(1);
        }
        (*in_0).data = __caryll_allocate_clean(
            (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(
                (*(*in_0).offset.offset((*in_0).count as isize)).wrapping_sub(1 as uint32_t)
                    as size_t,
            ),
            46 as ::core::ffi::c_ulong,
        ) as *mut uint8_t;
        memcpy(
            (*in_0).data as *mut ::core::ffi::c_void,
            data.offset(pos as isize)
                .offset(3 as ::core::ffi::c_int as isize)
                .offset(
                    (*in_0)
                        .count
                        .wrapping_add(1 as arity_t)
                        .wrapping_mul((*in_0).offSize as arity_t) as isize,
                ) as *const ::core::ffi::c_void,
            (*(*in_0).offset.offset((*in_0).count as isize)).wrapping_sub(1 as uint32_t) as size_t,
        );
    } else {
        (*in_0).offset = ::core::ptr::null_mut::<uint32_t>();
        (*in_0).data = ::core::ptr::null_mut::<uint8_t>();
    };
}
unsafe extern "C" fn newIndexByCallback(
    mut context: *mut ::core::ffi::c_void,
    mut length: uint32_t,
    mut fn_0: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> *mut caryll_Buffer,
    >,
) -> *mut cff_Index {
    let mut idx: *mut cff_Index = (
        cff_iIndex.create.expect("non-null function pointer"))();
    (*idx).count = length as arity_t;
    (*idx).offset = __caryll_allocate_clean(
        (::core::mem::size_of::<uint32_t>() as size_t)
            .wrapping_mul((*idx).count.wrapping_add(1 as arity_t) as size_t),
        57 as ::core::ffi::c_ulong,
    ) as *mut uint32_t;
    *(*idx).offset.offset(0 as ::core::ffi::c_int as isize) = 1 as uint32_t;
    (*idx).data = ::core::ptr::null_mut::<uint8_t>();
    let mut used: size_t = 0 as size_t;
    let mut blank: size_t = 0 as size_t;
    let mut i: arity_t = 0 as arity_t;
    while i < length {
        let mut blob: *mut caryll_Buffer =
            fn_0.expect("non-null function pointer")(context, i as uint32_t);
        if blank < (*blob).size {
            used = used.wrapping_add((*blob).size);
            blank = used >> 1 as ::core::ffi::c_int & 0xffffff as ::core::ffi::c_int as size_t;
            (*idx).data = __caryll_reallocate(
                (*idx).data as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<uint8_t>() as size_t)
                    .wrapping_mul(used.wrapping_add(blank)),
                68 as ::core::ffi::c_ulong,
            ) as *mut uint8_t;
        } else {
            used = used.wrapping_add((*blob).size);
            blank = blank.wrapping_sub((*blob).size);
        }
        *(*idx).offset.offset(i.wrapping_add(1 as arity_t) as isize) = (*blob)
            .size
            .wrapping_add(*(*idx).offset.offset(i as isize) as size_t)
            as uint32_t;
        memcpy(
            (*idx)
                .data
                .offset(*(*idx).offset.offset(i as isize) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize))
                as *mut ::core::ffi::c_void,
            (*blob).data as *const ::core::ffi::c_void,
            (*blob).size,
        );
        buffree(blob);
        i = i.wrapping_add(1);
    }
    (*idx).offSize = 4 as uint8_t;
    return idx;
}
unsafe extern "C" fn buildIndex(mut index: *const cff_Index) -> *mut caryll_Buffer {
    let mut blob: *mut caryll_Buffer = bufnew();
    if (*index).count == 0 {
        bufwrite8(blob, 0 as uint8_t);
        bufwrite8(blob, 0 as uint8_t);
        bufwrite8(blob, 0 as uint8_t);
        return blob;
    }
    let mut lastOffset: uint32_t = *(*index).offset.offset((*index).count as isize);
    let mut offSize: uint8_t = 4 as uint8_t;
    if lastOffset < 0x100 as uint32_t {
        offSize = 1 as uint8_t;
    } else if lastOffset < 0x10000 as uint32_t {
        offSize = 2 as uint8_t;
    } else if lastOffset < 0x1000000 as uint32_t {
        offSize = 3 as uint8_t;
    } else {
        offSize = 4 as uint8_t;
    }
    if (*index).count != 0 as arity_t {
        (*blob).size = (3 as uint32_t)
            .wrapping_add(
                (*(*index).offset.offset((*index).count as isize)).wrapping_sub(1 as uint32_t),
            )
            .wrapping_add(
                ((*index).count as uint32_t)
                    .wrapping_add(1 as uint32_t)
                    .wrapping_mul(offSize as uint32_t),
            ) as size_t;
    } else {
        (*blob).size = 3 as size_t;
    }
    (*blob).data = __caryll_allocate_clean(
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul((*blob).size),
        107 as ::core::ffi::c_ulong,
    ) as *mut uint8_t;
    *(*blob).data.offset(0 as ::core::ffi::c_int as isize) =
        (*index).count.wrapping_div(256 as arity_t) as uint8_t;
    *(*blob).data.offset(1 as ::core::ffi::c_int as isize) =
        (*index).count.wrapping_rem(256 as arity_t) as uint8_t;
    *(*blob).data.offset(2 as ::core::ffi::c_int as isize) = offSize;
    if (*index).count > 0 as arity_t {
        let mut i: arity_t = 0 as arity_t;
        while i <= (*index).count {
            match offSize as ::core::ffi::c_int {
                1 => {
                    *(*blob).data.offset((3 as arity_t).wrapping_add(i) as isize) =
                        *(*index).offset.offset(i as isize) as uint8_t;
                }
                2 => {
                    *(*blob).data.offset(
                        (3 as arity_t).wrapping_add(i.wrapping_mul(2 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize)).wrapping_div(256 as uint32_t)
                        as uint8_t;
                    *(*blob).data.offset(
                        (4 as arity_t).wrapping_add(i.wrapping_mul(2 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize)).wrapping_rem(256 as uint32_t)
                        as uint8_t;
                }
                3 => {
                    *(*blob).data.offset(
                        (3 as arity_t).wrapping_add(i.wrapping_mul(3 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize)).wrapping_div(65536 as uint32_t)
                        as uint8_t;
                    *(*blob).data.offset(
                        (4 as arity_t).wrapping_add(i.wrapping_mul(3 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_rem(65536 as uint32_t)
                        .wrapping_div(256 as uint32_t) as uint8_t;
                    *(*blob).data.offset(
                        (5 as arity_t).wrapping_add(i.wrapping_mul(3 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_rem(65536 as uint32_t)
                        .wrapping_rem(256 as uint32_t) as uint8_t;
                }
                4 => {
                    *(*blob).data.offset(
                        (3 as arity_t).wrapping_add(i.wrapping_mul(4 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_div(65536 as uint32_t)
                        .wrapping_div(256 as uint32_t) as uint8_t;
                    *(*blob).data.offset(
                        (4 as arity_t).wrapping_add(i.wrapping_mul(4 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_div(65536 as uint32_t)
                        .wrapping_rem(256 as uint32_t) as uint8_t;
                    *(*blob).data.offset(
                        (5 as arity_t).wrapping_add(i.wrapping_mul(4 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_rem(65536 as uint32_t)
                        .wrapping_div(256 as uint32_t) as uint8_t;
                    *(*blob).data.offset(
                        (6 as arity_t).wrapping_add(i.wrapping_mul(4 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_rem(65536 as uint32_t)
                        .wrapping_rem(256 as uint32_t) as uint8_t;
                }
                _ => {}
            }
            i = i.wrapping_add(1);
        }
        if !(*index).data.is_null() {
            memcpy(
                (*blob)
                    .data
                    .offset(3 as ::core::ffi::c_int as isize)
                    .offset(
                        (*index)
                            .count
                            .wrapping_add(1 as arity_t)
                            .wrapping_mul(offSize as arity_t) as isize,
                    ) as *mut ::core::ffi::c_void,
                (*index).data as *const ::core::ffi::c_void,
                (*(*index).offset.offset((*index).count as isize)).wrapping_sub(1 as uint32_t)
                    as size_t,
            );
        }
    }
    (*blob).cursor = (*blob).size;
    return blob;
}
#[no_mangle]
pub static mut cff_iIndex: __caryll_elementinterface_cff_Index = {
    __caryll_elementinterface_cff_Index {
        init: Some(cff_Index_init as unsafe extern "C" fn(*mut cff_Index) -> ()),
        copy: Some(cff_Index_copy as unsafe extern "C" fn(*mut cff_Index, *const cff_Index) -> ()),
        move_0: Some(cff_Index_move as unsafe extern "C" fn(*mut cff_Index, *mut cff_Index) -> ()),
        dispose: Some(cff_Index_dispose as unsafe extern "C" fn(*mut cff_Index) -> ()),
        replace: Some(cff_Index_replace as unsafe extern "C" fn(*mut cff_Index, cff_Index) -> ()),
        copyReplace: Some(
            cff_Index_copyReplace as unsafe extern "C" fn(*mut cff_Index, cff_Index) -> (),
        ),
        create: Some(cff_Index_create),
        free: Some(cff_Index_free as unsafe extern "C" fn(*mut cff_Index) -> ()),
        empty: Some(emptyIndex as unsafe extern "C" fn(*mut cff_Index) -> ()),
        getLength: Some(getIndexLength as unsafe extern "C" fn(*const cff_Index) -> uint32_t),
        parse: Some(
            extractIndex as unsafe extern "C" fn(*mut uint8_t, uint32_t, *mut cff_Index) -> (),
        ),
        fromCallback: Some(
            newIndexByCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    uint32_t,
                    Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            uint32_t,
                        ) -> *mut caryll_Buffer,
                    >,
                ) -> *mut cff_Index,
        ),
        build: Some(buildIndex as unsafe extern "C" fn(*const cff_Index) -> *mut caryll_Buffer),
    }
};
