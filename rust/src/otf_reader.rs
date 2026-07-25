pub mod unconsolidate;

use libc::{free};
extern "C" {
    static otfcc_iFont: __caryll_elementinterface_otfcc_Font;
    fn otfcc_readFvar(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_fvar;
    fn otfcc_readHead(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_head;
    fn otfcc_readGlyf(
        packet: otfcc_Packet,
        options: *const otfcc_Options,
        ctx: *const GlyfIOContext,
    ) -> *mut table_glyf;
    fn otfcc_readCFFAndGlyfTables(
        packet: otfcc_Packet,
        options: *const otfcc_Options,
        head: *const table_head,
    ) -> table_CFFAndGlyf;
    fn otfcc_readMaxp(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_maxp;
    fn otfcc_readHhea(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_hhea;
    fn otfcc_readVhea(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_vhea;
    fn otfcc_readVmtx(
        packet: otfcc_Packet,
        options: *const otfcc_Options,
        vhea: *mut table_vhea,
        maxp: *mut table_maxp,
    ) -> *mut table_vmtx;
    fn otfcc_readOS_2(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_OS_2;
    fn otfcc_readPost(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_post;
    fn otfcc_readName(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_name;
    fn otfcc_readMeta(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_meta;
    fn otfcc_readCmap(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_cmap;
    fn otfcc_readCvt(
        packet: otfcc_Packet,
        options: *const otfcc_Options,
        tag: u32,
    ) -> *mut table_cvt;
    fn otfcc_readFpgmPrep(
        packet: otfcc_Packet,
        options: *const otfcc_Options,
        tag: u32,
    ) -> *mut table_fpgm_prep;
    fn otfcc_readGasp(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_gasp;
    fn otfcc_readVDMX(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_VDMX;
    fn otfcc_readLTSH(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_LTSH;
    fn otfcc_readVORG(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_VORG;
    fn otfcc_readGDEF(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_GDEF;
    fn otfcc_readBASE(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_BASE;
    fn otfcc_readOtl(
        packet: otfcc_Packet,
        options: *const otfcc_Options,
        tag: u32,
        maxGlyphs: glyphid_t,
    ) -> *mut table_OTL;
    fn otfcc_readCPAL(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_CPAL;
    fn otfcc_readCOLR(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_COLR;
    fn otfcc_readSVG(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_SVG;
    fn otfcc_readTSI(
        packet: otfcc_Packet,
        options: *const otfcc_Options,
        tagIndex: u32,
        tagText: u32,
    ) -> *mut table_TSI;
    fn otfcc_readTSI5(packet: otfcc_Packet, options: *const otfcc_Options) -> *mut table_TSI5;
    fn otfcc_unconsolidateFont(font: *mut otfcc_Font, options: *const otfcc_Options);
    fn otfcc_readHmtx(
        packet: otfcc_Packet,
        options: *const otfcc_Options,
        hhea: *mut table_hhea,
        maxp: *mut table_maxp,
    ) -> *mut table_hmtx;
}





use crate::support::alloc::{__caryll_allocate_clean};


use crate::support::options::{otfcc_Options};
use crate::support::primitives::{glyphid_t, shapeid_t};

use crate::font::caryll_font::{FONTTYPE_CFF, FONTTYPE_TTF, __caryll_elementinterface_otfcc_Font, otfcc_Font, otfcc_IFontBuilder, otfcc_font_subtype};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece, otfcc_SplineFontContainer};


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
use crate::table::_TSI::{table_TSI};
use crate::table::cmap::{table_cmap};
use crate::table::cvt::{table_cvt};
use crate::table::fpgm_prep::{table_fpgm_prep};
use crate::table::fvar::{table_fvar};
use crate::table::gasp::{table_gasp};
use crate::table::glyf::{GlyfIOContext, table_glyf};

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





unsafe extern "C" fn decideFontSubtypeOTF(
    sfnt: *mut otfcc_SplineFontContainer,
    index: u32,
) -> otfcc_font_subtype {
    // c2rust's translation of a FOREACH_TABLE-style macro: the
    // __fortable_keep/__notfound/__fortable_k2 flags simulate a
    // single-iteration inner scope purely to give the original C a labeled
    // break/continue target. Traced by hand: the whole thing reduces to
    // "return FONTTYPE_CFF at the first 'CFF ' tag, else FONTTYPE_TTF".
    let packet: otfcc_Packet = *(*sfnt).packets.offset(index as isize);
    for i in 0..packet.numTables as ::core::ffi::c_int {
        let table: otfcc_PacketPiece = *packet.pieces.offset(i as isize);
        if table.tag == 1128678944i32 as u32 {
            return FONTTYPE_CFF;
        }
    }
    return FONTTYPE_TTF;
}
// otfcc_Options and otfcc_Font are duplicated per-file by c2rust (like every
// other type in this crate); the trait boundary uses erased c_void pointers
// so this trait can be shared with json_reader.rs without deduping those
// pervasively-used types. Casts are confined to the boundary; the pointee
// layout is unchanged (same technique already relied on for the excluded
// dump/parse/build methods in Track 1's package vtables).
pub(crate) trait FontBuilder {
    unsafe fn read(
        buf: *mut ::core::ffi::c_void,
        len: u32,
        options: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}
struct OtfReader;
impl FontBuilder for OtfReader {
    unsafe fn read(
    mut _sfnt: *mut ::core::ffi::c_void,
    mut index: u32,
    options: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let options = options as *const otfcc_Options;
    let mut sfnt: *mut otfcc_SplineFontContainer = _sfnt as *mut otfcc_SplineFontContainer;
    if (*sfnt).count.wrapping_sub(1 as u32) < index {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    } else {
        let font: *mut otfcc_Font = (
            otfcc_iFont.create.expect("non-null function pointer"))();
        let packet: otfcc_Packet = *(*sfnt).packets.offset(index as isize);
        (*font).subtype = decideFontSubtypeOTF(sfnt, index);
        (*font).fvar = otfcc_readFvar(packet, options);
        (*font).head = otfcc_readHead(packet, options);
        (*font).maxp = otfcc_readMaxp(packet, options);
        (*font).name = otfcc_readName(packet, options);
        (*font).meta = otfcc_readMeta(packet, options);
        (*font).OS_2 = otfcc_readOS_2(packet, options);
        (*font).post = otfcc_readPost(packet, options);
        (*font).hhea = otfcc_readHhea(packet, options);
        (*font).cmap = otfcc_readCmap(packet, options);
        if (*font).subtype == FONTTYPE_TTF {
            (*font).hmtx = otfcc_readHmtx(packet, options, (*font).hhea, (*font).maxp);
            (*font).vhea = otfcc_readVhea(packet, options);
            if !(*font).vhea.is_null() {
                (*font).vmtx = otfcc_readVmtx(packet, options, (*font).vhea, (*font).maxp);
            }
            (*font).fpgm = otfcc_readFpgmPrep(packet, options, 1718642541i32 as u32);
            (*font).prep = otfcc_readFpgmPrep(packet, options, 1886545264i32 as u32);
            (*font).cvt_ = otfcc_readCvt(packet, options, 1668707360i32 as u32);
            (*font).gasp = otfcc_readGasp(packet, options);
            (*font).VDMX = otfcc_readVDMX(packet, options);
            (*font).LTSH = otfcc_readLTSH(packet, options);
            let mut ctx: GlyfIOContext = GlyfIOContext {
                locaIsLong: (*(*font).head).indexToLocFormat != 0,
                numGlyphs: (*(*font).maxp).numGlyphs as glyphid_t,
                nPhantomPoints: 4 as shapeid_t,
                fvar: (*font).fvar,
                hasVerticalMetrics: false,
                exportFDSelect: false,
            };
            (*font).glyf = otfcc_readGlyf(packet, options, &raw mut ctx);
        } else {
            let mut cffpr: table_CFFAndGlyf =
                otfcc_readCFFAndGlyfTables(packet, options, (*font).head);
            (*font).CFF_ = cffpr.meta;
            (*font).glyf = cffpr.glyphs;
            (*font).vhea = otfcc_readVhea(packet, options);
            if !(*font).vhea.is_null() {
                (*font).vmtx = otfcc_readVmtx(packet, options, (*font).vhea, (*font).maxp);
                (*font).VORG = otfcc_readVORG(packet, options);
            }
        }
        if !(*font).glyf.is_null() {
            (*font).GSUB = otfcc_readOtl(
                packet,
                options,
                1196643650i32 as u32,
                (*(*font).glyf).length as glyphid_t,
            );
            (*font).GPOS = otfcc_readOtl(
                packet,
                options,
                1196445523i32 as u32,
                (*(*font).glyf).length as glyphid_t,
            );
            (*font).GDEF = otfcc_readGDEF(packet, options);
        }
        (*font).BASE = otfcc_readBASE(packet, options);
        (*font).CPAL = otfcc_readCPAL(packet, options);
        (*font).COLR = otfcc_readCOLR(packet, options);
        (*font).SVG_ = otfcc_readSVG(packet, options);
        (*font).TSI_01 = otfcc_readTSI(
            packet,
            options,
            1414744368i32 as u32,
            1414744369i32 as u32,
        );
        (*font).TSI_23 = otfcc_readTSI(
            packet,
            options,
            1414744370i32 as u32,
            1414744371i32 as u32,
        );
        (*font).TSI5 = otfcc_readTSI5(packet, options);
        otfcc_unconsolidateFont(font, options);
        return font as *mut ::core::ffi::c_void;
    };
    }
}
unsafe extern "C" fn readOtf(
    mut _sfnt: *mut ::core::ffi::c_void,
    mut index: u32,
    mut options: *const otfcc_Options,
) -> *mut otfcc_Font {
    <OtfReader as FontBuilder>::read(_sfnt, index, options as *const ::core::ffi::c_void)
        as *mut otfcc_Font
}
#[inline]
unsafe extern "C" fn freeReader(mut self_0: *mut otfcc_IFontBuilder) {
    free(self_0 as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_newOTFReader() -> *mut otfcc_IFontBuilder {
    let mut reader: *mut otfcc_IFontBuilder = ::core::ptr::null_mut::<otfcc_IFontBuilder>();
    reader = __caryll_allocate_clean(
        ::core::mem::size_of::<otfcc_IFontBuilder>() as usize,
        85 as ::core::ffi::c_ulong,
    ) as *mut otfcc_IFontBuilder;
    (*reader).read = Some(
        readOtf
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                u32,
                *const otfcc_Options,
            ) -> *mut otfcc_Font,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                u32,
                *const otfcc_Options,
            ) -> *mut otfcc_Font,
        >;
    (*reader).free = Some(freeReader as unsafe extern "C" fn(*mut otfcc_IFontBuilder) -> ())
        as Option<unsafe extern "C" fn(*mut otfcc_IFontBuilder) -> ()>;
    return reader;
}
