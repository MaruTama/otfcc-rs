use libc::{free, strcmp};
extern "C" {
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    static otl_iClassDef: __otfcc_IClassDef;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
}

use crate::table::otl::classdef::{otl_ClassDef_create, pushClassDef, otl_ClassDef};
use crate::table::otl::coverage::{otl_Coverage};
use crate::support::handle::{handle_fromIndex, otfcc_GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphclass_t, glyphid_t};
use crate::vendor::json::{json_object, json_type, json_value};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_PacketPiece {
    pub tag: u32,
    pub checkSum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Packet {
    pub sfnt_version: u32,
    pub numTables: u16,
    pub searchRange: u16,
    pub entrySelector: u16,
    pub rangeShift: u16,
    pub pieces: *mut otfcc_PacketPiece,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __otfcc_IClassDef {
    pub init: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otl_ClassDef, *const otl_ClassDef) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otl_ClassDef, *mut otl_ClassDef) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otl_ClassDef, otl_ClassDef) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otl_ClassDef>,
    pub free: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
    pub push:
        Option<unsafe extern "C" fn(*mut otl_ClassDef, otfcc_GlyphHandle, glyphclass_t) -> ()>,
    pub read: Option<unsafe extern "C" fn(*const u8, u32, u32) -> *mut otl_ClassDef>,
    pub expand:
        Option<unsafe extern "C" fn(*mut otl_Coverage, *mut otl_ClassDef) -> *mut otl_ClassDef>,
    pub dump: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut json_value>,
    pub parse: Option<unsafe extern "C" fn(*const json_value) -> *mut otl_ClassDef>,
    pub build: Option<unsafe extern "C" fn(*const otl_ClassDef) -> *mut caryll_Buffer>,
    pub shrink: Option<unsafe extern "C" fn(*mut otl_ClassDef) -> ()>,
}
pub type table_TSI5 = otl_ClassDef;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn otfcc_readTSI5(
    packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
) -> *mut table_TSI5 {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1414744373i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut tsi5: *mut table_TSI5 =
                        otl_ClassDef_create() as *mut table_TSI5;
                    let mut j: glyphid_t = 0 as glyphid_t;
                    while ((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32)
                        < table.length
                    {
                        pushClassDef(
                            tsi5 as *mut otl_ClassDef,
                            handle_fromIndex(j)
                                as otfcc_GlyphHandle,
                            read_16u(table.data.offset(
                                (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                            )) as glyphclass_t,
                        );
                        j = j.wrapping_add(1);
                    }
                    return tsi5;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_TSI5>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpTSI5(
    mut table: *const table_TSI5,
    mut root: *mut json_value,
    mut _options: *const otfcc_Options,
) {
    if table.is_null() {
        return;
    }
    json_object_push(
        root,
        b"TSI5\0" as *const u8 as *const ::core::ffi::c_char,
        otl_iClassDef.dump.expect("non-null function pointer")(table as *const otl_ClassDef),
    );
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseTSI5(
    mut root: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut table_TSI5 {
    let mut _tsi: *mut json_value = ::core::ptr::null_mut::<json_value>();
    _tsi = json_obj_get_type(
        root,
        b"TSI5\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if _tsi.is_null() {
        return ::core::ptr::null_mut::<table_TSI5>();
    }
    return otl_iClassDef.parse.expect("non-null function pointer")(_tsi) as *mut table_TSI5;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildTSI5(
    mut tsi5: *const table_TSI5,
    mut _options: *const otfcc_Options,
    mut numGlyphs: glyphid_t,
) -> *mut caryll_Buffer {
    if tsi5.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut tsi5cls: *mut u16 = ::core::ptr::null_mut::<u16>();
    tsi5cls = __caryll_allocate_clean(
        (::core::mem::size_of::<u16>() as usize).wrapping_mul(numGlyphs as usize),
        27 as ::core::ffi::c_ulong,
    ) as *mut u16;
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_int) < (*tsi5).numGlyphs as ::core::ffi::c_int {
        if ((*(*tsi5).glyphs.offset(j as isize)).index as ::core::ffi::c_int)
            < numGlyphs as ::core::ffi::c_int
        {
            *tsi5cls.offset((*(*tsi5).glyphs.offset(j as isize)).index as isize) =
                *(*tsi5).classes.offset(j as isize) as u16;
        }
        j = j.wrapping_add(1);
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as ::core::ffi::c_int) < numGlyphs as ::core::ffi::c_int {
        bufwrite16b(buf, *tsi5cls.offset(j_0 as isize));
        j_0 = j_0.wrapping_add(1);
    }
    free(tsi5cls as *mut ::core::ffi::c_void);
    tsi5cls = ::core::ptr::null_mut::<u16>();
    return buf;
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
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
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
