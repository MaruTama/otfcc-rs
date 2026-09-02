#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::built_json::{BuiltValue, json_object_push};
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::glyph_order::GlyphOrder;
use crate::support::glyph_order::otfcc_set_glyph_order_by_gid;
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_obj_get_type, json_obj_getbool, json_obj_getnum,
};
use crate::support::primitives::{F16Dot16, GlyphId};
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::vendor::json::JsonType;

pub struct PostTable {
    pub version: F16Dot16,
    pub italic_angle: F16Dot16,
    pub underline_position: i16,
    pub underline_thickness: i16,
    pub is_fixed_pitch: u32,
    pub min_mem_type42: u32,
    pub max_mem_type42: u32,
    pub min_mem_type1: u32,
    pub max_mem_type1: u32,
    pub post_name_map: Option<Box<GlyphOrder>>,
}
// `post_name_map` (populated only for a version-2.0 'post' table, during
// OTF read) is an owned `GlyphOrder`, built directly via `Box::new` --
// matching `Font.glyph_order`'s "accumulator is `Option<Box<X>>` from the
// start" idiom (see `consolidate.rs`/`json_reader.rs`). `GlyphOrder` itself
// has no `Drop` impl (its own `Vec`/`BTreeMap`/`HashMap` fields' drop glue
// is sufficient -- see `support/glyph_order.rs`), so `Box`'s own drop glue
// is enough here too and `PostTable` needs no `Drop` impl of its own: it
// owns no other raw allocation.
//
// `post_name_map` is a standalone allocation, not an alias into
// `Font.glyph_order`: `otf_reader/unconsolidate.rs`'s `create_glyph_order`
// only ever *reads* from it (to backfill glyph names from a version-2.0
// 'post' table) and never stores it anywhere else, so there is no other
// owner to worry about aliasing with.
static STANDARD_MAC_NAMES: [&::core::ffi::CStr; 258] = [
    c".notdef",
    c".null",
    c"nonmarkingreturn",
    c"space",
    c"exclam",
    c"quotedbl",
    c"numbersign",
    c"dollar",
    c"percent",
    c"ampersand",
    c"quotesingle",
    c"parenleft",
    c"parenright",
    c"asterisk",
    c"plus",
    c"comma",
    c"hyphen",
    c"period",
    c"slash",
    c"zero",
    c"one",
    c"two",
    c"three",
    c"four",
    c"five",
    c"six",
    c"seven",
    c"eight",
    c"nine",
    c"colon",
    c"semicolon",
    c"less",
    c"equal",
    c"greater",
    c"question",
    c"at",
    c"A",
    c"B",
    c"C",
    c"D",
    c"E",
    c"F",
    c"G",
    c"H",
    c"I",
    c"J",
    c"K",
    c"L",
    c"M",
    c"N",
    c"O",
    c"P",
    c"Q",
    c"R",
    c"S",
    c"T",
    c"U",
    c"V",
    c"W",
    c"X",
    c"Y",
    c"Z",
    c"bracketleft",
    c"backslash",
    c"bracketright",
    c"asciicircum",
    c"underscore",
    c"grave",
    c"a",
    c"b",
    c"c",
    c"d",
    c"e",
    c"f",
    c"g",
    c"h",
    c"i",
    c"j",
    c"k",
    c"l",
    c"m",
    c"n",
    c"o",
    c"p",
    c"q",
    c"r",
    c"s",
    c"t",
    c"u",
    c"v",
    c"w",
    c"x",
    c"y",
    c"z",
    c"braceleft",
    c"bar",
    c"braceright",
    c"asciitilde",
    c"Adieresis",
    c"Aring",
    c"Ccedilla",
    c"Eacute",
    c"Ntilde",
    c"Odieresis",
    c"Udieresis",
    c"aacute",
    c"agrave",
    c"acircumflex",
    c"adieresis",
    c"atilde",
    c"aring",
    c"ccedilla",
    c"eacute",
    c"egrave",
    c"ecircumflex",
    c"edieresis",
    c"iacute",
    c"igrave",
    c"icircumflex",
    c"idieresis",
    c"ntilde",
    c"oacute",
    c"ograve",
    c"ocircumflex",
    c"odieresis",
    c"otilde",
    c"uacute",
    c"ugrave",
    c"ucircumflex",
    c"udieresis",
    c"dagger",
    c"degree",
    c"cent",
    c"sterling",
    c"section",
    c"bullet",
    c"paragraph",
    c"germandbls",
    c"registered",
    c"copyright",
    c"trademark",
    c"acute",
    c"dieresis",
    c"notequal",
    c"AE",
    c"Oslash",
    c"infinity",
    c"plusminus",
    c"lessequal",
    c"greaterequal",
    c"yen",
    c"mu",
    c"partialdiff",
    c"summation",
    c"product",
    c"pi",
    c"integral",
    c"ordfeminine",
    c"ordmasculine",
    c"Omega",
    c"ae",
    c"oslash",
    c"questiondown",
    c"exclamdown",
    c"logicalnot",
    c"radical",
    c"florin",
    c"approxequal",
    c"Delta",
    c"guillemotleft",
    c"guillemotright",
    c"ellipsis",
    c"nonbreakingspace",
    c"Agrave",
    c"Atilde",
    c"Otilde",
    c"OE",
    c"oe",
    c"endash",
    c"emdash",
    c"quotedblleft",
    c"quotedblright",
    c"quoteleft",
    c"quoteright",
    c"divide",
    c"lozenge",
    c"ydieresis",
    c"Ydieresis",
    c"fraction",
    c"currency",
    c"guilsinglleft",
    c"guilsinglright",
    c"fi",
    c"fl",
    c"daggerdbl",
    c"periodcentered",
    c"quotesinglbase",
    c"quotedblbase",
    c"perthousand",
    c"Acircumflex",
    c"Ecircumflex",
    c"Aacute",
    c"Edieresis",
    c"Egrave",
    c"Iacute",
    c"Icircumflex",
    c"Idieresis",
    c"Igrave",
    c"Oacute",
    c"Ocircumflex",
    c"apple",
    c"Ograve",
    c"Uacute",
    c"Ucircumflex",
    c"Ugrave",
    c"dotlessi",
    c"circumflex",
    c"tilde",
    c"macron",
    c"breve",
    c"dotaccent",
    c"ring",
    c"cedilla",
    c"hungarumlaut",
    c"ogonek",
    c"caron",
    c"Lslash",
    c"lslash",
    c"Scaron",
    c"scaron",
    c"Zcaron",
    c"zcaron",
    c"brokenbar",
    c"Eth",
    c"eth",
    c"Yacute",
    c"yacute",
    c"Thorn",
    c"thorn",
    c"minus",
    c"multiply",
    c"onesuperior",
    c"twosuperior",
    c"threesuperior",
    c"onehalf",
    c"onequarter",
    c"threequarters",
    c"franc",
    c"Gbreve",
    c"gbreve",
    c"Idotaccent",
    c"Scedilla",
    c"scedilla",
    c"Cacute",
    c"cacute",
    c"Ccaron",
    c"ccaron",
    c"dcroat",
];
// The fallible, allocation-free half of the read: parses the fixed header
// and (for a version-2.0 table) the whole name-index/name-heap structure
// into plain owned Rust values first, and only builds the `GlyphOrder` (via
// `otfcc_glyph_order_create`/`otfcc_set_glyph_order_by_gid`, both raw-
// pointer FFI-shaped calls) once every read has already succeeded -- so an
// `Err` here never leaves a partially-built `GlyphOrder` to clean up.
struct ParsedPost {
    fixed: PostFixedHeader,
    // `None` unless `fixed.version == 0x20000`.
    names: Option<Vec<(GlyphId, Vec<u8>)>>,
}
struct PostFixedHeader {
    version: F16Dot16,
    italic_angle: F16Dot16,
    underline_position: i16,
    underline_thickness: i16,
    is_fixed_pitch: u32,
    min_mem_type42: u32,
    max_mem_type42: u32,
    min_mem_type1: u32,
    max_mem_type1: u32,
}

