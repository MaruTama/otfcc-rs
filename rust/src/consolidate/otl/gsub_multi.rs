#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::table::otl::coverage::{Coverage, shrink_coverage};
use crate::support::handle::{handle_from_consolidated, GlyphHandle};

use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};
use crate::vendor::sds::{SdsRaw};

use crate::font::caryll_font::{Font};
























use crate::table::otl::{GsubMultiEntry, Subtable, GsubMultiSubtable, OtlTable};





use crate::consolidate::otl::common::{fontop_consolidate_coverage};
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::table::otl::subtables::gsub_multi::{dispose_gsub_multi_subtable};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree};




pub unsafe extern "C" fn consolidate_gsub_multi(
    mut font: *mut Font,
    mut _table: *mut OtlTable,
    mut _subtable: *mut Subtable,
    mut options: *const Options,
) -> bool {
    let mut subtable: *mut GsubMultiSubtable = &raw mut (*_subtable).gsub_multi as *mut GsubMultiSubtable;
    // Deduplicates by `from.index`, first occurrence wins -- a later
    // duplicate's already-consolidated `to` coverage is simply dropped along
    // with the rest of the pre-dedup `subtable` when it's disposed below,
    // exactly what the uthash HASH_FIND-then-skip-if-found this replaced
    // did. `BTreeMap`, not `IndexMap`: the original also did a HASH_SORT by
    // `fromid` right before reading entries back out, so the final order is
    // ascending by glyph id, not insertion order -- a `BTreeMap`'s iteration
    // order already is that, for free.
    let mut seen: std::collections::BTreeMap<i32, (SdsRaw, *mut Coverage)> =
        std::collections::BTreeMap::new();
    let mut k: GlyphId = 0 as GlyphId;
    while (k as usize) < (*subtable).len() {
        if !OTFCC_PKG_GLYPH_ORDER
            .consolidate_handle
            .expect("non-null function pointer")(
            (*font).glyph_order,
            &raw mut (&mut (*subtable))[k as usize].from,
        ) {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[Consolidate] Ignored missing glyph /",
                    (&(*subtable))[k as usize].from.name,
                    b".\n",
                ),
            );
        } else {
            fontop_consolidate_coverage(font, (&(*subtable))[k as usize].to, options);
            shrink_coverage(
                (&(*subtable))[k as usize].to,
                false,
            );
            if (*(&(*subtable))[k as usize].to).is_empty() {
                (*(*options).logger)
                    .log_sds
                    .expect(
                        "non-null function pointer",
                    )(
                    (*options).logger as *mut ILogger,
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"[Consolidate] Ignoring empty one-to-many / alternative substitution for glyph /",
                        (&(*subtable))[k as usize].from.name,
                        b".\n",
                    ),
                );
            } else {
                let fromid: i32 = (&(*subtable))[k as usize].from.index as i32;
                if !seen.contains_key(&fromid) {
                    let fromname: SdsRaw = sdsdup((&(*subtable))[k as usize].from.name);
                    let to: *mut Coverage = (&(*subtable))[k as usize].to;
                    let ref mut fresh0 = (&mut (*subtable))[k as usize].to;
                    *fresh0 = ::core::ptr::null_mut::<Coverage>();
                    seen.insert(fromid, (fromname, to));
                }
            }
        }
        k = k.wrapping_add(1);
    }
    dispose_gsub_multi_subtable(subtable);
    for (fromid, (fromname, to)) in seen {
        (*subtable).push(GsubMultiEntry {
            from: handle_from_consolidated(fromid as GlyphId, fromname) as GlyphHandle,
            to,
        });
        sdsfree(fromname);
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
