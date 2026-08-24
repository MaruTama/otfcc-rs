/// What a CFF DICT/CharString token decodes to: an operator, or a number in
/// one of the two forms CFF encodes.
///
/// Used to be a C-shaped `struct { t: CffValueType, c2rust_unnamed: union {
/// i: i32, d: f64 } }` -- `t` decided which arm of the union was live, and
/// nothing but caller discipline stopped a mismatched read (`.i` while
/// `t == Double`, say). Folding the tag and payload into one enum makes
/// that check-before-read discipline the compiler's job: there is no
/// `.c2rust_unnamed` left to read past a `match`.
///
/// The DICT reader and the CharString reader used to have their own
/// vocabulary for the same three live states -- `CffValueType::Operator`/
/// `CS2_OPERATOR`, `CffValueType::Integer`/`CS2_OPERAND`,
/// `CffValueType::Double`/`CS2_FRACTION` -- because each reader built a
/// value by writing `.t` and `.c2rust_unnamed` as two separate steps, and
/// giving the second writer its own names made its code read naturally.
/// Building a value now means naming the variant directly at the one
/// point it's constructed (`CffValue::Operator(op)`, `CffValue::Double(d)`),
/// which already reads naturally in either vocabulary, so the `CS2_*`
/// aliases are gone rather than ported.
///
/// `Unset` is a name this port adds; C had none, and wrote `(CffValueType)0`
/// in DICT-lookup-miss initializers and let `calloc` supply it everywhere
/// else. The state is real and reachable, not padding -- `parse_dict_key`
/// returns it to mean "key not found", and [`cffnum`] treats it (like
/// `Operator`) as "not a number", returning `0.0`.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CffValue {
    Unset,
    /// A DICT/CharString operator opcode. 2-byte (`12 <n>`-escaped)
    /// operators are pre-combined the same way the original packed them
    /// into one `i32`: `(12 << 8) | n`.
    Operator(i32),
    Integer(i32),
    Double(f64),
}
/// `Integer`/`Double` as an `f64`; `Unset`/`Operator` (not a number) as
/// `0.0` -- the same three-way split the original had, just matched on
/// the enum directly instead of `t` then `.c2rust_unnamed`.
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
