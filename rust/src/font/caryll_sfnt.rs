#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{SEEK_SET, fclose, fread, fseek};

use crate::support::stdio::FILE;
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
// `false` means a `fread` inside hit EOF partway through -- a truncated
// file, not an in-memory bug -- and the caller (`otfcc_read_sfnt`) tears
// down the partially-built `font` and returns null instead. This replaced
// `otfcc_get16u`/`otfcc_get32u`'s own `exit()` on read failure: `otfccdump.
// rs`'s caller already null-checks `otfcc_read_sfnt`'s return and logs a
// clean "Cannot read SFNT file ...". Exit." through the normal `Logger`
// channel, so routing failure there instead of exiting deep in this loop
// only reuses an error path that already existed, rather than adding one.
unsafe fn otfcc_read_packets(mut font: *mut SplineFontContainer, mut file: *mut FILE) -> bool {
    let mut count: u32 = 0 as u32;
    while count < (*font).count {
        let offsets = &(*font).offsets;
        fseek(
            file,
            offsets[count as usize] as ::core::ffi::c_long,
            SEEK_SET,
        );
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
            let packets = &mut (*font).packets;
            let packet = &mut packets[count as usize];
            packet.sfnt_version = sfnt_version;
            packet.num_tables = num_tables;
            packet.search_range = search_range;
            packet.entry_selector = entry_selector;
            packet.range_shift = range_shift;
            let mut i: u32 = 0 as u32;
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
                i = i.wrapping_add(1);
            }
        }
        // Bounded by packet 0's `num_tables`, not this packet's own -- a
        // quirk preserved exactly from the original C (`(*(*font).packets.
        // offset(0)).num_tables`), not "fixed" here since this is a
        // mechanical ownership conversion, not a behavior change.
        let packet_0_num_tables = {
            let packets = &(*font).packets;
            packets[0].num_tables
        };
        {
            let packets = &mut (*font).packets;
            let packet = &mut packets[count as usize];
            let mut i_0: u32 = 0 as u32;
            while i_0 < packet_0_num_tables as u32 {
                fseek(
                    file,
                    packet.pieces[i_0 as usize].offset as ::core::ffi::c_long,
                    SEEK_SET,
                );
                fread(
                    packet.pieces[i_0 as usize].data.as_mut_ptr() as *mut ::core::ffi::c_void,
                    packet.pieces[i_0 as usize].length as usize,
                    1 as usize,
                    file,
                );
                i_0 = i_0.wrapping_add(1);
            }
        }
        count = count.wrapping_add(1);
    }
    true
}
// Reads the header/directory fields; `otfcc_read_sfnt` (below) owns
// allocating and tearing down `font` around this call. Split out so a
// truncated-file failure partway through -- signalled the same way
// `otfcc_read_packets` does, by returning `false` -- can be handled once,
// in one place, instead of duplicating the "free `font`, return null"
// cleanup at every read site.
unsafe fn otfcc_read_sfnt_body(font: *mut SplineFontContainer, file: *mut FILE) -> bool {
    let Some(type_0) = otfcc_get32u(file) else {
        return false;
    };
    (*font).type_0 = type_0;
    match (*font).type_0 {
        crate::tag::SFNT_VERSION_OTTO
        | crate::tag::SFNT_VERSION_TRUE_TYPE
        | crate::tag::SFNT_VERSION_MAC_TRUE
        | crate::tag::SFNT_VERSION_MAC_TYPE1 => {
            (*font).count = 1 as u32;
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
            (&mut (*font).offsets)[0] = 0 as u32;
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
            let mut i: u32 = 0 as u32;
            while i < (*font).count {
                let Some(v) = otfcc_get32u(file) else {
                    return false;
                };
                (&mut (*font).offsets)[i as usize] = v;
                i = i.wrapping_add(1);
            }
            otfcc_read_packets(font, file)
        }
        _ => {
            (*font).count = 0 as u32;
            (*font).offsets = Vec::new();
            (*font).packets = Vec::new();
            true
        }
    }
}
pub unsafe fn otfcc_read_sfnt(mut file: *mut FILE) -> *mut SplineFontContainer {
    if file.is_null() {
        return ::core::ptr::null_mut::<SplineFontContainer>();
    }
    let font: *mut SplineFontContainer = Box::into_raw(Box::new(SplineFontContainer {
        type_0: 0,
        count: 0,
        offsets: Vec::new(),
        packets: Vec::new(),
    }));
    let ok = otfcc_read_sfnt_body(font, file);
    fclose(file);
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
// `None` on a short read (EOF partway through, i.e. a truncated file) --
// used to `fprintf` a raw message straight to `stderr` and `exit()`
// immediately, bypassing the `Logger` and whatever caller was in the
// middle of assembling. See `otfcc_read_packets`'s comment for where the
// failure actually surfaces instead.
//
// `fread` copies the file's big-endian bytes into `tmp` byte-for-byte, so
// `tmp`'s in-memory bytes are already correct -- it's only `tmp`'s *value*
// that's wrong on a little-endian host (the standard library's
// `from_be`/`from_be_bytes` distinction: this is the former, since the
// bytes need no reordering, only the value needs reinterpreting). This
// used to be `otfcc_endian_convert16`/`32`, a `EndianProbe16`/`32` union
// pair that ran a runtime host-endianness probe and conditionally
// byte-swapped -- `u16::from_be`/`u32::from_be` are the same operation,
// built into the standard library without a union.
#[inline]
unsafe fn otfcc_get16u(mut file: *mut FILE) -> Option<u16> {
    let mut tmp: u16 = 0;
    let mut size_read: usize = fread(
        &raw mut tmp as *mut ::core::ffi::c_void,
        2 as usize,
        1 as usize,
        file,
    ) as usize;
    if size_read == 0 {
        return None;
    }
    Some(u16::from_be(tmp))
}
#[inline]
unsafe fn otfcc_get32u(mut file: *mut FILE) -> Option<u32> {
    let mut tmp: u32 = 0;
    let mut size_read: usize = fread(
        &raw mut tmp as *mut ::core::ffi::c_void,
        4 as usize,
        1 as usize,
        file,
    ) as usize;
    if size_read == 0 {
        return None;
    }
    Some(u32::from_be(tmp))
}
