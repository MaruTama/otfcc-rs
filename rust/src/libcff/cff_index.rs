#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{memcpy};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::buffer::{Buffer};
use crate::support::primitives::{Arity};
use crate::support::buffer::{buffree, bufnew, bufwrite8};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CffIndexCountType {
    U16 = 0,
    U32 = 1,
}
// `offset`/`data` were `__caryll_allocate_clean`'d/`free`'d raw arrays,
// sized from a font-byte-derived `count` in `extract_index` (the parse
// path) -- a genuine untrusted-input-driven allocation, not just style.
// `Vec` removes the manual free pair and the OOB-write risk a counting
// mistake there would have caused. Neither array is ever aliased outside
// this struct's own accessor functions, so no `Copy`/`Clone` derive
// survives (matches every other malloc-array-to-Vec conversion this crate
// has made).
#[repr(C)]
pub struct CffIndex {
    pub count_type: CffIndexCountType,
    pub count: Arity,
    pub off_size: u8,
    pub offset: Vec<u32>,
    pub data: Vec<u8>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffIndexElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CffIndex) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CffIndex, *const CffIndex) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CffIndex) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut CffIndex>,
    pub free: Option<unsafe extern "C" fn(*mut CffIndex) -> ()>,
    pub empty: Option<unsafe extern "C" fn(*mut CffIndex) -> ()>,
    pub get_length: Option<unsafe extern "C" fn(*const CffIndex) -> u32>,
    pub parse: Option<unsafe extern "C" fn(*mut u8, u32, *mut CffIndex) -> ()>,
    pub from_callback: Option<
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
unsafe extern "C" fn dispose_cff_index(mut in_0: *mut CffIndex) {
    (*in_0).offset = Vec::new();
    (*in_0).data = Vec::new();
}
#[inline]
unsafe extern "C" fn cff_index_dispose(mut x: *mut CffIndex) {
    dispose_cff_index(x);
}
#[inline]
unsafe extern "C" fn cff_index_copy(mut _dst: *mut CffIndex, mut _src: *const CffIndex) {
    // Confirmed dead: `CFF_I_INDEX.copy` has no call site anywhere in the
    // crate. The old `memcpy`-based body was only safe by accident, back
    // when both fields were raw pointers a bitwise copy could alias
    // harmlessly (nothing ever exercised the aliasing); now that `offset`/
    // `data` are `Vec`s, a `memcpy` would double-free. Kept as a loud
    // failure instead of silently reintroducing that risk if this ever
    // gets wired up.
    unreachable!("CffIndex::copy is dead code and unsound for owned Vec data")
}
#[inline]
unsafe extern "C" fn cff_index_free(mut x: *mut CffIndex) {
    if x.is_null() {
        return;
    }
    // `offset`/`data` are still freed here exactly as before -- only the
    // outer shell's own allocator changed, from a bare `malloc`/`free`
    // pair to `Box::into_raw`/`Box::from_raw`. Every `CFF_I_INDEX.create`/
    // `.free` call site pairs consistently (confirmed by grep: no generic
    // adapter reclaims a `*mut CffIndex` any other way, unlike
    // `GposPairSubtable`'s `subtable_from_raw`), so this is self-contained.
    cff_index_dispose(x);
    drop(Box::from_raw(x));
}
#[inline]
unsafe extern "C" fn cff_index_create() -> *mut CffIndex {
    // `Box::new` of an explicit all-zero literal, not `malloc` + `cff_index_
    // init`'s `memset`: same fields, same zero values, but a real Rust
    // allocation from here on -- see `cff_index_free`'s matching `Box::
    // from_raw`. `cff_index_init` itself stays (and keeps using `memset`):
    // `CFF_I_INDEX.init` also zero-initializes a stack-local `CffIndex` at
    // its one other call site (`table/cff.rs`), which was never a `malloc`/
    // `Box` allocation to begin with.
    Box::into_raw(Box::new(CffIndex {
        count_type: CffIndexCountType::U16,
        count: 0 as Arity,
        off_size: 0,
        offset: Vec::new(),
        data: Vec::new(),
    }))
}
#[inline]
unsafe extern "C" fn cff_index_init(mut x: *mut CffIndex) {
    // No all-zero bit pattern is a valid `CffIndex` any more (it owns two
    // `Vec` fields), so place a valid empty value directly instead of the
    // old `memset`.
    ::core::ptr::write(
        x,
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 0 as Arity,
            off_size: 0,
            offset: Vec::new(),
            data: Vec::new(),
        },
    );
}
unsafe extern "C" fn get_index_length(mut i: *const CffIndex) -> u32 {
    if (*i).count != 0 as Arity {
        let offset = &(*i).offset;
        return (3 as u32)
            .wrapping_add((offset[(*i).count as usize]).wrapping_sub(1 as u32))
            .wrapping_add(
                ((*i).count as u32)
                    .wrapping_add(1 as u32)
                    .wrapping_mul((*i).off_size as u32),
            );
    } else {
        return 3 as u32;
    };
}
unsafe extern "C" fn empty_index(mut i: *mut CffIndex) {
    CFF_I_INDEX.dispose.expect("non-null function pointer")(i);
    (*i).count_type = CffIndexCountType::U16;
    (*i).count = 0 as Arity;
    (*i).off_size = 0;
}
unsafe extern "C" fn extract_index(
    mut data: *mut u8,
    mut pos: u32,
    mut in_0: *mut CffIndex,
) {
    (*in_0).count = gu2(data, pos) as Arity;
    (*in_0).off_size = gu1(data, pos.wrapping_add(2 as u32)) as u8;
    if (*in_0).count > 0 as Arity {
        let mut offset: Vec<u32> =
            Vec::with_capacity((*in_0).count.wrapping_add(1 as Arity) as usize);
        let mut i: Arity = 0 as Arity;
        while i <= (*in_0).count {
            offset.push(match (*in_0).off_size as ::core::ffi::c_int {
                1 => gu1(
                    data,
                    pos.wrapping_add(3 as u32)
                        .wrapping_add((i as u32).wrapping_mul((*in_0).off_size as u32)),
                ),
                2 => gu2(
                    data,
                    pos.wrapping_add(3 as u32)
                        .wrapping_add((i as u32).wrapping_mul((*in_0).off_size as u32)),
                ),
                3 => gu3(
                    data,
                    pos.wrapping_add(3 as u32)
                        .wrapping_add((i as u32).wrapping_mul((*in_0).off_size as u32)),
                ),
                4 => gu4(
                    data,
                    pos.wrapping_add(3 as u32)
                        .wrapping_add((i as u32).wrapping_mul((*in_0).off_size as u32)),
                ),
                _ => 0 as u32,
            });
            i = i.wrapping_add(1);
        }
        let data_len: usize =
            (offset[(*in_0).count as usize]).wrapping_sub(1 as u32) as usize;
        (*in_0).offset = offset;
        let mut buf: Vec<u8> = vec![0 as u8; data_len];
        memcpy(
            buf.as_mut_ptr() as *mut ::core::ffi::c_void,
            data.offset(pos as isize)
                .offset(3 as ::core::ffi::c_int as isize)
                .offset(
                    (*in_0)
                        .count
                        .wrapping_add(1 as Arity)
                        .wrapping_mul((*in_0).off_size as Arity) as isize,
                ) as *const ::core::ffi::c_void,
            data_len,
        );
        (*in_0).data = buf;
    } else {
        (*in_0).offset = Vec::new();
        (*in_0).data = Vec::new();
    };
}
unsafe extern "C" fn new_index_by_callback(
    mut context: *mut ::core::ffi::c_void,
    mut length: u32,
    mut fn_0: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
    >,
) -> *mut CffIndex {
    let mut idx: *mut CffIndex = (
        CFF_I_INDEX.create.expect("non-null function pointer"))();
    (*idx).count = length as Arity;
    let mut offset: Vec<u32> = vec![0 as u32; (*idx).count.wrapping_add(1 as Arity) as usize];
    offset[0 as usize] = 1 as u32;
    let mut data: Vec<u8> = Vec::new();
    let mut used: usize = 0 as usize;
    let mut blank: usize = 0 as usize;
    let mut i: Arity = 0 as Arity;
    while i < length {
        let mut blob: *mut Buffer =
            fn_0.expect("non-null function pointer")(context, i as u32);
        if blank < (*blob).size {
            used = used.wrapping_add((*blob).size);
            blank = used >> 1 as ::core::ffi::c_int & 0xffffff as ::core::ffi::c_int as usize;
            data.resize(used.wrapping_add(blank), 0 as u8);
        } else {
            used = used.wrapping_add((*blob).size);
            blank = blank.wrapping_sub((*blob).size);
        }
        let write_at: usize = (offset[i as usize] as usize).wrapping_sub(1 as usize);
        let blob_size: usize = (*blob).size;
        offset[i.wrapping_add(1 as Arity) as usize] =
            blob_size.wrapping_add(offset[i as usize] as usize) as u32;
        data[write_at..write_at.wrapping_add(blob_size)]
            .copy_from_slice(::core::slice::from_raw_parts((*blob).data, blob_size));
        buffree(blob);
        i = i.wrapping_add(1);
    }
    data.truncate(used);
    (*idx).offset = offset;
    (*idx).data = data;
    (*idx).off_size = 4 as u8;
    return idx;
}
unsafe extern "C" fn build_index(mut index: *const CffIndex) -> *mut Buffer {
    let mut blob: *mut Buffer = bufnew();
    if (*index).count == 0 {
        bufwrite8(blob, 0 as u8);
        bufwrite8(blob, 0 as u8);
        bufwrite8(blob, 0 as u8);
        return blob;
    }
    let offset = &(*index).offset;
    let mut last_offset: u32 = offset[(*index).count as usize];
    let mut off_size: u8 = 4 as u8;
    if last_offset < 0x100 as u32 {
        off_size = 1 as u8;
    } else if last_offset < 0x10000 as u32 {
        off_size = 2 as u8;
    } else if last_offset < 0x1000000 as u32 {
        off_size = 3 as u8;
    } else {
        off_size = 4 as u8;
    }
    if (*index).count != 0 as Arity {
        (*blob).size = (3 as u32)
            .wrapping_add(
                (offset[(*index).count as usize]).wrapping_sub(1 as u32),
            )
            .wrapping_add(
                ((*index).count as u32)
                    .wrapping_add(1 as u32)
                    .wrapping_mul(off_size as u32),
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
    *(*blob).data.offset(2 as ::core::ffi::c_int as isize) = off_size;
    if (*index).count > 0 as Arity {
        let mut i: Arity = 0 as Arity;
        while i <= (*index).count {
            let offset_i: u32 = offset[i as usize];
            match off_size as ::core::ffi::c_int {
                1 => {
                    *(*blob).data.offset((3 as Arity).wrapping_add(i) as isize) =
                        offset_i as u8;
                }
                2 => {
                    *(*blob).data.offset(
                        (3 as Arity).wrapping_add(i.wrapping_mul(2 as Arity)) as isize,
                    ) = offset_i.wrapping_div(256 as u32)
                        as u8;
                    *(*blob).data.offset(
                        (4 as Arity).wrapping_add(i.wrapping_mul(2 as Arity)) as isize,
                    ) = offset_i.wrapping_rem(256 as u32)
                        as u8;
                }
                3 => {
                    *(*blob).data.offset(
                        (3 as Arity).wrapping_add(i.wrapping_mul(3 as Arity)) as isize,
                    ) = offset_i.wrapping_div(65536 as u32)
                        as u8;
                    *(*blob).data.offset(
                        (4 as Arity).wrapping_add(i.wrapping_mul(3 as Arity)) as isize,
                    ) = offset_i
                        .wrapping_rem(65536 as u32)
                        .wrapping_div(256 as u32) as u8;
                    *(*blob).data.offset(
                        (5 as Arity).wrapping_add(i.wrapping_mul(3 as Arity)) as isize,
                    ) = offset_i
                        .wrapping_rem(65536 as u32)
                        .wrapping_rem(256 as u32) as u8;
                }
                4 => {
                    *(*blob).data.offset(
                        (3 as Arity).wrapping_add(i.wrapping_mul(4 as Arity)) as isize,
                    ) = offset_i
                        .wrapping_div(65536 as u32)
                        .wrapping_div(256 as u32) as u8;
                    *(*blob).data.offset(
                        (4 as Arity).wrapping_add(i.wrapping_mul(4 as Arity)) as isize,
                    ) = offset_i
                        .wrapping_div(65536 as u32)
                        .wrapping_rem(256 as u32) as u8;
                    *(*blob).data.offset(
                        (5 as Arity).wrapping_add(i.wrapping_mul(4 as Arity)) as isize,
                    ) = offset_i
                        .wrapping_rem(65536 as u32)
                        .wrapping_div(256 as u32) as u8;
                    *(*blob).data.offset(
                        (6 as Arity).wrapping_add(i.wrapping_mul(4 as Arity)) as isize,
                    ) = offset_i
                        .wrapping_rem(65536 as u32)
                        .wrapping_rem(256 as u32) as u8;
                }
                _ => {}
            }
            i = i.wrapping_add(1);
        }
        if !(*index).data.is_empty() {
            memcpy(
                (*blob)
                    .data
                    .offset(3 as ::core::ffi::c_int as isize)
                    .offset(
                        (*index)
                            .count
                            .wrapping_add(1 as Arity)
                            .wrapping_mul(off_size as Arity) as isize,
                    ) as *mut ::core::ffi::c_void,
                (*index).data.as_ptr() as *const ::core::ffi::c_void,
                (offset[(*index).count as usize]).wrapping_sub(1 as u32)
                    as usize,
            );
        }
    }
    (*blob).cursor = (*blob).size;
    return blob;
}
pub static CFF_I_INDEX: CffIndexElementInterface = {
    CffIndexElementInterface {
        init: Some(cff_index_init as unsafe extern "C" fn(*mut CffIndex) -> ()),
        copy: Some(cff_index_copy as unsafe extern "C" fn(*mut CffIndex, *const CffIndex) -> ()),
        dispose: Some(cff_index_dispose as unsafe extern "C" fn(*mut CffIndex) -> ()),
        create: Some(cff_index_create),
        free: Some(cff_index_free as unsafe extern "C" fn(*mut CffIndex) -> ()),
        empty: Some(empty_index as unsafe extern "C" fn(*mut CffIndex) -> ()),
        get_length: Some(get_index_length as unsafe extern "C" fn(*const CffIndex) -> u32),
        parse: Some(
            extract_index as unsafe extern "C" fn(*mut u8, u32, *mut CffIndex) -> (),
        ),
        from_callback: Some(
            new_index_by_callback
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
        build: Some(build_index as unsafe extern "C" fn(*const CffIndex) -> *mut Buffer),
    }
};
