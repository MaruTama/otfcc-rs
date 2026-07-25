#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};
unsafe extern "C" {
    fn json_object_new(length: usize) -> *mut json_value;
    fn otfcc_dumpFvar(
        table: *const table_fvar,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpHead(
        table: *const table_head,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpGlyf(
        table: *const table_glyf,
        root: *mut json_value,
        options: *const otfcc_Options,
        ctx: *const GlyfIOContext,
    );
    fn otfcc_dumpCFF(table: *const table_CFF, root: *mut json_value, options: *const otfcc_Options);
    fn otfcc_dumpMaxp(
        table: *const table_maxp,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpHhea(
        table: *const table_hhea,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpVhea(
        table: *const table_vhea,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpOS_2(
        table: *const table_OS_2,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpPost(
        table: *const table_post,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpName(
        table: *const table_name,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpMeta(
        table: *const table_meta,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpCmap(
        cmap: *const table_cmap,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpCvt(
        table: *const table_cvt,
        root: *mut json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    );
    fn table_dumpTableFpgmPrep(
        table: *const table_fpgm_prep,
        root: *mut json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    );
    fn otfcc_dumpGasp(
        table: *const table_gasp,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpVDMX(
        table: *const table_VDMX,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpGDEF(
        gdef: *const table_GDEF,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpBASE(
        base: *const table_BASE,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpOtl(
        table: *const table_OTL,
        root: *mut json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    );
    fn otfcc_dumpCPAL(
        table: *const table_CPAL,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpCOLR(
        table: *const table_COLR,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
    fn otfcc_dumpSVG(svg: *const table_SVG, root: *mut json_value, options: *const otfcc_Options);
    fn otfcc_dumpTSI(
        table: *const table_TSI,
        root: *mut json_value,
        options: *const otfcc_Options,
        tag: *const ::core::ffi::c_char,
    );
    fn otfcc_dumpTSI5(
        table: *const table_TSI5,
        root: *mut json_value,
        options: *const otfcc_Options,
    );
}





use crate::support::alloc::{__caryll_allocate_clean};
use crate::otf_writer::FontSerializer;


use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphid_t, shapeid_t};

use crate::vendor::json::{json_value};
use crate::font::caryll_font::{otfcc_Font, otfcc_IFontSerializer};
use crate::support::{NULL};

use crate::table::BASE::{table_BASE};
use crate::table::CFF::{table_CFF};
use crate::table::COLR::{table_COLR};
use crate::table::CPAL::{table_CPAL};
use crate::table::GDEF::{table_GDEF};

use crate::table::OS_2::{table_OS_2};
use crate::table::SVG::{table_SVG};
use crate::table::TSI5::{table_TSI5};

use crate::table::_TSI::{table_TSI};
use crate::table::cmap::{table_cmap};
use crate::table::cvt::{table_cvt};
use crate::table::fpgm_prep::{table_fpgm_prep};
use crate::table::fvar::{table_fvar};
use crate::table::gasp::{table_gasp};
use crate::table::glyf::{GlyfIOContext, table_glyf};

use crate::table::head::{table_head};
use crate::table::hhea::{table_hhea};

use crate::table::maxp::{table_maxp};
use crate::table::meta::types::{table_meta};
use crate::table::name::{table_name};
use crate::table::otl::{table_OTL};
use crate::table::post::{table_post};
use crate::table::vdmx::types::{table_VDMX};
use crate::table::vhea::{table_vhea};






struct JsonSerializer;
impl FontSerializer for JsonSerializer {
    unsafe fn serialize(
        font: *mut ::core::ffi::c_void,
        options: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void {
    let font = font as *mut otfcc_Font;
    let options = options as *const otfcc_Options;
    let mut root: *mut json_value = json_object_new(48 as usize);
    if root.is_null() {
        return NULL;
    }
    otfcc_dumpFvar((*font).fvar, root, options);
    otfcc_dumpHead((*font).head, root, options);
    otfcc_dumpHhea((*font).hhea, root, options);
    otfcc_dumpMaxp((*font).maxp, root, options);
    otfcc_dumpVhea((*font).vhea, root, options);
    otfcc_dumpPost((*font).post, root, options);
    otfcc_dumpOS_2((*font).OS_2, root, options);
    otfcc_dumpName((*font).name, root, options);
    otfcc_dumpMeta((*font).meta, root, options);
    otfcc_dumpCmap((*font).cmap, root, options);
    otfcc_dumpCFF((*font).CFF_, root, options);
    let mut ctx: GlyfIOContext = GlyfIOContext {
        locaIsLong: (*(*font).head).indexToLocFormat != 0,
        numGlyphs: (*(*font).maxp).numGlyphs as glyphid_t,
        nPhantomPoints: 4 as shapeid_t,
        fvar: (*font).fvar,
        hasVerticalMetrics: !(*font).vhea.is_null(),
        exportFDSelect: !(*font).CFF_.is_null() && (*(*font).CFF_).isCID as ::core::ffi::c_int != 0,
    };
    otfcc_dumpGlyf((*font).glyf, root, options, &raw mut ctx);
    if !(*options).ignore_hints {
        table_dumpTableFpgmPrep(
            (*font).fpgm,
            root,
            options,
            b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
        );
        table_dumpTableFpgmPrep(
            (*font).prep,
            root,
            options,
            b"prep\0" as *const u8 as *const ::core::ffi::c_char,
        );
        otfcc_dumpCvt(
            (*font).cvt_,
            root,
            options,
            b"cvt_\0" as *const u8 as *const ::core::ffi::c_char,
        );
        otfcc_dumpGasp((*font).gasp, root, options);
    }
    otfcc_dumpVDMX((*font).VDMX, root, options);
    otfcc_dumpOtl(
        (*font).GSUB,
        root,
        options,
        b"GSUB\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dumpOtl(
        (*font).GPOS,
        root,
        options,
        b"GPOS\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dumpGDEF((*font).GDEF, root, options);
    otfcc_dumpBASE((*font).BASE, root, options);
    otfcc_dumpCPAL((*font).CPAL, root, options);
    otfcc_dumpCOLR((*font).COLR, root, options);
    otfcc_dumpSVG((*font).SVG_, root, options);
    otfcc_dumpTSI(
        (*font).TSI_01,
        root,
        options,
        b"TSI_01\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dumpTSI(
        (*font).TSI_23,
        root,
        options,
        b"TSI_23\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dumpTSI5((*font).TSI5, root, options);
    return root as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn serializeToJson(
    mut font: *mut otfcc_Font,
    mut options: *const otfcc_Options,
) -> *mut ::core::ffi::c_void {
    <JsonSerializer as FontSerializer>::serialize(
        font as *mut ::core::ffi::c_void,
        options as *const ::core::ffi::c_void,
    )
}
unsafe extern "C" fn freeJsonWriter(mut self_0: *mut otfcc_IFontSerializer) {
    free(self_0 as *mut ::core::ffi::c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_newJsonWriter() -> *mut otfcc_IFontSerializer {
    let mut writer: *mut otfcc_IFontSerializer = ::core::ptr::null_mut::<otfcc_IFontSerializer>();
    writer = __caryll_allocate_clean(
        ::core::mem::size_of::<otfcc_IFontSerializer>() as usize,
        52 as ::core::ffi::c_ulong,
    ) as *mut otfcc_IFontSerializer;
    (*writer).serialize = Some(
        serializeToJson
            as unsafe extern "C" fn(
                *mut otfcc_Font,
                *const otfcc_Options,
            ) -> *mut ::core::ffi::c_void,
    )
        as Option<
            unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Options) -> *mut ::core::ffi::c_void,
        >;
    (*writer).free = Some(freeJsonWriter as unsafe extern "C" fn(*mut otfcc_IFontSerializer) -> ())
        as Option<unsafe extern "C" fn(*mut otfcc_IFontSerializer) -> ()>;
    return writer;
}
