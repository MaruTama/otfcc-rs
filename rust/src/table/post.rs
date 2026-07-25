use libc::{free, malloc, memcpy, memset, strcmp};
extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite8(buf: *mut caryll_Buffer, byte: u8);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn bufwrite_sds(buf: *mut caryll_Buffer, str: sds);
    fn otfcc_from_fixed(x: f16dot16) -> ::core::ffi::c_double;
    fn otfcc_to_fixed(x: ::core::ffi::c_double) -> f16dot16;
    static otfcc_pkgGlyphOrder: otfcc_GlyphOrderPackage;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_double_new(_: ::core::ffi::c_double) -> *mut json_value;
    fn json_boolean_new(_: ::core::ffi::c_int) -> *mut json_value;
}

use crate::support::binio::{read_16u, read_32u, read_32s};
use crate::logger::{otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{f16dot16, font_file_pointer, glyphid_t};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, sds, sdshdr16, sdshdr32, sdshdr64, sdshdr8};
use crate::vendor::json::{json_boolean, json_double, json_integer, json_object, json_type, json_value};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};
use crate::support::{NULL};
use crate::support::glyph_order::{otfcc_GlyphOrder, otfcc_GlyphOrderEntry, otfcc_GlyphOrderPackage};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_post {
    pub version: f16dot16,
    pub italicAngle: f16dot16,
    pub underlinePosition: i16,
    pub underlineThickness: i16,
    pub isFixedPitch: u32,
    pub minMemType42: u32,
    pub maxMemType42: u32,
    pub minMemType1: u32,
    pub maxMemType1: u32,
    pub post_name_map: *mut otfcc_GlyphOrder,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_table_post {
    pub init: Option<unsafe extern "C" fn(*mut table_post) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_post, *const table_post) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_post, *mut table_post) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_post) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_post, table_post) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_post, table_post) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_post>,
    pub free: Option<unsafe extern "C" fn(*mut table_post) -> ()>,
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
static mut standardMacNames: [*const ::core::ffi::c_char; 258] = [
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".null\0" as *const u8 as *const ::core::ffi::c_char,
    b"nonmarkingreturn\0" as *const u8 as *const ::core::ffi::c_char,
    b"space\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclam\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"numbersign\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollar\0" as *const u8 as *const ::core::ffi::c_char,
    b"percent\0" as *const u8 as *const ::core::ffi::c_char,
    b"ampersand\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asterisk\0" as *const u8 as *const ::core::ffi::c_char,
    b"plus\0" as *const u8 as *const ::core::ffi::c_char,
    b"comma\0" as *const u8 as *const ::core::ffi::c_char,
    b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
    b"period\0" as *const u8 as *const ::core::ffi::c_char,
    b"slash\0" as *const u8 as *const ::core::ffi::c_char,
    b"zero\0" as *const u8 as *const ::core::ffi::c_char,
    b"one\0" as *const u8 as *const ::core::ffi::c_char,
    b"two\0" as *const u8 as *const ::core::ffi::c_char,
    b"three\0" as *const u8 as *const ::core::ffi::c_char,
    b"four\0" as *const u8 as *const ::core::ffi::c_char,
    b"five\0" as *const u8 as *const ::core::ffi::c_char,
    b"six\0" as *const u8 as *const ::core::ffi::c_char,
    b"seven\0" as *const u8 as *const ::core::ffi::c_char,
    b"eight\0" as *const u8 as *const ::core::ffi::c_char,
    b"nine\0" as *const u8 as *const ::core::ffi::c_char,
    b"colon\0" as *const u8 as *const ::core::ffi::c_char,
    b"semicolon\0" as *const u8 as *const ::core::ffi::c_char,
    b"less\0" as *const u8 as *const ::core::ffi::c_char,
    b"equal\0" as *const u8 as *const ::core::ffi::c_char,
    b"greater\0" as *const u8 as *const ::core::ffi::c_char,
    b"question\0" as *const u8 as *const ::core::ffi::c_char,
    b"at\0" as *const u8 as *const ::core::ffi::c_char,
    b"A\0" as *const u8 as *const ::core::ffi::c_char,
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"C\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"E\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"I\0" as *const u8 as *const ::core::ffi::c_char,
    b"J\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"O\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Q\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"U\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
    b"X\0" as *const u8 as *const ::core::ffi::c_char,
    b"Y\0" as *const u8 as *const ::core::ffi::c_char,
    b"Z\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"backslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char,
    b"underscore\0" as *const u8 as *const ::core::ffi::c_char,
    b"grave\0" as *const u8 as *const ::core::ffi::c_char,
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"b\0" as *const u8 as *const ::core::ffi::c_char,
    b"c\0" as *const u8 as *const ::core::ffi::c_char,
    b"d\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"f\0" as *const u8 as *const ::core::ffi::c_char,
    b"g\0" as *const u8 as *const ::core::ffi::c_char,
    b"h\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"j\0" as *const u8 as *const ::core::ffi::c_char,
    b"k\0" as *const u8 as *const ::core::ffi::c_char,
    b"l\0" as *const u8 as *const ::core::ffi::c_char,
    b"m\0" as *const u8 as *const ::core::ffi::c_char,
    b"n\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"p\0" as *const u8 as *const ::core::ffi::c_char,
    b"q\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\0" as *const u8 as *const ::core::ffi::c_char,
    b"s\0" as *const u8 as *const ::core::ffi::c_char,
    b"t\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"v\0" as *const u8 as *const ::core::ffi::c_char,
    b"w\0" as *const u8 as *const ::core::ffi::c_char,
    b"x\0" as *const u8 as *const ::core::ffi::c_char,
    b"y\0" as *const u8 as *const ::core::ffi::c_char,
    b"z\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"bar\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"dagger\0" as *const u8 as *const ::core::ffi::c_char,
    b"degree\0" as *const u8 as *const ::core::ffi::c_char,
    b"cent\0" as *const u8 as *const ::core::ffi::c_char,
    b"sterling\0" as *const u8 as *const ::core::ffi::c_char,
    b"section\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"paragraph\0" as *const u8 as *const ::core::ffi::c_char,
    b"germandbls\0" as *const u8 as *const ::core::ffi::c_char,
    b"registered\0" as *const u8 as *const ::core::ffi::c_char,
    b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
    b"trademark\0" as *const u8 as *const ::core::ffi::c_char,
    b"acute\0" as *const u8 as *const ::core::ffi::c_char,
    b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"notequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"AE\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"infinity\0" as *const u8 as *const ::core::ffi::c_char,
    b"plusminus\0" as *const u8 as *const ::core::ffi::c_char,
    b"lessequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"greaterequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"yen\0" as *const u8 as *const ::core::ffi::c_char,
    b"mu\0" as *const u8 as *const ::core::ffi::c_char,
    b"partialdiff\0" as *const u8 as *const ::core::ffi::c_char,
    b"summation\0" as *const u8 as *const ::core::ffi::c_char,
    b"product\0" as *const u8 as *const ::core::ffi::c_char,
    b"pi\0" as *const u8 as *const ::core::ffi::c_char,
    b"integral\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char,
    b"Omega\0" as *const u8 as *const ::core::ffi::c_char,
    b"ae\0" as *const u8 as *const ::core::ffi::c_char,
    b"oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char,
    b"logicalnot\0" as *const u8 as *const ::core::ffi::c_char,
    b"radical\0" as *const u8 as *const ::core::ffi::c_char,
    b"florin\0" as *const u8 as *const ::core::ffi::c_char,
    b"approxequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Delta\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char,
    b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char,
    b"nonbreakingspace\0" as *const u8 as *const ::core::ffi::c_char,
    b"Agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"OE\0" as *const u8 as *const ::core::ffi::c_char,
    b"oe\0" as *const u8 as *const ::core::ffi::c_char,
    b"endash\0" as *const u8 as *const ::core::ffi::c_char,
    b"emdash\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteright\0" as *const u8 as *const ::core::ffi::c_char,
    b"divide\0" as *const u8 as *const ::core::ffi::c_char,
    b"lozenge\0" as *const u8 as *const ::core::ffi::c_char,
    b"ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"fraction\0" as *const u8 as *const ::core::ffi::c_char,
    b"currency\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char,
    b"fi\0" as *const u8 as *const ::core::ffi::c_char,
    b"fl\0" as *const u8 as *const ::core::ffi::c_char,
    b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"perthousand\0" as *const u8 as *const ::core::ffi::c_char,
    b"Acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"apple\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"Uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char,
    b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"macron\0" as *const u8 as *const ::core::ffi::c_char,
    b"breve\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"ring\0" as *const u8 as *const ::core::ffi::c_char,
    b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char,
    b"ogonek\0" as *const u8 as *const ::core::ffi::c_char,
    b"caron\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"lslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"Scaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"scaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"Zcaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"zcaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"brokenbar\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"Yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"minus\0" as *const u8 as *const ::core::ffi::c_char,
    b"multiply\0" as *const u8 as *const ::core::ffi::c_char,
    b"onesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"twosuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"threesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"onehalf\0" as *const u8 as *const ::core::ffi::c_char,
    b"onequarter\0" as *const u8 as *const ::core::ffi::c_char,
    b"threequarters\0" as *const u8 as *const ::core::ffi::c_char,
    b"franc\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gbreve\0" as *const u8 as *const ::core::ffi::c_char,
    b"gbreve\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idotaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"Scedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"scedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"cacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"ccaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"dcroat\0" as *const u8 as *const ::core::ffi::c_char,
];
#[inline]
unsafe extern "C" fn initPost(mut post: *mut table_post) {
    memset(
        post as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_post>() as usize,
    );
    (*post).version = 0x30000 as ::core::ffi::c_int as f16dot16;
}
#[inline]
unsafe extern "C" fn disposePost(mut post: *mut table_post) {
    if !(*post).post_name_map.is_null() {
        otfcc_pkgGlyphOrder.free.expect("non-null function pointer")((*post).post_name_map);
    }
}
#[inline]
unsafe extern "C" fn table_post_dispose(mut x: *mut table_post) {
    disposePost(x);
}
#[inline]
unsafe extern "C" fn table_post_free(mut x: *mut table_post) {
    if x.is_null() {
        return;
    }
    table_post_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_post_create() -> *mut table_post {
    let mut x: *mut table_post =
        malloc(::core::mem::size_of::<table_post>() as usize) as *mut table_post;
    table_post_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_post_init(mut x: *mut table_post) {
    initPost(x);
}
#[inline]
unsafe extern "C" fn table_post_copy(mut dst: *mut table_post, mut src: *const table_post) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_post>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_post_copyReplace(mut dst: *mut table_post, src: table_post) {
    table_post_dispose(dst);
    table_post_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_post_move(mut dst: *mut table_post, mut src: *mut table_post) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_post>() as usize,
    );
    table_post_init(src);
}
#[inline]
unsafe extern "C" fn table_post_replace(mut dst: *mut table_post, src: table_post) {
    table_post_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_post>() as usize,
    );
}
#[no_mangle]
pub static mut iTable_post: __caryll_elementinterface_table_post = {
    __caryll_elementinterface_table_post {
        init: Some(table_post_init as unsafe extern "C" fn(*mut table_post) -> ()),
        copy: Some(
            table_post_copy as unsafe extern "C" fn(*mut table_post, *const table_post) -> (),
        ),
        move_0: Some(
            table_post_move as unsafe extern "C" fn(*mut table_post, *mut table_post) -> (),
        ),
        dispose: Some(table_post_dispose as unsafe extern "C" fn(*mut table_post) -> ()),
        replace: Some(
            table_post_replace as unsafe extern "C" fn(*mut table_post, table_post) -> (),
        ),
        copyReplace: Some(
            table_post_copyReplace as unsafe extern "C" fn(*mut table_post, table_post) -> (),
        ),
        create: Some(table_post_create),
        free: Some(table_post_free as unsafe extern "C" fn(*mut table_post) -> ()),
    }
};
#[no_mangle]
pub unsafe extern "C" fn otfcc_readPost(
    packet: otfcc_Packet,
    mut _options: *const otfcc_Options,
) -> *mut table_post {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1886352244i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut post: *mut table_post =
                        (
                            iTable_post.create.expect("non-null function pointer"))();
                    (*post).version = read_32s(data as *const u8) as f16dot16;
                    (*post).italicAngle =
                        read_32u(data.offset(4 as ::core::ffi::c_int as isize) as *const u8)
                            as f16dot16;
                    (*post).underlinePosition =
                        read_16u(data.offset(8 as ::core::ffi::c_int as isize) as *const u8)
                            as i16;
                    (*post).underlineThickness =
                        read_16u(data.offset(10 as ::core::ffi::c_int as isize) as *const u8)
                            as i16;
                    (*post).isFixedPitch =
                        read_32u(data.offset(12 as ::core::ffi::c_int as isize) as *const u8);
                    (*post).minMemType42 =
                        read_32u(data.offset(16 as ::core::ffi::c_int as isize) as *const u8);
                    (*post).maxMemType42 =
                        read_32u(data.offset(20 as ::core::ffi::c_int as isize) as *const u8);
                    (*post).minMemType1 =
                        read_32u(data.offset(24 as ::core::ffi::c_int as isize) as *const u8);
                    (*post).maxMemType1 =
                        read_32u(data.offset(28 as ::core::ffi::c_int as isize) as *const u8);
                    (*post).post_name_map = ::core::ptr::null_mut::<otfcc_GlyphOrder>();
                    if (*post).version == 0x20000 as f16dot16 {
                        let mut map: *mut otfcc_GlyphOrder =
                            (
                                otfcc_pkgGlyphOrder
                                    .create
                                    .expect("non-null function pointer"))();
                        let mut pendingNames: [sds; 65536] =
                            [::core::ptr::null_mut::<::core::ffi::c_char>(); 65536];
                        memset(
                            &raw mut pendingNames as *mut sds as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<[sds; 65536]>() as usize,
                        );
                        let mut numberGlyphs: u16 = read_16u(
                            data.offset(32 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        let mut offset: u32 = (34 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * numberGlyphs as ::core::ffi::c_int)
                            as u32;
                        let mut pendingNameIndex: u16 = 0 as u16;
                        while pendingNameIndex as ::core::ffi::c_int <= 0xffff as ::core::ffi::c_int
                            && offset < table.length
                        {
                            let mut len: u8 = *data.offset(offset as isize);
                            let mut s: sds = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            if len as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                s = sdsnewlen(
                                    data.offset(offset as isize)
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as *const ::core::ffi::c_void,
                                    len as usize,
                                );
                            } else {
                                s = sdsempty();
                            }
                            offset = offset.wrapping_add(
                                (len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u32,
                            );
                            pendingNames[pendingNameIndex as usize] = s;
                            pendingNameIndex = (pendingNameIndex as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as u16;
                        }
                        let mut j: u16 = 0 as u16;
                        while (j as ::core::ffi::c_int) < numberGlyphs as ::core::ffi::c_int {
                            let mut nameMap: u16 =
                                read_16u(data.offset(34 as ::core::ffi::c_int as isize).offset(
                                    (2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                                ) as *const u8);
                            if nameMap as ::core::ffi::c_int >= 258 as ::core::ffi::c_int {
                                otfcc_pkgGlyphOrder
                                    .setByGID
                                    .expect("non-null function pointer")(
                                    map,
                                    j as glyphid_t,
                                    sdsdup(
                                        pendingNames[(nameMap as ::core::ffi::c_int
                                            - 258 as ::core::ffi::c_int)
                                            as usize],
                                    ),
                                );
                            } else {
                                otfcc_pkgGlyphOrder
                                    .setByGID
                                    .expect("non-null function pointer")(
                                    map,
                                    j as glyphid_t,
                                    sdsnew(standardMacNames[nameMap as usize]),
                                );
                            }
                            j = j.wrapping_add(1);
                        }
                        let mut j_0: u32 = 0 as u32;
                        while j_0 < pendingNameIndex as u32 {
                            sdsfree(pendingNames[j_0 as usize]);
                            j_0 = j_0.wrapping_add(1);
                        }
                        (*post).post_name_map = map;
                    }
                    return post;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<table_post>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpPost(
    mut table: *const table_post,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"post"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut post: *mut json_value = json_object_new(10 as usize);
        json_object_push(
            post,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            post,
            b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new(otfcc_from_fixed((*table).italicAngle) as i64),
        );
        json_object_push(
            post,
            b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).underlinePosition as i64),
        );
        json_object_push(
            post,
            b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).underlineThickness as i64),
        );
        json_object_push(
            post,
            b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*table).isFixedPitch as ::core::ffi::c_int),
        );
        json_object_push(
            post,
            b"minMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).minMemType42 as i64),
        );
        json_object_push(
            post,
            b"maxMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxMemType42 as i64),
        );
        json_object_push(
            post,
            b"minMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).minMemType1 as i64),
        );
        json_object_push(
            post,
            b"maxMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).maxMemType1 as i64),
        );
        json_object_push(
            root,
            b"post\0" as *const u8 as *const ::core::ffi::c_char,
            post,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parsePost(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_post {
    let mut post: *mut table_post = (
        iTable_post.create.expect("non-null function pointer"))();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"post\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), b"post"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            if (*options).short_post {
                (*post).version = 0x30000 as ::core::ffi::c_int as f16dot16;
            } else {
                (*post).version = otfcc_to_fixed(json_obj_getnum(
                    table,
                    b"version\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            }
            (*post).italicAngle = otfcc_to_fixed(json_obj_getnum(
                table,
                b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            (*post).underlinePosition = json_obj_getnum(
                table,
                b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            ) as i16;
            (*post).underlineThickness = json_obj_getnum(
                table,
                b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            ) as i16;
            (*post).isFixedPitch = json_obj_getbool(
                table,
                b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).minMemType42 = json_obj_getnum(
                table,
                b"minMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).maxMemType42 = json_obj_getnum(
                table,
                b"maxMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).minMemType1 = json_obj_getnum(
                table,
                b"minMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).maxMemType1 = json_obj_getnum(
                table,
                b"maxMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    return post;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildPost(
    mut post: *const table_post,
    mut glyphorder: *mut otfcc_GlyphOrder,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if post.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite32b(buf, (*post).version as u32);
    bufwrite32b(buf, (*post).italicAngle as u32);
    bufwrite16b(buf, (*post).underlinePosition as u16);
    bufwrite16b(buf, (*post).underlineThickness as u16);
    bufwrite32b(buf, (*post).isFixedPitch);
    bufwrite32b(buf, (*post).minMemType42);
    bufwrite32b(buf, (*post).maxMemType42);
    bufwrite32b(buf, (*post).minMemType1);
    bufwrite32b(buf, (*post).maxMemType1);
    if (*post).version == 0x20000 as f16dot16 {
        bufwrite16b(
            buf,
            (if !(*glyphorder).byName.is_null() {
                (*(*(*glyphorder).byName).hhName.tbl).num_items
            } else {
                0 as ::core::ffi::c_uint
            }) as u16,
        );
        let mut s: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
        let mut tmp: *mut otfcc_GlyphOrderEntry = ::core::ptr::null_mut::<otfcc_GlyphOrderEntry>();
        s = (*glyphorder).byName;
        tmp = (if !(*glyphorder).byName.is_null() {
            (*(*glyphorder).byName).hhName.next
        } else {
            NULL
        }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        while !s.is_null() {
            bufwrite16b(
                buf,
                (258 as ::core::ffi::c_int + (*s).gid as ::core::ffi::c_int) as u16,
            );
            s = tmp;
            tmp = (if !tmp.is_null() {
                (*tmp).hhName.next
            } else {
                NULL
            }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        }
        s = (*glyphorder).byName;
        tmp = (if !(*glyphorder).byName.is_null() {
            (*(*glyphorder).byName).hhName.next
        } else {
            NULL
        }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        while !s.is_null() {
            bufwrite8(buf, sdslen((*s).name) as u8);
            bufwrite_sds(buf, (*s).name);
            s = tmp;
            tmp = (if !tmp.is_null() {
                (*tmp).hhName.next
            } else {
                NULL
            }) as *mut otfcc_GlyphOrderEntry as *mut otfcc_GlyphOrderEntry;
        }
    }
    return buf;
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_getnum(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0.0f64;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0.0f64;
}
#[inline]
unsafe extern "C" fn json_obj_getbool(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> bool {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_boolean as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.boolean != 0;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return false;
}
