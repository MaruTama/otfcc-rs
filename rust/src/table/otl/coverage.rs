#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, qsort};

use crate::support::json_funcs::{preserialize};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dispose, Handle, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};
use crate::support::buffer::{Buffer};
use crate::support::primitives::{GlyphId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::buffer::{buffree, buflen, bufnew, bufwrite16b, bufwrite_bufdel};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_string_new_from_bytes};
use crate::vendor::sds::{sdsnewlen};
/// A glyph coverage set: C by way of c2rust had this as a hand-rolled
/// `malloc`/`realloc` array (`num_glyphs`/`capacity`/`glyphs: *mut
/// GlyphHandle`); it was never anything but a growable array of
/// `GlyphHandle`, so `Vec<GlyphHandle>` *is* `Coverage` now, not a struct
/// wrapping one -- same "C-native vector shape becomes a bare `pub type`"
/// call as `ColrTable`/`TsiTable` earlier in this migration.
pub type Coverage = Vec<GlyphHandle>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ICoverage {
    pub dump: Option<unsafe extern "C" fn(*const Coverage) -> *mut JsonValue>,
    pub parse: Option<unsafe extern "C" fn(*const JsonValue) -> *mut Coverage>,
    pub build: Option<unsafe extern "C" fn(*const Coverage) -> *mut Buffer>,
    pub build_format:
        Option<unsafe extern "C" fn(*const Coverage, u16) -> *mut Buffer>,
}
pub(crate) unsafe extern "C" fn otl_coverage_create() -> *mut Coverage {
    // `.write()`, not a field assignment: this is placement-constructing a
    // fresh `Vec` into unwritten `malloc`'d memory (`Coverage` is a bare
    // `Vec<GlyphHandle>` now), so there is nothing to read or drop first --
    // same reasoning as `ColrTable`/`TsiTable`.
    let x: *mut Coverage = malloc(::core::mem::size_of::<Coverage>() as usize) as *mut Coverage;
    x.write(Vec::new());
    x
}
pub(crate) unsafe extern "C" fn otl_coverage_free(mut x: *mut Coverage) {
    if x.is_null() {
        return;
    }
    otl_coverage_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub(crate) unsafe extern "C" fn otl_coverage_dispose(x: *mut Coverage) {
    // Dropping the old `Vec` here (via assignment) runs each element's
    // `Handle::drop` in turn, freeing every glyph name -- the explicit
    // per-element `otfcc_handle_dispose` loop this replaced is now
    // redundant, the same finding as the `Handle` Drop/Clone PR.
    *x = Vec::new();
}
/// Adopt a `otl_coverage_create()`/vtable-`.parse()`-style raw `*mut
/// Coverage` into an owned `Coverage` (`Vec<GlyphHandle>`) value -- the
/// "unwrap_X_table" idiom used throughout Stage 6-4, for the many
/// `XxxSubtable`/`XxxEntry` fields that hold a coverage table by value now
/// instead of by raw pointer. `ptr::read` moves the `Vec` out, `free`
/// releases just the emptied malloc'd shell.
pub(crate) unsafe fn coverage_from_raw(raw: *mut Coverage) -> Coverage {
    let value = ::core::ptr::read(raw);
    free(raw as *mut ::core::ffi::c_void);
    value
}
// `Handle` (aliased `GlyphHandle`) now owns a `Vec<u8>` name, so passing it
// by value trips `improper_ctypes_definitions`; this is never called across
// a real FFI boundary (c2rust artifact, not `#[no_mangle]`).
#[allow(improper_ctypes_definitions)]
pub(crate) unsafe extern "C" fn push_to_coverage(coverage: *mut Coverage, h: GlyphHandle) {
    (*coverage).push(h);
}
pub(crate) unsafe extern "C" fn read_coverage(
    mut data: *const u8,
    mut table_length: u32,
    mut offset: u32,
) -> *mut Coverage {
    let mut coverage: *mut Coverage = otl_coverage_create();
    if table_length < offset.wrapping_add(4 as u32) {
        return coverage;
    }
    let mut format: u16 = read_16u(data.offset(offset as isize));
    match format as ::core::ffi::c_int {
        1 => {
            let mut glyph_count: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize),
            );
            if table_length
                < offset.wrapping_add(4 as u32).wrapping_add(
                    (glyph_count as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                )
            {
                return coverage;
            }
            // `HASH_SORT`-by-`covIndex` is a no-op here: `covIndex` was
            // assigned `j` (this loop's own position) at insert time, and
            // only a first occurrence of each `gid` is ever inserted, so
            // the sequence of inserted `covIndex` values is already
            // strictly increasing -- sorting by it reproduces insertion
            // order exactly. `IndexSet` (insertion-order-preserving,
            // dedups on `.insert()`) needs no explicit sort step at all.
            let mut h: indexmap::IndexSet<GlyphId> = indexmap::IndexSet::new();
            let mut j: u16 = 0 as u16;
            while (j as ::core::ffi::c_int) < glyph_count as ::core::ffi::c_int {
                let gid: GlyphId = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize),
                );
                h.insert(gid);
                j = j.wrapping_add(1);
            }
            for gid in h.into_iter() {
                push_to_coverage(coverage, handle_from_index(gid) as GlyphHandle);
            }
        }
        2 => {
            let mut range_count: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize),
            );
            if table_length
                < offset.wrapping_add(4 as u32).wrapping_add(
                    (range_count as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as u32,
                )
            {
                return coverage;
            }
            // Unlike format 1, `covIndex` here is `startCoverageIndex + k`
            // (`k` the absolute gid, per the original C -- see
            // rust/README.md) which is *not* generally monotonic with
            // insertion order once ranges overlap or run out of order, so
            // the `HASH_SORT`-by-`covIndex` step is not a no-op and must
            // be reproduced explicitly: dedup-by-gid (first occurrence
            // wins) via `IndexMap`, then a stable sort by the stored
            // `covIndex`, matching `HASH_SORT`'s documented mergesort
            // stability for ties.
            let mut h: indexmap::IndexMap<GlyphId, ::core::ffi::c_int> = indexmap::IndexMap::new();
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
                let start_coverage_index: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize),
                );
                let mut k: ::core::ffi::c_int = start as ::core::ffi::c_int;
                while k <= end as ::core::ffi::c_int {
                    let cov_index: ::core::ffi::c_int = start_coverage_index as ::core::ffi::c_int + k;
                    h.entry(k as GlyphId).or_insert(cov_index);
                    k += 1;
                }
                j_0 = j_0.wrapping_add(1);
            }
            let mut entries: Vec<(GlyphId, ::core::ffi::c_int)> = h.into_iter().collect();
            entries.sort_by_key(|&(_, cov_index)| cov_index);
            for (gid, _) in entries {
                push_to_coverage(coverage, handle_from_index(gid) as GlyphHandle);
            }
        }
        _ => {}
    }
    return coverage;
}
pub(crate) unsafe extern "C" fn dump_coverage(coverage: *const Coverage) -> *mut JsonValue {
    let mut a: *mut JsonValue = json_array_new((*coverage).len());
    for j in 0..(*coverage).len() {
        json_array_push(
            a,
            json_string_new_from_bytes(&(&(*coverage))[j].name),
        );
    }
    return preserialize(a);
}
pub(crate) unsafe extern "C" fn parse_coverage(mut cov: *const JsonValue) -> *mut Coverage {
    let mut c: *mut Coverage = otl_coverage_create();
    if cov.is_null()
        || (*cov).type_0 != JsonType::Array
    {
        return c;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < (*cov).u.array.length {
        if (**(*cov).u.array.values.offset(j as isize)).type_0 == JsonType::String
        {
            push_to_coverage(
                c,
                handle_from_name(sdsnewlen(
                    (**(*cov).u.array.values.offset(j as isize)).u.string.ptr
                        as *const ::core::ffi::c_void,
                    (**(*cov).u.array.values.offset(j as isize)).u.string.length as usize,
                )) as GlyphHandle,
            );
        }
        j = j.wrapping_add(1);
    }
    return c;
}
unsafe extern "C" fn by_gid(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return *(a as *mut GlyphId) as ::core::ffi::c_int
        - *(b as *mut GlyphId) as ::core::ffi::c_int;
}
pub(crate) unsafe extern "C" fn build_coverage_format(
    mut coverage: *const Coverage,
    mut format: u16,
) -> *mut Buffer {
    if (*coverage).is_empty() {
        let mut buf: *mut Buffer = bufnew();
        bufwrite16b(buf, 2 as u16);
        bufwrite16b(buf, 0 as u16);
        return buf;
    }
    let mut r: *mut GlyphId = ::core::ptr::null_mut::<GlyphId>();
    r = __caryll_allocate_clean(
        (::core::mem::size_of::<GlyphId>() as usize).wrapping_mul((*coverage).len()),
        144 as ::core::ffi::c_ulong,
    ) as *mut GlyphId;
    let mut jj: GlyphId = 0 as GlyphId;
    for j in 0..(*coverage).len() {
        *r.offset(jj as isize) = (&(*coverage))[j].index;
        jj = jj.wrapping_add(1);
    }
    qsort(
        r as *mut ::core::ffi::c_void,
        jj as usize,
        ::core::mem::size_of::<GlyphId>() as usize,
        Some(
            by_gid
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut format1: *mut Buffer = bufnew();
    bufwrite16b(format1, 1 as u16);
    bufwrite16b(format1, jj as u16);
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        bufwrite16b(format1, *r.offset(j_0 as isize) as u16);
        j_0 = j_0.wrapping_add(1);
    }
    if (jj as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<GlyphId>();
        return format1;
    }
    let mut format2: *mut Buffer = bufnew();
    bufwrite16b(format2, 2 as u16);
    let mut ranges: *mut Buffer = bufnew();
    let mut start_gid: GlyphId = *r.offset(0 as ::core::ffi::c_int as isize);
    let mut end_gid: GlyphId = start_gid;
    let mut last_gid: GlyphId = start_gid;
    let mut n_ranges: GlyphId = 0 as GlyphId;
    let mut j_1: GlyphId = 1 as GlyphId;
    while (j_1 as ::core::ffi::c_int) < jj as ::core::ffi::c_int {
        let mut current: GlyphId = *r.offset(j_1 as isize);
        if !(current as ::core::ffi::c_int <= last_gid as ::core::ffi::c_int) {
            if current as ::core::ffi::c_int
                == end_gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int
            {
                end_gid = current;
            } else {
                bufwrite16b(ranges, start_gid as u16);
                bufwrite16b(ranges, end_gid as u16);
                bufwrite16b(
                    ranges,
                    (j_1 as ::core::ffi::c_int + start_gid as ::core::ffi::c_int
                        - end_gid as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as u16,
                );
                n_ranges = (n_ranges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
                end_gid = current;
                start_gid = end_gid;
            }
            last_gid = current;
        }
        j_1 = j_1.wrapping_add(1);
    }
    bufwrite16b(ranges, start_gid as u16);
    bufwrite16b(ranges, end_gid as u16);
    bufwrite16b(
        ranges,
        (jj as ::core::ffi::c_int + start_gid as ::core::ffi::c_int
            - end_gid as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as u16,
    );
    n_ranges = (n_ranges as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
    bufwrite16b(format2, n_ranges as u16);
    bufwrite_bufdel(format2, ranges);
    if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        buffree(format2);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<GlyphId>();
        return format1;
    } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        buffree(format1);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<GlyphId>();
        return format2;
    } else if buflen(format1) < buflen(format2) {
        buffree(format2);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<GlyphId>();
        return format1;
    } else {
        buffree(format1);
        free(r as *mut ::core::ffi::c_void);
        r = ::core::ptr::null_mut::<GlyphId>();
        return format2;
    };
}
pub(crate) unsafe extern "C" fn build_coverage(mut coverage: *const Coverage) -> *mut Buffer {
    return build_coverage_format(coverage, 0 as u16);
}
unsafe extern "C" fn by_handle_gid(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut GlyphHandle)).index as ::core::ffi::c_int
        - (*(b as *mut GlyphHandle)).index as ::core::ffi::c_int;
}
pub(crate) unsafe extern "C" fn shrink_coverage(coverage: *mut Coverage, dosort: bool) {
    if coverage.is_null() {
        return;
    }
    // Two `truncate`s, not one `num_glyphs = k` at the end as the original
    // did: each `truncate` lets `Vec`'s own drop glue free every handle
    // past the new length, including ones this function's own compaction
    // loops never got around to calling `otfcc_handle_dispose` on directly
    // (a survivor that gets superseded by a *later* compaction write, but
    // never becomes a write target itself, is exactly that case) -- the
    // original leaked that name; `truncate` doesn't.
    let mut k: usize = 0;
    for j in 0..(*coverage).len() {
        if !(&(*coverage))[j].name.is_empty() {
            let elem = (&(*coverage))[j].clone();
            (&mut (*coverage))[k] = elem;
            k += 1;
        } else {
            otfcc_handle_dispose(&raw mut (&mut (*coverage))[j] as *mut Handle);
        }
    }
    (*coverage).truncate(k);
    if dosort {
        qsort(
            (*coverage).as_mut_ptr() as *mut ::core::ffi::c_void,
            (*coverage).len(),
            ::core::mem::size_of::<GlyphHandle>() as usize,
            Some(
                by_handle_gid
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        let mut skip: usize = 0;
        let mut rear: usize = 1;
        while rear < (*coverage).len() {
            if (&(*coverage))[rear].index == (&(*coverage))[rear - skip - 1].index {
                otfcc_handle_dispose(&raw mut (&mut (*coverage))[rear] as *mut Handle);
                skip += 1;
            } else {
                let elem = (&(*coverage))[rear].clone();
                (&mut (*coverage))[rear - skip] = elem;
            }
            rear += 1;
        }
        let new_len = (*coverage).len() - skip;
        (*coverage).truncate(new_len);
    }
}
pub static OTL_I_COVERAGE: ICoverage = {
    ICoverage {
        dump: Some(dump_coverage as unsafe extern "C" fn(*const Coverage) -> *mut JsonValue),
        parse: Some(parse_coverage as unsafe extern "C" fn(*const JsonValue) -> *mut Coverage),
        build: Some(
            build_coverage as unsafe extern "C" fn(*const Coverage) -> *mut Buffer,
        ),
        build_format: Some(
            build_coverage_format
                as unsafe extern "C" fn(*const Coverage, u16) -> *mut Buffer,
        ),
    }
};
