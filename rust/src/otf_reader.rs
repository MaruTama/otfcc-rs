#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod unconsolidate;

use libc::{free};





use crate::support::alloc::{__caryll_allocate_clean};


use crate::support::options::{Options};
use crate::support::primitives::{GlyphId, ShapeId};

use crate::font::caryll_font::{FontSubtype, Font, IFontBuilder};
use crate::font::caryll_sfnt::{Packet, PacketPiece, SplineFontContainer};


use crate::table::cff::{CffAndGlyf};
use crate::table::glyf::GlyfIOContext;

use crate::font::caryll_font::{OTFCC_I_FONT};
use crate::otf_reader::unconsolidate::{otfcc_unconsolidate_font};
use crate::table::base::{otfcc_read_base};
use crate::table::cff::{otfcc_read_cff_and_glyf_tables};
use crate::table::colr::{otfcc_read_colr};
use crate::table::cpal::{otfcc_read_cpal};
use crate::table::gdef::{otfcc_read_gdef};
use crate::table::ltsh::{otfcc_read_ltsh};
use crate::table::os_2::{otfcc_read_os_2};
use crate::table::svg::{otfcc_read_svg};
use crate::table::tsi5::{otfcc_read_tsi5};
use crate::table::vorg::{otfcc_read_vorg};
use crate::table::_tsi::{otfcc_read_tsi};
use crate::table::cmap::{otfcc_read_cmap};
use crate::table::cvt::{otfcc_read_cvt};
use crate::table::fpgm_prep::{otfcc_read_fpgm_prep};
use crate::table::fvar::{otfcc_read_fvar};
use crate::table::gasp::{otfcc_read_gasp};
use crate::table::glyf::read::{otfcc_read_glyf};
use crate::table::head::{otfcc_read_head};
use crate::table::hhea::{otfcc_read_hhea};
use crate::table::hmtx::{otfcc_read_hmtx};
use crate::table::maxp::{otfcc_read_maxp};
use crate::table::meta::read::{otfcc_read_meta};
use crate::table::name::{otfcc_read_name};
use crate::table::otl::read::{otfcc_read_otl};
use crate::table::post::{otfcc_read_post};
use crate::table::vdmx::funcs::{otfcc_read_vdmx};
use crate::table::vhea::{otfcc_read_vhea};
use crate::table::vmtx::{otfcc_read_vmtx};





