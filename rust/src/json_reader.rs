#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{strlen, strtol};

use crate::logger::{LOG_VL_NOTICE, LoggerType, logger_log_sds};
use crate::otf_reader::FontBuilder;
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_key_at,
    json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_str_bytes, json_type_of,
};

use crate::font::caryll_font::{Font, FontSubtype};
use crate::support::NULL;
use crate::support::glyph_order::{GlyphOrder, GlyphOrderEntry, GlyphOrderPass};
use crate::support::options::Options;
use crate::support::primitives::GlyphId;
use crate::vendor::json::JsonType;

use crate::font::caryll_font::otfcc_font_create;
use crate::table::_tsi::otfcc_parse_tsi;
use crate::table::base::otfcc_parse_base;
use crate::table::cff::otfcc_parse_cff;
use crate::table::cmap::otfcc_parse_cmap;
use crate::table::colr::otfcc_parse_colr;
use crate::table::cpal::otfcc_parse_cpal;
use crate::table::cvt::otfcc_parse_cvt;
use crate::table::fpgm_prep::otfcc_parse_fpgm_prep;
use crate::table::gasp::otfcc_parse_gasp;
use crate::table::gdef::otfcc_parse_gdef;
use crate::table::glyf::otfcc_parse_glyf;
use crate::table::head::otfcc_parse_head;
use crate::table::hhea::otfcc_parse_hhea;
use crate::table::maxp::otfcc_parse_maxp;
use crate::table::meta::parse::otfcc_parse_meta;
use crate::table::name::otfcc_parse_name;
use crate::table::os_2::otfcc_parse_os_2;
use crate::table::otl::parse::otfcc_parse_otl;
use crate::table::post::otfcc_parse_post;
use crate::table::svg::otfcc_parse_svg;
use crate::table::tsi5::otfcc_parse_tsi5;
use crate::table::vdmx::funcs::otfcc_parse_vdmx;
use crate::table::vhea::otfcc_parse_vhea;

