#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod unconsolidate;

use crate::support::options::Options;
use crate::support::primitives::{GlyphId, ShapeId};

use crate::font::caryll_font::{Font, FontSubtype};
use crate::font::caryll_sfnt::{Packet, PacketPiece, SplineFontContainer};

use crate::table::cff::{CffAndGlyf, unwrap_cff_table};
use crate::table::glyf::{GlyfIOContext, unwrap_glyf_table};

use crate::font::caryll_font::otfcc_font_create;
use crate::otf_reader::unconsolidate::otfcc_unconsolidate_font;
use crate::table::_tsi::otfcc_read_tsi;
use crate::table::base::otfcc_read_base;
use crate::table::cff::otfcc_read_cff_and_glyf_tables;
use crate::table::cmap::otfcc_read_cmap;
use crate::table::colr::otfcc_read_colr;
use crate::table::cpal::otfcc_read_cpal;
use crate::table::cvt::otfcc_read_cvt;
use crate::table::fpgm_prep::otfcc_read_fpgm_prep;
use crate::table::fvar::{FvarTable, otfcc_read_fvar};
use crate::table::gasp::otfcc_read_gasp;
use crate::table::gdef::otfcc_read_gdef;
use crate::table::glyf::read::otfcc_read_glyf;
use crate::table::head::{HeadTable, otfcc_read_head};
use crate::table::hhea::otfcc_read_hhea;
use crate::table::hmtx::otfcc_read_hmtx;
use crate::table::ltsh::otfcc_read_ltsh;
use crate::table::maxp::otfcc_read_maxp;
use crate::table::meta::read::otfcc_read_meta;
use crate::table::name::otfcc_read_name;
use crate::table::os_2::otfcc_read_os_2;
use crate::table::otl::read::otfcc_read_otl;
use crate::table::post::otfcc_read_post;
use crate::table::svg::otfcc_read_svg;
use crate::table::tsi5::otfcc_read_tsi5;
use crate::table::vdmx::funcs::otfcc_read_vdmx;
use crate::table::vhea::otfcc_read_vhea;
use crate::table::vmtx::otfcc_read_vmtx;
use crate::table::vorg::otfcc_read_vorg;

