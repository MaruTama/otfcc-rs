#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, memcpy};


use crate::support::parsed_json::{ParsedValue, json_obj_get};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer};
use crate::vendor::json::JsonValue;
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::buffer::{bufnew, bufwrite_bytes};
use crate::support::ttinstr::{dump_ttinstr, parse_ttinstr};
use crate::vendor::json_builder::{json_object_push};
use crate::vendor::sds::{sdsempty};

// `tag` is written on every construction path (read: unconditionally
// null/empty; parse: `sdsnew(tag)`, now `CStr::from_ptr(tag).to_bytes()`)
// but never actually read back anywhere in this file or its callers --
// confirmed by grep before converting. Kept as a real field regardless
// (removing it outright would be a scope-creeping cleanup riding along
// with a type conversion); `Vec<u8>` replaces the raw `sds`, which forces
// `Copy` off this struct. `.copy` (`table_fpgm_prep_copy`, a raw memcpy)
// was already dead -- confirmed via the same "grep the call sites,
// walk up if the caller itself is unreached" check used throughout this
// migration (only `font/caryll_font.rs` uses this table's vtable, and
// only through `.free`) -- so it's deleted rather than made unsound.
#[repr(C)]
pub struct FpgmPrepTable {
    pub tag: Vec<u8>,
    pub length: u32,
    pub bytes: *mut u8,
}
// Stage 6-4 "Box化": `bytes` is the only allocation this struct owns, same
// shape as `LtshTable`/`VorgTable`/`CvtTable`. The entire vtable is
// deleted: `.copy` was already confirmed dead (see the comment above),
// and grepping confirms only `.free` was ever called from outside this
// file (from `caryll_font.rs`'s table disposal, for both the `fpgm` and
// `prep` fields, which share this type).
impl Drop for FpgmPrepTable {
    fn drop(&mut self) {
        unsafe {
            if !self.bytes.is_null() {
                free(self.bytes as *mut ::core::ffi::c_void);
                self.bytes = ::core::ptr::null_mut::<u8>();
            }
        }
    }
}
pub unsafe extern "C" fn otfcc_read_fpgm_prep(
    packet: Packet,
    mut _options: *const Options,
    mut tag: u32,
) -> Option<Box<FpgmPrepTable>> {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == tag {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    let bytes = __caryll_allocate_clean(
                        (::core::mem::size_of::<u8>() as usize)
                            .wrapping_mul(length as usize),
                        22 as ::core::ffi::c_ulong,
                    ) as *mut u8;
                    if !bytes.is_null() {
                        memcpy(
                            bytes as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            length as usize,
                        );
                        return Some(Box::new(FpgmPrepTable { tag: Vec::new(), length, bytes }));
                    }
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
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn table_dump_table_fpgm_prep(
    table: Option<&FpgmPrepTable>,
    mut root: *mut JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), tag),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        json_object_push(
            root,
            tag,
            dump_ttinstr((*table).bytes, (*table).length, options),
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn make_fpgm_prep_instr(
    mut _t: *mut ::core::ffi::c_void,
    mut instrs: *mut u8,
    mut length: u32,
) {
    let mut t: *mut FpgmPrepTable = _t as *mut FpgmPrepTable;
    (*t).length = length;
    (*t).bytes = instrs;
}
pub unsafe extern "C" fn wrong_fpgm_prep_instr(
    mut _t: *mut ::core::ffi::c_void,
    mut _reason: *mut ::core::ffi::c_char,
    mut _pos: ::core::ffi::c_int,
) {
}
pub unsafe extern "C" fn otfcc_parse_fpgm_prep(
    mut root: *const ParsedValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> Option<Box<FpgmPrepTable>> {
    let mut t: Option<Box<FpgmPrepTable>> = None;
    let mut table: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    table = json_obj_get(root, tag);
    if !table.is_null() {
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), tag),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut boxed = Box::new(FpgmPrepTable {
                tag: ::core::ffi::CStr::from_ptr(tag).to_bytes().to_vec(),
                length: 0,
                bytes: ::core::ptr::null_mut::<u8>(),
            });
            parse_ttinstr(
                table,
                boxed.as_mut() as *mut FpgmPrepTable as *mut ::core::ffi::c_void,
                Some(
                    make_fpgm_prep_instr
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut u8,
                            u32,
                        ) -> (),
                ),
                Some(
                    wrong_fpgm_prep_instr
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
            );
            t = Some(boxed);
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return t;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_fpgm_prep(
    table: Option<&FpgmPrepTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let table = match table {
        Some(t) => t,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    bufwrite_bytes(buf, (*table).length as usize, (*table).bytes);
    return buf;
}
