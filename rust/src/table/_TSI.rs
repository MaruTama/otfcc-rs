use libc::{free, malloc, memcpy, memset, qsort, strcmp};
extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn bufwrite_sds(buf: *mut caryll_Buffer, str: sds);
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new_length(
        length: ::core::ffi::c_uint,
        _: *const ::core::ffi::c_char,
    ) -> *mut json_value;
}
use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_copy, otfcc_Handle_dispose, otfcc_Handle_empty, otfcc_Handle_init, otfcc_Handle, otfcc_GlyphHandle, HANDLE_STATE_EMPTY};
use crate::support::binio::{read_16u, read_32u};
use crate::support::cvec::{
    cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push,
    cvec_resize_to, CVecRaw,
};
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
    pub integer: i64,
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
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: u8,
    pub alloc: u8,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: u16,
    pub alloc: u16,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: u32,
    pub alloc: u32,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: u64,
    pub alloc: u64,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: usize,
    pub size: usize,
    pub free: usize,
    pub data: *mut u8,
}
pub type glyphid_t = u16;
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
            u8,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, u8, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, u8) -> ()>,
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
pub type tsi_EntryType = ::core::ffi::c_uint;
pub const TSI_RESERVED_FFFC: tsi_EntryType = 4;
pub const TSI_CVT: tsi_EntryType = 3;
pub const TSI_PREP: tsi_EntryType = 2;
pub const TSI_FPGM: tsi_EntryType = 1;
pub const TSI_GLYPH: tsi_EntryType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tsi_Entry {
    pub type_0: tsi_EntryType,
    pub glyph: otfcc_GlyphHandle,
    pub content: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_tsi_Entry {
    pub init: Option<unsafe extern "C" fn(*mut tsi_Entry) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut tsi_Entry, *const tsi_Entry) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut tsi_Entry, *mut tsi_Entry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut tsi_Entry) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut tsi_Entry, tsi_Entry) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut tsi_Entry, tsi_Entry) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_TSI {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut tsi_Entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_table_TSI {
    pub init: Option<unsafe extern "C" fn(*mut table_TSI) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_TSI, *const table_TSI) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_TSI, *mut table_TSI) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_TSI) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_TSI, table_TSI) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_TSI, table_TSI) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_TSI>,
    pub free: Option<unsafe extern "C" fn(*mut table_TSI) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut table_TSI, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_TSI, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut table_TSI>,
    pub fill: Option<unsafe extern "C" fn(*mut table_TSI, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_TSI) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_TSI, tsi_Entry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_TSI) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_TSI) -> tsi_Entry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_TSI, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut table_TSI,
            Option<unsafe extern "C" fn(*const tsi_Entry, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut table_TSI,
            Option<unsafe extern "C" fn(*const tsi_Entry, *const tsi_Entry) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tsi_BuildTarget {
    pub indexPart: *mut caryll_Buffer,
    pub textPart: *mut caryll_Buffer,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SDS_TYPE_5: ::core::ffi::c_int = 0;
pub const SDS_TYPE_8: ::core::ffi::c_int = 1;
pub const SDS_TYPE_16: ::core::ffi::c_int = 2;
pub const SDS_TYPE_32: ::core::ffi::c_int = 3;
pub const SDS_TYPE_64: ::core::ffi::c_int = 4;
pub const SDS_TYPE_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SDS_TYPE_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
#[inline]
unsafe extern "C" fn initTSIEntry(mut entry: *mut tsi_Entry) {
    otfcc_Handle_init(&raw mut (*entry).glyph);
    (*entry).type_0 = TSI_GLYPH;
    (*entry).content = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[inline]
unsafe extern "C" fn copyTSIEntry(mut dst: *mut tsi_Entry, mut src: *const tsi_Entry) {
    otfcc_Handle_copy(
        &raw mut (*dst).glyph,
        &raw const (*src).glyph,
    );
    (*dst).type_0 = (*src).type_0;
    (*dst).content = sdsdup((*src).content);
}
#[inline]
unsafe extern "C" fn disposeTSIEntry(mut entry: *mut tsi_Entry) {
    otfcc_Handle_dispose(&raw mut (*entry).glyph);
    sdsfree((*entry).content);
}
#[inline]
unsafe extern "C" fn tsi_Entry_replace(mut dst: *mut tsi_Entry, src: tsi_Entry) {
    tsi_Entry_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<tsi_Entry>() as usize,
    );
}
#[inline]
unsafe extern "C" fn tsi_Entry_init(mut x: *mut tsi_Entry) {
    initTSIEntry(x);
}
#[inline]
unsafe extern "C" fn tsi_Entry_move(mut dst: *mut tsi_Entry, mut src: *mut tsi_Entry) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<tsi_Entry>() as usize,
    );
    tsi_Entry_init(src);
}
#[no_mangle]
pub static mut tsi_iEntry: __caryll_elementinterface_tsi_Entry = {
    __caryll_elementinterface_tsi_Entry {
        init: Some(tsi_Entry_init as unsafe extern "C" fn(*mut tsi_Entry) -> ()),
        copy: Some(tsi_Entry_copy as unsafe extern "C" fn(*mut tsi_Entry, *const tsi_Entry) -> ()),
        move_0: Some(tsi_Entry_move as unsafe extern "C" fn(*mut tsi_Entry, *mut tsi_Entry) -> ()),
        dispose: Some(tsi_Entry_dispose as unsafe extern "C" fn(*mut tsi_Entry) -> ()),
        replace: Some(tsi_Entry_replace as unsafe extern "C" fn(*mut tsi_Entry, tsi_Entry) -> ()),
        copyReplace: Some(
            tsi_Entry_copyReplace as unsafe extern "C" fn(*mut tsi_Entry, tsi_Entry) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn tsi_Entry_dispose(mut x: *mut tsi_Entry) {
    disposeTSIEntry(x);
}
#[inline]
unsafe extern "C" fn tsi_Entry_copy(mut dst: *mut tsi_Entry, mut src: *const tsi_Entry) {
    copyTSIEntry(dst, src);
}
#[inline]
unsafe extern "C" fn tsi_Entry_copyReplace(mut dst: *mut tsi_Entry, src: tsi_Entry) {
    tsi_Entry_dispose(dst);
    tsi_Entry_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_TSI_fill(mut arr: *mut table_TSI, mut n: usize) {
    while (*arr).length < n {
        let mut x: tsi_Entry = tsi_Entry {
            type_0: TSI_GLYPH,
            glyph: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            content: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        if tsi_iEntry.init.is_some() {
            tsi_iEntry.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<tsi_Entry>() as usize,
            );
        }
        table_TSI_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_TSI_move(dst: *mut table_TSI, src: *mut table_TSI) {
    cvec_move(table_TSI_as_cvec(dst), table_TSI_as_cvec(src));
}
#[inline]
unsafe fn table_TSI_as_cvec(arr: *mut table_TSI) -> *mut CVecRaw<tsi_Entry> {
    arr as *mut CVecRaw<tsi_Entry>
}
#[inline]
unsafe extern "C" fn table_TSI_init(arr: *mut table_TSI) {
    cvec_init(table_TSI_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_TSI_filterEnv(
    mut arr: *mut table_TSI,
    mut fn_0: Option<unsafe extern "C" fn(*const tsi_Entry, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut tsi_Entry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if tsi_iEntry.dispose.is_some() {
                tsi_iEntry.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut tsi_Entry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn table_TSI_disposeItem(mut arr: *mut table_TSI, mut n: usize) {
    if tsi_iEntry.dispose.is_some() {
        tsi_iEntry.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut tsi_Entry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_TSI_sort(
    mut arr: *mut table_TSI,
    mut fn_0: Option<
        unsafe extern "C" fn(*const tsi_Entry, *const tsi_Entry) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<tsi_Entry>() as usize,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const tsi_Entry, *const tsi_Entry) -> ::core::ffi::c_int>,
            __compar_fn_t,
        >(fn_0),
    );
}
#[no_mangle]
pub static mut table_iTSI: __caryll_vectorinterface_table_TSI = {
    __caryll_vectorinterface_table_TSI {
        init: Some(table_TSI_init as unsafe extern "C" fn(*mut table_TSI) -> ()),
        copy: Some(table_TSI_copy as unsafe extern "C" fn(*mut table_TSI, *const table_TSI) -> ()),
        move_0: Some(table_TSI_move as unsafe extern "C" fn(*mut table_TSI, *mut table_TSI) -> ()),
        dispose: Some(table_TSI_dispose as unsafe extern "C" fn(*mut table_TSI) -> ()),
        replace: Some(table_TSI_replace as unsafe extern "C" fn(*mut table_TSI, table_TSI) -> ()),
        copyReplace: Some(
            table_TSI_copyReplace as unsafe extern "C" fn(*mut table_TSI, table_TSI) -> (),
        ),
        create: Some(table_TSI_create),
        free: Some(table_TSI_free as unsafe extern "C" fn(*mut table_TSI) -> ()),
        initN: Some(table_TSI_initN as unsafe extern "C" fn(*mut table_TSI, usize) -> ()),
        initCapN: Some(table_TSI_initCapN as unsafe extern "C" fn(*mut table_TSI, usize) -> ()),
        createN: Some(table_TSI_createN as unsafe extern "C" fn(usize) -> *mut table_TSI),
        fill: Some(table_TSI_fill as unsafe extern "C" fn(*mut table_TSI, usize) -> ()),
        clear: Some(table_TSI_dispose as unsafe extern "C" fn(*mut table_TSI) -> ()),
        push: Some(table_TSI_push as unsafe extern "C" fn(*mut table_TSI, tsi_Entry) -> ()),
        shrinkToFit: Some(table_TSI_shrinkToFit as unsafe extern "C" fn(*mut table_TSI) -> ()),
        pop: Some(table_TSI_pop as unsafe extern "C" fn(*mut table_TSI) -> tsi_Entry),
        disposeItem: Some(
            table_TSI_disposeItem as unsafe extern "C" fn(*mut table_TSI, usize) -> (),
        ),
        filterEnv: Some(
            table_TSI_filterEnv
                as unsafe extern "C" fn(
                    *mut table_TSI,
                    Option<unsafe extern "C" fn(*const tsi_Entry, *mut ::core::ffi::c_void) -> bool>,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_TSI_sort
                as unsafe extern "C" fn(
                    *mut table_TSI,
                    Option<
                        unsafe extern "C" fn(
                            *const tsi_Entry,
                            *const tsi_Entry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn table_TSI_push(arr: *mut table_TSI, elem: tsi_Entry) {
    cvec_push(table_TSI_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_TSI_grow(arr: *mut table_TSI) {
    cvec_grow(table_TSI_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_TSI_growTo(arr: *mut table_TSI, target: usize) {
    cvec_grow_to(table_TSI_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_TSI_pop(arr: *mut table_TSI) -> tsi_Entry {
    cvec_pop(table_TSI_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_TSI_copyReplace(mut dst: *mut table_TSI, src: table_TSI) {
    table_TSI_dispose(dst);
    table_TSI_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_TSI_copy(mut dst: *mut table_TSI, mut src: *const table_TSI) {
    table_TSI_init(dst);
    table_TSI_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if tsi_iEntry.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            tsi_iEntry.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut tsi_Entry,
                (*src).items.offset(j as isize) as *mut tsi_Entry as *const tsi_Entry,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn table_TSI_dispose(mut arr: *mut table_TSI) {
    if arr.is_null() {
        return;
    }
    if tsi_iEntry.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            tsi_iEntry.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut tsi_Entry
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<tsi_Entry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn table_TSI_replace(mut dst: *mut table_TSI, src: table_TSI) {
    table_TSI_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_TSI>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_TSI_initCapN(mut arr: *mut table_TSI, mut n: usize) {
    table_TSI_init(arr);
    table_TSI_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn table_TSI_growToN(arr: *mut table_TSI, target: usize) {
    cvec_grow_to_n(table_TSI_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_TSI_initN(mut arr: *mut table_TSI, mut n: usize) {
    table_TSI_init(arr);
    table_TSI_growToN(arr, n);
    table_TSI_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_TSI_free(mut x: *mut table_TSI) {
    if x.is_null() {
        return;
    }
    table_TSI_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_TSI_createN(mut n: usize) -> *mut table_TSI {
    let mut t: *mut table_TSI =
        malloc(::core::mem::size_of::<table_TSI>() as usize) as *mut table_TSI;
    table_TSI_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_TSI_create() -> *mut table_TSI {
    let mut x: *mut table_TSI =
        malloc(::core::mem::size_of::<table_TSI>() as usize) as *mut table_TSI;
    table_TSI_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_TSI_shrinkToFit(mut arr: *mut table_TSI) {
    table_TSI_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_TSI_resizeTo(arr: *mut table_TSI, target: usize) {
    cvec_resize_to(table_TSI_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn isValidGID(mut gid: u16, mut tagIndex: u32) -> bool {
    if tagIndex == 1414744368i32 as u32 {
        return gid as ::core::ffi::c_int != 0xfffe as ::core::ffi::c_int
            && gid as ::core::ffi::c_int != 0xfffc as ::core::ffi::c_int;
    } else {
        return (gid as ::core::ffi::c_int) < 0xfffa as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readTSI(
    packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
    mut tagIndex: u32,
    mut tagText: u32,
) -> *mut table_TSI {
    let mut textPart: otfcc_PacketPiece = otfcc_PacketPiece {
        tag: 0,
        checkSum: 0,
        offset: 0,
        length: 0,
        data: ::core::ptr::null_mut::<u8>(),
    };
    textPart.tag = 0 as u32;
    let mut indexPart: otfcc_PacketPiece = otfcc_PacketPiece {
        tag: 0,
        checkSum: 0,
        offset: 0,
        length: 0,
        data: ::core::ptr::null_mut::<u8>(),
    };
    indexPart.tag = 0 as u32;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut tableIx: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if tableIx.tag == tagIndex {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    indexPart = tableIx;
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    let mut __fortable_keep_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound_0 != 0
        && __fortable_keep_0 != 0
        && __fortable_count_0 < packet.numTables as ::core::ffi::c_int
    {
        let mut tableTx: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count_0 as isize);
        while __fortable_keep_0 != 0 {
            if tableTx.tag == tagText {
                let mut __fortable_k2_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2_0 != 0 {
                    textPart = tableTx;
                    __fortable_k2_0 = 0 as ::core::ffi::c_int;
                    __notfound_0 = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep_0 = (__fortable_keep_0 == 0) as ::core::ffi::c_int;
        }
        __fortable_keep_0 = (__fortable_keep_0 == 0) as ::core::ffi::c_int;
        __fortable_count_0 += 1;
    }
    if textPart.tag == 0 || indexPart.tag == 0 {
        return ::core::ptr::null_mut::<table_TSI>();
    }
    let mut tsi: *mut table_TSI = (
        table_iTSI.create.expect("non-null function pointer"))();
    let mut j: u32 = 0 as u32;
    while j.wrapping_mul(8 as u32) < indexPart.length {
        let mut gid: u16 = read_16u(
            indexPart
                .data
                .offset(j.wrapping_mul(8 as u32) as isize),
        );
        let mut textLength: u32 = read_16u(
            indexPart
                .data
                .offset(j.wrapping_mul(8 as u32) as isize)
                .offset(2 as ::core::ffi::c_int as isize),
        ) as u32;
        let mut textOffset: u32 = read_32u(
            indexPart
                .data
                .offset(j.wrapping_mul(8 as u32) as isize)
                .offset(4 as ::core::ffi::c_int as isize),
        );
        if !(!isValidGID(gid, tagIndex) || textOffset >= textPart.length || textLength == 0) {
            let mut predictedTextLength: u32 = textPart.length.wrapping_sub(textOffset);
            let mut k: glyphid_t = j.wrapping_add(1 as u32) as glyphid_t;
            while ((k as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as u32)
                < indexPart.length
            {
                let mut gidK: u16 = read_16u(
                    indexPart
                        .data
                        .offset((k as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize),
                );
                let mut textOffsetK: u32 = read_32u(
                    indexPart
                        .data
                        .offset((k as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize),
                );
                if gidK as ::core::ffi::c_int != 0xfffe as ::core::ffi::c_int
                    && textOffsetK < textPart.length
                    && textOffsetK > textOffset
                {
                    predictedTextLength = textOffsetK.wrapping_sub(textOffset);
                    break;
                } else {
                    k = k.wrapping_add(1);
                }
            }
            if textLength >= 0x8000 as u32 {
                textLength = predictedTextLength;
            }
            let mut entry: tsi_Entry = tsi_Entry {
                type_0: TSI_GLYPH,
                glyph: otfcc_Handle {
                    state: HANDLE_STATE_EMPTY,
                    index: 0,
                    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                },
                content: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            };
            match gid as ::core::ffi::c_int {
                65530 => {
                    entry.type_0 = TSI_PREP;
                    otfcc_Handle_init(&raw mut entry.glyph);
                }
                65531 => {
                    entry.type_0 = TSI_CVT;
                    otfcc_Handle_init(&raw mut entry.glyph);
                }
                65533 => {
                    entry.type_0 = TSI_FPGM;
                    otfcc_Handle_init(&raw mut entry.glyph);
                }
                _ => {
                    entry.type_0 = TSI_GLYPH;
                    entry.glyph = handle_fromIndex(
                        gid as glyphid_t,
                    ) as otfcc_GlyphHandle;
                }
            }
            entry.content = sdsnewlen(
                textPart.data.offset(textOffset as isize) as *const ::core::ffi::c_void,
                textLength as usize,
            );
            table_iTSI.push.expect("non-null function pointer")(tsi, entry);
        }
        j = j.wrapping_add(1);
    }
    return tsi;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpTSI(
    mut tsi: *const table_TSI,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) {
    if tsi.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            tag,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _tsi: *mut json_value = json_object_new(2 as usize);
        let mut _glyphs: *mut json_value = json_object_new((*tsi).length);
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < (*tsi).length {
            let mut entry: *mut tsi_Entry = (*tsi).items.offset(__caryll_index as isize);
            while keep != 0 {
                if !((*entry).type_0 as ::core::ffi::c_uint
                    != TSI_GLYPH as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    json_object_push(
                        _glyphs,
                        (*entry).glyph.name as *const ::core::ffi::c_char,
                        json_string_new_length(
                            sdslen((*entry).content) as ::core::ffi::c_uint,
                            (*entry).content as *const ::core::ffi::c_char,
                        ),
                    );
                }
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        let mut _extra: *mut json_value = json_object_new((*tsi).length);
        let mut __caryll_index_0: usize = 0 as usize;
        let mut keep_0: usize = 1 as usize;
        while keep_0 != 0 && __caryll_index_0 < (*tsi).length {
            let mut entry_0: *mut tsi_Entry = (*tsi).items.offset(__caryll_index_0 as isize);
            while keep_0 != 0 {
                if !((*entry_0).type_0 as ::core::ffi::c_uint
                    == TSI_GLYPH as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    let mut extraKey: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    match (*entry_0).type_0 as ::core::ffi::c_uint {
                        3 => {
                            extraKey = b"cvt\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        1 => {
                            extraKey = b"fpgm\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        2 => {
                            extraKey = b"prep\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        _ => {
                            extraKey = b"reserved\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                    }
                    json_object_push(
                        _extra,
                        extraKey,
                        json_string_new_length(
                            sdslen((*entry_0).content) as ::core::ffi::c_uint,
                            (*entry_0).content as *const ::core::ffi::c_char,
                        ),
                    );
                }
                keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
            }
            keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
            __caryll_index_0 = __caryll_index_0.wrapping_add(1);
        }
        json_object_push(
            _tsi,
            b"glyphs\0" as *const u8 as *const ::core::ffi::c_char,
            _glyphs,
        );
        json_object_push(
            _tsi,
            b"extra\0" as *const u8 as *const ::core::ffi::c_char,
            _extra,
        );
        json_object_push(root, tag, _tsi);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseTSI(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut table_TSI {
    let mut _tsi: *mut json_value = ::core::ptr::null_mut::<json_value>();
    _tsi = json_obj_get_type(root, tag, json_object);
    if _tsi.is_null() {
        return ::core::ptr::null_mut::<table_TSI>();
    }
    let mut tsi: *mut table_TSI = (
        table_iTSI.create.expect("non-null function pointer"))();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            tag,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _glyphs: *mut json_value = json_obj_get_type(
            _tsi,
            b"glyphs\0" as *const u8 as *const ::core::ffi::c_char,
            json_object,
        );
        if !_glyphs.is_null() {
            let mut j: u32 = 0 as u32;
            while j < (*_glyphs).u.object.length as u32 {
                let mut _gid: *mut ::core::ffi::c_char =
                    (*(*_glyphs).u.object.values.offset(j as isize)).name;
                let mut _gidlen: usize =
                    (*(*_glyphs).u.object.values.offset(j as isize)).name_length as usize;
                let mut _content: *mut json_value =
                    (*(*_glyphs).u.object.values.offset(j as isize)).value as *mut json_value;
                if !(_content.is_null()
                    || (*_content).type_0 as ::core::ffi::c_uint
                        != json_string as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    table_iTSI.push.expect("non-null function pointer")(
                        tsi,
                        tsi_Entry {
                            type_0: TSI_GLYPH,
                            glyph: handle_fromName(
                                sdsnewlen(_gid as *const ::core::ffi::c_void, _gidlen),
                            ) as otfcc_GlyphHandle,
                            content: sdsnewlen(
                                (*_content).u.string.ptr as *const ::core::ffi::c_void,
                                (*_content).u.string.length as usize,
                            ),
                        },
                    );
                }
                j = j.wrapping_add(1);
            }
        }
        let mut _extra: *mut json_value = json_obj_get_type(
            _tsi,
            b"extra\0" as *const u8 as *const ::core::ffi::c_char,
            json_object,
        );
        if !_extra.is_null() {
            let mut j_0: u32 = 0 as u32;
            while j_0 < (*_extra).u.object.length as u32 {
                let mut _key: *mut ::core::ffi::c_char =
                    (*(*_extra).u.object.values.offset(j_0 as isize)).name;
                let mut _content_0: *mut json_value =
                    (*(*_extra).u.object.values.offset(j_0 as isize)).value as *mut json_value;
                if !(_content_0.is_null()
                    || (*_content_0).type_0 as ::core::ffi::c_uint
                        != json_string as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    if strcmp(_key, b"cvt\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        table_iTSI.push.expect("non-null function pointer")(
                            tsi,
                            tsi_Entry {
                                type_0: TSI_CVT,
                                glyph: otfcc_Handle_empty() as otfcc_GlyphHandle,
                                content: sdsnewlen(
                                    (*_content_0).u.string.ptr as *const ::core::ffi::c_void,
                                    (*_content_0).u.string.length as usize,
                                ),
                            },
                        );
                    } else if strcmp(_key, b"fpgm\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        table_iTSI.push.expect("non-null function pointer")(
                            tsi,
                            tsi_Entry {
                                type_0: TSI_FPGM,
                                glyph: otfcc_Handle_empty() as otfcc_GlyphHandle,
                                content: sdsnewlen(
                                    (*_content_0).u.string.ptr as *const ::core::ffi::c_void,
                                    (*_content_0).u.string.length as usize,
                                ),
                            },
                        );
                    } else if strcmp(_key, b"prep\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        table_iTSI.push.expect("non-null function pointer")(
                            tsi,
                            tsi_Entry {
                                type_0: TSI_PREP,
                                glyph: otfcc_Handle_empty() as otfcc_GlyphHandle,
                                content: sdsnewlen(
                                    (*_content_0).u.string.ptr as *const ::core::ffi::c_void,
                                    (*_content_0).u.string.length as usize,
                                ),
                            },
                        );
                    }
                }
                j_0 = j_0.wrapping_add(1);
            }
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
    return tsi;
}
unsafe extern "C" fn propergid(mut entry: *mut tsi_Entry, type_0: tsi_EntryType) -> glyphid_t {
    match type_0 as ::core::ffi::c_uint {
        3 => return 0xfffb as glyphid_t,
        1 => return 0xfffd as glyphid_t,
        2 => return 0xfffa as glyphid_t,
        4 => return 0xfffc as glyphid_t,
        0 => return (*entry).glyph.index,
        _ => {}
    }
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn pushTSIEntries(
    mut target: *mut tsi_BuildTarget,
    mut tsi: *const table_TSI,
    type_0: tsi_EntryType,
    minN: glyphid_t,
) {
    let mut itemsPushed: glyphid_t = 0 as glyphid_t;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*tsi).length {
        let mut entry: *mut tsi_Entry = (*tsi).items.offset(__caryll_index as isize);
        while keep != 0 {
            if !((*entry).type_0 as ::core::ffi::c_uint != type_0 as ::core::ffi::c_uint) {
                let mut lengthSofar: usize = (*(*target).textPart).cursor;
                bufwrite_sds((*target).textPart, (*entry).content);
                let mut lengthAfter: usize = (*(*target).textPart).cursor;
                bufwrite16b((*target).indexPart, propergid(entry, type_0) as u16);
                if lengthAfter.wrapping_sub(lengthSofar) < 0x8000 as usize {
                    bufwrite16b(
                        (*target).indexPart,
                        lengthAfter.wrapping_sub(lengthSofar) as u16,
                    );
                } else {
                    bufwrite16b((*target).indexPart, 0x8000 as u16);
                }
                bufwrite32b((*target).indexPart, lengthSofar as u32);
                itemsPushed =
                    (itemsPushed as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    while (itemsPushed as ::core::ffi::c_int) < minN as ::core::ffi::c_int {
        bufwrite16b(
            (*target).indexPart,
            propergid(::core::ptr::null_mut::<tsi_Entry>(), type_0) as u16,
        );
        bufwrite16b((*target).indexPart, 0 as u16);
        bufwrite32b(
            (*target).indexPart,
            (*(*target).textPart).cursor as u32,
        );
        itemsPushed = (itemsPushed as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as glyphid_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildTSI(
    mut tsi: *const table_TSI,
    mut _options: *const otfcc_Options,
) -> tsi_BuildTarget {
    let mut target: tsi_BuildTarget = tsi_BuildTarget {
        indexPart: ::core::ptr::null_mut::<caryll_Buffer>(),
        textPart: ::core::ptr::null_mut::<caryll_Buffer>(),
    };
    if tsi.is_null() {
        target.textPart = ::core::ptr::null_mut::<caryll_Buffer>();
        target.indexPart = ::core::ptr::null_mut::<caryll_Buffer>();
    } else {
        target.textPart = bufnew();
        target.indexPart = bufnew();
        pushTSIEntries(&raw mut target, tsi, TSI_GLYPH, 0 as glyphid_t);
        bufwrite16b(target.indexPart, 0xfffe as u16);
        bufwrite16b(target.indexPart, 0 as u16);
        bufwrite32b(target.indexPart, 0xabfc1f34 as u32);
        pushTSIEntries(&raw mut target, tsi, TSI_PREP, 1 as glyphid_t);
        pushTSIEntries(&raw mut target, tsi, TSI_CVT, 1 as glyphid_t);
        pushTSIEntries(&raw mut target, tsi, TSI_RESERVED_FFFC, 1 as glyphid_t);
        pushTSIEntries(&raw mut target, tsi, TSI_FPGM, 1 as glyphid_t);
    }
    return target;
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
