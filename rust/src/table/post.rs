#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};

use crate::support::binio::{read_16u, read_32u, read_32s};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, FontFilePointer, GlyphId};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::{JsonType, JsonValue};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::{NULL};
use crate::support::glyph_order::{GlyphOrder, GlyphOrderEntry};
use crate::support::json_funcs::{json_obj_get_type, json_obj_getbool, json_obj_getnum};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b, bufwrite8, bufwrite_sds};
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::vendor::json_builder::{json_boolean_new, json_double_new, json_integer_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsdup, sdsempty, sdsfree, sdsnew, sdsnewlen};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PostTable {
    pub version: F16Dot16,
    pub italic_angle: F16Dot16,
    pub underline_position: i16,
    pub underline_thickness: i16,
    pub is_fixed_pitch: u32,
    pub min_mem_type42: u32,
    pub max_mem_type42: u32,
    pub min_mem_type1: u32,
    pub max_mem_type1: u32,
    pub post_name_map: *mut GlyphOrder,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PostTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut PostTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut PostTable, *const PostTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut PostTable, *mut PostTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut PostTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut PostTable, PostTable) -> ()>,
    pub copy_replace: Option<unsafe extern "C" fn(*mut PostTable, PostTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut PostTable>,
    pub free: Option<unsafe extern "C" fn(*mut PostTable) -> ()>,
}
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
static STANDARD_MAC_NAMES: [&::core::ffi::CStr; 258] = [
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
unsafe extern "C" fn init_post(mut post: *mut PostTable) {
    memset(
        post as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PostTable>() as usize,
    );
    (*post).version = 0x30000 as ::core::ffi::c_int as F16Dot16;
}
#[inline]
unsafe extern "C" fn dispose_post(mut post: *mut PostTable) {
    if !(*post).post_name_map.is_null() {
        OTFCC_PKG_GLYPH_ORDER.free.expect("non-null function pointer")((*post).post_name_map);
    }
}
#[inline]
unsafe extern "C" fn table_post_dispose(mut x: *mut PostTable) {
    dispose_post(x);
}
#[inline]
unsafe extern "C" fn table_post_free(mut x: *mut PostTable) {
    if x.is_null() {
        return;
    }
    table_post_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_post_create() -> *mut PostTable {
    let mut x: *mut PostTable =
        malloc(::core::mem::size_of::<PostTable>() as usize) as *mut PostTable;
    table_post_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_post_init(mut x: *mut PostTable) {
    init_post(x);
}
#[inline]
unsafe extern "C" fn table_post_copy(mut dst: *mut PostTable, mut src: *const PostTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<PostTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_post_copy_replace(mut dst: *mut PostTable, src: PostTable) {
    table_post_dispose(dst);
    table_post_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_post_move(mut dst: *mut PostTable, mut src: *mut PostTable) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<PostTable>() as usize,
    );
    table_post_init(src);
}
#[inline]
unsafe extern "C" fn table_post_replace(mut dst: *mut PostTable, src: PostTable) {
    table_post_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<PostTable>() as usize,
    );
}
pub static I_TABLE_POST: PostTableElementInterface = {
    PostTableElementInterface {
        init: Some(table_post_init as unsafe extern "C" fn(*mut PostTable) -> ()),
        copy: Some(
            table_post_copy as unsafe extern "C" fn(*mut PostTable, *const PostTable) -> (),
        ),
        move_0: Some(
            table_post_move as unsafe extern "C" fn(*mut PostTable, *mut PostTable) -> (),
        ),
        dispose: Some(table_post_dispose as unsafe extern "C" fn(*mut PostTable) -> ()),
        replace: Some(
            table_post_replace as unsafe extern "C" fn(*mut PostTable, PostTable) -> (),
        ),
        copy_replace: Some(
            table_post_copy_replace as unsafe extern "C" fn(*mut PostTable, PostTable) -> (),
        ),
        create: Some(table_post_create),
        free: Some(table_post_free as unsafe extern "C" fn(*mut PostTable) -> ()),
    }
};
pub unsafe extern "C" fn otfcc_read_post(
    packet: Packet,
    mut _options: *const Options,
) -> *mut PostTable {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1886352244i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut post: *mut PostTable =
                        (
                            I_TABLE_POST.create.expect("non-null function pointer"))();
                    (*post).version = read_32s(data as *const u8) as F16Dot16;
                    (*post).italic_angle =
                        read_32u(data.offset(4 as ::core::ffi::c_int as isize) as *const u8)
                            as F16Dot16;
                    (*post).underline_position =
                        read_16u(data.offset(8 as ::core::ffi::c_int as isize) as *const u8)
                            as i16;
                    (*post).underline_thickness =
                        read_16u(data.offset(10 as ::core::ffi::c_int as isize) as *const u8)
                            as i16;
                    (*post).is_fixed_pitch =
                        read_32u(data.offset(12 as ::core::ffi::c_int as isize) as *const u8);
                    (*post).min_mem_type42 =
                        read_32u(data.offset(16 as ::core::ffi::c_int as isize) as *const u8);
                    (*post).max_mem_type42 =
                        read_32u(data.offset(20 as ::core::ffi::c_int as isize) as *const u8);
                    (*post).min_mem_type1 =
                        read_32u(data.offset(24 as ::core::ffi::c_int as isize) as *const u8);
                    (*post).max_mem_type1 =
                        read_32u(data.offset(28 as ::core::ffi::c_int as isize) as *const u8);
                    (*post).post_name_map = ::core::ptr::null_mut::<GlyphOrder>();
                    if (*post).version == 0x20000 as F16Dot16 {
                        let mut map: *mut GlyphOrder =
                            (
                                OTFCC_PKG_GLYPH_ORDER
                                    .create
                                    .expect("non-null function pointer"))();
                        let mut pending_names: [SdsRaw; 65536] =
                            [::core::ptr::null_mut::<::core::ffi::c_char>(); 65536];
                        memset(
                            &raw mut pending_names as *mut SdsRaw as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<[SdsRaw; 65536]>() as usize,
                        );
                        let mut number_glyphs: u16 = read_16u(
                            data.offset(32 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        let mut offset: u32 = (34 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * number_glyphs as ::core::ffi::c_int)
                            as u32;
                        let mut pending_name_index: u16 = 0 as u16;
                        while pending_name_index as ::core::ffi::c_int <= 0xffff as ::core::ffi::c_int
                            && offset < table.length
                        {
                            let mut len: u8 = *data.offset(offset as isize);
                            let mut s: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
                            pending_names[pending_name_index as usize] = s;
                            pending_name_index = (pending_name_index as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as u16;
                        }
                        let mut j: u16 = 0 as u16;
                        while (j as ::core::ffi::c_int) < number_glyphs as ::core::ffi::c_int {
                            let mut name_map: u16 =
                                read_16u(data.offset(34 as ::core::ffi::c_int as isize).offset(
                                    (2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                                ) as *const u8);
                            if name_map as ::core::ffi::c_int >= 258 as ::core::ffi::c_int {
                                OTFCC_PKG_GLYPH_ORDER
                                    .set_by_gid
                                    .expect("non-null function pointer")(
                                    map,
                                    j as GlyphId,
                                    sdsdup(
                                        pending_names[(name_map as ::core::ffi::c_int
                                            - 258 as ::core::ffi::c_int)
                                            as usize],
                                    ),
                                );
                            } else {
                                OTFCC_PKG_GLYPH_ORDER
                                    .set_by_gid
                                    .expect("non-null function pointer")(
                                    map,
                                    j as GlyphId,
                                    sdsnew(STANDARD_MAC_NAMES[name_map as usize].as_ptr()),
                                );
                            }
                            j = j.wrapping_add(1);
                        }
                        let mut j_0: u32 = 0 as u32;
                        while j_0 < pending_name_index as u32 {
                            sdsfree(pending_names[j_0 as usize]);
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
    return ::core::ptr::null_mut::<PostTable>();
}
pub unsafe extern "C" fn otfcc_dump_post(
    mut table: *const PostTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"post"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut post: *mut JsonValue = json_object_new(10 as usize);
        json_object_push(
            post,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            post,
            b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new(otfcc_from_fixed((*table).italic_angle) as i64),
        );
        json_object_push(
            post,
            b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).underline_position as i64),
        );
        json_object_push(
            post,
            b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).underline_thickness as i64),
        );
        json_object_push(
            post,
            b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*table).is_fixed_pitch as ::core::ffi::c_int),
        );
        json_object_push(
            post,
            b"minMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).min_mem_type42 as i64),
        );
        json_object_push(
            post,
            b"maxMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_mem_type42 as i64),
        );
        json_object_push(
            post,
            b"minMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).min_mem_type1 as i64),
        );
        json_object_push(
            post,
            b"maxMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_mem_type1 as i64),
        );
        json_object_push(
            root,
            b"post\0" as *const u8 as *const ::core::ffi::c_char,
            post,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_post(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut PostTable {
    let mut post: *mut PostTable = (
        I_TABLE_POST.create.expect("non-null function pointer"))();
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"post\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"post"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            if (*options).short_post {
                (*post).version = 0x30000 as ::core::ffi::c_int as F16Dot16;
            } else {
                (*post).version = otfcc_to_fixed(json_obj_getnum(
                    table,
                    b"version\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            }
            (*post).italic_angle = otfcc_to_fixed(json_obj_getnum(
                table,
                b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            (*post).underline_position = json_obj_getnum(
                table,
                b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            ) as i16;
            (*post).underline_thickness = json_obj_getnum(
                table,
                b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            ) as i16;
            (*post).is_fixed_pitch = json_obj_getbool(
                table,
                b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).min_mem_type42 = json_obj_getnum(
                table,
                b"minMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).max_mem_type42 = json_obj_getnum(
                table,
                b"maxMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).min_mem_type1 = json_obj_getnum(
                table,
                b"minMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).max_mem_type1 = json_obj_getnum(
                table,
                b"maxMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return post;
}
pub unsafe extern "C" fn otfcc_build_post(
    mut post: *const PostTable,
    mut glyphorder: *mut GlyphOrder,
    mut _options: *const Options,
) -> *mut Buffer {
    if post.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite32b(buf, (*post).version as u32);
    bufwrite32b(buf, (*post).italic_angle as u32);
    bufwrite16b(buf, (*post).underline_position as u16);
    bufwrite16b(buf, (*post).underline_thickness as u16);
    bufwrite32b(buf, (*post).is_fixed_pitch);
    bufwrite32b(buf, (*post).min_mem_type42);
    bufwrite32b(buf, (*post).max_mem_type42);
    bufwrite32b(buf, (*post).min_mem_type1);
    bufwrite32b(buf, (*post).max_mem_type1);
    if (*post).version == 0x20000 as F16Dot16 {
        bufwrite16b(
            buf,
            (if !(*glyphorder).by_name.is_null() {
                (*(*(*glyphorder).by_name).hh_name.tbl).num_items
            } else {
                0 as ::core::ffi::c_uint
            }) as u16,
        );
        let mut s: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
        let mut tmp: *mut GlyphOrderEntry = ::core::ptr::null_mut::<GlyphOrderEntry>();
        s = (*glyphorder).by_name;
        tmp = (if !(*glyphorder).by_name.is_null() {
            (*(*glyphorder).by_name).hh_name.next
        } else {
            NULL
        }) as *mut GlyphOrderEntry as *mut GlyphOrderEntry;
        while !s.is_null() {
            bufwrite16b(
                buf,
                (258 as ::core::ffi::c_int + (*s).gid as ::core::ffi::c_int) as u16,
            );
            s = tmp;
            tmp = (if !tmp.is_null() {
                (*tmp).hh_name.next
            } else {
                NULL
            }) as *mut GlyphOrderEntry as *mut GlyphOrderEntry;
        }
        s = (*glyphorder).by_name;
        tmp = (if !(*glyphorder).by_name.is_null() {
            (*(*glyphorder).by_name).hh_name.next
        } else {
            NULL
        }) as *mut GlyphOrderEntry as *mut GlyphOrderEntry;
        while !s.is_null() {
            bufwrite8(buf, sdslen((*s).name) as u8);
            bufwrite_sds(buf, (*s).name);
            s = tmp;
            tmp = (if !tmp.is_null() {
                (*tmp).hh_name.next
            } else {
                NULL
            }) as *mut GlyphOrderEntry as *mut GlyphOrderEntry;
        }
    }
    return buf;
}
