use crate::font::sfnt::Packet;
use crate::support::fmt::{Byte, Dec5, Hex2};
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, TableId};
use otfcc_binary::{FontReader, ReadError};

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
// number of (script, langSys) pairs `read_script_list` will actually
// process, independent of how many any individual `require_room` check
// would otherwise allow -- generous past what any real script/language
// coverage table needs (a script with dozens of language systems is
// already unusual) while stopping the aliasing amplification at a small
// fraction of the CI timeout.
const MAX_TOTAL_LANGUAGES: u32 = 10_000;
// Lookups and their subtables. `read_otl_lookup` reads a `subtableCount`
// (raw `u16`) whose only guard is that its own offset array fits in the
// table, and the LookupList's own `lookupCount` likewise. Nothing stops
// lookup offsets from aliasing one lookup, or subtable offsets from
// aliasing one subtable, so a few KB of table can declare billions of
// subtables. Fuzzing found a lookup with 65,535 declared chaining
// subtables and a table of ~10,300 lookups whose JSON alone was ~217 MB.
//
// Both used to be capped separately: 300 lookups per table and 1,000
// subtables per lookup. Real fonts pass both: Noto Serif Tibetan has 1,197
// GSUB lookups, Noto Sans Duployan 420, Noto Sans Balinese 337 GPOS
// lookups, and one GSUB lookup of Gulzar has 3,483 subtables. What those
// caps bounded was the product, the number of subtables read, so that is
// what is budgeted now, for the whole table (`OtlReadBudget::subtables`),
// and the lookup count is left as the font declares it. The most any of
// those fonts has is 5,840 (Noto Sans Duployan's GSUB), so 100,000 leaves
// a wide margin while staying below the 300,000 the two caps allowed.
pub(crate) const MAX_TOTAL_SUBTABLES_PER_TABLE: u32 = 100_000;
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
// A fourth amplification axis, in `read_feature_list`'s Feature List
// loop (not `parse_language`'s -- a different function, despite the
// similar-sounding name): `featureCount` (raw `u16`, up to 65535) and each
// feature's `lookupIndexCount` (also up to 65535, bounds-checked only
// against that one feature table's own bytes) are read independently. A
// crafted table can point many features at one feature table claiming a
// large `lookupIndexCount`, so the inner-loop reads scale with their
// product, not with the table's real size; CI's fuzz job found a table
// whose Feature List alone took 25+ seconds and over 2048 MB.
//
// That used to be stopped by capping each factor: 500 features per table
// and 300 lookups per feature. Real fonts pass the first: Cormorant has 610
// GSUB features, and `tests/payload/Cormorant-Medium.otf` lost 110 of
// them. Only the product needs a bound, so the lookup indices read are
// budgeted for the whole table (`OtlReadBudget::feature_lookup_refs`) and
// the feature count is left as the font declares it. 150,000 is the
// product the two caps allowed; the most any font measured has is 816
// (Cormorant's GSUB).
pub(crate) const MAX_TOTAL_FEATURE_LOOKUP_REFS_PER_TABLE: u32 = 150_000;

