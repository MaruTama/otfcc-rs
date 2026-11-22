//! GPOS lookup type 2, pair adjustment. Both formats are held as a class
//! matrix; building writes whichever format comes out smaller.

use crate::support::handle::{GlyphHandle, handle_from_index};
use crate::table::otl::budget::{Limit, OtlReadBudget};
use crate::table::otl::classdef::{ClassDef, expand_class_def, push_class_def, read_class_def};
use crate::table::otl::coverage::{Coverage, push_to_coverage, read_coverage, shrink_coverage};
use otfcc_json::ParsedValue;

use otfcc_binary::FontReader;

use crate::support::primitives::{GlyphClass, GlyphId, Pos, TableId};
use crate::table::otl::classdef::{build_class_def, dump_class_def, parse_class_def};
use crate::table::otl::coverage::build_coverage;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::subtables::gpos_common::{
    FORMAT_DWIDTH, bk_gpos_value, gpos_dump_value, gpos_parse_value, position_format_length,
    position_zero, read_gpos_value, required_position_format,
};
use crate::table::otl::{GposPairSubtable, PositionValue, Subtable};
use otfcc_binary::Buffer;
use otfcc_binary::bk::block::bk_new_block_from_buffer;
use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use otfcc_binary::bk::graph::BkGraph;
use otfcc_binary::bk::graph::{
    bk_build_graph, bk_estimate_size_of_graph, bk_minimize_graph, bk_new_graph_from_root_block,
    bk_untangle_graph,
};
use otfcc_json::BuiltValue;
use otfcc_json::JsonType;

