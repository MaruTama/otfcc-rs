use libc::{free, malloc, memcpy, memset, qsort, strcmp};
extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdsgrowzero(s: sds, len: usize) -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn buffree(buf: *mut caryll_Buffer);
    fn bufseek(buf: *mut caryll_Buffer, pos: usize);
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite_bytes(buf: *mut caryll_Buffer, size: usize, str: *const u8);
    fn bufwrite_buf(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new_length(
        length: ::core::ffi::c_uint,
        _: *const ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn base64_encode(src: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
    fn base64_decode(src: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
    fn utf16be_to_utf8(inb: *const u8, inlenb: ::core::ffi::c_int) -> sds;
    fn utf8toutf16be(_in: sds, out_bytes: *mut usize) -> *mut u8;
}
use crate::support::binio::{read_16u};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, sds, sdshdr16, sdshdr32, sdshdr64, sdshdr8};
use crate::vendor::json::{json_array, json_double, json_integer, json_object, json_string, json_type, json_value};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};
use crate::support::{__compar_fn_t};
use crate::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_NameRecord {
    pub platformID: u16,
    pub encodingID: u16,
    pub languageID: u16,
    pub nameID: u16,
    pub nameString: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otfcc_NameRecord {
    pub init: Option<unsafe extern "C" fn(*mut otfcc_NameRecord) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otfcc_NameRecord, *const otfcc_NameRecord) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otfcc_NameRecord, *mut otfcc_NameRecord) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_NameRecord) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otfcc_NameRecord, otfcc_NameRecord) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otfcc_NameRecord, otfcc_NameRecord) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_name {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut otfcc_NameRecord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_table_name {
    pub init: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_name, *const table_name) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_name, *mut table_name) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_name, table_name) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_name, table_name) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_name>,
    pub free: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut table_name, usize) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut table_name, usize) -> ()>,
    pub createN: Option<unsafe extern "C" fn(usize) -> *mut table_name>,
    pub fill: Option<unsafe extern "C" fn(*mut table_name, usize) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut table_name, otfcc_NameRecord) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut table_name) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut table_name) -> otfcc_NameRecord>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut table_name, usize) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut table_name,
            Option<unsafe extern "C" fn(*const otfcc_NameRecord, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut table_name,
            Option<
                unsafe extern "C" fn(
                    *const otfcc_NameRecord,
                    *const otfcc_NameRecord,
                ) -> ::core::ffi::c_int,
            >,
        ) -> (),
    >,
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
pub const COPYRIGHT_LEN: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
unsafe extern "C" fn nameRecordDtor(mut entry: *mut otfcc_NameRecord) {
    sdsfree((*entry).nameString);
    (*entry).nameString = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_move(
    mut dst: *mut otfcc_NameRecord,
    mut src: *mut otfcc_NameRecord,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_NameRecord>() as usize,
    );
    otfcc_NameRecord_init(src);
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_init(mut x: *mut otfcc_NameRecord) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<otfcc_NameRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_replace(
    mut dst: *mut otfcc_NameRecord,
    src: otfcc_NameRecord,
) {
    otfcc_NameRecord_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_NameRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_copyReplace(
    mut dst: *mut otfcc_NameRecord,
    src: otfcc_NameRecord,
) {
    otfcc_NameRecord_dispose(dst);
    otfcc_NameRecord_copy(dst, &raw const src);
}
#[no_mangle]
pub static mut otfcc_iNameRecord: __caryll_elementinterface_otfcc_NameRecord = {
    __caryll_elementinterface_otfcc_NameRecord {
        init: Some(otfcc_NameRecord_init as unsafe extern "C" fn(*mut otfcc_NameRecord) -> ()),
        copy: Some(
            otfcc_NameRecord_copy
                as unsafe extern "C" fn(*mut otfcc_NameRecord, *const otfcc_NameRecord) -> (),
        ),
        move_0: Some(
            otfcc_NameRecord_move
                as unsafe extern "C" fn(*mut otfcc_NameRecord, *mut otfcc_NameRecord) -> (),
        ),
        dispose: Some(
            otfcc_NameRecord_dispose as unsafe extern "C" fn(*mut otfcc_NameRecord) -> (),
        ),
        replace: Some(
            otfcc_NameRecord_replace
                as unsafe extern "C" fn(*mut otfcc_NameRecord, otfcc_NameRecord) -> (),
        ),
        copyReplace: Some(
            otfcc_NameRecord_copyReplace
                as unsafe extern "C" fn(*mut otfcc_NameRecord, otfcc_NameRecord) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn otfcc_NameRecord_dispose(mut x: *mut otfcc_NameRecord) {
    nameRecordDtor(x);
}
#[inline]
unsafe extern "C" fn otfcc_NameRecord_copy(
    mut dst: *mut otfcc_NameRecord,
    mut src: *const otfcc_NameRecord,
) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_NameRecord>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_name_growTo(arr: *mut table_name, target: usize) {
    cvec_grow_to(table_name_as_cvec(arr), target);
}
#[inline]
unsafe fn table_name_as_cvec(arr: *mut table_name) -> *mut CVecRaw<otfcc_NameRecord> {
    arr as *mut CVecRaw<otfcc_NameRecord>
}
#[inline]
unsafe extern "C" fn table_name_init(arr: *mut table_name) {
    cvec_init(table_name_as_cvec(arr));
}
#[inline]
unsafe extern "C" fn table_name_filterEnv(
    mut arr: *mut table_name,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otfcc_NameRecord, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otfcc_NameRecord,
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
                    (*arr).items.offset(k as isize) as *mut otfcc_NameRecord,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe extern "C" fn table_name_disposeItem(mut arr: *mut table_name, mut n: usize) {
    if otfcc_iNameRecord.dispose.is_some() {
        otfcc_iNameRecord
            .dispose
            .expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otfcc_NameRecord
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn table_name_sort(
    mut arr: *mut table_name,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otfcc_NameRecord,
            *const otfcc_NameRecord,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otfcc_NameRecord>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otfcc_NameRecord,
                    *const otfcc_NameRecord,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn table_name_fill(mut arr: *mut table_name, mut n: usize) {
    while (*arr).length < n {
        let mut x: otfcc_NameRecord = otfcc_NameRecord {
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
                ::core::mem::size_of::<otfcc_NameRecord>() as usize,
            );
        }
        table_name_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn table_name_push(arr: *mut table_name, elem: otfcc_NameRecord) {
    cvec_push(table_name_as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn table_name_grow(arr: *mut table_name) {
    cvec_grow(table_name_as_cvec(arr));
}
#[no_mangle]
pub static mut table_iName: __caryll_vectorinterface_table_name = {
    __caryll_vectorinterface_table_name {
        init: Some(table_name_init as unsafe extern "C" fn(*mut table_name) -> ()),
        copy: Some(
            table_name_copy as unsafe extern "C" fn(*mut table_name, *const table_name) -> (),
        ),
        move_0: Some(
            table_name_move as unsafe extern "C" fn(*mut table_name, *mut table_name) -> (),
        ),
        dispose: Some(table_name_dispose as unsafe extern "C" fn(*mut table_name) -> ()),
        replace: Some(
            table_name_replace as unsafe extern "C" fn(*mut table_name, table_name) -> (),
        ),
        copyReplace: Some(
            table_name_copyReplace as unsafe extern "C" fn(*mut table_name, table_name) -> (),
        ),
        create: Some(table_name_create),
        free: Some(table_name_free as unsafe extern "C" fn(*mut table_name) -> ()),
        initN: Some(table_name_initN as unsafe extern "C" fn(*mut table_name, usize) -> ()),
        initCapN: Some(table_name_initCapN as unsafe extern "C" fn(*mut table_name, usize) -> ()),
        createN: Some(table_name_createN as unsafe extern "C" fn(usize) -> *mut table_name),
        fill: Some(table_name_fill as unsafe extern "C" fn(*mut table_name, usize) -> ()),
        clear: Some(table_name_dispose as unsafe extern "C" fn(*mut table_name) -> ()),
        push: Some(
            table_name_push as unsafe extern "C" fn(*mut table_name, otfcc_NameRecord) -> (),
        ),
        shrinkToFit: Some(table_name_shrinkToFit as unsafe extern "C" fn(*mut table_name) -> ()),
        pop: Some(table_name_pop as unsafe extern "C" fn(*mut table_name) -> otfcc_NameRecord),
        disposeItem: Some(
            table_name_disposeItem as unsafe extern "C" fn(*mut table_name, usize) -> (),
        ),
        filterEnv: Some(
            table_name_filterEnv
                as unsafe extern "C" fn(
                    *mut table_name,
                    Option<
                        unsafe extern "C" fn(
                            *const otfcc_NameRecord,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            table_name_sort
                as unsafe extern "C" fn(
                    *mut table_name,
                    Option<
                        unsafe extern "C" fn(
                            *const otfcc_NameRecord,
                            *const otfcc_NameRecord,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn table_name_pop(arr: *mut table_name) -> otfcc_NameRecord {
    cvec_pop(table_name_as_cvec(arr))
}
#[inline]
unsafe extern "C" fn table_name_copyReplace(mut dst: *mut table_name, src: table_name) {
    table_name_dispose(dst);
    table_name_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_name_copy(mut dst: *mut table_name, mut src: *const table_name) {
    table_name_init(dst);
    table_name_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if otfcc_iNameRecord.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            otfcc_iNameRecord.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otfcc_NameRecord,
                (*src).items.offset(j as isize) as *mut otfcc_NameRecord as *const otfcc_NameRecord,
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
unsafe extern "C" fn table_name_dispose(mut arr: *mut table_name) {
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
                (*arr).items.offset(j as isize) as *mut otfcc_NameRecord
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otfcc_NameRecord>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn table_name_replace(mut dst: *mut table_name, src: table_name) {
    table_name_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_name>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_name_initCapN(mut arr: *mut table_name, mut n: usize) {
    table_name_init(arr);
    table_name_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn table_name_growToN(arr: *mut table_name, target: usize) {
    cvec_grow_to_n(table_name_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_name_initN(mut arr: *mut table_name, mut n: usize) {
    table_name_init(arr);
    table_name_growToN(arr, n);
    table_name_fill(arr, n);
}
#[inline]
unsafe extern "C" fn table_name_free(mut x: *mut table_name) {
    if x.is_null() {
        return;
    }
    table_name_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn table_name_createN(mut n: usize) -> *mut table_name {
    let mut t: *mut table_name =
        malloc(::core::mem::size_of::<table_name>() as usize) as *mut table_name;
    table_name_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn table_name_create() -> *mut table_name {
    let mut x: *mut table_name =
        malloc(::core::mem::size_of::<table_name>() as usize) as *mut table_name;
    table_name_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_name_shrinkToFit(mut arr: *mut table_name) {
    table_name_resizeTo(arr, (*arr).length);
}
#[inline]
unsafe extern "C" fn table_name_resizeTo(arr: *mut table_name, target: usize) {
    cvec_resize_to(table_name_as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn table_name_move(dst: *mut table_name, src: *mut table_name) {
    cvec_move(table_name_as_cvec(dst), table_name_as_cvec(src));
}
unsafe extern "C" fn shouldDecodeAsUTF16(mut record: *const otfcc_NameRecord) -> bool {
    return (*record).platformID as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*record).platformID as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            && (*record).encodingID as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        || (*record).platformID as ::core::ffi::c_int == 3 as ::core::ffi::c_int
            && ((*record).encodingID as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*record).encodingID as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                || (*record).encodingID as ::core::ffi::c_int == 10 as ::core::ffi::c_int);
}
unsafe extern "C" fn shouldDecodeAsBytes(mut record: *const otfcc_NameRecord) -> bool {
    return (*record).platformID as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && (*record).encodingID as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*record).languageID as ::core::ffi::c_int == 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readName(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_name {
    let mut count: u32 = 0;
    let mut stringOffset: u32 = 0;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1851878757i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut name: *mut table_name = ::core::ptr::null_mut::<table_name>();
                    let mut data: font_file_pointer = table.data as font_file_pointer;
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
                                let mut record: otfcc_NameRecord = otfcc_NameRecord {
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
                                    let mut nameString: sds = sdsnewlen(
                                        data.offset(stringOffset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const ::core::ffi::c_void,
                                        length_0 as usize,
                                    );
                                    record.nameString = nameString;
                                } else if shouldDecodeAsUTF16(&raw mut record) {
                                    let mut nameString_0: sds = utf16be_to_utf8(
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
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as u8,
                        log_type_warning,
                        crate::sdsbuild!(sdsempty(), b"table 'name' corrupted.\n"),
                    );
                    if !name.is_null() {
                        table_iName.free.expect("non-null function pointer")(name);
                        name = ::core::ptr::null_mut::<table_name>();
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
    return ::core::ptr::null_mut::<table_name>();
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpName(
    mut name: *const table_name,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if name.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        crate::sdsbuild!(sdsempty(), b"name"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _name: *mut json_value = json_array_new((*name).length);
        let mut j: u16 = 0 as u16;
        while (j as usize) < (*name).length {
            let mut r: *mut otfcc_NameRecord =
                (*name).items.offset(j as isize) as *mut otfcc_NameRecord;
            let mut record: *mut json_value = json_object_new(5 as usize);
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
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
unsafe extern "C" fn name_record_sort(
    mut a: *const otfcc_NameRecord,
    mut b: *const otfcc_NameRecord,
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
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseName(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_name {
    let mut name: *mut table_name = (
        table_iName.create.expect("non-null function pointer"))();
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        json_array,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), b"name"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut j: u32 = 0 as u32;
            while j < (*table).u.array.length as u32 {
                if !(*(*table).u.array.values.offset(j as isize)).is_null()
                    && (**(*table).u.array.values.offset(j as isize)).type_0 as ::core::ffi::c_uint
                        == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let mut _record: *mut json_value =
                        *(*table).u.array.values.offset(j as isize) as *mut json_value;
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
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Missing or invalid platformID for name entry %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                j,
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
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Missing or invalid encodingID for name entry %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                j,
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
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Missing or invalid languageID for name entry %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                j,
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
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Missing or invalid nameID for name entry %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                j,
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
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            sdscatprintf(
                                sdsempty(),
                                b"Missing or invalid name string for name entry %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                j,
                            ),
                        );
                    } else {
                        let mut record: otfcc_NameRecord = otfcc_NameRecord {
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
                        let mut str: *mut json_value = json_obj_get_type(
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
                            *const otfcc_NameRecord,
                            *const otfcc_NameRecord,
                        ) -> ::core::ffi::c_int,
                ),
            );
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildName(
    mut name: *const table_name,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if name.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, (*name).length as u16);
    bufwrite16b(buf, 0 as u16);
    let mut strings: *mut caryll_Buffer = bufnew();
    let mut j: u16 = 0 as u16;
    while (j as usize) < (*name).length {
        let mut record: *mut otfcc_NameRecord =
            (*name).items.offset(j as isize) as *mut otfcc_NameRecord;
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
    let mut copyright: sds = sdscatprintf(
        sdsempty(),
        b"-- By OTFCC %d.%d.%d --\0" as *const u8 as *const ::core::ffi::c_char,
        MAIN_VER,
        SECONDARY_VER,
        PATCH_VER,
    );
    sdsgrowzero(copyright, COPYRIGHT_LEN as usize);
    bufwrite_bytes(strings, COPYRIGHT_LEN as usize, copyright as *mut u8);
    sdsfree(copyright);
    let mut stringsOffset: usize = (*buf).cursor;
    bufwrite_buf(buf, strings);
    bufseek(buf, 4 as usize);
    bufwrite16b(buf, stringsOffset as u16);
    buffree(strings);
    return buf;
}
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_getint(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> i32 {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as i32;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as i32;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl as i32;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return 0 as i32;
}