use crate::table::otl::budget::OtlReadBudget;
use crate::table::otl::constants::{SCRIPT_LANGUAGE_SEPARATOR, USE_MARK_FILTERING_SET};
use crate::table::otl::feature_params::read_feature_params;
use crate::table::otl::kind::lookup_kind;
use crate::table::otl::subtables::chaining::read::otl_read_contextual;
use crate::table::otl::subtables::extend::{read_otl_gpos_extend, read_otl_gsub_extend};
use crate::table::otl::{
    Feature, FeatureIdx, FeatureList, LanguageSystem, Lookup, LookupIdx, LookupType,
    OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CONTEXT, OTL_TYPE_GPOS_EXTEND, OTL_TYPE_GPOS_UNKNOWN,
    OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_CONTEXT, OTL_TYPE_GSUB_EXTEND, OTL_TYPE_GSUB_UNKNOWN,
    OTL_TYPE_UNKNOWN, OtlTable, Subtable,
};
use crate::table::otl::{new_feature, new_language, new_lookup};
/// Reads one subtable of a lookup of type `lookup_type`. Context subtables
/// are read straight into the chaining representation, and extension
/// subtables are unwrapped into the subtable they point at, so neither type
/// is left once reading is done; every other type is read by its
/// [`LookupKind`](crate::table::otl::kind::LookupKind).
pub fn read_otl_subtable(
    data: &[u8],
    subtable_offset: u32,
    lookup_type: LookupType,
    max_glyphs: GlyphId,
    options: &Options,
    budget: &mut OtlReadBudget,
) -> Option<Box<Subtable>> {
    match lookup_type {
        OTL_TYPE_GSUB_CONTEXT | OTL_TYPE_GPOS_CONTEXT => {
            otl_read_contextual(data, subtable_offset, max_glyphs, budget).map(Box::new)
        }
        OTL_TYPE_GSUB_EXTEND => {
            read_otl_gsub_extend(data, subtable_offset, max_glyphs, options, budget).map(Box::new)
        }
        OTL_TYPE_GPOS_EXTEND => {
            read_otl_gpos_extend(data, subtable_offset, max_glyphs, options, budget).map(Box::new)
        }
        _ => lookup_kind(lookup_type)?
            .read_subtable(data, subtable_offset, max_glyphs, budget)
            .map(Box::new),
    }
}
// Reads a LangSys. Its `featureIndex` array is checked against the table
// along with the header; on failure, this one language system is left with
// no features rather than failing the whole table.
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
                lang.required_feature = Some(FeatureIdx(u32::from(rid)));
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
                    lang.features.push(FeatureIdx(u32::from(feature_index)));
                }
            }
        }
        Err(_) => {
            lang.features.clear();
            lang.required_feature = None;
        }
    }
}
// Reads the script, feature and lookup lists. Any failed check, at any
// depth, discards the whole table. Every array is checked against the table
// before it is read, `langSysRecords` included.
fn parse_otl_common(
    data: &[u8],
    lookup_type_base: LookupType,
    options: &Options,
    budget: &mut OtlReadBudget,
) -> Result<Box<OtlTable>, ReadError> {
    let mut table_box: Box<OtlTable> = Box::new(OtlTable {
        lookups: Vec::new(),
        features: Vec::new(),
        languages: Vec::new(),
    });

    let script_list_offset = u32::from(FontReader::new(data).at(4)?.u16()?);
    let feature_list_offset = u32::from(FontReader::new(data).at(6)?.u16()?);
    let lookup_list_offset = u32::from(FontReader::new(data).at(8)?.u16()?);

    read_lookup_list(data, lookup_list_offset, lookup_type_base, &mut table_box)?;
    read_feature_list(data, feature_list_offset, &mut table_box, options, budget)?;
    read_script_list(data, script_list_offset, &mut table_box)?;
    name_unnamed_lookups(&mut table_box, options);
    Ok(table_box)
}
/// Reads the LookupList: each lookup's offset and type. The subtables are
/// read later, by `read_otl_lookup`.
fn read_lookup_list(
    data: &[u8],
    lookup_list_offset: u32,
    lookup_type_base: LookupType,
    table: &mut OtlTable,
) -> Result<(), ReadError> {
    let mut lr = FontReader::new(data).at(lookup_list_offset as usize)?;
    let lookup_count = lr.u16()?;
    lr.require_room(lookup_count as usize, 2)?;
    for _ in 0..lookup_count {
        let mut lookup: Box<Lookup> = new_lookup();
        let lookup_offset = lookup_list_offset.wrapping_add(u32::from(lr.u16()?));
        // Needs 6 bytes at `lookup_offset` (lookupType/lookupFlag/
        // subtableCount): only the first 2 are read here, but all 6 must be
        // present before the lookup is pushed, as otfcc required.
        let mut hr = FontReader::new(data).at(lookup_offset as usize)?;
        hr.require_room(6, 1)?;
        lookup._offset = lookup_offset;
        lookup.lookup_type = LookupType::from_file(lookup_type_base, hr.u16()?);
        table.lookups.push(Some(lookup));
    }
    Ok(())
}
/// Reads the FeatureList, naming each feature after its tag and index, and
/// naming each lookup after the first feature that references it.
fn read_feature_list(
    data: &[u8],
    feature_list_offset: u32,
    table: &mut OtlTable,
    options: &Options,
    budget: &mut OtlReadBudget,
) -> Result<(), ReadError> {
    let mut fr = FontReader::new(data).at(feature_list_offset as usize)?;
    let feature_count = fr.u16()?;
    fr.require_room(feature_count as usize, 6)?;
    let mut lnk: TableId = 0;
    for j in 0..feature_count {
        let tag = fr.u32()?;
        let feature_offset = feature_list_offset.wrapping_add(u32::from(fr.u16()?));
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
                Dec5(i32::from(j)),
            );
        } else {
            feature.name = crate::bytesbuild!(
                Byte((tag >> 24 & 0xff) as u8),
                Byte((tag >> 16 & 0xff) as u8),
                Byte((tag >> 8 & 0xff) as u8),
                Byte((tag & 0xff) as u8),
                b"_",
                Dec5(i32::from(j)),
            );
        }
        let mut fer = FontReader::new(data).at(feature_offset as usize)?;
        let params_offset = fer.u16()?;
        if params_offset != 0 {
            feature.params = read_feature_params(
                data,
                feature_offset as usize + usize::from(params_offset),
                &tag.to_be_bytes(),
                budget,
            );
        }
        let feature_lookup_count = fer.u16()?;
        fer.require_room(feature_lookup_count as usize, 2)?;
        for _ in 0..feature_lookup_count {
            // See `MAX_TOTAL_FEATURE_LOOKUP_REFS_PER_TABLE`.
            if !budget.take_feature_lookup_ref() {
                break;
            }
            let lookupid = fer.u16()?;
            if (lookupid as usize) < table.lookups.len() {
                // Every slot is `Some` at this point in construction --
                // holes only ever appear later, via consolidation.
                let feature_lookup = table.lookups[lookupid as usize]
                    .as_mut()
                    .expect("freshly read lookup slot should not be empty");
                if feature_lookup.name.is_empty() {
                    if let Some(prefix) = &options.glyph_name_prefix {
                        feature_lookup.name = crate::bytesbuild!(
                            b"lookup_",
                            prefix,
                            b"_",
                            Byte((tag >> 24 & 0xff) as u8),
                            Byte((tag >> 16 & 0xff) as u8),
                            Byte((tag >> 8 & 0xff) as u8),
                            Byte((tag & 0xff) as u8),
                            b"_",
                            i32::from(lnk),
                        );
                        lnk = lnk.wrapping_add(1);
                    } else {
                        feature_lookup.name = crate::bytesbuild!(
                            b"lookup_",
                            Byte((tag >> 24 & 0xff) as u8),
                            Byte((tag >> 16 & 0xff) as u8),
                            Byte((tag >> 8 & 0xff) as u8),
                            Byte((tag & 0xff) as u8),
                            b"_",
                            i32::from(lnk),
                        );
                        lnk = lnk.wrapping_add(1);
                    }
                }
                // A borrowed cross-reference into `table.lookups`, not
                // an owned value -- `table.lookups` is already fully
                // built, in final order, before any feature is parsed, so
                // `lookupid` already *is* the final `LookupIdx`, no remap
                // needed (unlike the JSON-parse path, see
                // `table/otl/parse.rs`).
                feature.lookups.push(LookupIdx(u32::from(lookupid)));
            }
        }
        table.features.push(Some(feature));
    }
    Ok(())
}
/// Reads the ScriptList: one `LanguageSystem` per script's default language
/// and per `LangSysRecord`, up to `MAX_TOTAL_LANGUAGES` in total.
fn read_script_list(
    data: &[u8],
    script_list_offset: u32,
    table: &mut OtlTable,
) -> Result<(), ReadError> {
    let mut sr = FontReader::new(data).at(script_list_offset as usize)?;
    let script_count = sr.u16()?;
    sr.require_room(script_count as usize, 6)?;
    let mut total_languages: u32 = 0;
    let mut total_feature_refs: u32 = MAX_TOTAL_FEATURE_REFS_PER_TABLE;
    'scripts: for _ in 0..script_count {
        let script_tag = sr.u32()?;
        let script_offset = script_list_offset.wrapping_add(u32::from(sr.u16()?));
        let mut so = FontReader::new(data).at(script_offset as usize)?;
        let default_lang_offset = so.u16()?;
        let lang_sys_count = so.u16()?;
        if default_lang_offset != 0 {
            if total_languages >= MAX_TOTAL_LANGUAGES {
                break 'scripts;
            }
            total_languages += 1;
            let mut lang: Box<LanguageSystem> = new_language();
            lang.name = crate::bytesbuild!(
                Byte((script_tag >> 24 & 0xff) as u8),
                Byte((script_tag >> 16 & 0xff) as u8),
                Byte((script_tag >> 8 & 0xff) as u8),
                Byte((script_tag & 0xff) as u8),
                Byte(SCRIPT_LANGUAGE_SEPARATOR),
                b"DFLT",
            );
            parse_language(
                data,
                script_offset.wrapping_add(u32::from(default_lang_offset)),
                &mut lang,
                &table.features,
                &mut total_feature_refs,
            );
            table.languages.push(lang);
        }
        // `lang_sys_count` comes from the file: check the whole
        // `langSysRecords[]` array fits before reading it.
        so.require_room(lang_sys_count as usize, 6)?;
        for _ in 0..lang_sys_count {
            let lang_tag = so.u32()?;
            let lang_sys = so.u16()?;
            if total_languages >= MAX_TOTAL_LANGUAGES {
                break 'scripts;
            }
            total_languages += 1;
            let mut language: Box<LanguageSystem> = new_language();
            language.name = crate::bytesbuild!(
                Byte((script_tag >> 24 & 0xff) as u8),
                Byte((script_tag >> 16 & 0xff) as u8),
                Byte((script_tag >> 8 & 0xff) as u8),
                Byte((script_tag & 0xff) as u8),
                Byte(SCRIPT_LANGUAGE_SEPARATOR),
                Byte((lang_tag >> 24 & 0xff) as u8),
                Byte((lang_tag >> 16 & 0xff) as u8),
                Byte((lang_tag >> 8 & 0xff) as u8),
                Byte((lang_tag & 0xff) as u8),
            );
            parse_language(
                data,
                script_offset.wrapping_add(u32::from(lang_sys)),
                &mut language,
                &table.features,
                &mut total_feature_refs,
            );
            table.languages.push(language);
        }
    }
    if total_languages >= MAX_TOTAL_LANGUAGES {
        tracing::warn!(
            "[otl] Total script/language count exceeded {}; the rest of this table's scripts are ignored.\n",
            MAX_TOTAL_LANGUAGES as i32
        );
    }
    Ok(())
}
/// Names every lookup no feature referenced, after its type and index.
fn name_unnamed_lookups(table: &mut OtlTable, options: &Options) {
    // Every slot is still `Some` here -- holes only ever appear later, via
    // consolidation, well after this function returns.
    for (j_3, lookup) in table.lookups.iter_mut().flatten().enumerate() {
        if lookup.name.is_empty() {
            if let Some(prefix) = &options.glyph_name_prefix {
                lookup.name = crate::bytesbuild!(
                    b"lookup_",
                    prefix,
                    b"_",
                    Hex2(lookup.lookup_type.raw()),
                    b"_",
                    j_3 as i32,
                );
            } else {
                lookup.name = crate::bytesbuild!(
                    b"lookup_",
                    Hex2(lookup.lookup_type.raw()),
                    b"_",
                    j_3 as i32,
                );
            }
        }
    }
}
fn read_otl_lookup(
    data: &[u8],
    lookup: &mut Lookup,
    max_glyphs: GlyphId,
    options: &Options,
    budget: &mut OtlReadBudget,
) {
    let parsed = FontReader::new(data)
        .at(lookup._offset as usize)
        .and_then(|mut r| {
            r.skip(2)?; // lookupType, already resolved into lookup_type
            let flags = r.u16()?;
            let subtable_count = r.u16()?;
            r.require_room(subtable_count as usize, 2)?;
            // See `MAX_TOTAL_SUBTABLES_PER_TABLE`.
            let read_count = budget.take_subtables(subtable_count);
            let mut subtable_offsets = Vec::with_capacity(read_count as usize);
            for _ in 0..read_count {
                subtable_offsets.push(lookup._offset.wrapping_add(u32::from(r.u16()?)));
            }
            if subtable_count == 0 {
                return Err(ReadError {
                    needed: 1,
                    available: 0,
                });
            }
            // The set index follows every subtable offset, read or not.
            let mark_filtering_set = if flags & USE_MARK_FILTERING_SET != 0 {
                r.skip(2 * usize::from(subtable_count - read_count))?;
                Some(r.u16()?)
            } else {
                None
            };
            Ok((
                flags & !USE_MARK_FILTERING_SET,
                mark_filtering_set,
                subtable_offsets,
            ))
        });
    let (flags, mark_filtering_set, subtable_offsets) = match parsed {
        Ok(v) => v,
        Err(_) => {
            lookup.lookup_type = OTL_TYPE_UNKNOWN;
            return;
        }
    };
    lookup.flags = flags;
    lookup.mark_filtering_set = mark_filtering_set;
    for subtable_offset in subtable_offsets {
        let subtable = read_otl_subtable(
            data,
            subtable_offset,
            lookup.lookup_type,
            max_glyphs,
            options,
            budget,
        );
        lookup.subtables.push(subtable);
    }
    if lookup.lookup_type == OTL_TYPE_GSUB_EXTEND || lookup.lookup_type == OTL_TYPE_GPOS_EXTEND {
        lookup.lookup_type = OTL_TYPE_UNKNOWN;
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
                ext.lookup_type
            })
        }) {
            lookup.lookup_type = ext_type;
        }
        if lookup.lookup_type != OTL_TYPE_UNKNOWN {
            for slot in lookup.subtables.iter_mut() {
                // `.take()` moves this slot's element (if any) out and
                // leaves `None` behind.
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
                    let ext_type = ext.lookup_type;
                    let nested = ext.subtable.take();
                    if ext_type == lookup.lookup_type {
                        *slot = nested;
                    } else {
                        // A scratch `Lookup` holding the mismatched subtable,
                        // dropped at the end of this block.
                        let mut temp: Box<Lookup> = new_lookup();
                        temp.lookup_type = ext_type;
                        temp.subtables.push(nested);
                        drop(temp);
                        // Slot already `None` from `.take()` above.
                    }
                }
            }
        } else {
            // The `Extend` placeholders still in `lookup.subtables` drop with
            // the lookup.
            return;
        }
    }
    if lookup.lookup_type == OTL_TYPE_GSUB_CONTEXT {
        lookup.lookup_type = OTL_TYPE_GSUB_CHAINING;
    }
    if lookup.lookup_type == OTL_TYPE_GPOS_CONTEXT {
        lookup.lookup_type = OTL_TYPE_GPOS_CHAINING;
    }
}
pub fn read_otl(
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
    // OTL read failures are not logged, unlike most table readers.
    // One budget for the whole table (GSUB or GPOS), shared by its feature
    // list and by every lookup and subtable read below, so it bounds this
    // table's total cost rather than each part's separately -- see
    // `OtlReadBudget`.
    let mut budget = OtlReadBudget::for_table(if tag == crate::tag::TAG_GSUB {
        "GSUB"
    } else if tag == crate::tag::TAG_GPOS {
        "GPOS"
    } else {
        "OTL"
    });
    let mut otl_box = parse_otl_common(&table.data, lookup_type_base, options, &mut budget).ok()?;
    // Every slot is still `Some` here -- this is the same freshly-built
    // table `parse_otl_common` just returned, before any consolidation.
    for lookup in otl_box.lookups.iter_mut().flatten() {
        read_otl_lookup(&table.data, lookup, max_glyphs, options, &mut budget);
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
    // so `read_otl_subtable` -- unconverted, out of this PR's scope
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
        let otl = parse_otl_common(
            &data,
            OTL_TYPE_GSUB_UNKNOWN,
            &options,
            &mut OtlReadBudget::new(),
        )
        .unwrap();
        assert_eq!(otl.lookups.len(), 1);
        assert_eq!(otl.features.len(), 1);
        assert_eq!(otl.features[0].as_ref().unwrap().name, b"liga_00000"); // Dec5 zero-pads the index
        assert_eq!(otl.features[0].as_ref().unwrap().lookups.len(), 1);
        assert_eq!(otl.languages.len(), 1); // only the non-default langSys; defaultLangSys was 0
    }

    #[test]
    fn lang_sys_records_array_larger_than_declared_is_rejected_instead_of_reading_oob() {
        // `lang_sys_count` is a full u16 from the file, so the
        // `langSysRecords[]` array must be checked against the table.
        // `langSysCount` here claims 2 records (12 bytes needed from the
        // array's start), but the table is truncated right after the one
        // real record's 6 bytes; `require_room` must catch the shortfall.
        let mut data = well_formed_gsub();
        data[44..46].copy_from_slice(&2u16.to_be_bytes()); // langSysCount: claims 2, only 1 present
        data.truncate(52); // cuts off right after the one real langSysRecord
        let options = zeroed_options();
        assert!(
            parse_otl_common(
                &data,
                OTL_TYPE_GSUB_UNKNOWN,
                &options,
                &mut OtlReadBudget::new()
            )
            .is_err()
        );
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
        let otl = parse_otl_common(
            &data,
            OTL_TYPE_GSUB_UNKNOWN,
            &options,
            &mut OtlReadBudget::new(),
        )
        .unwrap();
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
    #[cfg_attr(
        miri,
        ignore = "far too slow to run meaningfully under Miri's interpreter; ~10,100 parse_language calls are needed to exceed the real MAX_TOTAL_LANGUAGES cap"
    )]
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
        let otl = parse_otl_common(
            &data,
            OTL_TYPE_GSUB_UNKNOWN,
            &options,
            &mut OtlReadBudget::new(),
        )
        .unwrap();
        assert_eq!(otl.languages.len(), MAX_TOTAL_LANGUAGES as usize);
    }

    #[test]
    fn read_otl_lookup_reads_subtable_offsets() {
        // A standalone lookup table, independent of `well_formed_gsub`'s
        // layout: lookupType(2)@0 (unused by `read_otl_lookup`
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
        // `read_otl_subtable` (unconverted, out of scope) falls
        // through to its null-return arm -- this test only checks that one
        // subtable slot was appended, not what's in it.
        lookup.lookup_type = OTL_TYPE_GSUB_UNKNOWN;
        read_otl_lookup(&data, &mut lookup, 0, &options, &mut OtlReadBudget::new());
        assert_eq!(lookup.subtables.len(), 1);
    }

    #[test]
    fn subtable_count_zero_marks_the_lookup_unknown() {
        let data = well_formed_gsub(); // subtableCount is already 0
        let options = zeroed_options();
        let mut otl = parse_otl_common(
            &data,
            OTL_TYPE_GSUB_UNKNOWN,
            &options,
            &mut OtlReadBudget::new(),
        )
        .unwrap();
        read_otl_lookup(
            &data,
            otl.lookups[0].as_mut().unwrap(),
            0,
            &options,
            &mut OtlReadBudget::new(),
        );
        assert_eq!(
            otl.lookups[0].as_ref().unwrap().lookup_type,
            OTL_TYPE_UNKNOWN
        );
    }
}

