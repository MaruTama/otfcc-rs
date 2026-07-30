#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};
use crate::table::otl::classdef::{ClassDef, otl_ClassDef_free};




use crate::support::options::{Options};




use crate::support::{NULL};
use crate::support::glyph_order::GlyphOrder;
use crate::table::BASE::BaseTable;
use crate::table::CFF::CffTable;
use crate::table::COLR::ColrTable;
use crate::table::CPAL::CpalTable;
use crate::table::GDEF::GdefTable;
use crate::table::LTSH::LtshTable;
use crate::table::OS_2::Os2Table;
use crate::table::SVG::SvgTable;
use crate::table::TSI5::{Tsi5Table};
use crate::table::VORG::VorgTable;
use crate::table::_TSI::TsiTable;
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
use crate::consolidate::{otfcc_consolidateFont};
use crate::support::glyph_order::{otfcc_pkgGlyphOrder};
use crate::table::BASE::{table_iBASE};
use crate::table::CFF::{table_iCFF};
use crate::table::COLR::{table_iCOLR};
use crate::table::CPAL::{table_iCPAL};
use crate::table::GDEF::{table_iGDEF};
use crate::table::LTSH::{table_iLTSH};
use crate::table::OS_2::{table_iOS_2};
use crate::table::SVG::{table_iSVG};
use crate::table::VORG::{table_iVORG};
use crate::table::_TSI::{table_iTSI};
use crate::table::cmap::{table_iCmap};
use crate::table::cvt::{table_iCvt};
use crate::table::fpgm_prep::{table_iFpgm_prep};
use crate::table::gasp::{table_iGasp};
use crate::table::glyf::{table_iGlyf};
use crate::table::head::{table_iHead};
use crate::table::hhea::{table_iHhea};
use crate::table::hmtx::{table_iHmtx};
use crate::table::maxp::{table_iMaxp};
use crate::table::meta::types::{table_iMeta};
use crate::table::name::{table_iName};
use crate::table::otl::{table_iOTL};
use crate::table::post::{iTable_post};
use crate::table::vhea::{table_iVhea};
use crate::table::vmtx::{table_iVmtx};





