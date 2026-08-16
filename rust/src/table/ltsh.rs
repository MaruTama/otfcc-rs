#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, memcpy};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite8};


#[repr(C)]
pub struct LtshTable {
    pub version: u16,
    pub num_glyphs: GlyphId,
    pub y_pels: *mut u8,
}
// Stage 6-4 pilot for `Font`'s `*mut X`-typed table fields: `y_pels` is the
// only allocation this struct owns, so `Box<LtshTable>` (via `Font.ltsh:
// Option<Box<LtshTable>>`) plus this `Drop` impl replaces the entire
// `LtshTableElementInterface` vtable that used to exist here -- grepping
// confirmed only `.free` was ever called from outside this file (from
// `font/caryll_font.rs`'s table disposal), and `.init`/`.copy`/`.create`/
// `.dispose` were never called at all (`otfcc_read_ltsh`/`stat_ltsh`
// already built via `__caryll_allocate_clean` directly, not through the
// vtable's `.create`). `Copy`/`Clone` dropped: a `Drop` impl and `Copy`
// are mutually exclusive, and `y_pels` needing single ownership means
// `Copy` was already semantically wrong before this PR, just unenforced.
impl Drop for LtshTable {
    fn drop(&mut self) {
        unsafe {
            if !self.y_pels.is_null() {
                free(self.y_pels as *mut ::core::ffi::c_void);
                self.y_pels = ::core::ptr::null_mut::<u8>();
            }
        }
    }
}
pub unsafe fn otfcc_read_ltsh(
    packet: &Packet,
    mut _options: *const Options,
) -> Option<Box<LtshTable>> {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_LTSH {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data.as_ptr() as FontFilePointer;
                    let version = read_16u(data as *const u8);
                    let num_glyphs =
                        read_16u(data.offset(2 as ::core::ffi::c_int as isize) as *const u8)
                            as GlyphId;
                    let y_pels = __caryll_allocate_clean(
                        (::core::mem::size_of::<u8>() as usize)
                            .wrapping_mul(num_glyphs as usize),
                        18 as ::core::ffi::c_ulong,
                    ) as *mut u8;
                    memcpy(
                        y_pels as *mut ::core::ffi::c_void,
                        data.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                        num_glyphs as usize,
                    );
                    return Some(Box::new(LtshTable { version, num_glyphs, y_pels }));
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return None;
}
// `Option<&LtshTable>`, not `*const LtshTable`: internal-only call (never
// crosses the real FFI boundary, see `rust/README.md`), and the crate's
// only caller now hands `(*font).ltsh.as_deref()` from `Font.ltsh:
// Option<Box<LtshTable>>`.
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_ltsh(
    ltsh: Option<&LtshTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let ltsh = match ltsh {
        Some(l) => l,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, ltsh.num_glyphs as u16);
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < ltsh.num_glyphs as ::core::ffi::c_int {
        bufwrite8(buf, *ltsh.y_pels.offset(j as isize));
        j = j.wrapping_add(1);
    }
    return buf;
}
