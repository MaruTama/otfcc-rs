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
// Was `CoverageReaderHandler`, a `*mut c_void` userdata pointer threaded
// alongside a fn-pointer typedef shared by all three concrete readers
// below (`single_coverage`/`class_coverage`/`format3_coverage`) -- the
// `*mut c_void` existed purely so `class_coverage`'s `ClassDefs` borrow
// could travel through a shape the other two readers (which never needed
// any userdata at all) were also forced to accept. `general_read_
// contextual_rule`/`general_read_chaining_rule` (the only two callers)
// now take the reader as a generic `impl FnMut(&[u8], u16, u32,
// ContextKind, GlyphId) -> Coverage` instead: `single_coverage`/
// `format3_coverage` (already plain safe `fn`s with this exact signature)
// are passed directly as function items, and `class_coverage`'s call sites capture
// `&ClassDefs` in a closure instead of casting a raw pointer to it --
// which in turn lets `class_coverage` itself take `&ClassDefs` and drop
// `unsafe fn` too (its only unsafe operations were the userdata cast and
// the raw-pointer derefs of `defs`/`cov` that cast enabled).
// Stage 7-2-c "inner Box化": `bc`/`ic`/`fc` become `Option<Box<ClassDef>>`,
// the same shape `table/otl.rs`'s `ChainingRuleSet.bc`/`.ic`/`.fc` (a
// *different* struct, despite the identical field shape -- that one is the
// long-lived, publicly stored classification result; this one is a
// transient scratch struct used only as `class_coverage`'s `void*`
// userdata for the duration of a single `read_contextual_format2`/
// `read_chaining_format2` call) already use for this same `ClassDef` type.
// `Copy`/`Clone` dropped: `Box` isn't `Copy`, and a grep of every `.bc`/
// `.ic`/`.fc`/`ClassDefs` touch site in this file (the only file that
// mentions this type) confirmed none of them ever copied the struct by
// value in the first place -- every access already went through a raw
// pointer (`cds: *mut ClassDefs` / `defs: *mut ClassDefs`), so dropping the
// derive changes no call site's shape, only what `bc`/`ic`/`fc` own.
#[derive(Debug)]
pub struct ClassDefs {
    pub bc: Option<Box<ClassDef>>,
    pub ic: Option<Box<ClassDef>>,
    pub fc: Option<Box<ClassDef>>,
}
/// The `class_zero_glyphs` limit of `OtlReadBudget`. See also
/// `MAX_TOTAL_CLASS_COVERAGE_CALLS` below for what this bounds and why: a single `class_coverage` call
/// can scan up to `max_glyphs` (the font's own declared glyph count, up
/// to 65535) or `cd.glyphs.len()` candidates, and it is called once per
/// input/backtrack/lookahead position in a rule (see
/// `general_read_contextual_rule`'s own loop) -- a subtable holding many
/// rules, each with several positions, against a font declaring many
/// glyphs, multiplies into gigabytes from a subtable of only a few
/// hundred KB (ASan-confirmed: a fuzz-found font OOM'd at ~1.8GB this
/// way). Sized around real usage, not just adversarial safety: this
/// budget is global across a whole table now (see the scope note after
/// `MAX_TOTAL_CLASS_COVERAGE_CALLS`), and `tests/payload/NotoNastaliqUrdu-Regular.ttf` -- a
/// real, legitimately complex Nastaliq-script font already in this
/// repo's golden corpus -- genuinely uses ~10.7 million of these units in
/// its own GSUB table alone (confirmed by instrumenting a debug build).
/// 20 million leaves that font ~1.87x of headroom while still only
/// costing well under a second even if a whole table's worth of
/// subtables all hit it (the original 10 million figure came from timing
/// a single call in isolation, before this budget's scope changed from
/// per-subtable to per-table; a later, separate fix -- `otl/read.rs`'s
/// `MAX_TOTAL_LOOKUPS_PER_TABLE`/`MAX_TOTAL_FEATURE_REFS_PER_TABLE` --
/// turned out to matter far more for peak memory than this budget's exact
/// size did, so this stays modestly above real usage rather than as
/// generous as an earlier, since-retightened 200-million figure).
pub(crate) const MAX_TOTAL_CLASS_ZERO_COVERAGE_GLYPHS: u32 = 20_000_000;
/// The `class_coverage_calls` limit of `OtlReadBudget`: bounds the number of
/// `class_coverage` *calls* themselves, independent of how much work (if
/// any) each one does internally -- what actually stops a fuzz-found
/// font whose rules reference an empty classdef, so the `class_zero_glyphs`
/// limit above never triggers at all, from taking 20-30s on sheer call volume
/// (well past a million calls/second's worth of fixed per-call overhead).
pub(crate) const MAX_TOTAL_CLASS_COVERAGE_CALLS: u32 = 70_000;
// Scope of the two limits above: they used to live as fields on
// `ClassDefs`, reset fresh for every subtable (one `ClassDefs` per
// `read_contextual_format2`/`read_chaining_format2` call). That bounded
// each *subtable's* cost, but not a *lookup's* or a *table's*:
// `otl/read.rs`'s `MAX_TOTAL_SUBTABLES_PER_LOOKUP` caps subtable count at
// 1,000 per lookup precisely because it was previously unbounded, and
// 1,000 subtables each getting their own fresh allowance multiplies right
// back into the same class of hang these limits exist to prevent (fuzzing
// confirmed it: capping rules-per-subtable and subtables-per-lookup
// individually still left a lookup with ~700 subtables taking 20+ seconds
// in `class_coverage` alone). They are now `OtlReadBudget`'s
// `class_zero_glyphs`/`class_coverage_calls`, created once per
// `read_otl` call (once per GSUB or GPOS table), which bounds the
// whole table's total `class_coverage` cost.
/// Bounds the number of contextual/chaining rules actually built across a
/// *whole table* (every subtable of every lookup combined -- see
/// `OtlReadBudget::rules`, one budget per `read_otl` call).
/// Each `chainSubClassSet`/`subRuleSet` entry's own rule count is
/// individually bounds-checked against the table (its rule-offset array
/// must fit), but nothing stopped an attacker from declaring dozens of such
/// entries that each carry a legitimately-shaped but enormous count:
/// fuzzing found a single format2 subtable with `chainSubClassSetCount =
/// 44`, whose per-entry rule counts summed past 500,000 rules, each paying
/// for its own heap allocation plus a `class_coverage`/`single_coverage`
/// call. An earlier version of this fix capped the count *per subtable*
/// instead of per table, which bounded one subtable's cost but not a
/// lookup's or a table's: `otl/read.rs`'s `MAX_TOTAL_SUBTABLES_PER_LOOKUP`
/// caps subtable count at 1,000 per lookup precisely because it was
/// previously unbounded too, and up to 1,000 subtables each getting their
/// own fresh per-subtable rule allowance multiplies right back into the
/// same hang (fuzzing confirmed a lookup with ~700 subtables still took
/// 20+ seconds after the per-subtable cap alone). Real fonts have at most
/// a few hundred contextual rules per subtable and nowhere near this many
/// subtables per table, so this cap is far above any legitimate usage
/// while keeping worst-case adversarial cost to well under a second.
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
/// itself), or lookahead (glyphs after it) -- the OpenType spec's own
/// 1/2/3 numbering for the three `ClassDef` slots a format-2 rule can
/// draw from. Replaces the `1_u16`/`2_u16`/`3_u16` magic numbers
/// `class_coverage` used to compare its own `kind` parameter against.
/// `single_coverage`/`format3_coverage` ignore this value entirely
/// (their own coverage doesn't depend on which side of the rule it's
/// for), but still take a real `ContextKind` rather than `Option<_>` or a
/// leftover `u16`, so every implementer of the shared `fn_0` callback
/// shape agrees on one type for this parameter.
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
    // `class_coverage`'s `fn_0` slot) only ever asks for a `kind` whose
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
    // `general_read_contextual_rule`/`general_read_chaining_rule` call
    // this once per input/backtrack/lookahead position in a rule, and a
    // subtable can hold a huge number of tiny rules -- so *every* loop
    // below (not just the ones that end up pushing a glyph) is charged
    // against `(*defs).class_zero_budget`, one unit per iteration,
    // shared across every call this `ClassDefs` sees while reading the
    // whole subtable. That bounds TOTAL scanning work across the whole
    // subtable to a fixed budget regardless of `cls`, `max_glyphs`, this
    // classdef's own size, or how many times this gets called -- a
    // per-push-only budget (an earlier version of this fix) still let a
    // "dense" classdef (few glyphs actually pushed, but every one of
    // `max_glyphs` still has to be checked) or the plain `cls != 0`
    // linear scan (already O(cd.glyphs.len()), no quadratic factor to
    // fix, but still uncapped per call) hang on the same fuzz-found
    // font this was found on, by racking up iterations that never
    // decremented anything.
    //
    // `cls == 0` ("every glyph not otherwise classified") used to also
    // be a *quadratic* linear scan over `(*cd).glyphs` for every one of
    // up to `max_glyphs` candidates -- O(max_glyphs * cd.glyphs.len())
    // just to find which glyphs are classified, on top of the memory
    // amplification this same budget also guards against (ASan-
    // confirmed OOM: ~1.8GB from a fuzz-found font). A bitmap over
    // `0..max_glyphs` (at most 65535 bits, built once in
    // O(cd.glyphs.len())) turns the lookup into O(1), making that part
    // O(max_glyphs + cd.glyphs.len()) -- also drops the original's
    // separate, identical count-then-populate double scan: counting
    // ahead only ever fed a since-removed `Vec::with_capacity`-style
    // early return, so folding it into one pass changes nothing
    // observable for any input this budget doesn't itself cut off.
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
        // Left as a `while`, not converted: `j_2` is a `GlyphId` (`u16`),
        // but the bound it is compared against is `cd.glyphs.len()`
        // (`usize`), which -- unlike `max_glyphs` above -- is not itself
        // capped to fit in a `u16` by this function's own signature. A
        // `format 2` `ClassDef` can legitimately hold exactly 65536
        // distinct glyphs (`table/otl/classdef.rs::read_class_def`'s
        // format-2 branch dedups by `GlyphId` into an `IndexMap`, whose
        // key space is the full `u16` range), so `cd.glyphs.len() ==
        // 65536` is reachable, not merely theoretical. In that exact case
        // the original's `j_2 = j_2.wrapping_add(1)` wraps `0xffff` back
        // to `0` *before* `(j_2 as usize) < cd.glyphs.len()` ever goes
        // false, so the `while` keeps re-scanning the same 65536 entries
        // (each full pass re-charging and, on a matching `cls`, re-pushing
        // every match) until `budget.class_zero_left()` alone ends it -- a
        // materially different outcome (many repeated passes, and
        // correspondingly many duplicate pushes) than a single `for j_2
        // in 0..cd.glyphs.len()` pass would produce. Converting this one
        // would silently change what a maximal-`ClassDef` input does, so
        // per the task's own "leave it alone rather than guess" rule, it
        // stays a `while`.
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
// Every guard below is expressed as a `FontReader` read or `require_room`
// call in the exact sequence the original's hand-written `table_length <
// ...` checks ran in, so the set of inputs accepted/rejected is unchanged
// (`require_room`'s `checked_mul` cannot itself matter here: every count
// this file reads is a `u16`, so `count * stride` can never overflow
// `usize`). The one behavior change is fidelity, not scope: a `FontReader`
// read only ever demands exactly the bytes the value it is producing
// needs, where a few of the original's guards reserved a handful of extra
// bytes beyond what the following reads actually touched (see
// `read_contextual_format2`'s and `read_chaining_format2`'s "no slop"
// note below) -- always in the safe direction (rejecting strictly less
// than before), documented per-function where it applies.
pub fn general_read_contextual_rule(
    slice: &[u8],
    offset: u32,
    start_gid: u16,
    minus_one: bool,
    mut fn_0: impl FnMut(&[u8], u16, u32, ContextKind, GlyphId, &mut OtlReadBudget) -> Coverage,
    max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Box<ChainingRule>> {
    let minus_one_q: u16 = minus_one as u16;

    let mut header = FontReader::new(slice).at(offset as usize).ok()?;
    let n_input = header.u16().ok()?;
    let n_apply = header.u16().ok()?;
    // Matches the original's own guard exactly: it reserves `2*n_input`
    // bytes for the input-glyph array even on the `minus_one` path, where
    // only `n_input - minus_one_q` entries are actually read below (one
    // slot more than strictly needed -- preserved as-is, not tightened).
    let needed = (n_input as usize) * 2 + (n_apply as usize) * 4;
    header.require_room(needed, 1).ok()?;

    // `n_input - minus_one_q` in the original ran in signed `c_int`
    // arithmetic, so a malformed `n_input < minus_one_q` (possible: the
    // `minus_one` slot above is unconditional, independent of `n_input`'s
    // own value) gave a negative loop bound and simply ran zero
    // iterations. `saturating_sub` reproduces that same "zero iterations"
    // outcome without the panic a plain `u16` subtraction would give here.
    let n_input_read = n_input.saturating_sub(minus_one_q);
    // See `MAX_POSITIONS_PER_RULE`'s own doc comment: only the *build*
    // loop below is capped, not `n_input_read` itself (still used
    // uncapped for `lookup_base` below, matching the original's byte
    // layout).
    let n_input_built = n_input_read.min(MAX_POSITIONS_PER_RULE);
    let match_count = minus_one_q.wrapping_add(n_input_built);

    // `Box` is the allocation, the struct literal is the zero-init the old
    // `__caryll_allocate_clean` provided -- same shape as `new_lookup`/
    // `new_glyf_glyph`.
    let mut rule: Box<ChainingRule> = Box::new(ChainingRule {
        match_count: match_count as TableId,
        input_begins: 0 as TableId,
        input_ends: match_count as TableId,
        match_0: Vec::new(),
        apply: Vec::new(),
    });
    // Filled in order below (the `minus_one` slot first, then the rest
    // sequentially) -- every one of the `match_count` slots is written
    // exactly once, in increasing index order, so `.push()` is the direct
    // replacement for the old `jj`-indexed writes into
    // `__caryll_allocate_clean`'d memory (`jj` itself is gone: it was only
    // ever used as that index).
    rule.match_0 = Vec::with_capacity(rule.match_count as usize);
    if minus_one {
        rule.match_0
            .push(fn_0(
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
        rule.match_0
            .push(fn_0(
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

        // Second pass: build, re-deriving each offset exactly as the first
        // pass did (nothing here is retained across passes, matching the
        // original's own two-pass structure). `ruleset` is a real `&mut
        // ChainingRuleSet` borrowed from the owned `subtable` (Stage L-5) --
        // held across the whole loop below the same way the old raw pointer
        // was, but now the borrow checker (not just convention) guarantees
        // `subtable` can't be freed out from under it.
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
        // The original reserved 4 extra bytes here (`offset+12+2*count`)
        // beyond what the `classSetOffset` array at `offset+8` actually
        // needs (`offset+8+2*count`) -- always-safe over-conservative slop,
        // now exactly the array's real requirement.
        if header
            .require_room(chain_sub_class_set_cnt as usize, 2)
            .is_err()
        {
            break 'parse None;
        }

        cds = Some(ClassDefs {
            bc: None,
            // `classdef_from_raw`/`read_class_def` are the still-raw-pointer-
            // shaped c2rust residue `classdef.rs` itself hasn't converted yet
            // (out of this stage's scope) -- same one-line `unsafe` wrapping
            // `table/gdef.rs`'s callers already use for this exact pattern.
            ic: Some(Box::new(read_class_def(slice, offset.wrapping_add(ic_rel as u32)))),
            fc: None,
        });

        // First pass: validate every non-empty ClassSet's own header +
        // rule-offset array. The original had NO guard at all here -- every
        // read below (`srs_count` itself, and its rule-offset array) ran
        // straight off `offset + src_offset` with no bounds check, a real
        // out-of-bounds read on a malformed `ChainSubClassSet` offset. Now
        // guarded like every sibling array in this file.
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

    // `cds` (now a plain `Option<ClassDefs>` local) auto-drops when this
    // function returns -- `class_coverage` only ever borrowed it via
    // `&ClassDefs` during the loop above, never took ownership, so no
    // manual cleanup is needed the way the old `Box<ClassDefs>` round trip
    // required.
    if result.is_some() { Some(subtable) } else { None }
}
pub fn otl_read_contextual(
    data: &[u8],
    offset: u32,
    max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    // Built directly as an owned `Box`, a valid empty `Poly` ruleset from
    // the start -- Stage L-5 dropped the two-step `subtable_chaining_
    // create()` (a fresh `Canonical`)-then-overwrite dance this used to do,
    // since there is no longer a raw-pointer "shell" that has to exist
    // before its contents are known.
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
    mut fn_0: impl FnMut(&[u8], u16, u32, ContextKind, GlyphId, &mut OtlReadBudget) -> Coverage,
    max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Box<ChainingRule>> {
    let minus_one_q: u16 = minus_one as u16;

    // Four counts read back-to-back, each immediately followed by a skip
    // over the array it introduces -- `n_back`/`backtrackArray`,
    // `n_input`/`inputArray` (minus the `minus_one` slot), `n_lookaround`/
    // `lookaheadArray`, then `n_apply` itself. This sequence of
    // read-then-`require_room`-then-skip steps enforces exactly the same
    // cumulative byte requirement the original's four incremental
    // `table_length < ...` guards did (each of those checked the running
    // total so far plus room for the next 2-byte count field; here each
    // step's own `u16()`/`require_room` call demands precisely that).
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

    // See `MAX_POSITIONS_PER_RULE`'s own doc comment: only the *build*
    // loops below are capped, not `n_back`/`n_input_read`/`n_lookaround`
    // themselves (still used uncapped for `input_base`/`lookaround_base`/
    // `apply_base` below, matching the original's byte layout).
    // `match_count`/`input_begins`/`input_ends` are computed purely from
    // these *built* (capped) counts, so they always agree with
    // `match_0`'s actual length by construction -- including the
    // `n_input < minus_one_q` edge case the pre-existing comment below
    // already had to reason about, which this sidesteps rather than
    // duplicates.
    let n_back_built = n_back.min(MAX_POSITIONS_PER_RULE);
    let n_input_built = n_input_read.min(MAX_POSITIONS_PER_RULE);
    let n_lookaround_built = n_lookaround.min(MAX_POSITIONS_PER_RULE);
    let input_begins = n_back_built;
    let input_ends = n_back_built.wrapping_add(minus_one_q).wrapping_add(n_input_built);
    let match_count = input_ends.wrapping_add(n_lookaround_built);
    // `Box` is the allocation, the struct literal is the zero-init the old
    // `__caryll_allocate_clean` provided -- see `general_read_contextual_rule`.
    let mut rule: Box<ChainingRule> = Box::new(ChainingRule {
        match_count: match_count as TableId,
        input_begins,
        input_ends: input_ends as TableId,
        match_0: Vec::new(),
        apply: Vec::new(),
    });
    // Filled in order below (backtrack, then the `minus_one` slot, then
    // input, then lookaround) -- every one of the `match_count` slots is
    // written exactly once, in increasing index order, so `.push()` is the
    // direct replacement for the old `jj`-indexed writes (`jj` itself is
    // gone: it was only ever used as that index).
    rule.match_0 = Vec::with_capacity(match_count as usize);
    for j in 0..n_back_built {
        let gid = FontReader::new(slice)
            .at(offset as usize + 2 + 2 * j as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.match_0
            .push(fn_0(
                slice,
                gid,
                offset,
                ContextKind::Backtrack,
                max_glyphs,
                budget,
            ));
    }
    if minus_one {
        rule.match_0
            .push(fn_0(
                slice,
                start_gid,
                offset,
                ContextKind::Input,
                max_glyphs,
                budget,
            ));
    }
    // Array positions derived the same way `header`'s cursor validated
    // them above (cumulative `usize` addition on the *reduced* counts),
    // rather than by re-subtracting `minus_one_q` from `input_ends`/
    // `match_count` the way the original's pointer arithmetic did --
    // avoids a `u16` underflow when a malformed `n_input < minus_one_q`
    // makes those two disagree (see `n_input_read` above), and always
    // agrees with them when they don't.
    let input_base = offset as usize + 4 + 2 * n_back as usize;
    for j0 in 0..n_input_built {
        let gid = FontReader::new(slice)
            .at(input_base + 2 * j0 as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.match_0
            .push(fn_0(
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
        rule.match_0
            .push(fn_0(
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

        // `classdef_from_raw`/`read_class_def` are the still-raw-pointer-
        // shaped c2rust residue `classdef.rs` itself hasn't converted yet
        // (out of this stage's scope) -- same one-line `unsafe` wrapping
        // `table/gdef.rs`'s callers already use for this exact pattern.
        cds = Some(ClassDefs {
            bc: Some(Box::new(read_class_def(slice, offset.wrapping_add(bc_rel as u32)))),
            ic: Some(Box::new(read_class_def(slice, offset.wrapping_add(ic_rel as u32)))),
            fc: Some(Box::new(read_class_def(slice, offset.wrapping_add(fc_rel as u32)))),
        });

        // First pass: validate every non-empty ClassSet's own header +
        // rule-offset array. The original had NO guard at all here (same
        // missing-guard shape as `read_contextual_format2`'s ClassSet
        // loop) -- every read below ran straight off `offset + src_offset`
        // with no bounds check. Now guarded like every sibling array.
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
// Was a manual meet-in-the-middle index-swapping loop over
// `*mut *mut Coverage` -- exactly `[T]::reverse` on the backtrack
// sub-slice, now that `match_0` is a real `Vec<Coverage>`. `input_begins
// == 0` (nothing to reverse) falls out of slicing an empty range.
fn reverse_backtracks(rule: &mut ChainingRule) {
    let input_begins = rule.input_begins as usize;
    rule.match_0[..input_begins].reverse();
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
        assert_eq!(rule.match_0.len(), 1);
        assert_eq!(glyphs_of(&rule.match_0[0]), vec![42]);
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
        assert_eq!(glyphs_of(&rule.match_0[0]), vec![5]);
    }

    #[test]
    fn context_format1_rule_set_count_mismatched_with_coverage_is_rejected() {
        // chainSubRuleSetCount (2) doesn't match the coverage's glyph
        // count (1) -- the original's own consistency check, preserved.
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
        // The original read `srs_count` (and its rule-offset array)
        // straight off `offset + classSetOffset[j]` with no guard at all --
        // a `classSetOffset` pointing past `table_length` read out of
        // bounds. `classSetOffset[0]` here (5000) is far past this
        // 10-byte table.
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
            rule.match_0
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
        // `minus_one` (coverage-implied) slot filled -- the original
        // computed `nInput - minus_one_q` in signed `c_int` arithmetic and
        // simply ran zero array-read iterations; a naive `u16` port of
        // that subtraction would panic on overflow instead. Reached via
        // `general_read_chaining_rule` directly since `read_chaining_format1`
        // (the real caller of this path) also requires a fully valid,
        // consistent outer coverage/ruleset structure this test isn't
        // trying to build.
        let mut data = [0u8; 10];
        data[0..2].copy_from_slice(&0u16.to_be_bytes()); // nBack
        data[2..4].copy_from_slice(&0u16.to_be_bytes()); // nInput (malformed: 0)
        data[4..6].copy_from_slice(&0u16.to_be_bytes()); // nLookaround
        data[6..8].copy_from_slice(&0u16.to_be_bytes()); // nApply
        let rule = general_read_chaining_rule(&data, 0, 7, true, single_coverage, 100, &mut OtlReadBudget::new());
        let rule = rule.unwrap();
        // Only the `minus_one` slot (glyph 7, from `start_gid`) is
        // filled; the (empty) input array contributes nothing.
        assert_eq!(rule.match_0.len(), 1);
        assert_eq!(glyphs_of(&rule.match_0[0]), vec![7]);
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
