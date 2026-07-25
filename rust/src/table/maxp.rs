use libc::{free, malloc, memcpy, memset, strcmp};
extern "C" {
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double;
    fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
}
use crate::support::binio::{read_16u, read_32s};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{f16dot16, font_file_pointer};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_double, json_integer, json_object, json_type, json_value};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_maxp {
    pub version: f16dot16,
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
pub struct __caryll_elementinterface_table_maxp {
    pub init: Option<unsafe extern "C" fn(*mut table_maxp) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_maxp, *const table_maxp) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_maxp, *mut table_maxp) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_maxp) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_maxp, table_maxp) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_maxp, table_maxp) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_maxp>,
    pub free: Option<unsafe extern "C" fn(*mut table_maxp) -> ()>,
}
#[inline]
unsafe extern "C" fn initMaxp(mut maxp: *mut table_maxp) {
    memset(
        maxp as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_maxp>() as usize,
    );
    (*maxp).version = 0x10000 as ::core::ffi::c_int as f16dot16;
}
#[inline]
unsafe extern "C" fn disposeMaxp(mut _maxp: *mut table_maxp) {}
#[inline]
unsafe extern "C" fn table_maxp_replace(mut dst: *mut table_maxp, src: table_maxp) {
    table_maxp_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_maxp>() as usize,
    );
}
#[no_mangle]
pub static mut table_iMaxp: __caryll_elementinterface_table_maxp = {
    __caryll_elementinterface_table_maxp {
        init: Some(table_maxp_init as unsafe extern "C" fn(*mut table_maxp) -> ()),
        copy: Some(
            table_maxp_copy as unsafe extern "C" fn(*mut table_maxp, *const table_maxp) -> (),
        ),
        move_0: Some(
            table_maxp_move as unsafe extern "C" fn(*mut table_maxp, *mut table_maxp) -> (),
        ),
        dispose: Some(table_maxp_dispose as unsafe extern "C" fn(*mut table_maxp) -> ()),
        replace: Some(
            table_maxp_replace as unsafe extern "C" fn(*mut table_maxp, table_maxp) -> (),
        ),
        copyReplace: Some(
            table_maxp_copyReplace as unsafe extern "C" fn(*mut table_maxp, table_maxp) -> (),
        ),
        create: Some(table_maxp_create),
        free: Some(table_maxp_free as unsafe extern "C" fn(*mut table_maxp) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_maxp_create() -> *mut table_maxp {
    let mut x: *mut table_maxp =
        malloc(::core::mem::size_of::<table_maxp>() as usize) as *mut table_maxp;
    table_maxp_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_maxp_init(mut x: *mut table_maxp) {
    initMaxp(x);
}
#[inline]
unsafe extern "C" fn table_maxp_free(mut x: *mut table_maxp) {
    if x.is_null() {
        return;
    }
    table_maxp_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_maxp_copyReplace(mut dst: *mut table_maxp, src: table_maxp) {
    table_maxp_dispose(dst);
    table_maxp_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_maxp_copy(mut dst: *mut table_maxp, mut src: *const table_maxp) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_maxp>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_maxp_dispose(mut x: *mut table_maxp) {
    disposeMaxp(x);
}
#[inline]
unsafe extern "C" fn table_maxp_move(mut dst: *mut table_maxp, mut src: *mut table_maxp) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_maxp>() as usize,
    );
    table_maxp_init(src);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readMaxp(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_maxp {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1835104368i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    if length != 32 as u32 && length != 6 as u32 {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"table 'maxp' corrupted.\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                    } else {
                        let mut maxp: *mut table_maxp =
                            (
                                table_iMaxp.create.expect("non-null function pointer"))();
                        (*maxp).version = read_32s(data as *const u8) as f16dot16;
                        (*maxp).numGlyphs = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if (*maxp).version == 0x10000 as f16dot16 {
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
    return ::core::ptr::null_mut::<table_maxp>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpMaxp(
    mut table: *const table_maxp,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut maxp: *mut json_value = json_object_new(15 as usize);
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
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseMaxp(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_maxp {
    let mut maxp: *mut table_maxp = (
        table_iMaxp.create.expect("non-null function pointer"))();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
            ),
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
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    return maxp;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildMaxp(
    mut maxp: *const table_maxp,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if maxp.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite32b(buf, (*maxp).version as u32);
    bufwrite16b(buf, (*maxp).numGlyphs);
    if (*maxp).version > 0x5000 as f16dot16 {
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
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
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
