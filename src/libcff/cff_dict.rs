// Stage M-10 removed this file's last `unsafe` (the `cff_dict_free` shell
// around `CffDict`), so the file-level allow for implicit-unsafe-in-
// unsafe-fn is gone too.
use crate::libcff::CffDictOperator;
use crate::libcff::cff_codecs::{
    cff_decode_cff_token, cff_encode_cff_float, cff_encode_cff_integer, cff_encode_cff_operator,
};
use crate::libcff::cff_value::CffValue;
use crate::support::buffer::Buffer;

// `vals` was `__caryll_allocate_clean`'d/`free`'d, sized from `cnt` -- an
// operand count read out of untrusted CFF DICT bytes in `parse_dict`, the
// same class of risk `CffIndex.offset`/`.data` had. `Vec` removes the
// manual free pair and the OOB-write risk a counting mistake there would
// have caused.
#[derive(Debug)]
pub struct CffDictEntry {
    pub op: CffDictOperator,
    pub vals: Vec<CffValue>,
}
// `ents` was similarly `__caryll_reallocate`'d one entry at a time while
// parsing untrusted DICT bytes (`parse_dict`'s `count`), and in the write
// (build) path (`table/cff.rs`'s `cffdict_givemeablank`).
#[derive(Debug)]
pub struct CffDict {
    pub ents: Vec<CffDictEntry>,
}
// `cff_dict_free` (a `Box::into_raw`/`Box::from_raw` shell around a
// `CffDict` every producer -- `table/cff.rs`'s `cff_make_fd_dict`/
// `cff_make_private_dict` -- already built as a plain owned local before
// boxing it) is gone as of Stage M-10, matching this migration's Stage
// M-3 treatment of `ClassDef`: those two functions return `CffDict` by
// value now, and their callers just let the local drop.
// `data` used to be a raw `(*const u8, u32)` pair: the loop itself always
// respected `len` correctly (each iteration passes `&data[pos..]`, a
// bounds-checked slice, to `cff_decode_cff_token`), but every
// call site had to construct that pointer from a font-byte-derived offset
// with no bounds check of its own -- the Private DICT's `offset`/`length`
// operands are attacker-controlled, and three call sites
// (`cff_parser.rs::parse_cff_bytecode`, `cff_parser.rs::cff_parse_subr`,
// `table/cff.rs::callback_extract_fd`'s operator-18 arm) turned them
// straight into `raw_data.offset(private_off)` with nothing stopping
// `private_off`/`private_len` from running past the real buffer. Taking
// `&[u8]` moves the one bounds check those three sites need to their own
// call sites (each now builds this slice via `.get(start..).and_then(|s|
// s.get(..len))`, falling back to the existing "not found" path on
// failure) while every already-safe call site (reading `top_dict.data`/
// `fdarray.data`/`font_dict.data`, which `extract_index` already bounds-
// checked) just drops the redundant manual pointer diff.
// Was a `*mut c_void` context pointer + `Option<unsafe fn(..., *mut
// c_void)>` callback, type-erasing the three concrete callbacks
// (`callback_get_key` here, `table/cff.rs`'s `callback_extract_private`/
// `callback_extract_fd`) behind a shared shape purely so one function
// pointer type could stand in for all three -- the same "type erasure
// that was never actually needed" pattern Stage 9 Phase 9 found in
// `libcff/cff_index.rs`'s `new_index_by_callback` (resolved there by
// taking `impl Iterator` instead). Each call site already knows its own
// concrete callback at compile time, so a generic `impl FnMut` closure
// carries the same information with no unsafe function-pointer cast and
// no context pointer to reinterpret -- callers whose callback body still
// touches raw pointers (`callback_extract_private`/`callback_extract_fd`)
// keep doing so inside their own closure, unrelated to this signature.
pub(crate) fn parse_to_callback(data: &[u8], mut callback: impl FnMut(CffDictOperator, u8, &[CffValue])) {
    let mut index: u8 = 0_u8;
    let mut val: CffValue = CffValue::Unset;
    let mut stack: [CffValue; 256] = [CffValue::Unset; 256];
    let mut pos: usize = 0;
    while pos < data.len() {
        // Same fix as `cff_parse_outline`'s equivalent loop: the token
        // itself, not just where it starts, must stay within `data`.
        let Some(adv) = cff_decode_cff_token(&data[pos..], &mut val) else {
            break;
        };
        match val {
            CffValue::Operator(op) => {
                callback(CffDictOperator(op as u32), index, &stack[..index as usize]);
                index = 0_u8;
            }
            CffValue::Integer(_) | CffValue::Double(_) => {
                stack[index as usize] = val;
                index = index.wrapping_add(1);
            }
            CffValue::Unset => {}
        }
        pos += adv as usize;
    }
}
pub(crate) fn parse_dict_key(data: &[u8], op: CffDictOperator, idx: u32) -> CffValue {
    let mut res = CffValue::Unset;
    // `idx` is a 0-based operand index, so a valid read needs `idx < top`
    // (there are exactly `top` operands, at indices `0..top`) -- this was
    // `idx <= top`, an off-by-one that let `idx == top` (no operand at
    // all, e.g. this operator with zero pushed operands and `idx == 0`)
    // through. Against the original's raw pointer into a fixed 256-entry
    // array that silently read a stale/adjacent slot rather than
    // panicking; converting `stack` to a `top`-length slice (an earlier
    // PR) turned that same off-by-one into a reachable `index out of
    // bounds` panic, caught by `cargo fuzz run otf_parse`. Tightening to
    // `idx < top` is the actual fix -- not a change in what counts as
    // "found", just removing an already-wrong read of one index past the
    // operator's real operand list.
    parse_to_callback(data, |cur_op, top, stack| {
        if cur_op == op && idx < top as u32 {
            res = stack[idx as usize];
        }
    });
    return res;
}
/// `parse_dict_key`'s value as a plain `i32`, `-1` if the key wasn't
/// present or wasn't a number -- the "not found" convention every one of
/// this crate's `parse_dict_key(...)` call sites already relied on (the
/// original encoded it by writing `-1` into the "not found" sentinel's
/// union payload and trusting every caller to read `.i` without checking
/// `.t` first). Computed here by actually matching the variant instead,
/// so a caller can no longer misread a legitimately-`Double` DICT value
/// as a bogus offset/length by reading the wrong union arm.
pub(crate) fn parse_dict_key_int(data: &[u8], op: CffDictOperator, idx: u32) -> i32 {
    match parse_dict_key(data, op, idx) {
        CffValue::Integer(i) => i,
        CffValue::Double(d) => d as i32,
        CffValue::Unset | CffValue::Operator(_) => -1,
    }
}
pub(crate) fn build_dict(dict: &CffDict) -> Buffer {
    let mut blob = Buffer::new();
    let ents = &dict.ents;
    for ent in ents {
        for val in &ent.vals {
            let blob_val: Buffer = match *val {
                CffValue::Integer(i) => cff_encode_cff_integer(i),
                CffValue::Double(d) => cff_encode_cff_float(d),
                CffValue::Unset | CffValue::Operator(_) => cff_encode_cff_integer(0_i32),
            };
            blob.write_buffer_owned(blob_val);
        }
        blob.write_buffer_owned(cff_encode_cff_operator(ent.op));
    }
    blob
}
#[cfg(test)]
mod tests {
    use super::*;