unsafe fn decide_font_subtype_otf(sfnt: *mut SplineFontContainer, index: u32) -> FontSubtype {
    // c2rust's translation of a FOREACH_TABLE-style macro: the
    // __fortable_keep/__notfound/__fortable_k2 flags simulate a
    // single-iteration inner scope purely to give the original C a labeled
    // break/continue target. Traced by hand: the whole thing reduces to
    // "return FontSubtype::Cff at the first 'cff ' tag, else FontSubtype::Ttf".
    let sfnt_packets = &(*sfnt).packets;
    let packet: &Packet = &sfnt_packets[index as usize];
    for i in 0..packet.num_tables as i32 {
        let table: &PacketPiece = &packet.pieces[i as usize];
        if table.tag == crate::tag::TAG_CFF {
            return FontSubtype::Cff;
        }
    }
    return FontSubtype::Ttf;
}
// Options and Font are duplicated per-file by c2rust (like every
// other type in this crate); the trait boundary uses erased c_void pointers
// so this trait can be shared with json_reader.rs without deduping those
// pervasively-used types. Casts are confined to the boundary; the pointee
// layout is unchanged (same technique already relied on for the excluded
// dump/parse/build methods in Track 1's package vtables).
pub(crate) trait FontBuilder {
    unsafe fn read(
        buf: *mut ::core::ffi::c_void,
        len: u32,
        options: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}
struct OtfReader;
impl FontBuilder for OtfReader {
    unsafe fn read(
        mut _sfnt: *mut ::core::ffi::c_void,
        index: u32,
        options: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void {
        let options: &Options = &*(options as *const Options);
        let sfnt: *mut SplineFontContainer = _sfnt as *mut SplineFontContainer;
        if (*sfnt).count.wrapping_sub(1_u32) < index {
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        } else {
            let font: *mut Font = (otfcc_font_create)();
            let sfnt_packets = &(*sfnt).packets;
            let packet: &Packet = &sfnt_packets[index as usize];
            (*font).subtype = decide_font_subtype_otf(sfnt, index);
            (*font).fvar = otfcc_read_fvar(packet, options);
            (*font).head = otfcc_read_head(packet, options);
            (*font).maxp = otfcc_read_maxp(packet, options);
            (*font).name = otfcc_read_name(packet, options);
            (*font).meta = otfcc_read_meta(packet, options);
            (*font).os_2 = otfcc_read_os_2(packet, options);
            (*font).post = otfcc_read_post(packet, options);
            (*font).hhea = otfcc_read_hhea(packet, options);
            (*font).cmap = otfcc_read_cmap(packet, options);
            if (*font).subtype == FontSubtype::Ttf {
                (*font).hmtx = otfcc_read_hmtx(
                    packet,
                    options,
                    (*font).hhea.as_deref(),
                    (*font).maxp.as_deref(),
                );
                (*font).vhea = otfcc_read_vhea(packet, options);
                if (*font).vhea.is_some() {
                    (*font).vmtx = otfcc_read_vmtx(
                        packet,
                        options,
                        (*font).vhea.as_deref(),
                        (*font).maxp.as_deref(),
                    );
                }
                (*font).fpgm = otfcc_read_fpgm_prep(packet, crate::tag::TAG_FPGM);
                (*font).prep = otfcc_read_fpgm_prep(packet, crate::tag::TAG_PREP);
                (*font).cvt_ = otfcc_read_cvt(packet, crate::tag::TAG_CVT);
                (*font).gasp = otfcc_read_gasp(packet, options);
                (*font).vdmx = otfcc_read_vdmx(packet, options);
                (*font).ltsh = otfcc_read_ltsh(packet, options);
                // `loca_is_long`/`num_glyphs` come from `head`/`maxp`, which
                // -- unlike the CFF branch below, which already tolerates a
                // missing `head` via `.map_or(null(), ...)` -- this branch
                // used to `.unwrap()` unconditionally. A malformed font
                // missing (or failing to parse) either table turned into a
                // panic here instead of the "skip this table, keep going"
                // every other reader in this function already does; a
                // fuzz-found input with a `glyf`/`loca` pair but no `maxp`
                // hit exactly this. `glyf` genuinely cannot be read without
                // both, so it is left `None` (its default) rather than
                // guessing at either value.
                if (*font).head.is_some() && (*font).maxp.is_some() {
                    let mut ctx: GlyfIOContext = GlyfIOContext {
                        loca_is_long: (*font).head.as_deref().unwrap().index_to_loc_format != 0,
                        num_glyphs: (*font).maxp.as_deref().unwrap().num_glyphs as GlyphId,
                        n_phantom_points: 4 as ShapeId,
                        fvar: (*font)
                            .fvar
                            .as_deref_mut()
                            .map_or(::core::ptr::null_mut(), |f| f as *mut FvarTable),
                        has_vertical_metrics: false,
                        export_fd_select: false,
                    };
                    (*font).glyf = otfcc_read_glyf(packet, options, &raw mut ctx);
                }
            } else {
                let cffpr: CffAndGlyf = otfcc_read_cff_and_glyf_tables(
                    packet,
                    options,
                    (*font)
                        .head
                        .as_deref()
                        .map_or(::core::ptr::null(), |h| h as *const HeadTable),
                );
                (*font).cff = unwrap_cff_table(cffpr.meta);
                (*font).glyf = unwrap_glyf_table(cffpr.glyphs);
                (*font).vhea = otfcc_read_vhea(packet, options);
                if (*font).vhea.is_some() {
                    (*font).vmtx = otfcc_read_vmtx(
                        packet,
                        options,
                        (*font).vhea.as_deref(),
                        (*font).maxp.as_deref(),
                    );
                    (*font).vorg = otfcc_read_vorg(packet, options);
                }
            }
            if let Some(glyf) = (*font).glyf.as_ref() {
                let num_glyphs = glyf.len() as GlyphId;
                (*font).gsub = otfcc_read_otl(packet, options, crate::tag::TAG_GSUB, num_glyphs);
                (*font).gpos = otfcc_read_otl(packet, options, crate::tag::TAG_GPOS, num_glyphs);
                (*font).gdef = otfcc_read_gdef(packet);
            }
            (*font).base = otfcc_read_base(packet, options);
            (*font).cpal = otfcc_read_cpal(packet);
            (*font).colr = otfcc_read_colr(packet, options);
            (*font).svg = otfcc_read_svg(packet);
            (*font).tsi_01 = otfcc_read_tsi(packet, crate::tag::TAG_TSI0, crate::tag::TAG_TSI1);
            (*font).tsi_23 = otfcc_read_tsi(packet, crate::tag::TAG_TSI2, crate::tag::TAG_TSI3);
            (*font).tsi5 = otfcc_read_tsi5(packet);
            otfcc_unconsolidate_font(font, options);
            return font as *mut ::core::ffi::c_void;
        };
    }
}
pub unsafe fn read_otf(
    mut _sfnt: *mut ::core::ffi::c_void,
    index: u32,
    options: &Options,
) -> *mut Font {
    <OtfReader as FontBuilder>::read(
        _sfnt,
        index,
        options as *const Options as *const ::core::ffi::c_void,
    ) as *mut Font
}

#[cfg(test)]
mod regression_tests {
    use crate::consolidate::otfcc_consolidate_font;
    use crate::font::caryll_font::otfcc_font_free;
    use crate::font::caryll_sfnt::{otfcc_delete_sfnt, otfcc_read_sfnt_from_reader};
    use crate::logger::{Logger, otfcc_new_empty_target};
    use crate::support::options::{otfcc_delete_options, otfcc_new_options};
    use std::cell::RefCell;
    use std::io::Cursor;
    use std::time::{Duration, Instant};

