use crate::support::options::Options;

use crate::font::model::Font;
use crate::font::table_registry::DUMP_ORDER;
use crate::support::glyph_file_name::GlyphFileNames;
use otfcc_json::{BuiltValue, JsonSerializeOpts, JsonStreamWriter, json_serialize_ex};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

/// Dumps a consolidated font into the JSON value tree otfccdump prints.
pub fn serialize_to_json(font: &mut Font, options: &Options) -> BuiltValue {
    let mut tree = TreeSink {
        root: BuiltValue::new_object(48),
        open: Vec::new(),
    };
    dump_font(font, options, &mut tree);

    tree.root
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

/// The file in a `v2` dump directory that holds everything but the glyphs.
pub const SPLIT_MAIN_FILE: &str = "font.json";
/// The subdirectory of a `v2` dump directory that holds one file per glyph.
pub const SPLIT_GLYPH_DIR: &str = "glyf";

/// Writes a consolidated font as a `v2` dump into the directory `dir`, which
/// is created if needed: every glyph of `glyf` goes to a file of its own under
/// `glyf/`, and `font.json` holds the rest, with `glyf` mapping each glyph
/// name to its file, in glyph order. Each file is laid out with `opts`.
pub fn write_split_dir(
    font: &mut Font,
    options: &Options,
    dir: &Path,
    opts: JsonSerializeOpts,
) -> std::io::Result<()> {
    std::fs::create_dir_all(dir.join(SPLIT_GLYPH_DIR))?;
    let main = BufWriter::new(File::create(dir.join(SPLIT_MAIN_FILE))?);
    let mut sink = SplitSink {
        main: JsonStreamWriter::new(main, opts),
        dir: dir.to_path_buf(),
        opts,
        depth: 0,
        glyph_index: None,
        names: GlyphFileNames::new(),
        error: None,
    };
    sink.main.begin_object();
    dump_font(font, options, &mut sink);
    sink.main.end_object();
    if let Some(e) = sink.error {
        return Err(e);
    }
    sink.main.finish()?.flush()
}

/// A [`DumpSink`] that sends the members of the top-level `glyf` object to
/// files of their own and everything else to `font.json`.
struct SplitSink {
    main: JsonStreamWriter<BufWriter<File>>,
    dir: PathBuf,
    opts: JsonSerializeOpts,
    /// How many objects opened by `begin_field_object` are still open.
    depth: usize,
    /// While the top-level `glyf` object is open: its index, glyph name to
    /// file, in the order the glyphs come.
    glyph_index: Option<BuiltValue>,
    names: GlyphFileNames,
    /// The first error writing a glyph file, reported once the dump ends.
    error: Option<std::io::Error>,
}

impl SplitSink {
    fn write_glyph(&mut self, name: &[u8], value: &BuiltValue) -> String {
        let file = self.names.file_name(name);
        let mut bytes = json_serialize_ex(value, self.opts);
        bytes.push(b'\n');
        let path = self.dir.join(SPLIT_GLYPH_DIR).join(&file);
        if let Err(e) = std::fs::write(path, bytes)
            && self.error.is_none()
        {
            self.error = Some(e);
        }
        format!("{SPLIT_GLYPH_DIR}/{file}")
    }
}

impl DumpSink for SplitSink {
    fn field(&mut self, key: &[u8], value: BuiltValue) {
        if self.depth == 1 && self.glyph_index.is_some() {
            let file = self.write_glyph(key, &value);
            if let Some(index) = self.glyph_index.as_mut() {
                index.push_field(key, BuiltValue::Str(file.into_bytes()));
            }
        } else {
            self.main.field(key, value);
        }
    }
    fn begin_field_object(&mut self, key: &[u8]) {
        if self.depth == 0 && key == SPLIT_GLYPH_DIR.as_bytes() {
            self.glyph_index = Some(BuiltValue::new_object(0));
        } else {
            self.main.begin_field_object(key);
        }
        self.depth += 1;
    }
    fn end_object(&mut self) {
        self.depth -= 1;
        if self.depth == 0
            && let Some(index) = self.glyph_index.take()
        {
            self.main.field(SPLIT_GLYPH_DIR.as_bytes(), index);
        } else {
            self.main.end_object();
        }
    }
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
