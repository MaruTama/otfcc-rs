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
use crate::consolidate::otl::common::fontop_consolidate_class_def;
use crate::consolidate::otl::gdef::consolidate_gdef;
use crate::consolidate::{
    consolidate_cmap, consolidate_colr, consolidate_glyf, consolidate_otl_table, consolidate_tsi,
};
use crate::font::model::{Font, FontSubtype};
use crate::font::sfnt::Packet;
use crate::font::sfnt_builder::{SfntBuilder, sfnt_builder_push_table};
use crate::logger::ByteStr;
use crate::support::built_json::BuiltValue;
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
use crate::support::primitives::{GlyphId, ShapeId, count_u16};
use crate::table::tsi::{TsiBuildTarget, TsiTable, build_tsi, dump_tsi, parse_tsi, read_tsi};
use crate::table::base::{build_base, dump_base, parse_base, read_base};
use crate::table::cff::{
    CffAndGlyfOwned, CffAndGlyfRef, build_cff, dump_cff, parse_cff, read_cff_and_glyf_tables,
};
use crate::table::cmap::{build_cmap, dump_cmap, parse_cmap, read_cmap};
use crate::table::colr::{build_colr, dump_colr, parse_colr, read_colr};
use crate::table::cpal::{build_cpal, dump_cpal, parse_cpal, read_cpal};
use crate::table::cvt::{build_cvt, dump_cvt, parse_cvt, read_cvt};
use crate::table::fpgm_prep::{
    FpgmPrepTable, build_fpgm_prep, parse_fpgm_prep, read_fpgm_prep, table_dump_table_fpgm_prep,
};
use crate::table::fvar::{dump_fvar, read_fvar};
use crate::table::gasp::{build_gasp, dump_gasp, parse_gasp, read_gasp};
use crate::table::gdef::{build_gdef, dump_gdef, parse_gdef, read_gdef};
use crate::table::glyf::build::build_glyf;
use crate::table::glyf::read::read_glyf;
use crate::table::glyf::{GlyfAndLocaBuffers, GlyfIOContext, dump_glyf, parse_glyf};
use crate::table::head::{build_head, dump_head, parse_head, read_head};
use crate::table::hhea::{build_hhea, dump_hhea, parse_hhea, read_hhea};
use crate::table::hmtx::{build_hmtx, read_hmtx};
use crate::table::ltsh::{build_ltsh, read_ltsh};
use crate::table::maxp::{build_maxp, dump_maxp, parse_maxp, read_maxp};
use crate::table::meta::build_meta;
use crate::table::meta::dump_meta;
use crate::table::meta::parse_meta;
use crate::table::meta::read_meta;
use crate::table::name::{build_name, dump_name, parse_name, read_name};
use crate::table::os_2::{build_os_2, dump_os_2, parse_os_2, read_os_2};
use crate::table::otl::OtlTable;
use crate::table::otl::build::build_otl;
use crate::table::otl::dump::dump_otl;
use crate::table::otl::parse::parse_otl;
use crate::table::otl::read::read_otl;
use crate::table::post::{build_post, dump_post, parse_post, read_post};
use crate::table::svg::{build_svg, dump_svg, parse_svg, read_svg};
use crate::table::tsi5::{build_tsi5, dump_tsi5, parse_tsi5, read_tsi5};
use crate::table::vdmx::{build_vdmx, dump_vdmx, parse_vdmx, read_vdmx};
use crate::table::vhea::{build_vhea, dump_vhea, parse_vhea, read_vhea};
use crate::table::vmtx::{build_vmtx, read_vmtx};
use crate::table::vorg::{build_vorg, read_vorg};
use crate::tag::{
    TAG_BASE, TAG_CFF, TAG_CMAP, TAG_COLR, TAG_CPAL, TAG_CVT, TAG_FPGM, TAG_GASP, TAG_GDEF,
    TAG_GLYF, TAG_GPOS, TAG_GSUB, TAG_HEAD, TAG_HHEA, TAG_HMTX, TAG_LOCA, TAG_LTSH, TAG_MAXP,
    TAG_META, TAG_NAME, TAG_OS_2, TAG_POST, TAG_PREP, TAG_SVG, TAG_TSI0, TAG_TSI1, TAG_TSI2,
    TAG_TSI3, TAG_TSI5, TAG_VDMX, TAG_VHEA, TAG_VMTX, TAG_VORG,
};

