use crate::support::options::Options;

use crate::font::model::Font;
use crate::font::table_registry::DUMP_ORDER;
use otfcc_json::{BuiltValue, JsonStreamWriter};
use std::io::Write;

/// Dumps a consolidated font into the JSON value tree otfccdump prints.
///
/// Was a `FontSerializer` impl on a zero-sized `JsonSerializer` marker
/// struct plus a casting wrapper; see `otf_reader::read_otf` for why that
/// trait is gone. Returning the `BuiltValue` itself rather than a
/// `BuiltValue::into_raw` pointer drops the last reason that bridge
/// existed on this path.
pub fn serialize_to_json(font: &mut Font, options: &Options) -> BuiltValue {
    let mut tree = TreeSink {
        root: BuiltValue::new_object(48),
        open: Vec::new(),
    };
    dump_font(font, options, &mut tree);
    let root = tree.root;
    return root;
}

/// Writes a consolidated font as JSON straight into `out`, one member of
/// the root object at a time, so that no member's tree outlives its being
/// written. The bytes are the ones [`serialize_to_json`] and then
/// `json_serialize_ex` produce, without ever holding the whole tree or the
/// whole text. Call [`JsonStreamWriter::finish`] afterwards for the first
/// write error, if any.
pub fn stream_json<W: Write>(font: &mut Font, options: &Options, out: &mut JsonStreamWriter<W>) {
    out.begin_object();
    dump_font(font, options, out);
    out.end_object();
}

fn dump_font(font: &mut Font, options: &Options, sink: &mut dyn DumpSink) {
    for table in DUMP_ORDER {
        table.dump_to(font, sink, options);
    }
}

/// Where a table's dump goes, member by member: into a tree
/// ([`serialize_to_json`]) or straight out as text ([`stream_json`]).
pub trait DumpSink {
    /// Adds `key: value` to the innermost open object.
    fn field(&mut self, key: &[u8], value: BuiltValue);
    /// Adds `key` with an empty object as its value, and opens that object
    /// for the following members until [`end_object`](Self::end_object).
    fn begin_field_object(&mut self, key: &[u8]);
    /// Closes the object [`begin_field_object`](Self::begin_field_object)
    /// opened last.
    fn end_object(&mut self);
}

struct TreeSink {
    root: BuiltValue,
    /// Objects opened by `begin_field_object`, outermost first, each with
    /// its key in the object around it.
    open: Vec<(Vec<u8>, BuiltValue)>,
}

impl DumpSink for TreeSink {
    fn field(&mut self, key: &[u8], value: BuiltValue) {
        match self.open.last_mut() {
            Some((_, object)) => object.push_field(key, value),
            None => self.root.push_field(key, value),
        }
    }
    fn begin_field_object(&mut self, key: &[u8]) {
        self.open.push((key.to_vec(), BuiltValue::new_object(0)));
    }
    fn end_object(&mut self) {
        let (key, object) = self.open.pop().expect("end_object without an open object");
        self.field(&key, object);
    }
}

impl<W: Write> DumpSink for JsonStreamWriter<W> {
    fn field(&mut self, key: &[u8], value: BuiltValue) {
        JsonStreamWriter::field(self, key, value);
    }
    fn begin_field_object(&mut self, key: &[u8]) {
        JsonStreamWriter::begin_field_object(self, key);
    }
    fn end_object(&mut self) {
        JsonStreamWriter::end_object(self);
    }
}
