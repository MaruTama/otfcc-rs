//! What each table of a [`Font`] does at every stage of the pipeline, in
//! one place.
//!
//! A font's tables used to be wired into each pipeline by hand: reading a
//! binary font, reading otfcc's JSON and so on were each a function listing
//! every table with its own call. Each table now implements [`FontTable`]
//! once, and each pipeline walks its own order list.
//!
//! The order lists are not shared, because each pipeline's order shows in
//! its output and the orders differ: the order tables are read in is the
//! order their warnings are logged in. Every method takes the whole `Font`,
//! because reading or writing one table often depends on others (`hmtx`
//! needs `hhea` and `maxp`, `glyf` needs `head` and `maxp`), and a method
//! that has nothing to do for a font -- a TrueType-only table in a CFF font,
//! say -- simply returns.
use crate::font::caryll_font::{Font, FontSubtype};
use crate::font::caryll_sfnt::Packet;
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
use crate::support::primitives::{GlyphId, ShapeId, count_u16};
use crate::table::_tsi::{TsiTable, parse_tsi, read_tsi};
use crate::table::base::{parse_base, read_base};
use crate::table::cff::{CffAndGlyfOwned, parse_cff, read_cff_and_glyf_tables};
use crate::table::cmap::{parse_cmap, read_cmap};
use crate::table::colr::{parse_colr, read_colr};
use crate::table::cpal::{parse_cpal, read_cpal};
use crate::table::cvt::{parse_cvt, read_cvt};
use crate::table::fpgm_prep::{FpgmPrepTable, parse_fpgm_prep, read_fpgm_prep};
use crate::table::fvar::read_fvar;
use crate::table::gasp::{parse_gasp, read_gasp};
use crate::table::gdef::{parse_gdef, read_gdef};
use crate::table::glyf::read::read_glyf;
use crate::table::glyf::{GlyfIOContext, parse_glyf};
use crate::table::head::{parse_head, read_head};
use crate::table::hhea::{parse_hhea, read_hhea};
use crate::table::hmtx::read_hmtx;
use crate::table::ltsh::read_ltsh;
use crate::table::maxp::{parse_maxp, read_maxp};
use crate::table::meta::parse::parse_meta;
use crate::table::meta::read::read_meta;
use crate::table::name::{parse_name, read_name};
use crate::table::os_2::{parse_os_2, read_os_2};
use crate::table::otl::OtlTable;
use crate::table::otl::parse::parse_otl;
use crate::table::otl::read::read_otl;
use crate::table::post::{parse_post, read_post};
use crate::table::svg::{parse_svg, read_svg};
use crate::table::tsi5::{parse_tsi5, read_tsi5};
use crate::table::vdmx::funcs::{parse_vdmx, read_vdmx};
use crate::table::vhea::{parse_vhea, read_vhea};
use crate::table::vmtx::read_vmtx;
use crate::table::vorg::read_vorg;
use crate::tag::{TAG_CVT, TAG_FPGM, TAG_GPOS, TAG_GSUB, TAG_PREP, TAG_TSI0, TAG_TSI1, TAG_TSI2, TAG_TSI3};

/// One table of a font, as each pipeline sees it. Every method does
/// nothing by default: not every table takes part in every pipeline (`hmtx`
/// is never in JSON, `fvar` is never written back), and those that do may
/// only apply to some fonts.
pub trait FontTable: Sync {
    /// Reads this table from a binary font.
    fn read(&self, _font: &mut Font, _packet: &Packet, _options: &Options) {}
    /// Reads this table from otfcc's JSON.
    fn parse(&self, _font: &mut Font, _root: &mut ParsedValue, _options: &Options) {}
}

fn is_ttf(font: &Font) -> bool {
    font.subtype == FontSubtype::Ttf
}

struct Fvar;
impl FontTable for Fvar {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.fvar = read_fvar(packet);
    }
}

struct Head;
impl FontTable for Head {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.head = read_head(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.head = parse_head(root);
    }
}

struct Hhea;
impl FontTable for Hhea {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.hhea = read_hhea(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.hhea = parse_hhea(root);
    }
}

struct Maxp;
impl FontTable for Maxp {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.maxp = read_maxp(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.maxp = parse_maxp(root);
    }
}

struct Os2;
impl FontTable for Os2 {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.os_2 = read_os_2(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.os_2 = parse_os_2(root);
    }
}

/// Only read from TrueType fonts: a CFF font's horizontal metrics come from
/// its CFF table. Never in JSON; the metrics live on the glyphs there.
struct Hmtx;
impl FontTable for Hmtx {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        if is_ttf(font) {
            font.hmtx = read_hmtx(packet, font.hhea.as_deref(), font.maxp.as_deref());
        }
    }
}

struct Post;
impl FontTable for Post {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.post = read_post(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, options: &Options) {
        font.post = parse_post(root, options);
    }
}

struct Vhea;
impl FontTable for Vhea {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.vhea = read_vhea(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.vhea = parse_vhea(root);
    }
}

