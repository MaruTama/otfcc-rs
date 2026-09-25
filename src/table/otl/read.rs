use crate::font::caryll_sfnt::Packet;
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, TableId};
use crate::support::fmt::{Byte, Dec5, Hex2};

// A `ScriptList` entry's own `Script` table offset, and a `Script` table's
// own `LangSysRecord` offsets, are ordinary offsets into the shared table
// buffer -- nothing requires them to be distinct or non-overlapping. Nor
// does anything cap `scriptCount`/`langSysCount` against each other:
// `require_room` bounds each individually against the table's real length
// at the position it's read from, but many different (script, langSys)
// pairs can legally alias the *same* bytes elsewhere in the table. A
// small `ScriptList` (a few real bytes) can therefore drive an
// arbitrarily large number of `parse_language` calls, each independently
// bounded but not bounded *in aggregate* -- `cargo fuzz run otf_parse`
// found a mutated GSUB table (real tag/count fields, `parse_language`'s
// own `feature_count` loop dominating a `sample` profile of the hang)
// that took 30+ minutes in CI. `MAX_TOTAL_LANGUAGES` caps the total
// number of (script, langSys) pairs `parse_otl_common` will actually
// process, independent of how many any individual `require_room` check
// would otherwise allow -- generous past what any real script/language
// coverage table needs (a script with dozens of language systems is
// already unusual) while stopping the aliasing amplification at a small
// fraction of the CI timeout.
const MAX_TOTAL_LANGUAGES: u32 = 10_000;
// Same amplification shape one level down: `otfcc_read_otl_lookup` reads a
// `subtable_count` (raw `u16`) whose only guard is that its own
// offset array fits in the table -- true for any large enough table
// regardless of how many subtable offsets it declares, and nothing stops
// those offsets from aliasing each other or from each subtable itself
// being expensive to build (fuzzing found a lookup with 65535 declared
// chaining subtables, several thousand of them independently valid and
// each producing pages of `[Consolidate]` warnings downstream -- see
// `chaining/read.rs`'s own `MAX_TOTAL_RULES_PER_TABLE`/
// `MAX_APPLY_PER_RULE`/`MAX_POSITIONS_PER_RULE` for the amplification
// layers underneath this one). Real lookups have at most a few dozen
// subtables even in large fonts, so this cap is far above legitimate
// usage.
const MAX_TOTAL_SUBTABLES_PER_LOOKUP: u16 = 1_000;
// One level up from `MAX_TOTAL_SUBTABLES_PER_LOOKUP`: `parse_otl_common`'s
// own `LookupList` loop reads `lookup_count` (raw `u16`, up to 65535) with
// only `require_room` guarding that its own offset array fits -- true for
// any large enough table. Every per-lookup cap below this one only bounds
// what *one* lookup costs; nothing bounded how many lookups a table could
// declare. Fuzzing found a table with ~10,300 lookups whose combined JSON
// output alone was ~217MB (and whose combined in-memory footprint, well
// past what `MAX_TOTAL_RULES_PER_TABLE`'s own budget change was, turned
// out not to move the needle on -- lookup *count* itself was the
// remaining uncapped multiplier). Real fonts -- even `tests/payload/
// NotoNastaliqUrdu-Regular.ttf`, deliberately complex, in this repo's own
// golden corpus -- have at most a few hundred lookups per table (175, in
// that font's own GSUB). 300 stays generously above any legitimate use
// (~1.7x that font's own count) while keeping worst-case output size and
// memory bounded -- tightened from an initial, still-too-loose 500 after
// CI's ASan-instrumented fuzzing kept finding OOMs marginally over its
// 2048MB limit even with every cap in this module active at once; see
// `MAX_TOTAL_FEATURE_REFS_PER_TABLE`'s own comment for the sibling cap
// tightened at the same time.
pub(crate) const MAX_TOTAL_LOOKUPS_PER_TABLE: u16 = 300;
// A third, independent amplification axis found in the same investigation:
// `parse_language`'s own `feature_count` (raw `u16`, up to 65535 per
// language) is bounds-checked only against that one `LangSys` table's own
// bytes, and nothing capped it against `MAX_TOTAL_LANGUAGES`'s own budget
// -- up to 10,000 languages, each independently pushing up to 65,535
// feature references, is billions of pushes in theory. In practice this
// was the dominant contributor behind one fuzz-found file whose GSUB JSON
// alone stayed ~213MB regardless of every other cap in this module,
// traced to a single `LangSys`'s `features` array serializing an enormous
// number of (mostly duplicate, aliased) feature-name references. Global
// across the whole table (like `MAX_TOTAL_RULES_PER_TABLE`), not
// per-language, for the same "per-factor caps still let the product
// explode" reason relative to `MAX_TOTAL_LANGUAGES`. Tightened from an
// initial 100,000 to 50,000 -- real usage is tiny (tens, not thousands,
// even in `NotoNastaliqUrdu-Regular.ttf`), so this still leaves generous
// headroom; the tightening came from CI's ASan-instrumented fuzzing still
// finding OOMs marginally over its 2048MB limit with every cap in this
// module active, not from this cap specifically being identified as the
// culprit -- several caps across `otl/read.rs` and `chaining/read.rs`
// were tightened together in that round.
pub(crate) const MAX_TOTAL_FEATURE_REFS_PER_TABLE: u32 = 50_000;
// A fourth amplification axis, in `parse_otl_common`'s own Feature List
// loop (not `parse_language`'s -- a different function, despite the
// similar-sounding name): `feature_count` (raw `u16`, up to 65535) had no
// cap at all, unlike `lookup_count` a few lines above it, and each
// feature's own `lookup_count_0` (also up to 65535, bounds-checked only
// against that one feature table's own bytes) is read independently. A
// crafted table can point many different `feature_count` entries at
// `feature_offset`s that overlap or repeat, each claiming a large
// `lookup_count_0` -- the total inner-loop reads scale with
// `feature_count * lookup_count_0`, not with the table's real size,
// exactly the "per-factor caps still let the product explode" shape
// `MAX_TOTAL_FEATURE_REFS_PER_TABLE` already closed one level up.
// CI's fuzz job found a table whose Feature List alone hung
// `parse_otl_common` for 25+ seconds and pushed RSS over the 2048MB
// limit. Capping each factor independently (this constant on the outer
// loop, `MAX_TOTAL_LOOKUPS_PER_TABLE` -- reused, since a feature cannot
// legitimately reference more distinct lookups than the table itself
// declares -- on the inner one) bounds the worst-case product to
// `MAX_TOTAL_FEATURES_PER_TABLE * MAX_TOTAL_LOOKUPS_PER_TABLE`
// (150,000), without needing a shared cross-call budget the way
// `MAX_TOTAL_FEATURE_REFS_PER_TABLE` did -- unlike languages (up to
// 10,000 per table), a table's own feature count is capped right here,
// in the same loop, not accumulated across many separate calls. Real
// fonts have at most a few dozen features per table (`NotoNastaliqUrdu-
// Regular.ttf`'s own GSUB has well under 100); 500 stays generously
// above any legitimate use.
pub(crate) const MAX_TOTAL_FEATURES_PER_TABLE: u16 = 500;

