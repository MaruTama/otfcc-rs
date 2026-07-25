#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memcpy, memset, strcmp, strlen};
unsafe extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscat(s: sds, t: *const ::core::ffi::c_char) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn bufwrite_sds(buf: *mut caryll_Buffer, str: sds);
    fn bufwrite_bufdel(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
    fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double;
    fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16;
    static iVQ: __caryll_vectorinterface_VQ;
    static glyf_iPoint: __caryll_elementinterface_glyf_Point;
    static glyf_iContour: __caryll_vectorinterface_glyf_Contour;
    static glyf_iContourList: __caryll_vectorinterface_glyf_ContourList;
    static glyf_iStemDefList: __caryll_vectorinterface_glyf_StemDefList;
    static glyf_iMaskList: __caryll_vectorinterface_glyf_MaskList;
    static table_iGlyf: __caryll_vectorinterface_table_glyf;
    fn otfcc_newGlyf_glyph() -> *mut glyf_Glyph;
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
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
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
    fn json_boolean_new(_: ::core::ffi::c_int) -> *mut json_value;
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn json_new_VQ(z: VQ, fvar: *const table_fvar) -> *mut json_value;
    fn cffnum(v: cff_Value) -> ::core::ffi::c_double;
    static cff_iIndex: __caryll_elementinterface_cff_Index;
    static cff_iDict: __caryll_elementinterface_cff_Dict;
    fn cff_build_Charset(cset: cff_Charset) -> *mut caryll_Buffer;
    fn cff_close_FDSelect(fds: cff_FDSelect);
    fn cff_build_FDSelect(fd: cff_FDSelect) -> *mut caryll_Buffer;
    fn sdsget_cff_sid(idx: u16, str: cff_Index) -> sds;
    fn cff_encodeCffOperator(val: i32) -> *mut caryll_Buffer;
    fn cff_buildOffset(val: i32) -> *mut caryll_Buffer;
    fn cff_buildHeader() -> *mut caryll_Buffer;
    fn cff_parseSubr(
        idx: u16,
        raw: *mut u8,
        fdarray: cff_Index,
        select: cff_FDSelect,
        subr: *mut cff_Index,
    ) -> u8;
    fn cff_parseOutline(
        data: *mut u8,
        len: u32,
        gsubr: cff_Index,
        lsubr: cff_Index,
        stack: *mut cff_Stack,
        outline: *mut ::core::ffi::c_void,
        methods: cff_IOutlineBuilder,
        options: *const otfcc_Options,
    );
    fn cff_openStream(
        data: *mut u8,
        len: u32,
        options: *const otfcc_Options,
    ) -> *mut cff_File;
    fn cff_close(file: *mut cff_File);
    fn cff_compileGlyphToIL(
        g: *mut glyf_Glyph,
        defaultWidth: u16,
        nominalWidth: u16,
    ) -> *mut cff_CharstringIL;
    fn cff_optimizeIL(il: *mut cff_CharstringIL, options: *const otfcc_Options);
    static cff_iSubrGraph: __caryll_elementinterface_cff_SubrGraph;
    fn cff_insertILToGraph(g: *mut cff_SubrGraph, il: *mut cff_CharstringIL);
    fn cff_ilGraphToBuffers(
        g: *mut cff_SubrGraph,
        s: *mut *mut caryll_Buffer,
        gs: *mut *mut caryll_Buffer,
        ls: *mut *mut caryll_Buffer,
        options: *const otfcc_Options,
    );
}

use crate::support::handle::{handle_fromIndex, otfcc_FDHandle};

use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::logger::{otfcc_ILogger};
use crate::support::buffer::{bufninit, caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{arity_t, cffsid_t, f16dot16, font_file_pointer, glyphid_t, pos_t, scale_t, shapeid_t, tableid_t};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, sds, sdshdr16, sdshdr32, sdshdr64, sdshdr8};
use crate::vendor::json::{json_array, json_double, json_integer, json_object, json_string, json_type, json_value};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};
use crate::libcff::{cff_File, cff_IOutlineBuilder, cff_Stack, op_BlueFuzz, op_BlueScale, op_BlueShift, op_BlueValues, op_CIDCount, op_CIDFontRevision, op_CIDFontVersion, op_CharStrings, op_Copyright, op_ExpansionFactor, op_FDArray, op_FDSelect, op_FamilyBlues, op_FamilyName, op_FamilyOtherBlues, op_FontBBox, op_FontMatrix, op_FontName, op_ForceBold, op_FullName, op_ItalicAngle, op_LanguageGroup, op_Notice, op_OtherBlues, op_Private, op_ROS, op_StdHW, op_StdVW, op_StemSnapH, op_StemSnapV, op_StrokeWidth, op_Subrs, op_UIDBase, op_UnderlinePosition, op_UnderlineThickness, op_Weight, op_charset, op_defaultWidthX, op_initialRandomSeed, op_isFixedPitch, op_nominalWidthX, op_version};
use crate::libcff::cff_charset::{cff_CHARSET_FORMAT2, cff_CHARSET_ISOADOBE, cff_Charset, cff_CharsetRangeFormat2};
use crate::libcff::cff_dict::{__caryll_elementinterface_cff_Dict, cff_Dict, cff_DictEntry};
use crate::libcff::cff_fdselect::{cff_FDSELECT_FORMAT3, cff_FDSELECT_UNSPECED, cff_FDSelect, cff_FDSelectRangeFormat3};
use crate::libcff::cff_index::{CFF_INDEX_16, __caryll_elementinterface_cff_Index, cff_Index};
use crate::libcff::cff_value::{cff_DOUBLE, cff_INTEGER, cff_Value, cff_ValueBody, cff_Value_Type};
use crate::libcff::charstring_il::{cff_CharstringIL, cff_CharstringInstruction};
use crate::libcff::subr::{__caryll_elementinterface_cff_SubrGraph, cff_SubrDiagramIndex, cff_SubrGraph, cff_SubrRule};
use crate::support::{NULL, false_0, true_0};
use crate::table::fvar::{table_fvar};
use crate::table::glyf::{__caryll_elementinterface_glyf_Point, __caryll_vectorinterface_glyf_Contour, __caryll_vectorinterface_glyf_ContourList, __caryll_vectorinterface_glyf_MaskList, __caryll_vectorinterface_glyf_StemDefList, __caryll_vectorinterface_table_glyf, glyf_Contour, glyf_Glyph, glyf_GlyphPtr, glyf_MaskList, glyf_Point, glyf_PostscriptHintMask, glyf_PostscriptStemDef, table_glyf};
use crate::table::head::{table_head};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UT_hash_bucket, UT_hash_handle, UT_hash_table};