    /// `tests/fuzz-corpus/known-issues/otf-parse-cff-per-glyph-stack-
    /// realloc-hang.bin` (CID-keyed CFF, `CharStrings` count mutated to
    /// 65535 -- the corpus's max u16) used to spend 30+ seconds and
    /// multiple gigabytes of allocator churn in `table/cff.rs`'s
    /// `build_outline`, which allocated a fresh 0x10000-entry `Vec<
    /// CffValue>` operand stack on *every glyph* instead of once for the
    /// whole font (found by `cargo fuzz run otf_parse`, see `rust/
    /// README.md`'s "Next steps" for the fix and how it was isolated).
    /// 10 seconds is generous slack over the ~4 seconds this takes under
    /// `cargo fuzz`'s ASan-instrumented build (this plain `cargo test
    /// --release` build has no such instrumentation, so it's meaningfully
    /// faster) -- comfortably below the original bug's 30+ second hang,
    /// comfortably above any plausible legitimate variance.
    #[test]
    // Reads a fixture from disk; same rationale as `parsed_json.rs`'s
    // `every_committed_payload_json_parses`.
    #[cfg_attr(
        miri,
        ignore = "reads a fixture from disk, needs -Zmiri-disable-isolation; also far too slow to run meaningfully under Miri's interpreter"
    )]
    fn cff_font_with_huge_glyph_count_parses_promptly() {
        let bytes = std::fs::read(
            "../tests/fuzz-corpus/known-issues/otf-parse-cff-per-glyph-stack-realloc-hang.bin",
        )
        .unwrap();
        unsafe {
            let sfnt = otfcc_read_sfnt_from_reader(&mut Cursor::new(bytes.as_slice()));
            assert!(!sfnt.is_null());

            let options = otfcc_new_options();
            (*options).logger = RefCell::new(Logger::new(otfcc_new_empty_target()));

            let start = Instant::now();
            let font = super::read_otf(sfnt as *mut ::core::ffi::c_void, 0, &*options);
            let elapsed = start.elapsed();

            otfcc_delete_sfnt(sfnt);
            if !font.is_null() {
                otfcc_font_free(font);
            }
            otfcc_delete_options(options);

            assert!(
                elapsed < Duration::from_secs(10),
                "read_otf took {elapsed:?}, expected well under 10s"
            );
        }
    }

