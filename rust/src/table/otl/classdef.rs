#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, qsort};

use crate::support::json_funcs::{preserialize};
use crate::table::otl::coverage::{Coverage};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dispose, Handle, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::support::buffer::{Buffer};
use crate::support::primitives::{GlyphClass, GlyphId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite_bufdel};
use crate::vendor::json_builder::{json_integer_new, json_object_new, json_object_push_bytes_key};
use crate::vendor::sds::{sdsnewlen};
/// `glyphs`/`classes` were a hand-rolled `malloc`/`realloc` pair of parallel
/// arrays (grown, pushed to, and truncated only ever together -- confirmed
/// by survey before this conversion), now `Vec<GlyphHandle>`/
/// `Vec<GlyphClass>`. `maxclass` is a running maximum scalar, not part of
/// either array, so `ClassDef` stays a real (if now `Vec`-holding) struct
/// rather than collapsing to a bare `pub type` the way `Coverage` did.
#[derive(Clone)]
#[repr(C)]
pub struct ClassDef {
    pub maxclass: GlyphClass,
    pub glyphs: Vec<GlyphHandle>,
    pub classes: Vec<GlyphClass>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IClassDef {
    pub free: Option<unsafe extern "C" fn(*mut ClassDef) -> ()>,
    pub dump: Option<unsafe extern "C" fn(*const ClassDef) -> *mut JsonValue>,
    pub parse: Option<unsafe extern "C" fn(*const JsonValue) -> *mut ClassDef>,
    pub build: Option<unsafe extern "C" fn(*const ClassDef) -> *mut Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut ClassDef) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClassDefSortRecord {
    pub gid: GlyphId,
    pub cid: GlyphClass,
}
unsafe extern "C" fn dispose_class_def(cd: *mut ClassDef) {
    // Dropping the old `Vec`s (via assignment) runs each glyph's
    // `Handle::drop`, freeing every name -- the explicit per-element
    // `otfcc_handle_dispose` loop this replaced is now redundant, same
    // finding as `Coverage`'s dispose.
    (*cd).glyphs = Vec::new();
    (*cd).classes = Vec::new();
}
pub(crate) unsafe extern "C" fn otl_class_def_free(mut x: *mut ClassDef) {
    if x.is_null() {
        return;
    }
    otl_class_def_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub(crate) unsafe extern "C" fn otl_class_def_dispose(x: *mut ClassDef) {
    dispose_class_def(x);
}
pub(crate) unsafe extern "C" fn otl_class_def_create() -> *mut ClassDef {
    // Whole-struct placement write, not a field assignment: nothing needs
    // to be read or dropped first regardless of what `malloc` left behind
    // (see `otl_coverage_create`/`ColrTable`).
    let x: *mut ClassDef = malloc(::core::mem::size_of::<ClassDef>() as usize) as *mut ClassDef;
    x.write(ClassDef {
        maxclass: 0,
        glyphs: Vec::new(),
        classes: Vec::new(),
    });
    x
}
/// Adopt a `otl_class_def_create()`/`read_class_def()`/vtable-`.parse()`-style
/// raw `*mut ClassDef` into an owned `Option<Box<ClassDef>>` -- the same
/// "unwrap_X_table" idiom as `coverage_from_raw`, but `Option`-wrapped since
/// (unlike `Coverage`) `ClassDef`-producing calls can legitimately return
/// null (`parse_class_def` on a non-object JSON value). `ptr::read` moves the
/// struct (and its owned `Vec`s) out, `free` releases just the emptied
/// malloc'd shell; the `Box` is a fresh Rust allocation holding the moved
/// value, not the original malloc'd memory.
pub(crate) unsafe fn classdef_from_raw(raw: *mut ClassDef) -> Option<Box<ClassDef>> {
    if raw.is_null() {
        return None;
    }
    let value = ::core::ptr::read(raw);
    free(raw as *mut ::core::ffi::c_void);
    Some(Box::new(value))
}
// `Handle` (aliased `GlyphHandle`) now owns a `Vec<u8>` name, so passing it
// by value trips `improper_ctypes_definitions`; this is never called across
// a real FFI boundary (c2rust artifact, not `#[no_mangle]`).
#[allow(improper_ctypes_definitions)]
pub(crate) unsafe extern "C" fn push_class_def(cd: *mut ClassDef, h: GlyphHandle, cls: GlyphClass) {
    (*cd).glyphs.push(h);
    (*cd).classes.push(cls);
    if cls as ::core::ffi::c_int > (*cd).maxclass as ::core::ffi::c_int {
        (*cd).maxclass = cls;
    }
}
pub(crate) unsafe extern "C" fn read_class_def(
    mut data: *const u8,
    mut table_length: u32,
    mut offset: u32,
) -> *mut ClassDef {
    let mut cd: *mut ClassDef = otl_class_def_create();
    if table_length < offset.wrapping_add(4 as u32) {
        return cd;
    }
    let mut format: u16 = read_16u(data.offset(offset as isize));
    if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && table_length >= offset.wrapping_add(6 as u32)
    {
        let mut start_gid: GlyphId = read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize),
        ) as GlyphId;
        let mut count: GlyphId = read_16u(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize),
        ) as GlyphId;
        if count as ::core::ffi::c_int != 0
            && table_length
                >= offset.wrapping_add(6 as u32).wrapping_add(
                    (count as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                )
        {
            let mut j: GlyphId = 0 as GlyphId;
            while (j as ::core::ffi::c_int) < count as ::core::ffi::c_int {
                push_class_def(
                    cd,
                    handle_from_index(
                        (start_gid as ::core::ffi::c_int + j as ::core::ffi::c_int) as GlyphId,
                    ) as GlyphHandle,
                    read_16u(
                        data.offset(offset as isize)
                            .offset(6 as ::core::ffi::c_int as isize)
                            .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize),
                    ) as GlyphClass,
                );
                j = j.wrapping_add(1);
            }
            return cd;
        }
    } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        let mut range_count: u16 = read_16u(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize),
        );
        if table_length
            < offset.wrapping_add(4 as u32).wrapping_add(
                (range_count as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as u32,
            )
        {
            return cd;
        }
        // `covIndex` is repurposed here to carry the class value, not a
        // coverage position -- `HASH_SORT`-by-it therefore orders the
        // final `ClassDef` by ascending *class value*, not by gid. That is
        // observable (it's the order `dump_class_def` walks), so it must
        // be reproduced exactly: dedup-by-gid (first occurrence wins) via
        // `IndexMap`, then a stable sort by the stored class value.
        let mut h: indexmap::IndexMap<GlyphId, GlyphClass> = indexmap::IndexMap::new();
        let mut j_0: u16 = 0 as u16;
        while (j_0 as ::core::ffi::c_int) < range_count as ::core::ffi::c_int {
            let start: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize),
            );
            let end: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize)
                    .offset(2 as ::core::ffi::c_int as isize),
            );
            let cls: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize)
                    .offset(4 as ::core::ffi::c_int as isize),
            );
            let mut k: ::core::ffi::c_int = start as ::core::ffi::c_int;
            while k <= end as ::core::ffi::c_int {
                h.entry(k as GlyphId).or_insert(cls as GlyphClass);
                k += 1;
            }
            j_0 = j_0.wrapping_add(1);
        }
        let mut entries: Vec<(GlyphId, GlyphClass)> = h.into_iter().collect();
        entries.sort_by_key(|&(_, cls)| cls);
        for (gid, cls) in entries {
            push_class_def(cd, handle_from_index(gid) as GlyphHandle, cls);
        }
        return cd;
    }
    return cd;
}
pub(crate) unsafe extern "C" fn expand_class_def(
    mut cov: *mut Coverage,
    mut ocd: *mut ClassDef,
) -> *mut ClassDef {
    let mut cd: *mut ClassDef = otl_class_def_create();
    // No `HASH_SORT` call anywhere in the original -- the final walk is
    // plain insertion order (uthash's natural `.next` list), which
    // `IndexMap` reproduces directly with no separate sort step. `ocd`'s
    // entries (deduped by gid, first occurrence wins) are inserted first,
    // in `ocd`'s own order; then every glyph in `cov` not already present
    // is added with class 0, in `cov`'s order -- exactly the two phases
    // below, sharing one map the way the original shares one hash table.
    let mut h: indexmap::IndexMap<GlyphId, GlyphClass> = indexmap::IndexMap::new();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*ocd).glyphs.len() {
        let gid: GlyphId = (&(*ocd).glyphs)[j as usize].index;
        let cid: GlyphClass = (&(*ocd).classes)[j as usize];
        h.entry(gid).or_insert(cid);
        j = j.wrapping_add(1);
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*cov).len() {
        let gid_0: GlyphId = (&(*cov))[j_0 as usize].index;
        h.entry(gid_0).or_insert(0 as GlyphClass);
        j_0 = j_0.wrapping_add(1);
    }
    for (gid, cid) in h.into_iter() {
        push_class_def(cd, handle_from_index(gid) as GlyphHandle, cid);
    }
    otl_class_def_free(ocd);
    return cd;
}
pub(crate) unsafe extern "C" fn dump_class_def(cd: *const ClassDef) -> *mut JsonValue {
    let mut a: *mut JsonValue = json_object_new((*cd).glyphs.len());
    for j in 0..(*cd).glyphs.len() {
        json_object_push_bytes_key(
            a,
            &(&(*cd).glyphs)[j].name,
            json_integer_new((&(*cd).classes)[j] as i64),
        );
    }
    return preserialize(a);
}
pub(crate) unsafe extern "C" fn parse_class_def(mut _cd: *const JsonValue) -> *mut ClassDef {
    if _cd.is_null()
        || (*_cd).type_0 != JsonType::Object
    {
        return ::core::ptr::null_mut::<ClassDef>();
    }
    let mut cd: *mut ClassDef = otl_class_def_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < (*_cd).u.object.length {
        let mut h: GlyphHandle =
            handle_from_name(sdsnewlen(
                (*(*_cd).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
                (*(*_cd).u.object.values.offset(j as isize)).name_length as usize,
            )) as GlyphHandle;
        let mut _cid: *mut JsonValue =
            (*(*_cd).u.object.values.offset(j as isize)).value as *mut JsonValue;
        let mut cls: GlyphClass = 0 as GlyphClass;
        if (*_cid).type_0 == JsonType::Integer
        {
            cls = (*_cid).u.integer as GlyphClass;
        } else if (*_cid).type_0 == JsonType::Double
        {
            cls = (*_cid).u.dbl as GlyphClass;
        }
        push_class_def(cd, h as GlyphHandle, cls);
        j = j.wrapping_add(1);
    }
    return cd;
}
unsafe extern "C" fn by_gid(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut ClassDefSortRecord)).gid as ::core::ffi::c_int
        - (*(b as *mut ClassDefSortRecord)).gid as ::core::ffi::c_int;
}
pub(crate) unsafe extern "C" fn build_class_def(mut cd: *const ClassDef) -> *mut Buffer {
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 2 as u16);
    if (*cd).glyphs.is_empty() {
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    let mut r: *mut ClassDefSortRecord = ::core::ptr::null_mut::<ClassDefSortRecord>();
    r = __caryll_allocate_clean(
        (::core::mem::size_of::<ClassDefSortRecord>() as usize).wrapping_mul((*cd).glyphs.len()),
        167 as ::core::ffi::c_ulong,
    ) as *mut ClassDefSortRecord;
    let mut jj: GlyphId = 0 as GlyphId;
    for j in 0..(*cd).glyphs.len() {
        if (&(*cd).classes)[j] != 0 {
            (*r.offset(jj as isize)).gid = (&(*cd).glyphs)[j].index;
            (*r.offset(jj as isize)).cid = (&(*cd).classes)[j];
            jj = jj.wrapping_add(1);
        }
    }
    if jj == 0 {
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<ClassDefSortRecord>();
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    qsort(
        r as *mut ::core::ffi::c_void,
        jj as usize,
        ::core::mem::size_of::<ClassDefSortRecord>() as usize,
        Some(
            by_gid
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut start_gid: GlyphId = (*r.offset(0 as ::core::ffi::c_int as isize)).gid;
    let mut end_gid: GlyphId = start_gid;
    let mut last_class: GlyphClass = (*r.offset(0 as ::core::ffi::c_int as isize)).cid;
    let mut n_ranges: GlyphId = 0 as GlyphId;
    let mut last_gid: GlyphId = start_gid;
    let mut ranges: *mut Buffer = bufnew();
    let mut j_0: GlyphId = 1 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        let mut current: GlyphId = (*r.offset(j_0 as isize)).gid;
        if !(current as ::core::ffi::c_int <= last_gid as ::core::ffi::c_int) {
            if current as ::core::ffi::c_int
                == end_gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                && (*r.offset(j_0 as isize)).cid as ::core::ffi::c_int
                    == last_class as ::core::ffi::c_int
            {
                end_gid = current;
            } else {
                bufwrite16b(ranges, start_gid as u16);
                bufwrite16b(ranges, end_gid as u16);
                bufwrite16b(ranges, last_class as u16);
                n_ranges = (n_ranges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
                end_gid = current;
                start_gid = end_gid;
                last_class = (*r.offset(j_0 as isize)).cid;
            }
            last_gid = current;
        }
        j_0 = j_0.wrapping_add(1);
    }
    bufwrite16b(ranges, start_gid as u16);
    bufwrite16b(ranges, end_gid as u16);
    bufwrite16b(ranges, last_class as u16);
    n_ranges = (n_ranges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
    bufwrite16b(buf, n_ranges as u16);
    bufwrite_bufdel(buf, ranges);
    free(r as *mut ::core::ffi::c_void);
    r = ::core::ptr::null_mut::<ClassDefSortRecord>();
    return buf;
}
pub(crate) unsafe extern "C" fn shrink_class_def(cd: *mut ClassDef) {
    // Single `truncate` at the end lets `Vec`'s drop glue free any handle
    // this loop's own `otfcc_handle_dispose` calls didn't reach -- same
    // reasoning as `shrink_coverage`.
    let mut k: usize = 0;
    for j in 0..(*cd).glyphs.len() {
        if !(&(*cd).glyphs)[j].name.is_empty() {
            let g = (&(*cd).glyphs)[j].clone();
            let c = (&(*cd).classes)[j];
            (&mut (*cd).glyphs)[k] = g;
            (&mut (*cd).classes)[k] = c;
            k += 1;
        } else {
            otfcc_handle_dispose(&raw mut (&mut (*cd).glyphs)[j] as *mut Handle);
        }
    }
    (*cd).glyphs.truncate(k);
    (*cd).classes.truncate(k);
}
pub static OTL_I_CLASS_DEF: IClassDef = {
    IClassDef {
        free: Some(otl_class_def_free as unsafe extern "C" fn(*mut ClassDef) -> ()),
        dump: Some(dump_class_def as unsafe extern "C" fn(*const ClassDef) -> *mut JsonValue),
        parse: Some(parse_class_def as unsafe extern "C" fn(*const JsonValue) -> *mut ClassDef),
        build: Some(
            build_class_def as unsafe extern "C" fn(*const ClassDef) -> *mut Buffer,
        ),
        shrink: Some(shrink_class_def as unsafe extern "C" fn(*mut ClassDef) -> ()),
    }
};
