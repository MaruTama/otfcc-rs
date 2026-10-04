//! `otfccdump` writes its JSON as it dumps it (`stream_json`), while the
//! library's `serialize_to_json` builds the tree that benches, fuzz targets
//! and FFI-side tests use. Both go through each table's `dump_to`, and the
//! tables that stream (`glyf`, `cmap`) do so member by member; this checks
//! that the two paths give the same bytes on every payload font.
mod support;

use otfcc_json::{
    JSON_SERIALIZE_MODE_MULTILINE, JSON_SERIALIZE_MODE_PACKED, JsonSerializeOpts, JsonStreamWriter,
    json_serialize_ex,
};
use otfcc_rust::consolidate::consolidate_font;
use otfcc_rust::font::model::Font;
use otfcc_rust::font::sfnt::read_sfnt;
use otfcc_rust::json_writer::{serialize_to_json, stream_json};
use otfcc_rust::otf_reader::read_otf;
use otfcc_rust::support::options::Options;

fn read_font(path: &std::path::Path, options: &Options) -> Box<Font> {
    let sfnt = read_sfnt(path).unwrap_or_else(|| panic!("cannot read {}", path.display()));
    let mut font = read_otf(&sfnt, 0, options).unwrap_or_else(|| panic!("cannot parse {}", path.display()));
    consolidate_font(&mut font, options);
    font
}

#[test]
fn streamed_dump_matches_the_built_tree() {
    let mut fonts: Vec<_> = std::fs::read_dir(support::repo_root().join("tests/payload"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "ttf" || x == "otf"))
        // 65,535 glyphs and a million cmap entries take most of a minute
        // here for nothing the other fonts don't already cover.
        .filter(|p| !p.ends_with("FDArrayTest65535.otf"))
        .collect();
    fonts.sort();
    assert!(fonts.len() >= 10, "payload fonts not found");
    for path in &fonts {
        for (decimal_cmap, ignore_glyph_order) in [(false, false), (true, true)] {
            let options = Options {
                decimal_cmap,
                ignore_glyph_order,
                ..Options::default()
            };
            for mode in [JSON_SERIALIZE_MODE_PACKED, JSON_SERIALIZE_MODE_MULTILINE] {
                let opts = JsonSerializeOpts { mode, opts: 0, indent_size: 4 };
                let tree = serialize_to_json(&mut read_font(path, &options), &options);
                let mut writer = JsonStreamWriter::new(Vec::new(), opts);
                stream_json(&mut read_font(path, &options), &options, &mut writer);
                assert!(
                    writer.finish().unwrap() == json_serialize_ex(&tree, opts),
                    "{} (decimal_cmap {decimal_cmap}, ignore_glyph_order {ignore_glyph_order}, mode {mode})",
                    path.display()
                );
            }
        }
    }
}