#[cfg(test)]
mod table_wide_limit_tests {
    use super::*;
    use crate::table::otl::budget::Limit;

    /// A GSUB with `lookups` lookups and `features` features, every one of
    /// which lists `lookups_per_feature` lookups. The lookups all point at
    /// one lookup table and the features at one feature table, which the
    /// format allows.
    fn gsub(lookups: u16, features: u16, lookups_per_feature: u16) -> Vec<u8> {
        let mut data = vec![0u8; 10];
        let put = |data: &mut Vec<u8>, v: u16| data.extend_from_slice(&v.to_be_bytes());
        // LookupList at 10: count, offsets, then the one lookup table.
        let lookup_list = data.len();
        put(&mut data, lookups);
        let lookup_table_rel = 2 + 2 * lookups;
        for _ in 0..lookups {
            put(&mut data, lookup_table_rel);
        }
        put(&mut data, 4); // lookupType: ligature
        put(&mut data, 0); // lookupFlag
        put(&mut data, 0); // subTableCount
        // FeatureList: count, records, then the one feature table.
        let feature_list = data.len();
        put(&mut data, features);
        let feature_table_rel = 2 + 6 * features;
        for _ in 0..features {
            data.extend_from_slice(b"liga");
            put(&mut data, feature_table_rel);
        }
        put(&mut data, 0); // featureParamsOffset
        put(&mut data, lookups_per_feature);
        for j in 0..lookups_per_feature {
            put(&mut data, j % lookups.max(1));
        }
        // ScriptList with no scripts.
        let script_list = data.len();
        put(&mut data, 0);
        data[4..6].copy_from_slice(&(script_list as u16).to_be_bytes());
        data[6..8].copy_from_slice(&(feature_list as u16).to_be_bytes());
        data[8..10].copy_from_slice(&(lookup_list as u16).to_be_bytes());
        data
    }