/// Never in JSON; the metrics live on the glyphs there.
struct Vmtx;
impl FontTable for Vmtx {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        if font.vhea.is_some() {
            font.vmtx = read_vmtx(packet, font.vhea.as_deref(), font.maxp.as_deref());
        }
    }
}

/// Only read from CFF fonts with vertical metrics. Never in JSON; the
/// vertical origins live on the glyphs there.
struct Vorg;
impl FontTable for Vorg {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        if !is_ttf(font) && font.vhea.is_some() {
            font.vorg = read_vorg(packet);
        }
    }
}

/// Reading a CFF font's CFF table also gives it its glyphs.
struct Cff;
impl FontTable for Cff {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        if is_ttf(font) {
            return;
        }
        let cffpr: CffAndGlyfOwned = read_cff_and_glyf_tables(packet, font.head.as_deref());
        font.cff = cffpr.meta;
        font.glyf = cffpr.glyphs;
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, options: &Options) {
        font.cff = parse_cff(root, options);
    }
}

/// The glyphs. A CFF font gets its glyphs when its CFF table is read; this
/// reads a TrueType font's, from `glyf` and `loca`.
struct Glyf;
impl FontTable for Glyf {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        if !is_ttf(font) {
            return;
        }
        // `loca_is_long`/`num_glyphs` come from `head`/`maxp`. A malformed
        // font missing (or failing to parse) either table used to panic
        // here instead of the "skip this table, keep going" every other
        // reader does; a fuzz-found input with a `glyf`/`loca` pair but no
        // `maxp` hit exactly this. `glyf` genuinely cannot be read without
        // both, so it is left `None` rather than guessing at either value.
        let (Some(head), Some(maxp)) = (font.head.as_deref(), font.maxp.as_deref()) else {
            return;
        };
        let mut ctx: GlyfIOContext = GlyfIOContext {
            loca_is_long: head.index_to_loc_format != 0,
            num_glyphs: maxp.num_glyphs as GlyphId,
            n_phantom_points: 4 as ShapeId,
            fvar: font.fvar.as_deref_mut(),
            has_vertical_metrics: false,
            export_fd_select: false,
        };
        font.glyf = read_glyf(packet, &mut ctx);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, options: &Options) {
        font.glyf = parse_glyf(root, font.glyph_order.as_deref(), options);
    }
}

struct Cmap;
impl FontTable for Cmap {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.cmap = read_cmap(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.cmap = parse_cmap(root);
    }
}

struct Name;
impl FontTable for Name {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.name = read_name(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.name = parse_name(root);
    }
}

struct Meta;
impl FontTable for Meta {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.meta = read_meta(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.meta = parse_meta(root);
    }
}

/// `fpgm` or `prep`: both are a bare run of TrueType instructions, handled
/// the same way.
enum Program {
    Fpgm,
    Prep,
}
impl Program {
    fn slot<'a>(&self, font: &'a mut Font) -> &'a mut Option<Box<FpgmPrepTable>> {
        match self {
            Program::Fpgm => &mut font.fpgm,
            Program::Prep => &mut font.prep,
        }
    }
    fn tag(&self) -> u32 {
        match self {
            Program::Fpgm => TAG_FPGM,
            Program::Prep => TAG_PREP,
        }
    }
    fn key(&self) -> &'static [u8] {
        match self {
            Program::Fpgm => b"fpgm",
            Program::Prep => b"prep",
        }
    }
}
/// Hinting tables (this, `cvt ` and `gasp`) are only read from TrueType
/// fonts, and are left out of JSON when hints are ignored.
impl FontTable for Program {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        if is_ttf(font) {
            *self.slot(font) = read_fpgm_prep(packet, self.tag());
        }
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, options: &Options) {
        if !options.ignore_hints {
            *self.slot(font) = parse_fpgm_prep(root, self.key());
        }
    }
}

struct Cvt;
impl FontTable for Cvt {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        if is_ttf(font) {
            font.cvt_ = read_cvt(packet, TAG_CVT);
        }
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, options: &Options) {
        if !options.ignore_hints {
            font.cvt_ = parse_cvt(root, b"cvt_");
        }
    }
}

struct Gasp;
impl FontTable for Gasp {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        if is_ttf(font) {
            font.gasp = read_gasp(packet);
        }
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, options: &Options) {
        if !options.ignore_hints {
            font.gasp = parse_gasp(root);
        }
    }
}

/// Only read from TrueType fonts.
struct Vdmx;
impl FontTable for Vdmx {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        if is_ttf(font) {
            font.vdmx = read_vdmx(packet);
        }
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.vdmx = parse_vdmx(root);
    }
}

/// Only read from TrueType fonts. Never in JSON: it is recomputed from the
/// glyphs.
struct Ltsh;
impl FontTable for Ltsh {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        if is_ttf(font) {
            font.ltsh = read_ltsh(packet);
        }
    }
}

