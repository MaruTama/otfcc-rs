#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::handle::{
    GlyphHandle, Handle, HandleState, LookupHandle, handle_from_index, otfcc_handle_dup,
};
use crate::table::otl::classdef::{ClassDef, otl_class_def_create, push_class_def};
use crate::table::otl::coverage::{
    Coverage, coverage_from_raw, otl_coverage_create, push_to_coverage,
};

use crate::support::alloc::__caryll_allocate_clean;
use crate::support::buffer::Buffer;
use crate::support::primitives::{GlyphClass, GlyphId, TableId};

use crate::table::otl::classdef::classdef_from_raw;
use crate::table::otl::subtables::chaining::build::{
    otfcc_build_chaining, otfcc_build_contextual, otfcc_chaining_lookup_is_contextual_lookup,
};
use crate::table::otl::subtables::chaining::common::{
    chaining_is_canonical, chaining_rule_mut, chaining_ruleset_mut, subtable_chaining_free,
};
use crate::table::otl::{
    ChainLookupApplication, ChainingRule, ChainingRuleSet, ChainingSubtable, Lookup, Subtable,
    SubtablePtr, subtable_at,
};
#[derive(Clone)]
pub struct ClassifierValue {
    pub gname: Vec<u8>,
    pub cls: ::core::ffi::c_int,
}
unsafe fn class_compatible(
    h: &mut std::collections::BTreeMap<GlyphId, ClassifierValue>,
    mut cov: *mut Coverage,
    mut past: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*cov).len() == 0 as usize {
        return 1 as ::core::ffi::c_int;
    }
    let gid: GlyphId = (&(*cov))[0].index;
    match h.get(&gid).map(|v| v.cls) {
        Some(cls) => {
            let mut j: GlyphId = 1 as GlyphId;
            while (j as usize) < (*cov).len() {
                let gid_0: GlyphId = (&(*cov))[j as usize].index;
                match h.get(&gid_0) {
                    Some(ss) if ss.cls == cls => {}
                    _ => return 0 as ::core::ffi::c_int,
                }
                j = j.wrapping_add(1);
            }
            // Original built a throwaway `revh` -- a hash of `cov`'s own
            // (deduped) glyph ids, values unused (only ever a presence
            // check) -- to answer the *reverse* question below. Only
            // presence and position matter, same finding as
            // `PairClassifierHash`, so a bare `HashSet<GlyphId>` replaces
            // it, with no `gname`/`cls` payload to carry at all.
            let mut revset: std::collections::HashSet<GlyphId> = std::collections::HashSet::new();
            let mut j_0: GlyphId = 0 as GlyphId;
            while (j_0 as usize) < (*cov).len() {
                revset.insert((&(*cov))[j_0 as usize].index);
                j_0 = j_0.wrapping_add(1);
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
            return if allcheck {
                cls
            } else {
                0 as ::core::ffi::c_int
            };
        }
        None => {
            let mut j_1: GlyphId = 1 as GlyphId;
            while (j_1 as usize) < (*cov).len() {
                let gid_3: GlyphId = (&(*cov))[j_1 as usize].index;
                if h.contains_key(&gid_3) {
                    return 0 as ::core::ffi::c_int;
                }
                j_1 = j_1.wrapping_add(1);
            }
            let new_cls: ::core::ffi::c_int = *past + 1 as ::core::ffi::c_int;
            let mut j_2: GlyphId = 0 as GlyphId;
            while (j_2 as usize) < (*cov).len() {
                let gid_4: GlyphId = (&(*cov))[j_2 as usize].index;
                let gname: Vec<u8> = (&(*cov))[j_2 as usize].name.clone();
                h.entry(gid_4).or_insert(ClassifierValue {
                    gname,
                    cls: new_cls,
                });
                j_2 = j_2.wrapping_add(1);
            }
            *past += 1 as ::core::ffi::c_int;
            return 1 as ::core::ffi::c_int;
        }
    }
}
unsafe fn build_rule(
    mut rule: *mut ChainingRule,
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
        match_count: (*rule).match_count,
        input_begins: (*rule).input_begins,
        input_ends: (*rule).input_ends,
        match_0: Vec::with_capacity((*rule).match_count as usize),
        apply: Vec::new(),
    });
    let mut m: TableId = 0 as TableId;
    while (m as ::core::ffi::c_int) < (*rule).match_count as ::core::ffi::c_int {
        let cov: *mut Coverage = otl_coverage_create();
        if (&(*rule).match_0)[m as usize].len() > 0 as usize {
            let h: &std::collections::BTreeMap<GlyphId, ClassifierValue> =
                if (m as ::core::ffi::c_int) < (*rule).input_begins as ::core::ffi::c_int {
                    hb
                } else if (m as ::core::ffi::c_int) < (*rule).input_ends as ::core::ffi::c_int {
                    hi
                } else {
                    hf
                };
            let gid: GlyphId = (&(*rule).match_0)[m as usize][0].index;
            // `h.get(&gid)` is unreachable-as-`None` in practice: every
            // glyph reaching this point already passed `class_compatible`
            // for this same `h`, which never returns success without
            // having inserted (or already found) that glyph. The
            // fallback to class 0 mirrors the empty-coverage `else`
            // branch below rather than asserting, matching this
            // migration's established handling of `None` arms that the
            // algorithm's own invariants rule out (see `ClassNameHash`
            // in rust/README.md).
            let cls: GlyphClass = match h.get(&gid) {
                Some(v) => v.cls as GlyphClass,
                None => 0 as GlyphClass,
            };
            push_to_coverage(cov, handle_from_index(cls) as GlyphHandle);
        } else {
            push_to_coverage(cov, handle_from_index(0 as GlyphId) as GlyphHandle);
        }
        (*new_rule).match_0.push(coverage_from_raw(cov));
        m = m.wrapping_add(1);
    }
    // Plain assignment is fine here (unlike the calloc'd-memory case
    // elsewhere in this crate): `Box::new` above already gave `.apply` a
    // valid empty `Vec`, so there's a real (if empty) value to drop first.
    (*new_rule).apply = Vec::with_capacity((*rule).apply.len());
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*rule).apply.len() {
        let index = (&(*rule).apply)[j as usize].index;
        let lookup =
            otfcc_handle_dup((&(*rule).apply)[j as usize].lookup.clone() as Handle) as LookupHandle;
        (*new_rule)
            .apply
            .push(ChainLookupApplication { index, lookup });
        j = j.wrapping_add(1);
    }
    return new_rule;
}
unsafe fn to_class(h: &std::collections::BTreeMap<GlyphId, ClassifierValue>) -> *mut ClassDef {
    // The dedup key (gid) and the original's `HASH_SORT` key (also gid,
    // via `by_gid_clsh`) are the same, so `BTreeMap`'s natural `Ord`
    // reproduces the sorted walk with no separate sort step -- the
    // `by_gid_clsh` comparator itself is gone, subsumed entirely by the
    // container. Borrowing rather than consuming `h` matches the
    // original, where `to_class` sorts and reads but never frees --
    // disposal was always the caller's job, and here that's simply
    // `try_classify_around` letting its `BTreeMap`s drop at scope exit.
    let cd: *mut ClassDef = otl_class_def_create();
    for (&gid, v) in h.iter() {
        push_class_def(
            cd,
            Handle {
                state: HandleState::Consolidated,
                index: gid,
                name: v.gname.clone(),
            } as GlyphHandle,
            v.cls as GlyphClass,
        );
    }
    return cd;
}
pub unsafe fn try_classify_around(
    mut lookup: *const Lookup,
    mut j: TableId,
    mut classified_st: *mut *mut ChainingSubtable,
) -> TableId {
    let mut current_block: u64;
    let mut compatible_count: TableId = 0 as TableId;
    let mut hb: std::collections::BTreeMap<GlyphId, ClassifierValue> =
        std::collections::BTreeMap::new();
    let mut hi: std::collections::BTreeMap<GlyphId, ClassifierValue> =
        std::collections::BTreeMap::new();
    let mut hf: std::collections::BTreeMap<GlyphId, ClassifierValue> =
        std::collections::BTreeMap::new();
    let subtable0_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, j as usize);
    let Subtable::Chaining(mut_subtable0) = &mut *subtable0_ptr else {
        unreachable!()
    };
    let mut subtable0: *mut ChainingSubtable = mut_subtable0;
    let mut classno_b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut classno_i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut classno_f: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rule0: *mut ChainingRule = chaining_rule_mut(subtable0);
    let mut m: TableId = 0 as TableId;
    loop {
        if !((m as ::core::ffi::c_int) < (*rule0).match_count as ::core::ffi::c_int) {
            current_block = 12349973810996921269;
            break;
        }
        let mut check: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (m as ::core::ffi::c_int) < (*rule0).input_begins as ::core::ffi::c_int {
            check = class_compatible(
                &mut hb,
                &mut (&mut (*rule0).match_0)[m as usize] as *mut Coverage,
                &raw mut classno_b,
            );
        } else if (m as ::core::ffi::c_int) < (*rule0).input_ends as ::core::ffi::c_int {
            check = class_compatible(
                &mut hi,
                &mut (&mut (*rule0).match_0)[m as usize] as *mut Coverage,
                &raw mut classno_i,
            );
        } else {
            check = class_compatible(
                &mut hf,
                &mut (&mut (*rule0).match_0)[m as usize] as *mut Coverage,
                &raw mut classno_f,
            );
        }
        if check == 0 {
            current_block = 1622411330066726685;
            break;
        }
        m = m.wrapping_add(1);
    }
    match current_block {
        12349973810996921269 => {
            let mut k: TableId = (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
            's_74: while (k as usize) < (*lookup).subtables.len() {
                let k_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, k as usize);
                let Subtable::Chaining(mut_subtable_k) = &mut *k_ptr else {
                    unreachable!()
                };
                let subtable_k: *mut ChainingSubtable = mut_subtable_k;
                let mut rule: *mut ChainingRule = chaining_rule_mut(subtable_k);
                let mut allcheck: bool = true;
                let mut m_0: TableId = 0 as TableId;
                while (m_0 as ::core::ffi::c_int) < (*rule).match_count as ::core::ffi::c_int {
                    let mut check_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if (m_0 as ::core::ffi::c_int) < (*rule).input_begins as ::core::ffi::c_int {
                        check_0 = class_compatible(
                            &mut hb,
                            &mut (&mut (*rule).match_0)[m_0 as usize] as *mut Coverage,
                            &raw mut classno_b,
                        );
                    } else if (m_0 as ::core::ffi::c_int) < (*rule).input_ends as ::core::ffi::c_int
                    {
                        check_0 = class_compatible(
                            &mut hi,
                            &mut (&mut (*rule).match_0)[m_0 as usize] as *mut Coverage,
                            &raw mut classno_i,
                        );
                    } else {
                        check_0 = class_compatible(
                            &mut hf,
                            &mut (&mut (*rule).match_0)[m_0 as usize] as *mut Coverage,
                            &raw mut classno_f,
                        );
                    }
                    if check_0 == 0 {
                        allcheck = false;
                        break 's_74;
                    } else {
                        m_0 = m_0.wrapping_add(1);
                    }
                }
                if allcheck {
                    compatible_count = (compatible_count as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as TableId;
                }
                k = k.wrapping_add(1);
            }
            if compatible_count as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                subtable0 = __caryll_allocate_clean(
                    ::core::mem::size_of::<ChainingSubtable>() as usize,
                    170 as ::core::ffi::c_ulong,
                ) as *mut ChainingSubtable;
                // Place a valid `Classified` value directly -- the zeroed
                // memory `__caryll_allocate_clean` hands back is not a
                // valid `ChainingSubtable` bit pattern (it owns `Vec`/
                // `Option<Box<_>>` fields), so there is nothing to drop
                // first, same reasoning as `otl_init_chaining`. `bc`/`ic`/
                // `fc` start `None` and are filled in below, after `to_class`
                // (unavailable yet, still needs `hb`/`hi`/`hf` built first).
                ::core::ptr::write(
                    subtable0,
                    ChainingSubtable::Classified(ChainingRuleSet {
                        rules: Vec::with_capacity(
                            (compatible_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as usize,
                        ),
                        ..Default::default()
                    }),
                );
                let ruleset: *mut ChainingRuleSet = chaining_ruleset_mut(subtable0);
                (*ruleset)
                    .rules
                    .push(Some(build_rule(rule0, &hb, &hi, &hf)));
                let mut kk: TableId = 1 as TableId;
                let mut k_0: TableId =
                    (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
                while (k_0 as usize) < (*lookup).subtables.len()
                    && (kk as ::core::ffi::c_int)
                        < compatible_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                {
                    let k_0_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, k_0 as usize);
                    let Subtable::Chaining(mut_subtable_k_0) = &mut *k_0_ptr else {
                        unreachable!()
                    };
                    let subtable_k_0: *mut ChainingSubtable = mut_subtable_k_0;
                    let mut rule_0: *mut ChainingRule = chaining_rule_mut(subtable_k_0);
                    (*ruleset)
                        .rules
                        .push(Some(build_rule(rule_0, &hb, &hi, &hf)));
                    kk = kk.wrapping_add(1);
                    k_0 = k_0.wrapping_add(1);
                }
                (*ruleset).bc = classdef_from_raw(to_class(&hb));
                (*ruleset).ic = classdef_from_raw(to_class(&hi));
                (*ruleset).fc = classdef_from_raw(to_class(&hf));
                *classified_st = subtable0;
            }
        }
        _ => {}
    }
    // hb/hi/hf are owned BTreeMaps now (not uthash nodes reached via
    // a raw *mut), so they need no manual HASH_ITER+HASH_DEL+free walk
    // here -- they simply drop when this function returns, whether or
    // not to_class borrowed them above.
    if compatible_count as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        return compatible_count;
    } else {
        return 0 as TableId;
    };
}
pub unsafe fn otfcc_classified_build_chaining(
    mut lookup: *const Lookup,
    subtable_buffers: &mut Vec<*mut Buffer>,
    mut last_offset: *mut usize,
) -> TableId {
    let mut is_contextual: bool = otfcc_chaining_lookup_is_contextual_lookup(lookup);
    let mut subtables_written: TableId = 0 as TableId;
    subtable_buffers.clear();
    subtable_buffers.reserve((*lookup).subtables.len());
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*lookup).subtables.len() {
        let j_ptr: SubtablePtr = subtable_at(&(*lookup).subtables, j as usize);
        let Subtable::Chaining(mut_st0) = &mut *j_ptr else {
            unreachable!()
        };
        let st0: *mut ChainingSubtable = mut_st0;
        if chaining_is_canonical(st0) {
            let mut st: *mut ChainingSubtable = st0;
            j = (j as ::core::ffi::c_int
                + try_classify_around(lookup, j, &raw mut st) as ::core::ffi::c_int)
                as TableId;
            let mut buf: *mut Buffer = if is_contextual as ::core::ffi::c_int != 0 {
                otfcc_build_contextual(st)
            } else {
                otfcc_build_chaining(st)
            };
            if st != st0 {
                subtable_chaining_free(st);
            }
            subtable_buffers.push(buf);
            *last_offset = (*last_offset).wrapping_add((*buf).data.len());
            subtables_written =
                (subtables_written as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
        }
        j = j.wrapping_add(1);
    }
    return subtables_written;
}
