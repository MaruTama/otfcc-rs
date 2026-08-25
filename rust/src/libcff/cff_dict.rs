#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::libcff::CffDictOperator;
use crate::libcff::cff_codecs::{
    cff_decode_cff_token, cff_encode_cff_float, cff_encode_cff_integer, cff_encode_cff_operator,
};
use crate::libcff::cff_value::CffValue;
use crate::support::buffer::Buffer;
use crate::support::buffer::{bufnew, bufwrite_bufdel};

// `vals` was `__caryll_allocate_clean`'d/`free`'d, sized from `cnt` -- an
// operand count read out of untrusted CFF DICT bytes in `parse_dict`, the
// same class of risk `CffIndex.offset`/`.data` had. `Vec` removes the
// manual free pair and the OOB-write risk a counting mistake there would
// have caused.
pub struct CffDictEntry {
    pub op: CffDictOperator,
    pub vals: Vec<CffValue>,
}
// `ents` was similarly `__caryll_reallocate`'d one entry at a time while
// parsing untrusted DICT bytes (`parse_dict`'s `count`), and in the write
// (build) path (`table/cff.rs`'s `cffdict_givemeablank`).
pub struct CffDict {
    pub ents: Vec<CffDictEntry>,
}
#[derive(Copy, Clone)]
pub struct CffGetKeyContext {
    pub found: bool,
    pub res: CffValue,
    pub op: CffDictOperator,
    pub idx: u32,
}
#[inline]
unsafe fn dispose_dict(dict: *mut CffDict) {
    (*dict).ents = Vec::new();
}
#[inline]
pub(crate) unsafe fn cff_dict_create() -> *mut CffDict {
    // `Box::new` of an explicit all-zero literal, not `malloc` + a `memset`
    // init -- see `cff_dict_free`'s matching `Box::from_raw`.
    Box::into_raw(Box::new(CffDict { ents: Vec::new() }))
}
#[inline]
pub(crate) unsafe fn cff_dict_free(x: *mut CffDict) {
    if x.is_null() {
        return;
    }
    // `ents`/each entry's `vals` are still freed here exactly as before --
    // only the outer shell's own allocator changed, from a bare `malloc`/
    // `free` pair to `Box::into_raw`/`Box::from_raw`. Every `cff_dict_
    // create`/`cff_dict_free` call site pairs consistently (confirmed by
    // grep: no generic adapter reclaims a `*mut CffDict` any other way,
    // unlike `GposPairSubtable`'s `subtable_from_raw`), so this is
    // self-contained.
    cff_dict_dispose(x);
    drop(Box::from_raw(x));
}
#[inline]
unsafe fn cff_dict_dispose(x: *mut CffDict) {
    dispose_dict(x);
}
pub(crate) unsafe fn parse_to_callback(
    data: *const u8,
    len: u32,
    context: *mut ::core::ffi::c_void,
    callback: Option<
        unsafe extern "C" fn(CffDictOperator, u8, *mut CffValue, *mut ::core::ffi::c_void) -> (),
    >,
) {
    let mut index: u8 = 0 as u8;
    let mut advance: u32;
    let mut val: CffValue = CffValue::Unset;
    let mut stack: [CffValue; 256] = [CffValue::Unset; 256];
    let mut temp: *const u8 = data;
    while temp < data.offset(len as isize) {
        // Same fix as `cff_parse_outline`'s equivalent loop: the token
        // itself, not just where it starts, must stay within `len`.
        let remaining = data.offset(len as isize).offset_from(temp) as usize;
        let Some(adv) = cff_decode_cff_token(temp, remaining, &raw mut val) else {
            break;
        };
        advance = adv;
        match val {
            CffValue::Operator(op) => {
                callback.expect("non-null function pointer")(
                    CffDictOperator(op as u32),
                    index,
                    &raw mut stack as *mut CffValue,
                    context,
                );
                index = 0 as u8;
            }
            CffValue::Integer(_) | CffValue::Double(_) => {
                let fresh0 = index;
                index = index.wrapping_add(1);
                stack[fresh0 as usize] = val;
            }
            CffValue::Unset => {}
        }
        temp = temp.offset(advance as isize);
    }
}
unsafe extern "C" fn callback_get_key(
    op: CffDictOperator,
    top: u8,
    stack: *mut CffValue,
    mut _context: *mut ::core::ffi::c_void,
) {
    let context: *mut CffGetKeyContext = _context as *mut CffGetKeyContext;
    if op == (*context).op && (*context).idx <= top as u32 {
        (*context).found = true;
        (*context).res = *stack.offset((*context).idx as isize);
    }
}
pub(crate) unsafe fn parse_dict_key(
    data: *const u8,
    len: u32,
    op: CffDictOperator,
    idx: u32,
) -> CffValue {
    let mut context: CffGetKeyContext = CffGetKeyContext {
        found: false,
        res: CffValue::Unset,
        op: CffDictOperator(0),
        idx: 0,
    };
    context.found = false;
    context.idx = idx;
    context.op = op;
    context.res = CffValue::Unset;
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
/// `parse_dict_key`'s value as a plain `i32`, `-1` if the key wasn't
/// present or wasn't a number -- the "not found" convention every one of
/// this crate's `parse_dict_key(...)` call sites already relied on (the
/// original encoded it by writing `-1` into the "not found" sentinel's
/// union payload and trusting every caller to read `.i` without checking
/// `.t` first). Computed here by actually matching the variant instead,
/// so a caller can no longer misread a legitimately-`Double` DICT value
/// as a bogus offset/length by reading the wrong union arm.
pub(crate) unsafe fn parse_dict_key_int(
    data: *const u8,
    len: u32,
    op: CffDictOperator,
    idx: u32,
) -> i32 {
    match parse_dict_key(data, len, op, idx) {
        CffValue::Integer(i) => i,
        CffValue::Double(d) => d as i32,
        CffValue::Unset | CffValue::Operator(_) => -1,
    }
}
pub(crate) unsafe fn build_dict(dict: *const CffDict) -> *mut Buffer {
    let blob: *mut Buffer = bufnew();
    let ents = &(*dict).ents;
    let mut i: usize = 0;
    while i < ents.len() {
        let vals = &ents[i].vals;
        let mut j: usize = 0;
        while j < vals.len() {
            let blob_val: *mut Buffer = match vals[j] {
                CffValue::Integer(i) => cff_encode_cff_integer(i),
                CffValue::Double(d) => cff_encode_cff_float(d),
                CffValue::Unset | CffValue::Operator(_) => cff_encode_cff_integer(0 as i32),
            };
            bufwrite_bufdel(blob, blob_val);
            j = j.wrapping_add(1);
        }
        bufwrite_bufdel(blob, cff_encode_cff_operator(ents[i].op));
        i = i.wrapping_add(1);
    }
    return blob;
}
