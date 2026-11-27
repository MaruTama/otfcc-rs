use crate::support::primitives::{Pos, ShapeId};

use crate::vf::vv::VV;
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct VqAxisSpan {
    pub start: Pos,
    pub peak: Pos,
    pub end: Pos,
}
/// A variation region: one span per axis. `spans` owns its storage, so the
/// struct is not `Copy`. `RegionKey` in `table/fvar.rs` hashes and compares
/// `dimensions` and `spans` as two separate byte views.
#[derive(Clone, Debug)]
pub struct VqRegion {
    pub dimensions: ShapeId,
    pub spans: Vec<VqAxisSpan>,
}
pub fn vq_create_region(dimensions: ShapeId) -> Box<VqRegion> {
    Box::new(VqRegion {
        dimensions,
        spans: Vec::with_capacity(dimensions as usize),
    })
}
// Orders regions by `dimensions`, then `spans` (`VqAxisSpan` derives
// `PartialOrd`, lexicographic over `start`/`peak`/`end`). Used as an
// ordering key (`vqs_compare`, for sorting) and an equality check
// (`vqs_compatible`, via `== 0`). `RegionKey` (`table/fvar.rs`) is the
// byte-for-byte identity used for `IndexMap` dedup.
pub fn vq_compare_region(a: &VqRegion, b: &VqRegion) -> i32 {
    if a.dimensions < b.dimensions {
        return -1;
    }
    if a.dimensions > b.dimensions {
        return 1;
    }
    match a.spans.partial_cmp(&b.spans) {
        Some(::core::cmp::Ordering::Less) => -1,
        Some(::core::cmp::Ordering::Greater) => 1,
        _ => 0,
    }
}
pub fn vq_axis_span_is_one(s: &VqAxisSpan) -> bool {
    let a: Pos = s.start;
    let p: Pos = s.peak;
    let z: Pos = s.end;
    a > p || p > z || a < 0.0 && z > 0.0 && p != 0.0 || p == 0.0
}
#[inline]
fn weight_axis_region(as_0: &VqAxisSpan, x: Pos) -> Pos {
    let a: Pos = as_0.start;
    let p: Pos = as_0.peak;
    let z: Pos = as_0.end;
    if a > p || p > z || a < 0.0 && z > 0.0 && p != 0.0 || p == 0.0 {
        Pos::from(1)
    } else if x < a || x > z {
        0.0
    } else if x == p {
        Pos::from(1)
    } else if x < p {
        (x - a) / (p - a)
    } else {
        (z - x) / (z - p)
    }
}
pub fn vq_region_get_weight(r: &VqRegion, v: &VV) -> Pos {
    let coords: &Vec<Pos> = v;
    let mut w: Pos = Pos::from(1);
    if !coords.is_empty() {
        for (span, &coord) in r
            .spans
            .iter()
            .zip(coords.iter())
            .take(r.dimensions as usize)
        {
            w *= weight_axis_region(span, coord);
        }
    }
    w
}
