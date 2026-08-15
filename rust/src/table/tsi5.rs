#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::parsed_json::{ParsedValue, json_obj_get_type};
use crate::table::otl::classdef::{ClassDef, otl_class_def_create, push_class_def};

use crate::support::handle::{handle_from_index, GlyphHandle};

use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphClass, GlyphId};
use crate::vendor::json::{JsonType};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::buffer::{bufnew, bufwrite16b};
use crate::table::otl::classdef::{OTL_I_CLASS_DEF};
use crate::support::built_json::{BuiltValue, json_object_push};


pub type Tsi5Table = ClassDef;
// Stage 6-4 "Box化": `Font.tsi5` becomes `Option<Box<Tsi5Table>>`.
// `ClassDef` itself stays a raw-pointer-constructible type everywhere else
// in the crate (`GdefTable.glyph_class_def`/`.mark_attach_class_def`, the
// `OTL_I_CLASS_DEF` package used throughout `otl`/`gdef` consolidation) --
// widening `otl_class_def_create`/`OTL_I_CLASS_DEF.parse` themselves to
// return `Box<ClassDef>` would ripple across all of those, well beyond this
// field's own scope. Instead, `unwrap_class_def` "adopts" the value into a
// genuine `Box`: since `otl_class_def_create` itself allocates via
// `Box::into_raw` now, `Box::from_raw` reclaims that exact allocation
// directly -- no read-then-free-then-reallocate needed (and reaching for
// `free` here would be wrong regardless: it must match `Box::into_raw`, not
// libc's allocator, even though the two happen to coincide today).
unsafe fn unwrap_class_def(raw: *mut ClassDef) -> Box<ClassDef> {
    Box::from_raw(raw)
}
pub unsafe extern "C" fn otfcc_read_tsi5(
    packet: &Packet,
    mut _options: *const Options,
) -> Option<Box<Tsi5Table>> {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_TSI5 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut tsi5: *mut Tsi5Table =
                        otl_class_def_create() as *mut Tsi5Table;
                    let mut j: GlyphId = 0 as GlyphId;
                    while ((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32)
                        < table.length
                    {
                        push_class_def(
                            tsi5 as *mut ClassDef,
                            handle_from_index(j)
                                as GlyphHandle,
                            read_16u(table.data.as_ptr().offset(
                                (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                            )) as GlyphClass,
                        );
                        j = j.wrapping_add(1);
                    }
                    return Some(unwrap_class_def(tsi5));
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return None;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_dump_tsi5(
    table: Option<&Tsi5Table>,
    mut root: *mut BuiltValue,
    mut _options: *const Options,
) {
    let table = match table {
        Some(t) => t as *const Tsi5Table,
        None => return,
    };
    json_object_push(
        root,
        b"TSI5\0" as *const u8 as *const ::core::ffi::c_char,
        OTL_I_CLASS_DEF.dump.expect("non-null function pointer")(table as *const ClassDef),
    );
}
pub unsafe extern "C" fn otfcc_parse_tsi5(
    mut root: *const ParsedValue,
    mut _options: *const Options,
) -> Option<Box<Tsi5Table>> {
    let mut _tsi: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    _tsi = json_obj_get_type(
        root,
        b"TSI5\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if _tsi.is_null() {
        return None;
    }
    let raw = OTL_I_CLASS_DEF.parse.expect("non-null function pointer")(_tsi);
    if raw.is_null() {
        return None;
    }
    return Some(unwrap_class_def(raw as *mut ClassDef));
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_tsi5(
    tsi5: Option<&Tsi5Table>,
    mut _options: *const Options,
    mut num_glyphs: GlyphId,
) -> *mut Buffer {
    let tsi5 = match tsi5 {
        Some(t) => t as *const Tsi5Table,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut tsi5cls: Vec<u16> = vec![0; num_glyphs as usize];
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*tsi5).glyphs.len() {
        if ((&(*tsi5).glyphs)[j as usize].index as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int
        {
            tsi5cls[(&(*tsi5).glyphs)[j as usize].index as usize] =
                (&(*tsi5).classes)[j as usize] as u16;
        }
        j = j.wrapping_add(1);
    }
    let mut buf: *mut Buffer = bufnew();
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
        bufwrite16b(buf, tsi5cls[j_0 as usize]);
        j_0 = j_0.wrapping_add(1);
    }
    return buf;
}
