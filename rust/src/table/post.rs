#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, strcmp};
unsafe extern "C" {
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
use crate::vendor::json::{json_double, json_integer, json_object, json_type, json_value};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};
use crate::support::{NULL};
use crate::support::glyph_order::{otfcc_GlyphOrder, otfcc_GlyphOrderEntry, otfcc_GlyphOrderPackage};
use crate::support::json_funcs::{json_obj_getbool};

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
static standardMacNames: [&::core::ffi::CStr; 258] = [
    c".notdef",
    c".null",
    c"nonmarkingreturn",
    c"space",
    c"exclam",
    c"quotedbl",
    c"numbersign",
    c"dollar",
    c"percent",
    c"ampersand",
    c"quotesingle",
    c"parenleft",
    c"parenright",
    c"asterisk",
    c"plus",
    c"comma",
    c"hyphen",
    c"period",
    c"slash",
    c"zero",
    c"one",
    c"two",
    c"three",
    c"four",
    c"five",
    c"six",
    c"seven",
    c"eight",
    c"nine",
    c"colon",
    c"semicolon",
    c"less",
    c"equal",
    c"greater",
    c"question",
    c"at",
    c"A",
    c"B",
    c"C",
    c"D",
    c"E",
    c"F",
    c"G",
    c"H",
    c"I",
    c"J",
    c"K",
    c"L",
    c"M",
    c"N",
    c"O",
    c"P",
    c"Q",
    c"R",
    c"S",
    c"T",
    c"U",
    c"V",
    c"W",
    c"X",
    c"Y",
    c"Z",
    c"bracketleft",
    c"backslash",
    c"bracketright",
    c"asciicircum",
    c"underscore",
    c"grave",
    c"a",
    c"b",
    c"c",
    c"d",
    c"e",
    c"f",
    c"g",
    c"h",
    c"i",
    c"j",
    c"k",
    c"l",
    c"m",
    c"n",
    c"o",
    c"p",
    c"q",
    c"r",
    c"s",
    c"t",
    c"u",
    c"v",
    c"w",
    c"x",
    c"y",
    c"z",
    c"braceleft",
    c"bar",
    c"braceright",
    c"asciitilde",
    c"Adieresis",
    c"Aring",
    c"Ccedilla",
    c"Eacute",
    c"Ntilde",
    c"Odieresis",
    c"Udieresis",
    c"aacute",
    c"agrave",
    c"acircumflex",
    c"adieresis",
    c"atilde",
    c"aring",
    c"ccedilla",
    c"eacute",
    c"egrave",
    c"ecircumflex",
    c"edieresis",
    c"iacute",
    c"igrave",
    c"icircumflex",
    c"idieresis",
    c"ntilde",
    c"oacute",
    c"ograve",
    c"ocircumflex",
    c"odieresis",
    c"otilde",
    c"uacute",
    c"ugrave",
    c"ucircumflex",
    c"udieresis",
    c"dagger",
    c"degree",
    c"cent",
    c"sterling",
    c"section",
    c"bullet",
    c"paragraph",
    c"germandbls",
    c"registered",
    c"copyright",
    c"trademark",
    c"acute",
    c"dieresis",
    c"notequal",
    c"AE",
    c"Oslash",
    c"infinity",
    c"plusminus",
    c"lessequal",
    c"greaterequal",
    c"yen",
    c"mu",
    c"partialdiff",
    c"summation",
    c"product",
    c"pi",
    c"integral",
    c"ordfeminine",
    c"ordmasculine",
    c"Omega",
    c"ae",
    c"oslash",
    c"questiondown",
    c"exclamdown",
    c"logicalnot",
    c"radical",
    c"florin",
    c"approxequal",
    c"Delta",
    c"guillemotleft",
    c"guillemotright",
    c"ellipsis",
    c"nonbreakingspace",
    c"Agrave",
    c"Atilde",
    c"Otilde",
    c"OE",
    c"oe",
    c"endash",
    c"emdash",
    c"quotedblleft",
    c"quotedblright",
    c"quoteleft",
    c"quoteright",
    c"divide",
    c"lozenge",
    c"ydieresis",
    c"Ydieresis",
    c"fraction",
    c"currency",
    c"guilsinglleft",
    c"guilsinglright",
    c"fi",
    c"fl",
    c"daggerdbl",
    c"periodcentered",
    c"quotesinglbase",
    c"quotedblbase",
    c"perthousand",
    c"Acircumflex",
    c"Ecircumflex",
    c"Aacute",
    c"Edieresis",
    c"Egrave",
    c"Iacute",
    c"Icircumflex",
    c"Idieresis",
    c"Igrave",
    c"Oacute",
    c"Ocircumflex",
    c"apple",
    c"Ograve",
    c"Uacute",
    c"Ucircumflex",
    c"Ugrave",
    c"dotlessi",
    c"circumflex",
    c"tilde",
    c"macron",
    c"breve",
    c"dotaccent",
    c"ring",
    c"cedilla",
    c"hungarumlaut",
    c"ogonek",
    c"caron",
    c"Lslash",
    c"lslash",
    c"Scaron",
    c"scaron",
    c"Zcaron",
    c"zcaron",
    c"brokenbar",
    c"Eth",
    c"eth",
    c"Yacute",
    c"yacute",
    c"Thorn",
    c"thorn",
    c"minus",
    c"multiply",
    c"onesuperior",
    c"twosuperior",
    c"threesuperior",
    c"onehalf",
    c"onequarter",
    c"threequarters",
    c"franc",
    c"Gbreve",
    c"gbreve",
    c"Idotaccent",
    c"Scedilla",
    c"scedilla",
    c"Cacute",
    c"cacute",
    c"Ccaron",
    c"ccaron",
    c"dcroat",
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
#[unsafe(no_mangle)]
pub static iTable_post: __caryll_elementinterface_table_post = {
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
#[unsafe(no_mangle)]
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
                                    sdsnew(standardMacNames[nameMap as usize].as_ptr()),
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
        || (*obj).type_0 != json_object
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
        || (*obj).type_0 != json_object
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
                && (*cv).type_0 == json_integer
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 == json_double
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0.0f64;
}