use crate::table::otl::constants::SCRIPT_LANGUAGE_SEPARATOR;
use crate::table::otl::subtables::chaining::read::{otl_read_chaining, otl_read_contextual};
use crate::table::otl::subtables::extend::{
    otfcc_read_otl_gpos_extend, otfcc_read_otl_gsub_extend,
};
use crate::table::otl::subtables::gpos_cursive::otl_read_gpos_cursive;
use crate::table::otl::subtables::gpos_mark_to_ligature::otl_read_gpos_mark_to_ligature;
use crate::table::otl::subtables::gpos_mark_to_single::otl_read_gpos_mark_to_single;
use crate::table::otl::subtables::gpos_pair::otl_read_gpos_pair;
use crate::table::otl::subtables::gpos_single::otl_read_gpos_single;
use crate::table::otl::subtables::gsub_ligature::otl_read_gsub_ligature;
use crate::table::otl::subtables::gsub_multi::otl_read_gsub_multi;
use crate::table::otl::subtables::gsub_reverse::otl_read_gsub_reverse;
use crate::table::otl::subtables::gsub_single::otl_read_gsub_single;
use crate::table::otl::{
    Feature, FeatureIdx, FeatureList, LanguageSystem, Lookup, LookupIdx, LookupType,
    OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CONTEXT, OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_EXTEND,
    OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK,
    OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GPOS_UNKNOWN, OTL_TYPE_GSUB_ALTERNATE,
    OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_CONTEXT, OTL_TYPE_GSUB_EXTEND, OTL_TYPE_GSUB_LIGATURE,
    OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, OTL_TYPE_GSUB_UNKNOWN,
    OTL_TYPE_UNKNOWN, OtlTable, Subtable,
};
use crate::table::otl::{
    new_feature, new_language, new_lookup, otl_feature_ref_list_dispose,
};
// `data` used to be a raw `FontFilePointer`/`table_length` pair,
// reconstructed into a slice via `from_raw_parts` at the top of every one
// of the flat readers below -- a pure round trip, since the one production
// caller (`otfcc_read_otl_lookup`, below) always held a real `&[u8]` before
// breaking it apart to call in here. The nine flat subtable readers (Stage
// L-3) and, since Stage L-5, the chaining/contextual readers as well now
// take `&[u8]` directly and return `Option<Subtable>`/`Option<Box<Subtable>>`
// via `.map(Box::new)`; only the still-`*mut Subtable`-returning `extend`
// arms (unconverted, out of this PR's scope) reconstruct the raw parts
// locally, right where they're still needed, and adopt their result via
// `subtable_list_slot` -- the same `Box::from_raw` bridge `otfcc_read_otl_
// lookup` used to apply to this whole function's own return value, now
// pushed down to just the arms that still produce a raw pointer.
pub fn otfcc_read_otl_subtable(
    data: &[u8],
    subtable_offset: u32,
    lookup_type: LookupType,
    max_glyphs: GlyphId,
    options: &Options,
) -> Option<Box<Subtable>> {
    match lookup_type {
        OTL_TYPE_GSUB_SINGLE => otl_read_gsub_single(data, subtable_offset, max_glyphs).map(Box::new),
        OTL_TYPE_GSUB_MULTIPLE => otl_read_gsub_multi(data, subtable_offset, max_glyphs).map(Box::new),
        OTL_TYPE_GSUB_ALTERNATE => otl_read_gsub_multi(data, subtable_offset, max_glyphs).map(Box::new),
        OTL_TYPE_GSUB_LIGATURE => {
            otl_read_gsub_ligature(data, subtable_offset, max_glyphs).map(Box::new)
        }
        OTL_TYPE_GSUB_CHAINING => {
            otl_read_chaining(data, subtable_offset, max_glyphs, options).map(Box::new)
        }
        OTL_TYPE_GSUB_REVERSE => {
            otl_read_gsub_reverse(data, subtable_offset, max_glyphs).map(Box::new)
        }
        OTL_TYPE_GPOS_CHAINING => {
            otl_read_chaining(data, subtable_offset, max_glyphs, options).map(Box::new)
        }
        OTL_TYPE_GSUB_CONTEXT => {
            otl_read_contextual(data, subtable_offset, max_glyphs, options).map(Box::new)
        }
        OTL_TYPE_GPOS_CONTEXT => {
            otl_read_contextual(data, subtable_offset, max_glyphs, options).map(Box::new)
        }
        OTL_TYPE_GPOS_SINGLE => otl_read_gpos_single(data, subtable_offset, max_glyphs).map(Box::new),
        OTL_TYPE_GPOS_PAIR => otl_read_gpos_pair(data, subtable_offset, max_glyphs).map(Box::new),
        OTL_TYPE_GPOS_CURSIVE => {
            otl_read_gpos_cursive(data, subtable_offset, max_glyphs).map(Box::new)
        }
        OTL_TYPE_GPOS_MARK_TO_BASE => {
            otl_read_gpos_mark_to_single(data, subtable_offset, max_glyphs).map(Box::new)
        }
        OTL_TYPE_GPOS_MARK_TO_MARK => {
            otl_read_gpos_mark_to_single(data, subtable_offset, max_glyphs).map(Box::new)
        }
        OTL_TYPE_GPOS_MARK_TO_LIGATURE => {
            otl_read_gpos_mark_to_ligature(data, subtable_offset, max_glyphs).map(Box::new)
        }
        OTL_TYPE_GSUB_EXTEND => {
            otfcc_read_otl_gsub_extend(data, subtable_offset, max_glyphs, options).map(Box::new)
        }
        OTL_TYPE_GPOS_EXTEND => {
            otfcc_read_otl_gpos_extend(data, subtable_offset, max_glyphs, options).map(Box::new)
        }
        _ => None,
    }
}
// The original's own guard covered only the 6-byte header
// (`lookupOrder`/`requiredFeatureIndex`/`featureCount`); the
// `featureIndex[]` array that follows had no length check at all before
// the loop that reads `feature_count` (attacker-controlled, up to 65535)
// entries of it -- a real, previously-undocumented unchecked-array read,
// same class as the `langSysRecords` bug in `otfcc_read_otl_common`
// below. `require_room` closes both. A failure at either point falls
// back to the original's own recovery: clear this one language's
// `required_feature`/`features` rather than aborting the whole table
// (`otl_feature_ref_list_dispose` matches the original's cleanup call).
fn parse_language(
    data: &[u8],
    base: u32,
    lang: &mut LanguageSystem,
    features: &FeatureList,
    feature_ref_budget: &mut u32,
) {
    let parsed = FontReader::new(data).at(base as usize).and_then(|mut r| {
        r.skip(2)?; // lookupOrder, unused
        let rid = r.u16()?;
        let feature_count = r.u16()?;
        r.require_room(feature_count as usize, 2)?;
        let mut feature_indices = Vec::with_capacity(feature_count as usize);
        for _ in 0..feature_count {
            feature_indices.push(r.u16()?);
        }
        Ok((rid, feature_indices))
    });
    match parsed {
        Ok((rid, feature_indices)) => {
            // `table_box.features` (the caller's `features` here) is
            // already fully built, in final order, before any language is
            // parsed -- so `rid`/`feature_index` below are already the
            // final `FeatureIdx` values, no remap needed (unlike the
            // JSON-parse path's `figure_out_languages_from_json`, see
            // `table/otl/parse.rs`).
            if (rid as usize) < features.len() {
                lang.required_feature = Some(FeatureIdx(rid as u32));
            } else {
                lang.required_feature = None;
            }
            // See `MAX_TOTAL_FEATURE_REFS_PER_TABLE`'s own doc comment:
            // this budget is shared across every `parse_language` call for
            // the whole table, not reset per language.
            for feature_index in feature_indices {
                if *feature_ref_budget == 0 {
                    break;
                }
                *feature_ref_budget -= 1;
                if (feature_index as usize) < features.len() {
                    lang.features.push(FeatureIdx(feature_index as u32));
                }
            }
        }
        Err(_) => {
            otl_feature_ref_list_dispose(&mut lang.features);
            lang.required_feature = None;
        }
    }
}
// Every guard failure in the original, at any nesting depth, falls
// through to the same `return None;` at the very bottom -- discarding
// `table_box` (lookups/features/languages already pushed included, all
// the way). That single-outcome-on-any-failure shape is exactly what `?`
// propagation on a `Result` gives for free, which is what lets this
// rewrite flatten five levels of nested `if`/`current_block` goto-
// emulation into one function with early returns.
//
// Two real, previously-undocumented bugs fixed along the way (beyond the
// `wrapping_add` overflow-defeats-guard class already fixed in
// `cmap.rs`/`coverage.rs`/`classdef.rs`): the `langSysRecords` array
// (read via `lang_tag`/`lang_sys` below) had *no* length guard at all
// before this rewrite -- `lang_sys_count` is attacker-controlled and
// unbounded, so a script with a large `lang_sys_count` read straight past
// the table. `require_room` before that loop closes it. The other is in
// `parse_language`, see its own comment.
fn parse_otl_common(
    data: &[u8],
    lookup_type_base: LookupType,
    options: &Options,
) -> Result<Box<OtlTable>, ReadError> {
    let mut table_box: Box<OtlTable> = Box::new(OtlTable {
        lookups: Vec::new(),
        features: Vec::new(),
        languages: Vec::new(),
    });

    let script_list_offset = FontReader::new(data).at(4)?.u16()? as u32;
    let feature_list_offset = FontReader::new(data).at(6)?.u16()? as u32;
    let lookup_list_offset = FontReader::new(data).at(8)?.u16()? as u32;

    // -- Lookup list --
    let mut lr = FontReader::new(data).at(lookup_list_offset as usize)?;
    let lookup_count = lr.u16()?;
    lr.require_room(lookup_count as usize, 2)?;
    for _ in 0..lookup_count.min(MAX_TOTAL_LOOKUPS_PER_TABLE) {
        let mut lookup: Box<Lookup> = new_lookup();
        let lookup_offset = lookup_list_offset.wrapping_add(lr.u16()? as u32);
        // Needs 6 bytes at `lookup_offset` (lookupType/lookupFlag/
        // subtableCount): only the first 2 are read here, but the
        // original required all 6 up front, before the lookup was even
        // pushed, so this is checked the same way.
        let mut hr = FontReader::new(data).at(lookup_offset as usize)?;
        hr.require_room(6, 1)?;
        lookup._offset = lookup_offset;
        lookup.type_0 = LookupType::from_file(lookup_type_base, hr.u16()?);
        table_box.lookups.push(Some(lookup));
    }

    // -- Feature list --
    let mut fr = FontReader::new(data).at(feature_list_offset as usize)?;
    let feature_count = fr.u16()?;
    fr.require_room(feature_count as usize, 6)?;
    let mut lnk: TableId = 0;
    for j in 0..feature_count.min(MAX_TOTAL_FEATURES_PER_TABLE) {
        let tag = fr.u32()?;
        let feature_offset = feature_list_offset.wrapping_add(fr.u16()? as u32);
        let mut feature: Box<Feature> = new_feature();
        if let Some(prefix) = &options.glyph_name_prefix {
            feature.name = crate::bytesbuild!(
                Byte((tag >> 24 & 0xff) as u8),
                Byte((tag >> 16 & 0xff) as u8),
                Byte((tag >> 8 & 0xff) as u8),
                Byte((tag & 0xff) as u8),
                b"_",
                prefix,
                b"_",
                Dec5(j as i32),
            );
        } else {
            feature.name = crate::bytesbuild!(
                Byte((tag >> 24 & 0xff) as u8),
                Byte((tag >> 16 & 0xff) as u8),
                Byte((tag >> 8 & 0xff) as u8),
                Byte((tag & 0xff) as u8),
                b"_",
                Dec5(j as i32),
            );
        }
        let mut fer = FontReader::new(data).at(feature_offset as usize)?;
        fer.skip(2)?; // featureParams, unused
        let lookup_count_0 = fer.u16()?;
        fer.require_room(lookup_count_0 as usize, 2)?;
        for _ in 0..lookup_count_0.min(MAX_TOTAL_LOOKUPS_PER_TABLE) {
            let lookupid = fer.u16()?;
            if (lookupid as usize) < table_box.lookups.len() {
                // Every slot is `Some` at this point in construction --
                // holes only ever appear later, via consolidation.
                let lookup_0 = table_box.lookups[lookupid as usize]
                    .as_mut()
                    .expect("freshly read lookup slot should not be empty");
                if lookup_0.name.is_empty() {
                    if let Some(prefix) = &options.glyph_name_prefix {
                        lookup_0.name = crate::bytesbuild!(
                            b"lookup_",
                            prefix,
                            b"_",
                            Byte((tag >> 24 & 0xff) as u8),
                            Byte((tag >> 16 & 0xff) as u8),
                            Byte((tag >> 8 & 0xff) as u8),
                            Byte((tag & 0xff) as u8),
                            b"_",
                            lnk as i32,
                        );
                        lnk = lnk.wrapping_add(1);
                    } else {
                        lookup_0.name = crate::bytesbuild!(
                            b"lookup_",
                            Byte((tag >> 24 & 0xff) as u8),
                            Byte((tag >> 16 & 0xff) as u8),
                            Byte((tag >> 8 & 0xff) as u8),
                            Byte((tag & 0xff) as u8),
                            b"_",
                            lnk as i32,
                        );
                        lnk = lnk.wrapping_add(1);
                    }
                }
                // A borrowed cross-reference into `table_box.lookups`, not
                // an owned value -- `table_box.lookups` is already fully
                // built, in final order, before any feature is parsed, so
                // `lookupid` already *is* the final `LookupIdx`, no remap
                // needed (unlike the JSON-parse path, see
                // `table/otl/parse.rs`).
                feature.lookups.push(LookupIdx(lookupid as u32));
            }
        }
        table_box.features.push(Some(feature));
    }

    // -- Script list --
    let mut sr = FontReader::new(data).at(script_list_offset as usize)?;
    let script_count = sr.u16()?;
    sr.require_room(script_count as usize, 6)?;
    let mut total_languages: u32 = 0;
    let mut total_feature_refs: u32 = MAX_TOTAL_FEATURE_REFS_PER_TABLE;
    'scripts: for _ in 0..script_count {
        let tag_0 = sr.u32()?;
        let script_offset_0 = script_list_offset.wrapping_add(sr.u16()? as u32);
        let mut so = FontReader::new(data).at(script_offset_0 as usize)?;
        let default_lang_system_0 = so.u16()?;
        let lang_sys_count = so.u16()?;
        if default_lang_system_0 != 0 {
            if total_languages >= MAX_TOTAL_LANGUAGES {
                break 'scripts;
            }
            total_languages += 1;
            let mut lang: Box<LanguageSystem> = new_language();
            lang.name = crate::bytesbuild!(
                Byte((tag_0 >> 24 & 0xff) as u8),
                Byte((tag_0 >> 16 & 0xff) as u8),
                Byte((tag_0 >> 8 & 0xff) as u8),
                Byte((tag_0 & 0xff) as u8),
                Byte(SCRIPT_LANGUAGE_SEPARATOR as u8),
                b"DFLT",
            );
            parse_language(
                data,
                script_offset_0.wrapping_add(default_lang_system_0 as u32),
                &mut lang,
                &table_box.features,
                &mut total_feature_refs,
            );
            table_box.languages.push(lang);
        }
        // `langSysRecords[]` -- see this function's top comment: the
        // original read `lang_sys_count` (attacker-controlled) entries of
        // this array with no length check at all.
        so.require_room(lang_sys_count as usize, 6)?;
        for _ in 0..lang_sys_count {
            let lang_tag = so.u32()?;
            let lang_sys = so.u16()?;
            if total_languages >= MAX_TOTAL_LANGUAGES {
                break 'scripts;
            }
            total_languages += 1;
            let mut lang_0: Box<LanguageSystem> = new_language();
            lang_0.name = crate::bytesbuild!(
                Byte((tag_0 >> 24 & 0xff) as u8),
                Byte((tag_0 >> 16 & 0xff) as u8),
                Byte((tag_0 >> 8 & 0xff) as u8),
                Byte((tag_0 & 0xff) as u8),
                Byte(SCRIPT_LANGUAGE_SEPARATOR as u8),
                Byte((lang_tag >> 24 & 0xff) as u8),
                Byte((lang_tag >> 16 & 0xff) as u8),
                Byte((lang_tag >> 8 & 0xff) as u8),
                Byte((lang_tag & 0xff) as u8),
            );
            parse_language(
                data,
                script_offset_0.wrapping_add(lang_sys as u32),
                &mut lang_0,
                &table_box.features,
                &mut total_feature_refs,
            );
            table_box.languages.push(lang_0);
        }
    }
    if total_languages >= MAX_TOTAL_LANGUAGES {
        logger_log_sds(
            &mut options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"[otl] Total script/language count exceeded ",
                MAX_TOTAL_LANGUAGES as i32,
                b"; the rest of this table's scripts are ignored.\n",
            ),
        );
    }

    // Every slot is still `Some` here -- holes only ever appear later, via
    // consolidation, well after this function returns.
    for (j_3, lookup) in table_box.lookups.iter_mut().flatten().enumerate() {
        if lookup.name.is_empty() {
            if let Some(prefix) = &options.glyph_name_prefix {
                lookup.name = crate::bytesbuild!(
                    b"lookup_",
                    prefix,
                    b"_",
                    Hex2(lookup.type_0.raw()),
                    b"_",
                    j_3 as i32,
                );
            } else {
                lookup.name = crate::bytesbuild!(
                    b"lookup_",
                    Hex2(lookup.type_0.raw()),
                    b"_",
                    j_3 as i32,
                );
            }
        }
    }
    Ok(table_box)
}
fn otfcc_read_otl_lookup(data: &[u8], lookup: &mut Lookup, max_glyphs: GlyphId, options: &Options) {
    let parsed = FontReader::new(data)
        .at(lookup._offset as usize)
        .and_then(|mut r| {
            r.skip(2)?; // lookupType, already resolved into type_0
            let flags = r.u16()?;
            let subtable_count = r.u16()?;
            r.require_room(subtable_count as usize, 2)?;
            let capped_count = subtable_count.min(MAX_TOTAL_SUBTABLES_PER_LOOKUP);
            let mut subtable_offsets = Vec::with_capacity(capped_count as usize);
            for _ in 0..capped_count {
                subtable_offsets.push(lookup._offset.wrapping_add(r.u16()? as u32));
            }
            if subtable_count == 0 {
                return Err(ReadError {
                    needed: 1,
                    available: 0,
                });
            }
            Ok((flags, subtable_offsets))
        });
    let (flags, subtable_offsets) = match parsed {
        Ok(v) => v,
        Err(_) => {
            lookup.type_0 = OTL_TYPE_UNKNOWN;
            return;
        }
    };
    lookup.flags = flags;
    for subtable_offset in subtable_offsets {
        let subtable =
            otfcc_read_otl_subtable(data, subtable_offset, lookup.type_0, max_glyphs, options);
        lookup.subtables.push(subtable);
    }
    if lookup.type_0 == OTL_TYPE_GSUB_EXTEND || lookup.type_0 == OTL_TYPE_GPOS_EXTEND {
        lookup.type_0 = OTL_TYPE_UNKNOWN;
        // First `Some` slot (holes only appear via later consolidation,
        // but this dispatch runs right after the read above, so a linear
        // search rather than assuming slot 0 is still correct) decides
        // the lookup's real type; every slot is a known `Extend`
        // placeholder here.
        if let Some(ext_type) = lookup.subtables.iter().find_map(|slot| {
            slot.as_ref().map(|elem| {
                let Subtable::Extend(ext) = elem.as_ref() else {
                    unreachable!()
                };
                ext.type_0
            })
        }) {
            lookup.type_0 = ext_type;
        }
        if lookup.type_0 != OTL_TYPE_UNKNOWN {
            for slot in lookup.subtables.iter_mut() {
                // `.take()` both reads this slot's element (if any) and
                // leaves `None` behind -- the direct replacement for the old
                // "copy the raw pointer out, then separately null the slot"
                // two-step, and the only correct one: a `Box` can't be
                // copied, only moved.
                if let Some(mut elem) = slot.take() {
                    // Every element in this list is known to be an `Extend`
                    // placeholder -- that is what `OTL_TYPE_GSUB_EXTEND`/
                    // `OTL_TYPE_GPOS_EXTEND` means -- so unwrapping it is
                    // infallible. Matched through a `&mut` reference, not
                    // moved by value: `Subtable` has a manual `Drop` impl,
                    // so Rust forbids moving a field out of an owned value
                    // of that type (E0509) even when, as here, the field is
                    // that variant's entire payload -- `ext.subtable.take()`
                    // extracts ownership of the nested `Option<Box<Subtable>>`
                    // in place instead, leaving `elem` holding an empty
                    // `Extend` shell that drops trivially (its `subtable`
                    // is `None`) once this block ends.
                    let Subtable::Extend(ext) = &mut *elem else {
                        unreachable!()
                    };
                    let ext_type = ext.type_0;
                    let nested = ext.subtable.take();
                    if ext_type == lookup.type_0 {
                        *slot = nested;
                    } else {
                        // A scratch `Lookup` purely to reuse its (now `Drop`-driven)
                        // type-dispatched subtable teardown on this one subtable --
                        // never pushed anywhere, so it's just let go out of scope
                        // instead of the old explicit `otfcc_delete_lookup` call.
                        let mut temp: Box<Lookup> = new_lookup();
                        temp.type_0 = ext_type;
                        temp.subtables.push(nested);
                        drop(temp);
                        // Slot already `None` from `.take()` above.
                    }
                }
            }
        } else {
            // Was `otl_subtable_list_dispose_dependent(..); return;` -- with
            // `SubtableList` now `Vec<Option<Box<Subtable>>>`, there is
            // nothing left to eagerly dispose: whatever remains in
            // `lookup.subtables` (still holding valid, un-expanded `Extend`
            // placeholders) tears down correctly whenever `lookup` itself
            // eventually drops, since `Subtable::drop` dispatches off each
            // element's own enum tag, not `lookup.type_0` -- which this
            // function already overwrote to `OTL_TYPE_UNKNOWN` above, before
            // B-1 this would have been the wrong type to free by.
            return;
        }
    }
    if lookup.type_0 == OTL_TYPE_GSUB_CONTEXT {
        lookup.type_0 = OTL_TYPE_GSUB_CHAINING;
    }
    if lookup.type_0 == OTL_TYPE_GPOS_CONTEXT {
        lookup.type_0 = OTL_TYPE_GPOS_CHAINING;
    }
}
pub fn otfcc_read_otl(
    packet: &Packet,
    options: &Options,
    tag: u32,
    max_glyphs: GlyphId,
) -> Option<Box<OtlTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == tag)?;
    let lookup_type_base = if tag == crate::tag::TAG_GSUB {
        OTL_TYPE_GSUB_UNKNOWN
    } else if tag == crate::tag::TAG_GPOS {
        OTL_TYPE_GPOS_UNKNOWN
    } else {
        OTL_TYPE_UNKNOWN
    };
    // No "corrupted" log on failure here, matching the original: OTL
    // parse failures are silent (unlike most other table readers).
    let mut otl_box = parse_otl_common(&table.data, lookup_type_base, options).ok()?;
    // See `chaining::read::reset_class_coverage_budgets`'s own doc comment:
    // this must run once per table (GSUB or GPOS), before any of this
    // table's lookups are read, so the budget bounds this whole table's
    // total `class_coverage` cost rather than resetting fresh per subtable.
    crate::table::otl::subtables::chaining::read::reset_class_coverage_budgets();
    crate::table::otl::coverage::reset_coverage_entry_build_budget();
    // Every slot is still `Some` here -- this is the same freshly-built
    // table `parse_otl_common` just returned, before any consolidation.
    for lookup in otl_box.lookups.iter_mut().flatten() {
        otfcc_read_otl_lookup(&table.data, lookup, max_glyphs, options);
    }
    Some(otl_box)
}

