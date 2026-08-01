#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};
use crate::table::otl::classdef::{ClassDef, otl_class_def_free};




use crate::support::options::{Options};




use crate::support::{NULL};
use crate::support::glyph_order::GlyphOrder;
use crate::table::base::BaseTable;
use crate::table::cff::CffTable;
use crate::table::colr::ColrTable;
use crate::table::cpal::CpalTable;
use crate::table::gdef::GdefTable;
use crate::table::ltsh::LtshTable;
use crate::table::os_2::Os2Table;
use crate::table::svg::SvgTable;
use crate::table::tsi5::{Tsi5Table};
use crate::table::vorg::VorgTable;
use crate::table::_tsi::TsiTable;
use crate::table::cmap::CmapTable;
use crate::table::cvt::CvtTable;
use crate::table::fpgm_prep::FpgmPrepTable;
use crate::table::fvar::{FvarTable};
use crate::table::gasp::GaspTable;
use crate::table::glyf::GlyfTable;
use crate::table::hdmx::{HdmxTable};
use crate::table::head::HeadTable;
use crate::table::hhea::HheaTable;
use crate::table::hmtx::HmtxTable;
use crate::table::maxp::MaxpTable;
use crate::table::meta::types::MetaTable;
use crate::table::name::NameTable;
use crate::table::otl::OtlTable;
use crate::table::post::PostTable;
use crate::table::vdmx::types::{VdmxTable};
use crate::table::vhea::VheaTable;
use crate::table::vmtx::VmtxTable;
use crate::consolidate::{otfcc_consolidate_font};
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::table::base::{TABLE_I_BASE};
use crate::table::cff::{TABLE_I_CFF};
use crate::table::colr::{table_colr_free};
use crate::table::cpal::{table_cpal_free};
use crate::table::gdef::{table_gdef_free};
use crate::table::ltsh::{TABLE_I_LTSH};
use crate::table::os_2::{TABLE_I_OS_2};
use crate::table::svg::{table_svg_free};
use crate::table::vorg::{TABLE_I_VORG};
use crate::table::_tsi::{table_tsi_free};
use crate::table::cmap::{TABLE_I_CMAP};
use crate::table::cvt::{TABLE_I_CVT};
use crate::table::fpgm_prep::{TABLE_I_FPGM_PREP};
use crate::table::gasp::{TABLE_I_GASP};
use crate::table::glyf::{TABLE_I_GLYF};
use crate::table::head::{TABLE_I_HEAD};
use crate::table::hhea::{TABLE_I_HHEA};
use crate::table::hmtx::{TABLE_I_HMTX};
use crate::table::maxp::{TABLE_I_MAXP};
use crate::table::meta::types::{TABLE_I_META};
use crate::table::name::{table_name_create, table_name_free};
use crate::table::otl::{table_otl_create, table_otl_free};
use crate::table::post::{I_TABLE_POST};
use crate::table::vhea::{TABLE_I_VHEA};
use crate::table::vmtx::{TABLE_I_VMTX};





