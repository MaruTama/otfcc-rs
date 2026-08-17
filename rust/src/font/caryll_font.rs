#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memset};








use crate::support::glyph_order::GlyphOrder;
use crate::table::base::BaseTable;
use crate::table::cff::CffTable;
use crate::table::colr::ColrTable;
use crate::table::cpal::CpalTable;
use crate::table::gdef::GdefTable;
use crate::table::ltsh::LtshTable;
use crate::table::os_2::Os2Table;
use crate::table::svg::SvgTable;
use crate::table::tsi5::{Tsi5Table};
use crate::table::vorg::VorgTable;
use crate::table::_tsi::TsiTable;
use crate::table::cmap::CmapTable;
use crate::table::cvt::CvtTable;
use crate::table::fpgm_prep::FpgmPrepTable;
use crate::table::fvar::{FvarTable};
use crate::table::gasp::GaspTable;
use crate::table::glyf::GlyfTable;
use crate::table::hdmx::{HdmxTable};
use crate::table::head::HeadTable;
use crate::table::hhea::HheaTable;
use crate::table::hmtx::HmtxTable;
use crate::table::maxp::MaxpTable;
use crate::table::meta::types::MetaTable;
use crate::table::name::NameTable;
use crate::table::otl::OtlTable;
use crate::table::post::PostTable;
use crate::table::vdmx::types::{VdmxTable};
use crate::table::vhea::VheaTable;
use crate::table::vmtx::VmtxTable;





