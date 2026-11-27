//! Reading contextual and chaining contextual subtables of all three
//! formats into rule sets, within the table's `OtlReadBudget`.

use crate::support::handle::{GlyphHandle, LookupHandle, handle_from_index};
use crate::table::otl::budget::{Limit, OtlReadBudget};
use crate::table::otl::classdef::{ClassDef, read_class_def};
use crate::table::otl::coverage::{Coverage, push_to_coverage, read_coverage};

use otfcc_binary::FontReader;

use crate::support::primitives::{GlyphClass, GlyphId, TableId};

use crate::table::otl::subtables::chaining::common::chaining_ruleset_mut;
use crate::table::otl::{
    ChainLookupApplication, ChainingRule, ChainingRuleSet, ChainingSubtable, Subtable,
};
/// The backtrack, input and lookahead class definitions of a format 2
/// subtable, which `class_coverage` turns into coverages. A contextual
/// subtable has only `ic`.
#[derive(Debug)]
struct ClassDefs {
    bc: Option<Box<ClassDef>>,
    ic: Option<Box<ClassDef>>,
    fc: Option<Box<ClassDef>>,
}
/// The GSUB or GPOS table being read, the font's glyph count, and the
/// table-wide allowance the readers draw from. Every reader of this file
/// takes it in place of those three values.
struct TableReader<'a> {
    data: &'a [u8],
    max_glyphs: GlyphId,
    budget: &'a mut OtlReadBudget,
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
/// 500,000 rules.
///
/// This used to be 15,000, which is what bounded memory, as 15,000 rules of
/// up to 150 positions each. Real fonts have more rules than that: Gulzar's
/// GSUB has 27,937, and lost the rest. The positions are what cost memory,
/// so they have a budget of their own now
/// (`MAX_TOTAL_RULE_POSITIONS_PER_TABLE`, the same 2,250,000), and the rule
/// count only needs to stop the per-rule overhead: 100,000 is over three
/// times Gulzar's.
pub(crate) const MAX_TOTAL_RULES_PER_TABLE: u32 = 100_000;
/// Bounds the glyph positions built across every contextual/chaining rule of
/// a whole table (`OtlReadBudget::rule_positions`): 15,000 rules of up to
/// 150 positions, the most the earlier rule limit allowed. Real fonts use a
/// fraction of it (Gulzar's GSUB about 150,000, Noto Nastaliq Urdu's about
/// 60,000). A rule that does not fit is dropped whole.
pub(crate) const MAX_TOTAL_RULE_POSITIONS_PER_TABLE: u32 = 2_250_000;
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
enum ContextKind {
    Backtrack = 1,
    Input = 2,
    Lookahead = 3,
}
/// The coverage of one position of a format 1 rule, which stores glyph ids:
/// just the glyph `gid`.
fn single_coverage(gid: u16) -> Coverage {
    let mut cov = Coverage::new();
    push_to_coverage(&mut cov, handle_from_index(gid) as GlyphHandle);
    cov
}
/// The coverage of one position of a format 2 rule, which stores class
/// numbers: the glyphs given class `cls` by the backtrack, input or
/// lookahead class definition, chosen by `kind`. Class 0 is every glyph
/// below `t.max_glyphs` that the class definition gives no other class. The
/// work is charged to `t.budget`; once it runs out, the coverage is cut
/// short.
fn class_coverage(
    t: &mut TableReader<'_>,
    cls: u16,
    kind: ContextKind,
    defs: &ClassDefs,
) -> Coverage {
    let max_glyphs = t.max_glyphs;
    let budget = &mut *t.budget;
    // A contextual format 2 subtable sets only `ic`, but its rules only ask
    // for `ContextKind::Input`; a chaining one sets all three.
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
    if i32::from(cls) == 0 {
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
            if i32::from(cd.classes[j]) > 0 {
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
            if i32::from(cd.classes[j_2 as usize]) == i32::from(cls) {
                push_to_coverage(&mut cov, cd.glyphs[j_2 as usize].clone());
            }
            budget.charge_class_zero();
            j_2 = j_2.wrapping_add(1);
        }
    }
    cov
}
/// The coverage of one position of a format 3 rule, which stores offsets to
/// Coverage tables. `shift` is relative to the subtable, and `rule_offset`
/// is the rule's position, two bytes past the subtable's start.
fn format3_coverage(t: &mut TableReader<'_>, shift: u16, rule_offset: u32) -> Coverage {
    read_coverage(
        t.data,
        rule_offset.wrapping_add(u32::from(shift)).wrapping_sub(2),
        t.budget,
    )
}
/// Reads one contextual rule (input sequence and lookup records) at
/// `offset`, turning each stored input value into a coverage with
/// `coverage_of`. In formats 1 and 2 the first input position is not
/// stored; `first_input` gives its coverage: the covered glyph, or the
/// covered glyphs of the class, that selected this rule set. Returns `None`
/// if the rule does not fit in the table.
// The `FontReader` reads and `require_room` checks below never reserve
// bytes the following reads do not use.
fn general_read_contextual_rule(
    t: &mut TableReader<'_>,
    offset: u32,
    first_input: Option<Coverage>,
    mut coverage_of: impl FnMut(&mut TableReader<'_>, u16, u32, ContextKind) -> Coverage,
) -> Option<Box<ChainingRule>> {
    let slice = t.data;
    // 1 when the first input position comes from `first_input`, else 0.
    let implied_inputs: u16 = u16::from(first_input.is_some());

    let mut header = FontReader::new(slice).at(offset as usize).ok()?;
    let n_input = header.u16().ok()?;
    let n_apply = header.u16().ok()?;
    // Reserves `2 * n_input` bytes even when `first_input` is given, which reads
    // one entry fewer; kept so the same inputs are accepted.
    let needed = (n_input as usize) * 2 + (n_apply as usize) * 4;
    header.require_room(needed, 1).ok()?;

    // A malformed `n_input < implied_inputs` reads nothing, rather than
    // underflowing.
    let n_input_read = n_input.saturating_sub(implied_inputs);
    // Only the build loop below is capped (see `MAX_POSITIONS_PER_RULE`);
    // `n_input_read` still sets `lookup_base`.
    let n_input_built = n_input_read.min(MAX_POSITIONS_PER_RULE);
    if n_input_built < n_input_read {
        t.budget.reached(Limit::RulePositions);
    }
    // See `MAX_TOTAL_RULE_POSITIONS_PER_TABLE`.
    if !t.budget.try_spend_rule_positions(u32::from(n_input_built)) {
        return None;
    }
    let match_count = implied_inputs.wrapping_add(n_input_built);

    let mut rule: Box<ChainingRule> = Box::new(ChainingRule {
        match_count: match_count as TableId,
        input_begins: 0 as TableId,
        input_ends: match_count as TableId,
        sequence: Vec::new(),
        apply: Vec::new(),
    });
    // Filled in order: the `first_input` slot first, then the rest.
    rule.sequence = Vec::with_capacity(rule.match_count as usize);
    if let Some(first) = first_input {
        rule.sequence.push(first);
    }
    for j in 0..n_input_built {
        let gid = u16_at(slice, offset as usize + 4 + 2 * j as usize);
        rule.sequence
            .push(coverage_of(t, gid, offset, ContextKind::Input));
    }

    if n_apply as usize > MAX_APPLY_PER_RULE {
        t.budget.reached(Limit::RuleApplications);
    }
    rule.apply = Vec::with_capacity((n_apply as usize).min(MAX_APPLY_PER_RULE));
    let lookup_base = offset as usize + 4 + 2 * n_input_read as usize;
    for j0 in 0..n_apply.min(MAX_APPLY_PER_RULE as u16) {
        let lookup_pos = lookup_base + 4 * j0 as usize;
        let seq_index = u16_at(slice, lookup_pos);
        let lookup_index = u16_at(slice, lookup_pos + 2);
        let index = rule.input_begins.wrapping_add(seq_index);
        let lookup = handle_from_index(lookup_index) as LookupHandle;
        rule.apply.push(ChainLookupApplication { index, lookup });
    }
    reverse_backtracks(&mut rule);
    Some(rule)
}
/// The big-endian `u16` at byte `pos` of `slice`, for positions that an
/// earlier `require_room` pass has already shown to lie inside the table.
/// A failure here would be a bug in that pass, not bad input.
fn u16_at(slice: &[u8], pos: usize) -> u16 {
    FontReader::new(slice)
        .at(pos)
        .and_then(|mut r| r.u16())
        .expect("the position was bounds-checked by an earlier require_room pass")
}
/// Reads one chaining rule (backtrack, input and lookahead sequences, then
/// lookup records) at `offset`; otherwise the same as
/// `general_read_contextual_rule`. The backtrack sequence is stored in
/// reading order, not the font's nearest-first order.
fn general_read_chaining_rule(
    t: &mut TableReader<'_>,
    offset: u32,
    first_input: Option<Coverage>,
    mut coverage_of: impl FnMut(&mut TableReader<'_>, u16, u32, ContextKind) -> Coverage,
) -> Option<Box<ChainingRule>> {
    let slice = t.data;
    let implied_inputs: u16 = u16::from(first_input.is_some());

    // Four counts, each followed by the array it introduces: backtrack,
    // input (without the `first_input` slot), lookahead, then the lookup
    // records. Each read checks the bytes it needs.
    let mut header = FontReader::new(slice).at(offset as usize).ok()?;
    let n_back = header.u16().ok()?;
    header.require_room(n_back as usize, 2).ok()?;
    header.skip(n_back as usize * 2).ok()?;
    let n_input = header.u16().ok()?;
    let n_input_read = n_input.saturating_sub(implied_inputs);
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
    if n_back_built < n_back || n_input_built < n_input_read || n_lookaround_built < n_lookaround {
        t.budget.reached(Limit::RulePositions);
    }
    // See `MAX_TOTAL_RULE_POSITIONS_PER_TABLE`.
    let positions =
        u32::from(n_back_built) + u32::from(n_input_built) + u32::from(n_lookaround_built);
    if !t.budget.try_spend_rule_positions(positions) {
        return None;
    }
    let input_begins = n_back_built;
    let input_ends = n_back_built
        .wrapping_add(implied_inputs)
        .wrapping_add(n_input_built);
    let match_count = input_ends.wrapping_add(n_lookaround_built);
    let mut rule: Box<ChainingRule> = Box::new(ChainingRule {
        match_count: match_count as TableId,
        input_begins,
        input_ends: input_ends as TableId,
        sequence: Vec::new(),
        apply: Vec::new(),
    });
    // Filled in order: backtrack, the `first_input` slot, input, lookahead.
    rule.sequence = Vec::with_capacity(match_count as usize);
    for j in 0..n_back_built {
        let gid = u16_at(slice, offset as usize + 2 + 2 * j as usize);
        rule.sequence
            .push(coverage_of(t, gid, offset, ContextKind::Backtrack));
    }
    if let Some(first) = first_input {
        rule.sequence.push(first);
    }
    // Array positions from the counts as read, by addition, so a malformed
    // `n_input < implied_inputs` cannot underflow.
    let input_base = offset as usize + 4 + 2 * n_back as usize;
    for j0 in 0..n_input_built {
        let gid = u16_at(slice, input_base + 2 * j0 as usize);
        rule.sequence
            .push(coverage_of(t, gid, offset, ContextKind::Input));
    }
    let lookaround_base = input_base + 2 * n_input_read as usize + 2;
    for j1 in 0..n_lookaround_built {
        let gid = u16_at(slice, lookaround_base + 2 * j1 as usize);
        rule.sequence
            .push(coverage_of(t, gid, offset, ContextKind::Lookahead));
    }

    if n_apply as usize > MAX_APPLY_PER_RULE {
        t.budget.reached(Limit::RuleApplications);
    }
    rule.apply = Vec::with_capacity((n_apply as usize).min(MAX_APPLY_PER_RULE));
    let apply_base = lookaround_base + 2 * n_lookaround as usize + 2;
    for j2 in 0..n_apply.min(MAX_APPLY_PER_RULE as u16) {
        let lookup_pos = apply_base + 4 * j2 as usize;
        let seq_index = u16_at(slice, lookup_pos);
        let lookup_index = u16_at(slice, lookup_pos + 2);
        let index = rule.input_begins.wrapping_add(seq_index);
        let lookup = handle_from_index(lookup_index) as LookupHandle;
        rule.apply.push(ChainLookupApplication { index, lookup });
    }
    reverse_backtracks(&mut rule);
    Some(rule)
}
/// Which of the two lookup families a subtable belongs to. They share the
/// subtable layouts below and differ only in the rule format and in the
/// class definitions a format 2 subtable carries.
#[derive(Copy, Clone, Debug)]
enum RuleShape {
    /// Contextual (GSUB 5, GPOS 7): input sequence only.
    Contextual,
    /// Chaining contextual (GSUB 6, GPOS 8): backtrack, input and lookahead.
    Chaining,
}
impl RuleShape {
    fn read_rule(
        self,
        t: &mut TableReader<'_>,
        offset: u32,
        first_input: Option<Coverage>,
        coverage_of: impl FnMut(&mut TableReader<'_>, u16, u32, ContextKind) -> Coverage,
    ) -> Option<Box<ChainingRule>> {
        match self {
            RuleShape::Contextual => {
                general_read_contextual_rule(t, offset, first_input, coverage_of)
            }
            RuleShape::Chaining => general_read_chaining_rule(t, offset, first_input, coverage_of),
        }
    }
}
/// Checks the rule count and rule-offset array of the rule set at
/// `set_offset` and returns the count, or `None` if they do not fit.
fn checked_rule_count(slice: &[u8], set_offset: usize) -> Option<u16> {
    let mut header = FontReader::new(slice).at(set_offset).ok()?;
    let count = header.u16().ok()?;
    header.require_room(count as usize, 2).ok()?;
    Some(count)
}
/// Format 1: one rule set per covered glyph, rules written in glyph ids.
fn read_format1(
    shape: RuleShape,
    t: &mut TableReader<'_>,
    offset: u32,
    subtable: &mut ChainingSubtable,
) -> Option<()> {
    let slice = t.data;
    let mut header = FontReader::new(slice).at(offset as usize + 2).ok()?;
    let cov_rel = header.u16().ok()?;
    let rule_set_count = header.u16().ok()?;
    let cov_offset = offset.wrapping_add(u32::from(cov_rel));
    let first_coverage: Coverage = read_coverage(slice, cov_offset, t.budget);
    if rule_set_count as usize != first_coverage.len() {
        return None;
    }
    header.require_room(rule_set_count as usize, 2).ok()?;

    // First pass: validate every rule set's own header and rule-offset array.
    // A NULL rule set offset is a covered glyph with no rules, as in format 2.
    let mut total_rules: usize = 0;
    for j in 0..rule_set_count {
        let srs_rel = u16_at(slice, offset as usize + 6 + 2 * j as usize);
        if srs_rel == 0 {
            continue;
        }
        let srs_offset = offset.wrapping_add(u32::from(srs_rel));
        let srs_count = checked_rule_count(slice, srs_offset as usize)?;
        total_rules = total_rules.saturating_add(srs_count as usize);
    }

    // Second pass: build, re-deriving each offset as the first pass did.
    let ruleset = chaining_ruleset_mut(subtable);
    // Grown as rules are built rather than reserved from `total_rules`:
    // the counts come from the file, and a rule set that builds nothing
    // (skipped, or out of budget) would still hold the reservation, once
    // per subtable.
    ruleset.rules = Vec::new();
    'rulesets: for j in 0..rule_set_count {
        let srs_rel = u16_at(slice, offset as usize + 6 + 2 * j as usize);
        if srs_rel == 0 {
            continue;
        }
        let srs_offset = offset.wrapping_add(u32::from(srs_rel));
        let srs_count = u16_at(slice, srs_offset as usize);
        for k in 0..srs_count {
            if !t.budget.take_rule() {
                break 'rulesets;
            }
            let sr_rel = u16_at(slice, srs_offset as usize + 2 + 2 * k as usize);
            let sr_offset = srs_offset.wrapping_add(u32::from(sr_rel));
            let first_glyph = first_coverage[j as usize].index as u16;
            let rule = shape.read_rule(
                t,
                sr_offset,
                Some(single_coverage(first_glyph)),
                |_, gid, _, _| single_coverage(gid),
            );
            push_rule(&mut ruleset.rules, rule);
        }
    }
    Some(())
}
/// Format 2: one rule set per input class, rules written in class numbers.
fn read_format2(
    shape: RuleShape,
    t: &mut TableReader<'_>,
    offset: u32,
    subtable: &mut ChainingSubtable,
) -> Option<()> {
    let slice = t.data;
    let class_def_at = |rel: u16| {
        Some(Box::new(read_class_def(
            slice,
            offset.wrapping_add(u32::from(rel)),
        )))
    };
    let mut header = FontReader::new(slice).at(offset as usize + 2).ok()?;
    let cov_rel = header.u16().ok()?;
    // The class definition offsets, then the class set count and offsets.
    let (cds, class_set_count, sets_base) = match shape {
        RuleShape::Contextual => {
            let ic_rel = header.u16().ok()?;
            let class_set_count = header.u16().ok()?;
            header.require_room(class_set_count as usize, 2).ok()?;
            let cds = ClassDefs {
                bc: None,
                ic: class_def_at(ic_rel),
                fc: None,
            };
            (cds, class_set_count, offset as usize + 8)
        }
        RuleShape::Chaining => {
            let bc_rel = header.u16().ok()?;
            let ic_rel = header.u16().ok()?;
            let fc_rel = header.u16().ok()?;
            let class_set_count = header.u16().ok()?;
            header.require_room(class_set_count as usize, 2).ok()?;
            let cds = ClassDefs {
                bc: class_def_at(bc_rel),
                ic: class_def_at(ic_rel),
                fc: class_def_at(fc_rel),
            };
            (cds, class_set_count, offset as usize + 12)
        }
    };

    // First pass: check every non-empty class set's header and rule offsets.
    let mut total_rules: usize = 0;
    for j in 0..class_set_count {
        let src_rel = u16_at(slice, sets_base + 2 * j as usize);
        if src_rel == 0 {
            continue;
        }
        let srs_count = checked_rule_count(slice, offset as usize + src_rel as usize)?;
        total_rules = total_rules.saturating_add(srs_count as usize);
    }

    // The first input glyph must be in the subtable's coverage, so the first
    // position of a class set's rules is the covered glyphs of that class,
    // not every glyph of it. A class set none of whose glyphs is covered
    // can never match and is skipped.
    let first_coverage = read_coverage(slice, offset.wrapping_add(u32::from(cov_rel)), t.budget);
    let first_by_class = covered_glyphs_by_class(
        &first_coverage,
        t.max_glyphs,
        cds.ic
            .as_deref()
            .expect("format 2 always reads an input ClassDef"),
        class_set_count,
    );

    let ruleset = chaining_ruleset_mut(subtable);
    // Grown as rules are built rather than reserved from `total_rules`:
    // the counts come from the file, and a rule set that builds nothing
    // (skipped, or out of budget) would still hold the reservation, once
    // per subtable.
    ruleset.rules = Vec::new();
    'class_sets: for j in 0..class_set_count {
        let src_rel = u16_at(slice, sets_base + 2 * j as usize);
        if src_rel == 0 || first_by_class[j as usize].is_empty() {
            continue;
        }
        let srs_count = u16_at(slice, offset as usize + src_rel as usize);
        // A rule offset repeated within a class set is the same rule again,
        // which can never match where the first did not: read once.
        let mut seen_rules = std::collections::HashSet::new();
        for k in 0..srs_count {
            let sr_rel = u16_at(
                slice,
                offset as usize + src_rel as usize + 2 + 2 * k as usize,
            );
            if !seen_rules.insert(sr_rel) {
                continue;
            }
            if !t.budget.take_rule() {
                break 'class_sets;
            }
            let sr_offset = offset
                .wrapping_add(u32::from(src_rel))
                .wrapping_add(u32::from(sr_rel));
            // Each rule holds its own copy of the class's covered glyphs, so
            // a class set of many rules aliasing one rule copies them as many
            // times: charged like the glyphs `class_coverage` builds for the
            // other positions.
            if !t
                .budget
                .try_spend_class_glyphs(first_by_class[j as usize].len())
            {
                break 'class_sets;
            }
            let first = first_by_class[j as usize].clone();
            let rule = shape.read_rule(t, sr_offset, Some(first), |t, cls, _, kind| {
                class_coverage(t, cls, kind, &cds)
            });
            push_rule(&mut ruleset.rules, rule);
        }
    }
    Some(())
}
/// The glyphs of `coverage` grouped by their class in `ic`, one coverage per
/// class below `class_count`. A glyph `ic` does not list is class 0; a glyph
/// whose class has no class set is dropped. A glyph id the font does not
/// have (`max_glyphs` or above) is left out too: it can never be matched,
/// and copied into every rule of its class it would only be warned about
/// once per copy.
fn covered_glyphs_by_class(
    coverage: &Coverage,
    max_glyphs: GlyphId,
    ic: &ClassDef,
    class_count: u16,
) -> Vec<Coverage> {
    let class_of: std::collections::HashMap<GlyphId, GlyphClass> = ic
        .glyphs
        .iter()
        .zip(ic.classes.iter())
        .map(|(g, &c)| (g.index, c))
        .collect();
    let mut by_class: Vec<Coverage> = (0..class_count).map(|_| Coverage::new()).collect();
    for g in coverage.iter().filter(|g| g.index < max_glyphs) {
        let cls = class_of.get(&g.index).copied().unwrap_or(0);
        if let Some(cov) = by_class.get_mut(cls as usize) {
            push_to_coverage(cov, g.clone());
        }
    }
    by_class
}
/// Adds a rule read by `RuleShape::read_rule`, or nothing if it was `None`.
/// `None` means this one rule's own header was malformed; the rule set
/// that points at it was already checked and fits the table, so the rest of
/// the subtable is kept. `unconsolidate_chaining` expects every stored slot
/// to be `Some`.
fn push_rule(rules: &mut Vec<Option<Box<ChainingRule>>>, rule: Option<Box<ChainingRule>>) {
    if rule.is_some() {
        rules.push(rule);
    }
}
fn read_subtable(
    shape: RuleShape,
    data: &[u8],
    offset: u32,
    max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    let mut t = TableReader {
        data,
        max_glyphs,
        budget,
    };
    let mut subtable = ChainingSubtable::Poly(ChainingRuleSet::default());
    let format = FontReader::new(data)
        .at(offset as usize)
        .and_then(|mut r| r.u16())
        .unwrap_or(0);
    match format {
        1 => read_format1(shape, &mut t, offset, &mut subtable)?,
        2 => read_format2(shape, &mut t, offset, &mut subtable)?,
        3 => {
            let rule = shape.read_rule(
                &mut t,
                offset.wrapping_add(2),
                None,
                |t, shift, rule_offset, _| format3_coverage(t, shift, rule_offset),
            );
            push_rule(&mut chaining_ruleset_mut(&mut subtable).rules, rule);
        }
        _ => {
            tracing::warn!("Unsupported format {}.\n", i32::from(format));
            return None;
        }
    }
    Some(Subtable::Chaining(subtable))
}
/// Reads a contextual subtable (GSUB 5, GPOS 7) of any of the three formats
/// into a rule set. Returns `None` for an unknown format or a malformed
/// format 1 or 2 header; a single malformed rule is dropped instead.
pub fn otl_read_contextual(
    data: &[u8],
    offset: u32,
    max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    read_subtable(RuleShape::Contextual, data, offset, max_glyphs, budget)
}
/// Reads a chaining contextual subtable (GSUB 6, GPOS 8); otherwise the
/// same as `otl_read_contextual`.
pub fn otl_read_chaining(
    data: &[u8],
    offset: u32,
    max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    read_subtable(RuleShape::Chaining, data, offset, max_glyphs, budget)
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
        let mut budget = OtlReadBudget {
            class_zero_glyphs: 7,
            ..OtlReadBudget::new()
        };

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

        let mut t = TableReader {
            data: &[],
            max_glyphs: 10,
            budget: &mut budget,
        };
        let cov = class_coverage(&mut t, 0, ContextKind::Backtrack, &defs);

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
        // ChainSubRule at 12 (with a first input): nInput=1 (the coverage's own
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

    /// A format 1 context subtable with one rule set of `rules` rules, all
    /// pointing at one rule of `inputs` input glyphs (the first one is the
    /// coverage's glyph).
    fn context_format1_with_rules(rules: u16, inputs: u16) -> Vec<u8> {
        let rule_set = 8usize;
        let rule = rule_set + 2 + 2 * rules as usize;
        let coverage = rule + 4 + 2 * (inputs as usize - 1);
        let mut data = vec![0u8; coverage + 6];
        let put = |data: &mut Vec<u8>, at: usize, v: u16| {
            data[at..at + 2].copy_from_slice(&v.to_be_bytes())
        };
        put(&mut data, 0, 1); // format
        put(&mut data, 2, coverage as u16);
        put(&mut data, 4, 1); // ruleSetCount
        put(&mut data, 6, rule_set as u16);
        put(&mut data, rule_set, rules);
        for k in 0..rules as usize {
            put(&mut data, rule_set + 2 + 2 * k, (rule - rule_set) as u16);
        }
        put(&mut data, rule, inputs);
        put(&mut data, rule + 2, 0); // nApply
        for j in 1..inputs as usize {
            put(&mut data, rule + 2 + 2 * j, 6);
        }
        put(&mut data, coverage, 1); // coverage format 1
        put(&mut data, coverage + 2, 1);
        put(&mut data, coverage + 4, 5);
        data
    }

    // A rule set offset may be NULL: that covered glyph has no rules. It
    // was followed to the subtable's start, which read a nonsense rule set
    // or failed the whole subtable.
    #[test]
    fn a_null_format_1_rule_set_is_a_glyph_without_rules() {
        let mut data = context_format1_with_rules(1, 2);
        // A second covered glyph (6), whose rule set is NULL.
        let coverage = usize::from(u16::from_be_bytes([data[2], data[3]]));
        data[4..6].copy_from_slice(&2u16.to_be_bytes()); // ruleSetCount
        // Room for the second ruleSetOffset: shift everything after byte 8.
        data.splice(8..8, [0, 0]);
        let shift = |v: u16| (v + 2).to_be_bytes();
        let cov = shift(coverage as u16);
        data[2..4].copy_from_slice(&cov);
        let rs = u16::from_be_bytes([data[6], data[7]]);
        data[6..8].copy_from_slice(&shift(rs));
        data[coverage + 2 + 2..coverage + 2 + 4].copy_from_slice(&2u16.to_be_bytes());
        data.extend_from_slice(&6u16.to_be_bytes());
        let mut budget = OtlReadBudget::new();
        let before = budget.rules;
        let sub = otl_read_contextual(&data, 0, 100, &mut budget).unwrap();
        assert_eq!(rule_count(&sub), 1);
        // Only the real rule was read: the NULL rule set was not followed
        // to a rule made of the subtable's own header.
        assert_eq!(before - budget.rules, 1);
    }

    fn rule_count(sub: &Subtable) -> usize {
        let Subtable::Chaining(ChainingSubtable::Poly(ruleset)) = sub else {
            unreachable!()
        };
        ruleset.rules.len()
    }

    #[test]
    fn more_rules_than_the_old_limit_of_15000_are_read() {
        // Gulzar's GSUB has 27,937 rules.
        let data = context_format1_with_rules(30_000, 3);
        let mut budget = OtlReadBudget::new();
        let sub = otl_read_contextual(&data, 0, 100, &mut budget).unwrap();
        assert_eq!(rule_count(&sub), 30_000);
        assert!(!budget.has_reached(Limit::Rules));
        assert!(!budget.has_reached(Limit::AllRulePositions));
    }

    #[test]
    fn rules_stop_when_the_positions_budget_is_spent() {
        let data = context_format1_with_rules(10, 4);
        let mut budget = OtlReadBudget {
            rule_positions: 10,
            ..OtlReadBudget::new()
        };
        // Each rule builds three positions after the coverage's glyph.
        let sub = otl_read_contextual(&data, 0, 100, &mut budget).unwrap();
        assert_eq!(rule_count(&sub), 3);
        assert_eq!(budget.rule_positions, 1);
        assert!(budget.has_reached(Limit::AllRulePositions));
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

    /// A contextual format 2 subtable with one single-glyph rule in each of
    /// class sets 0 and 1. Coverage: glyphs 3 and 7. ClassDef: glyphs 5, 6
    /// and 7 are class 1, every other glyph class 0.
    fn context_format2_with_coverage() -> Vec<u8> {
        #[rustfmt::skip]
        let words: [u16; 24] = [
            2,                  // format
            28,                 // coverageOffset
            36,                 // classDefOffset
            2,                  // classSetCount
            12,                 // classSetOffset[0]
            20,                 // classSetOffset[1]
            1, 4,               // class set 0 at 12: one rule, at 16
            1, 0,               // rule at 16: glyphCount 1, no lookups
            1, 4,               // class set 1 at 20: one rule, at 24
            1, 0,               // rule at 24
            1, 2, 3, 7,         // coverage at 28: format 1, glyphs 3 and 7
            1, 5, 3, 1, 1, 1,   // class def at 36: format 1, glyphs 5..=7 class 1
        ];
        words.iter().flat_map(|w| w.to_be_bytes()).collect()
    }

    fn first_positions(data: &[u8]) -> Vec<Vec<GlyphId>> {
        let sub = otl_read_contextual(data, 0, 100, &mut OtlReadBudget::new()).unwrap();
        let Subtable::Chaining(ChainingSubtable::Poly(ruleset)) = sub else {
            unreachable!()
        };
        ruleset
            .rules
            .iter()
            .map(|r| glyphs_of(&r.as_ref().unwrap().sequence[0]))
            .collect()
    }

    #[test]
    fn context_format2_first_input_is_limited_to_the_coverage() {
        // Class 0 is not every unclassified glyph, only covered glyph 3;
        // class 1 is not glyphs 5, 6 and 7, only covered glyph 7.
        assert_eq!(
            first_positions(&context_format2_with_coverage()),
            vec![vec![3], vec![7]]
        );
    }

    #[test]
    fn context_format2_class_set_with_no_covered_glyph_is_skipped() {
        // Glyph 7 removed from the coverage: no covered glyph is class 1, so
        // class set 1's rule can never match and is not read.
        let mut data = context_format2_with_coverage();
        data[34..36].copy_from_slice(&3u16.to_be_bytes());
        assert_eq!(first_positions(&data), vec![vec![3]]);
    }

    /// A format 2 context subtable: a one-range coverage of glyphs
    /// `0..covered` (all class 0, the ClassDef has no ranges) and one class
    /// set of `rules` rules, all at the offset of one 1-glyph rule.
    fn context_format2_aliased_rules(covered: u16, rules: u16) -> Vec<u8> {
        let mut data = Vec::new();
        let rule_at = 10 + 2 + 2 * rules;
        let cov_at = rule_at + 4;
        let cd_at = cov_at + 10;
        for v in [2, cov_at, cd_at, 1, 10, rules] {
            data.extend_from_slice(&v.to_be_bytes());
        }
        for _ in 0..rules {
            data.extend_from_slice(&(rule_at - 10).to_be_bytes());
        }
        for v in [1u16, 0, 2, 1, 0, covered - 1, 0, 2, 0] {
            data.extend_from_slice(&v.to_be_bytes());
        }
        data
    }

    // A class set's rules can all point at one rule. Each copied the
    // class's covered glyphs into its first position, uncharged, so a few
    // kilobytes made gigabytes. The repeated rule is read once now, and the
    // copies are charged.
    #[test]
    fn rules_aliasing_one_rule_are_read_once() {
        let data = context_format2_aliased_rules(1000, 3000);
        let mut budget = OtlReadBudget::new();
        let sub = otl_read_contextual(&data, 0, 1000, &mut budget).unwrap();
        let Subtable::Chaining(ChainingSubtable::Poly(ruleset)) = &sub else {
            unreachable!()
        };
        assert_eq!(ruleset.rules.len(), 1);
        assert_eq!(ruleset.rules[0].as_ref().unwrap().sequence[0].len(), 1000);
    }

    #[test]
    fn copied_first_glyphs_are_charged_to_the_class_budget() {
        let data = context_format2_aliased_rules(1000, 1);
        let mut budget = OtlReadBudget {
            class_zero_glyphs: 999,
            ..OtlReadBudget::new()
        };
        let sub = otl_read_contextual(&data, 0, 1000, &mut budget).unwrap();
        let Subtable::Chaining(ChainingSubtable::Poly(ruleset)) = &sub else {
            unreachable!()
        };
        assert!(ruleset.rules.is_empty());
        assert!(budget.has_reached(Limit::ClassZeroGlyphs));
    }

    // A coverage glyph the font does not have can never match, and is not
    // copied into the rules.
    #[test]
    fn covered_glyphs_past_the_glyph_count_are_left_out() {
        let data = context_format2_aliased_rules(1000, 1);
        let mut budget = OtlReadBudget::new();
        let sub = otl_read_contextual(&data, 0, 10, &mut budget).unwrap();
        let Subtable::Chaining(ChainingSubtable::Poly(ruleset)) = &sub else {
            unreachable!()
        };
        assert_eq!(ruleset.rules[0].as_ref().unwrap().sequence[0].len(), 10);
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
            rule.sequence.iter().map(glyphs_of).collect::<Vec<_>>(),
            vec![vec![1], vec![2], vec![3]]
        );
        assert_eq!(rule.input_begins, 1);
        assert_eq!(rule.input_ends, 2);
    }

    #[test]
    fn chaining_rule_with_input_count_below_the_first_input_slot_does_not_panic() {
        // A malformed `nInput` of 0 while this call site gives a
        // `first_input` (coverage-implied) slot: `nInput - 1` must give zero
        // array reads rather than panic on `u16` underflow.
        // Reached via `general_read_chaining_rule` directly since
        // `read_chaining_format1` (the real caller of this path) also
        // requires a fully valid, consistent outer coverage/ruleset
        // structure this test isn't trying to build.
        let mut data = [0u8; 10];
        data[0..2].copy_from_slice(&0u16.to_be_bytes()); // nBack
        data[2..4].copy_from_slice(&0u16.to_be_bytes()); // nInput (malformed: 0)
        data[4..6].copy_from_slice(&0u16.to_be_bytes()); // nLookaround
        data[6..8].copy_from_slice(&0u16.to_be_bytes()); // nApply
        let mut budget = OtlReadBudget::new();
        let mut t = TableReader {
            data: &data,
            max_glyphs: 100,
            budget: &mut budget,
        };
        let rule =
            general_read_chaining_rule(&mut t, 0, Some(single_coverage(7)), |_, gid, _, _| {
                single_coverage(gid)
            });
        let rule = rule.unwrap();
        // Only the `first_input` slot (glyph 7) is
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
