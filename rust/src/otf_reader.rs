#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod unconsolidate;

use libc::{free};





use crate::support::alloc::{__caryll_allocate_clean};


use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, ShapeId};

use crate::font::caryll_font::{FontSubtype, Font, IFontBuilder};
use crate::font::caryll_sfnt::{Packet, PacketPiece, SplineFontContainer};


use crate::table::CFF::{CffAndGlyf};
use crate::table::glyf::GlyfIOContext;

use crate::font::caryll_font::{OTFCC_I_FONT};
use crate::otf_reader::unconsolidate::{otfcc_unconsolidateFont};
use crate::table::BASE::{otfcc_readBASE};
use crate::table::CFF::{otfcc_readCFFAndGlyfTables};
use crate::table::COLR::{otfcc_readCOLR};
use crate::table::CPAL::{otfcc_readCPAL};
use crate::table::GDEF::{otfcc_readGDEF};
use crate::table::LTSH::{otfcc_readLTSH};
use crate::table::OS_2::{otfcc_readOS_2};
use crate::table::SVG::{otfcc_readSVG};
use crate::table::TSI5::{otfcc_readTSI5};
use crate::table::VORG::{otfcc_readVORG};
use crate::table::_TSI::{otfcc_readTSI};
use crate::table::cmap::{otfcc_readCmap};
use crate::table::cvt::{otfcc_readCvt};
use crate::table::fpgm_prep::{otfcc_readFpgmPrep};
use crate::table::fvar::{otfcc_readFvar};
use crate::table::gasp::{otfcc_readGasp};
use crate::table::glyf::read::{otfcc_readGlyf};
use crate::table::head::{otfcc_readHead};
use crate::table::hhea::{otfcc_readHhea};
use crate::table::hmtx::{otfcc_readHmtx};
use crate::table::maxp::{otfcc_readMaxp};
use crate::table::meta::read::{otfcc_readMeta};
use crate::table::name::{otfcc_readName};
use crate::table::otl::read::{otfcc_readOtl};
use crate::table::post::{otfcc_readPost};
use crate::table::vdmx::funcs::{otfcc_readVDMX};
use crate::table::vhea::{otfcc_readVhea};
use crate::table::vmtx::{otfcc_readVmtx};





unsafe extern "C" fn decideFontSubtypeOTF(
    sfnt: *mut SplineFontContainer,
    index: u32,
) -> FontSubtype {
    // c2rust's translation of a FOREACH_TABLE-style macro: the
    // __fortable_keep/__notfound/__fortable_k2 flags simulate a
    // single-iteration inner scope purely to give the original C a labeled
    // break/continue target. Traced by hand: the whole thing reduces to
    // "return FontSubtype::Cff at the first 'CFF ' tag, else FontSubtype::Ttf".
    let packet: Packet = *(*sfnt).packets.offset(index as isize);
    for i in 0..packet.numTables as ::core::ffi::c_int {
        let table: PacketPiece = *packet.pieces.offset(i as isize);
        if table.tag == 1128678944i32 as u32 {
            return FontSubtype::Cff;
        }
    }
    return FontSubtype::Ttf;
}
// Options and Font are duplicated per-file by c2rust (like every
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
    let options = options as *const Options;
    let mut sfnt: *mut SplineFontContainer = _sfnt as *mut SplineFontContainer;
    if (*sfnt).count.wrapping_sub(1 as u32) < index {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    } else {
        let font: *mut Font = (
            OTFCC_I_FONT.create.expect("non-null function pointer"))();
        let packet: Packet = *(*sfnt).packets.offset(index as isize);
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
        if (*font).subtype == FontSubtype::Ttf {
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
                numGlyphs: (*(*font).maxp).numGlyphs as GlyphId,
                nPhantomPoints: 4 as ShapeId,
                fvar: (*font).fvar,
                hasVerticalMetrics: false,
                exportFDSelect: false,
            };
            (*font).glyf = otfcc_readGlyf(packet, options, &raw mut ctx);
        } else {
            let mut cffpr: CffAndGlyf =
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
                (*(*font).glyf).length as GlyphId,
            );
            (*font).GPOS = otfcc_readOtl(
                packet,
                options,
                1196445523i32 as u32,
                (*(*font).glyf).length as GlyphId,
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
    mut options: *const Options,
) -> *mut Font {
    <OtfReader as FontBuilder>::read(_sfnt, index, options as *const ::core::ffi::c_void)
        as *mut Font
}
#[inline]
unsafe extern "C" fn freeReader(mut self_0: *mut IFontBuilder) {
    free(self_0 as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_newOTFReader() -> *mut IFontBuilder {
    let mut reader: *mut IFontBuilder = ::core::ptr::null_mut::<IFontBuilder>();
    reader = __caryll_allocate_clean(
        ::core::mem::size_of::<IFontBuilder>() as usize,
        85 as ::core::ffi::c_ulong,
    ) as *mut IFontBuilder;
    (*reader).read = Some(
        readOtf
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                u32,
                *const Options,
            ) -> *mut Font,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                u32,
                *const Options,
            ) -> *mut Font,
        >;
    (*reader).free = Some(freeReader as unsafe extern "C" fn(*mut IFontBuilder) -> ())
        as Option<unsafe extern "C" fn(*mut IFontBuilder) -> ()>;
    return reader;
}
