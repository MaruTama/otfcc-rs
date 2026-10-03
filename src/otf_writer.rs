pub mod stat;

use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::primitives::{GlyphId, count_u16};

use crate::font::caryll_font::{Font, FontSubtype};
use crate::font::caryll_sfnt_builder::SfntBuilder;

use crate::table::_tsi::TsiBuildTarget;
use crate::table::cff::CffAndGlyfRef;

use crate::table::glyf::GlyfAndLocaBuffers;

use crate::font::caryll_sfnt_builder::{
    sfnt_builder_push_table, sfnt_builder_serialize,
};
use crate::otf_writer::stat::{stat_font, unstat_font};
use crate::table::_tsi::build_tsi;
use crate::table::base::build_base;
use crate::table::cff::build_cff;
use crate::table::cmap::build_cmap;
use crate::table::colr::build_colr;
use crate::table::cpal::build_cpal;
use crate::table::cvt::build_cvt;
use crate::table::fpgm_prep::build_fpgm_prep;
use crate::table::gasp::build_gasp;
use crate::table::gdef::build_gdef;
use crate::table::glyf::build::build_glyf;
use crate::table::head::build_head;
use crate::table::hhea::build_hhea;
use crate::table::hmtx::build_hmtx;
use crate::table::ltsh::build_ltsh;
use crate::table::maxp::build_maxp;
use crate::table::meta::build::build_meta;
use crate::table::name::build_name;
use crate::table::os_2::build_os_2;
use crate::table::otl::build::build_otl;
use crate::table::post::build_post;
use crate::table::svg::build_svg;
use crate::table::tsi5::build_tsi5;
use crate::table::vdmx::funcs::build_vdmx;
use crate::table::vhea::build_vhea;
use crate::table::vmtx::build_vmtx;
use crate::table::vorg::build_vorg;

