#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::parsed_json::{ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_getnum, json_type_of};
use crate::support::binio::{read_8u, read_16u, read_16s};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{ShapeId};
use crate::vendor::json::{JsonType};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::vdmx::types::{VdmxTable, VdmxRatioRange, VdmxRecord};
use crate::bk::bkgraph::{bk_build_block_no_minimize};
use crate::support::built_json::{BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push};
pub unsafe extern "C" fn otfcc_read_vdmx(
    packet: Packet,
    mut options: *const Options,
) -> Option<Box<VdmxTable>> {
    let mut version: u16 = 0;
    let mut num_ratios: u16 = 0;
    let mut vdmx: Option<Box<VdmxTable>> = None;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_VDMX {
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
                            vdmx = Some(Box::new(VdmxTable { version, ratios: Vec::new() }));
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
                                    b_charset: 0,
                                    x_ratio: 0,
                                    y_start_ratio: 0,
                                    y_end_ratio: 0,
                                    records: Vec::new(),
                                };
                                r.b_charset = read_8u(
                                    table
                                        .data
                                        .offset(ratio_range_offset as isize)
                                        .offset(0 as ::core::ffi::c_int as isize),
                                );
                                r.x_ratio = read_8u(
                                    table
                                        .data
                                        .offset(ratio_range_offset as isize)
                                        .offset(1 as ::core::ffi::c_int as isize),
                                );
                                r.y_start_ratio = read_8u(
                                    table
                                        .data
                                        .offset(ratio_range_offset as isize)
                                        .offset(2 as ::core::ffi::c_int as isize),
                                );
                                r.y_end_ratio = read_8u(
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
                                    let mut y_pel_height: u16 = read_16u(
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
                                    let mut y_max: i16 = read_16s(
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
                                    let mut y_min: i16 = read_16s(
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
                                    r.records.push(VdmxRecord {
                                        y_pel_height: y_pel_height,
                                        y_max: y_max,
                                        y_min: y_min,
                                    });
                                    j = j.wrapping_add(1);
                                }
                                vdmx.as_mut().unwrap().ratios.push(r);
                                g = g.wrapping_add(1);
                            }
                            return vdmx;
                        }
                    }
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(b"Table 'VDMX' corrupted.\n"),
                    );
                    vdmx = None;
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
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_dump_vdmx(
    vdmx: Option<&VdmxTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let vdmx = match vdmx {
        Some(v) => v,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::bytesbuild!(b"VDMX"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _vdmx: *mut BuiltValue = json_object_new(2 as usize);
        json_object_push(
            _vdmx,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*vdmx).version as i64),
        );
        let ratios: &Vec<VdmxRatioRange> = &(*vdmx).ratios;
        let mut _ratios: *mut BuiltValue = json_array_new(ratios.len());
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < ratios.len() {
            let rr: &VdmxRatioRange = &ratios[__caryll_index];
            while keep != 0 {
                let mut _rr: *mut BuiltValue = json_object_new(5 as usize);
                json_object_push(
                    _rr,
                    b"bCharset\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).b_charset as i64),
                );
                json_object_push(
                    _rr,
                    b"xRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).x_ratio as i64),
                );
                json_object_push(
                    _rr,
                    b"yStartRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).y_start_ratio as i64),
                );
                json_object_push(
                    _rr,
                    b"yEndRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).y_end_ratio as i64),
                );
                let mut _records: *mut BuiltValue = json_array_new(rr.records.len());
                let mut __caryll_index_0: usize = 0 as usize;
                let mut keep_0: usize = 1 as usize;
                while keep_0 != 0 && __caryll_index_0 < rr.records.len() {
                    let r: &VdmxRecord = &rr.records[__caryll_index_0];
                    while keep_0 != 0 {
                        let mut _r: *mut BuiltValue = json_object_new(3 as usize);
                        json_object_push(
                            _r,
                            b"yPelHeight\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).y_pel_height as i64),
                        );
                        json_object_push(
                            _r,
                            b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).y_max as i64),
                        );
                        json_object_push(
                            _r,
                            b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).y_min as i64),
                        );
                        json_array_push(_records, _r);
                        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    }
                    keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
                    __caryll_index_0 = __caryll_index_0.wrapping_add(1);
                }
                json_object_push(
                    _rr,
                    b"records\0" as *const u8 as *const ::core::ffi::c_char,
                    _records,
                );
                json_array_push(_ratios, _rr);
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        json_object_push(
            _vdmx,
            b"ratios\0" as *const u8 as *const ::core::ffi::c_char,
            _ratios,
        );
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
    mut root: *const ParsedValue,
    mut options: *const Options,
) -> Option<Box<VdmxTable>> {
    let mut _vdmx: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    _vdmx = json_obj_get_type(
        root,
        b"VDMX\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if _vdmx.is_null() {
        return None;
    }
    let mut vdmx: Box<VdmxTable> = Box::new(VdmxTable { version: 0, ratios: Vec::new() });
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::bytesbuild!(b"VDMX"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        (*vdmx).version = json_obj_getnum(
            _vdmx,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u16;
        let mut _ratios: *const ParsedValue = json_obj_get_type(
            _vdmx,
            b"ratios\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
        );
        let mut j: usize = 0 as usize;
        while j < json_arr_len(_ratios) as usize {
            let mut _rr: *const ParsedValue = json_arr_at(_ratios, j as u32);
            if !(_rr.is_null()
                || json_type_of(_rr) != JsonType::Object)
            {
                let mut r: VdmxRatioRange = VdmxRatioRange {
                    b_charset: 0,
                    x_ratio: 0,
                    y_start_ratio: 0,
                    y_end_ratio: 0,
                    records: Vec::new(),
                };
                r.b_charset = json_obj_getnum(
                    _rr,
                    b"bCharset\0" as *const u8 as *const ::core::ffi::c_char,
                ) as u8;
                r.x_ratio =
                    json_obj_getnum(_rr, b"xRatio\0" as *const u8 as *const ::core::ffi::c_char)
                        as u8;
                r.y_start_ratio = json_obj_getnum(
                    _rr,
                    b"yStartRatio\0" as *const u8 as *const ::core::ffi::c_char,
                ) as u8;
                r.y_end_ratio = json_obj_getnum(
                    _rr,
                    b"yEndRatio\0" as *const u8 as *const ::core::ffi::c_char,
                ) as u8;
                let mut _records: *const ParsedValue = json_obj_get_type(
                    _rr,
                    b"records\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                );
                if !_records.is_null() {
                    let mut j_0: usize = 0 as usize;
                    while j_0 < json_arr_len(_records) as usize {
                        let mut _r: *const ParsedValue = json_arr_at(_records, j_0 as u32);
                        if !(_r.is_null()
                            || json_type_of(_r) != JsonType::Object)
                        {
                            r.records.push(VdmxRecord {
                                y_pel_height: json_obj_getnum(
                                    _r,
                                    b"yPelHeight\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as u16,
                                y_max: json_obj_getnum(
                                    _r,
                                    b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as i16,
                                y_min: json_obj_getnum(
                                    _r,
                                    b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as i16,
                            });
                        }
                        j_0 = j_0.wrapping_add(1);
                    }
                    (*vdmx).ratios.push(r);
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return Some(vdmx);
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_vdmx(
    vdmx: Option<&VdmxTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let vdmx = match vdmx {
        Some(v) => v,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let ratios: &Vec<VdmxRatioRange> = &(*vdmx).ratios;
    if ratios.is_empty() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*vdmx).version as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (ratios.len()) as u32), bk_int(BkCellType::B16, (ratios.len()) as u32)]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < ratios.len() {
        let rr: &VdmxRatioRange = &ratios[__caryll_index];
        while keep != 0 {
            bk_push(root, &[bk_int(BkCellType::B8, (rr.b_charset as ::core::ffi::c_int) as u32), bk_int(BkCellType::B8, (rr.x_ratio as ::core::ffi::c_int) as u32), bk_int(BkCellType::B8, (rr.y_start_ratio as ::core::ffi::c_int) as u32), bk_int(BkCellType::B8, (rr.y_end_ratio as ::core::ffi::c_int) as u32)]);
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut __caryll_index_0: usize = 0 as usize;
    let mut keep_0: usize = 1 as usize;
    while keep_0 != 0 && __caryll_index_0 < ratios.len() {
        let rr_0: &VdmxRatioRange = &ratios[__caryll_index_0];
        while keep_0 != 0 {
            let mut startsz: u16 = 0xffff as u16;
            let mut endsz: u16 = 0 as u16;
            let mut __caryll_index_1: usize = 0 as usize;
            let mut keep_1: usize = 1 as usize;
            while keep_1 != 0 && __caryll_index_1 < rr_0.records.len() {
                let r: &VdmxRecord = &rr_0.records[__caryll_index_1];
                while keep_1 != 0 {
                    if startsz as ::core::ffi::c_int > r.y_pel_height as ::core::ffi::c_int {
                        startsz = r.y_pel_height;
                    }
                    if (endsz as ::core::ffi::c_int) < r.y_pel_height as ::core::ffi::c_int {
                        endsz = r.y_pel_height;
                    }
                    keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
                }
                keep_1 = (keep_1 == 0) as ::core::ffi::c_int as usize;
                __caryll_index_1 = __caryll_index_1.wrapping_add(1);
            }
            let mut group: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (rr_0.records.len()) as u32), bk_int(BkCellType::B8, (startsz as ::core::ffi::c_int) as u32), bk_int(BkCellType::B8, (endsz as ::core::ffi::c_int) as u32)]);
            let mut __caryll_index_2: usize = 0 as usize;
            let mut keep_2: usize = 1 as usize;
            while keep_2 != 0 && __caryll_index_2 < rr_0.records.len() {
                let r_0: &VdmxRecord = &rr_0.records[__caryll_index_2];
                while keep_2 != 0 {
                    bk_push(group, &[bk_int(BkCellType::B16, (r_0.y_pel_height as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (r_0.y_max as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (r_0.y_min as ::core::ffi::c_int) as u32)]);
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
