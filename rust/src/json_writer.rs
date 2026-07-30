#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};





use crate::support::alloc::{__caryll_allocate_clean};
use crate::otf_writer::FontSerializer;


use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, ShapeId};

use crate::vendor::json::{JsonValue};
use crate::font::caryll_font::{Font, IFontSerializer};
use crate::support::{NULL};



use crate::table::glyf::GlyfIOContext;


use crate::table::BASE::{otfcc_dump_base};
use crate::table::CFF::{otfcc_dump_cff};
use crate::table::COLR::{otfcc_dump_colr};
use crate::table::CPAL::{otfcc_dump_cpal};
use crate::table::GDEF::{otfcc_dump_gdef};
use crate::table::OS_2::{otfcc_dump_os_2};
use crate::table::SVG::{otfcc_dump_svg};
use crate::table::TSI5::{otfcc_dump_tsi5};
use crate::table::_TSI::{otfcc_dump_tsi};
use crate::table::cmap::{otfcc_dump_cmap};
use crate::table::cvt::{otfcc_dump_cvt};
use crate::table::fpgm_prep::{table_dump_table_fpgm_prep};
use crate::table::fvar::{otfcc_dump_fvar};
use crate::table::gasp::{otfcc_dump_gasp};
use crate::table::glyf::{otfcc_dump_glyf};
use crate::table::head::{otfcc_dump_head};
use crate::table::hhea::{otfcc_dump_hhea};
use crate::table::maxp::{otfcc_dump_maxp};
use crate::table::meta::dump::{otfcc_dump_meta};
use crate::table::name::{otfcc_dump_name};
use crate::table::otl::dump::{otfcc_dump_otl};
use crate::table::post::{otfcc_dump_post};
use crate::table::vdmx::funcs::{otfcc_dump_vdmx};
use crate::table::vhea::{otfcc_dump_vhea};
use crate::vendor::json_builder::{json_object_new};






struct JsonSerializer;
impl FontSerializer for JsonSerializer {
    unsafe fn serialize(
        font: *mut ::core::ffi::c_void,
        options: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void {
    let font = font as *mut Font;
    let options = options as *const Options;
    let mut root: *mut JsonValue = json_object_new(48 as usize);
    if root.is_null() {
        return NULL;
    }
    otfcc_dump_fvar((*font).fvar, root, options);
    otfcc_dump_head((*font).head, root, options);
    otfcc_dump_hhea((*font).hhea, root, options);
    otfcc_dump_maxp((*font).maxp, root, options);
    otfcc_dump_vhea((*font).vhea, root, options);
    otfcc_dump_post((*font).post, root, options);
    otfcc_dump_os_2((*font).OS_2, root, options);
    otfcc_dump_name((*font).name, root, options);
    otfcc_dump_meta((*font).meta, root, options);
    otfcc_dump_cmap((*font).cmap, root, options);
    otfcc_dump_cff((*font).CFF_, root, options);
    let mut ctx: GlyfIOContext = GlyfIOContext {
        locaIsLong: (*(*font).head).indexToLocFormat != 0,
        numGlyphs: (*(*font).maxp).numGlyphs as GlyphId,
        nPhantomPoints: 4 as ShapeId,
        fvar: (*font).fvar,
        hasVerticalMetrics: !(*font).vhea.is_null(),
        exportFDSelect: !(*font).CFF_.is_null() && (*(*font).CFF_).isCID as ::core::ffi::c_int != 0,
    };
    otfcc_dump_glyf((*font).glyf, root, options, &raw mut ctx);
    if !(*options).ignore_hints {
        table_dump_table_fpgm_prep(
            (*font).fpgm,
            root,
            options,
            b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
        );
        table_dump_table_fpgm_prep(
            (*font).prep,
            root,
            options,
            b"prep\0" as *const u8 as *const ::core::ffi::c_char,
        );
        otfcc_dump_cvt(
            (*font).cvt_,
            root,
            options,
            b"cvt_\0" as *const u8 as *const ::core::ffi::c_char,
        );
        otfcc_dump_gasp((*font).gasp, root, options);
    }
    otfcc_dump_vdmx((*font).VDMX, root, options);
    otfcc_dump_otl(
        (*font).GSUB,
        root,
        options,
        b"GSUB\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dump_otl(
        (*font).GPOS,
        root,
        options,
        b"GPOS\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dump_gdef((*font).GDEF, root, options);
    otfcc_dump_base((*font).BASE, root, options);
    otfcc_dump_cpal((*font).CPAL, root, options);
    otfcc_dump_colr((*font).COLR, root, options);
    otfcc_dump_svg((*font).SVG_, root, options);
    otfcc_dump_tsi(
        (*font).TSI_01,
        root,
        options,
        b"TSI_01\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dump_tsi(
        (*font).TSI_23,
        root,
        options,
        b"TSI_23\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dump_tsi5((*font).TSI5, root, options);
    return root as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn serialize_to_json(
    mut font: *mut Font,
    mut options: *const Options,
) -> *mut ::core::ffi::c_void {
    <JsonSerializer as FontSerializer>::serialize(
        font as *mut ::core::ffi::c_void,
        options as *const ::core::ffi::c_void,
    )
}
unsafe extern "C" fn free_json_writer(mut self_0: *mut IFontSerializer) {
    free(self_0 as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_new_json_writer() -> *mut IFontSerializer {
    let mut writer: *mut IFontSerializer = ::core::ptr::null_mut::<IFontSerializer>();
    writer = __caryll_allocate_clean(
        ::core::mem::size_of::<IFontSerializer>() as usize,
        52 as ::core::ffi::c_ulong,
    ) as *mut IFontSerializer;
    (*writer).serialize = Some(
        serialize_to_json
            as unsafe extern "C" fn(
                *mut Font,
                *const Options,
            ) -> *mut ::core::ffi::c_void,
    )
        as Option<
            unsafe extern "C" fn(*mut Font, *const Options) -> *mut ::core::ffi::c_void,
        >;
    (*writer).free = Some(free_json_writer as unsafe extern "C" fn(*mut IFontSerializer) -> ())
        as Option<unsafe extern "C" fn(*mut IFontSerializer) -> ()>;
    return writer;
}
