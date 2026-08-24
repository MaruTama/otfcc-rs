#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::ffi::OsStrExt;

// `data` was `__caryll_allocate_clean`'d/`free`'d, sized from `length` --
// read straight out of the SFNT table directory, i.e. untrusted font bytes.
// The same risk class `CffIndex`/`CffDict` closed: a counting mistake in
// `otfcc_read_packets` below would have been an immediate OOB write: `Vec`
// removes that structurally.
pub struct PacketPiece {
    pub tag: u32,
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: Vec<u8>,
}
// `pieces` was similarly `__caryll_allocate_clean`'d/`free`'d, sized from
// `num_tables` (also untrusted). `Packet` used to derive `Copy` purely so
// every `table/*.rs` parser (~30 files) and `otf_reader.rs`'s `otfcc_read_sfnt`
// (which reuses one `packet` across ~20 sequential calls) could pass it by
// value without borrow-checker friction -- none of those sites ever needed
// ownership, only read access, so every one of them now takes `&Packet`
// instead. `Copy` is dropped along with the raw pointer it was papering over.
pub struct Packet {
    pub sfnt_version: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub pieces: Vec<PacketPiece>,
}
// `offsets`/`packets` were `__caryll_allocate_clean`'d/`free`'d, sized from
// `count` (either `1`, or read from a TTC header -- also untrusted).
pub struct SplineFontContainer {
    pub type_0: u32,
    pub count: u32,
    pub offsets: Vec<u32>,
    pub packets: Vec<Packet>,
}
// `false` on any I/O failure -- EOF partway through a read, or a seek past
// the end of file, either one meaning a truncated or otherwise malformed
// file, not an in-memory bug -- and the caller (`otfcc_read_sfnt`) tears
// down the partially-built `font` and returns null instead. `otfccdump.rs`'s
// caller already null-checks `otfcc_read_sfnt`'s return and logs a clean
// "Cannot read SFNT file ...". Exit." through the normal `Logger` channel,
// so routing failure there reuses an error path that already existed.
//
// This used to read each table's actual bytes with `fread`, discarding the
// return value -- so a table whose declared `length` ran past the actual
// end of a truncated file was silently zero-padded instead of failing the
// read. `Read::read_exact` (below, and in `otfcc_get16u`/`32`) fails
// instead, the same way the header/directory fields already did.
unsafe fn otfcc_read_packets<R: Read + Seek>(
    font: *mut SplineFontContainer,
    file: &mut R,
) -> bool {
    let font: &mut SplineFontContainer = &mut *font;
    let mut count: u32 = 0;
    while count < font.count {
        let offset = font.offsets[count as usize];
        if file.seek(SeekFrom::Start(offset as u64)).is_err() {
            return false;
        }
        let Some(sfnt_version) = otfcc_get32u(file) else {
            return false;
        };
        let Some(num_tables) = otfcc_get16u(file) else {
            return false;
        };
        let Some(search_range) = otfcc_get16u(file) else {
            return false;
        };
        let Some(entry_selector) = otfcc_get16u(file) else {
            return false;
        };
        let Some(range_shift) = otfcc_get16u(file) else {
            return false;
        };
        {
            let packet = &mut font.packets[count as usize];
            packet.sfnt_version = sfnt_version;
            packet.num_tables = num_tables;
            packet.search_range = search_range;
            packet.entry_selector = entry_selector;
            packet.range_shift = range_shift;
            let mut i: u32 = 0;
            while i < packet.num_tables as u32 {
                let Some(tag) = otfcc_get32u(file) else {
                    return false;
                };
                let Some(check_sum) = otfcc_get32u(file) else {
                    return false;
                };
                let Some(offset) = otfcc_get32u(file) else {
                    return false;
                };
                let Some(length) = otfcc_get32u(file) else {
                    return false;
                };
                packet.pieces.push(PacketPiece {
                    tag,
                    check_sum,
                    offset,
                    length,
                    data: vec![0u8; length as usize],
                });
                i += 1;
            }
        }
        // Bounded by packet 0's `num_tables`, not this packet's own -- a
        // quirk preserved exactly from the original C (`(*(*font).packets.
        // offset(0)).num_tables`), not "fixed" here since this is a
        // mechanical ownership conversion, not a behavior change.
        let packet_0_num_tables = font.packets[0].num_tables;
        {
            let packet = &mut font.packets[count as usize];
            let mut i_0: u32 = 0;
            while i_0 < packet_0_num_tables as u32 {
                let piece = &mut packet.pieces[i_0 as usize];
                if file.seek(SeekFrom::Start(piece.offset as u64)).is_err() {
                    return false;
                }
                if file.read_exact(&mut piece.data).is_err() {
                    return false;
                }
                i_0 += 1;
            }
        }
        count += 1;
    }
    true
}
// Reads the header/directory fields; `otfcc_read_sfnt` (below) owns
// allocating and tearing down `font` around this call. Split out so a
// truncated-file failure partway through -- signalled the same way
// `otfcc_read_packets` does, by returning `false` -- can be handled once,
// in one place, instead of duplicating the "free `font`, return null"
// cleanup at every read site.
unsafe fn otfcc_read_sfnt_body<R: Read + Seek>(
    font: *mut SplineFontContainer,
    file: &mut R,
) -> bool {
    let Some(type_0) = otfcc_get32u(file) else {
        return false;
    };
    (*font).type_0 = type_0;
    match (*font).type_0 {
        crate::tag::SFNT_VERSION_OTTO
        | crate::tag::SFNT_VERSION_TRUE_TYPE
        | crate::tag::SFNT_VERSION_MAC_TRUE
        | crate::tag::SFNT_VERSION_MAC_TYPE1 => {
            (*font).count = 1;
            (*font).offsets = vec![0];
            (*font).packets = (0..(*font).count)
                .map(|_| Packet {
                    sfnt_version: 0,
                    num_tables: 0,
                    search_range: 0,
                    entry_selector: 0,
                    range_shift: 0,
                    pieces: Vec::new(),
                })
                .collect();
            otfcc_read_packets(font, file)
        }
        crate::tag::SFNT_TTC_TAG => {
            let Some(_ttc_version) = otfcc_get32u(file) else {
                return false;
            };
            let Some(count) = otfcc_get32u(file) else {
                return false;
            };
            (*font).count = count;
            (*font).offsets = vec![0; (*font).count as usize];
            (*font).packets = (0..(*font).count)
                .map(|_| Packet {
                    sfnt_version: 0,
                    num_tables: 0,
                    search_range: 0,
                    entry_selector: 0,
                    range_shift: 0,
                    pieces: Vec::new(),
                })
                .collect();
            let mut i: u32 = 0;
            let offsets: &mut Vec<u32> = &mut (*font).offsets;
            while i < offsets.len() as u32 {
                let Some(v) = otfcc_get32u(file) else {
                    return false;
                };
                offsets[i as usize] = v;
                i += 1;
            }
            otfcc_read_packets(font, file)
        }
        _ => {
            (*font).count = 0;
            (*font).offsets = Vec::new();
            (*font).packets = Vec::new();
            true
        }
    }
}
/// Opens and reads an SFNT (or TTC) file by path, returning null on any
/// failure -- the file doesn't exist, isn't readable, or is truncated/
/// malformed partway through. `otfccdump.rs`'s caller null-checks the
/// result and logs accordingly; there is no separate "couldn't open" vs.
/// "couldn't parse" signal, matching how this always worked (previously,
/// the caller `fopen`'d the file itself and this function null-checked
/// that pointer -- opening now happens in here instead, so the caller no
/// longer needs a `libc::fopen`/`FILE*` of its own at all).
pub unsafe fn otfcc_read_sfnt(path: *const ::core::ffi::c_char) -> *mut SplineFontContainer {
    if path.is_null() {
        return ::core::ptr::null_mut::<SplineFontContainer>();
    }
    let path_bytes = unsafe { ::core::ffi::CStr::from_ptr(path) }.to_bytes();
    let os_path = std::ffi::OsStr::from_bytes(path_bytes);
    let Ok(mut file) = std::fs::File::open(std::path::Path::new(os_path)) else {
        return ::core::ptr::null_mut::<SplineFontContainer>();
    };
    otfcc_read_sfnt_from_reader(&mut file)
}
/// [`otfcc_read_sfnt`]'s file-opening split from its actual reading, for
/// callers that already have bytes in memory rather than a path -- the
/// `otf_parse` fuzz target uses this with a `std::io::Cursor<&[u8]>` over
/// the fuzzer-provided input instead of writing it to a real temp file on
/// every one of its thousands-per-process iterations (this used to be
/// `fmemopen` wrapping a byte buffer as a `FILE*`, back when
/// `otfcc_read_sfnt` itself was `FILE*`-shaped).
pub unsafe fn otfcc_read_sfnt_from_reader<R: Read + Seek>(
    file: &mut R,
) -> *mut SplineFontContainer {
    let font: *mut SplineFontContainer = Box::into_raw(Box::new(SplineFontContainer {
        type_0: 0,
        count: 0,
        offsets: Vec::new(),
        packets: Vec::new(),
    }));
    let ok = otfcc_read_sfnt_body(font, file);
    if !ok {
        drop(Box::from_raw(font));
        return ::core::ptr::null_mut::<SplineFontContainer>();
    }
    return font;
}
pub unsafe fn otfcc_delete_sfnt(mut font: *mut SplineFontContainer) {
    if font.is_null() {
        return;
    }
    drop(Box::from_raw(font));
}
// `None` on a short read (EOF partway through, i.e. a truncated file).
// `read_exact` reports that as an `Err` on its own -- no separate
// byte-count check needed the way `fread`'s return value did.
fn otfcc_get16u<R: Read>(file: &mut R) -> Option<u16> {
    let mut buf = [0u8; 2];
    file.read_exact(&mut buf).ok()?;
    Some(u16::from_be_bytes(buf))
}
fn otfcc_get32u<R: Read>(file: &mut R) -> Option<u32> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf).ok()?;
    Some(u32::from_be_bytes(buf))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_file(bytes: &[u8]) -> std::ffi::CString {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "otfcc-caryll-sfnt-test-{:?}-{}",
            std::thread::current().id(),
            bytes.len()
        ));
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(bytes).unwrap();
        std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap()
    }

    #[test]
    #[cfg_attr(miri, ignore = "calls std::fs::File::open on a real path, unsupported under Miri's default isolation")]
    fn nonexistent_path_returns_null() {
        unsafe {
            let path = std::ffi::CString::new("/nonexistent/otfcc-test-path").unwrap();
            assert!(otfcc_read_sfnt(path.as_ptr()).is_null());
        }
    }

    #[test]
    fn null_path_returns_null() {
        unsafe {
            assert!(otfcc_read_sfnt(::core::ptr::null()).is_null());
        }
    }

    // The bug this file's rewrite fixes: a table whose declared length runs
    // past the truncated file's actual end used to be silently zero-padded
    // (`fread`'s return value discarded) instead of failing the read.
    #[test]
    #[cfg_attr(miri, ignore = "writes/opens a real temp file, unsupported under Miri's default isolation")]
    fn table_length_past_truncated_file_end_fails_instead_of_zero_padding() {
        unsafe {
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
            assert!(otfcc_read_sfnt(path.as_ptr()).is_null());
            let _ = std::fs::remove_file(std::path::Path::new(
                std::ffi::OsStr::from_bytes(path.as_bytes()),
            ));
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "writes/opens a real temp file, unsupported under Miri's default isolation")]
    fn well_formed_single_table_font_reads_its_bytes_back() {
        unsafe {
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
            let sfnt = otfcc_read_sfnt(path.as_ptr());
            assert!(!sfnt.is_null());
            let font: &SplineFontContainer = &*sfnt;
            assert_eq!(font.count, 1);
            assert_eq!(font.packets[0].pieces.len(), 1);
            assert_eq!(font.packets[0].pieces[0].data, b"DATA");
            otfcc_delete_sfnt(sfnt);
            let _ = std::fs::remove_file(std::path::Path::new(
                std::ffi::OsStr::from_bytes(path.as_bytes()),
            ));
        }
    }
}
