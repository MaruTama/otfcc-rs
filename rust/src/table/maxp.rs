#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};
use crate::support::json_funcs::{json_obj_get_type, json_obj_getnum};
use crate::support::binio::{read_16u, read_32s};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, FontFilePointer};
use crate::vendor::json::{JsonType, JsonValue};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b};
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::vendor::json_builder::{json_double_new, json_integer_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsempty};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MaxpTable {
    pub version: F16Dot16,
    pub numGlyphs: u16,
    pub maxPoints: u16,
    pub maxContours: u16,
    pub maxCompositePoints: u16,
    pub maxCompositeContours: u16,
    pub maxZones: u16,
    pub maxTwilightPoints: u16,
    pub maxStorage: u16,
    pub maxFunctionDefs: u16,
    pub maxInstructionDefs: u16,
    pub maxStackElements: u16,
    pub maxSizeOfInstructions: u16,
    pub maxComponentElements: u16,
    pub maxComponentDepth: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MaxpTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut MaxpTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut MaxpTable, *const MaxpTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut MaxpTable, *mut MaxpTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut MaxpTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut MaxpTable, MaxpTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut MaxpTable, MaxpTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut MaxpTable>,
    pub free: Option<unsafe extern "C" fn(*mut MaxpTable) -> ()>,
}
#[inline]
unsafe extern "C" fn init_maxp(mut maxp: *mut MaxpTable) {
    memset(
        maxp as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<MaxpTable>() as usize,
    );
    (*maxp).version = 0x10000 as ::core::ffi::c_int as F16Dot16;
}
#[inline]
unsafe extern "C" fn dispose_maxp(mut _maxp: *mut MaxpTable) {}
#[inline]
unsafe extern "C" fn table_maxp_replace(mut dst: *mut MaxpTable, src: MaxpTable) {
    table_maxp_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MaxpTable>() as usize,
    );
}
pub static TABLE_I_MAXP: MaxpTableElementInterface = {
    MaxpTableElementInterface {
        init: Some(table_maxp_init as unsafe extern "C" fn(*mut MaxpTable) -> ()),
        copy: Some(
            table_maxp_copy as unsafe extern "C" fn(*mut MaxpTable, *const MaxpTable) -> (),
        ),
        move_0: Some(
            table_maxp_move as unsafe extern "C" fn(*mut MaxpTable, *mut MaxpTable) -> (),
        ),
        dispose: Some(table_maxp_dispose as unsafe extern "C" fn(*mut MaxpTable) -> ()),
        replace: Some(
            table_maxp_replace as unsafe extern "C" fn(*mut MaxpTable, MaxpTable) -> (),
        ),
        copyReplace: Some(
            table_maxp_copy_replace as unsafe extern "C" fn(*mut MaxpTable, MaxpTable) -> (),
        ),
        create: Some(table_maxp_create),
        free: Some(table_maxp_free as unsafe extern "C" fn(*mut MaxpTable) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_maxp_create() -> *mut MaxpTable {
    let mut x: *mut MaxpTable =
        malloc(::core::mem::size_of::<MaxpTable>() as usize) as *mut MaxpTable;
    table_maxp_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_maxp_init(mut x: *mut MaxpTable) {
    init_maxp(x);
}
#[inline]
unsafe extern "C" fn table_maxp_free(mut x: *mut MaxpTable) {
    if x.is_null() {
        return;
    }
    table_maxp_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_maxp_copy_replace(mut dst: *mut MaxpTable, src: MaxpTable) {
    table_maxp_dispose(dst);
    table_maxp_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_maxp_copy(mut dst: *mut MaxpTable, mut src: *const MaxpTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MaxpTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_maxp_dispose(mut x: *mut MaxpTable) {
    dispose_maxp(x);
}
#[inline]
unsafe extern "C" fn table_maxp_move(mut dst: *mut MaxpTable, mut src: *mut MaxpTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<MaxpTable>() as usize,
    );
    table_maxp_init(src);
}
pub unsafe extern "C" fn otfcc_read_maxp(
    packet: Packet,
    mut options: *const Options,
) -> *mut MaxpTable {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1835104368i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if length != 32 as u32 && length != 6 as u32 {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::sdsbuild!(sdsempty(), b"table 'maxp' corrupted.\n"),
                        );
                    } else {
                        let mut maxp: *mut MaxpTable =
                            (
                                TABLE_I_MAXP.create.expect("non-null function pointer"))();
                        (*maxp).version = read_32s(data as *const u8) as F16Dot16;
                        (*maxp).numGlyphs = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if (*maxp).version == 0x10000 as F16Dot16 {
                            (*maxp).maxPoints = read_16u(
                                data.offset(6 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            (*maxp).maxContours = read_16u(
                                data.offset(8 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            (*maxp).maxCompositePoints =
                                read_16u(data.offset(10 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).maxCompositeContours =
                                read_16u(data.offset(12 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).maxZones =
                                read_16u(data.offset(14 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).maxTwilightPoints =
                                read_16u(data.offset(16 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).maxStorage =
                                read_16u(data.offset(18 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).maxFunctionDefs =
                                read_16u(data.offset(20 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).maxInstructionDefs =
                                read_16u(data.offset(22 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).maxStackElements =
                                read_16u(data.offset(24 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).maxSizeOfInstructions =
                                read_16u(data.offset(26 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).maxComponentElements =
                                read_16u(data.offset(28 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*maxp).maxComponentDepth =
                                read_16u(data.offset(30 as ::core::ffi::c_int as isize)
                                    as *const u8);
                        } else {
                            (*maxp).maxPoints = 0 as u16;
                            (*maxp).maxContours = 0 as u16;
                            (*maxp).maxCompositePoints = 0 as u16;
                            (*maxp).maxCompositeContours = 0 as u16;
                            (*maxp).maxZones = 0 as u16;
                            (*maxp).maxTwilightPoints = 0 as u16;
                            (*maxp).maxStorage = 0 as u16;
                            (*maxp).maxFunctionDefs = 0 as u16;
                            (*maxp).maxInstructionDefs = 0 as u16;
                            (*maxp).maxStackElements = 0 as u16;
                            (*maxp).maxSizeOfInstructions = 0 as u16;
                            (*maxp).maxComponentElements = 0 as u16;
                            (*maxp).maxComponentDepth = 0 as u16;
                        }
                        return maxp;
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
    return ::core::ptr::null_mut::<MaxpTable>();
}
pub unsafe extern "C" fn otfcc_dump_maxp(
    mut table: *const MaxpTable,
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
        crate::sdsbuild!(sdsempty(), b"maxp"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut maxp: *mut JsonValue = json_object_new(15 as usize);
        json_object_push(
            maxp,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            maxp,
            b"numGlyphs\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).numGlyphs as i64),
        );
        json_object_push(
            maxp,
            b"maxPoints\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxPoints as i64),
        );
        json_object_push(
            maxp,
            b"maxContours\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxContours as i64),
        );
        json_object_push(
            maxp,
            b"maxCompositePoints\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxCompositePoints as i64),
        );
        json_object_push(
            maxp,
            b"maxCompositeContours\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxCompositeContours as i64),
        );
        json_object_push(
            maxp,
            b"maxZones\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxZones as i64),
        );
        json_object_push(
            maxp,
            b"maxTwilightPoints\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxTwilightPoints as i64),
        );
        json_object_push(
            maxp,
            b"maxStorage\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxStorage as i64),
        );
        json_object_push(
            maxp,
            b"maxFunctionDefs\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxFunctionDefs as i64),
        );
        json_object_push(
            maxp,
            b"maxInstructionDefs\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxInstructionDefs as i64),
        );
        json_object_push(
            maxp,
            b"maxStackElements\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxStackElements as i64),
        );
        json_object_push(
            maxp,
            b"maxSizeOfInstructions\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxSizeOfInstructions as i64),
        );
        json_object_push(
            maxp,
            b"maxComponentElements\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxComponentElements as i64),
        );
        json_object_push(
            maxp,
            b"maxComponentDepth\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxComponentDepth as i64),
        );
        json_object_push(
            root,
            b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
            maxp,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_maxp(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut MaxpTable {
    let mut maxp: *mut MaxpTable = (
        TABLE_I_MAXP.create.expect("non-null function pointer"))();
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"maxp"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*maxp).version = otfcc_to_fixed(json_obj_getnum(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            (*maxp).numGlyphs = json_obj_getnum(
                table,
                b"numGlyphs\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).maxZones = json_obj_getnum(
                table,
                b"maxZones\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).maxTwilightPoints = json_obj_getnum(
                table,
                b"maxTwilightPoints\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).maxStorage = json_obj_getnum(
                table,
                b"maxStorage\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).maxFunctionDefs = json_obj_getnum(
                table,
                b"maxFunctionDefs\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).maxInstructionDefs = json_obj_getnum(
                table,
                b"maxInstructionDefs\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            (*maxp).maxStackElements = json_obj_getnum(
                table,
                b"maxStackElements\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u16;
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return maxp;
}
pub unsafe extern "C" fn otfcc_build_maxp(
    mut maxp: *const MaxpTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if maxp.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite32b(buf, (*maxp).version as u32);
    bufwrite16b(buf, (*maxp).numGlyphs);
    if (*maxp).version > 0x5000 as F16Dot16 {
        bufwrite16b(buf, (*maxp).maxPoints);
        bufwrite16b(buf, (*maxp).maxContours);
        bufwrite16b(buf, (*maxp).maxCompositePoints);
        bufwrite16b(buf, (*maxp).maxCompositeContours);
        bufwrite16b(buf, (*maxp).maxZones);
        bufwrite16b(buf, (*maxp).maxTwilightPoints);
        bufwrite16b(buf, (*maxp).maxStorage);
        bufwrite16b(buf, (*maxp).maxFunctionDefs);
        bufwrite16b(buf, (*maxp).maxInstructionDefs);
        bufwrite16b(buf, (*maxp).maxStackElements);
        bufwrite16b(buf, (*maxp).maxSizeOfInstructions);
        bufwrite16b(buf, (*maxp).maxComponentElements);
        bufwrite16b(buf, (*maxp).maxComponentDepth);
    }
    return buf;
}