    /// A `cargo fuzz run otf_parse` CI job found `tests/fuzz-corpus/known-
    /// issues/otf-parse-otl-contextual-amplification-hang.bin`: a GSUB
    /// table whose contextual/chaining subtables compound *four* separate
    /// unbounded counts, each individually bounds-checked against the
    /// table (its own array fits) but none bounded in aggregate --
    /// `otl/read.rs`'s lookup list (`lookup_count`, up to 65535), one
    /// lookup's own subtable list (`subtable_count`), one subtable's
    /// `chainSubClassSet`/`subRuleSet` rule counts (`chaining/read.rs`'s
    /// `read_contextual_format2` et al.), and one rule's own backtrack/
    /// input/lookahead/apply position counts (`general_read_contextual_
    /// rule`/`general_read_chaining_rule`). This file rode several of
    /// those close to their ceiling at once: ASan first reported it as a
    /// ~1.8GB OOM, and once that was capped it became a 30+ second hang
    /// (millions of `class_coverage` calls, each allocating a `Coverage`
    /// it immediately discards). Fixed with a chain of budgets: `otl/
    /// read.rs`'s `MAX_TOTAL_SUBTABLES_PER_LOOKUP` caps subtables per
    /// lookup; `chaining/read.rs`'s `MAX_TOTAL_RULES_PER_TABLE` (a global,
    /// per-`otfcc_read_otl`-call budget, not per-subtable -- an earlier,
    /// per-subtable-only version of this cap still let many subtables
    /// each spend their own full allowance) caps rules built across the
    /// whole table; `MAX_APPLY_PER_RULE`/`MAX_POSITIONS_PER_RULE` cap one
    /// rule's own apply/position counts; and `CLASS_ZERO_BUDGET`/
    /// `CLASS_COVERAGE_CALL_BUDGET` (also globalized from an earlier,
    /// per-subtable version for the same reason) cap `class_coverage`'s
    /// total work across the whole table. A fifth, unrelated bug fell out
    /// of the same file: `general_read_contextual_rule`/`_chaining_rule`
    /// can return `None` for one malformed rule inside an otherwise valid
    /// ruleset (its own offset pointed at a truncated/malformed rule
    /// header) -- the four `read_*` call sites used to push that `None`
    /// straight into `ChainingRuleSet.rules` regardless, and `otf_reader/
    /// unconsolidate.rs`'s `unconsolidate_chaining` `.expect()`ed every
    /// slot to be `Some`, so this file also panicked before it could even
    /// reach the hang. Now skipped instead of pushed, matching this
    /// crate's usual "malformed sub-part is dropped, not fatal" shape.
    #[test]
    // Reads a fixture from disk; same rationale as `parsed_json.rs`'s
    // `every_committed_payload_json_parses`.
    #[cfg_attr(
        miri,
        ignore = "reads a fixture from disk, needs -Zmiri-disable-isolation; also far too slow to run meaningfully under Miri's interpreter"
    )]
    fn otl_contextual_amplification_font_parses_promptly() {
        let bytes = std::fs::read(
            "../tests/fuzz-corpus/known-issues/otf-parse-otl-contextual-amplification-hang.bin",
        )
        .unwrap();
        unsafe {
            let sfnt = otfcc_read_sfnt_from_reader(&mut Cursor::new(bytes.as_slice()));
            assert!(!sfnt.is_null());

            let options = otfcc_new_options();
            (*options).logger = RefCell::new(Logger::new(otfcc_new_empty_target()));

            let start = Instant::now();
            let font = super::read_otf(sfnt as *mut ::core::ffi::c_void, 0, &*options);
            let elapsed = start.elapsed();

            otfcc_delete_sfnt(sfnt);
            if !font.is_null() {
                otfcc_font_free(font);
            }
            otfcc_delete_options(options);

            assert!(
                elapsed < Duration::from_secs(10),
                "read_otf took {elapsed:?}, expected well under 10s"
            );
        }
    }

    /// A follow-up `cargo fuzz run otf_parse` CI job (after the fix above
    /// landed) found `tests/fuzz-corpus/known-issues/otf-parse-otl-
    /// feature-ref-amplification-oom.bin`: a genuinely *different* bug in
    /// the same file (`otl/read.rs`), an out-of-memory this time rather
    /// than a hang -- libFuzzer's ASan-instrumented build hit its 2048MB
    /// `-rss_limit_mb`. `parse_language`'s own `feature_count` (a raw
    /// `u16`, up to 65535 per language) was bounds-checked only against
    /// that one `LangSys` table's own bytes, with no cap against `MAX_
    /// TOTAL_LANGUAGES`'s own per-table budget -- one pathological
    /// `LangSys` table pushed a huge number of (mostly duplicate, aliased)
    /// feature references into a single language's `features` list.
    /// Separately, `parse_otl_common`'s own `lookup_count` (also raw,
    /// up to 65535) had no cap at all: this same file also had ~10,300
    /// lookups, each contributing its own (otherwise-bounded) content to
    /// the output. Neither factor alone explained the memory use on its
    /// own -- bisecting by tightening `MAX_TOTAL_RULES_PER_TABLE` down to
    /// 2,000 (10x below what `tests/payload/NotoNastaliqUrdu-Regular.ttf`
    /// needs) barely moved this file's peak RSS at all, which is what
    /// pointed at `lookup_count`/`feature_count` instead. Fixed with `MAX_
    /// TOTAL_LOOKUPS_PER_TABLE` (500) and `MAX_TOTAL_FEATURE_REFS_PER_
    /// TABLE` (100,000, global across the table like `MAX_TOTAL_RULES_
    /// PER_TABLE`). Confirmed locally (native, no ASan): this exact file's
    /// peak RSS dropped from ~1GB to ~34MB.
    #[test]
    #[cfg_attr(
        miri,
        ignore = "reads a fixture from disk, needs -Zmiri-disable-isolation; also far too slow to run meaningfully under Miri's interpreter"
    )]
    fn otl_feature_ref_amplification_font_parses_promptly() {
        let bytes = std::fs::read(
            "../tests/fuzz-corpus/known-issues/otf-parse-otl-feature-ref-amplification-oom.bin",
        )
        .unwrap();
        unsafe {
            let sfnt = otfcc_read_sfnt_from_reader(&mut Cursor::new(bytes.as_slice()));
            assert!(!sfnt.is_null());

            let options = otfcc_new_options();
            (*options).logger = RefCell::new(Logger::new(otfcc_new_empty_target()));

            let start = Instant::now();
            let font = super::read_otf(sfnt as *mut ::core::ffi::c_void, 0, &*options);
            let elapsed = start.elapsed();

            // The wall-clock check below alone doesn't actually catch this
            // regression: this file's memory blowup happens fast enough on
            // native, uninstrumented hardware that even the fully-uncapped
            // version parses in well under a second here -- it only
            // crossed libFuzzer's 2048MB `-rss_limit_mb` under ASan's
            // memory-overhead multiplier in CI. Assert the actual
            // invariant the fix establishes instead: neither cap was
            // exceeded, for either table.
            assert!(!font.is_null());
            for otl in [(*font).gsub.as_deref(), (*font).gpos.as_deref()]
                .into_iter()
                .flatten()
            {
                assert!(
                    otl.lookups.len()
                        <= crate::table::otl::read::MAX_TOTAL_LOOKUPS_PER_TABLE as usize,
                    "lookups.len() = {} exceeds MAX_TOTAL_LOOKUPS_PER_TABLE",
                    otl.lookups.len()
                );
                let total_feature_refs: usize =
                    otl.languages.iter().map(|lang| lang.features.len()).sum();
                assert!(
                    total_feature_refs
                        <= crate::table::otl::read::MAX_TOTAL_FEATURE_REFS_PER_TABLE as usize,
                    "total feature refs = {total_feature_refs} exceeds MAX_TOTAL_FEATURE_REFS_PER_TABLE"
                );
            }

            otfcc_delete_sfnt(sfnt);
            if !font.is_null() {
                otfcc_font_free(font);
            }
            otfcc_delete_options(options);

            assert!(
                elapsed < Duration::from_secs(10),
                "read_otf took {elapsed:?}, expected well under 10s"
            );
        }
    }

    /// A follow-up `cargo fuzz run otf_parse` CI job (this time triggered
    /// by an unrelated PR that merely touched nearby code, not this
    /// amplification itself) found a *third*, independent amplification
    /// axis in the same file: `parse_otl_common`'s own Feature List loop
    /// (not `parse_language`'s) had no cap on `feature_count` (a raw
    /// `u16`, up to 65535), and each feature's own `lookup_count_0` (also
    /// raw, up to 65535, bounds-checked only against that one feature's
    /// own bytes) was read independently. A crafted table pointing many
    /// `feature_count` entries at overlapping/repeated `feature_offset`s,
    /// each claiming a large `lookup_count_0`, made total inner-loop
    /// reads scale with `feature_count * lookup_count_0` rather than the
    /// table's real size -- CI hung for 25+ seconds and exceeded
    /// libFuzzer's 2048MB `-rss_limit_mb`. Fixed by capping both factors
    /// independently: `MAX_TOTAL_FEATURES_PER_TABLE` on the outer loop,
    /// `MAX_TOTAL_LOOKUPS_PER_TABLE` (reused) on the inner one.
    #[test]
    #[cfg_attr(
        miri,
        ignore = "reads a fixture from disk, needs -Zmiri-disable-isolation; also far too slow to run meaningfully under Miri's interpreter"
    )]
    fn otl_feature_list_amplification_font_parses_promptly() {
        let bytes = std::fs::read(
            "../tests/fuzz-corpus/known-issues/otf-parse-otl-feature-list-amplification-hang.bin",
        )
        .unwrap();
        unsafe {
            let sfnt = otfcc_read_sfnt_from_reader(&mut Cursor::new(bytes.as_slice()));
            assert!(!sfnt.is_null());

            let options = otfcc_new_options();
            (*options).logger = RefCell::new(Logger::new(otfcc_new_empty_target()));

            let start = Instant::now();
            let font = super::read_otf(sfnt as *mut ::core::ffi::c_void, 0, &*options);
            let elapsed = start.elapsed();

            // Same reasoning as the feature-ref-amplification test above:
            // assert the actual invariant the fix establishes, not just
            // wall-clock time (which may not reliably distinguish "fixed"
            // from "fast enough on this hardware" alone).
            assert!(!font.is_null());
            for otl in [(*font).gsub.as_deref(), (*font).gpos.as_deref()]
                .into_iter()
                .flatten()
            {
                assert!(
                    otl.features.len()
                        <= crate::table::otl::read::MAX_TOTAL_FEATURES_PER_TABLE as usize,
                    "features.len() = {} exceeds MAX_TOTAL_FEATURES_PER_TABLE",
                    otl.features.len()
                );
                for feature in &otl.features {
                    assert!(
                        feature.lookups.len()
                            <= crate::table::otl::read::MAX_TOTAL_LOOKUPS_PER_TABLE as usize,
                        "feature.lookups.len() = {} exceeds MAX_TOTAL_LOOKUPS_PER_TABLE",
                        feature.lookups.len()
                    );
                }
            }

            otfcc_delete_sfnt(sfnt);
            if !font.is_null() {
                otfcc_font_free(font);
            }
            otfcc_delete_options(options);

            assert!(
                elapsed < Duration::from_secs(10),
                "read_otf took {elapsed:?}, expected well under 10s"
            );
        }
    }

    /// A `cargo fuzz run otf_parse` CI job found this: a TTF-subtype font
    /// (no `CFF ` table, so `decide_font_subtype_otf` defaults to `Ttf`)
    /// with a `head` table but no `maxp` table panicked with `called
    /// `Option::unwrap()` on a `None` value` at the `GlyfIOContext`
    /// construction inside `OtfReader::read`'s TTF branch -- unlike the
    /// CFF branch right below it (which already tolerates a missing
    /// `head` via `.map_or(null(), ...)`), this branch unconditionally
    /// `.unwrap()`ed both `head.index_to_loc_format` and `maxp.
    /// num_glyphs`. Fixed by skipping the whole `glyf`-read block (leaving
    /// `font.glyf` at its default `None`) unless both are present, the
    /// same "skip this table, keep going" shape `vhea`/`vmtx` right above
    /// it already use.
    ///
    /// Minimal synthetic sfnt: version `\x00\x01\x00\x00` (TrueType),
    /// one table (`head`, 54 bytes, valid enough to parse), no `maxp`
    /// table at all.
    #[test]
    fn ttf_font_missing_maxp_does_not_panic() {
        let mut data = Vec::new();
        data.extend_from_slice(&0x00010000u32.to_be_bytes()); // sfnt version
        data.extend_from_slice(&1u16.to_be_bytes()); // numTables
        data.extend_from_slice(&[0u8; 6]); // searchRange, entrySelector, rangeShift
        data.extend_from_slice(b"head");
        data.extend_from_slice(&0u32.to_be_bytes()); // checkSum (unverified)
        data.extend_from_slice(&28u32.to_be_bytes()); // offset: right after the 12+16-byte directory
        data.extend_from_slice(&54u32.to_be_bytes()); // length
        assert_eq!(data.len(), 28);
        data.extend_from_slice(&0x00010000u32.to_be_bytes()); // head.version
        data.extend_from_slice(&[0u8; 50]); // the rest of head's 54 bytes, all zero is fine
        assert_eq!(data.len(), 28 + 54);

        unsafe {
            let sfnt = otfcc_read_sfnt_from_reader(&mut Cursor::new(data.as_slice()));
            assert!(!sfnt.is_null());

            let options = otfcc_new_options();
            (*options).logger = RefCell::new(Logger::new(otfcc_new_empty_target()));

            let font = super::read_otf(sfnt as *mut ::core::ffi::c_void, 0, &*options);

            otfcc_delete_sfnt(sfnt);
            assert!(!font.is_null());
            assert!((*font).maxp.is_none());
            assert!((*font).glyf.is_none());
            otfcc_font_free(font);
            otfcc_delete_options(options);
        }
    }

    /// A follow-up `cargo fuzz run otf_dump` CI job found a second,
    /// independent panic from the exact same "TTF font missing `maxp`"
    /// shape the test above already covers on the *read* side --
    /// `json_writer.rs`'s `JsonSerializer::serialize` (the `otf_dump`
    /// path, not `otf_parse`) builds its own `GlyfIOContext` for the dump
    /// step and unconditionally `.unwrap()`ed `(*font).head`/`(*font).
    /// maxp` there too, independently of `OtfReader::read`'s own fix
    /// above. That fix only stops `font.glyf` from being *populated* when
    /// `head`/`maxp` are missing -- it does nothing to stop `font.head`/
    /// `font.maxp` themselves from legitimately being `None`, which is
    /// exactly what `json_writer.rs`'s own unwraps still choked on.
    /// `otfcc_dump_glyf` itself already no-ops on a `None` table, so the
    /// fix is the same shape as the read-side one: skip building `ctx`
    /// (and calling `otfcc_dump_glyf`) unless both `head` and `maxp` are
    /// present.
    #[test]
    fn dump_of_ttf_font_missing_maxp_does_not_panic() {
        let mut data = Vec::new();
        data.extend_from_slice(&0x00010000u32.to_be_bytes()); // sfnt version
        data.extend_from_slice(&1u16.to_be_bytes()); // numTables
        data.extend_from_slice(&[0u8; 6]); // searchRange, entrySelector, rangeShift
        data.extend_from_slice(b"head");
        data.extend_from_slice(&0u32.to_be_bytes()); // checkSum (unverified)
        data.extend_from_slice(&28u32.to_be_bytes()); // offset: right after the 12+16-byte directory
        data.extend_from_slice(&54u32.to_be_bytes()); // length
        assert_eq!(data.len(), 28);
        data.extend_from_slice(&0x00010000u32.to_be_bytes()); // head.version
        data.extend_from_slice(&[0u8; 50]); // the rest of head's 54 bytes, all zero is fine
        assert_eq!(data.len(), 28 + 54);

        unsafe {
            let sfnt = otfcc_read_sfnt_from_reader(&mut Cursor::new(data.as_slice()));
            assert!(!sfnt.is_null());

            let options = otfcc_new_options();
            (*options).logger = RefCell::new(Logger::new(otfcc_new_empty_target()));

            let font = super::read_otf(sfnt as *mut ::core::ffi::c_void, 0, &*options);
            otfcc_delete_sfnt(sfnt);
            assert!(!font.is_null());
            assert!((*font).maxp.is_none());

            let json = crate::json_writer::serialize_to_json(font, &*options)
                as *mut crate::support::built_json::BuiltValue;
            assert!(!json.is_null());
            drop(Box::from_raw(json));

            otfcc_font_free(font);
            otfcc_delete_options(options);
        }
    }

    /// `tests/fuzz-corpus/known-issues/otf-dump-otl-coverage-consolidate-
    /// amplification-hang.bin`: a `cargo fuzz run otf_dump` CI job found
    /// this (`otf_dump` runs `read_otf` *and* `otfcc_consolidate_font`,
    /// unlike `otf_parse` above -- see `otf_dump.rs`'s own doc comment for
    /// why that gap matters). Chased through two distinct amplifications
    /// stacked on the same font, each independently bounded but not
    /// jointly, before this test terminated promptly:
    ///
    /// 1. `table/otl/coverage.rs::read_coverage`'s Coverage-format-2
    ///    handler: `range_count` and each range's own `start..=end` span
    ///    are each individually bounded, but their *product* was not --
    ///    this font's coverage table expanded into billions of
    ///    `IndexMap::entry` calls. Fixed by
    ///    `reset_coverage_range_expansion_budget`'s table-wide budget.
    /// 2. `consolidate.rs::__declare_otl_consolidation`: once (1)'s fix
    ///    made most of this font's ~300,000 possible subtable slots
    ///    (`MAX_TOTAL_LOOKUPS_PER_TABLE` * `MAX_TOTAL_SUBTABLES_PER_
    ///    LOOKUP`) fail to parse, this loop's "Ignored empty subtable"
    ///    warning was unconditionally built for every one of them --
    ///    despite `verbosity_limit` defaulting to 0, below the
    ///    `LOG_VL_IMPORTANT` these warnings log at, meaning none of them
    ///    were ever actually displayed. Fixed by checking `verbosity_
    ///    limit` before building the message, not after.
    ///
    /// Neither fix's own test (in `coverage.rs`/`consolidate.rs`) alone
    /// reproduces this specific font's full cost the way running the
    /// actual fuzz-found bytes through both stages together does, which
    /// is why this test exists in addition to those. This build's
    /// ASan-instrumented `cargo fuzz` counterpart took 34s (down from a
    /// CI timeout at 1753s, past the 1200s per-unit alarm); this plain
    /// `cargo test --release` build has no such instrumentation, so 15
    /// seconds is generous slack while staying far below the original
    /// hang.
    #[test]
    #[cfg_attr(
        miri,
        ignore = "reads a fixture from disk, needs -Zmiri-disable-isolation; also far too slow to run meaningfully under Miri's interpreter"
    )]
    fn otl_coverage_and_consolidate_log_amplification_font_dumps_promptly() {
        let bytes = std::fs::read(
            "../tests/fuzz-corpus/known-issues/otf-dump-otl-coverage-consolidate-amplification-hang.bin",
        )
        .unwrap();
        unsafe {
            let sfnt = otfcc_read_sfnt_from_reader(&mut Cursor::new(bytes.as_slice()));
            assert!(!sfnt.is_null());

            let options = otfcc_new_options();
            (*options).logger = RefCell::new(Logger::new(otfcc_new_empty_target()));

            let start = Instant::now();
            let font = super::read_otf(sfnt as *mut ::core::ffi::c_void, 0, &*options);
            if !font.is_null() {
                otfcc_consolidate_font(font, &*options);
            }
            let elapsed = start.elapsed();

            otfcc_delete_sfnt(sfnt);
            if !font.is_null() {
                otfcc_font_free(font);
            }
            otfcc_delete_options(options);

            assert!(
                elapsed < Duration::from_secs(15),
                "read_otf + otfcc_consolidate_font took {elapsed:?}, expected well under 15s"
            );
        }
    }
}
