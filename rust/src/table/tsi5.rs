#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::parsed_json::ParsedValue;
use crate::table::otl::classdef::{ClassDef, push_class_def};

use crate::support::handle::handle_from_index;

use crate::support::font_reader::FontReader;

use crate::font::caryll_sfnt::Packet;
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::primitives::{GlyphClass, GlyphId};
use crate::table::otl::classdef::{dump_class_def, parse_class_def};
use crate::vendor::json::JsonType;

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
pub fn otfcc_read_tsi5(packet: &Packet) -> Option<Box<Tsi5Table>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_TSI5)?;
    // Built as a plain local value rather than through
    // `otl_class_def_create()`/`unwrap_class_def` (both stay, for
    // `classdef.rs`'s own raw-pointer-constructible callers elsewhere) --
    // `push_class_def` is already a safe `fn`, so nothing here needs a box
    // until the very end.
    let mut tsi5 = ClassDef {
        maxclass: 0,
        glyphs: Vec::new(),
        classes: Vec::new(),
    };
    let mut r = FontReader::new(&table.data);
    let mut j: GlyphId = 0 as GlyphId;
    while let Ok(class) = r.u16() {
        push_class_def(&mut tsi5, handle_from_index(j), class as GlyphClass);
        j = j.wrapping_add(1);
    }
    Some(Box::new(tsi5))
}
pub fn otfcc_dump_tsi5(table: Option<&Tsi5Table>, root: &mut BuiltValue) {
    let Some(table) = table else {
        return;
    };
    root.push_field(b"TSI5", dump_class_def(table));
}
pub fn otfcc_parse_tsi5(root: &ParsedValue) -> Option<Box<Tsi5Table>> {
    let tsi = root.get_typed(b"TSI5", JsonType::Object)?;
    let raw = parse_class_def(Some(tsi));
    if raw.is_null() {
        return None;
    }
    // `parse_class_def` genuinely can return null (an empty/absent object),
    // so this is a real check, not shell residue -- `unwrap_class_def`
    // stays `unsafe fn` (its own `Box::from_raw` boundary), narrow bridge
    // only.
    Some(unsafe { unwrap_class_def(raw) })
}
pub fn otfcc_build_tsi5(tsi5: Option<&Tsi5Table>, num_glyphs: GlyphId) -> Option<Buffer> {
    let tsi5 = tsi5?;
    let mut tsi5cls: Vec<u16> = vec![0; num_glyphs as usize];
    for j in 0..tsi5.glyphs.len() {
        if (tsi5.glyphs[j].index as i32) < num_glyphs as i32 {
            tsi5cls[tsi5.glyphs[j].index as usize] = tsi5.classes[j];
        }
    }
    let mut buf = Buffer::new();
    for j_0 in 0..num_glyphs {
        buf.write_u16be(tsi5cls[j_0 as usize]);
    }
    Some(buf)
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
        let packet = packet_with_tsi5(data);
        let table = otfcc_read_tsi5(&packet).unwrap();
        assert_eq!(table.classes, vec![5, 300]);
        assert_eq!(table.glyphs.len(), 2);
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
        let packet = packet_with_tsi5(data);
        let table = otfcc_read_tsi5(&packet).unwrap();
        assert_eq!(table.classes, vec![5]);
    }

    #[test]
    fn empty_table_produces_an_empty_class_def() {
        let packet = packet_with_tsi5(Vec::new());
        let table = otfcc_read_tsi5(&packet).unwrap();
        assert!(table.classes.is_empty());
    }
}
