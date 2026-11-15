use super::*;
use crate::libcff::index::parse_index;
use crate::libcff::charstring_il::{CffCharstringIl, il_push_op, il_push_operand};
use crate::libcff::{OP_ENDCHAR, OP_HLINETO, OP_RMOVETO};

fn simple_glyph_il(x: f64, y: f64) -> CffCharstringIl {
    let mut il = CffCharstringIl { instr: Vec::new() };
    il_push_operand(&mut il, x);
    il_push_op(&mut il, OP_RMOVETO);
    il_push_operand(&mut il, y);
    il_push_op(&mut il, OP_HLINETO);
    il_push_op(&mut il, OP_ENDCHAR);
    il
}

fn index_count(buf: &Buffer) -> u32 {
    let idx = parse_index(&buf.data, 0);
    idx.count
}

fn build(glyphs: &[CffCharstringIl], do_subroutinize: bool) -> (Buffer, Buffer, Buffer) {
    let mut g = CffSubrGraph::new(do_subroutinize);
    for il in glyphs {
        insert_il_to_graph(&mut g, il);
    }
    il_graph_to_buffers(&mut g).unwrap()
}

#[test]
fn empty_graph_produces_an_empty_char_strings_index() {
    let (s, gs, ls) = build(&[], false);
    assert_eq!(index_count(&s), 0);
    assert_eq!(index_count(&gs), 0);
    assert_eq!(index_count(&ls), 0);
}

#[test]
fn one_glyph_with_subroutinize_off_produces_one_char_string_and_no_subroutines() {
    let il = simple_glyph_il(10.0, 20.0);
    let (s, gs, ls) = build(&[il], false);
    assert_eq!(index_count(&s), 1);
    assert_eq!(index_count(&gs), 0);
    assert_eq!(index_count(&ls), 0);
}

#[test]
fn two_identical_glyphs_with_subroutinize_on_extract_a_shared_subroutine() {
    let il1 = simple_glyph_il(10.0, 20.0);
    let il2 = simple_glyph_il(10.0, 20.0);
    let (s, gs, ls) = build(&[il1, il2], true);
    assert_eq!(index_count(&s), 2);
    // The identical [rmoveto, hlineto] pair repeated across both
    // glyphs is exactly the doublet `append_node_to_graph` checks
    // for on every append -- it should be extracted into one
    // shared subroutine (local or global depending on the
    // max_l_subrs/max_g_subrs split, so check both).
    assert!(index_count(&gs) + index_count(&ls) >= 1);
}

#[test]
fn two_identical_glyphs_subroutinized_have_a_smaller_char_strings_index() {
    // The *total* size (char strings + subr indexes) isn't guaranteed
    // to shrink for an example this tiny -- the subr INDEX header and
    // the CALLSUBR/CALLGSUBR bytes are overhead that can outweigh the
    // savings for just two short glyphs; that's expected, not a bug.
    // What subroutinization does guarantee is that the *char strings*
    // themselves get smaller once the duplicated sequence is replaced
    // by a short subroutine call in each glyph.
    let il1 = simple_glyph_il(10.0, 20.0);
    let il2 = simple_glyph_il(10.0, 20.0);
    let (s_on, _gs_on, _ls_on) = build(&[il1.clone(), il2.clone()], true);
    let char_strings_on = s_on.data.len();

    let (s_off, _gs_off, _ls_off) = build(&[il1, il2], false);
    let char_strings_off = s_off.data.len();

    assert!(char_strings_on < char_strings_off);
}

#[test]
fn two_different_glyphs_with_subroutinize_on_extract_no_subroutine() {
    let il1 = simple_glyph_il(10.0, 20.0);
    let il2 = simple_glyph_il(30.0, 40.0);
    let (s, gs, ls) = build(&[il1, il2], true);
    assert_eq!(index_count(&s), 2);
    assert_eq!(index_count(&gs), 0);
    assert_eq!(index_count(&ls), 0);
}
