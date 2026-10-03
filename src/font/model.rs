
use crate::support::glyph_order::GlyphOrder;
use crate::table::tsi::TsiTable;
use crate::table::base::BaseTable;
use crate::table::cff::CffTable;
use crate::table::cmap::CmapTable;
use crate::table::colr::ColrTable;
use crate::table::cpal::CpalTable;
use crate::table::cvt::CvtTable;
use crate::table::fpgm_prep::FpgmPrepTable;
use crate::table::fvar::FvarTable;
use crate::table::gasp::GaspTable;
use crate::table::gdef::GdefTable;
use crate::table::glyf::GlyfTable;
use crate::table::head::HeadTable;
use crate::table::hhea::HheaTable;
use crate::table::hmtx::HmtxTable;
use crate::table::ltsh::LtshTable;
use crate::table::maxp::MaxpTable;
use crate::table::meta::MetaTable;
use crate::table::name::NameTable;
use crate::table::os_2::Os2Table;
use crate::table::otl::OtlTable;
use crate::table::post::PostTable;
use crate::table::svg::SvgTable;
use crate::table::tsi5::Tsi5Table;
use crate::table::vdmx::VdmxTable;
use crate::table::vhea::VheaTable;
use crate::table::vmtx::VmtxTable;
use crate::table::vorg::VorgTable;

// `Copy, Clone` dropped: `Font` gained `ltsh: Option<Box<LtshTable>>` (Stage
// 6-4 pilot), which is never `Copy`. Grepping confirms `Font` is accessed
// exclusively via `*mut Font`/`(*font).field` throughout the crate -- never
// returned, constructed as a value literal, or `.clone()`'d -- so dropping
// the derive is safe (same check as `CffTable`/`NameRecord`/`GlyphOrderEntry`).
#[derive(Debug)]
pub struct Font {
    pub subtype: FontSubtype,
    pub fvar: Option<Box<FvarTable>>,
    pub head: Option<Box<HeadTable>>,
    pub hhea: Option<Box<HheaTable>>,
    pub maxp: Option<Box<MaxpTable>>,
    pub os_2: Option<Box<Os2Table>>,
    pub hmtx: Option<Box<HmtxTable>>,
    pub post: Option<Box<PostTable>>,
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
/// An empty TrueType `Font` -- every table absent, no glyph order yet. Was
/// `otfcc_font_create() -> *mut Font` (a bare `Box::into_raw`) paired with
/// `otfcc_font_free` (a bare `drop(Box::from_raw(..))`), a shell every
/// caller had to remember to pair by hand; an owned `Box<Font>` (built from
/// this) now frees itself.
impl Default for Font {
    fn default() -> Self {
        Font {
            subtype: FontSubtype::Ttf,
            fvar: None,
            head: None,
            hhea: None,
            maxp: None,
            os_2: None,
            hmtx: None,
            post: None,
            vhea: None,
            vmtx: None,
            vorg: None,
            cff: None,
            glyf: None,
            cmap: None,
            name: None,
            meta: None,
            fpgm: None,
            prep: None,
            cvt_: None,
            gasp: None,
            vdmx: None,
            ltsh: None,
            gsub: None,
            gpos: None,
            gdef: None,
            base: None,
            cpal: None,
            colr: None,
            svg: None,
            tsi_01: None,
            tsi_23: None,
            tsi5: None,
            glyph_order: None,
        }
    }
}
