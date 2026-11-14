use super::strncmp_eq;

#[test]
fn identical_bytes_compare_equal() {
    assert!(strncmp_eq(b"abcd", b"abcd", 4));
}

#[test]
fn a_difference_within_n_compares_unequal() {
    assert!(!strncmp_eq(b"abcd", b"abXd", 4));
}

#[test]
fn a_difference_at_or_past_n_is_not_examined() {
    assert!(strncmp_eq(b"abcX", b"abcY", 3));
}

#[test]
fn a_shared_leading_nul_stops_the_comparison_early_even_if_later_bytes_differ() {
    // The exact quirk `ident_node`'s own comment calls out: charstring
    // bytecode isn't a C string, but `strncmp`'s early termination on a
    // NUL byte still applies to it, so two byte sequences sharing a
    // NUL-terminated prefix compare "equal" here even though the bytes
    // after the NUL differ.
    assert!(strncmp_eq(b"ab\0X", b"ab\0Y", 4));
}
