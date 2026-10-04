#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see RUST_MIGRATION.md

use crate::logger::ByteStr;
use crate::support::json_limits::{MAX_ENTRIES, find_oversized_collection};
use otfcc_json::ParsedValue;

use crate::font::model::{Font, FontSubtype};
use crate::font::table_registry::PARSE_ORDER;
use crate::support::glyph_order::{GlyphOrder, GlyphOrderEntry, GlyphOrderPass};
use crate::support::options::Options;
use crate::support::primitives::GlyphId;
use otfcc_json::JsonType;

fn decide_font_subtype_from_json(root: &ParsedValue) -> FontSubtype {
    if root.get_typed(b"CFF_", JsonType::Object).is_some() {
        FontSubtype::Cff
    } else {
        FontSubtype::Ttf
    }
}
// `name` is `Vec<u8>` now instead of `SdsRaw`: the duplicate-name path
// used to leave the old `SdsRaw` `name` deliberately un-freed (a
// pre-existing leak this migration didn't own until it reached this
// function directly), but that hazard is gone by construction -- an
// unused `Vec<u8>` just drops.
//
// Never a real FFI boundary -- internal call site only, same rationale
// as every other instance of this allow in the crate.
fn set_order_by_name(go: &mut GlyphOrder, name: Vec<u8>, order_type: GlyphOrderPass, order_entry: u32) {
    match go.by_name.get(&name).copied() {
        None => {
            go.entries.push(GlyphOrderEntry {
                gid: -1_i32 as GlyphId,
                name: name.clone(),
                order_type,
                order_entry,
            });
            let idx = go.entries.len() - 1;
            go.by_name.insert(name, idx);
        }
        Some(idx) => {
            let entry = &mut go.entries[idx];
            if entry.order_type > order_type {
                entry.order_type = order_type;
                entry.order_entry = order_entry;
            }
        }
    }
}
fn order_glyphs(go: &mut GlyphOrder) {
    let mut idxs: Vec<usize> = go.by_name.values().copied().collect();
    idxs.sort_by(|&a, &b| {
        let ea = &go.entries[a];
        let eb = &go.entries[b];
        (ea.order_type, ea.order_entry).cmp(&(eb.order_type, eb.order_entry))
    });
    let mut gid: GlyphId = 0 as GlyphId;
    for &idx in idxs.iter() {
        go.entries[idx].gid = gid;
        go.by_gid.insert(gid, idx);
        gid = (gid as i32 + 1_i32) as GlyphId;
    }
}
// `name` is a borrowed `&[u8]` now, not `SdsRaw` -- this function only
// ever reads it for the `by_name` lookup, never stores it, so there was
// never any ownership to plumb through in the first place.
//
// Never a real FFI boundary -- internal call sites only, same rationale
// as every other instance of this allow in the crate.
fn escalate_glyph_order_by_name(
    go: &mut GlyphOrder,
    name: &[u8],
    order_type: GlyphOrderPass,
    order_entry: u32,
) {
    if let Some(&idx) = go.by_name.get(name) {
        let entry = &mut go.entries[idx];
        if entry.order_type > order_type {
            entry.order_type = order_type;
            entry.order_entry = order_entry;
        }
    }
}
fn place_order_entries_from_glyf(table: &ParsedValue, go: &mut GlyphOrder) {
    let Some(fields) = table.as_object() else {
        return;
    };
    for (j, (key, _)) in fields.iter().enumerate() {
        let gname: Vec<u8> = key[..key.len() - 1].to_vec();
        let j = j as u32;
        if gname.as_slice() == b".notdef" {
            set_order_by_name(go, gname, GlyphOrderPass::Notdef, 0_u32);
        } else if gname.as_slice() == b".null" {
            set_order_by_name(go, gname, GlyphOrderPass::Notdef, 1_u32);
        } else {
            set_order_by_name(go, gname, GlyphOrderPass::Glyf, j);
        }
    }
}
// `strlen`/pointer arithmetic on `key.as_ptr()`: a separate, not-yet-
// converted raw-C-string shell -- same shape as `table/cmap.rs`'s
// `parse_unicode` (this function inlines the identical U+XXXX-or-decimal
// parse and stays unsafe for the same reason), out of scope here.
// The `U+XXXX`-or-decimal object-key parse this used to inline byte for
// byte (`strlen`/`strtol`/`.offset()` over the key's raw storage) is
// `table/cmap.rs`'s own `parse_unicode` -- the old comment here said so and
// then duplicated it anyway. Calling it directly is the whole function's
// unsafety gone, and leaves one parser to keep correct instead of two.
fn place_order_entries_from_cmap(table: &ParsedValue, go: &mut GlyphOrder) {
    let Some(fields) = table.as_object() else {
        return;
    };
    for (key, item) in fields {
        // A key that is not a code point is warned about once, when the
        // cmap table itself is read; here it is just skipped.
        let Some(unicode) = crate::table::cmap::parse_unicode(&key[..key.len() - 1]) else {
            continue;
        };
        if let Some(bytes) = item.as_str_bytes()
            && unicode > 0 && unicode <= 0x10ffff {
                let gname: Vec<u8> = bytes.to_vec();
                escalate_glyph_order_by_name(go, &gname, GlyphOrderPass::Cmap, unicode);
            }
    }
}
fn place_order_entries_from_subtable(table: &ParsedValue, go: &mut GlyphOrder, zero_only: bool) {
    let Some(items) = table.as_array() else {
        return;
    };
    let uplimit = if zero_only { items.len().min(1) } else { items.len() };
    for (j, item) in items.iter().enumerate().take(uplimit) {
        if let Some(bytes) = item.as_str_bytes() {
            let gname: Vec<u8> = bytes.to_vec();
            escalate_glyph_order_by_name(go, &gname, GlyphOrderPass::GlyphOrder, j as u32);
        }
    }
}
fn parse_glyph_order(root: &ParsedValue, options: &Options) -> Option<Box<GlyphOrder>> {
    // Built directly via `Box::new`, not `OTFCC_PKG_GLYPH_ORDER.create`
    // (`malloc`) + `Box::from_raw` -- see the matching note in
    // `consolidate.rs`'s `consolidate_font`. `go` borrows `go_box` for
    // the rest of this function (unchanged from here down).
    let mut go_box: Box<GlyphOrder> = Box::new(GlyphOrder {
        entries: Vec::new(),
        by_gid: ::std::collections::BTreeMap::new(),
        by_name: ::std::collections::HashMap::new(),
    });
    let go: &mut GlyphOrder = go_box.as_mut();
    if root.as_object().is_none() {
        return Some(go_box);
    }
    if let Some(table) = root.get_typed(b"glyf", JsonType::Object) {
        place_order_entries_from_glyf(table, go);
        if let Some(table) = root.get_typed(b"cmap", JsonType::Object) {
            // place_order_entries_from_cmap is a separate, not-yet-converted
            // raw-C-string shell -- out of scope here, so this is a narrow
            // unsafe {} rather than the whole function, the same way
            // vf/vq.rs's vqs_compare bridges to vq_compare_region.
            place_order_entries_from_cmap(table, go);
        }
        if let Some(table) = root.get_typed(b"glyph_order", JsonType::Array) {
            let mut ignore_glyph_order: bool = options.ignore_glyph_order;
            if ignore_glyph_order && root.get_typed(b"SVG_", JsonType::Array).is_some() {
                tracing::info!("OpenType SVG table detected. Glyph order is preserved.");
                ignore_glyph_order = false;
            }
            place_order_entries_from_subtable(table, go, ignore_glyph_order);
        }
    }
    order_glyphs(go);
    return Some(go_box);
}
/// Builds a font from an already-parsed JSON tree.
///
/// Was a `FontBuilder` impl on a zero-sized `JsonReader` marker struct
/// plus a casting wrapper; see `otf_reader::read_otf` for why that trait
/// is gone. The subfont index the erased signature forced this side to
/// accept was never read -- a JSON tree describes exactly one font --
/// so it is dropped rather than kept as a silently-ignored parameter.
///
/// `root` is `&mut ParsedValue`, not `&ParsedValue`. An earlier revision of
/// this function kept `root: &ParsedValue` (`read_json` was `unsafe fn`)
/// and reborrowed it into a `&mut ParsedValue` at each of the three call
/// sites that needed one (`parse_glyf`, then `parse_otl` twice
/// for GSUB/GPOS) via an explicit `as *mut` cast, on the reasoning that
/// "nothing else reads `root` during this call" was enough to make it
/// sound. **That reasoning is wrong, and Miri caught it on the very next
/// CI run**: a `&T`-typed reference's own tag caps every pointer derived
/// from it at `SharedReadOnly` for that borrow's whole lifetime under
/// Stacked Borrows, independent of what else does or doesn't read through
/// it -- casting it to `*mut` and dereferencing mutably is undefined
/// behavior unconditionally, not something "nothing else aliases it" can
/// excuse away. Every one of this function's three real call sites already
/// owns its `ParsedValue` as a mutable local that is never read again
/// afterward (`ffi/dll.rs`, `bin/otfccbuild.rs`, `benches/support/mod.rs`),
/// so taking `&mut ParsedValue` here costs nothing at any of them, and
/// every call this function makes to `parse_glyf`/`parse_otl`
/// (both `&mut ParsedValue` themselves, Stage M-32/M-33) is now a plain,
/// ordinary, sound reborrow -- no raw pointer and no `unsafe` anywhere in
/// this function, closing the JSON-parse `unsafe fn` trio this migration's
/// "Stage 7-4 plan" set out to make safe (M-31 through this, its own
/// planned M-34).
pub fn read_json(root: &mut ParsedValue, options: &Options) -> Option<Box<Font>> {
    // Counts in an OpenType table are 16 bits, so a JSON collection with
    // 65,536 or more members cannot become a font -- and used to make
    // `otfccbuild` hang (65,536 `glyf` entries, 65,536 references on one
    // glyph), panic (65,536 mark classes) or write a table whose count had
    // wrapped. See `support::json_limits` for the rule and its exceptions.
    // Reject it here, before any glyph-order or table work, the way any other
    // JSON that cannot become a font is rejected: `None` is what `otfccbuild`
    // reports as "Cannot parse JSON file ... as a font" and what
    // `otfccbuild_json_otf` turns into a null buffer.
    if let Some(found) = find_oversized_collection(root) {
        tracing::error!("Too many entries in \"{}\": {} (at most {} are supported; counts in an OpenType table are 16-bit).\n", ByteStr(&found.path), found.len as u32, MAX_ENTRIES as u32);
        return None;
    }
    let mut font: Box<Font> = Box::default();
    font.subtype = decide_font_subtype_from_json(root);
    font.glyph_order = parse_glyph_order(root, options);
    for table in PARSE_ORDER {
        table.parse(&mut font, root, options);
    }
    Some(font)
}

