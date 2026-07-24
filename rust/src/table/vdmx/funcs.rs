extern "C" {
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    static vdmx_iGroup: __caryll_vectorinterface_vdmx_Group;
    static vdmx_iRatioRange: __caryll_elementinterface_vdmx_RatioRange;
    static vdmx_iRatioRangeList: __caryll_vectorinterface_vdmx_RatioRagneList;
    static table_iVDMX: __caryll_elementinterface_table_VDMX;
    fn json_array_new(length: size_t) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: size_t) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_integer_new(_: int64_t) -> *mut json_value;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_build_Block_noMinimize(root: *mut bk_Block) -> *mut caryll_Buffer;
}
use crate::support::binio::{read_8u, read_16u, read_16s};
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int16_t = __int16_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type json_type = ::core::ffi::c_uint;
pub const json_pre_serialized: json_type = 8;
pub const json_null: json_type = 7;
pub const json_boolean: json_type = 6;
pub const json_string: json_type = 5;
pub const json_double: json_type = 4;
pub const json_integer: json_type = 3;
pub const json_array: json_type = 2;
pub const json_object: json_type = 1;
pub const json_none: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_value {
    pub parent: *mut _json_value,
    pub type_0: json_type,
    pub u: C2RustUnnamed_0,
    pub _reserved: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub next_alloc: *mut _json_value,
    pub object_mem: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub boolean: ::core::ffi::c_int,
    pub integer: int64_t,
    pub dbl: ::core::ffi::c_double,
    pub string: C2RustUnnamed_3,
    pub object: C2RustUnnamed_2,
    pub array: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut json_object_entry,
}
pub type json_object_entry = _json_object_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_object_entry {
    pub name: *mut ::core::ffi::c_char,
    pub name_length: ::core::ffi::c_uint,
    pub value: *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub length: ::core::ffi::c_uint,
    pub ptr: *mut ::core::ffi::c_char,
}
pub type json_value = _json_value;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type shapeid_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILoggerTarget {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()>,
}
pub type otfcc_LoggerType = ::core::ffi::c_uint;
pub const log_type_progress: otfcc_LoggerType = 3;
pub const log_type_info: otfcc_LoggerType = 2;
pub const log_type_warning: otfcc_LoggerType = 1;
pub const log_type_error: otfcc_LoggerType = 0;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed_4 = 10;
pub const log_vl_info: C2RustUnnamed_4 = 5;
pub const log_vl_notice: C2RustUnnamed_4 = 2;
pub const log_vl_important: C2RustUnnamed_4 = 1;
pub const log_vl_critical: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILogger {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub indent: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub indentSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub start: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub startSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub log: Option<
        unsafe extern "C" fn(
            *mut otfcc_ILogger,
            uint8_t,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t) -> ()>,
    pub getTarget: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Options {
    pub debug_wait_on_start: bool,
    pub ignore_glyph_order: bool,
    pub ignore_hints: bool,
    pub has_vertical_metrics: bool,
    pub export_fdselect: bool,
    pub keep_average_char_width: bool,
    pub keep_unicode_ranges: bool,
    pub short_post: bool,
    pub dummy_DSIG: bool,
    pub keep_modified_time: bool,
    pub instr_as_bytes: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub cff_short_vmtx: bool,
    pub merge_lookups: bool,
    pub merge_features: bool,
    pub force_cid: bool,
    pub cff_rollCharString: bool,
    pub cff_doSubroutinize: bool,
    pub stub_cmap4: bool,
    pub decimal_cmap: bool,
    pub name_glyphs_by_hash: bool,
    pub name_glyphs_by_gid: bool,
    pub glyph_name_prefix: *mut ::core::ffi::c_char,
    pub logger: *mut otfcc_ILogger,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_PacketPiece {
    pub tag: uint32_t,
    pub checkSum: uint32_t,
    pub offset: uint32_t,
    pub length: uint32_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Packet {
    pub sfnt_version: uint32_t,
    pub numTables: uint16_t,
    pub searchRange: uint16_t,
    pub entrySelector: uint16_t,
    pub rangeShift: uint16_t,
    pub pieces: *mut otfcc_PacketPiece,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_Record {
    pub yPelHeight: uint16_t,
    pub yMax: int16_t,
    pub yMin: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_Group {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut vdmx_Record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_vdmx_Group {
    pub init: Option<unsafe extern "C" fn(*mut vdmx_Group) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut vdmx_Group, *const vdmx_Group) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut vdmx_Group, *mut vdmx_Group) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut vdmx_Group) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut vdmx_Group, vdmx_Group) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut vdmx_Group, vdmx_Group) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut vdmx_Group>,
    pub free: Option<unsafe extern "C" fn(*mut vdmx_Group) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut vdmx_Group, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut vdmx_Group, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut vdmx_Group>,
    pub fill: Option<unsafe extern "C" fn(*mut vdmx_Group, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut vdmx_Group) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut vdmx_Group, vdmx_Record) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut vdmx_Group) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut vdmx_Group) -> vdmx_Record>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut vdmx_Group, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut vdmx_Group,
            Option<unsafe extern "C" fn(*const vdmx_Record, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut vdmx_Group,
            Option<
                unsafe extern "C" fn(*const vdmx_Record, *const vdmx_Record) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_RatioRange {
    pub bCharset: uint8_t,
    pub xRatio: uint8_t,
    pub yStartRatio: uint8_t,
    pub yEndRatio: uint8_t,
    pub records: vdmx_Group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_vdmx_RatioRange {
    pub init: Option<unsafe extern "C" fn(*mut vdmx_RatioRange) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut vdmx_RatioRange, *const vdmx_RatioRange) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut vdmx_RatioRange, *mut vdmx_RatioRange) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut vdmx_RatioRange) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut vdmx_RatioRange, vdmx_RatioRange) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut vdmx_RatioRange, vdmx_RatioRange) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdmx_RatioRagneList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut vdmx_RatioRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_vdmx_RatioRagneList {
    pub init: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, *const vdmx_RatioRagneList) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, *mut vdmx_RatioRagneList) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, vdmx_RatioRagneList) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, vdmx_RatioRagneList) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut vdmx_RatioRagneList>,
    pub free: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut vdmx_RatioRagneList>,
    pub fill: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, vdmx_RatioRange) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList) -> vdmx_RatioRange>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut vdmx_RatioRagneList, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut vdmx_RatioRagneList,
            Option<unsafe extern "C" fn(*const vdmx_RatioRange, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut vdmx_RatioRagneList,
            Option<
                unsafe extern "C" fn(
                    *const vdmx_RatioRange,
                    *const vdmx_RatioRange,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_VDMX {
    pub version: uint16_t,
    pub ratios: vdmx_RatioRagneList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_VDMX {
    pub init: Option<unsafe extern "C" fn(*mut table_VDMX) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_VDMX, *const table_VDMX) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_VDMX, *mut table_VDMX) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_VDMX) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_VDMX, table_VDMX) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_VDMX, table_VDMX) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_VDMX>,
    pub free: Option<unsafe extern "C" fn(*mut table_VDMX) -> ()>,
}
pub type bk_Block = __caryll_bkblock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_bkblock {
    pub _visitstate: bk_cell_visit_state,
    pub _index: uint32_t,
    pub _height: uint32_t,
    pub _depth: uint32_t,
    pub length: uint32_t,
    pub free: uint32_t,
    pub cells: *mut bk_Cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bk_Cell {
    pub t: bk_CellType,
    pub c2rust_unnamed: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub z: uint32_t,
    pub p: *mut __caryll_bkblock,
}
pub type bk_CellType = ::core::ffi::c_uint;
pub const bkembed: bk_CellType = 255;
pub const bkcopy: bk_CellType = 254;
pub const sp32: bk_CellType = 129;
pub const sp16: bk_CellType = 128;
pub const p32: bk_CellType = 17;
pub const p16: bk_CellType = 16;
pub const b32: bk_CellType = 3;
pub const b16: bk_CellType = 2;
pub const b8: bk_CellType = 1;
pub const bkover: bk_CellType = 0;
pub type bk_cell_visit_state = ::core::ffi::c_uint;
pub const VISIT_BLACK: bk_cell_visit_state = 2;
pub const VISIT_GRAY: bk_cell_visit_state = 1;
pub const VISIT_WHITE: bk_cell_visit_state = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn otfcc_readVDMX(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_VDMX {
    let mut version: uint16_t = 0;
    let mut numRatios: uint16_t = 0;
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
            if table.tag == 1447316824i32 as uint32_t {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    if !(table.length < 6 as uint32_t) {
                        version = read_16u(table.data.offset(0 as ::core::ffi::c_int as isize));
                        numRatios = read_16u(table.data.offset(4 as ::core::ffi::c_int as isize));
                        if !(table.length
                            < (6 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int * numRatios as ::core::ffi::c_int)
                                as uint32_t)
                        {
                            vdmx = (
                                table_iVDMX.create.expect("non-null function pointer"))();
                            (*vdmx).version = version;
                            let mut g: shapeid_t = 0 as shapeid_t;
                            while (g as ::core::ffi::c_int) < numRatios as ::core::ffi::c_int {
                                let ratioRangeOffset: size_t = (6 as ::core::ffi::c_int
                                    + 4 as ::core::ffi::c_int * g as ::core::ffi::c_int)
                                    as size_t;
                                let offsetOffset: size_t = (6 as ::core::ffi::c_int
                                    + 4 as ::core::ffi::c_int * numRatios as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * g as ::core::ffi::c_int)
                                    as size_t;
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
                                let mut groupOffset: uint16_t =
                                    read_16u(table.data.offset(offsetOffset as isize));
                                let mut recs: uint16_t = read_16u(
                                    table
                                        .data
                                        .offset(groupOffset as ::core::ffi::c_int as isize)
                                        .offset(0 as ::core::ffi::c_int as isize),
                                );
                                let mut j: uint16_t = 0 as uint16_t;
                                while (j as ::core::ffi::c_int) < recs as ::core::ffi::c_int {
                                    let mut yPelHeight: uint16_t = read_16u(
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
                                    let mut yMax: int16_t = read_16s(
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
                                    let mut yMin: int16_t = read_16s(
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
                        log_vl_important as ::core::ffi::c_int as uint8_t,
                        log_type_warning,
                        sdscatprintf(
                            sdsempty(),
                            b"Table 'VDMX' corrupted.\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        ),
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
#[no_mangle]
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
        sdscatprintf(
            sdsempty(),
            b"VDMX\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _vdmx: *mut json_value = json_object_new(2 as size_t);
        json_object_push(
            _vdmx,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*vdmx).version as int64_t),
        );
        let mut _ratios: *mut json_value = json_array_new((*vdmx).ratios.length);
        json_object_push(
            _vdmx,
            b"ratios\0" as *const u8 as *const ::core::ffi::c_char,
            _ratios,
        );
        let mut __caryll_index: size_t = 0 as size_t;
        let mut keep: size_t = 1 as size_t;
        while keep != 0 && __caryll_index < (*vdmx).ratios.length {
            let mut rr: *mut vdmx_RatioRange = (*vdmx).ratios.items.offset(__caryll_index as isize);
            while keep != 0 {
                let mut _rr: *mut json_value = json_object_new(5 as size_t);
                json_array_push(_ratios, _rr);
                json_object_push(
                    _rr,
                    b"bCharset\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).bCharset as int64_t),
                );
                json_object_push(
                    _rr,
                    b"xRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).xRatio as int64_t),
                );
                json_object_push(
                    _rr,
                    b"yStartRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).yStartRatio as int64_t),
                );
                json_object_push(
                    _rr,
                    b"yEndRatio\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*rr).yEndRatio as int64_t),
                );
                let mut _records: *mut json_value = json_array_new((*rr).records.length);
                json_object_push(
                    _rr,
                    b"records\0" as *const u8 as *const ::core::ffi::c_char,
                    _records,
                );
                let mut __caryll_index_0: size_t = 0 as size_t;
                let mut keep_0: size_t = 1 as size_t;
                while keep_0 != 0 && __caryll_index_0 < (*rr).records.length {
                    let mut r: *mut vdmx_Record =
                        (*rr).records.items.offset(__caryll_index_0 as isize);
                    while keep_0 != 0 {
                        let mut _r: *mut json_value = json_object_new(3 as size_t);
                        json_array_push(_records, _r);
                        json_object_push(
                            _r,
                            b"yPelHeight\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).yPelHeight as int64_t),
                        );
                        json_object_push(
                            _r,
                            b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).yMax as int64_t),
                        );
                        json_object_push(
                            _r,
                            b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                            json_integer_new((*r).yMin as int64_t),
                        );
                        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as size_t;
                    }
                    keep_0 = (keep_0 == 0) as ::core::ffi::c_int as size_t;
                    __caryll_index_0 = __caryll_index_0.wrapping_add(1);
                }
                keep = (keep == 0) as ::core::ffi::c_int as size_t;
            }
            keep = (keep == 0) as ::core::ffi::c_int as size_t;
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
#[no_mangle]
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
        sdscatprintf(
            sdsempty(),
            b"VDMX\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        (*vdmx).version = json_obj_getnum(
            _vdmx,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
        ) as uint16_t;
        let mut _ratios: *mut json_value = json_obj_get_type(
            _vdmx,
            b"ratios\0" as *const u8 as *const ::core::ffi::c_char,
            json_array,
        );
        let mut j: size_t = 0 as size_t;
        while j < (*_ratios).u.array.length as size_t {
            let mut _rr: *mut json_value =
                *(*_ratios).u.array.values.offset(j as isize) as *mut json_value;
            if !(_rr.is_null()
                || (*_rr).type_0 as ::core::ffi::c_uint
                    != json_object as ::core::ffi::c_int as ::core::ffi::c_uint)
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
                ) as uint8_t;
                r.xRatio =
                    json_obj_getnum(_rr, b"xRatio\0" as *const u8 as *const ::core::ffi::c_char)
                        as uint8_t;
                r.yStartRatio = json_obj_getnum(
                    _rr,
                    b"yStartRatio\0" as *const u8 as *const ::core::ffi::c_char,
                ) as uint8_t;
                r.yEndRatio = json_obj_getnum(
                    _rr,
                    b"yEndRatio\0" as *const u8 as *const ::core::ffi::c_char,
                ) as uint8_t;
                let mut _records: *mut json_value = json_obj_get_type(
                    _rr,
                    b"records\0" as *const u8 as *const ::core::ffi::c_char,
                    json_array,
                );
                if _records.is_null() {
                    vdmx_iRatioRange.dispose.expect("non-null function pointer")(&raw mut r);
                } else {
                    let mut j_0: size_t = 0 as size_t;
                    while j_0 < (*_records).u.array.length as size_t {
                        let mut _r: *mut json_value =
                            *(*_records).u.array.values.offset(j_0 as isize) as *mut json_value;
                        if !(_r.is_null()
                            || (*_r).type_0 as ::core::ffi::c_uint
                                != json_object as ::core::ffi::c_int as ::core::ffi::c_uint)
                        {
                            vdmx_iGroup.push.expect("non-null function pointer")(
                                &raw mut r.records,
                                vdmx_Record {
                                    yPelHeight: json_obj_getnum(
                                        _r,
                                        b"yPelHeight\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) as uint16_t,
                                    yMax: json_obj_getnum(
                                        _r,
                                        b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) as int16_t,
                                    yMin: json_obj_getnum(
                                        _r,
                                        b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) as int16_t,
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
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildVDMX(
    mut vdmx: *const table_VDMX,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if vdmx.is_null() || (*vdmx).ratios.length == 0 {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut root: *mut bk_Block = bk_new_Block(
        b16 as ::core::ffi::c_int,
        (*vdmx).version as ::core::ffi::c_int,
        b16 as ::core::ffi::c_int,
        (*vdmx).ratios.length,
        b16 as ::core::ffi::c_int,
        (*vdmx).ratios.length,
        bkover as ::core::ffi::c_int,
    );
    let mut __caryll_index: size_t = 0 as size_t;
    let mut keep: size_t = 1 as size_t;
    while keep != 0 && __caryll_index < (*vdmx).ratios.length {
        let mut rr: *mut vdmx_RatioRange = (*vdmx).ratios.items.offset(__caryll_index as isize);
        while keep != 0 {
            bk_push(
                root,
                b8 as ::core::ffi::c_int,
                (*rr).bCharset as ::core::ffi::c_int,
                b8 as ::core::ffi::c_int,
                (*rr).xRatio as ::core::ffi::c_int,
                b8 as ::core::ffi::c_int,
                (*rr).yStartRatio as ::core::ffi::c_int,
                b8 as ::core::ffi::c_int,
                (*rr).yEndRatio as ::core::ffi::c_int,
                bkover as ::core::ffi::c_int,
            );
            keep = (keep == 0) as ::core::ffi::c_int as size_t;
        }
        keep = (keep == 0) as ::core::ffi::c_int as size_t;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut __caryll_index_0: size_t = 0 as size_t;
    let mut keep_0: size_t = 1 as size_t;
    while keep_0 != 0 && __caryll_index_0 < (*vdmx).ratios.length {
        let mut rr_0: *mut vdmx_RatioRange = (*vdmx).ratios.items.offset(__caryll_index_0 as isize);
        while keep_0 != 0 {
            let mut startsz: uint16_t = 0xffff as uint16_t;
            let mut endsz: uint16_t = 0 as uint16_t;
            let mut __caryll_index_1: size_t = 0 as size_t;
            let mut keep_1: size_t = 1 as size_t;
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
                    keep_1 = (keep_1 == 0) as ::core::ffi::c_int as size_t;
                }
                keep_1 = (keep_1 == 0) as ::core::ffi::c_int as size_t;
                __caryll_index_1 = __caryll_index_1.wrapping_add(1);
            }
            let mut group: *mut bk_Block = bk_new_Block(
                b16 as ::core::ffi::c_int,
                (*rr_0).records.length,
                b8 as ::core::ffi::c_int,
                startsz as ::core::ffi::c_int,
                b8 as ::core::ffi::c_int,
                endsz as ::core::ffi::c_int,
                bkover as ::core::ffi::c_int,
            );
            let mut __caryll_index_2: size_t = 0 as size_t;
            let mut keep_2: size_t = 1 as size_t;
            while keep_2 != 0 && __caryll_index_2 < (*rr_0).records.length {
                let mut r_0: *mut vdmx_Record =
                    (*rr_0).records.items.offset(__caryll_index_2 as isize);
                while keep_2 != 0 {
                    bk_push(
                        group,
                        b16 as ::core::ffi::c_int,
                        (*r_0).yPelHeight as ::core::ffi::c_int,
                        b16 as ::core::ffi::c_int,
                        (*r_0).yMax as ::core::ffi::c_int,
                        b16 as ::core::ffi::c_int,
                        (*r_0).yMin as ::core::ffi::c_int,
                        bkover as ::core::ffi::c_int,
                    );
                    keep_2 = (keep_2 == 0) as ::core::ffi::c_int as size_t;
                }
                keep_2 = (keep_2 == 0) as ::core::ffi::c_int as size_t;
                __caryll_index_2 = __caryll_index_2.wrapping_add(1);
            }
            bk_push(
                root,
                p16 as ::core::ffi::c_int,
                group,
                bkover as ::core::ffi::c_int,
            );
            keep_0 = (keep_0 == 0) as ::core::ffi::c_int as size_t;
        }
        keep_0 = (keep_0 == 0) as ::core::ffi::c_int as size_t;
        __caryll_index_0 = __caryll_index_0.wrapping_add(1);
    }
    return bk_build_Block_noMinimize(root);
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_getnum(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0.0f64;
    }
    let mut _k: uint32_t = 0 as uint32_t;
    while _k < (*obj).u.object.length as uint32_t {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0.0f64;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
