use crate::support::handle::{GlyphHandle, Handle, HandleState};
use crate::table::otl::coverage::{Coverage, shrink_coverage};

use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};

use crate::support::options::Options;
use crate::support::primitives::GlyphId;

use crate::font::caryll_font::Font;

use crate::table::otl::{GsubMultiEntry, OtlTable, Subtable};

use crate::consolidate::otl::common::fontop_consolidate_coverage;
use crate::support::glyph_order::otfcc_gord_consolidate_handle;
use crate::table::otl::subtables::gsub_multi::dispose_gsub_multi_subtable;

pub fn consolidate_gsub_multi(
    font: &Font,
    _table: *const OtlTable,
    _subtable: &mut Subtable,
    options: &Options,
) -> bool {
    let Subtable::GsubMulti(subtable) = _subtable else {
        unreachable!()
    };
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
    while (k as usize) < subtable.len() {
        // Guaranteed `Some`: `consolidate_otl` (and hence this function)
        // only ever runs when `glyf` is present, and `otfcc_consolidate_font`
        // always populates `glyph_order` before that, whenever `glyf` is
        // present.
        if !otfcc_gord_consolidate_handle(
            font.glyph_order.as_deref().unwrap(),
            &mut subtable[k as usize].from,
        ) {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(
                    b"[Consolidate] Ignored missing glyph /",
                    &subtable[k as usize].from.name,
                    b".\n",
                ),
            );
        } else {
            fontop_consolidate_coverage(
                font.glyph_order.as_deref().unwrap(),
                &mut subtable[k as usize].to,
                options,
            );
            shrink_coverage(&mut subtable[k as usize].to, false);
            if subtable[k as usize].to.is_empty() {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_IMPORTANT,
                    LoggerType::Warning,
                    crate::bytesbuild!(b"[Consolidate] Ignoring empty one-to-many / alternative substitution for glyph /",
                        &subtable[k as usize].from.name,
                        b".\n",
                    ),
                );
            } else {
                let fromid: i32 = subtable[k as usize].from.index as i32;
                if !seen.contains_key(&fromid) {
                    let fromname: Vec<u8> = subtable[k as usize].from.name.clone();
                    let to: Coverage = ::core::mem::take(&mut subtable[k as usize].to);
                    seen.insert(fromid, (fromname, to));
                }
            }
        }
        k = k.wrapping_add(1);
    }
    dispose_gsub_multi_subtable(subtable);
    for (fromid, (fromname, to)) in seen {
        subtable.push(GsubMultiEntry {
            from: Handle {
                state: HandleState::Consolidated,
                index: fromid as GlyphId,
                name: fromname,
            } as GlyphHandle,
            to,
        });
    }
    subtable.len() == 0_usize
}
pub fn consolidate_gsub_alternative(
    font: &Font,
    table: *const OtlTable,
    _subtable: &mut Subtable,
    options: &Options,
) -> bool {
    consolidate_gsub_multi(font, table, _subtable, options)
}
