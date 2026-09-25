use crate::support::handle::{GlyphHandle, Handle, HandleState, handle_from_index};
use crate::table::otl::classdef::{ClassDef, push_class_def};
use crate::table::otl::coverage::{Coverage, push_to_coverage};

use crate::support::buffer::Buffer;
use crate::support::primitives::{GlyphClass, GlyphId, TableId};

use crate::table::otl::subtables::chaining::build::{
    otfcc_build_chaining, otfcc_build_contextual, otfcc_chaining_lookup_is_contextual_lookup,
};
use crate::table::otl::subtables::chaining::common::{
    chaining_is_canonical, chaining_rule_const, chaining_subtable_ref,
};
use crate::table::otl::{
    ChainLookupApplication, ChainingRule, ChainingRuleSet, ChainingSubtable, Lookup, Subtable,
};
#[derive(Clone, Debug)]
pub struct ClassifierValue {
    pub gname: Vec<u8>,
    pub cls: i32,
}
fn class_compatible(
    h: &mut std::collections::BTreeMap<GlyphId, ClassifierValue>,
    cov: &Coverage,
    past: &mut i32,
) -> i32 {
    if cov.is_empty() {
        return 1_i32;
    }
    let gid: GlyphId = cov[0].index;
    match h.get(&gid).map(|v| v.cls) {
        Some(cls) => {
            for entry in cov.iter().skip(1) {
                match h.get(&entry.index) {
                    Some(ss) if ss.cls == cls => {}
                    _ => return 0_i32,
                }
            }
            // Original built a throwaway `revh` -- a hash of `cov`'s own
            // (deduped) glyph ids, values unused (only ever a presence
            // check) -- to answer the *reverse* question below. Only
            // presence and position matter, same finding as
            // `PairClassifierHash`, so a bare `HashSet<GlyphId>` replaces
            // it, with no `gname`/`cls` payload to carry at all.
            let mut revset: std::collections::HashSet<GlyphId> = std::collections::HashSet::new();
            for entry in cov.iter() {
                revset.insert(entry.index);
            }
            // `allcheck`: every glyph already classified under `cls` in
            // `h` (not just the ones from this `cov`) must also be a
            // member of `cov`'s own glyph set -- i.e. `cov` must be
            // *exactly* the class's existing membership, not a subset,
            // even though every one of `cov`'s own glyphs already
            // shares `cls`.
            let allcheck: bool = h
                .iter()
                .filter(|&(_, v)| v.cls == cls)
                .all(|(gid_2, _)| revset.contains(gid_2));
            if allcheck { cls } else { 0_i32 }
        }
        None => {
            for entry in cov.iter().skip(1) {
                if h.contains_key(&entry.index) {
                    return 0_i32;
                }
            }
            let new_cls: i32 = *past + 1_i32;
            for entry in cov.iter() {
                h.entry(entry.index).or_insert(ClassifierValue {
                    gname: entry.name.clone(),
                    cls: new_cls,
                });
            }
            *past += 1_i32;
            1_i32
        }
    }
}
fn build_rule(
    rule: &ChainingRule,
    hb: &std::collections::BTreeMap<GlyphId, ClassifierValue>,
    hi: &std::collections::BTreeMap<GlyphId, ClassifierValue>,
    hf: &std::collections::BTreeMap<GlyphId, ClassifierValue>,
) -> Box<ChainingRule> {
    // `Box` is the allocation, the struct literal is the zero-init the old
    // `__caryll_allocate_clean` provided -- see `read.rs`'s
    // `general_read_contextual_rule`. This never fails (building from
    // already-valid in-memory data, not parsing untrusted bytes), so
    // unlike the `read.rs` constructors this returns `Box`, not `Option<Box>`.
    let mut new_rule: Box<ChainingRule> = Box::new(ChainingRule {
        match_count: rule.match_count,
        input_begins: rule.input_begins,
        input_ends: rule.input_ends,
        match_0: Vec::with_capacity(rule.match_count as usize),
        apply: Vec::new(),
    });
    // Bounded by `rule.match_count`, not assumed equal to
    // `rule.match_0.len()` (it is `Vec::with_capacity`d to that count by
    // callers, but this function itself has no reason to rely on the
    // two agreeing) -- `.take(rule.match_count as usize)` preserves the
    // original's own bound exactly.
    for (m, match_entry) in rule
        .match_0
        .iter()
        .enumerate()
        .take(rule.match_count as usize)
    {
        // Built as a plain local `Vec` and pushed directly -- no need for
        // the `otl_coverage_create()`/`coverage_from_raw()` raw-pointer
        // round trip other constructors use, since `Coverage` is just
        // `Vec<GlyphHandle>` and this function never hands the pointer to
        // anyone else in between.
        let mut cov: Coverage = Coverage::new();
        if !match_entry.is_empty() {
            let h: &std::collections::BTreeMap<GlyphId, ClassifierValue> =
                if (m as i32) < rule.input_begins as i32 {
                    hb
                } else if (m as i32) < rule.input_ends as i32 {
                    hi
                } else {
                    hf
                };
            let gid: GlyphId = match_entry[0].index;
            // `h.get(&gid)` is unreachable-as-`None` in practice: every
            // glyph reaching this point already passed `class_compatible`
            // for this same `h`, which never returns success without
            // having inserted (or already found) that glyph. The
            // fallback to class 0 mirrors the empty-coverage `else`
            // branch below rather than asserting, matching this
            // migration's established handling of `None` arms that the
            // algorithm's own invariants rule out (see `ClassNameHash`
            // in RUST_MIGRATION.md).
            let cls: GlyphClass = match h.get(&gid) {
                Some(v) => v.cls as GlyphClass,
                None => 0 as GlyphClass,
            };
            push_to_coverage(&mut cov, handle_from_index(cls) as GlyphHandle);
        } else {
            push_to_coverage(&mut cov, handle_from_index(0 as GlyphId) as GlyphHandle);
        }
        new_rule.match_0.push(cov);
    }
    // Plain assignment is fine here (unlike the calloc'd-memory case
    // elsewhere in this crate): `Box::new` above already gave `.apply` a
    // valid empty `Vec`, so there's a real (if empty) value to drop first.
    new_rule.apply = Vec::with_capacity(rule.apply.len());
    for entry in rule.apply.iter() {
        new_rule.apply.push(ChainLookupApplication {
            index: entry.index,
            lookup: entry.lookup.clone(),
        });
    }
    new_rule
}
fn to_class(h: &std::collections::BTreeMap<GlyphId, ClassifierValue>) -> Box<ClassDef> {
    // The dedup key (gid) and the original's `HASH_SORT` key (also gid,
    // via `by_gid_clsh`) are the same, so `BTreeMap`'s natural `Ord`
    // reproduces the sorted walk with no separate sort step -- the
    // `by_gid_clsh` comparator itself is gone, subsumed entirely by the
    // container. Borrowing rather than consuming `h` matches the
    // original, where `to_class` sorts and reads but never frees --
    // disposal was always the caller's job, and here that's simply
    // `try_classify_around` letting its `BTreeMap`s drop at scope exit.
    //
    // Built as a plain local `Box` (matching `otl_class_def_create`'s own
    // zero-init literal) rather than routing through that raw-pointer
    // constructor -- returning `Box<ClassDef>` directly lets the caller
    // assign straight into `Option<Box<ClassDef>>` with `Some(...)`,
    // no `classdef_from_raw` bridge needed.
    let mut cd = Box::new(ClassDef {
        maxclass: 0,
        glyphs: Vec::new(),
        classes: Vec::new(),
    });
    for (&gid, v) in h.iter() {
        push_class_def(
            &mut cd,
            Handle {
                state: HandleState::Consolidated,
                index: gid,
                name: v.gname.clone(),
            } as GlyphHandle,
            v.cls as GlyphClass,
        );
    }
    cd
}
/// Looks for a run of consecutive `Canonical` subtables starting at `j`
/// (inclusive) that all classify compatibly against the same three
/// backtrack/input/lookahead `ClassifierValue` maps, and -- if it finds
/// more than one -- builds the single `Classified` subtable that replaces
/// the whole run.
///
/// Returns `Some((run_len, classified))` when a run of more than one
/// compatible subtable was found (`run_len` is how many subtables at and
/// after `j` the caller should skip past, and `classified` is the owned
/// replacement value to build from instead of `subtables[j]` itself), or
/// `None` when `j`'s own subtable has no compatible neighbor to merge with
/// (the caller should build from `subtables[j]` unchanged).
///
/// Stage L-6: `classified` used to be built by `__caryll_allocate_clean`
/// (a raw `*mut ChainingSubtable`, `ptr::write`-initialized in place) and
/// handed back through a `classified_st: *mut *mut ChainingSubtable` out
/// parameter; the caller then had to compare that pointer against the
/// original by identity (`if st != st0 { subtable_chaining_free(st) }`) to
/// know whether a new allocation needs freeing. Returning the value owned
/// in an `Option` instead removes both the calloc'd intermediate and the
/// pointer-identity check: there is nothing to free by hand, because a
/// `None` never allocated anything and a `Some` simply drops when the
/// caller is done with it.
pub fn try_classify_around(
    subtables: &[Option<Box<Subtable>>],
    j: usize,
) -> Option<(usize, ChainingSubtable)> {
    let mut hb: std::collections::BTreeMap<GlyphId, ClassifierValue> =
        std::collections::BTreeMap::new();
    let mut hi: std::collections::BTreeMap<GlyphId, ClassifierValue> =
        std::collections::BTreeMap::new();
    let mut hf: std::collections::BTreeMap<GlyphId, ClassifierValue> =
        std::collections::BTreeMap::new();
    let mut classno_b: i32 = 0_i32;
    let mut classno_i: i32 = 0_i32;
    let mut classno_f: i32 = 0_i32;

    let rule0 = chaining_rule_const(chaining_subtable_ref(&subtables[j]));

    // Was a `current_block`-flagged `loop`: this runs to completion (every
    // one of `rule0`'s own matches is class-compatible) or stops early on
    // the first incompatible one -- `rule0_is_compatible` records which.
    let mut rule0_is_compatible = true;
    for (m, cov) in rule0.match_0.iter().enumerate().take(rule0.match_count as usize) {
        let (h, classno) = if m < rule0.input_begins as usize {
            (&mut hb, &mut classno_b)
        } else if m < rule0.input_ends as usize {
            (&mut hi, &mut classno_i)
        } else {
            (&mut hf, &mut classno_f)
        };
        if class_compatible(h, cov, classno) == 0 {
            rule0_is_compatible = false;
            break;
        }
    }
    if !rule0_is_compatible {
        return None;
    }

    // Scan forward for a run of subtables that all classify compatibly
    // against the same maps -- `compatible_count` is how many of them (not
    // counting `rule0` itself) qualify. `allcheck` in the original C-shaped
    // code was declared `true` and never reassigned anywhere in this loop
    // (any incompatible subtable broke the loop outright, via `break
    // 's_74`, before ever reaching the `if allcheck` check) -- dead, so
    // there is nothing to reproduce beyond "reaching the bottom of the loop
    // body always counts".
    let mut compatible_count: usize = 0;
    'run: for slot in subtables.iter().skip(j + 1) {
        let rule = chaining_rule_const(chaining_subtable_ref(slot));
        for (m, cov) in rule.match_0.iter().enumerate().take(rule.match_count as usize) {
            let (h, classno) = if m < rule.input_begins as usize {
                (&mut hb, &mut classno_b)
            } else if m < rule.input_ends as usize {
                (&mut hi, &mut classno_i)
            } else {
                (&mut hf, &mut classno_f)
            };
            if class_compatible(h, cov, classno) == 0 {
                break 'run;
            }
        }
        compatible_count += 1;
    }

    if compatible_count <= 1 {
        return None;
    }

    let mut rules: Vec<Option<Box<ChainingRule>>> = Vec::with_capacity(compatible_count + 1);
    rules.push(Some(build_rule(rule0, &hb, &hi, &hf)));
    for slot in subtables.iter().skip(j + 1).take(compatible_count) {
        let rule_k = chaining_rule_const(chaining_subtable_ref(slot));
        rules.push(Some(build_rule(rule_k, &hb, &hi, &hf)));
    }
    let ruleset = ChainingRuleSet {
        rules,
        bc: Some(to_class(&hb)),
        ic: Some(to_class(&hi)),
        fc: Some(to_class(&hf)),
    };
    Some((compatible_count, ChainingSubtable::Classified(ruleset)))
}
pub fn otfcc_classified_build_chaining(
    lookup: &Lookup,
    subtable_buffers: &mut Vec<Buffer>,
    last_offset: &mut usize,
) -> TableId {
    let is_contextual = otfcc_chaining_lookup_is_contextual_lookup(lookup);
    let mut subtables_written: TableId = 0;
    subtable_buffers.clear();
    subtable_buffers.reserve(lookup.subtables.len());
    let mut j: usize = 0;
    while j < lookup.subtables.len() {
        let st0 = chaining_subtable_ref(&lookup.subtables[j]);
        if chaining_is_canonical(st0) {
            // `classified` is the owned replacement (if a compatible run of
            // more than one subtable was found) -- it lives only for this
            // iteration and drops automatically once `buf` has been built
            // from it, no manual free needed either way.
            let classified = try_classify_around(&lookup.subtables, j);
            let buf: Buffer = match &classified {
                Some((_, owned)) => {
                    if is_contextual {
                        otfcc_build_contextual(owned)
                    } else {
                        otfcc_build_chaining(owned)
                    }
                }
                None => {
                    if is_contextual {
                        otfcc_build_contextual(st0)
                    } else {
                        otfcc_build_chaining(st0)
                    }
                }
            };
            if let Some((run_len, _)) = classified {
                j += run_len;
            }
            *last_offset += buf.data.len();
            subtable_buffers.push(buf);
            subtables_written += 1;
        }
        j += 1;
    }
    subtables_written
}