#[cfg(test)]
mod glyph_count_limit_tests {
    use super::*;
    use otfcc_json::parse_json;

    /// `{"glyf":{"g0":{},"g1":{},...}}` with `count` empty glyph objects.
    fn font_json_with_glyph_count(count: usize) -> Vec<u8> {
        let mut json = String::from("{\"glyf\":{");
        for i in 0..count {
            if i != 0 {
                json.push(',');
            }
            json.push_str(&format!("\"g{i}\":{{}}"));
        }
        json.push_str("}}");
        json.into_bytes()
    }

    fn read(count: usize) -> Option<Box<Font>> {
        let mut root = parse_json(&font_json_with_glyph_count(count)).expect("test JSON parses");
        read_json(&mut root, &Options::default())
    }

    #[test]
    #[cfg_attr(miri, ignore = "far too slow to run meaningfully under Miri's interpreter; needs a genuine 65,536-entry glyf object to hit the limit")]
    fn glyf_object_past_the_16_bit_glyph_limit_is_rejected_instead_of_hanging() {
        // Before the limit existed, `read_json` accepted this input and
        // `otfccbuild` then spun forever in `consolidate_glyf` (a `u16`
        // counter that can never reach a length of 65,536). The hang is a
        // stage later than what this test can reach, so what it pins is the
        // rejection that now happens first.
        assert!(read(MAX_ENTRIES + 1).is_none());
    }

