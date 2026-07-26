#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod stat;

use libc::{free};





use crate::support::alloc::{__caryll_allocate_clean};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};

use crate::font::caryll_font::{FONTTYPE_CFF, FONTTYPE_TTF, Font, IFontSerializer};
use crate::font::caryll_sfnt_builder::{SfntBuilder};

use crate::table::CFF::{CffAndGlyf};
use crate::table::_TSI::TsiBuildTarget;

use crate::table::glyf::GlyfAndLocaBuffers;

use crate::font::caryll_sfnt_builder::{otfcc_SFNTBuilder_pushTable, otfcc_SFNTBuilder_serialize, otfcc_deleteSFNTBuilder, otfcc_newSFNTBuilder};
use crate::otf_writer::stat::{otfcc_statFont, otfcc_unstatFont};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b};
use crate::table::BASE::{otfcc_buildBASE};
use crate::table::CFF::{otfcc_buildCFF};
use crate::table::COLR::{otfcc_buildCOLR};
use crate::table::CPAL::{otfcc_buildCPAL};
use crate::table::GDEF::{otfcc_buildGDEF};
use crate::table::LTSH::{otfcc_buildLTSH};
use crate::table::OS_2::{otfcc_buildOS_2};
use crate::table::SVG::{otfcc_buildSVG};
use crate::table::TSI5::{otfcc_buildTSI5};
use crate::table::VORG::{otfcc_buildVORG};
use crate::table::_TSI::{otfcc_buildTSI};
use crate::table::cmap::{otfcc_buildCmap};
use crate::table::cvt::{otfcc_buildCvt};
use crate::table::fpgm_prep::{otfcc_buildFpgmPrep};
use crate::table::gasp::{otfcc_buildGasp};
use crate::table::glyf::build::{otfcc_buildGlyf};
use crate::table::head::{otfcc_buildHead};
use crate::table::hhea::{otfcc_buildHhea};
use crate::table::hmtx::{otfcc_buildHmtx};
use crate::table::maxp::{otfcc_buildMaxp};
use crate::table::meta::build::{otfcc_buildMeta};
use crate::table::name::{otfcc_buildName};
use crate::table::otl::build::{otfcc_buildOtl};
use crate::table::post::{otfcc_buildPost};
use crate::table::vdmx::funcs::{otfcc_buildVDMX};
use crate::table::vhea::{otfcc_buildVhea};
use crate::table::vmtx::{otfcc_buildVmtx};





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
    otfcc_statFont(font, options);
    let mut builder: *mut SfntBuilder = otfcc_newSFNTBuilder(
        (if (*font).subtype == FONTTYPE_CFF {
            1330926671i32
        } else {
            0x10000 as ::core::ffi::c_int
        }) as u32,
        options,
    );
    if (*font).subtype == FONTTYPE_TTF {
        let mut pair: GlyfAndLocaBuffers =
            otfcc_buildGlyf((*font).glyf, (*font).head, options);
        otfcc_SFNTBuilder_pushTable(builder, 1735162214i32 as u32, pair.glyf);
        otfcc_SFNTBuilder_pushTable(builder, 1819239265i32 as u32, pair.loca);
    } else {
        let mut r: CffAndGlyf = CffAndGlyf {
            meta: (*font).CFF_,
            glyphs: (*font).glyf,
        };
        otfcc_SFNTBuilder_pushTable(
            builder,
            1128678944i32 as u32,
            otfcc_buildCFF(r, options),
        );
    }
    otfcc_SFNTBuilder_pushTable(
        builder,
        1751474532i32 as u32,
        otfcc_buildHead((*font).head, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1751672161i32 as u32,
        otfcc_buildHhea((*font).hhea, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1330851634i32 as u32,
        otfcc_buildOS_2((*font).OS_2, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1835104368i32 as u32,
        otfcc_buildMaxp((*font).maxp, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1851878757i32 as u32,
        otfcc_buildName((*font).name, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1835365473i32 as u32,
        otfcc_buildMeta((*font).meta, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1886352244i32 as u32,
        otfcc_buildPost((*font).post, (*font).glyph_order, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1668112752i32 as u32,
        otfcc_buildCmap((*font).cmap, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1734439792i32 as u32,
        otfcc_buildGasp((*font).gasp, options),
    );
    if (*font).subtype == FONTTYPE_TTF {
        otfcc_SFNTBuilder_pushTable(
            builder,
            1718642541i32 as u32,
            otfcc_buildFpgmPrep((*font).fpgm, options),
        );
        otfcc_SFNTBuilder_pushTable(
            builder,
            1886545264i32 as u32,
            otfcc_buildFpgmPrep((*font).prep, options),
        );
        otfcc_SFNTBuilder_pushTable(
            builder,
            1668707360i32 as u32,
            otfcc_buildCvt((*font).cvt_, options),
        );
        otfcc_SFNTBuilder_pushTable(
            builder,
            1280594760i32 as u32,
            otfcc_buildLTSH((*font).LTSH, options),
        );
        otfcc_SFNTBuilder_pushTable(
            builder,
            1447316824i32 as u32,
            otfcc_buildVDMX((*font).VDMX, options),
        );
    }
    if !(*font).hhea.is_null() && !(*font).maxp.is_null() && !(*font).hmtx.is_null() {
        let mut hmtx_counta: u16 = (*(*font).hhea).numberOfMetrics;
        let mut hmtx_countk: u16 = ((*(*font).maxp).numGlyphs as ::core::ffi::c_int
            - (*(*font).hhea).numberOfMetrics as ::core::ffi::c_int)
            as u16;
        otfcc_SFNTBuilder_pushTable(
            builder,
            1752003704i32 as u32,
            otfcc_buildHmtx(
                (*font).hmtx,
                hmtx_counta as GlyphId,
                hmtx_countk as GlyphId,
                options,
            ),
        );
    }
    otfcc_SFNTBuilder_pushTable(
        builder,
        1986553185i32 as u32,
        otfcc_buildVhea((*font).vhea, options),
    );
    if !(*font).vhea.is_null() && !(*font).maxp.is_null() && !(*font).vmtx.is_null() {
        let mut vmtx_counta: u16 = (*(*font).vhea).numOfLongVerMetrics;
        let mut vmtx_countk: u16 = ((*(*font).maxp).numGlyphs as ::core::ffi::c_int
            - (*(*font).vhea).numOfLongVerMetrics as ::core::ffi::c_int)
            as u16;
        otfcc_SFNTBuilder_pushTable(
            builder,
            1986884728i32 as u32,
            otfcc_buildVmtx(
                (*font).vmtx,
                vmtx_counta as GlyphId,
                vmtx_countk as GlyphId,
                options,
            ),
        );
    }
    otfcc_SFNTBuilder_pushTable(
        builder,
        1448038983i32 as u32,
        otfcc_buildVORG((*font).VORG, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1196643650i32 as u32,
        otfcc_buildOtl(
            (*font).GSUB,
            options,
            b"GSUB\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1196445523i32 as u32,
        otfcc_buildOtl(
            (*font).GPOS,
            options,
            b"GPOS\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1195656518i32 as u32,
        otfcc_buildGDEF((*font).GDEF, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1111577413i32 as u32,
        otfcc_buildBASE((*font).BASE, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1129333068i32 as u32,
        otfcc_buildCPAL((*font).CPAL, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1129270354i32 as u32,
        otfcc_buildCOLR((*font).COLR, options),
    );
    otfcc_SFNTBuilder_pushTable(
        builder,
        1398163232i32 as u32,
        otfcc_buildSVG((*font).SVG_, options),
    );
    let mut target: TsiBuildTarget = otfcc_buildTSI((*font).TSI_01, options);
    otfcc_SFNTBuilder_pushTable(builder, 1414744368i32 as u32, target.indexPart);
    otfcc_SFNTBuilder_pushTable(builder, 1414744369i32 as u32, target.textPart);
    let mut target_0: TsiBuildTarget = otfcc_buildTSI((*font).TSI_23, options);
    otfcc_SFNTBuilder_pushTable(builder, 1414744370i32 as u32, target_0.indexPart);
    otfcc_SFNTBuilder_pushTable(builder, 1414744371i32 as u32, target_0.textPart);
    if !(*font).glyf.is_null() {
        otfcc_SFNTBuilder_pushTable(
            builder,
            1414744373i32 as u32,
            otfcc_buildTSI5((*font).TSI5, options, (*(*font).glyf).length as GlyphId),
        );
    }
    if (*options).dummy_DSIG {
        let mut dsig: *mut Buffer = bufnew();
        bufwrite32b(dsig, 0x1 as u32);
        bufwrite16b(dsig, 0 as u16);
        bufwrite16b(dsig, 0 as u16);
        otfcc_SFNTBuilder_pushTable(builder, 1146308935i32 as u32, dsig);
    }
    let mut otf: *mut Buffer = otfcc_SFNTBuilder_serialize(builder);
    otfcc_deleteSFNTBuilder(builder);
    otfcc_unstatFont(font, options);
    return otf as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn serializeToOTF(
    mut font: *mut Font,
    mut options: *const Options,
) -> *mut ::core::ffi::c_void {
    <OtfSerializer as FontSerializer>::serialize(
        font as *mut ::core::ffi::c_void,
        options as *const ::core::ffi::c_void,
    )
}
unsafe extern "C" fn freeFontWriter(mut self_0: *mut IFontSerializer) {
    free(self_0 as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_newOTFWriter() -> *mut IFontSerializer {
    let mut writer: *mut IFontSerializer = ::core::ptr::null_mut::<IFontSerializer>();
    writer = __caryll_allocate_clean(
        ::core::mem::size_of::<IFontSerializer>() as usize,
        100 as ::core::ffi::c_ulong,
    ) as *mut IFontSerializer;
    (*writer).serialize = Some(
        serializeToOTF
            as unsafe extern "C" fn(
                *mut Font,
                *const Options,
            ) -> *mut ::core::ffi::c_void,
    )
        as Option<
            unsafe extern "C" fn(*mut Font, *const Options) -> *mut ::core::ffi::c_void,
        >;
    (*writer).free = Some(freeFontWriter as unsafe extern "C" fn(*mut IFontSerializer) -> ())
        as Option<unsafe extern "C" fn(*mut IFontSerializer) -> ()>;
    return writer;
}
