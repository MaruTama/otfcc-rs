#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};





use crate::support::alloc::{__caryll_allocate_clean};
use crate::otf_writer::FontSerializer;


use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphid_t, shapeid_t};

use crate::vendor::json::{json_value};
use crate::font::caryll_font::{otfcc_Font, otfcc_IFontSerializer};
use crate::support::{NULL};



use crate::table::glyf::GlyfIOContext;


use crate::table::BASE::{otfcc_dumpBASE};
use crate::table::CFF::{otfcc_dumpCFF};
use crate::table::COLR::{otfcc_dumpCOLR};
use crate::table::CPAL::{otfcc_dumpCPAL};
use crate::table::GDEF::{otfcc_dumpGDEF};
use crate::table::OS_2::{otfcc_dumpOS_2};
use crate::table::SVG::{otfcc_dumpSVG};
use crate::table::TSI5::{otfcc_dumpTSI5};
use crate::table::_TSI::{otfcc_dumpTSI};
use crate::table::cmap::{otfcc_dumpCmap};
use crate::table::cvt::{otfcc_dumpCvt};
use crate::table::fpgm_prep::{table_dumpTableFpgmPrep};
use crate::table::fvar::{otfcc_dumpFvar};
use crate::table::gasp::{otfcc_dumpGasp};
use crate::table::glyf::{otfcc_dumpGlyf};
use crate::table::head::{otfcc_dumpHead};
use crate::table::hhea::{otfcc_dumpHhea};
use crate::table::maxp::{otfcc_dumpMaxp};
use crate::table::meta::dump::{otfcc_dumpMeta};
use crate::table::name::{otfcc_dumpName};
use crate::table::otl::dump::{otfcc_dumpOtl};
use crate::table::post::{otfcc_dumpPost};
use crate::table::vdmx::funcs::{otfcc_dumpVDMX};
use crate::table::vhea::{otfcc_dumpVhea};
use crate::vendor::json_builder::{json_object_new};






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
