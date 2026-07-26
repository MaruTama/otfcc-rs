#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort, strcmp};
unsafe extern "C" {
    fn sdsempty() -> sds;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_object_push_length(
        object: *mut json_value,
        name_length: ::core::ffi::c_uint,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new_length(
        length: ::core::ffi::c_uint,
        _: *const ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
}


use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::binio::{read_16u, read_16s, read_32u};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, pos_t, tableid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_double, json_integer, json_object, json_string, json_type, json_value};
use crate::bk::bkblock::{b16, b32, bk_Block, bk_int, bk_new_Block, bk_ptr, bk_push, p16};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseValue {
    pub tag: u32,
    pub coordinate: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseScriptEntry {
    pub tag: u32,
    pub defaultBaselineTag: u32,
    pub baseValuesCount: tableid_t,
    pub baseValues: *mut otl_BaseValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_BaseAxis {
    pub scriptCount: tableid_t,
    pub entries: *mut otl_BaseScriptEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_BASE {
    pub horizontal: *mut otl_BaseAxis,
    pub vertical: *mut otl_BaseAxis,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_BASE {
    pub init: Option<unsafe extern "C" fn(*mut table_BASE) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_BASE, *const table_BASE) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_BASE, *mut table_BASE) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_BASE) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_BASE, table_BASE) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_BASE, table_BASE) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_BASE>,
    pub free: Option<unsafe extern "C" fn(*mut table_BASE) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base_TagList {
    pub size: tableid_t,
    pub items: *mut u32,
}
unsafe extern "C" fn deleteBaseAxis(mut axis: *mut otl_BaseAxis) {
    if axis.is_null() {
        return;
    }
    if !(*axis).entries.is_null() {
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*axis).scriptCount as ::core::ffi::c_int {
            if !(*(*axis).entries.offset(j as isize)).baseValues.is_null() {
                free((*(*axis).entries.offset(j as isize)).baseValues as *mut ::core::ffi::c_void);
                let ref mut fresh0 = (*(*axis).entries.offset(j as isize)).baseValues;
                *fresh0 = ::core::ptr::null_mut::<otl_BaseValue>();
            }
            j = j.wrapping_add(1);
        }
        free((*axis).entries as *mut ::core::ffi::c_void);
        (*axis).entries = ::core::ptr::null_mut::<otl_BaseScriptEntry>();
    }
}
#[inline]
unsafe extern "C" fn disposeBASE(mut base: *mut table_BASE) {
    deleteBaseAxis((*base).horizontal);
    deleteBaseAxis((*base).vertical);
}
#[inline]
unsafe extern "C" fn table_BASE_dispose(mut x: *mut table_BASE) {
    disposeBASE(x);
}
#[inline]
unsafe extern "C" fn table_BASE_create() -> *mut table_BASE {
    let mut x: *mut table_BASE =
        malloc(::core::mem::size_of::<table_BASE>() as usize) as *mut table_BASE;
    table_BASE_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_BASE_init(mut x: *mut table_BASE) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_BASE>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_BASE_copyReplace(mut dst: *mut table_BASE, src: table_BASE) {
    table_BASE_dispose(dst);
    table_BASE_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_BASE_copy(mut dst: *mut table_BASE, mut src: *const table_BASE) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_BASE>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_BASE_replace(mut dst: *mut table_BASE, src: table_BASE) {
    table_BASE_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_BASE>() as usize,
    );
}
#[unsafe(no_mangle)]
pub static table_iBASE: __caryll_elementinterface_table_BASE = {
    __caryll_elementinterface_table_BASE {
        init: Some(table_BASE_init as unsafe extern "C" fn(*mut table_BASE) -> ()),
        copy: Some(
            table_BASE_copy as unsafe extern "C" fn(*mut table_BASE, *const table_BASE) -> (),
        ),
        move_0: Some(
            table_BASE_move as unsafe extern "C" fn(*mut table_BASE, *mut table_BASE) -> (),
        ),
        dispose: Some(table_BASE_dispose as unsafe extern "C" fn(*mut table_BASE) -> ()),
        replace: Some(
            table_BASE_replace as unsafe extern "C" fn(*mut table_BASE, table_BASE) -> (),
        ),
        copyReplace: Some(
            table_BASE_copyReplace as unsafe extern "C" fn(*mut table_BASE, table_BASE) -> (),
        ),
        create: Some(table_BASE_create),
        free: Some(table_BASE_free as unsafe extern "C" fn(*mut table_BASE) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_BASE_move(mut dst: *mut table_BASE, mut src: *mut table_BASE) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_BASE>() as usize,
    );
    table_BASE_init(src);
}
#[inline]
unsafe extern "C" fn table_BASE_free(mut x: *mut table_BASE) {
    if x.is_null() {
        return;
    }
    table_BASE_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn readBaseValue(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u16,
) -> i16 {
    if tableLength < (offset as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as u32 {
        return 0 as i16;
    } else {
        return read_16s(
            data.offset(offset as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        );
    };
}
unsafe extern "C" fn readBaseScript(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u16,
    mut entry: *mut otl_BaseScriptEntry,
    mut baseTagList: *mut u32,
    mut nBaseTags: u16,
) {
    let mut baseValuesOffset: u16 = 0;
    (*entry).baseValuesCount = 0 as tableid_t;
    (*entry).baseValues = ::core::ptr::null_mut::<otl_BaseValue>();
    (*entry).defaultBaselineTag = 0 as u32;
    if !(tableLength < (offset as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u32) {
        baseValuesOffset =
            read_16u(data.offset(offset as ::core::ffi::c_int as isize) as *const u8);
        if baseValuesOffset != 0 {
            baseValuesOffset =
                (baseValuesOffset as ::core::ffi::c_int + offset as ::core::ffi::c_int) as u16;
            if !(tableLength
                < (baseValuesOffset as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as u32)
            {
                let mut defaultIndex: u16 =
                    (read_16u(data.offset(baseValuesOffset as ::core::ffi::c_int as isize)
                        as *const u8) as ::core::ffi::c_int
                        % nBaseTags as ::core::ffi::c_int) as u16;
                (*entry).defaultBaselineTag = *baseTagList.offset(defaultIndex as isize);
                (*entry).baseValuesCount = read_16u(
                    data.offset(baseValuesOffset as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const u8,
                ) as tableid_t;
                if !((*entry).baseValuesCount as ::core::ffi::c_int
                    != nBaseTags as ::core::ffi::c_int)
                {
                    if !(tableLength
                        < (baseValuesOffset as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * (*entry).baseValuesCount as ::core::ffi::c_int)
                            as u32)
                    {
                        (*entry).baseValues = __caryll_allocate_clean(
                            (::core::mem::size_of::<otl_BaseValue>() as usize)
                                .wrapping_mul((*entry).baseValuesCount as usize),
                            44 as ::core::ffi::c_ulong,
                        ) as *mut otl_BaseValue;
                        let mut j: tableid_t = 0 as tableid_t;
                        while (j as ::core::ffi::c_int)
                            < (*entry).baseValuesCount as ::core::ffi::c_int
                        {
                            (*(*entry).baseValues.offset(j as isize)).tag =
                                *baseTagList.offset(j as isize);
                            let mut _valOffset: u16 = read_16u(
                                data.offset(baseValuesOffset as ::core::ffi::c_int as isize)
                                    .offset(4 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (2 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            );
                            if _valOffset != 0 {
                                (*(*entry).baseValues.offset(j as isize)).coordinate = readBaseValue(
                                    data,
                                    tableLength,
                                    (baseValuesOffset as ::core::ffi::c_int
                                        + _valOffset as ::core::ffi::c_int)
                                        as u16,
                                )
                                    as pos_t;
                            } else {
                                (*(*entry).baseValues.offset(j as isize)).coordinate =
                                    0 as ::core::ffi::c_int as pos_t;
                            }
                            j = j.wrapping_add(1);
                        }
                        return;
                    }
                }
            }
        }
    }
    (*entry).baseValuesCount = 0 as tableid_t;
    if !(*entry).baseValues.is_null() {
        free((*entry).baseValues as *mut ::core::ffi::c_void);
        (*entry).baseValues = ::core::ptr::null_mut::<otl_BaseValue>();
    }
    (*entry).baseValues = ::core::ptr::null_mut::<otl_BaseValue>();
    (*entry).defaultBaselineTag = 0 as u32;
}
unsafe extern "C" fn readAxis(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u16,
) -> *mut otl_BaseAxis {
    let mut baseTagListOffset: u16 = 0;
    let mut nBaseTags: u16 = 0;
    let mut baseScriptListOffset: u16 = 0;
    let mut nBaseScripts: tableid_t = 0;
    let mut axis: *mut otl_BaseAxis = ::core::ptr::null_mut::<otl_BaseAxis>();
    let mut baseTagList: *mut u32 = ::core::ptr::null_mut::<u32>();
    if !(tableLength < (offset as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as u32) {
        baseTagListOffset = (offset as ::core::ffi::c_int
            + read_16u(data.offset(offset as ::core::ffi::c_int as isize) as *const u8)
                as ::core::ffi::c_int) as u16;
        if !(baseTagListOffset as ::core::ffi::c_int <= offset as ::core::ffi::c_int) {
            if !(tableLength
                < (baseTagListOffset as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u32)
            {
                nBaseTags = read_16u(
                    data.offset(baseTagListOffset as ::core::ffi::c_int as isize) as *const u8,
                );
                if !(nBaseTags == 0) {
                    if !(tableLength
                        < (baseTagListOffset as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int * nBaseTags as ::core::ffi::c_int)
                            as u32)
                    {
                        baseTagList = __caryll_allocate_clean(
                            (::core::mem::size_of::<u32>() as usize)
                                .wrapping_mul(nBaseTags as usize),
                            77 as ::core::ffi::c_ulong,
                        ) as *mut u32;
                        let mut j: u16 = 0 as u16;
                        while (j as ::core::ffi::c_int) < nBaseTags as ::core::ffi::c_int {
                            *baseTagList.offset(j as isize) = read_32u(
                                data.offset(baseTagListOffset as ::core::ffi::c_int as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            );
                            j = j.wrapping_add(1);
                        }
                        baseScriptListOffset = (offset as ::core::ffi::c_int
                            + read_16u(
                                data.offset(offset as ::core::ffi::c_int as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as *const u8,
                            ) as ::core::ffi::c_int)
                            as u16;
                        if !(baseScriptListOffset as ::core::ffi::c_int
                            <= offset as ::core::ffi::c_int)
                        {
                            if !(tableLength
                                < (baseScriptListOffset as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int)
                                    as u32)
                            {
                                nBaseScripts = read_16u(
                                    data.offset(baseScriptListOffset as ::core::ffi::c_int as isize)
                                        as *const u8,
                                ) as tableid_t;
                                if !(tableLength
                                    < (baseScriptListOffset as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int
                                        + 6 as ::core::ffi::c_int
                                            * nBaseScripts as ::core::ffi::c_int)
                                        as u32)
                                {
                                    axis = __caryll_allocate_clean(
                                        ::core::mem::size_of::<otl_BaseAxis>() as usize,
                                        87 as ::core::ffi::c_ulong,
                                    )
                                        as *mut otl_BaseAxis;
                                    (*axis).scriptCount = nBaseScripts;
                                    (*axis).entries = __caryll_allocate_clean(
                                        (::core::mem::size_of::<otl_BaseScriptEntry>() as usize)
                                            .wrapping_mul(nBaseScripts as usize),
                                        89 as ::core::ffi::c_ulong,
                                    )
                                        as *mut otl_BaseScriptEntry;
                                    let mut j_0: tableid_t = 0 as tableid_t;
                                    while (j_0 as ::core::ffi::c_int)
                                        < nBaseScripts as ::core::ffi::c_int
                                    {
                                        (*(*axis).entries.offset(j_0 as isize)).tag = read_32u(
                                            data.offset(
                                                baseScriptListOffset as ::core::ffi::c_int as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (6 as ::core::ffi::c_int
                                                    * j_0 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as *const u8,
                                        );
                                        let mut baseScriptOffset: u16 = read_16u(
                                            data.offset(
                                                baseScriptListOffset as ::core::ffi::c_int as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (6 as ::core::ffi::c_int
                                                    * j_0 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(4 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        );
                                        if baseScriptOffset != 0 {
                                            readBaseScript(
                                                data,
                                                tableLength,
                                                (baseScriptListOffset as ::core::ffi::c_int
                                                    + baseScriptOffset as ::core::ffi::c_int)
                                                    as u16,
                                                (*axis).entries.offset(j_0 as isize)
                                                    as *mut otl_BaseScriptEntry,
                                                baseTagList,
                                                nBaseTags,
                                            );
                                        } else {
                                            (*(*axis).entries.offset(j_0 as isize))
                                                .baseValuesCount = 0 as tableid_t;
                                            let ref mut fresh1 =
                                                (*(*axis).entries.offset(j_0 as isize)).baseValues;
                                            *fresh1 = ::core::ptr::null_mut::<otl_BaseValue>();
                                            (*(*axis).entries.offset(j_0 as isize))
                                                .defaultBaselineTag = 0 as u32;
                                        }
                                        j_0 = j_0.wrapping_add(1);
                                    }
                                    return axis;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !baseTagList.is_null() {
        free(baseTagList as *mut ::core::ffi::c_void);
        baseTagList = ::core::ptr::null_mut::<u32>();
    }
    deleteBaseAxis(axis);
    axis = ::core::ptr::null_mut::<otl_BaseAxis>();
    return axis;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_readBASE(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_BASE {
    let mut base: *mut table_BASE = ::core::ptr::null_mut::<table_BASE>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1111577413i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut offsetH: u16 = 0;
                    let mut offsetV: u16 = 0;
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut tableLength: u32 = table.length;
                    if tableLength < 8 as u32 {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important,
                            log_type_warning,
                            crate::sdsbuild!(sdsempty(), b"Table 'BASE' Corrupted"),
                        );
                        table_iBASE.free.expect("non-null function pointer")(base);
                        base = ::core::ptr::null_mut::<table_BASE>();
                    } else {
                        base = __caryll_allocate_clean(
                            ::core::mem::size_of::<table_BASE>() as usize,
                            116 as ::core::ffi::c_ulong,
                        ) as *mut table_BASE;
                        offsetH = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if offsetH != 0 {
                            (*base).horizontal = readAxis(data, tableLength, offsetH);
                        }
                        offsetV = read_16u(
                            data.offset(6 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if offsetV != 0 {
                            (*base).vertical = readAxis(data, tableLength, offsetV);
                        }
                        return base;
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
    return base;
}
unsafe extern "C" fn axisToJson(mut axis: *const otl_BaseAxis) -> *mut json_value {
    let mut _axis: *mut json_value = json_object_new((*axis).scriptCount as usize);
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*axis).scriptCount as ::core::ffi::c_int {
        if !((*(*axis).entries.offset(j as isize)).tag == 0) {
            let mut _entry: *mut json_value = json_object_new(3 as usize);
            if (*(*axis).entries.offset(j as isize)).defaultBaselineTag != 0 {
                let mut tag: [::core::ffi::c_char; 4] = [0; 4];
                tag2str(
                    (*(*axis).entries.offset(j as isize)).defaultBaselineTag,
                    &raw mut tag as *mut ::core::ffi::c_char,
                );
                json_object_push(
                    _entry,
                    b"defaultBaseline\0" as *const u8 as *const ::core::ffi::c_char,
                    json_string_new_length(
                        4 as ::core::ffi::c_uint,
                        &raw mut tag as *mut ::core::ffi::c_char,
                    ),
                );
            }
            let mut _values: *mut json_value =
                json_object_new((*(*axis).entries.offset(j as isize)).baseValuesCount as usize);
            let mut k: tableid_t = 0 as tableid_t;
            while (k as ::core::ffi::c_int)
                < (*(*axis).entries.offset(j as isize)).baseValuesCount as ::core::ffi::c_int
            {
                if (*(*(*axis).entries.offset(j as isize))
                    .baseValues
                    .offset(k as isize))
                .tag != 0
                {
                    json_object_push_tag(
                        _values,
                        (*(*(*axis).entries.offset(j as isize))
                            .baseValues
                            .offset(k as isize))
                        .tag,
                        json_new_position(
                            (*(*(*axis).entries.offset(j as isize))
                                .baseValues
                                .offset(k as isize))
                            .coordinate,
                        ),
                    );
                }
                k = k.wrapping_add(1);
            }
            json_object_push(
                _entry,
                b"baselines\0" as *const u8 as *const ::core::ffi::c_char,
                _values,
            );
            json_object_push_tag(_axis, (*(*axis).entries.offset(j as isize)).tag, _entry);
        }
        j = j.wrapping_add(1);
    }
    return _axis;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_dumpBASE(
    mut base: *const table_BASE,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if base.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"BASE"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _base: *mut json_value = json_object_new(2 as usize);
        if !(*base).horizontal.is_null() {
            json_object_push(
                _base,
                b"horizontal\0" as *const u8 as *const ::core::ffi::c_char,
                axisToJson((*base).horizontal),
            );
        }
        if !(*base).vertical.is_null() {
            json_object_push(
                _base,
                b"vertical\0" as *const u8 as *const ::core::ffi::c_char,
                axisToJson((*base).vertical),
            );
        }
        json_object_push(
            root,
            b"BASE\0" as *const u8 as *const ::core::ffi::c_char,
            _base,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
unsafe extern "C" fn baseScriptFromJson(
    mut _sr: *const json_value,
    mut entry: *mut otl_BaseScriptEntry,
) {
    (*entry).defaultBaselineTag = str2tag(json_obj_getstr_share(
        _sr,
        b"defaultBaseline\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    let mut _basevalues: *mut json_value = json_obj_get_type(
        _sr,
        b"baselines\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if _basevalues.is_null() {
        (*entry).baseValuesCount = 0 as tableid_t;
        (*entry).baseValues = ::core::ptr::null_mut::<otl_BaseValue>();
    } else {
        (*entry).baseValuesCount = (*_basevalues).u.object.length as tableid_t;
        (*entry).baseValues = __caryll_allocate_clean(
            (::core::mem::size_of::<otl_BaseValue>() as usize)
                .wrapping_mul((*entry).baseValuesCount as usize),
            171 as ::core::ffi::c_ulong,
        ) as *mut otl_BaseValue;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*entry).baseValuesCount as ::core::ffi::c_int {
            (*(*entry).baseValues.offset(j as isize)).tag =
                str2tag((*(*_basevalues).u.object.values.offset(j as isize)).name);
            (*(*entry).baseValues.offset(j as isize)).coordinate =
                json_numof((*(*_basevalues).u.object.values.offset(j as isize)).value) as pos_t;
            j = j.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn by_script_tag(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut otl_BaseScriptEntry))
        .tag
        .wrapping_sub((*(b as *mut otl_BaseScriptEntry)).tag) as ::core::ffi::c_int;
}
unsafe extern "C" fn axisFromJson(mut _axis: *const json_value) -> *mut otl_BaseAxis {
    if _axis.is_null() {
        return ::core::ptr::null_mut::<otl_BaseAxis>();
    }
    let mut axis: *mut otl_BaseAxis = ::core::ptr::null_mut::<otl_BaseAxis>();
    axis = __caryll_allocate_clean(
        ::core::mem::size_of::<otl_BaseAxis>() as usize,
        186 as ::core::ffi::c_ulong,
    ) as *mut otl_BaseAxis;
    (*axis).scriptCount = (*_axis).u.object.length as tableid_t;
    (*axis).entries = __caryll_allocate_clean(
        (::core::mem::size_of::<otl_BaseScriptEntry>() as usize)
            .wrapping_mul((*axis).scriptCount as usize),
        188 as ::core::ffi::c_ulong,
    ) as *mut otl_BaseScriptEntry;
    let mut jj: tableid_t = 0 as tableid_t;
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*axis).scriptCount as ::core::ffi::c_int {
        if !(*(*_axis).u.object.values.offset(j as isize))
            .value
            .is_null()
            && (*(*(*_axis).u.object.values.offset(j as isize)).value).type_0 == json_object
        {
            (*(*axis).entries.offset(jj as isize)).tag =
                str2tag((*(*_axis).u.object.values.offset(j as isize)).name);
            baseScriptFromJson(
                (*(*_axis).u.object.values.offset(j as isize)).value,
                (*axis).entries.offset(jj as isize) as *mut otl_BaseScriptEntry,
            );
            jj = jj.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    (*axis).scriptCount = jj;
    qsort(
        (*axis).entries as *mut ::core::ffi::c_void,
        (*axis).scriptCount as usize,
        ::core::mem::size_of::<otl_BaseScriptEntry>() as usize,
        Some(
            by_script_tag
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    return axis;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_parseBASE(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_BASE {
    let mut base: *mut table_BASE = ::core::ptr::null_mut::<table_BASE>();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"BASE\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), b"BASE"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            base = __caryll_allocate_clean(
                ::core::mem::size_of::<table_BASE>() as usize,
                208 as ::core::ffi::c_ulong,
            ) as *mut table_BASE;
            (*base).horizontal = axisFromJson(json_obj_get_type(
                table,
                b"horizontal\0" as *const u8 as *const ::core::ffi::c_char,
                json_object,
            ));
            (*base).vertical = axisFromJson(json_obj_get_type(
                table,
                b"vertical\0" as *const u8 as *const ::core::ffi::c_char,
                json_object,
            ));
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    return base;
}
unsafe extern "C" fn by_tag(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut u32)).wrapping_sub(*(b as *mut u32)) as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn axisToBk(mut axis: *const otl_BaseAxis) -> *mut bk_Block {
    if axis.is_null() {
        return ::core::ptr::null_mut::<bk_Block>();
    }
    let mut taglist: base_TagList = base_TagList {
        size: 0,
        items: ::core::ptr::null_mut::<u32>(),
    };
    taglist.size = 0 as tableid_t;
    taglist.items = ::core::ptr::null_mut::<u32>();
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < (*axis).scriptCount as ::core::ffi::c_int {
        let mut entry: *mut otl_BaseScriptEntry =
            (*axis).entries.offset(j as isize) as *mut otl_BaseScriptEntry;
        if (*entry).defaultBaselineTag != 0 {
            let mut found: bool = false;
            let mut jk: tableid_t = 0 as tableid_t;
            while (jk as ::core::ffi::c_int) < taglist.size as ::core::ffi::c_int {
                if *taglist.items.offset(jk as isize) == (*entry).defaultBaselineTag {
                    found = true;
                    break;
                } else {
                    jk = jk.wrapping_add(1);
                }
            }
            if !found {
                taglist.size =
                    (taglist.size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as tableid_t;
                taglist.items = __caryll_reallocate(
                    taglist.items as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<u32>() as usize)
                        .wrapping_mul(taglist.size as usize),
                    241 as ::core::ffi::c_ulong,
                ) as *mut u32;
                *taglist.items.offset(
                    (taglist.size as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) = (*entry).defaultBaselineTag;
            }
        }
        let mut k: tableid_t = 0 as tableid_t;
        while (k as ::core::ffi::c_int) < (*entry).baseValuesCount as ::core::ffi::c_int {
            let mut tag: u32 = (*(*entry).baseValues.offset(k as isize)).tag;
            let mut found_0: bool = false;
            let mut jk_0: tableid_t = 0 as tableid_t;
            while (jk_0 as ::core::ffi::c_int) < taglist.size as ::core::ffi::c_int {
                if *taglist.items.offset(jk_0 as isize) == tag {
                    found_0 = true;
                    break;
                } else {
                    jk_0 = jk_0.wrapping_add(1);
                }
            }
            if !found_0 {
                taglist.size =
                    (taglist.size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as tableid_t;
                taglist.items = __caryll_reallocate(
                    taglist.items as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<u32>() as usize)
                        .wrapping_mul(taglist.size as usize),
                    256 as ::core::ffi::c_ulong,
                ) as *mut u32;
                *taglist.items.offset(
                    (taglist.size as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) = tag;
            }
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    qsort(
        taglist.items as *mut ::core::ffi::c_void,
        taglist.size as usize,
        ::core::mem::size_of::<u32>() as usize,
        Some(
            by_tag
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut baseTagList: *mut bk_Block = bk_new_Block(&[bk_int(b16, (taglist.size as ::core::ffi::c_int) as u32)]);
    let mut j_0: tableid_t = 0 as tableid_t;
    while (j_0 as ::core::ffi::c_int) < taglist.size as ::core::ffi::c_int {
        bk_push(baseTagList, &[bk_int(b32, (*taglist.items.offset(j_0 as isize)) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    let mut baseScriptList: *mut bk_Block = bk_new_Block(&[bk_int(b16, ((*axis).scriptCount as ::core::ffi::c_int) as u32)]);
    let mut j_1: tableid_t = 0 as tableid_t;
    while (j_1 as ::core::ffi::c_int) < (*axis).scriptCount as ::core::ffi::c_int {
        let mut entry_0: *mut otl_BaseScriptEntry =
            (*axis).entries.offset(j_1 as isize) as *mut otl_BaseScriptEntry;
        let mut baseValues: *mut bk_Block = bk_new_Block(&[]);
        let mut defaultIndex: tableid_t = 0 as tableid_t;
        let mut m: tableid_t = 0 as tableid_t;
        while (m as ::core::ffi::c_int) < taglist.size as ::core::ffi::c_int {
            if *taglist.items.offset(m as isize) == (*entry_0).defaultBaselineTag {
                defaultIndex = m;
                break;
            } else {
                m = m.wrapping_add(1);
            }
        }
        bk_push(baseValues, &[bk_int(b16, (defaultIndex as ::core::ffi::c_int) as u32)]);
        bk_push(baseValues, &[bk_int(b16, (taglist.size as ::core::ffi::c_int) as u32)]);
        let mut m_0: usize = 0 as usize;
        while m_0 < taglist.size as usize {
            let mut found_1: bool = false;
            let mut foundIndex: tableid_t = 0 as tableid_t;
            let mut k_0: tableid_t = 0 as tableid_t;
            while (k_0 as ::core::ffi::c_int) < (*entry_0).baseValuesCount as ::core::ffi::c_int {
                if (*(*entry_0).baseValues.offset(k_0 as isize)).tag
                    == *taglist.items.offset(m_0 as isize)
                {
                    found_1 = true;
                    foundIndex = k_0;
                    break;
                } else {
                    k_0 = k_0.wrapping_add(1);
                }
            }
            if found_1 {
                bk_push(baseValues, &[bk_ptr(p16, bk_new_Block(&[bk_int(b16, 1 as u32), bk_int(b16, ((*(*entry_0).baseValues.offset(foundIndex as isize)).coordinate as i16
                            as ::core::ffi::c_int) as u32)]))]);
            } else {
                bk_push(baseValues, &[bk_ptr(p16, bk_new_Block(&[bk_int(b16, 1 as u32), bk_int(b16, 0 as u32)]))]);
            }
            m_0 = m_0.wrapping_add(1);
        }
        let mut scriptRecord: *mut bk_Block = bk_new_Block(&[bk_ptr(p16, baseValues), bk_ptr(p16, ::core::ptr::null_mut()), bk_int(b16, 0 as u32)]);
        bk_push(baseScriptList, &[bk_int(b32, ((*entry_0).tag) as u32), bk_ptr(p16, scriptRecord)]);
        j_1 = j_1.wrapping_add(1);
    }
    free(taglist.items as *mut ::core::ffi::c_void);
    taglist.items = ::core::ptr::null_mut::<u32>();
    return bk_new_Block(&[bk_ptr(p16, baseTagList), bk_ptr(p16, baseScriptList)]);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_buildBASE(
    mut base: *const table_BASE,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if base.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut root: *mut bk_Block = bk_new_Block(&[bk_int(b32, 0x10000 as u32), bk_ptr(p16, axisToBk((*base).horizontal)), bk_ptr(p16, axisToBk((*base).vertical))]);
    return bk_build_Block(root);
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 != json_object
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
unsafe extern "C" fn json_obj_getstr_share(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut v: *mut json_value = json_obj_get_type(obj, key, json_string);
    if v.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        return (*v).u.string.ptr;
    };
}
#[inline]
unsafe extern "C" fn json_object_push_tag(
    mut a: *mut json_value,
    mut tag: u32,
    mut b: *mut json_value,
) -> *mut json_value {
    let mut tags: [::core::ffi::c_char; 4] = [
        ((tag & 0xff000000 as u32) >> 24 as ::core::ffi::c_int) as ::core::ffi::c_char,
        ((tag & 0xff0000 as u32) >> 16 as ::core::ffi::c_int) as ::core::ffi::c_char,
        ((tag & 0xff00 as u32) >> 8 as ::core::ffi::c_int) as ::core::ffi::c_char,
        (tag & 0xff as u32) as ::core::ffi::c_char,
    ];
    return json_object_push_length(
        a,
        4 as ::core::ffi::c_uint,
        &raw mut tags as *mut ::core::ffi::c_char,
        b,
    );
}
#[inline]
unsafe extern "C" fn json_numof(mut cv: *const json_value) -> ::core::ffi::c_double {
    if !cv.is_null()
        && (*cv).type_0 == json_integer
    {
        return (*cv).u.integer as ::core::ffi::c_double;
    }
    if !cv.is_null()
        && (*cv).type_0 == json_double
    {
        return (*cv).u.dbl;
    }
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
}
#[inline]
unsafe extern "C" fn json_new_position(mut z: pos_t) -> *mut json_value {
    if round(z as ::core::ffi::c_double) == z {
        return json_integer_new(z as i64);
    } else {
        return json_double_new(z as ::core::ffi::c_double);
    };
}
#[inline]
unsafe extern "C" fn tag2str(mut tag: u32, mut tags: *mut ::core::ffi::c_char) {
    *tags.offset(0 as ::core::ffi::c_int as isize) =
        (tag >> 24 as ::core::ffi::c_int & 0xff as u32) as ::core::ffi::c_char;
    *tags.offset(1 as ::core::ffi::c_int as isize) =
        (tag >> 16 as ::core::ffi::c_int & 0xff as u32) as ::core::ffi::c_char;
    *tags.offset(2 as ::core::ffi::c_int as isize) =
        (tag >> 8 as ::core::ffi::c_int & 0xff as u32) as ::core::ffi::c_char;
    *tags.offset(3 as ::core::ffi::c_int as isize) =
        (tag & 0xff as u32) as ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn str2tag(mut tags: *const ::core::ffi::c_char) -> u32 {
    if tags.is_null() {
        return 0 as u32;
    }
    let mut tag: u32 = 0 as u32;
    let mut len: u8 = 0 as u8;
    while *tags as ::core::ffi::c_int != 0 && (len as ::core::ffi::c_int) < 4 as ::core::ffi::c_int
    {
        tag = tag << 8 as ::core::ffi::c_int | *tags as u32;
        tags = tags.offset(1);
        len = len.wrapping_add(1);
    }
    while (len as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        tag = tag << 8 as ::core::ffi::c_int | ' ' as i32 as u32;
        len = len.wrapping_add(1);
    }
    return tag;
}
