#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::logger::{logger_finish, logger_start_sds};
use crate::support::built_json::BuiltValue;
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
    dumper: Option<unsafe fn(*const Subtable) -> BuiltValue>,
    lookup: *const Lookup,
    dump: &mut BuiltValue,
) {
    if (*lookup).type_0 == llt {
        dump.push_field(b"type", BuiltValue::str_truncated_at_nul(llt.name().to_bytes()));
        dump.push_field(
            b"flags",
            BuiltValue::dump_flags((*lookup).flags as i32, &LOOKUP_FLAGS_LABELS),
        );
        if (*lookup).flags as i32 >> 8_i32 != 0 {
            dump.push_field(
                b"markAttachmentType",
                BuiltValue::Int(((*lookup).flags as i32 >> 8_i32) as i64),
            );
        }
        let mut subtables = BuiltValue::new_array((*lookup).subtables.len());
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*lookup).subtables.len() {
            if let Some(sub) = &(&(*lookup).subtables)[j as usize] {
                subtables.push_item(dumper.expect("non-null function pointer")(
                    sub.as_ref() as *const Subtable,
                ));
            }
            j = j.wrapping_add(1);
        }
        dump.push_field(b"subtables", subtables);
    }
}
unsafe fn _dump_lookup(lookup: *const Lookup) -> BuiltValue {
    let mut dump = BuiltValue::new_object(5);
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_SINGLE,
        Some(otl_gsub_dump_single as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_MULTIPLE,
        Some(otl_gsub_dump_multi as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_ALTERNATE,
        Some(otl_gsub_dump_multi as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_LIGATURE,
        Some(otl_gsub_dump_ligature as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_CHAINING,
        Some(otl_dump_chaining as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_REVERSE,
        Some(otl_gsub_dump_reverse as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_CHAINING,
        Some(otl_dump_chaining as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_SINGLE,
        Some(otl_gpos_dump_single as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_PAIR,
        Some(otl_gpos_dump_pair as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_CURSIVE,
        Some(otl_gpos_dump_cursive as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_BASE,
        Some(otl_gpos_dump_mark_to_single as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_MARK,
        Some(otl_gpos_dump_mark_to_single as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_LIGATURE,
        Some(otl_gpos_dump_mark_to_ligature as unsafe fn(*const Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    dump
}
pub unsafe fn otfcc_dump_otl(
    table: Option<&OtlTable>,
    root: &mut BuiltValue,
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
        let mut otl = BuiltValue::new_object(3);
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"Languages"),
        );
        let mut ___loggedstep_v_0: bool = true;
        while ___loggedstep_v_0 {
            let mut languages = BuiltValue::new_object((*table).languages.len());
            let mut j: TableId = 0 as TableId;
            while (j as usize) < (*table).languages.len() {
                let mut _lang = BuiltValue::new_object(5);
                let lang: *const LanguageSystem = &raw const *(&(*table).languages)[j as usize];
                if !(*lang).required_feature.is_null() {
                    _lang.push_field(
                        b"requiredFeature",
                        BuiltValue::str_truncated_at_nul(&(*(*lang).required_feature).name),
                    );
                }
                let mut features = BuiltValue::new_array((*lang).features.len());
                let mut k: TableId = 0 as TableId;
                while (k as usize) < (*lang).features.len() {
                    if !(&(*lang).features)[k as usize].is_null() {
                        features.push_item(BuiltValue::str_truncated_at_nul(
                            &(*(&(*lang).features)[k as usize]).name,
                        ));
                    }
                    k = k.wrapping_add(1);
                }
                _lang.push_field(b"features", features.preserialize());
                languages.push_field_bytes_key(&(*lang).name, _lang);
                j = j.wrapping_add(1);
            }
            otl.push_field(b"languages", languages);
            ___loggedstep_v_0 = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"Features"),
        );
        let mut ___loggedstep_v_1: bool = true;
        while ___loggedstep_v_1 {
            let mut features_0 = BuiltValue::new_object((*table).features.len());
            let mut j_0: TableId = 0 as TableId;
            while (j_0 as usize) < (*table).features.len() {
                let feature: *const Feature = &raw const *(&(*table).features)[j_0 as usize];
                let mut _feature = BuiltValue::new_array((*feature).lookups.len());
                let mut k_0: TableId = 0 as TableId;
                while (k_0 as usize) < (*feature).lookups.len() {
                    if !(&(*feature).lookups)[k_0 as usize].is_null() {
                        _feature.push_item(BuiltValue::str_truncated_at_nul(
                            &(*(&(*feature).lookups)[k_0 as usize]).name,
                        ));
                    }
                    k_0 = k_0.wrapping_add(1);
                }
                features_0.push_field_bytes_key(&(*feature).name, _feature.preserialize());
                j_0 = j_0.wrapping_add(1);
            }
            otl.push_field(b"features", features_0);
            ___loggedstep_v_1 = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"Lookups"),
        );
        let mut ___loggedstep_v_2: bool = true;
        while ___loggedstep_v_2 {
            let mut lookups = BuiltValue::new_object((*table).lookups.len());
            let mut lookup_order = BuiltValue::new_array((*table).lookups.len());
            let mut j_1: TableId = 0 as TableId;
            while (j_1 as usize) < (*table).lookups.len() {
                let lookup: *const Lookup = &raw const *(&(*table).lookups)[j_1 as usize];
                let _lookup = _dump_lookup(lookup);
                lookups.push_field_bytes_key(&(*lookup).name, _lookup);
                lookup_order.push_item(BuiltValue::str_truncated_at_nul(&(*lookup).name));
                j_1 = j_1.wrapping_add(1);
            }
            otl.push_field(b"lookups", lookups);
            otl.push_field(b"lookupOrder", lookup_order);
            ___loggedstep_v_2 = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
        root.push_field(::core::ffi::CStr::from_ptr(tag).to_bytes(), otl);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
