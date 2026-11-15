use super::*;

use crate::support::handle::{FdHandle, handle_from_index};

use crate::font::sfnt::Packet;
use crate::libcff::CffDictOperator;
use crate::libcff::charset::CffCharset;
use crate::libcff::fdselect::CffFdSelect;
use crate::libcff::index::{CffIndex, CffIndexCountType};
use crate::libcff::value::CffValue;
use crate::libcff::{
    CffFile, CffStack, OP_BLUE_FUZZ, OP_BLUE_SCALE, OP_BLUE_SHIFT, OP_BLUE_VALUES,
    TYPE2_TRANSIENT_ARRAY, OP_COPYRIGHT,
    OP_DEFAULT_WIDTH_X, OP_EXPANSION_FACTOR, OP_FAMILY_BLUES, OP_FAMILY_NAME,
    OP_FAMILY_OTHER_BLUES, OP_FONT_BBOX, OP_FONT_MATRIX, OP_FONT_NAME,
    OP_FORCE_BOLD, OP_FULL_NAME, OP_INITIAL_RANDOM_SEED, OP_IS_FIXED_PITCH, OP_ITALIC_ANGLE,
    OP_LANGUAGE_GROUP, OP_NOMINAL_WIDTH_X, OP_NOTICE, OP_OTHER_BLUES, OP_PRIVATE, OP_ROS,
    OP_STD_HW, OP_STD_VW, OP_STEM_SNAP_H, OP_STEM_SNAP_V, OP_STROKE_WIDTH,
    OP_UNDERLINE_POSITION, OP_UNDERLINE_THICKNESS, OP_VERSION, OP_WEIGHT,
};
use crate::support::primitives::{Arity, CffSid, GlyphId, Pos, Scale, ShapeId, TableId};
use crate::table::glyf::{
    Contour, GlyfTable, Glyph, MaskList, Point, PostscriptHintMask, PostscriptStemDef, StemDefList,
};
use crate::table::head::HeadTable;

use crate::vf::vq::VQ;

use crate::libcff::dict::parse_to_callback;
use crate::libcff::parser::{open_stream, parse_outline, parse_subr};
use crate::libcff::string::get_cff_sid;
use crate::libcff::value::cffnum;
use crate::support::primitives::{from_fixed, to_fixed};
use crate::table::glyf::{StemMask, new_glyf_glyph, table_glyf_create_n};
use crate::vf::vq::{
    vq_compare, vq_create_still, vq_inplace_plus, vq_neutral, vq_point_linear_tfm,
    vq_scale,
};

