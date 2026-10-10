use crate::logger::ByteStr;
use otfcc_binary::bk::block::bk_new_block_from_buffer;
use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use otfcc_binary::bk::graph::bk_build_block;
use otfcc_binary::Buffer;
use crate::support::primitives::TableId;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::subtables::chaining::build::chaining_lookup_is_contextual_lookup;
use crate::table::otl::kind::lookup_kind;
use crate::table::otl::{
    FeatureIdx, LanguageSystem, Lookup, LookupIdx, OTL_TYPE_GPOS_EXTEND, OTL_TYPE_GPOS_UNKNOWN,
    OTL_TYPE_GSUB_EXTEND, OTL_TYPE_GSUB_SINGLE, OTL_TYPE_GSUB_UNKNOWN, OtlTable, Subtable,
    iter_subtables,
};
/// Maps each *storage* index into `OtlTable.lookups`/`.features`
/// (`LookupIdx.0`/`FeatureIdx.0` as `usize`) to its *dense* position in the
/// binary LookupList/FeatureList this module writes -- `Some(dense_pos)`
/// for a live (`Some`) slot, `None` for a hole consolidation punched.
/// Holes are omitted from binary output entirely (no empty placeholder
/// slot, matching what the old `Vec::retain`-based compaction did before
/// `OtlTable.lookups`/`.features` became hole-preserving); a *live* lookup
/// that happens to build zero binary subtables is a different, pre-existing
/// case (see `write_otl_lookups`'s own handling) and is not a hole.
fn storage_to_dense<T>(list: &[Option<T>]) -> Vec<Option<u16>> {
    let mut dense: u16 = 0;
    list.iter()
        .map(|slot| {
            slot.as_ref().map(|_| {
                let pos = dense;
                dense += 1;
                pos
            })
        })
        .collect()
}
pub const LARGE_SUBTABLE_LIMIT: i32 = 4096_i32;
fn feature_name_to_tag(name: &[u8]) -> u32 {
    let mut tag: u32 = 0_u32;
    if !name.is_empty() {
        tag |= ((name[0_usize] as i32) << 24_i32) as u32;
    } else {
        tag |= ((' ' as i32 as u8 as i32) << 24_i32) as u32;
    }
    if name.len() > 1_usize {
        tag |= ((name[1_usize] as i32) << 16_i32) as u32;
    } else {
        tag |= ((' ' as i32 as u8 as i32) << 16_i32) as u32;
    }
    if name.len() > 2_usize {
        tag |= ((name[2_usize] as i32) << 8_i32) as u32;
    } else {
        tag |= ((' ' as i32 as u8 as i32) << 8_i32) as u32;
    }
    if name.len() > 3_usize {
        tag |= (name[3_usize] as i32) as u32;
    } else {
        tag |= (' ' as i32 as u8 as i32) as u32;
    }
    return tag;
}
/// What building one lookup's subtables updates besides producing them: the
/// output buffer list, the running offset all lookups' subtables together
/// have reached so far, whether this lookup is large enough to want the
/// extension format by itself, and the build heuristics. Handed to each
/// kind's [`LookupKind::build_lookup`](crate::table::otl::kind::LookupKind::build_lookup).
pub struct LookupWriteCtx<'a> {
    pub subtables: &'a mut Vec<Buffer>,
    pub last_offset: &'a mut usize,
    pub prefer_extension_for_this_lut: &'a mut bool,
    pub heuristics: BuildHeuristics,
}
/// Builds each subtable of `lookup` into one buffer with `build`.
pub fn write_each_subtable(
    lookup: &Lookup,
    ctx: &mut LookupWriteCtx,
    build: fn(&Subtable, BuildHeuristics) -> Buffer,
) -> TableId {
    ctx.subtables.clear();
    ctx.subtables.reserve(lookup.subtables.len());
    // Bytes these subtables take in the lookup list: their own size, or
    // 8 bytes each when they go out of line behind extension subtables.
    let mut total_buf_size_short: usize = 0;
    let mut total_buf_size_ext: usize = 0;
    for subtable in iter_subtables(&lookup.subtables) {
        let buf: Buffer = build(subtable, ctx.heuristics);
        total_buf_size_short += buf.data.len();
        ctx.subtables.push(buf);
        total_buf_size_ext += 8;
    }
    if total_buf_size_short > LARGE_SUBTABLE_LIMIT as usize {
        *ctx.last_offset += total_buf_size_ext;
        *ctx.prefer_extension_for_this_lut = true;
    } else {
        *ctx.last_offset += total_buf_size_short;
        *ctx.prefer_extension_for_this_lut = false;
    }
    return lookup.subtables.len() as TableId;
}
/// Builds each subtable of `lookup` with `build`, which may split one
/// subtable into several buffers.
pub fn write_each_subtable_split(
    lookup: &Lookup,
    ctx: &mut LookupWriteCtx,
    build: fn(&Subtable, BuildHeuristics) -> Vec<Buffer>,
) -> TableId {
    ctx.subtables.clear();
    let mut total_buf_size_short: usize = 0;
    for subtable in iter_subtables(&lookup.subtables) {
        for buf in build(subtable, ctx.heuristics) {
            total_buf_size_short += buf.data.len();
            ctx.subtables.push(buf);
        }
    }
    let total = ctx.subtables.len() as TableId;
    if total_buf_size_short > LARGE_SUBTABLE_LIMIT as usize {
        *ctx.last_offset += 8 * total as usize;
        *ctx.prefer_extension_for_this_lut = true;
    } else {
        *ctx.last_offset += total_buf_size_short;
        *ctx.prefer_extension_for_this_lut = false;
    }
    return total;
}
fn get_lookup_heuristics(table: &OtlTable, lut_idx: LookupIdx, lut: &Lookup) -> BuildHeuristics {
    let mut heu: BuildHeuristics = BuildHeuristics::empty();
    if lut.lookup_type == OTL_TYPE_GSUB_SINGLE {
        // `fea.lookups[k]`/`lut_idx` are both storage-space `LookupIdx`
        // values -- comparing them directly is exactly what the old
        // pointer-identity comparison did, just spelled as an index
        // instead of a `*const Lookup`. No dense remap needed here: this
        // only asks "does this feature reference this lookup", not "at
        // what binary position".
        for fea in table.features.iter().flatten() {
            if feature_name_to_tag(&fea.name) == crate::tag::TAG_VERT
                && fea.lookups.contains(&lut_idx)
            {
                heu.insert(BuildHeuristics::GSUB_VERT);
            }
        }
    }
    return heu;
}
fn write_otl_lookups(table: &OtlTable, tag: &[u8]) -> BkBlock {
    // Storage-space `(LookupIdx, &Lookup)` pairs, holes (consolidation-
    // punched `None` slots) skipped -- every array below is indexed by
    // *this* sequence's dense position, matching the binary LookupList's
    // own positions exactly (a hole is omitted entirely, never written as
    // an empty placeholder; see `storage_to_dense`'s own doc comment for
    // how that differs from a *live* lookup that happens to build zero
    // binary subtables, handled unchanged a few lines down).
    let live: Vec<(LookupIdx, &Lookup)> = table
        .lookups
        .iter()
        .enumerate()
        .filter_map(|(i, l)| l.as_deref().map(|l| (LookupIdx(i as u32), l)))
        .collect();
    // `subtables`/`subtable_quantity`/`prefer_ext_for_this_lut` were three
    // separately `__caryll_allocate_clean`'d, index-parallel arrays, sized
    // once to `lookups.len()` and never resized after -- `Vec`s built the
    // same way (`vec![default; live.len()]`) reproduce the exact same
    // shape without a matching `free()` trio to remember at every exit
    // point below.
    let mut subtables: Vec<Vec<Buffer>> = vec![Vec::new(); live.len()];
    let mut subtable_quantity: Vec<TableId> = vec![0 as TableId; live.len()];
    let mut prefer_ext_for_this_lut: Vec<bool> = vec![false; live.len()];
    let mut last_offset: usize = 0_usize;
    for j in 0..live.len() {
        let (lookup_idx, lookup) = live[j];
        let heu: BuildHeuristics = get_lookup_heuristics(table, lookup_idx, lookup);
        tracing::debug!("Building lookup {} ({}/{})\n", ByteStr(&lookup.name), j as i32, live.len() as u32);
        let Some(kind) = lookup_kind(lookup.lookup_type) else {
            continue;
        };
        subtable_quantity[j] = kind.build_lookup(
            lookup,
            &mut LookupWriteCtx {
                subtables: &mut subtables[j],
                last_offset: &mut last_offset,
                prefer_extension_for_this_lut: &mut prefer_ext_for_this_lut[j],
                heuristics: heu,
            },
        );
    }
    // The lookup list and lookup headers: a count and an offset per
    // lookup, then 6 bytes plus 2 per subtable for each non-empty lookup.
    let mut header_size: usize = 2 + 2 * live.len();
    for &quantity in subtable_quantity.iter() {
        if quantity != 0 {
            header_size += 6 + 2 * quantity as usize;
        }
    }
    // Wraps when the headers alone pass 0xff00, which then never asks for
    // extension subtables, as it always has.
    let use_extended: bool = last_offset >= 0xff00_usize.wrapping_sub(header_size);
    let mut root: BkBlock = bk_new_block(vec![bk_int(BkCellType::B16, (live.len()) as u32)]);
    for j_1 in 0..live.len() {
        let (_, lookup_0) = live[j_1];
        if subtable_quantity[j_1] == 0 {
            tracing::info!("Lookup {} is empty.\n", ByteStr(&lookup_0.name));
        }
        let can_be_contextual: bool = chaining_lookup_is_contextual_lookup(lookup_0);
        let use_extended_for_it: bool =
            use_extended as i32 != 0 || prefer_ext_for_this_lut[j_1] as i32 != 0;
        if use_extended_for_it {
            tracing::info!("[OTFCC-fea] Using extended OpenType table layout for {}/{}.\n", ByteStr(tag), ByteStr(&lookup_0.name));
        }
        // The format number the file wants, which is the lookup type with its
        // table's base taken back off -- `LookupType::file_format`, the
        // same nested comparison C spelled out here and again below.
        let lookup_type: u16 = (if use_extended_for_it {
            if lookup_0.lookup_type > OTL_TYPE_GPOS_UNKNOWN {
                OTL_TYPE_GPOS_EXTEND.file_format()
            } else if lookup_0.lookup_type > OTL_TYPE_GSUB_UNKNOWN {
                OTL_TYPE_GSUB_EXTEND.file_format()
            } else {
                0
            }
        } else {
            lookup_0
                .lookup_type
                .file_format()
                .wrapping_sub(can_be_contextual as u32)
        }) as u16;
        let mut blk: BkBlock = bk_new_block(vec![
            bk_int(BkCellType::B16, (lookup_type as i32) as u32),
            bk_int(BkCellType::B16, (lookup_0.flags as i32) as u32),
            bk_int(
                BkCellType::B16,
                (subtable_quantity[j_1] as i32) as u32,
            ),
        ]);
        // Bounded by `subtable_quantity[j_1]`, not assumed equal to
        // `subtables[j_1].len()` (same count-vs-length caution
        // established in PR #422/#423/#426-428, even though the two are
        // always equal by construction here -- `build_lookup` returns
        // exactly the count it pushed).
        let quantity = subtable_quantity[j_1] as usize;
        for buf in subtables[j_1].iter_mut().take(quantity) {
            if use_extended_for_it {
                let extension_lookup_type: u16 = lookup_0
                    .lookup_type
                    .file_format()
                    .wrapping_sub(can_be_contextual as u32)
                    as u16;
                let stub: BkBlock = bk_new_block(vec![
                    bk_int(BkCellType::B16, 1_u32),
                    bk_int(BkCellType::B16, (extension_lookup_type as i32) as u32),
                    bk_ptr(
                        BkCellType::P32,
                        bk_new_block_from_buffer(Some(::core::mem::take(buf))),
                    ),
                ]);
                bk_push(&mut blk, vec![bk_ptr(BkCellType::P16, Some(stub))]);
            } else {
                bk_push(
                    &mut blk,
                    vec![bk_ptr(
                        BkCellType::P16,
                        bk_new_block_from_buffer(Some(::core::mem::take(buf))),
                    )],
                );
            }
        }
        bk_push(&mut blk, vec![bk_int(BkCellType::B16, 0_u32)]);
        bk_push(&mut root, vec![bk_ptr(BkCellType::P16, Some(blk))]);
    }
    return root;
}
fn write_otl_features(table: &OtlTable, lookup_dense: &[Option<u16>]) -> BkBlock {
    // Same "storage index, holes skipped, dense position is the binary
    // position" shape as `write_otl_lookups`'s own `live`.
    let live: Vec<&crate::table::otl::Feature> =
        table.features.iter().flatten().map(Box::as_ref).collect();
    let mut root: BkBlock = bk_new_block(vec![bk_int(BkCellType::B16, (live.len()) as u32)]);
    for &feature in live.iter() {
        let mut fea: BkBlock = bk_new_block(vec![
            bk_ptr(BkCellType::P16, None),
            bk_int(BkCellType::B16, (feature.lookups.len()) as u32),
        ]);
        for lookup_ref in feature.lookups.iter() {
            // Every `Feature.lookups` entry was already validated to
            // reference a *live* lookup by the fixed-point consolidation
            // pass (see `consolidate_otl_table`'s `otl_lookup_ref_list_
            // filter_env` call) -- resolving here through `lookup_dense`
            // should never see a hole; `.expect` turns "it somehow did"
            // into a loud panic rather than a silently wrong binary index.
            let dense = lookup_dense[lookup_ref.0 as usize]
                .expect("Feature.lookups should only ever reference a live lookup");
            bk_push(&mut fea, vec![bk_int(BkCellType::B16, dense as u32)]);
        }
        bk_push(
            &mut root,
            vec![
                bk_int(BkCellType::B32, feature_name_to_tag(&feature.name)),
                bk_ptr(BkCellType::P16, Some(fea)),
            ],
        );
    }
    return root;
}
// Resolves a `FeatureIdx` (storage index) to its dense binary position via
// `feature_dense` -- `0xffff` (the binary format's own "no feature" /
// out-of-range sentinel) both when there is no reference at all (`None`)
// and when `feature_dense` reports a hole, matching what the old raw-
// pointer `feature_index`'s linear "not found" fallthrough already did for
// a dangling/absent target.
fn feature_index(idx: Option<FeatureIdx>, feature_dense: &[Option<u16>]) -> TableId {
    idx.and_then(|i| feature_dense[i.0 as usize])
        .map_or(0xffff as TableId, |d| d as TableId)
}
fn write_language(
    lang: Option<&LanguageSystem>,
    feature_dense: &[Option<u16>],
) -> Option<BkBlock> {
    let lang = lang?;
    let mut root: BkBlock = bk_new_block(vec![
        bk_ptr(BkCellType::P16, None),
        bk_int(
            BkCellType::B16,
            (feature_index(lang.required_feature, feature_dense) as i32) as u32,
        ),
        bk_int(BkCellType::B16, (lang.features.len()) as u32),
    ]);
    for &feature in lang.features.iter() {
        bk_push(
            &mut root,
            vec![bk_int(
                BkCellType::B16,
                (feature_index(Some(feature), feature_dense) as i32) as u32,
            )],
        );
    }
    return Some(root);
}
fn write_script(
    dl: Option<&LanguageSystem>,
    ll: &[&LanguageSystem],
    feature_dense: &[Option<u16>],
) -> BkBlock {
    let mut root: BkBlock = bk_new_block(vec![
        bk_ptr(BkCellType::P16, write_language(dl, feature_dense)),
        bk_int(BkCellType::B16, (ll.len()) as u32),
    ]);
    for &lang_sys in ll.iter() {
        let tag: &[u8] = &lang_sys.name[5..9];
        bk_push(
            &mut root,
            vec![
                bk_int(BkCellType::B32, feature_name_to_tag(tag)),
                bk_ptr(BkCellType::P16, write_language(Some(lang_sys), feature_dense)),
            ],
        );
    }
    return root;
}
fn write_otl_script_and_languages(table: &OtlTable, feature_dense: &[Option<u16>]) -> BkBlock {
    // Groups languages by script tag (the first 4 bytes of `language.name`),
    // tracking each script's default (dflt/DFLT) language separately from
    // its other languages, in the order languages are first seen. Unlike
    // every other uthash instance converted so far in this migration, the
    // original C here never calls `HASH_SORT` before its `HASH_ITER` --
    // output order is insertion order, not tag order, so `BTreeMap` (which
    // this migration has used for every prior instance) is the wrong
    // container. A plain `Vec` with a linear "already seen" scan preserves
    // insertion order directly; the number of distinct scripts in a real
    // font is small (typically single digits), so the O(n) scan costs
    // nothing observable -- not worth introducing an `indexmap` dependency
    // for a handful of entries (that crate remains the intended tool for
    // the much larger order-dependent uthash tables noted in RUST_MIGRATION.md).
    //
    // A later language with the same script tag whose name is *also*
    // dflt/DFLT silently overwrites the script's recorded default -- the
    // original never guarded against a second default and neither does
    // this rewrite; not a case this function warns about.
    struct ScriptGroup<'a> {
        tag: Vec<u8>,
        default_language: Option<&'a LanguageSystem>,
        languages: Vec<&'a LanguageSystem>,
    }
    let mut scripts: Vec<ScriptGroup> = Vec::new();
    for language in table.languages.iter() {
        let script_tag: Vec<u8> = language.name[..4].to_vec();
        // Behaviorally identical to the original `strncmp(..., 4)` early-NUL
        // comparison: the compared window never contains an embedded NUL, so
        // direct byte-slice equality can never disagree with `strncmp`'s
        // verdict here.
        let is_default: bool =
            &language.name[5..9] == b"DFLT" || &language.name[5..9] == b"dflt";
        let mut found: Option<usize> = None;
        for (idx, group) in scripts.iter().enumerate() {
            if group.tag == script_tag {
                found = Some(idx);
                break;
            }
        }
        match found {
            Some(idx) => {
                if is_default {
                    scripts[idx].default_language = Some(language);
                } else {
                    scripts[idx].languages.push(language);
                }
            }
            None => {
                if is_default {
                    scripts.push(ScriptGroup {
                        tag: script_tag,
                        default_language: Some(language),
                        languages: Vec::new(),
                    });
                } else {
                    scripts.push(ScriptGroup {
                        tag: script_tag,
                        default_language: None,
                        languages: vec![language],
                    });
                }
            }
        }
    }
    let mut root: BkBlock = bk_new_block(vec![bk_int(BkCellType::B16, (scripts.len()) as u32)]);
    for group in &scripts {
        bk_push(
            &mut root,
            vec![
                bk_int(BkCellType::B32, feature_name_to_tag(&group.tag)),
                bk_ptr(
                    BkCellType::P16,
                    Some(write_script(
                        group.default_language,
                        &group.languages,
                        feature_dense,
                    )),
                ),
            ],
        );
    }
    return root;
}
pub fn build_otl(table: Option<&OtlTable>, tag: &[u8]) -> Option<Buffer> {
    let table: &OtlTable = table?;
    let buf: Option<Buffer>;
    let stage = crate::logger::stage(ByteStr(tag));
    {
        let lookup_dense = storage_to_dense(&table.lookups);
        let feature_dense = storage_to_dense(&table.features);
        let lookups: BkBlock = write_otl_lookups(table, tag);
        let features: BkBlock = write_otl_features(table, &lookup_dense);
        let languages: BkBlock = write_otl_script_and_languages(table, &feature_dense);
        let root: BkBlock = bk_new_block(vec![
            bk_int(BkCellType::B32, 0x10000_u32),
            bk_ptr(BkCellType::P16, Some(languages)),
            bk_ptr(BkCellType::P16, Some(features)),
            bk_ptr(BkCellType::P16, Some(lookups)),
        ]);
        buf = Some(bk_build_block(root));
        stage.finish();
    }
    return buf;
}
