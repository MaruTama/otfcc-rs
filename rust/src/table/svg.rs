#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, strcmp};
use crate::support::json_funcs::{json_arr_at, json_arr_len, json_obj_get_type, json_obj_getint, json_obj_getsds, json_obj_getstr_share};
use crate::support::binio::{read_16u, read_32u};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::bk::bkblock::{bk_new_block_from_buffer_copy};
use crate::bk::bkgraph::{bk_build_block};
use crate::support::base64::{base64_encode};
use crate::support::buffer::{buffree, bufnew, bufwrite_buf, bufwrite_bytes};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_string_new, json_string_new_length};
use crate::vendor::sds::{sdsempty, sdsfree, sdslen};

#[repr(C)]
pub struct SvgAssignment {
    pub start: GlyphId,
    pub end: GlyphId,
    pub document: *mut Buffer,
}
// C由来の時点で素のベクタ形（ラッパー構造体なし）。要素の `document: *mut Buffer`
// は所有物だが `Handle`/`sds` ではなく `Buffer`（`buffree`）で、この形の Vec 化は
// 初めて（`svg_assignment_dup` が `bufnew`+`bufwrite_buf` で本物のディープコピー
// を行う——`ColrTable`/`TsiTable` と同じく `.clone()` には頼れない）。
//
// Stage 6-4 "Box化": `Font.svg` becomes `Option<Vec<SvgAssignment>>` (not
// `Option<Box<Vec<...>>>` -- `Vec` already owns its own heap buffer, a
// second `Box` layer would be pure overhead). For a plain `Vec<T>`'s own
// `Drop` to correctly free every element's `document`, `T` needs a real
// `Drop` impl -- `Copy`/`Clone` dropped (mutually exclusive with `Drop`;
// nothing in this file relied on either: duplication always went through
// `svg_assignment_dup`'s deep copy, never a derive).
pub type SvgTable = Vec<SvgAssignment>;
impl Drop for SvgAssignment {
    fn drop(&mut self) {
        unsafe {
            buffree(self.document);
        }
    }
}
#[inline]
unsafe fn svg_assignment_empty() -> SvgAssignment {
    SvgAssignment {
        start: 0,
        end: 0,
        document: ::core::ptr::null_mut::<Buffer>(),
    }
}
/// 本物のディープコピー（`document` を指す `Buffer` を複製する）。
/// `document`(`*mut Buffer`)を素朴に`.clone()`すると単なるポインタのビット
/// コピーになりエイリアスしてしまうため、`otfcc_build_svg`のソート済み
/// コピー生成には使えない——明示的にディープコピーする。
unsafe fn svg_assignment_dup(src: &SvgAssignment) -> SvgAssignment {
    let mut dst: SvgAssignment = svg_assignment_empty();
    dst.start = src.start;
    dst.end = src.end;
    dst.document = bufnew();
    bufwrite_buf(dst.document, src.document);
    dst
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_read_svg(
    packet: Packet,
    mut _options: *const Options,
) -> Option<SvgTable> {
    let mut offset_to_svg_doc_index: u32 = 0;
    let mut num_entries: u16 = 0;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_SVG {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    if !(table.length < 10 as u32) {
                        offset_to_svg_doc_index =
                            read_32u(table.data.offset(2 as ::core::ffi::c_int as isize));
                        if !(table.length < offset_to_svg_doc_index.wrapping_add(2 as u32)) {
                            num_entries = read_16u(table.data.offset(offset_to_svg_doc_index as isize));
                            if !(table.length
                                < offset_to_svg_doc_index
                                    .wrapping_add(2 as u32)
                                    .wrapping_add(
                                        (12 as ::core::ffi::c_int
                                            * num_entries as ::core::ffi::c_int)
                                            as u32,
                                    ))
                            {
                                let mut svg: SvgTable = Vec::new();
                                let mut j: GlyphId = 0 as GlyphId;
                                while (j as ::core::ffi::c_int) < num_entries as ::core::ffi::c_int {
                                    let mut record: FontFilePointer = table
                                        .data
                                        .offset(offset_to_svg_doc_index as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (12 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                as isize,
                                        );
                                    let mut asg: SvgAssignment = svg_assignment_empty();
                                    asg.start = read_16u(record as *const u8) as GlyphId;
                                    asg.end =
                                        read_16u(record.offset(2 as ::core::ffi::c_int as isize)
                                            as *const u8)
                                            as GlyphId;
                                    let mut docstart: u32 =
                                        read_32u(record.offset(4 as ::core::ffi::c_int as isize)
                                            as *const u8);
                                    let mut doclen: u32 =
                                        read_32u(record.offset(8 as ::core::ffi::c_int as isize)
                                            as *const u8);
                                    if offset_to_svg_doc_index
                                        .wrapping_add(docstart)
                                        .wrapping_add(doclen)
                                        <= table.length
                                    {
                                        asg.document = bufnew();
                                        bufwrite_bytes(
                                            asg.document,
                                            doclen as usize,
                                            table
                                                .data
                                                .offset(offset_to_svg_doc_index as isize)
                                                .offset(docstart as isize),
                                        );
                                    } else {
                                        asg.document = bufnew();
                                    }
                                    svg.push(asg);
                                    j = j.wrapping_add(1);
                                }
                                return Some(svg);
                            }
                        }
                    }
                    // No `svg` to dispose here: every path that constructs
                    // one (deep inside the nested guards above) returns
                    // immediately afterward, so this branch is only ever
                    // reached before any allocation happens.
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return None;
}
unsafe extern "C" fn can_use_plain_format(mut buf: *const Buffer) -> bool {
    return (*buf).size > 4 as usize
        && *(*buf).data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '<' as i32
        && *(*buf).data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 's' as i32
        && *(*buf).data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'v' as i32
        && *(*buf).data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'g' as i32
        || (*buf).size > 5 as usize
            && *(*buf).data.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '<' as i32
            && *(*buf).data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '?' as i32
            && *(*buf).data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'x' as i32
            && *(*buf).data.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'm' as i32
            && *(*buf).data.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'l' as i32;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_dump_svg(
    svg: Option<&SvgTable>,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    let svg = match svg {
        Some(s) => s,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"SVG "),
    );
    let entries: &Vec<SvgAssignment> = svg;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _svg: *mut JsonValue = json_array_new(entries.len());
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < entries.len() {
            let a: &SvgAssignment = &entries[__caryll_index];
            while keep != 0 {
                let mut _a: *mut JsonValue = json_object_new(4 as usize);
                json_object_push(
                    _a,
                    b"start\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*a).start as i64),
                );
                json_object_push(
                    _a,
                    b"end\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*a).end as i64),
                );
                if can_use_plain_format((*a).document) {
                    json_object_push(
                        _a,
                        b"format\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new(b"plain\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                    json_object_push(
                        _a,
                        b"document\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            (*(*a).document).size as ::core::ffi::c_uint,
                            (*(*a).document).data as *mut ::core::ffi::c_char,
                        ),
                    );
                } else {
                    let mut len: usize = 0 as usize;
                    let mut buf: *mut u8 =
                        base64_encode((*(*a).document).data, (*(*a).document).size, &raw mut len);
                    json_object_push(
                        _a,
                        b"format\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new(b"base64\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                    json_object_push(
                        _a,
                        b"document\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new_length(
                            len as ::core::ffi::c_uint,
                            buf as *mut ::core::ffi::c_char,
                        ),
                    );
                    free(buf as *mut ::core::ffi::c_void);
                    buf = ::core::ptr::null_mut::<u8>();
                }
                json_array_push(_svg, _a);
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        json_object_push(
            root,
            b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
            _svg,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_parse_svg(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> Option<SvgTable> {
    let mut _svg: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    _svg = json_obj_get_type(
        root,
        b"SVG_\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if _svg.is_null() {
        return None;
    }
    let mut svg: SvgTable = Vec::new();
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"SVG "),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut j: GlyphId = 0 as GlyphId;
        while (j as ::core::ffi::c_uint) < json_arr_len(_svg) {
            let mut _a: *mut JsonValue = json_arr_at(_svg, j as u32);
            if !(_a.is_null()
                || (*_a).type_0 != JsonType::Object)
            {
                let mut format: *const ::core::ffi::c_char = json_obj_getstr_share(
                    _a,
                    b"format\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let mut doc: SdsRaw =
                    json_obj_getsds(_a, b"document\0" as *const u8 as *const ::core::ffi::c_char);
                if !(format.is_null() || doc.is_null()) {
                    let mut asg: SvgAssignment = svg_assignment_empty();
                    asg.start =
                        json_obj_getint(_a, b"start\0" as *const u8 as *const ::core::ffi::c_char)
                            as GlyphId;
                    asg.end =
                        json_obj_getint(_a, b"end\0" as *const u8 as *const ::core::ffi::c_char)
                            as GlyphId;
                    if strcmp(
                        format,
                        b"plain\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        asg.document = bufnew();
                        bufwrite_bytes(asg.document, sdslen(doc), doc as *mut u8);
                        sdsfree(doc);
                    } else {
                        asg.document = bufnew();
                        let mut len: usize = 0 as usize;
                        let mut buf: *mut u8 =
                            base64_encode(doc as *mut u8, sdslen(doc), &raw mut len);
                        bufwrite_bytes(asg.document, len, buf);
                        free(buf as *mut ::core::ffi::c_void);
                        buf = ::core::ptr::null_mut::<u8>();
                        sdsfree(doc);
                    }
                    svg.push(asg);
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return Some(svg);
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_svg(
    _svg: Option<&SvgTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let _svg = match _svg {
        Some(s) if !s.is_empty() => s,
        _ => return ::core::ptr::null_mut::<Buffer>(),
    };
    // `TABLE_I_SVG.copy` の代わりに各要素を `svg_assignment_dup` で明示的に
    // ディープコピー（`ColrTable`/`TsiTable` の前例どおり `.clone()` は不可）。
    let mut svg: SvgTable = _svg.iter().map(|a| svg_assignment_dup(a)).collect();
    svg.sort_by(|a, b| a.start.cmp(&b.start));
    let mut major: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (svg.len()) as u32)]);
    let mut __caryll_index: usize = 0 as usize;
    let mut keep: usize = 1 as usize;
    while keep != 0 && __caryll_index < svg.len() {
        let a: &SvgAssignment = &svg[__caryll_index];
        while keep != 0 {
            bk_push(major, &[bk_int(BkCellType::B16, ((*a).start as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*a).end as ::core::ffi::c_int) as u32), bk_ptr(BkCellType::P32, bk_new_block_from_buffer_copy((*a).document)), bk_int(BkCellType::B32, ((*(*a).document).size) as u32)]);
            keep = (keep == 0) as ::core::ffi::c_int as usize;
        }
        keep = (keep == 0) as ::core::ffi::c_int as usize;
        __caryll_index = __caryll_index.wrapping_add(1);
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 0 as u32), bk_ptr(BkCellType::P32, major), bk_int(BkCellType::B32, 0 as u32)]);
    // `svg` drops naturally at the end of this scope -- each `SvgAssignment`
    // has a real `Drop` impl now, so the explicit `table_svg_dispose` call
    // this used to need is gone along with that function.
    return bk_build_block(root);
}
