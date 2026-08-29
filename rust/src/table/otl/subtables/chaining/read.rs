#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{
    GlyphHandle, Handle, LookupHandle, handle_from_index, otfcc_handle_dup,
};
use crate::table::otl::classdef::{ClassDef, classdef_from_raw, read_class_def};
use crate::table::otl::coverage::{
    Coverage, coverage_from_raw, otl_coverage_create, otl_coverage_free, push_to_coverage,
    read_coverage,
};

use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::font_reader::FontReader;

use crate::support::options::Options;
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};

use crate::support::NULL;
use crate::table::otl::subtables::chaining::common::{
    chaining_ruleset_mut, subtable_chaining_create,
};
use crate::table::otl::{
    ChainLookupApplication, ChainingRule, ChainingRuleSet, ChainingSubtable, Subtable,
    subtable_from_raw,
};
pub type CoverageReaderHandler = Option<
    unsafe fn(
        FontFilePointer,
        u32,
        u16,
        u32,
        u16,
        GlyphId,
        *mut ::core::ffi::c_void,
    ) -> *mut Coverage,
>;
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
pub struct ClassDefs {
    pub bc: Option<Box<ClassDef>>,
    pub ic: Option<Box<ClassDef>>,
    pub fc: Option<Box<ClassDef>>,
}
/// See the two budgets below (`CLASS_ZERO_BUDGET`/`CLASS_COVERAGE_CALL_
/// BUDGET`) for what this bounds and why: a single `class_coverage` call
/// can scan up to `max_glyphs` (the font's own declared glyph count, up
/// to 65535) or `cd.glyphs.len()` candidates, and it is called once per
/// input/backtrack/lookahead position in a rule (see
/// `general_read_contextual_rule`'s own loop) -- a subtable holding many
/// rules, each with several positions, against a font declaring many
/// glyphs, multiplies into gigabytes from a subtable of only a few
/// hundred KB (ASan-confirmed: a fuzz-found font OOM'd at ~1.8GB this
/// way). Sized around real usage, not just adversarial safety: this
/// budget is global across a whole table now (see `CLASS_ZERO_BUDGET`'s
/// own comment), and `tests/payload/NotoNastaliqUrdu-Regular.ttf` -- a
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
const MAX_TOTAL_CLASS_ZERO_COVERAGE_GLYPHS: u32 = 20_000_000;
/// See `CLASS_COVERAGE_CALL_BUDGET` below: bounds the number of
/// `class_coverage` *calls* themselves, independent of how much work (if
/// any) each one does internally -- what actually stops a fuzz-found
/// font whose rules reference an empty classdef, so `CLASS_ZERO_BUDGET`
/// above never triggers at all, from taking 20-30s on sheer call volume
/// (well past a million calls/second's worth of fixed per-call overhead).
const MAX_TOTAL_CLASS_COVERAGE_CALLS: u32 = 70_000;
/// These two budgets used to live as fields on `ClassDefs`, reset fresh
/// for every subtable (one `ClassDefs` per `read_contextual_format2`/
/// `read_chaining_format2` call). That bounded each *subtable's* cost,
/// but not a *lookup's* or a *table's*: `otl/read.rs`'s
/// `MAX_TOTAL_SUBTABLES_PER_LOOKUP` caps subtable count at 1,000 per
/// lookup precisely because it was previously unbounded, and 1,000
/// subtables each getting their own fresh 10-million/200,000 allowance
/// multiplies right back into the same class of hang this budget exists
/// to prevent (fuzzing confirmed it: capping rules-per-subtable and
/// subtables-per-lookup individually still left a lookup with ~700
/// subtables taking 20+ seconds in `class_coverage` alone). Global,
/// process-wide statics -- reset once per `otfcc_read_otl` call (see
/// `reset_class_coverage_budgets`), i.e. once per GSUB or GPOS table, not
/// once per subtable -- close that gap by bounding the whole table's
/// total `class_coverage` cost, not each subtable's independently. Safe
/// as plain statics (no `Mutex`/`RefCell` needed) because this crate is
/// single-threaded throughout, same reasoning as `Options::logger`'s own
/// `RefCell`.
static CLASS_ZERO_BUDGET: ::core::sync::atomic::AtomicU32 =
    ::core::sync::atomic::AtomicU32::new(MAX_TOTAL_CLASS_ZERO_COVERAGE_GLYPHS);
