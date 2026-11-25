use super::*;

use crate::libcff::CffDictOperator;
use crate::libcff::charset::{CffCharset, CffCharsetRangeFormat1, CffCharsetRangeFormat2};
use crate::libcff::charstring_il::CffCharstringIl;
use crate::libcff::dict::{CffDict, CffDictEntry};
use crate::libcff::fdselect::{CffFdSelect, CffFdSelectRangeFormat3};
use crate::libcff::index::CffIndex;
use crate::libcff::subr::CffSubrGraph;
use crate::libcff::value::CffValue;
use crate::libcff::{
    OP_BLUE_FUZZ, OP_BLUE_SCALE, OP_BLUE_SHIFT, OP_BLUE_VALUES, OP_CHAR_STRINGS, OP_CHARSET,
    OP_CID_COUNT, OP_CID_FONT_REVISION, OP_CID_FONT_VERSION, OP_COPYRIGHT, OP_DEFAULT_WIDTH_X,
    OP_EXPANSION_FACTOR, OP_FAMILY_BLUES, OP_FAMILY_NAME, OP_FAMILY_OTHER_BLUES, OP_FD_ARRAY,
    OP_FD_SELECT, OP_FONT_BBOX, OP_FONT_MATRIX, OP_FONT_NAME, OP_FORCE_BOLD, OP_FULL_NAME,
    OP_INITIAL_RANDOM_SEED, OP_IS_FIXED_PITCH, OP_ITALIC_ANGLE, OP_LANGUAGE_GROUP,
    OP_NOMINAL_WIDTH_X, OP_NOTICE, OP_OTHER_BLUES, OP_PRIVATE, OP_ROS, OP_STD_HW, OP_STD_VW,
    OP_STEM_SNAP_H, OP_STEM_SNAP_V, OP_STROKE_WIDTH, OP_SUBRS, OP_UID_BASE, OP_UNDERLINE_POSITION,
    OP_UNDERLINE_THICKNESS, OP_VERSION, OP_WEIGHT,
};
use crate::support::options::Options;
use crate::support::primitives::Arity;
use crate::table::glyf::GlyfTable;
use otfcc_binary::Buffer;

use crate::libcff::charset::build_charset;
use crate::libcff::charstring_il::{compile_glyph_to_il, optimize_il};
use crate::libcff::codecs::encode_operator;
use crate::libcff::dict::build_dict;
use crate::libcff::fdselect::build_fd_select;
use crate::libcff::index::{build_index, new_index_by_callback};
use crate::libcff::subr::{il_graph_to_buffers, insert_il_to_graph};
use crate::libcff::writer::{build_header, build_offset};
use crate::support::error::BuildError;
use crate::support::primitives::until_nul;
use crate::vf::vq::vq_get_still;

fn cff_make_charstrings(
    context: &mut CffCharstringBuilderContext,
) -> Result<(Buffer, Buffer, Buffer), BuildError> {
    let glyf: &GlyfTable = context.glyf;
    if glyf.is_empty() {
        // With 0 glyphs, `il_graph_to_buffers` below never runs, so
        // the caller (`writecff_cid_keyed`) still needs three empty (but
        // real) `Buffer`s here.
        return Ok((Buffer::new(), Buffer::new(), Buffer::new()));
    }
    let options: &Options = context.options;
    for entry in glyf.iter() {
        let mut il: CffCharstringIl = compile_glyph_to_il(
            entry
                .as_deref()
                .expect("every glyph slot is filled by the time the CFF table is built"),
            context.default_width,
            context.nominal_width_x,
        );
        optimize_il(&mut il, options);
        insert_il_to_graph(&mut context.graph, &il);
    }
    il_graph_to_buffers(&mut context.graph)
}
// Returns the SID of `s`, registering it if new: strings get SIDs from 391
// up in the order they are first seen, and the `IndexMap` keeps that order
// for writing. Strings are compared up to their first NUL, but the first
// one registered is stored, and written, in full.
fn sidof(h: &mut indexmap::IndexMap<Vec<u8>, Vec<u8>>, s: &[u8]) -> i32 {
    let key: Vec<u8> = until_nul(s).to_vec();
    if let Some(idx) = h.get_index_of(&key) {
        return 391 + idx as i32;
    }
    let idx = h.len();
    h.insert(key, s.to_vec());
    391 + idx as i32
}
fn cffdict_givemeablank(dict: &mut CffDict) -> &mut CffDictEntry {
    dict.ents.push(CffDictEntry {
        op: CffDictOperator(0),
        vals: Vec::new(),
    });
    dict.ents
        .last_mut()
        .expect("an entry was pushed just above")
}
/// Append a DICT entry whose operands are numbers. See also
/// [`cffdict_input_ints`].
fn cffdict_input_doubles(dict: &mut CffDict, op: CffDictOperator, values: &[f64]) {
    let mut vals: Vec<CffValue> = Vec::with_capacity(values.len());
    for &x in values.iter() {
        // A whole number is stored as an integer, which decides whether
        // the DICT encodes it as an integer or a real operand.
        // `f64::round` rounds half away from zero, like C's `round()`.
        vals.push(if x == x.round() {
            CffValue::Integer(x.round() as i32)
        } else {
            CffValue::Double(x)
        });
    }
    let last = cffdict_givemeablank(dict);
    last.op = op;
    last.vals = vals;
}

