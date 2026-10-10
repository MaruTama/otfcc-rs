/// What a CFF DICT/CharString token decodes to: an operator, or a number in
/// one of the two forms CFF encodes.
///
/// `Unset` means "no value": `parse_dict_key` returns it for a key that
/// wasn't found, and [`cffnum`] treats it (like `Operator`) as "not a
/// number", returning `0.0`.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CffValue {
    Unset,
    /// A DICT/CharString operator opcode. 2-byte (`12 <n>`-escaped)
    /// operators are packed into one `i32` as `(12 << 8) | n`.
    Operator(i32),
    Integer(i32),
    Double(f64),
}
/// `Integer`/`Double` as an `f64`; `Unset`/`Operator` (not a number) as
/// `0.0`.
pub fn cffnum(val: CffValue) -> f64 {
    match val {
        CffValue::Integer(i) => i as f64,
        CffValue::Double(d) => d,
        CffValue::Unset | CffValue::Operator(_) => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unset_and_operator_are_not_numbers() {
        assert_eq!(cffnum(CffValue::Unset), 0.0);
        assert_eq!(cffnum(CffValue::Operator(42)), 0.0);
    }

    #[test]
    fn integer_and_double_convert_to_f64() {
        assert_eq!(cffnum(CffValue::Integer(-39)), -39.0);
        assert_eq!(cffnum(CffValue::Double(2.5)), 2.5);
    }
}