    #[test]
    #[cfg_attr(miri, ignore = "far too slow to run meaningfully under Miri's interpreter; needs a genuine 65,535-entry glyf object to sit exactly on the limit")]
    fn glyf_object_exactly_at_the_16_bit_glyph_limit_is_accepted() {
        // Pins the off-by-one: 65,535 glyphs is what `maxp.numGlyphs` (a
        // `u16`) can hold, and what the `FDArrayTest65535.otf` fixture
        // already exercises through the binary reader, so it must still load.
        let font = read(MAX_ENTRIES).expect("65,535 glyphs is within the limit");
        assert_eq!(font.glyf.as_ref().map(|g| g.len()), Some(MAX_ENTRIES));
    }

    #[test]
    fn a_font_with_no_glyf_object_is_not_affected_by_the_limit() {
        let mut root = parse_json(b"{}").expect("test JSON parses");
        assert!(read_json(&mut root, &Options::default()).is_some());
    }
}

#[cfg(test)]
mod layout_collection_limit_tests {
    use super::*;
    use otfcc_json::parse_json;

    fn read(json: &str) -> Option<Box<Font>> {
        let mut root = parse_json(json.as_bytes()).expect("test JSON parses");
        read_json(&mut root, &Options::default())
    }

    // Two layout-table shapes that used to get past `read_json` and fail
    // later: a ligature with 65,536 components truncated its 16-bit count while
    // keeping every anchor, and 65,536 distinct mark classes wrapped the class
    // count to 0 and panicked indexing by class id. Both are now stopped by the
    // one shape rule in `support::json_limits`, before any table parser runs.
    #[test]
    #[cfg_attr(miri, ignore = "needs a genuine 65,536-element array")]
    fn a_mark_to_ligature_with_65536_components_is_rejected() {
        let components = vec!["{}"; MAX_ENTRIES + 1].join(",");
        let json = format!(
            r#"{{"GPOS":{{"lookups":{{"l":{{"type":"gpos_mark_to_ligature","subtables":[{{"marks":{{}},"bases":{{"A":[{components}]}}}}]}}}}}}}}"#
        );
        assert!(read(&json).is_none());
    }

    #[test]
    #[cfg_attr(miri, ignore = "needs a genuine 65,536-entry marks object")]
    fn a_mark_subtable_with_65536_mark_classes_is_rejected() {
        let marks: Vec<String> = (0..=MAX_ENTRIES)
            .map(|i| format!(r#""g{i}":{{"class":"c{i}","x":0,"y":0}}"#))
            .collect();
        let json = format!(
            r#"{{"GPOS":{{"lookups":{{"l":{{"type":"gpos_mark_to_base","subtables":[{{"marks":{{{}}},"bases":{{}}}}]}}}}}}}}"#,
            marks.join(",")
        );
        assert!(read(&json).is_none());
    }
}

