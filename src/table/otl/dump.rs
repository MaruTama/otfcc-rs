use crate::logger::ByteStr;
use otfcc_json::BuiltValue;
use crate::table::otl::constants::LOOKUP_FLAGS_LABELS;
use crate::table::otl::kind::lookup_kind;
use crate::table::otl::{Feature, Lookup, OtlTable};
fn _dump_lookup(lookup: &Lookup) -> BuiltValue {
    let mut dump = BuiltValue::new_object(5);
    // A lookup of no known kind dumps as an empty object.
    let Some(kind) = lookup_kind(lookup.lookup_type) else {
        return dump;
    };
    dump.push_field(b"type", BuiltValue::str_truncated_at_nul(lookup.lookup_type.name().as_bytes()));
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
        subtables.push_item(kind.dump_subtable(sub.as_ref()));
    }
    dump.push_field(b"subtables", subtables);
    dump
}
pub fn dump_otl(table: Option<&OtlTable>, root: &mut BuiltValue, tag: &[u8]) {
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
    let stage = crate::logger::stage(ByteStr(tag));
    {
        let mut otl = BuiltValue::new_object(3);
        let stage_2 = crate::logger::stage("Languages");
        {
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
            stage_2.finish();
        }
        let stage_2 = crate::logger::stage("Features");
        {
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
            stage_2.finish();
        }
        let stage_2 = crate::logger::stage("Lookups");
        {
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
            stage_2.finish();
        }
        root.push_field(tag, otl);
        stage.finish();
    }
}
