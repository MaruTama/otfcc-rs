#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::parsed_json::{ParsedValue, json_obj_get_type};
use crate::table::otl::classdef::{ClassDef, otl_class_def_create, push_class_def};

use crate::support::handle::{handle_from_index, GlyphHandle};

use crate::support::font_reader::{FontReader};

use crate::support::buffer::{Buffer};
use crate::support::primitives::{GlyphClass, GlyphId};
use crate::vendor::json::{JsonType};
use crate::font::caryll_sfnt::{Packet};
use crate::support::buffer::{bufnew, bufwrite16b};
use crate::table::otl::classdef::{dump_class_def, parse_class_def};
use crate::support::built_json::{BuiltValue, json_object_push};


pub type Tsi5Table = ClassDef;
// Stage 6-4 "Box化": `Font.tsi5` becomes `Option<Box<Tsi5Table>>`.
// `ClassDef` itself stays a raw-pointer-constructible type everywhere else
// in the crate (`otl_class_def_create`/`parse_class_def`/`read_class_def`
// used throughout `otl`/`gdef` construction and consolidation, and adopted
// into an owned `Option<Box<ClassDef>>` only at each field's own assignment
// site via `classdef_from_raw` -- see `GdefTable.glyph_class_def`/
// `.mark_attach_class_def`, Stage 7-2-c) -- widening those constructors
// themselves to return `Box<ClassDef>` would ripple across all of those,
// well beyond this field's own scope. Instead, `unwrap_class_def` "adopts"
// the value into a genuine `Box`: since `otl_class_def_create` itself allocates via
// `Box::into_raw` now, `Box::from_raw` reclaims that exact allocation
// directly -- no read-then-free-then-reallocate needed (and reaching for
// `free` here would be wrong regardless: it must match `Box::into_raw`, not
// libc's allocator, even though the two happen to coincide today).
unsafe fn unwrap_class_def(raw: *mut ClassDef) -> Box<ClassDef> {
    Box::from_raw(raw)
}
// The original loop condition (`j * 2 < table.length`) admitted one
// out-of-bounds 2-byte read whenever `table.length` was odd: e.g. a
// 1-byte table has `j = 0` satisfy `0 < 1`, then reads bytes `[0, 1]` --
// the second of which does not exist. `FontReader::u16` requires both
// bytes to actually be present, so the loop below now stops one entry
// earlier on an odd-length table instead of reading past the end; a
// well-formed (even-length) table parses identically to before.
pub unsafe fn otfcc_read_tsi5(
    packet: &Packet,
) -> Option<Box<Tsi5Table>> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_TSI5)?;
    let tsi5: *mut Tsi5Table = otl_class_def_create() as *mut Tsi5Table;
    let mut r = FontReader::new(&table.data);
    let mut j: GlyphId = 0 as GlyphId;
    while let Ok(class) = r.u16() {
        push_class_def(
            tsi5 as *mut ClassDef,
            handle_from_index(j) as GlyphHandle,
            class as GlyphClass,
        );
        j = j.wrapping_add(1);
    }
    Some(unwrap_class_def(tsi5))
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_tsi5(
    table: Option<&Tsi5Table>,
    mut root: *mut BuiltValue,
) {
    let table = match table {
        Some(t) => t as *const Tsi5Table,
        None => return,
    };
    json_object_push(
        root,
        b"TSI5\0" as *const u8 as *const ::core::ffi::c_char,
        dump_class_def(table as *const ClassDef),
    );
}
pub unsafe fn otfcc_parse_tsi5(
    mut root: *const ParsedValue,
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
    let raw = parse_class_def(_tsi);
    if raw.is_null() {
        return None;
    }
    return Some(unwrap_class_def(raw as *mut ClassDef));
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_tsi5(
    tsi5: Option<&Tsi5Table>,
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

#[cfg(test)]
mod otfcc_read_tsi5_tests {
    use super::*;
    use crate::font::caryll_sfnt::PacketPiece;

    fn packet_with_tsi5(data: Vec<u8>) -> Packet {
        Packet {
            sfnt_version: 0,
            num_tables: 1,
            search_range: 0,
            entry_selector: 0,
            range_shift: 0,
            pieces: vec![PacketPiece {
                tag: crate::tag::TAG_TSI5,
                check_sum: 0,
                offset: 0,
                length: data.len() as u32,
                data,
            }],
        }
    }

    #[test]
    fn even_length_table_reads_every_class() {
        // Two glyphs: gid 0 -> class 5, gid 1 -> class 300.
        let data = vec![0x00, 0x05, 0x01, 0x2C];
        unsafe {
            let packet = packet_with_tsi5(data);
            let table = otfcc_read_tsi5(&packet).unwrap();
            assert_eq!(table.classes, vec![5, 300]);
            assert_eq!(table.glyphs.len(), 2);
        }
    }

    #[test]
    fn odd_length_table_drops_the_trailing_byte_instead_of_reading_oob() {
        // No committed payload has a TSI5 table (checked by hand against
        // every tests/payload/*.ttf), so this is the only coverage of the
        // original off-by-one: `j * 2 < table.length` let a 1-byte table
        // read 2 bytes -- 1 byte past the end. FontReader::u16 requires
        // both bytes to be present, so the trailing odd byte is dropped
        // instead of read.
        let data = vec![0x00, 0x05, 0xFF]; // one full entry + one stray byte
        unsafe {
            let packet = packet_with_tsi5(data);
            let table = otfcc_read_tsi5(&packet).unwrap();
            assert_eq!(table.classes, vec![5]);
        }
    }

    #[test]
    fn empty_table_produces_an_empty_class_def() {
        unsafe {
            let packet = packet_with_tsi5(Vec::new());
            let table = otfcc_read_tsi5(&packet).unwrap();
            assert!(table.classes.is_empty());
        }
    }
}
