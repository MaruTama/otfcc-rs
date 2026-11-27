//! The `cmap` table: the mapping from code points, and from variation
//! sequences, to glyphs. This file holds the data model and the JSON
//! dump and parse; `read` and `build` handle the binary table.

pub mod build;
pub mod read;

use crate::json_writer::DumpSink;
use crate::logger::ByteStr;
use crate::support::handle::{GlyphHandle, handle_from_index, handle_from_name};
use otfcc_json::ParsedValue;

use crate::support::fmt::Hex4Upper;
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, Unicode, until_nul};
use otfcc_json::BuiltValue;
use otfcc_json::JsonType;
/// A Unicode variation sequence: a base code point and its variation selector.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CmapUvsKey {
    pub unicode: u32,
    pub selector: u32,
}
/// `unicodes` maps code points to glyphs, and `uvs` maps (code point,
/// variation selector) pairs to glyphs. Both are `BTreeMap`s, so they
/// iterate in key order, which is the order the dump and the binary writer
/// need.
#[derive(Debug)]
pub struct CmapTable {
    pub unicodes: std::collections::BTreeMap<i32, GlyphHandle>,
    pub uvs: std::collections::BTreeMap<CmapUvsKey, GlyphHandle>,
    /// Whether the font's Windows mapping is the Symbol encoding (3/0)
    /// rather than Unicode: a font whose only Windows subtables are 3/0 ones.
    /// Its mappings are in `unicodes` all the same. Dumped as
    /// `"cmap_encoding": "symbol"`; the build then files its format 4
    /// subtable under 3/0 alone.
    pub symbol: bool,
}
/// Maps `key` to `glyph` unless it is already mapped; returns whether it
/// was added. The first mapping of a key always wins.
fn map_if_absent<K: Ord>(
    map: &mut std::collections::BTreeMap<K, GlyphHandle>,
    key: K,
    glyph: GlyphHandle,
) -> bool {
    match map.entry(key) {
        std::collections::btree_map::Entry::Vacant(v) => {
            v.insert(glyph);
            true
        }
        std::collections::btree_map::Entry::Occupied(_) => false,
    }
}
/// Maps code point `c` to glyph id `gid`. Returns `false`, keeping the existing
/// mapping, when `c` is already mapped.
pub fn encode_cmap_by_index(cmap: &mut CmapTable, c: i32, gid: u16) -> bool {
    map_if_absent(
        &mut cmap.unicodes,
        c,
        handle_from_index(gid as GlyphId) as GlyphHandle,
    )
}
/// Maps `c` to the glyph named `name`. Returns `false`, keeping the existing
/// mapping, when `c` is already mapped.
pub fn encode_cmap_by_name(cmap: &mut CmapTable, c: i32, name: Vec<u8>) -> bool {
    map_if_absent(
        &mut cmap.unicodes,
        c,
        handle_from_name(Some(name)) as GlyphHandle,
    )
}
/// Removes the mapping of `c`. Returns whether there was one.
pub fn unmap_cmap(cmap: &mut CmapTable, c: i32) -> bool {
    cmap.unicodes.remove(&c).is_some()
}
/// The glyph `c` is mapped to, if any.
pub fn cmap_lookup(cmap: &CmapTable, c: i32) -> Option<&GlyphHandle> {
    cmap.unicodes.get(&c)
}
/// Like [`encode_cmap_by_index`], for a variation sequence.
pub fn encode_cmap_uvs_by_index(cmap: &mut CmapTable, c: CmapUvsKey, gid: u16) -> bool {
    map_if_absent(
        &mut cmap.uvs,
        c,
        handle_from_index(gid as GlyphId) as GlyphHandle,
    )
}
/// Like [`encode_cmap_by_name`], for a variation sequence.
pub fn encode_cmap_uvs_by_name(cmap: &mut CmapTable, c: CmapUvsKey, name: Vec<u8>) -> bool {
    map_if_absent(
        &mut cmap.uvs,
        c,
        handle_from_name(Some(name)) as GlyphHandle,
    )
}
/// Removes the mapping of the variation sequence `c`. Returns whether there was
/// one.
pub fn unmap_cmap_uvs(cmap: &mut CmapTable, c: CmapUvsKey) -> bool {
    cmap.uvs.remove(&c).is_some()
}
/// The glyph the variation sequence `c` is mapped to, if any.
pub fn cmap_lookup_uvs(cmap: &CmapTable, c: CmapUvsKey) -> Option<&GlyphHandle> {
    cmap.uvs.get(&c)
}
/// Writes `cmap` and `cmap_uvs` into `sink` one mapping at a time, so a
/// sink that writes straight out never holds the whole table's JSON. Keys
/// and glyph names are cut at the first NUL, as `push_field_bytes_key` does.
pub fn dump_cmap(table: Option<&CmapTable>, sink: &mut dyn DumpSink, options: &Options) {
    let Some(table) = table else { return };
    let stage = crate::logger::stage("cmap");
    if !table.unicodes.is_empty() {
        sink.begin_field_object(b"cmap");
        for (&unicode, glyph) in table.unicodes.iter() {
            if !glyph.name.is_empty() {
                let key: Vec<u8> = if options.decimal_cmap {
                    crate::bytesbuild!(unicode)
                } else {
                    crate::bytesbuild!(b"U+", Hex4Upper(unicode as u32))
                };
                sink.field(
                    until_nul(&key),
                    BuiltValue::str_truncated_at_nul(&glyph.name),
                );
            }
        }
        sink.end_object();
    }
    if table.symbol {
        sink.field(b"cmap_encoding", BuiltValue::Str(b"symbol".to_vec()));
    }
    if !table.uvs.is_empty() {
        sink.begin_field_object(b"cmap_uvs");
        for (key, glyph) in table.uvs.iter() {
            if !glyph.name.is_empty() {
                let field_name: Vec<u8> = if options.decimal_cmap {
                    crate::bytesbuild!(key.unicode, b" ", key.selector,)
                } else {
                    crate::bytesbuild!(
                        b"U+",
                        Hex4Upper(key.unicode),
                        b" U+",
                        Hex4Upper(key.selector),
                    )
                };
                sink.field(
                    until_nul(&field_name),
                    BuiltValue::str_truncated_at_nul(&glyph.name),
                );
            }
        }
        sink.end_object();
    }
    stage.finish();
}
/// The code point a JSON `cmap` key names: `U+` followed by hex digits, or
/// decimal digits. Anything else (signs, spaces, trailing text, `0x`, an
/// empty number, one too large for `u32`) gives `None`. `unicode_str` is
/// the key without its trailing NUL.
// The digit check comes first because `from_str_radix` would also accept a
// leading `+`.
pub(crate) fn parse_unicode(unicode_str: &[u8]) -> Option<Unicode> {
    let (digits, radix) = match unicode_str.strip_prefix(b"U+") {
        Some(hex) => (hex, 16),
        None => (unicode_str, 10),
    };
    if digits.is_empty() || !digits.iter().all(|&b| (b as char).is_digit(radix)) {
        return None;
    }
    Unicode::from_str_radix(std::str::from_utf8(digits).ok()?, radix).ok()
}
/// Whether `u` may be mapped: U+0000 to U+10FFFF. Fonts map U+0000
/// (often to `.null`), and the reader keeps it, so it is kept here too.
fn is_code_point(u: Unicode) -> bool {
    u <= 0x10ffff
}
fn parse_cmap_unicodes(cmap: &mut CmapTable, table: Option<&ParsedValue>) {
    let Some(fields) = table.and_then(ParsedValue::as_object) else {
        return;
    };
    for (key, item) in fields {
        let key = &key[..key.len() - 1];
        let Some(unicode) = parse_unicode(key) else {
            tracing::warn!(
                "cmap key \"{}\" is not a code point. This mapping is ignored.",
                ByteStr(key)
            );
            continue;
        };
        let Some(bytes) = item.as_str_bytes() else {
            continue;
        };
        if !is_code_point(unicode) {
            continue;
        }
        let gname: Vec<u8> = bytes.to_vec();
        if !encode_cmap_by_name(cmap, unicode as i32, gname.clone())
            && let Some(current_map) = cmap_lookup(cmap, unicode as i32)
        {
            tracing::warn!(
                "U+{:04X} is already mapped to {}. Assignment to {} is ignored.",
                unicode,
                ByteStr(&current_map.name),
                ByteStr(&gname)
            );
        }
    }
}
// A UVS key is two `parse_unicode` keys separated by the first space; no
// space, or either half not a code point, means it is not a UVS key.
fn parse_uvs_key(uvs_str: &[u8]) -> Option<CmapUvsKey> {
    let pos = uvs_str.iter().position(|&b| b == b' ')?;
    Some(CmapUvsKey {
        unicode: parse_unicode(&uvs_str[..pos])?,
        selector: parse_unicode(&uvs_str[pos + 1..])?,
    })
}
fn parse_cmap_uvs(cmap: &mut CmapTable, table: Option<&ParsedValue>) {
    let Some(fields) = table.and_then(ParsedValue::as_object) else {
        return;
    };
    for (key, item) in fields {
        let key = &key[..key.len() - 1];
        let Some(k) = parse_uvs_key(key) else {
            tracing::warn!(
                "cmap_uvs key \"{}\" is not a pair of code points. This mapping is ignored.",
                ByteStr(key)
            );
            continue;
        };
        let Some(bytes) = item.as_str_bytes() else {
            continue;
        };
        // A variation sequence on U+0000, or with selector 0, means nothing.
        if !(is_code_point(k.unicode) && is_code_point(k.selector))
            || k.unicode == 0
            || k.selector == 0
        {
            continue;
        }
        let gname: Vec<u8> = bytes.to_vec();
        if !encode_cmap_uvs_by_name(cmap, k, gname.clone())
            && let Some(current_map) = cmap_lookup_uvs(cmap, k)
        {
            tracing::warn!(
                "UVS U+{:04X} U+{:04X} is already mapped to {}. Assignment to {} is ignored.",
                k.unicode,
                k.selector,
                ByteStr(&current_map.name),
                ByteStr(&gname)
            );
        }
    }
}
/// Reads the `"cmap"` and `"cmap_uvs"` objects of `root`, and
/// `"cmap_encoding": "symbol"` for a font mapped under Windows Symbol. Keys that are not code
/// points (or, for `"cmap_uvs"`, a pair of code points separated by a space),
/// values that are not strings, and code points outside 0..=0x10FFFF are
/// ignored. When a key repeats, the first assignment wins and a warning is
/// logged. Glyphs are referenced by name, to be resolved against the glyph order
/// later.
pub fn parse_cmap(root: &ParsedValue) -> Option<Box<CmapTable>> {
    root.as_object()?;
    let mut cmap_box: Box<CmapTable> = Box::new(CmapTable {
        unicodes: std::collections::BTreeMap::new(),
        uvs: std::collections::BTreeMap::new(),
        symbol: root
            .get_typed(b"cmap_encoding", JsonType::String)
            .and_then(ParsedValue::as_str_bytes)
            == Some(b"symbol".as_slice()),
    });
    let cmap: &mut CmapTable = cmap_box.as_mut();
    let stage = crate::logger::stage("cmap");
    parse_cmap_unicodes(cmap, root.get_typed(b"cmap", JsonType::Object));
    stage.finish();
    let stage = crate::logger::stage("cmap_uvs");
    parse_cmap_uvs(cmap, root.get_typed(b"cmap_uvs", JsonType::Object));
    stage.finish();
    Some(cmap_box)
}

