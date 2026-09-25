#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see RUST_MIGRATION.md

use crate::logger::{LOG_VL_NOTICE, LoggerType, logger_log_sds};
use crate::support::parsed_json::ParsedValue;

use crate::font::caryll_font::{Font, FontSubtype};
use crate::support::glyph_order::{GlyphOrder, GlyphOrderEntry, GlyphOrderPass};
use crate::support::options::Options;
use crate::support::primitives::GlyphId;
use crate::vendor::json::JsonType;

use crate::table::_tsi::otfcc_parse_tsi;
use crate::table::base::otfcc_parse_base;
use crate::table::cff::otfcc_parse_cff;
use crate::table::cmap::otfcc_parse_cmap;
use crate::table::colr::otfcc_parse_colr;
use crate::table::cpal::otfcc_parse_cpal;
use crate::table::cvt::otfcc_parse_cvt;
use crate::table::fpgm_prep::otfcc_parse_fpgm_prep;
use crate::table::gasp::otfcc_parse_gasp;
use crate::table::gdef::otfcc_parse_gdef;
use crate::table::glyf::otfcc_parse_glyf;
use crate::table::head::otfcc_parse_head;
use crate::table::hhea::otfcc_parse_hhea;
use crate::table::maxp::otfcc_parse_maxp;
use crate::table::meta::parse::otfcc_parse_meta;
use crate::table::name::otfcc_parse_name;
use crate::table::os_2::otfcc_parse_os_2;
use crate::table::otl::parse::otfcc_parse_otl;
use crate::table::post::otfcc_parse_post;
use crate::table::svg::otfcc_parse_svg;
use crate::table::tsi5::otfcc_parse_tsi5;
use crate::table::vdmx::funcs::otfcc_parse_vdmx;
use crate::table::vhea::otfcc_parse_vhea;

fn otfcc_decide_font_subtype_from_json(root: &ParsedValue) -> FontSubtype {
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
        let unicode = crate::table::cmap::parse_unicode(&key[..key.len() - 1]);
        if let Some(bytes) = item.as_str_bytes() {
            if unicode > 0 && unicode <= 0x10ffff {
                let gname: Vec<u8> = bytes.to_vec();
                escalate_glyph_order_by_name(go, &gname, GlyphOrderPass::Cmap, unicode as u32);
            }
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
    // `consolidate.rs`'s `otfcc_consolidate_font`. `go` borrows `go_box` for
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
                logger_log_sds(
                    &mut options.logger.borrow_mut(),
                    LOG_VL_NOTICE,
                    LoggerType::Info,
                    crate::bytesbuild!(b"OpenType SVG table detected. Glyph order is preserved.",),
                );
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
pub unsafe fn read_json(root: &ParsedValue, options: &Options) -> Option<Box<Font>> {
    let mut font: Box<Font> = Box::default();
    font.subtype = otfcc_decide_font_subtype_from_json(root);
    font.glyph_order = parse_glyph_order(root, options);
    font.glyf = otfcc_parse_glyf(root, font.glyph_order.as_deref(), options);
    font.cff = otfcc_parse_cff(root, options);
    font.head = otfcc_parse_head(root, options);
    font.hhea = otfcc_parse_hhea(root, options);
    font.os_2 = otfcc_parse_os_2(root, options);
    font.maxp = otfcc_parse_maxp(root, options);
    font.post = otfcc_parse_post(root, options);
    font.name = otfcc_parse_name(root, options);
    font.meta = otfcc_parse_meta(root, options);
    font.cmap = otfcc_parse_cmap(root, options);
    if !options.ignore_hints {
        font.fpgm = otfcc_parse_fpgm_prep(
            root,
            options,
            b"fpgm",
        );
        font.prep = otfcc_parse_fpgm_prep(
            root,
            options,
            b"prep",
        );
        font.cvt_ = otfcc_parse_cvt(
            root,
            options,
            b"cvt_",
        );
        font.gasp = otfcc_parse_gasp(root, options);
    }
    font.vdmx = otfcc_parse_vdmx(root, options);
    font.vhea = otfcc_parse_vhea(root, options);
    if font.glyf.is_some() {
        font.gsub = otfcc_parse_otl(
            root,
            options,
            b"GSUB",
        );
        font.gpos = otfcc_parse_otl(
            root,
            options,
            b"GPOS",
        );
        font.gdef = otfcc_parse_gdef(root, options);
    }
    font.base = otfcc_parse_base(root, options);
    font.cpal = otfcc_parse_cpal(root, options);
    font.colr = otfcc_parse_colr(root, options);
    font.svg = otfcc_parse_svg(root, options);
    font.tsi_01 = otfcc_parse_tsi(
        root,
        options,
        b"TSI_01",
    );
    font.tsi_23 = otfcc_parse_tsi(
        root,
        options,
        b"TSI_23",
    );
    font.tsi5 = otfcc_parse_tsi5(root);
    Some(font)
}
