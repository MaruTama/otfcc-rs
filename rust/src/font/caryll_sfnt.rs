#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{SEEK_SET, exit, fclose, fprintf, fread, fseek};

use crate::support::stdio::{FILE, stderr};
use crate::support::{EXIT_FAILURE};
use crate::support::binio::{EndianProbe16, EndianProbe32};
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
unsafe fn otfcc_read_packets(
    mut font: *mut SplineFontContainer,
    mut file: *mut FILE,
) {
    let mut count: u32 = 0 as u32;
    while count < (*font).count {
        let offsets = &(*font).offsets;
        fseek(
            file,
            offsets[count as usize] as ::core::ffi::c_long,
            SEEK_SET,
        );
        let sfnt_version = otfcc_get32u(file);
        let num_tables = otfcc_get16u(file);
        let search_range = otfcc_get16u(file);
        let entry_selector = otfcc_get16u(file);
        let range_shift = otfcc_get16u(file);
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
                let tag = otfcc_get32u(file);
                let check_sum = otfcc_get32u(file);
                let offset = otfcc_get32u(file);
                let length = otfcc_get32u(file);
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
}
pub unsafe fn otfcc_read_sfnt(mut file: *mut FILE) -> *mut SplineFontContainer {
    if file.is_null() {
        return ::core::ptr::null_mut::<SplineFontContainer>();
    }
    let mut font: *mut SplineFontContainer = Box::into_raw(Box::new(SplineFontContainer {
        type_0: 0,
        count: 0,
        offsets: Vec::new(),
        packets: Vec::new(),
    }));
    (*font).type_0 = otfcc_get32u(file);
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
            otfcc_read_packets(font, file);
        }
        crate::tag::SFNT_TTC_TAG => {
            otfcc_get32u(file);
            (*font).count = otfcc_get32u(file);
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
                let v = otfcc_get32u(file);
                (&mut (*font).offsets)[i as usize] = v;
                i = i.wrapping_add(1);
            }
            otfcc_read_packets(font, file);
        }
        _ => {
            (*font).count = 0 as u32;
            (*font).offsets = Vec::new();
            (*font).packets = Vec::new();
        }
    }
    fclose(file);
    return font;
}
pub unsafe fn otfcc_delete_sfnt(mut font: *mut SplineFontContainer) {
    if font.is_null() {
        return;
    }
    drop(Box::from_raw(font));
}
#[inline]
unsafe fn otfcc_check_endian() -> bool {
    let mut check_union: EndianProbe16 = EndianProbe16 {
        i2: 1 as ::core::ffi::c_int as u16,
    };
    return check_union.i1[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 1 as ::core::ffi::c_int;
}
#[inline]
unsafe fn otfcc_endian_convert16(mut i: u16) -> u16 {
    if otfcc_check_endian() {
        let mut src: EndianProbe16 = EndianProbe16 { i1: [0; 2] };
        let mut des: EndianProbe16 = EndianProbe16 { i1: [0; 2] };
        src.i2 = i;
        des.i1[0 as ::core::ffi::c_int as usize] = src.i1[1 as ::core::ffi::c_int as usize];
        des.i1[1 as ::core::ffi::c_int as usize] = src.i1[0 as ::core::ffi::c_int as usize];
        return des.i2;
    } else {
        return i;
    };
}
#[inline]
unsafe fn otfcc_endian_convert32(mut i: u32) -> u32 {
    if otfcc_check_endian() {
        let mut src: EndianProbe32 = EndianProbe32 { i1: [0; 4] };
        let mut des: EndianProbe32 = EndianProbe32 { i1: [0; 4] };
        src.i4 = i;
        des.i1[0 as ::core::ffi::c_int as usize] = src.i1[3 as ::core::ffi::c_int as usize];
        des.i1[1 as ::core::ffi::c_int as usize] = src.i1[2 as ::core::ffi::c_int as usize];
        des.i1[2 as ::core::ffi::c_int as usize] = src.i1[1 as ::core::ffi::c_int as usize];
        des.i1[3 as ::core::ffi::c_int as usize] = src.i1[0 as ::core::ffi::c_int as usize];
        return des.i4;
    } else {
        return i;
    };
}
#[inline]
unsafe fn otfcc_get16u(mut file: *mut FILE) -> u16 {
    let mut tmp: u16 = 0;
    let mut size_read: usize = fread(
        &raw mut tmp as *mut ::core::ffi::c_void,
        2 as usize,
        1 as usize,
        file,
    ) as usize;
    if size_read == 0 {
        fprintf(
            stderr,
            b"File corruption of terminated unexpectedly.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    return otfcc_endian_convert16(tmp);
}
#[inline]
unsafe fn otfcc_get32u(mut file: *mut FILE) -> u32 {
    let mut tmp: u32 = 0;
    let mut size_read: usize = fread(
        &raw mut tmp as *mut ::core::ffi::c_void,
        4 as usize,
        1 as usize,
        file,
    ) as usize;
    if size_read == 0 {
        fprintf(
            stderr,
            b"File corruption of terminated unexpectedly.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    return otfcc_endian_convert32(tmp);
}
