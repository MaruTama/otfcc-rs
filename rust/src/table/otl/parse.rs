#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::logger::{
    LOG_VL_IMPORTANT, LOG_VL_NOTICE, LoggerType, logger_dedent, logger_finish, logger_log_sds,
    logger_start_sds,
};
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
use crate::table::otl::constants::LOOKUP_FLAGS_LABELS;
use crate::table::otl::constants::SCRIPT_LANGUAGE_SEPARATOR;
use crate::table::otl::subtables::chaining::parse::otl_parse_chaining;
use crate::table::otl::subtables::gpos_cursive::otl_gpos_parse_cursive;
use crate::table::otl::subtables::gpos_mark_to_ligature::otl_gpos_parse_mark_to_ligature;
use crate::table::otl::subtables::gpos_mark_to_single::otl_gpos_parse_mark_to_single;
use crate::table::otl::subtables::gpos_pair::otl_gpos_parse_pair;
use crate::table::otl::subtables::gpos_single::otl_gpos_parse_single;
use crate::table::otl::subtables::gsub_ligature::otl_gsub_parse_ligature;
use crate::table::otl::subtables::gsub_multi::otl_gsub_parse_multi;
use crate::table::otl::subtables::gsub_reverse::otl_gsub_parse_reverse;
use crate::table::otl::subtables::gsub_single::otl_gsub_parse_single;
use crate::table::otl::{
    Feature, FeatureRef, FeatureRefList, LanguageSystem, Lookup, LookupRef, LookupRefList,
    LookupType, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_MARK_TO_BASE,
    OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR,
    OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_LIGATURE,
    OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, OtlTable, Subtable,
};
use crate::table::otl::{
    new_feature, new_language, new_lookup, otfcc_delete_lookup, otl_feature_ref_list_dispose,
    otl_feature_ref_list_replace, otl_lookup_ref_list_dispose, otl_lookup_ref_list_replace,
    subtable_list_slot,
};
use crate::vendor::json::JsonType;
/// Replaces the uthash-based `FeatureHash`. Same shape as `LookupEntry`
/// (see its doc comment) and for the same reason: a real feature
/// declaration is rejected if its name already exists, but an alias
/// entry (a JSON string value under `"features"`) only checks that its
/// *target* name exists, never its own -- so this stays a plain `Vec`
/// with reverse (most-recent-wins) search rather than a dedup map,
/// even though the eventual sort key (`by_feature_name`, byte-wise on
/// `name`) happens to equal the would-be dedup key.
pub struct FeatureEntry {
    pub name: Vec<u8>,
    pub alias: bool,
    pub feature: *mut Feature,
}
/// Replaces the uthash-based `LookupHash`. Not a dedup map: `name` is not
/// unique -- real (non-alias) entries are rejected up front by
/// `_declare_lookup_parser`'s own "already exists" check before ever
/// reaching insertion, but an *alias* entry's own name is never checked
/// against existing entries (only its alias *target*'s name is looked
/// up), so two entries can legitimately share a `name`. uthash's
/// bucket-prepend insertion means `HASH_FIND` on a duplicated key always
/// returns the most-recently-inserted match; `.iter().rev().find(...)`
/// reproduces that "most recent wins" lookup exactly, which is why this
/// stays a plain `Vec` (preserving insertion order for that purpose)
/// rather than a `HashMap`/`BTreeMap` keyed by name.
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
    /// treatment, fixed in Rust only (see rust/README.md).
    pub alias: bool,
    pub lookup: *mut Lookup,
    pub order_type: LookupOrderType,
    pub order_val: u16,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u32)]
