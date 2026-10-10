
use crate::support::handle::{FdHandle, handle_from_index};

use crate::font::sfnt::Packet;
use crate::libcff::CffDictOperator;
use crate::libcff::charset::{CffCharset, CffCharsetRangeFormat2};
use crate::libcff::dict::{CffDict, CffDictEntry};
use crate::libcff::fdselect::{CffFdSelect, CffFdSelectRangeFormat3};
use crate::libcff::index::{CffIndex, CffIndexCountType};
use crate::libcff::value::CffValue;
use crate::libcff::charstring_il::CffCharstringIl;
use crate::libcff::subr::CffSubrGraph;
use crate::libcff::{
    CffFile, CffStack, OP_BLUE_FUZZ, OP_BLUE_SCALE, OP_BLUE_SHIFT, OP_BLUE_VALUES, OP_CHAR_STRINGS,
    TYPE2_TRANSIENT_ARRAY,
    OP_CHARSET, OP_CID_COUNT, OP_CID_FONT_REVISION, OP_CID_FONT_VERSION, OP_COPYRIGHT,
    OP_DEFAULT_WIDTH_X, OP_EXPANSION_FACTOR, OP_FAMILY_BLUES, OP_FAMILY_NAME,
    OP_FAMILY_OTHER_BLUES, OP_FD_ARRAY, OP_FD_SELECT, OP_FONT_BBOX, OP_FONT_MATRIX, OP_FONT_NAME,
    OP_FORCE_BOLD, OP_FULL_NAME, OP_INITIAL_RANDOM_SEED, OP_IS_FIXED_PITCH, OP_ITALIC_ANGLE,
    OP_LANGUAGE_GROUP, OP_NOMINAL_WIDTH_X, OP_NOTICE, OP_OTHER_BLUES, OP_PRIVATE, OP_ROS,
    OP_STD_HW, OP_STD_VW, OP_STEM_SNAP_H, OP_STEM_SNAP_V, OP_STROKE_WIDTH, OP_SUBRS, OP_UID_BASE,
    OP_UNDERLINE_POSITION, OP_UNDERLINE_THICKNESS, OP_VERSION, OP_WEIGHT,
};
use otfcc_binary::Buffer;
use crate::support::options::Options;
use crate::support::primitives::{Arity, CffSid, GlyphId, Pos, Scale, ShapeId, TableId};
use crate::table::glyf::{
    Contour, GlyfTable, Glyph, MaskList, Point, PostscriptHintMask, PostscriptStemDef, StemDefList,
};
use crate::table::head::HeadTable;
use otfcc_json::JsonType;

use otfcc_json::ParsedValue;
use crate::vf::vq::VQ;

use crate::libcff::charset::cff_build_charset;
use crate::libcff::codecs::cff_encode_cff_operator;
use crate::libcff::dict::{build_dict, parse_to_callback};
use crate::libcff::fdselect::cff_build_fd_select;
use crate::libcff::index::{build_index, new_empty_cff_index, new_index_by_callback};
use crate::libcff::parser::{cff_open_stream, cff_parse_outline, cff_parse_subr};
use crate::libcff::string::get_cff_sid;
use crate::libcff::value::cffnum;
use crate::libcff::writer::{cff_build_header, cff_build_offset};
use crate::libcff::charstring_il::{cff_compile_glyph_to_il, cff_optimize_il};
use crate::libcff::subr::{
    cff_il_graph_to_buffers, cff_insert_il_to_graph, cff_subr_graph_dispose, cff_subr_graph_init,
};
use otfcc_json::BuiltValue;
use crate::support::primitives::{from_fixed, to_fixed, until_nul};
use crate::table::fvar::json_new_vq;
use crate::table::glyf::{StemMask, new_glyf_glyph, table_glyf_create_n};
use crate::vf::vq::{
    vq_compare, vq_create_still, vq_get_still, vq_inplace_plus, vq_neutral, vq_point_linear_tfm,
    vq_scale,
};

#[derive(Clone, Debug)]
pub struct CffFontMatrix {
    pub a: Scale,
    pub b: Scale,
    pub c: Scale,
    pub d: Scale,
    pub x: VQ,
    pub y: VQ,
}
#[derive(Debug)]
pub struct CffPrivateDict {
    pub blue_values: Vec<f64>,
    pub other_blues: Vec<f64>,
    pub family_blues: Vec<f64>,
    pub family_other_blues: Vec<f64>,
    pub blue_scale: f64,
    pub blue_shift: f64,
    pub blue_fuzz: f64,
    pub std_hw: f64,
    pub std_vw: f64,
    pub stem_snap_h: Vec<f64>,
    pub stem_snap_v: Vec<f64>,
    pub force_bold: bool,
    pub language_group: u32,
    pub expansion_factor: f64,
    pub initial_random_seed: f64,
    pub default_width_x: f64,
    pub nominal_width_x: f64,
}
#[derive(Debug)]
pub struct CffTable {
    pub font_name: Vec<u8>,
    pub is_cid: bool,
    pub version: Vec<u8>,
    pub notice: Vec<u8>,
    pub copyright: Vec<u8>,
    pub full_name: Vec<u8>,
    pub family_name: Vec<u8>,
    pub weight: Vec<u8>,
    pub is_fixed_pitch: bool,
    pub italic_angle: f64,
    pub underline_position: f64,
    pub underline_thickness: f64,
    pub font_b_box_top: f64,
    pub font_b_box_bottom: f64,
    pub font_b_box_left: f64,
    pub font_b_box_right: f64,
    pub stroke_width: f64,
    pub private_dict: Option<Box<CffPrivateDict>>,
    pub font_matrix: Option<Box<CffFontMatrix>>,
    pub cid_registry: Vec<u8>,
    pub cid_ordering: Vec<u8>,
    pub cid_supplement: u32,
    pub cid_font_version: f64,
    pub cid_font_revision: f64,
    pub cid_count: u32,
    pub uid_base: u32,
    pub fd_array: Vec<Box<CffTable>>,
}
// Reading and writing use different types: reading produces an owned pair,
// writing borrows the `Font`'s own fields.
/// The owned result of reading a `CFF ` table (`read_cff_and_glyf_tables`).
/// Both fields are `None` when the packet has no `CFF ` table, or its Top
/// DICT INDEX is empty.
#[derive(Default, Debug)]
pub struct CffAndGlyfOwned {
    pub meta: Option<Box<CffTable>>,
    pub glyphs: Option<GlyfTable>,
}
/// The borrowed view `build_cff` needs: a `Font`'s own `cff`/`glyf`
/// fields, reborrowed for the duration of one build. `meta` is `&mut`
/// (`writecff_cid_keyed` mutates it -- `cff_compile_nameindex` clears
/// `font_name` once it's been written out) and required, matching every
/// call site's existing assumption that a CFF-subtype font has a CFF
/// table (previously an unchecked null deref if it didn't; now an
/// explicit, documented one at the one call site that builds this).
/// `glyphs` is `Option<&GlyfTable>` -- a font with a `CFF_` table but no
/// `glyf` table at all is a real, reachable case (`writecff_cid_keyed`
/// substitutes a local empty `GlyfTable` for `None`), and nothing here
/// ever mutates it, so a shared borrow suffices.
#[derive(Debug)]
pub struct CffAndGlyfRef<'a> {
    pub meta: &'a mut CffTable,
    pub glyphs: Option<&'a GlyfTable>,
}
// State for the Top/Font/Private DICT extraction phase. The glyphs do not
// exist yet in this phase; `read_cff_and_glyf_tables` builds them afterwards.
#[derive(Debug)]
struct CffFdExtractContext<'a> {
    fd_array_index: i32,
    meta: &'a mut CffTable,
    cff_file: &'a CffFile,
}
#[derive(Debug)]
pub struct OutlineBuilderContext<'a> {
    pub g: &'a mut Glyph,
    pub j_contour: ShapeId,
    pub j_point: ShapeId,
    pub default_width_x: f64,
    pub nominal_width_x: f64,
    pub defined_h_stems: u8,
    pub defined_v_stems: u8,
    pub defined_hint_masks: u8,
    pub defined_contour_masks: u8,
    pub randx: u64,
}
#[derive(Debug)]
pub struct CffCharstringBuilderContext<'a> {
    pub glyf: &'a GlyfTable,
    pub default_width: u16,
    pub nominal_width_x: u16,
    pub options: &'a Options,
    pub graph: CffSubrGraph,
}
pub static DEFAULT_BLUE_SCALE: f64 = 0.039625f64;
pub static DEFAULT_BLUE_SHIFT: f64 =
    7_f64;