fn parse_post(data: &[u8]) -> Result<ParsedPost, ReadError> {
    let mut r = FontReader::new(data);
    let fixed = PostFixedHeader {
        version: r.i32()?,
        italic_angle: r.u32()? as F16Dot16,
        underline_position: r.i16()?,
        underline_thickness: r.i16()?,
        is_fixed_pitch: r.u32()?,
        min_mem_type42: r.u32()?,
        max_mem_type42: r.u32()?,
        min_mem_type1: r.u32()?,
        max_mem_type1: r.u32()?,
    };
    if fixed.version != 0x20000 {
        return Ok(ParsedPost { fixed, names: None });
    }
    let number_glyphs = r.u16()?;
    // The fixed-size `glyphNameIndex[numberOfGlyphs]` array itself must be
    // present -- `require_room` also rejects a `number_glyphs` so large
    // that `2 * number_glyphs` would overflow before ever comparing
    // against the table's real (much smaller) length.
    r.require_room(number_glyphs as usize, 2)?;
    let mut offset = 34usize.wrapping_add(2 * number_glyphs as usize);
    // Pascal-string name heap: a 1-byte length prefix, then that many
    // bytes, repeated until the table ends. Bounded purely by `offset <
    // data.len()`, same as the original C -- each entry consumes at least
    // 1 byte, so this always terminates.
    let mut pending_names: Vec<Vec<u8>> = Vec::new();
    while offset < data.len() {
        let mut sr = r.at(offset)?;
        let len = sr.u8()?;
        let name = if len > 0 {
            sr.bytes(len as usize)?.to_vec()
        } else {
            Vec::new()
        };
        offset = offset.wrapping_add(1 + len as usize);
        pending_names.push(name);
    }
    let mut names = Vec::with_capacity(number_glyphs as usize);
    for j in 0..number_glyphs {
        let mut nr = r.at(34usize.wrapping_add(2 * j as usize))?;
        let name_map = nr.u16()?;
        let name = if name_map as usize >= 258 {
            match pending_names.get(name_map as usize - 258) {
                Some(n) => n.clone(),
                // A `glyphNameIndex` entry pointing past the actual name
                // heap: the original C read whatever `pending_names[...]`
                // happened to occupy past the end of the allocation. There
                // is no well-formed name to recover here, so this glyph
                // gets an empty one instead of that garbage -- the same
                // "corrupted input loses this one piece of data instead of
                // reading past a buffer" trade the rest of this migration
                // makes.
                None => Vec::new(),
            }
        } else {
            STANDARD_MAC_NAMES[name_map as usize].to_bytes().to_vec()
        };
        names.push((j as GlyphId, name));
    }
    Ok(ParsedPost {
        fixed,
        names: Some(names),
    })
}

