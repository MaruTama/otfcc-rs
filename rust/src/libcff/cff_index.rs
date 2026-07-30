#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};


use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::buffer::{Buffer};
use crate::support::primitives::{Arity};
use crate::support::buffer::{buffree, bufnew, bufwrite8};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CffIndexCountType {
    U16 = 0,
    U32 = 1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffIndex {
    pub countType: CffIndexCountType,
    pub count: Arity,
    pub offSize: u8,
    pub offset: *mut u32,
    pub data: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffIndexElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CffIndex) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CffIndex, *const CffIndex) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut CffIndex, *mut CffIndex) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CffIndex) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut CffIndex, CffIndex) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut CffIndex, CffIndex) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut CffIndex>,
    pub free: Option<unsafe extern "C" fn(*mut CffIndex) -> ()>,
    pub empty: Option<unsafe extern "C" fn(*mut CffIndex) -> ()>,
    pub getLength: Option<unsafe extern "C" fn(*const CffIndex) -> u32>,
    pub parse: Option<unsafe extern "C" fn(*mut u8, u32, *mut CffIndex) -> ()>,
    pub fromCallback: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            u32,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer>,
        ) -> *mut CffIndex,
    >,
    pub build: Option<unsafe extern "C" fn(*const CffIndex) -> *mut Buffer>,
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
unsafe extern "C" fn disposeCffIndex(mut in_0: *mut CffIndex) {
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
unsafe extern "C" fn cff_Index_copyReplace(mut dst: *mut CffIndex, src: CffIndex) {
    cff_Index_dispose(dst);
    cff_Index_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cff_Index_dispose(mut x: *mut CffIndex) {
    disposeCffIndex(x);
}
#[inline]
unsafe extern "C" fn cff_Index_copy(mut dst: *mut CffIndex, mut src: *const CffIndex) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CffIndex>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cff_Index_free(mut x: *mut CffIndex) {
    if x.is_null() {
        return;
    }
    cff_Index_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn cff_Index_create() -> *mut CffIndex {
    let mut x: *mut CffIndex =
        malloc(::core::mem::size_of::<CffIndex>() as usize) as *mut CffIndex;
    cff_Index_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn cff_Index_init(mut x: *mut CffIndex) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<CffIndex>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cff_Index_replace(mut dst: *mut CffIndex, src: CffIndex) {
    cff_Index_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CffIndex>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cff_Index_move(mut dst: *mut CffIndex, mut src: *mut CffIndex) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<CffIndex>() as usize,
    );
    cff_Index_init(src);
}
unsafe extern "C" fn getIndexLength(mut i: *const CffIndex) -> u32 {
    if (*i).count != 0 as Arity {
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
unsafe extern "C" fn emptyIndex(mut i: *mut CffIndex) {
    CFF_I_INDEX.dispose.expect("non-null function pointer")(i);
    memset(
        i as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<CffIndex>() as usize,
    );
}
unsafe extern "C" fn extractIndex(
    mut data: *mut u8,
    mut pos: u32,
    mut in_0: *mut CffIndex,
) {
    (*in_0).count = gu2(data, pos) as Arity;
    (*in_0).offSize = gu1(data, pos.wrapping_add(2 as u32)) as u8;
    if (*in_0).count > 0 as Arity {
        (*in_0).offset = __caryll_allocate_clean(
            (::core::mem::size_of::<u32>() as usize)
                .wrapping_mul((*in_0).count.wrapping_add(1 as Arity) as usize),
            27 as ::core::ffi::c_ulong,
        ) as *mut u32;
        let mut i: Arity = 0 as Arity;
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
                        .wrapping_add(1 as Arity)
                        .wrapping_mul((*in_0).offSize as Arity) as isize,
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
        unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
    >,
) -> *mut CffIndex {
    let mut idx: *mut CffIndex = (
        CFF_I_INDEX.create.expect("non-null function pointer"))();
    (*idx).count = length as Arity;
    (*idx).offset = __caryll_allocate_clean(
        (::core::mem::size_of::<u32>() as usize)
            .wrapping_mul((*idx).count.wrapping_add(1 as Arity) as usize),
        57 as ::core::ffi::c_ulong,
    ) as *mut u32;
    *(*idx).offset.offset(0 as ::core::ffi::c_int as isize) = 1 as u32;
    (*idx).data = ::core::ptr::null_mut::<u8>();
    let mut used: usize = 0 as usize;
    let mut blank: usize = 0 as usize;
    let mut i: Arity = 0 as Arity;
    while i < length {
        let mut blob: *mut Buffer =
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
        *(*idx).offset.offset(i.wrapping_add(1 as Arity) as isize) = (*blob)
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
unsafe extern "C" fn buildIndex(mut index: *const CffIndex) -> *mut Buffer {
    let mut blob: *mut Buffer = bufnew();
    if (*index).count == 0 {
        bufwrite8(blob, 0 as u8);
        bufwrite8(blob, 0 as u8);
        bufwrite8(blob, 0 as u8);
        return blob;
    }
    let mut last_offset: u32 = *(*index).offset.offset((*index).count as isize);
    let mut offSize: u8 = 4 as u8;
    if last_offset < 0x100 as u32 {
        offSize = 1 as u8;
    } else if last_offset < 0x10000 as u32 {
        offSize = 2 as u8;
    } else if last_offset < 0x1000000 as u32 {
        offSize = 3 as u8;
    } else {
        offSize = 4 as u8;
    }
    if (*index).count != 0 as Arity {
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
        (*index).count.wrapping_div(256 as Arity) as u8;
    *(*blob).data.offset(1 as ::core::ffi::c_int as isize) =
        (*index).count.wrapping_rem(256 as Arity) as u8;
    *(*blob).data.offset(2 as ::core::ffi::c_int as isize) = offSize;
    if (*index).count > 0 as Arity {
        let mut i: Arity = 0 as Arity;
        while i <= (*index).count {
            match offSize as ::core::ffi::c_int {
                1 => {
                    *(*blob).data.offset((3 as Arity).wrapping_add(i) as isize) =
                        *(*index).offset.offset(i as isize) as u8;
                }
                2 => {
                    *(*blob).data.offset(
                        (3 as Arity).wrapping_add(i.wrapping_mul(2 as Arity)) as isize,
                    ) = (*(*index).offset.offset(i as isize)).wrapping_div(256 as u32)
                        as u8;
                    *(*blob).data.offset(
                        (4 as Arity).wrapping_add(i.wrapping_mul(2 as Arity)) as isize,
                    ) = (*(*index).offset.offset(i as isize)).wrapping_rem(256 as u32)
                        as u8;
                }
                3 => {
                    *(*blob).data.offset(
                        (3 as Arity).wrapping_add(i.wrapping_mul(3 as Arity)) as isize,
                    ) = (*(*index).offset.offset(i as isize)).wrapping_div(65536 as u32)
                        as u8;
                    *(*blob).data.offset(
                        (4 as Arity).wrapping_add(i.wrapping_mul(3 as Arity)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_rem(65536 as u32)
                        .wrapping_div(256 as u32) as u8;
                    *(*blob).data.offset(
                        (5 as Arity).wrapping_add(i.wrapping_mul(3 as Arity)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_rem(65536 as u32)
                        .wrapping_rem(256 as u32) as u8;
                }
                4 => {
                    *(*blob).data.offset(
                        (3 as Arity).wrapping_add(i.wrapping_mul(4 as Arity)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_div(65536 as u32)
                        .wrapping_div(256 as u32) as u8;
                    *(*blob).data.offset(
                        (4 as Arity).wrapping_add(i.wrapping_mul(4 as Arity)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_div(65536 as u32)
                        .wrapping_rem(256 as u32) as u8;
                    *(*blob).data.offset(
                        (5 as Arity).wrapping_add(i.wrapping_mul(4 as Arity)) as isize,
                    ) = (*(*index).offset.offset(i as isize))
                        .wrapping_rem(65536 as u32)
                        .wrapping_div(256 as u32) as u8;
                    *(*blob).data.offset(
                        (6 as Arity).wrapping_add(i.wrapping_mul(4 as Arity)) as isize,
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
                            .wrapping_add(1 as Arity)
                            .wrapping_mul(offSize as Arity) as isize,
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
pub static CFF_I_INDEX: CffIndexElementInterface = {
    CffIndexElementInterface {
        init: Some(cff_Index_init as unsafe extern "C" fn(*mut CffIndex) -> ()),
        copy: Some(cff_Index_copy as unsafe extern "C" fn(*mut CffIndex, *const CffIndex) -> ()),
        move_0: Some(cff_Index_move as unsafe extern "C" fn(*mut CffIndex, *mut CffIndex) -> ()),
        dispose: Some(cff_Index_dispose as unsafe extern "C" fn(*mut CffIndex) -> ()),
        replace: Some(cff_Index_replace as unsafe extern "C" fn(*mut CffIndex, CffIndex) -> ()),
        copyReplace: Some(
            cff_Index_copyReplace as unsafe extern "C" fn(*mut CffIndex, CffIndex) -> (),
        ),
        create: Some(cff_Index_create),
        free: Some(cff_Index_free as unsafe extern "C" fn(*mut CffIndex) -> ()),
        empty: Some(emptyIndex as unsafe extern "C" fn(*mut CffIndex) -> ()),
        getLength: Some(getIndexLength as unsafe extern "C" fn(*const CffIndex) -> u32),
        parse: Some(
            extractIndex as unsafe extern "C" fn(*mut u8, u32, *mut CffIndex) -> (),
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
                        ) -> *mut Buffer,
                    >,
                ) -> *mut CffIndex,
        ),
        build: Some(buildIndex as unsafe extern "C" fn(*const CffIndex) -> *mut Buffer),
    }
};
