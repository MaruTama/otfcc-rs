extern "C" {
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
}
use crate::support::handle::{otfcc_GlyphHandle};
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: ::core::ffi::c_uint,
    pub expand_mult: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut ::core::ffi::c_void,
    pub next: *mut ::core::ffi::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *mut ::core::ffi::c_void,
    pub keylen: ::core::ffi::c_uint,
    pub hashv: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: ::core::ffi::c_uint,
    pub log2_num_buckets: ::core::ffi::c_uint,
    pub num_items: ::core::ffi::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: isize,
    pub ideal_chain_maxlen: ::core::ffi::c_uint,
    pub nonideal_items: ::core::ffi::c_uint,
    pub ineff_expands: ::core::ffi::c_uint,
    pub noexpand: ::core::ffi::c_uint,
    pub signature: u32,
}
pub type glyphid_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_GlyphOrderEntry {
    pub gid: glyphid_t,
    pub name: sds,
    pub orderType: u8,
    pub orderEntry: u32,
    pub hhID: UT_hash_handle,
    pub hhName: UT_hash_handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_GlyphOrder {
    pub byGID: *mut otfcc_GlyphOrderEntry,
    pub byName: *mut otfcc_GlyphOrderEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_GlyphOrderPackage {
    pub init: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, *const otfcc_GlyphOrder) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, *mut otfcc_GlyphOrder) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, otfcc_GlyphOrder) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, otfcc_GlyphOrder) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut otfcc_GlyphOrder>,
    pub free: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder) -> ()>,
    pub setByGID: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, glyphid_t, sds) -> sds>,
    pub setByName: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, sds, glyphid_t) -> bool>,
    pub nameAField_Shared:
        Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, glyphid_t, *mut sds) -> bool>,
    pub consolidateHandle:
        Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, *mut otfcc_GlyphHandle) -> bool>,
    pub lookupName: Option<unsafe extern "C" fn(*mut otfcc_GlyphOrder, sds) -> bool>,
}
#[no_mangle]
pub unsafe extern "C" fn aglfn_setupNames(mut map: *mut otfcc_GlyphOrder) {
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x41 as glyphid_t,
        sdsnew(b"A\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc6 as glyphid_t,
        sdsnew(b"AE\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1fc as glyphid_t,
        sdsnew(b"AEacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc1 as glyphid_t,
        sdsnew(b"Aacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x102 as glyphid_t,
        sdsnew(b"Abreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc2 as glyphid_t,
        sdsnew(b"Acircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc4 as glyphid_t,
        sdsnew(b"Adieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc0 as glyphid_t,
        sdsnew(b"Agrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x391 as glyphid_t,
        sdsnew(b"Alpha\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x386 as glyphid_t,
        sdsnew(b"Alphatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x100 as glyphid_t,
        sdsnew(b"Amacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x104 as glyphid_t,
        sdsnew(b"Aogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc5 as glyphid_t,
        sdsnew(b"Aring\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1fa as glyphid_t,
        sdsnew(b"Aringacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc3 as glyphid_t,
        sdsnew(b"Atilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x42 as glyphid_t,
        sdsnew(b"B\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x392 as glyphid_t,
        sdsnew(b"Beta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x43 as glyphid_t,
        sdsnew(b"C\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x106 as glyphid_t,
        sdsnew(b"Cacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10c as glyphid_t,
        sdsnew(b"Ccaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc7 as glyphid_t,
        sdsnew(b"Ccedilla\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x108 as glyphid_t,
        sdsnew(b"Ccircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10a as glyphid_t,
        sdsnew(b"Cdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a7 as glyphid_t,
        sdsnew(b"Chi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x44 as glyphid_t,
        sdsnew(b"D\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10e as glyphid_t,
        sdsnew(b"Dcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x110 as glyphid_t,
        sdsnew(b"Dcroat\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2206 as glyphid_t,
        sdsnew(b"Delta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x45 as glyphid_t,
        sdsnew(b"E\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc9 as glyphid_t,
        sdsnew(b"Eacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x114 as glyphid_t,
        sdsnew(b"Ebreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11a as glyphid_t,
        sdsnew(b"Ecaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xca as glyphid_t,
        sdsnew(b"Ecircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xcb as glyphid_t,
        sdsnew(b"Edieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x116 as glyphid_t,
        sdsnew(b"Edotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xc8 as glyphid_t,
        sdsnew(b"Egrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x112 as glyphid_t,
        sdsnew(b"Emacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14a as glyphid_t,
        sdsnew(b"Eng\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x118 as glyphid_t,
        sdsnew(b"Eogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x395 as glyphid_t,
        sdsnew(b"Epsilon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x388 as glyphid_t,
        sdsnew(b"Epsilontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x397 as glyphid_t,
        sdsnew(b"Eta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x389 as glyphid_t,
        sdsnew(b"Etatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd0 as glyphid_t,
        sdsnew(b"Eth\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20ac as glyphid_t,
        sdsnew(b"Euro\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x46 as glyphid_t,
        sdsnew(b"F\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x47 as glyphid_t,
        sdsnew(b"G\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x393 as glyphid_t,
        sdsnew(b"Gamma\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11e as glyphid_t,
        sdsnew(b"Gbreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e6 as glyphid_t,
        sdsnew(b"Gcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11c as glyphid_t,
        sdsnew(b"Gcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x120 as glyphid_t,
        sdsnew(b"Gdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x48 as glyphid_t,
        sdsnew(b"H\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25cf as glyphid_t,
        sdsnew(b"H18533\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25aa as glyphid_t,
        sdsnew(b"H18543\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25ab as glyphid_t,
        sdsnew(b"H18551\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25a1 as glyphid_t,
        sdsnew(b"H22073\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x126 as glyphid_t,
        sdsnew(b"Hbar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x124 as glyphid_t,
        sdsnew(b"Hcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x49 as glyphid_t,
        sdsnew(b"I\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x132 as glyphid_t,
        sdsnew(b"IJ\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xcd as glyphid_t,
        sdsnew(b"Iacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12c as glyphid_t,
        sdsnew(b"Ibreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xce as glyphid_t,
        sdsnew(b"Icircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xcf as glyphid_t,
        sdsnew(b"Idieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x130 as glyphid_t,
        sdsnew(b"Idotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2111 as glyphid_t,
        sdsnew(b"Ifraktur\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xcc as glyphid_t,
        sdsnew(b"Igrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12a as glyphid_t,
        sdsnew(b"Imacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12e as glyphid_t,
        sdsnew(b"Iogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x399 as glyphid_t,
        sdsnew(b"Iota\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3aa as glyphid_t,
        sdsnew(b"Iotadieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x38a as glyphid_t,
        sdsnew(b"Iotatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x128 as glyphid_t,
        sdsnew(b"Itilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4a as glyphid_t,
        sdsnew(b"J\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x134 as glyphid_t,
        sdsnew(b"Jcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4b as glyphid_t,
        sdsnew(b"K\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39a as glyphid_t,
        sdsnew(b"Kappa\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4c as glyphid_t,
        sdsnew(b"L\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x139 as glyphid_t,
        sdsnew(b"Lacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39b as glyphid_t,
        sdsnew(b"Lambda\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x13d as glyphid_t,
        sdsnew(b"Lcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x13f as glyphid_t,
        sdsnew(b"Ldot\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x141 as glyphid_t,
        sdsnew(b"Lslash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4d as glyphid_t,
        sdsnew(b"M\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39c as glyphid_t,
        sdsnew(b"Mu\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4e as glyphid_t,
        sdsnew(b"N\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x143 as glyphid_t,
        sdsnew(b"Nacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x147 as glyphid_t,
        sdsnew(b"Ncaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd1 as glyphid_t,
        sdsnew(b"Ntilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39d as glyphid_t,
        sdsnew(b"Nu\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x4f as glyphid_t,
        sdsnew(b"O\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x152 as glyphid_t,
        sdsnew(b"OE\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd3 as glyphid_t,
        sdsnew(b"Oacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14e as glyphid_t,
        sdsnew(b"Obreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd4 as glyphid_t,
        sdsnew(b"Ocircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd6 as glyphid_t,
        sdsnew(b"Odieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd2 as glyphid_t,
        sdsnew(b"Ograve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1a0 as glyphid_t,
        sdsnew(b"Ohorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x150 as glyphid_t,
        sdsnew(b"Ohungarumlaut\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14c as glyphid_t,
        sdsnew(b"Omacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2126 as glyphid_t,
        sdsnew(b"Omega\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x38f as glyphid_t,
        sdsnew(b"Omegatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39f as glyphid_t,
        sdsnew(b"Omicron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x38c as glyphid_t,
        sdsnew(b"Omicrontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd8 as glyphid_t,
        sdsnew(b"Oslash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1fe as glyphid_t,
        sdsnew(b"Oslashacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd5 as glyphid_t,
        sdsnew(b"Otilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x50 as glyphid_t,
        sdsnew(b"P\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a6 as glyphid_t,
        sdsnew(b"Phi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a0 as glyphid_t,
        sdsnew(b"Pi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a8 as glyphid_t,
        sdsnew(b"Psi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x51 as glyphid_t,
        sdsnew(b"Q\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x52 as glyphid_t,
        sdsnew(b"R\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x154 as glyphid_t,
        sdsnew(b"Racute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x158 as glyphid_t,
        sdsnew(b"Rcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x211c as glyphid_t,
        sdsnew(b"Rfraktur\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a1 as glyphid_t,
        sdsnew(b"Rho\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x53 as glyphid_t,
        sdsnew(b"S\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x250c as glyphid_t,
        sdsnew(b"SF010000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2514 as glyphid_t,
        sdsnew(b"SF020000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2510 as glyphid_t,
        sdsnew(b"SF030000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2518 as glyphid_t,
        sdsnew(b"SF040000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x253c as glyphid_t,
        sdsnew(b"SF050000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x252c as glyphid_t,
        sdsnew(b"SF060000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2534 as glyphid_t,
        sdsnew(b"SF070000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x251c as glyphid_t,
        sdsnew(b"SF080000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2524 as glyphid_t,
        sdsnew(b"SF090000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2500 as glyphid_t,
        sdsnew(b"SF100000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2502 as glyphid_t,
        sdsnew(b"SF110000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2561 as glyphid_t,
        sdsnew(b"SF190000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2562 as glyphid_t,
        sdsnew(b"SF200000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2556 as glyphid_t,
        sdsnew(b"SF210000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2555 as glyphid_t,
        sdsnew(b"SF220000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2563 as glyphid_t,
        sdsnew(b"SF230000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2551 as glyphid_t,
        sdsnew(b"SF240000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2557 as glyphid_t,
        sdsnew(b"SF250000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255d as glyphid_t,
        sdsnew(b"SF260000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255c as glyphid_t,
        sdsnew(b"SF270000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255b as glyphid_t,
        sdsnew(b"SF280000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255e as glyphid_t,
        sdsnew(b"SF360000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255f as glyphid_t,
        sdsnew(b"SF370000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x255a as glyphid_t,
        sdsnew(b"SF380000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2554 as glyphid_t,
        sdsnew(b"SF390000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2569 as glyphid_t,
        sdsnew(b"SF400000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2566 as glyphid_t,
        sdsnew(b"SF410000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2560 as glyphid_t,
        sdsnew(b"SF420000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2550 as glyphid_t,
        sdsnew(b"SF430000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x256c as glyphid_t,
        sdsnew(b"SF440000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2567 as glyphid_t,
        sdsnew(b"SF450000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2568 as glyphid_t,
        sdsnew(b"SF460000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2564 as glyphid_t,
        sdsnew(b"SF470000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2565 as glyphid_t,
        sdsnew(b"SF480000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2559 as glyphid_t,
        sdsnew(b"SF490000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2558 as glyphid_t,
        sdsnew(b"SF500000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2552 as glyphid_t,
        sdsnew(b"SF510000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2553 as glyphid_t,
        sdsnew(b"SF520000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x256b as glyphid_t,
        sdsnew(b"SF530000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x256a as glyphid_t,
        sdsnew(b"SF540000\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15a as glyphid_t,
        sdsnew(b"Sacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x160 as glyphid_t,
        sdsnew(b"Scaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15e as glyphid_t,
        sdsnew(b"Scedilla\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15c as glyphid_t,
        sdsnew(b"Scircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a3 as glyphid_t,
        sdsnew(b"Sigma\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x54 as glyphid_t,
        sdsnew(b"T\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a4 as glyphid_t,
        sdsnew(b"Tau\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x166 as glyphid_t,
        sdsnew(b"Tbar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x164 as glyphid_t,
        sdsnew(b"Tcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x398 as glyphid_t,
        sdsnew(b"Theta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xde as glyphid_t,
        sdsnew(b"Thorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x55 as glyphid_t,
        sdsnew(b"U\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xda as glyphid_t,
        sdsnew(b"Uacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16c as glyphid_t,
        sdsnew(b"Ubreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xdb as glyphid_t,
        sdsnew(b"Ucircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xdc as glyphid_t,
        sdsnew(b"Udieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd9 as glyphid_t,
        sdsnew(b"Ugrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1af as glyphid_t,
        sdsnew(b"Uhorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x170 as glyphid_t,
        sdsnew(b"Uhungarumlaut\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16a as glyphid_t,
        sdsnew(b"Umacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x172 as glyphid_t,
        sdsnew(b"Uogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a5 as glyphid_t,
        sdsnew(b"Upsilon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3d2 as glyphid_t,
        sdsnew(b"Upsilon1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ab as glyphid_t,
        sdsnew(b"Upsilondieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x38e as glyphid_t,
        sdsnew(b"Upsilontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16e as glyphid_t,
        sdsnew(b"Uring\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x168 as glyphid_t,
        sdsnew(b"Utilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x56 as glyphid_t,
        sdsnew(b"V\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x57 as glyphid_t,
        sdsnew(b"W\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e82 as glyphid_t,
        sdsnew(b"Wacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x174 as glyphid_t,
        sdsnew(b"Wcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e84 as glyphid_t,
        sdsnew(b"Wdieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e80 as glyphid_t,
        sdsnew(b"Wgrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x58 as glyphid_t,
        sdsnew(b"X\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39e as glyphid_t,
        sdsnew(b"Xi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x59 as glyphid_t,
        sdsnew(b"Y\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xdd as glyphid_t,
        sdsnew(b"Yacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x176 as glyphid_t,
        sdsnew(b"Ycircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x178 as glyphid_t,
        sdsnew(b"Ydieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1ef2 as glyphid_t,
        sdsnew(b"Ygrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5a as glyphid_t,
        sdsnew(b"Z\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x179 as glyphid_t,
        sdsnew(b"Zacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17d as glyphid_t,
        sdsnew(b"Zcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17b as glyphid_t,
        sdsnew(b"Zdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x396 as glyphid_t,
        sdsnew(b"Zeta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x61 as glyphid_t,
        sdsnew(b"a\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe1 as glyphid_t,
        sdsnew(b"aacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x103 as glyphid_t,
        sdsnew(b"abreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe2 as glyphid_t,
        sdsnew(b"acircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb4 as glyphid_t,
        sdsnew(b"acute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x301 as glyphid_t,
        sdsnew(b"acutecomb\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe4 as glyphid_t,
        sdsnew(b"adieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe6 as glyphid_t,
        sdsnew(b"ae\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1fd as glyphid_t,
        sdsnew(b"aeacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe0 as glyphid_t,
        sdsnew(b"agrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2135 as glyphid_t,
        sdsnew(b"aleph\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b1 as glyphid_t,
        sdsnew(b"alpha\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ac as glyphid_t,
        sdsnew(b"alphatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x101 as glyphid_t,
        sdsnew(b"amacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x26 as glyphid_t,
        sdsnew(b"ampersand\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2220 as glyphid_t,
        sdsnew(b"angle\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2329 as glyphid_t,
        sdsnew(b"angleleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x232a as glyphid_t,
        sdsnew(b"angleright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x387 as glyphid_t,
        sdsnew(b"anoteleia\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x105 as glyphid_t,
        sdsnew(b"aogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2248 as glyphid_t,
        sdsnew(b"approxequal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe5 as glyphid_t,
        sdsnew(b"aring\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1fb as glyphid_t,
        sdsnew(b"aringacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2194 as glyphid_t,
        sdsnew(b"arrowboth\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21d4 as glyphid_t,
        sdsnew(b"arrowdblboth\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21d3 as glyphid_t,
        sdsnew(b"arrowdbldown\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21d0 as glyphid_t,
        sdsnew(b"arrowdblleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21d2 as glyphid_t,
        sdsnew(b"arrowdblright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21d1 as glyphid_t,
        sdsnew(b"arrowdblup\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2193 as glyphid_t,
        sdsnew(b"arrowdown\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2190 as glyphid_t,
        sdsnew(b"arrowleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2192 as glyphid_t,
        sdsnew(b"arrowright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2191 as glyphid_t,
        sdsnew(b"arrowup\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2195 as glyphid_t,
        sdsnew(b"arrowupdn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21a8 as glyphid_t,
        sdsnew(b"arrowupdnbse\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5e as glyphid_t,
        sdsnew(b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x7e as glyphid_t,
        sdsnew(b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2a as glyphid_t,
        sdsnew(b"asterisk\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2217 as glyphid_t,
        sdsnew(b"asteriskmath\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x40 as glyphid_t,
        sdsnew(b"at\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe3 as glyphid_t,
        sdsnew(b"atilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x62 as glyphid_t,
        sdsnew(b"b\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5c as glyphid_t,
        sdsnew(b"backslash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x7c as glyphid_t,
        sdsnew(b"bar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b2 as glyphid_t,
        sdsnew(b"beta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2588 as glyphid_t,
        sdsnew(b"block\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x7b as glyphid_t,
        sdsnew(b"braceleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x7d as glyphid_t,
        sdsnew(b"braceright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5b as glyphid_t,
        sdsnew(b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5d as glyphid_t,
        sdsnew(b"bracketright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2d8 as glyphid_t,
        sdsnew(b"breve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa6 as glyphid_t,
        sdsnew(b"brokenbar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2022 as glyphid_t,
        sdsnew(b"bullet\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x63 as glyphid_t,
        sdsnew(b"c\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x107 as glyphid_t,
        sdsnew(b"cacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2c7 as glyphid_t,
        sdsnew(b"caron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21b5 as glyphid_t,
        sdsnew(b"carriagereturn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10d as glyphid_t,
        sdsnew(b"ccaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe7 as glyphid_t,
        sdsnew(b"ccedilla\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x109 as glyphid_t,
        sdsnew(b"ccircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10b as glyphid_t,
        sdsnew(b"cdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb8 as glyphid_t,
        sdsnew(b"cedilla\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa2 as glyphid_t,
        sdsnew(b"cent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c7 as glyphid_t,
        sdsnew(b"chi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25cb as glyphid_t,
        sdsnew(b"circle\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2297 as glyphid_t,
        sdsnew(b"circlemultiply\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2295 as glyphid_t,
        sdsnew(b"circleplus\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2c6 as glyphid_t,
        sdsnew(b"circumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2663 as glyphid_t,
        sdsnew(b"club\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3a as glyphid_t,
        sdsnew(b"colon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20a1 as glyphid_t,
        sdsnew(b"colonmonetary\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2c as glyphid_t,
        sdsnew(b"comma\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2245 as glyphid_t,
        sdsnew(b"congruent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa9 as glyphid_t,
        sdsnew(b"copyright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa4 as glyphid_t,
        sdsnew(b"currency\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x64 as glyphid_t,
        sdsnew(b"d\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2020 as glyphid_t,
        sdsnew(b"dagger\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2021 as glyphid_t,
        sdsnew(b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x10f as glyphid_t,
        sdsnew(b"dcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x111 as glyphid_t,
        sdsnew(b"dcroat\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb0 as glyphid_t,
        sdsnew(b"degree\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b4 as glyphid_t,
        sdsnew(b"delta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2666 as glyphid_t,
        sdsnew(b"diamond\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa8 as glyphid_t,
        sdsnew(b"dieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x385 as glyphid_t,
        sdsnew(b"dieresistonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf7 as glyphid_t,
        sdsnew(b"divide\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2593 as glyphid_t,
        sdsnew(b"dkshade\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2584 as glyphid_t,
        sdsnew(b"dnblock\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x24 as glyphid_t,
        sdsnew(b"dollar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20ab as glyphid_t,
        sdsnew(b"dong\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2d9 as glyphid_t,
        sdsnew(b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x323 as glyphid_t,
        sdsnew(b"dotbelowcomb\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x131 as glyphid_t,
        sdsnew(b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x22c5 as glyphid_t,
        sdsnew(b"dotmath\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x65 as glyphid_t,
        sdsnew(b"e\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe9 as glyphid_t,
        sdsnew(b"eacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x115 as glyphid_t,
        sdsnew(b"ebreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11b as glyphid_t,
        sdsnew(b"ecaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xea as glyphid_t,
        sdsnew(b"ecircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xeb as glyphid_t,
        sdsnew(b"edieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x117 as glyphid_t,
        sdsnew(b"edotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xe8 as glyphid_t,
        sdsnew(b"egrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x38 as glyphid_t,
        sdsnew(b"eight\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2208 as glyphid_t,
        sdsnew(b"element\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2026 as glyphid_t,
        sdsnew(b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x113 as glyphid_t,
        sdsnew(b"emacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2014 as glyphid_t,
        sdsnew(b"emdash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2205 as glyphid_t,
        sdsnew(b"emptyset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2013 as glyphid_t,
        sdsnew(b"endash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14b as glyphid_t,
        sdsnew(b"eng\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x119 as glyphid_t,
        sdsnew(b"eogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b5 as glyphid_t,
        sdsnew(b"epsilon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ad as glyphid_t,
        sdsnew(b"epsilontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3d as glyphid_t,
        sdsnew(b"equal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2261 as glyphid_t,
        sdsnew(b"equivalence\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x212e as glyphid_t,
        sdsnew(b"estimated\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b7 as glyphid_t,
        sdsnew(b"eta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ae as glyphid_t,
        sdsnew(b"etatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf0 as glyphid_t,
        sdsnew(b"eth\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x21 as glyphid_t,
        sdsnew(b"exclam\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x203c as glyphid_t,
        sdsnew(b"exclamdbl\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa1 as glyphid_t,
        sdsnew(b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2203 as glyphid_t,
        sdsnew(b"existential\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x66 as glyphid_t,
        sdsnew(b"f\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2640 as glyphid_t,
        sdsnew(b"female\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2012 as glyphid_t,
        sdsnew(b"figuredash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25a0 as glyphid_t,
        sdsnew(b"filledbox\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25ac as glyphid_t,
        sdsnew(b"filledrect\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x35 as glyphid_t,
        sdsnew(b"five\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x215d as glyphid_t,
        sdsnew(b"fiveeighths\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x192 as glyphid_t,
        sdsnew(b"florin\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x34 as glyphid_t,
        sdsnew(b"four\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2044 as glyphid_t,
        sdsnew(b"fraction\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20a3 as glyphid_t,
        sdsnew(b"franc\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x67 as glyphid_t,
        sdsnew(b"g\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b3 as glyphid_t,
        sdsnew(b"gamma\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11f as glyphid_t,
        sdsnew(b"gbreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e7 as glyphid_t,
        sdsnew(b"gcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x11d as glyphid_t,
        sdsnew(b"gcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x121 as glyphid_t,
        sdsnew(b"gdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xdf as glyphid_t,
        sdsnew(b"germandbls\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2207 as glyphid_t,
        sdsnew(b"gradient\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x60 as glyphid_t,
        sdsnew(b"grave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x300 as glyphid_t,
        sdsnew(b"gravecomb\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3e as glyphid_t,
        sdsnew(b"greater\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2265 as glyphid_t,
        sdsnew(b"greaterequal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xab as glyphid_t,
        sdsnew(b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xbb as glyphid_t,
        sdsnew(b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2039 as glyphid_t,
        sdsnew(b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x203a as glyphid_t,
        sdsnew(b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x68 as glyphid_t,
        sdsnew(b"h\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x127 as glyphid_t,
        sdsnew(b"hbar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x125 as glyphid_t,
        sdsnew(b"hcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2665 as glyphid_t,
        sdsnew(b"heart\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x309 as glyphid_t,
        sdsnew(b"hookabovecomb\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2302 as glyphid_t,
        sdsnew(b"house\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2dd as glyphid_t,
        sdsnew(b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2d as glyphid_t,
        sdsnew(b"hyphen\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x69 as glyphid_t,
        sdsnew(b"i\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xed as glyphid_t,
        sdsnew(b"iacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12d as glyphid_t,
        sdsnew(b"ibreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xee as glyphid_t,
        sdsnew(b"icircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xef as glyphid_t,
        sdsnew(b"idieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xec as glyphid_t,
        sdsnew(b"igrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x133 as glyphid_t,
        sdsnew(b"ij\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12b as glyphid_t,
        sdsnew(b"imacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x221e as glyphid_t,
        sdsnew(b"infinity\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x222b as glyphid_t,
        sdsnew(b"integral\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2321 as glyphid_t,
        sdsnew(b"integralbt\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2320 as glyphid_t,
        sdsnew(b"integraltp\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2229 as glyphid_t,
        sdsnew(b"intersection\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25d8 as glyphid_t,
        sdsnew(b"invbullet\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25d9 as glyphid_t,
        sdsnew(b"invcircle\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x263b as glyphid_t,
        sdsnew(b"invsmileface\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x12f as glyphid_t,
        sdsnew(b"iogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b9 as glyphid_t,
        sdsnew(b"iota\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ca as glyphid_t,
        sdsnew(b"iotadieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x390 as glyphid_t,
        sdsnew(b"iotadieresistonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3af as glyphid_t,
        sdsnew(b"iotatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x129 as glyphid_t,
        sdsnew(b"itilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6a as glyphid_t,
        sdsnew(b"j\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x135 as glyphid_t,
        sdsnew(b"jcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6b as glyphid_t,
        sdsnew(b"k\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ba as glyphid_t,
        sdsnew(b"kappa\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x138 as glyphid_t,
        sdsnew(b"kgreenlandic\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6c as glyphid_t,
        sdsnew(b"l\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x13a as glyphid_t,
        sdsnew(b"lacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3bb as glyphid_t,
        sdsnew(b"lambda\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x13e as glyphid_t,
        sdsnew(b"lcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x140 as glyphid_t,
        sdsnew(b"ldot\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c as glyphid_t,
        sdsnew(b"less\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2264 as glyphid_t,
        sdsnew(b"lessequal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x258c as glyphid_t,
        sdsnew(b"lfblock\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20a4 as glyphid_t,
        sdsnew(b"lira\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2227 as glyphid_t,
        sdsnew(b"logicaland\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xac as glyphid_t,
        sdsnew(b"logicalnot\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2228 as glyphid_t,
        sdsnew(b"logicalor\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17f as glyphid_t,
        sdsnew(b"longs\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25ca as glyphid_t,
        sdsnew(b"lozenge\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x142 as glyphid_t,
        sdsnew(b"lslash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2591 as glyphid_t,
        sdsnew(b"ltshade\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6d as glyphid_t,
        sdsnew(b"m\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xaf as glyphid_t,
        sdsnew(b"macron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2642 as glyphid_t,
        sdsnew(b"male\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2212 as glyphid_t,
        sdsnew(b"minus\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2032 as glyphid_t,
        sdsnew(b"minute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb5 as glyphid_t,
        sdsnew(b"mu\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xd7 as glyphid_t,
        sdsnew(b"multiply\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x266a as glyphid_t,
        sdsnew(b"musicalnote\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x266b as glyphid_t,
        sdsnew(b"musicalnotedbl\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6e as glyphid_t,
        sdsnew(b"n\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x144 as glyphid_t,
        sdsnew(b"nacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x149 as glyphid_t,
        sdsnew(b"napostrophe\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x148 as glyphid_t,
        sdsnew(b"ncaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x39 as glyphid_t,
        sdsnew(b"nine\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2209 as glyphid_t,
        sdsnew(b"notelement\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2260 as glyphid_t,
        sdsnew(b"notequal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2284 as glyphid_t,
        sdsnew(b"notsubset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf1 as glyphid_t,
        sdsnew(b"ntilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3bd as glyphid_t,
        sdsnew(b"nu\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x23 as glyphid_t,
        sdsnew(b"numbersign\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x6f as glyphid_t,
        sdsnew(b"o\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf3 as glyphid_t,
        sdsnew(b"oacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14f as glyphid_t,
        sdsnew(b"obreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf4 as glyphid_t,
        sdsnew(b"ocircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf6 as glyphid_t,
        sdsnew(b"odieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x153 as glyphid_t,
        sdsnew(b"oe\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2db as glyphid_t,
        sdsnew(b"ogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf2 as glyphid_t,
        sdsnew(b"ograve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1a1 as glyphid_t,
        sdsnew(b"ohorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x151 as glyphid_t,
        sdsnew(b"ohungarumlaut\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x14d as glyphid_t,
        sdsnew(b"omacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c9 as glyphid_t,
        sdsnew(b"omega\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3d6 as glyphid_t,
        sdsnew(b"omega1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3ce as glyphid_t,
        sdsnew(b"omegatonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3bf as glyphid_t,
        sdsnew(b"omicron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3cc as glyphid_t,
        sdsnew(b"omicrontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x31 as glyphid_t,
        sdsnew(b"one\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2024 as glyphid_t,
        sdsnew(b"onedotenleader\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x215b as glyphid_t,
        sdsnew(b"oneeighth\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xbd as glyphid_t,
        sdsnew(b"onehalf\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xbc as glyphid_t,
        sdsnew(b"onequarter\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2153 as glyphid_t,
        sdsnew(b"onethird\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25e6 as glyphid_t,
        sdsnew(b"openbullet\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xaa as glyphid_t,
        sdsnew(b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xba as glyphid_t,
        sdsnew(b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x221f as glyphid_t,
        sdsnew(b"orthogonal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf8 as glyphid_t,
        sdsnew(b"oslash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1ff as glyphid_t,
        sdsnew(b"oslashacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf5 as glyphid_t,
        sdsnew(b"otilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x70 as glyphid_t,
        sdsnew(b"p\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb6 as glyphid_t,
        sdsnew(b"paragraph\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x28 as glyphid_t,
        sdsnew(b"parenleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x29 as glyphid_t,
        sdsnew(b"parenright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2202 as glyphid_t,
        sdsnew(b"partialdiff\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25 as glyphid_t,
        sdsnew(b"percent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2e as glyphid_t,
        sdsnew(b"period\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb7 as glyphid_t,
        sdsnew(b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x22a5 as glyphid_t,
        sdsnew(b"perpendicular\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2030 as glyphid_t,
        sdsnew(b"perthousand\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20a7 as glyphid_t,
        sdsnew(b"peseta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c6 as glyphid_t,
        sdsnew(b"phi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3d5 as glyphid_t,
        sdsnew(b"phi1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c0 as glyphid_t,
        sdsnew(b"pi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2b as glyphid_t,
        sdsnew(b"plus\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xb1 as glyphid_t,
        sdsnew(b"plusminus\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x211e as glyphid_t,
        sdsnew(b"prescription\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x220f as glyphid_t,
        sdsnew(b"product\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2282 as glyphid_t,
        sdsnew(b"propersubset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2283 as glyphid_t,
        sdsnew(b"propersuperset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x221d as glyphid_t,
        sdsnew(b"proportional\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c8 as glyphid_t,
        sdsnew(b"psi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x71 as glyphid_t,
        sdsnew(b"q\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3f as glyphid_t,
        sdsnew(b"question\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xbf as glyphid_t,
        sdsnew(b"questiondown\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x22 as glyphid_t,
        sdsnew(b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x201e as glyphid_t,
        sdsnew(b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x201c as glyphid_t,
        sdsnew(b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x201d as glyphid_t,
        sdsnew(b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2018 as glyphid_t,
        sdsnew(b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x201b as glyphid_t,
        sdsnew(b"quotereversed\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2019 as glyphid_t,
        sdsnew(b"quoteright\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x201a as glyphid_t,
        sdsnew(b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x27 as glyphid_t,
        sdsnew(b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x72 as glyphid_t,
        sdsnew(b"r\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x155 as glyphid_t,
        sdsnew(b"racute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x221a as glyphid_t,
        sdsnew(b"radical\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x159 as glyphid_t,
        sdsnew(b"rcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2286 as glyphid_t,
        sdsnew(b"reflexsubset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2287 as glyphid_t,
        sdsnew(b"reflexsuperset\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xae as glyphid_t,
        sdsnew(b"registered\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2310 as glyphid_t,
        sdsnew(b"revlogicalnot\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c1 as glyphid_t,
        sdsnew(b"rho\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2da as glyphid_t,
        sdsnew(b"ring\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2590 as glyphid_t,
        sdsnew(b"rtblock\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x73 as glyphid_t,
        sdsnew(b"s\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15b as glyphid_t,
        sdsnew(b"sacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x161 as glyphid_t,
        sdsnew(b"scaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15f as glyphid_t,
        sdsnew(b"scedilla\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x15d as glyphid_t,
        sdsnew(b"scircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2033 as glyphid_t,
        sdsnew(b"second\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa7 as glyphid_t,
        sdsnew(b"section\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b as glyphid_t,
        sdsnew(b"semicolon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x37 as glyphid_t,
        sdsnew(b"seven\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x215e as glyphid_t,
        sdsnew(b"seveneighths\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2592 as glyphid_t,
        sdsnew(b"shade\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c3 as glyphid_t,
        sdsnew(b"sigma\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c2 as glyphid_t,
        sdsnew(b"sigma1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x223c as glyphid_t,
        sdsnew(b"similar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x36 as glyphid_t,
        sdsnew(b"six\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2f as glyphid_t,
        sdsnew(b"slash\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x263a as glyphid_t,
        sdsnew(b"smileface\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x20 as glyphid_t,
        sdsnew(b"space\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2660 as glyphid_t,
        sdsnew(b"spade\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa3 as glyphid_t,
        sdsnew(b"sterling\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x220b as glyphid_t,
        sdsnew(b"suchthat\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2211 as glyphid_t,
        sdsnew(b"summation\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x263c as glyphid_t,
        sdsnew(b"sun\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x74 as glyphid_t,
        sdsnew(b"t\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c4 as glyphid_t,
        sdsnew(b"tau\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x167 as glyphid_t,
        sdsnew(b"tbar\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x165 as glyphid_t,
        sdsnew(b"tcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2234 as glyphid_t,
        sdsnew(b"therefore\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b8 as glyphid_t,
        sdsnew(b"theta\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3d1 as glyphid_t,
        sdsnew(b"theta1\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xfe as glyphid_t,
        sdsnew(b"thorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x33 as glyphid_t,
        sdsnew(b"three\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x215c as glyphid_t,
        sdsnew(b"threeeighths\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xbe as glyphid_t,
        sdsnew(b"threequarters\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2dc as glyphid_t,
        sdsnew(b"tilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x303 as glyphid_t,
        sdsnew(b"tildecomb\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x384 as glyphid_t,
        sdsnew(b"tonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2122 as glyphid_t,
        sdsnew(b"trademark\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25bc as glyphid_t,
        sdsnew(b"triagdn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25c4 as glyphid_t,
        sdsnew(b"triaglf\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25ba as glyphid_t,
        sdsnew(b"triagrt\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x25b2 as glyphid_t,
        sdsnew(b"triagup\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x32 as glyphid_t,
        sdsnew(b"two\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2025 as glyphid_t,
        sdsnew(b"twodotenleader\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2154 as glyphid_t,
        sdsnew(b"twothirds\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x75 as glyphid_t,
        sdsnew(b"u\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xfa as glyphid_t,
        sdsnew(b"uacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16d as glyphid_t,
        sdsnew(b"ubreve\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xfb as glyphid_t,
        sdsnew(b"ucircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xfc as glyphid_t,
        sdsnew(b"udieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xf9 as glyphid_t,
        sdsnew(b"ugrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1b0 as glyphid_t,
        sdsnew(b"uhorn\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x171 as glyphid_t,
        sdsnew(b"uhungarumlaut\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16b as glyphid_t,
        sdsnew(b"umacron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x5f as glyphid_t,
        sdsnew(b"underscore\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2017 as glyphid_t,
        sdsnew(b"underscoredbl\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x222a as glyphid_t,
        sdsnew(b"union\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2200 as glyphid_t,
        sdsnew(b"universal\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x173 as glyphid_t,
        sdsnew(b"uogonek\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2580 as glyphid_t,
        sdsnew(b"upblock\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3c5 as glyphid_t,
        sdsnew(b"upsilon\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3cb as glyphid_t,
        sdsnew(b"upsilondieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b0 as glyphid_t,
        sdsnew(b"upsilondieresistonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3cd as glyphid_t,
        sdsnew(b"upsilontonos\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x16f as glyphid_t,
        sdsnew(b"uring\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x169 as glyphid_t,
        sdsnew(b"utilde\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x76 as glyphid_t,
        sdsnew(b"v\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x77 as glyphid_t,
        sdsnew(b"w\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e83 as glyphid_t,
        sdsnew(b"wacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x175 as glyphid_t,
        sdsnew(b"wcircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e85 as glyphid_t,
        sdsnew(b"wdieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x2118 as glyphid_t,
        sdsnew(b"weierstrass\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1e81 as glyphid_t,
        sdsnew(b"wgrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x78 as glyphid_t,
        sdsnew(b"x\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3be as glyphid_t,
        sdsnew(b"xi\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x79 as glyphid_t,
        sdsnew(b"y\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xfd as glyphid_t,
        sdsnew(b"yacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x177 as glyphid_t,
        sdsnew(b"ycircumflex\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xff as glyphid_t,
        sdsnew(b"ydieresis\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0xa5 as glyphid_t,
        sdsnew(b"yen\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x1ef3 as glyphid_t,
        sdsnew(b"ygrave\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x7a as glyphid_t,
        sdsnew(b"z\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17a as glyphid_t,
        sdsnew(b"zacute\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17e as glyphid_t,
        sdsnew(b"zcaron\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x17c as glyphid_t,
        sdsnew(b"zdotaccent\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x30 as glyphid_t,
        sdsnew(b"zero\0" as *const u8 as *const ::core::ffi::c_char),
    );
    otfcc_pkgGlyphOrder
        .setByGID
        .expect("non-null function pointer")(
        map,
        0x3b6 as glyphid_t,
        sdsnew(b"zeta\0" as *const u8 as *const ::core::ffi::c_char),
    );
}
