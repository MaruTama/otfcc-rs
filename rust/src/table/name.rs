#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset, qsort};
use crate::support::json_funcs::{json_obj_get_type, json_obj_getint};
use crate::support::binio::{read_16u};
use crate::logger::{log_type_warning, log_vl_important, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::vendor::json::{json_array, json_integer, json_object, json_string, JsonValue};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::{ComparFn};
use crate::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};
use crate::support::base64::{base64_decode, base64_encode};
use crate::support::buffer::{buffree, bufnew, bufseek, bufwrite16b, bufwrite_buf, bufwrite_bytes};
use crate::support::unicodeconv::{utf16be_to_utf8, utf8toutf16be};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_string_new_length};
use crate::vendor::sds::{sdsempty, sdsfree, sdsgrowzero, sdsnewlen};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameRecord {
    pub platformID: u16,
    pub encodingID: u16,
    pub languageID: u16,
    pub nameID: u16,
    pub nameString: SdsRaw,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameRecordElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut NameRecord) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut NameRecord, *const NameRecord) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut NameRecord, *mut NameRecord) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut NameRecord) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut NameRecord, NameRecord) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut NameRecord, NameRecord) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameTable {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut NameRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameTableVectorInterface {
    pub init: Option<unsafe extern "C" fn(*mut NameTable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut NameTable, *const NameTable) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut NameTable, *mut NameTable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut NameTable) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut NameTable, NameTable) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut NameTable, NameTable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut NameTable>,
    pub free: Option<unsafe extern "C" fn(*mut NameTable) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut NameTable, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut NameTable, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut NameTable>,
    pub fill: Option<unsafe extern "C" fn(*mut NameTable, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut NameTable) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut NameTable, NameRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut NameTable) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut NameTable) -> NameRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut NameTable, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut NameTable,
            Option<unsafe extern "C" fn(*const NameRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut NameTable,
            Option<
                unsafe extern "C" fn(
                    *const NameRecord,
                    *const NameRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
pub const COPYRIGHT_LEN: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
unsafe extern "C" fn nameRecordDtor(mut entry: *mut NameRecord) {
    sdsfree((*entry).nameString);
    (*entry).nameString = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_move(
    mut dst: *mut NameRecord,
    mut src: *mut NameRecord,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<NameRecord>() as usize,
    );
    otfcc_NameRecord_init(src);
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_init(mut x: *mut NameRecord) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<NameRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_replace(
    mut dst: *mut NameRecord,
    src: NameRecord,
) {
    otfcc_NameRecord_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<NameRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_copyReplace(
    mut dst: *mut NameRecord,
    src: NameRecord,
) {
    otfcc_NameRecord_dispose(dst);
    otfcc_NameRecord_copy(dst, &raw const src);
}
pub static otfcc_iNameRecord: NameRecordElementInterface = {
    NameRecordElementInterface {
        init: Some(otfcc_NameRecord_init as unsafe extern "C" fn(*mut NameRecord) -> ()),
        copy: Some(
            otfcc_NameRecord_copy
                as unsafe extern "C" fn(*mut NameRecord, *const NameRecord) -> (),
        ),
        move_0: Some(
            otfcc_NameRecord_move
                as unsafe extern "C" fn(*mut NameRecord, *mut NameRecord) -> (),
        ),
        dispose: Some(
            otfcc_NameRecord_dispose as unsafe extern "C" fn(*mut NameRecord) -> (),
        ),
        replace: Some(
            otfcc_NameRecord_replace
                as unsafe extern "C" fn(*mut NameRecord, NameRecord) -> (),
        ),
        copyReplace: Some(
            otfcc_NameRecord_copyReplace
                as unsafe extern "C" fn(*mut NameRecord, NameRecord) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otfcc_NameRecord_dispose(mut x: *mut NameRecord) {
    nameRecordDtor(x);
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_copy(
    mut dst: *mut NameRecord,
    mut src: *const NameRecord,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<NameRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_name_growTo(arr: *mut NameTable, target: usize) {
    cvec_grow_to(table_name_as_cvec(arr), target);
}
#[inline]
unsafe fn table_name_as_cvec(arr: *mut NameTable) -> *mut CVecRaw<NameRecord> {
    arr as *mut CVecRaw<NameRecord>
}
#[inline]
unsafe extern "C" fn table_name_init(arr: *mut NameTable) {
    cvec_init(table_name_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_name_filterEnv(
    mut arr: *mut NameTable,
    mut fn_0: Option<
        unsafe extern "C" fn(*const NameRecord, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut NameRecord,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if otfcc_iNameRecord.dispose.is_some() {
                otfcc_iNameRecord
                    .dispose
                    .expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut NameRecord,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn table_name_disposeItem(mut arr: *mut NameTable, mut n: usize) {
    if otfcc_iNameRecord.dispose.is_some() {
        otfcc_iNameRecord
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut NameRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_name_sort(
    mut arr: *mut NameTable,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const NameRecord,
            *const NameRecord,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<NameRecord>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const NameRecord,
                    *const NameRecord,
                ) -> ::core::ffi::c_int,
            >,
            ComparFn,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn table_name_fill(mut arr: *mut NameTable, mut n: usize) {
    while (*arr).length < n {
        let mut x: NameRecord = NameRecord {
            platformID: 0,
            encodingID: 0,
            languageID: 0,
            nameID: 0,
            nameString: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        if otfcc_iNameRecord.init.is_some() {
            otfcc_iNameRecord.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<NameRecord>() as usize,
            );
        }
        table_name_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_name_push(arr: *mut NameTable, elem: NameRecord) {
    cvec_push(table_name_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_name_grow(arr: *mut NameTable) {
    cvec_grow(table_name_as_cvec(arr));
}
pub static table_iName: NameTableVectorInterface = {
    NameTableVectorInterface {
        init: Some(table_name_init as unsafe extern "C" fn(*mut NameTable) -> ()),
        copy: Some(
            table_name_copy as unsafe extern "C" fn(*mut NameTable, *const NameTable) -> (),
        ),
        move_0: Some(
            table_name_move as unsafe extern "C" fn(*mut NameTable, *mut NameTable) -> (),
        ),
        dispose: Some(table_name_dispose as unsafe extern "C" fn(*mut NameTable) -> ()),
        replace: Some(
            table_name_replace as unsafe extern "C" fn(*mut NameTable, NameTable) -> (),
        ),
        copyReplace: Some(
            table_name_copyReplace as unsafe extern "C" fn(*mut NameTable, NameTable) -> (),
        ),
        create: Some(table_name_create),
        free: Some(table_name_free as unsafe extern "C" fn(*mut NameTable) -> ()),
        initN: Some(table_name_initN as unsafe extern "C" fn(*mut NameTable, usize) -> ()),
        initCapN: Some(table_name_initCapN as unsafe extern "C" fn(*mut NameTable, usize) -> ()),
        createN: Some(table_name_createN as unsafe extern "C" fn(usize) -> *mut NameTable),
        fill: Some(table_name_fill as unsafe extern "C" fn(*mut NameTable, usize) -> ()),
        clear: Some(table_name_dispose as unsafe extern "C" fn(*mut NameTable) -> ()),
        push: Some(
            table_name_push as unsafe extern "C" fn(*mut NameTable, NameRecord) -> (),
        ),
        shrinkToFit: Some(table_name_shrinkToFit as unsafe extern "C" fn(*mut NameTable) -> ()),
        pop: Some(table_name_pop as unsafe extern "C" fn(*mut NameTable) -> NameRecord),
        disposeItem: Some(
            table_name_disposeItem as unsafe extern "C" fn(*mut NameTable, usize) -> (),
        ),
        filterEnv: Some(
            table_name_filterEnv
                as unsafe extern "C" fn(
                    *mut NameTable,
                    Option<
                        unsafe extern "C" fn(
                            *const NameRecord,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_name_sort
                as unsafe extern "C" fn(
                    *mut NameTable,
                    Option<
                        unsafe extern "C" fn(
                            *const NameRecord,
                            *const NameRecord,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn table_name_pop(arr: *mut NameTable) -> NameRecord {
    cvec_pop(table_name_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_name_copyReplace(mut dst: *mut NameTable, src: NameTable) {
    table_name_dispose(dst);
    table_name_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_name_copy(mut dst: *mut NameTable, mut src: *const NameTable) {
    table_name_init(dst);
    table_name_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otfcc_iNameRecord.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otfcc_iNameRecord.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut NameRecord,
                (*src).items.offset(j as isize) as *mut NameRecord as *const NameRecord,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn table_name_dispose(mut arr: *mut NameTable) {
    if arr.is_null() {
        return;
    }
    if otfcc_iNameRecord.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            otfcc_iNameRecord
                .dispose
                .expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut NameRecord
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<NameRecord>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn table_name_replace(mut dst: *mut NameTable, src: NameTable) {
    table_name_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<NameTable>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_name_initCapN(mut arr: *mut NameTable, mut n: usize) {
    table_name_init(arr);
    table_name_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn table_name_growToN(arr: *mut NameTable, target: usize) {
    cvec_grow_to_n(table_name_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_name_initN(mut arr: *mut NameTable, mut n: usize) {
    table_name_init(arr);
    table_name_growToN(arr, n);
    table_name_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_name_free(mut x: *mut NameTable) {
    if x.is_null() {
        return;
    }
    table_name_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_name_createN(mut n: usize) -> *mut NameTable {
    let mut t: *mut NameTable =
        malloc(::core::mem::size_of::<NameTable>() as usize) as *mut NameTable;
    table_name_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_name_create() -> *mut NameTable {
    let mut x: *mut NameTable =
        malloc(::core::mem::size_of::<NameTable>() as usize) as *mut NameTable;
    table_name_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_name_shrinkToFit(mut arr: *mut NameTable) {
    table_name_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_name_resizeTo(arr: *mut NameTable, target: usize) {
    cvec_resize_to(table_name_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_name_move(dst: *mut NameTable, src: *mut NameTable) {
    cvec_move(table_name_as_cvec(dst), table_name_as_cvec(src));
}
unsafe extern "C" fn shouldDecodeAsUTF16(mut record: *const NameRecord) -> bool {
    return (*record).platformID as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*record).platformID as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            && (*record).encodingID as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        || (*record).platformID as ::core::ffi::c_int == 3 as ::core::ffi::c_int
            && ((*record).encodingID as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*record).encodingID as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                || (*record).encodingID as ::core::ffi::c_int == 10 as ::core::ffi::c_int);
}
unsafe extern "C" fn shouldDecodeAsBytes(mut record: *const NameRecord) -> bool {
    return (*record).platformID as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && (*record).encodingID as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*record).languageID as ::core::ffi::c_int == 0 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn otfcc_readName(
    packet: Packet,
    mut options: *const Options,
) -> *mut NameTable {
    let mut count: u32 = 0;
    let mut stringOffset: u32 = 0;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1851878757i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut name: *mut NameTable = ::core::ptr::null_mut::<NameTable>();
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if !(length < 6 as u32) {
                        count = read_16u(
                            data.offset(2 as ::core::ffi::c_int as isize) as *const u8
                        ) as u32;
                        stringOffset = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        ) as u32;
                        if !(length
                            < (6 as u32).wrapping_add((12 as u32).wrapping_mul(count)))
                        {
                            name = (
                                table_iName.create.expect("non-null function pointer"))();
                            let mut j: u16 = 0 as u16;
                            while (j as u32) < count {
                                let mut record: NameRecord = NameRecord {
                                    platformID: 0,
                                    encodingID: 0,
                                    languageID: 0,
                                    nameID: 0,
                                    nameString: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                };
                                record.platformID = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize).offset(
                                        (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                                );
                                record.encodingID = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                record.languageID = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(4 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                record.nameID = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(6 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                record.nameString = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                let mut length_0: u16 = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(8 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                let mut offset: u16 = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(10 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                if shouldDecodeAsBytes(&raw mut record) {
                                    let mut nameString: SdsRaw = sdsnewlen(
                                        data.offset(stringOffset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const ::core::ffi::c_void,
                                        length_0 as usize,
                                    );
                                    record.nameString = nameString;
                                } else if shouldDecodeAsUTF16(&raw mut record) {
                                    let mut nameString_0: SdsRaw = utf16be_to_utf8(
                                        data.offset(stringOffset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const u8,
                                        length_0 as ::core::ffi::c_int,
                                    );
                                    record.nameString = nameString_0;
                                } else {
                                    let mut len: usize = 0 as usize;
                                    let mut buf: *mut u8 = base64_encode(
                                        data.offset(stringOffset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const u8,
                                        length_0 as usize,
                                        &raw mut len,
                                    );
                                    record.nameString =
                                        sdsnewlen(buf as *const ::core::ffi::c_void, len);
                                    free(buf as *mut ::core::ffi::c_void);
                                    buf = ::core::ptr::null_mut::<u8>();
                                }
                                table_iName.push.expect("non-null function pointer")(name, record);
                                j = j.wrapping_add(1);
                            }
                            return name;
                        }
                    }
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        log_vl_important,
                        log_type_warning,
                        crate::sdsbuild!(sdsempty(), b"table 'name' corrupted.\n"),
                    );
                    if !name.is_null() {
                        table_iName.free.expect("non-null function pointer")(name);
                        name = ::core::ptr::null_mut::<NameTable>();
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
    return ::core::ptr::null_mut::<NameTable>();
}
pub unsafe extern "C" fn otfcc_dumpName(
    mut name: *const NameTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if name.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"name"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _name: *mut JsonValue = json_array_new((*name).length);
        let mut j: u16 = 0 as u16;
        while (j as usize) < (*name).length {
            let mut r: *mut NameRecord =
                (*name).items.offset(j as isize) as *mut NameRecord;
            let mut record: *mut JsonValue = json_object_new(5 as usize);
            json_object_push(
                record,
                b"platformID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).platformID as i64),
            );
            json_object_push(
                record,
                b"encodingID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).encodingID as i64),
            );
            json_object_push(
                record,
                b"languageID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).languageID as i64),
            );
            json_object_push(
                record,
                b"nameID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).nameID as i64),
            );
            json_object_push(
                record,
                b"nameString\0" as *const u8 as *const ::core::ffi::c_char,
                json_string_new_length(
                    sdslen((*r).nameString) as ::core::ffi::c_uint,
                    (*r).nameString as *const ::core::ffi::c_char,
                ),
            );
            json_array_push(_name, record);
            j = j.wrapping_add(1);
        }
        json_object_push(
            root,
            b"name\0" as *const u8 as *const ::core::ffi::c_char,
            _name,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
unsafe extern "C" fn name_record_sort(
    mut a: *const NameRecord,
    mut b: *const NameRecord,
) -> ::core::ffi::c_int {
    if (*a).platformID as ::core::ffi::c_int != (*b).platformID as ::core::ffi::c_int {
        return (*a).platformID as ::core::ffi::c_int - (*b).platformID as ::core::ffi::c_int;
    }
    if (*a).encodingID as ::core::ffi::c_int != (*b).encodingID as ::core::ffi::c_int {
        return (*a).encodingID as ::core::ffi::c_int - (*b).encodingID as ::core::ffi::c_int;
    }
    if (*a).languageID as ::core::ffi::c_int != (*b).languageID as ::core::ffi::c_int {
        return (*a).languageID as ::core::ffi::c_int - (*b).languageID as ::core::ffi::c_int;
    }
    return (*a).nameID as ::core::ffi::c_int - (*b).nameID as ::core::ffi::c_int;
}
pub unsafe extern "C" fn otfcc_parseName(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut NameTable {
    let mut name: *mut NameTable = (
        table_iName.create.expect("non-null function pointer"))();
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"name"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut j: u32 = 0 as u32;
            while j < (*table).u.array.length as u32 {
                if !(*(*table).u.array.values.offset(j as isize)).is_null()
                    && (**(*table).u.array.values.offset(j as isize)).type_0 == json_object
                {
                    let mut _record: *mut JsonValue =
                        *(*table).u.array.values.offset(j as isize) as *mut JsonValue;
                    if json_obj_get_type(
                        _record,
                        b"platformID\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            log_vl_important,
                            log_type_warning,
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Missing or invalid platformID for name entry ",
                                j,
                                b"\n",
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"encodingID\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            log_vl_important,
                            log_type_warning,
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Missing or invalid encodingID for name entry ",
                                j,
                                b"\n",
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"languageID\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            log_vl_important,
                            log_type_warning,
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Missing or invalid languageID for name entry ",
                                j,
                                b"\n",
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"nameID\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            log_vl_important,
                            log_type_warning,
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Missing or invalid nameID for name entry ",
                                j,
                                b"\n",
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"nameString\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .logSDS
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            log_vl_important,
                            log_type_warning,
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Missing or invalid name string for name entry ",
                                j,
                                b"\n",
                            ),
                        );
                    } else {
                        let mut record: NameRecord = NameRecord {
                            platformID: 0,
                            encodingID: 0,
                            languageID: 0,
                            nameID: 0,
                            nameString: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        };
                        record.platformID = json_obj_getint(
                            _record,
                            b"platformID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as u16;
                        record.encodingID = json_obj_getint(
                            _record,
                            b"encodingID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as u16;
                        record.languageID = json_obj_getint(
                            _record,
                            b"languageID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as u16;
                        record.nameID = json_obj_getint(
                            _record,
                            b"nameID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as u16;
                        let mut str: *mut JsonValue = json_obj_get_type(
                            _record,
                            b"nameString\0" as *const u8 as *const ::core::ffi::c_char,
                            json_string,
                        );
                        record.nameString = sdsnewlen(
                            (*str).u.string.ptr as *const ::core::ffi::c_void,
                            (*str).u.string.length as usize,
                        );
                        table_iName.push.expect("non-null function pointer")(name, record);
                    }
                }
                j = j.wrapping_add(1);
            }
            table_iName.sort.expect("non-null function pointer")(
                name,
                Some(
                    name_record_sort
                        as unsafe extern "C" fn(
                            *const NameRecord,
                            *const NameRecord,
                        ) -> ::core::ffi::c_int,
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
    return name;
}
pub unsafe extern "C" fn otfcc_buildName(
    mut name: *const NameTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if name.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, (*name).length as u16);
    bufwrite16b(buf, 0 as u16);
    let mut strings: *mut Buffer = bufnew();
    let mut j: u16 = 0 as u16;
    while (j as usize) < (*name).length {
        let mut record: *mut NameRecord =
            (*name).items.offset(j as isize) as *mut NameRecord;
        bufwrite16b(buf, (*record).platformID);
        bufwrite16b(buf, (*record).encodingID);
        bufwrite16b(buf, (*record).languageID);
        bufwrite16b(buf, (*record).nameID);
        let mut cbefore: usize = (*strings).cursor;
        if shouldDecodeAsUTF16(record) {
            let mut words: usize = 0;
            let mut u16: *mut u8 = utf8toutf16be((*record).nameString, &raw mut words);
            bufwrite_bytes(strings, words, u16);
            free(u16 as *mut ::core::ffi::c_void);
            u16 = ::core::ptr::null_mut::<u8>();
        } else if shouldDecodeAsBytes(record) {
            bufwrite_bytes(
                strings,
                sdslen((*record).nameString),
                (*record).nameString as *mut u8,
            );
        } else {
            let mut length: usize = 0;
            let mut decoded: *mut u8 = base64_decode(
                (*record).nameString as *mut u8,
                sdslen((*record).nameString),
                &raw mut length,
            );
            bufwrite_bytes(strings, length, decoded);
            free(decoded as *mut ::core::ffi::c_void);
            decoded = ::core::ptr::null_mut::<u8>();
        }
        let mut cafter: usize = (*strings).cursor;
        bufwrite16b(buf, cafter.wrapping_sub(cbefore) as u16);
        bufwrite16b(buf, cbefore as u16);
        j = j.wrapping_add(1);
    }
    let mut copyright: SdsRaw = crate::sdsbuild!(
        sdsempty(),
        b"-- By OTFCC ",
        MAIN_VER,
        b".",
        SECONDARY_VER,
        b".",
        PATCH_VER,
        b" --",
    );
    // `sdsgrowzero` may reallocate, so its result has to be assigned back.
    // `name.c:188` drops it -- a use-after-free that has never fired only
    // because `sdscatprintf` happened to over-allocate: it grew the buffer to
    // twice the 21-byte version string, and 42 bytes is (just) enough for the
    // 32 this then asks for. Appending the string in pieces allocates 24, so
    // the growth reallocates, and the stale pointer aborts in `sdsfree`.
    copyright = sdsgrowzero(copyright, COPYRIGHT_LEN as usize);
    bufwrite_bytes(strings, COPYRIGHT_LEN as usize, copyright as *mut u8);
    sdsfree(copyright);
    let mut stringsOffset: usize = (*buf).cursor;
    bufwrite_buf(buf, strings);
    bufseek(buf, 4 as usize);
    bufwrite16b(buf, stringsOffset as u16);
    buffree(strings);
    return buf;
}
