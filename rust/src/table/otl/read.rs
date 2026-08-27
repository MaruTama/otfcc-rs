#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::font::caryll_sfnt::Packet;
use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};
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
    Feature, FeatureList, FeatureRef, LanguageSystem, Lookup, LookupRef, LookupType,
    OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CONTEXT, OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_EXTEND,
    OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK,
    OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GPOS_UNKNOWN, OTL_TYPE_GSUB_ALTERNATE,
    OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_CONTEXT, OTL_TYPE_GSUB_EXTEND, OTL_TYPE_GSUB_LIGATURE,
    OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, OTL_TYPE_GSUB_UNKNOWN,
    OTL_TYPE_UNKNOWN, OtlTable, Subtable,
};
use crate::table::otl::{
    new_feature, new_language, new_lookup, otl_feature_ref_list_dispose, subtable_list_slot,
};
pub unsafe fn otfcc_read_otl_subtable(
    data: FontFilePointer,
    table_length: u32,
    subtable_offset: u32,
    lookup_type: LookupType,
    max_glyphs: GlyphId,
    options: &Options,
) -> *mut Subtable {
    match lookup_type {
        OTL_TYPE_GSUB_SINGLE => {
            return otl_read_gsub_single(data, table_length, subtable_offset, max_glyphs);
        }
        OTL_TYPE_GSUB_MULTIPLE => {
            return otl_read_gsub_multi(data, table_length, subtable_offset, max_glyphs);
        }
        OTL_TYPE_GSUB_ALTERNATE => {
            return otl_read_gsub_multi(data, table_length, subtable_offset, max_glyphs);
        }
        OTL_TYPE_GSUB_LIGATURE => {
            return otl_read_gsub_ligature(data, table_length, subtable_offset, max_glyphs);
        }
        OTL_TYPE_GSUB_CHAINING => {
            return otl_read_chaining(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GSUB_REVERSE => {
            return otl_read_gsub_reverse(data, table_length, subtable_offset, max_glyphs);
        }
        OTL_TYPE_GPOS_CHAINING => {
            return otl_read_chaining(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GSUB_CONTEXT => {
            return otl_read_contextual(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GPOS_CONTEXT => {
            return otl_read_contextual(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GPOS_SINGLE => {
            return otl_read_gpos_single(data, table_length, subtable_offset, max_glyphs);
        }
        OTL_TYPE_GPOS_PAIR => {
            return otl_read_gpos_pair(data, table_length, subtable_offset, max_glyphs);
        }
        OTL_TYPE_GPOS_CURSIVE => {
            return otl_read_gpos_cursive(data, table_length, subtable_offset, max_glyphs);
        }
        OTL_TYPE_GPOS_MARK_TO_BASE => {
            return otl_read_gpos_mark_to_single(data, table_length, subtable_offset, max_glyphs);
        }
        OTL_TYPE_GPOS_MARK_TO_MARK => {
            return otl_read_gpos_mark_to_single(data, table_length, subtable_offset, max_glyphs);
        }
        OTL_TYPE_GPOS_MARK_TO_LIGATURE => {
            return otl_read_gpos_mark_to_ligature(data, table_length, subtable_offset, max_glyphs);
        }
        OTL_TYPE_GSUB_EXTEND => {
            return otfcc_read_otl_gsub_extend(
                data,
                table_length,
                subtable_offset,
                max_glyphs,
                options,
            );
        }
        OTL_TYPE_GPOS_EXTEND => {
            return otfcc_read_otl_gpos_extend(
                data,
                table_length,
                subtable_offset,
                max_glyphs,
                options,
            );
        }
        _ => return ::core::ptr::null_mut::<Subtable>(),
    };
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
unsafe fn parse_language(
    data: &[u8],
    base: u32,
    lang: *mut LanguageSystem,
    features: *mut FeatureList,
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
            if (rid as usize) < (*features).len() {
                (*lang).required_feature = &raw const *(&(*features))[rid as usize] as FeatureRef;
            } else {
                (*lang).required_feature = ::core::ptr::null::<Feature>();
            }
            for feature_index in feature_indices {
                if (feature_index as usize) < (*features).len() {
                    (*lang)
                        .features
                        .push(&raw const *(&(*features))[feature_index as usize] as FeatureRef);
                }
            }
        }
        Err(_) => {
            otl_feature_ref_list_dispose(&raw mut (*lang).features);
            (*lang).required_feature = ::core::ptr::null::<Feature>();
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
unsafe fn parse_otl_common(
    data: &[u8],
    lookup_type_base: LookupType,
    options: &Options,
) -> Result<Box<OtlTable>, ReadError> {
    let mut table_box: Box<OtlTable> = Box::new(OtlTable {
        lookups: Vec::new(),
        features: Vec::new(),
        languages: Vec::new(),
    });
    let table: *mut OtlTable = table_box.as_mut() as *mut OtlTable;

    let script_list_offset = FontReader::new(data).at(4)?.u16()? as u32;
    let feature_list_offset = FontReader::new(data).at(6)?.u16()? as u32;
    let lookup_list_offset = FontReader::new(data).at(8)?.u16()? as u32;

    // -- Lookup list --
    let mut lr = FontReader::new(data).at(lookup_list_offset as usize)?;
    let lookup_count = lr.u16()?;
    lr.require_room(lookup_count as usize, 2)?;
    for _ in 0..lookup_count {
        let mut lookup: Box<Lookup> = new_lookup();
        let lookup_offset = lookup_list_offset.wrapping_add(lr.u16()? as u32);
        // Needs 6 bytes at `lookup_offset` (lookupType/lookupFlag/
        // subtableCount): only the first 2 are read here, but the
        // original required all 6 up front, before the lookup was even
        // pushed, so this is checked the same way.
        let mut hr = FontReader::new(data).at(lookup_offset as usize)?;
        hr.require_room(6, 1)?;
        (*lookup)._offset = lookup_offset;
        (*lookup).type_0 = LookupType::from_file(lookup_type_base, hr.u16()?);
        (*table).lookups.push(lookup);
    }

    // -- Feature list --
    let mut fr = FontReader::new(data).at(feature_list_offset as usize)?;
    let feature_count = fr.u16()?;
    fr.require_room(feature_count as usize, 6)?;
    let mut lnk: TableId = 0;
    for j in 0..feature_count {
        let tag = fr.u32()?;
        let feature_offset = feature_list_offset.wrapping_add(fr.u16()? as u32);
        let mut feature: Box<Feature> = new_feature();
        if !options.glyph_name_prefix.is_null() {
            (*feature).name = crate::bytesbuild!(
                Byte((tag >> 24 & 0xff) as u8),
                Byte((tag >> 16 & 0xff) as u8),
                Byte((tag >> 8 & 0xff) as u8),
                Byte((tag & 0xff) as u8),
                b"_",
                options.glyph_name_prefix,
                b"_",
                Dec5(j as ::core::ffi::c_int),
            );
        } else {
            (*feature).name = crate::bytesbuild!(
                Byte((tag >> 24 & 0xff) as u8),
                Byte((tag >> 16 & 0xff) as u8),
                Byte((tag >> 8 & 0xff) as u8),
                Byte((tag & 0xff) as u8),
                b"_",
                Dec5(j as ::core::ffi::c_int),
            );
        }
        let mut fer = FontReader::new(data).at(feature_offset as usize)?;
        fer.skip(2)?; // featureParams, unused
        let lookup_count_0 = fer.u16()?;
        fer.require_room(lookup_count_0 as usize, 2)?;
        for _ in 0..lookup_count_0 {
            let lookupid = fer.u16()?;
            if (lookupid as usize) < (*table).lookups.len() {
                let lookup_0: *mut Lookup = &raw mut *(&mut (*table).lookups)[lookupid as usize];
                if (*lookup_0).name.is_empty() {
                    if !options.glyph_name_prefix.is_null() {
                        let fresh3 = lnk;
                        lnk = lnk.wrapping_add(1);
                        (*lookup_0).name = crate::bytesbuild!(
                            b"lookup_",
                            options.glyph_name_prefix,
                            b"_",
                            Byte((tag >> 24 & 0xff) as u8),
                            Byte((tag >> 16 & 0xff) as u8),
                            Byte((tag >> 8 & 0xff) as u8),
                            Byte((tag & 0xff) as u8),
                            b"_",
                            fresh3 as ::core::ffi::c_int,
                        );
                    } else {
                        let fresh4 = lnk;
                        lnk = lnk.wrapping_add(1);
                        (*lookup_0).name = crate::bytesbuild!(
                            b"lookup_",
                            Byte((tag >> 24 & 0xff) as u8),
                            Byte((tag >> 16 & 0xff) as u8),
                            Byte((tag >> 8 & 0xff) as u8),
                            Byte((tag & 0xff) as u8),
                            b"_",
                            fresh4 as ::core::ffi::c_int,
                        );
                    }
                }
                (*feature).lookups.push(lookup_0 as LookupRef);
            }
        }
        (*table).features.push(feature);
    }

    // -- Script list --
    let mut sr = FontReader::new(data).at(script_list_offset as usize)?;
    let script_count = sr.u16()?;
    sr.require_room(script_count as usize, 6)?;
    let mut total_languages: u32 = 0;
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
            (*lang).name = crate::bytesbuild!(
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
                &raw mut *lang,
                &raw mut (*table).features,
            );
            (*table).languages.push(lang);
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
            (*lang_0).name = crate::bytesbuild!(
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
                &raw mut *lang_0,
                &raw mut (*table).features,
            );
            (*table).languages.push(lang_0);
        }
    }
    if total_languages >= MAX_TOTAL_LANGUAGES {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"[otl] Total script/language count exceeded ",
                MAX_TOTAL_LANGUAGES as ::core::ffi::c_int,
                b"; the rest of this table's scripts are ignored.\n",
            ),
        );
    }

    for j_3 in 0..(*table).lookups.len() {
        if (*(&(*table).lookups)[j_3]).name.is_empty() {
            if !options.glyph_name_prefix.is_null() {
                (*(&mut (*table).lookups)[j_3]).name = crate::bytesbuild!(
                    b"lookup_",
                    options.glyph_name_prefix,
                    b"_",
                    Hex2((*(&(*table).lookups)[j_3]).type_0.raw()),
                    b"_",
                    j_3 as ::core::ffi::c_int,
                );
            } else {
                (*(&mut (*table).lookups)[j_3]).name = crate::bytesbuild!(
                    b"lookup_",
                    Hex2((*(&(*table).lookups)[j_3]).type_0.raw()),
                    b"_",
                    j_3 as ::core::ffi::c_int,
                );
            }
        }
    }
    Ok(table_box)
}
unsafe fn otfcc_read_otl_lookup(
    data: &[u8],
    lookup: *mut Lookup,
    max_glyphs: GlyphId,
    options: &Options,
) {
    let parsed = FontReader::new(data)
        .at((*lookup)._offset as usize)
        .and_then(|mut r| {
            r.skip(2)?; // lookupType, already resolved into type_0
            let flags = r.u16()?;
            let subtable_count = r.u16()?;
            r.require_room(subtable_count as usize, 2)?;
            let capped_count = subtable_count.min(MAX_TOTAL_SUBTABLES_PER_LOOKUP);
            let mut subtable_offsets = Vec::with_capacity(capped_count as usize);
            for _ in 0..capped_count {
                subtable_offsets.push((*lookup)._offset.wrapping_add(r.u16()? as u32));
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
            (*lookup).type_0 = OTL_TYPE_UNKNOWN;
            return;
        }
    };
    (*lookup).flags = flags;
    // `otfcc_read_otl_subtable` and everything below it (`subtables/*`)
    // still takes a raw pointer/length pair -- not yet converted to
    // `FontReader`. `data`/`table.data.len()` is the same
    // pointer/length pair the original passed, unchanged.
    let raw_data = data.as_ptr() as FontFilePointer;
    let table_length = data.len() as u32;
    for subtable_offset in subtable_offsets {
        let subtable: *mut Subtable = otfcc_read_otl_subtable(
            raw_data,
            table_length,
            subtable_offset,
            (*lookup).type_0,
            max_glyphs,
            options,
        );
        (*lookup).subtables.push(subtable_list_slot(subtable));
    }
    if (*lookup).type_0 == OTL_TYPE_GSUB_EXTEND || (*lookup).type_0 == OTL_TYPE_GPOS_EXTEND {
        (*lookup).type_0 = OTL_TYPE_UNKNOWN;
        let mut j_0: TableId = 0 as TableId;
        while (j_0 as usize) < (*lookup).subtables.len() {
            if let Some(elem) = &(&(*lookup).subtables)[j_0 as usize] {
                let Subtable::Extend(ext) = elem.as_ref() else {
                    unreachable!()
                };
                (*lookup).type_0 = ext.type_0;
                break;
            } else {
                j_0 = j_0.wrapping_add(1);
            }
        }
        if (*lookup).type_0 != OTL_TYPE_UNKNOWN {
            let mut j_1: TableId = 0 as TableId;
            while (j_1 as usize) < (*lookup).subtables.len() {
                // `.take()` both reads this slot's element (if any) and
                // leaves `None` behind -- the direct replacement for the old
                // "copy the raw pointer out, then separately null the slot"
                // two-step, and the only correct one: a `Box` can't be
                // copied, only moved.
                if let Some(elem) = (&mut (*lookup).subtables)[j_1 as usize].take() {
                    // Every element in this list is known to be an `Extend`
                    // placeholder -- that is what `OTL_TYPE_GSUB_EXTEND`/
                    // `OTL_TYPE_GPOS_EXTEND` means -- so unwrapping it is
                    // infallible. Moving `ExtendSubtable` out of `*elem`
                    // (it's `Copy`) also deallocates `elem`'s own heap slot,
                    // same as the old explicit `Box::from_raw(..)` drop did.
                    let Subtable::Extend(ext) = *elem else {
                        unreachable!()
                    };
                    if ext.type_0 == (*lookup).type_0 {
                        // `.subtable`'s ownership transfers to become the new
                        // list element.
                        (&mut (*lookup).subtables)[j_1 as usize] = subtable_list_slot(ext.subtable);
                    } else {
                        // A scratch `Lookup` purely to reuse its (now `Drop`-driven)
                        // type-dispatched subtable teardown on this one subtable --
                        // never pushed anywhere, so it's just let go out of scope
                        // instead of the old explicit `otfcc_delete_lookup` call.
                        let mut temp: Box<Lookup> = new_lookup();
                        (*temp).type_0 = ext.type_0;
                        (*temp).subtables.push(subtable_list_slot(ext.subtable));
                        drop(temp);
                        // Slot already `None` from `.take()` above.
                    }
                }
                j_1 = j_1.wrapping_add(1);
            }
        } else {
            // Was `otl_subtable_list_dispose_dependent(..); return;` -- with
            // `SubtableList` now `Vec<Option<Box<Subtable>>>`, there is
            // nothing left to eagerly dispose: whatever remains in
            // `(*lookup).subtables` (still holding valid, un-expanded
            // `Extend` placeholders) tears down correctly whenever `lookup`
            // itself eventually drops, since `Subtable::drop` dispatches off
            // each element's own enum tag, not `(*lookup).type_0` -- which
            // this function already overwrote to `OTL_TYPE_UNKNOWN` above,
            // before B-1 this would have been the wrong type to free by.
            return;
        }
    }
    if (*lookup).type_0 == OTL_TYPE_GSUB_CONTEXT {
        (*lookup).type_0 = OTL_TYPE_GSUB_CHAINING;
    }
    if (*lookup).type_0 == OTL_TYPE_GPOS_CONTEXT {
        (*lookup).type_0 = OTL_TYPE_GPOS_CHAINING;
    }
}
pub unsafe fn otfcc_read_otl(
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
    let otl_ptr: *mut OtlTable = otl_box.as_mut() as *mut OtlTable;
    // See `chaining::read::reset_class_coverage_budgets`'s own doc comment:
    // this must run once per table (GSUB or GPOS), before any of this
    // table's lookups are read, so the budget bounds this whole table's
    // total `class_coverage` cost rather than resetting fresh per subtable.
    crate::table::otl::subtables::chaining::read::reset_class_coverage_budgets();
    for j in 0..(*otl_ptr).lookups.len() {
        otfcc_read_otl_lookup(
            &table.data,
            &raw mut *(&mut (*otl_ptr).lookups)[j],
            max_glyphs,
            options,
        );
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
        unsafe {
            let otl = parse_otl_common(&data, OTL_TYPE_GSUB_UNKNOWN, &options).unwrap();
            assert_eq!(otl.lookups.len(), 1);
            assert_eq!(otl.features.len(), 1);
            assert_eq!(otl.features[0].name, b"liga_00000"); // Dec5 zero-pads the index
            assert_eq!(otl.features[0].lookups.len(), 1);
            assert_eq!(otl.languages.len(), 1); // only the non-default langSys; defaultLangSys was 0
        }
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
        unsafe {
            assert!(parse_otl_common(&data, OTL_TYPE_GSUB_UNKNOWN, &options).is_err());
        }
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
        unsafe {
            let otl = parse_otl_common(&data, OTL_TYPE_GSUB_UNKNOWN, &options).unwrap();
            assert_eq!(otl.languages.len(), 1);
            assert!(otl.languages[0].features.is_empty());
            assert!(otl.languages[0].required_feature.is_null());
        }
    }

    #[test]
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
        unsafe {
            let otl = parse_otl_common(&data, OTL_TYPE_GSUB_UNKNOWN, &options).unwrap();
            assert_eq!(otl.languages.len(), MAX_TOTAL_LANGUAGES as usize);
        }
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
        unsafe {
            (*lookup)._offset = 0;
            // Not GSUB_EXTEND/GPOS_EXTEND, so the extend-unwrap branch
            // below is skipped; not a real per-format type either, so
            // `otfcc_read_otl_subtable` (unconverted, out of scope) falls
            // through to its null-return arm -- this test only checks
            // that one subtable slot was appended, not what's in it.
            (*lookup).type_0 = OTL_TYPE_GSUB_UNKNOWN;
            let lookup_ptr: *mut Lookup = lookup.as_mut() as *mut Lookup;
            otfcc_read_otl_lookup(&data, lookup_ptr, 0, &options);
            assert_eq!((*lookup_ptr).subtables.len(), 1);
        }
    }

    #[test]
    fn subtable_count_zero_marks_the_lookup_unknown() {
        let data = well_formed_gsub(); // subtableCount is already 0
        let options = zeroed_options();
        unsafe {
            let mut otl = parse_otl_common(&data, OTL_TYPE_GSUB_UNKNOWN, &options).unwrap();
            let lookup_ptr: *mut Lookup = &raw mut *otl.lookups[0];
            otfcc_read_otl_lookup(&data, lookup_ptr, 0, &options);
            assert_eq!((*lookup_ptr).type_0, OTL_TYPE_UNKNOWN);
        }
    }
}