pub enum LookupOrderType {
    Force = 0,
    File = 1,
}
unsafe fn _parse_lookup(
    lookup: *const ParsedValue,
    lookup_name: &[u8],
    options: &Options,
    lh: &mut Vec<LookupEntry>,
) -> bool {
    let mut parsed: bool = false;
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GSUB_SINGLE,
            Some(otl_gsub_parse_single as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GSUB_MULTIPLE,
            Some(otl_gsub_parse_multi as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GSUB_ALTERNATE,
            Some(otl_gsub_parse_multi as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GSUB_LIGATURE,
            Some(
                otl_gsub_parse_ligature as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable,
            ),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GSUB_CHAINING,
            Some(otl_parse_chaining as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GSUB_REVERSE,
            Some(
                otl_gsub_parse_reverse as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable,
            ),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GPOS_SINGLE,
            Some(otl_gpos_parse_single as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GPOS_PAIR,
            Some(otl_gpos_parse_pair as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GPOS_CURSIVE,
            Some(
                otl_gpos_parse_cursive as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable,
            ),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GPOS_CHAINING,
            Some(otl_parse_chaining as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GPOS_MARK_TO_BASE,
            Some(
                otl_gpos_parse_mark_to_single
                    as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable,
            ),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GPOS_MARK_TO_MARK,
            Some(
                otl_gpos_parse_mark_to_single
                    as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable,
            ),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GPOS_MARK_TO_LIGATURE,
            Some(
                otl_gpos_parse_mark_to_ligature
                    as unsafe fn(*const ParsedValue, &Options) -> *mut Subtable,
            ),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    return parsed;
}
unsafe fn _declare_lookup_parser(
    llt: LookupType,
    parser: Option<unsafe fn(*const ParsedValue, &Options) -> *mut Subtable>,
    _lookup: *const ParsedValue,
    lookup_name: &[u8],
    options: &Options,
    lh: &mut Vec<LookupEntry>,
) -> bool {
    let lv = unsafe { _lookup.as_ref() };
    let type_0 = lv.and_then(|v| v.get_typed(b"type", JsonType::String));
    let matches_type = type_0
        .and_then(ParsedValue::as_str_bytes)
        .is_some_and(|b| b == llt.name().to_bytes());
    if !matches_type {
        if type_0.is_none() {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"Lookup ",
                    lookup_name,
                    b" does not have a valid 'type' field.",
                ),
            );
        }
        return false;
    }
    let name_bytes: Vec<u8> = lookup_name.to_vec();
    if lh.iter().any(|e| e.name == name_bytes) {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"Lookup ", lookup_name, b" already exists."),
        );
        return false;
    }
    let Some(subtables) = lv.and_then(|v| v.get_typed(b"subtables", JsonType::Array)) else {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"Lookup ",
                lookup_name,
                b" does not have a valid subtable list.",
            ),
        );
        return false;
    };
    // Transient owner, same shape as `FeatureHash.feature`: raw here because
    // `LookupHash.lookup` is raw, `Box::into_raw` at construction,
    // `Box::from_raw` either at the rejection path below
    // (`otfcc_delete_lookup`) or at the one non-alias push site far below.
    let lookup: *mut Lookup = Box::into_raw(new_lookup());
    (*lookup).type_0 = llt;
    (*lookup).flags = lv
        .and_then(|v| v.get(b"flags"))
        .map_or(0, |v| v.flags(&LOOKUP_FLAGS_LABELS)) as u16;
    let mark_attachment_type: u16 = lv.map_or(0, |v| v.get_int(b"markAttachmentType")) as u16;
    if mark_attachment_type != 0 {
        (*lookup).flags = ((*lookup).flags as i32
            | (mark_attachment_type as i32) << 8_i32)
            as u16;
    }
    let subtable_items = subtables.as_array().unwrap();
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(lookup_name),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        for _subtable in subtable_items {
            if _subtable.as_object().is_some() {
                let _st: *mut Subtable = parser.expect("non-null function pointer")(
                    _subtable as *const ParsedValue,
                    options,
                );
                (*lookup).subtables.push(subtable_list_slot(_st));
            }
        }
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    if (*lookup).subtables.is_empty() {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"Lookup ", lookup_name, b" does not have any subtables."),
        );
        otfcc_delete_lookup(lookup);
        return false;
    }
    let order_val: u16 = lh.len() as u16;
    (*lookup).name = name_bytes.clone();
    lh.push(LookupEntry {
        name: name_bytes,
        alias: false,
        lookup,
        order_type: LookupOrderType::File,
        order_val,
    });
    return true;
}
unsafe fn figure_out_lookups_from_json(
    lookups: *const ParsedValue,
    options: &Options,
) -> Vec<LookupEntry> {
    let mut lh: Vec<LookupEntry> = Vec::new();
    let Some(fields) = unsafe { lookups.as_ref() }.and_then(ParsedValue::as_object) else {
        return lh;
    };
    for (key, lookup_val) in fields {
        let lookup_name = &key[..key.len() - 1];
        if lookup_val.as_object().is_some() {
            let parsed: bool =
                _parse_lookup(lookup_val as *const ParsedValue, lookup_name, options, &mut lh);
            if !parsed {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[OTFCC-fea] Ignoring invalid or unsupported lookup ",
                        lookup_name,
                        b".\n",
                    ),
                );
            }
        } else if let Some(thatname_bytes) = lookup_val.as_str_bytes() {
            // Alias's own name is never checked against existing entries
            // here (only the alias *target*'s name, `thatname_bytes`, is
            // looked up) -- see `LookupEntry`'s doc comment on why this
            // stays a `Vec` with a reverse (most-recent-wins) search
            // rather than a dedup map.
            let thatname_owned = thatname_bytes.to_vec();
            if let Some(target_lookup) = lh
                .iter()
                .rev()
                .find(|e| e.name == thatname_owned)
                .map(|e| e.lookup)
            {
                let order_val: u16 = lh.len() as u16;
                lh.push(LookupEntry {
                    name: lookup_name.to_vec(),
                    alias: true,
                    lookup: target_lookup,
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
/// The one place in this module that mutates a parsed JSON object in
/// place mid-walk (see the plan doc's Stage 11 investigation): every
/// duplicate `d[k]` whose value equals an earlier `d[j]` (and, when
/// `sametag`, whose key shares `d[j]`'s first 4 bytes) is rewritten in
/// place to alias `d[j]`'s own key, so a later pass over `d` only ever
/// sees one real definition per distinct value.
///
/// `j` and `k` are always different indices (`k` ranges over `j+1..`),
/// so despite mutating `d[k]` while still comparing against `d[j]`, this
/// is a plain sequential read-then-index-write, never two overlapping
/// borrows of the same slot -- confirmed by the investigation before
/// converting this off the old free-function shell, and pinned by
/// `feature_merger_tests` below. Each iteration re-reads `d.as_object()`
/// fresh rather than holding a borrow across the `set_field` call, the
/// same "resolve at the point of use" pattern `libcff/subr.rs`'s
/// `resolve_subr_ref` established for a comparable aliasing shape in
/// Stage 9.
unsafe fn feature_merger_activate(
    d: *mut ParsedValue,
    sametag: bool,
    objtype: *const ::core::ffi::c_char,
    options: &Options,
) {
    let Some(d) = (unsafe { d.as_mut() }) else {
        return;
    };
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
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_NOTICE,
                LoggerType::Info,
                crate::bytesbuild!(
                    b"[OTFCC-fea] Merged duplicate ",
                    objtype,
                    b" '",
                    kthat,
                    b"' into '",
                    kthis,
                    b"'.\n",
                ),
            );
        }
    }
}
unsafe fn figure_out_features_from_json(
    features: *mut ParsedValue,
    lh: &Vec<LookupEntry>,
    tag: *const ::core::ffi::c_char,
    options: &Options,
) -> Vec<FeatureEntry> {
    let mut fh: Vec<FeatureEntry> = Vec::new();
    if options.merge_features {
        feature_merger_activate(
            features,
            true,
            b"feature\0" as *const u8 as *const ::core::ffi::c_char,
            options,
        );
    }
    // `feature_merger_activate` (above) is the only thing that ever
    // mutates `features`'s tree, and it has already returned by the time
    // this shared reborrow is taken -- no interleaving between the write
    // above and the reads below.
    let Some(fields) = unsafe { features.as_ref() }.and_then(ParsedValue::as_object) else {
        return fh;
    };
    for (feature_name_key, feature_val) in fields {
        let feature_name = &feature_name_key[..feature_name_key.len() - 1];
        if let Some(items) = feature_val.as_array() {
            let mut al: LookupRefList = Vec::new();
            for term in items {
                if let Some(term_bytes) = term.as_str_bytes() {
                    let term_owned = term_bytes.to_vec();
                    let item = lh.iter().rev().find(|e| e.name == term_owned);
                    if let Some(item) = item {
                        al.push(item.lookup as LookupRef);
                    } else {
                        logger_log_sds(
                            &mut *options.logger.borrow_mut(),
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(
                                b"Lookup assignment ",
                                term_bytes,
                                b" for feature [",
                                tag,
                                b"/",
                                feature_name,
                                b"] is missing or invalid.",
                            ),
                        );
                    }
                }
            }
            if !al.is_empty() {
                let feature_name_bytes: Vec<u8> = feature_name.to_vec();
                if !fh.iter().any(|e| e.name == feature_name_bytes) {
                    // Transient owner, same shape as `LookupEntry.lookup`:
                    // `Box::into_raw` here, `Box::from_raw` at the one
                    // non-alias push site in `otfcc_parse_otl` -- an alias
                    // entry's copy of the same pointer is never freed on
                    // its own.
                    let feature: *mut Feature = Box::into_raw(new_feature());
                    (*feature).name = feature_name_bytes.clone();
                    otl_lookup_ref_list_replace(&raw mut (*feature).lookups, al);
                    fh.push(FeatureEntry {
                        name: feature_name_bytes,
                        alias: false,
                        feature,
                    });
                } else {
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[OTFCC-fea] Duplicate feature for [",
                            tag,
                            b"/",
                            feature_name,
                            b"]. This feature will be ignored.\n",
                        ),
                    );
                    otl_lookup_ref_list_dispose(&raw mut al);
                }
            } else {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[OTFCC-fea] There is no valid lookup assignments for [",
                        tag,
                        b"/",
                        feature_name,
                        b"]. This feature will be ignored.\n",
                    ),
                );
                otl_lookup_ref_list_dispose(&raw mut al);
            }
        } else if let Some(target_bytes) = feature_val.as_str_bytes() {
            let target_owned = target_bytes.to_vec();
            if let Some(target_feature) = fh
                .iter()
                .rev()
                .find(|e| e.name == target_owned)
                .map(|e| e.feature)
            {
                fh.push(FeatureEntry {
                    name: feature_name.to_vec(),
                    alias: true,
                    feature: target_feature,
                });
            }
        }
    }
    return fh;
}
pub fn is_valid_language_name(name: &[u8]) -> bool {
    return name.len() == 9_usize && name[4] == SCRIPT_LANGUAGE_SEPARATOR as u8;
}
unsafe fn figure_out_languages_from_json(
    languages: *const ParsedValue,
    fh: &Vec<FeatureEntry>,
    tag: *const ::core::ffi::c_char,
    options: &Options,
) -> std::collections::BTreeMap<Vec<u8>, *mut LanguageSystem> {
    let mut sh: std::collections::BTreeMap<Vec<u8>, *mut LanguageSystem> =
        std::collections::BTreeMap::new();
    let Some(fields) = unsafe { languages.as_ref() }.and_then(ParsedValue::as_object) else {
        return sh;
    };
    for (key, language_val) in fields {
        let language_name = &key[..key.len() - 1];
        if is_valid_language_name(language_name) && language_val.as_object().is_some() {
            let mut required_feature: *mut Feature = ::core::ptr::null_mut::<Feature>();
            if let Some(rf_bytes) = language_val
                .get_typed(b"requiredFeature", JsonType::String)
                .and_then(ParsedValue::as_str_bytes)
            {
                let rf_owned = rf_bytes.to_vec();
                if let Some(rf) = fh.iter().rev().find(|e| e.name == rf_owned) {
                    required_feature = rf.feature;
                }
            }
            let mut af: FeatureRefList = Vec::new();
            if let Some(items) = language_val
                .get_typed(b"features", JsonType::Array)
                .and_then(ParsedValue::as_array)
            {
                for term in items {
                    if let Some(term_bytes) = term.as_str_bytes() {
                        let term_owned = term_bytes.to_vec();
                        if let Some(item) = fh.iter().rev().find(|e| e.name == term_owned) {
                            af.push(item.feature as FeatureRef);
                        }
                    }
                }
            }
            if !required_feature.is_null() || !af.is_empty() {
                let language_name_bytes: Vec<u8> = language_name.to_vec();
                if !sh.contains_key(&language_name_bytes) {
                    // Transient owner, same shape as `LookupEntry.lookup`/
                    // `FeatureEntry.feature`: `Box::into_raw` here,
                    // `Box::from_raw` at the one push site in
                    // `otfcc_parse_otl` -- unlike those two, `LanguageHash`
                    // has no alias mechanism at all (no JSON string-value
                    // case is handled for `"languages"`, confirmed by
                    // grep before starting), so every entry here really
                    // is unique and really does get pushed.
                    let language: *mut LanguageSystem = Box::into_raw(new_language());
                    (*language).name = language_name_bytes.clone();
                    (*language).required_feature = required_feature as FeatureRef;
                    otl_feature_ref_list_replace(&raw mut (*language).features, af);
                    sh.insert(language_name_bytes, language);
                } else {
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[OTFCC-fea] Duplicate language item [",
                            tag,
                            b"/",
                            language_name,
                            b"]. This language term will be ignored.\n",
                        ),
                    );
                    otl_feature_ref_list_dispose(&raw mut af);
                }
            } else {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(
                        b"[OTFCC-fea] There is no valid feature assignments for [",
                        tag,
                        b"/",
                        language_name,
                        b"]. This language term will be ignored.\n",
                    ),
                );
                otl_feature_ref_list_dispose(&raw mut af);
            }
        }
    }
    return sh;
}
pub unsafe fn otfcc_parse_otl(
    root: &ParsedValue,
    options: &Options,
    tag: *const ::core::ffi::c_char,
) -> Option<Box<OtlTable>> {
    let otl: *mut OtlTable;
    let mut otl_box: Option<Box<OtlTable>> = None;
    // `table`/`languages`/`features`/`lookups`/`lookup_order` all stay raw
    // pointers here, each resolved via its own fresh, single-expression
    // `.as_ref()` reborrow rather than one `&ParsedValue` binding held
    // across the whole function -- `features` gets mutated in place
    // inside `figure_out_features_from_json` (via `feature_merger_
    // activate`), and a persisted shared reference to any ancestor of
    // that subtree (`table`, ultimately `root`) would still be "live" at
    // that point under Rust's aliasing rules even if never read again.
    // Resolving fresh each time, the same principle `feature_merger_
    // activate` itself uses internally, sidesteps that entirely: every
    // reborrow here is a temporary that retires at the end of its own
    // statement, long before the mutation happens.
    let tag_key = unsafe { ::core::ffi::CStr::from_ptr(tag) }.to_bytes();
    let table: *const ParsedValue = root
        .get_typed(tag_key, JsonType::Object)
        .map_or(::core::ptr::null(), |v| v as *const ParsedValue);
    if !table.is_null() {
        otl_box = Some(Box::new(OtlTable {
            lookups: Vec::new(),
            features: Vec::new(),
            languages: Vec::new(),
        }));
        otl = otl_box.as_mut().unwrap().as_mut() as *mut OtlTable;
        let languages: *const ParsedValue = unsafe { table.as_ref() }
            .and_then(|t| t.get_typed(b"languages", JsonType::Object))
            .map_or(::core::ptr::null(), |v| v as *const ParsedValue);
        let features: *mut ParsedValue = unsafe { table.as_ref() }
            .and_then(|t| t.get_typed(b"features", JsonType::Object))
            .map_or(::core::ptr::null_mut(), |v| {
                v as *const ParsedValue as *mut ParsedValue
            });
        let lookups: *const ParsedValue = unsafe { table.as_ref() }
            .and_then(|t| t.get_typed(b"lookups", JsonType::Object))
            .map_or(::core::ptr::null(), |v| v as *const ParsedValue);
        if !(languages.is_null() || features.is_null() || lookups.is_null()) {
            logger_start_sds(&mut *options.logger.borrow_mut(), crate::bytesbuild!(tag));
            // No longer a `___loggedstep_v`/`current_block`-flagged `loop`
            // simulating "run this block once, then jump past the
            // `logger_finish`+early-return on failure" -- the block below
            // always runs exactly once; the only branch is whether the
            // parsed table came out non-empty. On success, `logger_finish`
            // + `return otl_box` immediately; on failure, `logger_dedent`
            // and fall through to the shared "log a warning, return None"
            // tail below instead.
            let mut lh: Vec<LookupEntry> = figure_out_lookups_from_json(lookups, options);
            let lookup_order: *const ParsedValue = unsafe { table.as_ref() }
                .and_then(|t| t.get_typed(b"lookupOrder", JsonType::Array))
                .map_or(::core::ptr::null(), |v| v as *const ParsedValue);
            if let Some(items) = unsafe { lookup_order.as_ref() }.and_then(ParsedValue::as_array)
            {
                for (j, ln) in items.iter().enumerate() {
                    if let Some(ln_bytes) = ln.as_str_bytes() {
                        let ln_owned = ln_bytes.to_vec();
                        if let Some(item) = lh.iter_mut().rev().find(|e| e.name == ln_owned) {
                            item.order_type = LookupOrderType::Force;
                            item.order_val = j as u16;
                        }
                    }
                }
            }
            let mut fh: Vec<FeatureEntry> =
                figure_out_features_from_json(features, &lh, tag, options);
            let sh: std::collections::BTreeMap<Vec<u8>, *mut LanguageSystem> =
                figure_out_languages_from_json(languages, &fh, tag, options);
            if lh.is_empty() || fh.is_empty() || sh.is_empty() {
                logger_dedent(&mut *options.logger.borrow_mut());
            } else {
                // `lh` is an owned `Vec` now, not a chain of uthash
                // nodes reached via a raw pointer, so there is no
                // manual HASH_ITER+HASH_DEL+free walk here -- sorting
                // by (order_type, order_val) (what `HASH_SORT` with
                // `by_lookup_order` did, deferred from where that call
                // used to sit, right after the lookupOrder loop above,
                // since nothing in between needed `lh` in sorted order,
                // only by-name lookup) and then draining it are both
                // just `Vec` operations, and the `Vec` itself drops
                // for free once this scope ends.
                lh.sort_by(|a, b| {
                    a.order_type
                        .cmp(&b.order_type)
                        .then(a.order_val.cmp(&b.order_val))
                });
                for entry in lh.into_iter() {
                    if !entry.alias {
                        // Takes ownership back from the transient
                        // owner (see the `Box::into_raw`/`new_lookup`
                        // at construction); an alias entry's copy of
                        // the same pointer is never pushed and never
                        // freed on its own -- see `LookupEntry.alias`'s
                        // doc comment.
                        (*otl).lookups.push(Box::from_raw(entry.lookup));
                    }
                }
                // Same shape as `lh` above: `by_feature_name` sorted
                // by `name` (which happened to equal the would-be
                // dedup key, unlike `lh`'s order_type/order_val), so
                // the sort is `.name`'s byte-wise `Ord` -- matching
                // `strcmp` on NUL-free byte sequences, the same
                // equivalence this migration relies on for every
                // `Vec<u8>`-keyed sort (see `ClassNameHash`).
                fh.sort_by(|a, b| a.name.cmp(&b.name));
                for entry in fh.into_iter() {
                    if !entry.alias {
                        // Takes ownership back from the transient
                        // owner (see the `Box::into_raw` above); an
                        // alias entry's copy of the same pointer is
                        // never pushed and never freed on its own.
                        (*otl).features.push(Box::from_raw(entry.feature));
                    }
                }
                for (_, language) in sh.into_iter() {
                    // Takes ownership back from the transient owner
                    // (see the `Box::into_raw` in
                    // `figure_out_languages_from_json`); every entry
                    // reaches this push -- `LanguageHash` has no
                    // alias mechanism to skip.
                    (*otl).languages.push(Box::from_raw(language));
                }
                logger_finish(&mut *options.logger.borrow_mut());
                return otl_box;
            }
        }
    }
    if otl_box.is_some() {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"[OTFCC-fea] Ignoring invalid or incomplete OTL table ",
                tag,
                b".\n",
            ),
        );
    }
    return None;
}

#[cfg(test)]
mod feature_merger_tests {
    use super::*;

    /// Direct coverage for `feature_merger_activate`'s in-place mutation
    /// -- the Stage 11 investigation's "hard case" -- run under Miri to
    /// confirm the re-resolve-fresh design has no aliasing UB, not just
    /// that it produces the right answer natively.
    #[test]
    fn merges_a_later_duplicate_into_the_first_occurrence_when_tags_match() {
        let arr = ParsedValue::Array(vec![ParsedValue::Str(b"a\0".to_vec())]);
        let mut d = ParsedValue::Object(vec![
            (b"test1\0".to_vec(), arr.clone()),
            (b"test2\0".to_vec(), arr),
        ]);
        let options = Options::default();
        unsafe {
            feature_merger_activate(&mut d as *mut ParsedValue, true, c"feature".as_ptr(), &options);
        }
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
        let options = Options::default();
        unsafe {
            feature_merger_activate(&mut d as *mut ParsedValue, true, c"feature".as_ptr(), &options);
        }
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
        let options = Options::default();
        unsafe {
            feature_merger_activate(&mut d as *mut ParsedValue, false, c"lookup".as_ptr(), &options);
        }
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
