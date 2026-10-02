use crate::support::options::Options;
use crate::support::primitives::{GlyphId, ShapeId};

use crate::font::caryll_font::Font;
use crate::support::built_json::BuiltValue;

use crate::table::glyf::GlyfIOContext;

use crate::table::_tsi::dump_tsi;
use crate::table::base::dump_base;
use crate::table::cff::dump_cff;
use crate::table::cmap::dump_cmap;
use crate::table::colr::dump_colr;
use crate::table::cpal::dump_cpal;
use crate::table::cvt::dump_cvt;
use crate::table::fpgm_prep::table_dump_table_fpgm_prep;
use crate::table::fvar::dump_fvar;
use crate::table::gasp::dump_gasp;
use crate::table::gdef::dump_gdef;
use crate::table::glyf::dump_glyf;
use crate::table::head::dump_head;
use crate::table::hhea::dump_hhea;
use crate::table::maxp::dump_maxp;
use crate::table::meta::dump::dump_meta;
use crate::table::name::dump_name;
use crate::table::os_2::dump_os_2;
use crate::table::otl::dump::dump_otl;
use crate::table::post::dump_post;
use crate::table::svg::dump_svg;
use crate::table::tsi5::dump_tsi5;
use crate::table::vdmx::funcs::dump_vdmx;
use crate::table::vhea::dump_vhea;

/// Dumps a consolidated font into the JSON value tree otfccdump prints.
///
/// Was a `FontSerializer` impl on a zero-sized `JsonSerializer` marker
/// struct plus a casting wrapper; see `otf_reader::read_otf` for why that
/// trait is gone. Returning the `BuiltValue` itself rather than a
/// `BuiltValue::into_raw` pointer drops the last reason that bridge
/// existed on this path.
pub fn serialize_to_json(font: &mut Font, options: &Options) -> BuiltValue {
    let mut root = BuiltValue::new_object(48);
    dump_fvar(font.fvar.as_deref(), &mut root);
    dump_head(font.head.as_deref(), &mut root);
    dump_hhea(font.hhea.as_deref(), &mut root);
    dump_maxp(font.maxp.as_deref(), &mut root);
    dump_vhea(font.vhea.as_deref(), &mut root);
    dump_post(font.post.as_deref(), &mut root);
    dump_os_2(font.os_2.as_deref(), &mut root);
    dump_name(font.name.as_ref(), &mut root);
    dump_meta(font.meta.as_deref(), &mut root);
    dump_cmap(font.cmap.as_deref(), &mut root, options);
    dump_cff(font.cff.as_deref(), &mut root);
    // `GlyfIOContext` needs both `head` (for `index_to_loc_format`) and
    // `maxp` (for `num_glyphs`) -- a malformed/CFF-flavored font can
    // legitimately have neither, the same "head+maxp missing" case
    // `read_otf`'s own TTF branch already treats as "no glyf
    // data" (leaving `font.glyf` at `None`) rather than panicking.
    // `dump_glyf` itself already no-ops on a `None` table, so
    // building `ctx` (which unconditionally unwrapped both) was the
    // only thing that could panic here -- skip the whole block instead.
    if let (Some(head), Some(maxp)) = (font.head.as_deref(), font.maxp.as_deref()) {
        let ctx: GlyfIOContext = GlyfIOContext {
            loca_is_long: head.index_to_loc_format != 0,
            num_glyphs: maxp.num_glyphs as GlyphId,
            n_phantom_points: 4 as ShapeId,
            fvar: font.fvar.as_deref_mut(),
            has_vertical_metrics: font.vhea.is_some(),
            export_fd_select: font.cff.as_deref().is_some_and(|c| c.is_cid),
        };
        dump_glyf(font.glyf.as_ref(), &mut root, options, &ctx);
    }
    if !options.ignore_hints {
        table_dump_table_fpgm_prep(
            font.fpgm.as_deref(),
            &mut root,
            options,
            b"fpgm",
        );
        table_dump_table_fpgm_prep(
            font.prep.as_deref(),
            &mut root,
            options,
            b"prep",
        );
        dump_cvt(
            font.cvt_.as_deref(),
            &mut root,
            b"cvt_",
        );
        dump_gasp(font.gasp.as_deref(), &mut root);
    }
    dump_vdmx(font.vdmx.as_deref(), &mut root);
    dump_otl(
        font.gsub.as_deref(),
        &mut root,
        b"GSUB",
    );
    dump_otl(
        font.gpos.as_deref(),
        &mut root,
        b"GPOS",
    );
    dump_gdef(font.gdef.as_deref(), &mut root);
    dump_base(font.base.as_deref(), &mut root);
    dump_cpal(font.cpal.as_deref(), &mut root);
    dump_colr(font.colr.as_ref(), &mut root);
    dump_svg(font.svg.as_ref(), &mut root);
    dump_tsi(
        font.tsi_01.as_ref(),
        &mut root,
        b"TSI_01",
    );
    dump_tsi(
        font.tsi_23.as_ref(),
        &mut root,
        b"TSI_23",
    );
    dump_tsi5(font.tsi5.as_deref(), &mut root);
    return root;
}
