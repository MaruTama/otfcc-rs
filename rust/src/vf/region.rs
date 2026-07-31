#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, memcpy, strncmp};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::primitives::{Pos, ShapeId};

use crate::vf::vv::{VV};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VqAxisSpan {
    pub start: Pos,
    pub peak: Pos,
    pub end: Pos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VqRegion {
    pub dimensions: ShapeId,
    pub spans: [VqAxisSpan; 0],
}
pub unsafe extern "C" fn vq_create_region(mut dimensions: ShapeId) -> *mut VqRegion {
    let mut r: *mut VqRegion = ::core::ptr::null_mut::<VqRegion>();
    r = __caryll_allocate_clean(
        (::core::mem::size_of::<VqRegion>() as usize).wrapping_add(
            (::core::mem::size_of::<VqAxisSpan>() as usize).wrapping_mul(dimensions as usize),
        ),
        6 as ::core::ffi::c_ulong,
    ) as *mut VqRegion;
    (*r).dimensions = dimensions;
    return r;
}
pub unsafe extern "C" fn vq_delete_region(mut region: *mut VqRegion) {
    free(region as *mut ::core::ffi::c_void);
    region = ::core::ptr::null_mut::<VqRegion>();
}
pub unsafe extern "C" fn vq_copy_region(mut region: *const VqRegion) -> *mut VqRegion {
    let mut dst: *mut VqRegion = vq_create_region((*region).dimensions);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        region as *const ::core::ffi::c_void,
        (::core::mem::size_of::<VqRegion>() as usize).wrapping_add(
            (::core::mem::size_of::<VqAxisSpan>() as usize)
                .wrapping_mul((*region).dimensions as usize),
        ),
    );
    return dst;
}
pub unsafe extern "C" fn vq_compare_region(
    mut a: *const VqRegion,
    mut b: *const VqRegion,
) -> ::core::ffi::c_int {
    if ((*a).dimensions as ::core::ffi::c_int) < (*b).dimensions as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if (*a).dimensions as ::core::ffi::c_int > (*b).dimensions as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    return strncmp(
        a as *const ::core::ffi::c_char,
        b as *const ::core::ffi::c_char,
        (::core::mem::size_of::<VqRegion>() as usize).wrapping_add(
            (::core::mem::size_of::<VqAxisSpan>() as usize)
                .wrapping_mul((*a).dimensions as usize),
        ),
    );
}
pub unsafe extern "C" fn vq_axis_span_is_one(mut s: *const VqAxisSpan) -> bool {
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
unsafe extern "C" fn weight_axis_region(mut as_0: *const VqAxisSpan, x: Pos) -> Pos {
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
pub unsafe extern "C" fn vq_region_get_weight(mut r: *const VqRegion, v: *const VV) -> Pos {
    let coords: &Vec<Pos> = &*v;
    let mut w: Pos = 1 as ::core::ffi::c_int as Pos;
    let mut j: usize = 0 as usize;
    while j < (*r).dimensions as usize && !coords.is_empty() {
        w *= weight_axis_region(
            (&raw const (*r).spans as *const VqAxisSpan).offset(j as isize) as *const VqAxisSpan,
            coords[j],
        );
        j = j.wrapping_add(1);
    }
    return w;
}
pub unsafe extern "C" fn vq_show_region(mut _r: *const VqRegion) {}
