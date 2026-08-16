#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::binio::{read_16u, read_32u};

use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::maxp::{MaxpTable};

// `widths` was `__caryll_allocate_clean`'d/`free`'d, sized from `maxp`'s
// `num_glyphs`. `Copy` is dropped along with the raw pointer -- same
// "owning heap data through a raw pointer, freed elsewhere" smell a final
// full-crate audit flagged for `Packet`/`PacketPiece` before those were
// fixed; `HdmxTable`/`DeviceRecord` are confirmed dead code (see below) so
// this was inert, but converted for consistency rather than left as the
// one remaining instance of the pattern.
#[repr(C)]
pub struct DeviceRecord {
    pub pixel_size: u8,
    pub max_width: u8,
    pub widths: Vec<u8>,
}
#[repr(C)]
pub struct HdmxTable {
    pub version: u16,
    pub num_records: u16,
    pub size_device_record: u32,
    pub records: Vec<DeviceRecord>,
}
// Stage 6-4 "Box化": `HdmxTable` is entirely dead code -- `otfcc_read_hdmx`
// is never called from `otf_reader.rs` (HDMX has no wired build/dump path
// in this crate at all, confirmed by grepping the whole crate for
// `HdmxTable`/`hdmx` outside this file and `Font`'s own field list), so
// this conversion has zero call sites to update. Converted anyway for
// consistency with every other `Font` field. `records` (and each
// `DeviceRecord.widths`) is now a plain `Vec`, so the old manual
// `dispose_hdmx`/vtable-`Drop`-impl pair this replaced is no longer
// needed at all -- `Vec`'s own drop glue reaches every level.
pub unsafe fn otfcc_read_hdmx(
    mut packet: &Packet,
    mut _options: *const Options,
    mut maxp: *mut MaxpTable,
) -> Option<Box<HdmxTable>> {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_HDMX {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data.as_ptr() as FontFilePointer;
                    let version = read_16u(data as *const u8);
                    let num_records =
                        read_16u(data.offset(2 as ::core::ffi::c_int as isize) as *const u8);
                    let size_device_record =
                        read_32u(data.offset(4 as ::core::ffi::c_int as isize) as *const u8);
                    let mut records: Vec<DeviceRecord> = Vec::with_capacity(num_records as usize);
                    let mut i: u32 = 0 as u32;
                    while i < num_records as u32 {
                        let record_base = data
                            .offset(8 as ::core::ffi::c_int as isize)
                            .offset(i.wrapping_mul(
                                (2 as ::core::ffi::c_int + (*maxp).num_glyphs as ::core::ffi::c_int)
                                    as u32,
                            ) as isize);
                        let pixel_size = *record_base;
                        let max_width = *record_base.offset(1 as ::core::ffi::c_int as isize);
                        let widths = ::core::slice::from_raw_parts(
                            record_base.offset(2 as ::core::ffi::c_int as isize) as *const u8,
                            (*maxp).num_glyphs as usize,
                        )
                        .to_vec();
                        records.push(DeviceRecord {
                            pixel_size,
                            max_width,
                            widths,
                        });
                        i = i.wrapping_add(1);
                    }
                    return Some(Box::new(HdmxTable {
                        version,
                        num_records,
                        size_device_record,
                        records,
                    }));
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return None;
}
