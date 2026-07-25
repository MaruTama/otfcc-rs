use libc::{free, malloc, memcpy, memset};
extern "C" {
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite_bufdel(buf: *mut caryll_Buffer, that: *mut caryll_Buffer);
    fn cff_decodeCffToken(start: *const u8, val: *mut cff_Value) -> u32;
    fn cff_encodeCffOperator(val: i32) -> *mut caryll_Buffer;
    fn cff_encodeCffInteger(val: i32) -> *mut caryll_Buffer;
    fn cff_encodeCffFloat(val: ::core::ffi::c_double) -> *mut caryll_Buffer;
}


use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::support::buffer::{caryll_Buffer};
use crate::libcff::cff_value::{cff_DOUBLE, cff_INTEGER, cff_Value, cff_ValueBody, cff_Value_Type};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_DictEntry {
    pub op: u32,
    pub cnt: u32,
    pub vals: *mut cff_Value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Dict {
    pub count: u32,
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
    pub parse: Option<unsafe extern "C" fn(*const u8, u32) -> *mut cff_Dict>,
    pub parseToCallback: Option<
        unsafe extern "C" fn(
            *const u8,
            u32,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    u32,
                    u8,
                    *mut cff_Value,
                    *mut ::core::ffi::c_void,
                ) -> (),
            >,
        ) -> (),
    >,
    pub parseDictKey:
        Option<unsafe extern "C" fn(*const u8, u32, u32, u32) -> cff_Value>,
    pub build: Option<unsafe extern "C" fn(*const cff_Dict) -> *mut caryll_Buffer>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_get_key_context {
    pub found: bool,
    pub res: cff_Value,
    pub op: u32,
    pub idx: u32,
}
#[inline]
unsafe extern "C" fn disposeDict(mut dict: *mut cff_Dict) {
    let mut j: u32 = 0 as u32;
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
        malloc(::core::mem::size_of::<cff_Dict>() as usize) as *mut cff_Dict;
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
        ::core::mem::size_of::<cff_Dict>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cff_Dict_copy(mut dst: *mut cff_Dict, mut src: *const cff_Dict) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_Dict>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cff_Dict_replace(mut dst: *mut cff_Dict, src: cff_Dict) {
    cff_Dict_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_Dict>() as usize,
    );
}
#[inline]
unsafe extern "C" fn cff_Dict_move(mut dst: *mut cff_Dict, mut src: *mut cff_Dict) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cff_Dict>() as usize,
    );
    cff_Dict_init(src);
}
unsafe extern "C" fn parseDict(mut data: *const u8, len: u32) -> *mut cff_Dict {
    let mut dict: *mut cff_Dict = ::core::ptr::null_mut::<cff_Dict>();
    dict = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_Dict>() as usize,
        14 as ::core::ffi::c_ulong,
    ) as *mut cff_Dict;
    let mut index: u32 = 0 as u32;
    let mut advance: u32 = 0;
    let mut val: cff_Value = cff_Value {
        t: 0 as cff_Value_Type,
        c2rust_unnamed: cff_ValueBody { i: 0 },
    };
    let mut stack: [cff_Value; 48] = [cff_Value {
        t: 0 as cff_Value_Type,
        c2rust_unnamed: cff_ValueBody { i: 0 },
    }; 48];
    let mut temp: *const u8 = data;
    while temp < data.offset(len as isize) {
        advance = cff_decodeCffToken(temp, &raw mut val);
        match val.t as ::core::ffi::c_uint {
            1 => {
                (*dict).ents = __caryll_reallocate(
                    (*dict).ents as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<cff_DictEntry>() as usize)
                        .wrapping_mul((*dict).count.wrapping_add(1 as u32) as usize),
                    24 as ::core::ffi::c_ulong,
                ) as *mut cff_DictEntry;
                (*(*dict).ents.offset((*dict).count as isize)).op =
                    val.c2rust_unnamed.i as u32;
                (*(*dict).ents.offset((*dict).count as isize)).cnt = index;
                let ref mut fresh1 = (*(*dict).ents.offset((*dict).count as isize)).vals;
                *fresh1 = __caryll_allocate_clean(
                    (::core::mem::size_of::<cff_Value>() as usize).wrapping_mul(index as usize),
                    27 as ::core::ffi::c_ulong,
                ) as *mut cff_Value;
                memcpy(
                    (*(*dict).ents.offset((*dict).count as isize)).vals as *mut ::core::ffi::c_void,
                    &raw mut stack as *mut cff_Value as *const ::core::ffi::c_void,
                    (::core::mem::size_of::<cff_Value>() as usize).wrapping_mul(index as usize),
                );
                (*dict).count = (*dict).count.wrapping_add(1);
                index = 0 as u32;
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
    mut data: *const u8,
    len: u32,
    mut context: *mut ::core::ffi::c_void,
    mut callback: Option<
        unsafe extern "C" fn(u32, u8, *mut cff_Value, *mut ::core::ffi::c_void) -> (),
    >,
) {
    let mut index: u8 = 0 as u8;
    let mut advance: u32 = 0;
    let mut val: cff_Value = cff_Value {
        t: 0 as cff_Value_Type,
        c2rust_unnamed: cff_ValueBody { i: 0 },
    };
    let mut stack: [cff_Value; 256] = [cff_Value {
        t: 0 as cff_Value_Type,
        c2rust_unnamed: cff_ValueBody { i: 0 },
    }; 256];
    let mut temp: *const u8 = data;
    while temp < data.offset(len as isize) {
        advance = cff_decodeCffToken(temp, &raw mut val);
        match val.t as ::core::ffi::c_uint {
            1 => {
                callback.expect("non-null function pointer")(
                    val.c2rust_unnamed.i as u32,
                    index,
                    &raw mut stack as *mut cff_Value,
                    context,
                );
                index = 0 as u8;
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
    mut op: u32,
    mut top: u8,
    mut stack: *mut cff_Value,
    mut _context: *mut ::core::ffi::c_void,
) {
    let mut context: *mut cff_get_key_context = _context as *mut cff_get_key_context;
    if op == (*context).op && (*context).idx <= top as u32 {
        (*context).found = true;
        (*context).res = *stack.offset((*context).idx as isize);
    }
}
unsafe extern "C" fn parseDictKey(
    mut data: *const u8,
    len: u32,
    op: u32,
    idx: u32,
) -> cff_Value {
    let mut context: cff_get_key_context = cff_get_key_context {
        found: false,
        res: cff_Value {
            t: 0 as cff_Value_Type,
            c2rust_unnamed: cff_ValueBody { i: 0 },
        },
        op: 0,
        idx: 0,
    };
    context.found = false;
    context.idx = idx;
    context.op = op;
    context.res.t = 0 as cff_Value_Type;
    context.res.c2rust_unnamed.i = -(1 as ::core::ffi::c_int) as i32;
    parseToCallback(
        data,
        len,
        &raw mut context as *mut ::core::ffi::c_void,
        Some(
            callback_get_key
                as unsafe extern "C" fn(
                    u32,
                    u8,
                    *mut cff_Value,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
    );
    return context.res;
}
unsafe extern "C" fn buildDict(mut dict: *const cff_Dict) -> *mut caryll_Buffer {
    let mut blob: *mut caryll_Buffer = bufnew();
    let mut i: u32 = 0 as u32;
    while i < (*dict).count {
        let mut j: u32 = 0 as u32;
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
                blob_val = cff_encodeCffInteger(0 as i32);
            }
            bufwrite_bufdel(blob, blob_val);
            j = j.wrapping_add(1);
        }
        bufwrite_bufdel(
            blob,
            cff_encodeCffOperator((*(*dict).ents.offset(i as isize)).op as i32),
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
        parse: Some(parseDict as unsafe extern "C" fn(*const u8, u32) -> *mut cff_Dict),
        parseToCallback: Some(
            parseToCallback
                as unsafe extern "C" fn(
                    *const u8,
                    u32,
                    *mut ::core::ffi::c_void,
                    Option<
                        unsafe extern "C" fn(
                            u32,
                            u8,
                            *mut cff_Value,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                    >,
                ) -> (),
        ),
        parseDictKey: Some(
            parseDictKey
                as unsafe extern "C" fn(*const u8, u32, u32, u32) -> cff_Value,
        ),
        build: Some(buildDict as unsafe extern "C" fn(*const cff_Dict) -> *mut caryll_Buffer),
    }
};
