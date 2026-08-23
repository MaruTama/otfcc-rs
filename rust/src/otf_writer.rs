#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod stat;

use crate::support::buffer::Buffer;
use crate::support::glyph_order::GlyphOrder;
use crate::support::options::Options;
use crate::support::primitives::GlyphId;

use crate::font::caryll_font::{Font, FontSubtype};
use crate::font::caryll_sfnt_builder::SfntBuilder;

use crate::table::_tsi::TsiBuildTarget;
use crate::table::cff::{CffAndGlyf, CffTable};

use crate::table::glyf::{GlyfAndLocaBuffers, GlyfTable};

use crate::font::caryll_sfnt_builder::{
    otfcc_delete_sfnt_builder, otfcc_new_sfnt_builder, otfcc_sfnt_builder_push_table,
    otfcc_sfnt_builder_serialize,
};
use crate::otf_writer::stat::{otfcc_stat_font, otfcc_unstat_font};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b};
use crate::table::_tsi::otfcc_build_tsi;
use crate::table::base::otfcc_build_base;
use crate::table::cff::otfcc_build_cff;
use crate::table::cmap::otfcc_build_cmap;
use crate::table::colr::otfcc_build_colr;
use crate::table::cpal::otfcc_build_cpal;
use crate::table::cvt::otfcc_build_cvt;
use crate::table::fpgm_prep::otfcc_build_fpgm_prep;
use crate::table::gasp::otfcc_build_gasp;
use crate::table::gdef::otfcc_build_gdef;
use crate::table::glyf::build::otfcc_build_glyf;
use crate::table::head::{HeadTable, otfcc_build_head};
use crate::table::hhea::otfcc_build_hhea;
use crate::table::hmtx::otfcc_build_hmtx;
use crate::table::ltsh::otfcc_build_ltsh;
use crate::table::maxp::otfcc_build_maxp;
use crate::table::meta::build::otfcc_build_meta;
use crate::table::name::otfcc_build_name;
use crate::table::os_2::otfcc_build_os_2;
use crate::table::otl::build::otfcc_build_otl;
use crate::table::post::otfcc_build_post;
use crate::table::svg::otfcc_build_svg;
use crate::table::tsi5::otfcc_build_tsi5;
use crate::table::vdmx::funcs::otfcc_build_vdmx;
use crate::table::vhea::otfcc_build_vhea;
use crate::table::vmtx::otfcc_build_vmtx;
use crate::table::vorg::otfcc_build_vorg;

