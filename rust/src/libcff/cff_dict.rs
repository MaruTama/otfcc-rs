extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite_bufdel(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
    fn cff_decodeCffToken(start: *const uint8_t, val: *mut cff_Value) -> uint32_t;
    fn cff_encodeCffOperator(val: int32_t) -> *mut caryll_Buffer;
    fn cff_encodeCffInteger(val: int32_t) -> *mut caryll_Buffer;
    fn cff_encodeCffFloat(val: ::core::ffi::c_double) -> *mut caryll_Buffer;
}

use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type cff_Value_Type = ::core::ffi::c_uint;
pub const CS2_FRACTION: cff_Value_Type = 3;
pub const cff_DOUBLE: cff_Value_Type = 3;
pub const CS2_OPERAND: cff_Value_Type = 2;
pub const cff_INTEGER: cff_Value_Type = 2;
pub const CS2_OPERATOR: cff_Value_Type = 1;
pub const cff_OPERATOR: cff_Value_Type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Value {
    pub t: cff_Value_Type,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i: int32_t,
    pub d: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_DictEntry {
    pub op: uint32_t,
    pub cnt: uint32_t,
    pub vals: *mut cff_Value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Dict {
    pub count: uint32_t,
    pub ents: *mut cff_DictEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_cff_Dict {
    pub init: Option<unsafe extern "C" fn(*mut cff_Dict) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut cff_Dict, *const cff_Dict) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut cff_Dict, *mut cff_Dict) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut cff_Dict) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut cff_Dict, cff_Dict) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut cff_Dict, cff_Dict) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut cff_Dict>,
    pub free: Option<unsafe extern "C" fn(*mut cff_Dict) -> ()>,
    pub parse: Option<unsafe extern "C" fn(*const uint8_t, uint32_t) -> *mut cff_Dict>,
    pub parseToCallback: Option<
        unsafe extern "C" fn(
            *const uint8_t,
            uint32_t,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    uint32_t,
                    uint8_t,
                    *mut cff_Value,
                    *mut ::core::ffi::c_void,
                ) -> (),
            >,
        ) -> (),
    >,
    pub parseDictKey:
        Option<unsafe extern "C" fn(*const uint8_t, uint32_t, uint32_t, uint32_t) -> cff_Value>,
    pub build: Option<unsafe extern "C" fn(*const cff_Dict) -> *mut caryll_Buffer>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_get_key_context {
    pub found: bool,
    pub res: cff_Value,
    pub op: uint32_t,
    pub idx: uint32_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn disposeDict(mut dict: *mut cff_Dict) {
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*dict).count {
        free((*(*dict).ents.offset(j as isize)).vals as *mut ::core::ffi::c_void);
        let ref mut fresh3 = (*(*dict).ents.offset(j as isize)).vals;
        *fresh3 = ::core::ptr::null_mut::<cff_Value>();
        j = j.wrapping_add(1);
    }
    free((*dict).ents as *mut ::core::ffi::c_void);
    (*dict).ents = ::core::ptr::null_mut::<cff_DictEntry>();
}
#[inline]
unsafe extern "C" fn cff_Dict_copyReplace(mut dst: *mut cff_Dict, src: cff_Dict) {
    cff_Dict_dispose(dst);
    cff_Dict_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn cff_Dict_create() -> *mut cff_Dict {
    let mut x: *mut cff_Dict =
        malloc(::core::mem::size_of::<cff_Dict>() as size_t) as *mut cff_Dict;
    cff_Dict_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn cff_Dict_free(mut x: *mut cff_Dict) {
    if x.is_null() {
        return;
    }
    cff_Dict_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn cff_Dict_dispose(mut x: *mut cff_Dict) {
    disposeDict(x);
}
#[inline]
unsafe extern "C" fn cff_Dict_init(mut x: *mut cff_Dict) {
    memset(
        x as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cff_Dict>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn cff_Dict_copy(mut dst: *mut cff_Dict, mut src: *const cff_Dict) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_Dict>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn cff_Dict_replace(mut dst: *mut cff_Dict, src: cff_Dict) {
    cff_Dict_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_Dict>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn cff_Dict_move(mut dst: *mut cff_Dict, mut src: *mut cff_Dict) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_Dict>() as size_t,
    );
    cff_Dict_init(src);
}
unsafe extern "C" fn parseDict(mut data: *const uint8_t, len: uint32_t) -> *mut cff_Dict {
    let mut dict: *mut cff_Dict = ::core::ptr::null_mut::<cff_Dict>();
    dict = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_Dict>() as size_t,
        14 as ::core::ffi::c_ulong,
    ) as *mut cff_Dict;
    let mut index: uint32_t = 0 as uint32_t;
    let mut advance: uint32_t = 0;
    let mut val: cff_Value = cff_Value {
        t: 0 as cff_Value_Type,
        c2rust_unnamed: C2RustUnnamed { i: 0 },
    };
    let mut stack: [cff_Value; 48] = [cff_Value {
        t: 0 as cff_Value_Type,
        c2rust_unnamed: C2RustUnnamed { i: 0 },
    }; 48];
    let mut temp: *const uint8_t = data;
    while temp < data.offset(len as isize) {
        advance = cff_decodeCffToken(temp, &raw mut val);
        match val.t as ::core::ffi::c_uint {
            1 => {
                (*dict).ents = __caryll_reallocate(
                    (*dict).ents as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<cff_DictEntry>() as size_t)
                        .wrapping_mul((*dict).count.wrapping_add(1 as uint32_t) as size_t),
                    24 as ::core::ffi::c_ulong,
                ) as *mut cff_DictEntry;
                (*(*dict).ents.offset((*dict).count as isize)).op =
                    val.c2rust_unnamed.i as uint32_t;
                (*(*dict).ents.offset((*dict).count as isize)).cnt = index;
                let ref mut fresh1 = (*(*dict).ents.offset((*dict).count as isize)).vals;
                *fresh1 = __caryll_allocate_clean(
                    (::core::mem::size_of::<cff_Value>() as size_t).wrapping_mul(index as size_t),
                    27 as ::core::ffi::c_ulong,
                ) as *mut cff_Value;
                memcpy(
                    (*(*dict).ents.offset((*dict).count as isize)).vals as *mut ::core::ffi::c_void,
                    &raw mut stack as *mut cff_Value as *const ::core::ffi::c_void,
                    (::core::mem::size_of::<cff_Value>() as size_t).wrapping_mul(index as size_t),
                );
                (*dict).count = (*dict).count.wrapping_add(1);
                index = 0 as uint32_t;
            }
            2 | 3 => {
                let fresh2 = index;
                index = index.wrapping_add(1);
                stack[fresh2 as usize] = val;
            }
            _ => {}
        }
        temp = temp.offset(advance as isize);
    }
    return dict;
}
unsafe extern "C" fn parseToCallback(
    mut data: *const uint8_t,
    len: uint32_t,
    mut context: *mut ::core::ffi::c_void,
    mut callback: Option<
        unsafe extern "C" fn(uint32_t, uint8_t, *mut cff_Value, *mut ::core::ffi::c_void) -> (),
    >,
) {
    let mut index: uint8_t = 0 as uint8_t;
    let mut advance: uint32_t = 0;
    let mut val: cff_Value = cff_Value {
        t: 0 as cff_Value_Type,
        c2rust_unnamed: C2RustUnnamed { i: 0 },
    };
    let mut stack: [cff_Value; 256] = [cff_Value {
        t: 0 as cff_Value_Type,
        c2rust_unnamed: C2RustUnnamed { i: 0 },
    }; 256];
    let mut temp: *const uint8_t = data;
    while temp < data.offset(len as isize) {
        advance = cff_decodeCffToken(temp, &raw mut val);
        match val.t as ::core::ffi::c_uint {
            1 => {
                callback.expect("non-null function pointer")(
                    val.c2rust_unnamed.i as uint32_t,
                    index,
                    &raw mut stack as *mut cff_Value,
                    context,
                );
                index = 0 as uint8_t;
            }
            2 | 3 => {
                let fresh0 = index;
                index = index.wrapping_add(1);
                stack[fresh0 as usize] = val;
            }
            _ => {}
        }
        temp = temp.offset(advance as isize);
    }
}
unsafe extern "C" fn callback_get_key(
    mut op: uint32_t,
    mut top: uint8_t,
    mut stack: *mut cff_Value,
    mut _context: *mut ::core::ffi::c_void,
) {
    let mut context: *mut cff_get_key_context = _context as *mut cff_get_key_context;
    if op == (*context).op && (*context).idx <= top as uint32_t {
        (*context).found = true;
        (*context).res = *stack.offset((*context).idx as isize);
    }
}
unsafe extern "C" fn parseDictKey(
    mut data: *const uint8_t,
    len: uint32_t,
    op: uint32_t,
    idx: uint32_t,
) -> cff_Value {
    let mut context: cff_get_key_context = cff_get_key_context {
        found: false,
        res: cff_Value {
            t: 0 as cff_Value_Type,
            c2rust_unnamed: C2RustUnnamed { i: 0 },
        },
        op: 0,
        idx: 0,
    };
    context.found = false;
    context.idx = idx;
    context.op = op;
    context.res.t = 0 as cff_Value_Type;
    context.res.c2rust_unnamed.i = -(1 as ::core::ffi::c_int) as int32_t;
    parseToCallback(
        data,
        len,
        &raw mut context as *mut ::core::ffi::c_void,
        Some(
            callback_get_key
                as unsafe extern "C" fn(
                    uint32_t,
                    uint8_t,
                    *mut cff_Value,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
    );
    return context.res;
}
unsafe extern "C" fn buildDict(mut dict: *const cff_Dict) -> *mut caryll_Buffer {
    let mut blob: *mut caryll_Buffer = bufnew();
    let mut i: uint32_t = 0 as uint32_t;
    while i < (*dict).count {
        let mut j: uint32_t = 0 as uint32_t;
        while j < (*(*dict).ents.offset(i as isize)).cnt {
            let mut blob_val: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
            if (*(*(*dict).ents.offset(i as isize)).vals.offset(j as isize)).t
                as ::core::ffi::c_uint
                == cff_INTEGER as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                blob_val = cff_encodeCffInteger(
                    (*(*(*dict).ents.offset(i as isize)).vals.offset(j as isize))
                        .c2rust_unnamed
                        .i,
                );
            } else if (*(*(*dict).ents.offset(i as isize)).vals.offset(j as isize)).t
                as ::core::ffi::c_uint
                == cff_DOUBLE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                blob_val = cff_encodeCffFloat(
                    (*(*(*dict).ents.offset(i as isize)).vals.offset(j as isize))
                        .c2rust_unnamed
                        .d,
                );
            } else {
                blob_val = cff_encodeCffInteger(0 as int32_t);
            }
            bufwrite_bufdel(blob, blob_val);
            j = j.wrapping_add(1);
        }
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator((*(*dict).ents.offset(i as isize)).op as int32_t),
        );
        i = i.wrapping_add(1);
    }
    return blob;
}
#[no_mangle]
pub static mut cff_iDict: __caryll_elementinterface_cff_Dict = {
    __caryll_elementinterface_cff_Dict {
        init: Some(cff_Dict_init as unsafe extern "C" fn(*mut cff_Dict) -> ()),
        copy: Some(cff_Dict_copy as unsafe extern "C" fn(*mut cff_Dict, *const cff_Dict) -> ()),
        move_0: Some(cff_Dict_move as unsafe extern "C" fn(*mut cff_Dict, *mut cff_Dict) -> ()),
        dispose: Some(cff_Dict_dispose as unsafe extern "C" fn(*mut cff_Dict) -> ()),
        replace: Some(cff_Dict_replace as unsafe extern "C" fn(*mut cff_Dict, cff_Dict) -> ()),
        copyReplace: Some(
            cff_Dict_copyReplace as unsafe extern "C" fn(*mut cff_Dict, cff_Dict) -> (),
        ),
        create: Some(cff_Dict_create),
        free: Some(cff_Dict_free as unsafe extern "C" fn(*mut cff_Dict) -> ()),
        parse: Some(parseDict as unsafe extern "C" fn(*const uint8_t, uint32_t) -> *mut cff_Dict),
        parseToCallback: Some(
            parseToCallback
                as unsafe extern "C" fn(
                    *const uint8_t,
                    uint32_t,
                    *mut ::core::ffi::c_void,
                    Option<
                        unsafe extern "C" fn(
                            uint32_t,
                            uint8_t,
                            *mut cff_Value,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                    >,
                ) -> (),
        ),
        parseDictKey: Some(
            parseDictKey
                as unsafe extern "C" fn(*const uint8_t, uint32_t, uint32_t, uint32_t) -> cff_Value,
        ),
        build: Some(buildDict as unsafe extern "C" fn(*const cff_Dict) -> *mut caryll_Buffer),
    }
};
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
