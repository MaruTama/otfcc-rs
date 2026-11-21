use std::collections::HashMap;

use crate::table::otl::coverage::{Coverage, dump_coverage};
use crate::table::otl::subtables::chaining::common::{chaining_is_canonical, chaining_rule_const};
use crate::table::otl::{ChainingRule, ChainingSubtable, Lookup, Subtable};
use otfcc_json::BuiltValue;

/// The glyph lists that several rules of one chaining lookup share, each
/// with a name (`@c1`, `@c2`, ...) to write in their place. Rules read from
/// a class-based subtable repeat the same lists at many positions; naming
/// them keeps the JSON short, and editing a list changes every rule that
/// uses it. A list of one glyph is never named.
pub struct ChainingClassNames<'a> {
    /// The named lists, in the order they first appear.
    classes: Vec<&'a Coverage>,
    /// The index in `classes` of each named list, keyed by its glyph names.
    index: HashMap<Vec<&'a [u8]>, usize>,
}

fn coverage_key(cov: &Coverage) -> Vec<&[u8]> {
    cov.iter().map(|h| h.name.as_slice()).collect()
}

fn rule_positions(subtable: &ChainingSubtable) -> impl Iterator<Item = &Coverage> {
    let rule = chaining_rule_const(subtable);
    // Bounded by `rule.match_count`, which need not equal `sequence.len()`.
    rule.sequence.iter().take(rule.match_count as usize)
}

impl<'a> ChainingClassNames<'a> {
    /// Names every list of two or more glyphs that `lookup`'s rules use at
    /// two or more positions.
    pub fn of(lookup: &'a Lookup) -> Self {
        let canonical = || {
            lookup
                .subtables
                .iter()
                .flatten()
                .filter_map(|s| match &**s {
                    Subtable::Chaining(c) if chaining_is_canonical(c) => Some(c),
                    _ => None,
                })
        };
        let mut uses: HashMap<Vec<&[u8]>, usize> = HashMap::new();
        for cov in canonical().flat_map(rule_positions) {
            if cov.len() > 1 {
                *uses.entry(coverage_key(cov)).or_default() += 1;
            }
        }
        let mut names = ChainingClassNames {
            classes: Vec::new(),
            index: HashMap::new(),
        };
        for cov in canonical().flat_map(rule_positions) {
            let key = coverage_key(cov);
            if uses.get(&key).is_some_and(|&n| n > 1) && !names.index.contains_key(&key) {
                names.index.insert(key, names.classes.len());
                names.classes.push(cov);
            }
        }
        names
    }
    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }
    fn name(i: usize) -> Vec<u8> {
        format!("@c{}", i + 1).into_bytes()
    }
    /// The name given to `cov`, if it has one.
    fn name_of(&self, cov: &Coverage) -> Option<Vec<u8>> {
        self.index.get(&coverage_key(cov)).map(|&i| Self::name(i))
    }
    /// The lookup's `classes` object: each name with its glyph list.
    pub fn dump(&self) -> BuiltValue {
        let mut classes = BuiltValue::new_object(self.classes.len());
        for (i, cov) in self.classes.iter().enumerate() {
            classes.push_field(&Self::name(i), dump_coverage(cov));
        }
        classes
    }
}

/// Writes a single-rule subtable as `{ match, apply, inputBegins, inputEnds }`:
/// one coverage per position, the lookups applied at positions, and where
/// the input sequence starts and ends. A subtable still holding a rule set
/// is written as `null`.
pub fn otl_dump_chaining(subtable: &Subtable) -> BuiltValue {
    otl_dump_chaining_named(subtable, None)
}

/// [`otl_dump_chaining`], but a position whose glyph list `names` names is
/// written as that name instead of the list.
pub fn otl_dump_chaining_named(
    subtable: &Subtable,
    names: Option<&ChainingClassNames<'_>>,
) -> BuiltValue {
    let Subtable::Chaining(subtable) = subtable else {
        unreachable!()
    };
    if !chaining_is_canonical(subtable) {
        return BuiltValue::Null;
    }
    let rule: &ChainingRule = chaining_rule_const(subtable);
    let mut _st = BuiltValue::new_object(4);
    let mut _match = BuiltValue::new_array(rule.match_count as usize);
    for cov in rule_positions(subtable) {
        match names.and_then(|n| n.name_of(cov)) {
            Some(name) => _match.push_item(BuiltValue::Str(name)),
            None => _match.push_item(dump_coverage(cov)),
        }
    }
    _st.push_field(b"match", _match);
    let mut _apply = BuiltValue::new_array(rule.apply.len());
    for application in rule.apply.iter() {
        let mut _application = BuiltValue::new_object(2);
        _application.push_field(b"at", BuiltValue::Int(i64::from(application.index)));
        _application.push_field(
            b"lookup",
            BuiltValue::str_truncated_at_nul(&application.lookup.name),
        );
        _apply.push_item(_application);
    }
    _st.push_field(b"apply", _apply.preserialize());
    _st.push_field(
        b"inputBegins",
        BuiltValue::Int(i64::from(rule.input_begins)),
    );
    _st.push_field(b"inputEnds", BuiltValue::Int(i64::from(rule.input_ends)));
    _st
}

