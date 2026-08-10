#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, strcmp, strtol};





use crate::support::json_funcs::{json_obj_get_type};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::handle::{sds_to_vec};
use crate::otf_reader::FontBuilder;
use crate::logger::{LoggerType, LOG_VL_NOTICE, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::font::caryll_font::{FontSubtype, Font, IFontBuilder};
use crate::support::{NULL};
use crate::support::glyph_order::{GlyphOrderPass, GlyphOrder, GlyphOrderEntry};






use crate::font::caryll_font::{OTFCC_I_FONT};
use crate::table::base::{otfcc_parse_base};
use crate::table::cff::{otfcc_parse_cff};
use crate::table::colr::{otfcc_parse_colr};
use crate::table::cpal::{otfcc_parse_cpal};
use crate::table::gdef::{otfcc_parse_gdef};
use crate::table::os_2::{otfcc_parse_os_2};
use crate::table::svg::{otfcc_parse_svg};
use crate::table::tsi5::{otfcc_parse_tsi5};
use crate::table::_tsi::{otfcc_parse_tsi};
use crate::table::cmap::{otfcc_parse_cmap};
use crate::table::cvt::{otfcc_parse_cvt};
use crate::table::fpgm_prep::{otfcc_parse_fpgm_prep};
use crate::table::gasp::{otfcc_parse_gasp};
use crate::table::glyf::{otfcc_parse_glyf};
use crate::table::head::{otfcc_parse_head};
use crate::table::hhea::{otfcc_parse_hhea};
use crate::table::maxp::{otfcc_parse_maxp};
use crate::table::meta::parse::{otfcc_parse_meta};
use crate::table::name::{otfcc_parse_name};
use crate::table::otl::parse::{otfcc_parse_otl};
use crate::table::post::{otfcc_parse_post};
use crate::table::vdmx::funcs::{otfcc_parse_vdmx};
use crate::table::vhea::{otfcc_parse_vhea};
use crate::vendor::sds::{sdsempty, sdsfree, sdslen, sdsnewlen};




#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
unsafe extern "C" fn otfcc_decide_font_subtype_from_json(
    mut root: *const JsonValue,
) -> FontSubtype {
    if !json_obj_get_type(
        root,
        b"CFF_\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    )
    .is_null()
    {
        return FontSubtype::Cff;
    } else {
        return FontSubtype::Ttf;
    };
}
unsafe extern "C" fn set_order_by_name(
    mut go: *mut GlyphOrder,
    mut name: SdsRaw,
    mut order_type: GlyphOrderPass,
    mut order_entry: u32,
) {
    let name_bytes = sds_to_vec(name);
    match (*go).by_name.get(&name_bytes) {
        None => {
            let mut s: *mut GlyphOrderEntry = __caryll_allocate_clean(
                ::core::mem::size_of::<GlyphOrderEntry>() as usize,
                21 as ::core::ffi::c_ulong,
            ) as *mut GlyphOrderEntry;
            (*s).gid = -(1 as ::core::ffi::c_int) as GlyphId;
            (*s).name = name_bytes.clone();
            (*s).order_type = order_type;
            (*s).order_entry = order_entry;
            (*go).by_name.insert(name_bytes, s);
            // The original moved `name` into `(*s).name` here (no separate
            // free needed); now that the bytes are copied instead, the
            // now-redundant `sds` needs an explicit free.
            sdsfree(name);
        }
        Some(&s) => {
            if (*s).order_type > order_type {
                (*s).order_type = order_type;
                (*s).order_entry = order_entry;
            }
            // `name` is deliberately left un-freed here, matching the
            // original -- a pre-existing leak on the duplicate-name path
            // (none of this function's callers free it either), preserved
            // rather than fixed.
        }
    }
}
unsafe extern "C" fn order_glyphs(mut go: *mut GlyphOrder) {
    let mut entries: Vec<*mut GlyphOrderEntry> = (*go).by_name.values().copied().collect();
    entries.sort_by(|&a, &b| {
        ((*a).order_type, (*a).order_entry).cmp(&((*b).order_type, (*b).order_entry))
    });
    let mut gid: GlyphId = 0 as GlyphId;
    for &current in entries.iter() {
        (*current).gid = gid;
        (*go).by_gid.insert(gid, current);
        gid = (gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
    }
}
unsafe extern "C" fn escalate_glyph_order_by_name(
    mut go: *mut GlyphOrder,
    mut name: SdsRaw,
    mut order_type: GlyphOrderPass,
    mut order_entry: u32,
) {
    let name_bytes = std::slice::from_raw_parts(name as *const u8, sdslen(name)).to_vec();
    if let Some(&s) = (*go).by_name.get(&name_bytes) {
        if (*s).order_type > order_type {
            (*s).order_type = order_type;
            (*s).order_entry = order_entry;
        }
    }
}
unsafe extern "C" fn place_order_entries_from_glyf(
    mut table: *mut JsonValue,
    mut go: *mut GlyphOrder,
) {
    let mut j: u32 = 0 as u32;
    while j < (*table).u.object.length as u32 {
        let mut gname: SdsRaw = sdsnewlen(
            (*(*table).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*table).u.object.values.offset(j as isize)).name_length as usize,
        );
        if strcmp(
            gname as *const ::core::ffi::c_char,
            b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            set_order_by_name(
                go,
                gname,
                GlyphOrderPass::Notdef,
                0 as u32,
            );
        } else if strcmp(
            gname as *const ::core::ffi::c_char,
            b".null\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            set_order_by_name(
                go,
                gname,
                GlyphOrderPass::Notdef,
                1 as u32,
            );
        } else {
            set_order_by_name(go, gname, GlyphOrderPass::Glyf, j);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn place_order_entries_from_cmap(
    mut table: *mut JsonValue,
    mut go: *mut GlyphOrder,
) {
    let mut j: u32 = 0 as u32;
    while j < (*table).u.object.length as u32 {
        let mut unicode_str: SdsRaw = sdsnewlen(
            (*(*table).u.object.values.offset(j as isize)).name as *const ::core::ffi::c_void,
            (*(*table).u.object.values.offset(j as isize)).name_length as usize,
        );
        let mut item: *mut JsonValue =
            (*(*table).u.object.values.offset(j as isize)).value as *mut JsonValue;
        let mut unicode: i32 = 0;
        if sdslen(unicode_str) > 2 as usize
            && *unicode_str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'U' as i32
            && *unicode_str.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '+' as i32
        {
            unicode = strtol(
                unicode_str.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                16 as ::core::ffi::c_int,
            ) as i32;
        } else {
            unicode = atoi(unicode_str as *const ::core::ffi::c_char) as i32;
        }
        sdsfree(unicode_str);
        if (*item).type_0 == JsonType::String
            && unicode > 0 as i32
            && unicode <= 0x10ffff as i32
        {
            let mut gname: SdsRaw = sdsnewlen(
                (*item).u.string.ptr as *const ::core::ffi::c_void,
                (*item).u.string.length as usize,
            );
            escalate_glyph_order_by_name(
                go,
                gname,
                GlyphOrderPass::Cmap,
                unicode as u32,
            );
            sdsfree(gname);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn place_order_entries_from_subtable(
    mut table: *mut JsonValue,
    mut go: *mut GlyphOrder,
    mut zero_only: bool,
) {
    let mut uplimit: u32 = (*table).u.array.length as u32;
    if uplimit >= 1 as u32 && zero_only as ::core::ffi::c_int != 0 {
        uplimit = 1 as u32;
    }
    let mut j: u32 = 0 as u32;
    while j < uplimit {
        let mut item: *mut JsonValue =
            *(*table).u.array.values.offset(j as isize) as *mut JsonValue;
        if (*item).type_0 == JsonType::String
        {
            let mut gname: SdsRaw = sdsnewlen(
                (*item).u.string.ptr as *const ::core::ffi::c_void,
                (*item).u.string.length as usize,
            );
            escalate_glyph_order_by_name(
                go,
                gname,
                GlyphOrderPass::GlyphOrder,
                j,
            );
            sdsfree(gname);
        }
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn parse_glyph_order(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> Option<Box<GlyphOrder>> {
    // Built directly via `Box::new`, not `OTFCC_PKG_GLYPH_ORDER.create`
    // (`malloc`) + `Box::from_raw` -- see the matching note in
    // `consolidate.rs`'s `otfcc_consolidate_font`. `go` stays a raw-pointer
    // alias into `go_box` for the rest of this function (unchanged from
    // here down).
    let mut go_box: Box<GlyphOrder> = Box::new(GlyphOrder {
        by_gid: ::std::collections::BTreeMap::new(),
        by_name: ::std::collections::HashMap::new(),
    });
    let go: *mut GlyphOrder = go_box.as_mut() as *mut GlyphOrder;
    if (*root).type_0 != JsonType::Object
    {
        return Some(go_box);
    }
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        place_order_entries_from_glyf(table, go);
        table = json_obj_get_type(
            root,
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Object,
        );
        if !table.is_null() {
            place_order_entries_from_cmap(table, go);
        }
        table = json_obj_get_type(
            root,
            b"glyph_order\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
        );
        if !table.is_null() {
            let mut ignore_glyph_order: bool = (*options).ignore_glyph_order;
            if ignore_glyph_order as ::core::ffi::c_int != 0
                && !json_obj_get_type(
                    root,
                    b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                )
                .is_null()
            {
                (*(*options).logger)
                    .log_sds
                    .expect("non-null function pointer")(
                    (*options).logger as *mut ILogger,
                    LOG_VL_NOTICE,
                    LoggerType::Info,
                    crate::sdsbuild!(
                        sdsempty(),
                        b"OpenType SVG table detected. Glyph order is preserved.",
                    ),
                );
                ignore_glyph_order = false;
            }
            place_order_entries_from_subtable(table, go, ignore_glyph_order);
        }
    }
    order_glyphs(go);
    return Some(go_box);
}
struct JsonReader;
impl FontBuilder for JsonReader {
    unsafe fn read(
    mut _root: *mut ::core::ffi::c_void,
    mut _index: u32,
    options: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let options = options as *const Options;
    let mut root: *const JsonValue = _root as *mut JsonValue;
    let mut font: *mut Font = (
        OTFCC_I_FONT.create.expect("non-null function pointer"))();
    if font.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*font).subtype = otfcc_decide_font_subtype_from_json(root);
    (*font).glyph_order = parse_glyph_order(root, options);
    (*font).glyf = otfcc_parse_glyf(
        root,
        (*font).glyph_order.as_deref_mut().map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder),
        options,
    );
    (*font).cff = otfcc_parse_cff(root, options);
    (*font).head = otfcc_parse_head(root, options);
    (*font).hhea = otfcc_parse_hhea(root, options);
    (*font).os_2 = otfcc_parse_os_2(root, options);
    (*font).maxp = otfcc_parse_maxp(root, options);
    (*font).post = otfcc_parse_post(root, options);
    (*font).name = otfcc_parse_name(root, options);
    (*font).meta = otfcc_parse_meta(root, options);
    (*font).cmap = otfcc_parse_cmap(root, options);
    if !(*options).ignore_hints {
        (*font).fpgm = otfcc_parse_fpgm_prep(
            root,
            options,
            b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).prep = otfcc_parse_fpgm_prep(
            root,
            options,
            b"prep\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).cvt_ = otfcc_parse_cvt(
            root,
            options,
            b"cvt_\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).gasp = otfcc_parse_gasp(root, options);
    }
    (*font).vdmx = otfcc_parse_vdmx(root, options);
    (*font).vhea = otfcc_parse_vhea(root, options);
    if !(*font).glyf.is_null() {
        (*font).gsub = otfcc_parse_otl(
            root,
            options,
            b"GSUB\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).gpos = otfcc_parse_otl(
            root,
            options,
            b"GPOS\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*font).gdef = otfcc_parse_gdef(root, options);
    }
    (*font).base = otfcc_parse_base(root, options);
    (*font).cpal = otfcc_parse_cpal(root, options);
    (*font).colr = otfcc_parse_colr(root, options);
    (*font).svg = otfcc_parse_svg(root, options);
    (*font).tsi_01 = otfcc_parse_tsi(
        root,
        options,
        b"TSI_01\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*font).tsi_23 = otfcc_parse_tsi(
        root,
        options,
        b"TSI_23\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*font).tsi5 = otfcc_parse_tsi5(root, options);
    return font as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn read_json(
    mut _root: *mut ::core::ffi::c_void,
    mut _index: u32,
    mut options: *const Options,
) -> *mut Font {
    <JsonReader as FontBuilder>::read(_root, _index, options as *const ::core::ffi::c_void)
        as *mut Font
}
#[inline]
unsafe extern "C" fn free_reader(mut self_0: *mut IFontBuilder) {
    free(self_0 as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_new_json_reader() -> *mut IFontBuilder {
    let mut reader: *mut IFontBuilder = ::core::ptr::null_mut::<IFontBuilder>();
    reader = __caryll_allocate_clean(
        ::core::mem::size_of::<IFontBuilder>() as usize,
        177 as ::core::ffi::c_ulong,
    ) as *mut IFontBuilder;
    (*reader).read = Some(
        read_json
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