pub fn otfcc_read_post(packet: &Packet, options: &Options) -> Option<Box<PostTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_POST)?;
    let parsed = match parse_post(&table.data) {
        Ok(parsed) => parsed,
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"table 'post' corrupted.\n"),
            );
            return None;
        }
    };
    let mut post_val = PostTable {
        version: parsed.fixed.version,
        italic_angle: parsed.fixed.italic_angle,
        underline_position: parsed.fixed.underline_position,
        underline_thickness: parsed.fixed.underline_thickness,
        is_fixed_pitch: parsed.fixed.is_fixed_pitch,
        min_mem_type42: parsed.fixed.min_mem_type42,
        max_mem_type42: parsed.fixed.max_mem_type42,
        min_mem_type1: parsed.fixed.min_mem_type1,
        max_mem_type1: parsed.fixed.max_mem_type1,
        post_name_map: None,
    };
    if let Some(names) = parsed.names {
        let mut go_box: Box<GlyphOrder> = Box::new(GlyphOrder {
            entries: Vec::new(),
            by_gid: ::std::collections::BTreeMap::new(),
            by_name: ::std::collections::HashMap::new(),
        });
        let go: *mut GlyphOrder = go_box.as_mut() as *mut GlyphOrder;
        for (gid, name) in names {
            unsafe { otfcc_set_glyph_order_by_gid(go, gid, name) };
        }
        post_val.post_name_map = Some(go_box);
    }
    Some(Box::new(post_val))
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_post(
    table: Option<&PostTable>,
    root: *mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t as *const PostTable,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"post"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut post = BuiltValue::new_object(10);
        post.push_field(
            b"version",
            BuiltValue::Double(otfcc_from_fixed((*table).version)),
        );
        post.push_field(
            b"italicAngle",
            BuiltValue::Int(otfcc_from_fixed((*table).italic_angle) as i64),
        );
        post.push_field(
            b"underlinePosition",
            BuiltValue::Int((*table).underline_position as i64),
        );
        post.push_field(
            b"underlineThickness",
            BuiltValue::Int((*table).underline_thickness as i64),
        );
        post.push_field(b"isFixedPitch", BuiltValue::Bool((*table).is_fixed_pitch != 0));
        post.push_field(
            b"minMemType42",
            BuiltValue::Int((*table).min_mem_type42 as i64),
        );
        post.push_field(
            b"maxMemType42",
            BuiltValue::Int((*table).max_mem_type42 as i64),
        );
        post.push_field(
            b"minMemType1",
            BuiltValue::Int((*table).min_mem_type1 as i64),
        );
        post.push_field(
            b"maxMemType1",
            BuiltValue::Int((*table).max_mem_type1 as i64),
        );
        json_object_push(
            root,
            b"post\0" as *const u8 as *const ::core::ffi::c_char,
            post.into_raw(),
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_parse_post(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<PostTable>> {
    // `.version`'s `0x30000` default carries through if the "post" JSON key
    // is absent (never overwritten below in that case, unlike every other
    // field); `post_name_map` is never touched here regardless, so its
    // zeroed-to-`None` value (a valid bit pattern for `Option<Box<T>>` via
    // the null-pointer niche optimization) is already the old `init_post`'s
    // default.
    let mut post_val: PostTable = ::core::mem::zeroed();
    post_val.version = 0x30000_i32 as F16Dot16;
    let mut post_box: Box<PostTable> = Box::new(post_val);
    let post: *mut PostTable = post_box.as_mut() as *mut PostTable;
    let table: *const ParsedValue;
    table = json_obj_get_type(
        root,
        b"post\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"post"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            if options.short_post {
                (*post).version = 0x30000_i32 as F16Dot16;
            } else {
                (*post).version = otfcc_to_fixed(json_obj_getnum(
                    table,
                    b"version\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            }
            (*post).italic_angle = otfcc_to_fixed(json_obj_getnum(
                table,
                b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            (*post).underline_position = json_obj_getnum(
                table,
                b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            ) as i16;
            (*post).underline_thickness = json_obj_getnum(
                table,
                b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            ) as i16;
            (*post).is_fixed_pitch = json_obj_getbool(
                table,
                b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).min_mem_type42 = json_obj_getnum(
                table,
                b"minMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).max_mem_type42 = json_obj_getnum(
                table,
                b"maxMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).min_mem_type1 = json_obj_getnum(
                table,
                b"minMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).max_mem_type1 = json_obj_getnum(
                table,
                b"maxMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
    }
    return Some(post_box);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_post(
    post: Option<&PostTable>,
    glyphorder: *mut GlyphOrder,
) -> Option<Buffer> {
    let post = post?;
    let mut buf = Buffer::new();
    buf.write_u32be(post.version as u32);
    buf.write_u32be(post.italic_angle as u32);
    buf.write_u16be(post.underline_position as u16);
    buf.write_u16be(post.underline_thickness as u16);
    buf.write_u32be(post.is_fixed_pitch);
    buf.write_u32be(post.min_mem_type42);
    buf.write_u32be(post.max_mem_type42);
    buf.write_u32be(post.min_mem_type1);
    buf.write_u32be(post.max_mem_type1);
    if post.version == 0x20000 as F16Dot16 {
        // Walks `by_gid` (ascending gid order), not `by_name`: by the time
        // this runs, `by_name`'s uthash chain had already been sorted by
        // `order_glyphs` (json_reader.rs) into exactly this order and
        // `by_gid` built by walking it -- so this reproduces the original's
        // effective iteration order without depending on `HashMap`'s
        // (unspecified) iteration order the way a literal `by_name` walk
        // would have to.
        buf.write_u16be((*glyphorder).by_gid.len() as u16);
        for (_, &idx) in (*glyphorder).by_gid.iter() {
            let entry = &(&(*glyphorder).entries)[idx];
            buf.write_u16be((258_i32 + entry.gid as i32) as u16);
        }
        for (_, &idx) in (*glyphorder).by_gid.iter() {
            let entry = &(&(*glyphorder).entries)[idx];
            buf.write_u8(entry.name.len() as u8);
            buf.write_bytes(&entry.name);
        }
    }
    Some(buf)
}

#[cfg(test)]
mod parse_post_tests {
    use super::*;

    fn header(version: u32, underline_position: i16, underline_thickness: i16) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&version.to_be_bytes());
        b.extend_from_slice(&0u32.to_be_bytes()); // italic_angle
        b.extend_from_slice(&underline_position.to_be_bytes());
        b.extend_from_slice(&underline_thickness.to_be_bytes());
        b.extend_from_slice(&0u32.to_be_bytes()); // is_fixed_pitch
        b.extend_from_slice(&0u32.to_be_bytes()); // min_mem_type42
        b.extend_from_slice(&0u32.to_be_bytes()); // max_mem_type42
        b.extend_from_slice(&0u32.to_be_bytes()); // min_mem_type1
        b.extend_from_slice(&0u32.to_be_bytes()); // max_mem_type1
        assert_eq!(b.len(), 32);
        b
    }

    #[test]
    fn version_1_header_has_no_name_map() {
        let data = header(0x00010000, -100, 50);
        let parsed = parse_post(&data).unwrap();
        assert_eq!(parsed.fixed.version, 0x00010000);
        assert_eq!(parsed.fixed.underline_position, -100);
        assert_eq!(parsed.fixed.underline_thickness, 50);
        assert!(parsed.names.is_none());
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        // Only 10 of the required 32 header bytes -- this table used to be
        // read unconditionally (otfcc_read_post had no length check at
        // all), overreading up to 22 bytes past a table this short.
        let data = header(0x00010000, 0, 0);
        assert!(parse_post(&data[..10]).is_err());
    }

    #[test]
    fn version_2_header_resolves_pending_names() {
        let mut data = header(0x00020000, 0, 0);
        data.extend_from_slice(&1u16.to_be_bytes()); // numberOfGlyphs
        data.extend_from_slice(&258u16.to_be_bytes()); // glyphNameIndex[0] -> pending_names[0]
        data.push(1); // pascal string length
        data.push(b'A'); // pascal string bytes
        let parsed = parse_post(&data).unwrap();
        let names = parsed.names.unwrap();
        assert_eq!(names, vec![(0u16, b"A".to_vec())]);
    }

    #[test]
    fn standard_mac_name_index_below_258_does_not_touch_the_heap() {
        let mut data = header(0x00020000, 0, 0);
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes()); // glyphNameIndex[0] -> ".notdef"
        // No name heap bytes at all -- a standard-Mac-name entry must not
        // need one.
        let parsed = parse_post(&data).unwrap();
        assert_eq!(parsed.names.unwrap(), vec![(0u16, b".notdef".to_vec())]);
    }

    #[test]
    fn pascal_string_length_running_past_the_table_end_errs() {
        // This is the actual pre-existing overread: a name heap entry
        // whose declared length reaches past the table's real end. The
        // original C (and the first Rust translation) read the extra
        // bytes anyway, since the only loop guard was the *offset* of the
        // length byte itself, never `offset + 1 + len`.
        let mut data = header(0x00020000, 0, 0);
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&258u16.to_be_bytes());
        data.push(10); // claims 10 bytes follow
        data.push(b'A'); // only 1 actually does
        assert!(parse_post(&data).is_err());
    }

    #[test]
    fn glyph_name_index_past_the_name_heap_gets_an_empty_name_not_a_panic() {
        // glyphNameIndex[0] claims pending_names[0], but the name heap is
        // empty (the table ends exactly at the index array) -- the
        // original C read whatever bytes happened to sit past the
        // `pending_names` allocation; a naive Rust port of the same index
        // expression would panic instead. Neither is acceptable, so this
        // falls back to an empty name.
        let mut data = header(0x00020000, 0, 0);
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&258u16.to_be_bytes());
        let parsed = parse_post(&data).unwrap();
        assert_eq!(parsed.names.unwrap(), vec![(0u16, Vec::new())]);
    }

    #[test]
    fn glyph_name_index_array_shorter_than_declared_errs() {
        // number_glyphs says 5 entries but only 1 is actually present --
        // require_room on the fixed-size index array should catch this
        // before ever indexing past it.
        let mut data = header(0x00020000, 0, 0);
        data.extend_from_slice(&5u16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes()); // only one entry present
        assert!(parse_post(&data).is_err());
    }

    #[test]
    fn number_glyphs_large_enough_to_overflow_the_multiplication_errs() {
        let mut data = header(0x00020000, 0, 0);
        data.extend_from_slice(&0xFFFFu16.to_be_bytes());
        assert!(parse_post(&data).is_err());
    }
}
