use crate::support::handle::{
    GlyphHandle, LookupHandle, handle_from_index,
};
use crate::table::otl::classdef::{ClassDef, read_class_def};
use crate::table::otl::budget::OtlReadBudget;
use crate::table::otl::coverage::{Coverage, push_to_coverage, read_coverage};

use otfcc_binary::FontReader;

use crate::support::primitives::{GlyphId, TableId};

use crate::table::otl::subtables::chaining::common::chaining_ruleset_mut;
use crate::table::otl::{
    ChainLookupApplication, ChainingRule, ChainingRuleSet, ChainingSubtable, Subtable,
};
// The class definitions of a format 2 subtable, which `class_coverage`
// turns into coverages. The readers take the coverage reader as a closure,
// so this one can capture them.
#[derive(Debug)]
pub struct ClassDefs {
    pub bc: Option<Box<ClassDef>>,
    pub ic: Option<Box<ClassDef>>,
    pub fc: Option<Box<ClassDef>>,
}
/// The `class_zero_glyphs` limit of `OtlReadBudget`, shared by a whole
/// GSUB or GPOS table. Each `class_coverage` call can scan every glyph of
/// the font, once per rule position, so a subtable of a few hundred KB can
/// otherwise cost gigabytes (a fuzzed font reached 1.8 GB). NotoNastaliqUrdu,
/// a real and complex font, uses about 10.7 million units in its GSUB, so
/// 20 million leaves it room while keeping a whole table to under a second.
pub(crate) const MAX_TOTAL_CLASS_ZERO_COVERAGE_GLYPHS: u32 = 20_000_000;
/// The `class_coverage_calls` limit of `OtlReadBudget`: bounds the number of
/// `class_coverage` *calls* themselves, independent of how much work (if
/// any) each one does internally -- what actually stops a fuzz-found
/// font whose rules reference an empty classdef, so the `class_zero_glyphs`
/// limit above never triggers at all, from taking 20-30s on sheer call volume
/// (well past a million calls/second's worth of fixed per-call overhead).
pub(crate) const MAX_TOTAL_CLASS_COVERAGE_CALLS: u32 = 70_000;
// Both limits above are per table, not per subtable: a lookup may have up
// to 1,000 subtables, and a fresh allowance for each multiplied back into
// the same hang (a fuzzed lookup with ~700 subtables took 20+ seconds).
/// Bounds the number of contextual/chaining rules built across a whole table
/// (`OtlReadBudget::rules`). Each rule set's count fits the table, but many
/// sets can each carry a huge count: a fuzzed format 2 subtable summed past
/// 500,000 rules. Real fonts have at most a few hundred rules per subtable,
/// so this is far above legitimate use.
pub(crate) const MAX_TOTAL_RULES_PER_TABLE: u32 = 15_000;
/// Bounds how many `ChainLookupApplication` entries a single contextual/
/// chaining rule builds. `n_apply` is a raw `u16` read straight from the
/// rule header; the only existing guard (`require_room`) just checks the
/// array fits inside the table, which a large enough table happily allows.
/// `consolidate_chaining` logs one `[Consolidate] Quoting an invalid
/// lookup #N` warning (a heap-allocating `bytesbuild!` call, plus a stderr
/// write) for every entry whose `lookup_index` doesn't resolve --
/// fuzzing found a rule with tens of thousands of such entries, most
/// pointing nowhere, turning one rule into tens of thousands of log
/// writes. Real rules apply a handful of lookups at most, so this cap is
/// far above any legitimate usage while keeping worst-case log volume
/// (and the allocation/read work building the `apply` vec itself) small.
const MAX_APPLY_PER_RULE: usize = 50;
/// Bounds how many backtrack/input/lookahead positions a single
/// contextual/chaining rule actually builds `match_0` entries for.
/// `n_input` (and, in the chaining format, `n_back`/`n_lookaround` too)
/// are raw `u16`s read straight from the rule header, each independently
/// bounds-checked only against the table fitting the array it introduces
/// -- true for a large enough table. Even after `MAX_TOTAL_RULES_PER_
/// TABLE` bounds how many *rules* get built, fuzzing found that a handful
/// of rules with a huge position count each was enough on its own: every
/// position triggers a `class_coverage`/`single_coverage` call (and its
/// `Coverage` allocation) even once `class_coverage_calls` makes
/// that call's own internal work free, since the call itself -- and the
/// allocation it always makes before checking anything -- still happens.
/// Real rules match a handful of positions (single digits, rarely more
/// than a dozen), so this cap is far above legitimate usage. Only the
/// loop bounds are capped, not the byte offsets derived from the
/// uncapped counts (`lookup_base`/`input_base`/`lookaround_base`/
/// `apply_base` below) -- those offsets are already guaranteed in-bounds
/// by the `require_room` calls above (which validated the *uncapped*
/// sizes), so reading from them is safe regardless; only the resulting
/// glyph sequence for a rule this pathological is arbitrary, which is
/// fine for input this malformed. `match_count`/`input_begins`/
/// `input_ends` are computed from the *capped* counts specifically so
/// they always agree with `match_0`'s actual (possibly truncated) length
/// -- using the uncapped counts there would let downstream code (e.g.
/// `consolidate_chaining`) index past the end of `match_0`.
const MAX_POSITIONS_PER_RULE: u16 = 50;
/// Which side of a chaining/contextual rule a coverage lookup is for:
/// backtrack (glyphs before the input sequence), input (the sequence
/// itself), or lookahead (glyphs after it) -- the OpenType spec's 1/2/3
/// numbering of a format 2 rule's three `ClassDef`s. Only `class_coverage`
/// uses it.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ContextKind {
    Backtrack = 1,
    Input = 2,
    Lookahead = 3,
}
pub fn single_coverage(
    mut _data: &[u8],
    gid: u16,
    mut _offset: u32,
    mut _kind: ContextKind,
    _max_glyphs: GlyphId,
    _budget: &mut OtlReadBudget,
) -> Coverage {
    let mut cov = Coverage::new();
    push_to_coverage(&mut cov, handle_from_index(gid) as GlyphHandle);
    cov
}
pub fn class_coverage(
    mut _data: &[u8],
    cls: u16,
    mut _offset: u32,
    kind: ContextKind,
    max_glyphs: GlyphId,
    defs: &ClassDefs,
    budget: &mut OtlReadBudget,
) -> Coverage {
    // `.expect()`, not a null-pointer deref: every caller that reaches here
    // (`general_read_contextual_rule`/`general_read_chaining_rule` via
    // `class_coverage`'s `coverage_of` slot) only ever asks for a `kind` whose
    // matching field was populated by `read_contextual_format2`/
    // `read_chaining_format2` beforehand -- `read_class_def` itself never
    // returns null, so this can't actually fail; panicking instead of a
    // would-be null deref matches this migration's general "UB becomes a
    // panic" idiom (see e.g. `general_read_contextual_rule`'s own
    // `.expect()` calls above).
    let cd: &ClassDef = match kind {
        ContextKind::Backtrack => defs.bc.as_deref(),
        ContextKind::Input => defs.ic.as_deref(),
        ContextKind::Lookahead => defs.fc.as_deref(),
    }
    .expect("class_coverage: ClassDefs field for this `kind` was not populated");
    // Charged unconditionally, before doing anything else: a rule set
    // built almost entirely of degenerate rules referencing an *empty*
    // classdef (`cd.glyphs.len() == 0`) never enters either loop below,
    // so the per-iteration budget charges further down never fire at
    // all -- yet a fuzz-found font still called this well past a
    // million times in a few seconds (each call's own fixed overhead,
    // starting with the `Coverage` allocation right below, is what adds
    // up at that volume, not anything inside the loops). Bounding the
    // call *count* itself, not just work done inside any one call, is
    // what actually stops this on that input.
    if !budget.take_class_coverage_call() {
        return Coverage::new();
    }
    let mut cov = Coverage::new();
    // Called once per position of every rule, so every loop iteration below,
    // whether or not it pushes a glyph, is charged to the table's
    // `class_zero_glyphs` budget. Class 0 (every glyph not otherwise
    // classified) is found with a bitmap of the classified glyphs, built
    // once, instead of a scan per candidate.
    if cls as i32 == 0_i32 {
        let mut classified = vec![false; max_glyphs as usize];
        // `0..cd.glyphs.len()`: the bound is `cd.glyphs`'s own length,
        // fixed for this whole call (never mutated inside either loop),
        // and doesn't depend on `budget` -- the same range the `while`
        // walked one step at a time. `budget.class_zero_left()` only ever
        // causes an early `break`, checked first in the body, the same
        // position the `while`'s own budget check checked it
        // in; `budget.charge_class_zero()` stays unconditional and in the same
        // place, right before the loop variable would have advanced.
        for j in 0..cd.glyphs.len() {
            if !budget.class_zero_left() {
                break;
            }
            if cd.classes[j] as i32 > 0_i32 {
                let idx = cd.glyphs[j].index as usize;
                if idx < classified.len() {
                    classified[idx] = true;
                }
            }
            budget.charge_class_zero();
        }
        // `0..max_glyphs`: `max_glyphs` is a `GlyphId` (`u16`, so at most
        // 65535) fixed for the whole call, and, like the loop above,
        // doesn't depend on `budget` -- same range, same early-`break`
        // shape.
        for k in 0..max_glyphs {
            if !budget.class_zero_left() {
                break;
            }
            if !classified[k as usize] {
                push_to_coverage(&mut cov, handle_from_index(k) as GlyphHandle);
            }
            budget.charge_class_zero();
        }
    } else {
        // Kept as a `u16` `while` on purpose: a format 2 `ClassDef` can hold
        // all 65,536 glyphs, and then `j_2` wraps to 0 before reaching the
        // length, so the loop rescans until the budget runs out. A `for`
        // over the length would change what such an input produces.
        let mut j_2: GlyphId = 0 as GlyphId;
        while (j_2 as usize) < cd.glyphs.len() && budget.class_zero_left() {
            if cd.classes[j_2 as usize] as i32 == cls as i32 {
                push_to_coverage(
                    &mut cov,
                    cd.glyphs[j_2 as usize].clone(),
                );
            }
            budget.charge_class_zero();
            j_2 = j_2.wrapping_add(1);
        }
    }
    cov
}
pub fn format3_coverage(
    data: &[u8],
    shift: u16,
    mut _offset: u32,
    mut _kind: ContextKind,
    _max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Coverage {
    return read_coverage(data, _offset.wrapping_add(shift as u32).wrapping_sub(2_u32), budget);
}
// The `FontReader` reads and `require_room` checks below never reserve
// bytes the following reads do not use.
pub fn general_read_contextual_rule(
    slice: &[u8],
    offset: u32,
    start_gid: u16,
    minus_one: bool,
    mut coverage_of: impl FnMut(&[u8], u16, u32, ContextKind, GlyphId, &mut OtlReadBudget) -> Coverage,
    max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Box<ChainingRule>> {
    let minus_one_q: u16 = minus_one as u16;

    let mut header = FontReader::new(slice).at(offset as usize).ok()?;
    let n_input = header.u16().ok()?;
    let n_apply = header.u16().ok()?;
    // Reserves `2 * n_input` bytes even on the `minus_one` path, which reads
    // one entry fewer; kept so the same inputs are accepted.
    let needed = (n_input as usize) * 2 + (n_apply as usize) * 4;
    header.require_room(needed, 1).ok()?;

    // A malformed `n_input < minus_one_q` reads nothing, rather than
    // underflowing.
    let n_input_read = n_input.saturating_sub(minus_one_q);
    // Only the build loop below is capped (see `MAX_POSITIONS_PER_RULE`);
    // `n_input_read` still sets `lookup_base`.
    let n_input_built = n_input_read.min(MAX_POSITIONS_PER_RULE);
    let match_count = minus_one_q.wrapping_add(n_input_built);

    let mut rule: Box<ChainingRule> = Box::new(ChainingRule {
        match_count: match_count as TableId,
        input_begins: 0 as TableId,
        input_ends: match_count as TableId,
        sequence: Vec::new(),
        apply: Vec::new(),
    });
    // Filled in order: the `minus_one` slot first, then the rest.
    rule.sequence = Vec::with_capacity(rule.match_count as usize);
    if minus_one {
        rule.sequence
            .push(coverage_of(
                slice,
                start_gid,
                offset,
                ContextKind::Input,
                max_glyphs,
                budget,
            ));
    }
    for j in 0..n_input_built {
        let gid = FontReader::new(slice)
            .at(offset as usize + 4 + 2 * j as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.sequence
            .push(coverage_of(
                slice,
                gid,
                offset,
                ContextKind::Input,
                max_glyphs,
                budget,
            ));
    }

    rule.apply = Vec::with_capacity((n_apply as usize).min(MAX_APPLY_PER_RULE));
    let lookup_base = offset as usize + 4 + 2 * n_input_read as usize;
    for j0 in 0..n_apply.min(MAX_APPLY_PER_RULE as u16) {
        let mut lr = FontReader::new(slice)
            .at(lookup_base + 4 * j0 as usize)
            .unwrap();
        let seq_index = lr.u16().unwrap();
        let lookup_index = lr.u16().unwrap();
        let index = rule.input_begins.wrapping_add(seq_index);
        let lookup = handle_from_index(lookup_index) as LookupHandle;
        rule.apply.push(ChainLookupApplication { index, lookup });
    }
    reverse_backtracks(&mut rule);
    Some(rule)
}
fn read_contextual_format1(
    slice: &[u8],
    offset: u32,
    max_glyphs: GlyphId,
    mut subtable: Box<ChainingSubtable>,
    budget: &mut OtlReadBudget,
) -> Option<Box<ChainingSubtable>> {
    let result: Option<()> = 'parse: {
        let Ok(mut header) = FontReader::new(slice).at(offset as usize + 2) else {
            break 'parse None;
        };
        let Ok(cov_rel) = header.u16() else {
            break 'parse None;
        };
        let Ok(chain_sub_rule_set_count) = header.u16() else {
            break 'parse None;
        };
        let cov_offset = offset.wrapping_add(cov_rel as u32);
        let first_coverage: Coverage = read_coverage(slice, cov_offset, budget);
        if chain_sub_rule_set_count as usize != first_coverage.len() {
            break 'parse None;
        }
        if header
            .require_room(chain_sub_rule_set_count as usize, 2)
            .is_err()
        {
            break 'parse None;
        }

        // First pass: validate every ruleset's own header + rule-offset array.
        let mut total_rules: usize = 0;
        for j in 0..chain_sub_rule_set_count {
            let srs_rel = FontReader::new(slice)
                .at(offset as usize + 6 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            let srs_offset = offset.wrapping_add(srs_rel as u32);
            let Ok(mut srs_header) = FontReader::new(slice).at(srs_offset as usize) else {
                break 'parse None;
            };
            let Ok(srs_count) = srs_header.u16() else {
                break 'parse None;
            };
            if srs_header.require_room(srs_count as usize, 2).is_err() {
                break 'parse None;
            }
            total_rules = total_rules.saturating_add(srs_count as usize);
        }

        // Second pass: build, re-deriving each offset as the first pass did.
        let ruleset = chaining_ruleset_mut(&mut subtable);
        ruleset.rules = Vec::with_capacity(total_rules.min(MAX_TOTAL_RULES_PER_TABLE as usize));
        'rulesets: for j in 0..chain_sub_rule_set_count {
            let srs_rel = FontReader::new(slice)
                .at(offset as usize + 6 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            let srs_offset = offset.wrapping_add(srs_rel as u32);
            let srs_count = FontReader::new(slice)
                .at(srs_offset as usize)
                .unwrap()
                .u16()
                .unwrap();
            for k in 0..srs_count {
                if !budget.take_rule() {
                    break 'rulesets;
                }
                let sr_rel = FontReader::new(slice)
                    .at(srs_offset as usize + 2 + 2 * k as usize)
                    .unwrap()
                    .u16()
                    .unwrap();
                let sr_offset = srs_offset.wrapping_add(sr_rel as u32);
                let rule_ptr = general_read_contextual_rule(
                    slice,
                    sr_offset,
                    first_coverage[j as usize].index as u16,
                    true,
                    single_coverage,
                    max_glyphs,
                    budget,
                );
                // A `None` here means this one rule's own offset/header was
                // malformed (`general_read_contextual_rule`/`_chaining_rule`
                // returned early via `?`) -- the *outer* class-set/rule-set
                // array that pointed at it was still validated and fits the
                // table, so this is an isolated bad rule, not a reason to
                // fail the whole subtable. `unconsolidate_chaining` asserts
                // every slot here is `Some` (fuzzing found a font that
                // pushed a `None` and hit that `.expect()`), so drop it here
                // instead of ever storing a placeholder.
                if rule_ptr.is_some() {
                    ruleset.rules.push(rule_ptr);
                }
            }
        }
        break 'parse Some(());
    };

    if result.is_some() { Some(subtable) } else { None }
}
fn read_contextual_format2(
    slice: &[u8],
    offset: u32,
    max_glyphs: GlyphId,
    mut subtable: Box<ChainingSubtable>,
    budget: &mut OtlReadBudget,
) -> Option<Box<ChainingSubtable>> {
    let cds: Option<ClassDefs>;

    let result: Option<()> = 'parse: {
        let Ok(mut header) = FontReader::new(slice).at(offset as usize + 4) else {
            break 'parse None;
        };
        let Ok(ic_rel) = header.u16() else {
            break 'parse None;
        };
        let Ok(chain_sub_class_set_cnt) = header.u16() else {
            break 'parse None;
        };
        if header
            .require_room(chain_sub_class_set_cnt as usize, 2)
            .is_err()
        {
            break 'parse None;
        }

        cds = Some(ClassDefs {
            bc: None,
            ic: Some(Box::new(read_class_def(slice, offset.wrapping_add(ic_rel as u32)))),
            fc: None,
        });

        // First pass: check every non-empty class set's header and rule
        // offsets.
        let mut total_rules: usize = 0;
        for j in 0..chain_sub_class_set_cnt {
            let src_rel = FontReader::new(slice)
                .at(offset as usize + 8 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            if src_rel == 0 {
                continue;
            }
            let Ok(mut cs_header) = FontReader::new(slice).at(offset as usize + src_rel as usize)
            else {
                break 'parse None;
            };
            let Ok(srs_count) = cs_header.u16() else {
                break 'parse None;
            };
            if cs_header.require_room(srs_count as usize, 2).is_err() {
                break 'parse None;
            }
            total_rules = total_rules.saturating_add(srs_count as usize);
        }

        let ruleset = chaining_ruleset_mut(&mut subtable);
        ruleset.rules = Vec::with_capacity(total_rules.min(MAX_TOTAL_RULES_PER_TABLE as usize));
        'class_sets: for j in 0..chain_sub_class_set_cnt {
            let src_rel = FontReader::new(slice)
                .at(offset as usize + 8 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            if src_rel == 0 {
                continue;
            }
            let srs_count = FontReader::new(slice)
                .at(offset as usize + src_rel as usize)
                .unwrap()
                .u16()
                .unwrap();
            for k in 0..srs_count {
                if !budget.take_rule() {
                    break 'class_sets;
                }
                let sr_rel = FontReader::new(slice)
                    .at(offset as usize + src_rel as usize + 2 + 2 * k as usize)
                    .unwrap()
                    .u16()
                    .unwrap();
                let sr_offset = offset
                    .wrapping_add(src_rel as u32)
                    .wrapping_add(sr_rel as u32);
                let rule_ptr = general_read_contextual_rule(
                    slice,
                    sr_offset,
                    j,
                    true,
                    |slice, cls, offset, kind, max_glyphs, budget: &mut OtlReadBudget| {
                        class_coverage(slice, cls, offset, kind, max_glyphs, cds.as_ref().unwrap(), budget)
                    },
                    max_glyphs,
                    budget,
                );
                // A `None` here means this one rule's own offset/header was
                // malformed (`general_read_contextual_rule`/`_chaining_rule`
                // returned early via `?`) -- the *outer* class-set/rule-set
                // array that pointed at it was still validated and fits the
                // table, so this is an isolated bad rule, not a reason to
                // fail the whole subtable. `unconsolidate_chaining` asserts
                // every slot here is `Some` (fuzzing found a font that
                // pushed a `None` and hit that `.expect()`), so drop it here
                // instead of ever storing a placeholder.
                if rule_ptr.is_some() {
                    ruleset.rules.push(rule_ptr);
                }
            }
        }
        break 'parse Some(());
    };

    if result.is_some() { Some(subtable) } else { None }
}
pub fn otl_read_contextual(
    data: &[u8],
    offset: u32,
    max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    let mut subtable = Box::new(ChainingSubtable::Poly(ChainingRuleSet::default()));
    let mut format: u16 = 0_u16;
    if let Ok(mut r) = FontReader::new(data).at(offset as usize)
        && let Ok(f) = r.u16() {
            format = f;
        }
    match format {
        1 => {
            return read_contextual_format1(data, offset, max_glyphs, subtable, budget)
                .map(|s| Subtable::Chaining(*s));
        }
        2 => {
            return read_contextual_format2(data, offset, max_glyphs, subtable, budget)
                .map(|s| Subtable::Chaining(*s));
        }
        3 => {
            let rule_ptr = general_read_contextual_rule(
                data,
                offset.wrapping_add(2_u32),
                0_u16,
                false,
                format3_coverage,
                max_glyphs,
                budget,
            );
            // Same "malformed individual rule, not the whole subtable" case
            // as the format1/format2 loops above -- see their comment.
            if rule_ptr.is_some() {
                chaining_ruleset_mut(&mut subtable).rules.push(rule_ptr);
            }
            return Some(Subtable::Chaining(*subtable));
        }
        _ => {}
    }
    tracing::warn!("Unsupported format {}.\n", format as i32);
    // `subtable` (still just a local `Box`, never adopted into anything)
    // self-drops here -- no manual reclamation needed any more.
    None
}
pub fn general_read_chaining_rule(
    slice: &[u8],
    offset: u32,
    start_gid: u16,
    minus_one: bool,
    mut coverage_of: impl FnMut(&[u8], u16, u32, ContextKind, GlyphId, &mut OtlReadBudget) -> Coverage,
    max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Box<ChainingRule>> {
    let minus_one_q: u16 = minus_one as u16;

    // Four counts, each followed by the array it introduces: backtrack,
    // input (without the `minus_one` slot), lookahead, then the lookup
    // records. Each read checks the bytes it needs.
    let mut header = FontReader::new(slice).at(offset as usize).ok()?;
    let n_back = header.u16().ok()?;
    header.require_room(n_back as usize, 2).ok()?;
    header.skip(n_back as usize * 2).ok()?;
    let n_input = header.u16().ok()?;
    let n_input_read = n_input.saturating_sub(minus_one_q);
    header.require_room(n_input_read as usize, 2).ok()?;
    header.skip(n_input_read as usize * 2).ok()?;
    let n_lookaround = header.u16().ok()?;
    header.require_room(n_lookaround as usize, 2).ok()?;
    header.skip(n_lookaround as usize * 2).ok()?;
    let n_apply = header.u16().ok()?;
    header.require_room(n_apply as usize, 4).ok()?;

    // Only the build loops below are capped (see `MAX_POSITIONS_PER_RULE`);
    // the uncapped counts still locate the arrays. `match_count`,
    // `input_begins` and `input_ends` come from the capped counts, so they
    // always agree with `sequence`'s length.
    let n_back_built = n_back.min(MAX_POSITIONS_PER_RULE);
    let n_input_built = n_input_read.min(MAX_POSITIONS_PER_RULE);
    let n_lookaround_built = n_lookaround.min(MAX_POSITIONS_PER_RULE);
    let input_begins = n_back_built;
    let input_ends = n_back_built.wrapping_add(minus_one_q).wrapping_add(n_input_built);
    let match_count = input_ends.wrapping_add(n_lookaround_built);
    let mut rule: Box<ChainingRule> = Box::new(ChainingRule {
        match_count: match_count as TableId,
        input_begins,
        input_ends: input_ends as TableId,
        sequence: Vec::new(),
        apply: Vec::new(),
    });
    // Filled in order: backtrack, the `minus_one` slot, input, lookahead.
    rule.sequence = Vec::with_capacity(match_count as usize);
    for j in 0..n_back_built {
        let gid = FontReader::new(slice)
            .at(offset as usize + 2 + 2 * j as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.sequence
            .push(coverage_of(
                slice,
                gid,
                offset,
                ContextKind::Backtrack,
                max_glyphs,
                budget,
            ));
    }
    if minus_one {
        rule.sequence
            .push(coverage_of(
                slice,
                start_gid,
                offset,
                ContextKind::Input,
                max_glyphs,
                budget,
            ));
    }
    // Array positions from the counts as read, by addition, so a malformed
    // `n_input < minus_one_q` cannot underflow.
    let input_base = offset as usize + 4 + 2 * n_back as usize;
    for j0 in 0..n_input_built {
        let gid = FontReader::new(slice)
            .at(input_base + 2 * j0 as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.sequence
            .push(coverage_of(
                slice,
                gid,
                offset,
                ContextKind::Input,
                max_glyphs,
                budget,
            ));
    }
    let lookaround_base = input_base + 2 * n_input_read as usize + 2;
    for j1 in 0..n_lookaround_built {
        let gid = FontReader::new(slice)
            .at(lookaround_base + 2 * j1 as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.sequence
            .push(coverage_of(
                slice,
                gid,
                offset,
                ContextKind::Lookahead,
                max_glyphs,
                budget,
            ));
    }

    rule.apply = Vec::with_capacity((n_apply as usize).min(MAX_APPLY_PER_RULE));
    let apply_base = lookaround_base + 2 * n_lookaround as usize + 2;
    for j2 in 0..n_apply.min(MAX_APPLY_PER_RULE as u16) {
        let mut lr = FontReader::new(slice)
            .at(apply_base + 4 * j2 as usize)
            .unwrap();
        let seq_index = lr.u16().unwrap();
        let lookup_index = lr.u16().unwrap();
        let index = rule.input_begins.wrapping_add(seq_index);
        let lookup = handle_from_index(lookup_index) as LookupHandle;
        rule.apply.push(ChainLookupApplication { index, lookup });
    }
    reverse_backtracks(&mut rule);
    Some(rule)
}
fn read_chaining_format1(
    slice: &[u8],
    offset: u32,
    max_glyphs: GlyphId,
    mut subtable: Box<ChainingSubtable>,
    budget: &mut OtlReadBudget,
) -> Option<Box<ChainingSubtable>> {
    let result: Option<()> = 'parse: {
        let Ok(mut header) = FontReader::new(slice).at(offset as usize + 2) else {
            break 'parse None;
        };
        let Ok(cov_rel) = header.u16() else {
            break 'parse None;
        };
        let Ok(chain_sub_rule_set_count) = header.u16() else {
            break 'parse None;
        };
        let cov_offset = offset.wrapping_add(cov_rel as u32);
        let first_coverage: Coverage = read_coverage(slice, cov_offset, budget);
        if chain_sub_rule_set_count as usize != first_coverage.len() {
            break 'parse None;
        }
        if header
            .require_room(chain_sub_rule_set_count as usize, 2)
            .is_err()
        {
            break 'parse None;
        }

        let mut total_rules: usize = 0;
        for j in 0..chain_sub_rule_set_count {
            let srs_rel = FontReader::new(slice)
                .at(offset as usize + 6 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            let srs_offset = offset.wrapping_add(srs_rel as u32);
            let Ok(mut srs_header) = FontReader::new(slice).at(srs_offset as usize) else {
                break 'parse None;
            };
            let Ok(srs_count) = srs_header.u16() else {
                break 'parse None;
            };
            if srs_header.require_room(srs_count as usize, 2).is_err() {
                break 'parse None;
            }
            total_rules = total_rules.saturating_add(srs_count as usize);
        }

        let ruleset = chaining_ruleset_mut(&mut subtable);
        ruleset.rules = Vec::with_capacity(total_rules.min(MAX_TOTAL_RULES_PER_TABLE as usize));
        'rulesets: for j in 0..chain_sub_rule_set_count {
            let srs_rel = FontReader::new(slice)
                .at(offset as usize + 6 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            let srs_offset = offset.wrapping_add(srs_rel as u32);
            let srs_count = FontReader::new(slice)
                .at(srs_offset as usize)
                .unwrap()
                .u16()
                .unwrap();
            for k in 0..srs_count {
                if !budget.take_rule() {
                    break 'rulesets;
                }
                let sr_rel = FontReader::new(slice)
                    .at(srs_offset as usize + 2 + 2 * k as usize)
                    .unwrap()
                    .u16()
                    .unwrap();
                let sr_offset = srs_offset.wrapping_add(sr_rel as u32);
                let rule_ptr = general_read_chaining_rule(
                    slice,
                    sr_offset,
                    first_coverage[j as usize].index as u16,
                    true,
                    single_coverage,
                    max_glyphs,
                    budget,
                );
                // A `None` here means this one rule's own offset/header was
                // malformed (`general_read_contextual_rule`/`_chaining_rule`
                // returned early via `?`) -- the *outer* class-set/rule-set
                // array that pointed at it was still validated and fits the
                // table, so this is an isolated bad rule, not a reason to
                // fail the whole subtable. `unconsolidate_chaining` asserts
                // every slot here is `Some` (fuzzing found a font that
                // pushed a `None` and hit that `.expect()`), so drop it here
                // instead of ever storing a placeholder.
                if rule_ptr.is_some() {
                    ruleset.rules.push(rule_ptr);
                }
            }
        }
        break 'parse Some(());
    };

    if result.is_some() { Some(subtable) } else { None }
}
fn read_chaining_format2(
    slice: &[u8],
    offset: u32,
    max_glyphs: GlyphId,
    mut subtable: Box<ChainingSubtable>,
    budget: &mut OtlReadBudget,
) -> Option<Box<ChainingSubtable>> {
    let cds: Option<ClassDefs>;

    let result: Option<()> = 'parse: {
        let Ok(mut header) = FontReader::new(slice).at(offset as usize + 4) else {
            break 'parse None;
        };
        let Ok(bc_rel) = header.u16() else {
            break 'parse None;
        };
        let Ok(ic_rel) = header.u16() else {
            break 'parse None;
        };
        let Ok(fc_rel) = header.u16() else {
            break 'parse None;
        };
        let Ok(chain_sub_class_set_cnt) = header.u16() else {
            break 'parse None;
        };
        if header
            .require_room(chain_sub_class_set_cnt as usize, 2)
            .is_err()
        {
            break 'parse None;
        }

        cds = Some(ClassDefs {
            bc: Some(Box::new(read_class_def(slice, offset.wrapping_add(bc_rel as u32)))),
            ic: Some(Box::new(read_class_def(slice, offset.wrapping_add(ic_rel as u32)))),
            fc: Some(Box::new(read_class_def(slice, offset.wrapping_add(fc_rel as u32)))),
        });

        // First pass: check every non-empty class set's header and rule
        // offsets.
        let mut total_rules: usize = 0;
        for j in 0..chain_sub_class_set_cnt {
            let src_rel = FontReader::new(slice)
                .at(offset as usize + 12 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            if src_rel == 0 {
                continue;
            }
            let Ok(mut cs_header) = FontReader::new(slice).at(offset as usize + src_rel as usize)
            else {
                break 'parse None;
            };
            let Ok(srs_count) = cs_header.u16() else {
                break 'parse None;
            };
            if cs_header.require_room(srs_count as usize, 2).is_err() {
                break 'parse None;
            }
            total_rules = total_rules.saturating_add(srs_count as usize);
        }

        let ruleset = chaining_ruleset_mut(&mut subtable);
        ruleset.rules = Vec::with_capacity(total_rules.min(MAX_TOTAL_RULES_PER_TABLE as usize));
        'class_sets: for j in 0..chain_sub_class_set_cnt {
            let src_rel = FontReader::new(slice)
                .at(offset as usize + 12 + 2 * j as usize)
                .unwrap()
                .u16()
                .unwrap();
            if src_rel == 0 {
                continue;
            }
            let srs_count = FontReader::new(slice)
                .at(offset as usize + src_rel as usize)
                .unwrap()
                .u16()
                .unwrap();
            for k in 0..srs_count {
                if !budget.take_rule() {
                    break 'class_sets;
                }
                let dsr_rel = FontReader::new(slice)
                    .at(offset as usize + src_rel as usize + 2 + 2 * k as usize)
                    .unwrap()
                    .u16()
                    .unwrap();
                let sr_offset = offset
                    .wrapping_add(src_rel as u32)
                    .wrapping_add(dsr_rel as u32);
                let rule_ptr = general_read_chaining_rule(
                    slice,
                    sr_offset,
                    j,
                    true,
                    |slice, cls, offset, kind, max_glyphs, budget: &mut OtlReadBudget| {
                        class_coverage(slice, cls, offset, kind, max_glyphs, cds.as_ref().unwrap(), budget)
                    },
                    max_glyphs,
                    budget,
                );
                // A `None` here means this one rule's own offset/header was
                // malformed (`general_read_contextual_rule`/`_chaining_rule`
                // returned early via `?`) -- the *outer* class-set/rule-set
                // array that pointed at it was still validated and fits the
                // table, so this is an isolated bad rule, not a reason to
                // fail the whole subtable. `unconsolidate_chaining` asserts
                // every slot here is `Some` (fuzzing found a font that
                // pushed a `None` and hit that `.expect()`), so drop it here
                // instead of ever storing a placeholder.
                if rule_ptr.is_some() {
                    ruleset.rules.push(rule_ptr);
                }
            }
        }
        break 'parse Some(());
    };

    // `cds` (a plain `Option<ClassDefs>` local, same as `read_contextual_
    // format2`) auto-drops when this function returns -- no manual
    // cleanup needed.
    if result.is_some() { Some(subtable) } else { None }
}
pub fn otl_read_chaining(
    data: &[u8],
    offset: u32,
    max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    // See the identical comment in `otl_read_contextual`.
    let mut subtable = Box::new(ChainingSubtable::Poly(ChainingRuleSet::default()));
    let mut format: u16 = 0_u16;
    if let Ok(mut r) = FontReader::new(data).at(offset as usize)
        && let Ok(f) = r.u16() {
            format = f;
        }
    match format {
        1 => {
            return read_chaining_format1(data, offset, max_glyphs, subtable, budget)
                .map(|s| Subtable::Chaining(*s));
        }
        2 => {
            return read_chaining_format2(data, offset, max_glyphs, subtable, budget)
                .map(|s| Subtable::Chaining(*s));
        }
        3 => {
            let rule_ptr = general_read_chaining_rule(
                data,
                offset.wrapping_add(2_u32),
                0_u16,
                false,
                format3_coverage,
                max_glyphs,
                budget,
            );
            // Same "malformed individual rule, not the whole subtable" case
            // as the format1/format2 loops above -- see their comment.
            if rule_ptr.is_some() {
                chaining_ruleset_mut(&mut subtable).rules.push(rule_ptr);
            }
            return Some(Subtable::Chaining(*subtable));
        }
        _ => {}
    }
    tracing::warn!("Unsupported format {}.\n", format as i32);
    // `subtable` (still just a local `Box`, never adopted into anything)
    // self-drops here -- no manual reclamation needed any more.
    None
}
#[inline]
// The font stores the backtrack sequence nearest glyph first; put it in
// reading order.
fn reverse_backtracks(rule: &mut ChainingRule) {
    let input_begins = rule.input_begins as usize;
    rule.sequence[..input_begins].reverse();
}

#[cfg(test)]
mod chaining_read_tests {
    use super::*;

    fn glyphs_of(cov: &Coverage) -> Vec<GlyphId> {
        cov.iter().map(|h| h.index).collect()
    }

    #[test]
    fn class_coverage_cls_zero_budget_stops_mid_scan_at_the_exact_boundary() {
        // Pins both of `class_coverage`'s `cls == 0` loops -- the
        // classified-bitmap build and the unclassified-glyph push. Both
        // loops draw from the same `class_zero_glyphs` budget, charged
        // unconditionally once per iteration regardless of which loop, so
        // a small combined budget must exhaust across the two loops in
        // order: the first loop (3 glyphs) fully completes, leaving
        // exactly 4 units for the second loop (`max_glyphs == 10`), which
        // must then stop after processing indices `0..=3` and leave index
        // `4` untouched.
        let mut budget = OtlReadBudget { class_zero_glyphs: 7, ..OtlReadBudget::new() };

        let cd = ClassDef {
            maxclass: 1,
            glyphs: vec![
                handle_from_index(2) as GlyphHandle,
                handle_from_index(5) as GlyphHandle,
                handle_from_index(8) as GlyphHandle,
            ],
            classes: vec![1, 1, 1],
        };
        let defs = ClassDefs {
            bc: Some(Box::new(cd)),
            ic: None,
            fc: None,
        };

        let cov = class_coverage(&[], 0, 0, ContextKind::Backtrack, 10, &defs, &mut budget);

        assert_eq!(
            budget.class_zero_glyphs, 0,
            "the shared budget must be fully consumed across both loops"
        );
        assert_eq!(
            glyphs_of(&cov),
            vec![0, 1, 3],
            "unclassified glyphs 0, 1, 3 pushed in order -- index 2 is \
             skipped (classified in the first loop), and index 4 is never \
             reached (budget ran out after processing 0..=3)"
        );
    }

    #[test]
    fn context_format3_reads_a_single_glyph_based_rule() {
        // format=3, nInput=1, nApply=0, inputArray[0] = coverage shift (10,
        // relative to the format3 rule's own subtable start), coverage
        // table (format1, one glyph) at byte 10.
        let mut data = [0u8; 16];
        data[0..2].copy_from_slice(&3u16.to_be_bytes());
        data[2..4].copy_from_slice(&1u16.to_be_bytes()); // nInput
        data[4..6].copy_from_slice(&0u16.to_be_bytes()); // nApply
        data[6..8].copy_from_slice(&10u16.to_be_bytes()); // shift -> byte 10
        data[10..12].copy_from_slice(&1u16.to_be_bytes()); // coverage format 1
        data[12..14].copy_from_slice(&1u16.to_be_bytes()); // glyphCount
        data[14..16].copy_from_slice(&42u16.to_be_bytes()); // glyph
        let sub = otl_read_contextual(&data, 0, 100, &mut OtlReadBudget::new()).unwrap();
        let Subtable::Chaining(ref sub) = sub else {
            unreachable!()
        };
        let ChainingSubtable::Poly(ruleset) = sub else {
            unreachable!()
        };
        assert_eq!(ruleset.rules.len(), 1);
        let rule = ruleset.rules[0].as_ref().unwrap();
        assert_eq!(rule.sequence.len(), 1);
        assert_eq!(glyphs_of(&rule.sequence[0]), vec![42]);
        assert!(rule.apply.is_empty());
    }

    #[test]
    fn context_format1_reads_one_rule_set_with_one_rule() {
        // format=1, coverageOffset -> 16 (one glyph, id 5),
        // chainSubRuleSetCount=1, srsOffset[0] -> 8.
        // ChainSubRuleSet at 8: count=1, ruleOffset[0] -> 4 (abs 12).
        // ChainSubRule at 12 (minus_one=true): nInput=1 (the coverage's own
        // glyph fills the one slot), nApply=0.
        let mut data = [0u8; 22];
        data[0..2].copy_from_slice(&1u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&16u16.to_be_bytes()); // coverageOffset
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // chainSubRuleSetCount
        data[6..8].copy_from_slice(&8u16.to_be_bytes()); // srsOffset[0]
        data[8..10].copy_from_slice(&1u16.to_be_bytes()); // srs_count
        data[10..12].copy_from_slice(&4u16.to_be_bytes()); // ruleOffset[0] (rel. to 8 -> 12)
        data[12..14].copy_from_slice(&1u16.to_be_bytes()); // nInput
        data[14..16].copy_from_slice(&0u16.to_be_bytes()); // nApply
        data[16..18].copy_from_slice(&1u16.to_be_bytes()); // coverage format 1
        data[18..20].copy_from_slice(&1u16.to_be_bytes()); // glyphCount
        data[20..22].copy_from_slice(&5u16.to_be_bytes()); // glyph
        let sub = otl_read_contextual(&data, 0, 100, &mut OtlReadBudget::new()).unwrap();
        let Subtable::Chaining(ref sub) = sub else {
            unreachable!()
        };
        let ChainingSubtable::Poly(ruleset) = sub else {
            unreachable!()
        };
        assert_eq!(ruleset.rules.len(), 1);
        let rule = ruleset.rules[0].as_ref().unwrap();
        assert_eq!(rule.match_count, 1);
        assert_eq!(glyphs_of(&rule.sequence[0]), vec![5]);
    }

    #[test]
    fn context_format1_rule_set_count_mismatched_with_coverage_is_rejected() {
        // chainSubRuleSetCount (2) doesn't match the coverage's glyph
        // count (1) -- a consistency check upstream otfcc also makes.
        let mut data = [0u8; 12];
        data[0..2].copy_from_slice(&1u16.to_be_bytes());
        data[2..4].copy_from_slice(&6u16.to_be_bytes()); // coverageOffset -> 6
        data[4..6].copy_from_slice(&2u16.to_be_bytes()); // count = 2
        data[6..8].copy_from_slice(&1u16.to_be_bytes()); // coverage format 1
        data[8..10].copy_from_slice(&1u16.to_be_bytes()); // glyphCount = 1
        data[10..12].copy_from_slice(&9u16.to_be_bytes());
        assert!(otl_read_contextual(&data, 0, 100, &mut OtlReadBudget::new()).is_none());
    }

    #[test]
    fn context_format2_class_set_offset_past_the_table_end_is_rejected_instead_of_reading_oob() {
        // `srs_count` (and its rule-offset array) is read at
        // `offset + classSetOffset[j]`, which must be inside the table.
        // `classSetOffset[0]` here (5000) is far past this 10-byte table.
        let mut data = [0u8; 10];
        data[0..2].copy_from_slice(&2u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&0u16.to_be_bytes()); // unused field
        data[4..6].copy_from_slice(&10u16.to_be_bytes()); // classDefOffset (past end, handled gracefully)
        data[6..8].copy_from_slice(&1u16.to_be_bytes()); // chainSubClassSetCnt
        data[8..10].copy_from_slice(&5000u16.to_be_bytes()); // classSetOffset[0]
        assert!(otl_read_contextual(&data, 0, 100, &mut OtlReadBudget::new()).is_none());
    }

    #[test]
    fn context_format2_zero_class_set_offset_is_skipped() {
        // classSetOffset == 0 is a documented "no ruleset for this class"
        // sentinel, not a real offset -- must not be dereferenced.
        let mut data = [0u8; 10];
        data[0..2].copy_from_slice(&2u16.to_be_bytes());
        data[2..4].copy_from_slice(&0u16.to_be_bytes());
        data[4..6].copy_from_slice(&10u16.to_be_bytes());
        data[6..8].copy_from_slice(&1u16.to_be_bytes());
        data[8..10].copy_from_slice(&0u16.to_be_bytes()); // classSetOffset[0] = 0
        let sub = otl_read_contextual(&data, 0, 100, &mut OtlReadBudget::new()).unwrap();
        let Subtable::Chaining(ref sub) = sub else {
            unreachable!()
        };
        let ChainingSubtable::Poly(ruleset) = sub else {
            unreachable!()
        };
        assert!(ruleset.rules.is_empty());
    }

    #[test]
    fn chaining_format3_reads_backtrack_input_and_lookahead() {
        // format=3, nBack=1, backtrack shift -> byte 20 (glyph 1),
        // nInput=1, input shift -> byte 26 (glyph 2), nLookaround=1,
        // lookaround shift -> byte 32 (glyph 3), nApply=0.
        let mut data = [0u8; 38];
        data[0..2].copy_from_slice(&3u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&1u16.to_be_bytes()); // nBack
        data[4..6].copy_from_slice(&20u16.to_be_bytes()); // backtrack shift
        data[6..8].copy_from_slice(&1u16.to_be_bytes()); // nInput
        data[8..10].copy_from_slice(&26u16.to_be_bytes()); // input shift
        data[10..12].copy_from_slice(&1u16.to_be_bytes()); // nLookaround
        data[12..14].copy_from_slice(&32u16.to_be_bytes()); // lookaround shift
        data[14..16].copy_from_slice(&0u16.to_be_bytes()); // nApply
        // Coverage tables (format 1, one glyph each) for the three shifts.
        // Each `format3_coverage` call resolves to `offset + shift - 2`
        // where `offset` is `general_read_chaining_rule`'s own offset
        // (the dispatch's `offset + 2`, i.e. `2` here).
        data[20..22].copy_from_slice(&1u16.to_be_bytes());
        data[22..24].copy_from_slice(&1u16.to_be_bytes());
        data[24..26].copy_from_slice(&1u16.to_be_bytes()); // backtrack glyph
        data[26..28].copy_from_slice(&1u16.to_be_bytes());
        data[28..30].copy_from_slice(&1u16.to_be_bytes());
        data[30..32].copy_from_slice(&2u16.to_be_bytes()); // input glyph
        data[32..34].copy_from_slice(&1u16.to_be_bytes());
        data[34..36].copy_from_slice(&1u16.to_be_bytes());
        data[36..38].copy_from_slice(&3u16.to_be_bytes()); // lookaround glyph
        let sub = otl_read_chaining(&data, 0, 100, &mut OtlReadBudget::new()).unwrap();
        let Subtable::Chaining(ref sub) = sub else {
            unreachable!()
        };
        let ChainingSubtable::Poly(ruleset) = sub else {
            unreachable!()
        };
        assert_eq!(ruleset.rules.len(), 1);
        let rule = ruleset.rules[0].as_ref().unwrap();
        // backtrack is stored reversed; here there's only one entry so
        // the order is unaffected.
        assert_eq!(
            rule.sequence
                .iter()
                .map(glyphs_of)
                .collect::<Vec<_>>(),
            vec![vec![1], vec![2], vec![3]]
        );
        assert_eq!(rule.input_begins, 1);
        assert_eq!(rule.input_ends, 2);
    }

    #[test]
    fn chaining_rule_with_input_count_below_the_minus_one_slot_does_not_panic() {
        // A malformed `nInput` of 0 while this call site always wants the
        // `minus_one` (coverage-implied) slot filled: `nInput - minus_one`
        // must give zero array reads rather than panic on `u16` underflow.
        // Reached via `general_read_chaining_rule` directly since
        // `read_chaining_format1` (the real caller of this path) also
        // requires a fully valid, consistent outer coverage/ruleset
        // structure this test isn't trying to build.
        let mut data = [0u8; 10];
        data[0..2].copy_from_slice(&0u16.to_be_bytes()); // nBack
        data[2..4].copy_from_slice(&0u16.to_be_bytes()); // nInput (malformed: 0)
        data[4..6].copy_from_slice(&0u16.to_be_bytes()); // nLookaround
        data[6..8].copy_from_slice(&0u16.to_be_bytes()); // nApply
        let rule = general_read_chaining_rule(&data, 0, 7, true, single_coverage, 100, &mut OtlReadBudget::new());
        let rule = rule.unwrap();
        // Only the `minus_one` slot (glyph 7, from `start_gid`) is
        // filled; the (empty) input array contributes nothing.
        assert_eq!(rule.sequence.len(), 1);
        assert_eq!(glyphs_of(&rule.sequence[0]), vec![7]);
    }

    #[test]
    fn unsupported_format_logs_and_returns_null() {
        let data = [0u8, 9]; // format = 9
        // This path logs unconditionally, so `options.logger` must be a
        // real, usable `Logger`, not null -- automatic now that `Options::
        // default()`'s `logger` is a real (if `LoggerTarget::Empty`, i.e.
        // no-op-push) `Logger` rather than a null pointer.
        assert!(otl_read_contextual(&data, 0, 100, &mut OtlReadBudget::new()).is_none());
    }
}
