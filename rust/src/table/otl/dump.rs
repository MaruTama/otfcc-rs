#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::logger::{logger_finish, logger_start_sds};
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new,
    json_object_push, json_object_push_bytes_key, json_string_new, json_string_new_from_bytes,
    otfcc_dump_flags, preserialize,
};
use crate::support::options::Options;
use crate::support::primitives::TableId;
use crate::table::otl::constants::LOOKUP_FLAGS_LABELS;
use crate::table::otl::subtables::chaining::dump::otl_dump_chaining;
use crate::table::otl::subtables::gpos_cursive::otl_gpos_dump_cursive;
use crate::table::otl::subtables::gpos_mark_to_ligature::otl_gpos_dump_mark_to_ligature;
use crate::table::otl::subtables::gpos_mark_to_single::otl_gpos_dump_mark_to_single;
use crate::table::otl::subtables::gpos_pair::otl_gpos_dump_pair;
use crate::table::otl::subtables::gpos_single::otl_gpos_dump_single;
use crate::table::otl::subtables::gsub_ligature::otl_gsub_dump_ligature;
use crate::table::otl::subtables::gsub_multi::otl_gsub_dump_multi;
use crate::table::otl::subtables::gsub_reverse::otl_gsub_dump_reverse;
use crate::table::otl::subtables::gsub_single::otl_gsub_dump_single;
use crate::table::otl::{
    Feature, LanguageSystem, Lookup, LookupType, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE,
    OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK,
    OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING,
    OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE,
    OtlTable, Subtable,
};
// No longer `extern "C"`: each of the 10 concrete dumpers passed in below
// is used in exactly one fixed association with its own `LookupType` --
// this whole sequence of calls is a `match` in disguise, not real runtime
// dispatch through a varying value (confirmed by grep: none of the 10
// dumper functions are referenced anywhere outside this file).
unsafe fn _declare_lookup_dumper(
    llt: LookupType,
    dumper: Option<unsafe fn(*const Subtable) -> *mut BuiltValue>,
    lookup: *const Lookup,
    dump: *mut BuiltValue,
) {
    if (*lookup).type_0 == llt {
        json_object_push(
            dump,
            b"type\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new(llt.name().as_ptr()),
        );
        json_object_push(
            dump,
            b"flags\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags((*lookup).flags as ::core::ffi::c_int, &LOOKUP_FLAGS_LABELS),
        );
        if (*lookup).flags as ::core::ffi::c_int >> 8 as ::core::ffi::c_int != 0 {
            json_object_push(
                dump,
                b"markAttachmentType\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new(
                    ((*lookup).flags as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as i64,
                ),
            );
        }
        let subtables: *mut BuiltValue = json_array_new((*lookup).subtables.len());
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*lookup).subtables.len() {
            if let Some(sub) = &(&(*lookup).subtables)[j as usize] {
                json_array_push(
                    subtables,
                    dumper.expect("non-null function pointer")(sub.as_ref() as *const Subtable),
                );
            }
            j = j.wrapping_add(1);
        }
        json_object_push(
            dump,
            b"subtables\0" as *const u8 as *const ::core::ffi::c_char,
            subtables,
        );
    }
}
unsafe fn _dump_lookup(lookup: *const Lookup, dump: *mut BuiltValue) {
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_SINGLE,
        Some(otl_gsub_dump_single as unsafe fn(*const Subtable) -> *mut BuiltValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_MULTIPLE,
        Some(otl_gsub_dump_multi as unsafe fn(*const Subtable) -> *mut BuiltValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_ALTERNATE,
        Some(otl_gsub_dump_multi as unsafe fn(*const Subtable) -> *mut BuiltValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_LIGATURE,
        Some(otl_gsub_dump_ligature as unsafe fn(*const Subtable) -> *mut BuiltValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_CHAINING,
        Some(otl_dump_chaining as unsafe fn(*const Subtable) -> *mut BuiltValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_REVERSE,
        Some(otl_gsub_dump_reverse as unsafe fn(*const Subtable) -> *mut BuiltValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_CHAINING,
        Some(otl_dump_chaining as unsafe fn(*const Subtable) -> *mut BuiltValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_SINGLE,
        Some(otl_gpos_dump_single as unsafe fn(*const Subtable) -> *mut BuiltValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_PAIR,
        Some(otl_gpos_dump_pair as unsafe fn(*const Subtable) -> *mut BuiltValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_CURSIVE,
        Some(otl_gpos_dump_cursive as unsafe fn(*const Subtable) -> *mut BuiltValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_BASE,
        Some(
            otl_gpos_dump_mark_to_single
                as unsafe fn(*const Subtable) -> *mut BuiltValue,
        ),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_MARK,
        Some(
            otl_gpos_dump_mark_to_single
                as unsafe fn(*const Subtable) -> *mut BuiltValue,
        ),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_LIGATURE,
        Some(
            otl_gpos_dump_mark_to_ligature
                as unsafe fn(*const Subtable) -> *mut BuiltValue,
        ),
        lookup,
        dump,
    );
}
pub unsafe fn otfcc_dump_otl(
    table: Option<&OtlTable>,
    root: *mut BuiltValue,
    options: &Options,
    tag: *const ::core::ffi::c_char,
) {
    let table: *const OtlTable = table.map_or(::core::ptr::null(), |t| t as *const OtlTable);
    if table.is_null()
        || (*table).languages.is_empty()
        || (*table).lookups.is_empty()
        || (*table).features.is_empty()
    {
        return;
    }
    logger_start_sds(&mut *options.logger.borrow_mut(), crate::bytesbuild!(tag));
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let otl: *mut BuiltValue = json_object_new(3 as usize);
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"Languages"),
        );
        let mut ___loggedstep_v_0: bool = true;
        while ___loggedstep_v_0 {
            let languages: *mut BuiltValue = json_object_new((*table).languages.len());
            let mut j: TableId = 0 as TableId;
            while (j as usize) < (*table).languages.len() {
                let mut _lang: *mut BuiltValue = json_object_new(5 as usize);
                let lang: *const LanguageSystem = &raw const *(&(*table).languages)[j as usize];
                if !(*lang).required_feature.is_null() {
                    json_object_push(
                        _lang,
                        b"requiredFeature\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_from_bytes(&(*(*lang).required_feature).name),
                    );
                }
                let features: *mut BuiltValue = json_array_new((*lang).features.len());
                let mut k: TableId = 0 as TableId;
                while (k as usize) < (*lang).features.len() {
                    if !(&(*lang).features)[k as usize].is_null() {
                        json_array_push(
                            features,
                            json_string_new_from_bytes(&(*(&(*lang).features)[k as usize]).name),
                        );
                    }
                    k = k.wrapping_add(1);
                }
                json_object_push(
                    _lang,
                    b"features\0" as *const u8 as *const ::core::ffi::c_char,
                    preserialize(features),
                );
                json_object_push_bytes_key(languages, &(*lang).name, _lang);
                j = j.wrapping_add(1);
            }
            json_object_push(
                otl,
                b"languages\0" as *const u8 as *const ::core::ffi::c_char,
                languages,
            );
            ___loggedstep_v_0 = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"Features"),
        );
        let mut ___loggedstep_v_1: bool = true;
        while ___loggedstep_v_1 {
            let features_0: *mut BuiltValue = json_object_new((*table).features.len());
            let mut j_0: TableId = 0 as TableId;
            while (j_0 as usize) < (*table).features.len() {
                let feature: *const Feature = &raw const *(&(*table).features)[j_0 as usize];
                let mut _feature: *mut BuiltValue = json_array_new((*feature).lookups.len());
                let mut k_0: TableId = 0 as TableId;
                while (k_0 as usize) < (*feature).lookups.len() {
                    if !(&(*feature).lookups)[k_0 as usize].is_null() {
                        json_array_push(
                            _feature,
                            json_string_new_from_bytes(
                                &(*(&(*feature).lookups)[k_0 as usize]).name,
                            ),
                        );
                    }
                    k_0 = k_0.wrapping_add(1);
                }
                json_object_push_bytes_key(features_0, &(*feature).name, preserialize(_feature));
                j_0 = j_0.wrapping_add(1);
            }
            json_object_push(
                otl,
                b"features\0" as *const u8 as *const ::core::ffi::c_char,
                features_0,
            );
            ___loggedstep_v_1 = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"Lookups"),
        );
        let mut ___loggedstep_v_2: bool = true;
        while ___loggedstep_v_2 {
            let lookups: *mut BuiltValue = json_object_new((*table).lookups.len());
            let lookup_order: *mut BuiltValue = json_array_new((*table).lookups.len());
            let mut j_1: TableId = 0 as TableId;
            while (j_1 as usize) < (*table).lookups.len() {
                let mut _lookup: *mut BuiltValue = json_object_new(5 as usize);
                let lookup: *const Lookup = &raw const *(&(*table).lookups)[j_1 as usize];
                _dump_lookup(lookup, _lookup);
                json_object_push_bytes_key(lookups, &(*lookup).name, _lookup);
                json_array_push(lookup_order, json_string_new_from_bytes(&(*lookup).name));
                j_1 = j_1.wrapping_add(1);
            }
            json_object_push(
                otl,
                b"lookups\0" as *const u8 as *const ::core::ffi::c_char,
                lookups,
            );
            json_object_push(
                otl,
                b"lookupOrder\0" as *const u8 as *const ::core::ffi::c_char,
                lookup_order,
            );
            ___loggedstep_v_2 = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
        json_object_push(root, tag, otl);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
