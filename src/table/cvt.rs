use crate::font::sfnt::Packet;
use crate::support::base64::base64_decode;
use otfcc_binary::Buffer;
use otfcc_json::BuiltValue;
use otfcc_binary::FontReader;
use otfcc_json::ParsedValue;
use otfcc_json::JsonType;

#[derive(Debug)]
pub struct CvtTable {
    pub words: Vec<u16>,
}
// The word count is `length / 2` and the read loop is bounded by it, so no
// read can go past the end.
pub fn read_cvt(packet: &Packet, tag: u32) -> Option<Box<CvtTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == tag)?;
    let table_length = (table.data.len() / 2) as u32;
    let mut words: Vec<u16> = Vec::with_capacity(table_length as usize);
    let mut r = FontReader::new(&table.data);
    for _ in 0..table_length as usize {
        words.push(r.u16().expect(
            "table_length is derived from data.len(), so table_length u16 reads always fit",
        ));
    }
    Some(Box::new(CvtTable { words }))
}
pub fn dump_cvt(table: Option<&CvtTable>, root: &mut BuiltValue, tag: &[u8]) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    let stage = crate::logger::stage("cvt");
    {
        let mut arr = BuiltValue::new_array(table.words.len());
        for &w in &table.words {
            arr.push_item(BuiltValue::Int(w as i64));
        }
        root.push_field(tag, arr);
        stage.finish();
    }
}
pub fn parse_cvt(root: &ParsedValue, tag: &[u8]) -> Option<Box<CvtTable>> {
    let key = tag;
    if let Some(items) = root
        .get_typed(key, JsonType::Array)
        .and_then(ParsedValue::as_array)
    {
        let stage = crate::logger::stage("cvt");
        let mut words: Vec<u16> = Vec::with_capacity(items.len());
        for record in items {
            words.push(match record {
                ParsedValue::Int(i) => *i as u16,
                ParsedValue::Double(d) => *d as u16,
                _ => 0_u16,
            });
        }
        stage.finish();
        return Some(Box::new(CvtTable { words }));
    }
    if let Some(bytes) = root
        .get_typed(key, JsonType::String)
        .and_then(ParsedValue::as_str_bytes)
    {
        let stage = crate::logger::stage("cvt");
        let raw = base64_decode(bytes).unwrap_or_default();
        let table_length = raw.len() / 2;
        let mut words: Vec<u16> = Vec::with_capacity(table_length);
        for j in 0..table_length {
            words.push(u16::from_be_bytes([raw[2 * j], raw[2 * j + 1]]));
        }
        stage.finish();
        return Some(Box::new(CvtTable { words }));
    }
    None
}
pub fn build_cvt(table: Option<&CvtTable>) -> Option<Buffer> {
    let table = table?;
    let mut buf = Buffer::new();
    for &w in &table.words {
        buf.write_u16be(w);
    }
    Some(buf)
}
