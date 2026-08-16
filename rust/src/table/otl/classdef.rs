#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::parsed_json::{ParsedValue, json_dbl_val, json_int_val, json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_type_of};
use crate::table::otl::coverage::{Coverage};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dispose, Handle, GlyphHandle};

use crate::support::binio::{read_16u};
use crate::support::buffer::{Buffer};
use crate::support::primitives::{GlyphClass, GlyphId};
use crate::vendor::json::{JsonType};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite_bufdel};
use crate::support::built_json::{BuiltValue, json_integer_new, json_object_new, json_object_push_bytes_key, preserialize};
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
    pub dump: Option<unsafe extern "C" fn(*const ClassDef) -> *mut BuiltValue>,
    pub parse: Option<unsafe extern "C" fn(*const ParsedValue) -> *mut ClassDef>,
    pub build: Option<unsafe extern "C" fn(*const ClassDef) -> *mut Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut ClassDef) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClassDefSortRecord {
    pub gid: GlyphId,
    pub cid: GlyphClass,
}
pub(crate) unsafe extern "C" fn otl_class_def_free(mut x: *mut ClassDef) {
    if x.is_null() {
        return;
    }
    // Dropping the reclaimed `Box` drops `glyphs`/`classes` first (running
    // each glyph's `Handle::drop`, freeing every name -- the explicit
    // per-element `otfcc_handle_dispose` loop this used to need is
    // redundant, same finding as `Coverage`'s dispose), then deallocates
    // the shell.
    drop(Box::from_raw(x));
}
pub(crate) unsafe fn otl_class_def_create() -> *mut ClassDef {
    // A real Rust allocation now, not a `malloc`'d shell: `Box::into_raw`
    // gives back a pointer with the same shape (`*mut ClassDef`) every
    // caller already expects, but it must from here on only ever be
    // reclaimed with `Box::from_raw` (`otl_class_def_free`/
    // `classdef_from_raw` below, and `tsi5.rs`'s `unwrap_class_def`),
    // never a bare `free` -- mixing the two is exactly the hazard this
    // conversion removes.
    Box::into_raw(Box::new(ClassDef {
        maxclass: 0,
        glyphs: Vec::new(),
        classes: Vec::new(),
    }))
}
/// Adopt a `otl_class_def_create()`/`read_class_def()`/vtable-`.parse()`-style
/// raw `*mut ClassDef` into an owned `Option<Box<ClassDef>>` -- the same
/// "unwrap_X_table" idiom as `coverage_from_raw`, but `Option`-wrapped since
/// (unlike `Coverage`) `ClassDef`-producing calls can legitimately return
/// null (`parse_class_def` on a non-object JSON value). `Box::from_raw`
/// reclaims the exact allocation `otl_class_def_create` made -- no extra
/// copy into a fresh `Box` needed now that the original allocation already
/// is one.
pub(crate) unsafe fn classdef_from_raw(raw: *mut ClassDef) -> Option<Box<ClassDef>> {
    if raw.is_null() {
        return None;
    }
    Some(Box::from_raw(raw))
}
// `Handle` (aliased `GlyphHandle`) now owns a `Vec<u8>` name, so passing it
// by value trips `improper_ctypes_definitions`; this is never called across
// a real FFI boundary (c2rust artifact, not `#[no_mangle]`).
#[allow(improper_ctypes_definitions)]
pub(crate) unsafe fn push_class_def(cd: *mut ClassDef, h: GlyphHandle, cls: GlyphClass) {
    (*cd).glyphs.push(h);
    (*cd).classes.push(cls);
    if cls as ::core::ffi::c_int > (*cd).maxclass as ::core::ffi::c_int {
        (*cd).maxclass = cls;
    }
}
pub(crate) unsafe fn read_class_def(
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
pub(crate) unsafe fn expand_class_def(
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
pub(crate) unsafe extern "C" fn dump_class_def(cd: *const ClassDef) -> *mut BuiltValue {
    let mut a: *mut BuiltValue = json_object_new((*cd).glyphs.len());
    for j in 0..(*cd).glyphs.len() {
        json_object_push_bytes_key(
            a,
            &(&(*cd).glyphs)[j].name,
            json_integer_new((&(*cd).classes)[j] as i64),
        );
    }
    return preserialize(a);
}
pub(crate) unsafe extern "C" fn parse_class_def(mut _cd: *const ParsedValue) -> *mut ClassDef {
    if _cd.is_null()
        || json_type_of(_cd) != JsonType::Object
    {
        return ::core::ptr::null_mut::<ClassDef>();
    }
    let mut cd: *mut ClassDef = otl_class_def_create();
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < json_obj_len(_cd) {
        let mut h: GlyphHandle =
            handle_from_name(Some(json_obj_key_bytes_at(_cd, j as u32))) as GlyphHandle;
        let mut _cid: *const ParsedValue =
            json_obj_val_at(_cd, j as u32);
        let mut cls: GlyphClass = 0 as GlyphClass;
        if json_type_of(_cid) == JsonType::Integer
        {
            cls = json_int_val(_cid) as GlyphClass;
        } else if json_type_of(_cid) == JsonType::Double
        {
            cls = json_dbl_val(_cid) as GlyphClass;
        }
        push_class_def(cd, h as GlyphHandle, cls);
        j = j.wrapping_add(1);
    }
    return cd;
}
pub(crate) unsafe extern "C" fn build_class_def(mut cd: *const ClassDef) -> *mut Buffer {
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 2 as u16);
    if (*cd).glyphs.is_empty() {
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    // A local `Vec` scratch buffer, not a `__caryll_allocate_clean`/`qsort`/
    // `free` trio -- same simplification as `Coverage`'s `build_coverage_
    // format`, `sort_by_key` (stable) reproducing `by_gid`'s ordering.
    let mut r: Vec<ClassDefSortRecord> = Vec::new();
    for j in 0..(*cd).glyphs.len() {
        if (&(*cd).classes)[j] != 0 {
            r.push(ClassDefSortRecord {
                gid: (&(*cd).glyphs)[j].index,
                cid: (&(*cd).classes)[j],
            });
        }
    }
    let jj: GlyphId = r.len() as GlyphId;
    if jj == 0 {
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    r.sort_by_key(|rec| rec.gid);
    let mut start_gid: GlyphId = r[0].gid;
    let mut end_gid: GlyphId = start_gid;
    let mut last_class: GlyphClass = r[0].cid;
    let mut n_ranges: GlyphId = 0 as GlyphId;
    let mut last_gid: GlyphId = start_gid;
    let mut ranges: *mut Buffer = bufnew();
    let mut j_0: GlyphId = 1 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        let mut current: GlyphId = r[j_0 as usize].gid;
        if !(current as ::core::ffi::c_int <= last_gid as ::core::ffi::c_int) {
            if current as ::core::ffi::c_int
                == end_gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                && r[j_0 as usize].cid as ::core::ffi::c_int
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
                last_class = r[j_0 as usize].cid;
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
        dump: Some(dump_class_def as unsafe extern "C" fn(*const ClassDef) -> *mut BuiltValue),
        parse: Some(parse_class_def as unsafe extern "C" fn(*const ParsedValue) -> *mut ClassDef),
        build: Some(
            build_class_def as unsafe extern "C" fn(*const ClassDef) -> *mut Buffer,
        ),
        shrink: Some(shrink_class_def as unsafe extern "C" fn(*mut ClassDef) -> ()),
    }
};