use crate::vf::vq::{VQ, __caryll_vectorinterface_VQ, vq_SegList, vq_Segment};
use crate::support::json_funcs::{json_obj_getbool};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_FontMatrix {
    pub a: scale_t,
    pub b: scale_t,
    pub c: scale_t,
    pub d: scale_t,
    pub x: VQ,
    pub y: VQ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_PrivateDict {
    pub blueValuesCount: arity_t,
    pub blueValues: *mut ::core::ffi::c_double,
    pub otherBluesCount: arity_t,
    pub otherBlues: *mut ::core::ffi::c_double,
    pub familyBluesCount: arity_t,
    pub familyBlues: *mut ::core::ffi::c_double,
    pub familyOtherBluesCount: arity_t,
    pub familyOtherBlues: *mut ::core::ffi::c_double,
    pub blueScale: ::core::ffi::c_double,
    pub blueShift: ::core::ffi::c_double,
    pub blueFuzz: ::core::ffi::c_double,
    pub stdHW: ::core::ffi::c_double,
    pub stdVW: ::core::ffi::c_double,
    pub stemSnapHCount: arity_t,
    pub stemSnapH: *mut ::core::ffi::c_double,
    pub stemSnapVCount: arity_t,
    pub stemSnapV: *mut ::core::ffi::c_double,
    pub forceBold: bool,
    pub languageGroup: u32,
    pub expansionFactor: ::core::ffi::c_double,
    pub initialRandomSeed: ::core::ffi::c_double,
    pub defaultWidthX: ::core::ffi::c_double,
    pub nominalWidthX: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _table_CFF {
    pub fontName: sds,
    pub isCID: bool,
    pub version: sds,
    pub notice: sds,
    pub copyright: sds,
    pub fullName: sds,
    pub familyName: sds,
    pub weight: sds,
    pub isFixedPitch: bool,
    pub italicAngle: ::core::ffi::c_double,
    pub underlinePosition: ::core::ffi::c_double,
    pub underlineThickness: ::core::ffi::c_double,
    pub fontBBoxTop: ::core::ffi::c_double,
    pub fontBBoxBottom: ::core::ffi::c_double,
    pub fontBBoxLeft: ::core::ffi::c_double,
    pub fontBBoxRight: ::core::ffi::c_double,
    pub strokeWidth: ::core::ffi::c_double,
    pub privateDict: *mut cff_PrivateDict,
    pub fontMatrix: *mut cff_FontMatrix,
    pub cidRegistry: sds,
    pub cidOrdering: sds,
    pub cidSupplement: u32,
    pub cidFontVersion: ::core::ffi::c_double,
    pub cidFontRevision: ::core::ffi::c_double,
    pub cidCount: u32,
    pub UIDBase: u32,
    pub fdArrayCount: tableid_t,
    pub fdArray: *mut *mut table_CFF,
}
pub type table_CFF = _table_CFF;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_CFF {
    pub init: Option<unsafe extern "C" fn(*mut table_CFF) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_CFF, *const table_CFF) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_CFF, *mut table_CFF) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_CFF) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_CFF, table_CFF) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_CFF, table_CFF) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_CFF>,
    pub free: Option<unsafe extern "C" fn(*mut table_CFF) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_CFFAndGlyf {
    pub meta: *mut table_CFF,
    pub glyphs: *mut table_glyf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_extract_context {
    pub fdArrayIndex: i32,
    pub meta: *mut table_CFF,
    pub glyphs: *mut table_glyf,
    pub cffFile: *mut cff_File,
    pub seed: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outline_builder_context {
    pub g: *mut glyf_Glyph,
    pub jContour: shapeid_t,
    pub jPoint: shapeid_t,
    pub defaultWidthX: ::core::ffi::c_double,
    pub nominalWidthX: ::core::ffi::c_double,
    pub definedHStems: u8,
    pub definedVStems: u8,
    pub definedHintMasks: u8,
    pub definedContourMasks: u8,
    pub randx: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cff_DoubleBits {
    pub u: u64,
    pub d: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_sid_entry {
    pub sid: ::core::ffi::c_int,
    pub str_0: *mut ::core::ffi::c_char,
    pub hh: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_charstring_builder_context {
    pub glyf: *mut table_glyf,
    pub defaultWidth: u16,
    pub nominalWidthX: u16,
    pub options: *const otfcc_Options,
    pub graph: cff_SubrGraph,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdarray_compile_context {
    pub fdArray: *mut *mut table_CFF,
    pub stringHash: *mut *mut cff_sid_entry,
}
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
#[unsafe(no_mangle)]
pub static DEFAULT_BLUE_SCALE: ::core::ffi::c_double = 0.039625f64;
#[unsafe(no_mangle)]
pub static DEFAULT_BLUE_SHIFT: ::core::ffi::c_double =
    7 as ::core::ffi::c_int as ::core::ffi::c_double;
#[unsafe(no_mangle)]
pub static DEFAULT_BLUE_FUZZ: ::core::ffi::c_double =
    1 as ::core::ffi::c_int as ::core::ffi::c_double;
#[unsafe(no_mangle)]
pub static DEFAULT_EXPANSION_FACTOR: ::core::ffi::c_double = 0.06f64;
unsafe extern "C" fn otfcc_newCff_private() -> *mut cff_PrivateDict {
    let mut pd: *mut cff_PrivateDict = ::core::ptr::null_mut::<cff_PrivateDict>();
    pd = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_PrivateDict>() as usize,
        15 as ::core::ffi::c_ulong,
    ) as *mut cff_PrivateDict;
    (*pd).blueFuzz = DEFAULT_BLUE_FUZZ;
    (*pd).blueScale = DEFAULT_BLUE_SCALE;
    (*pd).blueShift = DEFAULT_BLUE_SHIFT;
    (*pd).expansionFactor = DEFAULT_EXPANSION_FACTOR;
    return pd;
}
#[inline]
unsafe extern "C" fn initFD(mut fd: *mut table_CFF) {
    memset(
        fd as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_CFF>() as usize,
    );
    (*fd).underlinePosition = -(100 as ::core::ffi::c_int) as ::core::ffi::c_double;
    (*fd).underlineThickness = 50 as ::core::ffi::c_int as ::core::ffi::c_double;
}
unsafe extern "C" fn otfcc_delete_privatedict(mut priv_0: *mut cff_PrivateDict) {
    if priv_0.is_null() {
        return;
    }
    free((*priv_0).blueValues as *mut ::core::ffi::c_void);
    (*priv_0).blueValues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free((*priv_0).otherBlues as *mut ::core::ffi::c_void);
    (*priv_0).otherBlues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free((*priv_0).familyBlues as *mut ::core::ffi::c_void);
    (*priv_0).familyBlues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free((*priv_0).familyOtherBlues as *mut ::core::ffi::c_void);
    (*priv_0).familyOtherBlues = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free((*priv_0).stemSnapH as *mut ::core::ffi::c_void);
    (*priv_0).stemSnapH = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free((*priv_0).stemSnapV as *mut ::core::ffi::c_void);
    (*priv_0).stemSnapV = ::core::ptr::null_mut::<::core::ffi::c_double>();
    free(priv_0 as *mut ::core::ffi::c_void);
    priv_0 = ::core::ptr::null_mut::<cff_PrivateDict>();
}
#[inline]
unsafe extern "C" fn disposeFontMatrix(mut fm: *mut cff_FontMatrix) {
    if fm.is_null() {
        return;
    }
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*fm).x);
    iVQ.dispose.expect("non-null function pointer")(&raw mut (*fm).y);
}
#[inline]
unsafe extern "C" fn disposeFD(mut fd: *mut table_CFF) {
    sdsfree((*fd).version);
    sdsfree((*fd).notice);
    sdsfree((*fd).copyright);
    sdsfree((*fd).fullName);
    sdsfree((*fd).familyName);
    sdsfree((*fd).weight);
    sdsfree((*fd).fontName);
    sdsfree((*fd).cidRegistry);
    sdsfree((*fd).cidOrdering);
    disposeFontMatrix((*fd).fontMatrix);
    free((*fd).fontMatrix as *mut ::core::ffi::c_void);
    (*fd).fontMatrix = ::core::ptr::null_mut::<cff_FontMatrix>();
    otfcc_delete_privatedict((*fd).privateDict);
    if !(*fd).fdArray.is_null() {
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*fd).fdArrayCount as ::core::ffi::c_int {
            table_iCFF.free.expect("non-null function pointer")(*(*fd).fdArray.offset(j as isize));
            j = j.wrapping_add(1);
        }
        free((*fd).fdArray as *mut ::core::ffi::c_void);
        (*fd).fdArray = ::core::ptr::null_mut::<*mut table_CFF>();
    }
}
#[inline]
unsafe extern "C" fn table_CFF_free(mut x: *mut table_CFF) {
    if x.is_null() {
        return;
    }
    table_CFF_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_CFF_replace(mut dst: *mut table_CFF, src: table_CFF) {
    table_CFF_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_CFF>() as usize,
    );
}
#[unsafe(no_mangle)]
pub static table_iCFF: __caryll_elementinterface_table_CFF = {
    __caryll_elementinterface_table_CFF {
        init: Some(table_CFF_init as unsafe extern "C" fn(*mut table_CFF) -> ()),
        copy: Some(table_CFF_copy as unsafe extern "C" fn(*mut table_CFF, *const table_CFF) -> ()),
        move_0: Some(table_CFF_move as unsafe extern "C" fn(*mut table_CFF, *mut table_CFF) -> ()),
        dispose: Some(table_CFF_dispose as unsafe extern "C" fn(*mut table_CFF) -> ()),
        replace: Some(table_CFF_replace as unsafe extern "C" fn(*mut table_CFF, table_CFF) -> ()),
        copyReplace: Some(
            table_CFF_copyReplace as unsafe extern "C" fn(*mut table_CFF, table_CFF) -> (),
        ),
        create: Some(table_CFF_create),
        free: Some(table_CFF_free as unsafe extern "C" fn(*mut table_CFF) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_CFF_move(mut dst: *mut table_CFF, mut src: *mut table_CFF) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_CFF>() as usize,
    );
    table_CFF_init(src);
}
#[inline]
unsafe extern "C" fn table_CFF_create() -> *mut table_CFF {
    let mut x: *mut table_CFF =
        malloc(::core::mem::size_of::<table_CFF>() as usize) as *mut table_CFF;
    table_CFF_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_CFF_init(mut x: *mut table_CFF) {
    initFD(x);
}
#[inline]
unsafe extern "C" fn table_CFF_copyReplace(mut dst: *mut table_CFF, src: table_CFF) {
    table_CFF_dispose(dst);
    table_CFF_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_CFF_dispose(mut x: *mut table_CFF) {
    disposeFD(x);
}
#[inline]
unsafe extern "C" fn table_CFF_copy(mut dst: *mut table_CFF, mut src: *const table_CFF) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_CFF>() as usize,
    );
}
unsafe extern "C" fn callback_extract_private(
    mut op: u32,
    mut top: u8,
    mut stack: *mut cff_Value,
    mut _context: *mut ::core::ffi::c_void,
) {
    let mut context: *mut cff_extract_context = _context as *mut cff_extract_context;
    let mut meta: *mut table_CFF = (*context).meta;
    if (*context).fdArrayIndex >= 0 as i32
        && (*context).fdArrayIndex < (*meta).fdArrayCount as i32
    {
        meta = *(*meta).fdArray.offset((*context).fdArrayIndex as isize);
    }
    let mut pd: *mut cff_PrivateDict = (*meta).privateDict;
    match op {
        6 => {
            (*pd).blueValuesCount = top as arity_t;
            (*pd).blueValues = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                    .wrapping_mul((*pd).blueValuesCount as usize),
                86 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j: arity_t = 0 as arity_t;
            while j < (*pd).blueValuesCount {
                *(*pd).blueValues.offset(j as isize) = cffnum(*stack.offset(j as isize));
                j = j.wrapping_add(1);
            }
        }
        7 => {
            (*pd).otherBluesCount = top as arity_t;
            (*pd).otherBlues = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                    .wrapping_mul((*pd).otherBluesCount as usize),
                94 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j_0: arity_t = 0 as arity_t;
            while j_0 < (*pd).otherBluesCount {
                *(*pd).otherBlues.offset(j_0 as isize) = cffnum(*stack.offset(j_0 as isize));
                j_0 = j_0.wrapping_add(1);
            }
        }
        8 => {
            (*pd).familyBluesCount = top as arity_t;
            (*pd).familyBlues = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                    .wrapping_mul((*pd).familyBluesCount as usize),
                102 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j_1: arity_t = 0 as arity_t;
            while j_1 < (*pd).familyBluesCount {
                *(*pd).familyBlues.offset(j_1 as isize) = cffnum(*stack.offset(j_1 as isize));
                j_1 = j_1.wrapping_add(1);
            }
        }
        9 => {
            (*pd).familyOtherBluesCount = top as arity_t;
            (*pd).familyOtherBlues = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                    .wrapping_mul((*pd).familyOtherBluesCount as usize),
                110 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j_2: arity_t = 0 as arity_t;
            while j_2 < (*pd).familyOtherBluesCount {
                *(*pd).familyOtherBlues.offset(j_2 as isize) = cffnum(*stack.offset(j_2 as isize));
                j_2 = j_2.wrapping_add(1);
            }
        }
        3084 => {
            (*pd).stemSnapHCount = top as arity_t;
            (*pd).stemSnapH = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                    .wrapping_mul((*pd).stemSnapHCount as usize),
                118 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j_3: arity_t = 0 as arity_t;
            while j_3 < (*pd).stemSnapHCount {
                *(*pd).stemSnapH.offset(j_3 as isize) = cffnum(*stack.offset(j_3 as isize));
                j_3 = j_3.wrapping_add(1);
            }
        }
        3085 => {
            (*pd).stemSnapVCount = top as arity_t;
            (*pd).stemSnapV = __caryll_allocate_clean(
                (::core::mem::size_of::<::core::ffi::c_double>() as usize)
                    .wrapping_mul((*pd).stemSnapVCount as usize),
                126 as ::core::ffi::c_ulong,
            ) as *mut ::core::ffi::c_double;
            let mut j_4: arity_t = 0 as arity_t;
            while j_4 < (*pd).stemSnapVCount {
                *(*pd).stemSnapV.offset(j_4 as isize) = cffnum(*stack.offset(j_4 as isize));
                j_4 = j_4.wrapping_add(1);
            }
        }
        3081 => {
            if top != 0 {
                (*pd).blueScale = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3082 => {
            if top != 0 {
                (*pd).blueShift = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3083 => {
            if top != 0 {
                (*pd).blueFuzz = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        10 => {
            if top != 0 {
                (*pd).stdHW = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        11 => {
            if top != 0 {
                (*pd).stdVW = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3086 => {
            if top != 0 {
                (*pd).forceBold = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) != 0.;
            }
        }
        3089 => {
            if top != 0 {
                (*pd).languageGroup = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) as u32;
            }
        }
        3090 => {
            if top != 0 {
                (*pd).expansionFactor = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3091 => {
            if top != 0 {
                (*pd).initialRandomSeed = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        20 => {
            if top != 0 {
                (*pd).defaultWidthX = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        21 => {
            if top != 0 {
                (*pd).nominalWidthX = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn callback_extract_fd(
    mut op: u32,
    mut top: u8,
    mut stack: *mut cff_Value,
    mut _context: *mut ::core::ffi::c_void,
) {
    let mut context: *mut cff_extract_context = _context as *mut cff_extract_context;
    let mut file: *mut cff_File = (*context).cffFile;
    let mut meta: *mut table_CFF = (*context).meta;
    if (*context).fdArrayIndex >= 0 as i32
        && (*context).fdArrayIndex < (*meta).fdArrayCount as i32
    {
        meta = *(*meta).fdArray.offset((*context).fdArrayIndex as isize);
    }
    match op {
        0 => {
            if top != 0 {
                (*meta).version = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    (*file).string,
                );
            }
        }
        1 => {
            if top != 0 {
                (*meta).notice = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    (*file).string,
                );
            }
        }
        3072 => {
            if top != 0 {
                (*meta).copyright = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    (*file).string,
                );
            }
        }
        3110 => {
            if top != 0 {
                (*meta).fontName = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    (*file).string,
                );
            }
        }
        2 => {
            if top != 0 {
                (*meta).fullName = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    (*file).string,
                );
            }
        }
        3 => {
            if top != 0 {
                (*meta).familyName = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    (*file).string,
                );
            }
        }
        4 => {
            if top != 0 {
                (*meta).weight = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    (*file).string,
                );
            }
        }
        5 => {
            if top as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                (*meta).fontBBoxLeft = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize),
                );
                (*meta).fontBBoxBottom = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize),
                );
                (*meta).fontBBoxRight = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize),
                );
                (*meta).fontBBoxTop = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3079 => {
            if top as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                (*meta).fontMatrix = __caryll_allocate_clean(
                    ::core::mem::size_of::<cff_FontMatrix>() as usize,
                    208 as ::core::ffi::c_ulong,
                ) as *mut cff_FontMatrix;
                (*(*meta).fontMatrix).a = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize),
                ) as scale_t;
                (*(*meta).fontMatrix).b = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize),
                ) as scale_t;
                (*(*meta).fontMatrix).c = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize),
                ) as scale_t;
                (*(*meta).fontMatrix).d = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize),
                ) as scale_t;
                (*(*meta).fontMatrix).x = iVQ.createStill.expect("non-null function pointer")(
                    cffnum(
                        *stack
                            .offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize),
                    ) as pos_t,
                );
                (*(*meta).fontMatrix).y = iVQ.createStill.expect("non-null function pointer")(
                    cffnum(
                        *stack
                            .offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                    ) as pos_t,
                );
            }
        }
        3073 => {
            if top != 0 {
                (*meta).isFixedPitch = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) != 0.;
            }
        }
        3074 => {
            if top != 0 {
                (*meta).italicAngle = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3075 => {
            if top != 0 {
                (*meta).underlinePosition = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3076 => {
            if top != 0 {
                (*meta).underlineThickness = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3080 => {
            if top != 0 {
                (*meta).strokeWidth = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        18 => {
            if top as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
                let mut privateLength: u32 = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize),
                ) as u32;
                let mut privateOffset: u32 = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) as u32;
                (*meta).privateDict = otfcc_newCff_private();
                cff_iDict
                    .parseToCallback
                    .expect("non-null function pointer")(
                    (*file).raw_data.offset(privateOffset as isize),
                    privateLength,
                    context as *mut ::core::ffi::c_void,
                    Some(
                        callback_extract_private
                            as unsafe extern "C" fn(
                                u32,
                                u8,
                                *mut cff_Value,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                );
            }
        }
        3102 => {
            if top as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                (*meta).isCID = true;
                (*meta).cidRegistry = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    (*file).string,
                );
                (*meta).cidOrdering = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    (*file).string,
                );
                (*meta).cidSupplement = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) as u32;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn callback_draw_setwidth(
    mut _context: *mut ::core::ffi::c_void,
    mut width: ::core::ffi::c_double,
) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    iVQ.replace.expect("non-null function pointer")(
        &raw mut (*(*context).g).advanceWidth,
        iVQ.createStill.expect("non-null function pointer")(
            width as pos_t + (*context).nominalWidthX as pos_t,
        ) as VQ,
    );
}
unsafe extern "C" fn callback_draw_next_contour(mut _context: *mut ::core::ffi::c_void) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    let mut c: glyf_Contour = glyf_Contour {
        length: 0,
        capacity: 0,
        items: ::core::ptr::null_mut::<glyf_Point>(),
    };
    glyf_iContour.init.expect("non-null function pointer")(&raw mut c);
    glyf_iContourList.push.expect("non-null function pointer")(
        &raw mut (*(*context).g).contours,
        c,
    );
    (*context).jContour = (*(*context).g).contours.length as shapeid_t;
    (*context).jPoint = 0 as shapeid_t;
}
unsafe extern "C" fn callback_draw_lineto(
    mut _context: *mut ::core::ffi::c_void,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    if (*context).jContour != 0 {
        let mut contour: *mut glyf_Contour =
            (*(*context).g).contours.items.offset(
                ((*context).jContour as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
            ) as *mut glyf_Contour;
        let mut z: glyf_Point = glyf_Point {
            x: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            onCurve: 0,
        };
        glyf_iPoint.init.expect("non-null function pointer")(&raw mut z);
        z.onCurve = true_0 as i8;
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z.x,
            iVQ.createStill.expect("non-null function pointer")(x1 as pos_t) as VQ,
        );
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z.y,
            iVQ.createStill.expect("non-null function pointer")(y1 as pos_t) as VQ,
        );
        glyf_iContour.push.expect("non-null function pointer")(contour, z);
        (*context).jPoint =
            ((*context).jPoint as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
    }
}
unsafe extern "C" fn callback_draw_curveto(
    mut _context: *mut ::core::ffi::c_void,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
    mut x3: ::core::ffi::c_double,
    mut y3: ::core::ffi::c_double,
) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    if (*context).jContour != 0 {
        let mut contour: *mut glyf_Contour =
            (*(*context).g).contours.items.offset(
                ((*context).jContour as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
            ) as *mut glyf_Contour;
        let mut z: glyf_Point = glyf_Point {
            x: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            onCurve: 0,
        };
        glyf_iPoint.init.expect("non-null function pointer")(&raw mut z);
        z.onCurve = false_0 as i8;
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z.x,
            iVQ.createStill.expect("non-null function pointer")(x1 as pos_t) as VQ,
        );
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z.y,
            iVQ.createStill.expect("non-null function pointer")(y1 as pos_t) as VQ,
        );
        glyf_iContour.push.expect("non-null function pointer")(contour, z);
        let mut z_0: glyf_Point = glyf_Point {
            x: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            onCurve: 0,
        };
        glyf_iPoint.init.expect("non-null function pointer")(&raw mut z_0);
        z_0.onCurve = false_0 as i8;
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z_0.x,
            iVQ.createStill.expect("non-null function pointer")(x2 as pos_t) as VQ,
        );
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z_0.y,
            iVQ.createStill.expect("non-null function pointer")(y2 as pos_t) as VQ,
        );
        glyf_iContour.push.expect("non-null function pointer")(contour, z_0);
        let mut z_1: glyf_Point = glyf_Point {
            x: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            y: VQ {
                kernel: 0.,
                shift: vq_SegList {
                    length: 0,
                    capacity: 0,
                    items: ::core::ptr::null_mut::<vq_Segment>(),
                },
            },
            onCurve: 0,
        };
        glyf_iPoint.init.expect("non-null function pointer")(&raw mut z_1);
        z_1.onCurve = true_0 as i8;
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z_1.x,
            iVQ.createStill.expect("non-null function pointer")(x3 as pos_t) as VQ,
        );
        iVQ.copyReplace.expect("non-null function pointer")(
            &raw mut z_1.y,
            iVQ.createStill.expect("non-null function pointer")(y3 as pos_t) as VQ,
        );
        glyf_iContour.push.expect("non-null function pointer")(contour, z_1);
        (*context).jPoint =
            ((*context).jPoint as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as shapeid_t;
    }
}
unsafe extern "C" fn callback_draw_sethint(
    mut _context: *mut ::core::ffi::c_void,
    mut isVertical: bool,
    mut position: ::core::ffi::c_double,
    mut width: ::core::ffi::c_double,
) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    glyf_iStemDefList.push.expect("non-null function pointer")(
        if isVertical as ::core::ffi::c_int != 0 {
            &raw mut (*(*context).g).stemV
        } else {
            &raw mut (*(*context).g).stemH
        },
        glyf_PostscriptStemDef {
            position: position as pos_t,
            width: width as pos_t,
            map: 0,
        },
    );
}
unsafe extern "C" fn callback_draw_setmask(
    mut _context: *mut ::core::ffi::c_void,
    mut isContourMask: bool,
    mut maskArray: *mut bool,
) {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    let mut maskList: *mut glyf_MaskList = if isContourMask as ::core::ffi::c_int != 0 {
        &raw mut (*(*context).g).contourMasks
    } else {
        &raw mut (*(*context).g).hintMasks
    };
    let mut mask: glyf_PostscriptHintMask = glyf_PostscriptHintMask {
        pointsBefore: 0,
        contoursBefore: 0,
        maskH: [false; 256],
        maskV: [false; 256],
    };
    if (*context).jContour != 0 {
        mask.contoursBefore =
            ((*context).jContour as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u16;
    } else {
        mask.contoursBefore = 0 as u16;
    }
    mask.pointsBefore = (*context).jPoint as u16;
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int {
        mask.maskH[j as usize] = if (j as usize) < (*(*context).g).stemH.length {
            *maskArray.offset(j as isize) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } != 0;
        mask.maskV[j as usize] = if (j as usize) < (*(*context).g).stemV.length {
            *maskArray.offset((j as usize).wrapping_add((*(*context).g).stemH.length) as isize)
                as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } != 0;
        j = j.wrapping_add(1);
    }
    free(maskArray as *mut ::core::ffi::c_void);
    maskArray = ::core::ptr::null_mut::<bool>();
    if (*maskList).length > 0 as usize
        && (*(*maskList)
            .items
            .offset((*maskList).length.wrapping_sub(1 as usize) as isize))
        .contoursBefore as ::core::ffi::c_int
            == mask.contoursBefore as ::core::ffi::c_int
        && (*(*maskList)
            .items
            .offset((*maskList).length.wrapping_sub(1 as usize) as isize))
        .pointsBefore as ::core::ffi::c_int
            == mask.pointsBefore as ::core::ffi::c_int
    {
        let mut j_0: shapeid_t = 0 as shapeid_t;
        while (j_0 as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int {
            (*(*maskList)
                .items
                .offset((*maskList).length.wrapping_sub(1 as usize) as isize))
            .maskH[j_0 as usize] = mask.maskH[j_0 as usize];
            (*(*maskList)
                .items
                .offset((*maskList).length.wrapping_sub(1 as usize) as isize))
            .maskV[j_0 as usize] = mask.maskV[j_0 as usize];
            j_0 = j_0.wrapping_add(1);
        }
    } else {
        glyf_iMaskList.push.expect("non-null function pointer")(maskList, mask);
        if isContourMask {
            (*context).definedContourMasks = ((*context).definedContourMasks as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as u8;
        } else {
            (*context).definedHintMasks = ((*context).definedHintMasks as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as u8;
        }
    };
}
unsafe extern "C" fn callback_draw_getrand(
    mut _context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_double {
    let mut context: *mut outline_builder_context = _context as *mut outline_builder_context;
    let mut x: u64 = (*context).randx;
    x ^= x >> 12 as ::core::ffi::c_int;
    x ^= x << 25 as ::core::ffi::c_int;
    x ^= x >> 27 as ::core::ffi::c_int;
    (*context).randx = x;
    let mut a: cff_DoubleBits = cff_DoubleBits { u: 0 };
    a.u = x.wrapping_mul(2685821657736338717 as u64);
    a.u = a.u >> 12 as ::core::ffi::c_int | 0x3ff0000000000000 as u64;
    let mut q: ::core::ffi::c_double = if a.u & 2048 as u64 != 0 {
        1.0f64 - 2.2204460492503131E-16f64 / 2.0f64
    } else {
        1.0f64
    };
    return a.d - q;
}
static drawPass: cff_IOutlineBuilder = {
    cff_IOutlineBuilder {
        setWidth: Some(
            callback_draw_setwidth
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> (),
        ),
        newContour: Some(
            callback_draw_next_contour as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        lineTo: Some(
            callback_draw_lineto
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        ),
        curveTo: Some(
            callback_draw_curveto
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        ),
        setHint: Some(
            callback_draw_sethint
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    bool,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        ),
        setMask: Some(
            callback_draw_setmask
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> (),
        ),
        getrand: Some(
            callback_draw_getrand
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double,
        ),
    }
};
unsafe extern "C" fn buildOutline(
    mut i: glyphid_t,
    mut context: *mut cff_extract_context,
    mut options: *const otfcc_Options,
) {
    let mut f: *mut cff_File = (*context).cffFile;
    let mut g: *mut glyf_Glyph = otfcc_newGlyf_glyph();
    let ref mut fresh8 = *(*(*context).glyphs).items.offset(i as isize);
    *fresh8 = g as glyf_GlyphPtr;
    let mut seed: u64 = (*context).seed;
    let mut localSubrs: cff_Index = cff_Index {
        countType: CFF_INDEX_16,
        count: 0,
        offSize: 0,
        offset: ::core::ptr::null_mut::<u32>(),
        data: ::core::ptr::null_mut::<u8>(),
    };
    cff_iIndex.init.expect("non-null function pointer")(&raw mut localSubrs);
    let mut stack: cff_Stack = cff_Stack {
        stack: ::core::ptr::null_mut::<cff_Value>(),
        transient: [cff_Value {
            t: 0 as cff_Value_Type,
            c2rust_unnamed: cff_ValueBody { i: 0 },
        }; 32],
        index: 0,
        max: 0,
        stem: 0,
    };
    stack.max = 0x10000 as arity_t;
    stack.stack = __caryll_allocate_clean(
        (::core::mem::size_of::<cff_Value>() as usize).wrapping_mul(stack.max as usize),
        407 as ::core::ffi::c_ulong,
    ) as *mut cff_Value;
    stack.index = 0 as arity_t;
    stack.stem = 0 as u8;
    let mut bc: outline_builder_context = outline_builder_context {
        g: g,
        jContour: 0 as shapeid_t,
        jPoint: 0 as shapeid_t,
        defaultWidthX: 0.0f64,
        nominalWidthX: 0.0f64,
        definedHStems: 0 as u8,
        definedVStems: 0 as u8,
        definedHintMasks: 0 as u8,
        definedContourMasks: 0 as u8,
        randx: 0 as u64,
    };
    let mut fd: u8 = 0 as u8;
    if (*f).fdselect.t != cff_FDSELECT_UNSPECED as ::core::ffi::c_int as u32 {
        fd = cff_parseSubr(
            i as u16,
            (*f).raw_data,
            (*f).font_dict,
            (*f).fdselect,
            &raw mut localSubrs,
        );
    } else {
        fd = cff_parseSubr(
            i as u16,
            (*f).raw_data,
            (*f).top_dict,
            (*f).fdselect,
            &raw mut localSubrs,
        );
    }
    (*g).fdSelect = handle_fromIndex(fd as glyphid_t)
        as otfcc_FDHandle;
    if !(*(*context).meta).fdArray.is_null()
        && (fd as ::core::ffi::c_int) < (*(*context).meta).fdArrayCount as ::core::ffi::c_int
        && !(**(*(*context).meta).fdArray.offset(fd as isize))
            .privateDict
            .is_null()
    {
        bc.defaultWidthX =
            (*(**(*(*context).meta).fdArray.offset(fd as isize)).privateDict).defaultWidthX;
        bc.nominalWidthX =
            (*(**(*(*context).meta).fdArray.offset(fd as isize)).privateDict).nominalWidthX;
    } else if !(*(*context).meta).privateDict.is_null() {
        bc.defaultWidthX = (*(*(*context).meta).privateDict).defaultWidthX;
        bc.nominalWidthX = (*(*(*context).meta).privateDict).nominalWidthX;
    }
    iVQ.replace.expect("non-null function pointer")(
        &raw mut (*g).advanceWidth,
        iVQ.createStill.expect("non-null function pointer")(bc.defaultWidthX as pos_t) as VQ,
    );
    let mut charStringPtr: *mut u8 = (*f)
        .char_strings
        .data
        .offset(*(*f).char_strings.offset.offset(i as isize) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut charStringLength: u32 = (*(*f)
        .char_strings
        .offset
        .offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
    .wrapping_sub(*(*f).char_strings.offset.offset(i as isize));
    stack.index = 0 as arity_t;
    stack.stem = 0 as u8;
    bc.jContour = 0 as shapeid_t;
    bc.jPoint = 0 as shapeid_t;
    bc.randx = seed;
    cff_parseOutline(
        charStringPtr,
        charStringLength,
        (*f).global_subr,
        localSubrs,
        &raw mut stack,
        &raw mut bc as *mut ::core::ffi::c_void,
        drawPass,
        options,
    );
    let mut cx: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut cy: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut j: shapeid_t = 0 as shapeid_t;
    while (j as usize) < (*g).contours.length {
        let mut contour: *mut glyf_Contour =
            (*g).contours.items.offset(j as isize) as *mut glyf_Contour;
        let mut k: shapeid_t = 0 as shapeid_t;
        while (k as usize) < (*contour).length {
            let mut z: *mut glyf_Point = (*contour).items.offset(k as isize) as *mut glyf_Point;
            iVQ.inplacePlus.expect("non-null function pointer")(&raw mut cx, (*z).x);
            iVQ.inplacePlus.expect("non-null function pointer")(&raw mut cy, (*z).y);
            iVQ.copyReplace.expect("non-null function pointer")(&raw mut (*z).x, cx);
            iVQ.copyReplace.expect("non-null function pointer")(&raw mut (*z).y, cy);
            k = k.wrapping_add(1);
        }
        if iVQ.compare.expect("non-null function pointer")(
            (*(*contour).items.offset(0 as ::core::ffi::c_int as isize)).x,
            (*(*contour)
                .items
                .offset((*contour).length.wrapping_sub(1 as usize) as isize))
            .x,
        ) == 0
            && iVQ.compare.expect("non-null function pointer")(
                (*(*contour).items.offset(0 as ::core::ffi::c_int as isize)).y,
                (*(*contour)
                    .items
                    .offset((*contour).length.wrapping_sub(1 as usize) as isize))
                .y,
            ) == 0
            && ((*(*contour).items.offset(0 as ::core::ffi::c_int as isize)).onCurve
                as ::core::ffi::c_int
                != 0
                && (*(*contour)
                    .items
                    .offset((*contour).length.wrapping_sub(1 as usize) as isize))
                .onCurve as ::core::ffi::c_int
                    != 0)
        {
            glyf_iContour.pop.expect("non-null function pointer")(contour);
        }
        glyf_iContour
            .shrinkToFit
            .expect("non-null function pointer")(contour);
        j = j.wrapping_add(1);
    }
    glyf_iContourList
        .shrinkToFit
        .expect("non-null function pointer")(&raw mut (*g).contours);
    iVQ.dispose.expect("non-null function pointer")(&raw mut cx);
    iVQ.dispose.expect("non-null function pointer")(&raw mut cy);
    cff_iIndex.dispose.expect("non-null function pointer")(&raw mut localSubrs);
    free(stack.stack as *mut ::core::ffi::c_void);
    stack.stack = ::core::ptr::null_mut::<cff_Value>();
    (*context).seed = bc.randx;
}
unsafe extern "C" fn formCIDString(mut cid: cffsid_t) -> sds {
    return crate::sdsbuild!(
        sdsnew(b"CID\0" as *const u8 as *const ::core::ffi::c_char),
        cid as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn nameGlyphsAccordingToCFF(mut context: *mut cff_extract_context) {
    let mut cffFile: *mut cff_File = (*context).cffFile;
    let mut glyphs: *mut table_glyf = (*context).glyphs;
    let mut charset: *mut cff_Charset = &raw mut (*cffFile).charsets;
    if (*(*context).meta).isCID {
        match (*charset).t {
            3 => {
                let mut j: glyphid_t = 0 as glyphid_t;
                while (j as u32) < (*charset).s {
                    let mut sid: cffsid_t =
                        *(*charset).c2rust_unnamed.f0.glyph.offset(j as isize) as cffsid_t;
                    let mut glyphname: sds = sdsget_cff_sid(sid as u16, (*cffFile).string);
                    if !glyphname.is_null() {
                        let ref mut fresh2 = (**(*glyphs)
                            .items
                            .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .name;
                        *fresh2 = glyphname;
                        (**(*glyphs).items.offset(
                            (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .cid = sid as glyphid_t;
                    }
                    j = j.wrapping_add(1);
                }
            }
            4 => {
                let mut glyphsNamedSofar: u32 = 1 as u32;
                let mut j_0: glyphid_t = 0 as glyphid_t;
                while (j_0 as u32) < (*charset).s {
                    let mut first: cffsid_t =
                        (*(*charset).c2rust_unnamed.f1.range1.offset(j_0 as isize)).first
                            as cffsid_t;
                    let mut k: glyphid_t = 0 as glyphid_t;
                    while k as ::core::ffi::c_int
                        <= (*(*charset).c2rust_unnamed.f1.range1.offset(j_0 as isize)).nleft
                            as ::core::ffi::c_int
                    {
                        let mut sid_0: cffsid_t =
                            (first as ::core::ffi::c_int + k as ::core::ffi::c_int) as cffsid_t;
                        let mut glyphname_0: sds = formCIDString(sid_0);
                        if (glyphsNamedSofar as usize) < (*glyphs).length && !glyphname_0.is_null()
                        {
                            let ref mut fresh3 =
                                (**(*glyphs).items.offset(glyphsNamedSofar as isize)).name;
                            *fresh3 = glyphname_0;
                            (**(*glyphs).items.offset(glyphsNamedSofar as isize)).cid =
                                sid_0 as glyphid_t;
                        }
                        glyphsNamedSofar = glyphsNamedSofar.wrapping_add(1);
                        k = k.wrapping_add(1);
                    }
                    j_0 = j_0.wrapping_add(1);
                }
            }
            5 => {
                let mut glyphsNamedSofar_0: u32 = 1 as u32;
                let mut j_1: glyphid_t = 0 as glyphid_t;
                while (j_1 as u32) < (*charset).s {
                    let mut first_0: cffsid_t =
                        (*(*charset).c2rust_unnamed.f2.range2.offset(j_1 as isize)).first
                            as cffsid_t;
                    let mut k_0: glyphid_t = 0 as glyphid_t;
                    while k_0 as ::core::ffi::c_int
                        <= (*(*charset).c2rust_unnamed.f2.range2.offset(j_1 as isize)).nleft
                            as ::core::ffi::c_int
                    {
                        let mut sid_1: cffsid_t =
                            (first_0 as ::core::ffi::c_int + k_0 as ::core::ffi::c_int) as cffsid_t;
                        let mut glyphname_1: sds = formCIDString(sid_1);
                        if (glyphsNamedSofar_0 as usize) < (*glyphs).length
                            && !glyphname_1.is_null()
                        {
                            let ref mut fresh4 =
                                (**(*glyphs).items.offset(glyphsNamedSofar_0 as isize)).name;
                            *fresh4 = glyphname_1;
                            (**(*glyphs).items.offset(glyphsNamedSofar_0 as isize)).cid =
                                sid_1 as glyphid_t;
                        }
                        glyphsNamedSofar_0 = glyphsNamedSofar_0.wrapping_add(1);
                        k_0 = k_0.wrapping_add(1);
                    }
                    j_1 = j_1.wrapping_add(1);
                }
            }
            _ => {}
        }
    } else {
        match (*charset).t {
            3 => {
                let mut j_2: glyphid_t = 0 as glyphid_t;
                while (j_2 as u32) < (*charset).s {
                    let mut sid_2: cffsid_t =
                        *(*charset).c2rust_unnamed.f0.glyph.offset(j_2 as isize) as cffsid_t;
                    let mut glyphname_2: sds = sdsget_cff_sid(sid_2 as u16, (*cffFile).string);
                    if !glyphname_2.is_null() {
                        let ref mut fresh5 = (**(*glyphs).items.offset(
                            (j_2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .name;
                        *fresh5 = glyphname_2;
                    }
                    j_2 = j_2.wrapping_add(1);
                }
            }
            4 => {
                let mut glyphsNamedSofar_1: u32 = 1 as u32;
                let mut j_3: glyphid_t = 0 as glyphid_t;
                while (j_3 as u32) < (*charset).s {
                    let mut first_1: glyphid_t =
                        (*(*charset).c2rust_unnamed.f1.range1.offset(j_3 as isize)).first
                            as glyphid_t;
                    let mut k_1: glyphid_t = 0 as glyphid_t;
                    while k_1 as ::core::ffi::c_int
                        <= (*(*charset).c2rust_unnamed.f1.range1.offset(j_3 as isize)).nleft
                            as ::core::ffi::c_int
                    {
                        let mut sid_3: cffsid_t =
                            (first_1 as ::core::ffi::c_int + k_1 as ::core::ffi::c_int) as cffsid_t;
                        let mut glyphname_3: sds =
                            sdsget_cff_sid(sid_3 as u16, (*cffFile).string);
                        if (glyphsNamedSofar_1 as usize) < (*glyphs).length
                            && !glyphname_3.is_null()
                        {
                            let ref mut fresh6 =
                                (**(*glyphs).items.offset(glyphsNamedSofar_1 as isize)).name;
                            *fresh6 = glyphname_3;
                        }
                        glyphsNamedSofar_1 = glyphsNamedSofar_1.wrapping_add(1);
                        k_1 = k_1.wrapping_add(1);
                    }
                    j_3 = j_3.wrapping_add(1);
                }
            }
            5 => {
                let mut glyphsNamedSofar_2: u32 = 1 as u32;
                let mut j_4: glyphid_t = 0 as glyphid_t;
                while (j_4 as u32) < (*charset).s {
                    let mut first_2: glyphid_t =
                        (*(*charset).c2rust_unnamed.f2.range2.offset(j_4 as isize)).first
                            as glyphid_t;
                    let mut k_2: glyphid_t = 0 as glyphid_t;
                    while k_2 as ::core::ffi::c_int
                        <= (*(*charset).c2rust_unnamed.f2.range2.offset(j_4 as isize)).nleft
                            as ::core::ffi::c_int
                    {
                        let mut sid_4: cffsid_t =
                            (first_2 as ::core::ffi::c_int + k_2 as ::core::ffi::c_int) as cffsid_t;
                        let mut glyphname_4: sds =
                            sdsget_cff_sid(sid_4 as u16, (*cffFile).string);
                        if (glyphsNamedSofar_2 as usize) < (*glyphs).length
                            && !glyphname_4.is_null()
                        {
                            let ref mut fresh7 =
                                (**(*glyphs).items.offset(glyphsNamedSofar_2 as isize)).name;
                            *fresh7 = glyphname_4;
                        }
                        glyphsNamedSofar_2 = glyphsNamedSofar_2.wrapping_add(1);
                        k_2 = k_2.wrapping_add(1);
                    }
                    j_4 = j_4.wrapping_add(1);
                }
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn qround(x: ::core::ffi::c_double) -> ::core::ffi::c_double {
    return otfcc_from_fixed(otfcc_to_fixed(x));
}
unsafe extern "C" fn applyCffMatrix(
    mut CFF_: *mut table_CFF,
    mut glyf: *mut table_glyf,
    mut head: *const table_head,
) {
    let mut jj: glyphid_t = 0 as glyphid_t;
    while (jj as usize) < (*glyf).length {
        let mut g: *mut glyf_Glyph = *(*glyf).items.offset(jj as isize) as *mut glyf_Glyph;
        let mut fd: *mut table_CFF = CFF_;
        if !(*fd).fdArray.is_null()
            && ((*g).fdSelect.index as ::core::ffi::c_int)
                < (*fd).fdArrayCount as ::core::ffi::c_int
        {
            fd = *(*fd).fdArray.offset((*g).fdSelect.index as isize);
        }
        if !(*fd).fontMatrix.is_null() {
            let mut a: scale_t = qround(
                (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    * (*(*fd).fontMatrix).a as ::core::ffi::c_double,
            ) as scale_t;
            let mut b: scale_t = qround(
                (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    * (*(*fd).fontMatrix).b as ::core::ffi::c_double,
            ) as scale_t;
            let mut c: scale_t = qround(
                (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    * (*(*fd).fontMatrix).c as ::core::ffi::c_double,
            ) as scale_t;
            let mut d: scale_t = qround(
                (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    * (*(*fd).fontMatrix).d as ::core::ffi::c_double,
            ) as scale_t;
            let mut x: VQ = iVQ.scale.expect("non-null function pointer")(
                (*(*fd).fontMatrix).x,
                (*head).unitsPerEm as scale_t,
            );
            x.kernel = qround(x.kernel as ::core::ffi::c_double) as pos_t;
            let mut y: VQ = iVQ.scale.expect("non-null function pointer")(
                (*(*fd).fontMatrix).y,
                (*head).unitsPerEm as scale_t,
            );
            y.kernel = qround(y.kernel as ::core::ffi::c_double) as pos_t;
            let mut j: shapeid_t = 0 as shapeid_t;
            while (j as usize) < (*g).contours.length {
                let mut contour: *mut glyf_Contour =
                    (*g).contours.items.offset(j as isize) as *mut glyf_Contour;
                let mut k: shapeid_t = 0 as shapeid_t;
                while (k as usize) < (*contour).length {
                    let mut zx: VQ = iVQ.dup.expect("non-null function pointer")(
                        (*(*contour).items.offset(k as isize)).x,
                    );
                    let mut zy: VQ = iVQ.dup.expect("non-null function pointer")(
                        (*(*contour).items.offset(k as isize)).y,
                    );
                    iVQ.replace.expect("non-null function pointer")(
                        &raw mut (*(*contour).items.offset(k as isize)).x,
                        iVQ.pointLinearTfm.expect("non-null function pointer")(
                            x, a as pos_t, zx, b as pos_t, zy,
                        ) as VQ,
                    );
                    iVQ.replace.expect("non-null function pointer")(
                        &raw mut (*(*contour).items.offset(k as isize)).y,
                        iVQ.pointLinearTfm.expect("non-null function pointer")(
                            y, c as pos_t, zx, d as pos_t, zy,
                        ) as VQ,
                    );
                    iVQ.dispose.expect("non-null function pointer")(&raw mut zx);
                    iVQ.dispose.expect("non-null function pointer")(&raw mut zy);
                    k = k.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
            iVQ.dispose.expect("non-null function pointer")(&raw mut x);
            iVQ.dispose.expect("non-null function pointer")(&raw mut y);
        }
        jj = jj.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_readCFFAndGlyfTables(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
    mut head: *const table_head,
) -> table_CFFAndGlyf {
    let mut ret: table_CFFAndGlyf = table_CFFAndGlyf {
        meta: ::core::ptr::null_mut::<table_CFF>(),
        glyphs: ::core::ptr::null_mut::<table_glyf>(),
    };
    ret.meta = ::core::ptr::null_mut::<table_CFF>();
    ret.glyphs = ::core::ptr::null_mut::<table_glyf>();
    let mut context: cff_extract_context = cff_extract_context {
        fdArrayIndex: 0,
        meta: ::core::ptr::null_mut::<table_CFF>(),
        glyphs: ::core::ptr::null_mut::<table_glyf>(),
        cffFile: ::core::ptr::null_mut::<cff_File>(),
        seed: 0,
    };
    context.fdArrayIndex = -(1 as ::core::ffi::c_int) as i32;
    context.meta = ::core::ptr::null_mut::<table_CFF>();
    context.glyphs = ::core::ptr::null_mut::<table_glyf>();
    context.cffFile = ::core::ptr::null_mut::<cff_File>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1128678944i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    let mut cffFile: *mut cff_File =
                        cff_openStream(data as *mut u8, length, options);
                    context.cffFile = cffFile;
                    context.meta = (
                        table_iCFF.create.expect("non-null function pointer"))();
                    cff_iDict
                        .parseToCallback
                        .expect("non-null function pointer")(
                        (*cffFile).top_dict.data,
                        (*(*cffFile)
                            .top_dict
                            .offset
                            .offset(1 as ::core::ffi::c_int as isize))
                        .wrapping_sub(
                            *(*cffFile)
                                .top_dict
                                .offset
                                .offset(0 as ::core::ffi::c_int as isize),
                        ),
                        &raw mut context as *mut ::core::ffi::c_void,
                        Some(
                            callback_extract_fd
                                as unsafe extern "C" fn(
                                    u32,
                                    u8,
                                    *mut cff_Value,
                                    *mut ::core::ffi::c_void,
                                ) -> (),
                        ),
                    );
                    if (*context.meta).fontName.is_null() {
                        (*context.meta).fontName = sdsget_cff_sid(391 as u16, (*cffFile).name);
                    }
                    if (*cffFile).font_dict.count != 0 {
                        (*context.meta).fdArrayCount = (*cffFile).font_dict.count as tableid_t;
                        (*context.meta).fdArray = __caryll_allocate_clean(
                            (::core::mem::size_of::<*mut table_CFF>() as usize)
                                .wrapping_mul((*context.meta).fdArrayCount as usize),
                            637 as ::core::ffi::c_ulong,
                        ) as *mut *mut table_CFF;
                        let mut j: tableid_t = 0 as tableid_t;
                        while (j as ::core::ffi::c_int)
                            < (*context.meta).fdArrayCount as ::core::ffi::c_int
                        {
                            let ref mut fresh0 = *(*context.meta).fdArray.offset(j as isize);
                            *fresh0 = (
                                table_iCFF.create.expect("non-null function pointer"))();
                            context.fdArrayIndex = j as i32;
                            cff_iDict
                                .parseToCallback
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*cffFile)
                                    .font_dict
                                    .data
                                    .offset(
                                        *(*cffFile).font_dict.offset.offset(j as isize) as isize,
                                    )
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                (*(*cffFile)
                                    .font_dict
                                    .offset
                                    .offset(
                                        (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                    ))
                                    .wrapping_sub(
                                        *(*cffFile).font_dict.offset.offset(j as isize),
                                    ),
                                &raw mut context as *mut ::core::ffi::c_void,
                                Some(
                                    callback_extract_fd
                                        as unsafe extern "C" fn(
                                            u32,
                                            u8,
                                            *mut cff_Value,
                                            *mut ::core::ffi::c_void,
                                        ) -> (),
                                ),
                            );
                            if (**(*context.meta).fdArray.offset(j as isize))
                                .fontName
                                .is_null()
                            {
                                let ref mut fresh1 =
                                    (**(*context.meta).fdArray.offset(j as isize)).fontName;
                                *fresh1 = crate::sdsbuild!(sdsempty(), b"_Subfont", j as ::core::ffi::c_int);
                            }
                            j = j.wrapping_add(1);
                        }
                    }
                    ret.meta = context.meta;
                    context.seed = 0x1234567887654321 as u64;
                    if !(*context.meta).privateDict.is_null() {
                        context.seed = (*(*context.meta).privateDict).initialRandomSeed as u64
                            ^ 0x1234567887654321 as u64;
                    }
                    let mut glyphs: *mut table_glyf =
                        table_iGlyf.createN.expect("non-null function pointer")(
                            (*cffFile).char_strings.count as usize,
                        );
                    context.glyphs = glyphs;
                    let mut j_0: glyphid_t = 0 as glyphid_t;
                    while (j_0 as usize) < (*glyphs).length {
                        buildOutline(j_0, &raw mut context, options);
                        j_0 = j_0.wrapping_add(1);
                    }
                    applyCffMatrix(context.meta, context.glyphs, head);
                    nameGlyphsAccordingToCFF(&raw mut context);
                    ret.glyphs = context.glyphs;
                    cff_close(cffFile);
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ret;
}
unsafe extern "C" fn pdDeltaToJson(
    mut target: *mut json_value,
    mut field: *const ::core::ffi::c_char,
    mut count: arity_t,
    mut values: *mut ::core::ffi::c_double,
) {
    if count == 0 || values.is_null() {
        return;
    }
    let mut a: *mut json_value = json_array_new(count as usize);
    let mut j: arity_t = 0 as arity_t;
    while j < count {
        json_array_push(a, json_double_new(*values.offset(j as isize)));
        j = j.wrapping_add(1);
    }
    json_object_push(target, field, a);
}
unsafe extern "C" fn pdToJson(mut pd: *const cff_PrivateDict) -> *mut json_value {
    let mut _pd: *mut json_value = json_object_new(24 as usize);
    pdDeltaToJson(
        _pd,
        b"blueValues\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).blueValuesCount,
        (*pd).blueValues,
    );
    pdDeltaToJson(
        _pd,
        b"otherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).otherBluesCount,
        (*pd).otherBlues,
    );
    pdDeltaToJson(
        _pd,
        b"familyBlues\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).familyBluesCount,
        (*pd).familyBlues,
    );
    pdDeltaToJson(
        _pd,
        b"familyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).familyOtherBluesCount,
        (*pd).familyOtherBlues,
    );
    pdDeltaToJson(
        _pd,
        b"stemSnapH\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).stemSnapHCount,
        (*pd).stemSnapH,
    );
    pdDeltaToJson(
        _pd,
        b"stemSnapV\0" as *const u8 as *const ::core::ffi::c_char,
        (*pd).stemSnapVCount,
        (*pd).stemSnapV,
    );
    if (*pd).blueScale != DEFAULT_BLUE_SCALE {
        json_object_push(
            _pd,
            b"blueScale\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).blueScale),
        );
    }
    if (*pd).blueShift != DEFAULT_BLUE_SHIFT {
        json_object_push(
            _pd,
            b"blueShift\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).blueShift),
        );
    }
    if (*pd).blueFuzz != DEFAULT_BLUE_FUZZ {
        json_object_push(
            _pd,
            b"blueFuzz\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).blueFuzz),
        );
    }
    if (*pd).stdHW != 0. {
        json_object_push(
            _pd,
            b"stdHW\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).stdHW),
        );
    }
    if (*pd).stdVW != 0. {
        json_object_push(
            _pd,
            b"stdVW\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).stdVW),
        );
    }
    if (*pd).forceBold {
        json_object_push(
            _pd,
            b"forceBold\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*pd).forceBold as ::core::ffi::c_int),
        );
    }
    if (*pd).languageGroup != 0 {
        json_object_push(
            _pd,
            b"languageGroup\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).languageGroup as ::core::ffi::c_double),
        );
    }
    if (*pd).expansionFactor != DEFAULT_EXPANSION_FACTOR {
        json_object_push(
            _pd,
            b"expansionFactor\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).expansionFactor),
        );
    }
    if (*pd).initialRandomSeed != 0. {
        json_object_push(
            _pd,
            b"initialRandomSeed\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).initialRandomSeed),
        );
    }
    if (*pd).defaultWidthX != 0. {
        json_object_push(
            _pd,
            b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).defaultWidthX),
        );
    }
    if (*pd).nominalWidthX != 0. {
        json_object_push(
            _pd,
            b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).nominalWidthX),
        );
    }
    return _pd;
}
unsafe extern "C" fn fdToJson(mut table: *const table_CFF) -> *mut json_value {
    let mut _CFF_: *mut json_value = json_object_new(24 as usize);
    if (*table).isCID {
        json_object_push(
            _CFF_,
            b"isCID\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*table).isCID as ::core::ffi::c_int),
        );
    }
    if !(*table).version.is_null() {
        json_object_push(
            _CFF_,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).version),
        );
    }
    if !(*table).notice.is_null() {
        json_object_push(
            _CFF_,
            b"notice\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).notice),
        );
    }
    if !(*table).copyright.is_null() {
        json_object_push(
            _CFF_,
            b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).copyright),
        );
    }
    if !(*table).fontName.is_null() {
        json_object_push(
            _CFF_,
            b"fontName\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).fontName),
        );
    }
    if !(*table).fullName.is_null() {
        json_object_push(
            _CFF_,
            b"fullName\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).fullName),
        );
    }
    if !(*table).familyName.is_null() {
        json_object_push(
            _CFF_,
            b"familyName\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).familyName),
        );
    }
    if !(*table).weight.is_null() {
        json_object_push(
            _CFF_,
            b"weight\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).weight),
        );
    }
    if (*table).isFixedPitch {
        json_object_push(
            _CFF_,
            b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*table).isFixedPitch as ::core::ffi::c_int),
        );
    }
    if (*table).italicAngle != 0. {
        json_object_push(
            _CFF_,
            b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).italicAngle),
        );
    }
    if (*table).underlinePosition != -(100 as ::core::ffi::c_int) as ::core::ffi::c_double {
        json_object_push(
            _CFF_,
            b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).underlinePosition),
        );
    }
    if (*table).underlineThickness != 50 as ::core::ffi::c_int as ::core::ffi::c_double {
        json_object_push(
            _CFF_,
            b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).underlineThickness),
        );
    }
    if (*table).strokeWidth != 0. {
        json_object_push(
            _CFF_,
            b"strokeWidth\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).strokeWidth),
        );
    }
    if (*table).fontBBoxLeft != 0. {
        json_object_push(
            _CFF_,
            b"fontBBoxLeft\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).fontBBoxLeft),
        );
    }
    if (*table).fontBBoxBottom != 0. {
        json_object_push(
            _CFF_,
            b"fontBBoxBottom\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).fontBBoxBottom),
        );
    }
    if (*table).fontBBoxRight != 0. {
        json_object_push(
            _CFF_,
            b"fontBBoxRight\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).fontBBoxRight),
        );
    }
    if (*table).fontBBoxTop != 0. {
        json_object_push(
            _CFF_,
            b"fontBBoxTop\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).fontBBoxTop),
        );
    }
    if !(*table).fontMatrix.is_null() {
        let mut _fontMatrix: *mut json_value = json_object_new(6 as usize);
        json_object_push(
            _fontMatrix,
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*(*table).fontMatrix).a as ::core::ffi::c_double),
        );
        json_object_push(
            _fontMatrix,
            b"b\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*(*table).fontMatrix).b as ::core::ffi::c_double),
        );
        json_object_push(
            _fontMatrix,
            b"c\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*(*table).fontMatrix).c as ::core::ffi::c_double),
        );
        json_object_push(
            _fontMatrix,
            b"d\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*(*table).fontMatrix).d as ::core::ffi::c_double),
        );
        json_object_push(
            _fontMatrix,
            b"x\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_VQ((*(*table).fontMatrix).x, ::core::ptr::null::<table_fvar>()),
        );
        json_object_push(
            _fontMatrix,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_VQ((*(*table).fontMatrix).y, ::core::ptr::null::<table_fvar>()),
        );
        json_object_push(
            _CFF_,
            b"fontMatrix\0" as *const u8 as *const ::core::ffi::c_char,
            _fontMatrix,
        );
    }
    if !(*table).privateDict.is_null() {
        json_object_push(
            _CFF_,
            b"privates\0" as *const u8 as *const ::core::ffi::c_char,
            pdToJson((*table).privateDict),
        );
    }
    if !(*table).cidRegistry.is_null() && !(*table).cidOrdering.is_null() {
        json_object_push(
            _CFF_,
            b"cidRegistry\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).cidRegistry),
        );
        json_object_push(
            _CFF_,
            b"cidOrdering\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds((*table).cidOrdering),
        );
        json_object_push(
            _CFF_,
            b"cidSupplement\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).cidSupplement as i64),
        );
    }
    if !(*table).fdArray.is_null() {
        let mut _fdArray: *mut json_value = json_object_new((*table).fdArrayCount as usize);
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*table).fdArrayCount as ::core::ffi::c_int {
            let mut name: sds = (**(*table).fdArray.offset(j as isize)).fontName;
            let ref mut fresh9 = (**(*table).fdArray.offset(j as isize)).fontName;
            *fresh9 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            json_object_push(
                _fdArray,
                name as *const ::core::ffi::c_char,
                fdToJson(*(*table).fdArray.offset(j as isize)),
            );
            let ref mut fresh10 = (**(*table).fdArray.offset(j as isize)).fontName;
            *fresh10 = name;
            j = j.wrapping_add(1);
        }
        json_object_push(
            _CFF_,
            b"fdArray\0" as *const u8 as *const ::core::ffi::c_char,
            _fdArray,
        );
    }
    return _CFF_;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_dumpCFF(
    mut table: *const table_CFF,
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
        crate::sdsbuild!(sdsempty(), b"CFF"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        json_object_push(
            root,
            b"CFF_\0" as *const u8 as *const ::core::ffi::c_char,
            fdToJson(table),
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
unsafe extern "C" fn pdDeltaFromJson(
    mut dump: *mut json_value,
    mut count: *mut arity_t,
    mut array: *mut *mut ::core::ffi::c_double,
) {
    if dump.is_null()
        || (*dump).type_0 as ::core::ffi::c_uint
            != json_array as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    *count = (*dump).u.array.length as arity_t;
    *array = __caryll_allocate_clean(
        (::core::mem::size_of::<::core::ffi::c_double>() as usize).wrapping_mul(*count as usize),
        785 as ::core::ffi::c_ulong,
    ) as *mut ::core::ffi::c_double;
    let mut j: arity_t = 0 as arity_t;
    while j < *count {
        *(*array).offset(j as isize) = json_numof(*(*dump).u.array.values.offset(j as isize));
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn pdFromJson(mut dump: *mut json_value) -> *mut cff_PrivateDict {
    if dump.is_null()
        || (*dump).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<cff_PrivateDict>();
    }
    let mut pd: *mut cff_PrivateDict = otfcc_newCff_private();
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"blueValues\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).blueValuesCount,
        &raw mut (*pd).blueValues,
    );
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"otherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).otherBluesCount,
        &raw mut (*pd).otherBlues,
    );
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"familyBlues\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).familyBluesCount,
        &raw mut (*pd).familyBlues,
    );
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"familyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).familyOtherBluesCount,
        &raw mut (*pd).familyOtherBlues,
    );
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"stemSnapH\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).stemSnapHCount,
        &raw mut (*pd).stemSnapH,
    );
    pdDeltaFromJson(
        json_obj_get(
            dump,
            b"stemSnapV\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &raw mut (*pd).stemSnapVCount,
        &raw mut (*pd).stemSnapV,
    );
    (*pd).blueScale = json_obj_getnum_fallback(
        dump,
        b"blueScale\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_BLUE_SCALE,
    );
    (*pd).blueShift = json_obj_getnum_fallback(
        dump,
        b"blueShift\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_BLUE_SHIFT,
    );
    (*pd).blueFuzz = json_obj_getnum_fallback(
        dump,
        b"blueFuzz\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_BLUE_FUZZ,
    );
    (*pd).stdHW = json_obj_getnum(dump, b"stdHW\0" as *const u8 as *const ::core::ffi::c_char);
    (*pd).stdVW = json_obj_getnum(dump, b"stdVW\0" as *const u8 as *const ::core::ffi::c_char);
    (*pd).forceBold = json_obj_getbool(
        dump,
        b"forceBold\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*pd).languageGroup = json_obj_getnum(
        dump,
        b"languageGroup\0" as *const u8 as *const ::core::ffi::c_char,
    ) as u32;
    (*pd).expansionFactor = json_obj_getnum_fallback(
        dump,
        b"expansionFactor\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_EXPANSION_FACTOR,
    );
    (*pd).initialRandomSeed = json_obj_getnum(
        dump,
        b"initialRandomSeed\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return pd;
}
unsafe extern "C" fn fdFromJson(
    mut dump: *const json_value,
    mut options: *const otfcc_Options,
    mut topLevel: bool,
) -> *mut table_CFF {
    let mut table: *mut table_CFF = (
        table_iCFF.create.expect("non-null function pointer"))();
    if dump.is_null()
        || (*dump).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return table;
    }
    (*table).version = json_obj_getsds(
        dump,
        b"version\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).notice = json_obj_getsds(dump, b"notice\0" as *const u8 as *const ::core::ffi::c_char);
    (*table).copyright = json_obj_getsds(
        dump,
        b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fontName = json_obj_getsds(
        dump,
        b"fontName\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fullName = json_obj_getsds(
        dump,
        b"fullName\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).familyName = json_obj_getsds(
        dump,
        b"familyName\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).weight = json_obj_getsds(dump, b"weight\0" as *const u8 as *const ::core::ffi::c_char);
    (*table).isFixedPitch = json_obj_getbool(
        dump,
        b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).italicAngle = json_obj_getnum(
        dump,
        b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).underlinePosition = json_obj_getnum_fallback(
        dump,
        b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
        -100.0f64,
    );
    (*table).underlineThickness = json_obj_getnum_fallback(
        dump,
        b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
        50.0f64,
    );
    (*table).strokeWidth = json_obj_getnum(
        dump,
        b"strokeWidth\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fontBBoxLeft = json_obj_getnum(
        dump,
        b"fontBBoxLeft\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fontBBoxBottom = json_obj_getnum(
        dump,
        b"fontBBoxBottom\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fontBBoxRight = json_obj_getnum(
        dump,
        b"fontBBoxRight\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).fontBBoxTop = json_obj_getnum(
        dump,
        b"fontBBoxTop\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).privateDict = pdFromJson(json_obj_get_type(
        dump,
        b"privates\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    ));
    (*table).cidRegistry = json_obj_getsds(
        dump,
        b"cidRegistry\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).cidOrdering = json_obj_getsds(
        dump,
        b"cidOrdering\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).cidSupplement = json_obj_getint(
        dump,
        b"cidSupplement\0" as *const u8 as *const ::core::ffi::c_char,
    ) as u32;
    (*table).UIDBase = json_obj_getint(
        dump,
        b"UIDBase\0" as *const u8 as *const ::core::ffi::c_char,
    ) as u32;
    (*table).cidCount = json_obj_getint(
        dump,
        b"cidCount\0" as *const u8 as *const ::core::ffi::c_char,
    ) as u32;
    (*table).cidFontVersion = json_obj_getnum(
        dump,
        b"cidFontVersion\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).cidFontRevision = json_obj_getnum(
        dump,
        b"cidFontRevision\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut fdarraydump: *mut json_value = json_obj_get_type(
        dump,
        b"fdArray\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !fdarraydump.is_null() {
        (*table).isCID = true;
        (*table).fdArrayCount = (*fdarraydump).u.object.length as tableid_t;
        (*table).fdArray = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut table_CFF>() as usize)
                .wrapping_mul((*table).fdArrayCount as usize),
            872 as ::core::ffi::c_ulong,
        ) as *mut *mut table_CFF;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*table).fdArrayCount as ::core::ffi::c_int {
            let ref mut fresh11 = *(*table).fdArray.offset(j as isize);
            *fresh11 = fdFromJson(
                (*(*fdarraydump).u.object.values.offset(j as isize)).value,
                options,
                false,
            );
            if !(**(*table).fdArray.offset(j as isize)).fontName.is_null() {
                sdsfree((**(*table).fdArray.offset(j as isize)).fontName);
            }
            let ref mut fresh12 = (**(*table).fdArray.offset(j as isize)).fontName;
            *fresh12 = sdsnewlen(
                (*(*fdarraydump).u.object.values.offset(j as isize)).name
                    as *const ::core::ffi::c_void,
                (*(*fdarraydump).u.object.values.offset(j as isize)).name_length as usize,
            );
            j = j.wrapping_add(1);
        }
    }
    if (*table).fontName.is_null() {
        (*table).fontName = sdsnew(b"CARYLL_CFFFONT\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*table).privateDict.is_null() {
        (*table).privateDict = otfcc_newCff_private();
    }
    if topLevel as ::core::ffi::c_int != 0
        && (*options).force_cid as ::core::ffi::c_int != 0
        && (*table).fdArray.is_null()
    {
        (*table).fdArrayCount = 1 as tableid_t;
        (*table).fdArray = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut table_CFF>() as usize)
                .wrapping_mul((*table).fdArrayCount as usize),
            885 as ::core::ffi::c_ulong,
        ) as *mut *mut table_CFF;
        let ref mut fresh13 = *(*table).fdArray.offset(0 as ::core::ffi::c_int as isize);
        *fresh13 = (
            table_iCFF.create.expect("non-null function pointer"))();
        let mut fd0: *mut table_CFF = *(*table).fdArray.offset(0 as ::core::ffi::c_int as isize);
        (*fd0).privateDict = (*table).privateDict;
        (*table).privateDict = otfcc_newCff_private();
        (*fd0).fontName = sdscat(
            sdsdup((*table).fontName),
            b"-subfont0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*table).isCID = true;
    }
    if (*table).isCID as ::core::ffi::c_int != 0 && (*table).cidRegistry.is_null() {
        (*table).cidRegistry = sdsnew(b"CARYLL\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*table).isCID as ::core::ffi::c_int != 0 && (*table).cidOrdering.is_null() {
        (*table).cidOrdering = sdsnew(b"OTFCCAUTOCID\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return table;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_parseCFF(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_CFF {
    let mut dump: *mut json_value = json_obj_get_type(
        root,
        b"CFF_\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if dump.is_null() {
        return ::core::ptr::null_mut::<table_CFF>();
    } else {
        let mut cff: *mut table_CFF = ::core::ptr::null_mut::<table_CFF>();
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), b"CFF"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            cff = fdFromJson(dump, options, true);
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        return cff;
    };
}
unsafe extern "C" fn cff_make_charstrings(
    mut context: *mut cff_charstring_builder_context,
    mut s: *mut *mut caryll_Buffer,
    mut gs: *mut *mut caryll_Buffer,
    mut ls: *mut *mut caryll_Buffer,
) {
    if (*(*context).glyf).length == 0 as usize {
        return;
    }
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*(*context).glyf).length {
        let mut il: *mut cff_CharstringIL = cff_compileGlyphToIL(
            *(*(*context).glyf).items.offset(j as isize) as *mut glyf_Glyph,
            (*context).defaultWidth,
            (*context).nominalWidthX,
        );
        cff_optimizeIL(il, (*context).options);
        cff_insertILToGraph(&raw mut (*context).graph, il);
        free((*il).instr as *mut ::core::ffi::c_void);
        (*il).instr = ::core::ptr::null_mut::<cff_CharstringInstruction>();
        free(il as *mut ::core::ffi::c_void);
        il = ::core::ptr::null_mut::<cff_CharstringIL>();
        j = j.wrapping_add(1);
    }
    cff_ilGraphToBuffers(&raw mut (*context).graph, s, gs, ls, (*context).options);
}
unsafe extern "C" fn sidof(mut h: *mut *mut cff_sid_entry, mut s: sds) -> ::core::ffi::c_int {
    let mut item: *mut cff_sid_entry = ::core::ptr::null_mut::<cff_sid_entry>();
    let mut _hf_hashv: ::core::ffi::c_uint = 0;
    let mut _hj_i: ::core::ffi::c_uint = 0;
    let mut _hj_j: ::core::ffi::c_uint = 0;
    let mut _hj_k: ::core::ffi::c_uint = 0;
    let mut _hj_key: *const ::core::ffi::c_uchar = s as *const ::core::ffi::c_uchar;
    _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
    _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
    _hj_i = _hj_j;
    _hj_k = strlen(s as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
    while _hj_k >= 12 as ::core::ffi::c_uint {
        _hj_i = _hj_i.wrapping_add(
            (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hj_j = _hj_j.wrapping_add(
            (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hf_hashv = _hf_hashv.wrapping_add(
            (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                .wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                )
                .wrapping_add(
                    (*_hj_key.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                ),
        );
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
        _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
    }
    _hf_hashv =
        _hf_hashv.wrapping_add(strlen(s as *const ::core::ffi::c_char) as ::core::ffi::c_uint);
    let mut current_block_50: u64;
    match _hj_k {
        11 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 2136187349163929158;
        }
        10 => {
            current_block_50 = 2136187349163929158;
        }
        9 => {
            current_block_50 = 15000087230028316373;
        }
        8 => {
            current_block_50 = 12618485878637048149;
        }
        7 => {
            current_block_50 = 10172284732124562867;
        }
        6 => {
            current_block_50 = 10320808845489400712;
        }
        5 => {
            current_block_50 = 17757077319915322283;
        }
        4 => {
            current_block_50 = 3097490847089818784;
        }
        3 => {
            current_block_50 = 6116987625208566775;
        }
        2 => {
            current_block_50 = 13858715045951221004;
        }
        1 => {
            current_block_50 = 18086712884960296808;
        }
        _ => {
            current_block_50 = 18435049525520518667;
        }
    }
    match current_block_50 {
        2136187349163929158 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 15000087230028316373;
        }
        _ => {}
    }
    match current_block_50 {
        15000087230028316373 => {
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 12618485878637048149;
        }
        _ => {}
    }
    match current_block_50 {
        12618485878637048149 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 10172284732124562867;
        }
        _ => {}
    }
    match current_block_50 {
        10172284732124562867 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 10320808845489400712;
        }
        _ => {}
    }
    match current_block_50 {
        10320808845489400712 => {
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 17757077319915322283;
        }
        _ => {}
    }
    match current_block_50 {
        17757077319915322283 => {
            _hj_j = _hj_j.wrapping_add(
                *_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
            current_block_50 = 3097490847089818784;
        }
        _ => {}
    }
    match current_block_50 {
        3097490847089818784 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 24 as ::core::ffi::c_int,
            );
            current_block_50 = 6116987625208566775;
        }
        _ => {}
    }
    match current_block_50 {
        6116987625208566775 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 16 as ::core::ffi::c_int,
            );
            current_block_50 = 13858715045951221004;
        }
        _ => {}
    }
    match current_block_50 {
        13858715045951221004 => {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    << 8 as ::core::ffi::c_int,
            );
            current_block_50 = 18086712884960296808;
        }
        _ => {}
    }
    match current_block_50 {
        18086712884960296808 => {
            _hj_i = _hj_i.wrapping_add(
                *_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint
            );
        }
        _ => {}
    }
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
    _hj_i = _hj_i.wrapping_sub(_hj_j);
    _hj_i = _hj_i.wrapping_sub(_hf_hashv);
    _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
    _hj_j = _hj_j.wrapping_sub(_hf_hashv);
    _hj_j = _hj_j.wrapping_sub(_hj_i);
    _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
    _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
    _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
    item = ::core::ptr::null_mut::<cff_sid_entry>();
    if !(*h).is_null() {
        let mut _hf_bkt: ::core::ffi::c_uint = 0;
        _hf_bkt = _hf_hashv
            & (*(**h).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize))
                .hh_head
                .is_null()
            {
                item = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                    as *mut ::core::ffi::c_char)
                    .offset(-(*(**h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut cff_sid_entry
                    as *mut cff_sid_entry;
            } else {
                item = ::core::ptr::null_mut::<cff_sid_entry>();
            }
            while !item.is_null() {
                if (*item).hh.hashv == _hf_hashv
                    && (*item).hh.keylen
                        == strlen(s as *const ::core::ffi::c_char) as ::core::ffi::c_uint
                {
                    if memcmp(
                        (*item).hh.key,
                        s as *const ::core::ffi::c_void,
                        strlen(s as *const ::core::ffi::c_char) as ::core::ffi::c_uint as usize,
                    ) == 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                if !(*item).hh.hh_next.is_null() {
                    item = ((*item).hh.hh_next as *mut ::core::ffi::c_char)
                        .offset(-(*(**h).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut cff_sid_entry
                        as *mut cff_sid_entry;
                } else {
                    item = ::core::ptr::null_mut::<cff_sid_entry>();
                }
            }
        }
    }
    if !item.is_null() {
        return 391 as ::core::ffi::c_int + (*item).sid;
    } else {
        item = __caryll_allocate_clean(
            ::core::mem::size_of::<cff_sid_entry>() as usize,
            949 as ::core::ffi::c_ulong,
        ) as *mut cff_sid_entry;
        (*item).sid = (if !(*h).is_null() {
            (*(**h).hh.tbl).num_items
        } else {
            0 as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
        (*item).str_0 = sdsdup(s) as *mut ::core::ffi::c_char;
        let mut _ha_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i_0: ::core::ffi::c_uint = 0;
        let mut _hj_j_0: ::core::ffi::c_uint = 0;
        let mut _hj_k_0: ::core::ffi::c_uint = 0;
        let mut _hj_key_0: *const ::core::ffi::c_uchar =
            (*item).str_0.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_uchar;
        _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i_0 = _hj_j_0;
        _hj_k_0 = strlen((*item).str_0) as ::core::ffi::c_uint;
        while _hj_k_0 >= 12 as ::core::ffi::c_uint {
            _hj_i_0 = _hj_i_0.wrapping_add(
                (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j_0 = _hj_j_0.wrapping_add(
                (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _ha_hashv = _ha_hashv.wrapping_add(
                (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key_0.offset(11 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
            _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
            _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _ha_hashv = _ha_hashv.wrapping_add(strlen((*item).str_0) as ::core::ffi::c_uint);
        let mut current_block_169: u64;
        match _hj_k_0 {
            11 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 8249353659195211371;
            }
            10 => {
                current_block_169 = 8249353659195211371;
            }
            9 => {
                current_block_169 = 3890748899046558515;
            }
            8 => {
                current_block_169 = 15908325233356615362;
            }
            7 => {
                current_block_169 = 4798082187389923100;
            }
            6 => {
                current_block_169 = 10664847837258368038;
            }
            5 => {
                current_block_169 = 13472887685141297691;
            }
            4 => {
                current_block_169 = 17259833882719531241;
            }
            3 => {
                current_block_169 = 441531399728294563;
            }
            2 => {
                current_block_169 = 8298588265412241393;
            }
            1 => {
                current_block_169 = 17576006425236317873;
            }
            _ => {
                current_block_169 = 16835199615365683821;
            }
        }
        match current_block_169 {
            8249353659195211371 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 3890748899046558515;
            }
            _ => {}
        }
        match current_block_169 {
            3890748899046558515 => {
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 15908325233356615362;
            }
            _ => {}
        }
        match current_block_169 {
            15908325233356615362 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 4798082187389923100;
            }
            _ => {}
        }
        match current_block_169 {
            4798082187389923100 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 10664847837258368038;
            }
            _ => {}
        }
        match current_block_169 {
            10664847837258368038 => {
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 13472887685141297691;
            }
            _ => {}
        }
        match current_block_169 {
            13472887685141297691 => {
                _hj_j_0 =
                    _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_169 = 17259833882719531241;
            }
            _ => {}
        }
        match current_block_169 {
            17259833882719531241 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_169 = 441531399728294563;
            }
            _ => {}
        }
        match current_block_169 {
            441531399728294563 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_169 = 8298588265412241393;
            }
            _ => {}
        }
        match current_block_169 {
            8298588265412241393 => {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_169 = 17576006425236317873;
            }
            _ => {}
        }
        match current_block_169 {
            17576006425236317873 => {
                _hj_i_0 =
                    _hj_i_0
                        .wrapping_add(*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
        _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
        _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
        _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
        _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
        _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
        _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
        _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
        (*item).hh.hashv = _ha_hashv;
        (*item).hh.key = (*item).str_0.offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*item).hh.keylen = strlen((*item).str_0) as ::core::ffi::c_uint;
        if (*h).is_null() {
            (*item).hh.next = NULL;
            (*item).hh.prev = NULL;
            (*item).hh.tbl = malloc(::core::mem::size_of::<UT_hash_table>() as usize)
                as *mut UT_hash_table as *mut UT_hash_table;
            if (*item).hh.tbl.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    (*item).hh.tbl as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<UT_hash_table>() as usize,
                );
                (*(*item).hh.tbl).tail = &raw mut (*item).hh as *mut UT_hash_handle;
                (*(*item).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                (*(*item).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                (*(*item).hh.tbl).hho = (&raw mut (*item).hh as *mut ::core::ffi::c_char)
                    .offset_from(item as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as isize;
                (*(*item).hh.tbl).buckets = malloc(
                    (32 as usize).wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                ) as *mut UT_hash_bucket;
                (*(*item).hh.tbl).signature = HASH_SIGNATURE as u32;
                if (*(*item).hh.tbl).buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*(*item).hh.tbl).buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                    );
                }
            }
            *h = item;
        } else {
            (*item).hh.tbl = (**h).hh.tbl;
            (*item).hh.next = NULL;
            (*item).hh.prev = ((*(**h).hh.tbl).tail as *mut ::core::ffi::c_char)
                .offset(-(*(**h).hh.tbl).hho)
                as *mut ::core::ffi::c_void;
            (*(*(**h).hh.tbl).tail).next = item as *mut ::core::ffi::c_void;
            (*(**h).hh.tbl).tail = &raw mut (*item).hh as *mut UT_hash_handle;
        }
        let mut _ha_bkt: ::core::ffi::c_uint = 0;
        (*(**h).hh.tbl).num_items = (*(**h).hh.tbl).num_items.wrapping_add(1);
        _ha_bkt = _ha_hashv
            & (*(**h).hh.tbl)
                .num_buckets
                .wrapping_sub(1 as ::core::ffi::c_uint);
        let mut _ha_head: *mut UT_hash_bucket =
            (*(**h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UT_hash_bucket;
        (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
        (*item).hh.hh_next = (*_ha_head).hh_head as *mut UT_hash_handle;
        (*item).hh.hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
        if !(*_ha_head).hh_head.is_null() {
            (*(*_ha_head).hh_head).hh_prev = &raw mut (*item).hh as *mut UT_hash_handle;
        }
        (*_ha_head).hh_head = &raw mut (*item).hh as *mut UT_hash_handle;
        if (*_ha_head).count
            >= (*_ha_head)
                .expand_mult
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
            && (*(*item).hh.tbl).noexpand == 0
        {
            let mut _he_bkt: ::core::ffi::c_uint = 0;
            let mut _he_bkt_i: ::core::ffi::c_uint = 0;
            let mut _he_thh: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_hh_nxt: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
            let mut _he_new_buckets: *mut UT_hash_bucket =
                ::core::ptr::null_mut::<UT_hash_bucket>();
            let mut _he_newbkt: *mut UT_hash_bucket = ::core::ptr::null_mut::<UT_hash_bucket>();
            _he_new_buckets = malloc(
                (2 as usize)
                    .wrapping_mul((*(*item).hh.tbl).num_buckets as usize)
                    .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
            ) as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as ::core::ffi::c_int));
            } else {
                memset(
                    _he_new_buckets as *mut ::core::ffi::c_void,
                    '\0' as i32,
                    (2 as usize)
                        .wrapping_mul((*(*item).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UT_hash_bucket>() as usize),
                );
                (*(*item).hh.tbl).ideal_chain_maxlen = ((*(*item).hh.tbl).num_items
                    >> (*(*item).hh.tbl)
                        .log2_num_buckets
                        .wrapping_add(1 as ::core::ffi::c_uint))
                .wrapping_add(
                    if (*(*item).hh.tbl).num_items
                        & (*(*item).hh.tbl)
                            .num_buckets
                            .wrapping_mul(2 as ::core::ffi::c_uint)
                            .wrapping_sub(1 as ::core::ffi::c_uint)
                        != 0 as ::core::ffi::c_uint
                    {
                        1 as ::core::ffi::c_uint
                    } else {
                        0 as ::core::ffi::c_uint
                    },
                );
                (*(*item).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                _he_bkt_i = 0 as ::core::ffi::c_uint;
                while _he_bkt_i < (*(*item).hh.tbl).num_buckets {
                    _he_thh = (*(*(*item).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                        as *mut UT_hash_handle;
                    while !_he_thh.is_null() {
                        _he_hh_nxt = (*_he_thh).hh_next;
                        _he_bkt = (*_he_thh).hashv
                            & (*(*item).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint);
                        _he_newbkt =
                            _he_new_buckets.offset(_he_bkt as isize) as *mut UT_hash_bucket;
                        (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                        if (*_he_newbkt).count > (*(*item).hh.tbl).ideal_chain_maxlen {
                            (*(*item).hh.tbl).nonideal_items =
                                (*(*item).hh.tbl).nonideal_items.wrapping_add(1);
                            (*_he_newbkt).expand_mult = (*_he_newbkt)
                                .count
                                .wrapping_div((*(*item).hh.tbl).ideal_chain_maxlen);
                        }
                        (*_he_thh).hh_prev = ::core::ptr::null_mut::<UT_hash_handle>();
                        (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UT_hash_handle;
                        if !(*_he_newbkt).hh_head.is_null() {
                            (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                        }
                        (*_he_newbkt).hh_head = _he_thh as *mut UT_hash_handle;
                        _he_thh = _he_hh_nxt;
                    }
                    _he_bkt_i = _he_bkt_i.wrapping_add(1);
                }
                free((*(*item).hh.tbl).buckets as *mut ::core::ffi::c_void);
                (*(*item).hh.tbl).num_buckets = (*(*item).hh.tbl)
                    .num_buckets
                    .wrapping_mul(2 as ::core::ffi::c_uint);
                (*(*item).hh.tbl).log2_num_buckets =
                    (*(*item).hh.tbl).log2_num_buckets.wrapping_add(1);
                (*(*item).hh.tbl).buckets = _he_new_buckets;
                (*(*item).hh.tbl).ineff_expands = if (*(*item).hh.tbl).nonideal_items
                    > (*(*item).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                {
                    (*(*item).hh.tbl)
                        .ineff_expands
                        .wrapping_add(1 as ::core::ffi::c_uint)
                } else {
                    0 as ::core::ffi::c_uint
                };
                if (*(*item).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                    (*(*item).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                }
            }
        }
        return 391 as ::core::ffi::c_int + (*item).sid;
    };
}
unsafe extern "C" fn cffdict_givemeablank(mut dict: *mut cff_Dict) -> *mut cff_DictEntry {
    (*dict).count = (*dict).count.wrapping_add(1);
    (*dict).ents = __caryll_reallocate(
        (*dict).ents as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<cff_DictEntry>() as usize).wrapping_mul((*dict).count as usize),
        959 as ::core::ffi::c_ulong,
    ) as *mut cff_DictEntry;
    return (*dict)
        .ents
        .offset((*dict).count.wrapping_sub(1 as u32) as isize)
        as *mut cff_DictEntry;
}
/// Append a DICT entry whose operands are numbers.
///
/// Was `cffdict_input(dict, op, t, arity, ...)`: a count, a value type, and that
/// many varargs read as `c_double` or `c_int` depending on `t`. Every one of the
/// 30 call sites passes either `cff_DOUBLE` with `pos_t` operands or
/// `cff_INTEGER` with integer ones, so the runtime branch on `t` is really two
/// functions -- this one and [`cffdict_input_ints`] -- and the count is the
/// slice's length.
unsafe fn cffdict_input_doubles(dict: *mut cff_Dict, op: u32, values: &[f64]) {
    let last: *mut cff_DictEntry = cffdict_givemeablank(dict);
    (*last).op = op;
    (*last).cnt = values.len() as u32;
    (*last).vals = __caryll_allocate_clean(
        (::core::mem::size_of::<cff_Value>() as usize).wrapping_mul(values.len()),
        966 as ::core::ffi::c_ulong,
    ) as *mut cff_Value;
    for (j, &x) in values.iter().enumerate() {
        let slot = (*last).vals.add(j);
        // A whole number is stored as an integer, which is what decides whether
        // the DICT is encoded with an integer or a real operand later.
        if x == round(x) {
            (*slot).t = cff_INTEGER;
            (*slot).c2rust_unnamed.i = round(x) as i32;
        } else {
            (*slot).t = cff_DOUBLE;
            (*slot).c2rust_unnamed.d = x;
        }
    }
}

/// Append a DICT entry whose operands are integers. See [`cffdict_input_doubles`].
unsafe fn cffdict_input_ints(dict: *mut cff_Dict, op: u32, values: &[i32]) {
    let last: *mut cff_DictEntry = cffdict_givemeablank(dict);
    (*last).op = op;
    (*last).cnt = values.len() as u32;
    (*last).vals = __caryll_allocate_clean(
        (::core::mem::size_of::<cff_Value>() as usize).wrapping_mul(values.len()),
        966 as ::core::ffi::c_ulong,
    ) as *mut cff_Value;
    for (j, &x) in values.iter().enumerate() {
        let slot = (*last).vals.add(j);
        (*slot).t = cff_INTEGER;
        (*slot).c2rust_unnamed.i = x;
    }
}

unsafe extern "C" fn cffdict_input_array(
    mut dict: *mut cff_Dict,
    mut op: u32,
    mut t: cff_Value_Type,
    mut arity: arity_t,
    mut arr: *mut ::core::ffi::c_double,
) {
    if arity == 0 || arr.is_null() {
        return;
    }
    let mut last: *mut cff_DictEntry = cffdict_givemeablank(dict);
    (*last).op = op;
    (*last).cnt = arity as u32;
    (*last).vals = __caryll_allocate_clean(
        (::core::mem::size_of::<cff_Value>() as usize).wrapping_mul(arity as usize),
        994 as ::core::ffi::c_ulong,
    ) as *mut cff_Value;
    let mut j: arity_t = 0 as arity_t;
    while j < arity {
        let mut x: ::core::ffi::c_double = *arr.offset(j as isize);
        if t as ::core::ffi::c_uint == cff_DOUBLE as ::core::ffi::c_int as ::core::ffi::c_uint {
            if x == round(x) {
                (*(*last).vals.offset(j as isize)).t = cff_INTEGER;
                (*(*last).vals.offset(j as isize)).c2rust_unnamed.i = round(x) as i32;
            } else {
                (*(*last).vals.offset(j as isize)).t = cff_DOUBLE;
                (*(*last).vals.offset(j as isize)).c2rust_unnamed.d = x;
            }
        } else {
            (*(*last).vals.offset(j as isize)).t = t;
            (*(*last).vals.offset(j as isize)).c2rust_unnamed.i = round(x) as i32;
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn cff_make_fd_dict(
    mut fd: *mut table_CFF,
    mut h: *mut *mut cff_sid_entry,
) -> *mut cff_Dict {
    let mut dict: *mut cff_Dict = (
        cff_iDict.create.expect("non-null function pointer"))();
    if !(*fd).cidRegistry.is_null() && !(*fd).cidOrdering.is_null() {
        cffdict_input_ints(dict, op_ROS as ::core::ffi::c_int as u32, &[(sidof(h, (*fd).cidRegistry)) as i32, (sidof(h, (*fd).cidOrdering)) as i32, ((*fd).cidSupplement) as i32]);
    }
    if !(*fd).version.is_null() {
        cffdict_input_ints(dict, op_version as ::core::ffi::c_int as u32, &[(sidof(h, (*fd).version)) as i32]);
    }
    if !(*fd).notice.is_null() {
        cffdict_input_ints(dict, op_Notice as ::core::ffi::c_int as u32, &[(sidof(h, (*fd).notice)) as i32]);
    }
    if !(*fd).copyright.is_null() {
        cffdict_input_ints(dict, op_Copyright as ::core::ffi::c_int as u32, &[(sidof(h, (*fd).copyright)) as i32]);
    }
    if !(*fd).fullName.is_null() {
        cffdict_input_ints(dict, op_FullName as ::core::ffi::c_int as u32, &[(sidof(h, (*fd).fullName)) as i32]);
    }
    if !(*fd).familyName.is_null() {
        cffdict_input_ints(dict, op_FamilyName as ::core::ffi::c_int as u32, &[(sidof(h, (*fd).familyName)) as i32]);
    }
    if !(*fd).weight.is_null() {
        cffdict_input_ints(dict, op_Weight as ::core::ffi::c_int as u32, &[(sidof(h, (*fd).weight)) as i32]);
    }
    cffdict_input_doubles(dict, op_FontBBox as ::core::ffi::c_int as u32, &[((*fd).fontBBoxLeft) as f64, ((*fd).fontBBoxBottom) as f64, ((*fd).fontBBoxRight) as f64, ((*fd).fontBBoxTop) as f64]);
    cffdict_input_ints(dict, op_isFixedPitch as ::core::ffi::c_int as u32, &[((*fd).isFixedPitch as ::core::ffi::c_int) as i32]);
    cffdict_input_doubles(dict, op_ItalicAngle as ::core::ffi::c_int as u32, &[((*fd).italicAngle) as f64]);
    cffdict_input_doubles(dict, op_UnderlinePosition as ::core::ffi::c_int as u32, &[((*fd).underlinePosition) as f64]);
    cffdict_input_doubles(dict, op_UnderlineThickness as ::core::ffi::c_int as u32, &[((*fd).underlineThickness) as f64]);
    cffdict_input_doubles(dict, op_StrokeWidth as ::core::ffi::c_int as u32, &[((*fd).strokeWidth) as f64]);
    if !(*fd).fontMatrix.is_null() {
        cffdict_input_doubles(dict, op_FontMatrix as ::core::ffi::c_int as u32, &[((*(*fd).fontMatrix).a) as f64, ((*(*fd).fontMatrix).b) as f64, ((*(*fd).fontMatrix).c) as f64, ((*(*fd).fontMatrix).d) as f64, (iVQ.getStill.expect("non-null function pointer")((*(*fd).fontMatrix).x)) as f64, (iVQ.getStill.expect("non-null function pointer")((*(*fd).fontMatrix).y)) as f64]);
    }
    if !(*fd).fontName.is_null() {
        cffdict_input_ints(dict, op_FontName as ::core::ffi::c_int as u32, &[(sidof(h, (*fd).fontName)) as i32]);
    }
    if (*fd).cidFontVersion != 0. {
        cffdict_input_doubles(dict, op_CIDFontVersion as ::core::ffi::c_int as u32, &[((*fd).cidFontVersion) as f64]);
    }
    if (*fd).cidFontRevision != 0. {
        cffdict_input_doubles(dict, op_CIDFontRevision as ::core::ffi::c_int as u32, &[((*fd).cidFontRevision) as f64]);
    }
    if (*fd).cidCount != 0 {
        cffdict_input_ints(dict, op_CIDCount as ::core::ffi::c_int as u32, &[((*fd).cidCount) as i32]);
    }
    if (*fd).UIDBase != 0 {
        cffdict_input_ints(dict, op_UIDBase as ::core::ffi::c_int as u32, &[((*fd).UIDBase) as i32]);
    }
    return dict;
}
unsafe extern "C" fn cff_make_private_dict(mut pd: *mut cff_PrivateDict) -> *mut cff_Dict {
    let mut dict: *mut cff_Dict = ::core::ptr::null_mut::<cff_Dict>();
    dict = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_Dict>() as usize,
        1057 as ::core::ffi::c_ulong,
    ) as *mut cff_Dict;
    if pd.is_null() {
        return dict;
    }
    cffdict_input_array(
        dict,
        op_BlueValues as ::core::ffi::c_int as u32,
        cff_DOUBLE,
        (*pd).blueValuesCount,
        (*pd).blueValues,
    );
    cffdict_input_array(
        dict,
        op_OtherBlues as ::core::ffi::c_int as u32,
        cff_DOUBLE,
        (*pd).otherBluesCount,
        (*pd).otherBlues,
    );
    cffdict_input_array(
        dict,
        op_FamilyBlues as ::core::ffi::c_int as u32,
        cff_DOUBLE,
        (*pd).familyBluesCount,
        (*pd).familyBlues,
    );
    cffdict_input_array(
        dict,
        op_FamilyOtherBlues as ::core::ffi::c_int as u32,
        cff_DOUBLE,
        (*pd).familyOtherBluesCount,
        (*pd).familyOtherBlues,
    );
    cffdict_input_array(
        dict,
        op_StemSnapH as ::core::ffi::c_int as u32,
        cff_DOUBLE,
        (*pd).stemSnapHCount,
        (*pd).stemSnapH,
    );
    cffdict_input_array(
        dict,
        op_StemSnapV as ::core::ffi::c_int as u32,
        cff_DOUBLE,
        (*pd).stemSnapVCount,
        (*pd).stemSnapV,
    );
    cffdict_input_doubles(dict, op_BlueScale as ::core::ffi::c_int as u32, &[((*pd).blueScale) as f64]);
    cffdict_input_doubles(dict, op_BlueShift as ::core::ffi::c_int as u32, &[((*pd).blueShift) as f64]);
    cffdict_input_doubles(dict, op_BlueFuzz as ::core::ffi::c_int as u32, &[((*pd).blueFuzz) as f64]);
    cffdict_input_doubles(dict, op_StdHW as ::core::ffi::c_int as u32, &[((*pd).stdHW) as f64]);
    cffdict_input_doubles(dict, op_StdVW as ::core::ffi::c_int as u32, &[((*pd).stdVW) as f64]);
    cffdict_input_ints(dict, op_ForceBold as ::core::ffi::c_int as u32, &[((*pd).forceBold as ::core::ffi::c_int) as i32]);
    cffdict_input_ints(dict, op_LanguageGroup as ::core::ffi::c_int as u32, &[((*pd).languageGroup) as i32]);
    cffdict_input_doubles(dict, op_ExpansionFactor as ::core::ffi::c_int as u32, &[((*pd).expansionFactor) as f64]);
    cffdict_input_doubles(dict, op_initialRandomSeed as ::core::ffi::c_int as u32, &[((*pd).initialRandomSeed) as f64]);
    cffdict_input_doubles(dict, op_defaultWidthX as ::core::ffi::c_int as u32, &[((*pd).defaultWidthX) as f64]);
    cffdict_input_doubles(dict, op_nominalWidthX as ::core::ffi::c_int as u32, &[((*pd).nominalWidthX) as f64]);
    return dict;
}
unsafe extern "C" fn by_sid(
    mut a: *mut cff_sid_entry,
    mut b: *mut cff_sid_entry,
) -> ::core::ffi::c_int {
    return (*a).sid - (*b).sid;
}
unsafe extern "C" fn callback_makestringindex(
    mut context: *mut ::core::ffi::c_void,
    mut i: u32,
) -> *mut caryll_Buffer {
    let mut blobs: *mut *mut caryll_Buffer = context as *mut *mut caryll_Buffer;
    return *blobs.offset(i as isize);
}
unsafe extern "C" fn cffstrings_to_indexblob(mut h: *mut *mut cff_sid_entry) -> *mut caryll_Buffer {
    let mut _hs_i: ::core::ffi::c_uint = 0;
    let mut _hs_looping: ::core::ffi::c_uint = 0;
    let mut _hs_nmerges: ::core::ffi::c_uint = 0;
    let mut _hs_insize: ::core::ffi::c_uint = 0;
    let mut _hs_psize: ::core::ffi::c_uint = 0;
    let mut _hs_qsize: ::core::ffi::c_uint = 0;
    let mut _hs_p: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_q: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_e: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_list: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    let mut _hs_tail: *mut UT_hash_handle = ::core::ptr::null_mut::<UT_hash_handle>();
    if !(*h).is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (**h).hh as *mut UT_hash_handle;
        while _hs_looping != 0 as ::core::ffi::c_uint {
            _hs_p = _hs_list;
            _hs_list = ::core::ptr::null_mut::<UT_hash_handle>();
            _hs_tail = ::core::ptr::null_mut::<UT_hash_handle>();
            _hs_nmerges = 0 as ::core::ffi::c_uint;
            while !_hs_p.is_null() {
                _hs_nmerges = _hs_nmerges.wrapping_add(1);
                _hs_q = _hs_p;
                _hs_psize = 0 as ::core::ffi::c_uint;
                _hs_i = 0 as ::core::ffi::c_uint;
                while _hs_i < _hs_insize {
                    _hs_psize = _hs_psize.wrapping_add(1);
                    _hs_q = (if !(*_hs_q).next.is_null() {
                        ((*_hs_q).next as *mut ::core::ffi::c_char)
                            .offset((*(**h).hh.tbl).hho)
                            as *mut UT_hash_handle
                    } else {
                        ::core::ptr::null_mut::<UT_hash_handle>()
                    }) as *mut UT_hash_handle;
                    if _hs_q.is_null() {
                        break;
                    }
                    _hs_i = _hs_i.wrapping_add(1);
                }
                _hs_qsize = _hs_insize;
                while _hs_psize != 0 as ::core::ffi::c_uint
                    || _hs_qsize != 0 as ::core::ffi::c_uint && !_hs_q.is_null()
                {
                    if _hs_psize == 0 as ::core::ffi::c_uint {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(**h).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    } else if _hs_qsize == 0 as ::core::ffi::c_uint || _hs_q.is_null() {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(**h).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if by_sid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut cff_sid_entry,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut cff_sid_entry,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(**h).hh.tbl).hho)
                                    as *mut UT_hash_handle
                            } else {
                                ::core::ptr::null_mut::<UT_hash_handle>()
                            }) as *mut UT_hash_handle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(**h).hh.tbl).hho)
                                as *mut UT_hash_handle
                        } else {
                            ::core::ptr::null_mut::<UT_hash_handle>()
                        }) as *mut UT_hash_handle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    } else {
                        _hs_list = _hs_e;
                    }
                    if !_hs_e.is_null() {
                        (*_hs_e).prev = if !_hs_tail.is_null() {
                            (_hs_tail as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    }
                    _hs_tail = _hs_e;
                }
                _hs_p = _hs_q;
            }
            if !_hs_tail.is_null() {
                (*_hs_tail).next = NULL;
            }
            if _hs_nmerges <= 1 as ::core::ffi::c_uint {
                _hs_looping = 0 as ::core::ffi::c_uint;
                (*(**h).hh.tbl).tail = _hs_tail;
                *h = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut cff_sid_entry
                    as *mut cff_sid_entry;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut blobs: *mut *mut caryll_Buffer = ::core::ptr::null_mut::<*mut caryll_Buffer>();
    let mut n: u32 = if !(*h).is_null() {
        (*(**h).hh.tbl).num_items as u32
    } else {
        0 as u32
    };
    blobs = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut caryll_Buffer>() as usize).wrapping_mul(n as usize),
        1097 as ::core::ffi::c_ulong,
    ) as *mut *mut caryll_Buffer;
    let mut j: u32 = 0 as u32;
    let mut item: *mut cff_sid_entry = ::core::ptr::null_mut::<cff_sid_entry>();
    let mut tmp: *mut cff_sid_entry = ::core::ptr::null_mut::<cff_sid_entry>();
    item = *h;
    tmp = (if !(*h).is_null() { (**h).hh.next } else { NULL }) as *mut cff_sid_entry
        as *mut cff_sid_entry;
    while !item.is_null() {
        let ref mut fresh15 = *blobs.offset(j as isize);
        *fresh15 = bufnew();
        bufwrite_sds(*blobs.offset(j as isize), (*item).str_0 as sds);
        let mut _hd_hh_del: *mut UT_hash_handle = &raw mut (*item).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(**h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((**h).hh.tbl as *mut ::core::ffi::c_void);
            *h = ::core::ptr::null_mut::<cff_sid_entry>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(**h).hh.tbl).tail {
                (*(**h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(**h).hh.tbl).hho)
                    as *mut UT_hash_handle
                    as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh16 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(**h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .next;
                *fresh16 = (*_hd_hh_del).next;
            } else {
                *h = (*_hd_hh_del).next as *mut cff_sid_entry as *mut cff_sid_entry;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh17 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(**h).hh.tbl).hho)
                    as *mut UT_hash_handle))
                    .prev;
                *fresh17 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(**h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UT_hash_bucket =
                (*(**h).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UT_hash_bucket;
            (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
            if (*_hd_head).hh_head == _hd_hh_del {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UT_hash_handle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(**h).hh.tbl).num_items = (*(**h).hh.tbl).num_items.wrapping_sub(1);
        }
        sdsfree((*item).str_0 as sds);
        free(item as *mut ::core::ffi::c_void);
        item = ::core::ptr::null_mut::<cff_sid_entry>();
        j = j.wrapping_add(1);
        item = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut cff_sid_entry
            as *mut cff_sid_entry;
    }
    let mut strings: *mut cff_Index = cff_iIndex.fromCallback.expect("non-null function pointer")(
        blobs as *mut ::core::ffi::c_void,
        n,
        Some(
            callback_makestringindex
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut caryll_Buffer,
        ),
    );
    free(blobs as *mut ::core::ffi::c_void);
    blobs = ::core::ptr::null_mut::<*mut caryll_Buffer>();
    let mut final_blob: *mut caryll_Buffer =
        cff_iIndex.build.expect("non-null function pointer")(strings);
    cff_iIndex.free.expect("non-null function pointer")(strings);
    (*final_blob).cursor = (*final_blob).size;
    return final_blob;
}
unsafe extern "C" fn cff_compile_nameindex(mut cff: *mut table_CFF) -> *mut caryll_Buffer {
    let mut nameIndex: *mut cff_Index = (
        cff_iIndex.create.expect("non-null function pointer"))();
    (*nameIndex).count = 1 as arity_t;
    (*nameIndex).offSize = 4 as u8;
    (*nameIndex).offset = __caryll_allocate_clean(
        (::core::mem::size_of::<u32>() as usize).wrapping_mul(2 as usize),
        1121 as ::core::ffi::c_ulong,
    ) as *mut u32;
    if (*cff).fontName.is_null() {
        (*cff).fontName = sdsnew(b"Caryll-CFF-FONT\0" as *const u8 as *const ::core::ffi::c_char);
    }
    *(*nameIndex).offset.offset(0 as ::core::ffi::c_int as isize) = 1 as u32;
    *(*nameIndex).offset.offset(1 as ::core::ffi::c_int as isize) =
        sdslen((*cff).fontName).wrapping_add(1 as usize) as u32;
    (*nameIndex).data = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize)
            .wrapping_mul((1 as usize).wrapping_add(sdslen((*cff).fontName))),
        1125 as ::core::ffi::c_ulong,
    ) as *mut u8;
    memcpy(
        (*nameIndex).data as *mut ::core::ffi::c_void,
        (*cff).fontName as *const ::core::ffi::c_void,
        sdslen((*cff).fontName),
    );
    let mut buf: *mut caryll_Buffer =
        cff_iIndex.build.expect("non-null function pointer")(nameIndex);
    cff_iIndex.free.expect("non-null function pointer")(nameIndex);
    if !(*cff).fontName.is_null() {
        sdsfree((*cff).fontName);
        (*cff).fontName = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return buf;
}
unsafe extern "C" fn cff_make_charset(
    mut cff: *mut table_CFF,
    mut glyf: *mut table_glyf,
    mut stringHash: *mut *mut cff_sid_entry,
) -> *mut caryll_Buffer {
    let mut charset: *mut cff_Charset = ::core::ptr::null_mut::<cff_Charset>();
    charset = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_Charset>() as usize,
        1140 as ::core::ffi::c_ulong,
    ) as *mut cff_Charset;
    if (*glyf).length > 1 as usize {
        (*charset).t = cff_CHARSET_FORMAT2 as ::core::ffi::c_int as u32;
        (*charset).s = 1 as u32;
        (*charset).c2rust_unnamed.f2.format = 2 as u8;
        (*charset).c2rust_unnamed.f2.range2 = __caryll_allocate_clean(
            ::core::mem::size_of::<cff_CharsetRangeFormat2>() as usize,
            1145 as ::core::ffi::c_ulong,
        ) as *mut cff_CharsetRangeFormat2;
        if (*cff).isCID {
            (*(*charset)
                .c2rust_unnamed
                .f2
                .range2
                .offset(0 as ::core::ffi::c_int as isize))
            .first = 1 as u16;
            (*(*charset)
                .c2rust_unnamed
                .f2
                .range2
                .offset(0 as ::core::ffi::c_int as isize))
            .nleft = (*glyf).length.wrapping_sub(2 as usize) as u16;
        } else {
            let mut j: glyphid_t = 1 as glyphid_t;
            while (j as usize) < (*glyf).length {
                sidof(stringHash, (**(*glyf).items.offset(j as isize)).name);
                j = j.wrapping_add(1);
            }
            (*(*charset)
                .c2rust_unnamed
                .f2
                .range2
                .offset(0 as ::core::ffi::c_int as isize))
            .first = sidof(
                stringHash,
                (**(*glyf).items.offset(1 as ::core::ffi::c_int as isize)).name,
            ) as u16;
            (*(*charset)
                .c2rust_unnamed
                .f2
                .range2
                .offset(0 as ::core::ffi::c_int as isize))
            .nleft = (*glyf).length.wrapping_sub(2 as usize) as u16;
        }
    } else {
        (*charset).t = cff_CHARSET_ISOADOBE as ::core::ffi::c_int as u32;
    }
    let mut c: *mut caryll_Buffer = cff_build_Charset(*charset);
    if (*charset).t == cff_CHARSET_FORMAT2 as ::core::ffi::c_int as u32 {
        free((*charset).c2rust_unnamed.f2.range2 as *mut ::core::ffi::c_void);
        (*charset).c2rust_unnamed.f2.range2 = ::core::ptr::null_mut::<cff_CharsetRangeFormat2>();
    }
    free(charset as *mut ::core::ffi::c_void);
    charset = ::core::ptr::null_mut::<cff_Charset>();
    return c;
}
unsafe extern "C" fn cff_make_fdselect(
    mut cff: *mut table_CFF,
    mut glyf: *mut table_glyf,
) -> *mut caryll_Buffer {
    let mut fdi0: u8 = 0;
    if !(*cff).isCID {
        return bufnew();
    }
    let mut ranges: u32 = 1 as u32;
    let mut current: u8 = 0 as u8;
    let mut fds: *mut cff_FDSelect = ::core::ptr::null_mut::<cff_FDSelect>();
    fds = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_FDSelect>() as usize,
        1171 as ::core::ffi::c_ulong,
    ) as *mut cff_FDSelect;
    (*fds).t = cff_FDSELECT_UNSPECED as ::core::ffi::c_int as u32;
    if !((*glyf).length == 0) {
        fdi0 = (**(*glyf).items.offset(0 as ::core::ffi::c_int as isize))
            .fdSelect
            .index as u8;
        if fdi0 as ::core::ffi::c_int > (*cff).fdArrayCount as ::core::ffi::c_int {
            fdi0 = 0 as u8;
        }
        current = fdi0;
        let mut j: glyphid_t = 1 as glyphid_t;
        while (j as usize) < (*glyf).length {
            let mut fdi: u8 = (**(*glyf).items.offset(j as isize)).fdSelect.index as u8;
            if fdi as ::core::ffi::c_int > (*cff).fdArrayCount as ::core::ffi::c_int {
                fdi = 0 as u8;
            }
            if fdi as ::core::ffi::c_int != current as ::core::ffi::c_int {
                current = fdi;
                ranges = ranges.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        (*fds).c2rust_unnamed.f3.range3 = __caryll_allocate_clean(
            (::core::mem::size_of::<cff_FDSelectRangeFormat3>() as usize)
                .wrapping_mul(ranges as usize),
            1185 as ::core::ffi::c_ulong,
        ) as *mut cff_FDSelectRangeFormat3;
        (*(*fds)
            .c2rust_unnamed
            .f3
            .range3
            .offset(0 as ::core::ffi::c_int as isize))
        .first = 0 as u16;
        current = fdi0;
        (*(*fds)
            .c2rust_unnamed
            .f3
            .range3
            .offset(0 as ::core::ffi::c_int as isize))
        .fd = current;
        let mut j_0: glyphid_t = 1 as glyphid_t;
        while (j_0 as usize) < (*glyf).length {
            let mut fdi_0: u8 =
                (**(*glyf).items.offset(j_0 as isize)).fdSelect.index as u8;
            if fdi_0 as ::core::ffi::c_int > (*cff).fdArrayCount as ::core::ffi::c_int {
                fdi_0 = 0 as u8;
            }
            if (**(*glyf).items.offset(j_0 as isize)).fdSelect.index as ::core::ffi::c_int
                != current as ::core::ffi::c_int
            {
                current = fdi_0;
                (*fds).s = (*fds).s.wrapping_add(1);
                (*(*fds).c2rust_unnamed.f3.range3.offset((*fds).s as isize)).first =
                    j_0 as u16;
                (*(*fds).c2rust_unnamed.f3.range3.offset((*fds).s as isize)).fd = current;
            }
            j_0 = j_0.wrapping_add(1);
        }
        (*fds).t = cff_FDSELECT_FORMAT3 as ::core::ffi::c_int as u32;
        (*fds).s = ranges;
        (*fds).c2rust_unnamed.f3.format = 3 as u8;
        (*fds).c2rust_unnamed.f3.nranges = ranges as u16;
        (*fds).c2rust_unnamed.f3.sentinel = (*glyf).length as u16;
    }
    let mut e: *mut caryll_Buffer = cff_build_FDSelect(*fds);
    cff_close_FDSelect(*fds);
    free(fds as *mut ::core::ffi::c_void);
    fds = ::core::ptr::null_mut::<cff_FDSelect>();
    return e;
}
unsafe extern "C" fn callback_makefd(
    mut _context: *mut ::core::ffi::c_void,
    mut i: u32,
) -> *mut caryll_Buffer {
    let mut context: *mut fdarray_compile_context = _context as *mut fdarray_compile_context;
    let mut fd: *mut cff_Dict = cff_make_fd_dict(
        *(*context).fdArray.offset(i as isize),
        (*context).stringHash,
    );
    let mut blob: *mut caryll_Buffer = cff_iDict.build.expect("non-null function pointer")(fd);
    bufwrite_bufdel(
        blob,
        cff_buildOffset(0xeeeeeeee as ::core::ffi::c_uint as i32),
    );
    bufwrite_bufdel(
        blob,
        cff_buildOffset(0xffffffff as ::core::ffi::c_uint as i32),
    );
    bufwrite_bufdel(
        blob,
        cff_encodeCffOperator(op_Private as ::core::ffi::c_int as i32),
    );
    cff_iDict.build.expect("non-null function pointer")(fd);
    return blob;
}
unsafe extern "C" fn cff_make_fdarray(
    mut fdArrayCount: tableid_t,
    mut fdArray: *mut *mut table_CFF,
    mut stringHash: *mut *mut cff_sid_entry,
) -> *mut cff_Index {
    let mut context: fdarray_compile_context = fdarray_compile_context {
        fdArray: ::core::ptr::null_mut::<*mut table_CFF>(),
        stringHash: ::core::ptr::null_mut::<*mut cff_sid_entry>(),
    };
    context.fdArray = fdArray;
    context.stringHash = stringHash;
    return cff_iIndex.fromCallback.expect("non-null function pointer")(
        &raw mut context as *mut ::core::ffi::c_void,
        fdArrayCount as u32,
        Some(
            callback_makefd
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut caryll_Buffer,
        ),
    );
}
unsafe extern "C" fn writecff_CIDKeyed(
    mut cff: *mut table_CFF,
    mut glyf: *mut table_glyf,
    mut options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    let mut blob: *mut caryll_Buffer = bufnew();
    let mut stringHash: *mut cff_sid_entry = ::core::ptr::null_mut::<cff_sid_entry>();
    let mut h: *mut caryll_Buffer = cff_buildHeader();
    let mut n: *mut caryll_Buffer = cff_compile_nameindex(cff);
    let mut top: *mut cff_Dict = cff_make_fd_dict(cff, &raw mut stringHash);
    let mut t: *mut caryll_Buffer = cff_iDict.build.expect("non-null function pointer")(top);
    cff_iDict.free.expect("non-null function pointer")(top);
    let mut top_pd: *mut cff_Dict = cff_make_private_dict((*cff).privateDict);
    let mut p: *mut caryll_Buffer = cff_iDict.build.expect("non-null function pointer")(top_pd);
    bufwrite_bufdel(
        p,
        cff_buildOffset(0xffffffff as ::core::ffi::c_uint as i32),
    );
    bufwrite_bufdel(
        p,
        cff_encodeCffOperator(op_Subrs as ::core::ffi::c_int as i32),
    );
    cff_iDict.free.expect("non-null function pointer")(top_pd);
    let mut e: *mut caryll_Buffer = cff_make_fdselect(cff, glyf);
    let mut fdArrayIndex: *mut cff_Index = ::core::ptr::null_mut::<cff_Index>();
    let mut r: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    if (*cff).isCID {
        fdArrayIndex = cff_make_fdarray((*cff).fdArrayCount, (*cff).fdArray, &raw mut stringHash);
        r = cff_iIndex.build.expect("non-null function pointer")(fdArrayIndex);
    } else {
        r = __caryll_allocate_clean(
            ::core::mem::size_of::<caryll_Buffer>() as usize,
            1265 as ::core::ffi::c_ulong,
        ) as *mut caryll_Buffer;
    }
    let mut c: *mut caryll_Buffer = cff_make_charset(cff, glyf, &raw mut stringHash);
    let mut i: *mut caryll_Buffer = cffstrings_to_indexblob(&raw mut stringHash);
    let mut s: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    let mut gs: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    let mut ls: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    let mut g2cContext: cff_charstring_builder_context = cff_charstring_builder_context {
        glyf: ::core::ptr::null_mut::<table_glyf>(),
        defaultWidth: 0,
        nominalWidthX: 0,
        options: ::core::ptr::null::<otfcc_Options>(),
        graph: cff_SubrGraph {
            root: ::core::ptr::null_mut::<cff_SubrRule>(),
            last: ::core::ptr::null_mut::<cff_SubrRule>(),
            diagramIndex: ::core::ptr::null_mut::<cff_SubrDiagramIndex>(),
            totalRules: 0,
            totalCharStrings: 0,
            doSubroutinize: false,
        },
    };
    g2cContext.glyf = glyf;
    g2cContext.defaultWidth = (*(*cff).privateDict).defaultWidthX as u16;
    g2cContext.nominalWidthX = (*(*cff).privateDict).nominalWidthX as u16;
    g2cContext.options = options;
    cff_iSubrGraph.init.expect("non-null function pointer")(&raw mut g2cContext.graph);
    g2cContext.graph.doSubroutinize = (*options).cff_doSubroutinize;
    cff_make_charstrings(&raw mut g2cContext, &raw mut s, &raw mut gs, &raw mut ls);
    cff_iSubrGraph.dispose.expect("non-null function pointer")(&raw mut g2cContext.graph);
    let mut additionalTopDictOpsSize: u32 = 0 as u32;
    let mut off: u32 = (*h)
        .size
        .wrapping_add((*n).size)
        .wrapping_add(11 as usize)
        .wrapping_add((*t).size) as u32;
    if (*c).size != 0 as usize {
        additionalTopDictOpsSize = additionalTopDictOpsSize.wrapping_add(6 as u32);
    }
    if (*e).size != 0 as usize {
        additionalTopDictOpsSize = additionalTopDictOpsSize.wrapping_add(7 as u32);
    }
    if (*s).size != 0 as usize {
        additionalTopDictOpsSize = additionalTopDictOpsSize.wrapping_add(6 as u32);
    }
    if (*p).size != 0 as usize {
        additionalTopDictOpsSize = additionalTopDictOpsSize.wrapping_add(11 as u32);
    }
    if (*r).size != 0 as usize {
        additionalTopDictOpsSize = additionalTopDictOpsSize.wrapping_add(7 as u32);
    }
    bufwrite_bufdel(blob, h);
    bufwrite_bufdel(blob, n);
    let mut delta_size: i32 = (*t)
        .size
        .wrapping_add(additionalTopDictOpsSize as usize)
        .wrapping_add(1 as usize) as u32 as i32;
    bufwrite_bufdel(
        blob,
        bufninit(&[0 as u8, 1 as u8, 4 as u8, 0 as u8, 0 as u8, 0 as u8, 1 as u8, (delta_size >> 24 as ::core::ffi::c_int & 0xff as i32) as u8, (delta_size >> 16 as ::core::ffi::c_int & 0xff as i32) as u8, (delta_size >> 8 as ::core::ffi::c_int & 0xff as i32) as u8, (delta_size & 0xff as i32) as u8]),
    );
    bufwrite_bufdel(blob, t);
    off = (off as usize).wrapping_add(
        (additionalTopDictOpsSize as usize)
            .wrapping_add((*i).size)
            .wrapping_add((*gs).size),
    ) as u32 as u32;
    if (*c).size != 0 as usize {
        bufwrite_bufdel(blob, cff_buildOffset(off as i32));
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator(op_charset as ::core::ffi::c_int as i32),
        );
        off = (off as usize).wrapping_add((*c).size) as u32 as u32;
    }
    if (*e).size != 0 as usize {
        bufwrite_bufdel(blob, cff_buildOffset(off as i32));
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator(op_FDSelect as ::core::ffi::c_int as i32),
        );
        off = (off as usize).wrapping_add((*e).size) as u32 as u32;
    }
    if (*s).size != 0 as usize {
        bufwrite_bufdel(blob, cff_buildOffset(off as i32));
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator(op_CharStrings as ::core::ffi::c_int as i32),
        );
        off = (off as usize).wrapping_add((*s).size) as u32 as u32;
    }
    if (*p).size != 0 as usize {
        bufwrite_bufdel(blob, cff_buildOffset((*p).size as u32 as i32));
        bufwrite_bufdel(blob, cff_buildOffset(off as i32));
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator(op_Private as ::core::ffi::c_int as i32),
        );
        off = (off as usize).wrapping_add((*p).size) as u32 as u32;
    }
    if (*r).size != 0 as usize {
        bufwrite_bufdel(blob, cff_buildOffset(off as i32));
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator(op_FDArray as ::core::ffi::c_int as i32),
        );
        off = (off as usize).wrapping_add((*r).size) as u32 as u32;
    }
    bufwrite_bufdel(blob, i);
    bufwrite_bufdel(blob, gs);
    bufwrite_bufdel(blob, c);
    bufwrite_bufdel(blob, e);
    bufwrite_bufdel(blob, s);
    let mut startingPositionOfPrivates: *mut usize = ::core::ptr::null_mut::<usize>();
    startingPositionOfPrivates = __caryll_allocate_clean(
        (::core::mem::size_of::<usize>() as usize).wrapping_mul(
            (1 as ::core::ffi::c_int + (*cff).fdArrayCount as ::core::ffi::c_int) as usize,
        ),
        1350 as ::core::ffi::c_ulong,
    ) as *mut usize;
    *startingPositionOfPrivates.offset(0 as ::core::ffi::c_int as isize) = (*blob).cursor;
    bufwrite_bufdel(blob, p);
    let mut endingPositionOfPrivates: *mut usize = ::core::ptr::null_mut::<usize>();
    endingPositionOfPrivates = __caryll_allocate_clean(
        (::core::mem::size_of::<usize>() as usize).wrapping_mul(
            (1 as ::core::ffi::c_int + (*cff).fdArrayCount as ::core::ffi::c_int) as usize,
        ),
        1354 as ::core::ffi::c_ulong,
    ) as *mut usize;
    *endingPositionOfPrivates.offset(0 as ::core::ffi::c_int as isize) = (*blob).cursor;
    if (*cff).isCID {
        let mut fdArrayPrivatesStartOffset: u32 = off;
        let mut fdArrayPrivates: *mut *mut caryll_Buffer =
            ::core::ptr::null_mut::<*mut caryll_Buffer>();
        fdArrayPrivates = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut caryll_Buffer>() as usize)
                .wrapping_mul((*cff).fdArrayCount as usize),
            1359 as ::core::ffi::c_ulong,
        ) as *mut *mut caryll_Buffer;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < (*cff).fdArrayCount as ::core::ffi::c_int {
            let mut pd: *mut cff_Dict =
                cff_make_private_dict((**(*cff).fdArray.offset(j as isize)).privateDict);
            let mut p_0: *mut caryll_Buffer =
                cff_iDict.build.expect("non-null function pointer")(pd);
            bufwrite_bufdel(
                p_0,
                cff_buildOffset(0xffffffff as ::core::ffi::c_uint as i32),
            );
            bufwrite_bufdel(
                p_0,
                cff_encodeCffOperator(op_Subrs as ::core::ffi::c_int as i32),
            );
            cff_iDict.free.expect("non-null function pointer")(pd);
            let ref mut fresh14 = *fdArrayPrivates.offset(j as isize);
            *fresh14 = p_0;
            let mut privateLengthPtr: *mut u8 = (*fdArrayIndex).data.offset(
                (*(*fdArrayIndex)
                    .offset
                    .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(11 as u32) as isize,
            ) as *mut u8;
            *privateLengthPtr.offset(0 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 24 as ::core::ffi::c_int & 0xff as usize) as u8;
            *privateLengthPtr.offset(1 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 16 as ::core::ffi::c_int & 0xff as usize) as u8;
            *privateLengthPtr.offset(2 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 8 as ::core::ffi::c_int & 0xff as usize) as u8;
            *privateLengthPtr.offset(3 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 0 as ::core::ffi::c_int & 0xff as usize) as u8;
            let mut privateOffsetPtr: *mut u8 = (*fdArrayIndex).data.offset(
                (*(*fdArrayIndex)
                    .offset
                    .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(6 as u32) as isize,
            ) as *mut u8;
            *privateOffsetPtr.offset(0 as ::core::ffi::c_int as isize) =
                (fdArrayPrivatesStartOffset >> 24 as ::core::ffi::c_int & 0xff as u32)
                    as u8;
            *privateOffsetPtr.offset(1 as ::core::ffi::c_int as isize) =
                (fdArrayPrivatesStartOffset >> 16 as ::core::ffi::c_int & 0xff as u32)
                    as u8;
            *privateOffsetPtr.offset(2 as ::core::ffi::c_int as isize) =
                (fdArrayPrivatesStartOffset >> 8 as ::core::ffi::c_int & 0xff as u32)
                    as u8;
            *privateOffsetPtr.offset(3 as ::core::ffi::c_int as isize) =
                (fdArrayPrivatesStartOffset >> 0 as ::core::ffi::c_int & 0xff as u32)
                    as u8;
            fdArrayPrivatesStartOffset = (fdArrayPrivatesStartOffset as usize)
                .wrapping_add((*p_0).size) as u32
                as u32;
            j = j.wrapping_add(1);
        }
        buffree(r);
        r = cff_iIndex.build.expect("non-null function pointer")(fdArrayIndex);
        cff_iIndex.free.expect("non-null function pointer")(fdArrayIndex);
        bufwrite_bufdel(blob, r);
        let mut j_0: tableid_t = 0 as tableid_t;
        while (j_0 as ::core::ffi::c_int) < (*cff).fdArrayCount as ::core::ffi::c_int {
            *startingPositionOfPrivates
                .offset((j_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (*blob).cursor;
            bufwrite_bufdel(blob, *fdArrayPrivates.offset(j_0 as isize));
            *endingPositionOfPrivates
                .offset((j_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (*blob).cursor;
            j_0 = j_0.wrapping_add(1);
        }
        free(fdArrayPrivates as *mut ::core::ffi::c_void);
        fdArrayPrivates = ::core::ptr::null_mut::<*mut caryll_Buffer>();
    } else {
        bufwrite_bufdel(blob, r);
    }
    let mut positionOfLocalSubroutines: usize = (*blob).cursor;
    bufwrite_bufdel(blob, ls);
    let mut j_1: tableid_t = 0 as tableid_t;
    while (j_1 as ::core::ffi::c_int)
        < (*cff).fdArrayCount as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    {
        let mut lsOffset: usize = positionOfLocalSubroutines
            .wrapping_sub(*startingPositionOfPrivates.offset(j_1 as isize));
        let mut ptr: *mut u8 = (*blob).data.offset(
            (*endingPositionOfPrivates.offset(j_1 as isize)).wrapping_sub(5 as usize) as isize,
        ) as *mut u8;
        *ptr.offset(0 as ::core::ffi::c_int as isize) =
            (lsOffset >> 24 as ::core::ffi::c_int & 0xff as usize) as u8;
        *ptr.offset(1 as ::core::ffi::c_int as isize) =
            (lsOffset >> 16 as ::core::ffi::c_int & 0xff as usize) as u8;
        *ptr.offset(2 as ::core::ffi::c_int as isize) =
            (lsOffset >> 8 as ::core::ffi::c_int & 0xff as usize) as u8;
        *ptr.offset(3 as ::core::ffi::c_int as isize) =
            (lsOffset >> 0 as ::core::ffi::c_int & 0xff as usize) as u8;
        j_1 = j_1.wrapping_add(1);
    }
    free(startingPositionOfPrivates as *mut ::core::ffi::c_void);
    startingPositionOfPrivates = ::core::ptr::null_mut::<usize>();
    free(endingPositionOfPrivates as *mut ::core::ffi::c_void);
    endingPositionOfPrivates = ::core::ptr::null_mut::<usize>();
    return blob;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_buildCFF(
    cffAndGlyf: table_CFFAndGlyf,
    mut options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    return writecff_CIDKeyed(cffAndGlyf.meta, cffAndGlyf.glyphs, options);
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
unsafe extern "C" fn json_obj_getsds(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> sds {
    let mut v: *mut json_value = json_obj_get_type(obj, key, json_string);
    if v.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        return sdsnewlen(
            (*v).u.string.ptr as *const ::core::ffi::c_void,
            (*v).u.string.length as usize,
        );
    };
}
#[inline]
unsafe extern "C" fn json_numof(mut cv: *const json_value) -> ::core::ffi::c_double {
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
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
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
#[inline]
unsafe extern "C" fn json_obj_getint(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> i32 {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as i32;
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
                return (*cv).u.integer as i32;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl as i32;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn json_obj_getnum_fallback(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    mut fallback: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return fallback;
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
    return fallback;
}
#[inline]
unsafe extern "C" fn json_from_sds(str: sds) -> *mut json_value {
    return json_string_new_length(
        sdslen(str) as ::core::ffi::c_uint,
        str as *const ::core::ffi::c_char,
    );
}