// `Copy, Clone` dropped: `Font` gained `ltsh: Option<Box<LtshTable>>` (Stage
// 6-4 pilot), which is never `Copy`. Grepping confirms `Font` is accessed
// exclusively via `*mut Font`/`(*font).field` throughout the crate -- never
// returned, constructed as a value literal, or `.clone()`'d -- so dropping
// the derive is safe (same check as `CffTable`/`NameRecord`/`GlyphOrderEntry`).
#[repr(C)]
pub struct Font {
    pub subtype: FontSubtype,
    pub fvar: Option<Box<FvarTable>>,
    pub head: Option<Box<HeadTable>>,
    pub hhea: Option<Box<HheaTable>>,
    pub maxp: Option<Box<MaxpTable>>,
    pub os_2: Option<Box<Os2Table>>,
    pub hmtx: Option<Box<HmtxTable>>,
    pub post: Option<Box<PostTable>>,
    pub hdmx: Option<Box<HdmxTable>>,
    pub vhea: Option<Box<VheaTable>>,
    pub vmtx: Option<Box<VmtxTable>>,
    pub vorg: Option<Box<VorgTable>>,
    pub cff: Option<Box<CffTable>>,
    pub glyf: Option<GlyfTable>,
    pub cmap: Option<Box<CmapTable>>,
    pub name: Option<NameTable>,
    pub meta: Option<Box<MetaTable>>,
    pub fpgm: Option<Box<FpgmPrepTable>>,
    pub prep: Option<Box<FpgmPrepTable>>,
    pub cvt_: Option<Box<CvtTable>>,
    pub gasp: Option<Box<GaspTable>>,
    pub vdmx: Option<Box<VdmxTable>>,
    pub ltsh: Option<Box<LtshTable>>,
    pub gsub: Option<Box<OtlTable>>,
    pub gpos: Option<Box<OtlTable>>,
    pub gdef: Option<Box<GdefTable>>,
    pub base: Option<Box<BaseTable>>,
    pub cpal: Option<Box<CpalTable>>,
    pub colr: Option<ColrTable>,
    pub svg: Option<SvgTable>,
    pub tsi_01: Option<TsiTable>,
    pub tsi_23: Option<TsiTable>,
    pub tsi5: Option<Box<Tsi5Table>>,
    pub glyph_order: Option<Box<GlyphOrder>>,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontSubtype {
    Ttf = 0,
    Cff = 1,
}
pub(crate) unsafe fn delete_font_table(mut font: *mut Font, tag: u32) {
    match tag {
        crate::tag::TAG_HEAD => {
            (*font).head = None;
            return;
        }
        crate::tag::TAG_HHEA => {
            (*font).hhea = None;
            return;
        }
        crate::tag::TAG_MAXP => {
            (*font).maxp = None;
            return;
        }
        crate::tag::TAG_OS_2_ALT | crate::tag::TAG_OS_2 => {
            (*font).os_2 = None;
            return;
        }
        crate::tag::TAG_NAME => {
            (*font).name = None;
            return;
        }
        crate::tag::TAG_META => {
            (*font).meta = None;
            return;
        }
        crate::tag::TAG_HMTX => {
            (*font).hmtx = None;
            return;
        }
        crate::tag::TAG_VMTX => {
            (*font).vmtx = None;
            return;
        }
        crate::tag::TAG_POST => {
            (*font).post = None;
            return;
        }
        crate::tag::TAG_VHEA => {
            (*font).vhea = None;
            return;
        }
        crate::tag::TAG_FPGM => {
            (*font).fpgm = None;
            return;
        }
        crate::tag::TAG_PREP => {
            (*font).prep = None;
            return;
        }
        crate::tag::TAG_CVT_ALT | crate::tag::TAG_CVT => {
            (*font).cvt_ = None;
            return;
        }
        crate::tag::TAG_GASP => {
            (*font).gasp = None;
            return;
        }
        crate::tag::TAG_CFF_ALT | crate::tag::TAG_CFF => {
            (*font).cff = None;
            return;
        }
        crate::tag::TAG_GLYF => {
            (*font).glyf = None;
            return;
        }
        crate::tag::TAG_CMAP => {
            (*font).cmap = None;
            return;
        }
        crate::tag::TAG_LTSH => {
            (*font).ltsh = None;
            return;
        }
        crate::tag::TAG_GSUB => {
            (*font).gsub = None;
            return;
        }
        crate::tag::TAG_GPOS => {
            (*font).gpos = None;
            return;
        }
        crate::tag::TAG_GDEF => {
            (*font).gdef = None;
            return;
        }
        crate::tag::TAG_BASE => {
            (*font).base = None;
            return;
        }
        crate::tag::TAG_VORG => {
            (*font).vorg = None;
            return;
        }
        crate::tag::TAG_CPAL => {
            (*font).cpal = None;
            return;
        }
        crate::tag::TAG_COLR => {
            (*font).colr = None;
            return;
        }
        crate::tag::TAG_SVG | crate::tag::TAG_SVG_ALT => {
            (*font).svg = None;
            return;
        }
        crate::tag::TAG_TSI0 | crate::tag::TAG_TSI1 => {
            (*font).tsi_01 = None;
            return;
        }
        crate::tag::TAG_TSI2 | crate::tag::TAG_TSI3 => {
            (*font).tsi_23 = None;
            return;
        }
        crate::tag::TAG_TSI5 => {
            (*font).tsi5 = None;
            return;
        }
        _ => {}
    };
}
#[inline]
unsafe fn init_font(mut font: *mut Font) {
    memset(
        font as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Font>() as usize,
    );
}
#[inline]
unsafe fn dispose_font(mut font: *mut Font) {
    delete_font_table(font, crate::tag::TAG_HEAD);
    delete_font_table(font, crate::tag::TAG_HHEA);
    delete_font_table(font, crate::tag::TAG_MAXP);
    delete_font_table(font, crate::tag::TAG_OS_2_ALT);
    delete_font_table(font, crate::tag::TAG_NAME);
    delete_font_table(font, crate::tag::TAG_META);
    delete_font_table(font, crate::tag::TAG_HMTX);
    delete_font_table(font, crate::tag::TAG_VMTX);
    delete_font_table(font, crate::tag::TAG_POST);
    delete_font_table(font, crate::tag::TAG_HDMX);
    delete_font_table(font, crate::tag::TAG_VHEA);
    delete_font_table(font, crate::tag::TAG_FPGM);
    delete_font_table(font, crate::tag::TAG_PREP);
    delete_font_table(font, crate::tag::TAG_CVT_ALT);
    delete_font_table(font, crate::tag::TAG_GASP);
    delete_font_table(font, crate::tag::TAG_CFF_ALT);
    delete_font_table(font, crate::tag::TAG_GLYF);
    delete_font_table(font, crate::tag::TAG_CMAP);
    delete_font_table(font, crate::tag::TAG_LTSH);
    delete_font_table(font, crate::tag::TAG_GSUB);
    delete_font_table(font, crate::tag::TAG_GPOS);
    delete_font_table(font, crate::tag::TAG_GDEF);
    delete_font_table(font, crate::tag::TAG_BASE);
    delete_font_table(font, crate::tag::TAG_VORG);
    delete_font_table(font, crate::tag::TAG_CPAL);
    delete_font_table(font, crate::tag::TAG_COLR);
    delete_font_table(font, crate::tag::TAG_SVG_ALT);
    delete_font_table(font, crate::tag::TAG_TSI0);
    delete_font_table(font, crate::tag::TAG_TSI2);
    delete_font_table(font, crate::tag::TAG_TSI5);
    (*font).glyph_order = None;
}
#[inline]
unsafe fn otfcc_font_dispose(mut x: *mut Font) {
    dispose_font(x);
}
#[inline]
pub unsafe fn otfcc_font_create() -> *mut Font {
    let mut x: *mut Font =
        malloc(::core::mem::size_of::<Font>() as usize) as *mut Font;
    otfcc_font_init(x);
    return x;
}
#[inline]
unsafe fn otfcc_font_init(mut x: *mut Font) {
    init_font(x);
}
#[inline]
pub unsafe fn otfcc_font_free(mut x: *mut Font) {
    if x.is_null() {
        return;
    }
    otfcc_font_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}

