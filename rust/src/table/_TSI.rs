#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort, strcmp};
use crate::support::json_funcs::{json_obj_get_type};
use crate::support::handle::{handle_fromIndex, handle_fromName, otfcc_Handle_copy, otfcc_Handle_dispose, otfcc_Handle_empty, otfcc_Handle_init, Handle, GlyphHandle, HANDLE_STATE_EMPTY};
use crate::support::binio::{read_16u, read_32u};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::{json_object, json_string, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::{ComparFn};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b, bufwrite_sds};
use crate::vendor::json_builder::{json_object_new, json_object_push, json_string_new_length};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree, sdsnewlen};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TsiEntryType {
    TSI_GLYPH = 0,
    TSI_FPGM = 1,
    TSI_PREP = 2,
    TSI_CVT = 3,
    TSI_RESERVED_FFFC = 4,
}
pub use TsiEntryType::*;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TsiEntry {
    pub type_0: TsiEntryType,
    pub glyph: GlyphHandle,
    pub content: SdsRaw,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TsiEntryElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut TsiEntry) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut TsiEntry, *const TsiEntry) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut TsiEntry, *mut TsiEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut TsiEntry) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut TsiEntry, TsiEntry) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut TsiEntry, TsiEntry) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TsiTable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut TsiEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TsiTableVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut TsiTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut TsiTable, *const TsiTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut TsiTable, *mut TsiTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut TsiTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut TsiTable, TsiTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut TsiTable, TsiTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut TsiTable>,
    pub free: Option<unsafe extern "C" fn(*mut TsiTable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut TsiTable, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut TsiTable, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut TsiTable>,
    pub fill: Option<unsafe extern "C" fn(*mut TsiTable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut TsiTable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut TsiTable, TsiEntry) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut TsiTable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut TsiTable) -> TsiEntry>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut TsiTable, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut TsiTable,
            Option<unsafe extern "C" fn(*const TsiEntry, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut TsiTable,
            Option<unsafe extern "C" fn(*const TsiEntry, *const TsiEntry) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TsiBuildTarget {
    pub indexPart: *mut Buffer,
    pub textPart: *mut Buffer,
}
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
#[inline]
unsafe extern "C" fn initTSIEntry(mut entry: *mut TsiEntry) {
    otfcc_Handle_init(&raw mut (*entry).glyph);
    (*entry).type_0 = TSI_GLYPH;
    (*entry).content = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[inline]
unsafe extern "C" fn copyTSIEntry(mut dst: *mut TsiEntry, mut src: *const TsiEntry) {
    otfcc_Handle_copy(
        &raw mut (*dst).glyph,
        &raw const (*src).glyph,
    );
    (*dst).type_0 = (*src).type_0;
    (*dst).content = sdsdup((*src).content);
}
#[inline]
unsafe extern "C" fn disposeTSIEntry(mut entry: *mut TsiEntry) {
    otfcc_Handle_dispose(&raw mut (*entry).glyph);
    sdsfree((*entry).content);
}
#[inline]
unsafe extern "C" fn tsi_Entry_replace(mut dst: *mut TsiEntry, src: TsiEntry) {
    tsi_Entry_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<TsiEntry>() as usize,
    );
}
#[inline]
unsafe extern "C" fn tsi_Entry_init(mut x: *mut TsiEntry) {
    initTSIEntry(x);
}
#[inline]
unsafe extern "C" fn tsi_Entry_move(mut dst: *mut TsiEntry, mut src: *mut TsiEntry) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<TsiEntry>() as usize,
    );
    tsi_Entry_init(src);
}
pub static tsi_iEntry: TsiEntryElementInterface = {
    TsiEntryElementInterface {
        init: Some(tsi_Entry_init as unsafe extern "C" fn(*mut TsiEntry) -> ()),
        copy: Some(tsi_Entry_copy as unsafe extern "C" fn(*mut TsiEntry, *const TsiEntry) -> ()),
        move_0: Some(tsi_Entry_move as unsafe extern "C" fn(*mut TsiEntry, *mut TsiEntry) -> ()),
        dispose: Some(tsi_Entry_dispose as unsafe extern "C" fn(*mut TsiEntry) -> ()),
        replace: Some(tsi_Entry_replace as unsafe extern "C" fn(*mut TsiEntry, TsiEntry) -> ()),
        copyReplace: Some(
            tsi_Entry_copyReplace as unsafe extern "C" fn(*mut TsiEntry, TsiEntry) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn tsi_Entry_dispose(mut x: *mut TsiEntry) {
    disposeTSIEntry(x);
}
#[inline]
unsafe extern "C" fn tsi_Entry_copy(mut dst: *mut TsiEntry, mut src: *const TsiEntry) {
    copyTSIEntry(dst, src);
}
#[inline]
unsafe extern "C" fn tsi_Entry_copyReplace(mut dst: *mut TsiEntry, src: TsiEntry) {
    tsi_Entry_dispose(dst);
    tsi_Entry_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_TSI_fill(mut arr: *mut TsiTable, mut n: usize) {
    while (*arr).length < n {
        let mut x: TsiEntry = TsiEntry {
            type_0: TSI_GLYPH,
            glyph: Handle {
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
                ::core::mem::size_of::<TsiEntry>() as usize,
            );
        }
        table_TSI_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_TSI_move(dst: *mut TsiTable, src: *mut TsiTable) {
    cvec_move(table_TSI_as_cvec(dst), table_TSI_as_cvec(src));
}
#[inline]
unsafe fn table_TSI_as_cvec(arr: *mut TsiTable) -> *mut CVecRaw<TsiEntry> {
    arr as *mut CVecRaw<TsiEntry>
}
#[inline]
unsafe extern "C" fn table_TSI_init(arr: *mut TsiTable) {
    cvec_init(table_TSI_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_TSI_filterEnv(
    mut arr: *mut TsiTable,
    mut fn_0: Option<unsafe extern "C" fn(*const TsiEntry, *mut ::core::ffi::c_void) -> bool>,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut TsiEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if tsi_iEntry.dispose.is_some() {
                tsi_iEntry.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut TsiEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn table_TSI_disposeItem(mut arr: *mut TsiTable, mut n: usize) {
    if tsi_iEntry.dispose.is_some() {
        tsi_iEntry.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut TsiEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_TSI_sort(
    mut arr: *mut TsiTable,
    mut fn_0: Option<
        unsafe extern "C" fn(*const TsiEntry, *const TsiEntry) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<TsiEntry>() as usize,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const TsiEntry, *const TsiEntry) -> ::core::ffi::c_int>,
            ComparFn,
        >(fn_0),
    );
}
pub static table_iTSI: TsiTableVectorInterface = {
    TsiTableVectorInterface {
        init: Some(table_TSI_init as unsafe extern "C" fn(*mut TsiTable) -> ()),
        copy: Some(table_TSI_copy as unsafe extern "C" fn(*mut TsiTable, *const TsiTable) -> ()),
        move_0: Some(table_TSI_move as unsafe extern "C" fn(*mut TsiTable, *mut TsiTable) -> ()),
        dispose: Some(table_TSI_dispose as unsafe extern "C" fn(*mut TsiTable) -> ()),
        replace: Some(table_TSI_replace as unsafe extern "C" fn(*mut TsiTable, TsiTable) -> ()),
        copyReplace: Some(
            table_TSI_copyReplace as unsafe extern "C" fn(*mut TsiTable, TsiTable) -> (),
        ),
        create: Some(table_TSI_create),
        free: Some(table_TSI_free as unsafe extern "C" fn(*mut TsiTable) -> ()),
        initN: Some(table_TSI_initN as unsafe extern "C" fn(*mut TsiTable, usize) -> ()),
        initCapN: Some(table_TSI_initCapN as unsafe extern "C" fn(*mut TsiTable, usize) -> ()),
        createN: Some(table_TSI_createN as unsafe extern "C" fn(usize) -> *mut TsiTable),
        fill: Some(table_TSI_fill as unsafe extern "C" fn(*mut TsiTable, usize) -> ()),
        clear: Some(table_TSI_dispose as unsafe extern "C" fn(*mut TsiTable) -> ()),
        push: Some(table_TSI_push as unsafe extern "C" fn(*mut TsiTable, TsiEntry) -> ()),
        shrinkToFit: Some(table_TSI_shrinkToFit as unsafe extern "C" fn(*mut TsiTable) -> ()),
        pop: Some(table_TSI_pop as unsafe extern "C" fn(*mut TsiTable) -> TsiEntry),
        disposeItem: Some(
            table_TSI_disposeItem as unsafe extern "C" fn(*mut TsiTable, usize) -> (),
        ),
        filterEnv: Some(
            table_TSI_filterEnv
                as unsafe extern "C" fn(
                    *mut TsiTable,
                    Option<unsafe extern "C" fn(*const TsiEntry, *mut ::core::ffi::c_void) -> bool>,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_TSI_sort
                as unsafe extern "C" fn(
                    *mut TsiTable,
                    Option<
                        unsafe extern "C" fn(
                            *const TsiEntry,
                            *const TsiEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn table_TSI_push(arr: *mut TsiTable, elem: TsiEntry) {
    cvec_push(table_TSI_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_TSI_grow(arr: *mut TsiTable) {
    cvec_grow(table_TSI_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_TSI_growTo(arr: *mut TsiTable, target: usize) {
    cvec_grow_to(table_TSI_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_TSI_pop(arr: *mut TsiTable) -> TsiEntry {
    cvec_pop(table_TSI_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_TSI_copyReplace(mut dst: *mut TsiTable, src: TsiTable) {
    table_TSI_dispose(dst);
    table_TSI_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_TSI_copy(mut dst: *mut TsiTable, mut src: *const TsiTable) {
    table_TSI_init(dst);
    table_TSI_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if tsi_iEntry.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            tsi_iEntry.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut TsiEntry,
                (*src).items.offset(j as isize) as *mut TsiEntry as *const TsiEntry,
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
unsafe extern "C" fn table_TSI_dispose(mut arr: *mut TsiTable) {
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
                (*arr).items.offset(j as isize) as *mut TsiEntry
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<TsiEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn table_TSI_replace(mut dst: *mut TsiTable, src: TsiTable) {
    table_TSI_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<TsiTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_TSI_initCapN(mut arr: *mut TsiTable, mut n: usize) {
    table_TSI_init(arr);
    table_TSI_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn table_TSI_growToN(arr: *mut TsiTable, target: usize) {
    cvec_grow_to_n(table_TSI_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_TSI_initN(mut arr: *mut TsiTable, mut n: usize) {
    table_TSI_init(arr);
    table_TSI_growToN(arr, n);
    table_TSI_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_TSI_free(mut x: *mut TsiTable) {
    if x.is_null() {
        return;
    }
    table_TSI_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_TSI_createN(mut n: usize) -> *mut TsiTable {
    let mut t: *mut TsiTable =
        malloc(::core::mem::size_of::<TsiTable>() as usize) as *mut TsiTable;
    table_TSI_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_TSI_create() -> *mut TsiTable {
    let mut x: *mut TsiTable =
        malloc(::core::mem::size_of::<TsiTable>() as usize) as *mut TsiTable;
    table_TSI_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_TSI_shrinkToFit(mut arr: *mut TsiTable) {
    table_TSI_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_TSI_resizeTo(arr: *mut TsiTable, target: usize) {
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
pub unsafe extern "C" fn otfcc_readTSI(
    packet: Packet,
    mut _options: *const Options,
    mut tagIndex: u32,
    mut tagText: u32,
) -> *mut TsiTable {
    let mut textPart: PacketPiece = PacketPiece {
        tag: 0,
        checkSum: 0,
        offset: 0,
        length: 0,
        data: ::core::ptr::null_mut::<u8>(),
    };
    textPart.tag = 0 as u32;
    let mut indexPart: PacketPiece = PacketPiece {
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
        let mut tableIx: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
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
        let mut tableTx: PacketPiece = *packet.pieces.offset(__fortable_count_0 as isize);
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
        return ::core::ptr::null_mut::<TsiTable>();
    }
    let mut tsi: *mut TsiTable = (
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
            let mut k: GlyphId = j.wrapping_add(1 as u32) as GlyphId;
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
            let mut entry: TsiEntry = TsiEntry {
                type_0: TSI_GLYPH,
                glyph: Handle {
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
                        gid as GlyphId,
                    ) as GlyphHandle;
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
pub unsafe extern "C" fn otfcc_dumpTSI(
    mut tsi: *const TsiTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) {
    if tsi.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), tag),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _tsi: *mut JsonValue = json_object_new(2 as usize);
        let mut _glyphs: *mut JsonValue = json_object_new((*tsi).length);
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < (*tsi).length {
            let mut entry: *mut TsiEntry = (*tsi).items.offset(__caryll_index as isize);
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
        let mut _extra: *mut JsonValue = json_object_new((*tsi).length);
        let mut __caryll_index_0: usize = 0 as usize;
        let mut keep_0: usize = 1 as usize;
        while keep_0 != 0 && __caryll_index_0 < (*tsi).length {
            let mut entry_0: *mut TsiEntry = (*tsi).items.offset(__caryll_index_0 as isize);
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
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parseTSI(
    mut root: *const JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut TsiTable {
    let mut _tsi: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    _tsi = json_obj_get_type(root, tag, json_object);
    if _tsi.is_null() {
        return ::core::ptr::null_mut::<TsiTable>();
    }
    let mut tsi: *mut TsiTable = (
        table_iTSI.create.expect("non-null function pointer"))();
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), tag),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _glyphs: *mut JsonValue = json_obj_get_type(
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
                let mut _content: *mut JsonValue =
                    (*(*_glyphs).u.object.values.offset(j as isize)).value as *mut JsonValue;
                if !(_content.is_null()
                    || (*_content).type_0 != json_string)
                {
                    table_iTSI.push.expect("non-null function pointer")(
                        tsi,
                        TsiEntry {
                            type_0: TSI_GLYPH,
                            glyph: handle_fromName(
                                sdsnewlen(_gid as *const ::core::ffi::c_void, _gidlen),
                            ) as GlyphHandle,
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
        let mut _extra: *mut JsonValue = json_obj_get_type(
            _tsi,
            b"extra\0" as *const u8 as *const ::core::ffi::c_char,
            json_object,
        );
        if !_extra.is_null() {
            let mut j_0: u32 = 0 as u32;
            while j_0 < (*_extra).u.object.length as u32 {
                let mut _key: *mut ::core::ffi::c_char =
                    (*(*_extra).u.object.values.offset(j_0 as isize)).name;
                let mut _content_0: *mut JsonValue =
                    (*(*_extra).u.object.values.offset(j_0 as isize)).value as *mut JsonValue;
                if !(_content_0.is_null()
                    || (*_content_0).type_0 != json_string)
                {
                    if strcmp(_key, b"cvt\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        table_iTSI.push.expect("non-null function pointer")(
                            tsi,
                            TsiEntry {
                                type_0: TSI_CVT,
                                glyph: otfcc_Handle_empty() as GlyphHandle,
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
                            TsiEntry {
                                type_0: TSI_FPGM,
                                glyph: otfcc_Handle_empty() as GlyphHandle,
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
                            TsiEntry {
                                type_0: TSI_PREP,
                                glyph: otfcc_Handle_empty() as GlyphHandle,
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
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return tsi;
}
unsafe extern "C" fn propergid(mut entry: *mut TsiEntry, type_0: TsiEntryType) -> GlyphId {
    match type_0 as ::core::ffi::c_uint {
        3 => return 0xfffb as GlyphId,
        1 => return 0xfffd as GlyphId,
        2 => return 0xfffa as GlyphId,
        4 => return 0xfffc as GlyphId,
        0 => return (*entry).glyph.index,
        _ => {}
    }
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn pushTSIEntries(
    mut target: *mut TsiBuildTarget,
    mut tsi: *const TsiTable,
    type_0: TsiEntryType,
    minN: GlyphId,
) {
    let mut itemsPushed: GlyphId = 0 as GlyphId;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*tsi).length {
        let mut entry: *mut TsiEntry = (*tsi).items.offset(__caryll_index as isize);
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
                    (itemsPushed as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    while (itemsPushed as ::core::ffi::c_int) < minN as ::core::ffi::c_int {
        bufwrite16b(
            (*target).indexPart,
            propergid(::core::ptr::null_mut::<TsiEntry>(), type_0) as u16,
        );
        bufwrite16b((*target).indexPart, 0 as u16);
        bufwrite32b(
            (*target).indexPart,
            (*(*target).textPart).cursor as u32,
        );
        itemsPushed = (itemsPushed as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
    }
}
pub unsafe extern "C" fn otfcc_buildTSI(
    mut tsi: *const TsiTable,
    mut _options: *const Options,
) -> TsiBuildTarget {
    let mut target: TsiBuildTarget = TsiBuildTarget {
        indexPart: ::core::ptr::null_mut::<Buffer>(),
        textPart: ::core::ptr::null_mut::<Buffer>(),
    };
    if tsi.is_null() {
        target.textPart = ::core::ptr::null_mut::<Buffer>();
        target.indexPart = ::core::ptr::null_mut::<Buffer>();
    } else {
        target.textPart = bufnew();
        target.indexPart = bufnew();
        pushTSIEntries(&raw mut target, tsi, TSI_GLYPH, 0 as GlyphId);
        bufwrite16b(target.indexPart, 0xfffe as u16);
        bufwrite16b(target.indexPart, 0 as u16);
        bufwrite32b(target.indexPart, 0xabfc1f34 as u32);
        pushTSIEntries(&raw mut target, tsi, TSI_PREP, 1 as GlyphId);
        pushTSIEntries(&raw mut target, tsi, TSI_CVT, 1 as GlyphId);
        pushTSIEntries(&raw mut target, tsi, TSI_RESERVED_FFFC, 1 as GlyphId);
        pushTSIEntries(&raw mut target, tsi, TSI_FPGM, 1 as GlyphId);
    }
    return target;
}