/// Serializes a consolidated font into sfnt (OTF/TTF) bytes.
///
/// Was a `FontSerializer` impl on a zero-sized `OtfSerializer` marker
/// struct plus a casting wrapper; see `otf_reader::read_otf` for why that
/// trait is gone. With the erased return type went the reason to hand back
/// a `Buffer::into_raw` pointer -- only `ffi/dll.rs`'s genuine `extern "C"`
/// boundary needs one, and it makes that conversion itself now.
pub fn serialize_to_otf(font: &mut Font, options: &Options) -> Buffer {
    stat_font(&mut *font, options);
    let mut builder = SfntBuilder::new(
        (if font.subtype == FontSubtype::Cff {
            crate::tag::SFNT_VERSION_OTTO as i32
        } else {
            crate::tag::SFNT_VERSION_TRUE_TYPE as i32
        }) as u32,
        options,
    );
    if font.subtype == FontSubtype::Ttf {
        let pair: GlyfAndLocaBuffers =
            build_glyf(font.glyf.as_ref(), font.head.as_deref_mut());
        sfnt_builder_push_table(&mut builder, crate::tag::TAG_GLYF, Some(pair.glyf));
        sfnt_builder_push_table(&mut builder, crate::tag::TAG_LOCA, Some(pair.loca));
    } else {
        // `CffAndGlyfRef` borrows straight into `font`'s own `cff`/`glyf`
        // fields -- no raw pointer, and (Stage M-10) no `unsafe` call
        // left here at all. `meta` is required: a CFF-subtype font is
        // assumed to have a CFF table, the same assumption the old
        // `.map_or(ptr::null_mut(), ...)` made implicitly (and left an
        // unchecked null deref inside `writecff_cid_keyed` if it ever
        // didn't hold) -- `.expect()` makes that assumption explicit
        // instead.
        let r = CffAndGlyfRef {
            meta: font
                .cff
                .as_deref_mut()
                .expect("a CFF-subtype font must have a CFF table to build"),
            glyphs: font.glyf.as_ref(),
        };
        let cff = build_cff(r, options);
        sfnt_builder_push_table(&mut builder, crate::tag::TAG_CFF, Some(cff));
    }
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_HEAD,
        build_head(font.head.as_deref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_HHEA,
        build_hhea(font.hhea.as_deref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_OS_2,
        build_os_2(font.os_2.as_deref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_MAXP,
        build_maxp(font.maxp.as_deref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_NAME,
        build_name(font.name.as_ref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_META,
        build_meta(font.meta.as_deref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_POST,
        build_post(font.post.as_deref(), font.glyph_order.as_deref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_CMAP,
        build_cmap(font.cmap.as_deref(), options),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_GASP,
        build_gasp(font.gasp.as_deref()),
    );
    if font.subtype == FontSubtype::Ttf {
        sfnt_builder_push_table(
            &mut builder,
            crate::tag::TAG_FPGM,
            build_fpgm_prep(font.fpgm.as_deref()),
        );
        sfnt_builder_push_table(
            &mut builder,
            crate::tag::TAG_PREP,
            build_fpgm_prep(font.prep.as_deref()),
        );
        sfnt_builder_push_table(
            &mut builder,
            crate::tag::TAG_CVT,
            build_cvt(font.cvt_.as_deref()),
        );
        sfnt_builder_push_table(
            &mut builder,
            crate::tag::TAG_LTSH,
            build_ltsh(font.ltsh.as_deref()),
        );
        sfnt_builder_push_table(
            &mut builder,
            crate::tag::TAG_VDMX,
            build_vdmx(font.vdmx.as_deref()),
        );
    }
    if font.hhea.is_some() && font.maxp.is_some() && font.hmtx.is_some() {
        let hmtx_counta: u16 = font.hhea.as_deref().unwrap().number_of_metrics;
        let hmtx_countk: u16 = (font.maxp.as_deref().unwrap().num_glyphs
            as i32
            - font.hhea.as_deref().unwrap().number_of_metrics as i32)
            as u16;
        sfnt_builder_push_table(
            &mut builder,
            crate::tag::TAG_HMTX,
            Some(build_hmtx(
                font.hmtx.as_deref(),
                hmtx_counta as GlyphId,
                hmtx_countk as GlyphId,
            )),
        );
    }
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_VHEA,
        build_vhea(font.vhea.as_deref()),
    );
    if font.vhea.is_some() && font.maxp.is_some() && font.vmtx.is_some() {
        let vmtx_counta: u16 = font.vhea.as_deref().unwrap().num_of_long_ver_metrics;
        let vmtx_countk: u16 = (font.maxp.as_deref().unwrap().num_glyphs
            as i32
            - font.vhea.as_deref().unwrap().num_of_long_ver_metrics as i32)
            as u16;
        sfnt_builder_push_table(
            &mut builder,
            crate::tag::TAG_VMTX,
            Some(build_vmtx(
                font.vmtx.as_deref(),
                vmtx_counta as GlyphId,
                vmtx_countk as GlyphId,
            )),
        );
    }
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_VORG,
        build_vorg(font.vorg.as_deref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_GSUB,
        build_otl(
            font.gsub.as_deref(),
            b"GSUB",
        ),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_GPOS,
        build_otl(
            font.gpos.as_deref(),
            b"GPOS",
        ),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_GDEF,
        build_gdef(font.gdef.as_deref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_BASE,
        build_base(font.base.as_deref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_CPAL,
        build_cpal(font.cpal.as_deref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_COLR,
        build_colr(font.colr.as_ref()),
    );
    sfnt_builder_push_table(
        &mut builder,
        crate::tag::TAG_SVG,
        build_svg(font.svg.as_ref()),
    );
    let target: TsiBuildTarget = build_tsi(font.tsi_01.as_ref());
    sfnt_builder_push_table(&mut builder, crate::tag::TAG_TSI0, target.index_part);
    sfnt_builder_push_table(&mut builder, crate::tag::TAG_TSI1, target.text_part);
    let target_0: TsiBuildTarget = build_tsi(font.tsi_23.as_ref());
    sfnt_builder_push_table(&mut builder, crate::tag::TAG_TSI2, target_0.index_part);
    sfnt_builder_push_table(&mut builder, crate::tag::TAG_TSI3, target_0.text_part);
    if let Some(glyf) = font.glyf.as_ref() {
        sfnt_builder_push_table(
            &mut builder,
            crate::tag::TAG_TSI5,
            build_tsi5(font.tsi5.as_deref(), count_u16(glyf.len())),
        );
    }
    if options.dummy_dsig {
        let mut dsig = Buffer::new();
        dsig.write_u32be(0x1_u32);
        dsig.write_u16be(0_u16);
        dsig.write_u16be(0_u16);
        sfnt_builder_push_table(&mut builder, crate::tag::TAG_DSIG, Some(dsig));
    }
    let otf: Buffer = sfnt_builder_serialize(&builder);
    unstat_font(&mut *font);
    return otf;
}
