
use crate::logger::ByteStr;
use crate::support::options::Options;
use otfcc_json::ParsedValue;
use crate::table::otl::constants::LOOKUP_FLAGS_LABELS;
use crate::table::otl::constants::SCRIPT_LANGUAGE_SEPARATOR;
use crate::table::otl::kind::{LOOKUP_KINDS, LookupKind};
use crate::table::otl::{
    Feature, FeatureIdx, FeatureRefList, LanguageSystem, Lookup, LookupIdx, LookupRefList,
    OtlTable,
};
use crate::table::otl::{new_language, new_lookup};
use otfcc_json::JsonType;
/// A transient identity minted for a not-yet-collected `Lookup`, indexing
/// `PendingLookups.lookups` (position within `LookupEntry.lookup_id`'s own
/// backing store, *not* the final `OtlTable.lookups` position -- `lh` gets
/// sorted and partially drained (aliases are skipped) before that final
/// position exists, see `parse_otl`'s remap). Replaces the old
/// `LookupEntry.lookup: *mut Lookup`: a `Box`'s heap address survives being
/// moved into a `Vec` later with no extra work, which is what let the
/// pointer-based design get away with no remap step; an index does not
/// have that property, so the remap has to be added back explicitly.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct PendingLookupId(u32);
/// Same shape as `PendingLookupId`, for not-yet-collected `Feature`s.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct PendingFeatureId(u32);
/// The not-yet-collected `Feature` a `PendingFeatureId` points at. Not
/// `Feature` itself: `Feature.lookups` is `LookupRefList` (`Vec<LookupIdx>`,
/// *final* indices), but at this point `.lookups` can only hold
/// `PendingLookupId`s -- `lh` hasn't been sorted/drained into
/// `OtlTable.lookups` yet, so no final `LookupIdx` exists.
#[derive(Debug)]
struct PendingFeature {
    name: Vec<u8>,
    lookups: Vec<PendingLookupId>,
}
/// Same bundling shape as `PendingLookups`, for not-yet-collected
/// `Feature`s.
#[derive(Debug)]
struct PendingFeatures {
    entries: Vec<FeatureEntry>,
    features: Vec<Option<PendingFeature>>,
}
/// The not-yet-collected `LanguageSystem` a `figure_out_languages_from_json`
/// entry builds -- unlike `Lookup`/`Feature`, `LanguageSystem` has no
/// alias mechanism (see the comment below) and its own list
/// (`LangSystemList`) is never punch-holed by consolidation, so no
/// transient identity is needed for languages themselves -- only for the
/// `PendingFeatureId`s a language references, which still need remapping
/// to final `FeatureIdx`es once `fh` has been sorted and drained.
#[derive(Debug)]
struct PendingLanguage {
    name: Vec<u8>,
    required_feature: Option<PendingFeatureId>,
    features: Vec<PendingFeatureId>,
}
/// A feature declared in JSON. A real feature declaration is rejected if
/// its name already exists, but an alias entry (a JSON string value under
/// `"features"`) only checks that its *target* name exists, never its own
/// -- so this is a plain `Vec` searched most-recent-first, not a dedup map.
#[derive(Debug)]
pub struct FeatureEntry {
    pub name: Vec<u8>,
    pub alias: bool,
    feature_id: PendingFeatureId,
}
/// A lookup declared in JSON. `name` is not unique -- real (non-alias)
/// entries are rejected up front by `_declare_lookup_parser`'s own "already
/// exists" check, but an *alias* entry's own name is never checked against
/// existing entries (only its alias *target*'s name is looked up), so two
/// entries can share a `name`. Lookups by name search most-recent-first
/// (`.iter().rev().find(...)`), so the latest match wins.
#[derive(Debug)]
pub struct LookupEntry {
    pub name: Vec<u8>,
    /// Rust-only field, not present in `c/`'s `lookup_hash` -- the C
    /// original has this exact same "alias" shape (a JSON string value in
    /// `"lookups"` makes a second hash node share `.lookup` with an existing
    /// one) but no flag to tell the two apart, so its final drain loop
    /// pushes *every* node's `.lookup` into `otl->lookups`, including the
    /// alias's copy of an already-pushed pointer. `otl_iLookupList.dispose`
    /// then frees that pointer twice. Confirmed with a synthetic payload:
    /// segfaults C's otfccbuild, hangs Rust's pre-`Box` baseline. `Feature`'s
    /// parallel alias path (`FeatureHash.alias`, just above) already skips
    /// the push for its alias node -- this field gives `Lookup` the same
    /// treatment, fixed in Rust only (see RUST_MIGRATION.md).
    pub alias: bool,
    lookup_id: PendingLookupId,
    pub order_type: LookupOrderType,
    pub order_val: u16,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u32)]