#[cfg(test)]
mod parse_otl_common_tests {
    use super::*;

    fn zeroed_options() -> Options {
        Options::default()
    }

    // A minimal but complete GSUB-shaped table: one lookup (0 subtables,
    // so `otfcc_read_otl_subtable` -- unconverted, out of this PR's scope
    // -- is never reached), one feature referencing it, one script whose
    // single langSysRecord (not the default) references the feature.
    //
    // Layout (byte offsets): version 0..4, scriptListOffset(u16) @4,
    // featureListOffset(u16) @6, lookupListOffset(u16) @8;
    // lookupList @10 (count=1, entry@12); lookup table @14 (type=4,
    // flag=0, subtableCount=0); featureList @20 (count=1, tag='liga'
    // @22, offset@26); feature table @28 (featureParams unused,
    // lookupCount=1, lookupIndices=[0]); scriptList @34 (count=1,
    // tag='latn' @36, offset@40); script table @42 (defaultLangSys=0,
    // langSysCount=1, langSysRecord: tag @46, offset(rel. to script
    // table)=10 @50); langSys table @52 (lookupOrder unused,
    // requiredFeatureIndex=0xFFFF, featureCount=1, featureIndices=[0]).
    fn well_formed_gsub() -> Vec<u8> {
        let mut data = vec![0u8; 60];
        data[4..6].copy_from_slice(&34u16.to_be_bytes()); // scriptListOffset
        data[6..8].copy_from_slice(&20u16.to_be_bytes()); // featureListOffset
        data[8..10].copy_from_slice(&10u16.to_be_bytes()); // lookupListOffset

        data[10..12].copy_from_slice(&1u16.to_be_bytes()); // lookupCount
        data[12..14].copy_from_slice(&4u16.to_be_bytes()); // lookup[0] offset (rel. to 10) -> 14
        data[14..16].copy_from_slice(&4u16.to_be_bytes()); // lookupType
        data[18..20].copy_from_slice(&0u16.to_be_bytes()); // subtableCount

        data[20..22].copy_from_slice(&1u16.to_be_bytes()); // featureCount
        data[22..26].copy_from_slice(b"liga"); // featureTag
        data[26..28].copy_from_slice(&8u16.to_be_bytes()); // feature[0] offset (rel. to 20) -> 28
        data[30..32].copy_from_slice(&1u16.to_be_bytes()); // feature.lookupCount
        data[32..34].copy_from_slice(&0u16.to_be_bytes()); // feature.lookupIndices[0]

        data[34..36].copy_from_slice(&1u16.to_be_bytes()); // scriptCount
        data[36..40].copy_from_slice(b"latn"); // scriptTag
        data[40..42].copy_from_slice(&8u16.to_be_bytes()); // script[0] offset (rel. to 34) -> 42
        data[42..44].copy_from_slice(&0u16.to_be_bytes()); // defaultLangSys (none)
        data[44..46].copy_from_slice(&1u16.to_be_bytes()); // langSysCount
        data[46..50].copy_from_slice(b"dflt"); // langSysRecord.tag
        data[50..52].copy_from_slice(&10u16.to_be_bytes()); // langSysRecord offset (rel. to 42) -> 52
        data[54..56].copy_from_slice(&0xFFFFu16.to_be_bytes()); // requiredFeatureIndex (none)
        data[56..58].copy_from_slice(&1u16.to_be_bytes()); // langSys.featureCount
        data[58..60].copy_from_slice(&0u16.to_be_bytes()); // langSys.featureIndices[0]
        data
    }