    // `build_dict` was a nested index-loop pair (entries, then each
    // entry's operand values) with no index arithmetic beyond a plain
    // increment. Converted to nested `for`/`.iter()` loops -- this pins
    // down that an empty dict, a multi-operand entry, and multiple
    // entries all still serialize in the same order (operands then their
    // operator, per entry, entries in original order).
    #[test]
    fn build_dict_matches_manual_loop_semantics() {
        assert_eq!(build_dict(&CffDict { ents: Vec::new() }).len(), 0);

        let dict = CffDict {
            ents: vec![
                CffDictEntry {
                    op: CffDictOperator(15),
                    vals: vec![CffValue::Integer(1000), CffValue::Integer(2)],
                },
                CffDictEntry {
                    op: CffDictOperator(17),
                    vals: vec![CffValue::Integer(500)],
                },
            ],
        };
        let mut expected = Buffer::new();
        expected.write_buffer_owned(cff_encode_cff_integer(1000));
        expected.write_buffer_owned(cff_encode_cff_integer(2));
        expected.write_buffer_owned(cff_encode_cff_operator(CffDictOperator(15)));
        expected.write_buffer_owned(cff_encode_cff_integer(500));
        expected.write_buffer_owned(cff_encode_cff_operator(CffDictOperator(17)));
        assert_eq!(build_dict(&dict).data, expected.data);
    }
}
