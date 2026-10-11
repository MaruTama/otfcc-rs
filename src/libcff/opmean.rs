use crate::libcff::CffCharstringOperator;

pub fn cff_get_standard_arity(op: CffCharstringOperator) -> u8 {
    match op.0 {
        5 | 21 => return 2,
        6 | 7 => return 1,
        27 | 26 | 31 | 30 => return 4,
        8 => return 6,
        19 | 20 => return 0,
        _ => return 2,
    };
}