/// One pair of a PairPos format 1 PairSet while it is being built: the
/// second glyph and the values for the first and the second glyph.
#[derive(Copy, Clone, Debug)]
pub struct IndividualGposPair {
    pub gid: GlyphId,
    pub fv: PositionValue,
    pub sv: PositionValue,
}
// Format 1 is read field by field with `FontReader`, so a table too short
// for its header is rejected. Format 2's class grid needs `class1_count *
// class2_count` cells of `len1 + len2` bytes; the byte check uses checked
// arithmetic, since two `u16` counts can reach ~68.7 billion bytes.
//
// The cell count is also capped on its own: with both value formats 0 the
// grid takes no bytes, so the byte check passes for any count, yet the two
// grids are still allocated (a fuzzed 6000x6000 subtable reached 2.24 GB).
// The largest real product in this repo's fonts is Cormorant-Medium's
// 384 * 1560 = 599,040, so 2,000,000 leaves more than 3x headroom.
const MAX_TOTAL_GPOS_PAIR_CLASS_CELLS: usize = 2_000_000;
// The cap above is per subtable; this one is for the whole table, and covers
// both formats. A subtable's grids stay allocated until the table is read, so
// many subtables each under the per-subtable cap still add up: a fuzzed GPOS
// of ~100 format 1 subtables, each with an empty coverage (which reads as
// 65,536 classes of one cell), asked for ~20M cells and ~1.5 GB. The most any
// font in this repo spends is under 30,000 cells, so 3,000,000 leaves
// generous headroom. Each cell costs ~144 bytes once row overhead is counted,
// so the cap keeps the grids near 430 MB (8,000,000 allowed over 1 GB, which
// ran the fuzzer out of memory under ASan).
pub(crate) const MAX_TOTAL_GPOS_PAIR_GRID_CELLS_PER_TABLE: u32 = 3_000_000;
/// The second glyph of the pair record at `pos`. The PairSet's `require_room`
/// check (record count times stride) already showed every record of the set
/// lies inside `data`, so this cannot fail on input that got that far.
fn pair_second_glyph(data: &[u8], pos: usize) -> u16 {
    FontReader::new(data)
        .at(pos)
        .and_then(|mut r| r.u16())
        .expect("the PairSet's require_room covered every record")
}
/// Reads a pair adjustment subtable (GPOS 2) of either format. Both are kept
/// in the format 2 shape: a class definition for each side and a value
/// matrix per side. Returns `None` for an unknown format or a malformed
/// subtable.
pub fn otl_read_gpos_pair(
    data: &[u8],
    offset: u32,
    _max_glyphs: GlyphId,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    let subtable_format = FontReader::new(data)
        .at(offset as usize)
        .and_then(|mut r| r.u16())
        .ok()?;
    match subtable_format {
        1 => read_pair_format1(data, offset, budget),
        2 => read_pair_format2(data, offset, budget),
        _ => None,
    }
}
/// Format 1: one PairSet of individual pairs per covered first glyph.
fn read_pair_format1(data: &[u8], offset: u32, budget: &mut OtlReadBudget) -> Option<Subtable> {
    let mut subtable = GposPairSubtable {
        first: None,
        second: None,
        first_values: Vec::new(),
        second_values: Vec::new(),
        format: 1,
        listed_zeros: Default::default(),
    };
    let mut header = FontReader::new(data).at(offset as usize + 2).ok()?;
    let cov_rel = header.u16().ok()?;

    // Built as a local and moved into `subtable` once the branch has
    // succeeded; every early `return None` drops it.
    let cov: Coverage = read_coverage(data, offset.wrapping_add(u32::from(cov_rel)), budget);
    let first_cd = ClassDef {
        maxclass: (cov.len() as i32 - 1) as GlyphClass,
        classes: (0..cov.len()).map(|j| j as GlyphClass).collect(),
        glyphs: cov,
    };

    let format1 = header.u16().ok()?;
    let format2 = header.u16().ok()?;
    let len1 = position_format_length(format1);
    let len2 = position_format_length(format2);
    let pair_set_count = header.u16().ok()?;
    if pair_set_count as usize != first_cd.glyphs.len() {
        return None;
    }
    if header.require_room(pair_set_count as usize, 2).is_err() {
        return None;
    }
    let mut pair_set_offsets = Vec::with_capacity(pair_set_count as usize);
    for _ in 0..pair_set_count {
        pair_set_offsets.push(offset.wrapping_add(u32::from(
            header.u16().expect("require_room above covered this read"),
        )));
    }

    // Check every PairSet's header and record array first.
    let stride = 2usize + len1 as usize + len2 as usize;
    let mut pair_counts = Vec::with_capacity(pair_set_count as usize);
    for &pso in &pair_set_offsets {
        let mut pr = FontReader::new(data).at(pso as usize).ok()?;
        let pc = pr.u16().ok()?;
        if pr.require_room(pc as usize, stride).is_err() {
            return None;
        }
        pair_counts.push(pc);
    }

    // Format 1 lists pairs individually, so a class def for the second
    // glyphs is synthesized: each distinct second glyph, in order of
    // first appearance, gets the next class from 1 (class 0 means "not
    // covered"). The set is built while first reading the pairs,
    // looked up while placing the values, and walked at the end to
    // fill `subtable.second`.
    let mut h: indexmap::IndexSet<i32> = indexmap::IndexSet::new();
    for (i, &pso) in pair_set_offsets.iter().enumerate() {
        for k in 0..pair_counts[i] {
            let second_offset = pso as usize + 2 + stride * k as usize;
            // Already validated by the pass above.
            let second = i32::from(pair_second_glyph(data, second_offset));
            h.insert(second);
        }
    }

    let n_second = h.len();
    let mut second_cd = ClassDef {
        maxclass: n_second as GlyphClass,
        classes: vec![0 as GlyphClass; n_second],
        glyphs: vec![GlyphHandle::default(); n_second],
    };
    let class2_count = second_cd.maxclass as usize + 1;

    let first_class_count = first_cd.maxclass as usize + 1;
    // Both counts are small, but the two grids below hold their product:
    // many PairSets can share one set of records, so a few KB of table can
    // ask for hundreds of MB. Same cap, for the same reason, as format 2's
    // class grid (see `MAX_TOTAL_GPOS_PAIR_CLASS_CELLS`).
    if first_class_count.saturating_mul(class2_count) > MAX_TOTAL_GPOS_PAIR_CLASS_CELLS {
        budget.reached(Limit::PairSubtableCells);
        return None;
    }
    if !budget.try_spend_pair_grid_cells(first_class_count * class2_count) {
        return None;
    }
    let mut first_values: Vec<Vec<PositionValue>> =
        vec![vec![position_zero(); class2_count]; first_class_count];
    let mut second_values: Vec<Vec<PositionValue>> =
        vec![vec![position_zero(); class2_count]; first_class_count];

    for (j3, &pso) in pair_set_offsets.iter().enumerate() {
        for k1 in 0..pair_counts[j3] {
            let second_offset = pso as usize + 2 + stride * k1 as usize;
            let second = i32::from(pair_second_glyph(data, second_offset));
            if let Some(idx) = h.get_index_of(&second) {
                let cid = idx + 1;
                first_values[j3][cid] = read_gpos_value(data, (second_offset + 2) as u32, format1);
                second_values[j3][cid] =
                    read_gpos_value(data, (second_offset + 2 + len1 as usize) as u32, format2);
                let cell = (j3 as GlyphClass, cid as GlyphClass);
                if required_position_format(first_values[j3][cid]) == 0
                    && required_position_format(second_values[j3][cid]) == 0
                {
                    subtable.listed_zeros.insert(cell);
                } else {
                    subtable.listed_zeros.remove(&cell);
                }
            }
        }
    }
    subtable.first_values = first_values;
    subtable.second_values = second_values;
    for (jj, &gid) in h.iter().enumerate() {
        second_cd.glyphs[jj] = handle_from_index(gid as GlyphId) as GlyphHandle;
        second_cd.classes[jj] = (jj + 1) as GlyphClass;
    }
    subtable.first = Some(Box::new(first_cd));
    subtable.second = Some(Box::new(second_cd));
    Some(Subtable::GposPair(subtable))
}
/// Format 2: a class-by-class matrix of values.
fn read_pair_format2(data: &[u8], offset: u32, budget: &mut OtlReadBudget) -> Option<Subtable> {
    let mut subtable = GposPairSubtable {
        first: None,
        second: None,
        first_values: Vec::new(),
        second_values: Vec::new(),
        format: 2,
        listed_zeros: Default::default(),
    };
    let mut header = FontReader::new(data).at(offset as usize + 2).ok()?;
    let cov_rel = header.u16().ok()?;
    let format1_0 = header.u16().ok()?;
    let format2_0 = header.u16().ok()?;
    let cd1_rel = header.u16().ok()?;
    let cd2_rel = header.u16().ok()?;
    let class1_count = header.u16().ok()?;
    let class2_count = header.u16().ok()?;
    let len1_0 = position_format_length(format1_0);
    let len2_0 = position_format_length(format2_0);

    let first_coverage: Coverage =
        read_coverage(data, offset.wrapping_add(u32::from(cov_rel)), budget);
    // `expand_class_def` takes the class def it is given and returns a
    // new one, of the covered glyphs only.
    let mut first_cd = expand_class_def(
        &first_coverage,
        read_class_def(data, offset.wrapping_add(u32::from(cd1_rel))),
    );
    let mut second_cd = read_class_def(data, offset.wrapping_add(u32::from(cd2_rel)));
    if class1_count == 0 || class2_count == 0 {
        return None;
    }
    // A first glyph of a class past `class1Count` never matches (as in
    // HarfBuzz), so it is left out of the subtable altogether.
    let kept: Vec<(GlyphHandle, GlyphClass)> = first_cd
        .glyphs
        .drain(..)
        .zip(first_cd.classes.drain(..))
        .filter(|&(_, class)| class < class1_count)
        .collect();
    first_cd.maxclass = 0;
    for (glyph, class) in kept {
        push_class_def(&mut first_cd, glyph, class);
    }
    // A second glyph past `class2Count` would make its pairs not match,
    // which the class matrix cannot say: such a subtable is not read.
    if second_cd.maxclass >= class2_count {
        return None;
    }
    // The counts may declare classes no glyph is in. They are kept, so
    // the matrix keeps its shape and the subtable is written back the same.
    first_cd.maxclass = class1_count - 1;
    second_cd.maxclass = class2_count - 1;
    subtable.first = Some(Box::new(first_cd));
    subtable.second = Some(Box::new(second_cd));

    let stride = len1_0 as usize + len2_0 as usize;
    let total_cells = (class1_count as usize).checked_mul(class2_count as usize)?;
    // See `MAX_TOTAL_GPOS_PAIR_CLASS_CELLS`'s own doc comment:
    // `require_room` below is a no-op when `stride` is 0 (both
    // value formats absent), so `total_cells` needs its own,
    // stride-independent cap -- checked first, before either
    // `Vec::with_capacity(class2_count)` allocation below ever
    // runs, so a rejected subtable never pays for one.
    if total_cells > MAX_TOTAL_GPOS_PAIR_CLASS_CELLS {
        budget.reached(Limit::PairSubtableCells);
        return None;
    }
    if !budget.try_spend_pair_grid_cells(total_cells) {
        return None;
    }
    let matrix = FontReader::new(data).at(offset as usize + 16).ok()?;
    if matrix.require_room(total_cells, stride).is_err() {
        return None;
    }

    // Format 2 fills every cell exhaustively and in order, so
    // (unlike Format 1's `cid`-indexed overwrite pass) no
    // pre-sized placeholder grid is needed -- each row is just
    // pushed as it's read.
    let mut first_values: Vec<Vec<PositionValue>> = Vec::with_capacity(class1_count as usize);
    let mut second_values: Vec<Vec<PositionValue>> = Vec::with_capacity(class1_count as usize);
    for j4 in 0..u32::from(class1_count) {
        let mut row1 = Vec::with_capacity(class2_count as usize);
        let mut row2 = Vec::with_capacity(class2_count as usize);
        for k2 in 0..u32::from(class2_count) {
            // Safe from u32 overflow: `require_room` above already
            // rejected any input where `total_cells * stride`
            // wouldn't fit in `data`'s own length (itself bounded
            // by a `u32`-sized table), so every offset computed
            // here is bounded by that.
            let cell_offset = offset
                .wrapping_add(16)
                .wrapping_add((j4 * u32::from(class2_count) + k2) * stride as u32);
            row1.push(read_gpos_value(data, cell_offset, format1_0));
            row2.push(read_gpos_value(
                data,
                cell_offset + u32::from(len1_0),
                format2_0,
            ));
        }
        first_values.push(row1);
        second_values.push(row2);
    }
    subtable.first_values = first_values;
    subtable.second_values = second_values;
    Some(Subtable::GposPair(subtable))
}
/// Writes the subtable as `{ first, second, matrix }`: the two class
/// definitions and one row per first class. A cell is `0` when there is no
/// adjustment, a bare number when only the first glyph's advance changes,
/// and `{ first, second }` value records otherwise.
pub fn otl_gpos_dump_pair(_subtable: &Subtable) -> BuiltValue {
    let Subtable::GposPair(subtable) = _subtable else {
        unreachable!()
    };
    let (first_cd, second_cd) = subtable.class_defs();
    let mut st = BuiltValue::new_object(4);
    if subtable.format != 0 {
        st.push_field(b"format", BuiltValue::Int(i64::from(subtable.format)));
    }
    st.push_field(b"first", dump_class_def(first_cd));
    st.push_field(b"second", dump_class_def(second_cd));
    let mut mat = BuiltValue::new_array((i32::from(first_cd.maxclass) + 1) as usize);
    for j in 0..=first_cd.maxclass {
        let mut row = BuiltValue::new_array((i32::from(second_cd.maxclass) + 1) as usize);
        for k in 0..=second_cd.maxclass {
            let f1: u8 = required_position_format(subtable.first_values[j as usize][k as usize]);
            let f2: u8 = required_position_format(subtable.second_values[j as usize][k as usize]);
            if f1 | f2 != 0 {
                if u16::from(f1) == FORMAT_DWIDTH && f2 == 0 {
                    row.push_item(BuiltValue::position(
                        subtable.first_values[j as usize][k as usize].d_width,
                    ));
                } else {
                    let mut pair = BuiltValue::new_object(2);
                    if f1 != 0 {
                        pair.push_field(
                            b"first",
                            gpos_dump_value(subtable.first_values[j as usize][k as usize]),
                        );
                    }
                    if f2 != 0 {
                        pair.push_field(
                            b"second",
                            gpos_dump_value(subtable.second_values[j as usize][k as usize]),
                        );
                    }
                    row.push_item(pair);
                }
            } else if subtable.listed_zeros.contains(&(j, k)) {
                // Listed, so matched, though nothing moves.
                row.push_item(BuiltValue::new_object(0));
            } else {
                row.push_item(BuiltValue::position(Pos::from(0)));
            }
        }
        mat.push_item(row.preserialize());
    }
    st.push_field(b"matrix", mat);
    st
}

