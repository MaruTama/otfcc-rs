#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memset, strcmp, strlen, strncmp};





use crate::support::json_funcs::{json_obj_get, json_obj_get_type, json_obj_getint};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, LOG_VL_NOTICE, ILogger};
use crate::support::options::{Options};
use crate::support::primitives::{TableId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonValue, JsonType};
use crate::support::{NULL, TRUE_0};
use crate::table::otl::{Feature, FeatureRef, FeatureRefList, LanguageSystem, Lookup, LookupRef, LookupRefList, LookupType, Subtable, SubtablePtr, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, OtlTable};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::support::json_funcs::otfcc_parse_flags;
use crate::table::otl::constants::{LOOKUP_FLAGS_LABELS};
use crate::support::json_ident::{json_ident};
use crate::table::otl::{otfcc_delete_lookup, otl_feature_ref_list_dispose, otl_feature_ref_list_replace, otl_lookup_ref_list_dispose, otl_lookup_ref_list_replace, new_feature, new_language, new_lookup, table_otl_create, table_otl_free};
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
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree, sdsnew};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LanguageHash {
    pub name: *mut ::core::ffi::c_char,
    pub language: *mut LanguageSystem,
    pub hh: UtHashHandle,
}
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
    if type_0.is_null() || strcmp((*type_0).u.string.ptr, llt.name().as_ptr()) != 0 {
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
    let mut subtable_count: TableId = (*_subtables).u.array.length as TableId;
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
            let mut _subtable: *mut JsonValue =
                *(*_subtables).u.array.values.offset(j as isize) as *mut JsonValue;
            if !_subtable.is_null()
                && (*_subtable).type_0 == JsonType::Object
            {
                let mut _st: *mut Subtable =
                    parser.expect("non-null function pointer")(_subtable, options);
                (*lookup).subtables.push(_st as SubtablePtr);
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
    (*lookup).name = sdsnew(lookup_name) as SdsRaw;
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
    while j < (*lookups).u.object.length as u32 {
        let mut lookup_name: *mut ::core::ffi::c_char =
            (*(*lookups).u.object.values.offset(j as isize)).name;
        if (*(*(*lookups).u.object.values.offset(j as isize)).value).type_0 == JsonType::Object
        {
            let mut parsed: bool = _parse_lookup(
                (*(*lookups).u.object.values.offset(j as isize)).value as *mut JsonValue,
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
        } else if (*(*(*lookups).u.object.values.offset(j as isize)).value).type_0
            as ::core::ffi::c_uint
            == JsonType::String as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut thatname: *mut ::core::ffi::c_char =
                (*(*(*lookups).u.object.values.offset(j as isize)).value)
                    .u
                    .string
                    .ptr;
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
    while j < (*d).u.object.length as u32 {
        let mut jthis: *mut JsonValue =
            (*(*d).u.object.values.offset(j as isize)).value as *mut JsonValue;
        let mut kthis: *mut ::core::ffi::c_char = (*(*d).u.object.values.offset(j as isize)).name;
        let mut nkthis: u32 =
            (*(*d).u.object.values.offset(j as isize)).name_length as u32;
        if !((*jthis).type_0 != JsonType::Array
            && (*jthis).type_0 != JsonType::Object)
        {
            let mut k: u32 = j.wrapping_add(1 as u32);
            while k < (*d).u.object.length as u32 {
                let mut jthat: *mut JsonValue =
                    (*(*d).u.object.values.offset(k as isize)).value as *mut JsonValue;
                let mut kthat: *mut ::core::ffi::c_char =
                    (*(*d).u.object.values.offset(k as isize)).name;
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
    while j < (*features).u.object.length as u32 {
        let mut feature_name: *mut ::core::ffi::c_char =
            (*(*features).u.object.values.offset(j as isize)).name;
        let mut _feature: *mut JsonValue =
            (*(*features).u.object.values.offset(j as isize)).value as *mut JsonValue;
        if (*_feature).type_0 == JsonType::Array
        {
            let mut al: LookupRefList = Vec::new();
            let mut k: TableId = 0 as TableId;
            while (k as ::core::ffi::c_uint) < (*_feature).u.array.length {
                let mut term: *mut JsonValue =
                    *(*_feature).u.array.values.offset(k as isize) as *mut JsonValue;
                if !((*term).type_0 != JsonType::String)
                {
                    let term_bytes: Vec<u8> =
                        ::core::ffi::CStr::from_ptr((*term).u.string.ptr).to_bytes().to_vec();
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
                                (*term).u.string.ptr,
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
                    (*feature).name = sdsnew(feature_name) as SdsRaw;
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
            let target: *mut ::core::ffi::c_char = (*_feature).u.string.ptr;
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
unsafe extern "C" fn figure_out_languages_from_json(
    mut languages: *mut JsonValue,
    fh: &Vec<FeatureEntry>,
    mut tag: *const ::core::ffi::c_char,
    mut options: *const Options,
) -> *mut LanguageHash {
    let mut sh: *mut LanguageHash = ::core::ptr::null_mut::<LanguageHash>();
    let mut j: u32 = 0 as u32;
    while j < (*languages).u.object.length as u32 {
        let mut language_name: *mut ::core::ffi::c_char =
            (*(*languages).u.object.values.offset(j as isize)).name;
        let mut language_name_len: usize =
            (*(*languages).u.object.values.offset(j as isize)).name_length as usize;
        let mut _language: *mut JsonValue =
            (*(*languages).u.object.values.offset(j as isize)).value as *mut JsonValue;
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
                    ::core::ffi::CStr::from_ptr((*_rf).u.string.ptr).to_bytes().to_vec();
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
                while (k as ::core::ffi::c_uint) < (*_features).u.array.length {
                    let mut term: *mut JsonValue =
                        *(*_features).u.array.values.offset(k as isize) as *mut JsonValue;
                    if (*term).type_0 == JsonType::String
                    {
                        let term_bytes: Vec<u8> =
                            ::core::ffi::CStr::from_ptr((*term).u.string.ptr).to_bytes().to_vec();
                        if let Some(item) = fh.iter().rev().find(|e| e.name == term_bytes) {
                            af.push(item.feature as FeatureRef);
                        }
                    }
                    k = k.wrapping_add(1);
                }
            }
            if !required_feature.is_null() || af.len() > 0 as usize {
                let mut s: *mut LanguageHash = ::core::ptr::null_mut::<LanguageHash>();
                let mut _hf_hashv_1: ::core::ffi::c_uint = 0;
                let mut _hj_i_1: ::core::ffi::c_uint = 0;
                let mut _hj_j_1: ::core::ffi::c_uint = 0;
                let mut _hj_k_1: ::core::ffi::c_uint = 0;
                let mut _hj_key_1: *const ::core::ffi::c_uchar =
                    language_name as *const ::core::ffi::c_uchar;
                _hf_hashv_1 = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i_1 = _hj_j_1;
                _hj_k_1 = strlen(language_name) as ::core::ffi::c_uint;
                while _hj_k_1 >= 12 as ::core::ffi::c_uint {
                    _hj_i_1 = _hj_i_1.wrapping_add(
                        (*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_j_1 = _hj_j_1.wrapping_add(
                        (*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                        (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key_1.offset(11 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                    _hj_i_1 ^= _hf_hashv_1 >> 13 as ::core::ffi::c_int;
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                    _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                    _hf_hashv_1 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                    _hj_i_1 ^= _hf_hashv_1 >> 12 as ::core::ffi::c_int;
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                    _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                    _hf_hashv_1 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                    _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                    _hj_i_1 ^= _hf_hashv_1 >> 3 as ::core::ffi::c_int;
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                    _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                    _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                    _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                    _hf_hashv_1 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
                    _hj_key_1 = _hj_key_1.offset(12 as ::core::ffi::c_int as isize);
                    _hj_k_1 = _hj_k_1.wrapping_sub(12 as ::core::ffi::c_uint);
                }
                _hf_hashv_1 = _hf_hashv_1.wrapping_add(strlen(language_name) as ::core::ffi::c_uint);
                let mut current_block_293: u64;
                match _hj_k_1 {
                    11 => {
                        _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                            (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_293 = 9731351165472125272;
                    }
                    10 => {
                        current_block_293 = 9731351165472125272;
                    }
                    9 => {
                        current_block_293 = 18074274781206676365;
                    }
                    8 => {
                        current_block_293 = 2703713326810445295;
                    }
                    7 => {
                        current_block_293 = 2287669382481938074;
                    }
                    6 => {
                        current_block_293 = 53785787823454161;
                    }
                    5 => {
                        current_block_293 = 6602686309234237379;
                    }
                    4 => {
                        current_block_293 = 5710910050460311707;
                    }
                    3 => {
                        current_block_293 = 7949765165458945995;
                    }
                    2 => {
                        current_block_293 = 16157078313657245745;
                    }
                    1 => {
                        current_block_293 = 18435508912408360646;
                    }
                    _ => {
                        current_block_293 = 2627007089909013891;
                    }
                }
                match current_block_293 {
                    9731351165472125272 => {
                        _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                            (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_293 = 18074274781206676365;
                    }
                    _ => {}
                }
                match current_block_293 {
                    18074274781206676365 => {
                        _hf_hashv_1 = _hf_hashv_1.wrapping_add(
                            (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_293 = 2703713326810445295;
                    }
                    _ => {}
                }
                match current_block_293 {
                    2703713326810445295 => {
                        _hj_j_1 = _hj_j_1.wrapping_add(
                            (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_293 = 2287669382481938074;
                    }
                    _ => {}
                }
                match current_block_293 {
                    2287669382481938074 => {
                        _hj_j_1 = _hj_j_1.wrapping_add(
                            (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_293 = 53785787823454161;
                    }
                    _ => {}
                }
                match current_block_293 {
                    53785787823454161 => {
                        _hj_j_1 = _hj_j_1.wrapping_add(
                            (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_293 = 6602686309234237379;
                    }
                    _ => {}
                }
                match current_block_293 {
                    6602686309234237379 => {
                        _hj_j_1 = _hj_j_1
                            .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_293 = 5710910050460311707;
                    }
                    _ => {}
                }
                match current_block_293 {
                    5710910050460311707 => {
                        _hj_i_1 = _hj_i_1.wrapping_add(
                            (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_293 = 7949765165458945995;
                    }
                    _ => {}
                }
                match current_block_293 {
                    7949765165458945995 => {
                        _hj_i_1 = _hj_i_1.wrapping_add(
                            (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_293 = 16157078313657245745;
                    }
                    _ => {}
                }
                match current_block_293 {
                    16157078313657245745 => {
                        _hj_i_1 = _hj_i_1.wrapping_add(
                            (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_293 = 18435508912408360646;
                    }
                    _ => {}
                }
                match current_block_293 {
                    18435508912408360646 => {
                        _hj_i_1 = _hj_i_1
                            .wrapping_add(*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                    }
                    _ => {}
                }
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                _hj_i_1 ^= _hf_hashv_1 >> 13 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                _hf_hashv_1 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                _hj_i_1 ^= _hf_hashv_1 >> 12 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                _hf_hashv_1 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_1);
                _hj_i_1 ^= _hf_hashv_1 >> 3 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_1);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_i_1);
                _hf_hashv_1 = _hf_hashv_1.wrapping_sub(_hj_j_1);
                _hf_hashv_1 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
                s = ::core::ptr::null_mut::<LanguageHash>();
                if !sh.is_null() {
                    let mut _hf_bkt_1: ::core::ffi::c_uint = 0;
                    _hf_bkt_1 = _hf_hashv_1
                        & (*(*sh).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if !(*(*(*sh).hh.tbl).buckets.offset(_hf_bkt_1 as isize))
                            .hh_head
                            .is_null()
                        {
                            s = ((*(*(*sh).hh.tbl).buckets.offset(_hf_bkt_1 as isize)).hh_head
                                as *mut ::core::ffi::c_char)
                                .offset(-(*(*sh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut LanguageHash
                                as *mut LanguageHash;
                        } else {
                            s = ::core::ptr::null_mut::<LanguageHash>();
                        }
                        while !s.is_null() {
                            if (*s).hh.hashv == _hf_hashv_1
                                && (*s).hh.keylen == strlen(language_name) as ::core::ffi::c_uint
                            {
                                if memcmp(
                                    (*s).hh.key,
                                    language_name as *const ::core::ffi::c_void,
                                    strlen(language_name) as ::core::ffi::c_uint as usize,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            if !(*s).hh.hh_next.is_null() {
                                s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                    .offset(-(*(*sh).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut LanguageHash
                                    as *mut LanguageHash;
                            } else {
                                s = ::core::ptr::null_mut::<LanguageHash>();
                            }
                        }
                    }
                }
                if s.is_null() {
                    s = __caryll_allocate_clean(
                        ::core::mem::size_of::<LanguageHash>() as usize,
                        267 as ::core::ffi::c_ulong,
                    ) as *mut LanguageHash;
                    (*s).name = sdsnew(language_name) as *mut ::core::ffi::c_char;
                    // The uthash node is a transient owner: it holds the
                    // `LanguageSystem` as a raw pointer (it is itself
                    // `__caryll_allocate_clean`'d and `free()`d, so a `Box`
                    // field here would never be dropped) and hands ownership
                    // back to a `Box` at the push site below, before the node
                    // is freed. Every node reaches that push -- see the
                    // `Box::from_raw` there.
                    (*s).language = Box::into_raw(new_language());
                    (*(*s).language).name = sdsdup((*s).name as SdsRaw);
                    (*(*s).language).required_feature = required_feature as FeatureRef;
                    otl_feature_ref_list_replace(&raw mut (*(*s).language).features, af);
                    let mut _ha_hashv: ::core::ffi::c_uint = 0;
                    let mut _hj_i_2: ::core::ffi::c_uint = 0;
                    let mut _hj_j_2: ::core::ffi::c_uint = 0;
                    let mut _hj_k_2: ::core::ffi::c_uint = 0;
                    let mut _hj_key_2: *const ::core::ffi::c_uchar =
                        (*s).name.offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_uchar;
                    _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                    _hj_j_2 = 0x9e3779b9 as ::core::ffi::c_uint;
                    _hj_i_2 = _hj_j_2;
                    _hj_k_2 = strlen((*s).name) as ::core::ffi::c_uint;
                    while _hj_k_2 >= 12 as ::core::ffi::c_uint {
                        _hj_i_2 = _hj_i_2.wrapping_add(
                            (*_hj_key_2.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (*_hj_key_2.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                ),
                        );
                        _hj_j_2 = _hj_j_2.wrapping_add(
                            (*_hj_key_2.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (*_hj_key_2.offset(5 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(6 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(7 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                ),
                        );
                        _ha_hashv = _ha_hashv.wrapping_add(
                            (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                .wrapping_add(
                                    (*_hj_key_2.offset(9 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 8 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(10 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 16 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (*_hj_key_2.offset(11 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_uint)
                                        << 24 as ::core::ffi::c_int,
                                ),
                        );
                        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                        _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                        _hj_i_2 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                        _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                        _hj_j_2 ^= _hj_i_2 << 8 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                        _ha_hashv ^= _hj_j_2 >> 13 as ::core::ffi::c_int;
                        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                        _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                        _hj_i_2 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                        _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                        _hj_j_2 ^= _hj_i_2 << 16 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                        _ha_hashv ^= _hj_j_2 >> 5 as ::core::ffi::c_int;
                        _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                        _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                        _hj_i_2 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                        _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                        _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                        _hj_j_2 ^= _hj_i_2 << 10 as ::core::ffi::c_int;
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                        _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                        _ha_hashv ^= _hj_j_2 >> 15 as ::core::ffi::c_int;
                        _hj_key_2 = _hj_key_2.offset(12 as ::core::ffi::c_int as isize);
                        _hj_k_2 = _hj_k_2.wrapping_sub(12 as ::core::ffi::c_uint);
                    }
                    _ha_hashv = _ha_hashv.wrapping_add(strlen((*s).name) as ::core::ffi::c_uint);
                    let mut current_block_413: u64;
                    match _hj_k_2 {
                        11 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_2.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_413 = 6879240018607126872;
                        }
                        10 => {
                            current_block_413 = 6879240018607126872;
                        }
                        9 => {
                            current_block_413 = 14288784596357532072;
                        }
                        8 => {
                            current_block_413 = 12439072576376712565;
                        }
                        7 => {
                            current_block_413 = 7537225836018668986;
                        }
                        6 => {
                            current_block_413 = 16524316032370654208;
                        }
                        5 => {
                            current_block_413 = 15902484794574381277;
                        }
                        4 => {
                            current_block_413 = 10414648127620005808;
                        }
                        3 => {
                            current_block_413 = 15856974427001039731;
                        }
                        2 => {
                            current_block_413 = 13794339084290703930;
                        }
                        1 => {
                            current_block_413 = 6622003589927843354;
                        }
                        _ => {
                            current_block_413 = 14239264278009762102;
                        }
                    }
                    match current_block_413 {
                        6879240018607126872 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_2.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_413 = 14288784596357532072;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        14288784596357532072 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_2.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_413 = 12439072576376712565;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        12439072576376712565 => {
                            _hj_j_2 = _hj_j_2.wrapping_add(
                                (*_hj_key_2.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_413 = 7537225836018668986;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        7537225836018668986 => {
                            _hj_j_2 = _hj_j_2.wrapping_add(
                                (*_hj_key_2.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_413 = 16524316032370654208;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        16524316032370654208 => {
                            _hj_j_2 = _hj_j_2.wrapping_add(
                                (*_hj_key_2.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_413 = 15902484794574381277;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        15902484794574381277 => {
                            _hj_j_2 = _hj_j_2
                                .wrapping_add(*_hj_key_2.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                            current_block_413 = 10414648127620005808;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        10414648127620005808 => {
                            _hj_i_2 = _hj_i_2.wrapping_add(
                                (*_hj_key_2.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_413 = 15856974427001039731;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        15856974427001039731 => {
                            _hj_i_2 = _hj_i_2.wrapping_add(
                                (*_hj_key_2.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_413 = 13794339084290703930;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        13794339084290703930 => {
                            _hj_i_2 = _hj_i_2.wrapping_add(
                                (*_hj_key_2.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_413 = 6622003589927843354;
                        }
                        _ => {}
                    }
                    match current_block_413 {
                        6622003589927843354 => {
                            _hj_i_2 = _hj_i_2
                                .wrapping_add(*_hj_key_2.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                        }
                        _ => {}
                    }
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                    _hj_i_2 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 8 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                    _ha_hashv ^= _hj_j_2 >> 13 as ::core::ffi::c_int;
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                    _hj_i_2 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 16 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                    _ha_hashv ^= _hj_j_2 >> 5 as ::core::ffi::c_int;
                    _hj_i_2 = _hj_i_2.wrapping_sub(_hj_j_2);
                    _hj_i_2 = _hj_i_2.wrapping_sub(_ha_hashv);
                    _hj_i_2 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                    _hj_j_2 = _hj_j_2.wrapping_sub(_ha_hashv);
                    _hj_j_2 = _hj_j_2.wrapping_sub(_hj_i_2);
                    _hj_j_2 ^= _hj_i_2 << 10 as ::core::ffi::c_int;
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_2);
                    _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_2);
                    _ha_hashv ^= _hj_j_2 >> 15 as ::core::ffi::c_int;
                    (*s).hh.hashv = _ha_hashv;
                    (*s).hh.key = (*s).name.offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void;
                    (*s).hh.keylen = strlen((*s).name) as ::core::ffi::c_uint;
                    if sh.is_null() {
                        (*s).hh.next = NULL;
                        (*s).hh.prev = NULL;
                        (*s).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                            as *mut UtHashTable
                            as *mut UtHashTable;
                        if (*s).hh.tbl.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                (*s).hh.tbl as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                ::core::mem::size_of::<UtHashTable>() as usize,
                            );
                            (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
                            (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                            (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                            (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                                .offset_from(s as *mut ::core::ffi::c_char)
                                as ::core::ffi::c_long
                                as isize;
                            (*(*s).hh.tbl).buckets =
                                malloc((32 as usize).wrapping_mul(::core::mem::size_of::<
                                    UtHashBucket,
                                >(
                                )
                                    as usize))
                                    as *mut UtHashBucket;
                            (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                            if (*(*s).hh.tbl).buckets.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    (32 as usize).wrapping_mul(
                                        ::core::mem::size_of::<UtHashBucket>() as usize,
                                    ),
                                );
                            }
                        }
                        sh = s;
                    } else {
                        (*s).hh.tbl = (*sh).hh.tbl;
                        (*s).hh.next = NULL;
                        (*s).hh.prev = ((*(*sh).hh.tbl).tail as *mut ::core::ffi::c_char)
                            .offset(-(*(*sh).hh.tbl).hho)
                            as *mut ::core::ffi::c_void;
                        (*(*(*sh).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                        (*(*sh).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
                    }
                    let mut _ha_bkt: ::core::ffi::c_uint = 0;
                    (*(*sh).hh.tbl).num_items = (*(*sh).hh.tbl).num_items.wrapping_add(1);
                    _ha_bkt = _ha_hashv
                        & (*(*sh).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _ha_head: *mut UtHashBucket =
                        (*(*sh).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
                    (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
                    (*s).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
                    (*s).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                    if !(*_ha_head).hh_head.is_null() {
                        (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh as *mut UtHashHandle;
                    }
                    (*_ha_head).hh_head = &raw mut (*s).hh as *mut UtHashHandle;
                    if (*_ha_head).count
                        >= (*_ha_head)
                            .expand_mult
                            .wrapping_add(1 as ::core::ffi::c_uint)
                            .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                        && (*(*s).hh.tbl).noexpand == 0
                    {
                        let mut _he_bkt: ::core::ffi::c_uint = 0;
                        let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                        let mut _he_thh: *mut UtHashHandle =
                            ::core::ptr::null_mut::<UtHashHandle>();
                        let mut _he_hh_nxt: *mut UtHashHandle =
                            ::core::ptr::null_mut::<UtHashHandle>();
                        let mut _he_new_buckets: *mut UtHashBucket =
                            ::core::ptr::null_mut::<UtHashBucket>();
                        let mut _he_newbkt: *mut UtHashBucket =
                            ::core::ptr::null_mut::<UtHashBucket>();
                        _he_new_buckets = malloc(
                            (2 as usize)
                                .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                        ) as *mut UtHashBucket;
                        if _he_new_buckets.is_null() {
                            exit(-(1 as ::core::ffi::c_int));
                        } else {
                            memset(
                                _he_new_buckets as *mut ::core::ffi::c_void,
                                '\0' as i32,
                                (2 as usize)
                                    .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<UtHashBucket>() as usize
                                    ),
                            );
                            (*(*s).hh.tbl).ideal_chain_maxlen = ((*(*s).hh.tbl).num_items
                                >> (*(*s).hh.tbl)
                                    .log2_num_buckets
                                    .wrapping_add(1 as ::core::ffi::c_uint))
                            .wrapping_add(
                                if (*(*s).hh.tbl).num_items
                                    & (*(*s).hh.tbl)
                                        .num_buckets
                                        .wrapping_mul(2 as ::core::ffi::c_uint)
                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                    != 0 as ::core::ffi::c_uint
                                {
                                    1 as ::core::ffi::c_uint
                                } else {
                                    0 as ::core::ffi::c_uint
                                },
                            );
                            (*(*s).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                            _he_bkt_i = 0 as ::core::ffi::c_uint;
                            while _he_bkt_i < (*(*s).hh.tbl).num_buckets {
                                _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize))
                                    .hh_head
                                    as *mut UtHashHandle;
                                while !_he_thh.is_null() {
                                    _he_hh_nxt = (*_he_thh).hh_next;
                                    _he_bkt = (*_he_thh).hashv
                                        & (*(*s).hh.tbl)
                                            .num_buckets
                                            .wrapping_mul(2 as ::core::ffi::c_uint)
                                            .wrapping_sub(1 as ::core::ffi::c_uint);
                                    _he_newbkt = _he_new_buckets.offset(_he_bkt as isize)
                                        as *mut UtHashBucket;
                                    (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                    if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                        (*(*s).hh.tbl).nonideal_items =
                                            (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                        (*_he_newbkt).expand_mult = (*_he_newbkt)
                                            .count
                                            .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
                                    }
                                    (*_he_thh).hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                                    (*_he_thh).hh_next =
                                        (*_he_newbkt).hh_head as *mut UtHashHandle;
                                    if !(*_he_newbkt).hh_head.is_null() {
                                        (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                                    }
                                    (*_he_newbkt).hh_head = _he_thh as *mut UtHashHandle;
                                    _he_thh = _he_hh_nxt;
                                }
                                _he_bkt_i = _he_bkt_i.wrapping_add(1);
                            }
                            free((*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void);
                            (*(*s).hh.tbl).num_buckets = (*(*s).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint);
                            (*(*s).hh.tbl).log2_num_buckets =
                                (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
                            (*(*s).hh.tbl).buckets = _he_new_buckets;
                            (*(*s).hh.tbl).ineff_expands = if (*(*s).hh.tbl).nonideal_items
                                > (*(*s).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                            {
                                (*(*s).hh.tbl)
                                    .ineff_expands
                                    .wrapping_add(1 as ::core::ffi::c_uint)
                            } else {
                                0 as ::core::ffi::c_uint
                            };
                            if (*(*s).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                                (*(*s).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                            }
                        }
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
unsafe extern "C" fn by_language_name(
    mut a: *mut LanguageHash,
    mut b: *mut LanguageHash,
) -> ::core::ffi::c_int {
    return strcmp((*a).name, (*b).name);
}
pub unsafe extern "C" fn otfcc_parse_otl(
    mut root: *const JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut OtlTable {
    let mut languages: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    let mut features: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    let mut lookups: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    let mut current_block: u64;
    let mut otl: *mut OtlTable = ::core::ptr::null_mut::<OtlTable>();
    let mut table: *mut JsonValue = json_obj_get_type(root, tag, JsonType::Object);
    if !table.is_null() {
        otl = table_otl_create();
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
                    while (j as ::core::ffi::c_uint) < (*lookup_order).u.array.length {
                        let mut _ln: *mut JsonValue =
                            *(*lookup_order).u.array.values.offset(j as isize) as *mut JsonValue;
                        if !_ln.is_null()
                            && (*_ln).type_0 == JsonType::String
                        {
                            let ln_bytes: Vec<u8> =
                                ::core::ffi::CStr::from_ptr((*_ln).u.string.ptr).to_bytes().to_vec();
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
                let mut sh: *mut LanguageHash =
                    figure_out_languages_from_json(languages, &fh, tag, options);
                let mut _hs_i_1: ::core::ffi::c_uint = 0;
                let mut _hs_looping_1: ::core::ffi::c_uint = 0;
                let mut _hs_nmerges_1: ::core::ffi::c_uint = 0;
                let mut _hs_insize_1: ::core::ffi::c_uint = 0;
                let mut _hs_psize_1: ::core::ffi::c_uint = 0;
                let mut _hs_qsize_1: ::core::ffi::c_uint = 0;
                let mut _hs_p_1: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _hs_q_1: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _hs_e_1: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _hs_list_1: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _hs_tail_1: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                if !sh.is_null() {
                    _hs_insize_1 = 1 as ::core::ffi::c_uint;
                    _hs_looping_1 = 1 as ::core::ffi::c_uint;
                    _hs_list_1 = &raw mut (*sh).hh as *mut UtHashHandle;
                    while _hs_looping_1 != 0 as ::core::ffi::c_uint {
                        _hs_p_1 = _hs_list_1;
                        _hs_list_1 = ::core::ptr::null_mut::<UtHashHandle>();
                        _hs_tail_1 = ::core::ptr::null_mut::<UtHashHandle>();
                        _hs_nmerges_1 = 0 as ::core::ffi::c_uint;
                        while !_hs_p_1.is_null() {
                            _hs_nmerges_1 = _hs_nmerges_1.wrapping_add(1);
                            _hs_q_1 = _hs_p_1;
                            _hs_psize_1 = 0 as ::core::ffi::c_uint;
                            _hs_i_1 = 0 as ::core::ffi::c_uint;
                            while _hs_i_1 < _hs_insize_1 {
                                _hs_psize_1 = _hs_psize_1.wrapping_add(1);
                                _hs_q_1 = (if !(*_hs_q_1).next.is_null() {
                                    ((*_hs_q_1).next as *mut ::core::ffi::c_char)
                                        .offset((*(*sh).hh.tbl).hho)
                                        as *mut UtHashHandle
                                } else {
                                    ::core::ptr::null_mut::<UtHashHandle>()
                                }) as *mut UtHashHandle;
                                if _hs_q_1.is_null() {
                                    break;
                                }
                                _hs_i_1 = _hs_i_1.wrapping_add(1);
                            }
                            _hs_qsize_1 = _hs_insize_1;
                            while _hs_psize_1 != 0 as ::core::ffi::c_uint
                                || _hs_qsize_1 != 0 as ::core::ffi::c_uint && !_hs_q_1.is_null()
                            {
                                if _hs_psize_1 == 0 as ::core::ffi::c_uint {
                                    _hs_e_1 = _hs_q_1;
                                    _hs_q_1 = (if !(*_hs_q_1).next.is_null() {
                                        ((*_hs_q_1).next as *mut ::core::ffi::c_char)
                                            .offset((*(*sh).hh.tbl).hho)
                                            as *mut UtHashHandle
                                    } else {
                                        ::core::ptr::null_mut::<UtHashHandle>()
                                    })
                                        as *mut UtHashHandle;
                                    _hs_qsize_1 = _hs_qsize_1.wrapping_sub(1);
                                } else if _hs_qsize_1 == 0 as ::core::ffi::c_uint
                                    || _hs_q_1.is_null()
                                {
                                    _hs_e_1 = _hs_p_1;
                                    if !_hs_p_1.is_null() {
                                        _hs_p_1 = (if !(*_hs_p_1).next.is_null() {
                                            ((*_hs_p_1).next as *mut ::core::ffi::c_char)
                                                .offset((*(*sh).hh.tbl).hho)
                                                as *mut UtHashHandle
                                        } else {
                                            ::core::ptr::null_mut::<UtHashHandle>()
                                        })
                                            as *mut UtHashHandle;
                                    }
                                    _hs_psize_1 = _hs_psize_1.wrapping_sub(1);
                                } else if by_language_name(
                                    (_hs_p_1 as *mut ::core::ffi::c_char)
                                        .offset(-(*(*sh).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut LanguageHash,
                                    (_hs_q_1 as *mut ::core::ffi::c_char)
                                        .offset(-(*(*sh).hh.tbl).hho)
                                        as *mut ::core::ffi::c_void
                                        as *mut LanguageHash,
                                ) <= 0 as ::core::ffi::c_int
                                {
                                    _hs_e_1 = _hs_p_1;
                                    if !_hs_p_1.is_null() {
                                        _hs_p_1 = (if !(*_hs_p_1).next.is_null() {
                                            ((*_hs_p_1).next as *mut ::core::ffi::c_char)
                                                .offset((*(*sh).hh.tbl).hho)
                                                as *mut UtHashHandle
                                        } else {
                                            ::core::ptr::null_mut::<UtHashHandle>()
                                        })
                                            as *mut UtHashHandle;
                                    }
                                    _hs_psize_1 = _hs_psize_1.wrapping_sub(1);
                                } else {
                                    _hs_e_1 = _hs_q_1;
                                    _hs_q_1 = (if !(*_hs_q_1).next.is_null() {
                                        ((*_hs_q_1).next as *mut ::core::ffi::c_char)
                                            .offset((*(*sh).hh.tbl).hho)
                                            as *mut UtHashHandle
                                    } else {
                                        ::core::ptr::null_mut::<UtHashHandle>()
                                    })
                                        as *mut UtHashHandle;
                                    _hs_qsize_1 = _hs_qsize_1.wrapping_sub(1);
                                }
                                if !_hs_tail_1.is_null() {
                                    (*_hs_tail_1).next = if !_hs_e_1.is_null() {
                                        (_hs_e_1 as *mut ::core::ffi::c_char)
                                            .offset(-(*(*sh).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                    } else {
                                        NULL
                                    };
                                } else {
                                    _hs_list_1 = _hs_e_1;
                                }
                                if !_hs_e_1.is_null() {
                                    (*_hs_e_1).prev = if !_hs_tail_1.is_null() {
                                        (_hs_tail_1 as *mut ::core::ffi::c_char)
                                            .offset(-(*(*sh).hh.tbl).hho)
                                            as *mut ::core::ffi::c_void
                                    } else {
                                        NULL
                                    };
                                }
                                _hs_tail_1 = _hs_e_1;
                            }
                            _hs_p_1 = _hs_q_1;
                        }
                        if !_hs_tail_1.is_null() {
                            (*_hs_tail_1).next = NULL;
                        }
                        if _hs_nmerges_1 <= 1 as ::core::ffi::c_uint {
                            _hs_looping_1 = 0 as ::core::ffi::c_uint;
                            (*(*sh).hh.tbl).tail = _hs_tail_1;
                            sh = (_hs_list_1 as *mut ::core::ffi::c_char)
                                .offset(-(*(*sh).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut LanguageHash
                                as *mut LanguageHash;
                        }
                        _hs_insize_1 = _hs_insize_1.wrapping_mul(2 as ::core::ffi::c_uint);
                    }
                }
                if lh.is_empty()
                    || fh.is_empty()
                    || (if !sh.is_null() {
                        (*(*sh).hh.tbl).num_items
                    } else {
                        0 as ::core::ffi::c_uint
                    }) == 0
                {
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
                    let mut s_1: *mut LanguageHash = ::core::ptr::null_mut::<LanguageHash>();
                    let mut tmp_1: *mut LanguageHash = ::core::ptr::null_mut::<LanguageHash>();
                    s_1 = sh;
                    tmp_1 = (if !sh.is_null() { (*sh).hh.next } else { NULL }) as *mut LanguageHash
                        as *mut LanguageHash;
                    while !s_1.is_null() {
                        // Takes ownership back from the uthash node (see the
                        // `Box::into_raw` above); the node itself is freed a
                        // few lines below, without touching `.language`.
                        (*otl).languages.push(Box::from_raw((*s_1).language));
                        let mut _hd_hh_del_1: *mut UtHashHandle = &raw mut (*s_1).hh;
                        if (*_hd_hh_del_1).prev.is_null() && (*_hd_hh_del_1).next.is_null() {
                            free((*(*sh).hh.tbl).buckets as *mut ::core::ffi::c_void);
                            free((*sh).hh.tbl as *mut ::core::ffi::c_void);
                            sh = ::core::ptr::null_mut::<LanguageHash>();
                        } else {
                            let mut _hd_bkt_1: ::core::ffi::c_uint = 0;
                            if _hd_hh_del_1 == (*(*sh).hh.tbl).tail {
                                (*(*sh).hh.tbl).tail = ((*_hd_hh_del_1).prev
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*sh).hh.tbl).hho)
                                    as *mut UtHashHandle
                                    as *mut UtHashHandle;
                            }
                            if !(*_hd_hh_del_1).prev.is_null() {
                                let ref mut fresh4 = (*(((*_hd_hh_del_1).prev
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*sh).hh.tbl).hho)
                                    as *mut UtHashHandle))
                                    .next;
                                *fresh4 = (*_hd_hh_del_1).next;
                            } else {
                                sh = (*_hd_hh_del_1).next as *mut LanguageHash
                                    as *mut LanguageHash;
                            }
                            if !(*_hd_hh_del_1).next.is_null() {
                                let ref mut fresh5 = (*(((*_hd_hh_del_1).next
                                    as *mut ::core::ffi::c_char)
                                    .offset((*(*sh).hh.tbl).hho)
                                    as *mut UtHashHandle))
                                    .prev;
                                *fresh5 = (*_hd_hh_del_1).prev;
                            }
                            _hd_bkt_1 = (*_hd_hh_del_1).hashv
                                & (*(*sh).hh.tbl)
                                    .num_buckets
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            let mut _hd_head_1: *mut UtHashBucket =
                                (*(*sh).hh.tbl).buckets.offset(_hd_bkt_1 as isize)
                                    as *mut UtHashBucket;
                            (*_hd_head_1).count = (*_hd_head_1).count.wrapping_sub(1);
                            if (*_hd_head_1).hh_head == _hd_hh_del_1 {
                                (*_hd_head_1).hh_head =
                                    (*_hd_hh_del_1).hh_next as *mut UtHashHandle;
                            }
                            if !(*_hd_hh_del_1).hh_prev.is_null() {
                                (*(*_hd_hh_del_1).hh_prev).hh_next = (*_hd_hh_del_1).hh_next;
                            }
                            if !(*_hd_hh_del_1).hh_next.is_null() {
                                (*(*_hd_hh_del_1).hh_next).hh_prev = (*_hd_hh_del_1).hh_prev;
                            }
                            (*(*sh).hh.tbl).num_items = (*(*sh).hh.tbl).num_items.wrapping_sub(1);
                        }
                        sdsfree((*s_1).name as SdsRaw);
                        free(s_1 as *mut ::core::ffi::c_void);
                        s_1 = ::core::ptr::null_mut::<LanguageHash>();
                        s_1 = tmp_1;
                        tmp_1 = (if !tmp_1.is_null() {
                            (*tmp_1).hh.next
                        } else {
                            NULL
                        }) as *mut LanguageHash
                            as *mut LanguageHash;
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
                _ => return otl,
            }
        }
    }
    if !otl.is_null() {
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
        table_otl_free(otl);
    }
    return ::core::ptr::null_mut::<OtlTable>();
}