    #[test]
    fn every_lookup_and_feature_is_read_however_many_there_are() {
        // Past the 300 lookups, 500 features and 300 lookups per feature
        // that used to be read; real fonts have more than each.
        for (lookups, features, per_feature) in [(1200, 300, 400), (10, 700, 1)] {
            let data = gsub(lookups, features, per_feature);
            let mut budget = OtlReadBudget::new();
            let otl = parse_otl_common(
                &data,
                OTL_TYPE_GSUB_UNKNOWN,
                &Options::default(),
                &mut budget,
            )
            .unwrap();
            assert_eq!(otl.lookups.len(), lookups as usize);
            let counts: Vec<usize> = otl
                .features
                .iter()
                .flatten()
                .map(|feature| feature.lookups.len())
                .collect();
            assert_eq!(counts, vec![per_feature as usize; features as usize]);
            assert!(!budget.has_reached(Limit::FeatureLookupRefs));
        }
    }

    #[test]
    fn feature_lookup_refs_stop_at_the_table_wide_budget() {
        let data = gsub(10, 4, 10);
        let mut budget = OtlReadBudget {
            feature_lookup_refs: 25,
            ..OtlReadBudget::new()
        };
        let otl = parse_otl_common(
            &data,
            OTL_TYPE_GSUB_UNKNOWN,
            &Options::default(),
            &mut budget,
        )
        .unwrap();
        let counts: Vec<usize> = otl
            .features
            .iter()
            .flatten()
            .map(|feature| feature.lookups.len())
            .collect();
        assert_eq!(counts, [10, 10, 5, 0]);
        assert!(budget.has_reached(Limit::FeatureLookupRefs));
    }