pub enum LookupOrderType {
    Force = 0,
    File = 1,
}
/// The lookups read from the JSON, not yet in `OtlTable.lookups`, with the
/// name, alias and order used to resolve, sort and collect them (see
/// `PendingLookupId`).
#[derive(Debug)]
struct PendingLookups {
    entries: Vec<LookupEntry>,
    lookups: Vec<Option<Box<Lookup>>>,
}
fn _parse_lookup(
    lookup: Option<&ParsedValue>,
    lookup_name: &[u8],
    lh: &mut PendingLookups,
) -> bool {
    // Tries every kind in turn, even after the first one has warned about a
    // missing type: the warning is repeated once per kind, as it always was.
    LOOKUP_KINDS
        .iter()
        .any(|&kind| _declare_lookup_parser(kind, lookup, lookup_name, lh))
}
fn _declare_lookup_parser(
    kind: &dyn LookupKind,
    _lookup: Option<&ParsedValue>,
    lookup_name: &[u8],
    lh: &mut PendingLookups,
) -> bool {
    let lv = _lookup;
    let llt = kind.lookup_type();
    let type_name = lv.and_then(|v| v.get_typed(b"type", JsonType::String));
    let matches_type = type_name
        .and_then(ParsedValue::as_str_bytes)
        .is_some_and(|b| b == llt.name().as_bytes());
    if !matches_type {
        if type_name.is_none() {
            tracing::warn!("Lookup {} does not have a valid 'type' field.", ByteStr(lookup_name));
        }
        return false;
    }
    let name_bytes: Vec<u8> = lookup_name.to_vec();
    if lh.entries.iter().any(|e| e.name == name_bytes) {
        tracing::warn!("Lookup {} already exists.", ByteStr(lookup_name));
        return false;
    }
    let Some(subtables) = lv.and_then(|v| v.get_typed(b"subtables", JsonType::Array)) else {
        tracing::warn!("Lookup {} does not have a valid subtable list.", ByteStr(lookup_name));
        return false;
    };
    // Built as a local owned value, not `Box::into_raw`'d until the very
    // end (mirrors classdef.rs's/coverage.rs's read_class_def/
    // read_coverage restructuring) -- the rejection path just lets `lookup`
    // drop naturally instead of an explicit `otfcc_delete_lookup` call
    // (that function's own body is exactly `drop(Box::from_raw(...))`).
    // `LookupEntry.lookup` itself stays `*mut Lookup`: a transient owner
    // handed off at the one non-alias push site in `parse_otl`.
    let mut lookup: Box<Lookup> = new_lookup();
    lookup.lookup_type = llt;
    lookup.flags = lv
        .and_then(|v| v.get(b"flags"))
        .map_or(0, |v| v.flags(&LOOKUP_FLAGS_LABELS)) as u16;
    let mark_attachment_type: u16 = lv.map_or(0, |v| v.get_int(b"markAttachmentType")) as u16;
    if mark_attachment_type != 0 {
        lookup.flags = (lookup.flags as i32 | (mark_attachment_type as i32) << 8_i32) as u16;
    }
    let subtable_items = subtables.as_array().unwrap();
    let stage = crate::logger::stage(ByteStr(lookup_name));
    {
        for _subtable in subtable_items {
            if _subtable.as_object().is_some() {
                let st = kind.parse_subtable(Some(_subtable));
                lookup.subtables.push(st.map(Box::new));
            }
        }
        stage.finish();
    }
    if lookup.subtables.is_empty() {
        tracing::warn!("Lookup {} does not have any subtables.", ByteStr(lookup_name));
        return false;
    }
    let order_val: u16 = lh.entries.len() as u16;
    lookup.name = name_bytes.clone();
    let lookup_id = PendingLookupId(lh.lookups.len() as u32);
    lh.lookups.push(Some(lookup));
    lh.entries.push(LookupEntry {
        name: name_bytes,
        alias: false,
        lookup_id,
        order_type: LookupOrderType::File,
        order_val,
    });
    return true;
}
fn figure_out_lookups_from_json(lookups: Option<&ParsedValue>) -> PendingLookups {
    let mut lh = PendingLookups {
        entries: Vec::new(),
        lookups: Vec::new(),
    };
    let Some(fields) = lookups.and_then(ParsedValue::as_object) else {
        return lh;
    };
    for (key, lookup_val) in fields {
        let lookup_name = &key[..key.len() - 1];
        if lookup_val.as_object().is_some() {
            let parsed: bool = _parse_lookup(Some(lookup_val), lookup_name, &mut lh);
            if !parsed {
                tracing::warn!("[OTFCC-fea] Ignoring invalid or unsupported lookup {}.\n", ByteStr(lookup_name));
            }
        } else if let Some(thatname_bytes) = lookup_val.as_str_bytes() {
            // Alias's own name is never checked against existing entries
            // here (only the alias *target*'s name, `thatname_bytes`, is
            // looked up) -- see `LookupEntry`'s doc comment on why this
            // stays a `Vec` with a reverse (most-recent-wins) search
            // rather than a dedup map. The alias entry copies the
            // target's `lookup_id` rather than minting a new one -- the
            // same "never owns" invariant `LookupEntry.lookup`'s old
            // pointer-sharing had, just spelled as a shared index.
            let thatname_owned = thatname_bytes.to_vec();
            if let Some(target_lookup_id) = lh
                .entries
                .iter()
                .rev()
                .find(|e| e.name == thatname_owned)
                .map(|e| e.lookup_id)
            {
                let order_val: u16 = lh.entries.len() as u16;
                lh.entries.push(LookupEntry {
                    name: lookup_name.to_vec(),
                    alias: true,
                    lookup_id: target_lookup_id,
                    order_type: LookupOrderType::File,
                    order_val,
                });
            }
        }
    }
    return lh;
}
/// Replicates `strncmp(a, b, 4) == 0` for two of `ParsedValue`'s own
/// NUL-terminated key buffers (each carrying exactly one trailing NUL,
/// never an embedded one -- see `ParsedValue`'s own doc comment): compares
/// at most 4 bytes, stopping as soon as either side's NUL is reached,
/// matching `strncmp`'s own early termination on a shorter string.
fn tag4_matches(a: &[u8], b: &[u8]) -> bool {
    for i in 0..4 {
        let ac = a.get(i).copied().unwrap_or(0);
        let bc = b.get(i).copied().unwrap_or(0);
        if ac != bc {
            return false;
        }
        if ac == 0 {
            break;
        }
    }
    true
}
/// Rewrites, in place, every later `d[k]` whose value equals an earlier
/// `d[j]` (and, when `sametag`, whose key shares `d[j]`'s first 4 bytes)
/// into an alias of `d[j]`'s key, so a later pass sees one definition per
/// distinct value. `k > j` always, and each step re-reads `d` before
/// writing, so no two borrows overlap (`feature_merger_tests` runs this
/// under Miri).
fn feature_merger_activate(d: &mut ParsedValue, sametag: bool, objtype: &[u8]) {
    let n = match d.as_object() {
        Some(fields) => fields.len(),
        None => return,
    };
    for j in 0..n {
        let jthis_is_container = matches!(
            d.as_object().unwrap()[j].1,
            ParsedValue::Array(_) | ParsedValue::Object(_)
        );
        if !jthis_is_container {
            continue;
        }
        for k in (j + 1)..n {
            let matched = {
                let fields = d.as_object().unwrap();
                fields[j].1 == fields[k].1
                    && (!sametag || tag4_matches(&fields[j].0, &fields[k].0))
            };
            if !matched {
                continue;
            }
            let kthis_bytes = d.as_object().unwrap()[j].0.clone();
            let mut alias_str: Vec<u8> = kthis_bytes[..kthis_bytes.len() - 1].to_vec();
            alias_str.push(0);
            d.set_field(k, ParsedValue::Str(alias_str));
            let fields = d.as_object().unwrap();
            let kthis = &fields[j].0;
            let kthat = &fields[k].0;
            tracing::info!("[OTFCC-fea] Merged duplicate {} '{}' into '{}'.\n", ByteStr(objtype), ByteStr(kthat), ByteStr(kthis));
        }
    }
}
fn figure_out_features_from_json(
    features: &mut ParsedValue,
    lh: &PendingLookups,
    tag: &[u8],
    options: &Options,
) -> PendingFeatures {
    let mut fh = PendingFeatures {
        entries: Vec::new(),
        features: Vec::new(),
    };
    if options.merge_features {
        feature_merger_activate(features, true, b"feature");
    }
    // `feature_merger_activate` (above) is the only thing that ever
    // mutates `features`'s tree, and it has already returned by the time
    // this shared reborrow is taken -- no interleaving between the write
    // above and the reads below.
    let Some(fields) = features.as_object() else {
        return fh;
    };
    for (feature_name_key, feature_val) in fields {
        let feature_name = &feature_name_key[..feature_name_key.len() - 1];
        if let Some(items) = feature_val.as_array() {
            let mut al: Vec<PendingLookupId> = Vec::new();
            for term in items {
                if let Some(term_bytes) = term.as_str_bytes() {
                    let term_owned = term_bytes.to_vec();
                    let item = lh.entries.iter().rev().find(|e| e.name == term_owned);
                    if let Some(item) = item {
                        al.push(item.lookup_id);
                    } else {
                        tracing::warn!("Lookup assignment {} for feature [{}/{}] is missing or invalid.", ByteStr(term_bytes), ByteStr(tag), ByteStr(feature_name));
                    }
                }
            }
            if !al.is_empty() {
                let feature_name_bytes: Vec<u8> = feature_name.to_vec();
                if !fh.entries.iter().any(|e| e.name == feature_name_bytes) {
                    // Built as a local owned value, only actually
                    // allocated into `OtlTable.features` at the very end
                    // (`parse_otl`'s remap, once `fh` has been
                    // sorted and this pending feature's `PendingLookupId`s
                    // can be rewritten into final `LookupIdx`es); an alias
                    // entry's copy of the same `feature_id` is never
                    // finalized on its own -- see `LookupEntry.alias`'s
                    // doc comment for the shared reason.
                    let feature_id = PendingFeatureId(fh.features.len() as u32);
                    fh.features.push(Some(PendingFeature {
                        name: feature_name_bytes.clone(),
                        lookups: al,
                    }));
                    fh.entries.push(FeatureEntry {
                        name: feature_name_bytes,
                        alias: false,
                        feature_id,
                    });
                } else {
                    tracing::warn!("[OTFCC-fea] Duplicate feature for [{}/{}]. This feature will be ignored.\n", ByteStr(tag), ByteStr(feature_name));
                }
            } else {
                tracing::warn!("[OTFCC-fea] There is no valid lookup assignments for [{}/{}]. This feature will be ignored.\n", ByteStr(tag), ByteStr(feature_name));
            }
        } else if let Some(target_bytes) = feature_val.as_str_bytes() {
            let target_owned = target_bytes.to_vec();
            if let Some(target_feature_id) = fh
                .entries
                .iter()
                .rev()
                .find(|e| e.name == target_owned)
                .map(|e| e.feature_id)
            {
                fh.entries.push(FeatureEntry {
                    name: feature_name.to_vec(),
                    alias: true,
                    feature_id: target_feature_id,
                });
            }
        }
    }
    return fh;
}
pub fn is_valid_language_name(name: &[u8]) -> bool {
    return name.len() == 9_usize && name[4] == SCRIPT_LANGUAGE_SEPARATOR;
}
fn figure_out_languages_from_json(
    languages: Option<&ParsedValue>,
    fh: &PendingFeatures,
    tag: &[u8],
) -> std::collections::BTreeMap<Vec<u8>, PendingLanguage> {
    let mut sh: std::collections::BTreeMap<Vec<u8>, PendingLanguage> =
        std::collections::BTreeMap::new();
    let Some(fields) = languages.and_then(ParsedValue::as_object) else {
        return sh;
    };
    for (key, language_val) in fields {
        let language_name = &key[..key.len() - 1];
        if is_valid_language_name(language_name) && language_val.as_object().is_some() {
            let mut required_feature: Option<PendingFeatureId> = None;
            if let Some(rf_bytes) = language_val
                .get_typed(b"requiredFeature", JsonType::String)
                .and_then(ParsedValue::as_str_bytes)
            {
                let rf_owned = rf_bytes.to_vec();
                if let Some(rf) = fh.entries.iter().rev().find(|e| e.name == rf_owned) {
                    required_feature = Some(rf.feature_id);
                }
            }
            let mut af: Vec<PendingFeatureId> = Vec::new();
            if let Some(items) = language_val
                .get_typed(b"features", JsonType::Array)
                .and_then(ParsedValue::as_array)
            {
                for term in items {
                    if let Some(term_bytes) = term.as_str_bytes() {
                        let term_owned = term_bytes.to_vec();
                        if let Some(item) = fh.entries.iter().rev().find(|e| e.name == term_owned)
                        {
                            af.push(item.feature_id);
                        }
                    }
                }
            }
            if required_feature.is_some() || !af.is_empty() {
                let language_name_bytes: Vec<u8> = language_name.to_vec();
                if !sh.contains_key(&language_name_bytes) {
                    // Built as a local owned value, only actually
                    // allocated into `OtlTable.languages` at the very end
                    // (`parse_otl`'s remap) -- unlike
                    // `LookupEntry`/`FeatureEntry`, `LanguageHash` has no
                    // alias mechanism at all (no JSON string-value case is
                    // handled for `"languages"`, confirmed by grep before
                    // starting), so every entry here really is unique and
                    // really does get pushed.
                    sh.insert(
                        language_name_bytes.clone(),
                        PendingLanguage {
                            name: language_name_bytes,
                            required_feature,
                            features: af,
                        },
                    );
                } else {
                    tracing::warn!("[OTFCC-fea] Duplicate language item [{}/{}]. This language term will be ignored.\n", ByteStr(tag), ByteStr(language_name));
                }
            } else {
                tracing::warn!("[OTFCC-fea] There is no valid feature assignments for [{}/{}]. This language term will be ignored.\n", ByteStr(tag), ByteStr(language_name));
            }
        }
    }
    return sh;
}
/// Reads the `tag` table (`"GSUB"`/`"GPOS"`) from `root`. `lookups` and
/// `lookupOrder` are read first, into a `PendingLookups`; then `features`
/// is rewritten in place (`feature_merger_activate`); then `languages` is
/// read. Nothing is parsed, and no lookup is warned about, unless all three
/// of `lookups`, `features` and `languages` are present.
pub fn parse_otl(root: &mut ParsedValue, options: &Options, tag: &[u8]) -> Option<Box<OtlTable>> {
    let table = root.get_typed_mut(tag, JsonType::Object)?;
    let mut otl_box: Box<OtlTable> = Box::new(OtlTable {
        lookups: Vec::new(),
        features: Vec::new(),
        languages: Vec::new(),
    });
    let languages_present = table.get_typed(b"languages", JsonType::Object).is_some();
    let features_present = table.get_typed(b"features", JsonType::Object).is_some();
    let lookups_present = table.get_typed(b"lookups", JsonType::Object).is_some();
    if languages_present && features_present && lookups_present {
        let stage = crate::logger::stage(ByteStr(tag));
        let mut lh: PendingLookups = figure_out_lookups_from_json(
            table.get_typed(b"lookups", JsonType::Object),
        );
        if let Some(items) = table
            .get_typed(b"lookupOrder", JsonType::Array)
            .and_then(ParsedValue::as_array)
        {
            for (j, ln) in items.iter().enumerate() {
                if let Some(ln_bytes) = ln.as_str_bytes() {
                    let ln_owned = ln_bytes.to_vec();
                    if let Some(item) = lh.entries.iter_mut().rev().find(|e| e.name == ln_owned)
                    {
                        item.order_type = LookupOrderType::Force;
                        item.order_val = j as u16;
                    }
                }
            }
        }
        let mut fh: PendingFeatures = figure_out_features_from_json(
            // `features_present` above already confirmed this is `Some`.
            table
                .get_typed_mut(b"features", JsonType::Object)
                .expect("features_present confirmed this child exists and is an object"),
            &lh,
            tag,
            options,
        );
        let sh: std::collections::BTreeMap<Vec<u8>, PendingLanguage> =
            figure_out_languages_from_json(
                table.get_typed(b"languages", JsonType::Object),
                &fh,
                tag,
            );
        if lh.entries.is_empty() || fh.entries.is_empty() || sh.is_empty() {
            // Nothing usable: close the stage without a `Finish` line.
            drop(stage);
        } else {
            // Sort by (order_type, order_val), then take the lookups out in
            // that order.
            lh.entries.sort_by(|a, b| {
                a.order_type
                    .cmp(&b.order_type)
                    .then(a.order_val.cmp(&b.order_val))
            });
            // The `PendingLookupId`s held by features and language systems
            // were given out before this sort; map each to the lookup's final
            // index. Only a non-alias entry takes its id's slot, so no slot is
            // taken twice.
            let mut lookup_remap: Vec<Option<LookupIdx>> = vec![None; lh.lookups.len()];
            for entry in lh.entries.into_iter() {
                if !entry.alias {
                    let taken = lh.lookups[entry.lookup_id.0 as usize]
                        .take()
                        .expect("non-alias LookupEntry's pending lookup should not have been taken yet");
                    // `otl_box` is accessed directly here (and at the
                    // `features`/`languages` push sites below) rather
                    // than through a cached `otl: *mut OtlTable` local
                    // re-derived at each site, the way the pre-M-33
                    // code did: `otl_box` is now a plain owned
                    // `Box<OtlTable>`, not reached through any
                    // aliasing concern of its own, so there is nothing
                    // a cached pointer (or a fresh reborrow standing
                    // in for one) would buy here beyond what a plain
                    // field access already gives for free.
                    let idx = LookupIdx(otl_box.lookups.len() as u32);
                    otl_box.lookups.push(Some(taken));
                    lookup_remap[entry.lookup_id.0 as usize] = Some(idx);
                }
            }
            // Same shape as `lh.entries` above: `by_feature_name`
            // sorted by `name` (which happened to equal the would-be
            // dedup key, unlike `lh`'s order_type/order_val), so the
            // sort is `.name`'s byte-wise `Ord` -- matching `strcmp`
            // on NUL-free byte sequences, the same equivalence this
            // migration relies on for every `Vec<u8>`-keyed sort (see
            // `ClassNameHash`). Each finalized `Feature`'s `.lookups`
            // is rewritten through `lookup_remap` in the same pass
            // that finalizes it, since a `PendingFeature`'s
            // `PendingLookupId`s are only meaningful before this
            // point.
            fh.entries.sort_by(|a, b| a.name.cmp(&b.name));
            let mut feature_remap: Vec<Option<FeatureIdx>> = vec![None; fh.features.len()];
            for entry in fh.entries.into_iter() {
                if !entry.alias {
                    let pending = fh.features[entry.feature_id.0 as usize]
                        .take()
                        .expect("non-alias FeatureEntry's pending feature should not have been taken yet");
                    let remapped_lookups: LookupRefList = pending
                        .lookups
                        .iter()
                        .map(|pending_id| {
                            lookup_remap[pending_id.0 as usize]
                                .expect("every PendingLookupId a feature references should have been remapped")
                        })
                        .collect();
                    let idx = FeatureIdx(otl_box.features.len() as u32);
                    otl_box.features.push(Some(Box::new(Feature {
                        name: pending.name,
                        lookups: remapped_lookups,
                    })));
                    feature_remap[entry.feature_id.0 as usize] = Some(idx);
                }
            }
            // `LanguageHash` has no alias mechanism at all (see
            // `figure_out_languages_from_json`'s own comment), so
            // every entry here really is unique and really does get
            // pushed -- each `PendingLanguage`'s `PendingFeatureId`s
            // are remapped through `feature_remap` the same way
            // `Feature.lookups` was above.
            for (_, language) in sh.into_iter() {
                let required_feature = language.required_feature.map(|pending_id| {
                    feature_remap[pending_id.0 as usize]
                        .expect("every PendingFeatureId a language references should have been remapped")
                });
                let features: FeatureRefList = language
                    .features
                    .iter()
                    .map(|pending_id| {
                        feature_remap[pending_id.0 as usize]
                            .expect("every PendingFeatureId a language references should have been remapped")
                    })
                    .collect();
                let mut language_box: Box<LanguageSystem> = new_language();
                language_box.name = language.name;
                language_box.required_feature = required_feature;
                language_box.features = features;
                otl_box.languages.push(language_box);
            }
            stage.finish();
            return Some(otl_box);
        }
    }
    tracing::warn!("[OTFCC-fea] Ignoring invalid or incomplete OTL table {}.\n", ByteStr(tag));
    None
}