// Font/Options are duplicated per-file by c2rust; the trait
// boundary uses erased c_void pointers so this trait can be shared with
// json_writer.rs without deduping those pervasively-used types (same
// reasoning as FontBuilder in otf_reader.rs).
pub(crate) trait FontSerializer {
    unsafe fn serialize(
        font: *mut ::core::ffi::c_void,
        options: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}
struct OtfSerializer;
impl FontSerializer for OtfSerializer {
    unsafe fn serialize(
        font: *mut ::core::ffi::c_void,
        options: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void {
        let font = font as *mut Font;
        let options: &Options = &*(options as *const Options);
        otfcc_stat_font(font, options);
        let mut builder: *mut SfntBuilder = otfcc_new_sfnt_builder(
            (if (*font).subtype == FontSubtype::Cff {
                crate::tag::SFNT_VERSION_OTTO as ::core::ffi::c_int
            } else {
                crate::tag::SFNT_VERSION_TRUE_TYPE as ::core::ffi::c_int
            }) as u32,
            options,
        );
        if (*font).subtype == FontSubtype::Ttf {
            let mut pair: GlyfAndLocaBuffers = otfcc_build_glyf(
                (*font).glyf.as_ref(),
                (*font)
                    .head
                    .as_deref_mut()
                    .map_or(::core::ptr::null_mut(), |h| h as *mut HeadTable),
            );
            otfcc_sfnt_builder_push_table(builder, crate::tag::TAG_GLYF, pair.glyf);
            otfcc_sfnt_builder_push_table(builder, crate::tag::TAG_LOCA, pair.loca);
        } else {
            let mut r: CffAndGlyf = CffAndGlyf {
                meta: (*font)
                    .cff
                    .as_deref_mut()
                    .map_or(::core::ptr::null_mut(), |c| c as *mut CffTable),
                glyphs: (*font)
                    .glyf
                    .as_mut()
                    .map_or(::core::ptr::null_mut(), |g| g as *mut GlyfTable),
            };
            otfcc_sfnt_builder_push_table(
                builder,
                crate::tag::TAG_CFF,
                otfcc_build_cff(r, options),
            );
        }
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_HEAD,
            otfcc_build_head((*font).head.as_deref()),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_HHEA,
            otfcc_build_hhea((*font).hhea.as_deref()),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_OS_2,
            otfcc_build_os_2((*font).os_2.as_deref()),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_MAXP,
            otfcc_build_maxp((*font).maxp.as_deref()),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_NAME,
            otfcc_build_name((*font).name.as_ref()),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_META,
            otfcc_build_meta((*font).meta.as_deref()),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_POST,
            otfcc_build_post(
                (*font).post.as_deref(),
                (*font)
                    .glyph_order
                    .as_deref_mut()
                    .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder),
            ),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_CMAP,
            otfcc_build_cmap((*font).cmap.as_deref(), options),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_GASP,
            otfcc_build_gasp((*font).gasp.as_deref()),
        );
        if (*font).subtype == FontSubtype::Ttf {
            otfcc_sfnt_builder_push_table(
                builder,
                crate::tag::TAG_FPGM,
                otfcc_build_fpgm_prep((*font).fpgm.as_deref()),
            );
            otfcc_sfnt_builder_push_table(
                builder,
                crate::tag::TAG_PREP,
                otfcc_build_fpgm_prep((*font).prep.as_deref()),
            );
            otfcc_sfnt_builder_push_table(
                builder,
                crate::tag::TAG_CVT,
                otfcc_build_cvt((*font).cvt_.as_deref()),
            );
            otfcc_sfnt_builder_push_table(
                builder,
                crate::tag::TAG_LTSH,
                otfcc_build_ltsh((*font).ltsh.as_deref()),
            );
            otfcc_sfnt_builder_push_table(
                builder,
                crate::tag::TAG_VDMX,
                otfcc_build_vdmx((*font).vdmx.as_deref()),
            );
        }
        if (*font).hhea.is_some() && (*font).maxp.is_some() && (*font).hmtx.is_some() {
            let mut hmtx_counta: u16 = (*font).hhea.as_deref().unwrap().number_of_metrics;
            let mut hmtx_countk: u16 = ((*font).maxp.as_deref().unwrap().num_glyphs
                as ::core::ffi::c_int
                - (*font).hhea.as_deref().unwrap().number_of_metrics as ::core::ffi::c_int)
                as u16;
            otfcc_sfnt_builder_push_table(
                builder,
                crate::tag::TAG_HMTX,
                otfcc_build_hmtx(
                    (*font).hmtx.as_deref(),
                    hmtx_counta as GlyphId,
                    hmtx_countk as GlyphId,
                ),
            );
        }
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_VHEA,
            otfcc_build_vhea((*font).vhea.as_deref()),
        );
        if (*font).vhea.is_some() && (*font).maxp.is_some() && (*font).vmtx.is_some() {
            let mut vmtx_counta: u16 = (*font).vhea.as_deref().unwrap().num_of_long_ver_metrics;
            let mut vmtx_countk: u16 = ((*font).maxp.as_deref().unwrap().num_glyphs
                as ::core::ffi::c_int
                - (*font).vhea.as_deref().unwrap().num_of_long_ver_metrics as ::core::ffi::c_int)
                as u16;
            otfcc_sfnt_builder_push_table(
                builder,
                crate::tag::TAG_VMTX,
                otfcc_build_vmtx(
                    (*font).vmtx.as_deref(),
                    vmtx_counta as GlyphId,
                    vmtx_countk as GlyphId,
                ),
            );
        }
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_VORG,
            otfcc_build_vorg((*font).vorg.as_deref()),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_GSUB,
            otfcc_build_otl(
                (*font).gsub.as_deref(),
                options,
                b"GSUB\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_GPOS,
            otfcc_build_otl(
                (*font).gpos.as_deref(),
                options,
                b"GPOS\0" as *const u8 as *const ::core::ffi::c_char,
            ),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_GDEF,
            otfcc_build_gdef((*font).gdef.as_deref()),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_BASE,
            otfcc_build_base((*font).base.as_deref()),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_CPAL,
            otfcc_build_cpal((*font).cpal.as_deref()),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_COLR,
            otfcc_build_colr((*font).colr.as_ref()),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            crate::tag::TAG_SVG,
            otfcc_build_svg((*font).svg.as_ref()),
        );
        let mut target: TsiBuildTarget = otfcc_build_tsi((*font).tsi_01.as_ref());
        otfcc_sfnt_builder_push_table(builder, crate::tag::TAG_TSI0, target.index_part);
        otfcc_sfnt_builder_push_table(builder, crate::tag::TAG_TSI1, target.text_part);
        let mut target_0: TsiBuildTarget = otfcc_build_tsi((*font).tsi_23.as_ref());
        otfcc_sfnt_builder_push_table(builder, crate::tag::TAG_TSI2, target_0.index_part);
        otfcc_sfnt_builder_push_table(builder, crate::tag::TAG_TSI3, target_0.text_part);
        if let Some(glyf) = (*font).glyf.as_ref() {
            otfcc_sfnt_builder_push_table(
                builder,
                crate::tag::TAG_TSI5,
                otfcc_build_tsi5((*font).tsi5.as_deref(), glyf.len() as GlyphId),
            );
        }
        if options.dummy_dsig {
            let mut dsig: *mut Buffer = bufnew();
            bufwrite32b(dsig, 0x1 as u32);
            bufwrite16b(dsig, 0 as u16);
            bufwrite16b(dsig, 0 as u16);
            otfcc_sfnt_builder_push_table(builder, crate::tag::TAG_DSIG, dsig);
        }
        let mut otf: *mut Buffer = otfcc_sfnt_builder_serialize(builder);
        otfcc_delete_sfnt_builder(builder);
        otfcc_unstat_font(font);
        return otf as *mut ::core::ffi::c_void;
    }
}
pub unsafe fn serialize_to_otf(
    mut font: *mut Font,
    mut options: &Options,
) -> *mut ::core::ffi::c_void {
    <OtfSerializer as FontSerializer>::serialize(
        font as *mut ::core::ffi::c_void,
        options as *const Options as *const ::core::ffi::c_void,
    )
}
