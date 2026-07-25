pub mod stat;

use libc::{free};
extern "C" {
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn otfcc_buildHead(
        head: *const table_head,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildGlyf(
        table: *const table_glyf,
        head: *mut table_head,
        options: *const otfcc_Options,
    ) -> table_GlyfAndLocaBuffers;
    fn otfcc_buildCFF(
        cffAndGlyf: table_CFFAndGlyf,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildMaxp(
        maxp: *const table_maxp,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildHhea(
        hhea: *const table_hhea,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildVhea(
        vhea: *const table_vhea,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildVmtx(
        table: *const table_vmtx,
        count_a: glyphid_t,
        count_k: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildOS_2(
        os_2: *const table_OS_2,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildPost(
        post: *const table_post,
        glyphorder: *mut otfcc_GlyphOrder,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildName(
        name: *const table_name,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildMeta(
        meta: *const table_meta,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildCmap(
        cmap: *const table_cmap,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildCvt(table: *const table_cvt, options: *const otfcc_Options)
        -> *mut caryll_Buffer;
    fn otfcc_buildFpgmPrep(
        table: *const table_fpgm_prep,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildGasp(
        table: *const table_gasp,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildVDMX(
        vdmx: *const table_VDMX,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildLTSH(
        ltsh: *const table_LTSH,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildVORG(
        table: *const table_VORG,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildGDEF(
        gdef: *const table_GDEF,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildBASE(
        base: *const table_BASE,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildOtl(
        table: *const table_OTL,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildCPAL(
        cpal: *const table_CPAL,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildCOLR(
        colr: *const table_COLR,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
    fn otfcc_buildSVG(svg: *const table_SVG, options: *const otfcc_Options) -> *mut caryll_Buffer;
    fn otfcc_buildTSI(TSI: *const table_TSI, options: *const otfcc_Options) -> tsi_BuildTarget;
    fn otfcc_buildTSI5(
        TSI: *const table_TSI5,
        options: *const otfcc_Options,
        numGlyphs: glyphid_t,
    ) -> *mut caryll_Buffer;
    fn otfcc_newSFNTBuilder(
        header: u32,
        options: *const otfcc_Options,
    ) -> *mut otfcc_SFNTBuilder;
    fn otfcc_SFNTBuilder_pushTable(
        builder: *mut otfcc_SFNTBuilder,
        tag: u32,
        buffer: *mut caryll_Buffer,
    );
    fn otfcc_deleteSFNTBuilder(builder: *mut otfcc_SFNTBuilder);
    fn otfcc_SFNTBuilder_serialize(builder: *mut otfcc_SFNTBuilder) -> *mut caryll_Buffer;
    fn otfcc_statFont(font: *mut otfcc_Font, options: *const otfcc_Options);
    fn otfcc_unstatFont(font: *mut otfcc_Font, options: *const otfcc_Options);
    fn otfcc_buildHmtx(
        table: *const table_hmtx,
        count_a: glyphid_t,
        count_k: glyphid_t,
        options: *const otfcc_Options,
    ) -> *mut caryll_Buffer;
}





use crate::support::alloc::{__caryll_allocate_clean};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphid_t};

use crate::font::caryll_font::{FONTTYPE_CFF, FONTTYPE_TTF, otfcc_Font, otfcc_IFontSerializer};
use crate::font::caryll_sfnt_builder::{otfcc_SFNTBuilder};

use crate::support::glyph_order::{otfcc_GlyphOrder};
use crate::table::BASE::{table_BASE};
use crate::table::CFF::{table_CFFAndGlyf};
use crate::table::COLR::{table_COLR};
use crate::table::CPAL::{table_CPAL};
use crate::table::GDEF::{table_GDEF};
use crate::table::LTSH::{table_LTSH};
use crate::table::OS_2::{table_OS_2};
use crate::table::SVG::{table_SVG};
use crate::table::TSI5::{table_TSI5};
use crate::table::VORG::{table_VORG};
use crate::table::_TSI::{table_TSI, tsi_BuildTarget};
use crate::table::cmap::{table_cmap};
use crate::table::cvt::{table_cvt};
use crate::table::fpgm_prep::{table_fpgm_prep};

use crate::table::gasp::{table_gasp};
use crate::table::glyf::{table_GlyfAndLocaBuffers, table_glyf};

use crate::table::head::{table_head};
use crate::table::hhea::{table_hhea};
use crate::table::hmtx::{table_hmtx};
use crate::table::maxp::{table_maxp};
use crate::table::meta::types::{table_meta};
use crate::table::name::{table_name};
use crate::table::otl::{table_OTL};
use crate::table::post::{table_post};
use crate::table::vdmx::types::{table_VDMX};
use crate::table::vhea::{table_vhea};
use crate::table::vmtx::{table_vmtx};





// otfcc_Font/otfcc_Options are duplicated per-file by c2rust; the trait
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
    let font = font as *mut otfcc_Font;
    let options = options as *const otfcc_Options;
    otfcc_statFont(font, options);
    let mut builder: *mut otfcc_SFNTBuilder = otfcc_newSFNTBuilder(
        (if (*font).subtype == FONTTYPE_CFF {
            1330926671i32
        } else {
            0x10000 as ::core::ffi::c_int
        }) as u32,
        options,
    );
    if (*font).subtype == FONTTYPE_TTF {
        let mut pair: table_GlyfAndLocaBuffers =
            otfcc_buildGlyf((*font).glyf, (*font).head, options);
        otfcc_SFNTBuilder_pushTable(builder, 1735162214i32 as u32, pair.glyf);
        otfcc_SFNTBuilder_pushTable(builder, 1819239265i32 as u32, pair.loca);
    } else {
        let mut r: table_CFFAndGlyf = table_CFFAndGlyf {
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
                hmtx_counta as glyphid_t,
                hmtx_countk as glyphid_t,
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
                vmtx_counta as glyphid_t,
                vmtx_countk as glyphid_t,
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
    let mut target: tsi_BuildTarget = otfcc_buildTSI((*font).TSI_01, options);
    otfcc_SFNTBuilder_pushTable(builder, 1414744368i32 as u32, target.indexPart);
    otfcc_SFNTBuilder_pushTable(builder, 1414744369i32 as u32, target.textPart);
    let mut target_0: tsi_BuildTarget = otfcc_buildTSI((*font).TSI_23, options);
    otfcc_SFNTBuilder_pushTable(builder, 1414744370i32 as u32, target_0.indexPart);
    otfcc_SFNTBuilder_pushTable(builder, 1414744371i32 as u32, target_0.textPart);
    if !(*font).glyf.is_null() {
        otfcc_SFNTBuilder_pushTable(
            builder,
            1414744373i32 as u32,
            otfcc_buildTSI5((*font).TSI5, options, (*(*font).glyf).length as glyphid_t),
        );
    }
    if (*options).dummy_DSIG {
        let mut dsig: *mut caryll_Buffer = bufnew();
        bufwrite32b(dsig, 0x1 as u32);
        bufwrite16b(dsig, 0 as u16);
        bufwrite16b(dsig, 0 as u16);
        otfcc_SFNTBuilder_pushTable(builder, 1146308935i32 as u32, dsig);
    }
    let mut otf: *mut caryll_Buffer = otfcc_SFNTBuilder_serialize(builder);
    otfcc_deleteSFNTBuilder(builder);
    otfcc_unstatFont(font, options);
    return otf as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn serializeToOTF(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) -> *mut ::core::ffi::c_void {
    <OtfSerializer as FontSerializer>::serialize(
        font as *mut ::core::ffi::c_void,
        options as *const ::core::ffi::c_void,
    )
}
unsafe extern "C" fn freeFontWriter(mut self_0: *mut otfcc_IFontSerializer) {
    free(self_0 as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_newOTFWriter() -> *mut otfcc_IFontSerializer {
    let mut writer: *mut otfcc_IFontSerializer = ::core::ptr::null_mut::<otfcc_IFontSerializer>();
    writer = __caryll_allocate_clean(
        ::core::mem::size_of::<otfcc_IFontSerializer>() as usize,
        100 as ::core::ffi::c_ulong,
    ) as *mut otfcc_IFontSerializer;
    (*writer).serialize = Some(
        serializeToOTF
            as unsafe extern "C" fn(
                *mut otfcc_Font,
                *const otfcc_Options,
            ) -> *mut ::core::ffi::c_void,
    )
        as Option<
            unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Options) -> *mut ::core::ffi::c_void,
        >;
    (*writer).free = Some(freeFontWriter as unsafe extern "C" fn(*mut otfcc_IFontSerializer) -> ())
        as Option<unsafe extern "C" fn(*mut otfcc_IFontSerializer) -> ()>;
    return writer;
}