/// Reads the object written by `otl_gpos_dump_pair`, or `None` if `matrix`
/// or either class definition is missing, or the matrix is too large.
///
/// The matrix is held in full, as when reading a binary subtable, so it is
/// limited the same way: `MAX_TOTAL_GPOS_PAIR_CLASS_CELLS` cells for one
/// subtable and `budget`'s pair cells for the whole table. Whatever the
/// binary reader accepts, and so whatever `otfccdump` writes, is accepted
/// back.
pub fn otl_gpos_parse_pair(
    _subtable: Option<&ParsedValue>,
    budget: &mut OtlReadBudget,
) -> Option<Subtable> {
    let sv = _subtable;
    let mat = sv.and_then(|v| v.get_typed(b"matrix", JsonType::Array))?;
    let first: Option<ClassDef> =
        parse_class_def(sv.and_then(|v| v.get_typed(b"first", JsonType::Object)));
    let second: Option<ClassDef> =
        parse_class_def(sv.and_then(|v| v.get_typed(b"second", JsonType::Object)));
    let first_cd = first.as_ref()?;
    let second_cd = second.as_ref()?;
    // The value matrix has one row per first class and one column per
    // second class, numbered from 0 to the largest class used. Both counts
    // are written as 16-bit fields, so the largest class must be below
    // 65,535. And since a class number costs nothing to write in JSON, the
    // matrix size is capped too: past `MAX_TOTAL_GPOS_PAIR_CLASS_CELLS` a
    // few bytes of input would allocate gigabytes, and many subtables of a
    // few bytes each would do the same without the table-wide budget.
    let class1_count = first_cd.maxclass as usize + 1;
    let class2_count = second_cd.maxclass as usize + 1;
    if class1_count > u16::MAX as usize
        || class2_count > u16::MAX as usize
        || class1_count * class2_count > MAX_TOTAL_GPOS_PAIR_CLASS_CELLS
    {
        tracing::warn!(
            "[OTFCC-fea] Pair positioning subtable with {} x {} classes is too large. This subtable is ignored.\n",
            class1_count,
            class2_count
        );
        return None;
    }
    if !budget.try_spend_pair_grid_cells(class1_count * class2_count) {
        return None;
    }
    let mut first_values: Vec<Vec<PositionValue>> =
        vec![vec![position_zero(); class2_count]; class1_count];
    let mut second_values: Vec<Vec<PositionValue>> =
        vec![vec![position_zero(); class2_count]; class1_count];
    let mut listed_zeros = std::collections::BTreeSet::new();
    if let Some(rows) = mat.as_array() {
        for (row_i, row) in rows.iter().enumerate().take(class1_count) {
            if let Some(items) = row.as_array() {
                for (col_i, item) in items.iter().enumerate().take(class2_count) {
                    if let Some(i) = item.as_int() {
                        first_values[row_i][col_i].d_width = i as Pos;
                    } else if let Some(d) = item.as_double() {
                        first_values[row_i][col_i].d_width = d as Pos;
                    } else if item.as_object().is_some() {
                        first_values[row_i][col_i] = gpos_parse_value(item.get(b"first"));
                        second_values[row_i][col_i] = gpos_parse_value(item.get(b"second"));
                        // An object that moves nothing, such as `{}`, is a
                        // pair listed with no adjustment.
                        if required_position_format(first_values[row_i][col_i]) == 0
                            && required_position_format(second_values[row_i][col_i]) == 0
                        {
                            listed_zeros.insert((row_i as GlyphClass, col_i as GlyphClass));
                        }
                    }
                }
            }
        }
    }
    // 1 or 2 as written; anything else leaves the choice to the build.
    let format = match sv.and_then(|v| v.get_typed(b"format", JsonType::Integer)) {
        Some(v) if matches!(v.as_int(), Some(1 | 2)) => v.as_int().unwrap_or(0) as u16,
        _ => 0,
    };
    Some(Subtable::GposPair(GposPairSubtable {
        first: first.map(Box::new),
        second: second.map(Box::new),
        first_values,
        second_values,
        format,
        listed_zeros,
    }))
}
fn cov_from_cd(cd: &ClassDef) -> Coverage {
    let mut cov: Coverage = Vec::new();
    for glyph in cd.glyphs.iter() {
        push_to_coverage(&mut cov, glyph.clone());
    }
    cov
}
/// Whether format 1 lists the pair of classes `c1` and `c2`: it moves
/// something, or it was listed with no adjustment.
fn is_listed(subtable: &GposPairSubtable, c1: GlyphClass, c2: GlyphClass) -> bool {
    required_position_format(subtable.first_values[c1 as usize][c2 as usize]) != 0
        || required_position_format(subtable.second_values[c1 as usize][c2 as usize]) != 0
        || subtable.listed_zeros.contains(&(c1, c2))
}
/// Lays out the subtable as PairPos format 1, one pair record per glyph pair
/// with a non-zero adjustment or listed with none.
pub fn build_gpos_pair_individual(_subtable: &Subtable) -> BkBlock {
    let Subtable::GposPair(subtable) = _subtable else {
        unreachable!()
    };
    let (first_cd, second_cd) = subtable.class_defs();
    let mut format1: u16 = 0;
    let mut format2: u16 = 0;
    let class1_count: GlyphClass = (i32::from(first_cd.maxclass) + 1) as GlyphClass;
    let class2_count: GlyphClass = (i32::from(second_cd.maxclass) + 1) as GlyphClass;
    for j in 0..class1_count {
        for k in 0..class2_count {
            format1 = (i32::from(format1)
                | i32::from(required_position_format(
                    subtable.first_values[j as usize][k as usize],
                ))) as u16;
            format2 = (i32::from(format2)
                | i32::from(required_position_format(
                    subtable.second_values[j as usize][k as usize],
                ))) as u16;
        }
    }
    let mut pair_counts: Vec<GlyphId> = vec![0 as GlyphId; first_cd.glyphs.len()];
    for (count, &c1) in pair_counts.iter_mut().zip(first_cd.classes.iter()) {
        for &c2 in second_cd.classes.iter() {
            if is_listed(subtable, c1, c2) {
                *count = count.wrapping_add(1);
            }
        }
    }
    let mut cov: Coverage = cov_from_cd(first_cd);
    shrink_coverage(&mut cov, true);
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 1),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&cov))),
        ),
        bk_int(BkCellType::B16, i32::from(format1) as u32),
        bk_int(BkCellType::B16, i32::from(format2) as u32),
        bk_int(BkCellType::B16, (first_cd.glyphs.len() as i32) as u32),
    ]);
    for cov_entry in cov.iter() {
        let mut current_pair_count: TableId = 0 as TableId;
        let mut c1_0: GlyphClass = 0 as GlyphClass;
        for ((glyph, &c1), &count) in first_cd
            .glyphs
            .iter()
            .zip(first_cd.classes.iter())
            .zip(pair_counts.iter())
        {
            if i32::from(glyph.index) == i32::from(cov_entry.index) {
                c1_0 = c1;
                current_pair_count = count as TableId;
            }
        }
        let mut pair_set: BkBlock = bk_new_block(vec![bk_int(
            BkCellType::B16,
            i32::from(current_pair_count) as u32,
        )]);
        // Exactly `current_pair_count` entries (the counting pass used the
        // same test), sorted stably.
        let mut pairs: Vec<IndividualGposPair> = Vec::with_capacity(current_pair_count as usize);
        for k_2 in 0..second_cd.glyphs.len() {
            let c2_0: GlyphClass = second_cd.classes[k_2];
            if is_listed(subtable, c1_0, c2_0) {
                pairs.push(IndividualGposPair {
                    gid: second_cd.glyphs[k_2].index,
                    fv: subtable.first_values[c1_0 as usize][c2_0 as usize],
                    sv: subtable.second_values[c1_0 as usize][c2_0 as usize],
                });
            }
        }
        pairs.sort_by_key(|p| p.gid);
        for pair in &pairs {
            bk_push(
                &mut pair_set,
                vec![
                    bk_int(BkCellType::B16, i32::from(pair.gid) as u32),
                    bk_ptr(BkCellType::Embed, Some(bk_gpos_value(pair.fv, format1))),
                    bk_ptr(BkCellType::Embed, Some(bk_gpos_value(pair.sv, format2))),
                ],
            );
        }
        bk_push(&mut root, vec![bk_ptr(BkCellType::P16, Some(pair_set))]);
    }
    root
}
/// Lays out the subtable as PairPos format 2, the class matrix as is.
pub fn build_gpos_pair_classes(_subtable: &Subtable) -> BkBlock {
    let Subtable::GposPair(subtable) = _subtable else {
        unreachable!()
    };
    let (first_cd, second_cd) = subtable.class_defs();
    let mut format1: u16 = 0;
    let mut format2: u16 = 0;
    let class1_count: GlyphClass = (i32::from(first_cd.maxclass) + 1) as GlyphClass;
    let class2_count: GlyphClass = (i32::from(second_cd.maxclass) + 1) as GlyphClass;
    for j in 0..class1_count {
        for k in 0..class2_count {
            format1 = (i32::from(format1)
                | i32::from(required_position_format(
                    subtable.first_values[j as usize][k as usize],
                ))) as u16;
            format2 = (i32::from(format2)
                | i32::from(required_position_format(
                    subtable.second_values[j as usize][k as usize],
                ))) as u16;
        }
    }
    let cov: Coverage = cov_from_cd(first_cd);
    let mut root: BkBlock = bk_new_block(vec![
        bk_int(BkCellType::B16, 2),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_coverage(&cov))),
        ),
        bk_int(BkCellType::B16, i32::from(format1) as u32),
        bk_int(BkCellType::B16, i32::from(format2) as u32),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_class_def(first_cd))),
        ),
        bk_ptr(
            BkCellType::P16,
            bk_new_block_from_buffer(Some(build_class_def(second_cd))),
        ),
        bk_int(BkCellType::B16, i32::from(class1_count) as u32),
        bk_int(BkCellType::B16, i32::from(class2_count) as u32),
    ]);
    for row_i in 0..class1_count {
        for col_i in 0..class2_count {
            bk_push(
                &mut root,
                vec![
                    bk_ptr(
                        BkCellType::Embed,
                        Some(bk_gpos_value(
                            subtable.first_values[row_i as usize][col_i as usize],
                            format1,
                        )),
                    ),
                    bk_ptr(
                        BkCellType::Embed,
                        Some(bk_gpos_value(
                            subtable.second_values[row_i as usize][col_i as usize],
                            format2,
                        )),
                    ),
                ],
            );
        }
    }
    root
}
/// Writes the subtable in its own format, or, when that is not known,
/// whichever of PairPos format 1 and format 2 comes out smaller (format 1 on
/// a tie).
pub fn build_gpos_pair(_subtable: &Subtable, mut _heuristics: BuildHeuristics) -> Buffer {
    let Subtable::GposPair(subtable) = _subtable else {
        unreachable!()
    };
    if subtable.format == 1 || subtable.format == 2 {
        let block = if subtable.format == 1 {
            build_gpos_pair_individual(_subtable)
        } else {
            build_gpos_pair_classes(_subtable)
        };
        let mut graph: BkGraph = bk_new_graph_from_root_block(block);
        bk_minimize_graph(&mut graph);
        bk_untangle_graph(&mut graph);
        return bk_build_graph(&graph);
    }
    let format1: BkBlock = build_gpos_pair_individual(_subtable);
    let format2: BkBlock = build_gpos_pair_classes(_subtable);
    let mut g1: BkGraph = bk_new_graph_from_root_block(format1);
    let mut g2: BkGraph = bk_new_graph_from_root_block(format2);
    bk_minimize_graph(&mut g1);
    bk_minimize_graph(&mut g2);
    if bk_estimate_size_of_graph(&g1) > bk_estimate_size_of_graph(&g2) {
        bk_untangle_graph(&mut g2);
        bk_build_graph(&g2)
    } else {
        bk_untangle_graph(&mut g1);
        bk_build_graph(&g1)
    }
}

