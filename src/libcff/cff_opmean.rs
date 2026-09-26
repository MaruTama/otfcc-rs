use crate::libcff::CffCharstringOperator;

pub fn cff_get_standard_arity(op: CffCharstringOperator) -> u8 {
    match op.0 {
        5 | 21 => return 2_u8,
        6 | 7 => return 1_u8,
        27 | 26 | 31 | 30 => return 4_u8,
        8 => return 6_u8,
        19 | 20 => return 0_u8,
        _ => return 2_u8,
    };
}
