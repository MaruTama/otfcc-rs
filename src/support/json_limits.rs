#![forbid(unsafe_code)]
//! One place for "this JSON has more members than the font format can count".
//!
//! Almost every count in an OpenType table is 16 bits: glyphs, contours,
//! points, components, records, lookups, palettes, layers, subtable entries.
//! A JSON array or object has no such limit, and the table writers store
//! `len() as u16`. A collection of 65,536 or more members therefore either
//! hangs a `u16` loop counter that can never reach its length (the 65,536
//! glyph and 65,536 reference hangs) or panics on an index derived from a
//! wrapped count (65,536 mark classes), or silently writes a table whose
//! count disagrees with its contents. Rather than guard each table, the JSON
//! reader asks [`find_oversized_collection`] once, before any table is parsed,
//! and rejects the document the same way it rejects any other JSON that
//! cannot become a font.
//!
//! This is deliberately a check on *shape*, not on any one table: no
//! collection inside a table with 16-bit counts can legitimately be bigger,
//! so a table added later is covered without anyone remembering to add a
//! limit for it. The exceptions are the tables whose size is a byte or entry
//! count with a 32-bit length in the font, listed in [`UNBOUNDED_TABLES`].

use otfcc_json::ParsedValue;

/// The most members a collection may have: what a `u16` count can hold.
pub const MAX_ENTRIES: usize = u16::MAX as usize;

/// Top-level tables that may legitimately exceed [`MAX_ENTRIES`]: `cmap`
/// numbers up to U+10FFFF code points, and `fpgm`, `prep` and `cvt_` are
/// sized by a 32-bit table length rather than a 16-bit count.
const UNBOUNDED_TABLES: &[&[u8]] = &[b"cmap", b"cmap_uvs", b"fpgm", b"prep", b"cvt_"];

/// Where an oversized collection was found.
#[derive(Debug, PartialEq, Eq)]
pub struct OversizedCollection {
    /// `/`-separated keys and array indices from the document root.
    pub path: Vec<u8>,
    pub len: usize,
}

/// The first array or object in `root` with more than [`MAX_ENTRIES`] members,
/// outside [`UNBOUNDED_TABLES`].
pub fn find_oversized_collection(root: &ParsedValue) -> Option<OversizedCollection> {
    let fields = root.as_object()?;
    for (key, value) in fields {
        let key = trim_nul(key);
        if UNBOUNDED_TABLES.contains(&key) {
            continue;
        }
        if let Some(mut found) = check(value) {
            prepend(&mut found.path, key);
            return Some(found);
        }
    }
    None
}

fn check(value: &ParsedValue) -> Option<OversizedCollection> {
    match value {
        ParsedValue::Array(items) => {
            if items.len() > MAX_ENTRIES {
                return Some(OversizedCollection { path: Vec::new(), len: items.len() });
            }
            for (i, item) in items.iter().enumerate() {
                if let Some(mut found) = check(item) {
                    prepend(&mut found.path, i.to_string().as_bytes());
                    return Some(found);
                }
            }
            None
        }
        ParsedValue::Object(fields) => {
            if fields.len() > MAX_ENTRIES {
                return Some(OversizedCollection { path: Vec::new(), len: fields.len() });
            }
            for (key, item) in fields {
                if let Some(mut found) = check(item) {
                    prepend(&mut found.path, trim_nul(key));
                    return Some(found);
                }
            }
            None
        }
        _ => None,
    }
}

fn prepend(path: &mut Vec<u8>, segment: &[u8]) {
    let mut joined = Vec::with_capacity(1 + segment.len() + path.len());
    joined.push(b'/');
    joined.extend_from_slice(segment);
    joined.append(path);
    *path = joined;
}

/// Object keys carry the parser's storage-only trailing NUL.
fn trim_nul(key: &[u8]) -> &[u8] {
    key.strip_suffix(&[0]).unwrap_or(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use otfcc_json::parse_json;

    fn find(json: &str) -> Option<OversizedCollection> {
        find_oversized_collection(&parse_json(json.as_bytes()).expect("test JSON parses"))
    }

    fn array_of(n: usize) -> String {
        format!("[{}]", vec!["0"; n].join(","))
    }

    #[test]
    #[cfg_attr(miri, ignore = "needs genuine 65,536-member collections")]
    fn collections_at_the_limit_are_fine_and_one_past_is_reported_with_its_path() {
        assert_eq!(find(&format!(r#"{{"name":{}}}"#, array_of(MAX_ENTRIES))), None);
        assert_eq!(
            find(&format!(r#"{{"name":{}}}"#, array_of(MAX_ENTRIES + 1))),
            Some(OversizedCollection { path: b"/name".to_vec(), len: MAX_ENTRIES + 1 })
        );
    }

    #[test]
    #[cfg_attr(miri, ignore = "needs genuine 65,536-member collections")]
    fn the_path_names_every_key_and_array_index_down_to_the_collection() {
        let json = format!(r#"{{"glyf":{{"a":{{"contours":[[],{}]}}}}}}"#, array_of(MAX_ENTRIES + 1));
        assert_eq!(
            find(&json),
            Some(OversizedCollection {
                path: b"/glyf/a/contours/1".to_vec(),
                len: MAX_ENTRIES + 1
            })
        );
    }

    #[test]
    #[cfg_attr(miri, ignore = "needs genuine 65,536-member collections")]
    fn objects_are_counted_by_member_not_just_arrays() {
        let members: Vec<String> = (0..=MAX_ENTRIES).map(|i| format!(r#""g{i}":0"#)).collect();
        let found = find(&format!(r#"{{"glyf":{{{}}}}}"#, members.join(","))).unwrap();
        assert_eq!(found.path, b"/glyf");
    }

    #[test]
    #[cfg_attr(miri, ignore = "needs genuine 65,536-member collections")]
    fn the_tables_sized_by_a_32_bit_length_may_be_larger() {
        for table in ["cmap", "cmap_uvs", "fpgm", "prep", "cvt_"] {
            assert_eq!(find(&format!(r#"{{"{table}":{}}}"#, array_of(MAX_ENTRIES + 1))), None);
        }
    }
}