static CLASS_COVERAGE_CALL_BUDGET: ::core::sync::atomic::AtomicU32 =
    ::core::sync::atomic::AtomicU32::new(MAX_TOTAL_CLASS_COVERAGE_CALLS);
/// Must be called once per `otfcc_read_otl` call (once per GSUB/GPOS table
/// read), before that table's lookups are read -- see the doc comment on
/// the two statics above for why table-wide scope, not per-subtable, is
/// what actually bounds the cost.
pub(crate) fn reset_class_coverage_budgets() {
    CLASS_ZERO_BUDGET.store(
        MAX_TOTAL_CLASS_ZERO_COVERAGE_GLYPHS,
        ::core::sync::atomic::Ordering::Relaxed,
    );
    CLASS_COVERAGE_CALL_BUDGET.store(
        MAX_TOTAL_CLASS_COVERAGE_CALLS,
        ::core::sync::atomic::Ordering::Relaxed,
    );
    TOTAL_RULES_BUILT_BUDGET.store(
        MAX_TOTAL_RULES_PER_TABLE,
        ::core::sync::atomic::Ordering::Relaxed,
    );
}
/// Bounds the number of contextual/chaining rules actually built across a
/// *whole table* (every subtable of every lookup combined -- see
/// `reset_class_coverage_budgets`, called once per `otfcc_read_otl` call).
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
const MAX_TOTAL_RULES_PER_TABLE: u32 = 15_000;
static TOTAL_RULES_BUILT_BUDGET: ::core::sync::atomic::AtomicU32 =
    ::core::sync::atomic::AtomicU32::new(MAX_TOTAL_RULES_PER_TABLE);
