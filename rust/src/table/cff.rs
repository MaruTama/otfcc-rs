#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memset};
unsafe extern "C" {
    fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::support::handle::{handle_from_index, FdHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{ILogger};
use crate::support::buffer::{bufninit, Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{Arity, CffSid, FontFilePointer, GlyphId, Pos, Scale, ShapeId, TableId};
use crate::vendor::json::{JsonType};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::libcff::CffDictOperator;
use crate::libcff::{CffFile, CffIOutlineBuilder, CffStack, OP_BLUE_FUZZ, OP_BLUE_SCALE, OP_BLUE_SHIFT, OP_BLUE_VALUES, OP_CID_COUNT, OP_CID_FONT_REVISION, OP_CID_FONT_VERSION, OP_CHAR_STRINGS, OP_COPYRIGHT, OP_EXPANSION_FACTOR, OP_FD_ARRAY, OP_FD_SELECT, OP_FAMILY_BLUES, OP_FAMILY_NAME, OP_FAMILY_OTHER_BLUES, OP_FONT_BBOX, OP_FONT_MATRIX, OP_FONT_NAME, OP_FORCE_BOLD, OP_FULL_NAME, OP_ITALIC_ANGLE, OP_LANGUAGE_GROUP, OP_NOTICE, OP_OTHER_BLUES, OP_PRIVATE, OP_ROS, OP_STD_HW, OP_STD_VW, OP_STEM_SNAP_H, OP_STEM_SNAP_V, OP_STROKE_WIDTH, OP_SUBRS, OP_UID_BASE, OP_UNDERLINE_POSITION, OP_UNDERLINE_THICKNESS, OP_WEIGHT, OP_CHARSET, OP_DEFAULT_WIDTH_X, OP_INITIAL_RANDOM_SEED, OP_IS_FIXED_PITCH, OP_NOMINAL_WIDTH_X, OP_VERSION};
use crate::libcff::cff_charset::{CffCharset, CffCharsetRangeFormat2};
use crate::libcff::cff_dict::{CffDict, CffDictEntry};
use crate::libcff::cff_fdselect::{CffFdSelect, CffFdSelectRangeFormat3};
use crate::libcff::cff_index::{CffIndexCountType, CffIndex};
use crate::libcff::cff_value::{CffValueType, CffValue, CffValueBody};
use crate::libcff::charstring_il::{CffCharstringIl};
use crate::libcff::subr::{CffSubrGraph, CffSubrRule};
use crate::support::{FALSE_0, TRUE_0};
use crate::table::fvar::{FvarTable};
use crate::table::glyf::{Contour, Glyph, MaskList, Point, PostscriptHintMask, PostscriptStemDef, StemDefList, GlyfTable};
use crate::table::head::{HeadTable};


use crate::vf::vq::{VQ};
use crate::support::parsed_json::{ParsedValue, json_arr_at, json_arr_len, json_numof, json_obj_get, json_obj_get_type, json_obj_getbool, json_obj_getint, json_obj_getnum, json_obj_getnum_fallback, json_obj_getsds, json_obj_key_at, json_obj_key_len_at, json_obj_len, json_obj_val_at, json_type_of};

use crate::libcff::cff_charset::{cff_build_charset};
use crate::libcff::cff_codecs::{cff_encode_cff_operator};
use crate::libcff::cff_dict::{CFF_I_DICT};
use crate::libcff::cff_fdselect::{cff_build_fd_select};
use crate::libcff::cff_index::{CFF_I_INDEX};
use crate::libcff::cff_parser::{cff_close, cff_open_stream, cff_parse_outline, cff_parse_subr};
use crate::libcff::cff_string::{sdsget_cff_sid};
use crate::libcff::cff_value::{cffnum};
use crate::libcff::cff_writer::{cff_build_header, cff_build_offset};
use crate::libcff::charstring_il::{cff_compile_glyph_to_il, cff_optimize_il};
use crate::libcff::subr::{cff_subr_graph_init, cff_subr_graph_dispose, cff_il_graph_to_buffers, cff_insert_il_to_graph};
use crate::support::buffer::{buffree, bufnew, bufnwrite8, bufwrite_bufdel};
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::table::fvar::{json_new_vq};
use crate::table::glyf::{glyf_point_init, otfcc_new_glyf_glyph, table_glyf_create_n};
use crate::support::built_json::{BuiltValue, json_array_new, json_array_push, json_boolean_new, json_double_new, json_integer_new, json_object_new, json_object_push, json_object_push_bytes_key, json_string_new_length};
use crate::vf::vq::{I_VQ};

#[derive(Clone)]
#[repr(C)]
pub struct CffFontMatrix {
    pub a: Scale,
    pub b: Scale,
    pub c: Scale,
    pub d: Scale,
    pub x: VQ,
    pub y: VQ,
}
#[repr(C)]
pub struct CffPrivateDict {
    pub blue_values: Vec<::core::ffi::c_double>,
    pub other_blues: Vec<::core::ffi::c_double>,
    pub family_blues: Vec<::core::ffi::c_double>,
    pub family_other_blues: Vec<::core::ffi::c_double>,
    pub blue_scale: ::core::ffi::c_double,
    pub blue_shift: ::core::ffi::c_double,
    pub blue_fuzz: ::core::ffi::c_double,
    pub std_hw: ::core::ffi::c_double,
    pub std_vw: ::core::ffi::c_double,
    pub stem_snap_h: Vec<::core::ffi::c_double>,
    pub stem_snap_v: Vec<::core::ffi::c_double>,
    pub force_bold: bool,
    pub language_group: u32,
    pub expansion_factor: ::core::ffi::c_double,
    pub initial_random_seed: ::core::ffi::c_double,
    pub default_width_x: ::core::ffi::c_double,
    pub nominal_width_x: ::core::ffi::c_double,
}
// Stage 6-4 "Box化": `CffTable.private_dict` becomes
// `Option<Box<CffPrivateDict>>`. `Copy`/`Clone` dropped (nothing cloned
// this type -- confirmed by grep). The six `*mut c_double` arrays (each
// paired with its own `_count: Arity`, left raw at the time -- matching
// `CffTable.fd_array`'s own still-raw-pointer status then) are now plain
// `Vec<f64>`, so the custom `Drop` impl that used to free each one by hand
// is gone entirely: Rust's own field-by-field drop glue reaches them.
// Construction goes through `otfcc_new_cff_private()` returning
// `Box<CffPrivateDict>` directly at each call site, matching the
// `new_lookup`/`new_feature`/`GaspTable` precedent.
// `Copy`/`Clone` dropped: nine fields are now `Vec<u8>` (the `sds` sweep
// reached `CffTable`'s font-info fields). Every use of this type is behind
// `*mut CffTable`/`*const CffTable` (confirmed by grep before starting;
// `fd_array: *mut *mut CffTable` is likewise a pointer array, never a value
// copy) so there was no cascade to chase -- `table_cff_copy` (the `.copy`
// vtable slot, a raw `memcpy`) was already unreachable from any live call
// site and is deleted below rather than left as a would-be-unsound
// landmine, matching this migration's established pattern for confirmed
// -dead vtable slots.
#[repr(C)]
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
    pub italic_angle: ::core::ffi::c_double,
    pub underline_position: ::core::ffi::c_double,
    pub underline_thickness: ::core::ffi::c_double,
    pub font_b_box_top: ::core::ffi::c_double,
    pub font_b_box_bottom: ::core::ffi::c_double,
    pub font_b_box_left: ::core::ffi::c_double,
    pub font_b_box_right: ::core::ffi::c_double,
    pub stroke_width: ::core::ffi::c_double,
    pub private_dict: Option<Box<CffPrivateDict>>,
    pub font_matrix: Option<Box<CffFontMatrix>>,
    pub cid_registry: Vec<u8>,
    pub cid_ordering: Vec<u8>,
    pub cid_supplement: u32,
    pub cid_font_version: ::core::ffi::c_double,
    pub cid_font_revision: ::core::ffi::c_double,
    pub cid_count: u32,
    pub uid_base: u32,
    pub fd_array: Vec<Box<CffTable>>,
}
// Stage 6-4 "Box化", now complete for `CffTable`: `private_dict`/
// `font_matrix`/`fd_array` (self-referential -- each FD is itself a full
// `CffTable`) are all real owned Rust types now (`Option<Box<>>`,
// `Option<Box<>>`, `Vec<Box<CffTable>>`). Every field self-drops through
// ordinary compiler-generated field-by-field drop glue, so `CffTable`
// needs **no custom `Drop` impl at all** anymore -- the previous
// `impl Drop for CffTable` (which called `dispose_fd`, itself only doing
// real work for the now-gone raw `fd_array` recursive-free loop) is
// deleted outright, along with `dispose_fd`/`table_cff_dispose`/
// `table_cff_free` (all now fully dead -- confirmed via crate-wide grep).
// `table_cff_create`/`table_cff_init`/`init_fd` (`malloc`-based) stay:
// they're still how every `CffTable` value -- top-level or `fd_array`
// child -- gets constructed before being adopted into a `Box` via
// `unwrap_cff_table`, exactly as `Font.cff` itself already works.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffAndGlyf {
    pub meta: *mut CffTable,
    pub glyphs: *mut GlyfTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffExtractContext {
    pub fd_array_index: i32,
    pub meta: *mut CffTable,
    pub glyphs: *mut GlyfTable,
    pub cff_file: *mut CffFile,
    pub seed: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OutlineBuilderContext {
    pub g: *mut Glyph,
    pub j_contour: ShapeId,
    pub j_point: ShapeId,
    pub default_width_x: ::core::ffi::c_double,
    pub nominal_width_x: ::core::ffi::c_double,
    pub defined_h_stems: u8,
    pub defined_v_stems: u8,
    pub defined_hint_masks: u8,
    pub defined_contour_masks: u8,
    pub randx: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CffDoubleBits {
    pub u: u64,
    pub d: ::core::ffi::c_double,
}
#[repr(C)]
pub struct CffCharstringBuilderContext {
    pub glyf: *mut GlyfTable,
    pub default_width: u16,
    pub nominal_width_x: u16,
    pub options: *const Options,
    pub graph: CffSubrGraph,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FdArrayCompileContext {
    pub fd_array: *const Vec<Box<CffTable>>,
    pub string_hash: *mut indexmap::IndexMap<Vec<u8>, Vec<u8>>,
}
pub static DEFAULT_BLUE_SCALE: ::core::ffi::c_double = 0.039625f64;
pub static DEFAULT_BLUE_SHIFT: ::core::ffi::c_double =
    7 as ::core::ffi::c_int as ::core::ffi::c_double;
pub static DEFAULT_BLUE_FUZZ: ::core::ffi::c_double =
    1 as ::core::ffi::c_int as ::core::ffi::c_double;
pub static DEFAULT_EXPANSION_FACTOR: ::core::ffi::c_double = 0.06f64;
fn otfcc_new_cff_private() -> Box<CffPrivateDict> {
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
#[inline]
unsafe fn init_fd(mut fd: *mut CffTable) {
    memset(
        fd as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<CffTable>() as usize,
    );
    (*fd).underline_position = -(100 as ::core::ffi::c_int) as ::core::ffi::c_double;
    (*fd).underline_thickness = 50 as ::core::ffi::c_int as ::core::ffi::c_double;
}
#[inline]
unsafe fn table_cff_create() -> *mut CffTable {
    let mut x: *mut CffTable =
        malloc(::core::mem::size_of::<CffTable>() as usize) as *mut CffTable;
    table_cff_init(x);
    return x;
}
#[inline]
unsafe fn table_cff_init(mut x: *mut CffTable) {
    init_fd(x);
}
// `table_cff_create`/`fd_from_json` are shared between the top-level table
// (which becomes `Font.cff`) and `fd_array` children (still raw-pointer-
// owned, built the same way) -- widening either constructor to return a
// `Box` would force every recursive FD-array call site to also become
// `Box`-aware, well beyond this field's scope. Instead this adopts the
// malloc'd top-level pointer into a genuine `Box<CffTable>` at the one
// point it actually needs to become `Font.cff`: `ptr::read` moves the
// value out (a shallow copy -- the `Vec<u8>` fields' heap buffers and the
// `private_dict`/`font_matrix`/`fd_array` pointers all move with it, none
// of it is duplicated), then the now-empty malloc'd shell is released
// with a bare `free` (not `table_cff_free`, which would incorrectly
// re-drop everything a second time), and the value is placed into a
// fresh `Box::new` allocated via Rust's global allocator. Matches
// `unwrap_glyf_table`/`unwrap_class_def` from earlier in this migration.
pub(crate) unsafe fn unwrap_cff_table(raw: *mut CffTable) -> Option<Box<CffTable>> {
    if raw.is_null() {
        return None;
    }
    let value = ::core::ptr::read(raw);
    free(raw as *mut ::core::ffi::c_void);
    Some(Box::new(value))
}
unsafe extern "C" fn callback_extract_private(
    mut op: CffDictOperator,
    mut top: u8,
    mut stack: *mut CffValue,
    mut _context: *mut ::core::ffi::c_void,
) {
    let mut context: *mut CffExtractContext = _context as *mut CffExtractContext;
    let mut meta: *mut CffTable = (*context).meta;
    if (*context).fd_array_index >= 0 as i32
        && ((*context).fd_array_index as usize) < (*meta).fd_array.len()
    {
        meta = (&mut (*meta).fd_array)[(*context).fd_array_index as usize].as_mut() as *mut CffTable;
    }
    let mut pd: *mut CffPrivateDict = (*meta).private_dict.as_deref_mut().unwrap() as *mut CffPrivateDict;
    match op.0 {
        6 => {
            (*pd).blue_values = (0..top as Arity).map(|j| cffnum(*stack.offset(j as isize))).collect();
        }
        7 => {
            (*pd).other_blues = (0..top as Arity).map(|j| cffnum(*stack.offset(j as isize))).collect();
        }
        8 => {
            (*pd).family_blues = (0..top as Arity).map(|j| cffnum(*stack.offset(j as isize))).collect();
        }
        9 => {
            (*pd).family_other_blues =
                (0..top as Arity).map(|j| cffnum(*stack.offset(j as isize))).collect();
        }
        3084 => {
            (*pd).stem_snap_h = (0..top as Arity).map(|j| cffnum(*stack.offset(j as isize))).collect();
        }
        3085 => {
            (*pd).stem_snap_v = (0..top as Arity).map(|j| cffnum(*stack.offset(j as isize))).collect();
        }
        3081 => {
            if top != 0 {
                (*pd).blue_scale = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3082 => {
            if top != 0 {
                (*pd).blue_shift = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3083 => {
            if top != 0 {
                (*pd).blue_fuzz = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        10 => {
            if top != 0 {
                (*pd).std_hw = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        11 => {
            if top != 0 {
                (*pd).std_vw = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3086 => {
            if top != 0 {
                (*pd).force_bold = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) != 0.;
            }
        }
        3089 => {
            if top != 0 {
                (*pd).language_group = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) as u32;
            }
        }
        3090 => {
            if top != 0 {
                (*pd).expansion_factor = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3091 => {
            if top != 0 {
                (*pd).initial_random_seed = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        20 => {
            if top != 0 {
                (*pd).default_width_x = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        21 => {
            if top != 0 {
                (*pd).nominal_width_x = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn callback_extract_fd(
    mut op: CffDictOperator,
    mut top: u8,
    mut stack: *mut CffValue,
    mut _context: *mut ::core::ffi::c_void,
) {
    let mut context: *mut CffExtractContext = _context as *mut CffExtractContext;
    let mut file: *mut CffFile = (*context).cff_file;
    let mut meta: *mut CffTable = (*context).meta;
    if (*context).fd_array_index >= 0 as i32
        && ((*context).fd_array_index as usize) < (*meta).fd_array.len()
    {
        meta = (&mut (*meta).fd_array)[(*context).fd_array_index as usize].as_mut() as *mut CffTable;
    }
    match op.0 {
        0 => {
            if top != 0 {
                (*meta).version = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    &(*file).string,
                ).unwrap_or_default();
            }
        }
        1 => {
            if top != 0 {
                (*meta).notice = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    &(*file).string,
                ).unwrap_or_default();
            }
        }
        3072 => {
            if top != 0 {
                (*meta).copyright = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    &(*file).string,
                ).unwrap_or_default();
            }
        }
        3110 => {
            if top != 0 {
                (*meta).font_name = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    &(*file).string,
                ).unwrap_or_default();
            }
        }
        2 => {
            if top != 0 {
                (*meta).full_name = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    &(*file).string,
                ).unwrap_or_default();
            }
        }
        3 => {
            if top != 0 {
                (*meta).family_name = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    &(*file).string,
                ).unwrap_or_default();
            }
        }
        4 => {
            if top != 0 {
                (*meta).weight = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    &(*file).string,
                ).unwrap_or_default();
            }
        }
        5 => {
            if top as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                (*meta).font_b_box_left = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize),
                );
                (*meta).font_b_box_bottom = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize),
                );
                (*meta).font_b_box_right = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize),
                );
                (*meta).font_b_box_top = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3079 => {
            if top as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                (*meta).font_matrix = Some(Box::new(CffFontMatrix {
                    a: 0.,
                    b: 0.,
                    c: 0.,
                    d: 0.,
                    x: I_VQ.neutral.expect("non-null function pointer")(),
                    y: I_VQ.neutral.expect("non-null function pointer")(),
                }));
                let fm: *mut CffFontMatrix = (*meta).font_matrix.as_deref_mut().unwrap() as *mut CffFontMatrix;
                (*fm).a = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize),
                ) as Scale;
                (*fm).b = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize),
                ) as Scale;
                (*fm).c = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize),
                ) as Scale;
                (*fm).d = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize),
                ) as Scale;
                (*fm).x = I_VQ.create_still.expect("non-null function pointer")(
                    cffnum(
                        *stack
                            .offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize),
                    ) as Pos,
                );
                (*fm).y = I_VQ.create_still.expect("non-null function pointer")(
                    cffnum(
                        *stack
                            .offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                    ) as Pos,
                );
            }
        }
        3073 => {
            if top != 0 {
                (*meta).is_fixed_pitch = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) != 0.;
            }
        }
        3074 => {
            if top != 0 {
                (*meta).italic_angle = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3075 => {
            if top != 0 {
                (*meta).underline_position = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3076 => {
            if top != 0 {
                (*meta).underline_thickness = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        3080 => {
            if top != 0 {
                (*meta).stroke_width = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                );
            }
        }
        18 => {
            if top as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
                let mut private_length: u32 = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize),
                ) as u32;
                let mut private_offset: u32 = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) as u32;
                (*meta).private_dict = Some(otfcc_new_cff_private());
                CFF_I_DICT
                    .parse_to_callback
                    .expect("non-null function pointer")(
                    (*file).raw_data.offset(private_offset as isize),
                    private_length,
                    context as *mut ::core::ffi::c_void,
                    Some(
                        callback_extract_private
                            as unsafe extern "C" fn(
                                CffDictOperator,
                                u8,
                                *mut CffValue,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                    ),
                );
            }
        }
        3102 => {
            if top as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                (*meta).is_cid = true;
                (*meta).cid_registry = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    &(*file).string,
                ).unwrap_or_default();
                (*meta).cid_ordering = sdsget_cff_sid(
                    (*stack.offset((top as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize))
                        .c2rust_unnamed
                        .i as u16,
                    &(*file).string,
                ).unwrap_or_default();
                (*meta).cid_supplement = cffnum(
                    *stack.offset((top as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
                ) as u32;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn callback_draw_setwidth(
    mut _context: *mut ::core::ffi::c_void,
    mut width: ::core::ffi::c_double,
) {
    let mut context: *mut OutlineBuilderContext = _context as *mut OutlineBuilderContext;
    I_VQ.replace.expect("non-null function pointer")(
        &raw mut (*(*context).g).advance_width,
        I_VQ.create_still.expect("non-null function pointer")(
            width as Pos + (*context).nominal_width_x as Pos,
        ) as VQ,
    );
}
unsafe extern "C" fn callback_draw_next_contour(mut _context: *mut ::core::ffi::c_void) {
    let mut context: *mut OutlineBuilderContext = _context as *mut OutlineBuilderContext;
    (*(*context).g).contours.push(Vec::new());
    (*context).j_contour = (*(*context).g).contours.len() as ShapeId;
    (*context).j_point = 0 as ShapeId;
}
unsafe extern "C" fn callback_draw_lineto(
    mut _context: *mut ::core::ffi::c_void,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
) {
    let mut context: *mut OutlineBuilderContext = _context as *mut OutlineBuilderContext;
    if (*context).j_contour != 0 {
        let contour: *mut Contour = &raw mut (&mut (*(*context).g).contours)
            [((*context).j_contour as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize];
        let mut z: Point = Point {
            x: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            y: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            on_curve: 0,
        };
        glyf_point_init(&raw mut z);
        z.on_curve = TRUE_0 as i8;
        I_VQ.copy_replace.expect("non-null function pointer")(
            &raw mut z.x,
            I_VQ.create_still.expect("non-null function pointer")(x1 as Pos) as VQ,
        );
        I_VQ.copy_replace.expect("non-null function pointer")(
            &raw mut z.y,
            I_VQ.create_still.expect("non-null function pointer")(y1 as Pos) as VQ,
        );
        (*contour).push(z);
        (*context).j_point =
            ((*context).j_point as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ShapeId;
    }
}
unsafe extern "C" fn callback_draw_curveto(
    mut _context: *mut ::core::ffi::c_void,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
    mut x3: ::core::ffi::c_double,
    mut y3: ::core::ffi::c_double,
) {
    let mut context: *mut OutlineBuilderContext = _context as *mut OutlineBuilderContext;
    if (*context).j_contour != 0 {
        let contour: *mut Contour = &raw mut (&mut (*(*context).g).contours)
            [((*context).j_contour as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize];
        let mut z: Point = Point {
            x: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            y: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            on_curve: 0,
        };
        glyf_point_init(&raw mut z);
        z.on_curve = FALSE_0 as i8;
        I_VQ.copy_replace.expect("non-null function pointer")(
            &raw mut z.x,
            I_VQ.create_still.expect("non-null function pointer")(x1 as Pos) as VQ,
        );
        I_VQ.copy_replace.expect("non-null function pointer")(
            &raw mut z.y,
            I_VQ.create_still.expect("non-null function pointer")(y1 as Pos) as VQ,
        );
        (*contour).push(z);
        let mut z_0: Point = Point {
            x: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            y: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            on_curve: 0,
        };
        glyf_point_init(&raw mut z_0);
        z_0.on_curve = FALSE_0 as i8;
        I_VQ.copy_replace.expect("non-null function pointer")(
            &raw mut z_0.x,
            I_VQ.create_still.expect("non-null function pointer")(x2 as Pos) as VQ,
        );
        I_VQ.copy_replace.expect("non-null function pointer")(
            &raw mut z_0.y,
            I_VQ.create_still.expect("non-null function pointer")(y2 as Pos) as VQ,
        );
        (*contour).push(z_0);
        let mut z_1: Point = Point {
            x: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            y: VQ {
                kernel: 0.,
                shift: Vec::new(),
            },
            on_curve: 0,
        };
        glyf_point_init(&raw mut z_1);
        z_1.on_curve = TRUE_0 as i8;
        I_VQ.copy_replace.expect("non-null function pointer")(
            &raw mut z_1.x,
            I_VQ.create_still.expect("non-null function pointer")(x3 as Pos) as VQ,
        );
        I_VQ.copy_replace.expect("non-null function pointer")(
            &raw mut z_1.y,
            I_VQ.create_still.expect("non-null function pointer")(y3 as Pos) as VQ,
        );
        (*contour).push(z_1);
        (*context).j_point =
            ((*context).j_point as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as ShapeId;
    }
}
unsafe extern "C" fn callback_draw_sethint(
    mut _context: *mut ::core::ffi::c_void,
    mut is_vertical: bool,
    mut position: ::core::ffi::c_double,
    mut width: ::core::ffi::c_double,
) {
    let mut context: *mut OutlineBuilderContext = _context as *mut OutlineBuilderContext;
    let stems: &mut StemDefList = if is_vertical as ::core::ffi::c_int != 0 {
        &mut (*(*context).g).stem_v
    } else {
        &mut (*(*context).g).stem_h
    };
    stems.push(PostscriptStemDef {
        position: position as Pos,
        width: width as Pos,
        map: 0,
    });
}
unsafe extern "C" fn callback_draw_setmask(
    mut _context: *mut ::core::ffi::c_void,
    mut is_contour_mask: bool,
    mut mask_array: *mut bool,
) {
    let mut context: *mut OutlineBuilderContext = _context as *mut OutlineBuilderContext;
    let mask_list: &mut MaskList = if is_contour_mask as ::core::ffi::c_int != 0 {
        &mut (*(*context).g).contour_masks
    } else {
        &mut (*(*context).g).hint_masks
    };
    let mut mask: PostscriptHintMask = PostscriptHintMask {
        points_before: 0,
        contours_before: 0,
        mask_h: [false; 256],
        mask_v: [false; 256],
    };
    if (*context).j_contour != 0 {
        mask.contours_before =
            ((*context).j_contour as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u16;
    } else {
        mask.contours_before = 0 as u16;
    }
    mask.points_before = (*context).j_point as u16;
    let stem_h_len = (*(*context).g).stem_h.len();
    let stem_v_len = (*(*context).g).stem_v.len();
    let mut j: ShapeId = 0 as ShapeId;
    while (j as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int {
        mask.mask_h[j as usize] = if (j as usize) < stem_h_len {
            *mask_array.offset(j as isize) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } != 0;
        mask.mask_v[j as usize] = if (j as usize) < stem_v_len {
            *mask_array.offset((j as usize).wrapping_add(stem_h_len) as isize)
                as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } != 0;
        j = j.wrapping_add(1);
    }
    free(mask_array as *mut ::core::ffi::c_void);
    mask_array = ::core::ptr::null_mut::<bool>();
    if !mask_list.is_empty()
        && mask_list[mask_list.len() - 1].contours_before as ::core::ffi::c_int
            == mask.contours_before as ::core::ffi::c_int
        && mask_list[mask_list.len() - 1].points_before as ::core::ffi::c_int
            == mask.points_before as ::core::ffi::c_int
    {
        let last = mask_list.len() - 1;
        let mut j_0: ShapeId = 0 as ShapeId;
        while (j_0 as ::core::ffi::c_int) < 0x100 as ::core::ffi::c_int {
            mask_list[last].mask_h[j_0 as usize] = mask.mask_h[j_0 as usize];
            mask_list[last].mask_v[j_0 as usize] = mask.mask_v[j_0 as usize];
            j_0 = j_0.wrapping_add(1);
        }
    } else {
        mask_list.push(mask);
        if is_contour_mask {
            (*context).defined_contour_masks = ((*context).defined_contour_masks as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as u8;
        } else {
            (*context).defined_hint_masks = ((*context).defined_hint_masks as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as u8;
        }
    };
}
unsafe extern "C" fn callback_draw_getrand(
    mut _context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_double {
    let mut context: *mut OutlineBuilderContext = _context as *mut OutlineBuilderContext;
    let mut x: u64 = (*context).randx;
    x ^= x >> 12 as ::core::ffi::c_int;
    x ^= x << 25 as ::core::ffi::c_int;
    x ^= x >> 27 as ::core::ffi::c_int;
    (*context).randx = x;
    let mut a: CffDoubleBits = CffDoubleBits { u: 0 };
    a.u = x.wrapping_mul(2685821657736338717 as u64);
    a.u = a.u >> 12 as ::core::ffi::c_int | 0x3ff0000000000000 as u64;
    let mut q: ::core::ffi::c_double = if a.u & 2048 as u64 != 0 {
        1.0f64 - 2.2204460492503131E-16f64 / 2.0f64
    } else {
        1.0f64
    };
    return a.d - q;
}
static DRAW_PASS: CffIOutlineBuilder = {
    CffIOutlineBuilder {
        set_width: Some(
            callback_draw_setwidth
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> (),
        ),
        new_contour: Some(
            callback_draw_next_contour as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        line_to: Some(
            callback_draw_lineto
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        ),
        curve_to: Some(
            callback_draw_curveto
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        ),
        set_hint: Some(
            callback_draw_sethint
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    bool,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        ),
        set_mask: Some(
            callback_draw_setmask
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> (),
        ),
        getrand: Some(
            callback_draw_getrand
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double,
        ),
    }
};
unsafe fn build_outline(
    mut i: GlyphId,
    mut context: *mut CffExtractContext,
    mut options: *const Options,
) {
    let mut f: *mut CffFile = (*context).cff_file;
    // `g` keeps pointing at the same heap allocation for the rest of this
    // function (via `bc.g` below) even after the `Box` that owns it is
    // moved into the table -- moving a `Box` moves only the handle, not
    // the allocation it points at.
    let mut g_owner: Box<Glyph> = otfcc_new_glyf_glyph();
    let mut g: *mut Glyph = &raw mut *g_owner;
    (&mut (*(*context).glyphs))[i as usize] = Some(g_owner);
    let mut seed: u64 = (*context).seed;
    let mut local_subrs: CffIndex = CffIndex {
        count_type: CffIndexCountType::U16,
        count: 0,
        off_size: 0,
        offset: Vec::new(),
        data: Vec::new(),
    };
    CFF_I_INDEX.init.expect("non-null function pointer")(&raw mut local_subrs);
    let mut stack: CffStack = CffStack {
        stack: vec![
            CffValue {
                t: CffValueType::Unset,
                c2rust_unnamed: CffValueBody { i: 0 },
            };
            0x10000
        ],
        transient: [CffValue {
            t: CffValueType::Unset,
            c2rust_unnamed: CffValueBody { i: 0 },
        }; 32],
        index: 0,
        stem: 0,
    };
    let mut bc: OutlineBuilderContext = OutlineBuilderContext {
        g: g,
        j_contour: 0 as ShapeId,
        j_point: 0 as ShapeId,
        default_width_x: 0.0f64,
        nominal_width_x: 0.0f64,
        defined_h_stems: 0 as u8,
        defined_v_stems: 0 as u8,
        defined_hint_masks: 0 as u8,
        defined_contour_masks: 0 as u8,
        randx: 0 as u64,
    };
    let mut fd: u8 = 0 as u8;
    if !matches!((*f).fdselect, CffFdSelect::Unspecified) {
        fd = cff_parse_subr(
            i as u16,
            (*f).raw_data,
            &(*f).font_dict,
            &(*f).fdselect,
            &raw mut local_subrs,
        );
    } else {
        fd = cff_parse_subr(
            i as u16,
            (*f).raw_data,
            &(*f).top_dict,
            &(*f).fdselect,
            &raw mut local_subrs,
        );
    }
    (*g).fd_select = handle_from_index(fd as GlyphId)
        as FdHandle;
    let ctx_fd_array: &mut Vec<Box<CffTable>> = &mut (*(*context).meta).fd_array;
    if (fd as usize) < ctx_fd_array.len()
        && ctx_fd_array[fd as usize]
            .private_dict
            .is_some()
    {
        let pd = ctx_fd_array[fd as usize].private_dict.as_deref().unwrap();
        bc.default_width_x = pd.default_width_x;
        bc.nominal_width_x = pd.nominal_width_x;
    } else if let Some(pd) = (*(*context).meta).private_dict.as_deref() {
        bc.default_width_x = pd.default_width_x;
        bc.nominal_width_x = pd.nominal_width_x;
    }
    I_VQ.replace.expect("non-null function pointer")(
        &raw mut (*g).advance_width,
        I_VQ.create_still.expect("non-null function pointer")(bc.default_width_x as Pos) as VQ,
    );
    let char_strings_offset = &(*f).char_strings.offset;
    let mut char_string_ptr: *mut u8 = ((*f)
        .char_strings
        .data
        .as_ptr() as *mut u8)
        .offset(char_strings_offset[i as usize] as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut char_string_length: u32 = (char_strings_offset
        [(i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize])
    .wrapping_sub(char_strings_offset[i as usize]);
    stack.index = 0 as Arity;
    stack.stem = 0 as u8;
    bc.j_contour = 0 as ShapeId;
    bc.j_point = 0 as ShapeId;
    bc.randx = seed;
    cff_parse_outline(
        char_string_ptr,
        char_string_length,
        &(*f).global_subr,
        &local_subrs,
        &raw mut stack,
        &raw mut bc as *mut ::core::ffi::c_void,
        DRAW_PASS,
        options,
    );
    let mut cx: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    let mut cy: VQ =
        (I_VQ.neutral.expect("non-null function pointer"))();
    let mut j: ShapeId = 0 as ShapeId;
    while (j as usize) < (*g).contours.len() {
        let contour: *mut Contour = &raw mut (&mut (*g).contours)[j as usize];
        let mut k: ShapeId = 0 as ShapeId;
        while (k as usize) < (*contour).len() {
            let z: *mut Point = &raw mut (&mut (*contour))[k as usize];
            I_VQ.inplace_plus.expect("non-null function pointer")(&raw mut cx, (*z).x.clone());
            I_VQ.inplace_plus.expect("non-null function pointer")(&raw mut cy, (*z).y.clone());
            I_VQ.copy_replace.expect("non-null function pointer")(&raw mut (*z).x, cx.clone());
            I_VQ.copy_replace.expect("non-null function pointer")(&raw mut (*z).y, cy.clone());
            k = k.wrapping_add(1);
        }
        if I_VQ.compare.expect("non-null function pointer")(
            (&(*contour))[0 as usize].x.clone(),
            (&(*contour))[(*contour).len().wrapping_sub(1 as usize)].x.clone(),
        ) == 0
            && I_VQ.compare.expect("non-null function pointer")(
                (&(*contour))[0 as usize].y.clone(),
                (&(*contour))[(*contour).len().wrapping_sub(1 as usize)].y.clone(),
            ) == 0
            && ((&(*contour))[0 as usize].on_curve
                as ::core::ffi::c_int
                != 0
                && (&(*contour))[(*contour).len().wrapping_sub(1 as usize)]
                .on_curve as ::core::ffi::c_int
                    != 0)
        {
            (*contour).pop();
        }
        (*contour).shrink_to_fit();
        j = j.wrapping_add(1);
    }
    (*g).contours.shrink_to_fit();
    // `cx`/`cy` are plain owned locals, never moved out, so they auto-drop
    // when this function returns -- no explicit dispose call is needed.
    CFF_I_INDEX.dispose.expect("non-null function pointer")(&raw mut local_subrs);
    (*context).seed = bc.randx;
}
// Returns `Vec<u8>`, its only callers direct Rust call sites (never a real
// FFI boundary) -- goes away with the vtable/extern "C" cleanup, same as
// every other instance of this allow in the crate.
#[allow(improper_ctypes_definitions)]
unsafe fn form_cid_string(mut cid: CffSid) -> Vec<u8> {
    return crate::bytesbuild!(b"CID", cid as ::core::ffi::c_int);
}
unsafe fn name_glyphs_according_to_cff(mut context: *mut CffExtractContext) {
    let mut cff_file: *mut CffFile = (*context).cff_file;
    let mut glyphs: *mut GlyfTable = (*context).glyphs;
    let charset: &CffCharset = &(*cff_file).charsets;
    if (*(*context).meta).is_cid {
        match charset {
            CffCharset::Format0(glyph) => {
                for (j, &g) in glyph.iter().enumerate() {
                    let sid: CffSid = g as CffSid;
                    let glyphname: Option<Vec<u8>> = sdsget_cff_sid(sid as u16, &(*cff_file).string);
                    if let Some(glyphname) = glyphname {
                        let ref mut fresh2 = (&mut (*glyphs))[j + 1].as_mut().unwrap().name;
                        *fresh2 = glyphname;
                        (&mut (*glyphs))[j + 1].as_mut().unwrap().cid = sid as GlyphId;
                    }
                }
            }
            CffCharset::Format1(range1) => {
                let mut glyphs_named_sofar: u32 = 1 as u32;
                for r in range1 {
                    let first: CffSid = r.first as CffSid;
                    let mut k: GlyphId = 0 as GlyphId;
                    while k as ::core::ffi::c_int <= r.nleft as ::core::ffi::c_int {
                        let sid_0: CffSid = (first as ::core::ffi::c_int + k as ::core::ffi::c_int) as CffSid;
                        let glyphname_0: Vec<u8> = form_cid_string(sid_0);
                        if (glyphs_named_sofar as usize) < (*glyphs).len() {
                            let ref mut fresh3 =
                                (&mut (*glyphs))[glyphs_named_sofar as usize].as_mut().unwrap().name;
                            *fresh3 = glyphname_0;
                            (&mut (*glyphs))[glyphs_named_sofar as usize].as_mut().unwrap().cid =
                                sid_0 as GlyphId;
                        }
                        glyphs_named_sofar = glyphs_named_sofar.wrapping_add(1);
                        k = k.wrapping_add(1);
                    }
                }
            }
            CffCharset::Format2(range2) => {
                let mut glyphs_named_sofar_0: u32 = 1 as u32;
                for r in range2 {
                    let first_0: CffSid = r.first as CffSid;
                    let mut k_0: GlyphId = 0 as GlyphId;
                    while k_0 as ::core::ffi::c_int <= r.nleft as ::core::ffi::c_int {
                        let sid_1: CffSid =
                            (first_0 as ::core::ffi::c_int + k_0 as ::core::ffi::c_int) as CffSid;
                        let glyphname_1: Vec<u8> = form_cid_string(sid_1);
                        if (glyphs_named_sofar_0 as usize) < (*glyphs).len() {
                            let ref mut fresh4 =
                                (&mut (*glyphs))[glyphs_named_sofar_0 as usize].as_mut().unwrap().name;
                            *fresh4 = glyphname_1;
                            (&mut (*glyphs))[glyphs_named_sofar_0 as usize].as_mut().unwrap().cid =
                                sid_1 as GlyphId;
                        }
                        glyphs_named_sofar_0 = glyphs_named_sofar_0.wrapping_add(1);
                        k_0 = k_0.wrapping_add(1);
                    }
                }
            }
            _ => {}
        }
    } else {
        match charset {
            CffCharset::Format0(glyph) => {
                for (j_2, &g) in glyph.iter().enumerate() {
                    let sid_2: CffSid = g as CffSid;
                    let glyphname_2: Option<Vec<u8>> = sdsget_cff_sid(sid_2 as u16, &(*cff_file).string);
                    if let Some(glyphname_2) = glyphname_2 {
                        let ref mut fresh5 = (&mut (*glyphs))[j_2 + 1].as_mut().unwrap().name;
                        *fresh5 = glyphname_2;
                    }
                }
            }
            CffCharset::Format1(range1) => {
                let mut glyphs_named_sofar_1: u32 = 1 as u32;
                for r in range1 {
                    let first_1: GlyphId = r.first as GlyphId;
                    let mut k_1: GlyphId = 0 as GlyphId;
                    while k_1 as ::core::ffi::c_int <= r.nleft as ::core::ffi::c_int {
                        let sid_3: CffSid =
                            (first_1 as ::core::ffi::c_int + k_1 as ::core::ffi::c_int) as CffSid;
                        let glyphname_3: Option<Vec<u8>> =
                            sdsget_cff_sid(sid_3 as u16, &(*cff_file).string);
                        if (glyphs_named_sofar_1 as usize) < (*glyphs).len() {
                            if let Some(glyphname_3) = glyphname_3 {
                                let ref mut fresh6 =
                                    (&mut (*glyphs))[glyphs_named_sofar_1 as usize].as_mut().unwrap().name;
                                *fresh6 = glyphname_3;
                            }
                        }
                        glyphs_named_sofar_1 = glyphs_named_sofar_1.wrapping_add(1);
                        k_1 = k_1.wrapping_add(1);
                    }
                }
            }
            CffCharset::Format2(range2) => {
                let mut glyphs_named_sofar_2: u32 = 1 as u32;
                for r in range2 {
                    let first_2: GlyphId = r.first as GlyphId;
                    let mut k_2: GlyphId = 0 as GlyphId;
                    while k_2 as ::core::ffi::c_int <= r.nleft as ::core::ffi::c_int {
                        let sid_4: CffSid =
                            (first_2 as ::core::ffi::c_int + k_2 as ::core::ffi::c_int) as CffSid;
                        let glyphname_4: Option<Vec<u8>> =
                            sdsget_cff_sid(sid_4 as u16, &(*cff_file).string);
                        if (glyphs_named_sofar_2 as usize) < (*glyphs).len() {
                            if let Some(glyphname_4) = glyphname_4 {
                                let ref mut fresh7 =
                                    (&mut (*glyphs))[glyphs_named_sofar_2 as usize].as_mut().unwrap().name;
                                *fresh7 = glyphname_4;
                            }
                        }
                        glyphs_named_sofar_2 = glyphs_named_sofar_2.wrapping_add(1);
                        k_2 = k_2.wrapping_add(1);
                    }
                }
            }
            _ => {}
        }
    };
}
unsafe fn qround(x: ::core::ffi::c_double) -> ::core::ffi::c_double {
    return otfcc_from_fixed(otfcc_to_fixed(x));
}
unsafe fn apply_cff_matrix(
    mut cff: *mut CffTable,
    mut glyf: *mut GlyfTable,
    mut head: *const HeadTable,
) {
    let mut jj: GlyphId = 0 as GlyphId;
    while (jj as usize) < (*glyf).len() {
        let g: *mut Glyph = &raw mut **(&mut (*glyf))[jj as usize].as_mut().unwrap();
        let mut fd: *mut CffTable = cff;
        if ((*g).fd_select.index as usize) < (*fd).fd_array.len() {
            fd = (&mut (*fd).fd_array)[(*g).fd_select.index as usize].as_mut() as *mut CffTable;
        }
        if let Some(fm) = (*fd).font_matrix.as_deref() {
            let mut a: Scale = qround(
                (*head).units_per_em as ::core::ffi::c_int as ::core::ffi::c_double
                    * fm.a as ::core::ffi::c_double,
            ) as Scale;
            let mut b: Scale = qround(
                (*head).units_per_em as ::core::ffi::c_int as ::core::ffi::c_double
                    * fm.b as ::core::ffi::c_double,
            ) as Scale;
            let mut c: Scale = qround(
                (*head).units_per_em as ::core::ffi::c_int as ::core::ffi::c_double
                    * fm.c as ::core::ffi::c_double,
            ) as Scale;
            let mut d: Scale = qround(
                (*head).units_per_em as ::core::ffi::c_int as ::core::ffi::c_double
                    * fm.d as ::core::ffi::c_double,
            ) as Scale;
            let mut x: VQ = I_VQ.scale.expect("non-null function pointer")(
                fm.x.clone(),
                (*head).units_per_em as Scale,
            );
            x.kernel = qround(x.kernel as ::core::ffi::c_double) as Pos;
            let mut y: VQ = I_VQ.scale.expect("non-null function pointer")(
                fm.y.clone(),
                (*head).units_per_em as Scale,
            );
            y.kernel = qround(y.kernel as ::core::ffi::c_double) as Pos;
            let mut j: ShapeId = 0 as ShapeId;
            while (j as usize) < (*g).contours.len() {
                let contour: *mut Contour = &raw mut (&mut (*g).contours)[j as usize];
                let mut k: ShapeId = 0 as ShapeId;
                while (k as usize) < (*contour).len() {
                    let mut zx: VQ = I_VQ.dup.expect("non-null function pointer")(
                        (&(*contour))[k as usize].x.clone(),
                    );
                    let mut zy: VQ = I_VQ.dup.expect("non-null function pointer")(
                        (&(*contour))[k as usize].y.clone(),
                    );
                    I_VQ.replace.expect("non-null function pointer")(
                        &raw mut (&mut (*contour))[k as usize].x,
                        I_VQ.point_linear_tfm.expect("non-null function pointer")(
                            x.clone(), a as Pos, zx.clone(), b as Pos, zy.clone(),
                        ) as VQ,
                    );
                    I_VQ.replace.expect("non-null function pointer")(
                        &raw mut (&mut (*contour))[k as usize].y,
                        I_VQ.point_linear_tfm.expect("non-null function pointer")(
                            y.clone(), c as Pos, zx.clone(), d as Pos, zy.clone(),
                        ) as VQ,
                    );
                    // `zx`/`zy` are plain owned locals, never moved out, so
                    // they auto-drop at the end of this iteration -- no
                    // explicit dispose call is needed.
                    k = k.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
            // `x`/`y` are plain owned locals, never moved out, so they
            // auto-drop at the end of this block -- no explicit dispose
            // call is needed.
        }
        jj = jj.wrapping_add(1);
    }
}
pub unsafe fn otfcc_read_cff_and_glyf_tables(
    packet: &Packet,
    mut options: *const Options,
    mut head: *const HeadTable,
) -> CffAndGlyf {
    let mut ret: CffAndGlyf = CffAndGlyf {
        meta: ::core::ptr::null_mut::<CffTable>(),
        glyphs: ::core::ptr::null_mut::<GlyfTable>(),
    };
    ret.meta = ::core::ptr::null_mut::<CffTable>();
    ret.glyphs = ::core::ptr::null_mut::<GlyfTable>();
    let mut context: CffExtractContext = CffExtractContext {
        fd_array_index: 0,
        meta: ::core::ptr::null_mut::<CffTable>(),
        glyphs: ::core::ptr::null_mut::<GlyfTable>(),
        cff_file: ::core::ptr::null_mut::<CffFile>(),
        seed: 0,
    };
    context.fd_array_index = -(1 as ::core::ffi::c_int) as i32;
    context.meta = ::core::ptr::null_mut::<CffTable>();
    context.glyphs = ::core::ptr::null_mut::<GlyfTable>();
    context.cff_file = ::core::ptr::null_mut::<CffFile>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_CFF {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data.as_ptr() as FontFilePointer;
                    let mut length: u32 = table.length;
                    let mut cff_file: *mut CffFile =
                        cff_open_stream(data as *mut u8, length, options);
                    context.cff_file = cff_file;
                    context.meta = (
                        table_cff_create)();
                    CFF_I_DICT
                        .parse_to_callback
                        .expect("non-null function pointer")(
                        (*cff_file).top_dict.data.as_ptr(),
                        {
                            let top_dict_offset = &(*cff_file).top_dict.offset;
                            (top_dict_offset[1 as usize])
                                .wrapping_sub(top_dict_offset[0 as usize])
                        },
                        &raw mut context as *mut ::core::ffi::c_void,
                        Some(
                            callback_extract_fd
                                as unsafe extern "C" fn(
                                    CffDictOperator,
                                    u8,
                                    *mut CffValue,
                                    *mut ::core::ffi::c_void,
                                ) -> (),
                        ),
                    );
                    if (*context.meta).font_name.is_empty() {
                        (*context.meta).font_name = sdsget_cff_sid(391 as u16, &(*cff_file).name).unwrap_or_default();
                    }
                    if (*cff_file).font_dict.count != 0 {
                        let fd_count = (*cff_file).font_dict.count as usize;
                        (*context.meta).fd_array = Vec::with_capacity(fd_count);
                        let mut j: TableId = 0 as TableId;
                        while (j as usize) < fd_count {
                            // Pushed *before* the recursive parse below (not
                            // after): `context.fd_array_index` makes
                            // `callback_extract_fd`/`callback_extract_private`
                            // re-derive `meta` as `(*meta).fd_array[j]` while
                            // this element is still being populated, so it
                            // must already be present in the `Vec` at that
                            // index -- a `Box`'s heap address never moves,
                            // even if a later `push` reallocates the `Vec`'s
                            // own backing buffer of `Box` pointers.
                            (*context.meta).fd_array.push(
                                unwrap_cff_table(
                                    (table_cff_create)(),
                                )
                                .unwrap(),
                            );
                            context.fd_array_index = j as i32;
                            CFF_I_DICT
                                .parse_to_callback
                                .expect(
                                    "non-null function pointer",
                                )(
                                {
                                    let font_dict_offset = &(*cff_file).font_dict.offset;
                                    (*cff_file)
                                        .font_dict
                                        .data
                                        .as_ptr()
                                        .offset(
                                            font_dict_offset[j as usize] as isize,
                                        )
                                        .offset(-(1 as ::core::ffi::c_int as isize))
                                },
                                {
                                    let font_dict_offset = &(*cff_file).font_dict.offset;
                                    (font_dict_offset
                                        [(j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize])
                                        .wrapping_sub(
                                            font_dict_offset[j as usize],
                                        )
                                },
                                &raw mut context as *mut ::core::ffi::c_void,
                                Some(
                                    callback_extract_fd
                                        as unsafe extern "C" fn(
                                            CffDictOperator,
                                            u8,
                                            *mut CffValue,
                                            *mut ::core::ffi::c_void,
                                        ) -> (),
                                ),
                            );
                            if (&mut (*context.meta).fd_array)[j as usize]
                                .font_name
                                .is_empty()
                            {
                                (&mut (*context.meta).fd_array)[j as usize].font_name =
                                    crate::bytesbuild!(b"_Subfont", j as ::core::ffi::c_int);
                            }
                            j = j.wrapping_add(1);
                        }
                    }
                    ret.meta = context.meta;
                    context.seed = 0x1234567887654321 as u64;
                    if let Some(pd) = (*context.meta).private_dict.as_deref() {
                        context.seed = pd.initial_random_seed as u64
                            ^ 0x1234567887654321 as u64;
                    }
                    let glyphs: *mut GlyfTable =
                        table_glyf_create_n((*cff_file).char_strings.count as usize);
                    context.glyphs = glyphs;
                    let mut j_0: GlyphId = 0 as GlyphId;
                    while (j_0 as usize) < (*glyphs).len() {
                        build_outline(j_0, &raw mut context, options);
                        j_0 = j_0.wrapping_add(1);
                    }
                    apply_cff_matrix(context.meta, context.glyphs, head);
                    name_glyphs_according_to_cff(&raw mut context);
                    ret.glyphs = context.glyphs;
                    cff_close(cff_file);
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ret;
}
unsafe fn pd_delta_to_json(
    target: *mut BuiltValue,
    field: *const ::core::ffi::c_char,
    values: &[::core::ffi::c_double],
) {
    if values.is_empty() {
        return;
    }
    let a: *mut BuiltValue = json_array_new(values.len());
    for &x in values {
        json_array_push(a, json_double_new(x));
    }
    json_object_push(target, field, a);
}
unsafe fn pd_to_json(mut pd: *const CffPrivateDict) -> *mut BuiltValue {
    let mut _pd: *mut BuiltValue = json_object_new(24 as usize);
    pd_delta_to_json(
        _pd,
        b"blueValues\0" as *const u8 as *const ::core::ffi::c_char,
        &(*pd).blue_values,
    );
    pd_delta_to_json(
        _pd,
        b"otherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        &(*pd).other_blues,
    );
    pd_delta_to_json(
        _pd,
        b"familyBlues\0" as *const u8 as *const ::core::ffi::c_char,
        &(*pd).family_blues,
    );
    pd_delta_to_json(
        _pd,
        b"familyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        &(*pd).family_other_blues,
    );
    pd_delta_to_json(
        _pd,
        b"stemSnapH\0" as *const u8 as *const ::core::ffi::c_char,
        &(*pd).stem_snap_h,
    );
    pd_delta_to_json(
        _pd,
        b"stemSnapV\0" as *const u8 as *const ::core::ffi::c_char,
        &(*pd).stem_snap_v,
    );
    if (*pd).blue_scale != DEFAULT_BLUE_SCALE {
        json_object_push(
            _pd,
            b"blueScale\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).blue_scale),
        );
    }
    if (*pd).blue_shift != DEFAULT_BLUE_SHIFT {
        json_object_push(
            _pd,
            b"blueShift\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).blue_shift),
        );
    }
    if (*pd).blue_fuzz != DEFAULT_BLUE_FUZZ {
        json_object_push(
            _pd,
            b"blueFuzz\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).blue_fuzz),
        );
    }
    if (*pd).std_hw != 0. {
        json_object_push(
            _pd,
            b"stdHW\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).std_hw),
        );
    }
    if (*pd).std_vw != 0. {
        json_object_push(
            _pd,
            b"stdVW\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).std_vw),
        );
    }
    if (*pd).force_bold {
        json_object_push(
            _pd,
            b"forceBold\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*pd).force_bold as ::core::ffi::c_int),
        );
    }
    if (*pd).language_group != 0 {
        json_object_push(
            _pd,
            b"languageGroup\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).language_group as ::core::ffi::c_double),
        );
    }
    if (*pd).expansion_factor != DEFAULT_EXPANSION_FACTOR {
        json_object_push(
            _pd,
            b"expansionFactor\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).expansion_factor),
        );
    }
    if (*pd).initial_random_seed != 0. {
        json_object_push(
            _pd,
            b"initialRandomSeed\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).initial_random_seed),
        );
    }
    if (*pd).default_width_x != 0. {
        json_object_push(
            _pd,
            b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).default_width_x),
        );
    }
    if (*pd).nominal_width_x != 0. {
        json_object_push(
            _pd,
            b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*pd).nominal_width_x),
        );
    }
    return _pd;
}
unsafe fn fd_to_json(mut table: *const CffTable) -> *mut BuiltValue {
    let mut _cff: *mut BuiltValue = json_object_new(24 as usize);
    if (*table).is_cid {
        json_object_push(
            _cff,
            b"isCID\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*table).is_cid as ::core::ffi::c_int),
        );
    }
    if !(*table).version.is_empty() {
        json_object_push(
            _cff,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds(&(*table).version),
        );
    }
    if !(*table).notice.is_empty() {
        json_object_push(
            _cff,
            b"notice\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds(&(*table).notice),
        );
    }
    if !(*table).copyright.is_empty() {
        json_object_push(
            _cff,
            b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds(&(*table).copyright),
        );
    }
    if !(*table).font_name.is_empty() {
        json_object_push(
            _cff,
            b"fontName\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds(&(*table).font_name),
        );
    }
    if !(*table).full_name.is_empty() {
        json_object_push(
            _cff,
            b"fullName\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds(&(*table).full_name),
        );
    }
    if !(*table).family_name.is_empty() {
        json_object_push(
            _cff,
            b"familyName\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds(&(*table).family_name),
        );
    }
    if !(*table).weight.is_empty() {
        json_object_push(
            _cff,
            b"weight\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds(&(*table).weight),
        );
    }
    if (*table).is_fixed_pitch {
        json_object_push(
            _cff,
            b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*table).is_fixed_pitch as ::core::ffi::c_int),
        );
    }
    if (*table).italic_angle != 0. {
        json_object_push(
            _cff,
            b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).italic_angle),
        );
    }
    if (*table).underline_position != -(100 as ::core::ffi::c_int) as ::core::ffi::c_double {
        json_object_push(
            _cff,
            b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).underline_position),
        );
    }
    if (*table).underline_thickness != 50 as ::core::ffi::c_int as ::core::ffi::c_double {
        json_object_push(
            _cff,
            b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).underline_thickness),
        );
    }
    if (*table).stroke_width != 0. {
        json_object_push(
            _cff,
            b"strokeWidth\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).stroke_width),
        );
    }
    if (*table).font_b_box_left != 0. {
        json_object_push(
            _cff,
            b"fontBBoxLeft\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).font_b_box_left),
        );
    }
    if (*table).font_b_box_bottom != 0. {
        json_object_push(
            _cff,
            b"fontBBoxBottom\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).font_b_box_bottom),
        );
    }
    if (*table).font_b_box_right != 0. {
        json_object_push(
            _cff,
            b"fontBBoxRight\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).font_b_box_right),
        );
    }
    if (*table).font_b_box_top != 0. {
        json_object_push(
            _cff,
            b"fontBBoxTop\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new((*table).font_b_box_top),
        );
    }
    if let Some(fm) = (*table).font_matrix.as_deref() {
        let mut _font_matrix: *mut BuiltValue = json_object_new(6 as usize);
        json_object_push(
            _font_matrix,
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(fm.a as ::core::ffi::c_double),
        );
        json_object_push(
            _font_matrix,
            b"b\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(fm.b as ::core::ffi::c_double),
        );
        json_object_push(
            _font_matrix,
            b"c\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(fm.c as ::core::ffi::c_double),
        );
        json_object_push(
            _font_matrix,
            b"d\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(fm.d as ::core::ffi::c_double),
        );
        json_object_push(
            _font_matrix,
            b"x\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_vq(fm.x.clone(), ::core::ptr::null::<FvarTable>()),
        );
        json_object_push(
            _font_matrix,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_vq(fm.y.clone(), ::core::ptr::null::<FvarTable>()),
        );
        json_object_push(
            _cff,
            b"fontMatrix\0" as *const u8 as *const ::core::ffi::c_char,
            _font_matrix,
        );
    }
    if let Some(pd) = (*table).private_dict.as_deref() {
        json_object_push(
            _cff,
            b"privates\0" as *const u8 as *const ::core::ffi::c_char,
            pd_to_json(pd as *const CffPrivateDict),
        );
    }
    if !(*table).cid_registry.is_empty() && !(*table).cid_ordering.is_empty() {
        json_object_push(
            _cff,
            b"cidRegistry\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds(&(*table).cid_registry),
        );
        json_object_push(
            _cff,
            b"cidOrdering\0" as *const u8 as *const ::core::ffi::c_char,
            json_from_sds(&(*table).cid_ordering),
        );
        json_object_push(
            _cff,
            b"cidSupplement\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).cid_supplement as i64),
        );
    }
    if !(*table).fd_array.is_empty() {
        let mut _fd_array: *mut BuiltValue = json_object_new((*table).fd_array.len());
        // `table` is `*const CffTable`, but the take/restore below needs a
        // mutable place -- sound here because nothing else touches `table`
        // during a dump pass, matching the same "read-only signature,
        // temporary local mutation" shape the original raw-pointer-through-
        // a-second-pointer code already relied on.
        let table_mut: *mut CffTable = table as *mut CffTable;
        let fd_array: &mut Vec<Box<CffTable>> = &mut (*table_mut).fd_array;
        let mut j: TableId = 0 as TableId;
        while (j as usize) < fd_array.len() {
            let name: Vec<u8> =
                ::core::mem::take(&mut fd_array[j as usize].font_name);
            json_object_push_bytes_key(
                _fd_array,
                &name,
                fd_to_json(fd_array[j as usize].as_ref() as *const CffTable),
            );
            fd_array[j as usize].font_name = name;
            j = j.wrapping_add(1);
        }
        json_object_push(
            _cff,
            b"fdArray\0" as *const u8 as *const ::core::ffi::c_char,
            _fd_array,
        );
    }
    return _cff;
}
pub unsafe fn otfcc_dump_cff(
    mut table: Option<&CffTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let table: *const CffTable = table.map_or(::core::ptr::null(), |t| t as *const CffTable);
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::bytesbuild!(b"CFF"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        json_object_push(
            root,
            b"CFF_\0" as *const u8 as *const ::core::ffi::c_char,
            fd_to_json(table),
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
unsafe fn pd_delta_from_json(dump: *const ParsedValue) -> Vec<::core::ffi::c_double> {
    if dump.is_null() || json_type_of(dump) != JsonType::Array {
        return Vec::new();
    }
    (0..json_arr_len(dump)).map(|j| json_numof(json_arr_at(dump, j))).collect()
}
unsafe fn pd_from_json(mut dump: *const ParsedValue) -> Option<Box<CffPrivateDict>> {
    if dump.is_null()
        || json_type_of(dump) != JsonType::Object
    {
        return None;
    }
    let mut pd_box: Box<CffPrivateDict> = otfcc_new_cff_private();
    let pd: *mut CffPrivateDict = pd_box.as_mut() as *mut CffPrivateDict;
    (*pd).blue_values = pd_delta_from_json(json_obj_get(
        dump,
        b"blueValues\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    (*pd).other_blues = pd_delta_from_json(json_obj_get(
        dump,
        b"otherBlues\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    (*pd).family_blues = pd_delta_from_json(json_obj_get(
        dump,
        b"familyBlues\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    (*pd).family_other_blues = pd_delta_from_json(json_obj_get(
        dump,
        b"familyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    (*pd).stem_snap_h = pd_delta_from_json(json_obj_get(
        dump,
        b"stemSnapH\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    (*pd).stem_snap_v = pd_delta_from_json(json_obj_get(
        dump,
        b"stemSnapV\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    (*pd).blue_scale = json_obj_getnum_fallback(
        dump,
        b"blueScale\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_BLUE_SCALE,
    );
    (*pd).blue_shift = json_obj_getnum_fallback(
        dump,
        b"blueShift\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_BLUE_SHIFT,
    );
    (*pd).blue_fuzz = json_obj_getnum_fallback(
        dump,
        b"blueFuzz\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_BLUE_FUZZ,
    );
    (*pd).std_hw = json_obj_getnum(dump, b"stdHW\0" as *const u8 as *const ::core::ffi::c_char);
    (*pd).std_vw = json_obj_getnum(dump, b"stdVW\0" as *const u8 as *const ::core::ffi::c_char);
    (*pd).force_bold = json_obj_getbool(
        dump,
        b"forceBold\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*pd).language_group = json_obj_getnum(
        dump,
        b"languageGroup\0" as *const u8 as *const ::core::ffi::c_char,
    ) as u32;
    (*pd).expansion_factor = json_obj_getnum_fallback(
        dump,
        b"expansionFactor\0" as *const u8 as *const ::core::ffi::c_char,
        DEFAULT_EXPANSION_FACTOR,
    );
    (*pd).initial_random_seed = json_obj_getnum(
        dump,
        b"initialRandomSeed\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return Some(pd_box);
}
unsafe fn fd_from_json(
    mut dump: *const ParsedValue,
    mut options: *const Options,
    mut top_level: bool,
) -> *mut CffTable {
    let mut table: *mut CffTable = (
        table_cff_create)();
    if dump.is_null()
        || json_type_of(dump) != JsonType::Object
    {
        return table;
    }
    (*table).version = json_obj_getsds(
        dump,
        b"version\0" as *const u8 as *const ::core::ffi::c_char,
    ).unwrap_or_default();
    (*table).notice = json_obj_getsds(dump, b"notice\0" as *const u8 as *const ::core::ffi::c_char).unwrap_or_default();
    (*table).copyright = json_obj_getsds(
        dump,
        b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
    ).unwrap_or_default();
    (*table).font_name = json_obj_getsds(
        dump,
        b"fontName\0" as *const u8 as *const ::core::ffi::c_char,
    ).unwrap_or_default();
    (*table).full_name = json_obj_getsds(
        dump,
        b"fullName\0" as *const u8 as *const ::core::ffi::c_char,
    ).unwrap_or_default();
    (*table).family_name = json_obj_getsds(
        dump,
        b"familyName\0" as *const u8 as *const ::core::ffi::c_char,
    ).unwrap_or_default();
    (*table).weight = json_obj_getsds(dump, b"weight\0" as *const u8 as *const ::core::ffi::c_char).unwrap_or_default();
    (*table).is_fixed_pitch = json_obj_getbool(
        dump,
        b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).italic_angle = json_obj_getnum(
        dump,
        b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).underline_position = json_obj_getnum_fallback(
        dump,
        b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
        -100.0f64,
    );
    (*table).underline_thickness = json_obj_getnum_fallback(
        dump,
        b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
        50.0f64,
    );
    (*table).stroke_width = json_obj_getnum(
        dump,
        b"strokeWidth\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).font_b_box_left = json_obj_getnum(
        dump,
        b"fontBBoxLeft\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).font_b_box_bottom = json_obj_getnum(
        dump,
        b"fontBBoxBottom\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).font_b_box_right = json_obj_getnum(
        dump,
        b"fontBBoxRight\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).font_b_box_top = json_obj_getnum(
        dump,
        b"fontBBoxTop\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).private_dict = pd_from_json(json_obj_get_type(
        dump,
        b"privates\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    ));
    (*table).cid_registry = json_obj_getsds(
        dump,
        b"cidRegistry\0" as *const u8 as *const ::core::ffi::c_char,
    ).unwrap_or_default();
    (*table).cid_ordering = json_obj_getsds(
        dump,
        b"cidOrdering\0" as *const u8 as *const ::core::ffi::c_char,
    ).unwrap_or_default();
    (*table).cid_supplement = json_obj_getint(
        dump,
        b"cidSupplement\0" as *const u8 as *const ::core::ffi::c_char,
    ) as u32;
    (*table).uid_base = json_obj_getint(
        dump,
        b"UIDBase\0" as *const u8 as *const ::core::ffi::c_char,
    ) as u32;
    (*table).cid_count = json_obj_getint(
        dump,
        b"cidCount\0" as *const u8 as *const ::core::ffi::c_char,
    ) as u32;
    (*table).cid_font_version = json_obj_getnum(
        dump,
        b"cidFontVersion\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*table).cid_font_revision = json_obj_getnum(
        dump,
        b"cidFontRevision\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut fdarraydump: *const ParsedValue = json_obj_get_type(
        dump,
        b"fdArray\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !fdarraydump.is_null() {
        (*table).is_cid = true;
        let fd_count = json_obj_len(fdarraydump) as usize;
        (*table).fd_array = Vec::with_capacity(fd_count);
        let mut j: TableId = 0 as TableId;
        while (j as usize) < fd_count {
            // `fd_from_json` builds each child fully before returning
            // (unlike the binary-read path, which populates a `fd_array`
            // slot incrementally via a recursive callback) -- so there's
            // no need to push an empty placeholder first here.
            let mut fd_box: Box<CffTable> = unwrap_cff_table(fd_from_json(
                json_obj_val_at(fdarraydump, j as u32),
                options,
                false,
            ))
            .unwrap();
            fd_box.font_name = ::core::slice::from_raw_parts(
                json_obj_key_at(fdarraydump, j as u32) as *const u8,
                json_obj_key_len_at(fdarraydump, j as u32) as usize,
            ).to_vec();
            (*table).fd_array.push(fd_box);
            j = j.wrapping_add(1);
        }
    }
    if (*table).font_name.is_empty() {
        (*table).font_name = b"CARYLL_CFFFONT".to_vec();
    }
    if (*table).private_dict.is_none() {
        (*table).private_dict = Some(otfcc_new_cff_private());
    }
    if top_level as ::core::ffi::c_int != 0
        && (*options).force_cid as ::core::ffi::c_int != 0
        && (*table).fd_array.is_empty()
    {
        let mut fd0_box: Box<CffTable> = unwrap_cff_table(
            (table_cff_create)(),
        )
        .unwrap();
        fd0_box.private_dict = (*table).private_dict.take();
        (*table).private_dict = Some(otfcc_new_cff_private());
        let mut subfont0_name = (*table).font_name.clone();
        subfont0_name.extend_from_slice(b"-subfont0");
        fd0_box.font_name = subfont0_name;
        (*table).fd_array.push(fd0_box);
        (*table).is_cid = true;
    }
    if (*table).is_cid as ::core::ffi::c_int != 0 && (*table).cid_registry.is_empty() {
        (*table).cid_registry = b"CARYLL".to_vec();
    }
    if (*table).is_cid as ::core::ffi::c_int != 0 && (*table).cid_ordering.is_empty() {
        (*table).cid_ordering = b"OTFCCAUTOCID".to_vec();
    }
    return table;
}
pub unsafe fn otfcc_parse_cff(
    mut root: *const ParsedValue,
    mut options: *const Options,
) -> Option<Box<CffTable>> {
    let mut dump: *const ParsedValue = json_obj_get_type(
        root,
        b"CFF_\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if dump.is_null() {
        return None;
    } else {
        let mut cff: *mut CffTable = ::core::ptr::null_mut::<CffTable>();
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::bytesbuild!(b"CFF"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            cff = fd_from_json(dump, options, true);
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
        return unwrap_cff_table(cff);
    };
}
unsafe fn cff_make_charstrings(
    mut context: *mut CffCharstringBuilderContext,
    mut s: *mut *mut Buffer,
    mut gs: *mut *mut Buffer,
    mut ls: *mut *mut Buffer,
) {
    if (*(*context).glyf).is_empty() {
        return;
    }
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*(*context).glyf).len() {
        let mut il: *mut CffCharstringIl = cff_compile_glyph_to_il(
            (&(*(*context).glyf))[j as usize].as_deref().unwrap() as *const Glyph,
            (*context).default_width,
            (*context).nominal_width_x,
        );
        cff_optimize_il(il, (*context).options);
        cff_insert_il_to_graph(&raw mut (*context).graph, il);
        drop(Box::from_raw(il));
        il = ::core::ptr::null_mut::<CffCharstringIl>();
        j = j.wrapping_add(1);
    }
    cff_il_graph_to_buffers(&raw mut (*context).graph, s, gs, ls, (*context).options);
}
// Deduplicates by string content, first registration wins -- returns the
// existing SID if the string was already registered, otherwise assigns
// the next sequential SID (391 + the map's current length, matching the
// original's `391 + HASH_COUNT`-at-insert-time scheme exactly, since
// registration order is SID order here). `by_sid` fed a `HASH_SORT`
// before `cffstrings_to_indexblob` iterated (see there), so output order
// is ascending SID -- which is registration order by construction, so an
// `IndexMap` (insertion-ordered) needs no separate sort step the way
// `BTreeMap`-based instances did.
//
// The dedup key is `strlen`-bounded bytes (matching the original's Bob
// Jenkins hash + `memcmp`, both driven by `strlen(s)`), not the full sds
// length -- glyph names and font metadata strings can be arbitrary,
// non-ASCII byte content in this codebase (see this file's other notes
// on `%s`/non-UTF-8 glyph names), so an embedded NUL in a name is a real,
// if obscure, possibility worth preserving faithfully rather than
// assumed away. The *stored* value is still the full byte content of
// `s`, matching what the original's `.str_0` held and what `bufnwrite8`
// (in `cffstrings_to_indexblob`) writes out for real --
// so two strings identical only up to their first NUL byte are still
// treated as the same string for dedup purposes (the original's exact
// behavior), but the winning entry's full byte content, NUL and all, is
// still what ends up in the output.
unsafe fn sidof(h: *mut indexmap::IndexMap<Vec<u8>, Vec<u8>>, s: &[u8]) -> ::core::ffi::c_int {
    let key: Vec<u8> = match s.iter().position(|&b| b == 0) {
        Some(p) => s[..p].to_vec(),
        None => s.to_vec(),
    };
    if let Some(idx) = (*h).get_index_of(&key) {
        return 391 as ::core::ffi::c_int + idx as ::core::ffi::c_int;
    }
    let idx = (*h).len();
    (*h).insert(key, s.to_vec());
    return 391 as ::core::ffi::c_int + idx as ::core::ffi::c_int;
}
unsafe fn cffdict_givemeablank(mut dict: *mut CffDict) -> *mut CffDictEntry {
    (*dict).ents.push(CffDictEntry {
        op: CffDictOperator(0),
        vals: Vec::new(),
    });
    return (*dict).ents.last_mut().unwrap() as *mut CffDictEntry;
}
/// Append a DICT entry whose operands are numbers.
///
/// Was `cffdict_input(dict, op, t, arity, ...)`: a count, a value type, and that
/// many varargs read as `c_double` or `c_int` depending on `t`. Every one of the
/// 30 call sites passes either `CffValueType::Double` with `Pos` operands or
/// `CffValueType::Integer` with integer ones, so the runtime branch on `t` is really two
/// functions -- this one and [`cffdict_input_ints`] -- and the count is the
/// slice's length.
unsafe fn cffdict_input_doubles(dict: *mut CffDict, op: CffDictOperator, values: &[f64]) {
    let last: *mut CffDictEntry = cffdict_givemeablank(dict);
    (*last).op = op;
    let mut vals: Vec<CffValue> = Vec::with_capacity(values.len());
    for &x in values.iter() {
        // A whole number is stored as an integer, which is what decides whether
        // the DICT is encoded with an integer or a real operand later.
        vals.push(if x == round(x) {
            CffValue {
                t: CffValueType::Integer,
                c2rust_unnamed: CffValueBody { i: round(x) as i32 },
            }
        } else {
            CffValue {
                t: CffValueType::Double,
                c2rust_unnamed: CffValueBody { d: x },
            }
        });
    }
    (*last).vals = vals;
}

/// Append a DICT entry whose operands are integers. See [`cffdict_input_doubles`].
unsafe fn cffdict_input_ints(dict: *mut CffDict, op: CffDictOperator, values: &[i32]) {
    let last: *mut CffDictEntry = cffdict_givemeablank(dict);
    (*last).op = op;
    let mut vals: Vec<CffValue> = Vec::with_capacity(values.len());
    for &x in values.iter() {
        vals.push(CffValue {
            t: CffValueType::Integer,
            c2rust_unnamed: CffValueBody { i: x },
        });
    }
    (*last).vals = vals;
}

unsafe fn cffdict_input_array(
    dict: *mut CffDict,
    op: CffDictOperator,
    t: CffValueType,
    arr: &[::core::ffi::c_double],
) {
    if arr.is_empty() {
        return;
    }
    let last: *mut CffDictEntry = cffdict_givemeablank(dict);
    (*last).op = op;
    let mut vals: Vec<CffValue> = Vec::with_capacity(arr.len());
    for &x in arr.iter() {
        vals.push(if t == CffValueType::Double {
            if x == round(x) {
                CffValue {
                    t: CffValueType::Integer,
                    c2rust_unnamed: CffValueBody { i: round(x) as i32 },
                }
            } else {
                CffValue {
                    t: CffValueType::Double,
                    c2rust_unnamed: CffValueBody { d: x },
                }
            }
        } else {
            CffValue {
                t,
                c2rust_unnamed: CffValueBody { i: round(x) as i32 },
            }
        });
    }
    (*last).vals = vals;
}
unsafe fn cff_make_fd_dict(
    mut fd: *mut CffTable,
    mut h: *mut indexmap::IndexMap<Vec<u8>, Vec<u8>>,
) -> *mut CffDict {
    let mut dict: *mut CffDict = (
        CFF_I_DICT.create.expect("non-null function pointer"))();
    if !(*fd).cid_registry.is_empty() && !(*fd).cid_ordering.is_empty() {
        cffdict_input_ints(dict, OP_ROS, &[(sidof(h, &(*fd).cid_registry)) as i32, (sidof(h, &(*fd).cid_ordering)) as i32, ((*fd).cid_supplement) as i32]);
    }
    if !(*fd).version.is_empty() {
        cffdict_input_ints(dict, OP_VERSION, &[(sidof(h, &(*fd).version)) as i32]);
    }
    if !(*fd).notice.is_empty() {
        cffdict_input_ints(dict, OP_NOTICE, &[(sidof(h, &(*fd).notice)) as i32]);
    }
    if !(*fd).copyright.is_empty() {
        cffdict_input_ints(dict, OP_COPYRIGHT, &[(sidof(h, &(*fd).copyright)) as i32]);
    }
    if !(*fd).full_name.is_empty() {
        cffdict_input_ints(dict, OP_FULL_NAME, &[(sidof(h, &(*fd).full_name)) as i32]);
    }
    if !(*fd).family_name.is_empty() {
        cffdict_input_ints(dict, OP_FAMILY_NAME, &[(sidof(h, &(*fd).family_name)) as i32]);
    }
    if !(*fd).weight.is_empty() {
        cffdict_input_ints(dict, OP_WEIGHT, &[(sidof(h, &(*fd).weight)) as i32]);
    }
    cffdict_input_doubles(dict, OP_FONT_BBOX, &[((*fd).font_b_box_left) as f64, ((*fd).font_b_box_bottom) as f64, ((*fd).font_b_box_right) as f64, ((*fd).font_b_box_top) as f64]);
    cffdict_input_ints(dict, OP_IS_FIXED_PITCH, &[((*fd).is_fixed_pitch as ::core::ffi::c_int) as i32]);
    cffdict_input_doubles(dict, OP_ITALIC_ANGLE, &[((*fd).italic_angle) as f64]);
    cffdict_input_doubles(dict, OP_UNDERLINE_POSITION, &[((*fd).underline_position) as f64]);
    cffdict_input_doubles(dict, OP_UNDERLINE_THICKNESS, &[((*fd).underline_thickness) as f64]);
    cffdict_input_doubles(dict, OP_STROKE_WIDTH, &[((*fd).stroke_width) as f64]);
    if let Some(fm) = (*fd).font_matrix.as_deref() {
        cffdict_input_doubles(dict, OP_FONT_MATRIX, &[fm.a as f64, fm.b as f64, fm.c as f64, fm.d as f64, (I_VQ.get_still.expect("non-null function pointer")(fm.x.clone())) as f64, (I_VQ.get_still.expect("non-null function pointer")(fm.y.clone())) as f64]);
    }
    if !(*fd).font_name.is_empty() {
        cffdict_input_ints(dict, OP_FONT_NAME, &[(sidof(h, &(*fd).font_name)) as i32]);
    }
    if (*fd).cid_font_version != 0. {
        cffdict_input_doubles(dict, OP_CID_FONT_VERSION, &[((*fd).cid_font_version) as f64]);
    }
    if (*fd).cid_font_revision != 0. {
        cffdict_input_doubles(dict, OP_CID_FONT_REVISION, &[((*fd).cid_font_revision) as f64]);
    }
    if (*fd).cid_count != 0 {
        cffdict_input_ints(dict, OP_CID_COUNT, &[((*fd).cid_count) as i32]);
    }
    if (*fd).uid_base != 0 {
        cffdict_input_ints(dict, OP_UID_BASE, &[((*fd).uid_base) as i32]);
    }
    return dict;
}
unsafe fn cff_make_private_dict(mut pd: *mut CffPrivateDict) -> *mut CffDict {
    // Was `__caryll_allocate_clean` (calloc) -- unsound now that `CffDict`
    // owns `ents: Vec<CffDictEntry>`; an all-zero bit pattern is not a
    // valid `Vec`. `cff_make_fd_dict` two functions above already gets
    // this right via `CFF_I_DICT.create()`; this call site was missed.
    let dict: *mut CffDict = (CFF_I_DICT.create.expect("non-null function pointer"))();
    if pd.is_null() {
        return dict;
    }
    cffdict_input_array(
        dict,
        OP_BLUE_VALUES,
        CffValueType::Double,
        &(*pd).blue_values,
    );
    cffdict_input_array(
        dict,
        OP_OTHER_BLUES,
        CffValueType::Double,
        &(*pd).other_blues,
    );
    cffdict_input_array(
        dict,
        OP_FAMILY_BLUES,
        CffValueType::Double,
        &(*pd).family_blues,
    );
    cffdict_input_array(
        dict,
        OP_FAMILY_OTHER_BLUES,
        CffValueType::Double,
        &(*pd).family_other_blues,
    );
    cffdict_input_array(
        dict,
        OP_STEM_SNAP_H,
        CffValueType::Double,
        &(*pd).stem_snap_h,
    );
    cffdict_input_array(
        dict,
        OP_STEM_SNAP_V,
        CffValueType::Double,
        &(*pd).stem_snap_v
    );
    cffdict_input_doubles(dict, OP_BLUE_SCALE, &[((*pd).blue_scale) as f64]);
    cffdict_input_doubles(dict, OP_BLUE_SHIFT, &[((*pd).blue_shift) as f64]);
    cffdict_input_doubles(dict, OP_BLUE_FUZZ, &[((*pd).blue_fuzz) as f64]);
    cffdict_input_doubles(dict, OP_STD_HW, &[((*pd).std_hw) as f64]);
    cffdict_input_doubles(dict, OP_STD_VW, &[((*pd).std_vw) as f64]);
    cffdict_input_ints(dict, OP_FORCE_BOLD, &[((*pd).force_bold as ::core::ffi::c_int) as i32]);
    cffdict_input_ints(dict, OP_LANGUAGE_GROUP, &[((*pd).language_group) as i32]);
    cffdict_input_doubles(dict, OP_EXPANSION_FACTOR, &[((*pd).expansion_factor) as f64]);
    cffdict_input_doubles(dict, OP_INITIAL_RANDOM_SEED, &[((*pd).initial_random_seed) as f64]);
    cffdict_input_doubles(dict, OP_DEFAULT_WIDTH_X, &[((*pd).default_width_x) as f64]);
    cffdict_input_doubles(dict, OP_NOMINAL_WIDTH_X, &[((*pd).nominal_width_x) as f64]);
    return dict;
}
unsafe extern "C" fn callback_makestringindex(
    mut context: *mut ::core::ffi::c_void,
    mut i: u32,
) -> *mut Buffer {
    let mut blobs: *mut *mut Buffer = context as *mut *mut Buffer;
    return *blobs.offset(i as isize);
}
unsafe fn cffstrings_to_indexblob(h: *mut indexmap::IndexMap<Vec<u8>, Vec<u8>>) -> *mut Buffer {
    let n: u32 = (*h).len() as u32;
    // `blobs` was `__caryll_allocate_clean`'d/`free`'d, holding just the
    // pointer array `callback_makestringindex` indexes into through its
    // `*mut c_void` context -- freeing it (now: dropping the `Vec`) never
    // touches the `Buffer`s it points to, which `from_callback`/`build`
    // below consume on their own.
    let mut blobs: Vec<*mut Buffer> = Vec::with_capacity(n as usize);
    // `IndexMap`'s iteration order is insertion order, which is SID
    // order by construction (`sidof` assigns each new string the next
    // sequential SID), so no separate sort step is needed here the way
    // the original's `HASH_SORT` (via `by_sid`) was.
    for (_, value) in ::core::mem::take(&mut *h) {
        let blob: *mut Buffer = bufnew();
        bufnwrite8(blob, &value);
        blobs.push(blob);
    }
    let mut strings: *mut CffIndex = CFF_I_INDEX.from_callback.expect("non-null function pointer")(
        blobs.as_mut_ptr() as *mut ::core::ffi::c_void,
        n,
        Some(
            callback_makestringindex
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
        ),
    );
    let mut final_blob: *mut Buffer =
        CFF_I_INDEX.build.expect("non-null function pointer")(strings);
    CFF_I_INDEX.free.expect("non-null function pointer")(strings);
    (*final_blob).cursor = (*final_blob).size;
    return final_blob;
}
unsafe fn cff_compile_nameindex(mut cff: *mut CffTable) -> *mut Buffer {
    let mut name_index: *mut CffIndex = (
        CFF_I_INDEX.create.expect("non-null function pointer"))();
    (*name_index).count = 1 as Arity;
    (*name_index).off_size = 4 as u8;
    if (*cff).font_name.is_empty() {
        (*cff).font_name = b"Caryll-CFF-FONT".to_vec();
    }
    (*name_index).offset = vec![
        1 as u32,
        (*cff).font_name.len().wrapping_add(1 as usize) as u32,
    ];
    // Was `__caryll_allocate_clean`'d to `font_name.len() + 1` bytes but
    // only `font_name.len()` of them ever `memcpy`'d -- the trailing byte
    // stayed zero. `.push(0)` reproduces that exact trailing NUL.
    let mut name_data: Vec<u8> = (*cff).font_name.clone();
    name_data.push(0 as u8);
    (*name_index).data = name_data;
    let mut buf: *mut Buffer =
        CFF_I_INDEX.build.expect("non-null function pointer")(name_index);
    CFF_I_INDEX.free.expect("non-null function pointer")(name_index);
    (*cff).font_name = Vec::new();
    return buf;
}
unsafe fn cff_make_charset(
    mut cff: *mut CffTable,
    mut glyf: *mut GlyfTable,
    mut string_hash: *mut indexmap::IndexMap<Vec<u8>, Vec<u8>>,
) -> *mut Buffer {
    let charset: CffCharset = if (*glyf).len() > 1 as usize {
        let (first, nleft) = if (*cff).is_cid {
            (1 as u16, (*glyf).len().wrapping_sub(2 as usize) as u16)
        } else {
            let mut j: GlyphId = 1 as GlyphId;
            while (j as usize) < (*glyf).len() {
                sidof(string_hash, &(&(*glyf))[j as usize].as_deref().unwrap().name);
                j = j.wrapping_add(1);
            }
            (
                sidof(string_hash, &(&(*glyf))[1 as usize].as_deref().unwrap().name) as u16,
                (*glyf).len().wrapping_sub(2 as usize) as u16,
            )
        };
        CffCharset::Format2(vec![CffCharsetRangeFormat2 { first, nleft }])
    } else {
        CffCharset::IsoAdobe
    };
    let c: *mut Buffer = cff_build_charset(&charset);
    return c;
}
// The `range3` array's final length isn't pre-counted anymore -- a `Vec`
// absorbs the counting pass, same as `Coverage`/`ClassDef`/`gpos_pair.rs`'s
// own scratch-buffer conversions. `.s`'s old dual role (a running write
// cursor through the loop, overwritten with the final range count right
// after) collapses into a single sequential `.push()` per transition.
unsafe fn cff_make_fdselect(
    mut cff: *mut CffTable,
    mut glyf: *mut GlyfTable,
) -> *mut Buffer {
    if !(*cff).is_cid {
        return bufnew();
    }
    let fds: CffFdSelect = if !(*glyf).is_empty() {
        let mut fdi0: u8 = (&(*glyf))[0 as usize].as_deref().unwrap().fd_select.index as u8;
        if fdi0 as usize > (*cff).fd_array.len() {
            fdi0 = 0 as u8;
        }
        let mut current: u8 = fdi0;
        let mut range3: Vec<CffFdSelectRangeFormat3> =
            vec![CffFdSelectRangeFormat3 { first: 0 as u16, fd: current }];
        let mut j: GlyphId = 1 as GlyphId;
        while (j as usize) < (*glyf).len() {
            let mut fdi: u8 = (&(*glyf))[j as usize].as_deref().unwrap().fd_select.index as u8;
            if fdi as usize > (*cff).fd_array.len() {
                fdi = 0 as u8;
            }
            if fdi as ::core::ffi::c_int != current as ::core::ffi::c_int {
                current = fdi;
                range3.push(CffFdSelectRangeFormat3 { first: j as u16, fd: current });
            }
            j = j.wrapping_add(1);
        }
        CffFdSelect::Format3 { range3, sentinel: (*glyf).len() as u16 }
    } else {
        CffFdSelect::Unspecified
    };
    let e: *mut Buffer = cff_build_fd_select(&fds);
    return e;
}
unsafe extern "C" fn callback_makefd(
    mut _context: *mut ::core::ffi::c_void,
    mut i: u32,
) -> *mut Buffer {
    let mut context: *mut FdArrayCompileContext = _context as *mut FdArrayCompileContext;
    let mut fd: *mut CffDict = cff_make_fd_dict(
        (&(*(*context).fd_array))[i as usize].as_ref() as *const CffTable as *mut CffTable,
        (*context).string_hash,
    );
    let mut blob: *mut Buffer = CFF_I_DICT.build.expect("non-null function pointer")(fd);
    bufwrite_bufdel(
        blob,
        cff_build_offset(0xeeeeeeee as ::core::ffi::c_uint as i32),
    );
    bufwrite_bufdel(
        blob,
        cff_build_offset(0xffffffff as ::core::ffi::c_uint as i32),
    );
    bufwrite_bufdel(
        blob,
        cff_encode_cff_operator(OP_PRIVATE),
    );
    CFF_I_DICT.build.expect("non-null function pointer")(fd);
    return blob;
}
unsafe fn cff_make_fdarray(
    mut fd_array: *const Vec<Box<CffTable>>,
    mut string_hash: *mut indexmap::IndexMap<Vec<u8>, Vec<u8>>,
) -> *mut CffIndex {
    let mut context: FdArrayCompileContext = FdArrayCompileContext {
        fd_array: ::core::ptr::null::<Vec<Box<CffTable>>>(),
        string_hash: ::core::ptr::null_mut::<indexmap::IndexMap<Vec<u8>, Vec<u8>>>(),
    };
    context.fd_array = fd_array;
    context.string_hash = string_hash;
    return CFF_I_INDEX.from_callback.expect("non-null function pointer")(
        &raw mut context as *mut ::core::ffi::c_void,
        (*fd_array).len() as u32,
        Some(
            callback_makefd
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, u32) -> *mut Buffer,
        ),
    );
}
unsafe fn writecff_cid_keyed(
    mut cff: *mut CffTable,
    mut glyf: *mut GlyfTable,
    mut options: *const Options,
) -> *mut Buffer {
    let mut blob: *mut Buffer = bufnew();
    let mut string_hash: indexmap::IndexMap<Vec<u8>, Vec<u8>> = indexmap::IndexMap::new();
    let mut h: *mut Buffer = cff_build_header();
    let mut n: *mut Buffer = cff_compile_nameindex(cff);
    let mut top: *mut CffDict = cff_make_fd_dict(cff, &raw mut string_hash);
    let mut t: *mut Buffer = CFF_I_DICT.build.expect("non-null function pointer")(top);
    CFF_I_DICT.free.expect("non-null function pointer")(top);
    let mut top_pd: *mut CffDict = cff_make_private_dict((*cff).private_dict.as_deref_mut().map_or(::core::ptr::null_mut(), |pd| pd as *mut CffPrivateDict));
    let mut p: *mut Buffer = CFF_I_DICT.build.expect("non-null function pointer")(top_pd);
    bufwrite_bufdel(
        p,
        cff_build_offset(0xffffffff as ::core::ffi::c_uint as i32),
    );
    bufwrite_bufdel(
        p,
        cff_encode_cff_operator(OP_SUBRS),
    );
    CFF_I_DICT.free.expect("non-null function pointer")(top_pd);
    let mut e: *mut Buffer = cff_make_fdselect(cff, glyf);
    let mut fd_array_index: *mut CffIndex = ::core::ptr::null_mut::<CffIndex>();
    let mut r: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    if (*cff).is_cid {
        fd_array_index = cff_make_fdarray(&raw const (*cff).fd_array, &raw mut string_hash);
        r = CFF_I_INDEX.build.expect("non-null function pointer")(fd_array_index);
    } else {
        r = __caryll_allocate_clean(
            ::core::mem::size_of::<Buffer>() as usize,
            1265 as ::core::ffi::c_ulong,
        ) as *mut Buffer;
    }
    let mut c: *mut Buffer = cff_make_charset(cff, glyf, &raw mut string_hash);
    let mut i: *mut Buffer = cffstrings_to_indexblob(&raw mut string_hash);
    let mut s: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    let mut gs: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    let mut ls: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    let mut g2c_context: CffCharstringBuilderContext = CffCharstringBuilderContext {
        glyf: ::core::ptr::null_mut::<GlyfTable>(),
        default_width: 0,
        nominal_width_x: 0,
        options: ::core::ptr::null::<Options>(),
        graph: CffSubrGraph {
            root: ::core::ptr::null_mut::<CffSubrRule>(),
            last: ::core::ptr::null_mut::<CffSubrRule>(),
            diagram_index: std::collections::HashMap::new(),
            total_rules: 0,
            total_char_strings: 0,
            do_subroutinize: false,
        },
    };
    g2c_context.glyf = glyf;
    g2c_context.default_width = (*cff).private_dict.as_deref().unwrap().default_width_x as u16;
    g2c_context.nominal_width_x = (*cff).private_dict.as_deref().unwrap().nominal_width_x as u16;
    g2c_context.options = options;
    cff_subr_graph_init(&raw mut g2c_context.graph);
    g2c_context.graph.do_subroutinize = (*options).cff_do_subroutinize;
    cff_make_charstrings(&raw mut g2c_context, &raw mut s, &raw mut gs, &raw mut ls);
    cff_subr_graph_dispose(&raw mut g2c_context.graph);
    let mut additional_top_dict_ops_size: u32 = 0 as u32;
    let mut off: u32 = (*h)
        .size
        .wrapping_add((*n).size)
        .wrapping_add(11 as usize)
        .wrapping_add((*t).size) as u32;
    if (*c).size != 0 as usize {
        additional_top_dict_ops_size = additional_top_dict_ops_size.wrapping_add(6 as u32);
    }
    if (*e).size != 0 as usize {
        additional_top_dict_ops_size = additional_top_dict_ops_size.wrapping_add(7 as u32);
    }
    if (*s).size != 0 as usize {
        additional_top_dict_ops_size = additional_top_dict_ops_size.wrapping_add(6 as u32);
    }
    if (*p).size != 0 as usize {
        additional_top_dict_ops_size = additional_top_dict_ops_size.wrapping_add(11 as u32);
    }
    if (*r).size != 0 as usize {
        additional_top_dict_ops_size = additional_top_dict_ops_size.wrapping_add(7 as u32);
    }
    bufwrite_bufdel(blob, h);
    bufwrite_bufdel(blob, n);
    let mut delta_size: i32 = (*t)
        .size
        .wrapping_add(additional_top_dict_ops_size as usize)
        .wrapping_add(1 as usize) as u32 as i32;
    bufwrite_bufdel(
        blob,
        bufninit(&[0 as u8, 1 as u8, 4 as u8, 0 as u8, 0 as u8, 0 as u8, 1 as u8, (delta_size >> 24 as ::core::ffi::c_int & 0xff as i32) as u8, (delta_size >> 16 as ::core::ffi::c_int & 0xff as i32) as u8, (delta_size >> 8 as ::core::ffi::c_int & 0xff as i32) as u8, (delta_size & 0xff as i32) as u8]),
    );
    bufwrite_bufdel(blob, t);
    off = (off as usize).wrapping_add(
        (additional_top_dict_ops_size as usize)
            .wrapping_add((*i).size)
            .wrapping_add((*gs).size),
    ) as u32 as u32;
    if (*c).size != 0 as usize {
        bufwrite_bufdel(blob, cff_build_offset(off as i32));
        bufwrite_bufdel(
            blob,
            cff_encode_cff_operator(OP_CHARSET),
        );
        off = (off as usize).wrapping_add((*c).size) as u32 as u32;
    }
    if (*e).size != 0 as usize {
        bufwrite_bufdel(blob, cff_build_offset(off as i32));
        bufwrite_bufdel(
            blob,
            cff_encode_cff_operator(OP_FD_SELECT),
        );
        off = (off as usize).wrapping_add((*e).size) as u32 as u32;
    }
    if (*s).size != 0 as usize {
        bufwrite_bufdel(blob, cff_build_offset(off as i32));
        bufwrite_bufdel(
            blob,
            cff_encode_cff_operator(OP_CHAR_STRINGS),
        );
        off = (off as usize).wrapping_add((*s).size) as u32 as u32;
    }
    if (*p).size != 0 as usize {
        bufwrite_bufdel(blob, cff_build_offset((*p).size as u32 as i32));
        bufwrite_bufdel(blob, cff_build_offset(off as i32));
        bufwrite_bufdel(
            blob,
            cff_encode_cff_operator(OP_PRIVATE),
        );
        off = (off as usize).wrapping_add((*p).size) as u32 as u32;
    }
    if (*r).size != 0 as usize {
        bufwrite_bufdel(blob, cff_build_offset(off as i32));
        bufwrite_bufdel(
            blob,
            cff_encode_cff_operator(OP_FD_ARRAY),
        );
        off = (off as usize).wrapping_add((*r).size) as u32 as u32;
    }
    bufwrite_bufdel(blob, i);
    bufwrite_bufdel(blob, gs);
    bufwrite_bufdel(blob, c);
    bufwrite_bufdel(blob, e);
    bufwrite_bufdel(blob, s);
    let mut starting_position_of_privates: Vec<usize> = vec![0; 1 + (*cff).fd_array.len()];
    starting_position_of_privates[0] = (*blob).cursor;
    bufwrite_bufdel(blob, p);
    let mut ending_position_of_privates: Vec<usize> = vec![0; 1 + (*cff).fd_array.len()];
    ending_position_of_privates[0] = (*blob).cursor;
    if (*cff).is_cid {
        let mut fd_array_privates_start_offset: u32 = off;
        let mut fd_array_privates: Vec<*mut Buffer> = vec![::core::ptr::null_mut::<Buffer>(); (*cff).fd_array.len()];
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*cff).fd_array.len() {
            let mut pd: *mut CffDict =
                cff_make_private_dict((&mut (*cff).fd_array)[j as usize].private_dict.as_deref_mut().map_or(::core::ptr::null_mut(), |pd| pd as *mut CffPrivateDict));
            let mut p_0: *mut Buffer =
                CFF_I_DICT.build.expect("non-null function pointer")(pd);
            bufwrite_bufdel(
                p_0,
                cff_build_offset(0xffffffff as ::core::ffi::c_uint as i32),
            );
            bufwrite_bufdel(
                p_0,
                cff_encode_cff_operator(OP_SUBRS),
            );
            CFF_I_DICT.free.expect("non-null function pointer")(pd);
            fd_array_privates[j as usize] = p_0;
            let mut private_length_ptr: *mut u8 = {
                let fd_array_offset = &(*fd_array_index).offset;
                let off = (fd_array_offset
                    [(j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize])
                    .wrapping_sub(11 as u32) as isize;
                (*fd_array_index).data.as_mut_ptr().offset(off) as *mut u8
            };
            *private_length_ptr.offset(0 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 24 as ::core::ffi::c_int & 0xff as usize) as u8;
            *private_length_ptr.offset(1 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 16 as ::core::ffi::c_int & 0xff as usize) as u8;
            *private_length_ptr.offset(2 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 8 as ::core::ffi::c_int & 0xff as usize) as u8;
            *private_length_ptr.offset(3 as ::core::ffi::c_int as isize) =
                ((*p_0).size >> 0 as ::core::ffi::c_int & 0xff as usize) as u8;
            let mut private_offset_ptr: *mut u8 = {
                let fd_array_offset = &(*fd_array_index).offset;
                let off = (fd_array_offset
                    [(j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize])
                    .wrapping_sub(6 as u32) as isize;
                (*fd_array_index).data.as_mut_ptr().offset(off) as *mut u8
            };
            *private_offset_ptr.offset(0 as ::core::ffi::c_int as isize) =
                (fd_array_privates_start_offset >> 24 as ::core::ffi::c_int & 0xff as u32)
                    as u8;
            *private_offset_ptr.offset(1 as ::core::ffi::c_int as isize) =
                (fd_array_privates_start_offset >> 16 as ::core::ffi::c_int & 0xff as u32)
                    as u8;
            *private_offset_ptr.offset(2 as ::core::ffi::c_int as isize) =
                (fd_array_privates_start_offset >> 8 as ::core::ffi::c_int & 0xff as u32)
                    as u8;
            *private_offset_ptr.offset(3 as ::core::ffi::c_int as isize) =
                (fd_array_privates_start_offset >> 0 as ::core::ffi::c_int & 0xff as u32)
                    as u8;
            fd_array_privates_start_offset = (fd_array_privates_start_offset as usize)
                .wrapping_add((*p_0).size) as u32
                as u32;
            j = j.wrapping_add(1);
        }
        buffree(r);
        r = CFF_I_INDEX.build.expect("non-null function pointer")(fd_array_index);
        CFF_I_INDEX.free.expect("non-null function pointer")(fd_array_index);
        bufwrite_bufdel(blob, r);
        let mut j_0: TableId = 0 as TableId;
        while (j_0 as usize) < (*cff).fd_array.len() {
            starting_position_of_privates[(j_0 as usize) + 1] = (*blob).cursor;
            bufwrite_bufdel(blob, fd_array_privates[j_0 as usize]);
            ending_position_of_privates[(j_0 as usize) + 1] = (*blob).cursor;
            j_0 = j_0.wrapping_add(1);
        }
    } else {
        bufwrite_bufdel(blob, r);
    }
    let mut position_of_local_subroutines: usize = (*blob).cursor;
    bufwrite_bufdel(blob, ls);
    let mut j_1: TableId = 0 as TableId;
    while (j_1 as ::core::ffi::c_int)
        < (*cff).fd_array.len() as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    {
        let mut ls_offset: usize = position_of_local_subroutines
            .wrapping_sub(starting_position_of_privates[j_1 as usize]);
        let mut ptr: *mut u8 = (*blob).data.offset(
            (ending_position_of_privates[j_1 as usize]).wrapping_sub(5 as usize) as isize,
        ) as *mut u8;
        *ptr.offset(0 as ::core::ffi::c_int as isize) =
            (ls_offset >> 24 as ::core::ffi::c_int & 0xff as usize) as u8;
        *ptr.offset(1 as ::core::ffi::c_int as isize) =
            (ls_offset >> 16 as ::core::ffi::c_int & 0xff as usize) as u8;
        *ptr.offset(2 as ::core::ffi::c_int as isize) =
            (ls_offset >> 8 as ::core::ffi::c_int & 0xff as usize) as u8;
        *ptr.offset(3 as ::core::ffi::c_int as isize) =
            (ls_offset >> 0 as ::core::ffi::c_int & 0xff as usize) as u8;
        j_1 = j_1.wrapping_add(1);
    }
    return blob;
}
pub unsafe fn otfcc_build_cff(
    cff_and_glyf: CffAndGlyf,
    mut options: *const Options,
) -> *mut Buffer {
    return writecff_cid_keyed(cff_and_glyf.meta, cff_and_glyf.glyphs, options);
}
#[inline]
unsafe fn json_from_sds(str: &[u8]) -> *mut BuiltValue {
    return json_string_new_length(
        str.len() as ::core::ffi::c_uint,
        str.as_ptr() as *const ::core::ffi::c_char,
    );
}
