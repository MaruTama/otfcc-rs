use crate::logger::ByteStr;
use crate::font::sfnt::Packet;
use otfcc_binary::Buffer;
use otfcc_json::BuiltValue;
use crate::support::options::Options;
use otfcc_json::ParsedValue;
use crate::support::ttinstr::{dump_ttinstr, parse_ttinstr};

// `tag` is written on every construction path but never read back.
#[derive(Debug)]
pub struct FpgmPrepTable {
    pub tag: Vec<u8>,
    pub bytes: Vec<u8>,
}
// Copies the table's bytes verbatim; there is no structure to parse.
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
