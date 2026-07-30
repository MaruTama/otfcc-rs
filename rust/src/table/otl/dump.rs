#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md



use crate::logger::{ILogger};
use crate::support::options::{Options};
use crate::support::primitives::{TableId};
use crate::vendor::json::JsonValue;
use crate::table::otl::{Feature, LanguageSystem, Lookup, LookupType, Subtable, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, OtlTable};
use crate::support::json_funcs::{otfcc_dump_flags, preserialize};
use crate::table::otl::constants::{LOOKUP_FLAGS_LABELS};
use crate::table::otl::subtables::chaining::dump::{otl_dump_chaining};
use crate::table::otl::subtables::gpos_cursive::{otl_gpos_dump_cursive};
use crate::table::otl::subtables::gpos_mark_to_ligature::{otl_gpos_dump_markToLigature};
use crate::table::otl::subtables::gpos_mark_to_single::{otl_gpos_dump_markToSingle};
use crate::table::otl::subtables::gpos_pair::{otl_gpos_dump_pair};
use crate::table::otl::subtables::gpos_single::{otl_gpos_dump_single};
use crate::table::otl::subtables::gsub_ligature::{otl_gsub_dump_ligature};
use crate::table::otl::subtables::gsub_multi::{otl_gsub_dump_multi};
use crate::table::otl::subtables::gsub_reverse::{otl_gsub_dump_reverse};
use crate::table::otl::subtables::gsub_single::{otl_gsub_dump_single};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_string_new};
use crate::vendor::sds::{sdsempty};
unsafe extern "C" fn _declare_lookup_dumper(
    mut llt: LookupType,
    mut dumper: Option<unsafe extern "C" fn(*const Subtable) -> *mut JsonValue>,
    mut lookup: *mut Lookup,
    mut dump: *mut JsonValue,
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
            otfcc_dump_flags(
                (*lookup).flags as ::core::ffi::c_int,
                &LOOKUP_FLAGS_LABELS,
            ),
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
        let mut subtables: *mut JsonValue = json_array_new((*lookup).subtables.length);
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*lookup).subtables.length {
            if !(*(*lookup).subtables.items.offset(j as isize)).is_null() {
                json_array_push(
                    subtables,
                    dumper.expect("non-null function pointer")(
                        *(*lookup).subtables.items.offset(j as isize) as *const Subtable,
                    ),
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
unsafe extern "C" fn _dump_lookup(mut lookup: *mut Lookup, mut dump: *mut JsonValue) {
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_SINGLE,
        Some(otl_gsub_dump_single as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_MULTIPLE,
        Some(otl_gsub_dump_multi as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_ALTERNATE,
        Some(otl_gsub_dump_multi as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_LIGATURE,
        Some(
            otl_gsub_dump_ligature as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue,
        ),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_CHAINING,
        Some(otl_dump_chaining as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_REVERSE,
        Some(otl_gsub_dump_reverse as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_CHAINING,
        Some(otl_dump_chaining as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_SINGLE,
        Some(otl_gpos_dump_single as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_PAIR,
        Some(otl_gpos_dump_pair as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_CURSIVE,
        Some(otl_gpos_dump_cursive as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_BASE,
        Some(
            otl_gpos_dump_markToSingle
                as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue,
        ),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_MARK,
        Some(
            otl_gpos_dump_markToSingle
                as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue,
        ),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_LIGATURE,
        Some(
            otl_gpos_dump_markToLigature
                as unsafe extern "C" fn(*const Subtable) -> *mut JsonValue,
        ),
        lookup,
        dump,
    );
}
pub unsafe extern "C" fn otfcc_dumpOtl(
    mut table: *const OtlTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) {
    if table.is_null()
        || (*table).languages.length == 0
        || (*table).lookups.length == 0
        || (*table).features.length == 0
    {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), tag),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut otl: *mut JsonValue = json_object_new(3 as usize);
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"Languages"),
        );
        let mut ___loggedstep_v_0: bool = true;
        while ___loggedstep_v_0 {
            let mut languages: *mut JsonValue = json_object_new((*table).languages.length);
            let mut j: TableId = 0 as TableId;
            while (j as usize) < (*table).languages.length {
                let mut _lang: *mut JsonValue = json_object_new(5 as usize);
                let mut lang: *mut LanguageSystem =
                    *(*table).languages.items.offset(j as isize) as *mut LanguageSystem;
                if !(*lang).requiredFeature.is_null() {
                    json_object_push(
                        _lang,
                        b"requiredFeature\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new(
                            (*(*lang).requiredFeature).name as *const ::core::ffi::c_char,
                        ),
                    );
                }
                let mut features: *mut JsonValue = json_array_new((*lang).features.length);
                let mut k: TableId = 0 as TableId;
                while (k as usize) < (*lang).features.length {
                    if !(*(*lang).features.items.offset(k as isize)).is_null() {
                        json_array_push(
                            features,
                            json_string_new(
                                (**(*lang).features.items.offset(k as isize)).name
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                    }
                    k = k.wrapping_add(1);
                }
                json_object_push(
                    _lang,
                    b"features\0" as *const u8 as *const ::core::ffi::c_char,
                    preserialize(features),
                );
                json_object_push(languages, (*lang).name as *const ::core::ffi::c_char, _lang);
                j = j.wrapping_add(1);
            }
            json_object_push(
                otl,
                b"languages\0" as *const u8 as *const ::core::ffi::c_char,
                languages,
            );
            ___loggedstep_v_0 = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"Features"),
        );
        let mut ___loggedstep_v_1: bool = true;
        while ___loggedstep_v_1 {
            let mut features_0: *mut JsonValue = json_object_new((*table).features.length);
            let mut j_0: TableId = 0 as TableId;
            while (j_0 as usize) < (*table).features.length {
                let mut feature: *mut Feature =
                    *(*table).features.items.offset(j_0 as isize) as *mut Feature;
                let mut _feature: *mut JsonValue = json_array_new((*feature).lookups.length);
                let mut k_0: TableId = 0 as TableId;
                while (k_0 as usize) < (*feature).lookups.length {
                    if !(*(*feature).lookups.items.offset(k_0 as isize)).is_null() {
                        json_array_push(
                            _feature,
                            json_string_new(
                                (**(*feature).lookups.items.offset(k_0 as isize)).name
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                    }
                    k_0 = k_0.wrapping_add(1);
                }
                json_object_push(
                    features_0,
                    (*feature).name as *const ::core::ffi::c_char,
                    preserialize(_feature),
                );
                j_0 = j_0.wrapping_add(1);
            }
            json_object_push(
                otl,
                b"features\0" as *const u8 as *const ::core::ffi::c_char,
                features_0,
            );
            ___loggedstep_v_1 = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"Lookups"),
        );
        let mut ___loggedstep_v_2: bool = true;
        while ___loggedstep_v_2 {
            let mut lookups: *mut JsonValue = json_object_new((*table).lookups.length);
            let mut lookupOrder: *mut JsonValue = json_array_new((*table).lookups.length);
            let mut j_1: TableId = 0 as TableId;
            while (j_1 as usize) < (*table).lookups.length {
                let mut _lookup: *mut JsonValue = json_object_new(5 as usize);
                let mut lookup: *mut Lookup =
                    *(*table).lookups.items.offset(j_1 as isize) as *mut Lookup;
                _dump_lookup(lookup, _lookup);
                json_object_push(
                    lookups,
                    (*lookup).name as *const ::core::ffi::c_char,
                    _lookup,
                );
                json_array_push(
                    lookupOrder,
                    json_string_new((*lookup).name as *const ::core::ffi::c_char),
                );
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
                lookupOrder,
            );
            ___loggedstep_v_2 = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
        json_object_push(root, tag, otl);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