    #[test]
    fn subtables_stop_at_the_table_wide_budget() {
        // Two lookups of three subtables each, with room for four in all.
        let mut data = vec![0u8; 32];
        for (at, subtables) in [(0usize, 3u16), (16, 3)] {
            data[at..at + 2].copy_from_slice(&4u16.to_be_bytes()); // lookupType
            data[at + 4..at + 6].copy_from_slice(&subtables.to_be_bytes());
        }
        let mut budget = OtlReadBudget {
            subtables: 4,
            ..OtlReadBudget::new()
        };
        let mut counts = Vec::new();
        for offset in [0u32, 16] {
            let mut lookup = new_lookup();
            lookup._offset = offset;
            lookup.lookup_type = LookupType::from_file(OTL_TYPE_GSUB_UNKNOWN, 4);
            read_otl_lookup(&data, &mut lookup, 0, &Options::default(), &mut budget);
            counts.push(lookup.subtables.len());
        }
        assert_eq!(counts, [3, 1]);
        assert!(budget.has_reached(Limit::Subtables));
    }
}

#[cfg(test)]
mod mark_filtering_set_tests {
    use super::*;

    /// A lookup with `subtables` subtable offsets, `flags`, and the 16-bit
    /// `trailer` after the offsets.
    fn lookup_bytes(flags: u16, subtables: u16, trailer: u16) -> Vec<u8> {
        let mut data = Vec::new();
        for v in [4, flags, subtables] {
            data.extend_from_slice(&v.to_be_bytes());
        }
        for _ in 0..subtables {
            data.extend_from_slice(&0u16.to_be_bytes());
        }
        data.extend_from_slice(&trailer.to_be_bytes());
        data
    }