/// Atomically consumes one unit of `TOTAL_RULES_BUILT_BUDGET`; `true` means
/// the caller may build (and push) one more rule, `false` means the
/// table-wide budget is exhausted and the caller should stop adding rules
/// to this subtable (and, transitively, stop processing further
/// chainSubClassSets/subRuleSets/subtables/lookups in this table, since
/// every further rule would hit the same exhausted budget).
fn take_rule_budget() -> bool {
    TOTAL_RULES_BUILT_BUDGET
        .try_update(
            ::core::sync::atomic::Ordering::Relaxed,
            ::core::sync::atomic::Ordering::Relaxed,
            |b| b.checked_sub(1),
        )
        .is_ok()
}
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
/// `Coverage` allocation) even once `CLASS_COVERAGE_CALL_BUDGET` makes
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
pub unsafe fn single_coverage(
    mut _data: FontFilePointer,
    mut _table_length: u32,
    gid: u16,
    mut _offset: u32,
    mut _kind: u16,
    _max_glyphs: GlyphId,
    mut _userdata: *mut ::core::ffi::c_void,
) -> *mut Coverage {
    let cov: *mut Coverage = otl_coverage_create();
    push_to_coverage(cov, handle_from_index(gid) as GlyphHandle);
    return cov;
}
pub unsafe fn class_coverage(
    mut _data: FontFilePointer,
    mut _table_length: u32,
    cls: u16,
    mut _offset: u32,
    kind: u16,
    max_glyphs: GlyphId,
    mut _classdefs: *mut ::core::ffi::c_void,
) -> *mut Coverage {
    let defs: *mut ClassDefs = _classdefs as *mut ClassDefs;
    // `.expect()`, not a null-pointer deref: every caller that reaches here
    // (`general_read_contextual_rule`/`general_read_chaining_rule` via
    // `class_coverage`'s `fn_0` slot) only ever asks for a `kind` whose
    // matching field was populated by `read_contextual_format2`/
    // `read_chaining_format2` beforehand -- `read_class_def` itself never
    // returns null, so this can't actually fail; panicking instead of a
    // would-be null deref matches this migration's general "UB becomes a
    // panic" idiom (see e.g. `general_read_contextual_rule`'s own
    // `.expect()` calls above).
    let cd: *const ClassDef = if kind as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        (*defs).bc.as_deref()
    } else if kind as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        (*defs).ic.as_deref()
    } else {
        (*defs).fc.as_deref()
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
    if CLASS_COVERAGE_CALL_BUDGET
        .try_update(
            ::core::sync::atomic::Ordering::Relaxed,
            ::core::sync::atomic::Ordering::Relaxed,
            |b| b.checked_sub(1),
        )
        .is_err()
    {
        return otl_coverage_create();
    }
    let cov: *mut Coverage = otl_coverage_create();
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
    let zero_budget_left =
        || CLASS_ZERO_BUDGET.load(::core::sync::atomic::Ordering::Relaxed) > 0;
    let charge_zero_budget =
        || CLASS_ZERO_BUDGET.fetch_sub(1, ::core::sync::atomic::Ordering::Relaxed);
    if cls as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        let mut classified = vec![false; max_glyphs as usize];
        let mut j: usize = 0;
        while j < (*cd).glyphs.len() && zero_budget_left() {
            if (&(*cd).classes)[j] as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                let idx = (&(*cd).glyphs)[j].index as usize;
                if idx < classified.len() {
                    classified[idx] = true;
                }
            }
            charge_zero_budget();
            j += 1;
        }
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int) < max_glyphs as ::core::ffi::c_int && zero_budget_left() {
            if !classified[k as usize] {
                push_to_coverage(cov, handle_from_index(k) as GlyphHandle);
            }
            charge_zero_budget();
            k = k.wrapping_add(1);
        }
    } else {
        let mut j_2: GlyphId = 0 as GlyphId;
        while (j_2 as usize) < (*cd).glyphs.len() && zero_budget_left() {
            if (&(*cd).classes)[j_2 as usize] as ::core::ffi::c_int == cls as ::core::ffi::c_int {
                push_to_coverage(
                    cov,
                    otfcc_handle_dup((&(*cd).glyphs)[j_2 as usize].clone() as Handle)
                        as GlyphHandle,
                );
            }
            charge_zero_budget();
            j_2 = j_2.wrapping_add(1);
        }
    }
    return cov;
}
pub unsafe fn format3_coverage(
    data: FontFilePointer,
    table_length: u32,
    shift: u16,
    mut _offset: u32,
    mut _kind: u16,
    _max_glyphs: GlyphId,
    mut _userdata: *mut ::core::ffi::c_void,
) -> *mut Coverage {
    return read_coverage(
        data as *const u8,
        table_length,
        _offset.wrapping_add(shift as u32).wrapping_sub(2_u32),
    );
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
pub unsafe fn general_read_contextual_rule(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    start_gid: u16,
    minus_one: bool,
    fn_0: CoverageReaderHandler,
    max_glyphs: GlyphId,
    userdata: *mut ::core::ffi::c_void,
) -> Option<Box<ChainingRule>> {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
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
    // `otfcc_new_glyf_glyph`.
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
            .push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                data,
                table_length,
                start_gid,
                offset,
                2_u16,
                max_glyphs,
                userdata,
            )));
    }
    for j in 0..n_input_built {
        let gid = FontReader::new(slice)
            .at(offset as usize + 4 + 2 * j as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.match_0
            .push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                data,
                table_length,
                gid,
                offset,
                2_u16,
                max_glyphs,
                userdata,
            )));
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
    reverse_backtracks(&mut *rule as *mut ChainingRule);
    Some(rule)
}
unsafe fn read_contextual_format1(
    subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let mut first_coverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();

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
        // `read_coverage` always returns a valid (possibly empty) `Coverage`
        // shell, never null, even on malformed input -- see coverage.rs.
        first_coverage = read_coverage(data as *const u8, table_length, cov_offset);
        if chain_sub_rule_set_count as usize != (*first_coverage).len() {
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
        // original's own two-pass structure).
        let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
        (*ruleset).rules = Vec::with_capacity(total_rules.min(MAX_TOTAL_RULES_PER_TABLE as usize));
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
                if !take_rule_budget() {
                    break 'rulesets;
                }
                let sr_rel = FontReader::new(slice)
                    .at(srs_offset as usize + 2 + 2 * k as usize)
                    .unwrap()
                    .u16()
                    .unwrap();
                let sr_offset = srs_offset.wrapping_add(sr_rel as u32);
                let rule_ptr = general_read_contextual_rule(
                    data,
                    table_length,
                    sr_offset,
                    (&(*first_coverage))[j as usize].index as u16,
                    true,
                    Some(
                        single_coverage
                            as unsafe fn(
                                FontFilePointer,
                                u32,
                                u16,
                                u32,
                                u16,
                                GlyphId,
                                *mut ::core::ffi::c_void,
                            ) -> *mut Coverage,
                    ),
                    max_glyphs,
                    NULL,
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
                    // Same "malformed individual rule, not the whole subtable" case as
        // the format1/format2 loops above -- see their comment.
        if rule_ptr.is_some() {
            (*ruleset).rules.push(rule_ptr);
        }
                }
            }
        }
        break 'parse Some(());
    };

    // `first_coverage` was leaked on every failure path here (only the
    // success path below ever freed it) -- now freed exactly once,
    // unconditionally, regardless of which branch above bailed out.
    otl_coverage_free(first_coverage);
    if result.is_some() {
        return subtable;
    }
    // `subtable` is `subtable_chaining_create()`'s own `Box`-allocated
    // result (Stage 7-2-d), not `subtable_chaining_free`'s
    // `__caryll_allocate_clean`'d one -- reclaim it with `Box::from_raw`
    // directly, matching `subtable_from_raw`'s own reclamation.
    drop(Box::from_raw(subtable));
    ::core::ptr::null_mut::<ChainingSubtable>()
}
unsafe fn read_contextual_format2(
    subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let mut cds: *mut ClassDefs = ::core::ptr::null_mut::<ClassDefs>();

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

        cds = Box::into_raw(Box::new(ClassDefs {
            bc: None,
            ic: classdef_from_raw(read_class_def(
                data as *const u8,
                table_length,
                offset.wrapping_add(ic_rel as u32),
            )),
            fc: None,
        }));

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

        let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
        (*ruleset).rules = Vec::with_capacity(total_rules.min(MAX_TOTAL_RULES_PER_TABLE as usize));
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
                if !take_rule_budget() {
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
                    data,
                    table_length,
                    sr_offset,
                    j,
                    true,
                    Some(
                        class_coverage
                            as unsafe fn(
                                FontFilePointer,
                                u32,
                                u16,
                                u32,
                                u16,
                                GlyphId,
                                *mut ::core::ffi::c_void,
                            ) -> *mut Coverage,
                    ),
                    max_glyphs,
                    cds as *mut ::core::ffi::c_void,
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
                    // Same "malformed individual rule, not the whole subtable" case as
        // the format1/format2 loops above -- see their comment.
        if rule_ptr.is_some() {
            (*ruleset).rules.push(rule_ptr);
        }
                }
            }
        }
        break 'parse Some(());
    };

    // `cds` cleanup, run exactly once regardless of outcome -- this is the
    // fallthrough-leak fix that was already present (as a second, duplicated
    // copy of this same block) before this conversion; consolidated into
    // one unconditional cleanup rather than newly discovered here. Dropping
    // the reclaimed `Box<ClassDefs>` drops `bc`/`ic`/`fc` first (each an
    // `Option<Box<ClassDef>>`, self-dropping), then deallocates the shell --
    // exactly what the manual `otl_class_def_free` * 3 + `free` sequence
    // used to do by hand. `class_coverage` only ever *reads* through `cds`
    // during the loop above (via the raw pointer handed to it as `fn_0`'s
    // userdata); it never takes ownership away, so there is exactly one
    // owner to drop here, not two -- no double free.
    if !cds.is_null() {
        drop(Box::from_raw(cds));
    }
    if result.is_some() {
        return subtable;
    }
    // `subtable` is `subtable_chaining_create()`'s own `Box`-allocated
    // result (Stage 7-2-d), not `subtable_chaining_free`'s
    // `__caryll_allocate_clean`'d one -- reclaim it with `Box::from_raw`
    // directly, matching `subtable_from_raw`'s own reclamation.
    drop(Box::from_raw(subtable));
    ::core::ptr::null_mut::<ChainingSubtable>()
}
pub unsafe fn otl_read_contextual(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    max_glyphs: GlyphId,
    options: &Options,
) -> *mut Subtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let subtable: *mut ChainingSubtable = (subtable_chaining_create)();
    // `subtable` is fresh from `create()` (a valid, empty `Canonical`
    // value) -- replace it wholesale with a valid, empty `Poly` ruleset.
    // Every downstream construction path (format1/format2/format3, and the
    // error paths that dispose the subtable without ever reaching one) now
    // sees a valid, possibly-still-empty ruleset from this point on.
    *subtable = ChainingSubtable::Poly(ChainingRuleSet::default());
    let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
    let mut format: u16 = 0_u16;
    if let Ok(mut r) = FontReader::new(slice).at(offset as usize) {
        if let Ok(f) = r.u16() {
            format = f;
        }
    }
    if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        return subtable_from_raw(
            read_contextual_format1(subtable, data, table_length, offset, max_glyphs),
            Subtable::Chaining,
        );
    } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        return subtable_from_raw(
            read_contextual_format2(subtable, data, table_length, offset, max_glyphs),
            Subtable::Chaining,
        );
    } else if format as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
        let rule_ptr = general_read_contextual_rule(
            data,
            table_length,
            offset.wrapping_add(2_u32),
            0_u16,
            false,
            Some(
                format3_coverage
                    as unsafe fn(
                        FontFilePointer,
                        u32,
                        u16,
                        u32,
                        u16,
                        GlyphId,
                        *mut ::core::ffi::c_void,
                    ) -> *mut Coverage,
            ),
            max_glyphs,
            NULL,
        );
        // Same "malformed individual rule, not the whole subtable" case as
        // the format1/format2 loops above -- see their comment.
        if rule_ptr.is_some() {
            (*ruleset).rules.push(rule_ptr);
        }
        return subtable_from_raw(subtable, Subtable::Chaining);
    }
    logger_log_sds(
        &mut *options.logger.borrow_mut(),
        LOG_VL_IMPORTANT,
        LoggerType::Warning,
        crate::bytesbuild!(b"Unsupported format ", format as ::core::ffi::c_int, b".\n"),
    );
    // Same reasoning as the format1/format2 helpers' own error paths: this
    // is `subtable_chaining_create()`'s own `Box`-allocated result, reclaim
    // with `Box::from_raw`, not `subtable_chaining_free`.
    drop(Box::from_raw(subtable));
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe fn general_read_chaining_rule(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    start_gid: u16,
    minus_one: bool,
    fn_0: CoverageReaderHandler,
    max_glyphs: GlyphId,
    userdata: *mut ::core::ffi::c_void,
) -> Option<Box<ChainingRule>> {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
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
            .push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                data,
                table_length,
                gid,
                offset,
                1_u16,
                max_glyphs,
                userdata,
            )));
    }
    if minus_one {
        rule.match_0
            .push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                data,
                table_length,
                start_gid,
                offset,
                2_u16,
                max_glyphs,
                userdata,
            )));
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
            .push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                data,
                table_length,
                gid,
                offset,
                2_u16,
                max_glyphs,
                userdata,
            )));
    }
    let lookaround_base = input_base + 2 * n_input_read as usize + 2;
    for j1 in 0..n_lookaround_built {
        let gid = FontReader::new(slice)
            .at(lookaround_base + 2 * j1 as usize)
            .unwrap()
            .u16()
            .unwrap();
        rule.match_0
            .push(coverage_from_raw(fn_0.expect("non-null function pointer")(
                data,
                table_length,
                gid,
                offset,
                3_u16,
                max_glyphs,
                userdata,
            )));
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
    reverse_backtracks(&mut *rule as *mut ChainingRule);
    Some(rule)
}
unsafe fn read_chaining_format1(
    subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let mut first_coverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();

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
        // `read_coverage` always returns a valid (possibly empty) `Coverage`
        // shell, never null, even on malformed input -- see coverage.rs.
        first_coverage = read_coverage(data as *const u8, table_length, cov_offset);
        if chain_sub_rule_set_count as usize != (*first_coverage).len() {
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

        let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
        (*ruleset).rules = Vec::with_capacity(total_rules.min(MAX_TOTAL_RULES_PER_TABLE as usize));
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
                if !take_rule_budget() {
                    break 'rulesets;
                }
                let sr_rel = FontReader::new(slice)
                    .at(srs_offset as usize + 2 + 2 * k as usize)
                    .unwrap()
                    .u16()
                    .unwrap();
                let sr_offset = srs_offset.wrapping_add(sr_rel as u32);
                let rule_ptr = general_read_chaining_rule(
                    data,
                    table_length,
                    sr_offset,
                    (&(*first_coverage))[j as usize].index as u16,
                    true,
                    Some(
                        single_coverage
                            as unsafe fn(
                                FontFilePointer,
                                u32,
                                u16,
                                u32,
                                u16,
                                GlyphId,
                                *mut ::core::ffi::c_void,
                            ) -> *mut Coverage,
                    ),
                    max_glyphs,
                    NULL,
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
                    // Same "malformed individual rule, not the whole subtable" case as
        // the format1/format2 loops above -- see their comment.
        if rule_ptr.is_some() {
            (*ruleset).rules.push(rule_ptr);
        }
                }
            }
        }
        break 'parse Some(());
    };

    // Same fallthrough leak `read_contextual_format1` had: `first_coverage`
    // was only freed on the success path. Freed exactly once here instead.
    otl_coverage_free(first_coverage);
    if result.is_some() {
        return subtable;
    }
    // `subtable` is `subtable_chaining_create()`'s own `Box`-allocated
    // result (Stage 7-2-d), not `subtable_chaining_free`'s
    // `__caryll_allocate_clean`'d one -- reclaim it with `Box::from_raw`
    // directly, matching `subtable_from_raw`'s own reclamation.
    drop(Box::from_raw(subtable));
    ::core::ptr::null_mut::<ChainingSubtable>()
}
unsafe fn read_chaining_format2(
    subtable: *mut ChainingSubtable,
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    max_glyphs: GlyphId,
) -> *mut ChainingSubtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let mut cds: *mut ClassDefs = ::core::ptr::null_mut::<ClassDefs>();

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

        cds = Box::into_raw(Box::new(ClassDefs {
            bc: classdef_from_raw(read_class_def(
                data as *const u8,
                table_length,
                offset.wrapping_add(bc_rel as u32),
            )),
            ic: classdef_from_raw(read_class_def(
                data as *const u8,
                table_length,
                offset.wrapping_add(ic_rel as u32),
            )),
            fc: classdef_from_raw(read_class_def(
                data as *const u8,
                table_length,
                offset.wrapping_add(fc_rel as u32),
            )),
        }));

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

        let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
        (*ruleset).rules = Vec::with_capacity(total_rules.min(MAX_TOTAL_RULES_PER_TABLE as usize));
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
                if !take_rule_budget() {
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
                    data,
                    table_length,
                    sr_offset,
                    j,
                    true,
                    Some(
                        class_coverage
                            as unsafe fn(
                                FontFilePointer,
                                u32,
                                u16,
                                u32,
                                u16,
                                GlyphId,
                                *mut ::core::ffi::c_void,
                            ) -> *mut Coverage,
                    ),
                    max_glyphs,
                    cds as *mut ::core::ffi::c_void,
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
                    // Same "malformed individual rule, not the whole subtable" case as
        // the format1/format2 loops above -- see their comment.
        if rule_ptr.is_some() {
            (*ruleset).rules.push(rule_ptr);
        }
                }
            }
        }
        break 'parse Some(());
    };

    // `cds` cleanup, run exactly once regardless of outcome -- same
    // consolidation as `read_contextual_format2` (the fallthrough-leak fix
    // was already present as a duplicated block before this conversion).
    // See that function's comment for why a single `Box` drop is the exact
    // replacement for the old manual `otl_class_def_free` * 3 + `free`
    // sequence, with no double-free risk.
    if !cds.is_null() {
        drop(Box::from_raw(cds));
    }
    if result.is_some() {
        return subtable;
    }
    // `subtable` is `subtable_chaining_create()`'s own `Box`-allocated
    // result (Stage 7-2-d), not `subtable_chaining_free`'s
    // `__caryll_allocate_clean`'d one -- reclaim it with `Box::from_raw`
    // directly, matching `subtable_from_raw`'s own reclamation.
    drop(Box::from_raw(subtable));
    ::core::ptr::null_mut::<ChainingSubtable>()
}
pub unsafe fn otl_read_chaining(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    max_glyphs: GlyphId,
    options: &Options,
) -> *mut Subtable {
    let slice = ::core::slice::from_raw_parts(data, table_length as usize);
    let subtable: *mut ChainingSubtable = (subtable_chaining_create)();
    // See the identical comment in `otl_read_contextual`.
    *subtable = ChainingSubtable::Poly(ChainingRuleSet::default());
    let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable);
    let mut format: u16 = 0_u16;
    if let Ok(mut r) = FontReader::new(slice).at(offset as usize) {
        if let Ok(f) = r.u16() {
            format = f;
        }
    }
    if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        return subtable_from_raw(
            read_chaining_format1(subtable, data, table_length, offset, max_glyphs),
            Subtable::Chaining,
        );
    } else if format as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        return subtable_from_raw(
            read_chaining_format2(subtable, data, table_length, offset, max_glyphs),
            Subtable::Chaining,
        );
    } else if format as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
        let rule_ptr = general_read_chaining_rule(
            data,
            table_length,
            offset.wrapping_add(2_u32),
            0_u16,
            false,
            Some(
                format3_coverage
                    as unsafe fn(
                        FontFilePointer,
                        u32,
                        u16,
                        u32,
                        u16,
                        GlyphId,
                        *mut ::core::ffi::c_void,
                    ) -> *mut Coverage,
            ),
            max_glyphs,
            NULL,
        );
        // Same "malformed individual rule, not the whole subtable" case as
        // the format1/format2 loops above -- see their comment.
        if rule_ptr.is_some() {
            (*ruleset).rules.push(rule_ptr);
        }
        return subtable_from_raw(subtable, Subtable::Chaining);
    }
    logger_log_sds(
        &mut *options.logger.borrow_mut(),
        LOG_VL_IMPORTANT,
        LoggerType::Warning,
        crate::bytesbuild!(b"Unsupported format ", format as ::core::ffi::c_int, b".\n"),
    );
    // Same reasoning as the format1/format2 helpers' own error paths: this
    // is `subtable_chaining_create()`'s own `Box`-allocated result, reclaim
    // with `Box::from_raw`, not `subtable_chaining_free`.
    drop(Box::from_raw(subtable));
    return ::core::ptr::null_mut::<Subtable>();
}
#[inline]
// Was a manual meet-in-the-middle index-swapping loop over
// `*mut *mut Coverage` -- exactly `[T]::reverse` on the backtrack
// sub-slice, now that `match_0` is a real `Vec<Coverage>`. `input_begins
// == 0` (nothing to reverse) falls out of slicing an empty range.
unsafe fn reverse_backtracks(rule: *mut ChainingRule) {
    let input_begins = (*rule).input_begins as usize;
    (&mut (*rule).match_0)[..input_begins].reverse();
}