/// Append a DICT entry whose operands are integers. See [`cffdict_input_doubles`].
fn cffdict_input_ints(dict: &mut CffDict, op: CffDictOperator, values: &[i32]) {
    let mut vals: Vec<CffValue> = Vec::with_capacity(values.len());
    for &x in values.iter() {
        vals.push(CffValue::Integer(x));
    }
    let last = cffdict_givemeablank(dict);
    last.op = op;
    last.vals = vals;
}

/// Was generic over a `CffValueType` (`Double` vs `Integer` operand
/// encoding) passed in by the caller, but every one of its 6 call sites
/// (`cff_make_private_dict`) passed `Double` -- confirmed by grep, not
/// assumed, matching the dead-`Integer`-branch observation this file's own
/// doc comment already made above. That branch, and the `t` parameter that
/// selected it, are gone along with `CffValueType`. Kept as its own
/// function rather than replaced with direct `cffdict_input_doubles` calls
/// at each site because of the one behavior it has that
/// `cffdict_input_doubles` doesn't: skip emitting the DICT entry entirely
/// when `arr` is empty, instead of adding one with zero operands.
fn cffdict_input_array(dict: &mut CffDict, op: CffDictOperator, arr: &[f64]) {
    if arr.is_empty() {
        return;
    }
    cffdict_input_doubles(dict, op, arr);
}
fn cff_make_fd_dict(fd: &CffTable, h: &mut indexmap::IndexMap<Vec<u8>, Vec<u8>>) -> CffDict {
    let mut dict = CffDict { ents: Vec::new() };
    if !fd.cid_registry.is_empty() && !fd.cid_ordering.is_empty() {
        cffdict_input_ints(
            &mut dict,
            OP_ROS,
            &[
                sidof(h, &fd.cid_registry),
                sidof(h, &fd.cid_ordering),
                fd.cid_supplement as i32,
            ],
        );
    }
    if !fd.version.is_empty() {
        cffdict_input_ints(&mut dict, OP_VERSION, &[sidof(h, &fd.version)]);
    }
    if !fd.notice.is_empty() {
        cffdict_input_ints(&mut dict, OP_NOTICE, &[sidof(h, &fd.notice)]);
    }
    if !fd.copyright.is_empty() {
        cffdict_input_ints(&mut dict, OP_COPYRIGHT, &[sidof(h, &fd.copyright)]);
    }
    if !fd.full_name.is_empty() {
        cffdict_input_ints(&mut dict, OP_FULL_NAME, &[sidof(h, &fd.full_name)]);
    }
    if !fd.family_name.is_empty() {
        cffdict_input_ints(&mut dict, OP_FAMILY_NAME, &[sidof(h, &fd.family_name)]);
    }
    if !fd.weight.is_empty() {
        cffdict_input_ints(&mut dict, OP_WEIGHT, &[sidof(h, &fd.weight)]);
    }
    cffdict_input_doubles(
        &mut dict,
        OP_FONT_BBOX,
        &[
            fd.font_b_box_left,
            fd.font_b_box_bottom,
            fd.font_b_box_right,
            fd.font_b_box_top,
        ],
    );
    cffdict_input_ints(
        &mut dict,
        OP_IS_FIXED_PITCH,
        &[i32::from(fd.is_fixed_pitch)],
    );
    cffdict_input_doubles(&mut dict, OP_ITALIC_ANGLE, &[fd.italic_angle]);
    cffdict_input_doubles(&mut dict, OP_UNDERLINE_POSITION, &[fd.underline_position]);
    cffdict_input_doubles(&mut dict, OP_UNDERLINE_THICKNESS, &[fd.underline_thickness]);
    cffdict_input_doubles(&mut dict, OP_STROKE_WIDTH, &[fd.stroke_width]);
    if let Some(fm) = fd.font_matrix.as_deref() {
        cffdict_input_doubles(
            &mut dict,
            OP_FONT_MATRIX,
            &[
                fm.a,
                fm.b,
                fm.c,
                fm.d,
                vq_get_still(fm.x.clone()) as f64,
                vq_get_still(fm.y.clone()) as f64,
            ],
        );
    }
    if !fd.font_name.is_empty() {
        cffdict_input_ints(&mut dict, OP_FONT_NAME, &[sidof(h, &fd.font_name)]);
    }
    if fd.cid_font_version != 0. {
        cffdict_input_doubles(&mut dict, OP_CID_FONT_VERSION, &[fd.cid_font_version]);
    }
    if fd.cid_font_revision != 0. {
        cffdict_input_doubles(&mut dict, OP_CID_FONT_REVISION, &[fd.cid_font_revision]);
    }
    if fd.cid_count != 0 {
        cffdict_input_ints(&mut dict, OP_CID_COUNT, &[fd.cid_count as i32]);
    }
    if fd.uid_base != 0 {
        cffdict_input_ints(&mut dict, OP_UID_BASE, &[fd.uid_base as i32]);
    }
    dict
}
fn cff_make_private_dict(pd: Option<&CffPrivateDict>) -> CffDict {
    let mut dict = CffDict { ents: Vec::new() };
    let Some(pd) = pd else {
        return dict;
    };
    cffdict_input_array(&mut dict, OP_BLUE_VALUES, &pd.blue_values);
    cffdict_input_array(&mut dict, OP_OTHER_BLUES, &pd.other_blues);
    cffdict_input_array(&mut dict, OP_FAMILY_BLUES, &pd.family_blues);
    cffdict_input_array(&mut dict, OP_FAMILY_OTHER_BLUES, &pd.family_other_blues);
    cffdict_input_array(&mut dict, OP_STEM_SNAP_H, &pd.stem_snap_h);
    cffdict_input_array(&mut dict, OP_STEM_SNAP_V, &pd.stem_snap_v);
    cffdict_input_doubles(&mut dict, OP_BLUE_SCALE, &[pd.blue_scale]);
    cffdict_input_doubles(&mut dict, OP_BLUE_SHIFT, &[pd.blue_shift]);
    cffdict_input_doubles(&mut dict, OP_BLUE_FUZZ, &[pd.blue_fuzz]);
    cffdict_input_doubles(&mut dict, OP_STD_HW, &[pd.std_hw]);
    cffdict_input_doubles(&mut dict, OP_STD_VW, &[pd.std_vw]);
    cffdict_input_ints(&mut dict, OP_FORCE_BOLD, &[i32::from(pd.force_bold)]);
    cffdict_input_ints(&mut dict, OP_LANGUAGE_GROUP, &[pd.language_group as i32]);
    cffdict_input_doubles(&mut dict, OP_EXPANSION_FACTOR, &[pd.expansion_factor]);
    cffdict_input_doubles(&mut dict, OP_INITIAL_RANDOM_SEED, &[pd.initial_random_seed]);
    cffdict_input_doubles(&mut dict, OP_DEFAULT_WIDTH_X, &[pd.default_width_x]);
    cffdict_input_doubles(&mut dict, OP_NOMINAL_WIDTH_X, &[pd.nominal_width_x]);
    dict
}
fn cffstrings_to_indexblob(
    h: &mut indexmap::IndexMap<Vec<u8>, Vec<u8>>,
) -> Result<Buffer, BuildError> {
    let n: u32 = h.len() as u32;
    // The `IndexMap` is in SID order (see `sidof`).
    let blobs: Vec<Buffer> = ::core::mem::take(h)
        .into_iter()
        .map(|(_, value)| Buffer::from_bytes(&value))
        .collect();
    let strings: CffIndex = new_index_by_callback(n, blobs.into_iter());
    build_index(&strings)
}
fn cff_compile_nameindex(cff: &mut CffTable) -> Result<Buffer, BuildError> {
    if cff.font_name.is_empty() {
        cff.font_name = b"Caryll-CFF-FONT".to_vec();
    }
    // The name is written with a trailing NUL.
    let mut name_data: Vec<u8> = cff.font_name.clone();
    name_data.push(0);
    let name_index = CffIndex {
        count: 1 as Arity,
        off_size: 4,
        offset: vec![1, cff.font_name.len() as u32 + 1],
        data: name_data,
        ..Default::default()
    };
    let buf = build_index(&name_index)?;
    cff.font_name = Vec::new();
    Ok(buf)
}
/// The CID of each glyph after `.notdef`, in glyph order: the glyph's own
/// `cid`, so a subset CID font keeps its CIDs. A glyph without one (0), or
/// repeating an earlier glyph's, takes its glyph id, or when another glyph
/// already has that CID, the first free CID above the given ones: each CID
/// names one glyph (5176 §18). A font built without CIDs is numbered as before.
pub(crate) fn charset_cids(glyf: &GlyfTable) -> Vec<u16> {
    let given: Vec<u16> = glyf
        .iter()
        .skip(1)
        .map(|slot| slot.as_deref().map_or(0, |g| g.cid))
        .collect();
    let mut taken = std::collections::HashSet::new();
    let mut cids: Vec<Option<u16>> = given
        .iter()
        .map(|&cid| (cid != 0 && taken.insert(cid)).then_some(cid))
        .collect();
    let mut next = given.iter().copied().max().unwrap_or(0);
    for (j, cid) in cids.iter_mut().enumerate() {
        if cid.is_none() {
            let gid = (j + 1) as u16;
            let fallback = if taken.insert(gid) {
                gid
            } else {
                next = next.saturating_add(1);
                while !taken.insert(next) {
                    next = next.saturating_add(1);
                }
                next
            };
            *cid = Some(fallback);
        }
    }
    cids.into_iter().flatten().collect()
}
/// A charset for `ids`, the SIDs or CIDs of the glyphs after `.notdef`:
/// one run of consecutive ids as a format 2 range, and otherwise the
/// smallest of format 0 (two bytes per glyph) and the runs as format 1
/// (three bytes per run of up to 256) or format 2 (four bytes per run).
fn charset_for_ids(ids: &[u16]) -> CffCharset {
    let mut runs: Vec<(u16, u16)> = Vec::new();
    for &id in ids {
        match runs.last_mut() {
            Some((first, nleft)) if first.wrapping_add(*nleft).wrapping_add(1) == id => *nleft += 1,
            _ => runs.push((id, 0)),
        }
    }
    let format0 = 2 * ids.len();
    let format1: usize = runs
        .iter()
        .map(|&(_, n)| 3 * (usize::from(n) / 256 + 1))
        .sum();
    let format2 = 4 * runs.len();
    // One run (the ids of nearly every font) stays the single format 2
    // range it has always been written as.
    if runs.len() == 1 {
        let (first, nleft) = runs[0];
        return CffCharset::Format2(vec![CffCharsetRangeFormat2 { first, nleft }]);
    }
    if format0 <= format1.min(format2) {
        CffCharset::Format0(ids.to_vec())
    } else if format1 < format2 {
        let mut ranges = Vec::new();
        for (first, nleft) in runs {
            let mut at = 0u32;
            while at <= u32::from(nleft) {
                let n = (u32::from(nleft) - at).min(255);
                ranges.push(CffCharsetRangeFormat1 {
                    first: (u32::from(first) + at) as u16,
                    nleft: n as u8,
                });
                at += n + 1;
            }
        }
        CffCharset::Format1(ranges)
    } else {
        CffCharset::Format2(
            runs.into_iter()
                .map(|(first, nleft)| CffCharsetRangeFormat2 { first, nleft })
                .collect(),
        )
    }
}
/// The charset: each glyph's CID in a CID-keyed font, and the SID of its
/// name otherwise. The ids are written as they are, not assumed to follow
/// one another: a name can share the SID of an earlier string (a `Weight`
/// of `B` and a glyph `B`), and a subset CID font keeps its CIDs.
fn cff_make_charset(
    cff: &CffTable,
    glyf: &GlyfTable,
    string_hash: &mut indexmap::IndexMap<Vec<u8>, Vec<u8>>,
) -> Buffer {
    let charset: CffCharset = if glyf.len() > 1 {
        let ids: Vec<u16> = if cff.is_cid {
            charset_cids(glyf)
        } else {
            glyf.iter()
                .skip(1)
                .map(|entry| {
                    let name = &entry
                        .as_deref()
                        .expect("every glyph slot is filled by the time the CFF table is built")
                        .name;
                    sidof(string_hash, name) as u16
                })
                .collect()
        };
        charset_for_ids(&ids)
    } else {
        CffCharset::IsoAdobe
    };
    build_charset(&charset)
}
// The `range3` array's final length isn't pre-counted anymore -- a `Vec`
// absorbs the counting pass, same as `Coverage`/`ClassDef`/`gpos_pair.rs`'s
// own scratch-buffer conversions. `.s`'s old dual role (a running write
// cursor through the loop, overwritten with the final range count right
// after) collapses into a single sequential `.push()` per transition.
fn cff_make_fdselect(cff: &CffTable, glyf: &GlyfTable) -> Buffer {
    if !cff.is_cid {
        return Buffer::new();
    }
    let fds: CffFdSelect = if !glyf.is_empty() {
        let mut fdi0: u8 = glyf[0]
            .as_deref()
            .expect("every glyph slot is filled by the time the CFF table is built")
            .fd_select
            .index as u8;
        if fdi0 as usize > cff.fd_array.len() {
            fdi0 = 0;
        }
        let mut current: u8 = fdi0;
        let mut range3: Vec<CffFdSelectRangeFormat3> = vec![CffFdSelectRangeFormat3 {
            first: 0,
            fd: current,
        }];
        for (j, entry) in glyf.iter().enumerate().skip(1) {
            let mut fdi: u8 = entry
                .as_deref()
                .expect("every glyph slot is filled by the time the CFF table is built")
                .fd_select
                .index as u8;
            if fdi as usize > cff.fd_array.len() {
                fdi = 0;
            }
            if fdi != current {
                current = fdi;
                range3.push(CffFdSelectRangeFormat3 {
                    first: j as u16,
                    fd: current,
                });
            }
        }
        CffFdSelect::Format3 {
            range3,
            sentinel: glyf.len() as u16,
        }
    } else {
        CffFdSelect::Unspecified
    };
    build_fd_select(&fds)
}
fn compile_fd_buffer(
    fd_array: &[Box<CffTable>],
    string_hash: &mut indexmap::IndexMap<Vec<u8>, Vec<u8>>,
    i: u32,
) -> Buffer {
    let fd: CffDict = cff_make_fd_dict(&fd_array[i as usize], string_hash);
    let mut blob: Buffer = build_dict(&fd);
    blob.write_buffer_owned(build_offset(0xeeeeeeee_u32 as i32));
    blob.write_buffer_owned(build_offset(0xffffffff_u32 as i32));
    blob.write_buffer_owned(encode_operator(OP_PRIVATE));
    blob
}
fn cff_make_fdarray(
    fd_array: &[Box<CffTable>],
    string_hash: &mut indexmap::IndexMap<Vec<u8>, Vec<u8>>,
) -> CffIndex {
    let len = fd_array.len() as u32;
    new_index_by_callback(
        len,
        (0..len).map(|i| compile_fd_buffer(fd_array, string_hash, i)),
    )
}
/// Writes the `<offset> <operator>` pairs the top dict ends with: one per
/// non-empty section that follows it. `off` is where the first of those
/// sections starts, and moves past each one written.
fn write_top_dict_offsets(
    blob: &mut Buffer,
    off: &mut u32,
    charset: &Buffer,
    fd_select: &Buffer,
    char_strings: &Buffer,
    private_dict: &Buffer,
    fd_array: &Buffer,
) {
    if !charset.is_empty() {
        blob.write_buffer_owned(build_offset(*off as i32));
        blob.write_buffer_owned(encode_operator(OP_CHARSET));
        *off += charset.len() as u32;
    }
    if !fd_select.is_empty() {
        blob.write_buffer_owned(build_offset(*off as i32));
        blob.write_buffer_owned(encode_operator(OP_FD_SELECT));
        *off += fd_select.len() as u32;
    }
    if !char_strings.is_empty() {
        blob.write_buffer_owned(build_offset(*off as i32));
        blob.write_buffer_owned(encode_operator(OP_CHAR_STRINGS));
        *off += char_strings.len() as u32;
    }
    if !private_dict.is_empty() {
        blob.write_buffer_owned(build_offset(private_dict.len() as i32));
        blob.write_buffer_owned(build_offset(*off as i32));
        blob.write_buffer_owned(encode_operator(OP_PRIVATE));
        *off += private_dict.len() as u32;
    }
    if !fd_array.is_empty() {
        blob.write_buffer_owned(build_offset(*off as i32));
        blob.write_buffer_owned(encode_operator(OP_FD_ARRAY));
        *off += fd_array.len() as u32;
    }
}
/// Builds each font dict's private dict and patches its size and offset into
/// the font dict's `Private` operands inside `idx`. `first_offset` is where
/// the first of those private dicts will land in the file.
fn patch_fd_array_privates(
    fd_array: &[Box<CffTable>],
    idx: &mut CffIndex,
    first_offset: u32,
) -> Vec<Buffer> {
    let mut fd_array_privates_start_offset: u32 = first_offset;
    let mut fd_array_privates: Vec<Buffer> = Vec::with_capacity(fd_array.len());
    for (j, fd) in fd_array.iter().enumerate() {
        let pd: CffDict = cff_make_private_dict(fd.private_dict.as_deref());
        let mut private_dict = build_dict(&pd);
        // Placeholder for the local subroutines' offset, patched below.
        private_dict.write_buffer_owned(build_offset(-1));
        private_dict.write_buffer_owned(encode_operator(OP_SUBRS));
        // Each font dict ends with `<size> <offset> Private`, two 5-byte
        // operands and a 1-byte operator; patch both operands in place.
        let dict_end = idx.offset[j + 1] as usize;
        let size_at = dict_end - 11;
        idx.data[size_at..size_at + 4].copy_from_slice(&(private_dict.len() as u32).to_be_bytes());
        let offset_at = dict_end - 6;
        idx.data[offset_at..offset_at + 4]
            .copy_from_slice(&fd_array_privates_start_offset.to_be_bytes());
        fd_array_privates_start_offset += private_dict.len() as u32;
        fd_array_privates.push(private_dict);
    }
    fd_array_privates
}
/// Points every private dict's `Subrs` operand at the local subroutines.
fn patch_local_subrs_offsets(
    blob: &mut Buffer,
    starting_position_of_privates: &[usize],
    ending_position_of_privates: &[usize],
    position_of_local_subroutines: usize,
) {
    // One start/end pair per private dict: the top dict's, then each font
    // dict's.
    for (&start, &end) in starting_position_of_privates
        .iter()
        .zip(ending_position_of_privates.iter())
    {
        // Patch the `Subrs` placeholder at the end of each private dict:
        // the offset operand is the last 5 bytes before the operator.
        let ls_offset = (position_of_local_subroutines - start) as u32;
        let ptr_off: usize = end - 5;
        blob.data[ptr_off..ptr_off + 4].copy_from_slice(&ls_offset.to_be_bytes());
    }
}
pub(super) fn writecff_cid_keyed(
    cff: &mut CffTable,
    glyf: Option<&GlyfTable>,
    options: &Options,
) -> Result<Buffer, BuildError> {
    // `glyf` is `None` when the font has a `CFF_` table but no `glyf`
    // table at all (e.g. `{"CFF_": {}}`) -- every function below this
    // point reads it unconditionally, assuming a present-but-possibly-
    // empty `GlyfTable`. None of them mutate its length or write through
    // it, only read glyph data to emit CFF bytes, so a local empty stand-
    // in is exactly equivalent to "0 glyphs" for all of them.
    let empty_glyf: GlyfTable = Vec::new();
    let glyf: &GlyfTable = glyf.unwrap_or(&empty_glyf);
    let mut blob = Buffer::new();
    let mut string_hash: indexmap::IndexMap<Vec<u8>, Vec<u8>> = indexmap::IndexMap::new();
    let h = build_header();
    let n = cff_compile_nameindex(cff)?;
    let top: CffDict = cff_make_fd_dict(cff, &mut string_hash);
    let t = build_dict(&top);
    let top_pd: CffDict = cff_make_private_dict(cff.private_dict.as_deref());
    let mut p = build_dict(&top_pd);
    p.write_buffer_owned(build_offset(0xffffffff_u32 as i32));
    p.write_buffer_owned(encode_operator(OP_SUBRS));
    let e = cff_make_fdselect(cff, glyf);
    let mut fd_array_index: Option<CffIndex> = None;
    let mut r: Buffer;
    if cff.is_cid {
        let idx = cff_make_fdarray(&cff.fd_array, &mut string_hash);
        r = build_index(&idx)?;
        fd_array_index = Some(idx);
    } else {
        r = Buffer::new();
    }
    let c = cff_make_charset(cff, glyf, &mut string_hash);
    let i = cffstrings_to_indexblob(&mut string_hash)?;
    let mut g2c_context: CffCharstringBuilderContext = CffCharstringBuilderContext {
        glyf,
        default_width: cff
            .private_dict
            .as_deref()
            .expect("the top-level CFF table always has a private dict (parse_cff adds one)")
            .default_width_x as u16,
        nominal_width_x: cff
            .private_dict
            .as_deref()
            .expect("the top-level CFF table always has a private dict (parse_cff adds one)")
            .nominal_width_x as u16,
        options,
        graph: CffSubrGraph::new(options.cff_do_subroutinize),
    };
    let (s, gs, ls) = cff_make_charstrings(&mut g2c_context)?;
    // The top dict gets one offset operator per non-empty section that
    // follows it: 5 bytes for the offset plus the operator's 1 or 2.
    let mut additional_top_dict_ops_size: u32 = 0;
    for (section, op_size) in [(&c, 6), (&e, 7), (&s, 6), (&p, 11), (&r, 7)] {
        if !section.is_empty() {
            additional_top_dict_ops_size += op_size;
        }
    }
    // Header, name index and the top dict index's 11-byte header, then the
    // top dict itself.
    let mut off: u32 = (h.len() + n.len() + 11 + t.len()) as u32;
    blob.write_buffer_owned(h);
    blob.write_buffer_owned(n);
    // A one-entry top dict index with 4-byte offsets: count 1, offSize 4,
    // offsets 1 and 1 + the dict's size.
    let delta_size = (t.len() + additional_top_dict_ops_size as usize + 1) as u32;
    let mut top_dict_index_header = vec![0, 1, 4, 0, 0, 0, 1];
    top_dict_index_header.extend_from_slice(&delta_size.to_be_bytes());
    blob.write_buffer_owned(Buffer::from_bytes(&top_dict_index_header));
    blob.write_buffer_owned(t);
    off += additional_top_dict_ops_size + (i.len() + gs.len()) as u32;
    write_top_dict_offsets(&mut blob, &mut off, &c, &e, &s, &p, &r);
    blob.write_buffer_owned(i);
    blob.write_buffer_owned(gs);
    blob.write_buffer_owned(c);
    blob.write_buffer_owned(e);
    blob.write_buffer_owned(s);
    let mut starting_position_of_privates: Vec<usize> = vec![0; 1 + cff.fd_array.len()];
    starting_position_of_privates[0] = blob.pos();
    blob.write_buffer_owned(p);
    let mut ending_position_of_privates: Vec<usize> = vec![0; 1 + cff.fd_array.len()];
    ending_position_of_privates[0] = blob.pos();
    if cff.is_cid {
        // `fd_array_index` is set above exactly when `cff.is_cid` is true.
        let idx: &mut CffIndex = fd_array_index
            .as_mut()
            .expect("a CID font has an FDArray index");
        let fd_array_privates = patch_fd_array_privates(&cff.fd_array, idx, off);
        r = build_index(idx)?;
        blob.write_buffer_owned(r);
        for (j, private_dict) in fd_array_privates.into_iter().enumerate() {
            starting_position_of_privates[j + 1] = blob.pos();
            blob.write_buffer_owned(private_dict);
            ending_position_of_privates[j + 1] = blob.pos();
        }
    } else {
        blob.write_buffer_owned(r);
    }
    let position_of_local_subroutines: usize = blob.pos();
    blob.write_buffer_owned(ls);
    patch_local_subrs_offsets(
        &mut blob,
        &starting_position_of_privates,
        &ending_position_of_privates,
        position_of_local_subroutines,
    );
    Ok(blob)
}
/// Builds the `CFF ` table for `cff_and_glyf`: the glyphs' charstrings
/// (subroutinized according to `options`), the Top/Font/Private DICTs and the
/// charset (see `writecff_cid_keyed`).
///
/// Fails with `BuildError::TooManyEntries` when an INDEX would hold more than
/// 65535 entries (the CFF 1 limit on its 16-bit count).
pub fn build_cff(cff_and_glyf: CffAndGlyfRef, options: &Options) -> Result<Buffer, BuildError> {
    writecff_cid_keyed(cff_and_glyf.meta, cff_and_glyf.glyphs, options)
}

