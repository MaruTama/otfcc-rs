use crate::logger::{logger_finish, logger_start_sds};
use crate::support::built_json::BuiltValue;
use crate::support::options::Options;
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
    Feature, Lookup, LookupType, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE,
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
fn _declare_lookup_dumper(
    llt: LookupType,
    dumper: Option<fn(&Subtable) -> BuiltValue>,
    lookup: &Lookup,
    dump: &mut BuiltValue,
) {
    if lookup.type_0 == llt {
        dump.push_field(b"type", BuiltValue::str_truncated_at_nul(llt.name().to_bytes()));
        dump.push_field(
            b"flags",
            BuiltValue::dump_flags(lookup.flags as i32, &LOOKUP_FLAGS_LABELS),
        );
        if lookup.flags as i32 >> 8_i32 != 0 {
            dump.push_field(
                b"markAttachmentType",
                BuiltValue::Int((lookup.flags as i32 >> 8_i32) as i64),
            );
        }
        let mut subtables = BuiltValue::new_array(lookup.subtables.len());
        for sub in lookup.subtables.iter().flatten() {
            subtables.push_item(dumper.expect("non-null function pointer")(sub.as_ref()));
        }
        dump.push_field(b"subtables", subtables);
    }
}
fn _dump_lookup(lookup: &Lookup) -> BuiltValue {
    let mut dump = BuiltValue::new_object(5);
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_SINGLE,
        Some(otl_gsub_dump_single as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_MULTIPLE,
        Some(otl_gsub_dump_multi as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_ALTERNATE,
        Some(otl_gsub_dump_multi as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_LIGATURE,
        Some(otl_gsub_dump_ligature as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_CHAINING,
        Some(otl_dump_chaining as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GSUB_REVERSE,
        Some(otl_gsub_dump_reverse as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_CHAINING,
        Some(otl_dump_chaining as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_SINGLE,
        Some(otl_gpos_dump_single as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_PAIR,
        Some(otl_gpos_dump_pair as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_CURSIVE,
        Some(otl_gpos_dump_cursive as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_BASE,
        Some(otl_gpos_dump_mark_to_single as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_MARK,
        Some(otl_gpos_dump_mark_to_single as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    _declare_lookup_dumper(
        OTL_TYPE_GPOS_MARK_TO_LIGATURE,
        Some(otl_gpos_dump_mark_to_ligature as fn(&Subtable) -> BuiltValue),
        lookup,
        &mut dump,
    );
    dump
}
pub fn otfcc_dump_otl(table: Option<&OtlTable>, root: &mut BuiltValue, options: &Options, tag: &[u8]) {
    let Some(table) = table else { return };
    // `table.lookups`/`.features` are hole-preserving now -- a `None`-only
    // `Vec` (every lookup/feature punched by consolidation) is the "empty"
    // case this guard means to catch, not merely a non-empty backing `Vec`.
    // `table.languages` is unaffected (never hole-punched).
    if table.languages.is_empty()
        || table.lookups.iter().all(Option::is_none)
        || table.features.iter().all(Option::is_none)
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
            let mut languages = BuiltValue::new_object(table.languages.len());
            for lang in table.languages.iter() {
                let mut _lang = BuiltValue::new_object(5);
                // `required_feature`/`features` are `Option<FeatureIdx>`/
                // `FeatureRefList` (`Vec<FeatureIdx>`) -- indices into this
                // same `OtlTable`'s own `features` list. `feature_at`
                // resolving to `None` (an index a later consolidation pass
                // punched into a hole, or -- not expected in practice --
                // an out-of-range one) is treated as "no reference", the
                // same as the old `is_null()` check treated a null
                // pointer.
                if let Some(rf) =
                    lang.required_feature.and_then(|idx| crate::table::otl::feature_at(&table.features, idx))
                {
                    _lang.push_field(
                        b"requiredFeature",
                        BuiltValue::str_truncated_at_nul(&rf.name),
                    );
                }
                let mut features = BuiltValue::new_array(lang.features.len());
                for &feat_idx in &lang.features {
                    if let Some(f) = crate::table::otl::feature_at(&table.features, feat_idx) {
                        features.push_item(BuiltValue::str_truncated_at_nul(&f.name));
                    }
                }
                _lang.push_field(b"features", features.preserialize());
                languages.push_field_bytes_key(&lang.name, _lang);
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
            // `.filter_map` skips holes -- a `None` slot consolidation
            // punched has nothing to dump.
            let live_features: Vec<&Feature> =
                table.features.iter().filter_map(Option::as_deref).collect();
            let mut features_0 = BuiltValue::new_object(live_features.len());
            for feature in &live_features {
                let mut _feature = BuiltValue::new_array(feature.lookups.len());
                for &lookup_idx in &feature.lookups {
                    // `lookups` is `LookupRefList` (`Vec<LookupIdx>`) --
                    // same borrowed-cross-reference shape as
                    // `required_feature`/`features` above.
                    if let Some(lookup) = crate::table::otl::lookup_at(&table.lookups, lookup_idx) {
                        _feature.push_item(BuiltValue::str_truncated_at_nul(&lookup.name));
                    }
                }
                features_0.push_field_bytes_key(&feature.name, _feature.preserialize());
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
            // `.filter` skips holes, same as the features loop above.
            let live_lookups: Vec<&Lookup> = table.lookups.iter().filter_map(Option::as_deref).collect();
            let mut lookups = BuiltValue::new_object(live_lookups.len());
            let mut lookup_order = BuiltValue::new_array(live_lookups.len());
            for lookup in &live_lookups {
                let _lookup = _dump_lookup(lookup);
                lookups.push_field_bytes_key(&lookup.name, _lookup);
                lookup_order.push_item(BuiltValue::str_truncated_at_nul(&lookup.name));
            }
            otl.push_field(b"lookups", lookups);
            otl.push_field(b"lookupOrder", lookup_order);
            ___loggedstep_v_2 = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
        root.push_field(tag, otl);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
