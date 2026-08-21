#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};


use crate::support::parsed_json::{ParsedValue, json_obj_get};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::font::caryll_sfnt::{Packet};
use crate::support::buffer::{bufnew, bufwrite_bytes};
use crate::support::ttinstr::{dump_ttinstr, parse_ttinstr};
use crate::support::built_json::{BuiltValue, json_object_push};

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
// Unlike most of this batch, this was already memory-safe without a
// separate length guard: it copies the table's own `PacketPiece.data`
// verbatim (`length` bytes from a buffer that is always exactly `length`
// bytes long, per `font/caryll_sfnt.rs`'s invariant), so there is no
// declared-length-vs-actual-data mismatch to exploit -- and no field
// structure to parse, so there is nothing here for `FontReader` itself to
// add. Still dropped `__fortable_*`/the raw pointer for consistency with
// the rest of this stage, matching `table/cvt.rs::otfcc_read_cvt`'s
// precedent -- not a safety fix.
pub unsafe fn otfcc_read_fpgm_prep(
    packet: &Packet,
    mut tag: u32,
) -> Option<Box<FpgmPrepTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == tag)?;
    let length = table.data.len() as u32;
    let bytes = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize).wrapping_mul(length as usize),
        22 as ::core::ffi::c_ulong,
    ) as *mut u8;
    if bytes.is_null() {
        return None;
    }
    ::core::ptr::copy_nonoverlapping(table.data.as_ptr(), bytes, length as usize);
    Some(Box::new(FpgmPrepTable { tag: Vec::new(), length, bytes }))
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn table_dump_table_fpgm_prep(
    table: Option<&FpgmPrepTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    logger_start_sds(
        (*options).logger,
        crate::bytesbuild!(tag),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        json_object_push(
            root,
            tag,
            dump_ttinstr((*table).bytes, (*table).length, options),
        );
        ___loggedstep_v = false;
        logger_finish((*options).logger);
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
pub unsafe fn otfcc_parse_fpgm_prep(
    mut root: *const ParsedValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> Option<Box<FpgmPrepTable>> {
    let mut t: Option<Box<FpgmPrepTable>> = None;
    let mut table: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    table = json_obj_get(root, tag);
    if !table.is_null() {
        logger_start_sds(
            (*options).logger,
            crate::bytesbuild!(tag),
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
            logger_finish(
                (*options).logger
            );
        }
    }
    return t;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_fpgm_prep(
    table: Option<&FpgmPrepTable>,
) -> *mut Buffer {
    let table = match table {
        Some(t) => t,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    bufwrite_bytes(buf, (*table).length as usize, (*table).bytes);
    return buf;
}