#[cfg(test)]
mod feature_merger_tests {
    use super::*;

    /// Runs `feature_merger_activate`'s in-place rewrite (under Miri too).
    #[test]
    fn merges_a_later_duplicate_into_the_first_occurrence_when_tags_match() {
        let arr = ParsedValue::Array(vec![ParsedValue::Str(b"a\0".to_vec())]);
        let mut d = ParsedValue::Object(vec![
            (b"test1\0".to_vec(), arr.clone()),
            (b"test2\0".to_vec(), arr),
        ]);
        feature_merger_activate(&mut d, true, b"feature");
        let fields = d.as_object().unwrap();
        assert_eq!(fields[0].0, b"test1\0");
        assert_eq!(
            fields[0].1,
            ParsedValue::Array(vec![ParsedValue::Str(b"a\0".to_vec())])
        );
        // The duplicate's own key is untouched -- only its value becomes
        // an alias string naming the entry it duplicated.
        assert_eq!(fields[1].0, b"test2\0");
        assert_eq!(fields[1].1, ParsedValue::Str(b"test1\0".to_vec()));
    }

    #[test]
    fn does_not_merge_when_sametag_requires_a_matching_4_byte_prefix() {
        let arr = ParsedValue::Array(vec![ParsedValue::Str(b"a\0".to_vec())]);
        let mut d = ParsedValue::Object(vec![
            (b"aaaa1\0".to_vec(), arr.clone()),
            (b"bbbb1\0".to_vec(), arr),
        ]);
        feature_merger_activate(&mut d, true, b"feature");
        let fields = d.as_object().unwrap();
        // Neither entry is an alias: the first 4 bytes ("aaaa" vs "bbbb")
        // never match, so `tag4_matches` rejects every candidate pair.
        assert_eq!(
            fields[1].1,
            ParsedValue::Array(vec![ParsedValue::Str(b"a\0".to_vec())])
        );
    }

    #[test]
    fn merges_regardless_of_key_when_sametag_is_false() {
        let arr = ParsedValue::Array(vec![ParsedValue::Str(b"a\0".to_vec())]);
        let mut d = ParsedValue::Object(vec![
            (b"aaaa1\0".to_vec(), arr.clone()),
            (b"bbbb1\0".to_vec(), arr),
        ]);
        feature_merger_activate(&mut d, false, b"lookup");
        let fields = d.as_object().unwrap();
        assert_eq!(fields[1].1, ParsedValue::Str(b"aaaa1\0".to_vec()));
    }

    #[test]
    fn tag4_matches_stops_at_the_shorter_side_nul() {
        assert!(tag4_matches(b"ab\0", b"ab\0cd"));
        assert!(!tag4_matches(b"abcd\0", b"abce\0"));
        assert!(tag4_matches(b"abcd\0", b"abcd\0"));
    }
}
