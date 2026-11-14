pub mod build;
pub mod dump;
pub mod parse;
pub mod read;

pub use build::build_cff;
pub use dump::dump_cff;
pub use parse::parse_cff;
pub use read::read_cff_and_glyf_tables;

use crate::libcff::subr::CffSubrGraph;
use crate::libcff::CffFile;
use crate::support::options::Options;
use crate::support::primitives::{Scale, ShapeId};
use crate::table::glyf::{
    GlyfTable, Glyph,
};

use crate::vf::vq::VQ;

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
pub const DEFAULT_BLUE_SCALE: f64 = 0.039625;
pub const DEFAULT_BLUE_SHIFT: f64 = 7.0;
pub const DEFAULT_BLUE_FUZZ: f64 = 1.0;
pub const DEFAULT_EXPANSION_FACTOR: f64 = 0.06;
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
        underline_position: -100.0,
        underline_thickness: 50.0,
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

#[cfg(test)]
mod cff_matrix_no_head_regression_tests {
    use super::build::writecff_cid_keyed;
    use super::read::apply_cff_matrix;
    use super::*;
    use crate::font::sfnt::{Packet, PacketPiece};
    use crate::support::options::Options;
    use crate::table::glyf::{Point, new_glyf_glyph};

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
        let cff_bytes = writecff_cid_keyed(&mut cff, Some(&glyf), &options).unwrap();

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