#[cfg(test)]
mod chaining_class_names_tests {
    use super::*;
    use crate::support::handle::{GlyphHandle, handle_from_name};
    use crate::table::otl::coverage::push_to_coverage;
    use crate::table::otl::{OTL_TYPE_GSUB_CHAINING, new_lookup};
    use otfcc_json::{
        JSON_SERIALIZE_MODE_SINGLE_LINE, JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON,
        JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA, JSON_SERIALIZE_OPT_PACK_BRACKETS,
        JsonSerializeOpts, json_serialize_ex,
    };

    fn json(v: &BuiltValue) -> String {
        let opts = JsonSerializeOpts {
            mode: JSON_SERIALIZE_MODE_SINGLE_LINE,
            opts: JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON
                | JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA
                | JSON_SERIALIZE_OPT_PACK_BRACKETS,
            indent_size: 0,
        };
        String::from_utf8(json_serialize_ex(v, opts)).expect("UTF-8")
    }

    fn cov(names: &[&str]) -> Coverage {
        let mut c = Coverage::new();
        for n in names {
            push_to_coverage(
                &mut c,
                handle_from_name(Some(n.as_bytes().to_vec())) as GlyphHandle,
            );
        }
        c
    }

    fn lookup(rules: Vec<Vec<Coverage>>) -> Box<Lookup> {
        let mut l = new_lookup();
        l.lookup_type = OTL_TYPE_GSUB_CHAINING;
        for sequence in rules {
            let n = sequence.len() as u16;
            l.subtables.push(Some(Box::new(Subtable::Chaining(
                ChainingSubtable::Canonical(ChainingRule {
                    match_count: n,
                    input_begins: 0,
                    input_ends: n,
                    sequence,
                    apply: Vec::new(),
                }),
            ))));
        }
        l
    }

    /// Each subtable's `match`, serialized.
    fn matches(l: &Lookup, names: &ChainingClassNames<'_>) -> Vec<String> {
        l.subtables
            .iter()
            .flatten()
            .map(|s| {
                let BuiltValue::Object(fields) = otl_dump_chaining_named(s, Some(names)) else {
                    unreachable!()
                };
                json(&fields[0].1)
            })
            .collect()
    }

    #[test]
    fn only_lists_of_several_glyphs_used_more_than_once_are_named() {
        let l = lookup(vec![
            vec![cov(&["a", "b"]), cov(&["x"]), cov(&["p", "q"])],
            vec![cov(&["x"]), cov(&["a", "b"])],
        ]);
        let names = ChainingClassNames::of(&l);
        // ["a","b"] is used twice: named. ["x"] is one glyph and
        // ["p","q"] is used once: written as lists.
        assert_eq!(
            matches(&l, &names),
            vec![
                r#"["@c1",["x"],["p","q"]]"#.to_string(),
                r#"[["x"],"@c1"]"#.to_string(),
            ]
        );
        assert_eq!(json(&names.dump()), r#"{"@c1":["a","b"]}"#);
    }

    #[test]
    fn names_follow_the_order_lists_first_appear_in() {
        let l = lookup(vec![
            vec![cov(&["c", "d"]), cov(&["a", "b"])],
            vec![cov(&["a", "b"]), cov(&["c", "d"])],
        ]);
        let names = ChainingClassNames::of(&l);
        assert_eq!(json(&names.dump()), r#"{"@c1":["c","d"],"@c2":["a","b"]}"#);
    }

    #[test]
    fn the_same_glyphs_in_another_order_are_another_list() {
        // A list is written back exactly as it was read, so order counts.
        let l = lookup(vec![vec![cov(&["a", "b"])], vec![cov(&["b", "a"])]]);
        assert!(ChainingClassNames::of(&l).is_empty());
    }
}