/// `GSUB` or `GPOS`. Like `GDEF`, only read when the font has glyphs: the
/// lookups refer to glyphs by index.
enum Layout {
    Gsub,
    Gpos,
}
impl Layout {
    fn slot<'a>(&self, font: &'a mut Font) -> &'a mut Option<Box<OtlTable>> {
        match self {
            Layout::Gsub => &mut font.gsub,
            Layout::Gpos => &mut font.gpos,
        }
    }
    fn tag(&self) -> u32 {
        match self {
            Layout::Gsub => TAG_GSUB,
            Layout::Gpos => TAG_GPOS,
        }
    }
    fn key(&self) -> &'static [u8] {
        match self {
            Layout::Gsub => b"GSUB",
            Layout::Gpos => b"GPOS",
        }
    }
}
impl FontTable for Layout {
    fn read(&self, font: &mut Font, packet: &Packet, options: &Options) {
        if let Some(glyf) = font.glyf.as_ref() {
            let num_glyphs = count_u16(glyf.len());
            *self.slot(font) = read_otl(packet, options, self.tag(), num_glyphs);
        }
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, options: &Options) {
        if font.glyf.is_some() {
            *self.slot(font) = parse_otl(root, options, self.key());
        }
    }
}

struct Gdef;
impl FontTable for Gdef {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        if font.glyf.is_some() {
            font.gdef = read_gdef(packet);
        }
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        if font.glyf.is_some() {
            font.gdef = parse_gdef(root);
        }
    }
}

struct Base;
impl FontTable for Base {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.base = read_base(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.base = parse_base(root);
    }
}

struct Cpal;
impl FontTable for Cpal {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.cpal = read_cpal(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.cpal = parse_cpal(root);
    }
}

struct Colr;
impl FontTable for Colr {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.colr = read_colr(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.colr = parse_colr(root);
    }
}

struct Svg;
impl FontTable for Svg {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.svg = read_svg(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.svg = parse_svg(root);
    }
}

/// VTT's source tables, each an index table and a text table: `TSI0` and
/// `TSI1` hold glyph programs, `TSI2` and `TSI3` hold the other programs.
enum VttSource {
    Tsi01,
    Tsi23,
}
impl VttSource {
    fn slot<'a>(&self, font: &'a mut Font) -> &'a mut Option<TsiTable> {
        match self {
            VttSource::Tsi01 => &mut font.tsi_01,
            VttSource::Tsi23 => &mut font.tsi_23,
        }
    }
    fn tags(&self) -> (u32, u32) {
        match self {
            VttSource::Tsi01 => (TAG_TSI0, TAG_TSI1),
            VttSource::Tsi23 => (TAG_TSI2, TAG_TSI3),
        }
    }
    fn key(&self) -> &'static [u8] {
        match self {
            VttSource::Tsi01 => b"TSI_01",
            VttSource::Tsi23 => b"TSI_23",
        }
    }
}
impl FontTable for VttSource {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        let (index, text) = self.tags();
        *self.slot(font) = read_tsi(packet, index, text);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        *self.slot(font) = parse_tsi(root, self.key());
    }
}

struct Tsi5;
impl FontTable for Tsi5 {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.tsi5 = read_tsi5(packet);
    }
    fn parse(&self, font: &mut Font, root: &mut ParsedValue, _options: &Options) {
        font.tsi5 = parse_tsi5(root);
    }
}

/// The order a binary font's tables are read in. `hmtx` comes before `cff`
/// and the hinting tables between `vorg` and `glyf` because a TrueType font
/// reads `hmtx, vhea, vmtx, fpgm, ..., glyf` and a CFF font reads `CFF ,
/// vhea, vmtx, VORG`; each of those tables skips the fonts it does not
/// apply to, so this one list gives both orders.
pub static READ_ORDER: [&dyn FontTable; 31] = [
    &Fvar,
    &Head,
    &Maxp,
    &Name,
    &Meta,
    &Os2,
    &Post,
    &Hhea,
    &Cmap,
    &Hmtx,
    &Cff,
    &Vhea,
    &Vmtx,
    &Vorg,
    &Program::Fpgm,
    &Program::Prep,
    &Cvt,
    &Gasp,
    &Vdmx,
    &Ltsh,
    &Glyf,
    &Layout::Gsub,
    &Layout::Gpos,
    &Gdef,
    &Base,
    &Cpal,
    &Colr,
    &Svg,
    &VttSource::Tsi01,
    &VttSource::Tsi23,
    &Tsi5,
];

/// The order a font's tables are read from otfcc's JSON in, once its glyph
/// order is known.
pub static PARSE_ORDER: [&dyn FontTable; 26] = [
    &Glyf,
    &Cff,
    &Head,
    &Hhea,
    &Os2,
    &Maxp,
    &Post,
    &Name,
    &Meta,
    &Cmap,
    &Program::Fpgm,
    &Program::Prep,
    &Cvt,
    &Gasp,
    &Vdmx,
    &Vhea,
    &Layout::Gsub,
    &Layout::Gpos,
    &Gdef,
    &Base,
    &Cpal,
    &Colr,
    &Svg,
    &VttSource::Tsi01,
    &VttSource::Tsi23,
    &Tsi5,
];