    fn read(data: &[u8], budget: &mut OtlReadBudget) -> Box<Lookup> {
        let mut lookup = new_lookup();
        lookup.lookup_type = LookupType::from_file(OTL_TYPE_GSUB_UNKNOWN, 4);
        read_otl_lookup(data, &mut lookup, 0, &Options::default(), budget);
        lookup
    }

    #[test]
    fn the_set_after_the_subtable_offsets_is_read_and_the_flag_bit_cleared() {
        let lookup = read(&lookup_bytes(0x0018, 2, 7), &mut OtlReadBudget::new());
        assert_eq!(lookup.mark_filtering_set, Some(7));
        assert_eq!(lookup.flags, 0x0008); // ignoreMarks stays
    }

    #[test]
    fn without_the_flag_the_trailing_word_is_not_a_set() {
        let lookup = read(&lookup_bytes(0x0008, 2, 7), &mut OtlReadBudget::new());
        assert_eq!(lookup.mark_filtering_set, None);
    }

    #[test]
    fn the_set_is_found_when_not_every_subtable_is_read() {
        let mut budget = OtlReadBudget {
            subtables: 1,
            ..OtlReadBudget::new()
        };
        let lookup = read(&lookup_bytes(0x0010, 3, 9), &mut budget);
        assert_eq!(lookup.subtables.len(), 1);
        assert_eq!(lookup.mark_filtering_set, Some(9));
    }
}
