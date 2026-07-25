use libc::{free, memcpy, strncmp};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::primitives::{pos_t, shapeid_t};

use crate::vf::vv::{VV};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_AxisSpan {
    pub start: pos_t,
    pub peak: pos_t,
    pub end: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_Region {
    pub dimensions: shapeid_t,
    pub spans: [vq_AxisSpan; 0],
}
#[no_mangle]
pub unsafe extern "C" fn vq_createRegion(mut dimensions: shapeid_t) -> *mut vq_Region {
    let mut r: *mut vq_Region = ::core::ptr::null_mut::<vq_Region>();
    r = __caryll_allocate_clean(
        (::core::mem::size_of::<vq_Region>() as usize).wrapping_add(
            (::core::mem::size_of::<vq_AxisSpan>() as usize).wrapping_mul(dimensions as usize),
        ),
        6 as ::core::ffi::c_ulong,
    ) as *mut vq_Region;
    (*r).dimensions = dimensions;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn vq_deleteRegion(mut region: *mut vq_Region) {
    free(region as *mut ::core::ffi::c_void);
    region = ::core::ptr::null_mut::<vq_Region>();
}
#[no_mangle]
pub unsafe extern "C" fn vq_copyRegion(mut region: *const vq_Region) -> *mut vq_Region {
    let mut dst: *mut vq_Region = vq_createRegion((*region).dimensions);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        region as *const ::core::ffi::c_void,
        (::core::mem::size_of::<vq_Region>() as usize).wrapping_add(
            (::core::mem::size_of::<vq_AxisSpan>() as usize)
                .wrapping_mul((*region).dimensions as usize),
        ),
    );
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn vq_compareRegion(
    mut a: *const vq_Region,
    mut b: *const vq_Region,
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
        (::core::mem::size_of::<vq_Region>() as usize).wrapping_add(
            (::core::mem::size_of::<vq_AxisSpan>() as usize)
                .wrapping_mul((*a).dimensions as usize),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn vq_AxisSpanIsOne(mut s: *const vq_AxisSpan) -> bool {
    let a: pos_t = (*s).start;
    let p: pos_t = (*s).peak;
    let z: pos_t = (*s).end;
    return a > p
        || p > z
        || a < 0 as ::core::ffi::c_int as pos_t
            && z > 0 as ::core::ffi::c_int as pos_t
            && p != 0 as ::core::ffi::c_int as pos_t
        || p == 0 as ::core::ffi::c_int as pos_t;
}
#[inline]
unsafe extern "C" fn weightAxisRegion(mut as_0: *const vq_AxisSpan, x: pos_t) -> pos_t {
    let a: pos_t = (*as_0).start;
    let p: pos_t = (*as_0).peak;
    let z: pos_t = (*as_0).end;
    if a > p || p > z {
        return 1 as ::core::ffi::c_int as pos_t;
    } else if a < 0 as ::core::ffi::c_int as pos_t
        && z > 0 as ::core::ffi::c_int as pos_t
        && p != 0 as ::core::ffi::c_int as pos_t
    {
        return 1 as ::core::ffi::c_int as pos_t;
    } else if p == 0 as ::core::ffi::c_int as pos_t {
        return 1 as ::core::ffi::c_int as pos_t;
    } else if x < a || x > z {
        return 0 as ::core::ffi::c_int as pos_t;
    } else if x == p {
        return 1 as ::core::ffi::c_int as pos_t;
    } else if x < p {
        return (x - a) / (p - a);
    } else {
        return (z - x) / (z - p);
    };
}
#[no_mangle]
pub unsafe extern "C" fn vqRegionGetWeight(mut r: *const vq_Region, mut v: *const VV) -> pos_t {
    let mut w: pos_t = 1 as ::core::ffi::c_int as pos_t;
    let mut j: usize = 0 as usize;
    while j < (*r).dimensions as usize && (*v).length != 0 {
        w *= weightAxisRegion(
            (&raw const (*r).spans as *const vq_AxisSpan).offset(j as isize) as *const vq_AxisSpan,
            *(*v).items.offset(j as isize),
        );
        j = j.wrapping_add(1);
    }
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn vq_showRegion(mut _r: *const vq_Region) {}
