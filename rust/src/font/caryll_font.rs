use libc::{free, malloc, memcpy, memset};
extern "C" {
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    static otl_iClassDef: __otfcc_IClassDef;
    static table_iHead: __caryll_elementinterface_table_head;
    static table_iHhea: __caryll_elementinterface_table_hhea;
    static table_iMaxp: __caryll_elementinterface_table_maxp;
    static table_iOS_2: __caryll_elementinterface_table_OS_2;
    static table_iHmtx: __caryll_elementinterface_table_hmtx;
    static iTable_post: __caryll_elementinterface_table_post;
    static table_iVhea: __caryll_elementinterface_table_vhea;
    static table_iVORG: __caryll_elementinterface_table_VORG;
    static table_iGasp: __caryll_elementinterface_table_gasp;
    static table_iVmtx: __caryll_elementinterface_table_vmtx;
    static table_iGlyf: __caryll_vectorinterface_table_glyf;
    static table_iName: __caryll_vectorinterface_table_name;
    static table_iMeta: __caryll_elementinterface_table_meta;
    static table_iFpgm_prep: __caryll_elementinterface_table_fpgm_prep;
    static table_iCFF: __caryll_elementinterface_table_CFF;
    static table_iCmap: __caryll_elementinterface_table_cmap;
    static table_iOTL: __caryll_elementinterface_table_OTL;
    static table_iGDEF: __caryll_elementinterface_table_GDEF;
    static table_iLTSH: __caryll_elementinterface_table_LTSH;
    static table_iCPAL: __caryll_elementinterface_table_CPAL;
    static table_iBASE: __caryll_elementinterface_table_BASE;
    static table_iCOLR: __caryll_vectorinterface_table_COLR;
    static table_iSVG: __caryll_vectorinterface_table_SVG;
    static table_iTSI: __caryll_vectorinterface_table_TSI;
    static table_iCvt: __caryll_elementinterface_table_cvt;
    fn otfcc_consolidateFont(font: *mut otfcc_Font, options: *const otfcc_Options);
}
use crate::table::otl::classdef::{__otfcc_IClassDef, otl_ClassDef, otl_ClassDef_free};




use crate::support::options::{otfcc_Options};




use crate::support::{NULL};
use crate::support::glyph_order::{otfcc_GlyphOrder, otfcc_GlyphOrderPackage};
use crate::table::BASE::{__caryll_elementinterface_table_BASE, table_BASE};
use crate::table::CFF::{__caryll_elementinterface_table_CFF, table_CFF};
use crate::table::COLR::{__caryll_vectorinterface_table_COLR, table_COLR};
use crate::table::CPAL::{__caryll_elementinterface_table_CPAL, table_CPAL};
use crate::table::GDEF::{__caryll_elementinterface_table_GDEF, table_GDEF};
use crate::table::LTSH::{__caryll_elementinterface_table_LTSH, table_LTSH};
use crate::table::OS_2::{__caryll_elementinterface_table_OS_2, table_OS_2};
use crate::table::SVG::{__caryll_vectorinterface_table_SVG, table_SVG};
use crate::table::TSI5::{table_TSI5};
use crate::table::VORG::{__caryll_elementinterface_table_VORG, table_VORG};
use crate::table::_TSI::{__caryll_vectorinterface_table_TSI, table_TSI};
use crate::table::cmap::{__caryll_elementinterface_table_cmap, table_cmap};
use crate::table::cvt::{__caryll_elementinterface_table_cvt, table_cvt};
use crate::table::fpgm_prep::{__caryll_elementinterface_table_fpgm_prep, table_fpgm_prep};
use crate::table::fvar::{table_fvar};
use crate::table::gasp::{__caryll_elementinterface_table_gasp, table_gasp};
use crate::table::glyf::{__caryll_vectorinterface_table_glyf, table_glyf};
use crate::table::hdmx::{table_hdmx};
use crate::table::head::{__caryll_elementinterface_table_head, table_head};
use crate::table::hhea::{__caryll_elementinterface_table_hhea, table_hhea};
use crate::table::hmtx::{__caryll_elementinterface_table_hmtx, table_hmtx};
use crate::table::maxp::{__caryll_elementinterface_table_maxp, table_maxp};
use crate::table::meta::types::{__caryll_elementinterface_table_meta, table_meta};
use crate::table::name::{__caryll_vectorinterface_table_name, table_name};
use crate::table::otl::{__caryll_elementinterface_table_OTL, table_OTL};
use crate::table::post::{__caryll_elementinterface_table_post, table_post};
use crate::table::vdmx::types::{table_VDMX};
use crate::table::vhea::{__caryll_elementinterface_table_vhea, table_vhea};
use crate::table::vmtx::{__caryll_elementinterface_table_vmtx, table_vmtx};





