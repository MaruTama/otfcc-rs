#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};




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
use crate::table::cff::{TABLE_I_CFF};
use crate::table::name::{table_name_create};
use crate::table::otl::{table_otl_create, table_otl_free};





// `Copy, Clone` dropped: `Font` gained `ltsh: Option<Box<LtshTable>>` (Stage
// 6-4 pilot), which is never `Copy`. Grepping confirms `Font` is accessed
// exclusively via `*mut Font`/`(*font).field` throughout the crate -- never
// returned, constructed as a value literal, or `.clone()`'d -- so dropping
// the derive is safe (same check as `CffTable`/`NameRecord`/`GlyphOrderEntry`).
#[repr(C)]
pub struct Font {
    pub subtype: FontSubtype,
    pub fvar: Option<Box<FvarTable>>,
    pub head: Option<Box<HeadTable>>,
    pub hhea: Option<Box<HheaTable>>,
    pub maxp: Option<Box<MaxpTable>>,
    pub os_2: Option<Box<Os2Table>>,
    pub hmtx: Option<Box<HmtxTable>>,
    pub post: Option<Box<PostTable>>,
    pub hdmx: Option<Box<HdmxTable>>,
    pub vhea: Option<Box<VheaTable>>,
    pub vmtx: Option<Box<VmtxTable>>,
    pub vorg: Option<Box<VorgTable>>,
    pub cff: *mut CffTable,
    pub glyf: Option<GlyfTable>,
    pub cmap: Option<Box<CmapTable>>,
    pub name: Option<NameTable>,
    pub meta: Option<Box<MetaTable>>,
    pub fpgm: Option<Box<FpgmPrepTable>>,
    pub prep: Option<Box<FpgmPrepTable>>,
    pub cvt_: Option<Box<CvtTable>>,
    pub gasp: Option<Box<GaspTable>>,
    pub vdmx: Option<Box<VdmxTable>>,
    pub ltsh: Option<Box<LtshTable>>,
    pub gsub: *mut OtlTable,
    pub gpos: *mut OtlTable,
    pub gdef: Option<Box<GdefTable>>,
    pub base: Option<Box<BaseTable>>,
    pub cpal: Option<Box<CpalTable>>,
    pub colr: Option<ColrTable>,
    pub svg: Option<SvgTable>,
    pub tsi_01: Option<TsiTable>,
    pub tsi_23: Option<TsiTable>,
    pub tsi5: Option<Box<Tsi5Table>>,
    pub glyph_order: Option<Box<GlyphOrder>>,
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
            (*font).head = None;
            return;
        }
        1751672161 => {
            (*font).hhea = None;
            return;
        }
        1835104368 => {
            (*font).maxp = None;
            return;
        }
        1330863922 | 1330851634 => {
            (*font).os_2 = None;
            return;
        }
        1851878757 => {
            (*font).name = None;
            return;
        }
        1835365473 => {
            (*font).meta = None;
            return;
        }
        1752003704 => {
            (*font).hmtx = None;
            return;
        }
        1986884728 => {
            (*font).vmtx = None;
            return;
        }
        1886352244 => {
            (*font).post = None;
            return;
        }
        1986553185 => {
            (*font).vhea = None;
            return;
        }
        1718642541 => {
            (*font).fpgm = None;
            return;
        }
        1886545264 => {
            (*font).prep = None;
            return;
        }
        1668707423 | 1668707360 => {
            (*font).cvt_ = None;
            return;
        }
        1734439792 => {
            (*font).gasp = None;
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
            (*font).glyf = None;
            return;
        }
        1668112752 => {
            (*font).cmap = None;
            return;
        }
        1280594760 => {
            (*font).ltsh = None;
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
            (*font).gdef = None;
            return;
        }
        1111577413 => {
            (*font).base = None;
            return;
        }
        1448038983 => {
            (*font).vorg = None;
            return;
        }
        1129333068 => {
            (*font).cpal = None;
            return;
        }
        1129270354 => {
            (*font).colr = None;
            return;
        }
        1398163232 | 1398163295 => {
            (*font).svg = None;
            return;
        }
        1414744368 | 1414744369 => {
            (*font).tsi_01 = None;
            return;
        }
        1414744370 | 1414744371 => {
            (*font).tsi_23 = None;
            return;
        }
        1414744373 => {
            (*font).tsi5 = None;
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
    (*font).glyph_order = None;
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