pub static DEFAULT_BLUE_FUZZ: f64 =
    1_f64;
pub static DEFAULT_EXPANSION_FACTOR: f64 = 0.06f64;
fn new_cff_private() -> Box<CffPrivateDict> {
    Box::new(CffPrivateDict {
        blue_values: Vec::new(),
        other_blues: Vec::new(),
        family_blues: Vec::new(),
        family_other_blues: Vec::new(),
        blue_scale: DEFAULT_BLUE_SCALE,
        blue_shift: DEFAULT_BLUE_SHIFT,
        blue_fuzz: DEFAULT_BLUE_FUZZ,
        std_hw: 0.,
        std_vw: 0.,
        stem_snap_h: Vec::new(),
        stem_snap_v: Vec::new(),
        force_bold: false,
        language_group: 0,
        expansion_factor: DEFAULT_EXPANSION_FACTOR,
        initial_random_seed: 0.,
        default_width_x: 0.,
        nominal_width_x: 0.,
    })
}
fn table_cff_new() -> Box<CffTable> {
    Box::new(CffTable {
        font_name: Vec::new(),
        is_cid: false,
        version: Vec::new(),
        notice: Vec::new(),
        copyright: Vec::new(),
        full_name: Vec::new(),
        family_name: Vec::new(),
        weight: Vec::new(),
        is_fixed_pitch: false,
        italic_angle: 0 as f64,
        underline_position: -100_f64,
        underline_thickness: 50_f64,
        font_b_box_top: 0 as f64,
        font_b_box_bottom: 0 as f64,
        font_b_box_left: 0 as f64,
        font_b_box_right: 0 as f64,
        stroke_width: 0 as f64,
        private_dict: None,
        font_matrix: None,
        cid_registry: Vec::new(),
        cid_ordering: Vec::new(),
        cid_supplement: 0,
        cid_font_version: 0 as f64,
        cid_font_revision: 0 as f64,
        cid_count: 0,
        uid_base: 0,
        fd_array: Vec::new(),
    })
}
fn callback_extract_private(op: CffDictOperator, top: u8, stack: &[CffValue], context: &mut CffFdExtractContext) {
    let meta: &mut CffTable = if context.fd_array_index >= 0
        && (context.fd_array_index as usize) < context.meta.fd_array.len()
    {
        context.meta.fd_array[context.fd_array_index as usize].as_mut()
    } else {
        &mut *context.meta
    };
    let pd: &mut CffPrivateDict = meta.private_dict.as_deref_mut().unwrap();
    match op.0 {
        6 => {
            pd.blue_values = (0..top as Arity)
                .map(|j| cffnum(stack[j as usize]))
                .collect();
        }
        7 => {
            pd.other_blues = (0..top as Arity)
                .map(|j| cffnum(stack[j as usize]))
                .collect();
        }
        8 => {
            pd.family_blues = (0..top as Arity)
                .map(|j| cffnum(stack[j as usize]))
                .collect();
        }
        9 => {
            pd.family_other_blues = (0..top as Arity)
                .map(|j| cffnum(stack[j as usize]))
                .collect();
        }
        3084 => {
            pd.stem_snap_h = (0..top as Arity)
                .map(|j| cffnum(stack[j as usize]))
                .collect();
        }
        3085 => {
            pd.stem_snap_v = (0..top as Arity)
                .map(|j| cffnum(stack[j as usize]))
                .collect();
        }
        3081 => {
            if top != 0 {
                pd.blue_scale = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        3082 => {
            if top != 0 {
                pd.blue_shift = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        3083 => {
            if top != 0 {
                pd.blue_fuzz = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        10 => {
            if top != 0 {
                pd.std_hw = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        11 => {
            if top != 0 {
                pd.std_vw = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        3086 => {
            if top != 0 {
                pd.force_bold = cffnum(
                    stack[top as usize - 1],
                ) != 0.;
            }
        }
        3089 => {
            if top != 0 {
                pd.language_group = cffnum(
                    stack[top as usize - 1],
                ) as u32;
            }
        }
        3090 => {
            if top != 0 {
                pd.expansion_factor = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        3091 => {
            if top != 0 {
                pd.initial_random_seed = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        20 => {
            if top != 0 {
                pd.default_width_x = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        21
            if top != 0 => {
                pd.nominal_width_x = cffnum(
                    stack[top as usize - 1],
                );
            }
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
    match op.0 {
        0 => {
            if top != 0 {
                meta.version = get_cff_sid(
                    cffnum(
                        stack[top as usize - 1],
                    ) as u16,
                    &file.string,
                )
                .unwrap_or_default();
            }
        }
        1 => {
            if top != 0 {
                meta.notice = get_cff_sid(
                    cffnum(
                        stack[top as usize - 1],
                    ) as u16,
                    &file.string,
                )
                .unwrap_or_default();
            }
        }
        3072 => {
            if top != 0 {
                meta.copyright = get_cff_sid(
                    cffnum(
                        stack[top as usize - 1],
                    ) as u16,
                    &file.string,
                )
                .unwrap_or_default();
            }
        }
        3110 => {
            if top != 0 {
                meta.font_name = get_cff_sid(
                    cffnum(
                        stack[top as usize - 1],
                    ) as u16,
                    &file.string,
                )
                .unwrap_or_default();
            }
        }
        2 => {
            if top != 0 {
                meta.full_name = get_cff_sid(
                    cffnum(
                        stack[top as usize - 1],
                    ) as u16,
                    &file.string,
                )
                .unwrap_or_default();
            }
        }
        3 => {
            if top != 0 {
                meta.family_name = get_cff_sid(
                    cffnum(
                        stack[top as usize - 1],
                    ) as u16,
                    &file.string,
                )
                .unwrap_or_default();
            }
        }
        4 => {
            if top != 0 {
                meta.weight = get_cff_sid(
                    cffnum(
                        stack[top as usize - 1],
                    ) as u16,
                    &file.string,
                )
                .unwrap_or_default();
            }
        }
        5 => {
            if top >= 4 {
                meta.font_b_box_left = cffnum(
                    stack[top as usize - 4],
                );
                meta.font_b_box_bottom = cffnum(
                    stack[top as usize - 3],
                );
                meta.font_b_box_right = cffnum(
                    stack[top as usize - 2],
                );
                meta.font_b_box_top = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        3079 => {
            if top >= 6 {
                meta.font_matrix = Some(Box::new(CffFontMatrix {
                    a: 0.,
                    b: 0.,
                    c: 0.,
                    d: 0.,
                    x: vq_neutral(),
                    y: vq_neutral(),
                }));
                let fm: &mut CffFontMatrix = meta.font_matrix.as_deref_mut().unwrap();
                fm.a = cffnum(
                    stack[top as usize - 6],
                ) as Scale;
                fm.b = cffnum(
                    stack[top as usize - 5],
                ) as Scale;
                fm.c = cffnum(
                    stack[top as usize - 4],
                ) as Scale;
                fm.d = cffnum(
                    stack[top as usize - 3],
                ) as Scale;
                fm.x = vq_create_still(cffnum(
                    stack[top as usize - 2],
                ) as Pos);
                fm.y = vq_create_still(cffnum(
                    stack[top as usize - 1],
                ) as Pos);
            }
        }
        3073 => {
            if top != 0 {
                meta.is_fixed_pitch = cffnum(
                    stack[top as usize - 1],
                ) != 0.;
            }
        }
        3074 => {
            if top != 0 {
                meta.italic_angle = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        3075 => {
            if top != 0 {
                meta.underline_position = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        3076 => {
            if top != 0 {
                meta.underline_thickness = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        3080 => {
            if top != 0 {
                meta.stroke_width = cffnum(
                    stack[top as usize - 1],
                );
            }
        }
        18 => {
            if top >= 2 {
                let private_length: u32 = cffnum(
                    stack[top as usize - 2],
                ) as u32;
                let private_offset: u32 = cffnum(
                    stack[top as usize - 1],
                ) as u32;
                meta.private_dict = Some(new_cff_private());
                // The Private DICT's offset and length come from the font;
                // skip the DICT, keeping the default `private_dict`, when
                // they do not fit in the table.
                let raw_slice = file.raw_data.as_slice();
                if let Some(private_bytes) = raw_slice
                    .get(private_offset as usize..)
                    .and_then(|s| s.get(..private_length as usize))
                {
                    // `meta`'s last use was the assignment above -- its
                    // (and thus `context.meta`'s) borrow has already ended
                    // here, so this fresh reborrow of `context` as a whole
                    // is sound under NLL.
                    parse_to_callback(private_bytes, |op, top, stack| {
                        callback_extract_private(op, top, stack, context);
                    });
                }
            }
        }
        3102
            if top >= 3 => {
                meta.is_cid = true;
                meta.cid_registry = get_cff_sid(
                    cffnum(
                        stack[top as usize - 3],
                    ) as u16,
                    &file.string,
                )
                .unwrap_or_default();
                meta.cid_ordering = get_cff_sid(
                    cffnum(
                        stack[top as usize - 2],
                    ) as u16,
                    &file.string,
                )
                .unwrap_or_default();
                meta.cid_supplement = cffnum(
                    stack[top as usize - 1],
                ) as u32;
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
        mask.contours_before = (context.j_contour as i32 - 1_i32) as u16;
    } else {
        mask.contours_before = 0_u16;
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
            context.defined_contour_masks = (context.defined_contour_masks as i32 + 1_i32) as u8;
        } else {
            context.defined_hint_masks = (context.defined_hint_masks as i32 + 1_i32) as u8;
        }
    };
}
pub(crate) fn callback_draw_getrand(context: &mut OutlineBuilderContext) -> f64 {
    let mut x: u64 = context.randx;
    x ^= x >> 12_i32;
    x ^= x << 25_i32;
    x ^= x >> 27_i32;
    context.randx = x;
    // xorshift, then put the bits into an f64's mantissa with an exponent
    // of 0 to get a uniform double in [1, 2), and subtract 1.
    let mut bits: u64 = x.wrapping_mul(2685821657736338717_u64);
    bits = bits >> 12_i32 | 0x3ff0000000000000_u64;
    let q: f64 = if bits & 2048_u64 != 0 {
        1.0f64 - 2.220_446_049_250_313E-16_f64 / 2.0f64
    } else {
        1.0f64
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
        default_width_x: 0.0f64,
        nominal_width_x: 0.0f64,
        defined_h_stems: 0_u8,
        defined_v_stems: 0_u8,
        defined_hint_masks: 0_u8,
        defined_contour_masks: 0_u8,
        randx: 0_u64,
    };
    let f_raw_data = f.raw_data.as_slice();
    let fd: u8 = if !matches!(f.fdselect, CffFdSelect::Unspecified) {
        cff_parse_subr(
            i,
            f_raw_data,
            &f.font_dict,
            &f.fdselect,
            &mut local_subrs,
        )
    } else {
        cff_parse_subr(
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
    // CFF INDEX offsets are 1-based and `extract_index` already validated
    // this whole array (non-decreasing, every entry >= 1, and the final
    // entry exactly matches `data.len() + 1`) -- so `offset[i] - 1` is
    // always in `0..=data.len()` for any `i` within the INDEX's own
    // count, the same invariant `locate_subr`/`get_cff_sid` rely on.
    let char_string_start = (char_strings_offset[i as usize] - 1_u32) as usize;
    let char_string_end = (char_strings_offset[(i as i32 + 1_i32) as usize] - 1_u32) as usize;
    let char_string_bytes: &[u8] = &f.char_strings.data[char_string_start..char_string_end];
    bc.j_contour = 0 as ShapeId;
    bc.j_point = 0 as ShapeId;
    bc.randx = seed_val;
    let mut total_subr_calls: u32 = 0;
    cff_parse_outline(
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
fn apply_cff_matrix(cff: &CffTable, glyf: &mut GlyfTable, head: Option<&HeadTable>) {
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
        let cff_file: Box<CffFile> = cff_open_stream(&table.data);
        // A Top DICT INDEX with a count of 0 has no offsets to read.
        if cff_file.top_dict.count != 0 {
            let mut meta: Box<CffTable> = table_cff_new();

            // ---- Phase A: Top/Font DICT + Private DICT extraction ----
            {
                let mut context = CffFdExtractContext {
                    fd_array_index: -1_i32,
                    meta: &mut meta,
                    cff_file: &cff_file,
                };
                parse_to_callback(
                    {
                        let top_dict_len = {
                            let top_dict_offset = &cff_file.top_dict.offset;
                            (top_dict_offset[1_usize])
                                .wrapping_sub(top_dict_offset[0_usize])
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
                        get_cff_sid(391_u16, &cff_file.name).unwrap_or_default();
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
                                    + 1_i32)
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

            let mut seed: u64 = 0x1234567887654321_u64;
            if let Some(pd) = meta.private_dict.as_deref() {
                seed = pd.initial_random_seed as u64 ^ 0x1234567887654321_u64;
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
fn pd_delta_to_json(target: &mut BuiltValue, field: &[u8], values: &[f64]) {
    if values.is_empty() {
        return;
    }
    let mut a = BuiltValue::new_array(values.len());
    for &x in values {
        a.push_item(BuiltValue::Double(x));
    }
    target.push_field(field, a);
}
fn pd_to_json(pd: &CffPrivateDict) -> BuiltValue {
    let mut _pd = BuiltValue::new_object(24);
    pd_delta_to_json(&mut _pd, b"blueValues", &pd.blue_values);
    pd_delta_to_json(&mut _pd, b"otherBlues", &pd.other_blues);
    pd_delta_to_json(&mut _pd, b"familyBlues", &pd.family_blues);
    pd_delta_to_json(&mut _pd, b"familyOtherBlues", &pd.family_other_blues);
    pd_delta_to_json(&mut _pd, b"stemSnapH", &pd.stem_snap_h);
    pd_delta_to_json(&mut _pd, b"stemSnapV", &pd.stem_snap_v);
    if pd.blue_scale != DEFAULT_BLUE_SCALE {
        _pd.push_field(b"blueScale", BuiltValue::Double(pd.blue_scale));
    }
    if pd.blue_shift != DEFAULT_BLUE_SHIFT {
        _pd.push_field(b"blueShift", BuiltValue::Double(pd.blue_shift));
    }
    if pd.blue_fuzz != DEFAULT_BLUE_FUZZ {
        _pd.push_field(b"blueFuzz", BuiltValue::Double(pd.blue_fuzz));
    }
    if pd.std_hw != 0. {
        _pd.push_field(b"stdHW", BuiltValue::Double(pd.std_hw));
    }
    if pd.std_vw != 0. {
        _pd.push_field(b"stdVW", BuiltValue::Double(pd.std_vw));
    }
    if pd.force_bold {
        _pd.push_field(b"forceBold", BuiltValue::Bool(pd.force_bold));
    }
    if pd.language_group != 0 {
        _pd.push_field(
            b"languageGroup",
            BuiltValue::Double(pd.language_group as f64),
        );
    }
    if pd.expansion_factor != DEFAULT_EXPANSION_FACTOR {
        _pd.push_field(b"expansionFactor", BuiltValue::Double(pd.expansion_factor));
    }
    if pd.initial_random_seed != 0. {
        _pd.push_field(
            b"initialRandomSeed",
            BuiltValue::Double(pd.initial_random_seed),
        );
    }
    if pd.default_width_x != 0. {
        _pd.push_field(b"defaultWidthX", BuiltValue::Double(pd.default_width_x));
    }
    if pd.nominal_width_x != 0. {
        _pd.push_field(b"nominalWidthX", BuiltValue::Double(pd.nominal_width_x));
    }
    _pd
}
fn fd_to_json(table: &CffTable) -> BuiltValue {
    let mut _cff = BuiltValue::new_object(24);
    if table.is_cid {
        _cff.push_field(b"isCID", BuiltValue::Bool(table.is_cid));
    }
    if !table.version.is_empty() {
        _cff.push_field(b"version", json_from_sds(&table.version));
    }
    if !table.notice.is_empty() {
        _cff.push_field(b"notice", json_from_sds(&table.notice));
    }
    if !table.copyright.is_empty() {
        _cff.push_field(b"copyright", json_from_sds(&table.copyright));
    }
    if !table.font_name.is_empty() {
        _cff.push_field(b"fontName", json_from_sds(&table.font_name));
    }
    if !table.full_name.is_empty() {
        _cff.push_field(b"fullName", json_from_sds(&table.full_name));
    }
    if !table.family_name.is_empty() {
        _cff.push_field(b"familyName", json_from_sds(&table.family_name));
    }
    if !table.weight.is_empty() {
        _cff.push_field(b"weight", json_from_sds(&table.weight));
    }
    if table.is_fixed_pitch {
        _cff.push_field(b"isFixedPitch", BuiltValue::Bool(table.is_fixed_pitch));
    }
    if table.italic_angle != 0. {
        _cff.push_field(b"italicAngle", BuiltValue::Double(table.italic_angle));
    }
    if table.underline_position != -100_f64 {
        _cff.push_field(
            b"underlinePosition",
            BuiltValue::Double(table.underline_position),
        );
    }
    if table.underline_thickness != 50_f64 {
        _cff.push_field(
            b"underlineThickness",
            BuiltValue::Double(table.underline_thickness),
        );
    }
    if table.stroke_width != 0. {
        _cff.push_field(b"strokeWidth", BuiltValue::Double(table.stroke_width));
    }
    if table.font_b_box_left != 0. {
        _cff.push_field(b"fontBBoxLeft", BuiltValue::Double(table.font_b_box_left));
    }
    if table.font_b_box_bottom != 0. {
        _cff.push_field(
            b"fontBBoxBottom",
            BuiltValue::Double(table.font_b_box_bottom),
        );
    }
    if table.font_b_box_right != 0. {
        _cff.push_field(b"fontBBoxRight", BuiltValue::Double(table.font_b_box_right));
    }
    if table.font_b_box_top != 0. {
        _cff.push_field(b"fontBBoxTop", BuiltValue::Double(table.font_b_box_top));
    }
    if let Some(fm) = table.font_matrix.as_deref() {
        let mut _font_matrix = BuiltValue::new_object(6);
        _font_matrix.push_field(b"a", BuiltValue::Double(fm.a));
        _font_matrix.push_field(b"b", BuiltValue::Double(fm.b));
        _font_matrix.push_field(b"c", BuiltValue::Double(fm.c));
        _font_matrix.push_field(b"d", BuiltValue::Double(fm.d));
        // CFF fonts have no `fvar` table (no `Delta` segments can occur
        // here -- `json_new_vq` panics if that invariant is ever wrong).
        _font_matrix.push_field(b"x", json_new_vq(fm.x.clone(), None));
        _font_matrix.push_field(b"y", json_new_vq(fm.y.clone(), None));
        _cff.push_field(b"fontMatrix", _font_matrix);
    }
    if let Some(pd) = table.private_dict.as_deref() {
        _cff.push_field(b"privates", pd_to_json(pd));
    }
    if !table.cid_registry.is_empty() && !table.cid_ordering.is_empty() {
        _cff.push_field(b"cidRegistry", json_from_sds(&table.cid_registry));
        _cff.push_field(b"cidOrdering", json_from_sds(&table.cid_ordering));
        _cff.push_field(
            b"cidSupplement",
            BuiltValue::Int(table.cid_supplement as i64),
        );
    }
    if !table.fd_array.is_empty() {
        let mut _fd_array = BuiltValue::new_object(table.fd_array.len());
        for fd in &table.fd_array {
            // An FD's name is already its key in `fdArray`, so drop the
            // `fontName` field from its object.
            let mut child = fd_to_json(fd);
            if let BuiltValue::Object(fields) = &mut child {
                fields.retain(|(k, _)| k != b"fontName");
            }
            _fd_array.push_field_bytes_key(&fd.font_name, child);
        }
        _cff.push_field(b"fdArray", _fd_array);
    }
    _cff
}
pub fn dump_cff(table: Option<&CffTable>, root: &mut BuiltValue) {
    let Some(table) = table else {
        return;
    };
    let stage = crate::logger::stage("CFF");
    root.push_field(b"CFF_", fd_to_json(table));
    stage.finish();
}
fn pd_delta_from_json(dump: Option<&ParsedValue>) -> Vec<f64> {
    let Some(items) = dump.and_then(ParsedValue::as_array) else {
        return Vec::new();
    };
    items.iter().map(|v| v.as_num().unwrap_or(0.0)).collect()
}
fn pd_from_json(dump: Option<&ParsedValue>) -> Option<Box<CffPrivateDict>> {
    let dump = dump.filter(|v| v.as_object().is_some())?;
    let mut pd_box: Box<CffPrivateDict> = new_cff_private();
    pd_box.blue_values = pd_delta_from_json(dump.get(b"blueValues"));
    pd_box.other_blues = pd_delta_from_json(dump.get(b"otherBlues"));
    pd_box.family_blues = pd_delta_from_json(dump.get(b"familyBlues"));
    pd_box.family_other_blues = pd_delta_from_json(dump.get(b"familyOtherBlues"));
    pd_box.stem_snap_h = pd_delta_from_json(dump.get(b"stemSnapH"));
    pd_box.stem_snap_v = pd_delta_from_json(dump.get(b"stemSnapV"));
    pd_box.blue_scale = dump.get_num_or(b"blueScale", DEFAULT_BLUE_SCALE);
    pd_box.blue_shift = dump.get_num_or(b"blueShift", DEFAULT_BLUE_SHIFT);
    pd_box.blue_fuzz = dump.get_num_or(b"blueFuzz", DEFAULT_BLUE_FUZZ);
    pd_box.std_hw = dump.get_num(b"stdHW");
    pd_box.std_vw = dump.get_num(b"stdVW");
    pd_box.force_bold = dump.get_bool(b"forceBold");
    pd_box.language_group = dump.get_num(b"languageGroup") as u32;
    pd_box.expansion_factor = dump.get_num_or(b"expansionFactor", DEFAULT_EXPANSION_FACTOR);
    pd_box.initial_random_seed = dump.get_num(b"initialRandomSeed");
    Some(pd_box)
}
// Builds the whole tree of `CffTable`/`fd_array` children as owned local
// values first, `Box`ing each one only once it's fully populated.
fn fd_from_json(dump: Option<&ParsedValue>, options: &Options, top_level: bool) -> Box<CffTable> {
    let mut table = table_cff_new();
    let Some(dump) = dump.filter(|v| v.as_object().is_some()) else {
        return table;
    };
    table.version = dump.get_bytes_owned(b"version").unwrap_or_default();
    table.notice = dump.get_bytes_owned(b"notice").unwrap_or_default();
    table.copyright = dump.get_bytes_owned(b"copyright").unwrap_or_default();
    table.font_name = dump.get_bytes_owned(b"fontName").unwrap_or_default();
    table.full_name = dump.get_bytes_owned(b"fullName").unwrap_or_default();
    table.family_name = dump.get_bytes_owned(b"familyName").unwrap_or_default();
    table.weight = dump.get_bytes_owned(b"weight").unwrap_or_default();
    table.is_fixed_pitch = dump.get_bool(b"isFixedPitch");
    table.italic_angle = dump.get_num(b"italicAngle");
    table.underline_position = dump.get_num_or(b"underlinePosition", -100.0f64);
    table.underline_thickness = dump.get_num_or(b"underlineThickness", 50.0f64);
    table.stroke_width = dump.get_num(b"strokeWidth");
    table.font_b_box_left = dump.get_num(b"fontBBoxLeft");
    table.font_b_box_bottom = dump.get_num(b"fontBBoxBottom");
    table.font_b_box_right = dump.get_num(b"fontBBoxRight");
    table.font_b_box_top = dump.get_num(b"fontBBoxTop");
    table.private_dict = pd_from_json(dump.get_typed(b"privates", JsonType::Object));
    table.cid_registry = dump.get_bytes_owned(b"cidRegistry").unwrap_or_default();
    table.cid_ordering = dump.get_bytes_owned(b"cidOrdering").unwrap_or_default();
    table.cid_supplement = dump.get_int(b"cidSupplement") as u32;
    table.uid_base = dump.get_int(b"UIDBase") as u32;
    table.cid_count = dump.get_int(b"cidCount") as u32;
    table.cid_font_version = dump.get_num(b"cidFontVersion");
    table.cid_font_revision = dump.get_num(b"cidFontRevision");
    if let Some(fields) = dump
        .get_typed(b"fdArray", JsonType::Object)
        .and_then(ParsedValue::as_object)
    {
        table.is_cid = true;
        table.fd_array = Vec::with_capacity(fields.len());
        for (key, val) in fields {
            // `fd_from_json` builds each child fully before returning
            // (unlike the binary-read path, which populates a `fd_array`
            // slot incrementally via a recursive callback) -- so there's
            // no need to push an empty placeholder first here.
            let mut fd_box: Box<CffTable> = fd_from_json(Some(val), options, false);
            fd_box.font_name = key[..key.len() - 1].to_vec();
            table.fd_array.push(fd_box);
        }
    }
    if table.font_name.is_empty() {
        table.font_name = b"CARYLL_CFFFONT".to_vec();
    }
    if table.private_dict.is_none() {
        table.private_dict = Some(new_cff_private());
    }
    if top_level && options.force_cid && table.fd_array.is_empty() {
        let mut fd0_box: Box<CffTable> = table_cff_new();
        fd0_box.private_dict = table.private_dict.take();
        table.private_dict = Some(new_cff_private());
        let mut subfont0_name = table.font_name.clone();
        subfont0_name.extend_from_slice(b"-subfont0");
        fd0_box.font_name = subfont0_name;
        table.fd_array.push(fd0_box);
        table.is_cid = true;
    }
    if table.is_cid && table.cid_registry.is_empty() {
        table.cid_registry = b"CARYLL".to_vec();
    }
    if table.is_cid && table.cid_ordering.is_empty() {
        table.cid_ordering = b"OTFCCAUTOCID".to_vec();
    }
    table
}
pub fn parse_cff(root: &ParsedValue, options: &Options) -> Option<Box<CffTable>> {
    let dump = root.get_typed(b"CFF_", JsonType::Object)?;
    let stage = crate::logger::stage("CFF");
    let cff = fd_from_json(Some(dump), options, true);
    stage.finish();
    Some(cff)
}
fn cff_make_charstrings(context: &mut CffCharstringBuilderContext) -> (Buffer, Buffer, Buffer) {
    let glyf: &GlyfTable = context.glyf;
    if glyf.is_empty() {
        // With 0 glyphs, `cff_il_graph_to_buffers` below never runs, so
        // the caller (`writecff_cid_keyed`) still needs three empty (but
        // real) `Buffer`s here.
        return (Buffer::new(), Buffer::new(), Buffer::new());
    }
    let options: &Options = context.options;
    for entry in glyf.iter() {
        let mut il: CffCharstringIl = cff_compile_glyph_to_il(
            entry.as_deref().unwrap(),
            context.default_width,
            context.nominal_width_x,
        );
        cff_optimize_il(&mut il, options);
        cff_insert_il_to_graph(&mut context.graph, &il);
    }
    cff_il_graph_to_buffers(&mut context.graph)
}
// Returns the SID of `s`, registering it if new: strings get SIDs from 391
// up in the order they are first seen, and the `IndexMap` keeps that order
// for writing. Strings are compared up to their first NUL, but the first
// one registered is stored, and written, in full.
fn sidof(h: &mut indexmap::IndexMap<Vec<u8>, Vec<u8>>, s: &[u8]) -> i32 {
    let key: Vec<u8> = until_nul(s).to_vec();
    if let Some(idx) = h.get_index_of(&key) {
        return 391_i32 + idx as i32;
    }
    let idx = h.len();
    h.insert(key, s.to_vec());
    return 391_i32 + idx as i32;
}
fn cffdict_givemeablank(dict: &mut CffDict) -> &mut CffDictEntry {
    dict.ents.push(CffDictEntry {
        op: CffDictOperator(0),
        vals: Vec::new(),
    });
    dict.ents.last_mut().unwrap()
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
    cffdict_input_ints(&mut dict, OP_IS_FIXED_PITCH, &[fd.is_fixed_pitch as i32]);
    cffdict_input_doubles(&mut dict, OP_ITALIC_ANGLE, &[fd.italic_angle]);
    cffdict_input_doubles(&mut dict, OP_UNDERLINE_POSITION, &[fd.underline_position]);
    cffdict_input_doubles(
        &mut dict,
        OP_UNDERLINE_THICKNESS,
        &[fd.underline_thickness],
    );
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
    cffdict_input_ints(&mut dict, OP_FORCE_BOLD, &[pd.force_bold as i32]);
    cffdict_input_ints(&mut dict, OP_LANGUAGE_GROUP, &[pd.language_group as i32]);
    cffdict_input_doubles(&mut dict, OP_EXPANSION_FACTOR, &[pd.expansion_factor]);
    cffdict_input_doubles(&mut dict, OP_INITIAL_RANDOM_SEED, &[pd.initial_random_seed]);
    cffdict_input_doubles(&mut dict, OP_DEFAULT_WIDTH_X, &[pd.default_width_x]);
    cffdict_input_doubles(&mut dict, OP_NOMINAL_WIDTH_X, &[pd.nominal_width_x]);
    dict
}
fn cffstrings_to_indexblob(h: &mut indexmap::IndexMap<Vec<u8>, Vec<u8>>) -> Buffer {
    let n: u32 = h.len() as u32;
    // The `IndexMap` is in SID order (see `sidof`).
    let blobs: Vec<Buffer> = ::core::mem::take(h)
        .into_iter()
        .map(|(_, value)| Buffer::from_bytes(&value))
        .collect();
    let strings: CffIndex = new_index_by_callback(n, blobs.into_iter());
    build_index(&strings)
}
fn cff_compile_nameindex(cff: &mut CffTable) -> Buffer {
    if cff.font_name.is_empty() {
        cff.font_name = b"Caryll-CFF-FONT".to_vec();
    }
    let mut name_index = new_empty_cff_index();
    name_index.count = 1 as Arity;
    name_index.off_size = 4_u8;
    name_index.offset = vec![1, cff.font_name.len() as u32 + 1];
    // The name is written with a trailing NUL.
    let mut name_data: Vec<u8> = cff.font_name.clone();
    name_data.push(0_u8);
    name_index.data = name_data;
    let buf = build_index(&name_index);
    cff.font_name = Vec::new();
    buf
}
fn cff_make_charset(
    cff: &CffTable,
    glyf: &GlyfTable,
    string_hash: &mut indexmap::IndexMap<Vec<u8>, Vec<u8>>,
) -> Buffer {
    let charset: CffCharset = if glyf.len() > 1_usize {
        let (first, nleft) = if cff.is_cid {
            (1, (glyf.len() - 2) as u16)
        } else {
            for entry in glyf.iter().skip(1) {
                sidof(string_hash, &entry.as_deref().unwrap().name);
            }
            (
                sidof(string_hash, &glyf[1_usize].as_deref().unwrap().name) as u16,
                (glyf.len() - 2) as u16,
            )
        };
        CffCharset::Format2(vec![CffCharsetRangeFormat2 { first, nleft }])
    } else {
        CffCharset::IsoAdobe
    };
    return cff_build_charset(&charset);
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
        let mut fdi0: u8 = glyf[0_usize].as_deref().unwrap().fd_select.index as u8;
        if fdi0 as usize > cff.fd_array.len() {
            fdi0 = 0_u8;
        }
        let mut current: u8 = fdi0;
        let mut range3: Vec<CffFdSelectRangeFormat3> = vec![CffFdSelectRangeFormat3 {
            first: 0_u16,
            fd: current,
        }];
        for (j, entry) in glyf.iter().enumerate().skip(1) {
            let mut fdi: u8 = entry.as_deref().unwrap().fd_select.index as u8;
            if fdi as usize > cff.fd_array.len() {
                fdi = 0_u8;
            }
            if fdi as i32 != current as i32 {
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
    return cff_build_fd_select(&fds);
}
fn compile_fd_buffer(
    fd_array: &[Box<CffTable>],
    string_hash: &mut indexmap::IndexMap<Vec<u8>, Vec<u8>>,
    i: u32,
) -> Buffer {
    let fd: CffDict = cff_make_fd_dict(&fd_array[i as usize], string_hash);
    let mut blob: Buffer = build_dict(&fd);
    blob.write_buffer_owned(cff_build_offset(0xeeeeeeee_u32 as i32));
    blob.write_buffer_owned(cff_build_offset(0xffffffff_u32 as i32));
    blob.write_buffer_owned(cff_encode_cff_operator(OP_PRIVATE));
    blob
}
fn cff_make_fdarray(
    fd_array: &[Box<CffTable>],
    string_hash: &mut indexmap::IndexMap<Vec<u8>, Vec<u8>>,
) -> CffIndex {
    let len = fd_array.len() as u32;
    new_index_by_callback(len, (0..len).map(|i| compile_fd_buffer(fd_array, string_hash, i)))
}
fn writecff_cid_keyed(cff: &mut CffTable, glyf: Option<&GlyfTable>, options: &Options) -> Buffer {
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
    let h = cff_build_header();
    let n = cff_compile_nameindex(cff);
    let top: CffDict = cff_make_fd_dict(cff, &mut string_hash);
    let t = build_dict(&top);
    let top_pd: CffDict = cff_make_private_dict(cff.private_dict.as_deref());
    let mut p = build_dict(&top_pd);
    p.write_buffer_owned(cff_build_offset(0xffffffff_u32 as i32));
    p.write_buffer_owned(cff_encode_cff_operator(OP_SUBRS));
    let e = cff_make_fdselect(cff, glyf);
    let mut fd_array_index: Option<CffIndex> = None;
    let mut r: Buffer;
    if cff.is_cid {
        let idx = cff_make_fdarray(&cff.fd_array, &mut string_hash);
        r = build_index(&idx);
        fd_array_index = Some(idx);
    } else {
        r = Buffer::new();
    }
    let c = cff_make_charset(cff, glyf, &mut string_hash);
    let i = cffstrings_to_indexblob(&mut string_hash);
    let mut g2c_context: CffCharstringBuilderContext = CffCharstringBuilderContext {
        glyf,
        default_width: cff.private_dict.as_deref().unwrap().default_width_x as u16,
        nominal_width_x: cff.private_dict.as_deref().unwrap().nominal_width_x as u16,
        options,
        graph: CffSubrGraph::default(),
    };
    cff_subr_graph_init(&mut g2c_context.graph);
    g2c_context.graph.do_subroutinize = options.cff_do_subroutinize;
    let (s, gs, ls) = cff_make_charstrings(&mut g2c_context);
    cff_subr_graph_dispose(&mut g2c_context.graph);
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
    if !c.is_empty() {
        blob.write_buffer_owned(cff_build_offset(off as i32));
        blob.write_buffer_owned(cff_encode_cff_operator(OP_CHARSET));
        off += c.len() as u32;
    }
    if !e.is_empty() {
        blob.write_buffer_owned(cff_build_offset(off as i32));
        blob.write_buffer_owned(cff_encode_cff_operator(OP_FD_SELECT));
        off += e.len() as u32;
    }
    if !s.is_empty() {
        blob.write_buffer_owned(cff_build_offset(off as i32));
        blob.write_buffer_owned(cff_encode_cff_operator(OP_CHAR_STRINGS));
        off += s.len() as u32;
    }
    if !p.is_empty() {
        blob.write_buffer_owned(cff_build_offset(p.len() as i32));
        blob.write_buffer_owned(cff_build_offset(off as i32));
        blob.write_buffer_owned(cff_encode_cff_operator(OP_PRIVATE));
        off += p.len() as u32;
    }
    if !r.is_empty() {
        blob.write_buffer_owned(cff_build_offset(off as i32));
        blob.write_buffer_owned(cff_encode_cff_operator(OP_FD_ARRAY));
        off += r.len() as u32;
    }
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
        // `fd_array_index` is only ever `None` here when `cff.is_cid` is
        // false, so this `.unwrap()` can't fail -- same invariant the old
        // `*mut CffIndex` (null unless `is_cid`) encoded implicitly.
        let idx: &mut CffIndex = fd_array_index.as_mut().unwrap();
        let mut fd_array_privates_start_offset: u32 = off;
        let mut fd_array_privates: Vec<Buffer> = Vec::with_capacity(cff.fd_array.len());
        for (j, fd) in cff.fd_array.iter().enumerate() {
            let pd: CffDict = cff_make_private_dict(fd.private_dict.as_deref());
            let mut private_dict = build_dict(&pd);
            // Placeholder for the local subroutines' offset, patched below.
            private_dict.write_buffer_owned(cff_build_offset(-1));
            private_dict.write_buffer_owned(cff_encode_cff_operator(OP_SUBRS));
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
        r = build_index(idx);
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
    // Both Vecs were built to the same length (`1 + (*cff).fd_array.len()`)
    // above and never resized since -- zipping them needs no further
    // dereference of `cff` for a bound, unlike the loop this replaces.
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
    return blob;
}
pub fn build_cff(cff_and_glyf: CffAndGlyfRef, options: &Options) -> Buffer {
    writecff_cid_keyed(cff_and_glyf.meta, cff_and_glyf.glyphs, options)
}
#[inline]
fn json_from_sds(str: &[u8]) -> BuiltValue {
    BuiltValue::Str(str.to_vec())
}

#[cfg(test)]
mod cff_matrix_no_head_regression_tests {
    use super::*;
    use crate::font::sfnt::{Packet, PacketPiece};
    use crate::support::options::Options;

    // A CFF font with a Top DICT `FontMatrix` but no `head` table used to
    // crash `otfccdump` (upstream otfcc has the same null deref): applying
    // the matrix read `head` unconditionally.
    //
    // This builds the minimal scenario directly rather than shipping a
    // binary fixture: a `CffTable` with a real `font_matrix` and one
    // glyph with an actual point (so `apply_cff_matrix`'s scaling branch,
    // which is what dereferenced `head`, really runs), round-tripped
    // through the crate's own writer (`writecff_cid_keyed`) to get a
    // genuine CFF Top DICT `FontMatrix` operator in the bytes, then fed
    // back through the real read entry point
    // (`read_cff_and_glyf_tables`) with `head: None` -- exactly the
    // "font has no `head` table" case. `apply_cff_matrix` must take its
    // `None` branch and simply leave the outline unscaled.
    #[test]
    fn cff_font_matrix_with_no_head_table_does_not_crash() {
        let mut cff = table_cff_new();
        cff.private_dict = Some(new_cff_private());
        cff.font_matrix = Some(Box::new(CffFontMatrix {
            a: 0.5,
            b: 0.0,
            c: 0.0,
            d: 0.5,
            x: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            y: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
        }));

        let mut glyph = new_glyf_glyph();
        glyph.contours.push(vec![
            Point {
                x: VQ {
                    kernel: 10.0,
                    shift: Vec::new(),
                },
                y: VQ {
                    kernel: 10.0,
                    shift: Vec::new(),
                },
                on_curve: 1,
            },
            Point {
                x: VQ {
                    kernel: 20.0,
                    shift: Vec::new(),
                },
                y: VQ {
                    kernel: 10.0,
                    shift: Vec::new(),
                },
                on_curve: 1,
            },
            Point {
                x: VQ {
                    kernel: 20.0,
                    shift: Vec::new(),
                },
                y: VQ {
                    kernel: 20.0,
                    shift: Vec::new(),
                },
                on_curve: 1,
            },
        ]);
        let glyf: GlyfTable = vec![Some(glyph)];

        let options = Options::default();
        let cff_bytes = writecff_cid_keyed(&mut cff, Some(&glyf), &options);

        let packet = Packet {
            sfnt_version: crate::tag::SFNT_VERSION_OTTO,
            num_tables: 1,
            search_range: 0,
            entry_selector: 0,
            range_shift: 0,
            pieces: vec![PacketPiece {
                tag: crate::tag::TAG_CFF,
                check_sum: 0,
                offset: 0,
                length: cff_bytes.data.len() as u32,
                data: cff_bytes.data,
            }],
        };

        // The call that used to segfault: `head: None`, matching a
        // `Font` with no `head` table at all.
        let result = read_cff_and_glyf_tables(&packet, None);

        // Sanity: the FontMatrix really did round-trip through the
        // writer and back, and there is a real glyph to (not) scale --
        // otherwise this test would trivially "not crash" for the wrong
        // reason.
        let meta = result.meta.expect("CFF table should have been read back");
        assert!(
            meta.font_matrix.is_some(),
            "FontMatrix should have survived the write+read round trip"
        );
        let glyphs = result.glyphs.expect("glyf table should have been read back");
        assert_eq!(glyphs.len(), 1);
        assert!(
            !glyphs[0].as_ref().unwrap().contours.is_empty(),
            "the glyph's outline should have been built"
        );
    }

    // A Miri-friendly, no-FFI companion to the test above: exercises
    // `apply_cff_matrix` itself (the function whose signature changed
    // from a nullable `*const HeadTable` to `Option<&HeadTable>`) with
    // `head: None`, with no charstring writer in the loop to trip
    // Miri's `modf` limitation. Confirms the `None` branch is really
    // taken -- the point coordinate comes back unscaled, not silently
    // scaled by some default -- not just that nothing crashes.
    #[test]
    fn apply_cff_matrix_with_no_head_leaves_the_outline_unscaled() {
        let mut cff = table_cff_new();
        cff.font_matrix = Some(Box::new(CffFontMatrix {
            a: 2.0,
            b: 0.0,
            c: 0.0,
            d: 2.0,
            x: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            y: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
        }));
        let mut glyph = new_glyf_glyph();
        glyph.contours.push(vec![Point {
            x: VQ {
                kernel: 10.0,
                shift: Vec::new(),
            },
            y: VQ {
                kernel: 10.0,
                shift: Vec::new(),
            },
            on_curve: 1,
        }]);
        let mut glyf: GlyfTable = vec![Some(glyph)];

        apply_cff_matrix(&cff, &mut glyf, None);

        let point = &glyf[0].as_ref().unwrap().contours[0][0];
        assert_eq!(point.x.kernel, 10.0, "no head table means no scaling");
        assert_eq!(point.y.kernel, 10.0, "no head table means no scaling");
    }
}