/// One table of a font, as each pipeline sees it. Every method does
/// nothing by default: not every table takes part in every pipeline (`hmtx`
/// is never in JSON, `fvar` is never written back), and those that do may
/// only apply to some fonts.
pub trait FontTable: Sync {
    /// Reads this table from a binary font.
    fn read(&self, _font: &mut Font, _packet: &Packet, _options: &Options) {}
    /// Reads this table from otfcc's JSON.
    fn parse(&self, _font: &mut Font, _root: &mut ParsedValue, _options: &Options) {}
    /// Writes this table as otfcc's JSON.
    fn dump(&self, _font: &mut Font, _root: &mut BuiltValue, _options: &Options) {}
    /// Writes this table into a binary font.
    fn build(&self, _font: &mut Font, _builder: &mut SfntBuilder, _options: &Options) {}
    /// Resolves the glyph references in this table against the font's glyph
    /// order, and drops what does not resolve, before the font is written.
    fn consolidate(&self, _font: &mut Font, _options: &Options) {}
}

fn is_ttf(font: &Font) -> bool {
    font.subtype == FontSubtype::Ttf
}

struct Fvar;
impl FontTable for Fvar {
    fn read(&self, font: &mut Font, packet: &Packet, _options: &Options) {
        font.fvar = read_fvar(packet);
    }
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_fvar(font.fvar.as_deref(), root);
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_head(font.head.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_HEAD, build_head(font.head.as_deref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_hhea(font.hhea.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_HHEA, build_hhea(font.hhea.as_deref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_maxp(font.maxp.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_MAXP, build_maxp(font.maxp.as_deref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_os_2(font.os_2.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_OS_2, build_os_2(font.os_2.as_deref()));
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
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        let (Some(hhea), Some(maxp), Some(hmtx)) = (
            font.hhea.as_deref(),
            font.maxp.as_deref(),
            font.hmtx.as_deref(),
        ) else {
            return;
        };
        let count_a: u16 = hhea.number_of_metrics;
        let count_k: u16 = (maxp.num_glyphs as i32 - hhea.number_of_metrics as i32) as u16;
        sfnt_builder_push_table(
            builder,
            TAG_HMTX,
            Some(build_hmtx(
                Some(hmtx),
                count_a as GlyphId,
                count_k as GlyphId,
            )),
        );
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_post(font.post.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(
            builder,
            TAG_POST,
            build_post(font.post.as_deref(), font.glyph_order.as_deref()),
        );
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_vhea(font.vhea.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_VHEA, build_vhea(font.vhea.as_deref()));
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
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        let (Some(vhea), Some(maxp), Some(vmtx)) = (
            font.vhea.as_deref(),
            font.maxp.as_deref(),
            font.vmtx.as_deref(),
        ) else {
            return;
        };
        let count_a: u16 = vhea.num_of_long_ver_metrics;
        let count_k: u16 = (maxp.num_glyphs as i32 - vhea.num_of_long_ver_metrics as i32) as u16;
        sfnt_builder_push_table(
            builder,
            TAG_VMTX,
            Some(build_vmtx(
                Some(vmtx),
                count_a as GlyphId,
                count_k as GlyphId,
            )),
        );
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
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_VORG, build_vorg(font.vorg.as_deref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_cff(font.cff.as_deref(), root);
    }
    /// Builds a CFF font's CFF table, outlines included.
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, options: &Options) {
        if is_ttf(font) {
            return;
        }
        // A CFF-subtype font is assumed to have a CFF table, the same
        // assumption the original made implicitly (it left an unchecked null
        // deref inside `writecff_cid_keyed` if it ever didn't hold).
        let r = CffAndGlyfRef {
            meta: font
                .cff
                .as_deref_mut()
                .expect("a CFF-subtype font must have a CFF table to build"),
            glyphs: font.glyf.as_ref(),
        };
        sfnt_builder_push_table(builder, TAG_CFF, Some(build_cff(r, options)));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, options: &Options) {
        // `GlyfIOContext` needs both `head` (for `index_to_loc_format`) and
        // `maxp` (for `num_glyphs`); a malformed or CFF-flavored font can
        // legitimately have neither, the same case `read` leaves `glyf` at
        // `None` for, so there is nothing to dump then.
        let (Some(head), Some(maxp)) = (font.head.as_deref(), font.maxp.as_deref()) else {
            return;
        };
        let ctx: GlyfIOContext = GlyfIOContext {
            loca_is_long: head.index_to_loc_format != 0,
            num_glyphs: maxp.num_glyphs as GlyphId,
            n_phantom_points: 4 as ShapeId,
            fvar: font.fvar.as_deref_mut(),
            has_vertical_metrics: font.vhea.is_some(),
            export_fd_select: font.cff.as_deref().is_some_and(|c| c.is_cid),
        };
        dump_glyf(font.glyf.as_ref(), root, options, &ctx);
    }
    /// Builds a TrueType font's `glyf` and `loca`. This sets
    /// `head.indexToLocFormat`, so it has to come before `head` is built.
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        if !is_ttf(font) {
            return;
        }
        let pair: GlyfAndLocaBuffers = build_glyf(font.glyf.as_ref(), font.head.as_deref_mut());
        sfnt_builder_push_table(builder, TAG_GLYF, Some(pair.glyf));
        sfnt_builder_push_table(builder, TAG_LOCA, Some(pair.loca));
    }
    fn consolidate(&self, font: &mut Font, options: &Options) {
        let stage = crate::logger::stage("glyf");
        consolidate_glyf(font, options);
        stage.finish();
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, options: &Options) {
        dump_cmap(font.cmap.as_deref(), root, options);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, options: &Options) {
        sfnt_builder_push_table(builder, TAG_CMAP, build_cmap(font.cmap.as_deref(), options));
    }
    fn consolidate(&self, font: &mut Font, _options: &Options) {
        let stage = crate::logger::stage("cmap");
        consolidate_cmap(font);
        stage.finish();
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_name(font.name.as_ref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_NAME, build_name(font.name.as_ref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_meta(font.meta.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_META, build_meta(font.meta.as_deref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, options: &Options) {
        if !options.ignore_hints {
            table_dump_table_fpgm_prep(self.slot(font).as_deref(), root, options, self.key());
        }
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        if is_ttf(font) {
            sfnt_builder_push_table(
                builder,
                self.tag(),
                build_fpgm_prep(self.slot(font).as_deref()),
            );
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, options: &Options) {
        if !options.ignore_hints {
            dump_cvt(font.cvt_.as_deref(), root, b"cvt_");
        }
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        if is_ttf(font) {
            sfnt_builder_push_table(builder, TAG_CVT, build_cvt(font.cvt_.as_deref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, options: &Options) {
        if !options.ignore_hints {
            dump_gasp(font.gasp.as_deref(), root);
        }
    }
    /// Unlike the other hinting tables, written for CFF fonts too.
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_GASP, build_gasp(font.gasp.as_deref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_vdmx(font.vdmx.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        if is_ttf(font) {
            sfnt_builder_push_table(builder, TAG_VDMX, build_vdmx(font.vdmx.as_deref()));
        }
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
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        if is_ttf(font) {
            sfnt_builder_push_table(builder, TAG_LTSH, build_ltsh(font.ltsh.as_deref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_otl(self.slot(font).as_deref(), root, self.key());
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(
            builder,
            self.tag(),
            build_otl(self.slot(font).as_deref(), self.key()),
        );
    }
    fn consolidate(&self, font: &mut Font, options: &Options) {
        if font.glyf.is_none() {
            return;
        }
        let glyph_order = font.glyph_order.as_deref();
        let table = match self {
            Layout::Gsub => font.gsub.as_deref_mut(),
            Layout::Gpos => font.gpos.as_deref_mut(),
        };
        let stage = crate::logger::stage(ByteStr(self.key()));
        consolidate_otl_table(glyph_order, table, options);
        stage.finish();
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_gdef(font.gdef.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_GDEF, build_gdef(font.gdef.as_deref()));
    }
    fn consolidate(&self, font: &mut Font, _options: &Options) {
        if font.glyf.is_none() {
            return;
        }
        let stage = crate::logger::stage("GDEF");
        consolidate_gdef(font.glyph_order.as_deref(), font.gdef.as_deref_mut());
        stage.finish();
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_base(font.base.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_BASE, build_base(font.base.as_deref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_cpal(font.cpal.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_CPAL, build_cpal(font.cpal.as_deref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_colr(font.colr.as_ref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_COLR, build_colr(font.colr.as_ref()));
    }
    fn consolidate(&self, font: &mut Font, _options: &Options) {
        let stage = crate::logger::stage("COLR");
        consolidate_colr(font);
        stage.finish();
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_svg(font.svg.as_ref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        sfnt_builder_push_table(builder, TAG_SVG, build_svg(font.svg.as_ref()));
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_tsi(self.slot(font).as_ref(), root, self.key());
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        let (index, text) = self.tags();
        let target: TsiBuildTarget = build_tsi(self.slot(font).as_ref());
        sfnt_builder_push_table(builder, index, target.index_part);
        sfnt_builder_push_table(builder, text, target.text_part);
    }
    fn consolidate(&self, font: &mut Font, _options: &Options) {
        let stage = crate::logger::stage(ByteStr(self.key()));
        let tsi = match self {
            VttSource::Tsi01 => &mut font.tsi_01,
            VttSource::Tsi23 => &mut font.tsi_23,
        };
        if let (Some(glyf), Some(glyph_order)) = (font.glyf.as_ref(), font.glyph_order.as_deref()) {
            consolidate_tsi(glyf, glyph_order, tsi);
        }
        stage.finish();
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
    fn dump(&self, font: &mut Font, root: &mut BuiltValue, _options: &Options) {
        dump_tsi5(font.tsi5.as_deref(), root);
    }
    fn build(&self, font: &mut Font, builder: &mut SfntBuilder, _options: &Options) {
        if let Some(glyf) = font.glyf.as_ref() {
            sfnt_builder_push_table(
                builder,
                TAG_TSI5,
                build_tsi5(font.tsi5.as_deref(), count_u16(glyf.len())),
            );
        }
    }
    fn consolidate(&self, font: &mut Font, _options: &Options) {
        let stage = crate::logger::stage("TSI5");
        fontop_consolidate_class_def(font.glyph_order.as_deref(), font.tsi5.as_deref_mut());
        stage.finish();
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

/// The order a font's tables are written as JSON in, which is the order of
/// the JSON's keys.
pub static DUMP_ORDER: [&dyn FontTable; 27] = [
    &Fvar,
    &Head,
    &Hhea,
    &Maxp,
    &Vhea,
    &Post,
    &Os2,
    &Name,
    &Meta,
    &Cmap,
    &Cff,
    &Glyf,
    &Program::Fpgm,
    &Program::Prep,
    &Cvt,
    &Gasp,
    &Vdmx,
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

/// The order a font's tables are written into a binary font in. The tables
/// end up sorted by tag whatever the order, but building `glyf` changes
/// `head`, and building the layout tables logs, so the order still matters.
pub static BUILD_ORDER: [&dyn FontTable; 30] = [
    &Glyf,
    &Cff,
    &Head,
    &Hhea,
    &Os2,
    &Maxp,
    &Name,
    &Meta,
    &Post,
    &Cmap,
    &Gasp,
    &Program::Fpgm,
    &Program::Prep,
    &Cvt,
    &Ltsh,
    &Vdmx,
    &Hmtx,
    &Vhea,
    &Vmtx,
    &Vorg,
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

/// The order a font's tables are consolidated in, once it has a glyph
/// order.
pub static CONSOLIDATE_ORDER: [&dyn FontTable; 9] = [
    &Glyf,
    &Cmap,
    &Layout::Gsub,
    &Layout::Gpos,
    &Gdef,
    &Colr,
    &VttSource::Tsi01,
    &VttSource::Tsi23,
    &Tsi5,
];
