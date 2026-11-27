use crate::font::sfnt::Packet;
use crate::logger::ByteStr;
use crate::support::options::Options;
use crate::support::ttinstr::{dump_ttinstr, parse_ttinstr};
use otfcc_binary::Buffer;
use otfcc_json::BuiltValue;
use otfcc_json::ParsedValue;

// `tag` is written on every construction path but never read back.
/// The raw TrueType instructions of `fpgm` or `prep`. `tag` is only set when
/// the table is parsed from JSON; `read_fpgm_prep` leaves it empty.
#[derive(Debug)]
pub struct FpgmPrepTable {
    pub tag: Vec<u8>,
    pub bytes: Vec<u8>,
}
// Copies the table's bytes verbatim; there is no structure to parse.
/// Takes the bytes of the table with `tag` from `packet`, or `None` if the
/// font has no such table. The bytes are not checked.
pub fn read_fpgm_prep(packet: &Packet, tag: u32) -> Option<Box<FpgmPrepTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == tag)?;
    Some(Box::new(FpgmPrepTable {
        tag: Vec::new(),
        bytes: table.data.clone(),
    }))
}
/// Dumps the instructions under the key `tag` in `root`, using the
/// instruction dump format in `support::ttinstr`; does nothing for `None`.
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
/// Reads the instructions under `tag` in `root`, or `None` if the key is
/// absent. Instructions that cannot be parsed leave the table empty, with a
/// warning that says why and where.
pub fn parse_fpgm_prep(root: &ParsedValue, tag: &[u8]) -> Option<Box<FpgmPrepTable>> {
    let table = root.get(tag)?;
    let stage = crate::logger::stage(ByteStr(tag));
    let mut boxed = Box::new(FpgmPrepTable {
        tag: tag.to_vec(),
        bytes: Vec::new(),
    });
    parse_ttinstr(
        Some(table),
        |instrs| boxed.bytes = instrs,
        |reason, pos| {
            tracing::warn!(
                "[OTFCC] TrueType instructions parse error : {}, at {} in {}; the table is written empty.\n",
                ByteStr(reason),
                pos,
                ByteStr(tag)
            );
        },
    );
    stage.finish();
    Some(boxed)
}
/// Writes the stored instruction bytes unchanged, or `None` for no table.
pub fn build_fpgm_prep(table: Option<&FpgmPrepTable>) -> Option<Buffer> {
    let table = table?;
    let mut buf = Buffer::new();
    buf.write_bytes(&table.bytes);
    Some(buf)
}
