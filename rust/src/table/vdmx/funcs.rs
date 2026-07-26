#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::json_funcs::{json_obj_get_type, json_obj_getnum};
use crate::support::binio::{read_8u, read_16u, read_16s};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{shapeid_t};
use crate::vendor::json::{json_array, json_object, json_value};
use crate::bk::bkblock::{b16, b8, bk_Block, bk_int, bk_new_Block, bk_ptr, bk_push, p16};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};

use crate::table::vdmx::types::{table_VDMX, vdmx_Group, vdmx_RatioRange, vdmx_Record};
use crate::bk::bkgraph::{bk_build_Block_noMinimize};
use crate::table::vdmx::types::{table_iVDMX, vdmx_iGroup, vdmx_iRatioRange, vdmx_iRatioRangeList};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsempty};
pub unsafe extern "C" fn otfcc_readVDMX(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_VDMX {
    let mut version: u16 = 0;
    let mut numRatios: u16 = 0;
    let mut vdmx: *mut table_VDMX = ::core::ptr::null_mut::<table_VDMX>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1447316824i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    if !(table.length < 6 as u32) {
                        version = read_16u(table.data.offset(0 as ::core::ffi::c_int as isize));
                        numRatios = read_16u(table.data.offset(4 as ::core::ffi::c_int as isize));
                        if !(table.length
                            < (6 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int * numRatios as ::core::ffi::c_int)
                                as u32)
                        {
                            vdmx = (
                                table_iVDMX.create.expect("non-null function pointer"))();
                            (*vdmx).version = version;
                            let mut g: shapeid_t = 0 as shapeid_t;
                            while (g as ::core::ffi::c_int) < numRatios as ::core::ffi::c_int {
                                let ratioRangeOffset: usize = (6 as ::core::ffi::c_int
                                    + 4 as ::core::ffi::c_int * g as ::core::ffi::c_int)
                                    as usize;
                                let offsetOffset: usize = (6 as ::core::ffi::c_int
                                    + 4 as ::core::ffi::c_int * numRatios as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * g as ::core::ffi::c_int)
                                    as usize;
                                let mut r: vdmx_RatioRange = vdmx_RatioRange {
                                    bCharset: 0,
                                    xRatio: 0,
                                    yStartRatio: 0,
                                    yEndRatio: 0,
                                    records: vdmx_Group {
                                        length: 0,
                                        capacity: 0,
                                        items: ::core::ptr::null_mut::<vdmx_Record>(),
                                    },
                                };
                                vdmx_iRatioRange.init.expect("non-null function pointer")(
                                    &raw mut r,
                                );
                                r.bCharset = read_8u(
                                    table
                                        .data
                                        .offset(ratioRangeOffset as isize)
                                        .offset(0 as ::core::ffi::c_int as isize),
                                );
                                r.xRatio = read_8u(
                                    table
                                        .data
                                        .offset(ratioRangeOffset as isize)
                                        .offset(1 as ::core::ffi::c_int as isize),
                                );
                                r.yStartRatio = read_8u(
                                    table
                                        .data
                                        .offset(ratioRangeOffset as isize)
                                        .offset(2 as ::core::ffi::c_int as isize),
                                );
                                r.yEndRatio = read_8u(
                                    table
                                        .data
                                        .offset(ratioRangeOffset as isize)
                                        .offset(3 as ::core::ffi::c_int as isize),
                                );
                                let mut groupOffset: u16 =
                                    read_16u(table.data.offset(offsetOffset as isize));
                                let mut recs: u16 = read_16u(
                                    table
                                        .data
                                        .offset(groupOffset as ::core::ffi::c_int as isize)
                                        .offset(0 as ::core::ffi::c_int as isize),
                                );
                                let mut j: u16 = 0 as u16;
                                while (j as ::core::ffi::c_int) < recs as ::core::ffi::c_int {
                                    let mut yPelHeight: u16 = read_16u(
                                        table
                                            .data
                                            .offset(groupOffset as ::core::ffi::c_int as isize)
                                            .offset(4 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (j as ::core::ffi::c_int * 6 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(0 as ::core::ffi::c_int as isize),
                                    );
                                    let mut yMax: i16 = read_16s(
                                        table
                                            .data
                                            .offset(groupOffset as ::core::ffi::c_int as isize)
                                            .offset(4 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (j as ::core::ffi::c_int * 6 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize),
                                    );
                                    let mut yMin: i16 = read_16s(
                                        table
                                            .data
                                            .offset(groupOffset as ::core::ffi::c_int as isize)
                                            .offset(4 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (j as ::core::ffi::c_int * 6 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(4 as ::core::ffi::c_int as isize),
                                    );
                                    vdmx_iGroup.push.expect("non-null function pointer")(
                                        &raw mut r.records,
                                        vdmx_Record {
                                            yPelHeight: yPelHeight,
                                            yMax: yMax,
                                            yMin: yMin,
                                        },
                                    );
                                    j = j.wrapping_add(1);
                                }
                                vdmx_iRatioRangeList
                                    .push
                                    .expect("non-null function pointer")(
                                    &raw mut (*vdmx).ratios,
                                    r,
                                );
                                g = g.wrapping_add(1);
                            }
                            return vdmx;
                        }
                    }
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important,
                        log_type_warning,
                        crate::sdsbuild!(sdsempty(), b"Table 'VDMX' corrupted.\n"),
                    );
                    table_iVDMX.free.expect("non-null function pointer")(vdmx);
                    vdmx = ::core::ptr::null_mut::<table_VDMX>();
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return vdmx;
}
pub unsafe extern "C" fn otfcc_dumpVDMX(
    mut vdmx: *const table_VDMX,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if vdmx.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"VDMX"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _vdmx: *mut json_value = json_object_new(2 as usize);
        json_object_push(
            _vdmx,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*vdmx).version as i64),
        );
        let mut _ratios: *mut json_value = json_array_new((*vdmx).ratios.length);
        json_object_push(
            _vdmx,
            b"ratios\0" as *const u8 as *const ::core::ffi::c_char,
            _ratios,
        );
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < (*vdmx).ratios.length {
            let mut rr: *mut vdmx_RatioRange = (*vdmx).ratios.items.offset(__caryll_index as isize);
            while keep != 0 {
                let mut _rr: *mut json_value = json_object_new(5 as usize);
                json_array_push(_ratios, _rr);
                json_object_push(
                    _rr,
                    b"bCharset\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).bCharset as i64),
                );
                json_object_push(
                    _rr,
                    b"xRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).xRatio as i64),
                );
                json_object_push(
                    _rr,
                    b"yStartRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).yStartRatio as i64),
                );
                json_object_push(
                    _rr,
                    b"yEndRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).yEndRatio as i64),
                );
                let mut _records: *mut json_value = json_array_new((*rr).records.length);
                json_object_push(
                    _rr,
                    b"records\0" as *const u8 as *const ::core::ffi::c_char,
                    _records,
                );
                let mut __caryll_index_0: usize = 0 as usize;
                let mut keep_0: usize = 1 as usize;
                while keep_0 != 0 && __caryll_index_0 < (*rr).records.length {
                    let mut r: *mut vdmx_Record =
                        (*rr).records.items.offset(__caryll_index_0 as isize);
                    while keep_0 != 0 {
                        let mut _r: *mut json_value = json_object_new(3 as usize);
                        json_array_push(_records, _r);
                        json_object_push(
                            _r,
                            b"yPelHeight\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).yPelHeight as i64),
                        );
                        json_object_push(
                            _r,
                            b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).yMax as i64),
                        );
                        json_object_push(
                            _r,
                            b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).yMin as i64),
                        );
                        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    }
                    keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    __caryll_index_0 = __caryll_index_0.wrapping_add(1);
                }
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        json_object_push(
            root,
            b"VDMX\0" as *const u8 as *const ::core::ffi::c_char,
            _vdmx,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parseVDMX(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_VDMX {
    let mut _vdmx: *mut json_value = ::core::ptr::null_mut::<json_value>();
    _vdmx = json_obj_get_type(
        root,
        b"VDMX\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if _vdmx.is_null() {
        return ::core::ptr::null_mut::<table_VDMX>();
    }
    let mut vdmx: *mut table_VDMX = (
        table_iVDMX.create.expect("non-null function pointer"))();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"VDMX"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        (*vdmx).version = json_obj_getnum(
            _vdmx,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u16;
        let mut _ratios: *mut json_value = json_obj_get_type(
            _vdmx,
            b"ratios\0" as *const u8 as *const ::core::ffi::c_char,
            json_array,
        );
        let mut j: usize = 0 as usize;
        while j < (*_ratios).u.array.length as usize {
            let mut _rr: *mut json_value =
                *(*_ratios).u.array.values.offset(j as isize) as *mut json_value;
            if !(_rr.is_null()
                || (*_rr).type_0 != json_object)
            {
                let mut r: vdmx_RatioRange = vdmx_RatioRange {
                    bCharset: 0,
                    xRatio: 0,
                    yStartRatio: 0,
                    yEndRatio: 0,
                    records: vdmx_Group {
                        length: 0,
                        capacity: 0,
                        items: ::core::ptr::null_mut::<vdmx_Record>(),
                    },
                };
                vdmx_iRatioRange.init.expect("non-null function pointer")(&raw mut r);
                r.bCharset = json_obj_getnum(
                    _rr,
                    b"bCharset\0" as *const u8 as *const ::core::ffi::c_char,
                ) as u8;
                r.xRatio =
                    json_obj_getnum(_rr, b"xRatio\0" as *const u8 as *const ::core::ffi::c_char)
                        as u8;
                r.yStartRatio = json_obj_getnum(
                    _rr,
                    b"yStartRatio\0" as *const u8 as *const ::core::ffi::c_char,
                ) as u8;
                r.yEndRatio = json_obj_getnum(
                    _rr,
                    b"yEndRatio\0" as *const u8 as *const ::core::ffi::c_char,
                ) as u8;
                let mut _records: *mut json_value = json_obj_get_type(
                    _rr,
                    b"records\0" as *const u8 as *const ::core::ffi::c_char,
                    json_array,
                );
                if _records.is_null() {
                    vdmx_iRatioRange.dispose.expect("non-null function pointer")(&raw mut r);
                } else {
                    let mut j_0: usize = 0 as usize;
                    while j_0 < (*_records).u.array.length as usize {
                        let mut _r: *mut json_value =
                            *(*_records).u.array.values.offset(j_0 as isize) as *mut json_value;
                        if !(_r.is_null()
                            || (*_r).type_0 != json_object)
                        {
                            vdmx_iGroup.push.expect("non-null function pointer")(
                                &raw mut r.records,
                                vdmx_Record {
                                    yPelHeight: json_obj_getnum(
                                        _r,
                                        b"yPelHeight\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) as u16,
                                    yMax: json_obj_getnum(
                                        _r,
                                        b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) as i16,
                                    yMin: json_obj_getnum(
                                        _r,
                                        b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) as i16,
                                },
                            );
                        }
                        j_0 = j_0.wrapping_add(1);
                    }
                    vdmx_iRatioRangeList
                        .push
                        .expect("non-null function pointer")(
                        &raw mut (*vdmx).ratios, r
                    );
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    return vdmx;
}
pub unsafe extern "C" fn otfcc_buildVDMX(
    mut vdmx: *const table_VDMX,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if vdmx.is_null() || (*vdmx).ratios.length == 0 {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut root: *mut bk_Block = bk_new_Block(&[bk_int(b16, ((*vdmx).version as ::core::ffi::c_int) as u32), bk_int(b16, ((*vdmx).ratios.length) as u32), bk_int(b16, ((*vdmx).ratios.length) as u32)]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*vdmx).ratios.length {
        let mut rr: *mut vdmx_RatioRange = (*vdmx).ratios.items.offset(__caryll_index as isize);
        while keep != 0 {
            bk_push(root, &[bk_int(b8, ((*rr).bCharset as ::core::ffi::c_int) as u32), bk_int(b8, ((*rr).xRatio as ::core::ffi::c_int) as u32), bk_int(b8, ((*rr).yStartRatio as ::core::ffi::c_int) as u32), bk_int(b8, ((*rr).yEndRatio as ::core::ffi::c_int) as u32)]);
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut __caryll_index_0: usize = 0 as usize;
    let mut keep_0: usize = 1 as usize;
    while keep_0 != 0 && __caryll_index_0 < (*vdmx).ratios.length {
        let mut rr_0: *mut vdmx_RatioRange = (*vdmx).ratios.items.offset(__caryll_index_0 as isize);
        while keep_0 != 0 {
            let mut startsz: u16 = 0xffff as u16;
            let mut endsz: u16 = 0 as u16;
            let mut __caryll_index_1: usize = 0 as usize;
            let mut keep_1: usize = 1 as usize;
            while keep_1 != 0 && __caryll_index_1 < (*rr_0).records.length {
                let mut r: *mut vdmx_Record =
                    (*rr_0).records.items.offset(__caryll_index_1 as isize);
                while keep_1 != 0 {
                    if startsz as ::core::ffi::c_int > (*r).yPelHeight as ::core::ffi::c_int {
                        startsz = (*r).yPelHeight;
                    }
                    if (endsz as ::core::ffi::c_int) < (*r).yPelHeight as ::core::ffi::c_int {
                        endsz = (*r).yPelHeight;
                    }
                    keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
                }
                keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
                __caryll_index_1 = __caryll_index_1.wrapping_add(1);
            }
            let mut group: *mut bk_Block = bk_new_Block(&[bk_int(b16, ((*rr_0).records.length) as u32), bk_int(b8, (startsz as ::core::ffi::c_int) as u32), bk_int(b8, (endsz as ::core::ffi::c_int) as u32)]);
            let mut __caryll_index_2: usize = 0 as usize;
            let mut keep_2: usize = 1 as usize;
            while keep_2 != 0 && __caryll_index_2 < (*rr_0).records.length {
                let mut r_0: *mut vdmx_Record =
                    (*rr_0).records.items.offset(__caryll_index_2 as isize);
                while keep_2 != 0 {
                    bk_push(group, &[bk_int(b16, ((*r_0).yPelHeight as ::core::ffi::c_int) as u32), bk_int(b16, ((*r_0).yMax as ::core::ffi::c_int) as u32), bk_int(b16, ((*r_0).yMin as ::core::ffi::c_int) as u32)]);
                    keep_2 = (keep_2 == 0) as ::core::ffi::c_int as usize;
                }
                keep_2 = (keep_2 == 0) as ::core::ffi::c_int as usize;
                __caryll_index_2 = __caryll_index_2.wrapping_add(1);
            }
            bk_push(root, &[bk_ptr(p16, group)]);
            keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
        }
        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
        __caryll_index_0 = __caryll_index_0.wrapping_add(1);
    }
    return bk_build_Block_noMinimize(root);
}