#[derive(Copy, Clone)]
#[repr(C)]
pub struct _caryll_font {
    pub subtype: otfcc_font_subtype,
    pub fvar: *mut table_fvar,
    pub head: *mut table_head,
    pub hhea: *mut table_hhea,
    pub maxp: *mut table_maxp,
    pub OS_2: *mut table_OS_2,
    pub hmtx: *mut table_hmtx,
    pub post: *mut table_post,
    pub hdmx: *mut table_hdmx,
    pub vhea: *mut table_vhea,
    pub vmtx: *mut table_vmtx,
    pub VORG: *mut table_VORG,
    pub CFF_: *mut table_CFF,
    pub glyf: *mut table_glyf,
    pub cmap: *mut table_cmap,
    pub name: *mut table_name,
    pub meta: *mut table_meta,
    pub fpgm: *mut table_fpgm_prep,
    pub prep: *mut table_fpgm_prep,
    pub cvt_: *mut table_cvt,
    pub gasp: *mut table_gasp,
    pub VDMX: *mut table_VDMX,
    pub LTSH: *mut table_LTSH,
    pub GSUB: *mut table_OTL,
    pub GPOS: *mut table_OTL,
    pub GDEF: *mut table_GDEF,
    pub BASE: *mut table_BASE,
    pub CPAL: *mut table_CPAL,
    pub COLR: *mut table_COLR,
    pub SVG_: *mut table_SVG,
    pub TSI_01: *mut table_TSI,
    pub TSI_23: *mut table_TSI,
    pub TSI5: *mut table_TSI5,
    pub glyph_order: *mut otfcc_GlyphOrder,
}
pub type otfcc_font_subtype = ::core::ffi::c_uint;
pub const FONTTYPE_CFF: otfcc_font_subtype = 1;
pub const FONTTYPE_TTF: otfcc_font_subtype = 0;
pub type otfcc_Font = _caryll_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otfcc_Font {
    pub init: Option<unsafe extern "C" fn(*mut otfcc_Font) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Font) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otfcc_Font, *mut otfcc_Font) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_Font) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otfcc_Font, otfcc_Font) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otfcc_Font, otfcc_Font) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otfcc_Font>,
    pub free: Option<unsafe extern "C" fn(*mut otfcc_Font) -> ()>,
    pub consolidate: Option<unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Options) -> ()>,
    pub createTable:
        Option<unsafe extern "C" fn(*mut otfcc_Font, u32) -> *mut ::core::ffi::c_void>,
    pub deleteTable: Option<unsafe extern "C" fn(*mut otfcc_Font, u32) -> ()>,
}
unsafe extern "C" fn createFontTable(
    mut _font: *mut otfcc_Font,
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
unsafe extern "C" fn deleteFontTable(mut font: *mut otfcc_Font, tag: u32) {
    match tag {
        1751474532 => {
            if !(*font).head.is_null() {
                table_iHead.free.expect("non-null function pointer")((*font).head);
                (*font).head = ::core::ptr::null_mut::<table_head>();
            }
            return;
        }
        1751672161 => {
            if !(*font).hhea.is_null() {
                table_iHhea.free.expect("non-null function pointer")((*font).hhea);
                (*font).hhea = ::core::ptr::null_mut::<table_hhea>();
            }
            return;
        }
        1835104368 => {
            if !(*font).maxp.is_null() {
                table_iMaxp.free.expect("non-null function pointer")((*font).maxp);
                (*font).maxp = ::core::ptr::null_mut::<table_maxp>();
            }
            return;
        }
        1330863922 | 1330851634 => {
            if !(*font).OS_2.is_null() {
                table_iOS_2.free.expect("non-null function pointer")((*font).OS_2);
                (*font).OS_2 = ::core::ptr::null_mut::<table_OS_2>();
            }
            return;
        }
        1851878757 => {
            if !(*font).name.is_null() {
                table_iName.free.expect("non-null function pointer")((*font).name);
                (*font).name = ::core::ptr::null_mut::<table_name>();
            }
            return;
        }
        1835365473 => {
            if !(*font).meta.is_null() {
                table_iMeta.free.expect("non-null function pointer")((*font).meta);
                (*font).meta = ::core::ptr::null_mut::<table_meta>();
            }
            return;
        }
        1752003704 => {
            if !(*font).hmtx.is_null() {
                table_iHmtx.free.expect("non-null function pointer")((*font).hmtx);
                (*font).hmtx = ::core::ptr::null_mut::<table_hmtx>();
            }
            return;
        }
        1986884728 => {
            if !(*font).vmtx.is_null() {
                table_iVmtx.free.expect("non-null function pointer")((*font).vmtx);
                (*font).vmtx = ::core::ptr::null_mut::<table_vmtx>();
            }
            return;
        }
        1886352244 => {
            if !(*font).post.is_null() {
                iTable_post.free.expect("non-null function pointer")((*font).post);
                (*font).post = ::core::ptr::null_mut::<table_post>();
            }
            return;
        }
        1986553185 => {
            if !(*font).vhea.is_null() {
                table_iVhea.free.expect("non-null function pointer")((*font).vhea);
                (*font).vhea = ::core::ptr::null_mut::<table_vhea>();
            }
            return;
        }
        1718642541 => {
            if !(*font).fpgm.is_null() {
                table_iFpgm_prep.free.expect("non-null function pointer")((*font).fpgm);
                (*font).fpgm = ::core::ptr::null_mut::<table_fpgm_prep>();
            }
            return;
        }
        1886545264 => {
            if !(*font).prep.is_null() {
                table_iFpgm_prep.free.expect("non-null function pointer")((*font).prep);
                (*font).prep = ::core::ptr::null_mut::<table_fpgm_prep>();
            }
            return;
        }
        1668707423 | 1668707360 => {
            if !(*font).cvt_.is_null() {
                table_iCvt.free.expect("non-null function pointer")((*font).cvt_);
                (*font).cvt_ = ::core::ptr::null_mut::<table_cvt>();
            }
            return;
        }
        1734439792 => {
            if !(*font).gasp.is_null() {
                table_iGasp.free.expect("non-null function pointer")((*font).gasp);
                (*font).gasp = ::core::ptr::null_mut::<table_gasp>();
            }
            return;
        }
        1128679007 | 1128678944 => {
            if !(*font).CFF_.is_null() {
                table_iCFF.free.expect("non-null function pointer")((*font).CFF_);
                (*font).CFF_ = ::core::ptr::null_mut::<table_CFF>();
            }
            return;
        }
        1735162214 => {
            if !(*font).glyf.is_null() {
                table_iGlyf.free.expect("non-null function pointer")((*font).glyf);
                (*font).glyf = ::core::ptr::null_mut::<table_glyf>();
            }
            return;
        }
        1668112752 => {
            if !(*font).cmap.is_null() {
                table_iCmap.free.expect("non-null function pointer")((*font).cmap);
                (*font).cmap = ::core::ptr::null_mut::<table_cmap>();
            }
            return;
        }
        1280594760 => {
            if !(*font).LTSH.is_null() {
                table_iLTSH.free.expect("non-null function pointer")((*font).LTSH);
                (*font).LTSH = ::core::ptr::null_mut::<table_LTSH>();
            }
            return;
        }
        1196643650 => {
            if !(*font).GSUB.is_null() {
                table_iOTL.free.expect("non-null function pointer")((*font).GSUB);
                (*font).GSUB = ::core::ptr::null_mut::<table_OTL>();
            }
            return;
        }
        1196445523 => {
            if !(*font).GPOS.is_null() {
                table_iOTL.free.expect("non-null function pointer")((*font).GPOS);
                (*font).GPOS = ::core::ptr::null_mut::<table_OTL>();
            }
            return;
        }
        1195656518 => {
            if !(*font).GDEF.is_null() {
                table_iGDEF.free.expect("non-null function pointer")((*font).GDEF);
                (*font).GDEF = ::core::ptr::null_mut::<table_GDEF>();
            }
            return;
        }
        1111577413 => {
            if !(*font).BASE.is_null() {
                table_iBASE.free.expect("non-null function pointer")((*font).BASE);
                (*font).BASE = ::core::ptr::null_mut::<table_BASE>();
            }
            return;
        }
        1448038983 => {
            if !(*font).VORG.is_null() {
                table_iVORG.free.expect("non-null function pointer")((*font).VORG);
                (*font).VORG = ::core::ptr::null_mut::<table_VORG>();
            }
            return;
        }
        1129333068 => {
            if !(*font).CPAL.is_null() {
                table_iCPAL.free.expect("non-null function pointer")((*font).CPAL);
                (*font).CPAL = ::core::ptr::null_mut::<table_CPAL>();
            }
            return;
        }
        1129270354 => {
            if !(*font).COLR.is_null() {
                table_iCOLR.free.expect("non-null function pointer")((*font).COLR);
                (*font).COLR = ::core::ptr::null_mut::<table_COLR>();
            }
            return;
        }
        1398163232 | 1398163295 => {
            if !(*font).SVG_.is_null() {
                table_iSVG.free.expect("non-null function pointer")((*font).SVG_);
                (*font).SVG_ = ::core::ptr::null_mut::<table_SVG>();
            }
            return;
        }
        1414744368 | 1414744369 => {
            if !(*font).TSI_01.is_null() {
                table_iTSI.free.expect("non-null function pointer")((*font).TSI_01);
                (*font).TSI_01 = ::core::ptr::null_mut::<table_TSI>();
            }
            return;
        }
        1414744370 | 1414744371 => {
            if !(*font).TSI_23.is_null() {
                table_iTSI.free.expect("non-null function pointer")((*font).TSI_23);
                (*font).TSI_23 = ::core::ptr::null_mut::<table_TSI>();
            }
            return;
        }
        1414744373 => {
            if !(*font).TSI5.is_null() {
                otl_ClassDef_free(
                    (*font).TSI5 as *mut otl_ClassDef,
                );
                (*font).TSI5 = ::core::ptr::null_mut::<table_TSI5>();
            }
            return;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn initFont(mut font: *mut otfcc_Font) {
    memset(
        font as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<otfcc_Font>() as usize,
    );
}
#[inline]
unsafe extern "C" fn disposeFont(mut font: *mut otfcc_Font) {
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
unsafe extern "C" fn otfcc_Font_dispose(mut x: *mut otfcc_Font) {
    disposeFont(x);
}
#[inline]
unsafe extern "C" fn otfcc_Font_create() -> *mut otfcc_Font {
    let mut x: *mut otfcc_Font =
        malloc(::core::mem::size_of::<otfcc_Font>() as usize) as *mut otfcc_Font;
    otfcc_Font_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn otfcc_Font_init(mut x: *mut otfcc_Font) {
    initFont(x);
}
#[inline]
unsafe extern "C" fn otfcc_Font_free(mut x: *mut otfcc_Font) {
    if x.is_null() {
        return;
    }
    otfcc_Font_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otfcc_Font_copyReplace(mut dst: *mut otfcc_Font, src: otfcc_Font) {
    otfcc_Font_dispose(dst);
    otfcc_Font_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn otfcc_Font_copy(mut dst: *mut otfcc_Font, mut src: *const otfcc_Font) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_Font>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_Font_replace(mut dst: *mut otfcc_Font, src: otfcc_Font) {
    otfcc_Font_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_Font>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_Font_move(mut dst: *mut otfcc_Font, mut src: *mut otfcc_Font) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_Font>() as usize,
    );
    otfcc_Font_init(src);
}
#[no_mangle]
pub static mut otfcc_iFont: __caryll_elementinterface_otfcc_Font = {
    __caryll_elementinterface_otfcc_Font {
        init: Some(otfcc_Font_init as unsafe extern "C" fn(*mut otfcc_Font) -> ()),
        copy: Some(
            otfcc_Font_copy as unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Font) -> (),
        ),
        move_0: Some(
            otfcc_Font_move as unsafe extern "C" fn(*mut otfcc_Font, *mut otfcc_Font) -> (),
        ),
        dispose: Some(otfcc_Font_dispose as unsafe extern "C" fn(*mut otfcc_Font) -> ()),
        replace: Some(
            otfcc_Font_replace as unsafe extern "C" fn(*mut otfcc_Font, otfcc_Font) -> (),
        ),
        copyReplace: Some(
            otfcc_Font_copyReplace as unsafe extern "C" fn(*mut otfcc_Font, otfcc_Font) -> (),
        ),
        create: Some(otfcc_Font_create),
        free: Some(otfcc_Font_free as unsafe extern "C" fn(*mut otfcc_Font) -> ()),
        consolidate: Some(
            otfcc_consolidateFont
                as unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Options) -> (),
        ),
        createTable: Some(
            createFontTable
                as unsafe extern "C" fn(*mut otfcc_Font, u32) -> *mut ::core::ffi::c_void,
        ),
        deleteTable: Some(deleteFontTable as unsafe extern "C" fn(*mut otfcc_Font, u32) -> ()),
    }
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_IFontBuilder {
    pub read: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            u32,
            *const otfcc_Options,
        ) -> *mut otfcc_Font,
    >,
    pub free: Option<unsafe extern "C" fn(*mut otfcc_IFontBuilder) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_IFontSerializer {
    pub serialize: Option<
        unsafe extern "C" fn(*mut otfcc_Font, *const otfcc_Options) -> *mut ::core::ffi::c_void,
    >,
    pub free: Option<unsafe extern "C" fn(*mut otfcc_IFontSerializer) -> ()>,
}
