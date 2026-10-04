use crate::logger::ByteStr;
use crate::font::sfnt::Packet;
use otfcc_binary::Buffer;
use otfcc_json::BuiltValue;
use crate::support::options::Options;
use otfcc_json::ParsedValue;
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
// migration (only `font/model.rs` uses this table's vtable, and
// only through `.free`) -- so it's deleted rather than made unsound.
//
// Stage 7-2-c: `bytes` is now a `Vec<u8>` -- `length` (redundant with
// `.bytes.len()`) is dropped along with the manual `Drop` impl below;
// `Vec`'s own drop glue frees the buffer.
#[derive(Debug)]
pub struct FpgmPrepTable {
    pub tag: Vec<u8>,
    pub bytes: Vec<u8>,
}
// Unlike most of this batch, this was already memory-safe without a
// separate length guard: it copies the table's own `PacketPiece.data`
// verbatim (`length` bytes from a buffer that is always exactly `length`
// bytes long, per `font/sfnt.rs`'s invariant), so there is no
// declared-length-vs-actual-data mismatch to exploit -- and no field
// structure to parse, so there is nothing here for `FontReader` itself to
// add. `table.data` is already an owned `Vec<u8>` (`PacketPiece::data`),
// so this just clones it rather than routing through
// `__caryll_allocate_clean`/`copy_nonoverlapping` the way the raw-pointer
// version did.
pub fn read_fpgm_prep(packet: &Packet, tag: u32) -> Option<Box<FpgmPrepTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == tag)?;
    Some(Box::new(FpgmPrepTable {
        tag: Vec::new(),
        bytes: table.data.clone(),
    }))
}
pub fn table_dump_table_fpgm_prep(
    table: Option<&FpgmPrepTable>,
    root: &mut BuiltValue,
    options: &Options,
    tag: &[u8],
) {
    let Some(table) = table else {
        return;
    };
    let stage = crate::logger::stage(ByteStr(tag));
    let dumped = dump_ttinstr(&table.bytes, options);
    root.push_field(tag, dumped);
    stage.finish();
}
pub fn parse_fpgm_prep(
    root: &ParsedValue,
    tag: &[u8],
) -> Option<Box<FpgmPrepTable>> {
    let table = root.get(tag)?;
    let stage = crate::logger::stage(ByteStr(tag));
    let mut boxed = Box::new(FpgmPrepTable {
        tag: tag.to_vec(),
        bytes: Vec::new(),
    });
    parse_ttinstr(Some(table), |instrs| boxed.bytes = instrs, |_reason, _pos| {});
    stage.finish();
    Some(boxed)
}
pub fn build_fpgm_prep(table: Option<&FpgmPrepTable>) -> Option<Buffer> {
    let table = table?;
    let mut buf = Buffer::new();
    buf.write_bytes(&table.bytes);
    Some(buf)
}
