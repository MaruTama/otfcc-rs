#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u, read_32s, read_64u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, FontFilePointer};
use crate::vendor::json::{JsonType, JsonValue};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::json_funcs::{json_obj_get, json_obj_get_type, json_obj_getnum_fallback, otfcc_dump_flags, otfcc_parse_flags};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b, bufwrite64b};
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::vendor::json_builder::{json_double_new, json_integer_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsempty};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeadTable {
    pub version: F16Dot16,
    pub fontRevision: u32,
    pub checkSumAdjustment: u32,
    pub magicNumber: u32,
    pub flags: u16,
    pub unitsPerEm: u16,
    pub created: i64,
    pub modified: i64,
    pub xMin: i16,
    pub yMin: i16,
    pub xMax: i16,
    pub yMax: i16,
    pub macStyle: u16,
    pub lowestRecPPEM: u16,
    pub fontDirectoryHint: i16,
    pub indexToLocFormat: i16,
    pub glyphDataFormat: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeadTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut HeadTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut HeadTable, *const HeadTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut HeadTable, *mut HeadTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut HeadTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut HeadTable, HeadTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut HeadTable, HeadTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut HeadTable>,
    pub free: Option<unsafe extern "C" fn(*mut HeadTable) -> ()>,
}
#[inline]
unsafe extern "C" fn initHead(mut head: *mut HeadTable) {
    memset(
        head as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<HeadTable>() as usize,
    );
    (*head).magicNumber = 0x5f0f3cf5 as u32;
    (*head).unitsPerEm = 1000 as u16;
}
#[inline]
unsafe extern "C" fn disposeHead(mut _head: *mut HeadTable) {}
#[inline]
unsafe extern "C" fn table_head_replace(mut dst: *mut HeadTable, src: HeadTable) {
    table_head_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<HeadTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_head_free(mut x: *mut HeadTable) {
    if x.is_null() {
        return;
    }
    table_head_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_head_copyReplace(mut dst: *mut HeadTable, src: HeadTable) {
    table_head_dispose(dst);
    table_head_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_head_copy(mut dst: *mut HeadTable, mut src: *const HeadTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<HeadTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_head_dispose(mut x: *mut HeadTable) {
    disposeHead(x);
}
#[inline]
unsafe extern "C" fn table_head_move(mut dst: *mut HeadTable, mut src: *mut HeadTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<HeadTable>() as usize,
    );
    table_head_init(src);
}
pub static TABLE_I_HEAD: HeadTableElementInterface = {
    HeadTableElementInterface {
        init: Some(table_head_init as unsafe extern "C" fn(*mut HeadTable) -> ()),
        copy: Some(
            table_head_copy as unsafe extern "C" fn(*mut HeadTable, *const HeadTable) -> (),
        ),
        move_0: Some(
            table_head_move as unsafe extern "C" fn(*mut HeadTable, *mut HeadTable) -> (),
        ),
        dispose: Some(table_head_dispose as unsafe extern "C" fn(*mut HeadTable) -> ()),
        replace: Some(
            table_head_replace as unsafe extern "C" fn(*mut HeadTable, HeadTable) -> (),
        ),
        copyReplace: Some(
            table_head_copyReplace as unsafe extern "C" fn(*mut HeadTable, HeadTable) -> (),
        ),
        create: Some(table_head_create),
        free: Some(table_head_free as unsafe extern "C" fn(*mut HeadTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_head_create() -> *mut HeadTable {
    let mut x: *mut HeadTable =
        malloc(::core::mem::size_of::<HeadTable>() as usize) as *mut HeadTable;
    table_head_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_head_init(mut x: *mut HeadTable) {
    initHead(x);
}
pub unsafe extern "C" fn otfcc_readHead(
    packet: Packet,
    mut options: *const Options,
) -> *mut HeadTable {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1751474532i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if length < 54 as u32 {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::sdsbuild!(sdsempty(), b"table 'head' corrupted.\n"),
                        );
                    } else {
                        let mut head: *mut HeadTable = ::core::ptr::null_mut::<HeadTable>();
                        head = __caryll_allocate_clean(
                            ::core::mem::size_of::<HeadTable>() as usize,
                            24 as ::core::ffi::c_ulong,
                        ) as *mut HeadTable;
                        (*head).version = read_32s(data as *const u8) as F16Dot16;
                        (*head).fontRevision = read_32u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).checkSumAdjustment = read_32u(
                            data.offset(8 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).magicNumber = read_32u(
                            data.offset(12 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).flags = read_16u(
                            data.offset(16 as ::core::ffi::c_int as isize) as *const u8
                        );
                        (*head).unitsPerEm = read_16u(
                            data.offset(18 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).created = read_64u(
                            data.offset(20 as ::core::ffi::c_int as isize) as *const u8
                        ) as i64;
                        (*head).modified = read_64u(
                            data.offset(28 as ::core::ffi::c_int as isize) as *const u8
                        ) as i64;
                        (*head).xMin = read_16u(
                            data.offset(36 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).yMin = read_16u(
                            data.offset(38 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).xMax = read_16u(
                            data.offset(40 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).yMax = read_16u(
                            data.offset(42 as ::core::ffi::c_int as isize) as *const u8
                        ) as i16;
                        (*head).macStyle = read_16u(
                            data.offset(44 as ::core::ffi::c_int as isize) as *const u8
                        );
                        (*head).lowestRecPPEM = read_16u(
                            data.offset(46 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        (*head).fontDirectoryHint = read_16u(
                            data.offset(48 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*head).indexToLocFormat = read_16u(
                            data.offset(50 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        (*head).glyphDataFormat = read_16u(
                            data.offset(52 as ::core::ffi::c_int as isize) as *const u8,
                        ) as i16;
                        return head;
                    }
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<HeadTable>();
}
static HEAD_FLAGS_LABELS: [&::core::ffi::CStr; 15] = [
    c"baselineAtY_0",
    c"lsbAtX_0",
    c"instrMayDependOnPointSize",
    c"alwaysUseIntegerSize",
    c"instrMayAlterAdvanceWidth",
    c"designedForVertical",
    c"_reserved1",
    c"designedForComplexScript",
    c"hasMetamorphosisEffects",
    c"containsStrongRTL",
    c"containsIndicRearrangement",
    c"fontIsLossless",
    c"fontIsConverted",
    c"optimizedForCleartype",
    c"lastResortFont",
];
static MAC_STYLE_LABELS: [&::core::ffi::CStr; 7] = [
    c"bold",
    c"italic",
    c"underline",
    c"outline",
    c"shadow",
    c"condensed",
    c"extended",
];
pub unsafe extern "C" fn otfcc_dumpHead(
    mut table: *const HeadTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"head"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut head: *mut JsonValue = json_object_new(15 as usize);
        json_object_push(
            head,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            head,
            b"fontRevision\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).fontRevision as F16Dot16)),
        );
        json_object_push(
            head,
            b"flags\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).flags as ::core::ffi::c_int,
                &HEAD_FLAGS_LABELS,
            ),
        );
        json_object_push(
            head,
            b"unitsPerEm\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).unitsPerEm as i64),
        );
        json_object_push(
            head,
            b"created\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).created),
        );
        json_object_push(
            head,
            b"modified\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).modified),
        );
        json_object_push(
            head,
            b"xMin\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).xMin as i64),
        );
        json_object_push(
            head,
            b"xMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).xMax as i64),
        );
        json_object_push(
            head,
            b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).yMin as i64),
        );
        json_object_push(
            head,
            b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).yMax as i64),
        );
        json_object_push(
            head,
            b"macStyle\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).macStyle as ::core::ffi::c_int,
                &MAC_STYLE_LABELS,
            ),
        );
        json_object_push(
            head,
            b"lowestRecPPEM\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).lowestRecPPEM as i64),
        );
        json_object_push(
            head,
            b"fontDirectoryHint\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).fontDirectoryHint as i64),
        );
        json_object_push(
            head,
            b"indexToLocFormat\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).indexToLocFormat as i64),
        );
        json_object_push(
            head,
            b"glyphDataFormat\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).glyphDataFormat as i64),
        );
        json_object_push(
            root,
            b"head\0" as *const u8 as *const ::core::ffi::c_char,
            head,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parseHead(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut HeadTable {
    let mut head: *mut HeadTable = (
        TABLE_I_HEAD.create.expect("non-null function pointer"))();
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"head\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"head"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*head).version = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ));
            (*head).fontRevision = otfcc_to_fixed(json_obj_getnum_fallback(
                table,
                b"fontRevision\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            )) as u32;
            (*head).flags = otfcc_parse_flags(
                json_obj_get(table, b"flags\0" as *const u8 as *const ::core::ffi::c_char),
                &HEAD_FLAGS_LABELS,
            ) as u16;
            (*head).unitsPerEm = json_obj_getnum_fallback(
                table,
                b"unitsPerEm\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*head).created = json_obj_getnum_fallback(
                table,
                b"created\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i64;
            (*head).modified = json_obj_getnum_fallback(
                table,
                b"modified\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i64;
            (*head).xMin = json_obj_getnum_fallback(
                table,
                b"xMin\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).xMax = json_obj_getnum_fallback(
                table,
                b"xMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).yMin = json_obj_getnum_fallback(
                table,
                b"yMin\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).yMax = json_obj_getnum_fallback(
                table,
                b"yMax\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).macStyle = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"macStyle\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &MAC_STYLE_LABELS,
            ) as u16;
            (*head).lowestRecPPEM = json_obj_getnum_fallback(
                table,
                b"lowestRecPPEM\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*head).fontDirectoryHint = json_obj_getnum_fallback(
                table,
                b"fontDirectoryHint\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).indexToLocFormat = json_obj_getnum_fallback(
                table,
                b"indexToLocFormat\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*head).glyphDataFormat = json_obj_getnum_fallback(
                table,
                b"glyphDataFormat\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return head;
}
pub unsafe extern "C" fn otfcc_buildHead(
    mut head: *const HeadTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if head.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite32b(buf, (*head).version as u32);
    bufwrite32b(buf, (*head).fontRevision);
    bufwrite32b(buf, (*head).checkSumAdjustment);
    bufwrite32b(buf, (*head).magicNumber);
    bufwrite16b(buf, (*head).flags);
    bufwrite16b(buf, (*head).unitsPerEm);
    bufwrite64b(buf, (*head).created as u64);
    bufwrite64b(buf, (*head).modified as u64);
    bufwrite16b(buf, (*head).xMin as u16);
    bufwrite16b(buf, (*head).yMin as u16);
    bufwrite16b(buf, (*head).xMax as u16);
    bufwrite16b(buf, (*head).yMax as u16);
    bufwrite16b(buf, (*head).macStyle);
    bufwrite16b(buf, (*head).lowestRecPPEM);
    bufwrite16b(buf, (*head).fontDirectoryHint as u16);
    bufwrite16b(buf, (*head).indexToLocFormat as u16);
    bufwrite16b(buf, (*head).glyphDataFormat as u16);
    return buf;
}