#[inline]
unsafe fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
unsafe fn otfcc_decide_font_subtype_from_json(root: *const ParsedValue) -> FontSubtype {
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
// `name` is `Vec<u8>` now instead of `SdsRaw`: the duplicate-name path
// used to leave the old `SdsRaw` `name` deliberately un-freed (a
// pre-existing leak this migration didn't own until it reached this
// function directly), but that hazard is gone by construction -- an
// unused `Vec<u8>` just drops.
//
// Never a real FFI boundary -- internal call site only, same rationale
// as every other instance of this allow in the crate.
#[allow(improper_ctypes_definitions)]
unsafe fn set_order_by_name(
    go: *mut GlyphOrder,
    name: Vec<u8>,
    order_type: GlyphOrderPass,
    order_entry: u32,
) {
    match (*go).by_name.get(&name).copied() {
        None => {
            (*go).entries.push(GlyphOrderEntry {
                gid: -(1 as ::core::ffi::c_int) as GlyphId,
                name: name.clone(),
                order_type,
                order_entry,
            });
            let idx = (*go).entries.len() - 1;
            (*go).by_name.insert(name, idx);
        }
        Some(idx) => {
            let entry = &mut (&mut (*go).entries)[idx];
            if entry.order_type > order_type {
                entry.order_type = order_type;
                entry.order_entry = order_entry;
            }
        }
    }
}
unsafe fn order_glyphs(go: *mut GlyphOrder) {
    let mut idxs: Vec<usize> = (*go).by_name.values().copied().collect();
    idxs.sort_by(|&a, &b| {
        let ea = &(&(*go).entries)[a];
        let eb = &(&(*go).entries)[b];
        (ea.order_type, ea.order_entry).cmp(&(eb.order_type, eb.order_entry))
    });
    let mut gid: GlyphId = 0 as GlyphId;
    for &idx in idxs.iter() {
        (&mut (*go).entries)[idx].gid = gid;
        (*go).by_gid.insert(gid, idx);
        gid = (gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as GlyphId;
    }
}
// `name` is a borrowed `&[u8]` now, not `SdsRaw` -- this function only
// ever reads it for the `by_name` lookup, never stores it, so there was
// never any ownership to plumb through in the first place.
//
// Never a real FFI boundary -- internal call sites only, same rationale
// as every other instance of this allow in the crate.
#[allow(improper_ctypes_definitions)]
unsafe fn escalate_glyph_order_by_name(
    go: *mut GlyphOrder,
    name: &[u8],
    order_type: GlyphOrderPass,
    order_entry: u32,
) {
    if let Some(&idx) = (*go).by_name.get(name) {
        let entry = &mut (&mut (*go).entries)[idx];
        if entry.order_type > order_type {
            entry.order_type = order_type;
            entry.order_entry = order_entry;
        }
    }
}
unsafe fn place_order_entries_from_glyf(table: *const ParsedValue, go: *mut GlyphOrder) {
    let mut j: u32 = 0 as u32;
    while j < json_obj_len(table) as u32 {
        let gname: Vec<u8> = json_obj_key_bytes_at(table, j as u32);
        if gname.as_slice() == b".notdef" {
            set_order_by_name(go, gname, GlyphOrderPass::Notdef, 0 as u32);
        } else if gname.as_slice() == b".null" {
            set_order_by_name(go, gname, GlyphOrderPass::Notdef, 1 as u32);
        } else {
            set_order_by_name(go, gname, GlyphOrderPass::Glyf, j);
        }
        j = j.wrapping_add(1);
    }
}
unsafe fn place_order_entries_from_cmap(table: *const ParsedValue, go: *mut GlyphOrder) {
    let mut j: u32 = 0 as u32;
    while j < json_obj_len(table) as u32 {
        // Borrows `json_obj_key_at`'s pointer directly rather than an
        // owned `sds` copy -- every JSON object key is already
        // NUL-terminated in `ParsedValue`'s own storage, so `strlen` sees
        // the same length `sdslen` used to on the copy. Same reasoning as
        // `table/cmap.rs`'s `parse_unicode` (this function inlines the
        // identical U+XXXX-or-decimal parse).
        let unicode_str: *const ::core::ffi::c_char = json_obj_key_at(table, j as u32);
        let item: *const ParsedValue = json_obj_val_at(table, j as u32);
        let unicode: i32;
        if strlen(unicode_str) > 2 as usize
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
        if json_type_of(item) == JsonType::String
            && unicode > 0 as i32
            && unicode <= 0x10ffff as i32
        {
            let gname: Vec<u8> = json_str_bytes(item);
            escalate_glyph_order_by_name(go, &gname, GlyphOrderPass::Cmap, unicode as u32);
        }
        j = j.wrapping_add(1);
    }
}
unsafe fn place_order_entries_from_subtable(
    table: *const ParsedValue,
    go: *mut GlyphOrder,
    zero_only: bool,
) {
    let mut uplimit: u32 = json_arr_len(table);
    if uplimit >= 1 as u32 && zero_only as ::core::ffi::c_int != 0 {
        uplimit = 1 as u32;
    }
    let mut j: u32 = 0 as u32;
    while j < uplimit {
        let item: *const ParsedValue = json_arr_at(table, j as u32);
        if json_type_of(item) == JsonType::String {
            let gname: Vec<u8> = json_str_bytes(item);
            escalate_glyph_order_by_name(go, &gname, GlyphOrderPass::GlyphOrder, j);
        }
        j = j.wrapping_add(1);
    }
}
unsafe fn parse_glyph_order(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<GlyphOrder>> {
    // Built directly via `Box::new`, not `OTFCC_PKG_GLYPH_ORDER.create`
    // (`malloc`) + `Box::from_raw` -- see the matching note in
    // `consolidate.rs`'s `otfcc_consolidate_font`. `go` stays a raw-pointer
    // alias into `go_box` for the rest of this function (unchanged from
    // here down).
    let mut go_box: Box<GlyphOrder> = Box::new(GlyphOrder {
        entries: Vec::new(),
        by_gid: ::std::collections::BTreeMap::new(),
        by_name: ::std::collections::HashMap::new(),
    });
    let go: *mut GlyphOrder = go_box.as_mut() as *mut GlyphOrder;
    if json_type_of(root) != JsonType::Object {
        return Some(go_box);
    }
    let mut table: *const ParsedValue;
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
            let mut ignore_glyph_order: bool = options.ignore_glyph_order;
            if ignore_glyph_order as ::core::ffi::c_int != 0
                && !json_obj_get_type(
                    root,
                    b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                )
                .is_null()
            {
                logger_log_sds(
                    &mut *options.logger.borrow_mut(),
                    LOG_VL_NOTICE,
                    LoggerType::Info,
                    crate::bytesbuild!(b"OpenType SVG table detected. Glyph order is preserved.",),
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
        let options: &Options = &*(options as *const Options);
        let root: *const ParsedValue = _root as *const ParsedValue;
        let font: *mut Font = (otfcc_font_create)();
        if font.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        (*font).subtype = otfcc_decide_font_subtype_from_json(root);
        (*font).glyph_order = parse_glyph_order(root, options);
        (*font).glyf = otfcc_parse_glyf(
            root,
            (*font)
                .glyph_order
                .as_deref_mut()
                .map_or(::core::ptr::null_mut(), |g| g as *mut GlyphOrder),
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
        if !options.ignore_hints {
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
        if (*font).glyf.is_some() {
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
        (*font).tsi5 = otfcc_parse_tsi5(root);
        return font as *mut ::core::ffi::c_void;
    }
}
pub unsafe fn read_json(
    mut _root: *mut ::core::ffi::c_void,
    mut _index: u32,
    options: &Options,
) -> *mut Font {
    <JsonReader as FontBuilder>::read(
        _root,
        _index,
        options as *const Options as *const ::core::ffi::c_void,
    ) as *mut Font
}
