#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::buffer::{Buffer};
use crate::libcff::CffDictOperator;
use crate::libcff::cff_value::{CffValueType, CffValue, CffValueBody};
use crate::libcff::cff_codecs::{cff_decode_cff_token, cff_encode_cff_float, cff_encode_cff_integer, cff_encode_cff_operator};
use crate::support::buffer::{bufnew, bufwrite_bufdel};

// `vals` was `__caryll_allocate_clean`'d/`free`'d, sized from `cnt` -- an
// operand count read out of untrusted CFF DICT bytes in `parse_dict`, the
// same class of risk `CffIndex.offset`/`.data` had. `Vec` removes the
// manual free pair and the OOB-write risk a counting mistake there would
// have caused.
#[repr(C)]
pub struct CffDictEntry {
    pub op: CffDictOperator,
    pub vals: Vec<CffValue>,
}
// `ents` was similarly `__caryll_reallocate`'d one entry at a time while
// parsing untrusted DICT bytes (`parse_dict`'s `count`), and in the write
// (build) path (`table/cff.rs`'s `cffdict_givemeablank`).
#[repr(C)]
pub struct CffDict {
    pub ents: Vec<CffDictEntry>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffDictElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut CffDict) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut CffDict, *const CffDict) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut CffDict) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut CffDict>,
    pub free: Option<unsafe extern "C" fn(*mut CffDict) -> ()>,
    pub parse: Option<unsafe extern "C" fn(*const u8, u32) -> *mut CffDict>,
    pub parse_to_callback: Option<
        unsafe extern "C" fn(
            *const u8,
            u32,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    CffDictOperator,
                    u8,
                    *mut CffValue,
                    *mut ::core::ffi::c_void,
                ) -> (),
            >,
        ) -> (),
    >,
    pub parse_dict_key:
        Option<unsafe extern "C" fn(*const u8, u32, CffDictOperator, u32) -> CffValue>,
    pub build: Option<unsafe extern "C" fn(*const CffDict) -> *mut Buffer>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffGetKeyContext {
    pub found: bool,
    pub res: CffValue,
    pub op: CffDictOperator,
    pub idx: u32,
}
#[inline]
unsafe extern "C" fn dispose_dict(mut dict: *mut CffDict) {
    (*dict).ents = Vec::new();
}
#[inline]
unsafe extern "C" fn cff_dict_create() -> *mut CffDict {
    // `Box::new` of an explicit all-zero literal, not `malloc` + `cff_dict_
    // init`'s `memset` -- see `cff_dict_free`'s matching `Box::from_raw`.
    // `cff_dict_init` itself stays defined for `CFF_I_DICT.init` (a vtable
    // slot with no call site anywhere in the crate, same "present but
    // unreachable" shape as `subtable_gpos_pair_copy`).
    Box::into_raw(Box::new(CffDict {
        ents: Vec::new(),
    }))
}
#[inline]
unsafe extern "C" fn cff_dict_free(mut x: *mut CffDict) {
    if x.is_null() {
        return;
    }
    // `ents`/each entry's `vals` are still freed here exactly as before --
    // only the outer shell's own allocator changed, from a bare `malloc`/
    // `free` pair to `Box::into_raw`/`Box::from_raw`. Every `CFF_I_DICT.
    // create`/`.free` call site pairs consistently (confirmed by grep: no
    // generic adapter reclaims a `*mut CffDict` any other way, unlike
    // `GposPairSubtable`'s `subtable_from_raw`), so this is self-contained.
    cff_dict_dispose(x);
    drop(Box::from_raw(x));
}
#[inline]
unsafe extern "C" fn cff_dict_dispose(mut x: *mut CffDict) {
    dispose_dict(x);
}
#[inline]
unsafe extern "C" fn cff_dict_init(mut x: *mut CffDict) {
    // No all-zero bit pattern is a valid `CffDict` any more (it owns a
    // `Vec`), so place a valid empty value directly instead of the old
    // `memset`.
    ::core::ptr::write(x, CffDict { ents: Vec::new() });
}
#[inline]
unsafe extern "C" fn cff_dict_copy(mut _dst: *mut CffDict, mut _src: *const CffDict) {
    // Confirmed dead: `CFF_I_DICT.copy` has no call site anywhere in the
    // crate. The old `memcpy`-based body was only safe by accident, back
    // when both fields were raw pointers a bitwise copy could alias
    // harmlessly; now that `ents` (and each entry's `vals`) are `Vec`s, a
    // `memcpy` would double-free. Kept as a loud failure instead of
    // silently reintroducing that risk if this ever gets wired up.
    unreachable!("CffDict::copy is dead code and unsound for owned Vec data")
}
unsafe extern "C" fn parse_dict(mut data: *const u8, len: u32) -> *mut CffDict {
    let dict: *mut CffDict = Box::into_raw(Box::new(CffDict { ents: Vec::new() }));
    let mut index: u32 = 0 as u32;
    let mut advance: u32 = 0;
    let mut val: CffValue = CffValue {
        t: CffValueType::Unset,
        c2rust_unnamed: CffValueBody { i: 0 },
    };
    let mut stack: [CffValue; 48] = [CffValue {
        t: CffValueType::Unset,
        c2rust_unnamed: CffValueBody { i: 0 },
    }; 48];
    let mut temp: *const u8 = data;
    while temp < data.offset(len as isize) {
        advance = cff_decode_cff_token(temp, &raw mut val);
        match val.t {
            CffValueType::Operator => {
                (*dict).ents.push(CffDictEntry {
                    op: CffDictOperator(val.c2rust_unnamed.i as u32),
                    vals: stack[..index as usize].to_vec(),
                });
                index = 0 as u32;
            }
            CffValueType::Integer | CffValueType::Double => {
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
unsafe extern "C" fn parse_to_callback(
    mut data: *const u8,
    len: u32,
    mut context: *mut ::core::ffi::c_void,
    mut callback: Option<
        unsafe extern "C" fn(
            CffDictOperator,
            u8,
            *mut CffValue,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
) {
    let mut index: u8 = 0 as u8;
    let mut advance: u32 = 0;
    let mut val: CffValue = CffValue {
        t: CffValueType::Unset,
        c2rust_unnamed: CffValueBody { i: 0 },
    };
    let mut stack: [CffValue; 256] = [CffValue {
        t: CffValueType::Unset,
        c2rust_unnamed: CffValueBody { i: 0 },
    }; 256];
    let mut temp: *const u8 = data;
    while temp < data.offset(len as isize) {
        advance = cff_decode_cff_token(temp, &raw mut val);
        match val.t {
            CffValueType::Operator => {
                callback.expect("non-null function pointer")(
                    CffDictOperator(val.c2rust_unnamed.i as u32),
                    index,
                    &raw mut stack as *mut CffValue,
                    context,
                );
                index = 0 as u8;
            }
            CffValueType::Integer | CffValueType::Double => {
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
    mut op: CffDictOperator,
    mut top: u8,
    mut stack: *mut CffValue,
    mut _context: *mut ::core::ffi::c_void,
) {
    let mut context: *mut CffGetKeyContext = _context as *mut CffGetKeyContext;
    if op == (*context).op && (*context).idx <= top as u32 {
        (*context).found = true;
        (*context).res = *stack.offset((*context).idx as isize);
    }
}
unsafe extern "C" fn parse_dict_key(
    mut data: *const u8,
    len: u32,
    op: CffDictOperator,
    idx: u32,
) -> CffValue {
    let mut context: CffGetKeyContext = CffGetKeyContext {
        found: false,
        res: CffValue {
            t: CffValueType::Unset,
            c2rust_unnamed: CffValueBody { i: 0 },
        },
        op: CffDictOperator(0),
        idx: 0,
    };
    context.found = false;
    context.idx = idx;
    context.op = op;
    context.res.t = CffValueType::Unset;
    context.res.c2rust_unnamed.i = -(1 as ::core::ffi::c_int) as i32;
    parse_to_callback(
        data,
        len,
        &raw mut context as *mut ::core::ffi::c_void,
        Some(
            callback_get_key
                as unsafe extern "C" fn(
                    CffDictOperator,
                    u8,
                    *mut CffValue,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
    );
    return context.res;
}
unsafe extern "C" fn build_dict(mut dict: *const CffDict) -> *mut Buffer {
    let mut blob: *mut Buffer = bufnew();
    let ents = &(*dict).ents;
    let mut i: usize = 0;
    while i < ents.len() {
        let vals = &ents[i].vals;
        let mut j: usize = 0;
        while j < vals.len() {
            let mut blob_val: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
            if vals[j].t as ::core::ffi::c_uint
                == CffValueType::Integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                blob_val = cff_encode_cff_integer(vals[j].c2rust_unnamed.i);
            } else if vals[j].t as ::core::ffi::c_uint
                == CffValueType::Double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                blob_val = cff_encode_cff_float(vals[j].c2rust_unnamed.d);
            } else {
                blob_val = cff_encode_cff_integer(0 as i32);
            }
            bufwrite_bufdel(blob, blob_val);
            j = j.wrapping_add(1);
        }
        bufwrite_bufdel(blob, cff_encode_cff_operator(ents[i].op));
        i = i.wrapping_add(1);
    }
    return blob;
}
pub static CFF_I_DICT: CffDictElementInterface = {
    CffDictElementInterface {
        init: Some(cff_dict_init as unsafe extern "C" fn(*mut CffDict) -> ()),
        copy: Some(cff_dict_copy as unsafe extern "C" fn(*mut CffDict, *const CffDict) -> ()),
        dispose: Some(cff_dict_dispose as unsafe extern "C" fn(*mut CffDict) -> ()),
        create: Some(cff_dict_create),
        free: Some(cff_dict_free as unsafe extern "C" fn(*mut CffDict) -> ()),
        parse: Some(parse_dict as unsafe extern "C" fn(*const u8, u32) -> *mut CffDict),
        parse_to_callback: Some(
            parse_to_callback
                as unsafe extern "C" fn(
                    *const u8,
                    u32,
                    *mut ::core::ffi::c_void,
                    Option<
                        unsafe extern "C" fn(
                            CffDictOperator,
                            u8,
                            *mut CffValue,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                    >,
                ) -> (),
        ),
        parse_dict_key: Some(
            parse_dict_key
                as unsafe extern "C" fn(*const u8, u32, CffDictOperator, u32) -> CffValue,
        ),
        build: Some(build_dict as unsafe extern "C" fn(*const CffDict) -> *mut Buffer),
    }
};