    #[test]
    fn well_formed_table_links_lookup_feature_and_language() {
        let data = well_formed_gsub();
        let options = zeroed_options();
        let otl = parse_otl_common(&data, OTL_TYPE_GSUB_UNKNOWN, &options).unwrap();
        assert_eq!(otl.lookups.len(), 1);
        assert_eq!(otl.features.len(), 1);
        assert_eq!(otl.features[0].as_ref().unwrap().name, b"liga_00000"); // Dec5 zero-pads the index
        assert_eq!(otl.features[0].as_ref().unwrap().lookups.len(), 1);
        assert_eq!(otl.languages.len(), 1); // only the non-default langSys; defaultLangSys was 0
    }

    #[test]
    fn lang_sys_records_array_larger_than_declared_is_rejected_instead_of_reading_oob() {
        // The original had *no* length check on `langSysRecords[]` at
        // all -- `lang_sys_count` is a full attacker-controlled u16, and
        // the original read that many 6-byte records unconditionally.
        // `langSysCount` here claims 2 records (12 bytes needed from the
        // array's start), but the table is truncated right after the one
        // real record's 6 bytes -- confirming the new `require_room`
        // guard catches the shortfall rather than reading into whatever
        // (if anything) follows in memory.
        let mut data = well_formed_gsub();
        data[44..46].copy_from_slice(&2u16.to_be_bytes()); // langSysCount: claims 2, only 1 present
        data.truncate(52); // cuts off right after the one real langSysRecord
        let options = zeroed_options();
        assert!(parse_otl_common(&data, OTL_TYPE_GSUB_UNKNOWN, &options).is_err());
    }

