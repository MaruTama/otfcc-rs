#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::primitives::{GlyphId};
use crate::support::glyph_order::GlyphOrder;
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::vendor::sds::{sdsnew};

pub unsafe extern "C" fn aglfn_setup_names(mut map: *mut GlyphOrder) {
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x41 as GlyphId,
        sdsnew(b"A\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc6 as GlyphId,
        sdsnew(b"AE\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1fc as GlyphId,
        sdsnew(b"AEacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc1 as GlyphId,
        sdsnew(b"Aacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x102 as GlyphId,
        sdsnew(b"Abreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc2 as GlyphId,
        sdsnew(b"Acircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc4 as GlyphId,
        sdsnew(b"Adieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc0 as GlyphId,
        sdsnew(b"Agrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x391 as GlyphId,
        sdsnew(b"Alpha\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x386 as GlyphId,
        sdsnew(b"Alphatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x100 as GlyphId,
        sdsnew(b"Amacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x104 as GlyphId,
        sdsnew(b"Aogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc5 as GlyphId,
        sdsnew(b"Aring\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1fa as GlyphId,
        sdsnew(b"Aringacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc3 as GlyphId,
        sdsnew(b"Atilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x42 as GlyphId,
        sdsnew(b"B\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x392 as GlyphId,
        sdsnew(b"Beta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x43 as GlyphId,
        sdsnew(b"C\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x106 as GlyphId,
        sdsnew(b"Cacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10c as GlyphId,
        sdsnew(b"Ccaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc7 as GlyphId,
        sdsnew(b"Ccedilla\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x108 as GlyphId,
        sdsnew(b"Ccircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10a as GlyphId,
        sdsnew(b"Cdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a7 as GlyphId,
        sdsnew(b"Chi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x44 as GlyphId,
        sdsnew(b"D\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10e as GlyphId,
        sdsnew(b"Dcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x110 as GlyphId,
        sdsnew(b"Dcroat\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2206 as GlyphId,
        sdsnew(b"Delta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x45 as GlyphId,
        sdsnew(b"E\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc9 as GlyphId,
        sdsnew(b"Eacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x114 as GlyphId,
        sdsnew(b"Ebreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11a as GlyphId,
        sdsnew(b"Ecaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xca as GlyphId,
        sdsnew(b"Ecircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xcb as GlyphId,
        sdsnew(b"Edieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x116 as GlyphId,
        sdsnew(b"Edotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc8 as GlyphId,
        sdsnew(b"Egrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x112 as GlyphId,
        sdsnew(b"Emacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14a as GlyphId,
        sdsnew(b"Eng\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x118 as GlyphId,
        sdsnew(b"Eogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x395 as GlyphId,
        sdsnew(b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x388 as GlyphId,
        sdsnew(b"Epsilontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x397 as GlyphId,
        sdsnew(b"Eta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x389 as GlyphId,
        sdsnew(b"Etatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd0 as GlyphId,
        sdsnew(b"Eth\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20ac as GlyphId,
        sdsnew(b"Euro\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x46 as GlyphId,
        sdsnew(b"F\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x47 as GlyphId,
        sdsnew(b"G\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x393 as GlyphId,
        sdsnew(b"Gamma\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11e as GlyphId,
        sdsnew(b"Gbreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e6 as GlyphId,
        sdsnew(b"Gcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11c as GlyphId,
        sdsnew(b"Gcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x120 as GlyphId,
        sdsnew(b"Gdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x48 as GlyphId,
        sdsnew(b"H\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25cf as GlyphId,
        sdsnew(b"H18533\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25aa as GlyphId,
        sdsnew(b"H18543\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25ab as GlyphId,
        sdsnew(b"H18551\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25a1 as GlyphId,
        sdsnew(b"H22073\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x126 as GlyphId,
        sdsnew(b"Hbar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x124 as GlyphId,
        sdsnew(b"Hcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x49 as GlyphId,
        sdsnew(b"I\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x132 as GlyphId,
        sdsnew(b"IJ\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xcd as GlyphId,
        sdsnew(b"Iacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12c as GlyphId,
        sdsnew(b"Ibreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xce as GlyphId,
        sdsnew(b"Icircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xcf as GlyphId,
        sdsnew(b"Idieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x130 as GlyphId,
        sdsnew(b"Idotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2111 as GlyphId,
        sdsnew(b"Ifraktur\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xcc as GlyphId,
        sdsnew(b"Igrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12a as GlyphId,
        sdsnew(b"Imacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12e as GlyphId,
        sdsnew(b"Iogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x399 as GlyphId,
        sdsnew(b"Iota\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3aa as GlyphId,
        sdsnew(b"Iotadieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x38a as GlyphId,
        sdsnew(b"Iotatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x128 as GlyphId,
        sdsnew(b"Itilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4a as GlyphId,
        sdsnew(b"J\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x134 as GlyphId,
        sdsnew(b"Jcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4b as GlyphId,
        sdsnew(b"K\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39a as GlyphId,
        sdsnew(b"Kappa\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4c as GlyphId,
        sdsnew(b"L\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x139 as GlyphId,
        sdsnew(b"Lacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39b as GlyphId,
        sdsnew(b"Lambda\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x13d as GlyphId,
        sdsnew(b"Lcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x13f as GlyphId,
        sdsnew(b"Ldot\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x141 as GlyphId,
        sdsnew(b"Lslash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4d as GlyphId,
        sdsnew(b"M\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39c as GlyphId,
        sdsnew(b"Mu\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4e as GlyphId,
        sdsnew(b"N\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x143 as GlyphId,
        sdsnew(b"Nacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x147 as GlyphId,
        sdsnew(b"Ncaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd1 as GlyphId,
        sdsnew(b"Ntilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39d as GlyphId,
        sdsnew(b"Nu\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4f as GlyphId,
        sdsnew(b"O\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x152 as GlyphId,
        sdsnew(b"OE\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd3 as GlyphId,
        sdsnew(b"Oacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14e as GlyphId,
        sdsnew(b"Obreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd4 as GlyphId,
        sdsnew(b"Ocircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd6 as GlyphId,
        sdsnew(b"Odieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd2 as GlyphId,
        sdsnew(b"Ograve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1a0 as GlyphId,
        sdsnew(b"Ohorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x150 as GlyphId,
        sdsnew(b"Ohungarumlaut\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14c as GlyphId,
        sdsnew(b"Omacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2126 as GlyphId,
        sdsnew(b"Omega\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x38f as GlyphId,
        sdsnew(b"Omegatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39f as GlyphId,
        sdsnew(b"Omicron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x38c as GlyphId,
        sdsnew(b"Omicrontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd8 as GlyphId,
        sdsnew(b"Oslash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1fe as GlyphId,
        sdsnew(b"Oslashacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd5 as GlyphId,
        sdsnew(b"Otilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x50 as GlyphId,
        sdsnew(b"P\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a6 as GlyphId,
        sdsnew(b"Phi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a0 as GlyphId,
        sdsnew(b"Pi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a8 as GlyphId,
        sdsnew(b"Psi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x51 as GlyphId,
        sdsnew(b"Q\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x52 as GlyphId,
        sdsnew(b"R\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x154 as GlyphId,
        sdsnew(b"Racute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x158 as GlyphId,
        sdsnew(b"Rcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x211c as GlyphId,
        sdsnew(b"Rfraktur\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a1 as GlyphId,
        sdsnew(b"Rho\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x53 as GlyphId,
        sdsnew(b"S\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x250c as GlyphId,
        sdsnew(b"SF010000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2514 as GlyphId,
        sdsnew(b"SF020000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2510 as GlyphId,
        sdsnew(b"SF030000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2518 as GlyphId,
        sdsnew(b"SF040000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x253c as GlyphId,
        sdsnew(b"SF050000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x252c as GlyphId,
        sdsnew(b"SF060000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2534 as GlyphId,
        sdsnew(b"SF070000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x251c as GlyphId,
        sdsnew(b"SF080000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2524 as GlyphId,
        sdsnew(b"SF090000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2500 as GlyphId,
        sdsnew(b"SF100000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2502 as GlyphId,
        sdsnew(b"SF110000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2561 as GlyphId,
        sdsnew(b"SF190000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2562 as GlyphId,
        sdsnew(b"SF200000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2556 as GlyphId,
        sdsnew(b"SF210000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2555 as GlyphId,
        sdsnew(b"SF220000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2563 as GlyphId,
        sdsnew(b"SF230000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2551 as GlyphId,
        sdsnew(b"SF240000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2557 as GlyphId,
        sdsnew(b"SF250000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255d as GlyphId,
        sdsnew(b"SF260000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255c as GlyphId,
        sdsnew(b"SF270000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255b as GlyphId,
        sdsnew(b"SF280000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255e as GlyphId,
        sdsnew(b"SF360000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255f as GlyphId,
        sdsnew(b"SF370000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255a as GlyphId,
        sdsnew(b"SF380000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2554 as GlyphId,
        sdsnew(b"SF390000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2569 as GlyphId,
        sdsnew(b"SF400000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2566 as GlyphId,
        sdsnew(b"SF410000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2560 as GlyphId,
        sdsnew(b"SF420000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2550 as GlyphId,
        sdsnew(b"SF430000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x256c as GlyphId,
        sdsnew(b"SF440000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2567 as GlyphId,
        sdsnew(b"SF450000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2568 as GlyphId,
        sdsnew(b"SF460000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2564 as GlyphId,
        sdsnew(b"SF470000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2565 as GlyphId,
        sdsnew(b"SF480000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2559 as GlyphId,
        sdsnew(b"SF490000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2558 as GlyphId,
        sdsnew(b"SF500000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2552 as GlyphId,
        sdsnew(b"SF510000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2553 as GlyphId,
        sdsnew(b"SF520000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x256b as GlyphId,
        sdsnew(b"SF530000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x256a as GlyphId,
        sdsnew(b"SF540000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15a as GlyphId,
        sdsnew(b"Sacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x160 as GlyphId,
        sdsnew(b"Scaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15e as GlyphId,
        sdsnew(b"Scedilla\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15c as GlyphId,
        sdsnew(b"Scircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a3 as GlyphId,
        sdsnew(b"Sigma\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x54 as GlyphId,
        sdsnew(b"T\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a4 as GlyphId,
        sdsnew(b"Tau\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x166 as GlyphId,
        sdsnew(b"Tbar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x164 as GlyphId,
        sdsnew(b"Tcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x398 as GlyphId,
        sdsnew(b"Theta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xde as GlyphId,
        sdsnew(b"Thorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x55 as GlyphId,
        sdsnew(b"U\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xda as GlyphId,
        sdsnew(b"Uacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16c as GlyphId,
        sdsnew(b"Ubreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xdb as GlyphId,
        sdsnew(b"Ucircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xdc as GlyphId,
        sdsnew(b"Udieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd9 as GlyphId,
        sdsnew(b"Ugrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1af as GlyphId,
        sdsnew(b"Uhorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x170 as GlyphId,
        sdsnew(b"Uhungarumlaut\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16a as GlyphId,
        sdsnew(b"Umacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x172 as GlyphId,
        sdsnew(b"Uogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a5 as GlyphId,
        sdsnew(b"Upsilon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3d2 as GlyphId,
        sdsnew(b"Upsilon1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ab as GlyphId,
        sdsnew(b"Upsilondieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x38e as GlyphId,
        sdsnew(b"Upsilontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16e as GlyphId,
        sdsnew(b"Uring\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x168 as GlyphId,
        sdsnew(b"Utilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x56 as GlyphId,
        sdsnew(b"V\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x57 as GlyphId,
        sdsnew(b"W\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e82 as GlyphId,
        sdsnew(b"Wacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x174 as GlyphId,
        sdsnew(b"Wcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e84 as GlyphId,
        sdsnew(b"Wdieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e80 as GlyphId,
        sdsnew(b"Wgrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x58 as GlyphId,
        sdsnew(b"X\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39e as GlyphId,
        sdsnew(b"Xi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x59 as GlyphId,
        sdsnew(b"Y\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xdd as GlyphId,
        sdsnew(b"Yacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x176 as GlyphId,
        sdsnew(b"Ycircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x178 as GlyphId,
        sdsnew(b"Ydieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1ef2 as GlyphId,
        sdsnew(b"Ygrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5a as GlyphId,
        sdsnew(b"Z\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x179 as GlyphId,
        sdsnew(b"Zacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17d as GlyphId,
        sdsnew(b"Zcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17b as GlyphId,
        sdsnew(b"Zdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x396 as GlyphId,
        sdsnew(b"Zeta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x61 as GlyphId,
        sdsnew(b"a\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe1 as GlyphId,
        sdsnew(b"aacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x103 as GlyphId,
        sdsnew(b"abreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe2 as GlyphId,
        sdsnew(b"acircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb4 as GlyphId,
        sdsnew(b"acute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x301 as GlyphId,
        sdsnew(b"acutecomb\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe4 as GlyphId,
        sdsnew(b"adieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe6 as GlyphId,
        sdsnew(b"ae\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1fd as GlyphId,
        sdsnew(b"aeacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe0 as GlyphId,
        sdsnew(b"agrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2135 as GlyphId,
        sdsnew(b"aleph\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b1 as GlyphId,
        sdsnew(b"alpha\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ac as GlyphId,
        sdsnew(b"alphatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x101 as GlyphId,
        sdsnew(b"amacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x26 as GlyphId,
        sdsnew(b"ampersand\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2220 as GlyphId,
        sdsnew(b"angle\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2329 as GlyphId,
        sdsnew(b"angleleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x232a as GlyphId,
        sdsnew(b"angleright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x387 as GlyphId,
        sdsnew(b"anoteleia\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x105 as GlyphId,
        sdsnew(b"aogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2248 as GlyphId,
        sdsnew(b"approxequal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe5 as GlyphId,
        sdsnew(b"aring\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1fb as GlyphId,
        sdsnew(b"aringacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2194 as GlyphId,
        sdsnew(b"arrowboth\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21d4 as GlyphId,
        sdsnew(b"arrowdblboth\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21d3 as GlyphId,
        sdsnew(b"arrowdbldown\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21d0 as GlyphId,
        sdsnew(b"arrowdblleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21d2 as GlyphId,
        sdsnew(b"arrowdblright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21d1 as GlyphId,
        sdsnew(b"arrowdblup\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2193 as GlyphId,
        sdsnew(b"arrowdown\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2190 as GlyphId,
        sdsnew(b"arrowleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2192 as GlyphId,
        sdsnew(b"arrowright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2191 as GlyphId,
        sdsnew(b"arrowup\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2195 as GlyphId,
        sdsnew(b"arrowupdn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21a8 as GlyphId,
        sdsnew(b"arrowupdnbse\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5e as GlyphId,
        sdsnew(b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x7e as GlyphId,
        sdsnew(b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2a as GlyphId,
        sdsnew(b"asterisk\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2217 as GlyphId,
        sdsnew(b"asteriskmath\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x40 as GlyphId,
        sdsnew(b"at\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe3 as GlyphId,
        sdsnew(b"atilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x62 as GlyphId,
        sdsnew(b"b\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5c as GlyphId,
        sdsnew(b"backslash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x7c as GlyphId,
        sdsnew(b"bar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b2 as GlyphId,
        sdsnew(b"beta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2588 as GlyphId,
        sdsnew(b"block\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x7b as GlyphId,
        sdsnew(b"braceleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x7d as GlyphId,
        sdsnew(b"braceright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5b as GlyphId,
        sdsnew(b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5d as GlyphId,
        sdsnew(b"bracketright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2d8 as GlyphId,
        sdsnew(b"breve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa6 as GlyphId,
        sdsnew(b"brokenbar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2022 as GlyphId,
        sdsnew(b"bullet\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x63 as GlyphId,
        sdsnew(b"c\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x107 as GlyphId,
        sdsnew(b"cacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2c7 as GlyphId,
        sdsnew(b"caron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21b5 as GlyphId,
        sdsnew(b"carriagereturn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10d as GlyphId,
        sdsnew(b"ccaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe7 as GlyphId,
        sdsnew(b"ccedilla\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x109 as GlyphId,
        sdsnew(b"ccircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10b as GlyphId,
        sdsnew(b"cdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb8 as GlyphId,
        sdsnew(b"cedilla\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa2 as GlyphId,
        sdsnew(b"cent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c7 as GlyphId,
        sdsnew(b"chi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25cb as GlyphId,
        sdsnew(b"circle\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2297 as GlyphId,
        sdsnew(b"circlemultiply\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2295 as GlyphId,
        sdsnew(b"circleplus\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2c6 as GlyphId,
        sdsnew(b"circumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2663 as GlyphId,
        sdsnew(b"club\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a as GlyphId,
        sdsnew(b"colon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20a1 as GlyphId,
        sdsnew(b"colonmonetary\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2c as GlyphId,
        sdsnew(b"comma\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2245 as GlyphId,
        sdsnew(b"congruent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa9 as GlyphId,
        sdsnew(b"copyright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa4 as GlyphId,
        sdsnew(b"currency\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x64 as GlyphId,
        sdsnew(b"d\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2020 as GlyphId,
        sdsnew(b"dagger\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2021 as GlyphId,
        sdsnew(b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10f as GlyphId,
        sdsnew(b"dcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x111 as GlyphId,
        sdsnew(b"dcroat\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb0 as GlyphId,
        sdsnew(b"degree\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b4 as GlyphId,
        sdsnew(b"delta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2666 as GlyphId,
        sdsnew(b"diamond\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa8 as GlyphId,
        sdsnew(b"dieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x385 as GlyphId,
        sdsnew(b"dieresistonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf7 as GlyphId,
        sdsnew(b"divide\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2593 as GlyphId,
        sdsnew(b"dkshade\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2584 as GlyphId,
        sdsnew(b"dnblock\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x24 as GlyphId,
        sdsnew(b"dollar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20ab as GlyphId,
        sdsnew(b"dong\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2d9 as GlyphId,
        sdsnew(b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x323 as GlyphId,
        sdsnew(b"dotbelowcomb\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x131 as GlyphId,
        sdsnew(b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x22c5 as GlyphId,
        sdsnew(b"dotmath\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x65 as GlyphId,
        sdsnew(b"e\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe9 as GlyphId,
        sdsnew(b"eacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x115 as GlyphId,
        sdsnew(b"ebreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11b as GlyphId,
        sdsnew(b"ecaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xea as GlyphId,
        sdsnew(b"ecircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xeb as GlyphId,
        sdsnew(b"edieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x117 as GlyphId,
        sdsnew(b"edotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe8 as GlyphId,
        sdsnew(b"egrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x38 as GlyphId,
        sdsnew(b"eight\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2208 as GlyphId,
        sdsnew(b"element\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2026 as GlyphId,
        sdsnew(b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x113 as GlyphId,
        sdsnew(b"emacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2014 as GlyphId,
        sdsnew(b"emdash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2205 as GlyphId,
        sdsnew(b"emptyset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2013 as GlyphId,
        sdsnew(b"endash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14b as GlyphId,
        sdsnew(b"eng\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x119 as GlyphId,
        sdsnew(b"eogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b5 as GlyphId,
        sdsnew(b"epsilon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ad as GlyphId,
        sdsnew(b"epsilontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3d as GlyphId,
        sdsnew(b"equal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2261 as GlyphId,
        sdsnew(b"equivalence\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x212e as GlyphId,
        sdsnew(b"estimated\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b7 as GlyphId,
        sdsnew(b"eta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ae as GlyphId,
        sdsnew(b"etatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf0 as GlyphId,
        sdsnew(b"eth\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21 as GlyphId,
        sdsnew(b"exclam\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x203c as GlyphId,
        sdsnew(b"exclamdbl\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa1 as GlyphId,
        sdsnew(b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2203 as GlyphId,
        sdsnew(b"existential\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x66 as GlyphId,
        sdsnew(b"f\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2640 as GlyphId,
        sdsnew(b"female\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2012 as GlyphId,
        sdsnew(b"figuredash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25a0 as GlyphId,
        sdsnew(b"filledbox\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25ac as GlyphId,
        sdsnew(b"filledrect\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x35 as GlyphId,
        sdsnew(b"five\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x215d as GlyphId,
        sdsnew(b"fiveeighths\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x192 as GlyphId,
        sdsnew(b"florin\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x34 as GlyphId,
        sdsnew(b"four\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2044 as GlyphId,
        sdsnew(b"fraction\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20a3 as GlyphId,
        sdsnew(b"franc\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x67 as GlyphId,
        sdsnew(b"g\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b3 as GlyphId,
        sdsnew(b"gamma\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11f as GlyphId,
        sdsnew(b"gbreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e7 as GlyphId,
        sdsnew(b"gcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11d as GlyphId,
        sdsnew(b"gcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x121 as GlyphId,
        sdsnew(b"gdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xdf as GlyphId,
        sdsnew(b"germandbls\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2207 as GlyphId,
        sdsnew(b"gradient\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x60 as GlyphId,
        sdsnew(b"grave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x300 as GlyphId,
        sdsnew(b"gravecomb\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3e as GlyphId,
        sdsnew(b"greater\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2265 as GlyphId,
        sdsnew(b"greaterequal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xab as GlyphId,
        sdsnew(b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xbb as GlyphId,
        sdsnew(b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2039 as GlyphId,
        sdsnew(b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x203a as GlyphId,
        sdsnew(b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x68 as GlyphId,
        sdsnew(b"h\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x127 as GlyphId,
        sdsnew(b"hbar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x125 as GlyphId,
        sdsnew(b"hcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2665 as GlyphId,
        sdsnew(b"heart\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x309 as GlyphId,
        sdsnew(b"hookabovecomb\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2302 as GlyphId,
        sdsnew(b"house\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2dd as GlyphId,
        sdsnew(b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2d as GlyphId,
        sdsnew(b"hyphen\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x69 as GlyphId,
        sdsnew(b"i\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xed as GlyphId,
        sdsnew(b"iacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12d as GlyphId,
        sdsnew(b"ibreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xee as GlyphId,
        sdsnew(b"icircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xef as GlyphId,
        sdsnew(b"idieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xec as GlyphId,
        sdsnew(b"igrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x133 as GlyphId,
        sdsnew(b"ij\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12b as GlyphId,
        sdsnew(b"imacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x221e as GlyphId,
        sdsnew(b"infinity\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x222b as GlyphId,
        sdsnew(b"integral\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2321 as GlyphId,
        sdsnew(b"integralbt\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2320 as GlyphId,
        sdsnew(b"integraltp\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2229 as GlyphId,
        sdsnew(b"intersection\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25d8 as GlyphId,
        sdsnew(b"invbullet\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25d9 as GlyphId,
        sdsnew(b"invcircle\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x263b as GlyphId,
        sdsnew(b"invsmileface\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12f as GlyphId,
        sdsnew(b"iogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b9 as GlyphId,
        sdsnew(b"iota\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ca as GlyphId,
        sdsnew(b"iotadieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x390 as GlyphId,
        sdsnew(b"iotadieresistonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3af as GlyphId,
        sdsnew(b"iotatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x129 as GlyphId,
        sdsnew(b"itilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6a as GlyphId,
        sdsnew(b"j\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x135 as GlyphId,
        sdsnew(b"jcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6b as GlyphId,
        sdsnew(b"k\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ba as GlyphId,
        sdsnew(b"kappa\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x138 as GlyphId,
        sdsnew(b"kgreenlandic\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6c as GlyphId,
        sdsnew(b"l\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x13a as GlyphId,
        sdsnew(b"lacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3bb as GlyphId,
        sdsnew(b"lambda\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x13e as GlyphId,
        sdsnew(b"lcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x140 as GlyphId,
        sdsnew(b"ldot\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c as GlyphId,
        sdsnew(b"less\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2264 as GlyphId,
        sdsnew(b"lessequal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x258c as GlyphId,
        sdsnew(b"lfblock\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20a4 as GlyphId,
        sdsnew(b"lira\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2227 as GlyphId,
        sdsnew(b"logicaland\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xac as GlyphId,
        sdsnew(b"logicalnot\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2228 as GlyphId,
        sdsnew(b"logicalor\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17f as GlyphId,
        sdsnew(b"longs\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25ca as GlyphId,
        sdsnew(b"lozenge\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x142 as GlyphId,
        sdsnew(b"lslash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2591 as GlyphId,
        sdsnew(b"ltshade\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6d as GlyphId,
        sdsnew(b"m\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xaf as GlyphId,
        sdsnew(b"macron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2642 as GlyphId,
        sdsnew(b"male\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2212 as GlyphId,
        sdsnew(b"minus\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2032 as GlyphId,
        sdsnew(b"minute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb5 as GlyphId,
        sdsnew(b"mu\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd7 as GlyphId,
        sdsnew(b"multiply\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x266a as GlyphId,
        sdsnew(b"musicalnote\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x266b as GlyphId,
        sdsnew(b"musicalnotedbl\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6e as GlyphId,
        sdsnew(b"n\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x144 as GlyphId,
        sdsnew(b"nacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x149 as GlyphId,
        sdsnew(b"napostrophe\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x148 as GlyphId,
        sdsnew(b"ncaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39 as GlyphId,
        sdsnew(b"nine\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2209 as GlyphId,
        sdsnew(b"notelement\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2260 as GlyphId,
        sdsnew(b"notequal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2284 as GlyphId,
        sdsnew(b"notsubset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf1 as GlyphId,
        sdsnew(b"ntilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3bd as GlyphId,
        sdsnew(b"nu\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x23 as GlyphId,
        sdsnew(b"numbersign\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6f as GlyphId,
        sdsnew(b"o\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf3 as GlyphId,
        sdsnew(b"oacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14f as GlyphId,
        sdsnew(b"obreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf4 as GlyphId,
        sdsnew(b"ocircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf6 as GlyphId,
        sdsnew(b"odieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x153 as GlyphId,
        sdsnew(b"oe\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2db as GlyphId,
        sdsnew(b"ogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf2 as GlyphId,
        sdsnew(b"ograve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1a1 as GlyphId,
        sdsnew(b"ohorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x151 as GlyphId,
        sdsnew(b"ohungarumlaut\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14d as GlyphId,
        sdsnew(b"omacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c9 as GlyphId,
        sdsnew(b"omega\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3d6 as GlyphId,
        sdsnew(b"omega1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ce as GlyphId,
        sdsnew(b"omegatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3bf as GlyphId,
        sdsnew(b"omicron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3cc as GlyphId,
        sdsnew(b"omicrontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x31 as GlyphId,
        sdsnew(b"one\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2024 as GlyphId,
        sdsnew(b"onedotenleader\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x215b as GlyphId,
        sdsnew(b"oneeighth\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xbd as GlyphId,
        sdsnew(b"onehalf\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xbc as GlyphId,
        sdsnew(b"onequarter\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2153 as GlyphId,
        sdsnew(b"onethird\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25e6 as GlyphId,
        sdsnew(b"openbullet\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xaa as GlyphId,
        sdsnew(b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xba as GlyphId,
        sdsnew(b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x221f as GlyphId,
        sdsnew(b"orthogonal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf8 as GlyphId,
        sdsnew(b"oslash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1ff as GlyphId,
        sdsnew(b"oslashacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf5 as GlyphId,
        sdsnew(b"otilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x70 as GlyphId,
        sdsnew(b"p\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb6 as GlyphId,
        sdsnew(b"paragraph\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x28 as GlyphId,
        sdsnew(b"parenleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x29 as GlyphId,
        sdsnew(b"parenright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2202 as GlyphId,
        sdsnew(b"partialdiff\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25 as GlyphId,
        sdsnew(b"percent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2e as GlyphId,
        sdsnew(b"period\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb7 as GlyphId,
        sdsnew(b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x22a5 as GlyphId,
        sdsnew(b"perpendicular\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2030 as GlyphId,
        sdsnew(b"perthousand\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20a7 as GlyphId,
        sdsnew(b"peseta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c6 as GlyphId,
        sdsnew(b"phi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3d5 as GlyphId,
        sdsnew(b"phi1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c0 as GlyphId,
        sdsnew(b"pi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2b as GlyphId,
        sdsnew(b"plus\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb1 as GlyphId,
        sdsnew(b"plusminus\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x211e as GlyphId,
        sdsnew(b"prescription\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x220f as GlyphId,
        sdsnew(b"product\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2282 as GlyphId,
        sdsnew(b"propersubset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2283 as GlyphId,
        sdsnew(b"propersuperset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x221d as GlyphId,
        sdsnew(b"proportional\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c8 as GlyphId,
        sdsnew(b"psi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x71 as GlyphId,
        sdsnew(b"q\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3f as GlyphId,
        sdsnew(b"question\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xbf as GlyphId,
        sdsnew(b"questiondown\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x22 as GlyphId,
        sdsnew(b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x201e as GlyphId,
        sdsnew(b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x201c as GlyphId,
        sdsnew(b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x201d as GlyphId,
        sdsnew(b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2018 as GlyphId,
        sdsnew(b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x201b as GlyphId,
        sdsnew(b"quotereversed\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2019 as GlyphId,
        sdsnew(b"quoteright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x201a as GlyphId,
        sdsnew(b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x27 as GlyphId,
        sdsnew(b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x72 as GlyphId,
        sdsnew(b"r\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x155 as GlyphId,
        sdsnew(b"racute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x221a as GlyphId,
        sdsnew(b"radical\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x159 as GlyphId,
        sdsnew(b"rcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2286 as GlyphId,
        sdsnew(b"reflexsubset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2287 as GlyphId,
        sdsnew(b"reflexsuperset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xae as GlyphId,
        sdsnew(b"registered\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2310 as GlyphId,
        sdsnew(b"revlogicalnot\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c1 as GlyphId,
        sdsnew(b"rho\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2da as GlyphId,
        sdsnew(b"ring\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2590 as GlyphId,
        sdsnew(b"rtblock\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x73 as GlyphId,
        sdsnew(b"s\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15b as GlyphId,
        sdsnew(b"sacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x161 as GlyphId,
        sdsnew(b"scaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15f as GlyphId,
        sdsnew(b"scedilla\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15d as GlyphId,
        sdsnew(b"scircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2033 as GlyphId,
        sdsnew(b"second\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa7 as GlyphId,
        sdsnew(b"section\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b as GlyphId,
        sdsnew(b"semicolon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x37 as GlyphId,
        sdsnew(b"seven\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x215e as GlyphId,
        sdsnew(b"seveneighths\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2592 as GlyphId,
        sdsnew(b"shade\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c3 as GlyphId,
        sdsnew(b"sigma\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c2 as GlyphId,
        sdsnew(b"sigma1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x223c as GlyphId,
        sdsnew(b"similar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x36 as GlyphId,
        sdsnew(b"six\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2f as GlyphId,
        sdsnew(b"slash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x263a as GlyphId,
        sdsnew(b"smileface\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20 as GlyphId,
        sdsnew(b"space\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2660 as GlyphId,
        sdsnew(b"spade\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa3 as GlyphId,
        sdsnew(b"sterling\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x220b as GlyphId,
        sdsnew(b"suchthat\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2211 as GlyphId,
        sdsnew(b"summation\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x263c as GlyphId,
        sdsnew(b"sun\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x74 as GlyphId,
        sdsnew(b"t\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c4 as GlyphId,
        sdsnew(b"tau\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x167 as GlyphId,
        sdsnew(b"tbar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x165 as GlyphId,
        sdsnew(b"tcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2234 as GlyphId,
        sdsnew(b"therefore\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b8 as GlyphId,
        sdsnew(b"theta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3d1 as GlyphId,
        sdsnew(b"theta1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xfe as GlyphId,
        sdsnew(b"thorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x33 as GlyphId,
        sdsnew(b"three\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x215c as GlyphId,
        sdsnew(b"threeeighths\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xbe as GlyphId,
        sdsnew(b"threequarters\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2dc as GlyphId,
        sdsnew(b"tilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x303 as GlyphId,
        sdsnew(b"tildecomb\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x384 as GlyphId,
        sdsnew(b"tonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2122 as GlyphId,
        sdsnew(b"trademark\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25bc as GlyphId,
        sdsnew(b"triagdn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25c4 as GlyphId,
        sdsnew(b"triaglf\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25ba as GlyphId,
        sdsnew(b"triagrt\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25b2 as GlyphId,
        sdsnew(b"triagup\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x32 as GlyphId,
        sdsnew(b"two\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2025 as GlyphId,
        sdsnew(b"twodotenleader\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2154 as GlyphId,
        sdsnew(b"twothirds\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x75 as GlyphId,
        sdsnew(b"u\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xfa as GlyphId,
        sdsnew(b"uacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16d as GlyphId,
        sdsnew(b"ubreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xfb as GlyphId,
        sdsnew(b"ucircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xfc as GlyphId,
        sdsnew(b"udieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf9 as GlyphId,
        sdsnew(b"ugrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1b0 as GlyphId,
        sdsnew(b"uhorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x171 as GlyphId,
        sdsnew(b"uhungarumlaut\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16b as GlyphId,
        sdsnew(b"umacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5f as GlyphId,
        sdsnew(b"underscore\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2017 as GlyphId,
        sdsnew(b"underscoredbl\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x222a as GlyphId,
        sdsnew(b"union\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2200 as GlyphId,
        sdsnew(b"universal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x173 as GlyphId,
        sdsnew(b"uogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2580 as GlyphId,
        sdsnew(b"upblock\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c5 as GlyphId,
        sdsnew(b"upsilon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3cb as GlyphId,
        sdsnew(b"upsilondieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b0 as GlyphId,
        sdsnew(b"upsilondieresistonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3cd as GlyphId,
        sdsnew(b"upsilontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16f as GlyphId,
        sdsnew(b"uring\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x169 as GlyphId,
        sdsnew(b"utilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x76 as GlyphId,
        sdsnew(b"v\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x77 as GlyphId,
        sdsnew(b"w\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e83 as GlyphId,
        sdsnew(b"wacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x175 as GlyphId,
        sdsnew(b"wcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e85 as GlyphId,
        sdsnew(b"wdieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2118 as GlyphId,
        sdsnew(b"weierstrass\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e81 as GlyphId,
        sdsnew(b"wgrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x78 as GlyphId,
        sdsnew(b"x\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3be as GlyphId,
        sdsnew(b"xi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x79 as GlyphId,
        sdsnew(b"y\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xfd as GlyphId,
        sdsnew(b"yacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x177 as GlyphId,
        sdsnew(b"ycircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xff as GlyphId,
        sdsnew(b"ydieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa5 as GlyphId,
        sdsnew(b"yen\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1ef3 as GlyphId,
        sdsnew(b"ygrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x7a as GlyphId,
        sdsnew(b"z\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17a as GlyphId,
        sdsnew(b"zacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17e as GlyphId,
        sdsnew(b"zcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17c as GlyphId,
        sdsnew(b"zdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x30 as GlyphId,
        sdsnew(b"zero\0" as *const u8 as *const ::core::ffi::c_char),
    );
    OTFCC_PKG_GLYPH_ORDER
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b6 as GlyphId,
        sdsnew(b"zeta\0" as *const u8 as *const ::core::ffi::c_char),
    );
}