#[cfg(test)]
mod chaining_read_tests {
    use super::*;

    fn zeroed_options() -> Options {
        Options::default()
    }

    unsafe fn glyphs_of(cov: &Coverage) -> Vec<GlyphId> {
        cov.iter().map(|h| h.index).collect()
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
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::Chaining(sub) = &*boxed else {
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
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::Chaining(sub) = &*boxed else {
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
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(raw.is_null());
        }
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
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(raw.is_null());
        }
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
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::Chaining(sub) = &*boxed else {
                unreachable!()
            };
            let ChainingSubtable::Poly(ruleset) = sub else {
                unreachable!()
            };
            assert!(ruleset.rules.is_empty());
        }
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
        let options = zeroed_options();
        unsafe {
            let raw = otl_read_chaining(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(!raw.is_null());
            let boxed = Box::from_raw(raw);
            let Subtable::Chaining(sub) = &*boxed else {
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
                    .map(|c| glyphs_of(c))
                    .collect::<Vec<_>>(),
                vec![vec![1], vec![2], vec![3]]
            );
            assert_eq!(rule.input_begins, 1);
            assert_eq!(rule.input_ends, 2);
        }
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
        unsafe {
            let rule = general_read_chaining_rule(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                7,
                true,
                Some(
                    single_coverage
                        as unsafe fn(
                            FontFilePointer,
                            u32,
                            u16,
                            u32,
                            u16,
                            GlyphId,
                            *mut ::core::ffi::c_void,
                        ) -> *mut Coverage,
                ),
                100,
                NULL,
            );
            let rule = rule.unwrap();
            // Only the `minus_one` slot (glyph 7, from `start_gid`) is
            // filled; the (empty) input array contributes nothing.
            assert_eq!(rule.match_0.len(), 1);
            assert_eq!(glyphs_of(&rule.match_0[0]), vec![7]);
        }
    }

    #[test]
    fn unsupported_format_logs_and_returns_null() {
        let data = [0u8, 9]; // format = 9
        // This path logs unconditionally, so `options.logger` must be a
        // real, usable `Logger`, not null -- automatic now that `Options::
        // default()`'s `logger` is a real (if `LoggerTarget::Empty`, i.e.
        // no-op-push) `Logger` rather than a null pointer.
        let options = Options::default();
        unsafe {
            let raw = otl_read_contextual(
                data.as_ptr() as FontFilePointer,
                data.len() as u32,
                0,
                100,
                &options,
            );
            assert!(raw.is_null());
        }
    }
}