#[derive(Copy, Clone)]
#[repr(C)]
pub struct Font {
    pub subtype: FontSubtype,
    pub fvar: *mut FvarTable,
    pub head: *mut HeadTable,
    pub hhea: *mut HheaTable,
    pub maxp: *mut MaxpTable,
    pub OS_2: *mut Os2Table,
    pub hmtx: *mut HmtxTable,
    pub post: *mut PostTable,
    pub hdmx: *mut HdmxTable,
    pub vhea: *mut VheaTable,
    pub vmtx: *mut VmtxTable,
    pub VORG: *mut VorgTable,
    pub CFF_: *mut CffTable,
    pub glyf: *mut GlyfTable,
    pub cmap: *mut CmapTable,
    pub name: *mut NameTable,
    pub meta: *mut MetaTable,
    pub fpgm: *mut FpgmPrepTable,
    pub prep: *mut FpgmPrepTable,
    pub cvt_: *mut CvtTable,
    pub gasp: *mut GaspTable,
    pub VDMX: *mut VdmxTable,
    pub LTSH: *mut LtshTable,
    pub GSUB: *mut OtlTable,
    pub GPOS: *mut OtlTable,
    pub GDEF: *mut GdefTable,
    pub BASE: *mut BaseTable,
    pub CPAL: *mut CpalTable,
    pub COLR: *mut ColrTable,
    pub SVG_: *mut SvgTable,
    pub TSI_01: *mut TsiTable,
    pub TSI_23: *mut TsiTable,
    pub TSI5: *mut Tsi5Table,
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
    pub move_0: Option<unsafe extern "C" fn(*mut Font, *mut Font) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut Font) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut Font, Font) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut Font, Font) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut Font>,
    pub free: Option<unsafe extern "C" fn(*mut Font) -> ()>,
    pub consolidate: Option<unsafe extern "C" fn(*mut Font, *const Options) -> ()>,
    pub createTable:
        Option<unsafe extern "C" fn(*mut Font, u32) -> *mut ::core::ffi::c_void>,
    pub deleteTable: Option<unsafe extern "C" fn(*mut Font, u32) -> ()>,
}
unsafe extern "C" fn createFontTable(
    mut _font: *mut Font,
    tag: u32,
) -> *mut ::core::ffi::c_void {
    match tag {
        1851878757 => {
            return (
                table_iName.create.expect("non-null function pointer"))() as *mut ::core::ffi::c_void;
        }
        1196643650 | 1196445523 => {
            return (
                table_iOTL.create.expect("non-null function pointer"))() as *mut ::core::ffi::c_void;
        }
        _ => return NULL,
    };
}
unsafe extern "C" fn deleteFontTable(mut font: *mut Font, tag: u32) {
    match tag {
        1751474532 => {
            if !(*font).head.is_null() {
                table_iHead.free.expect("non-null function pointer")((*font).head);
                (*font).head = ::core::ptr::null_mut::<HeadTable>();
            }
            return;
        }
        1751672161 => {
            if !(*font).hhea.is_null() {
                table_iHhea.free.expect("non-null function pointer")((*font).hhea);
                (*font).hhea = ::core::ptr::null_mut::<HheaTable>();
            }
            return;
        }
        1835104368 => {
            if !(*font).maxp.is_null() {
                table_iMaxp.free.expect("non-null function pointer")((*font).maxp);
                (*font).maxp = ::core::ptr::null_mut::<MaxpTable>();
            }
            return;
        }
        1330863922 | 1330851634 => {
            if !(*font).OS_2.is_null() {
                table_iOS_2.free.expect("non-null function pointer")((*font).OS_2);
                (*font).OS_2 = ::core::ptr::null_mut::<Os2Table>();
            }
            return;
        }
        1851878757 => {
            if !(*font).name.is_null() {
                table_iName.free.expect("non-null function pointer")((*font).name);
                (*font).name = ::core::ptr::null_mut::<NameTable>();
            }
            return;
        }
        1835365473 => {
            if !(*font).meta.is_null() {
                table_iMeta.free.expect("non-null function pointer")((*font).meta);
                (*font).meta = ::core::ptr::null_mut::<MetaTable>();
            }
            return;
        }
        1752003704 => {
            if !(*font).hmtx.is_null() {
                table_iHmtx.free.expect("non-null function pointer")((*font).hmtx);
                (*font).hmtx = ::core::ptr::null_mut::<HmtxTable>();
            }
            return;
        }
        1986884728 => {
            if !(*font).vmtx.is_null() {
                table_iVmtx.free.expect("non-null function pointer")((*font).vmtx);
                (*font).vmtx = ::core::ptr::null_mut::<VmtxTable>();
            }
            return;
        }
        1886352244 => {
            if !(*font).post.is_null() {
                iTable_post.free.expect("non-null function pointer")((*font).post);
                (*font).post = ::core::ptr::null_mut::<PostTable>();
            }
            return;
        }
        1986553185 => {
            if !(*font).vhea.is_null() {
                table_iVhea.free.expect("non-null function pointer")((*font).vhea);
                (*font).vhea = ::core::ptr::null_mut::<VheaTable>();
            }
            return;
        }
        1718642541 => {
            if !(*font).fpgm.is_null() {
                table_iFpgm_prep.free.expect("non-null function pointer")((*font).fpgm);
                (*font).fpgm = ::core::ptr::null_mut::<FpgmPrepTable>();
            }
            return;
        }
        1886545264 => {
            if !(*font).prep.is_null() {
                table_iFpgm_prep.free.expect("non-null function pointer")((*font).prep);
                (*font).prep = ::core::ptr::null_mut::<FpgmPrepTable>();
            }
            return;
        }
        1668707423 | 1668707360 => {
            if !(*font).cvt_.is_null() {
                table_iCvt.free.expect("non-null function pointer")((*font).cvt_);
                (*font).cvt_ = ::core::ptr::null_mut::<CvtTable>();
            }
            return;
        }
        1734439792 => {
            if !(*font).gasp.is_null() {
                table_iGasp.free.expect("non-null function pointer")((*font).gasp);
                (*font).gasp = ::core::ptr::null_mut::<GaspTable>();
            }
            return;
        }
        1128679007 | 1128678944 => {
            if !(*font).CFF_.is_null() {
                table_iCFF.free.expect("non-null function pointer")((*font).CFF_);
                (*font).CFF_ = ::core::ptr::null_mut::<CffTable>();
            }
            return;
        }
        1735162214 => {
            if !(*font).glyf.is_null() {
                table_iGlyf.free.expect("non-null function pointer")((*font).glyf);
                (*font).glyf = ::core::ptr::null_mut::<GlyfTable>();
            }
            return;
        }
        1668112752 => {
            if !(*font).cmap.is_null() {
                table_iCmap.free.expect("non-null function pointer")((*font).cmap);
                (*font).cmap = ::core::ptr::null_mut::<CmapTable>();
            }
            return;
        }
        1280594760 => {
            if !(*font).LTSH.is_null() {
                table_iLTSH.free.expect("non-null function pointer")((*font).LTSH);
                (*font).LTSH = ::core::ptr::null_mut::<LtshTable>();
            }
            return;
        }
        1196643650 => {
            if !(*font).GSUB.is_null() {
                table_iOTL.free.expect("non-null function pointer")((*font).GSUB);
                (*font).GSUB = ::core::ptr::null_mut::<OtlTable>();
            }
            return;
        }
        1196445523 => {
            if !(*font).GPOS.is_null() {
                table_iOTL.free.expect("non-null function pointer")((*font).GPOS);
                (*font).GPOS = ::core::ptr::null_mut::<OtlTable>();
            }
            return;
        }
        1195656518 => {
            if !(*font).GDEF.is_null() {
                table_iGDEF.free.expect("non-null function pointer")((*font).GDEF);
                (*font).GDEF = ::core::ptr::null_mut::<GdefTable>();
            }
            return;
        }
        1111577413 => {
            if !(*font).BASE.is_null() {
                table_iBASE.free.expect("non-null function pointer")((*font).BASE);
                (*font).BASE = ::core::ptr::null_mut::<BaseTable>();
            }
            return;
        }
        1448038983 => {
            if !(*font).VORG.is_null() {
                table_iVORG.free.expect("non-null function pointer")((*font).VORG);
                (*font).VORG = ::core::ptr::null_mut::<VorgTable>();
            }
            return;
        }
        1129333068 => {
            if !(*font).CPAL.is_null() {
                table_iCPAL.free.expect("non-null function pointer")((*font).CPAL);
                (*font).CPAL = ::core::ptr::null_mut::<CpalTable>();
            }
            return;
        }
        1129270354 => {
            if !(*font).COLR.is_null() {
                table_iCOLR.free.expect("non-null function pointer")((*font).COLR);
                (*font).COLR = ::core::ptr::null_mut::<ColrTable>();
            }
            return;
        }
        1398163232 | 1398163295 => {
            if !(*font).SVG_.is_null() {
                table_iSVG.free.expect("non-null function pointer")((*font).SVG_);
                (*font).SVG_ = ::core::ptr::null_mut::<SvgTable>();
            }
            return;
        }
        1414744368 | 1414744369 => {
            if !(*font).TSI_01.is_null() {
                table_iTSI.free.expect("non-null function pointer")((*font).TSI_01);
                (*font).TSI_01 = ::core::ptr::null_mut::<TsiTable>();
            }
            return;
        }
        1414744370 | 1414744371 => {
            if !(*font).TSI_23.is_null() {
                table_iTSI.free.expect("non-null function pointer")((*font).TSI_23);
                (*font).TSI_23 = ::core::ptr::null_mut::<TsiTable>();
            }
            return;
        }
        1414744373 => {
            if !(*font).TSI5.is_null() {
                otl_ClassDef_free(
                    (*font).TSI5 as *mut ClassDef,
                );
                (*font).TSI5 = ::core::ptr::null_mut::<Tsi5Table>();
            }
            return;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn initFont(mut font: *mut Font) {
    memset(
        font as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Font>() as usize,
    );
}
#[inline]
unsafe extern "C" fn disposeFont(mut font: *mut Font) {
    deleteFontTable(font, 1751474532i32 as u32);
    deleteFontTable(font, 1751672161i32 as u32);
    deleteFontTable(font, 1835104368i32 as u32);
    deleteFontTable(font, 1330863922i32 as u32);
    deleteFontTable(font, 1851878757i32 as u32);
    deleteFontTable(font, 1835365473i32 as u32);
    deleteFontTable(font, 1752003704i32 as u32);
    deleteFontTable(font, 1986884728i32 as u32);
    deleteFontTable(font, 1886352244i32 as u32);
    deleteFontTable(font, 1751412088i32 as u32);
    deleteFontTable(font, 1986553185i32 as u32);
    deleteFontTable(font, 1718642541i32 as u32);
    deleteFontTable(font, 1886545264i32 as u32);
    deleteFontTable(font, 1668707423i32 as u32);
    deleteFontTable(font, 1734439792i32 as u32);
    deleteFontTable(font, 1128679007i32 as u32);
    deleteFontTable(font, 1735162214i32 as u32);
    deleteFontTable(font, 1668112752i32 as u32);
    deleteFontTable(font, 1280594760i32 as u32);
    deleteFontTable(font, 1196643650i32 as u32);
    deleteFontTable(font, 1196445523i32 as u32);
    deleteFontTable(font, 1195656518i32 as u32);
    deleteFontTable(font, 1111577413i32 as u32);
    deleteFontTable(font, 1448038983i32 as u32);
    deleteFontTable(font, 1129333068i32 as u32);
    deleteFontTable(font, 1129270354i32 as u32);
    deleteFontTable(font, 1398163295i32 as u32);
    deleteFontTable(font, 1414744368i32 as u32);
    deleteFontTable(font, 1414744370i32 as u32);
    deleteFontTable(font, 1414744373i32 as u32);
    otfcc_pkgGlyphOrder.free.expect("non-null function pointer")((*font).glyph_order);
}
#[inline]
unsafe extern "C" fn otfcc_Font_dispose(mut x: *mut Font) {
    disposeFont(x);
}
#[inline]
unsafe extern "C" fn otfcc_Font_create() -> *mut Font {
    let mut x: *mut Font =
        malloc(::core::mem::size_of::<Font>() as usize) as *mut Font;
    otfcc_Font_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otfcc_Font_init(mut x: *mut Font) {
    initFont(x);
}
#[inline]
unsafe extern "C" fn otfcc_Font_free(mut x: *mut Font) {
    if x.is_null() {
        return;
    }
    otfcc_Font_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otfcc_Font_copyReplace(mut dst: *mut Font, src: Font) {
    otfcc_Font_dispose(dst);
    otfcc_Font_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otfcc_Font_copy(mut dst: *mut Font, mut src: *const Font) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Font>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_Font_replace(mut dst: *mut Font, src: Font) {
    otfcc_Font_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Font>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_Font_move(mut dst: *mut Font, mut src: *mut Font) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Font>() as usize,
    );
    otfcc_Font_init(src);
}
pub static otfcc_iFont: FontElementInterface = {
    FontElementInterface {
        init: Some(otfcc_Font_init as unsafe extern "C" fn(*mut Font) -> ()),
        copy: Some(
            otfcc_Font_copy as unsafe extern "C" fn(*mut Font, *const Font) -> (),
        ),
        move_0: Some(
            otfcc_Font_move as unsafe extern "C" fn(*mut Font, *mut Font) -> (),
        ),
        dispose: Some(otfcc_Font_dispose as unsafe extern "C" fn(*mut Font) -> ()),
        replace: Some(
            otfcc_Font_replace as unsafe extern "C" fn(*mut Font, Font) -> (),
        ),
        copyReplace: Some(
            otfcc_Font_copyReplace as unsafe extern "C" fn(*mut Font, Font) -> (),
        ),
        create: Some(otfcc_Font_create),
        free: Some(otfcc_Font_free as unsafe extern "C" fn(*mut Font) -> ()),
        consolidate: Some(
            otfcc_consolidateFont
                as unsafe extern "C" fn(*mut Font, *const Options) -> (),
        ),
        createTable: Some(
            createFontTable
                as unsafe extern "C" fn(*mut Font, u32) -> *mut ::core::ffi::c_void,
        ),
        deleteTable: Some(deleteFontTable as unsafe extern "C" fn(*mut Font, u32) -> ()),
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