#[cfg(test)]
mod parse_unicode_tests {
    use super::*;

    #[test]
    fn hex_and_decimal_keys_parse() {
        assert_eq!(parse_unicode(b"U+0041"), Some(0x41));
        assert_eq!(parse_unicode(b"U+10ffff"), Some(0x10FFFF));
        assert_eq!(parse_unicode(b"65"), Some(65));
        assert_eq!(parse_unicode(b"0"), Some(0));
    }

    #[test]
    fn anything_else_is_not_a_code_point() {
        for key in [
            &b"65abc"[..],
            b" 65",
            b"65 ",
            b"+65",
            b"-65",
            b"",
            b"U+",
            b"U+0x41",
            b"U++41",
            b"u+0041",
            b"0x41",
            b"\xc3\xa9",
            b"99999999999",
        ] {
            assert_eq!(
                parse_unicode(key),
                None,
                "{:?}",
                String::from_utf8_lossy(key)
            );
        }
    }

    #[test]
    fn a_mapping_of_u_0000_is_kept() {
        // Fonts map U+0000 to `.null`; the dump writes it, so it is read back.
        let json = otfcc_json::parse_json(br#"{"0": ".null", "65": "A"}"#).unwrap();
        let mut cmap = CmapTable {
            unicodes: std::collections::BTreeMap::new(),
            uvs: std::collections::BTreeMap::new(),
            symbol: false,
        };
        parse_cmap_unicodes(&mut cmap, Some(&json));
        let keys: Vec<i32> = cmap.unicodes.keys().copied().collect();
        assert_eq!(keys, [0, 65]);
    }

    #[test]
    fn a_uvs_key_needs_two_code_points() {
        let k = parse_uvs_key(b"U+845B U+E0100").unwrap();
        assert_eq!((k.unicode, k.selector), (0x845B, 0xE0100));
        assert!(parse_uvs_key(b"U+845B").is_none());
        assert!(parse_uvs_key(b"U+845B x").is_none());
    }
}
