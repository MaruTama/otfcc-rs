#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::json_funcs::{json_obj_get_type, json_obj_getnum};
use crate::support::binio::{read_8u, read_16u, read_16s};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{ShapeId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::vdmx::types::{VdmxTable, VdmxGroup, VdmxRatioRange, VdmxRecord};
use crate::bk::bkgraph::{bk_build_block_no_minimize};
use crate::table::vdmx::types::{TABLE_I_VDMX, VDMX_I_GROUP, VDMX_I_RATIO_RANGE, VDMX_I_RATIO_RANGE_LIST};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsempty};
pub unsafe extern "C" fn otfcc_read_vdmx(
    packet: Packet,
    mut options: *const Options,
) -> *mut VdmxTable {
    let mut version: u16 = 0;
    let mut num_ratios: u16 = 0;
    let mut vdmx: *mut VdmxTable = ::core::ptr::null_mut::<VdmxTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1447316824i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    if !(table.length < 6 as u32) {
                        version = read_16u(table.data.offset(0 as ::core::ffi::c_int as isize));
                        num_ratios = read_16u(table.data.offset(4 as ::core::ffi::c_int as isize));
                        if !(table.length
                            < (6 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int * num_ratios as ::core::ffi::c_int)
                                as u32)
                        {
                            vdmx = (
                                TABLE_I_VDMX.create.expect("non-null function pointer"))();
                            (*vdmx).version = version;
                            let mut g: ShapeId = 0 as ShapeId;
                            while (g as ::core::ffi::c_int) < num_ratios as ::core::ffi::c_int {
                                let ratio_range_offset: usize = (6 as ::core::ffi::c_int
                                    + 4 as ::core::ffi::c_int * g as ::core::ffi::c_int)
                                    as usize;
                                let offset_offset: usize = (6 as ::core::ffi::c_int
                                    + 4 as ::core::ffi::c_int * num_ratios as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * g as ::core::ffi::c_int)
                                    as usize;
                                let mut r: VdmxRatioRange = VdmxRatioRange {
                                    bCharset: 0,
                                    xRatio: 0,
                                    yStartRatio: 0,
                                    yEndRatio: 0,
                                    records: VdmxGroup {
                                        length: 0,
                                        capacity: 0,
                                        items: ::core::ptr::null_mut::<VdmxRecord>(),
                                    },
                                };
                                VDMX_I_RATIO_RANGE.init.expect("non-null function pointer")(
                                    &raw mut r,
                                );
                                r.bCharset = read_8u(
                                    table
                                        .data
                                        .offset(ratio_range_offset as isize)
                                        .offset(0 as ::core::ffi::c_int as isize),
                                );
                                r.xRatio = read_8u(
                                    table
                                        .data
                                        .offset(ratio_range_offset as isize)
                                        .offset(1 as ::core::ffi::c_int as isize),
                                );
                                r.yStartRatio = read_8u(
                                    table
                                        .data
                                        .offset(ratio_range_offset as isize)
                                        .offset(2 as ::core::ffi::c_int as isize),
                                );
                                r.yEndRatio = read_8u(
                                    table
                                        .data
                                        .offset(ratio_range_offset as isize)
                                        .offset(3 as ::core::ffi::c_int as isize),
                                );
                                let mut group_offset: u16 =
                                    read_16u(table.data.offset(offset_offset as isize));
                                let mut recs: u16 = read_16u(
                                    table
                                        .data
                                        .offset(group_offset as ::core::ffi::c_int as isize)
                                        .offset(0 as ::core::ffi::c_int as isize),
                                );
                                let mut j: u16 = 0 as u16;
                                while (j as ::core::ffi::c_int) < recs as ::core::ffi::c_int {
                                    let mut yPelHeight: u16 = read_16u(
                                        table
                                            .data
                                            .offset(group_offset as ::core::ffi::c_int as isize)
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
                                            .offset(group_offset as ::core::ffi::c_int as isize)
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
                                            .offset(group_offset as ::core::ffi::c_int as isize)
                                            .offset(4 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (j as ::core::ffi::c_int * 6 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(4 as ::core::ffi::c_int as isize),
                                    );
                                    VDMX_I_GROUP.push.expect("non-null function pointer")(
                                        &raw mut r.records,
                                        VdmxRecord {
                                            yPelHeight: yPelHeight,
                                            yMax: yMax,
                                            yMin: yMin,
                                        },
                                    );
                                    j = j.wrapping_add(1);
                                }
                                VDMX_I_RATIO_RANGE_LIST
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
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(sdsempty(), b"Table 'VDMX' corrupted.\n"),
                    );
                    TABLE_I_VDMX.free.expect("non-null function pointer")(vdmx);
                    vdmx = ::core::ptr::null_mut::<VdmxTable>();
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
pub unsafe extern "C" fn otfcc_dump_vdmx(
    mut vdmx: *const VdmxTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if vdmx.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"VDMX"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _vdmx: *mut JsonValue = json_object_new(2 as usize);
        json_object_push(
            _vdmx,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*vdmx).version as i64),
        );
        let mut _ratios: *mut JsonValue = json_array_new((*vdmx).ratios.length);
        json_object_push(
            _vdmx,
            b"ratios\0" as *const u8 as *const ::core::ffi::c_char,
            _ratios,
        );
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < (*vdmx).ratios.length {
            let mut rr: *mut VdmxRatioRange = (*vdmx).ratios.items.offset(__caryll_index as isize);
            while keep != 0 {
                let mut _rr: *mut JsonValue = json_object_new(5 as usize);
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
                let mut _records: *mut JsonValue = json_array_new((*rr).records.length);
                json_object_push(
                    _rr,
                    b"records\0" as *const u8 as *const ::core::ffi::c_char,
                    _records,
                );
                let mut __caryll_index_0: usize = 0 as usize;
                let mut keep_0: usize = 1 as usize;
                while keep_0 != 0 && __caryll_index_0 < (*rr).records.length {
                    let mut r: *mut VdmxRecord =
                        (*rr).records.items.offset(__caryll_index_0 as isize);
                    while keep_0 != 0 {
                        let mut _r: *mut JsonValue = json_object_new(3 as usize);
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
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_vdmx(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut VdmxTable {
    let mut _vdmx: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    _vdmx = json_obj_get_type(
        root,
        b"VDMX\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if _vdmx.is_null() {
        return ::core::ptr::null_mut::<VdmxTable>();
    }
    let mut vdmx: *mut VdmxTable = (
        TABLE_I_VDMX.create.expect("non-null function pointer"))();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"VDMX"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        (*vdmx).version = json_obj_getnum(
            _vdmx,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u16;
        let mut _ratios: *mut JsonValue = json_obj_get_type(
            _vdmx,
            b"ratios\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
        );
        let mut j: usize = 0 as usize;
        while j < (*_ratios).u.array.length as usize {
            let mut _rr: *mut JsonValue =
                *(*_ratios).u.array.values.offset(j as isize) as *mut JsonValue;
            if !(_rr.is_null()
                || (*_rr).type_0 != JsonType::Object)
            {
                let mut r: VdmxRatioRange = VdmxRatioRange {
                    bCharset: 0,
                    xRatio: 0,
                    yStartRatio: 0,
                    yEndRatio: 0,
                    records: VdmxGroup {
                        length: 0,
                        capacity: 0,
                        items: ::core::ptr::null_mut::<VdmxRecord>(),
                    },
                };
                VDMX_I_RATIO_RANGE.init.expect("non-null function pointer")(&raw mut r);
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
                let mut _records: *mut JsonValue = json_obj_get_type(
                    _rr,
                    b"records\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                );
                if _records.is_null() {
                    VDMX_I_RATIO_RANGE.dispose.expect("non-null function pointer")(&raw mut r);
                } else {
                    let mut j_0: usize = 0 as usize;
                    while j_0 < (*_records).u.array.length as usize {
                        let mut _r: *mut JsonValue =
                            *(*_records).u.array.values.offset(j_0 as isize) as *mut JsonValue;
                        if !(_r.is_null()
                            || (*_r).type_0 != JsonType::Object)
                        {
                            VDMX_I_GROUP.push.expect("non-null function pointer")(
                                &raw mut r.records,
                                VdmxRecord {
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
                    VDMX_I_RATIO_RANGE_LIST
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
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return vdmx;
}
pub unsafe extern "C" fn otfcc_build_vdmx(
    mut vdmx: *const VdmxTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if vdmx.is_null() || (*vdmx).ratios.length == 0 {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*vdmx).version as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*vdmx).ratios.length) as u32), bk_int(BkCellType::B16, ((*vdmx).ratios.length) as u32)]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*vdmx).ratios.length {
        let mut rr: *mut VdmxRatioRange = (*vdmx).ratios.items.offset(__caryll_index as isize);
        while keep != 0 {
            bk_push(root, &[bk_int(BkCellType::B8, ((*rr).bCharset as ::core::ffi::c_int) as u32), bk_int(BkCellType::B8, ((*rr).xRatio as ::core::ffi::c_int) as u32), bk_int(BkCellType::B8, ((*rr).yStartRatio as ::core::ffi::c_int) as u32), bk_int(BkCellType::B8, ((*rr).yEndRatio as ::core::ffi::c_int) as u32)]);
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut __caryll_index_0: usize = 0 as usize;
    let mut keep_0: usize = 1 as usize;
    while keep_0 != 0 && __caryll_index_0 < (*vdmx).ratios.length {
        let mut rr_0: *mut VdmxRatioRange = (*vdmx).ratios.items.offset(__caryll_index_0 as isize);
        while keep_0 != 0 {
            let mut startsz: u16 = 0xffff as u16;
            let mut endsz: u16 = 0 as u16;
            let mut __caryll_index_1: usize = 0 as usize;
            let mut keep_1: usize = 1 as usize;
            while keep_1 != 0 && __caryll_index_1 < (*rr_0).records.length {
                let mut r: *mut VdmxRecord =
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
            let mut group: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*rr_0).records.length) as u32), bk_int(BkCellType::B8, (startsz as ::core::ffi::c_int) as u32), bk_int(BkCellType::B8, (endsz as ::core::ffi::c_int) as u32)]);
            let mut __caryll_index_2: usize = 0 as usize;
            let mut keep_2: usize = 1 as usize;
            while keep_2 != 0 && __caryll_index_2 < (*rr_0).records.length {
                let mut r_0: *mut VdmxRecord =
                    (*rr_0).records.items.offset(__caryll_index_2 as isize);
                while keep_2 != 0 {
                    bk_push(group, &[bk_int(BkCellType::B16, ((*r_0).yPelHeight as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*r_0).yMax as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*r_0).yMin as ::core::ffi::c_int) as u32)]);
                    keep_2 = (keep_2 == 0) as ::core::ffi::c_int as usize;
                }
                keep_2 = (keep_2 == 0) as ::core::ffi::c_int as usize;
                __caryll_index_2 = __caryll_index_2.wrapping_add(1);
            }
            bk_push(root, &[bk_ptr(BkCellType::P16, group)]);
            keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
        }
        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
        __caryll_index_0 = __caryll_index_0.wrapping_add(1);
    }
    return bk_build_block_no_minimize(root);
}