#[cfg(test)]
mod charset_tests {
    use super::*;

    fn format_of(c: &CffCharset) -> u8 {
        match c {
            CffCharset::Format0(_) => 0,
            CffCharset::Format1(_) => 1,
            CffCharset::Format2(_) => 2,
            _ => 255,
        }
    }

    fn glyf_with_cids(cids: &[u16]) -> GlyfTable {
        std::iter::once(0)
            .chain(cids.iter().copied())
            .map(|cid| {
                let mut g = crate::table::glyf::new_glyf_glyph();
                g.cid = cid;
                Some(g)
            })
            .collect()
    }

    // Each CID names one glyph. A glyph without a CID takes its glyph id,
    // unless another glyph already has that CID; a repeated CID is not kept.
    #[test]
    fn every_glyph_gets_a_cid_of_its_own() {
        // Glyph 1 has none, and glyph 2 has CID 1, so glyph 1 takes the first
        // free CID above the given ones (2); glyph 3 repeats CID 1 and takes
        // its glyph id.
        assert_eq!(charset_cids(&glyf_with_cids(&[0, 1, 1])), [2, 1, 3]);
        // Given CIDs are kept as they are.
        assert_eq!(
            charset_cids(&glyf_with_cids(&[5, 500, 1000])),
            [5, 500, 1000]
        );
        // No CIDs at all: numbered by glyph id, as before.
        assert_eq!(charset_cids(&glyf_with_cids(&[0, 0, 0])), [1, 2, 3]);
    }

    #[test]
    fn the_charset_format_fits_the_ids() {
        // One run, as nearly every font has: a single format 2 range.
        let c = charset_for_ids(&[1, 2, 3, 4]);
        assert_eq!(format_of(&c), 2);
        // Scattered ids: two bytes each in format 0.
        assert_eq!(format_of(&charset_for_ids(&[5, 500, 1000])), 0);
        // Short runs: format 1 (three bytes a run against four), with a run
        // of more than 256 split in two.
        let ids: Vec<u16> = (10..310)
            .chain(1000..1010)
            .chain(2000..2010)
            .chain(3000..3010)
            .collect();
        let CffCharset::Format1(ranges) = charset_for_ids(&ids) else {
            panic!("format 1 expected");
        };
        let ranges: Vec<(u16, u8)> = ranges.iter().map(|r| (r.first, r.nleft)).collect();
        assert_eq!(
            ranges,
            [(10, 255), (266, 43), (1000, 9), (2000, 9), (3000, 9)]
        );
        // Few long runs: format 2.
        let ids: Vec<u16> = (10..600).chain(1000..1600).collect();
        assert_eq!(format_of(&charset_for_ids(&ids)), 2);
    }
}
