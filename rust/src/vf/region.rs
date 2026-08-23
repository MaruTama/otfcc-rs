#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::primitives::{Pos, ShapeId};

use crate::vf::vv::VV;
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct VqAxisSpan {
    pub start: Pos,
    pub peak: Pos,
    pub end: Pos,
}
// Was a C "flexible array member" struct (`spans: [VqAxisSpan; 0]`,
// allocated as one `dimensions`-header-plus-trailing-spans block, indexed
// everywhere via manual pointer arithmetic past the struct's own address).
// Now a plain `Vec`-backed struct: `spans` owns its storage independently,
// so this can no longer be `Copy` (and doesn't need `#[repr(C)]` -- nothing
// reinterprets the whole struct as a contiguous byte blob any more; see
// `RegionKey` in `table/fvar.rs`, which now hashes/compares `dimensions`
// and `spans` as two separate byte views instead of one).
#[derive(Clone)]
pub struct VqRegion {
    pub dimensions: ShapeId,
    pub spans: Vec<VqAxisSpan>,
}
pub unsafe fn vq_create_region(dimensions: ShapeId) -> *mut VqRegion {
    Box::into_raw(Box::new(VqRegion {
        dimensions,
        spans: Vec::with_capacity(dimensions as usize),
    }))
}
pub unsafe fn vq_delete_region(region: *mut VqRegion) {
    drop(Box::from_raw(region));
}
pub unsafe fn vq_copy_region(region: *const VqRegion) -> *mut VqRegion {
    Box::into_raw(Box::new((*region).clone()))
}
// Was `strncmp` over the whole header+spans byte range (after a
// `dimensions` shortcut) -- a byte-identity check that made sense when
// `spans` was contiguous with the header in one allocation. Now compares
// `dimensions` then `spans` structurally (`VqAxisSpan` derives
// `PartialOrd`, lexicographic over `start`/`peak`/`end`, matching the
// field order the old byte comparison walked in practice). Only consumed
// as an ordering key (`vqs_compare`, for sorting) or an equality check
// (`vqs_compatible`, via `== 0`), never for anything relying on
// byte-for-byte identity -- that stricter semantics is preserved instead
// in `RegionKey` (`table/fvar.rs`), which still needs it for `IndexMap`
// dedup.
pub unsafe fn vq_compare_region(a: *const VqRegion, b: *const VqRegion) -> ::core::ffi::c_int {
    if (*a).dimensions < (*b).dimensions {
        return -1;
    }
    if (*a).dimensions > (*b).dimensions {
        return 1;
    }
    match (*a).spans.partial_cmp(&(*b).spans) {
        Some(::core::cmp::Ordering::Less) => -1,
        Some(::core::cmp::Ordering::Greater) => 1,
        _ => 0,
    }
}
pub unsafe fn vq_axis_span_is_one(mut s: *const VqAxisSpan) -> bool {
    let a: Pos = (*s).start;
    let p: Pos = (*s).peak;
    let z: Pos = (*s).end;
    return a > p
        || p > z
        || a < 0 as ::core::ffi::c_int as Pos
            && z > 0 as ::core::ffi::c_int as Pos
            && p != 0 as ::core::ffi::c_int as Pos
        || p == 0 as ::core::ffi::c_int as Pos;
}
#[inline]
unsafe fn weight_axis_region(mut as_0: *const VqAxisSpan, x: Pos) -> Pos {
    let a: Pos = (*as_0).start;
    let p: Pos = (*as_0).peak;
    let z: Pos = (*as_0).end;
    if a > p || p > z {
        return 1 as ::core::ffi::c_int as Pos;
    } else if a < 0 as ::core::ffi::c_int as Pos
        && z > 0 as ::core::ffi::c_int as Pos
        && p != 0 as ::core::ffi::c_int as Pos
    {
        return 1 as ::core::ffi::c_int as Pos;
    } else if p == 0 as ::core::ffi::c_int as Pos {
        return 1 as ::core::ffi::c_int as Pos;
    } else if x < a || x > z {
        return 0 as ::core::ffi::c_int as Pos;
    } else if x == p {
        return 1 as ::core::ffi::c_int as Pos;
    } else if x < p {
        return (x - a) / (p - a);
    } else {
        return (z - x) / (z - p);
    };
}
pub unsafe fn vq_region_get_weight(mut r: *const VqRegion, v: *const VV) -> Pos {
    let coords: &Vec<Pos> = &*v;
    let mut w: Pos = 1 as ::core::ffi::c_int as Pos;
    let mut j: usize = 0 as usize;
    while j < (*r).dimensions as usize && !coords.is_empty() {
        w *= weight_axis_region(&(&(*r).spans)[j] as *const VqAxisSpan, coords[j]);
        j = j.wrapping_add(1);
    }
    return w;
}
