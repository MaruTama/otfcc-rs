#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::table::otl::coverage::{Coverage, shrink_coverage};
use crate::support::handle::{Handle, HandleState, GlyphHandle};

use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};

use crate::font::caryll_font::{Font};
























use crate::table::otl::{GsubMultiEntry, Subtable, GsubMultiSubtable, OtlTable};





use crate::consolidate::otl::common::{fontop_consolidate_coverage};
use crate::support::glyph_order::{otfcc_gord_consolidate_handle, GlyphOrder};
use crate::table::otl::subtables::gsub_multi::{dispose_gsub_multi_subtable};




pub unsafe extern "C" fn consolidate_gsub_multi(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let Subtable::GsubMulti(mut_subtable) = &mut *_subtable else { unreachable!() };
    let subtable: *mut GsubMultiSubtable = mut_subtable;
    // Deduplicates by `from.index`, first occurrence wins -- a later
    // duplicate's already-consolidated `to` coverage is simply dropped along
    // with the rest of the pre-dedup `subtable` when it's disposed below,
    // exactly what the uthash HASH_FIND-then-skip-if-found this replaced
    // did. `BTreeMap`, not `IndexMap`: the original also did a HASH_SORT by
    // `fromid` right before reading entries back out, so the final order is
    // ascending by glyph id, not insertion order -- a `BTreeMap`'s iteration
    // order already is that, for free.
    let mut seen: std::collections::BTreeMap<i32, (Vec<u8>, Coverage)> =
        std::collections::BTreeMap::new();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*subtable).len() {
        if !otfcc_gord_consolidate_handle(
            (*font).glyph_order.as_deref_mut().map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder),
            &raw mut (&mut (*subtable))[k as usize].from,
        ) {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"[Consolidate] Ignored missing glyph /",
                    &(&(*subtable))[k as usize].from.name,
                    b".\n",
                ),
            );
        } else {
            fontop_consolidate_coverage(
                font,
                &mut (&mut (*subtable))[k as usize].to as *mut Coverage,
                options,
            );
            shrink_coverage(
                &mut (&mut (*subtable))[k as usize].to as *mut Coverage,
                false,
            );
            if (&(*subtable))[k as usize].to.is_empty() {
                (*(*options).logger)
                    .log_sds
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(b"[Consolidate] Ignoring empty one-to-many / alternative substitution for glyph /",
                        &(&(*subtable))[k as usize].from.name,
                        b".\n",
                    ),
                );
            } else {
                let fromid: i32 = (&(*subtable))[k as usize].from.index as i32;
                if !seen.contains_key(&fromid) {
                    let fromname: Vec<u8> = (&(*subtable))[k as usize].from.name.clone();
                    let to: Coverage =
                        ::core::mem::take(&mut (&mut (*subtable))[k as usize].to);
                    seen.insert(fromid, (fromname, to));
                }
            }
        }
        k = k.wrapping_add(1);
    }
    dispose_gsub_multi_subtable(subtable);
    for (fromid, (fromname, to)) in seen {
        (*subtable).push(GsubMultiEntry {
            from: Handle {
                state: HandleState::Consolidated,
                index: fromid as GlyphId,
                name: fromname,
            } as GlyphHandle,
            to,
        });
    }
    return (*subtable).len() == 0 as usize;
}
pub unsafe extern "C" fn consolidate_gsub_alternative(
    mut font: *mut Font,
    mut table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    return consolidate_gsub_multi(font, table, _subtable, options);
}
