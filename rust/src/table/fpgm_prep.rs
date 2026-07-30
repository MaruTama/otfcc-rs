#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};


use crate::support::json_funcs::{json_obj_get};
use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::JsonValue;
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::buffer::{bufnew, bufwrite_bytes};
use crate::support::ttinstr::{dump_ttinstr, parse_ttinstr};
use crate::vendor::json_builder::{json_object_push};
use crate::vendor::sds::{sdsempty, sdsfree, sdsnew};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpgmPrepTable {
    pub tag: SdsRaw,
    pub length: u32,
    pub bytes: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FpgmPrepTableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut FpgmPrepTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut FpgmPrepTable, *const FpgmPrepTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut FpgmPrepTable, *mut FpgmPrepTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut FpgmPrepTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut FpgmPrepTable, FpgmPrepTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut FpgmPrepTable, FpgmPrepTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut FpgmPrepTable>,
    pub free: Option<unsafe extern "C" fn(*mut FpgmPrepTable) -> ()>,
}
#[inline]
unsafe extern "C" fn dispose_fpgm_prep(mut table: *mut FpgmPrepTable) {
    if !(*table).tag.is_null() {
        sdsfree((*table).tag);
    }
    if !(*table).bytes.is_null() {
        free((*table).bytes as *mut ::core::ffi::c_void);
        (*table).bytes = ::core::ptr::null_mut::<u8>();
    }
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_init(mut x: *mut FpgmPrepTable) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<FpgmPrepTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_free(mut x: *mut FpgmPrepTable) {
    if x.is_null() {
        return;
    }
    table_fpgm_prep_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_create() -> *mut FpgmPrepTable {
    let mut x: *mut FpgmPrepTable =
        malloc(::core::mem::size_of::<FpgmPrepTable>() as usize) as *mut FpgmPrepTable;
    table_fpgm_prep_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_replace(mut dst: *mut FpgmPrepTable, src: FpgmPrepTable) {
    table_fpgm_prep_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FpgmPrepTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_copy_replace(
    mut dst: *mut FpgmPrepTable,
    src: FpgmPrepTable,
) {
    table_fpgm_prep_dispose(dst);
    table_fpgm_prep_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_copy(
    mut dst: *mut FpgmPrepTable,
    mut src: *const FpgmPrepTable,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FpgmPrepTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_move(
    mut dst: *mut FpgmPrepTable,
    mut src: *mut FpgmPrepTable,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<FpgmPrepTable>() as usize,
    );
    table_fpgm_prep_init(src);
}
#[inline]
unsafe extern "C" fn table_fpgm_prep_dispose(mut x: *mut FpgmPrepTable) {
    dispose_fpgm_prep(x);
}
pub static TABLE_I_FPGM_PREP: FpgmPrepTableElementInterface = {
    FpgmPrepTableElementInterface {
        init: Some(table_fpgm_prep_init as unsafe extern "C" fn(*mut FpgmPrepTable) -> ()),
        copy: Some(
            table_fpgm_prep_copy
                as unsafe extern "C" fn(*mut FpgmPrepTable, *const FpgmPrepTable) -> (),
        ),
        move_0: Some(
            table_fpgm_prep_move
                as unsafe extern "C" fn(*mut FpgmPrepTable, *mut FpgmPrepTable) -> (),
        ),
        dispose: Some(table_fpgm_prep_dispose as unsafe extern "C" fn(*mut FpgmPrepTable) -> ()),
        replace: Some(
            table_fpgm_prep_replace
                as unsafe extern "C" fn(*mut FpgmPrepTable, FpgmPrepTable) -> (),
        ),
        copyReplace: Some(
            table_fpgm_prep_copy_replace
                as unsafe extern "C" fn(*mut FpgmPrepTable, FpgmPrepTable) -> (),
        ),
        create: Some(table_fpgm_prep_create),
        free: Some(table_fpgm_prep_free as unsafe extern "C" fn(*mut FpgmPrepTable) -> ()),
    }
};
pub unsafe extern "C" fn otfcc_read_fpgm_prep(
    packet: Packet,
    mut _options: *const Options,
    mut tag: u32,
) -> *mut FpgmPrepTable {
    let mut t: *mut FpgmPrepTable = ::core::ptr::null_mut::<FpgmPrepTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == tag {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    t = (
                        TABLE_I_FPGM_PREP.create.expect("non-null function pointer"))();
                    (*t).tag = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    (*t).length = length;
                    (*t).bytes = __caryll_allocate_clean(
                        (::core::mem::size_of::<u8>() as usize)
                            .wrapping_mul(length as usize),
                        22 as ::core::ffi::c_ulong,
                    ) as *mut u8;
                    if (*t).bytes.is_null() {
                        TABLE_I_FPGM_PREP.free.expect("non-null function pointer")(t);
                        t = ::core::ptr::null_mut::<FpgmPrepTable>();
                    } else {
                        memcpy(
                            (*t).bytes as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            length as usize,
                        );
                        return t;
                    }
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<FpgmPrepTable>();
}
pub unsafe extern "C" fn table_dump_table_fpgm_prep(
    mut table: *const FpgmPrepTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), tag),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        json_object_push(
            root,
            tag,
            dump_ttinstr((*table).bytes, (*table).length, options),
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn make_fpgm_prep_instr(
    mut _t: *mut ::core::ffi::c_void,
    mut instrs: *mut u8,
    mut length: u32,
) {
    let mut t: *mut FpgmPrepTable = _t as *mut FpgmPrepTable;
    (*t).length = length;
    (*t).bytes = instrs;
}
pub unsafe extern "C" fn wrong_fpgm_prep_instr(
    mut _t: *mut ::core::ffi::c_void,
    mut _reason: *mut ::core::ffi::c_char,
    mut _pos: ::core::ffi::c_int,
) {
}
pub unsafe extern "C" fn otfcc_parse_fpgm_prep(
    mut root: *const JsonValue,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut FpgmPrepTable {
    let mut t: *mut FpgmPrepTable = ::core::ptr::null_mut::<FpgmPrepTable>();
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get(root, tag);
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), tag),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            t = (
                TABLE_I_FPGM_PREP.create.expect("non-null function pointer"))();
            (*t).tag = sdsnew(tag);
            parse_ttinstr(
                table,
                t as *mut ::core::ffi::c_void,
                Some(
                    make_fpgm_prep_instr
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut u8,
                            u32,
                        ) -> (),
                ),
                Some(
                    wrong_fpgm_prep_instr
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
            );
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return t;
}
pub unsafe extern "C" fn otfcc_build_fpgm_prep(
    mut table: *const FpgmPrepTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if table.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite_bytes(buf, (*table).length as usize, (*table).bytes);
    return buf;
}
