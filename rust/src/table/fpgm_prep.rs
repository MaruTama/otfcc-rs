#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::font::caryll_sfnt::Packet;
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
use crate::support::ttinstr::{dump_ttinstr, parse_ttinstr};

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
//
// Stage 7-2-c: `bytes` is now a `Vec<u8>` -- `length` (redundant with
// `.bytes.len()`) is dropped along with the manual `Drop` impl below;
// `Vec`'s own drop glue frees the buffer.
pub struct FpgmPrepTable {
    pub tag: Vec<u8>,
    pub bytes: Vec<u8>,
}
// Unlike most of this batch, this was already memory-safe without a
// separate length guard: it copies the table's own `PacketPiece.data`
// verbatim (`length` bytes from a buffer that is always exactly `length`
// bytes long, per `font/caryll_sfnt.rs`'s invariant), so there is no
// declared-length-vs-actual-data mismatch to exploit -- and no field
// structure to parse, so there is nothing here for `FontReader` itself to
// add. `table.data` is already an owned `Vec<u8>` (`PacketPiece::data`),
// so this just clones it rather than routing through
// `__caryll_allocate_clean`/`copy_nonoverlapping` the way the raw-pointer
// version did.
pub fn otfcc_read_fpgm_prep(packet: &Packet, tag: u32) -> Option<Box<FpgmPrepTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == tag)?;
    Some(Box::new(FpgmPrepTable {
        tag: Vec::new(),
        bytes: table.data.clone(),
    }))
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn table_dump_table_fpgm_prep(
    table: Option<&FpgmPrepTable>,
    root: &mut BuiltValue,
    options: &Options,
    tag: *const ::core::ffi::c_char,
) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    logger_start_sds(&mut *options.logger.borrow_mut(), crate::bytesbuild!(tag));
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        root.push_field(
            ::core::ffi::CStr::from_ptr(tag).to_bytes(),
            dump_ttinstr(
                (*table).bytes.as_ptr() as *mut u8,
                (*table).bytes.len() as u32,
                options,
            ),
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn make_fpgm_prep_instr(mut _t: *mut ::core::ffi::c_void, instrs: Vec<u8>) {
    let t: *mut FpgmPrepTable = _t as *mut FpgmPrepTable;
    (*t).bytes = instrs;
}
pub unsafe fn wrong_fpgm_prep_instr(
    mut _t: *mut ::core::ffi::c_void,
    mut _reason: *mut ::core::ffi::c_char,
    mut _pos: i32,
) {
}
pub unsafe fn otfcc_parse_fpgm_prep(
    root: &ParsedValue,
    options: &Options,
    tag: *const ::core::ffi::c_char,
) -> Option<Box<FpgmPrepTable>> {
    let mut t: Option<Box<FpgmPrepTable>> = None;
    let key = unsafe { ::core::ffi::CStr::from_ptr(tag) }.to_bytes();
    let table = root.get(key);
    if let Some(table) = table {
        let table = table as *const ParsedValue;
        logger_start_sds(&mut *options.logger.borrow_mut(), crate::bytesbuild!(tag));
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut boxed = Box::new(FpgmPrepTable {
                tag: ::core::ffi::CStr::from_ptr(tag).to_bytes().to_vec(),
                bytes: Vec::new(),
            });
            parse_ttinstr(
                table,
                boxed.as_mut() as *mut FpgmPrepTable as *mut ::core::ffi::c_void,
                Some(make_fpgm_prep_instr as unsafe fn(*mut ::core::ffi::c_void, Vec<u8>) -> ()),
                Some(
                    wrong_fpgm_prep_instr
                        as unsafe fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_char,
                            i32,
                        ) -> (),
                ),
            );
            t = Some(boxed);
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
    }
    return t;
}
pub fn otfcc_build_fpgm_prep(table: Option<&FpgmPrepTable>) -> Option<Buffer> {
    let table = table?;
    let mut buf = Buffer::new();
    buf.write_bytes(&table.bytes);
    Some(buf)
}
