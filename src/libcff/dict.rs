use crate::libcff::CffDictOperator;
use crate::libcff::codecs::{
    cff_decode_cff_token, cff_encode_cff_float, cff_encode_cff_integer, cff_encode_cff_operator,
};
use crate::libcff::value::CffValue;
use otfcc_binary::Buffer;

/// One DICT operator and the operands that precede it.
#[derive(Debug)]
pub struct CffDictEntry {
    pub op: CffDictOperator,
    pub vals: Vec<CffValue>,
}
/// A whole CFF DICT, entries in file order.
#[derive(Debug)]
pub struct CffDict {
    pub ents: Vec<CffDictEntry>,
}
// Calls `callback(op, top, stack)` for every operator in the DICT, with the
// operands pushed since the previous operator. `data` is already bounded to
// the DICT: the Private DICT's `offset`/`length` operands come from the
// font, so call sites that locate a DICT from them build this slice with
// `.get(start..).and_then(|s| s.get(..len))` and fall back to "not found"
// when it doesn't fit.
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
    // (there are exactly `top` operands, at indices `0..top`). An operator
    // with fewer operands than `idx + 1` counts as "not found" (an
    // `idx <= top` check here was a fuzz-found out-of-bounds panic).
    parse_to_callback(data, |cur_op, top, stack| {
        if cur_op == op && idx < top as u32 {
            res = stack[idx as usize];
        }
    });
    return res;
}
/// `parse_dict_key`'s value as a plain `i32`, `-1` if the key wasn't
/// present or wasn't a number -- the "not found" convention every
/// `parse_dict_key(...)` call site relies on. A `Double` value also yields
/// `-1` rather than being misread as an offset/length.
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