#[cfg(test)]
mod otl_read_gpos_pair_tests {
    use super::*;

    #[test]
    fn format1_reads_one_pair_and_synthesizes_the_second_class_def() {
        let mut data = [0u8; 24];
        data[0..2].copy_from_slice(&1u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&12u16.to_be_bytes()); // coverageOffset -> 12
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // valueFormat1: dx only
        data[6..8].copy_from_slice(&0u16.to_be_bytes()); // valueFormat2: none
        data[8..10].copy_from_slice(&1u16.to_be_bytes()); // pairSetCount
        data[10..12].copy_from_slice(&18u16.to_be_bytes()); // pairSetOffsets[0] -> 18
        // Coverage format 1 at byte 12: one glyph, id 10.
        data[12..14].copy_from_slice(&1u16.to_be_bytes());
        data[14..16].copy_from_slice(&1u16.to_be_bytes());
        data[16..18].copy_from_slice(&10u16.to_be_bytes());
        // PairSet at byte 18: one pair, second=20, value1.dx=50.
        data[18..20].copy_from_slice(&1u16.to_be_bytes());
        data[20..22].copy_from_slice(&20i16.to_be_bytes());
        data[22..24].copy_from_slice(&50i16.to_be_bytes());
        let result = otl_read_gpos_pair(&data, 0, 0, &mut OtlReadBudget::new());
        let Some(Subtable::GposPair(ref subtable)) = result else {
            unreachable!()
        };
        let first = subtable.first.as_ref().unwrap();
        let second = subtable.second.as_ref().unwrap();
        assert_eq!(
            first.glyphs.iter().map(|h| h.index).collect::<Vec<_>>(),
            vec![10]
        );
        assert_eq!(
            second.glyphs.iter().map(|h| h.index).collect::<Vec<_>>(),
            vec![20]
        );
        assert_eq!(second.classes, vec![1]);
        assert_eq!(subtable.first_values[0][1].dx, 50.0);
    }