unsafe extern "C" fn decide_font_subtype_otf(
    sfnt: *mut SplineFontContainer,
    index: u32,
) -> FontSubtype {
    // c2rust's translation of a FOREACH_TABLE-style macro: the
    // __fortable_keep/__notfound/__fortable_k2 flags simulate a
    // single-iteration inner scope purely to give the original C a labeled
    // break/continue target. Traced by hand: the whole thing reduces to
    // "return FontSubtype::Cff at the first 'cff ' tag, else FontSubtype::Ttf".
    let packet: Packet = *(*sfnt).packets.offset(index as isize);
    for i in 0..packet.num_tables as ::core::ffi::c_int {
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
        (*font).subtype = decide_font_subtype_otf(sfnt, index);
        (*font).fvar = otfcc_read_fvar(packet, options);
        (*font).head = otfcc_read_head(packet, options);
        (*font).maxp = otfcc_read_maxp(packet, options);
        (*font).name = otfcc_read_name(packet, options);
        (*font).meta = otfcc_read_meta(packet, options);
        (*font).os_2 = otfcc_read_os_2(packet, options);
        (*font).post = otfcc_read_post(packet, options);
        (*font).hhea = otfcc_read_hhea(packet, options);
        (*font).cmap = otfcc_read_cmap(packet, options);
        if (*font).subtype == FontSubtype::Ttf {
            (*font).hmtx = otfcc_read_hmtx(packet, options, (*font).hhea, (*font).maxp);
            (*font).vhea = otfcc_read_vhea(packet, options);
            if !(*font).vhea.is_null() {
                (*font).vmtx = otfcc_read_vmtx(packet, options, (*font).vhea, (*font).maxp);
            }
            (*font).fpgm = otfcc_read_fpgm_prep(packet, options, 1718642541i32 as u32);
            (*font).prep = otfcc_read_fpgm_prep(packet, options, 1886545264i32 as u32);
            (*font).cvt_ = otfcc_read_cvt(packet, options, 1668707360i32 as u32);
            (*font).gasp = otfcc_read_gasp(packet, options);
            (*font).vdmx = otfcc_read_vdmx(packet, options);
            (*font).ltsh = otfcc_read_ltsh(packet, options);
            let mut ctx: GlyfIOContext = GlyfIOContext {
                loca_is_long: (*(*font).head).index_to_loc_format != 0,
                num_glyphs: (*(*font).maxp).num_glyphs as GlyphId,
                n_phantom_points: 4 as ShapeId,
                fvar: (*font).fvar,
                has_vertical_metrics: false,
                export_fd_select: false,
            };
            (*font).glyf = otfcc_read_glyf(packet, options, &raw mut ctx);
        } else {
            let mut cffpr: CffAndGlyf =
                otfcc_read_cff_and_glyf_tables(packet, options, (*font).head);
            (*font).cff = cffpr.meta;
            (*font).glyf = cffpr.glyphs;
            (*font).vhea = otfcc_read_vhea(packet, options);
            if !(*font).vhea.is_null() {
                (*font).vmtx = otfcc_read_vmtx(packet, options, (*font).vhea, (*font).maxp);
                (*font).vorg = otfcc_read_vorg(packet, options);
            }
        }
        if !(*font).glyf.is_null() {
            (*font).gsub = otfcc_read_otl(
                packet,
                options,
                1196643650i32 as u32,
                (*(*font).glyf).length as GlyphId,
            );
            (*font).gpos = otfcc_read_otl(
                packet,
                options,
                1196445523i32 as u32,
                (*(*font).glyf).length as GlyphId,
            );
            (*font).gdef = otfcc_read_gdef(packet, options);
        }
        (*font).base = otfcc_read_base(packet, options);
        (*font).cpal = otfcc_read_cpal(packet, options);
        (*font).colr = otfcc_read_colr(packet, options);
        (*font).svg = otfcc_read_svg(packet, options);
        (*font).tsi_01 = otfcc_read_tsi(
            packet,
            options,
            1414744368i32 as u32,
            1414744369i32 as u32,
        );
        (*font).tsi_23 = otfcc_read_tsi(
            packet,
            options,
            1414744370i32 as u32,
            1414744371i32 as u32,
        );
        (*font).tsi5 = otfcc_read_tsi5(packet, options);
        otfcc_unconsolidate_font(font, options);
        return font as *mut ::core::ffi::c_void;
    };
    }
}
unsafe extern "C" fn read_otf(
    mut _sfnt: *mut ::core::ffi::c_void,
    mut index: u32,
    mut options: *const Options,
) -> *mut Font {
    <OtfReader as FontBuilder>::read(_sfnt, index, options as *const ::core::ffi::c_void)
        as *mut Font
}
#[inline]
unsafe extern "C" fn free_reader(mut self_0: *mut IFontBuilder) {
    free(self_0 as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_new_otf_reader() -> *mut IFontBuilder {
    let mut reader: *mut IFontBuilder = ::core::ptr::null_mut::<IFontBuilder>();
    reader = __caryll_allocate_clean(
        ::core::mem::size_of::<IFontBuilder>() as usize,
        85 as ::core::ffi::c_ulong,
    ) as *mut IFontBuilder;
    (*reader).read = Some(
        read_otf
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
    (*reader).free = Some(free_reader as unsafe extern "C" fn(*mut IFontBuilder) -> ())
        as Option<unsafe extern "C" fn(*mut IFontBuilder) -> ()>;
    return reader;
}
