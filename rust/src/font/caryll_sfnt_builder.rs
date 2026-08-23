#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::free;

use crate::logger::{LOG_VL_PROGRESS, LoggerType, logger_log_sds};
use crate::support::alloc::__caryll_allocate_clean;
use crate::support::binio::{EndianProbe16, EndianProbe32};
use crate::support::buffer::Buffer;
use crate::support::buffer::{
    buffree, buflen, buflongalign, bufnew, bufseek, bufwrite_buf, bufwrite16b, bufwrite32b,
};
use crate::support::options::Options;
use crate::vendor::sds::Byte;
pub struct SfntTableEntry {
    pub tag: ::core::ffi::c_int,
    pub length: u32,
    pub checksum: u32,
    pub buffer: *mut Buffer,
}
pub struct SfntBuilder {
    pub count: u32,
    pub header: u32,
    pub tables: std::collections::BTreeMap<::core::ffi::c_int, SfntTableEntry>,
    pub options: *const Options,
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
unsafe fn buf_checksum(mut buffer: *mut Buffer) -> u32 {
    let mut actual_length: u32 = buflen(buffer) as u32;
    buflongalign(buffer);
    let mut sum: u32 = 0 as u32;
    let mut start: *mut u32 = (*buffer).data.as_mut_ptr() as *mut u32;
    let mut end: *mut u32 = start.offset(
        ((actual_length.wrapping_add(3 as u32) & !(3 as ::core::ffi::c_int) as u32) as usize)
            .wrapping_div(::core::mem::size_of::<u32>()) as isize,
    );
    while start < end {
        let fresh3 = start;
        start = start.offset(1);
        sum = sum.wrapping_add(otfcc_endian_convert32(*fresh3));
    }
    return sum;
}
unsafe fn create_segment(tag: u32, buffer: *mut Buffer) -> SfntTableEntry {
    let length = buflen(buffer) as u32;
    buflongalign(buffer);
    let mut sum: u32 = 0 as u32;
    let mut start: *mut u32 = (*buffer).data.as_mut_ptr() as *mut u32;
    let mut end: *mut u32 = start.offset(
        ((length.wrapping_add(3 as u32) & !(3 as ::core::ffi::c_int) as u32) as usize)
            .wrapping_div(::core::mem::size_of::<u32>()) as isize,
    );
    while start < end {
        let fresh0 = start;
        start = start.offset(1);
        sum = sum.wrapping_add(otfcc_endian_convert32(*fresh0));
    }
    SfntTableEntry {
        tag: tag as ::core::ffi::c_int,
        length,
        checksum: sum,
        buffer,
    }
}
pub unsafe fn otfcc_new_sfnt_builder(mut header: u32, mut options: &Options) -> *mut SfntBuilder {
    let mut builder: *mut SfntBuilder = __caryll_allocate_clean(
        ::core::mem::size_of::<SfntBuilder>() as usize,
        40 as ::core::ffi::c_ulong,
    ) as *mut SfntBuilder;
    (*builder).count = 0 as u32;
    (*builder).header = header;
    // `BTreeMap` (a `std` type, but the caution established for
    // `IndexMap` in `table/fvar.rs` applies just as much: no documented
    // guarantee that an all-zero-bytes value is a safe-to-drop empty map)
    // gets `ptr::write`, not a plain assignment, so the calloc'd garbage
    // sitting here is never read or dropped.
    ::core::ptr::write(
        &raw mut (*builder).tables,
        std::collections::BTreeMap::new(),
    );
    (*builder).options = options;
    return builder;
}
pub unsafe fn otfcc_delete_sfnt_builder(mut builder: *mut SfntBuilder) {
    if builder.is_null() {
        return;
    }
    for (_, entry) in ::core::mem::take(&mut (*builder).tables) {
        buffree(entry.buffer);
    }
    free(builder as *mut ::core::ffi::c_void);
}
// Deduplicates by `tag`, first registration wins -- a later
// `otfcc_sfnt_builder_push_table` call for a tag already present just
// frees the newly-passed `buffer` and returns, silently, no warning
// logged (unlike the `consolidate/otl/*.rs` uthash instances earlier in
// this migration, which mostly do warn on a duplicate). `BTreeMap`, not
// `IndexMap`/`Vec`: `otfcc_sfnt_builder_serialize` (below) sorts entries
// by tag before writing the table directory -- required by the SFNT
// format itself, which mandates the directory be sorted ascending by
// tag -- so `BTreeMap`'s always-sorted iteration is exactly right here,
// the same shape as the six `consolidate/otl/*.rs` instances earlier in
// this migration and unlike `ScriptStatHash`/`FvarMaster`'s insertion
// order.
pub unsafe fn otfcc_sfnt_builder_push_table(
    mut builder: *mut SfntBuilder,
    mut tag: u32,
    mut buffer: *mut Buffer,
) {
    if builder.is_null() || buffer.is_null() {
        return;
    }
    let options: *const Options = (*builder).options;
    if (*builder).tables.contains_key(&(tag as ::core::ffi::c_int)) {
        buffree(buffer);
        return;
    }
    let entry = create_segment(tag, buffer);
    (*builder).tables.insert(tag as ::core::ffi::c_int, entry);
    logger_log_sds(
        &mut *(*options).logger.borrow_mut(),
        LOG_VL_PROGRESS,
        LoggerType::Progress,
        crate::bytesbuild!(
            b"OpenType table ",
            Byte((tag >> 24 as ::core::ffi::c_int & 0xff as u32) as u8),
            Byte((tag >> 16 as ::core::ffi::c_int & 0xff as u32) as u8),
            Byte((tag >> 8 as ::core::ffi::c_int & 0xff as u32) as u8),
            Byte((tag & 0xff as u32) as u8),
            b" successfully built.\n",
        ),
    );
}
pub unsafe fn otfcc_sfnt_builder_serialize(mut builder: *mut SfntBuilder) -> *mut Buffer {
    let mut buffer: *mut Buffer = bufnew();
    if builder.is_null() {
        return buffer;
    }
    let n_tables: u16 = (*builder).tables.len() as u16;
    let mut search_range: u16 = ((if (n_tables as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
        8 as ::core::ffi::c_int
    } else {
        if (n_tables as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            16 as ::core::ffi::c_int
        } else {
            if (n_tables as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
                32 as ::core::ffi::c_int
            } else {
                64 as ::core::ffi::c_int
            }
        }
    }) * 16 as ::core::ffi::c_int) as u16;
    bufwrite32b(buffer, (*builder).header);
    bufwrite16b(buffer, n_tables);
    bufwrite16b(buffer, search_range);
    bufwrite16b(
        buffer,
        (if (n_tables as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
            3 as ::core::ffi::c_int
        } else if (n_tables as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            4 as ::core::ffi::c_int
        } else if (n_tables as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            5 as ::core::ffi::c_int
        } else {
            6 as ::core::ffi::c_int
        }) as u16,
    );
    bufwrite16b(
        buffer,
        (n_tables as ::core::ffi::c_int * 16 as ::core::ffi::c_int
            - search_range as ::core::ffi::c_int) as u16,
    );
    let mut offset: usize = (12 as ::core::ffi::c_int
        + n_tables as ::core::ffi::c_int * 16 as ::core::ffi::c_int)
        as usize;
    let mut head_offset: usize = offset;
    for (tag, table) in (*builder).tables.iter() {
        bufwrite32b(buffer, *tag as u32);
        bufwrite32b(buffer, table.checksum);
        bufwrite32b(buffer, offset as u32);
        bufwrite32b(buffer, table.length);
        let cp: usize = (*buffer).cursor;
        bufseek(buffer, offset);
        bufwrite_buf(buffer, table.buffer);
        bufseek(buffer, cp);
        if *tag == crate::tag::TAG_HEAD as i32 {
            head_offset = offset;
        }
        offset = offset.wrapping_add(buflen(table.buffer));
    }
    let mut whole_checksum: u32 = buf_checksum(buffer);
    bufseek(buffer, head_offset.wrapping_add(8 as usize));
    bufwrite32b(buffer, (0xb1b0afba as u32).wrapping_sub(whole_checksum));
    return buffer;
}