    /// A format 1 subtable covering `n` glyphs whose `n` PairSets are all the
    /// same set of `n` pairs (no value records): `n` distinct second glyphs,
    /// so the synthesized grid has `n * (n + 1)` cells from about `6 * n` bytes.
    fn format1_shared_pairset_table(n: u16) -> Vec<u8> {
        let n_us = n as usize;
        let cov_at = 10 + 2 * n_us;
        let pairset_at = cov_at + 4 + 2 * n_us;
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes()); // format
        data.extend_from_slice(&(cov_at as u16).to_be_bytes()); // coverageOffset
        data.extend_from_slice(&0u16.to_be_bytes()); // valueFormat1: none
        data.extend_from_slice(&0u16.to_be_bytes()); // valueFormat2: none
        data.extend_from_slice(&n.to_be_bytes()); // pairSetCount
        for _ in 0..n {
            data.extend_from_slice(&(pairset_at as u16).to_be_bytes());
        }
        data.extend_from_slice(&1u16.to_be_bytes()); // coverage format 1
        data.extend_from_slice(&n.to_be_bytes());
        for g in 0..n {
            data.extend_from_slice(&g.to_be_bytes());
        }
        data.extend_from_slice(&n.to_be_bytes()); // pairValueCount
        for g in 0..n {
            data.extend_from_slice(&(1000 + g).to_be_bytes()); // secondGlyph
        }
        data
    }

    #[test]
    fn format1_grid_within_the_cell_cap_is_read() {
        // 50 * 51 = 2,550 cells, well under the 2,000,000 cap.
        let data = format1_shared_pairset_table(50);
        let result = otl_read_gpos_pair(&data, 0, 0, &mut OtlReadBudget::new());
        let Some(Subtable::GposPair(ref subtable)) = result else {
            panic!("a grid within the cap should be read")
        };
        assert_eq!(subtable.first_values.len(), 50);
        assert_eq!(subtable.first_values[0].len(), 51);
    }

    #[test]
    fn format1_grids_across_subtables_draw_on_one_table_wide_allowance() {
        // Each subtable has 2,550 cells, but the table's allowance (5,500)
        // only covers two of them.
        let data = format1_shared_pairset_table(50);
        let mut budget = OtlReadBudget {
            pair_grid_cells: 5_500,
            ..OtlReadBudget::new()
        };
        assert!(otl_read_gpos_pair(&data, 0, 0, &mut budget).is_some());
        assert!(otl_read_gpos_pair(&data, 0, 0, &mut budget).is_some());
        assert!(otl_read_gpos_pair(&data, 0, 0, &mut budget).is_none());
    }

    // Reading 2.25M pairs one by one is too slow under Miri's interpreter.
    #[cfg_attr(miri, ignore = "2.25M-pair table is too slow under Miri")]
    #[test]
    fn format1_grid_over_the_cell_cap_is_rejected_not_allocated() {
        // 1500 * 1501 = 2,251,500 cells from a ~9 KB table: a fuzzer found
        // the same shape asking for ~800 MB (two grids of ~12.5M cells).
        let data = format1_shared_pairset_table(1500);
        assert!(data.len() < 10_000);
        let result = otl_read_gpos_pair(&data, 0, 0, &mut OtlReadBudget::new());
        assert!(result.is_none());
    }

    #[test]
    fn format1_table_too_short_for_the_header_is_rejected_instead_of_reading_oob() {
        // `coverageOffset` and every header field after it must fit; a
        // table this short claiming format 1 must not be read past its end.
        let data = [0u8, 1]; // format = 1, nothing else
        let result = otl_read_gpos_pair(&data, 0, 0, &mut OtlReadBudget::new());
        assert!(result.is_none());
    }

    #[test]
    fn format2_reads_the_matrix_cell_for_each_class_pair() {
        let mut data = [0u8; 46];
        data[0..2].copy_from_slice(&2u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&24u16.to_be_bytes()); // coverageOffset -> 24
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // valueFormat1: dx only
        data[6..8].copy_from_slice(&0u16.to_be_bytes()); // valueFormat2: none
        data[8..10].copy_from_slice(&30u16.to_be_bytes()); // classDef1Offset -> 30
        data[10..12].copy_from_slice(&38u16.to_be_bytes()); // classDef2Offset -> 38
        data[12..14].copy_from_slice(&2u16.to_be_bytes()); // class1Count
        data[14..16].copy_from_slice(&2u16.to_be_bytes()); // class2Count
        // Matrix at byte 16: 2x2 cells, 2 bytes (dx) each; cell[1][1].dx=77.
        data[22..24].copy_from_slice(&77i16.to_be_bytes());
        // Coverage format 1 at byte 24: one glyph, id 10.
        data[24..26].copy_from_slice(&1u16.to_be_bytes());
        data[26..28].copy_from_slice(&1u16.to_be_bytes());
        data[28..30].copy_from_slice(&10u16.to_be_bytes());
        // ClassDef format 1 at byte 30: glyph 10 -> class 1.
        data[30..32].copy_from_slice(&1u16.to_be_bytes());
        data[32..34].copy_from_slice(&10u16.to_be_bytes());
        data[34..36].copy_from_slice(&1u16.to_be_bytes());
        data[36..38].copy_from_slice(&1u16.to_be_bytes());
        // ClassDef format 1 at byte 38: glyph 20 -> class 1.
        data[38..40].copy_from_slice(&1u16.to_be_bytes());
        data[40..42].copy_from_slice(&20u16.to_be_bytes());
        data[42..44].copy_from_slice(&1u16.to_be_bytes());
        data[44..46].copy_from_slice(&1u16.to_be_bytes());
        let result = otl_read_gpos_pair(&data, 0, 0, &mut OtlReadBudget::new());
        let Some(Subtable::GposPair(ref subtable)) = result else {
            unreachable!()
        };
        assert_eq!(subtable.first_values[1][1].dx, 77.0);
        assert_eq!(subtable.first_values[0][0].dx, 0.0);
    }

    /// A format 2 subtable with a dx-only matrix of `class1` x `class2`
    /// cells, all 5. The coverage lists `covered`; ClassDef1 maps each of
    /// `first` to its class, and ClassDef2 maps glyph 50 to class 1.
    fn format2(class1: u16, class2: u16, covered: &[u16], first: &[(u16, u16)]) -> Vec<u8> {
        let put = |d: &mut Vec<u8>, v: u16| d.extend_from_slice(&v.to_be_bytes());
        let matrix = 2 * usize::from(class1) * usize::from(class2);
        let coverage = 16 + matrix;
        let cd1 = coverage + 4 + 2 * covered.len();
        let cd2 = cd1 + 4 + 6 * first.len();
        let mut d = Vec::new();
        for v in [
            2,
            coverage as u16,
            1,
            0,
            cd1 as u16,
            cd2 as u16,
            class1,
            class2,
        ] {
            put(&mut d, v);
        }
        for _ in 0..matrix / 2 {
            put(&mut d, 5);
        }
        put(&mut d, 1);
        put(&mut d, covered.len() as u16);
        for &g in covered {
            put(&mut d, g);
        }
        // ClassDef format 2, one range per glyph.
        put(&mut d, 2);
        put(&mut d, first.len() as u16);
        for &(g, c) in first {
            for v in [g, g, c] {
                put(&mut d, v);
            }
        }
        d.truncate(cd2);
        for v in [2, 1, 50, 50, 1] {
            put(&mut d, v);
        }
        d
    }

    fn first_glyphs(data: &[u8]) -> Option<Vec<(u16, u16)>> {
        let Subtable::GposPair(st) = otl_read_gpos_pair(data, 0, 0, &mut OtlReadBudget::new())?
        else {
            unreachable!()
        };
        let cd = st.first.as_ref().unwrap();
        Some(
            cd.glyphs
                .iter()
                .map(|g| g.index)
                .zip(cd.classes.iter().copied())
                .collect(),
        )
    }

    #[test]
    fn format2_first_glyphs_outside_the_coverage_are_left_out() {
        // Glyph 11 is in ClassDef1 (as class 0, as some fonts list it) but
        // not covered: it never matches, and used to be covered once built.
        let data = format2(2, 2, &[10], &[(10, 1), (11, 0)]);
        assert_eq!(first_glyphs(&data), Some(vec![(10, 1)]));
    }

    #[test]
    fn format2_declared_classes_no_glyph_uses_are_kept() {
        // Three first classes and three second classes declared; only
        // classes 0 and 1 are used. Such a subtable used to be dropped.
        let data = format2(3, 3, &[10], &[(10, 1)]);
        let Some(Subtable::GposPair(st)) =
            otl_read_gpos_pair(&data, 0, 0, &mut OtlReadBudget::new())
        else {
            panic!("not read")
        };
        assert_eq!(st.first.as_ref().unwrap().maxclass, 2);
        assert_eq!(st.second.as_ref().unwrap().maxclass, 2);
        assert_eq!(st.first_values[1][1].dx, 5.0);
    }

    #[test]
    fn format2_first_glyphs_of_a_class_past_the_count_are_left_out() {
        // Glyph 11 is covered but in class 2 of a 2-class subtable: it
        // never matches, so it is not kept as a class 0 glyph either.
        let data = format2(2, 2, &[10, 11], &[(10, 1), (11, 2)]);
        assert_eq!(first_glyphs(&data), Some(vec![(10, 1)]));
    }

    #[test]
    fn format2_max_class_counts_are_rejected_not_read_oob() {
        // `class1_count * class2_count * (len1+len2)` can reach ~68.7
        // billion (`class1_count`/`class2_count` are independently
        // unbounded u16 fields), far past `i32::MAX`. A single-range
        // ClassDef format 2 entry cheaply claims a `maxclass` of
        // `u16::MAX - 1` without needing anywhere near that many real
        // entries, so this reaches the guard with a tiny buffer.
        let mut data = [0u8; 42];
        data[0..2].copy_from_slice(&2u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&16u16.to_be_bytes()); // coverageOffset -> 16
        data[4..6].copy_from_slice(&1u16.to_be_bytes()); // valueFormat1
        data[6..8].copy_from_slice(&1u16.to_be_bytes()); // valueFormat2
        data[8..10].copy_from_slice(&22u16.to_be_bytes()); // classDef1Offset -> 22
        data[10..12].copy_from_slice(&32u16.to_be_bytes()); // classDef2Offset -> 32
        data[12..14].copy_from_slice(&u16::MAX.to_be_bytes()); // class1Count
        data[14..16].copy_from_slice(&u16::MAX.to_be_bytes()); // class2Count
        // Coverage format 1 at byte 16: one glyph, id 10.
        data[16..18].copy_from_slice(&1u16.to_be_bytes());
        data[18..20].copy_from_slice(&1u16.to_be_bytes());
        data[20..22].copy_from_slice(&10u16.to_be_bytes());
        // ClassDef format 2 at byte 22: one range, glyph 10, class u16::MAX-1.
        data[22..24].copy_from_slice(&2u16.to_be_bytes());
        data[24..26].copy_from_slice(&1u16.to_be_bytes());
        data[26..28].copy_from_slice(&10u16.to_be_bytes());
        data[28..30].copy_from_slice(&10u16.to_be_bytes());
        data[30..32].copy_from_slice(&(u16::MAX - 1).to_be_bytes());
        // ClassDef format 2 at byte 32: one range, glyph 20, class u16::MAX-1.
        data[32..34].copy_from_slice(&2u16.to_be_bytes());
        data[34..36].copy_from_slice(&1u16.to_be_bytes());
        data[36..38].copy_from_slice(&20u16.to_be_bytes());
        data[38..40].copy_from_slice(&20u16.to_be_bytes());
        data[40..42].copy_from_slice(&(u16::MAX - 1).to_be_bytes());
        let result = otl_read_gpos_pair(&data, 0, 0, &mut OtlReadBudget::new());
        assert!(result.is_none());
    }

    #[test]
    fn format2_zero_stride_class_counts_are_capped_not_allocated_unbounded() {
        // See `MAX_TOTAL_GPOS_PAIR_CLASS_CELLS`'s own doc comment: with
        // both value formats 0, `stride` is 0 and the sibling test above
        // (`format2_max_class_counts_are_rejected_not_read_oob`, whose
        // byte layout this one otherwise mirrors) no longer reaches its
        // guard -- `require_room(total_cells, 0)` always succeeds, no
        // matter how huge `total_cells` is. Without this file's own
        // stride-independent cap, `class1Count = class2Count = u16::MAX`
        // here would try to allocate two ~4.29-billion-cell `Vec<Vec<
        // PositionValue>>` grids from this same 42-byte buffer.
        let mut data = [0u8; 42];
        data[0..2].copy_from_slice(&2u16.to_be_bytes()); // format
        data[2..4].copy_from_slice(&16u16.to_be_bytes()); // coverageOffset -> 16
        data[4..6].copy_from_slice(&0u16.to_be_bytes()); // valueFormat1: NONE
        data[6..8].copy_from_slice(&0u16.to_be_bytes()); // valueFormat2: NONE
        data[8..10].copy_from_slice(&22u16.to_be_bytes()); // classDef1Offset -> 22
        data[10..12].copy_from_slice(&32u16.to_be_bytes()); // classDef2Offset -> 32
        data[12..14].copy_from_slice(&u16::MAX.to_be_bytes()); // class1Count
        data[14..16].copy_from_slice(&u16::MAX.to_be_bytes()); // class2Count
        // Coverage format 1 at byte 16: one glyph, id 10.
        data[16..18].copy_from_slice(&1u16.to_be_bytes());
        data[18..20].copy_from_slice(&1u16.to_be_bytes());
        data[20..22].copy_from_slice(&10u16.to_be_bytes());
        // ClassDef format 2 at byte 22: one range, glyph 10, class u16::MAX-1.
        data[22..24].copy_from_slice(&2u16.to_be_bytes());
        data[24..26].copy_from_slice(&1u16.to_be_bytes());
        data[26..28].copy_from_slice(&10u16.to_be_bytes());
        data[28..30].copy_from_slice(&10u16.to_be_bytes());
        data[30..32].copy_from_slice(&(u16::MAX - 1).to_be_bytes());
        // ClassDef format 2 at byte 32: one range, glyph 20, class u16::MAX-1.
        data[32..34].copy_from_slice(&2u16.to_be_bytes());
        data[34..36].copy_from_slice(&1u16.to_be_bytes());
        data[36..38].copy_from_slice(&20u16.to_be_bytes());
        data[38..40].copy_from_slice(&20u16.to_be_bytes());
        data[40..42].copy_from_slice(&(u16::MAX - 1).to_be_bytes());
        let result = otl_read_gpos_pair(&data, 0, 0, &mut OtlReadBudget::new());
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod otl_gpos_parse_pair_class_limit_tests {
    use super::*;
    use otfcc_json::parse_json;

    fn parse(json: &str) -> Option<Subtable> {
        otl_gpos_parse_pair(
            Some(&parse_json(json.as_bytes()).unwrap()),
            &mut OtlReadBudget::new(),
        )
    }

    #[test]
    fn a_small_class_matrix_is_parsed() {
        let st =
            parse(r#"{"first": {"a": 1}, "second": {"b": 2}, "matrix": [[0, 0, 0], [0, 0, 5]]}"#);
        let Some(Subtable::GposPair(st)) = &st else {
            panic!("not parsed")
        };
        assert_eq!(st.first_values.len(), 2);
        assert_eq!(st.first_values[1].len(), 3);
        assert_eq!(st.first_values[1][2].d_width, 5.0);
    }

    #[test]
    fn a_negative_class_is_ignored_instead_of_wrapping_to_65535() {
        // -1 used to become class 65535, wrap the class count to 0 and
        // panic when the subtable was built.
        let st = parse(
            r#"{"first": {"a": -1, "c": 1}, "second": {"b": 1}, "matrix": [[0, 0], [0, 7]]}"#,
        );
        let Some(Subtable::GposPair(st)) = &st else {
            panic!("not parsed")
        };
        assert_eq!(st.first.as_ref().unwrap().glyphs.len(), 1);
        assert_eq!(st.first_values.len(), 2);
    }

    #[test]
    fn class_65535_is_rejected() {
        assert!(parse(r#"{"first": {"a": 65535}, "second": {"b": 1}, "matrix": []}"#).is_none());
    }

    #[test]
    #[cfg_attr(miri, ignore = "allocates a million-entry matrix")]
    fn an_oversized_class_matrix_is_rejected_instead_of_allocated() {
        // 65,534 x 65,534 classes would be over four billion matrix entries.
        assert!(
            parse(r#"{"first": {"a": 65534}, "second": {"b": 65534}, "matrix": []}"#).is_none()
        );
        // Just under the limit is fine.
        assert!(parse(r#"{"first": {"a": 1413}, "second": {"b": 1413}, "matrix": []}"#).is_some());
    }

    #[test]
    #[cfg_attr(miri, ignore = "allocates a million-entry matrix")]
    fn a_matrix_the_binary_reader_accepts_is_accepted_back() {
        // EB Garamond has a format 1 subtable that reads as 1,006 x 1,133
        // classes; the JSON parser used to stop at 2^20 entries, so its
        // kerning was lost when the dump was built.
        assert!(parse(r#"{"first": {"a": 1005}, "second": {"b": 1132}, "matrix": []}"#).is_some());
    }

    #[test]
    fn the_table_wide_budget_is_shared_by_every_subtable() {
        let json = parse_json(br#"{"first": {"a": 9}, "second": {"b": 9}, "matrix": []}"#).unwrap();
        let mut budget = OtlReadBudget {
            pair_grid_cells: 150,
            ..OtlReadBudget::new()
        };
        assert!(otl_gpos_parse_pair(Some(&json), &mut budget).is_some());
        assert!(otl_gpos_parse_pair(Some(&json), &mut budget).is_none());
        assert_eq!(budget.pair_grid_cells, 50);
    }
}

#[cfg(test)]
mod pair_format_tests {
    use super::*;
    use otfcc_json::{
        JSON_SERIALIZE_MODE_SINGLE_LINE, JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON,
        JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA, JSON_SERIALIZE_OPT_PACK_BRACKETS,
        JsonSerializeOpts, json_serialize_ex, parse_json,
    };

    fn put(d: &mut Vec<u8>, v: u16) {
        d.extend_from_slice(&v.to_be_bytes());
    }

    /// Format 1, dx values: glyph 10 with glyph 20 (dx 0, listed) and with
    /// glyph 21 (dx 7).
    fn format1_with_a_listed_zero() -> Vec<u8> {
        let mut d = Vec::new();
        // format, coverage @12, valueFormat1 dx, valueFormat2 none, 1 set @18
        for v in [1, 12, 1, 0, 1, 18] {
            put(&mut d, v);
        }
        for v in [1, 1, 10] {
            put(&mut d, v); // coverage: glyph 10
        }
        for v in [2, 20, 0, 21, 7] {
            put(&mut d, v); // PairSet: two records
        }
        d
    }

    /// Format 2, dx values: glyph 10 (class 1) with glyph 20 (class 1) is
    /// dx 7; every other cell is 0, so format 1 would be smaller.
    fn format2_sparse() -> Vec<u8> {
        let mut d = Vec::new();
        // format, coverage @24, dx, none, classDef1 @30, classDef2 @38, 2x2
        for v in [2, 24, 1, 0, 30, 38, 2, 2] {
            put(&mut d, v);
        }
        for v in [0, 0, 0, 7] {
            put(&mut d, v); // matrix
        }
        for v in [1, 1, 10] {
            put(&mut d, v); // coverage
        }
        for v in [1, 10, 1, 1] {
            put(&mut d, v); // ClassDef format 1: glyph 10 -> 1
        }
        for v in [1, 20, 1, 1] {
            put(&mut d, v); // ClassDef format 1: glyph 20 -> 1
        }
        d
    }

    /// The subtable at the start of `data`, its glyphs named `g<id>`.
    fn read(data: &[u8]) -> GposPairSubtable {
        let Some(Subtable::GposPair(mut st)) =
            otl_read_gpos_pair(data, 0, 0, &mut OtlReadBudget::new())
        else {
            panic!("not read")
        };
        let (first, second) = st.class_defs_mut();
        for glyph in first.glyphs.iter_mut().chain(second.glyphs.iter_mut()) {
            glyph.name = format!("g{}", glyph.index).into_bytes();
        }
        st
    }

    fn json(st: &GposPairSubtable) -> String {
        let opts = JsonSerializeOpts {
            mode: JSON_SERIALIZE_MODE_SINGLE_LINE,
            opts: JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON
                | JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA
                | JSON_SERIALIZE_OPT_PACK_BRACKETS,
            indent_size: 0,
        };
        let dumped = otl_gpos_dump_pair(&Subtable::GposPair(st.clone()));
        String::from_utf8(json_serialize_ex(&dumped, opts)).unwrap()
    }

    fn parse(text: &str) -> GposPairSubtable {
        let value = parse_json(text.as_bytes()).unwrap();
        let Some(Subtable::GposPair(st)) =
            otl_gpos_parse_pair(Some(&value), &mut OtlReadBudget::new())
        else {
            panic!("not parsed")
        };
        st
    }

    #[test]
    fn a_format1_pair_listed_with_no_adjustment_is_kept() {
        let st = read(&format1_with_a_listed_zero());
        assert_eq!(st.format, 1);
        // Glyph 10 is first class 0; glyph 20 is second class 1.
        assert_eq!(
            st.listed_zeros.iter().copied().collect::<Vec<_>>(),
            [(0, 1)]
        );
        let text = json(&st);
        assert!(text.starts_with(r#"{"format":1,"#), "{text}");
        assert!(text.contains(r#"[0,{},{"first":{"dx":7}}]"#), "{text}");
        let back = parse(&text);
        assert_eq!(back.format, 1);
        assert_eq!(back.listed_zeros, st.listed_zeros);
    }

    #[test]
    fn a_format2_subtable_is_dumped_as_format2() {
        let st = read(&format2_sparse());
        assert_eq!(st.format, 2);
        assert!(st.listed_zeros.is_empty());
        let text = json(&st);
        assert!(text.starts_with(r#"{"format":2,"#), "{text}");
        assert_eq!(parse(&text).format, 2);
    }

    #[test]
    fn json_without_a_format_leaves_the_choice_to_the_build() {
        let st = parse(r#"{"first":{"a":1},"second":{"b":1},"matrix":[[0,0],[0,5]]}"#);
        assert_eq!(st.format, 0);
        assert!(st.listed_zeros.is_empty());
    }
}