    #[test]
    fn feature_index_array_larger_than_declared_falls_back_per_language_not_the_whole_table() {
        // `parse_language`'s own missing guard (see its comment): unlike
        // the langSysRecords bug above, a failure here is recoverable --
        // just this one language's features/required_feature are
        // cleared, the rest of the table still parses.
        let mut data = well_formed_gsub();
        data[56..58].copy_from_slice(&5u16.to_be_bytes()); // langSys.featureCount: claims 5, only 1 present
        let options = zeroed_options();
        let otl = parse_otl_common(&data, OTL_TYPE_GSUB_UNKNOWN, &options).unwrap();
        assert_eq!(otl.languages.len(), 1);
        assert!(otl.languages[0].features.is_empty());
        assert!(otl.languages[0].required_feature.is_none());
    }

    #[test]
    // `N` has to genuinely exceed the real `MAX_TOTAL_LANGUAGES` (10,000)
    // for this test to mean anything -- unlike the CffStack-sized Miri
    // slowdowns elsewhere in this crate, there's no smaller-but-
    // equivalent version of "prove the production cap value is actually
    // enforced." Running ~10,100 parse_language calls through Miri's
    // interpreter (125s, this crate's single slowest test) doesn't add
    // meaningfully more UB-detection confidence than the much smaller
    // parse_language tests elsewhere in this module already provide for
    // the same code; the native `cargo test` run stays the actual
    // regression guard for the cap value itself.
    #[cfg_attr(miri, ignore = "far too slow to run meaningfully under Miri's interpreter; ~10,100 parse_language calls are needed to exceed the real MAX_TOTAL_LANGUAGES cap")]
    fn total_language_count_across_the_whole_table_is_capped() {
        // One script whose own `langSysCount` alone (`MAX_TOTAL_LANGUAGES`
        // + 100) exceeds the budget -- every `langSysRecord`'s offset
        // aliases the *same* tiny (0-feature) LangSys table, so this
        // isn't testing how much distinct data exists in the table, only
        // how many times `parse_language` actually runs against it. This
        // is exactly the shape `cargo fuzz` found: a small `ScriptList`
        // (well-formed on its own) driving an unbounded number of cheap-
        // looking-individually-but-not-in-aggregate `parse_language`
        // calls via offset aliasing. Confirms `languages.len()` stops
        // growing at the budget instead of processing all of them.
        const N: u32 = MAX_TOTAL_LANGUAGES + 100;
        let mut data = Vec::new();
        data.extend_from_slice(&[0u8; 4]); // version
        data.extend_from_slice(&16u16.to_be_bytes()); // scriptListOffset
        data.extend_from_slice(&12u16.to_be_bytes()); // featureListOffset
        data.extend_from_slice(&10u16.to_be_bytes()); // lookupListOffset
        data.extend_from_slice(&0u16.to_be_bytes()); // LookupList @10, count=0
        data.extend_from_slice(&0u16.to_be_bytes()); // FeatureList @12, count=0
        data.extend_from_slice(&[0u8; 2]); // padding up to 16
        data.extend_from_slice(&1u16.to_be_bytes()); // ScriptList @16, scriptCount=1
        data.extend_from_slice(b"latn"); // scriptTag
        data.extend_from_slice(&8u16.to_be_bytes()); // script offset (rel to 16) -> 24
        // Script table @24
        data.extend_from_slice(&0u16.to_be_bytes()); // defaultLangSys = 0 (none)
        data.extend_from_slice(&(N as u16).to_be_bytes()); // langSysCount
        let shared_lang_sys_rel = 4u16 + 6 * (N as u16); // rel. to 24
        for i in 0..N {
            data.extend_from_slice(&i.to_be_bytes()); // lang_tag (arbitrary)
            data.extend_from_slice(&shared_lang_sys_rel.to_be_bytes());
        }
        // The one shared LangSys table every record above points at.
        data.extend_from_slice(&0u16.to_be_bytes()); // lookupOrder, unused
        data.extend_from_slice(&0xFFFFu16.to_be_bytes()); // requiredFeatureIndex (none)
        data.extend_from_slice(&0u16.to_be_bytes()); // featureCount = 0

        let options = zeroed_options();
        let otl = parse_otl_common(&data, OTL_TYPE_GSUB_UNKNOWN, &options).unwrap();
        assert_eq!(otl.languages.len(), MAX_TOTAL_LANGUAGES as usize);
    }

