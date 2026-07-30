#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort, strcmp};
use crate::support::json_funcs::{json_obj_get_type};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_copy, otfcc_handle_dispose, otfcc_handle_empty, otfcc_handle_init, Handle, GlyphHandle, HandleState};
use crate::support::binio::{read_16u, read_32u};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::{JsonType, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::{ComparFn};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b, bufwrite_sds};
use crate::vendor::json_builder::{json_object_new, json_object_push, json_string_new_length};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree, sdsnewlen};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TsiEntryType {
    Glyph = 0,
    Fpgm = 1,
    Prep = 2,
    Cvt = 3,
    ReservedFffc = 4,
}
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut TsiEntry, TsiEntry) -> ()>,
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
    pub copy_replace: Option<unsafe extern "C" fn(*mut TsiTable, TsiTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut TsiTable>,
    pub free: Option<unsafe extern "C" fn(*mut TsiTable) -> ()>,
    pub init_n: Option<unsafe extern "C" fn(*mut TsiTable, usize) -> ()>,
    pub init_cap_n: Option<unsafe extern "C" fn(*mut TsiTable, usize) -> ()>,
    pub create_n: Option<unsafe extern "C" fn(usize) -> *mut TsiTable>,
    pub fill: Option<unsafe extern "C" fn(*mut TsiTable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut TsiTable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut TsiTable, TsiEntry) -> ()>,
    pub shrink_to_fit: Option<unsafe extern "C" fn(*mut TsiTable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut TsiTable) -> TsiEntry>,
    pub dispose_item: Option<unsafe extern "C" fn(*mut TsiTable, usize) -> ()>,
    pub filter_env: Option<
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
    pub index_part: *mut Buffer,
    pub text_part: *mut Buffer,
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
unsafe extern "C" fn init_tsi_entry(mut entry: *mut TsiEntry) {
    otfcc_handle_init(&raw mut (*entry).glyph);
    (*entry).type_0 = TsiEntryType::Glyph;
    (*entry).content = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[inline]
unsafe extern "C" fn copy_tsi_entry(mut dst: *mut TsiEntry, mut src: *const TsiEntry) {
    otfcc_handle_copy(
        &raw mut (*dst).glyph,
        &raw const (*src).glyph,
    );
    (*dst).type_0 = (*src).type_0;
    (*dst).content = sdsdup((*src).content);
}
#[inline]
unsafe extern "C" fn dispose_tsi_entry(mut entry: *mut TsiEntry) {
    otfcc_handle_dispose(&raw mut (*entry).glyph);
    sdsfree((*entry).content);
}
#[inline]
unsafe extern "C" fn tsi_entry_replace(mut dst: *mut TsiEntry, src: TsiEntry) {
    tsi_entry_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<TsiEntry>() as usize,
    );
}
#[inline]
unsafe extern "C" fn tsi_entry_init(mut x: *mut TsiEntry) {
    init_tsi_entry(x);
}
#[inline]
unsafe extern "C" fn tsi_entry_move(mut dst: *mut TsiEntry, mut src: *mut TsiEntry) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<TsiEntry>() as usize,
    );
    tsi_entry_init(src);
}
pub static TSI_I_ENTRY: TsiEntryElementInterface = {
    TsiEntryElementInterface {
        init: Some(tsi_entry_init as unsafe extern "C" fn(*mut TsiEntry) -> ()),
        copy: Some(tsi_entry_copy as unsafe extern "C" fn(*mut TsiEntry, *const TsiEntry) -> ()),
        move_0: Some(tsi_entry_move as unsafe extern "C" fn(*mut TsiEntry, *mut TsiEntry) -> ()),
        dispose: Some(tsi_entry_dispose as unsafe extern "C" fn(*mut TsiEntry) -> ()),
        replace: Some(tsi_entry_replace as unsafe extern "C" fn(*mut TsiEntry, TsiEntry) -> ()),
        copy_replace: Some(
            tsi_entry_copy_replace as unsafe extern "C" fn(*mut TsiEntry, TsiEntry) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn tsi_entry_dispose(mut x: *mut TsiEntry) {
    dispose_tsi_entry(x);
}
#[inline]
unsafe extern "C" fn tsi_entry_copy(mut dst: *mut TsiEntry, mut src: *const TsiEntry) {
    copy_tsi_entry(dst, src);
}
#[inline]
unsafe extern "C" fn tsi_entry_copy_replace(mut dst: *mut TsiEntry, src: TsiEntry) {
    tsi_entry_dispose(dst);
    tsi_entry_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_tsi_fill(mut arr: *mut TsiTable, mut n: usize) {
    while (*arr).length < n {
        let mut x: TsiEntry = TsiEntry {
            type_0: TsiEntryType::Glyph,
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            content: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        if TSI_I_ENTRY.init.is_some() {
            TSI_I_ENTRY.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<TsiEntry>() as usize,
            );
        }
        table_tsi_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_tsi_move(dst: *mut TsiTable, src: *mut TsiTable) {
    cvec_move(table_tsi_as_cvec(dst), table_tsi_as_cvec(src));
}
#[inline]
unsafe fn table_tsi_as_cvec(arr: *mut TsiTable) -> *mut CVecRaw<TsiEntry> {
    arr as *mut CVecRaw<TsiEntry>
}
#[inline]
unsafe extern "C" fn table_tsi_init(arr: *mut TsiTable) {
    cvec_init(table_tsi_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_tsi_filter_env(
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
            if TSI_I_ENTRY.dispose.is_some() {
                TSI_I_ENTRY.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn table_tsi_dispose_item(mut arr: *mut TsiTable, mut n: usize) {
    if TSI_I_ENTRY.dispose.is_some() {
        TSI_I_ENTRY.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut TsiEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_tsi_sort(
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
pub static TABLE_I_TSI: TsiTableVectorInterface = {
    TsiTableVectorInterface {
        init: Some(table_tsi_init as unsafe extern "C" fn(*mut TsiTable) -> ()),
        copy: Some(table_tsi_copy as unsafe extern "C" fn(*mut TsiTable, *const TsiTable) -> ()),
        move_0: Some(table_tsi_move as unsafe extern "C" fn(*mut TsiTable, *mut TsiTable) -> ()),
        dispose: Some(table_tsi_dispose as unsafe extern "C" fn(*mut TsiTable) -> ()),
        replace: Some(table_tsi_replace as unsafe extern "C" fn(*mut TsiTable, TsiTable) -> ()),
        copy_replace: Some(
            table_tsi_copy_replace as unsafe extern "C" fn(*mut TsiTable, TsiTable) -> (),
        ),
        create: Some(table_tsi_create),
        free: Some(table_tsi_free as unsafe extern "C" fn(*mut TsiTable) -> ()),
        init_n: Some(table_tsi_init_n as unsafe extern "C" fn(*mut TsiTable, usize) -> ()),
        init_cap_n: Some(table_tsi_init_cap_n as unsafe extern "C" fn(*mut TsiTable, usize) -> ()),
        create_n: Some(table_tsi_create_n as unsafe extern "C" fn(usize) -> *mut TsiTable),
        fill: Some(table_tsi_fill as unsafe extern "C" fn(*mut TsiTable, usize) -> ()),
        clear: Some(table_tsi_dispose as unsafe extern "C" fn(*mut TsiTable) -> ()),
        push: Some(table_tsi_push as unsafe extern "C" fn(*mut TsiTable, TsiEntry) -> ()),
        shrink_to_fit: Some(table_tsi_shrink_to_fit as unsafe extern "C" fn(*mut TsiTable) -> ()),
        pop: Some(table_tsi_pop as unsafe extern "C" fn(*mut TsiTable) -> TsiEntry),
        dispose_item: Some(
            table_tsi_dispose_item as unsafe extern "C" fn(*mut TsiTable, usize) -> (),
        ),
        filter_env: Some(
            table_tsi_filter_env
                as unsafe extern "C" fn(
                    *mut TsiTable,
                    Option<unsafe extern "C" fn(*const TsiEntry, *mut ::core::ffi::c_void) -> bool>,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_tsi_sort
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
unsafe extern "C" fn table_tsi_push(arr: *mut TsiTable, elem: TsiEntry) {
    cvec_push(table_tsi_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_tsi_grow(arr: *mut TsiTable) {
    cvec_grow(table_tsi_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_tsi_grow_to(arr: *mut TsiTable, target: usize) {
    cvec_grow_to(table_tsi_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_tsi_pop(arr: *mut TsiTable) -> TsiEntry {
    cvec_pop(table_tsi_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_tsi_copy_replace(mut dst: *mut TsiTable, src: TsiTable) {
    table_tsi_dispose(dst);
    table_tsi_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_tsi_copy(mut dst: *mut TsiTable, mut src: *const TsiTable) {
    table_tsi_init(dst);
    table_tsi_grow_to(dst, (*src).length);
    (*dst).length = (*src).length;
    if TSI_I_ENTRY.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            TSI_I_ENTRY.copy.expect("non-null function pointer")(
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
unsafe extern "C" fn table_tsi_dispose(mut arr: *mut TsiTable) {
    if arr.is_null() {
        return;
    }
    if TSI_I_ENTRY.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            TSI_I_ENTRY.dispose.expect("non-null function pointer")(
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
unsafe extern "C" fn table_tsi_replace(mut dst: *mut TsiTable, src: TsiTable) {
    table_tsi_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<TsiTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_tsi_init_cap_n(mut arr: *mut TsiTable, mut n: usize) {
    table_tsi_init(arr);
    table_tsi_grow_to_n(arr, n);
}
#[inline]
unsafe extern "C" fn table_tsi_grow_to_n(arr: *mut TsiTable, target: usize) {
    cvec_grow_to_n(table_tsi_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_tsi_init_n(mut arr: *mut TsiTable, mut n: usize) {
    table_tsi_init(arr);
    table_tsi_grow_to_n(arr, n);
    table_tsi_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_tsi_free(mut x: *mut TsiTable) {
    if x.is_null() {
        return;
    }
    table_tsi_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_tsi_create_n(mut n: usize) -> *mut TsiTable {
    let mut t: *mut TsiTable =
        malloc(::core::mem::size_of::<TsiTable>() as usize) as *mut TsiTable;
    table_tsi_init_n(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_tsi_create() -> *mut TsiTable {
    let mut x: *mut TsiTable =
        malloc(::core::mem::size_of::<TsiTable>() as usize) as *mut TsiTable;
    table_tsi_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_tsi_shrink_to_fit(mut arr: *mut TsiTable) {
    table_tsi_resize_to(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_tsi_resize_to(arr: *mut TsiTable, target: usize) {
    cvec_resize_to(table_tsi_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn is_valid_gid(mut gid: u16, mut tag_index: u32) -> bool {
    if tag_index == 1414744368i32 as u32 {
        return gid as ::core::ffi::c_int != 0xfffe as ::core::ffi::c_int
            && gid as ::core::ffi::c_int != 0xfffc as ::core::ffi::c_int;
    } else {
        return (gid as ::core::ffi::c_int) < 0xfffa as ::core::ffi::c_int;
    };
}
pub unsafe extern "C" fn otfcc_read_tsi(
    packet: Packet,
    mut _options: *const Options,
    mut tag_index: u32,
    mut tag_text: u32,
) -> *mut TsiTable {
    let mut text_part: PacketPiece = PacketPiece {
        tag: 0,
        check_sum: 0,
        offset: 0,
        length: 0,
        data: ::core::ptr::null_mut::<u8>(),
    };
    text_part.tag = 0 as u32;
    let mut index_part: PacketPiece = PacketPiece {
        tag: 0,
        check_sum: 0,
        offset: 0,
        length: 0,
        data: ::core::ptr::null_mut::<u8>(),
    };
    index_part.tag = 0 as u32;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table_ix: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table_ix.tag == tag_index {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    index_part = table_ix;
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
        && __fortable_count_0 < packet.num_tables as ::core::ffi::c_int
    {
        let mut table_tx: PacketPiece = *packet.pieces.offset(__fortable_count_0 as isize);
        while __fortable_keep_0 != 0 {
            if table_tx.tag == tag_text {
                let mut __fortable_k2_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2_0 != 0 {
                    text_part = table_tx;
                    __fortable_k2_0 = 0 as ::core::ffi::c_int;
                    __notfound_0 = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep_0 = (__fortable_keep_0 == 0) as ::core::ffi::c_int;
        }
        __fortable_keep_0 = (__fortable_keep_0 == 0) as ::core::ffi::c_int;
        __fortable_count_0 += 1;
    }
    if text_part.tag == 0 || index_part.tag == 0 {
        return ::core::ptr::null_mut::<TsiTable>();
    }
    let mut tsi: *mut TsiTable = (
        TABLE_I_TSI.create.expect("non-null function pointer"))();
    let mut j: u32 = 0 as u32;
    while j.wrapping_mul(8 as u32) < index_part.length {
        let mut gid: u16 = read_16u(
            index_part
                .data
                .offset(j.wrapping_mul(8 as u32) as isize),
        );
        let mut text_length: u32 = read_16u(
            index_part
                .data
                .offset(j.wrapping_mul(8 as u32) as isize)
                .offset(2 as ::core::ffi::c_int as isize),
        ) as u32;
        let mut text_offset: u32 = read_32u(
            index_part
                .data
                .offset(j.wrapping_mul(8 as u32) as isize)
                .offset(4 as ::core::ffi::c_int as isize),
        );
        if !(!is_valid_gid(gid, tag_index) || text_offset >= text_part.length || text_length == 0) {
            let mut predicted_text_length: u32 = text_part.length.wrapping_sub(text_offset);
            let mut k: GlyphId = j.wrapping_add(1 as u32) as GlyphId;
            while ((k as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as u32)
                < index_part.length
            {
                let mut gid_k: u16 = read_16u(
                    index_part
                        .data
                        .offset((k as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize),
                );
                let mut text_offset_k: u32 = read_32u(
                    index_part
                        .data
                        .offset((k as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize),
                );
                if gid_k as ::core::ffi::c_int != 0xfffe as ::core::ffi::c_int
                    && text_offset_k < text_part.length
                    && text_offset_k > text_offset
                {
                    predicted_text_length = text_offset_k.wrapping_sub(text_offset);
                    break;
                } else {
                    k = k.wrapping_add(1);
                }
            }
            if text_length >= 0x8000 as u32 {
                text_length = predicted_text_length;
            }
            let mut entry: TsiEntry = TsiEntry {
                type_0: TsiEntryType::Glyph,
                glyph: Handle {
                    state: HandleState::Empty,
                    index: 0,
                    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                },
                content: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            };
            match gid as ::core::ffi::c_int {
                65530 => {
                    entry.type_0 = TsiEntryType::Prep;
                    otfcc_handle_init(&raw mut entry.glyph);
                }
                65531 => {
                    entry.type_0 = TsiEntryType::Cvt;
                    otfcc_handle_init(&raw mut entry.glyph);
                }
                65533 => {
                    entry.type_0 = TsiEntryType::Fpgm;
                    otfcc_handle_init(&raw mut entry.glyph);
                }
                _ => {
                    entry.type_0 = TsiEntryType::Glyph;
                    entry.glyph = handle_from_index(
                        gid as GlyphId,
                    ) as GlyphHandle;
                }
            }
            entry.content = sdsnewlen(
                text_part.data.offset(text_offset as isize) as *const ::core::ffi::c_void,
                text_length as usize,
            );
            TABLE_I_TSI.push.expect("non-null function pointer")(tsi, entry);
        }
        j = j.wrapping_add(1);
    }
    return tsi;
}
pub unsafe extern "C" fn otfcc_dump_tsi(
    mut tsi: *const TsiTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) {
    if tsi.is_null() {
        return;
    }
    (*(*options).logger)
        .start_sds
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
                    != TsiEntryType::Glyph as ::core::ffi::c_int as ::core::ffi::c_uint)
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
                    == TsiEntryType::Glyph as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    let mut extra_key: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    match (*entry_0).type_0 as ::core::ffi::c_uint {
                        3 => {
                            extra_key = b"cvt\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        1 => {
                            extra_key = b"fpgm\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        2 => {
                            extra_key = b"prep\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        _ => {
                            extra_key = b"reserved\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                    }
                    json_object_push(
                        _extra,
                        extra_key,
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
pub unsafe extern "C" fn otfcc_parse_tsi(
    mut root: *const JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut TsiTable {
    let mut _tsi: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    _tsi = json_obj_get_type(root, tag, JsonType::Object);
    if _tsi.is_null() {
        return ::core::ptr::null_mut::<TsiTable>();
    }
    let mut tsi: *mut TsiTable = (
        TABLE_I_TSI.create.expect("non-null function pointer"))();
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), tag),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _glyphs: *mut JsonValue = json_obj_get_type(
            _tsi,
            b"glyphs\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Object,
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
                    || (*_content).type_0 != JsonType::String)
                {
                    TABLE_I_TSI.push.expect("non-null function pointer")(
                        tsi,
                        TsiEntry {
                            type_0: TsiEntryType::Glyph,
                            glyph: handle_from_name(
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
            JsonType::Object,
        );
        if !_extra.is_null() {
            let mut j_0: u32 = 0 as u32;
            while j_0 < (*_extra).u.object.length as u32 {
                let mut _key: *mut ::core::ffi::c_char =
                    (*(*_extra).u.object.values.offset(j_0 as isize)).name;
                let mut _content_0: *mut JsonValue =
                    (*(*_extra).u.object.values.offset(j_0 as isize)).value as *mut JsonValue;
                if !(_content_0.is_null()
                    || (*_content_0).type_0 != JsonType::String)
                {
                    if strcmp(_key, b"cvt\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        TABLE_I_TSI.push.expect("non-null function pointer")(
                            tsi,
                            TsiEntry {
                                type_0: TsiEntryType::Cvt,
                                glyph: otfcc_handle_empty() as GlyphHandle,
                                content: sdsnewlen(
                                    (*_content_0).u.string.ptr as *const ::core::ffi::c_void,
                                    (*_content_0).u.string.length as usize,
                                ),
                            },
                        );
                    } else if strcmp(_key, b"fpgm\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        TABLE_I_TSI.push.expect("non-null function pointer")(
                            tsi,
                            TsiEntry {
                                type_0: TsiEntryType::Fpgm,
                                glyph: otfcc_handle_empty() as GlyphHandle,
                                content: sdsnewlen(
                                    (*_content_0).u.string.ptr as *const ::core::ffi::c_void,
                                    (*_content_0).u.string.length as usize,
                                ),
                            },
                        );
                    } else if strcmp(_key, b"prep\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        TABLE_I_TSI.push.expect("non-null function pointer")(
                            tsi,
                            TsiEntry {
                                type_0: TsiEntryType::Prep,
                                glyph: otfcc_handle_empty() as GlyphHandle,
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
unsafe extern "C" fn push_tsi_entries(
    mut target: *mut TsiBuildTarget,
    mut tsi: *const TsiTable,
    type_0: TsiEntryType,
    min_n: GlyphId,
) {
    let mut items_pushed: GlyphId = 0 as GlyphId;
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < (*tsi).length {
        let mut entry: *mut TsiEntry = (*tsi).items.offset(__caryll_index as isize);
        while keep != 0 {
            if !((*entry).type_0 as ::core::ffi::c_uint != type_0 as ::core::ffi::c_uint) {
                let mut length_sofar: usize = (*(*target).text_part).cursor;
                bufwrite_sds((*target).text_part, (*entry).content);
                let mut length_after: usize = (*(*target).text_part).cursor;
                bufwrite16b((*target).index_part, propergid(entry, type_0) as u16);
                if length_after.wrapping_sub(length_sofar) < 0x8000 as usize {
                    bufwrite16b(
                        (*target).index_part,
                        length_after.wrapping_sub(length_sofar) as u16,
                    );
                } else {
                    bufwrite16b((*target).index_part, 0x8000 as u16);
                }
                bufwrite32b((*target).index_part, length_sofar as u32);
                items_pushed =
                    (items_pushed as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    while (items_pushed as ::core::ffi::c_int) < min_n as ::core::ffi::c_int {
        bufwrite16b(
            (*target).index_part,
            propergid(::core::ptr::null_mut::<TsiEntry>(), type_0) as u16,
        );
        bufwrite16b((*target).index_part, 0 as u16);
        bufwrite32b(
            (*target).index_part,
            (*(*target).text_part).cursor as u32,
        );
        items_pushed = (items_pushed as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
    }
}
pub unsafe extern "C" fn otfcc_build_tsi(
    mut tsi: *const TsiTable,
    mut _options: *const Options,
) -> TsiBuildTarget {
    let mut target: TsiBuildTarget = TsiBuildTarget {
        index_part: ::core::ptr::null_mut::<Buffer>(),
        text_part: ::core::ptr::null_mut::<Buffer>(),
    };
    if tsi.is_null() {
        target.text_part = ::core::ptr::null_mut::<Buffer>();
        target.index_part = ::core::ptr::null_mut::<Buffer>();
    } else {
        target.text_part = bufnew();
        target.index_part = bufnew();
        push_tsi_entries(&raw mut target, tsi, TsiEntryType::Glyph, 0 as GlyphId);
        bufwrite16b(target.index_part, 0xfffe as u16);
        bufwrite16b(target.index_part, 0 as u16);
        bufwrite32b(target.index_part, 0xabfc1f34 as u32);
        push_tsi_entries(&raw mut target, tsi, TsiEntryType::Prep, 1 as GlyphId);
        push_tsi_entries(&raw mut target, tsi, TsiEntryType::Cvt, 1 as GlyphId);
        push_tsi_entries(&raw mut target, tsi, TsiEntryType::ReservedFffc, 1 as GlyphId);
        push_tsi_entries(&raw mut target, tsi, TsiEntryType::Fpgm, 1 as GlyphId);
    }
    return target;
}
