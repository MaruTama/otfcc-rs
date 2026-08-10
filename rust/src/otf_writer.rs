#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod stat;

use libc::{free};





use crate::support::alloc::{__caryll_allocate_clean};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};

use crate::font::caryll_font::{FontSubtype, Font, IFontSerializer};
use crate::font::caryll_sfnt_builder::{SfntBuilder};

use crate::table::cff::{CffAndGlyf};
use crate::table::_tsi::TsiBuildTarget;

use crate::table::glyf::GlyfAndLocaBuffers;

use crate::font::caryll_sfnt_builder::{otfcc_sfnt_builder_push_table, otfcc_sfnt_builder_serialize, otfcc_delete_sfnt_builder, otfcc_new_sfnt_builder};
use crate::otf_writer::stat::{otfcc_stat_font, otfcc_unstat_font};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b};
use crate::table::base::{otfcc_build_base};
use crate::table::cff::{otfcc_build_cff};
use crate::table::colr::{otfcc_build_colr};
use crate::table::cpal::{otfcc_build_cpal};
use crate::table::gdef::{otfcc_build_gdef};
use crate::table::ltsh::{otfcc_build_ltsh};
use crate::table::os_2::{otfcc_build_os_2};
use crate::table::svg::{otfcc_build_svg};
use crate::table::tsi5::{otfcc_build_tsi5};
use crate::table::vorg::{otfcc_build_vorg};
use crate::table::_tsi::{otfcc_build_tsi};
use crate::table::cmap::{otfcc_build_cmap};
use crate::table::cvt::{otfcc_build_cvt};
use crate::table::fpgm_prep::{otfcc_build_fpgm_prep};
use crate::table::gasp::{otfcc_build_gasp};
use crate::table::glyf::build::{otfcc_build_glyf};
use crate::table::head::{otfcc_build_head};
use crate::table::hhea::{otfcc_build_hhea};
use crate::table::hmtx::{otfcc_build_hmtx};
use crate::table::maxp::{otfcc_build_maxp};
use crate::table::meta::build::{otfcc_build_meta};
use crate::table::name::{otfcc_build_name};
use crate::table::otl::build::{otfcc_build_otl};
use crate::table::post::{otfcc_build_post};
use crate::table::vdmx::funcs::{otfcc_build_vdmx};
use crate::table::vhea::{otfcc_build_vhea};
use crate::table::vmtx::{otfcc_build_vmtx};





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
    let options = options as *const Options;
    otfcc_stat_font(font, options);
    let mut builder: *mut SfntBuilder = otfcc_new_sfnt_builder(
        (if (*font).subtype == FontSubtype::Cff {
            1330926671i32
        } else {
            0x10000 as ::core::ffi::c_int
        }) as u32,
        options,
    );
    if (*font).subtype == FontSubtype::Ttf {
        let mut pair: GlyfAndLocaBuffers =
            otfcc_build_glyf((*font).glyf, (*font).head, options);
        otfcc_sfnt_builder_push_table(builder, 1735162214i32 as u32, pair.glyf);
        otfcc_sfnt_builder_push_table(builder, 1819239265i32 as u32, pair.loca);
    } else {
        let mut r: CffAndGlyf = CffAndGlyf {
            meta: (*font).cff,
            glyphs: (*font).glyf,
        };
        otfcc_sfnt_builder_push_table(
            builder,
            1128678944i32 as u32,
            otfcc_build_cff(r, options),
        );
    }
    otfcc_sfnt_builder_push_table(
        builder,
        1751474532i32 as u32,
        otfcc_build_head((*font).head, options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1751672161i32 as u32,
        otfcc_build_hhea((*font).hhea, options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1330851634i32 as u32,
        otfcc_build_os_2((*font).os_2, options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1835104368i32 as u32,
        otfcc_build_maxp((*font).maxp, options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1851878757i32 as u32,
        otfcc_build_name((*font).name, options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1835365473i32 as u32,
        otfcc_build_meta((*font).meta.as_deref(), options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1886352244i32 as u32,
        otfcc_build_post((*font).post, (*font).glyph_order, options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1668112752i32 as u32,
        otfcc_build_cmap((*font).cmap, options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1734439792i32 as u32,
        otfcc_build_gasp((*font).gasp.as_deref(), options),
    );
    if (*font).subtype == FontSubtype::Ttf {
        otfcc_sfnt_builder_push_table(
            builder,
            1718642541i32 as u32,
            otfcc_build_fpgm_prep((*font).fpgm.as_deref(), options),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            1886545264i32 as u32,
            otfcc_build_fpgm_prep((*font).prep.as_deref(), options),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            1668707360i32 as u32,
            otfcc_build_cvt((*font).cvt_.as_deref(), options),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            1280594760i32 as u32,
            otfcc_build_ltsh((*font).ltsh.as_deref(), options),
        );
        otfcc_sfnt_builder_push_table(
            builder,
            1447316824i32 as u32,
            otfcc_build_vdmx((*font).vdmx.as_deref(), options),
        );
    }
    if !(*font).hhea.is_null() && !(*font).maxp.is_null() && (*font).hmtx.is_some() {
        let mut hmtx_counta: u16 = (*(*font).hhea).number_of_metrics;
        let mut hmtx_countk: u16 = ((*(*font).maxp).num_glyphs as ::core::ffi::c_int
            - (*(*font).hhea).number_of_metrics as ::core::ffi::c_int)
            as u16;
        otfcc_sfnt_builder_push_table(
            builder,
            1752003704i32 as u32,
            otfcc_build_hmtx(
                (*font).hmtx.as_deref(),
                hmtx_counta as GlyphId,
                hmtx_countk as GlyphId,
                options,
            ),
        );
    }
    otfcc_sfnt_builder_push_table(
        builder,
        1986553185i32 as u32,
        otfcc_build_vhea((*font).vhea, options),
    );
    if !(*font).vhea.is_null() && !(*font).maxp.is_null() && (*font).vmtx.is_some() {
        let mut vmtx_counta: u16 = (*(*font).vhea).num_of_long_ver_metrics;
        let mut vmtx_countk: u16 = ((*(*font).maxp).num_glyphs as ::core::ffi::c_int
            - (*(*font).vhea).num_of_long_ver_metrics as ::core::ffi::c_int)
            as u16;
        otfcc_sfnt_builder_push_table(
            builder,
            1986884728i32 as u32,
            otfcc_build_vmtx(
                (*font).vmtx.as_deref(),
                vmtx_counta as GlyphId,
                vmtx_countk as GlyphId,
                options,
            ),
        );
    }
    otfcc_sfnt_builder_push_table(
        builder,
        1448038983i32 as u32,
        otfcc_build_vorg((*font).vorg.as_deref(), options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1196643650i32 as u32,
        otfcc_build_otl(
            (*font).gsub,
            options,
            b"GSUB\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1196445523i32 as u32,
        otfcc_build_otl(
            (*font).gpos,
            options,
            b"GPOS\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1195656518i32 as u32,
        otfcc_build_gdef((*font).gdef, options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1111577413i32 as u32,
        otfcc_build_base((*font).base, options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1129333068i32 as u32,
        otfcc_build_cpal((*font).cpal.as_deref(), options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1129270354i32 as u32,
        otfcc_build_colr((*font).colr, options),
    );
    otfcc_sfnt_builder_push_table(
        builder,
        1398163232i32 as u32,
        otfcc_build_svg((*font).svg, options),
    );
    let mut target: TsiBuildTarget = otfcc_build_tsi((*font).tsi_01, options);
    otfcc_sfnt_builder_push_table(builder, 1414744368i32 as u32, target.index_part);
    otfcc_sfnt_builder_push_table(builder, 1414744369i32 as u32, target.text_part);
    let mut target_0: TsiBuildTarget = otfcc_build_tsi((*font).tsi_23, options);
    otfcc_sfnt_builder_push_table(builder, 1414744370i32 as u32, target_0.index_part);
    otfcc_sfnt_builder_push_table(builder, 1414744371i32 as u32, target_0.text_part);
    if !(*font).glyf.is_null() {
        otfcc_sfnt_builder_push_table(
            builder,
            1414744373i32 as u32,
            otfcc_build_tsi5((*font).tsi5, options, (*(*font).glyf).len() as GlyphId),
        );
    }
    if (*options).dummy_dsig {
        let mut dsig: *mut Buffer = bufnew();
        bufwrite32b(dsig, 0x1 as u32);
        bufwrite16b(dsig, 0 as u16);
        bufwrite16b(dsig, 0 as u16);
        otfcc_sfnt_builder_push_table(builder, 1146308935i32 as u32, dsig);
    }
    let mut otf: *mut Buffer = otfcc_sfnt_builder_serialize(builder);
    otfcc_delete_sfnt_builder(builder);
    otfcc_unstat_font(font, options);
    return otf as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn serialize_to_otf(
    mut font: *mut Font,
    mut options: *const Options,
) -> *mut ::core::ffi::c_void {
    <OtfSerializer as FontSerializer>::serialize(
        font as *mut ::core::ffi::c_void,
        options as *const ::core::ffi::c_void,
    )
}
unsafe extern "C" fn free_font_writer(mut self_0: *mut IFontSerializer) {
    free(self_0 as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_new_otf_writer() -> *mut IFontSerializer {
    let mut writer: *mut IFontSerializer = ::core::ptr::null_mut::<IFontSerializer>();
    writer = __caryll_allocate_clean(
        ::core::mem::size_of::<IFontSerializer>() as usize,
        100 as ::core::ffi::c_ulong,
    ) as *mut IFontSerializer;
    (*writer).serialize = Some(
        serialize_to_otf
            as unsafe extern "C" fn(
                *mut Font,
                *const Options,
            ) -> *mut ::core::ffi::c_void,
    )
        as Option<
            unsafe extern "C" fn(*mut Font, *const Options) -> *mut ::core::ffi::c_void,
        >;
    (*writer).free = Some(free_font_writer as unsafe extern "C" fn(*mut IFontSerializer) -> ())
        as Option<unsafe extern "C" fn(*mut IFontSerializer) -> ()>;
    return writer;
}
