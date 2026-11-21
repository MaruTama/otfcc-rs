//! File names for glyphs in the `v2` dump format, where each glyph is written
//! to a file of its own.
//!
//! The names follow the idea of UFO's user-name-to-file-name algorithm: they
//! must survive file systems that ignore letter case (macOS, Windows), so an
//! upper-case letter is followed by `_` (`A` becomes `A_`, `a` stays `a`).
//! Only ASCII letters, digits, `-` and `_` and inner `.` are kept; any other
//! byte (including every byte of a non-ASCII name) becomes `_`. A leading
//! `.` would hide the file, and a part named like a Windows device (`con`,
//! `aux`, `com1`, ...) cannot be opened there, so both get a `_` prefix. The
//! glyph's real name is kept in the index in `font.json`, so a file name only
//! has to be unique, not reversible.

use std::collections::HashSet;

/// The longest file name stem kept, in bytes, leaving room for a
/// de-duplication counter and `.json` within common 255-byte limits.
const MAX_STEM_LEN: usize = 200;

/// The Windows device names that cannot be used as a file name part.
const RESERVED: &[&str] = &[
    "con", "prn", "aux", "clock$", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7",
    "com8", "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

/// Hands out a distinct file name for each glyph of one font.
#[derive(Debug, Default)]
pub struct GlyphFileNames {
    /// The names handed out so far, lower-cased, so that names differing
    /// only in case also count as taken.
    used: HashSet<String>,
}

impl GlyphFileNames {
    pub fn new() -> Self {
        Self::default()
    }

    /// A file name (with `.json`) for the glyph `name`, different from every
    /// name this has returned before, ignoring case.
    pub fn file_name(&mut self, name: &[u8]) -> String {
        let stem = stem_of(name);
        let mut candidate = stem.clone();
        let mut counter = 1u32;
        while !self.used.insert(candidate.to_ascii_lowercase()) {
            counter += 1;
            candidate = format!("{stem}~{counter}");
        }
        candidate + ".json"
    }
}

fn stem_of(name: &[u8]) -> String {
    let mut stem = String::with_capacity(name.len() + 4);
    for (i, &b) in name.iter().enumerate() {
        match b {
            b'A'..=b'Z' => {
                stem.push(b as char);
                stem.push('_');
            }
            b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' => stem.push(b as char),
            b'.' if i > 0 => stem.push('.'),
            _ => stem.push('_'),
        }
    }
    if stem.is_empty() {
        stem.push('_');
    }
    let reserved = stem
        .split('.')
        .any(|part| RESERVED.contains(&part.to_ascii_lowercase().as_str()));
    if reserved {
        stem.insert(0, '_');
    }
    // Every character is ASCII, so any byte index is a character boundary.
    stem.truncate(MAX_STEM_LEN);
    stem
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(glyphs: &[&[u8]]) -> Vec<String> {
        let mut n = GlyphFileNames::new();
        glyphs.iter().map(|g| n.file_name(g)).collect()
    }

    #[test]
    fn upper_case_letters_are_marked_so_case_insensitive_systems_keep_them_apart() {
        assert_eq!(
            names(&[b"A", b"a", b"Aring"]),
            ["A_.json", "a.json", "A_ring.json"]
        );
    }

    #[test]
    fn a_leading_dot_and_unsafe_bytes_become_underscores() {
        assert_eq!(
            names(&[b".notdef", b"a/b", b"x:y", "é".as_bytes(), b"uni0041.alt"]),
            [
                "_notdef.json",
                "a_b.json",
                "x_y.json",
                "__.json",
                "uni0041.alt.json"
            ]
        );
    }

    #[test]
    fn windows_device_names_get_a_prefix() {
        assert_eq!(
            names(&[b"con", b"aux.alt", b"conx"]),
            ["_con.json", "_aux.alt.json", "conx.json"]
        );
    }

    #[test]
    fn names_that_would_collide_get_a_counter() {
        // `a/b` and `a:b` both map to `a_b`; `A_` and `A` both to `A_`.
        assert_eq!(
            names(&[b"a/b", b"a:b", b"A", b"A_", b""]),
            ["a_b.json", "a_b~2.json", "A_.json", "A__.json", "_.json"]
        );
    }

    #[test]
    fn long_names_are_cut() {
        let long = vec![b'a'; 400];
        assert_eq!(names(&[&long])[0].len(), MAX_STEM_LEN + ".json".len());
    }
}
