use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct PacketPiece {
    pub tag: u32,
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: Vec<u8>,
}
#[derive(Debug)]
pub struct Packet {
    pub sfnt_version: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub pieces: Vec<PacketPiece>,
}
#[derive(Debug)]
pub struct SplineFontContainer {
    pub header_tag: u32,
    pub count: u32,
    pub offsets: Vec<u32>,
    pub packets: Vec<Packet>,
}
// Reads every member font's table directory and table data. `false` on any
// I/O failure (a short read, or a seek past the end): the file is truncated
// or malformed, and `read_sfnt` then fails as a whole.
fn read_packets<R: Read + Seek>(font: &mut SplineFontContainer, file: &mut R) -> bool {
    // Table offsets and lengths come from the file, so each table is checked
    // against the file's real length before anything is allocated for it.
    let Ok(total_len) = file.seek(SeekFrom::End(0)) else {
        return false;
    };
    for count in 0..font.count {
        let offset = font.offsets[count as usize];
        if file.seek(SeekFrom::Start(offset as u64)).is_err() {
            return false;
        }
        let Some(sfnt_version) = get32u(file) else {
            return false;
        };
        let Some(num_tables) = get16u(file) else {
            return false;
        };
        let Some(search_range) = get16u(file) else {
            return false;
        };
        let Some(entry_selector) = get16u(file) else {
            return false;
        };
        let Some(range_shift) = get16u(file) else {
            return false;
        };
        {
            let packet = &mut font.packets[count as usize];
            packet.sfnt_version = sfnt_version;
            packet.num_tables = num_tables;
            packet.search_range = search_range;
            packet.entry_selector = entry_selector;
            packet.range_shift = range_shift;
            for _ in 0..packet.num_tables as u32 {
                let Some(tag) = get32u(file) else {
                    return false;
                };
                let Some(check_sum) = get32u(file) else {
                    return false;
                };
                let Some(offset) = get32u(file) else {
                    return false;
                };
                let Some(length) = get32u(file) else {
                    return false;
                };
                if offset as u64 + length as u64 > total_len {
                    return false;
                }
                packet.pieces.push(PacketPiece {
                    tag,
                    check_sum,
                    offset,
                    length,
                    data: vec![0u8; length as usize],
                });
            }
        }
        {
            let packet = &mut font.packets[count as usize];
            // Bounded by this member's own table count: TTC members can have
            // different numbers of tables.
            for i_0 in 0..packet.pieces.len() as u32 {
                let piece = &mut packet.pieces[i_0 as usize];
                if file.seek(SeekFrom::Start(piece.offset as u64)).is_err() {
                    return false;
                }
                if file.read_exact(&mut piece.data).is_err() {
                    return false;
                }
            }
        }
    }
    true
}
// Reads the header/directory fields; `read_sfnt` (below) owns
// allocating and tearing down `font` around this call. Split out so a
// truncated-file failure partway through -- signalled the same way
// `read_packets` does, by returning `false` -- can be handled once,
// in one place, instead of duplicating the "free `font`, return null"
// cleanup at every read site.
fn read_sfnt_body<R: Read + Seek>(font: &mut SplineFontContainer, file: &mut R) -> bool {
    let Some(header_tag) = get32u(file) else {
        return false;
    };
    font.header_tag = header_tag;
    match font.header_tag {
        crate::tag::SFNT_VERSION_OTTO
        | crate::tag::SFNT_VERSION_TRUE_TYPE
        | crate::tag::SFNT_VERSION_MAC_TRUE
        | crate::tag::SFNT_VERSION_MAC_TYPE1 => {
            font.count = 1;
            font.offsets = vec![0];
            font.packets = (0..font.count)
                .map(|_| Packet {
                    sfnt_version: 0,
                    num_tables: 0,
                    search_range: 0,
                    entry_selector: 0,
                    range_shift: 0,
                    pieces: Vec::new(),
                })
                .collect();
            read_packets(font, file)
        }
        crate::tag::SFNT_TTC_TAG => {
            let Some(_ttc_version) = get32u(file) else {
                return false;
            };
            let Some(count) = get32u(file) else {
                return false;
            };
            // `numFonts` comes from the file. Each member needs a 4-byte
            // offset after it, so it cannot exceed the 4-byte words left;
            // checking that first avoids a huge allocation.
            let Ok(current_pos) = file.stream_position() else {
                return false;
            };
            let Ok(total_len) = file.seek(SeekFrom::End(0)) else {
                return false;
            };
            if file.seek(SeekFrom::Start(current_pos)).is_err() {
                return false;
            }
            if count as u64 > total_len.saturating_sub(current_pos) / 4 {
                return false;
            }
            font.count = count;
            font.offsets = vec![0; font.count as usize];
            font.packets = (0..font.count)
                .map(|_| Packet {
                    sfnt_version: 0,
                    num_tables: 0,
                    search_range: 0,
                    entry_selector: 0,
                    range_shift: 0,
                    pieces: Vec::new(),
                })
                .collect();
            let offsets: &mut Vec<u32> = &mut font.offsets;
            for i in 0..offsets.len() as u32 {
                let Some(v) = get32u(file) else {
                    return false;
                };
                offsets[i as usize] = v;
            }
            read_packets(font, file)
        }
        _ => {
            font.count = 0;
            font.offsets = Vec::new();
            font.packets = Vec::new();
            true
        }
    }
}
/// Opens and reads an SFNT (or TTC) file by path, returning `None` on any
/// failure -- the file doesn't exist, isn't readable, or is truncated or
/// malformed.
pub fn read_sfnt(path: &std::path::Path) -> Option<SplineFontContainer> {
    let mut file = std::fs::File::open(path).ok()?;
    read_sfnt_from_reader(&mut file)
}
/// [`read_sfnt`] for bytes already in memory (the `otf_parse` fuzz target
/// passes a `std::io::Cursor` over its input).
pub fn read_sfnt_from_reader<R: Read + Seek>(file: &mut R) -> Option<SplineFontContainer> {
    let mut font = SplineFontContainer {
        header_tag: 0,
        count: 0,
        offsets: Vec::new(),
        packets: Vec::new(),
    };
    read_sfnt_body(&mut font, file).then_some(font)
}
// `None` on a short read (EOF partway through, i.e. a truncated file).
// `read_exact` reports that as an `Err` on its own -- no separate
// byte-count check needed the way `fread`'s return value did.
fn get16u<R: Read>(file: &mut R) -> Option<u16> {
    let mut buf = [0u8; 2];
    file.read_exact(&mut buf).ok()?;
    Some(u16::from_be_bytes(buf))
}
fn get32u<R: Read>(file: &mut R) -> Option<u32> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf).ok()?;
    Some(u32::from_be_bytes(buf))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_file(bytes: &[u8]) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "otfcc-caryll-sfnt-test-{:?}-{}",
            std::thread::current().id(),
            bytes.len()
        ));
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(bytes).unwrap();
        path
    }

    #[test]
    #[cfg_attr(miri, ignore = "calls std::fs::File::open on a real path, unsupported under Miri's default isolation")]
    fn nonexistent_path_returns_none() {
        assert!(read_sfnt(std::path::Path::new("/nonexistent/otfcc-test-path")).is_none());
    }

    // A table whose declared length runs past the truncated file's actual
    // end must fail the read, not be silently zero-padded.
    #[test]
    #[cfg_attr(miri, ignore = "writes/opens a real temp file, unsupported under Miri's default isolation")]
    fn table_length_past_truncated_file_end_fails_instead_of_zero_padding() {
        // A minimal one-table SFNT: header (12 bytes) + one 16-byte
        // directory entry declaring the table's length as 8 bytes, but
        // only 4 of those 8 bytes are actually present in the file.
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&crate::tag::SFNT_VERSION_TRUE_TYPE.to_be_bytes());
        bytes.extend_from_slice(&1u16.to_be_bytes()); // num_tables
        bytes.extend_from_slice(&0u16.to_be_bytes()); // search_range
        bytes.extend_from_slice(&0u16.to_be_bytes()); // entry_selector
        bytes.extend_from_slice(&0u16.to_be_bytes()); // range_shift
        let table_offset = 12 + 16u32;
        bytes.extend_from_slice(b"TEST"); // tag
        bytes.extend_from_slice(&0u32.to_be_bytes()); // check_sum
        bytes.extend_from_slice(&table_offset.to_be_bytes()); // offset
        bytes.extend_from_slice(&8u32.to_be_bytes()); // length (8, but...)
        bytes.extend_from_slice(&[0xAA; 4]); // ...only 4 bytes follow
        let path = write_temp_file(&bytes);
        assert!(read_sfnt(&path).is_none());
        let _ = std::fs::remove_file(&path);
    }

    // A table's declared length must be checked against the file's actual
    // size before it sizes an allocation, or a tiny file could request a
    // multi-gigabyte one. This declares a length one byte short of
    // u32::MAX in a file a few dozen bytes long; if the length-vs-file-size
    // check regressed, this test would hang or OOM instead of failing
    // promptly.
    #[test]
    #[cfg_attr(miri, ignore = "writes/opens a real temp file, unsupported under Miri's default isolation")]
    fn table_length_far_past_file_end_fails_without_allocating_it() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&crate::tag::SFNT_VERSION_TRUE_TYPE.to_be_bytes());
        bytes.extend_from_slice(&1u16.to_be_bytes()); // num_tables
        bytes.extend_from_slice(&0u16.to_be_bytes()); // search_range
        bytes.extend_from_slice(&0u16.to_be_bytes()); // entry_selector
        bytes.extend_from_slice(&0u16.to_be_bytes()); // range_shift
        let table_offset = 12 + 16u32;
        bytes.extend_from_slice(b"TEST"); // tag
        bytes.extend_from_slice(&0u32.to_be_bytes()); // check_sum
        bytes.extend_from_slice(&table_offset.to_be_bytes()); // offset
        bytes.extend_from_slice(&(u32::MAX - 1).to_be_bytes()); // length: ~4GB
        let path = write_temp_file(&bytes);
        assert!(read_sfnt(&path).is_none());
        let _ = std::fs::remove_file(&path);
    }

    // Fuzz-found: `numFonts` from a TTC header must be checked against the
    // file's actual size before it sizes `font.offsets`/`font.packets`, or
    // a 15-byte file could request a multi-gigabyte allocation. This
    // declares a `numFonts` far larger than the tiny file that follows
    // could possibly hold; if the count-vs-file-size check regressed, this
    // test would hang or OOM instead of failing promptly.
    // Each packet's data-read loop must be bounded by that packet's own
    // `num_tables`, not packet 0's -- otherwise any TTC whose first member
    // has *more* tables than a later one panics out of bounds. Two members:
    // the first with one table, the second with none.
    #[test]
    #[cfg_attr(miri, ignore = "writes/opens a real temp file, unsupported under Miri's default isolation")]
    fn ttc_member_with_fewer_tables_than_the_first_member_reads_cleanly() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&crate::tag::SFNT_TTC_TAG.to_be_bytes());
        bytes.extend_from_slice(&0x00010000u32.to_be_bytes()); // ttc version 1.0
        bytes.extend_from_slice(&2u32.to_be_bytes()); // numFonts
        bytes.extend_from_slice(&20u32.to_be_bytes()); // offsets[0]
        bytes.extend_from_slice(&48u32.to_be_bytes()); // offsets[1]
        // Member 0 (offset 20): one table.
        bytes.extend_from_slice(&crate::tag::SFNT_VERSION_TRUE_TYPE.to_be_bytes());
        bytes.extend_from_slice(&1u16.to_be_bytes()); // num_tables
        bytes.extend_from_slice(&0u16.to_be_bytes()); // search_range
        bytes.extend_from_slice(&0u16.to_be_bytes()); // entry_selector
        bytes.extend_from_slice(&0u16.to_be_bytes()); // range_shift
        bytes.extend_from_slice(b"TEST"); // tag
        bytes.extend_from_slice(&0u32.to_be_bytes()); // check_sum
        bytes.extend_from_slice(&0u32.to_be_bytes()); // offset
        bytes.extend_from_slice(&0u32.to_be_bytes()); // length
        // Member 1 (offset 48): zero tables.
        bytes.extend_from_slice(&crate::tag::SFNT_VERSION_TRUE_TYPE.to_be_bytes());
        bytes.extend_from_slice(&0u16.to_be_bytes()); // num_tables
        bytes.extend_from_slice(&0u16.to_be_bytes()); // search_range
        bytes.extend_from_slice(&0u16.to_be_bytes()); // entry_selector
        bytes.extend_from_slice(&0u16.to_be_bytes()); // range_shift
        assert_eq!(bytes.len(), 60);
        let path = write_temp_file(&bytes);
        let font = read_sfnt(&path).expect("font must parse");
        assert_eq!(font.count, 2);
        assert_eq!(font.packets[0].pieces.len(), 1);
        assert_eq!(font.packets[1].pieces.len(), 0);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    #[cfg_attr(miri, ignore = "writes/opens a real temp file, unsupported under Miri's default isolation")]
    fn ttc_count_far_past_file_end_fails_without_allocating_it() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&crate::tag::SFNT_TTC_TAG.to_be_bytes());
        bytes.extend_from_slice(&0x00010000u32.to_be_bytes()); // ttc version 1.0
        bytes.extend_from_slice(&(u32::MAX - 1).to_be_bytes()); // numFonts: ~4 billion
        let path = write_temp_file(&bytes);
        assert!(read_sfnt(&path).is_none());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    #[cfg_attr(miri, ignore = "writes/opens a real temp file, unsupported under Miri's default isolation")]
    fn well_formed_single_table_font_reads_its_bytes_back() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&crate::tag::SFNT_VERSION_TRUE_TYPE.to_be_bytes());
        bytes.extend_from_slice(&1u16.to_be_bytes());
        bytes.extend_from_slice(&0u16.to_be_bytes());
        bytes.extend_from_slice(&0u16.to_be_bytes());
        bytes.extend_from_slice(&0u16.to_be_bytes());
        let table_offset = 12 + 16u32;
        bytes.extend_from_slice(b"TEST");
        bytes.extend_from_slice(&0u32.to_be_bytes());
        bytes.extend_from_slice(&table_offset.to_be_bytes());
        bytes.extend_from_slice(&4u32.to_be_bytes());
        bytes.extend_from_slice(b"DATA");
        let path = write_temp_file(&bytes);
        let font = read_sfnt(&path).expect("font must parse");
        assert_eq!(font.count, 1);
        assert_eq!(font.packets[0].pieces.len(), 1);
        assert_eq!(font.packets[0].pieces[0].data, b"DATA");
        let _ = std::fs::remove_file(&path);
    }
}