#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub subtype: FontSubtype,
    pub fvar: *mut FvarTable,
    pub head: *mut HeadTable,
    pub hhea: *mut HheaTable,
    pub maxp: *mut MaxpTable,
    pub os_2: *mut Os2Table,
    pub hmtx: *mut HmtxTable,
    pub post: *mut PostTable,
    pub hdmx: *mut HdmxTable,
    pub vhea: *mut VheaTable,
    pub vmtx: *mut VmtxTable,
    pub vorg: *mut VorgTable,
    pub cff: *mut CffTable,
    pub glyf: *mut GlyfTable,
    pub cmap: *mut CmapTable,
    pub name: *mut NameTable,
    pub meta: *mut MetaTable,
    pub fpgm: *mut FpgmPrepTable,
    pub prep: *mut FpgmPrepTable,
    pub cvt_: *mut CvtTable,
    pub gasp: *mut GaspTable,
    pub vdmx: *mut VdmxTable,
    pub ltsh: *mut LtshTable,
    pub gsub: *mut OtlTable,
    pub gpos: *mut OtlTable,
    pub gdef: *mut GdefTable,
    pub base: *mut BaseTable,
    pub cpal: *mut CpalTable,
    pub colr: *mut ColrTable,
    pub svg: *mut SvgTable,
    pub tsi_01: *mut TsiTable,
    pub tsi_23: *mut TsiTable,
    pub tsi5: *mut Tsi5Table,
    pub glyph_order: *mut GlyphOrder,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontSubtype {
    Ttf = 0,
    Cff = 1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FontElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut Font) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut Font, *const Font) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut Font) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut Font>,
    pub free: Option<unsafe extern "C" fn(*mut Font) -> ()>,
    pub consolidate: Option<unsafe extern "C" fn(*mut Font, *const Options) -> ()>,
    pub create_table:
        Option<unsafe extern "C" fn(*mut Font, u32) -> *mut ::core::ffi::c_void>,
    pub delete_table: Option<unsafe extern "C" fn(*mut Font, u32) -> ()>,
}
unsafe extern "C" fn create_font_table(
    mut _font: *mut Font,
    tag: u32,
) -> *mut ::core::ffi::c_void {
    match tag {
        1851878757 => {
            return table_name_create() as *mut ::core::ffi::c_void;
        }
        1196643650 | 1196445523 => {
            return table_otl_create() as *mut ::core::ffi::c_void;
        }
        _ => return NULL,
    };
}
unsafe extern "C" fn delete_font_table(mut font: *mut Font, tag: u32) {
    match tag {
        1751474532 => {
            if !(*font).head.is_null() {
                TABLE_I_HEAD.free.expect("non-null function pointer")((*font).head);
                (*font).head = ::core::ptr::null_mut::<HeadTable>();
            }
            return;
        }
        1751672161 => {
            if !(*font).hhea.is_null() {
                TABLE_I_HHEA.free.expect("non-null function pointer")((*font).hhea);
                (*font).hhea = ::core::ptr::null_mut::<HheaTable>();
            }
            return;
        }
        1835104368 => {
            if !(*font).maxp.is_null() {
                TABLE_I_MAXP.free.expect("non-null function pointer")((*font).maxp);
                (*font).maxp = ::core::ptr::null_mut::<MaxpTable>();
            }
            return;
        }
        1330863922 | 1330851634 => {
            if !(*font).os_2.is_null() {
                TABLE_I_OS_2.free.expect("non-null function pointer")((*font).os_2);
                (*font).os_2 = ::core::ptr::null_mut::<Os2Table>();
            }
            return;
        }
        1851878757 => {
            if !(*font).name.is_null() {
                table_name_free((*font).name);
                (*font).name = ::core::ptr::null_mut::<NameTable>();
            }
            return;
        }
        1835365473 => {
            if !(*font).meta.is_null() {
                TABLE_I_META.free.expect("non-null function pointer")((*font).meta);
                (*font).meta = ::core::ptr::null_mut::<MetaTable>();
            }
            return;
        }
        1752003704 => {
            if !(*font).hmtx.is_null() {
                TABLE_I_HMTX.free.expect("non-null function pointer")((*font).hmtx);
                (*font).hmtx = ::core::ptr::null_mut::<HmtxTable>();
            }
            return;
        }
        1986884728 => {
            if !(*font).vmtx.is_null() {
                TABLE_I_VMTX.free.expect("non-null function pointer")((*font).vmtx);
                (*font).vmtx = ::core::ptr::null_mut::<VmtxTable>();
            }
            return;
        }
        1886352244 => {
            if !(*font).post.is_null() {
                I_TABLE_POST.free.expect("non-null function pointer")((*font).post);
                (*font).post = ::core::ptr::null_mut::<PostTable>();
            }
            return;
        }
        1986553185 => {
            if !(*font).vhea.is_null() {
                TABLE_I_VHEA.free.expect("non-null function pointer")((*font).vhea);
                (*font).vhea = ::core::ptr::null_mut::<VheaTable>();
            }
            return;
        }
        1718642541 => {
            if !(*font).fpgm.is_null() {
                TABLE_I_FPGM_PREP.free.expect("non-null function pointer")((*font).fpgm);
                (*font).fpgm = ::core::ptr::null_mut::<FpgmPrepTable>();
            }
            return;
        }
        1886545264 => {
            if !(*font).prep.is_null() {
                TABLE_I_FPGM_PREP.free.expect("non-null function pointer")((*font).prep);
                (*font).prep = ::core::ptr::null_mut::<FpgmPrepTable>();
            }
            return;
        }
        1668707423 | 1668707360 => {
            if !(*font).cvt_.is_null() {
                TABLE_I_CVT.free.expect("non-null function pointer")((*font).cvt_);
                (*font).cvt_ = ::core::ptr::null_mut::<CvtTable>();
            }
            return;
        }
        1734439792 => {
            if !(*font).gasp.is_null() {
                TABLE_I_GASP.free.expect("non-null function pointer")((*font).gasp);
                (*font).gasp = ::core::ptr::null_mut::<GaspTable>();
            }
            return;
        }
        1128679007 | 1128678944 => {
            if !(*font).cff.is_null() {
                TABLE_I_CFF.free.expect("non-null function pointer")((*font).cff);
                (*font).cff = ::core::ptr::null_mut::<CffTable>();
            }
            return;
        }
        1735162214 => {
            if !(*font).glyf.is_null() {
                TABLE_I_GLYF.free.expect("non-null function pointer")((*font).glyf);
                (*font).glyf = ::core::ptr::null_mut::<GlyfTable>();
            }
            return;
        }
        1668112752 => {
            if !(*font).cmap.is_null() {
                TABLE_I_CMAP.free.expect("non-null function pointer")((*font).cmap);
                (*font).cmap = ::core::ptr::null_mut::<CmapTable>();
            }
            return;
        }
        1280594760 => {
            if !(*font).ltsh.is_null() {
                TABLE_I_LTSH.free.expect("non-null function pointer")((*font).ltsh);
                (*font).ltsh = ::core::ptr::null_mut::<LtshTable>();
            }
            return;
        }
        1196643650 => {
            if !(*font).gsub.is_null() {
                table_otl_free((*font).gsub);
                (*font).gsub = ::core::ptr::null_mut::<OtlTable>();
            }
            return;
        }
        1196445523 => {
            if !(*font).gpos.is_null() {
                table_otl_free((*font).gpos);
                (*font).gpos = ::core::ptr::null_mut::<OtlTable>();
            }
            return;
        }
        1195656518 => {
            if !(*font).gdef.is_null() {
                table_gdef_free((*font).gdef);
                (*font).gdef = ::core::ptr::null_mut::<GdefTable>();
            }
            return;
        }
        1111577413 => {
            if !(*font).base.is_null() {
                TABLE_I_BASE.free.expect("non-null function pointer")((*font).base);
                (*font).base = ::core::ptr::null_mut::<BaseTable>();
            }
            return;
        }
        1448038983 => {
            if !(*font).vorg.is_null() {
                TABLE_I_VORG.free.expect("non-null function pointer")((*font).vorg);
                (*font).vorg = ::core::ptr::null_mut::<VorgTable>();
            }
            return;
        }
        1129333068 => {
            if !(*font).cpal.is_null() {
                table_cpal_free((*font).cpal);
                (*font).cpal = ::core::ptr::null_mut::<CpalTable>();
            }
            return;
        }
        1129270354 => {
            if !(*font).colr.is_null() {
                table_colr_free((*font).colr);
                (*font).colr = ::core::ptr::null_mut::<ColrTable>();
            }
            return;
        }
        1398163232 | 1398163295 => {
            if !(*font).svg.is_null() {
                table_svg_free((*font).svg);
                (*font).svg = ::core::ptr::null_mut::<SvgTable>();
            }
            return;
        }
        1414744368 | 1414744369 => {
            if !(*font).tsi_01.is_null() {
                table_tsi_free((*font).tsi_01);
                (*font).tsi_01 = ::core::ptr::null_mut::<TsiTable>();
            }
            return;
        }
        1414744370 | 1414744371 => {
            if !(*font).tsi_23.is_null() {
                table_tsi_free((*font).tsi_23);
                (*font).tsi_23 = ::core::ptr::null_mut::<TsiTable>();
            }
            return;
        }
        1414744373 => {
            if !(*font).tsi5.is_null() {
                otl_class_def_free(
                    (*font).tsi5 as *mut ClassDef,
                );
                (*font).tsi5 = ::core::ptr::null_mut::<Tsi5Table>();
            }
            return;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn init_font(mut font: *mut Font) {
    memset(
        font as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Font>() as usize,
    );
}
#[inline]
unsafe extern "C" fn dispose_font(mut font: *mut Font) {
    delete_font_table(font, 1751474532i32 as u32);
    delete_font_table(font, 1751672161i32 as u32);
    delete_font_table(font, 1835104368i32 as u32);
    delete_font_table(font, 1330863922i32 as u32);
    delete_font_table(font, 1851878757i32 as u32);
    delete_font_table(font, 1835365473i32 as u32);
    delete_font_table(font, 1752003704i32 as u32);
    delete_font_table(font, 1986884728i32 as u32);
    delete_font_table(font, 1886352244i32 as u32);
    delete_font_table(font, 1751412088i32 as u32);
    delete_font_table(font, 1986553185i32 as u32);
    delete_font_table(font, 1718642541i32 as u32);
    delete_font_table(font, 1886545264i32 as u32);
    delete_font_table(font, 1668707423i32 as u32);
    delete_font_table(font, 1734439792i32 as u32);
    delete_font_table(font, 1128679007i32 as u32);
    delete_font_table(font, 1735162214i32 as u32);
    delete_font_table(font, 1668112752i32 as u32);
    delete_font_table(font, 1280594760i32 as u32);
    delete_font_table(font, 1196643650i32 as u32);
    delete_font_table(font, 1196445523i32 as u32);
    delete_font_table(font, 1195656518i32 as u32);
    delete_font_table(font, 1111577413i32 as u32);
    delete_font_table(font, 1448038983i32 as u32);
    delete_font_table(font, 1129333068i32 as u32);
    delete_font_table(font, 1129270354i32 as u32);
    delete_font_table(font, 1398163295i32 as u32);
    delete_font_table(font, 1414744368i32 as u32);
    delete_font_table(font, 1414744370i32 as u32);
    delete_font_table(font, 1414744373i32 as u32);
    OTFCC_PKG_GLYPH_ORDER.free.expect("non-null function pointer")((*font).glyph_order);
}
#[inline]
unsafe extern "C" fn otfcc_font_dispose(mut x: *mut Font) {
    dispose_font(x);
}
#[inline]
unsafe extern "C" fn otfcc_font_create() -> *mut Font {
    let mut x: *mut Font =
        malloc(::core::mem::size_of::<Font>() as usize) as *mut Font;
    otfcc_font_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otfcc_font_init(mut x: *mut Font) {
    init_font(x);
}
#[inline]
unsafe extern "C" fn otfcc_font_free(mut x: *mut Font) {
    if x.is_null() {
        return;
    }
    otfcc_font_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otfcc_font_copy(mut dst: *mut Font, mut src: *const Font) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Font>() as usize,
    );
}
pub static OTFCC_I_FONT: FontElementInterface = {
    FontElementInterface {
        init: Some(otfcc_font_init as unsafe extern "C" fn(*mut Font) -> ()),
        copy: Some(
            otfcc_font_copy as unsafe extern "C" fn(*mut Font, *const Font) -> (),
        ),
        dispose: Some(otfcc_font_dispose as unsafe extern "C" fn(*mut Font) -> ()),
        create: Some(otfcc_font_create),
        free: Some(otfcc_font_free as unsafe extern "C" fn(*mut Font) -> ()),
        consolidate: Some(
            otfcc_consolidate_font
                as unsafe extern "C" fn(*mut Font, *const Options) -> (),
        ),
        create_table: Some(
            create_font_table
                as unsafe extern "C" fn(*mut Font, u32) -> *mut ::core::ffi::c_void,
        ),
        delete_table: Some(delete_font_table as unsafe extern "C" fn(*mut Font, u32) -> ()),
    }
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IFontBuilder {
    pub read: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            u32,
            *const Options,
        ) -> *mut Font,
    >,
    pub free: Option<unsafe extern "C" fn(*mut IFontBuilder) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IFontSerializer {
    pub serialize: Option<
        unsafe extern "C" fn(*mut Font, *const Options) -> *mut ::core::ffi::c_void,
    >,
    pub free: Option<unsafe extern "C" fn(*mut IFontSerializer) -> ()>,
}
