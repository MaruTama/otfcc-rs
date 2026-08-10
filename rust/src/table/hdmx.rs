#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, memcpy};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u, read_32u};

use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::maxp::{MaxpTable};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceRecord {
    pub pixel_size: u8,
    pub max_width: u8,
    pub widths: *mut u8,
}
#[repr(C)]
pub struct HdmxTable {
    pub version: u16,
    pub num_records: u16,
    pub size_device_record: u32,
    pub records: *mut DeviceRecord,
}
// Stage 6-4 "Box化": `HdmxTable` is entirely dead code -- `otfcc_read_hdmx`
// is never called from `otf_reader.rs` (HDMX has no wired build/dump path
// in this crate at all, confirmed by grepping the whole crate for
// `HdmxTable`/`hdmx` outside this file and `Font`'s own field list), so
// this conversion has zero call sites to update. Converted anyway for
// consistency with every other `Font` field. `records` is a C-style array
// of `DeviceRecord`, each owning a `widths: *mut u8` -- this `Drop` impl
// replaces the old `dispose_hdmx`/vtable pair exactly.
impl Drop for HdmxTable {
    fn drop(&mut self) {
        unsafe {
            if self.records.is_null() {
                return;
            }
            for i in 0..self.num_records as isize {
                let widths = &mut (*self.records.offset(i)).widths;
                if !widths.is_null() {
                    free(*widths as *mut ::core::ffi::c_void);
                    *widths = ::core::ptr::null_mut::<u8>();
                }
            }
            free(self.records as *mut ::core::ffi::c_void);
            self.records = ::core::ptr::null_mut::<DeviceRecord>();
        }
    }
}
pub unsafe extern "C" fn otfcc_read_hdmx(
    mut packet: Packet,
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
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1751412088i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let version = read_16u(data as *const u8);
                    let num_records =
                        read_16u(data.offset(2 as ::core::ffi::c_int as isize) as *const u8);
                    let size_device_record =
                        read_32u(data.offset(4 as ::core::ffi::c_int as isize) as *const u8);
                    let records = __caryll_allocate_clean(
                        (::core::mem::size_of::<DeviceRecord>() as usize)
                            .wrapping_mul(num_records as usize),
                        24 as ::core::ffi::c_ulong,
                    ) as *mut DeviceRecord;
                    let mut i: u32 = 0 as u32;
                    while i < num_records as u32 {
                        (*records.offset(i as isize)).pixel_size = *data
                            .offset(8 as ::core::ffi::c_int as isize)
                            .offset(i.wrapping_mul(
                                (2 as ::core::ffi::c_int + (*maxp).num_glyphs as ::core::ffi::c_int)
                                    as u32,
                            ) as isize);
                        (*records.offset(i as isize)).max_width = *data
                            .offset(8 as ::core::ffi::c_int as isize)
                            .offset(i.wrapping_mul(
                                (2 as ::core::ffi::c_int + (*maxp).num_glyphs as ::core::ffi::c_int)
                                    as u32,
                            ) as isize)
                            .offset(1 as ::core::ffi::c_int as isize);
                        let ref mut fresh1 = (*records.offset(i as isize)).widths;
                        *fresh1 = __caryll_allocate_clean(
                            (::core::mem::size_of::<u8>() as usize)
                                .wrapping_mul((*maxp).num_glyphs as usize),
                            29 as ::core::ffi::c_ulong,
                        ) as *mut u8;
                        memcpy(
                            (*records.offset(i as isize)).widths
                                as *mut ::core::ffi::c_void,
                            data.offset(8 as ::core::ffi::c_int as isize)
                                .offset(i.wrapping_mul(
                                    (2 as ::core::ffi::c_int
                                        + (*maxp).num_glyphs as ::core::ffi::c_int)
                                        as u32,
                                ) as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            (*maxp).num_glyphs as usize,
                        );
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
