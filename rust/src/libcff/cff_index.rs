use libc::{free, malloc, memcpy, memset};
extern "C" {
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn bufwrite8(buf: *mut caryll_Buffer, byte: u8);
}


use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::buffer::{caryll_Buffer};
use crate::support::primitives::{arity_t};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum cff_IndexCountType {
    CFF_INDEX_16 = 0,
    CFF_INDEX_32 = 1,
}
pub use cff_IndexCountType::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Index {
    pub countType: cff_IndexCountType,
    pub count: arity_t,
    pub offSize: u8,
    pub offset: *mut u32,
    pub data: *mut u8,
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
    pub getLength: Option<unsafe extern "C" fn(*const cff_Index) -> u32>,
    pub parse: Option<unsafe extern "C" fn(*mut u8, u32, *mut cff_Index) -> ()>,
    pub fromCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            u32,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut caryll_Buffer>,
        ) -> *mut cff_Index,
    >,
    pub build: Option<unsafe extern "C" fn(*const cff_Index) -> *mut caryll_Buffer>,
}
#[inline]
unsafe extern "C" fn gu1(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 = *s.offset(p as isize) as u32;
    return b0;
}
#[inline]
unsafe extern "C" fn gu2(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
    let mut b1: u32 = *s
        .offset(p as isize)
        .offset(1 as ::core::ffi::c_int as isize) as u32;
    return b0 | b1;
}
#[inline]
unsafe extern "C" fn gu3(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32;
    let mut b1: u32 =
        ((*s.offset(p as isize)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as u32;
    let mut b2: u32 = *s
        .offset(p as isize)
        .offset(2 as ::core::ffi::c_int as isize) as u32;
    return b0 | b1 | b2;
}
#[inline]
unsafe extern "C" fn gu4(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as u32;
    let mut b1: u32 =
        ((*s.offset(p as isize)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as u32;
    let mut b2: u32 =
        ((*s.offset(p as isize)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as u32;
    let mut b3: u32 = *s
        .offset(p as isize)
        .offset(3 as ::core::ffi::c_int as isize) as u32;
    return b0 | b1 | b2 | b3;
}
#[inline]
unsafe extern "C" fn disposeCffIndex(mut in_0: *mut cff_Index) {
    if !(*in_0).offset.is_null() {
        free((*in_0).offset as *mut ::core::ffi::c_void);
        (*in_0).offset = ::core::ptr::null_mut::<u32>();
    }
    if !(*in_0).data.is_null() {
        free((*in_0).data as *mut ::core::ffi::c_void);
        (*in_0).data = ::core::ptr::null_mut::<u8>();
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
        ::core::mem::size_of::<cff_Index>() as usize,
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
        malloc(::core::mem::size_of::<cff_Index>() as usize) as *mut cff_Index;
    cff_Index_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn cff_Index_init(mut x: *mut cff_Index) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cff_Index>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cff_Index_replace(mut dst: *mut cff_Index, src: cff_Index) {
    cff_Index_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_Index>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cff_Index_move(mut dst: *mut cff_Index, mut src: *mut cff_Index) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_Index>() as usize,
    );
    cff_Index_init(src);
}
unsafe extern "C" fn getIndexLength(mut i: *const cff_Index) -> u32 {
    if (*i).count != 0 as arity_t {
        return (3 as u32)
            .wrapping_add((*(*i).offset.offset((*i).count as isize)).wrapping_sub(1 as u32))
            .wrapping_add(
                ((*i).count as u32)
                    .wrapping_add(1 as u32)
                    .wrapping_mul((*i).offSize as u32),
            );
    } else {
        return 3 as u32;
    };
}
unsafe extern "C" fn emptyIndex(mut i: *mut cff_Index) {
    cff_iIndex.dispose.expect("non-null function pointer")(i);
    memset(
        i as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cff_Index>() as usize,
    );
}
unsafe extern "C" fn extractIndex(
    mut data: *mut u8,
    mut pos: u32,
    mut in_0: *mut cff_Index,
) {
    (*in_0).count = gu2(data, pos) as arity_t;
    (*in_0).offSize = gu1(data, pos.wrapping_add(2 as u32)) as u8;
    if (*in_0).count > 0 as arity_t {
        (*in_0).offset = __caryll_allocate_clean(
            (::core::mem::size_of::<u32>() as usize)
                .wrapping_mul((*in_0).count.wrapping_add(1 as arity_t) as usize),
            27 as ::core::ffi::c_ulong,
        ) as *mut u32;
        let mut i: arity_t = 0 as arity_t;
        while i <= (*in_0).count {
            match (*in_0).offSize as ::core::ffi::c_int {
                1 => {
                    *(*in_0).offset.offset(i as isize) = gu1(
                        data,
                        pos.wrapping_add(3 as u32).wrapping_add(
                            (i as u32).wrapping_mul((*in_0).offSize as u32),
                        ),
                    );
                }
                2 => {
                    *(*in_0).offset.offset(i as isize) = gu2(
                        data,
                        pos.wrapping_add(3 as u32).wrapping_add(
                            (i as u32).wrapping_mul((*in_0).offSize as u32),
                        ),
                    );
                }
                3 => {
                    *(*in_0).offset.offset(i as isize) = gu3(
                        data,
                        pos.wrapping_add(3 as u32).wrapping_add(
                            (i as u32).wrapping_mul((*in_0).offSize as u32),
                        ),
                    );
                }
                4 => {
                    *(*in_0).offset.offset(i as isize) = gu4(
                        data,
                        pos.wrapping_add(3 as u32).wrapping_add(
                            (i as u32).wrapping_mul((*in_0).offSize as u32),
                        ),
                    );
                }
                _ => {}
            }
            i = i.wrapping_add(1);
        }
        (*in_0).data = __caryll_allocate_clean(
            (::core::mem::size_of::<u8>() as usize).wrapping_mul(
                (*(*in_0).offset.offset((*in_0).count as isize)).wrapping_sub(1 as u32)
                    as usize,
            ),
            46 as ::core::ffi::c_ulong,
        ) as *mut u8;
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
            (*(*in_0).offset.offset((*in_0).count as isize)).wrapping_sub(1 as u32) as usize,
        );
    } else {
        (*in_0).offset = ::core::ptr::null_mut::<u32>();
        (*in_0).data = ::core::ptr::null_mut::<u8>();
    };
}
unsafe extern "C" fn newIndexByCallback(
    mut context: *mut ::core::ffi::c_void,
    mut length: u32,
    mut fn_0: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut caryll_Buffer,
    >,
) -> *mut cff_Index {
    let mut idx: *mut cff_Index = (
        cff_iIndex.create.expect("non-null function pointer"))();
    (*idx).count = length as arity_t;
    (*idx).offset = __caryll_allocate_clean(
        (::core::mem::size_of::<u32>() as usize)
            .wrapping_mul((*idx).count.wrapping_add(1 as arity_t) as usize),
        57 as ::core::ffi::c_ulong,
    ) as *mut u32;
    *(*idx).offset.offset(0 as ::core::ffi::c_int as isize) = 1 as u32;
    (*idx).data = ::core::ptr::null_mut::<u8>();
    let mut used: usize = 0 as usize;
    let mut blank: usize = 0 as usize;
    let mut i: arity_t = 0 as arity_t;
    while i < length {
        let mut blob: *mut caryll_Buffer =
            fn_0.expect("non-null function pointer")(context, i as u32);
        if blank < (*blob).size {
            used = used.wrapping_add((*blob).size);
            blank = used >> 1 as ::core::ffi::c_int & 0xffffff as ::core::ffi::c_int as usize;
            (*idx).data = __caryll_reallocate(
                (*idx).data as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<u8>() as usize)
                    .wrapping_mul(used.wrapping_add(blank)),
                68 as ::core::ffi::c_ulong,
            ) as *mut u8;
        } else {
            used = used.wrapping_add((*blob).size);
            blank = blank.wrapping_sub((*blob).size);
        }
        *(*idx).offset.offset(i.wrapping_add(1 as arity_t) as isize) = (*blob)
            .size
            .wrapping_add(*(*idx).offset.offset(i as isize) as usize)
            as u32;
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
    (*idx).offSize = 4 as u8;
    return idx;
}
unsafe extern "C" fn buildIndex(mut index: *const cff_Index) -> *mut caryll_Buffer {
    let mut blob: *mut caryll_Buffer = bufnew();
    if (*index).count == 0 {
        bufwrite8(blob, 0 as u8);
        bufwrite8(blob, 0 as u8);
        bufwrite8(blob, 0 as u8);
        return blob;
    }
    let mut lastOffset: u32 = *(*index).offset.offset((*index).count as isize);
    let mut offSize: u8 = 4 as u8;
    if lastOffset < 0x100 as u32 {
        offSize = 1 as u8;
    } else if lastOffset < 0x10000 as u32 {
        offSize = 2 as u8;
    } else if lastOffset < 0x1000000 as u32 {
        offSize = 3 as u8;
    } else {
        offSize = 4 as u8;
    }
    if (*index).count != 0 as arity_t {
        (*blob).size = (3 as u32)
            .wrapping_add(
                (*(*index).offset.offset((*index).count as isize)).wrapping_sub(1 as u32),
            )
            .wrapping_add(
                ((*index).count as u32)
                    .wrapping_add(1 as u32)
                    .wrapping_mul(offSize as u32),
            ) as usize;
    } else {
        (*blob).size = 3 as usize;
    }
    (*blob).data = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob).size),
        107 as ::core::ffi::c_ulong,
    ) as *mut u8;
    *(*blob).data.offset(0 as ::core::ffi::c_int as isize) =
        (*index).count.wrapping_div(256 as arity_t) as u8;
    *(*blob).data.offset(1 as ::core::ffi::c_int as isize) =
        (*index).count.wrapping_rem(256 as arity_t) as u8;
    *(*blob).data.offset(2 as ::core::ffi::c_int as isize) = offSize;
    if (*index).count > 0 as arity_t {
        let mut i: arity_t = 0 as arity_t;
        while i <= (*index).count {
            match offSize as ::core::ffi::c_int {
                1 => {
                    *(*blob).data.offset((3 as arity_t).wrapping_add(i) as isize) =
                        *(*index).offset.offset(i as isize) as u8;
                }
                2 => {
                    *(*blob).data.offset(
                        (3 as arity_t).wrapping_add(i.wrapping_mul(2 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize)).wrapping_div(256 as u32)
                        as u8;
                    *(*blob).data.offset(
                        (4 as arity_t).wrapping_add(i.wrapping_mul(2 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize)).wrapping_rem(256 as u32)
                        as u8;
                }
                3 => {
                    *(*blob).data.offset(
                        (3 as arity_t).wrapping_add(i.wrapping_mul(3 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize)).wrapping_div(65536 as u32)
                        as u8;
                    *(*blob).data.offset(
                        (4 as arity_t).wrapping_add(i.wrapping_mul(3 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_rem(65536 as u32)
                        .wrapping_div(256 as u32) as u8;
                    *(*blob).data.offset(
                        (5 as arity_t).wrapping_add(i.wrapping_mul(3 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_rem(65536 as u32)
                        .wrapping_rem(256 as u32) as u8;
                }
                4 => {
                    *(*blob).data.offset(
                        (3 as arity_t).wrapping_add(i.wrapping_mul(4 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_div(65536 as u32)
                        .wrapping_div(256 as u32) as u8;
                    *(*blob).data.offset(
                        (4 as arity_t).wrapping_add(i.wrapping_mul(4 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_div(65536 as u32)
                        .wrapping_rem(256 as u32) as u8;
                    *(*blob).data.offset(
                        (5 as arity_t).wrapping_add(i.wrapping_mul(4 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_rem(65536 as u32)
                        .wrapping_div(256 as u32) as u8;
                    *(*blob).data.offset(
                        (6 as arity_t).wrapping_add(i.wrapping_mul(4 as arity_t)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_rem(65536 as u32)
                        .wrapping_rem(256 as u32) as u8;
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
                (*(*index).offset.offset((*index).count as isize)).wrapping_sub(1 as u32)
                    as usize,
            );
        }
    }
    (*blob).cursor = (*blob).size;
    return blob;
}
#[no_mangle]
pub static cff_iIndex: __caryll_elementinterface_cff_Index = {
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
        getLength: Some(getIndexLength as unsafe extern "C" fn(*const cff_Index) -> u32),
        parse: Some(
            extractIndex as unsafe extern "C" fn(*mut u8, u32, *mut cff_Index) -> (),
        ),
        fromCallback: Some(
            newIndexByCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    u32,
                    Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            u32,
                        ) -> *mut caryll_Buffer,
                    >,
                ) -> *mut cff_Index,
        ),
        build: Some(buildIndex as unsafe extern "C" fn(*const cff_Index) -> *mut caryll_Buffer),
    }
};