    #[test]
    fn otfcc_read_otl_lookup_reads_subtable_offsets() {
        // A standalone lookup table, independent of `well_formed_gsub`'s
        // layout: lookupType(2)@0 (unused by `otfcc_read_otl_lookup`
        // itself -- already resolved by `parse_otl_common`),
        // lookupFlag(2)@2, subtableCount(2)@4=1, subtableOffsets[0](2)@6
        // (relative to the lookup's own offset, 0 here).
        let mut data = vec![0u8; 8];
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // subtableCount
        data[6..8].copy_from_slice(&2u16.to_be_bytes()); // subtableOffsets[0] -> 2 (unused by any real reader here)
        let options = zeroed_options();
        let mut lookup = new_lookup();
        lookup._offset = 0;
        // Not GSUB_EXTEND/GPOS_EXTEND, so the extend-unwrap branch below is
        // skipped; not a real per-format type either, so
        // `otfcc_read_otl_subtable` (unconverted, out of scope) falls
        // through to its null-return arm -- this test only checks that one
        // subtable slot was appended, not what's in it.
        lookup.type_0 = OTL_TYPE_GSUB_UNKNOWN;
        otfcc_read_otl_lookup(&data, &mut lookup, 0, &options);
        assert_eq!(lookup.subtables.len(), 1);
    }

    #[test]
    fn subtable_count_zero_marks_the_lookup_unknown() {
        let data = well_formed_gsub(); // subtableCount is already 0
        let options = zeroed_options();
        let mut otl = parse_otl_common(&data, OTL_TYPE_GSUB_UNKNOWN, &options).unwrap();
        otfcc_read_otl_lookup(&data, otl.lookups[0].as_mut().unwrap(), 0, &options);
        assert_eq!(otl.lookups[0].as_ref().unwrap().type_0, OTL_TYPE_UNKNOWN);
    }
}
