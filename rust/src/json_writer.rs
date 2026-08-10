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


use crate::table::base::{otfcc_dump_base};
use crate::table::cff::{otfcc_dump_cff};
use crate::table::colr::{otfcc_dump_colr};
use crate::table::cpal::{otfcc_dump_cpal};
use crate::table::gdef::{otfcc_dump_gdef};
use crate::table::os_2::{otfcc_dump_os_2};
use crate::table::svg::{otfcc_dump_svg};
use crate::table::tsi5::{otfcc_dump_tsi5};
use crate::table::_tsi::{otfcc_dump_tsi};
use crate::table::cmap::{otfcc_dump_cmap};
use crate::table::cvt::{otfcc_dump_cvt};
use crate::table::fpgm_prep::{table_dump_table_fpgm_prep};
use crate::table::fvar::{otfcc_dump_fvar, FvarTable};
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
    otfcc_dump_fvar((*font).fvar.as_deref(), root, options);
    otfcc_dump_head((*font).head.as_deref(), root, options);
    otfcc_dump_hhea((*font).hhea.as_deref(), root, options);
    otfcc_dump_maxp((*font).maxp.as_deref(), root, options);
    otfcc_dump_vhea((*font).vhea.as_deref(), root, options);
    otfcc_dump_post((*font).post.as_deref(), root, options);
    otfcc_dump_os_2((*font).os_2.as_deref(), root, options);
    otfcc_dump_name((*font).name, root, options);
    otfcc_dump_meta((*font).meta.as_deref(), root, options);
    otfcc_dump_cmap((*font).cmap.as_deref(), root, options);
    otfcc_dump_cff((*font).cff, root, options);
    let mut ctx: GlyfIOContext = GlyfIOContext {
        loca_is_long: (*font).head.as_deref().unwrap().index_to_loc_format != 0,
        num_glyphs: (*font).maxp.as_deref().unwrap().num_glyphs as GlyphId,
        n_phantom_points: 4 as ShapeId,
        fvar: (*font).fvar.as_deref_mut().map_or(::core::ptr::null_mut(), |f| f as *mut FvarTable),
        has_vertical_metrics: (*font).vhea.is_some(),
        export_fd_select: !(*font).cff.is_null() && (*(*font).cff).is_cid as ::core::ffi::c_int != 0,
    };
    otfcc_dump_glyf((*font).glyf, root, options, &raw mut ctx);
    if !(*options).ignore_hints {
        table_dump_table_fpgm_prep(
            (*font).fpgm.as_deref(),
            root,
            options,
            b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
        );
        table_dump_table_fpgm_prep(
            (*font).prep.as_deref(),
            root,
            options,
            b"prep\0" as *const u8 as *const ::core::ffi::c_char,
        );
        otfcc_dump_cvt(
            (*font).cvt_.as_deref(),
            root,
            options,
            b"cvt_\0" as *const u8 as *const ::core::ffi::c_char,
        );
        otfcc_dump_gasp((*font).gasp.as_deref(), root, options);
    }
    otfcc_dump_vdmx((*font).vdmx.as_deref(), root, options);
    otfcc_dump_otl(
        (*font).gsub,
        root,
        options,
        b"GSUB\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dump_otl(
        (*font).gpos,
        root,
        options,
        b"GPOS\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dump_gdef((*font).gdef.as_deref(), root, options);
    otfcc_dump_base((*font).base.as_deref(), root, options);
    otfcc_dump_cpal((*font).cpal.as_deref(), root, options);
    otfcc_dump_colr((*font).colr, root, options);
    otfcc_dump_svg((*font).svg, root, options);
    otfcc_dump_tsi(
        (*font).tsi_01,
        root,
        options,
        b"TSI_01\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dump_tsi(
        (*font).tsi_23,
        root,
        options,
        b"TSI_23\0" as *const u8 as *const ::core::ffi::c_char,
    );
    otfcc_dump_tsi5((*font).tsi5, root, options);
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
