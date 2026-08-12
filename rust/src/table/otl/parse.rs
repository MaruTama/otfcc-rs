#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{strcmp, strncmp};





use crate::support::json_funcs::{json_arr_at, json_arr_len, json_obj_get, json_obj_get_type, json_obj_getint, json_obj_key_at, json_obj_key_len_at, json_obj_len, json_obj_val_at, json_str_ptr};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, LOG_VL_NOTICE, ILogger};
use crate::support::options::{Options};
use crate::support::primitives::{TableId};
use crate::vendor::json::{JsonValue, JsonType};
use crate::support::{TRUE_0};
use crate::table::otl::{Feature, FeatureRef, FeatureRefList, LanguageSystem, Lookup, LookupRef, LookupRefList, LookupType, Subtable, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, OtlTable};
use crate::support::json_funcs::otfcc_parse_flags;
use crate::table::otl::constants::{LOOKUP_FLAGS_LABELS};
use crate::support::json_ident::{json_ident};
use crate::table::otl::{otfcc_delete_lookup, otl_feature_ref_list_dispose, otl_feature_ref_list_replace, otl_lookup_ref_list_dispose, otl_lookup_ref_list_replace, subtable_list_slot, new_feature, new_language, new_lookup};
use crate::table::otl::constants::{SCRIPT_LANGUAGE_SEPARATOR};
use crate::table::otl::subtables::chaining::parse::{otl_parse_chaining};
use crate::table::otl::subtables::gpos_cursive::{otl_gpos_parse_cursive};
use crate::table::otl::subtables::gpos_mark_to_ligature::{otl_gpos_parse_mark_to_ligature};
use crate::table::otl::subtables::gpos_mark_to_single::{otl_gpos_parse_mark_to_single};
use crate::table::otl::subtables::gpos_pair::{otl_gpos_parse_pair};
use crate::table::otl::subtables::gpos_single::{otl_gpos_parse_single};
use crate::table::otl::subtables::gsub_ligature::{otl_gsub_parse_ligature};
use crate::table::otl::subtables::gsub_multi::{otl_gsub_parse_multi};
use crate::table::otl::subtables::gsub_reverse::{otl_gsub_parse_reverse};
use crate::table::otl::subtables::gsub_single::{otl_gsub_parse_single};
use crate::vendor::json::{json_value_free};
use crate::vendor::json_builder::{json_string_new_length};
use crate::vendor::sds::{sdsempty};
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
unsafe extern "C" fn _parse_lookup(
    mut lookup: *mut JsonValue,
    mut lookup_name: *mut ::core::ffi::c_char,
    mut options: *const Options,
    lh: &mut Vec<LookupEntry>,
) -> bool {
    let mut parsed: bool = false;
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GSUB_SINGLE,
            Some(
                otl_gsub_parse_single
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
            ),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GSUB_MULTIPLE,
            Some(
                otl_gsub_parse_multi
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
            ),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GSUB_ALTERNATE,
            Some(
                otl_gsub_parse_multi
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
            ),
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
                otl_gsub_parse_ligature
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
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
            Some(
                otl_parse_chaining
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
            ),
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
                otl_gsub_parse_reverse
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
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
            Some(
                otl_gpos_parse_single
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
            ),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    if !parsed {
        parsed = _declare_lookup_parser(
            OTL_TYPE_GPOS_PAIR,
            Some(
                otl_gpos_parse_pair
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
            ),
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
                otl_gpos_parse_cursive
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
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
            Some(
                otl_parse_chaining
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
            ),
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
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
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
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
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
                    as unsafe extern "C" fn(
                        *const JsonValue,
                        *const Options,
                    ) -> *mut Subtable,
            ),
            lookup,
            lookup_name,
            options,
            lh,
        );
    }
    return parsed;
}
unsafe extern "C" fn _declare_lookup_parser(
    mut llt: LookupType,
    mut parser: Option<
        unsafe extern "C" fn(*const JsonValue, *const Options) -> *mut Subtable,
    >,
    mut _lookup: *mut JsonValue,
    mut lookup_name: *mut ::core::ffi::c_char,
    mut options: *const Options,
    lh: &mut Vec<LookupEntry>,
) -> bool {
    let mut type_0: *mut JsonValue = json_obj_get_type(
        _lookup,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::String,
    );
    if type_0.is_null() || strcmp(json_str_ptr(type_0), llt.name().as_ptr()) != 0 {
        if type_0.is_null() {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"Lookup ",
                    lookup_name,
                    b" does not have a valid 'type' field.",
                ),
            );
        }
        return false;
    }
    let name_bytes: Vec<u8> = ::core::ffi::CStr::from_ptr(lookup_name).to_bytes().to_vec();
    if lh.iter().any(|e| e.name == name_bytes) {
        (*(*options).logger)
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::sdsbuild!(sdsempty(), b"Lookup ", lookup_name, b" already exists."),
        );
        return false;
    }
    let mut _subtables: *mut JsonValue = json_obj_get_type(
        _lookup,
        b"subtables\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _subtables.is_null() {
        (*(*options).logger)
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::sdsbuild!(
                sdsempty(),
                b"Lookup ",
                lookup_name,
                b" does not have a valid subtable list.",
            ),
        );
        return false;
    }
    // Transient owner, same shape as `FeatureHash.feature`: raw here because
    // `LookupHash.lookup` is raw, `Box::into_raw` at construction,
    // `Box::from_raw` either at the rejection path below
    // (`otfcc_delete_lookup`) or at the one non-alias push site far below.
    let lookup: *mut Lookup = Box::into_raw(new_lookup());
    (*lookup).type_0 = llt;
    (*lookup).flags = otfcc_parse_flags(
        json_obj_get(
            _lookup,
            b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        ),
        &LOOKUP_FLAGS_LABELS,
    ) as u16;
    let mut mark_attachment_type: u16 = json_obj_getint(
        _lookup,
        b"markAttachmentType\0" as *const u8 as *const ::core::ffi::c_char,
    ) as u16;
    if mark_attachment_type != 0 {
        (*lookup).flags = ((*lookup).flags as ::core::ffi::c_int
            | (mark_attachment_type as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as u16;
    }
    let mut subtable_count: TableId = json_arr_len(_subtables) as TableId;
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), lookup_name),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_int) < subtable_count as ::core::ffi::c_int {
            let mut _subtable: *mut JsonValue = json_arr_at(_subtables, j as u32);
            if !_subtable.is_null()
                && (*_subtable).type_0 == JsonType::Object
            {
                let mut _st: *mut Subtable =
                    parser.expect("non-null function pointer")(_subtable, options);
                (*lookup).subtables.push(subtable_list_slot(_st));
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    if (*lookup).subtables.is_empty() {
        (*(*options).logger)
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::sdsbuild!(sdsempty(), b"Lookup ", lookup_name, b" does not have any subtables."),
        );
        otfcc_delete_lookup(lookup);
        return false;
    }
    let order_val: u16 = lh.len() as u16;
    (*lookup).name = ::core::ffi::CStr::from_ptr(lookup_name).to_bytes().to_vec();
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
    mut lookups: *mut JsonValue,
    mut options: *const Options,
) -> Vec<LookupEntry> {
    let mut lh: Vec<LookupEntry> = Vec::new();
    let mut j: u32 = 0 as u32;
    while j < json_obj_len(lookups) as u32 {
        let mut lookup_name: *mut ::core::ffi::c_char = json_obj_key_at(lookups, j as u32);
        let lookup_val = json_obj_val_at(lookups, j as u32);
        if (*lookup_val).type_0 == JsonType::Object
        {
            let mut parsed: bool = _parse_lookup(
                lookup_val,
                lookup_name,
                options,
                &mut lh,
            );
            if !parsed {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[OTFCC-fea] Ignoring invalid or unsupported lookup ",
                        lookup_name,
                        b".\n",
                    ),
                );
            }
        } else if (*lookup_val).type_0
            as ::core::ffi::c_uint
            == JsonType::String as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut thatname: *mut ::core::ffi::c_char = json_str_ptr(lookup_val);
            // Alias's own name is never checked against existing entries
            // here (only the alias *target*'s name, `thatname`, is looked
            // up) -- see `LookupEntry`'s doc comment on why this stays a
            // `Vec` with a reverse (most-recent-wins) search rather than
            // a dedup map.
            let thatname_bytes: Vec<u8> = ::core::ffi::CStr::from_ptr(thatname).to_bytes().to_vec();
            if let Some(target_lookup) = lh.iter().rev().find(|e| e.name == thatname_bytes).map(|e| e.lookup) {
                let order_val: u16 = lh.len() as u16;
                lh.push(LookupEntry {
                    name: ::core::ffi::CStr::from_ptr(lookup_name).to_bytes().to_vec(),
                    alias: true,
                    lookup: target_lookup,
                    order_type: LookupOrderType::File,
                    order_val,
                });
            }
        }
        j = j.wrapping_add(1);
    }
    return lh;
}
unsafe extern "C" fn feature_merger_activate(
    mut d: *mut JsonValue,
    sametag: bool,
    mut objtype: *const ::core::ffi::c_char,
    mut options: *const Options,
) {
    let mut j: u32 = 0 as u32;
    while j < json_obj_len(d) as u32 {
        let mut jthis: *mut JsonValue = json_obj_val_at(d, j as u32);
        let mut kthis: *mut ::core::ffi::c_char = json_obj_key_at(d, j as u32);
        let mut nkthis: u32 = json_obj_key_len_at(d, j as u32) as u32;
        if !((*jthis).type_0 != JsonType::Array
            && (*jthis).type_0 != JsonType::Object)
        {
            let mut k: u32 = j.wrapping_add(1 as u32);
            while k < json_obj_len(d) as u32 {
                let mut jthat: *mut JsonValue = json_obj_val_at(d, k as u32);
                let mut kthat: *mut ::core::ffi::c_char = json_obj_key_at(d, k as u32);
                if json_ident(jthis, jthat) as ::core::ffi::c_int != 0
                    && (if sametag as ::core::ffi::c_int != 0 {
                        (strncmp(kthis, kthat, 4 as usize) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                    } else {
                        TRUE_0
                    }) != 0
                {
                    json_value_free(jthat);
                    let mut v: *mut JsonValue =
                        json_string_new_length(nkthis as ::core::ffi::c_uint, kthis);
                    (*v).parent = d as *mut JsonValue;
                    let ref mut fresh6 = (*(*d).u.object.values.offset(k as isize)).value;
                    *fresh6 = v as *mut JsonValue;
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_NOTICE,
                        LoggerType::Info,
                        crate::sdsbuild!(
                            sdsempty(),
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
                k = k.wrapping_add(1);
            }
        }
        j = j.wrapping_add(1);
    }
}
unsafe fn figure_out_features_from_json(
    mut features: *mut JsonValue,
    lh: &Vec<LookupEntry>,
    mut tag: *const ::core::ffi::c_char,
    mut options: *const Options,
) -> Vec<FeatureEntry> {
    let mut fh: Vec<FeatureEntry> = Vec::new();
    if (*options).merge_features {
        feature_merger_activate(
            features,
            true,
            b"feature\0" as *const u8 as *const ::core::ffi::c_char,
            options,
        );
    }
    let mut j: u32 = 0 as u32;
    while j < json_obj_len(features) as u32 {
        let mut feature_name: *mut ::core::ffi::c_char = json_obj_key_at(features, j as u32);
        let mut _feature: *mut JsonValue = json_obj_val_at(features, j as u32);
        if (*_feature).type_0 == JsonType::Array
        {
            let mut al: LookupRefList = Vec::new();
            let mut k: TableId = 0 as TableId;
            while (k as ::core::ffi::c_uint) < json_arr_len(_feature) {
                let mut term: *mut JsonValue = json_arr_at(_feature, k as u32);
                if !((*term).type_0 != JsonType::String)
                {
                    let term_bytes: Vec<u8> =
                        ::core::ffi::CStr::from_ptr(json_str_ptr(term)).to_bytes().to_vec();
                    let item = lh.iter().rev().find(|e| e.name == term_bytes);
                    if let Some(item) = item {
                        al.push(item.lookup as LookupRef);
                    } else {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Lookup assignment ",
                                json_str_ptr(term),
                                b" for feature [",
                                tag,
                                b"/",
                                feature_name,
                                b"] is missing or invalid.",
                            ),
                        );
                    }
                }
                k = k.wrapping_add(1);
            }
            if al.len() > 0 as usize {
                let feature_name_bytes: Vec<u8> =
                    ::core::ffi::CStr::from_ptr(feature_name).to_bytes().to_vec();
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
                    (*(*options).logger)
                        .log_sds
                        .expect(
                            "non-null function pointer",
                        )(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(
                            sdsempty(),
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
                (*(*options).logger)
                    .log_sds
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[OTFCC-fea] There is no valid lookup assignments for [",
                        tag,
                        b"/",
                        feature_name,
                        b"]. This feature will be ignored.\n",
                    ),
                );
                otl_lookup_ref_list_dispose(&raw mut al);
            }
        } else if (*_feature).type_0 == JsonType::String
        {
            let target: *mut ::core::ffi::c_char = json_str_ptr(_feature);
            let target_bytes: Vec<u8> = ::core::ffi::CStr::from_ptr(target).to_bytes().to_vec();
            if let Some(target_feature) = fh.iter().rev().find(|e| e.name == target_bytes).map(|e| e.feature) {
                fh.push(FeatureEntry {
                    name: ::core::ffi::CStr::from_ptr(feature_name).to_bytes().to_vec(),
                    alias: true,
                    feature: target_feature,
                });
            }
        }
        j = j.wrapping_add(1);
    }
    return fh;
}
pub unsafe extern "C" fn is_valid_language_name(
    mut name: *const ::core::ffi::c_char,
    length: usize,
) -> bool {
    return length == 9 as usize
        && *name.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == SCRIPT_LANGUAGE_SEPARATOR as ::core::ffi::c_int;
}
unsafe fn figure_out_languages_from_json(
    mut languages: *mut JsonValue,
    fh: &Vec<FeatureEntry>,
    mut tag: *const ::core::ffi::c_char,
    mut options: *const Options,
) -> std::collections::BTreeMap<Vec<u8>, *mut LanguageSystem> {
    let mut sh: std::collections::BTreeMap<Vec<u8>, *mut LanguageSystem> = std::collections::BTreeMap::new();
    let mut j: u32 = 0 as u32;
    while j < json_obj_len(languages) as u32 {
        let mut language_name: *mut ::core::ffi::c_char = json_obj_key_at(languages, j as u32);
        let mut language_name_len: usize = json_obj_key_len_at(languages, j as u32) as usize;
        let mut _language: *mut JsonValue = json_obj_val_at(languages, j as u32);
        if is_valid_language_name(language_name, language_name_len) as ::core::ffi::c_int != 0
            && (*_language).type_0 == JsonType::Object
        {
            let mut required_feature: *mut Feature = ::core::ptr::null_mut::<Feature>();
            let mut _rf: *mut JsonValue = json_obj_get_type(
                _language,
                b"requiredFeature\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if !_rf.is_null() {
                let rf_bytes: Vec<u8> =
                    ::core::ffi::CStr::from_ptr(json_str_ptr(_rf)).to_bytes().to_vec();
                if let Some(rf) = fh.iter().rev().find(|e| e.name == rf_bytes) {
                    required_feature = rf.feature;
                }
            }
            let mut af: FeatureRefList = Vec::new();
            let mut _features: *mut JsonValue = json_obj_get_type(
                _language,
                b"features\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            );
            if !_features.is_null() {
                let mut k: TableId = 0 as TableId;
                while (k as ::core::ffi::c_uint) < json_arr_len(_features) {
                    let mut term: *mut JsonValue = json_arr_at(_features, k as u32);
                    if (*term).type_0 == JsonType::String
                    {
                        let term_bytes: Vec<u8> =
                            ::core::ffi::CStr::from_ptr(json_str_ptr(term)).to_bytes().to_vec();
                        if let Some(item) = fh.iter().rev().find(|e| e.name == term_bytes) {
                            af.push(item.feature as FeatureRef);
                        }
                    }
                    k = k.wrapping_add(1);
                }
            }
            if !required_feature.is_null() || af.len() > 0 as usize {
                let language_name_bytes: Vec<u8> =
                    ::core::ffi::CStr::from_ptr(language_name).to_bytes().to_vec();
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
                    (*(*options).logger)
                        .log_sds
                        .expect(
                            "non-null function pointer",
                        )(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(
                            sdsempty(),
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
                (*(*options).logger)
                    .log_sds
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
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
        j = j.wrapping_add(1);
    }
    return sh;
}
pub unsafe extern "C" fn otfcc_parse_otl(
    mut root: *const JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> Option<Box<OtlTable>> {
    let mut languages: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    let mut features: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    let mut lookups: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    let mut current_block: u64;
    let mut otl: *mut OtlTable = ::core::ptr::null_mut::<OtlTable>();
    let mut otl_box: Option<Box<OtlTable>> = None;
    let mut table: *mut JsonValue = json_obj_get_type(root, tag, JsonType::Object);
    if !table.is_null() {
        otl_box = Some(Box::new(OtlTable { lookups: Vec::new(), features: Vec::new(), languages: Vec::new() }));
        otl = otl_box.as_mut().unwrap().as_mut() as *mut OtlTable;
        languages = json_obj_get_type(
            table,
            b"languages\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Object,
        );
        features = json_obj_get_type(
            table,
            b"features\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Object,
        );
        lookups = json_obj_get_type(
            table,
            b"lookups\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Object,
        );
        if !(languages.is_null() || features.is_null() || lookups.is_null()) {
            (*(*options).logger)
                .start_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                crate::sdsbuild!(sdsempty(), tag),
            );
            let mut ___loggedstep_v: bool = true;
            loop {
                if !___loggedstep_v {
                    current_block = 5279571973604048562;
                    break;
                }
                let mut lh: Vec<LookupEntry> = figure_out_lookups_from_json(lookups, options);
                let mut lookup_order: *mut JsonValue = json_obj_get_type(
                    table,
                    b"lookupOrder\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                );
                if !lookup_order.is_null() {
                    let mut j: TableId = 0 as TableId;
                    while (j as ::core::ffi::c_uint) < json_arr_len(lookup_order) {
                        let mut _ln: *mut JsonValue = json_arr_at(lookup_order, j as u32);
                        if !_ln.is_null()
                            && (*_ln).type_0 == JsonType::String
                        {
                            let ln_bytes: Vec<u8> =
                                ::core::ffi::CStr::from_ptr(json_str_ptr(_ln)).to_bytes().to_vec();
                            if let Some(item) = lh.iter_mut().rev().find(|e| e.name == ln_bytes) {
                                item.order_type = LookupOrderType::Force;
                                item.order_val = j as u16;
                            }
                        }
                        j = j.wrapping_add(1);
                    }
                }
                let mut fh: Vec<FeatureEntry> =
                    figure_out_features_from_json(features, &lh, tag, options);
                let mut sh: std::collections::BTreeMap<Vec<u8>, *mut LanguageSystem> =
                    figure_out_languages_from_json(languages, &fh, tag, options);
                if lh.is_empty() || fh.is_empty() || sh.is_empty() {
                    (*(*options).logger)
                        .dedent
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                    );
                    current_block = 12498981253432484999;
                    break;
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
                    lh.sort_by(|a, b| a.order_type.cmp(&b.order_type).then(a.order_val.cmp(&b.order_val)));
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
                    ___loggedstep_v = false;
                    (*(*options).logger)
                        .finish
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                    );
                }
            }
            match current_block {
                12498981253432484999 => {}
                _ => return otl_box,
            }
        }
    }
    if otl_box.is_some() {
        (*(*options).logger)
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::sdsbuild!(
                sdsempty(),
                b"[OTFCC-fea] Ignoring invalid or incomplete OTL table ",
                tag,
                b".\n",
            ),
        );
    }
    return None;
}