fn callback_extract_private(op: CffDictOperator, top: u8, stack: &[CffValue], context: &mut CffFdExtractContext) {
    let meta: &mut CffTable = if context.fd_array_index >= 0
        && (context.fd_array_index as usize) < context.meta.fd_array.len()
    {
        context.meta.fd_array[context.fd_array_index as usize].as_mut()
    } else {
        &mut *context.meta
    };
    let pd: &mut CffPrivateDict = meta.private_dict.as_deref_mut().unwrap();
    // Every operand, in order (the delta-encoded arrays).
    let all = || (0..top as Arity).map(|j| cffnum(stack[j as usize])).collect::<Vec<f64>>();
    // The operand `n` places below the top; only read once `top >= n` is known.
    let last = |n: usize| cffnum(stack[top as usize - n]);
    match op {
        OP_BLUE_VALUES => pd.blue_values = all(),
        OP_OTHER_BLUES => pd.other_blues = all(),
        OP_FAMILY_BLUES => pd.family_blues = all(),
        OP_FAMILY_OTHER_BLUES => pd.family_other_blues = all(),
        OP_STEM_SNAP_H => pd.stem_snap_h = all(),
        OP_STEM_SNAP_V => pd.stem_snap_v = all(),
        OP_BLUE_SCALE if top != 0 => pd.blue_scale = last(1),
        OP_BLUE_SHIFT if top != 0 => pd.blue_shift = last(1),
        OP_BLUE_FUZZ if top != 0 => pd.blue_fuzz = last(1),
        OP_STD_HW if top != 0 => pd.std_hw = last(1),
        OP_STD_VW if top != 0 => pd.std_vw = last(1),
        OP_FORCE_BOLD if top != 0 => pd.force_bold = last(1) != 0.,
        OP_LANGUAGE_GROUP if top != 0 => pd.language_group = last(1) as u32,
        OP_EXPANSION_FACTOR if top != 0 => pd.expansion_factor = last(1),
        OP_INITIAL_RANDOM_SEED if top != 0 => pd.initial_random_seed = last(1),
        OP_DEFAULT_WIDTH_X if top != 0 => pd.default_width_x = last(1),
        OP_NOMINAL_WIDTH_X if top != 0 => pd.nominal_width_x = last(1),
        _ => {}
    };
}
fn callback_extract_fd(op: CffDictOperator, top: u8, stack: &[CffValue], context: &mut CffFdExtractContext) {
    let file: &CffFile = context.cff_file;
    let meta: &mut CffTable = if context.fd_array_index >= 0
        && (context.fd_array_index as usize) < context.meta.fd_array.len()
    {
        context.meta.fd_array[context.fd_array_index as usize].as_mut()
    } else {
        &mut *context.meta
    };
    // The operand `n` places below the top; only read once `top >= n` is known.
    let last = |n: usize| cffnum(stack[top as usize - n]);
    // That operand as a string id, resolved against the String INDEX.
    let sid = |n: usize| get_cff_sid(last(n) as u16, &file.string).unwrap_or_default();
    match op {
        OP_VERSION if top != 0 => meta.version = sid(1),
        OP_NOTICE if top != 0 => meta.notice = sid(1),
        OP_COPYRIGHT if top != 0 => meta.copyright = sid(1),
        OP_FONT_NAME if top != 0 => meta.font_name = sid(1),
        OP_FULL_NAME if top != 0 => meta.full_name = sid(1),
        OP_FAMILY_NAME if top != 0 => meta.family_name = sid(1),
        OP_WEIGHT if top != 0 => meta.weight = sid(1),
        OP_FONT_BBOX if top >= 4 => {
            meta.font_b_box_left = last(4);
            meta.font_b_box_bottom = last(3);
            meta.font_b_box_right = last(2);
            meta.font_b_box_top = last(1);
        }
        OP_FONT_MATRIX if top >= 6 => {
            meta.font_matrix = Some(Box::new(CffFontMatrix {
                a: last(6) as Scale,
                b: last(5) as Scale,
                c: last(4) as Scale,
                d: last(3) as Scale,
                x: vq_create_still(last(2) as Pos),
                y: vq_create_still(last(1) as Pos),
            }));
        }
        OP_IS_FIXED_PITCH if top != 0 => meta.is_fixed_pitch = last(1) != 0.,
        OP_ITALIC_ANGLE if top != 0 => meta.italic_angle = last(1),
        OP_UNDERLINE_POSITION if top != 0 => meta.underline_position = last(1),
        OP_UNDERLINE_THICKNESS if top != 0 => meta.underline_thickness = last(1),
        OP_STROKE_WIDTH if top != 0 => meta.stroke_width = last(1),
        OP_PRIVATE if top >= 2 => {
            let private_length: u32 = last(2) as u32;
            let private_offset: u32 = last(1) as u32;
            meta.private_dict = Some(new_cff_private());
            // The Private DICT's offset and length come from the font;
            // skip the DICT, keeping the default `private_dict`, when
            // they do not fit in the table.
            let raw_slice = file.raw_data.as_slice();
            if let Some(private_bytes) = raw_slice
                .get(private_offset as usize..)
                .and_then(|s| s.get(..private_length as usize))
            {
                // `meta`'s last use was the assignment above, so its borrow
                // of `context` has ended and `context` can be reborrowed.
                parse_to_callback(private_bytes, |op, top, stack| {
                    callback_extract_private(op, top, stack, context);
                });
            }
        }
        OP_ROS if top >= 3 => {
            meta.is_cid = true;
            meta.cid_registry = sid(3);
            meta.cid_ordering = sid(2);
            meta.cid_supplement = last(1) as u32;
        }
        _ => {}
    };
}
pub(crate) fn callback_draw_setwidth(context: &mut OutlineBuilderContext, width: f64) {
    context.g.advance_width = vq_create_still(width as Pos + context.nominal_width_x as Pos);
}
pub(crate) fn callback_draw_next_contour(context: &mut OutlineBuilderContext) {
    context.g.contours.push(Vec::new());
    context.j_contour = context.g.contours.len() as ShapeId;
    context.j_point = 0;
}
pub(crate) fn callback_draw_lineto(
    context: &mut OutlineBuilderContext,
    x1: f64,
    y1: f64,
) {
    if context.j_contour != 0 {
        let contour: &mut Contour = &mut context.g.contours[context.j_contour as usize - 1];
        contour.push(still_point(x1, y1, true));
        context.j_point = context.j_point.wrapping_add(1);
    }
}
/// A point with fixed coordinates, on or off the curve.
fn still_point(x: f64, y: f64, on_curve: bool) -> Point {
    Point {
        x: vq_create_still(x),
        y: vq_create_still(y),
        on_curve: on_curve as i8,
    }
}
pub(crate) fn callback_draw_curveto(
    context: &mut OutlineBuilderContext,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
) {
    if context.j_contour != 0 {
        let contour: &mut Contour = &mut context.g.contours[context.j_contour as usize - 1];
        contour.push(still_point(x1, y1, false));
        contour.push(still_point(x2, y2, false));
        contour.push(still_point(x3, y3, true));
        context.j_point = context.j_point.wrapping_add(3);
    }
}
pub(crate) fn callback_draw_sethint(
    context: &mut OutlineBuilderContext,
    is_vertical: bool,
    position: f64,
    width: f64,
) {
    let stems: &mut StemDefList = if is_vertical as i32 != 0 {
        &mut context.g.stem_v
    } else {
        &mut context.g.stem_h
    };
    stems.push(PostscriptStemDef {
        position: position as Pos,
        width: width as Pos,
        map: 0,
    });
}
pub(crate) fn callback_draw_setmask(
    context: &mut OutlineBuilderContext,
    is_contour_mask: bool,
    mask_array: &[bool],
) {
    let mask_list: &mut MaskList = if is_contour_mask as i32 != 0 {
        &mut context.g.contour_masks
    } else {
        &mut context.g.hint_masks
    };
    let mut mask: PostscriptHintMask = PostscriptHintMask {
        points_before: 0,
        contours_before: 0,
        mask_h: StemMask::default(),
        mask_v: StemMask::default(),
    };
    if context.j_contour != 0 {
        mask.contours_before = (context.j_contour as i32 - 1) as u16;
    } else {
        mask.contours_before = 0;
    }
    mask.points_before = context.j_point;
    let stem_h_len = context.g.stem_h.len();
    let stem_v_len = context.g.stem_v.len();
    // The first `stem_h_len` flags of `mask_array` are the horizontal
    // stems', the next `stem_v_len` the vertical stems'.
    for j in 0..StemMask::LEN {
        mask.mask_h.set(j, j < stem_h_len && mask_array[j]);
        mask.mask_v.set(j, j < stem_v_len && mask_array[j + stem_h_len]);
    }
    if !mask_list.is_empty()
        && mask_list[mask_list.len() - 1].contours_before as i32
            == mask.contours_before as i32
        && mask_list[mask_list.len() - 1].points_before as i32
            == mask.points_before as i32
    {
        let last = mask_list.len() - 1;
        mask_list[last].mask_h = mask.mask_h;
        mask_list[last].mask_v = mask.mask_v;
    } else {
        mask_list.push(mask);
        if is_contour_mask {
            context.defined_contour_masks = (context.defined_contour_masks as i32 + 1) as u8;
        } else {
            context.defined_hint_masks = (context.defined_hint_masks as i32 + 1) as u8;
        }
    };
}
pub(crate) fn callback_draw_getrand(context: &mut OutlineBuilderContext) -> f64 {
    let mut x: u64 = context.randx;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    context.randx = x;
    // xorshift, then put the bits into an f64's mantissa with an exponent
    // of 0 to get a uniform double in [1, 2), and subtract 1.
    let mut bits: u64 = x.wrapping_mul(2685821657736338717);
    bits = bits >> 12 | 0x3ff0000000000000;
    let q: f64 = if bits & 2048 != 0 {
        1.0f64 - 2.220_446_049_250_313E-16_f64 / 2.0f64
    } else {
        1.0
    };
    return f64::from_bits(bits) - q;
}
// `stack` is caller-owned and reused across every glyph in the font
// (`read_cff_and_glyf_tables`'s per-glyph loop constructs it once,
// outside the loop) rather than a fresh `CffStack` built on every call --
// `stack.stack` is a `Vec<CffValue>` fixed at `0x10000` entries (see
// `libcff.rs`'s `CffStack` doc comment: "generous", never approached by
// any real charstring), so reallocating and zero-filling it here on every
// single glyph turned an O(1)-per-glyph reset into an O(0x10000)-per-glyph
// allocation -- for a font with tens of thousands of glyphs, `cargo fuzz`
// found a mutated CID-keyed CFF table (`CharStrings` count pushed to
// 65535, the corpus's max u16) that spent 30+ seconds and multiple
// gigabytes of allocator churn entirely in this one `vec![CffValue::Unset;
// 0x10000]` call, repeated once per glyph -- confirmed by zeroing every
// other table in the fuzzer's input and rerunning (only the CFF table's
// presence mattered) and by `sample`-profiling the hang (allocator/
// `RawVecInner::with_capacity_in` frames dominated). Reusing one
// allocation for the whole font's glyph loop turns that into a single
// upfront cost; `index`/`stem`/`transient` (small, no heap) are still
// reset per glyph below, matching the fresh-`CffStack` semantics exactly
// -- only `stack.stack`'s backing allocation, and its stale byte contents
// past `index` (never read: every push/pop in the interpreter stays
// within `[0, index)`), are what's now carried over between glyphs.
// `meta`/`cff_file` are shared -- every access below (`fd_array`,
// `private_dict`, header fields) only ever reads through them, the same
// scope-of-use audit `name_glyphs_according_to_cff` already documents for
// its own copies of these two. `glyphs` alone needs `&mut`, to seat the new
// `Box<Glyph>` and then hand `bc.g` a lifetime-checked borrow of that same
// slot -- see `OutlineBuilderContext.g`'s own doc comment for why a plain
// borrow suffices there.
fn build_outline(
    i: GlyphId,
    meta: &CffTable,
    glyphs: &mut GlyfTable,
    cff_file: &CffFile,
    seed: &mut u64,
    stack: &mut CffStack,
) {
    stack.index = 0;
    stack.stem = 0;
    stack.transient = [CffValue::Unset; TYPE2_TRANSIENT_ARRAY];
    let f: &CffFile = cff_file;
    let g_owner: Box<Glyph> = new_glyf_glyph();
    glyphs[i as usize] = Some(g_owner);
    let seed_val: u64 = *seed;
    let mut local_subrs: CffIndex = CffIndex {
        count_type: CffIndexCountType::U16,
        count: 0,
        off_size: 0,
        offset: Vec::new(),
        data: Vec::new(),
    };
    let mut bc: OutlineBuilderContext = OutlineBuilderContext {
        g: glyphs[i as usize].as_deref_mut().unwrap(),
        j_contour: 0 as ShapeId,
        j_point: 0 as ShapeId,
        default_width_x: 0.0,
        nominal_width_x: 0.0,
        defined_h_stems: 0,
        defined_v_stems: 0,
        defined_hint_masks: 0,
        defined_contour_masks: 0,
        randx: 0,
    };
    let f_raw_data = f.raw_data.as_slice();
    let fd: u8 = if !matches!(f.fdselect, CffFdSelect::Unspecified) {
        parse_subr(
            i,
            f_raw_data,
            &f.font_dict,
            &f.fdselect,
            &mut local_subrs,
        )
    } else {
        parse_subr(
            i,
            f_raw_data,
            &f.top_dict,
            &f.fdselect,
            &mut local_subrs,
        )
    };
    bc.g.fd_select = handle_from_index(fd as GlyphId) as FdHandle;
    let ctx_fd_array: &Vec<Box<CffTable>> = &meta.fd_array;
    if (fd as usize) < ctx_fd_array.len() && ctx_fd_array[fd as usize].private_dict.is_some() {
        let pd = ctx_fd_array[fd as usize].private_dict.as_deref().unwrap();
        bc.default_width_x = pd.default_width_x;
        bc.nominal_width_x = pd.nominal_width_x;
    } else if let Some(pd) = meta.private_dict.as_deref() {
        bc.default_width_x = pd.default_width_x;
        bc.nominal_width_x = pd.nominal_width_x;
    }
    bc.g.advance_width = vq_create_still(bc.default_width_x as Pos);
    let char_strings_offset = &f.char_strings.offset;
    // CFF INDEX offsets are 1-based and `parse_index` already validated
    // this whole array (non-decreasing, every entry >= 1, and the final
    // entry exactly matches `data.len() + 1`) -- so `offset[i] - 1` is
    // always in `0..=data.len()` for any `i` within the INDEX's own
    // count, the same invariant `locate_subr`/`get_cff_sid` rely on.
    let char_string_start = (char_strings_offset[i as usize] - 1) as usize;
    let char_string_end = (char_strings_offset[(i as i32 + 1) as usize] - 1) as usize;
    let char_string_bytes: &[u8] = &f.char_strings.data[char_string_start..char_string_end];
    bc.j_contour = 0 as ShapeId;
    bc.j_point = 0 as ShapeId;
    bc.randx = seed_val;
    let mut total_subr_calls: u32 = 0;
    parse_outline(
        char_string_bytes,
        &f.global_subr,
        &local_subrs,
        stack,
        &mut bc,
        0,
        &mut total_subr_calls,
    );
    let mut cx: VQ = (vq_neutral)();
    let mut cy: VQ = (vq_neutral)();
    for contour in bc.g.contours.iter_mut() {
        for z in contour.iter_mut() {
            vq_inplace_plus(&mut cx, z.x.clone());
            vq_inplace_plus(&mut cy, z.y.clone());
            z.x = cx.clone();
            z.y = cy.clone();
        }
        // Drop a closing point that repeats the first, both on the curve.
        let (first, last) = (&contour[0], &contour[contour.len() - 1]);
        if vq_compare(first.x.clone(), last.x.clone()) == 0
            && vq_compare(first.y.clone(), last.y.clone()) == 0
            && first.on_curve != 0
            && last.on_curve != 0
        {
            contour.pop();
        }
        contour.shrink_to_fit();
    }
    bc.g.contours.shrink_to_fit();
    // `cx`/`cy`/`local_subrs` are plain owned locals, never moved out, so
    // they auto-drop when this function returns -- no explicit dispose
    // call is needed.
    *seed = bc.randx;
}
fn form_cid_string(cid: CffSid) -> Vec<u8> {
    return crate::bytesbuild!(b"CID", cid as i32);
}
fn name_glyphs_according_to_cff(meta: &CffTable, glyphs: &mut GlyfTable, cff_file: &CffFile) {
    let is_cid = meta.is_cid;
    // Ranges of format 1 and 2 charsets as (first SID, count), in glyph
    // order from glyph 1. The SID arithmetic wraps, as it always has, for a
    // range that runs past 65535.
    let ranges: Vec<(CffSid, u32)> = match &cff_file.charsets {
        CffCharset::Format0(sids) => {
            // An explicit SID per glyph. A CID font names its glyphs by
            // string too here, and records the SID as the CID.
            for (j, &sid) in sids.iter().enumerate() {
                if let Some(name) = get_cff_sid(sid, &cff_file.string) {
                    let glyph = glyphs[j + 1].as_mut().unwrap();
                    glyph.name = name;
                    if is_cid {
                        glyph.cid = sid as GlyphId;
                    }
                }
            }
            return;
        }
        CffCharset::Format1(ranges) => ranges.iter().map(|r| (r.first, r.nleft as u32 + 1)).collect(),
        CffCharset::Format2(ranges) => ranges.iter().map(|r| (r.first, r.nleft as u32 + 1)).collect(),
        _ => return,
    };
    let sids = ranges
        .into_iter()
        .flat_map(|(first, count)| (0..count).map(move |k| first.wrapping_add(k as CffSid)));
    for (slot, sid) in glyphs.iter_mut().skip(1).zip(sids) {
        let glyph = slot.as_mut().unwrap();
        if is_cid {
            glyph.name = form_cid_string(sid);
            glyph.cid = sid as GlyphId;
        } else if let Some(name) = get_cff_sid(sid, &cff_file.string) {
            glyph.name = name;
        }
    }
}
fn qround(x: f64) -> f64 {
    return from_fixed(to_fixed(x));
}
// Applies the Top DICT's `FontMatrix`, scaled by `head.unitsPerEm`. Without
// a `head` table there is nothing to scale by, so the outline stays as is.
pub(super) fn apply_cff_matrix(cff: &CffTable, glyf: &mut GlyfTable, head: Option<&HeadTable>) {
    let Some(head) = head else {
        return;
    };
    for gbox in glyf.iter_mut() {
        let g: &mut Glyph = gbox.as_mut().unwrap();
        let mut fd: &CffTable = cff;
        if (g.fd_select.index as usize) < fd.fd_array.len() {
            fd = &*fd.fd_array[g.fd_select.index as usize];
        }
        if let Some(fm) = fd.font_matrix.as_deref() {
            let a: Scale = qround(
                head.units_per_em as i32 as f64
                    * fm.a,
            ) as Scale;
            let b: Scale = qround(
                head.units_per_em as i32 as f64
                    * fm.b,
            ) as Scale;
            let c: Scale = qround(
                head.units_per_em as i32 as f64
                    * fm.c,
            ) as Scale;
            let d: Scale = qround(
                head.units_per_em as i32 as f64
                    * fm.d,
            ) as Scale;
            let mut x: VQ = vq_scale(fm.x.clone(), head.units_per_em as Scale);
            x.kernel = qround(x.kernel) as Pos;
            let mut y: VQ = vq_scale(fm.y.clone(), head.units_per_em as Scale);
            y.kernel = qround(y.kernel) as Pos;
            for contour in g.contours.iter_mut() {
                for point in contour.iter_mut() {
                    let zx: VQ = point.x.clone();
                    let zy: VQ = point.y.clone();
                    point.x = vq_point_linear_tfm(x.clone(), a as Pos, zx.clone(), b as Pos, zy.clone());
                    point.y = vq_point_linear_tfm(y.clone(), c as Pos, zx.clone(), d as Pos, zy.clone());
                    // `zx`/`zy` are plain owned locals, never moved out, so
                    // they auto-drop at the end of this iteration -- no
                    // explicit dispose call is needed.
                }
            }
            // `x`/`y` are plain owned locals, never moved out, so they
            // auto-drop at the end of this block -- no explicit dispose
            // call is needed.
        }
    }
}
pub fn read_cff_and_glyf_tables(
    packet: &Packet,
    head: Option<&HeadTable>,
) -> CffAndGlyfOwned {
    let mut ret: CffAndGlyfOwned = CffAndGlyfOwned::default();
    // Only the first `CFF ` table in the packet is read.
    if let Some(table) = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_CFF) {
        let cff_file: Box<CffFile> = open_stream(&table.data);
        // A Top DICT INDEX with a count of 0 has no offsets to read.
        if cff_file.top_dict.count != 0 {
            let mut meta: Box<CffTable> = table_cff_new();

            // ---- Phase A: Top/Font DICT + Private DICT extraction ----
            {
                let mut context = CffFdExtractContext {
                    fd_array_index: -1,
                    meta: &mut meta,
                    cff_file: &cff_file,
                };
                parse_to_callback(
                    {
                        let top_dict_len = {
                            let top_dict_offset = &cff_file.top_dict.offset;
                            (top_dict_offset[1])
                                .wrapping_sub(top_dict_offset[0])
                        } as usize;
                        let top_dict_data: &[u8] = &cff_file.top_dict.data;
                        top_dict_data.get(..top_dict_len).unwrap_or(&[])
                    },
                    |op, top, stack| {
                        callback_extract_fd(op, top, stack, &mut context);
                    },
                );
                if context.meta.font_name.is_empty() {
                    context.meta.font_name =
                        get_cff_sid(391, &cff_file.name).unwrap_or_default();
                }
                if cff_file.font_dict.count != 0 {
                    let fd_count = cff_file.font_dict.count as usize;
                    context.meta.fd_array = Vec::with_capacity(fd_count);
                    for j in 0..fd_count as TableId {
                        // Pushed *before* the recursive parse below (not
                        // after): `context.fd_array_index` makes
                        // `callback_extract_fd`/`callback_extract_private`
                        // re-derive `meta` as `(*meta).fd_array[j]` while
                        // this element is still being populated, so it
                        // must already be present in the `Vec` at that
                        // index -- a `Box`'s heap address never moves,
                        // even if a later `push` reallocates the `Vec`'s
                        // own backing buffer of `Box` pointers.
                        context.meta.fd_array.push(table_cff_new());
                        context.fd_array_index = j as i32;
                        parse_to_callback(
                            {
                                let font_dict_offset = &cff_file.font_dict.offset;
                                let start =
                                    font_dict_offset[j as usize].wrapping_sub(1) as usize;
                                let len = (font_dict_offset[(j as i32
                                    + 1)
                                    as usize])
                                    .wrapping_sub(font_dict_offset[j as usize])
                                    as usize;
                                let font_dict_data: &[u8] = &cff_file.font_dict.data;
                                font_dict_data
                                    .get(start..)
                                    .and_then(|s| s.get(..len))
                                    .unwrap_or(&[])
                            },
                            |op, top, stack| {
                                callback_extract_fd(op, top, stack, &mut context);
                            },
                        );
                        if context.meta.fd_array[j as usize].font_name.is_empty() {
                            context.meta.fd_array[j as usize].font_name =
                                crate::bytesbuild!(b"_Subfont", j as i32);
                        }
                    }
                }
            } // `context` (and its `&mut meta` borrow) ends here.

            let mut seed: u64 = 0x1234567887654321;
            if let Some(pd) = meta.private_dict.as_deref() {
                seed = pd.initial_random_seed as u64 ^ 0x1234567887654321;
            }
            let mut glyphs: GlyfTable = table_glyf_create_n(cff_file.char_strings.count as usize);

            // ---- Phase B: per-glyph outline building + naming ----
            {
                let meta_ref: &CffTable = &meta;
                let glyphs_ref: &mut GlyfTable = &mut glyphs;
                let cff_file_ref: &CffFile = &cff_file;
                // Allocated once for the whole font, not once per
                // glyph -- see `build_outline`'s doc comment.
                let mut outline_stack: CffStack = CffStack {
                    stack: vec![CffValue::Unset; 0x10000],
                    transient: [CffValue::Unset; TYPE2_TRANSIENT_ARRAY],
                    index: 0,
                    stem: 0,
                };
                for j_0 in 0..glyphs_ref.len() {
                    build_outline(
                        j_0 as GlyphId,
                        meta_ref,
                        glyphs_ref,
                        cff_file_ref,
                        &mut seed,
                        &mut outline_stack,
                    );
                }
                apply_cff_matrix(meta_ref, glyphs_ref, head);
                name_glyphs_according_to_cff(meta_ref, glyphs_ref, cff_file_ref);
            } // `meta_ref`/`glyphs_ref`/`cff_file_ref` end here.

            ret.meta = Some(meta);
            ret.glyphs = Some(glyphs);
        }
        // `cff_file`'s own `Drop` glue runs here, at the end of its scope
        // -- no explicit `Box::from_raw` + `drop` needed any more.
    }
    return ret;
}
