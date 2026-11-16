use crate::libcff::CffCharstringOperator;

pub fn get_standard_arity(op: CffCharstringOperator) -> u8 {
    match op.0 {
        5 | 21 => 2,
        6 | 7 => 1,
        27 | 26 | 31 | 30 => 4,
        8 => 6,
        19 | 20 => 0,
        _ => 2,
    }
}
