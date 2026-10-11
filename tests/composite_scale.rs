//! A composite glyph whose component is scaled by the same factor on both
//! axes (`a == d`, `b == c == 0`) builds and reads back like any other.
//!
//! Issue #4: Mengshen-pinyin-font works around "`a` and `d` equal makes the
//! glyph vanish" by adding 0.001 to `d`. The uniform-scale case is written
//! with the single-factor `WE_HAVE_A_SCALE` flag, so it takes a different
//! path from `a != d` (`WE_HAVE_AN_X_AND_Y_SCALE`); this pins that both paths
//! produce the same component with the same bounding box.

use otfcc_json::parse_json;
use otfcc_rust::consolidate::consolidate_font;
use otfcc_rust::json_reader::read_json;
use otfcc_rust::otf_writer::serialize_to_otf;
use otfcc_rust::support::options::Options;

const WE_HAVE_A_SCALE: u16 = 0x0008;
const WE_HAVE_AN_X_AND_Y_SCALE: u16 = 0x0040;

fn be16(b: &[u8], at: usize) -> u16 {
    u16::from_be_bytes([b[at], b[at + 1]])
}

/// Builds a font with a triangle glyph `base` and a composite `ref` that
/// places `base` with the given scale, and returns the `ref` glyph's bytes.
fn build_composite(a: f64, d: f64) -> Vec<u8> {
    let json = format!(
        r#"{{
            "head": {{"unitsPerEm": 1000}},
            "glyf": {{
                ".notdef": {{"advanceWidth": 500}},
                "base": {{"advanceWidth": 600, "contours": [[
                    {{"x": 0, "y": 0, "on": true}},
                    {{"x": 400, "y": 0, "on": true}},
                    {{"x": 400, "y": 800, "on": true}}
                ]]}},
                "ref": {{"advanceWidth": 600, "references": [
                    {{"glyph": "base", "x": 10, "y": 20, "a": {a}, "b": 0, "c": 0, "d": {d}}}
                ]}}
            }},
            "glyph_order": [".notdef", "base", "ref"]
        }}"#
    );
    let options = Options::default();
    let mut root = parse_json(json.as_bytes()).expect("the test JSON should parse");
    let mut font = read_json(&mut root, &options).expect("the test JSON should read as a font");
    consolidate_font(&mut font, &options);
    let sfnt = serialize_to_otf(&mut font, &options).data;

    // Find the glyf and loca tables in the table directory.
    // Offset table: version(4), numTables(2), ...; then 16-byte records of
    // tag(4), checksum(4), offset(4), length(4).
    let num_tables = be16(&sfnt, 4) as usize;
    assert_eq!(&sfnt[0..4], &[0, 1, 0, 0], "a TrueType-flavored sfnt");
    let table = |tag: &[u8; 4]| -> (usize, usize) {
        (0..num_tables)
            .map(|i| 12 + 16 * i)
            .find(|&e| &sfnt[e..e + 4] == tag)
            .map(|e| {
                let off = u32::from_be_bytes(sfnt[e + 8..e + 12].try_into().unwrap()) as usize;
                let len = u32::from_be_bytes(sfnt[e + 12..e + 16].try_into().unwrap()) as usize;
                (off, len)
            })
            .unwrap_or_else(|| panic!("no {} table", String::from_utf8_lossy(tag)))
    };
    let (glyf, _) = table(b"glyf");
    let (loca, _) = table(b"loca");
    // loca holds 16-bit offsets (in units of 2) or 32-bit ones, as head's
    // indexToLocFormat (at byte 50) says. Glyph 2 is `ref`.
    let (head, _) = table(b"head");
    let (start, end) = if be16(&sfnt, head + 50) == 0 {
        (be16(&sfnt, loca + 4) as usize * 2, be16(&sfnt, loca + 6) as usize * 2)
    } else {
        (
            u32::from_be_bytes(sfnt[loca + 8..loca + 12].try_into().unwrap()) as usize,
            u32::from_be_bytes(sfnt[loca + 12..loca + 16].try_into().unwrap()) as usize,
        )
    };
    assert!(end > start, "the composite glyph must not be empty (loca {start}..{end}, tables {num_tables})");
    sfnt[glyf + start..glyf + end].to_vec()
}

/// The composite's header: contour count -1, bounding box, then the first
/// component's flags and glyph index.
fn header(g: &[u8]) -> (i16, [i16; 4], u16, u16) {
    let s = |at| be16(g, at) as i16;
    (s(0), [s(2), s(4), s(6), s(8)], be16(g, 10), be16(g, 12))
}

#[test]
fn equal_scale_on_both_axes_is_a_single_scale_component() {
    let g = build_composite(0.5, 0.5);
    let (contours, bbox, flags, glyph) = header(&g);
    assert_eq!(contours, -1);
    assert_eq!(glyph, 1, "the component is the `base` glyph");
    assert_ne!(flags & WE_HAVE_A_SCALE, 0);
    assert_eq!(flags & WE_HAVE_AN_X_AND_Y_SCALE, 0);
    // base is 0..400 x 0..800, scaled by 0.5 and moved by (10, 20).
    assert_eq!(bbox, [10, 20, 210, 420]);
    // After the 2-byte arguments: one F2Dot14 factor, 0.5 = 0x2000.
    assert_eq!(be16(&g, 14 + 2), 0x2000);
}

#[test]
fn slightly_different_scales_are_an_x_and_y_scale_component() {
    let g = build_composite(0.5, 0.501);
    let (contours, bbox, flags, glyph) = header(&g);
    assert_eq!(contours, -1);
    assert_eq!(glyph, 1);
    assert_ne!(flags & WE_HAVE_AN_X_AND_Y_SCALE, 0);
    assert_eq!(flags & WE_HAVE_A_SCALE, 0);
    assert_eq!(bbox[0..2], [10, 20]);
    assert_eq!(bbox[2], 210);
}
