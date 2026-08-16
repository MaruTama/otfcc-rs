use crate::support::primitives::{Arity};
use crate::libcff::cff_charset::{CffCharset};
use crate::libcff::cff_fdselect::{CffFdSelect};
use crate::libcff::cff_index::{CffIndex};
use crate::libcff::cff_value::{CffValue};
pub mod cff_charset;
pub mod cff_codecs;
pub mod cff_dict;
pub mod cff_fdselect;
pub mod cff_index;
pub mod cff_opmean;
pub mod cff_parser;
pub mod cff_string;
pub mod cff_value;
pub mod cff_writer;
pub mod charstring_il;
pub mod subr;

/// A cff DICT operator, in otfcc's own encoding: the operator byte, or
/// `12 << 8 | b` for the two-byte operators the spec escapes with 12.
///
/// A newtype rather than an `enum`, because these are numbers, not a closed
/// set: nothing enumerates them, and an operator otfcc does not know must
/// travel through the dict machinery unchanged rather than fail to construct.
/// The same reasoning [`LookupType`](crate::table::otl::LookupType) is built
/// on.
///
/// **Not interchangeable with [`CffCharstringOperator`]**: 38 of the numbers
/// mean one thing in a DICT and something else in a CharString (`OP_NOTICE`
/// and `OP_HSTEM` are both 1, `OP_FD_ARRAY` and `OP_HFLEX1` are both 3108).
/// The names never collide and the two sets are read by disjoint code, so
/// nothing was ever wrong -- but while both were `i32` aliases the compiler
/// could not see the distinction, and a `cffdict_input_ints(dict, OP_HSTEM,
/// ..)` typo would have compiled. Now it does not.
///
/// `u32` is the width the dict machinery already used everywhere
/// (`CffDictEntry.op`, `parse_dict_key`, `CffGetKeyContext.op`), so wrapping
/// it removes the `as u32` at every call site instead of adding one.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct CffDictOperator(pub u32);

pub const OP_FONT_NAME: CffDictOperator = CffDictOperator(3110);

pub const OP_FD_SELECT: CffDictOperator = CffDictOperator(3109);

pub const OP_FD_ARRAY: CffDictOperator = CffDictOperator(3108);

pub const OP_UID_BASE: CffDictOperator = CffDictOperator(3107);

pub const OP_CID_COUNT: CffDictOperator = CffDictOperator(3106);

pub const OP_CID_FONT_TYPE: CffDictOperator = CffDictOperator(3105);

pub const OP_CID_FONT_REVISION: CffDictOperator = CffDictOperator(3104);

pub const OP_CID_FONT_VERSION: CffDictOperator = CffDictOperator(3103);

pub const OP_ROS: CffDictOperator = CffDictOperator(3102);

pub const OP_MAXSTACK: CffDictOperator = CffDictOperator(25);

pub const OP_VSTORE: CffDictOperator = CffDictOperator(24);

pub const OP_BASE_FONT_BLEND: CffDictOperator = CffDictOperator(3095);

pub const OP_BLEND: CffDictOperator = CffDictOperator(23);

pub const OP_BASE_FONT_NAME: CffDictOperator = CffDictOperator(3094);

pub const OP_VSINDEX: CffDictOperator = CffDictOperator(22);

pub const OP_POST_SCRIPT: CffDictOperator = CffDictOperator(3093);

pub const OP_NOMINAL_WIDTH_X: CffDictOperator = CffDictOperator(21);

pub const OP_SYNTHEIC_BASE: CffDictOperator = CffDictOperator(3092);

pub const OP_DEFAULT_WIDTH_X: CffDictOperator = CffDictOperator(20);

pub const OP_INITIAL_RANDOM_SEED: CffDictOperator = CffDictOperator(3091);

pub const OP_SUBRS: CffDictOperator = CffDictOperator(19);

pub const OP_EXPANSION_FACTOR: CffDictOperator = CffDictOperator(3090);

pub const OP_PRIVATE: CffDictOperator = CffDictOperator(18);

pub const OP_LANGUAGE_GROUP: CffDictOperator = CffDictOperator(3089);

pub const OP_CHAR_STRINGS: CffDictOperator = CffDictOperator(17);

pub const OP_ENCODING: CffDictOperator = CffDictOperator(16);

pub const OP_CHARSET: CffDictOperator = CffDictOperator(15);

pub const OP_FORCE_BOLD: CffDictOperator = CffDictOperator(3086);

pub const OP_XUID: CffDictOperator = CffDictOperator(14);

pub const OP_STEM_SNAP_V: CffDictOperator = CffDictOperator(3085);

pub const OP_UNIQUE_ID: CffDictOperator = CffDictOperator(13);

pub const OP_STEM_SNAP_H: CffDictOperator = CffDictOperator(3084);

pub const OP_BLUE_FUZZ: CffDictOperator = CffDictOperator(3083);

pub const OP_STD_VW: CffDictOperator = CffDictOperator(11);

pub const OP_BLUE_SHIFT: CffDictOperator = CffDictOperator(3082);

pub const OP_STD_HW: CffDictOperator = CffDictOperator(10);

pub const OP_BLUE_SCALE: CffDictOperator = CffDictOperator(3081);

pub const OP_FAMILY_OTHER_BLUES: CffDictOperator = CffDictOperator(9);

pub const OP_STROKE_WIDTH: CffDictOperator = CffDictOperator(3080);

pub const OP_FAMILY_BLUES: CffDictOperator = CffDictOperator(8);

pub const OP_FONT_MATRIX: CffDictOperator = CffDictOperator(3079);

pub const OP_OTHER_BLUES: CffDictOperator = CffDictOperator(7);

pub const OP_CHARSTRING_TYPE: CffDictOperator = CffDictOperator(3078);

pub const OP_BLUE_VALUES: CffDictOperator = CffDictOperator(6);

pub const OP_PAINT_TYPE: CffDictOperator = CffDictOperator(3077);

pub const OP_FONT_BBOX: CffDictOperator = CffDictOperator(5);

pub const OP_UNDERLINE_THICKNESS: CffDictOperator = CffDictOperator(3076);

pub const OP_WEIGHT: CffDictOperator = CffDictOperator(4);

pub const OP_UNDERLINE_POSITION: CffDictOperator = CffDictOperator(3075);

pub const OP_FAMILY_NAME: CffDictOperator = CffDictOperator(3);

pub const OP_ITALIC_ANGLE: CffDictOperator = CffDictOperator(3074);

pub const OP_FULL_NAME: CffDictOperator = CffDictOperator(2);

pub const OP_IS_FIXED_PITCH: CffDictOperator = CffDictOperator(3073);

pub const OP_NOTICE: CffDictOperator = CffDictOperator(1);

pub const OP_COPYRIGHT: CffDictOperator = CffDictOperator(3072);

pub const OP_VERSION: CffDictOperator = CffDictOperator(0);

/// A Type 2 CharString operator, encoded like [`CffDictOperator`] and, as noted
/// there, sharing 38 of its numbers while meaning something else by them.
///
/// `i32`, not `u32`, because a CharString operator is stored in
/// `CffCharstringInstruction`'s `i32` argument arm -- an arm the `Special`
/// instruction type also uses, for values that are not operators at all. That
/// field therefore stays a bare `i32`; what this newtype covers is everything
/// *flowing into* it (`il_push_op`, `il_matchop`, `zroll`, `opop_roll`,
/// `cff_get_standard_arity`, `cff_merge_cs2_operator`), which is where a DICT
/// operator could have been passed by mistake.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct CffCharstringOperator(pub i32);

pub const OP_FLEX1: CffCharstringOperator = CffCharstringOperator(3109);

pub const OP_HFLEX1: CffCharstringOperator = CffCharstringOperator(3108);

pub const OP_FLEX: CffCharstringOperator = CffCharstringOperator(3107);

pub const OP_HFLEX: CffCharstringOperator = CffCharstringOperator(3106);

pub const OP_HVCURVETO: CffCharstringOperator = CffCharstringOperator(31);

pub const OP_ROLL: CffCharstringOperator = CffCharstringOperator(3102);

pub const OP_VHCURVETO: CffCharstringOperator = CffCharstringOperator(30);

pub const OP_INDEX: CffCharstringOperator = CffCharstringOperator(3101);

pub const OP_CALLGSUBR: CffCharstringOperator = CffCharstringOperator(29);

pub const OP_EXCH: CffCharstringOperator = CffCharstringOperator(3100);

pub const OP_DUP: CffCharstringOperator = CffCharstringOperator(3099);

pub const OP_HHCURVETO: CffCharstringOperator = CffCharstringOperator(27);

pub const OP_SQRT: CffCharstringOperator = CffCharstringOperator(3098);

pub const OP_VVCURVETO: CffCharstringOperator = CffCharstringOperator(26);

pub const OP_RLINECURVE: CffCharstringOperator = CffCharstringOperator(25);

pub const OP_MUL: CffCharstringOperator = CffCharstringOperator(3096);

pub const OP_RCURVELINE: CffCharstringOperator = CffCharstringOperator(24);

pub const OP_RANDOM: CffCharstringOperator = CffCharstringOperator(3095);

pub const OP_VSTEMHM: CffCharstringOperator = CffCharstringOperator(23);

pub const OP_IFELSE: CffCharstringOperator = CffCharstringOperator(3094);

pub const OP_HMOVETO: CffCharstringOperator = CffCharstringOperator(22);

pub const OP_GET: CffCharstringOperator = CffCharstringOperator(3093);

pub const OP_RMOVETO: CffCharstringOperator = CffCharstringOperator(21);

pub const OP_PUT: CffCharstringOperator = CffCharstringOperator(3092);

pub const OP_CNTRMASK: CffCharstringOperator = CffCharstringOperator(20);

pub const OP_HINTMASK: CffCharstringOperator = CffCharstringOperator(19);

pub const OP_DROP: CffCharstringOperator = CffCharstringOperator(3090);

pub const OP_HSTEMHM: CffCharstringOperator = CffCharstringOperator(18);

pub const OP_CFF2BLEND: CffCharstringOperator = CffCharstringOperator(16);

pub const OP_EQ: CffCharstringOperator = CffCharstringOperator(3087);

pub const OP_CFF2VSIDX: CffCharstringOperator = CffCharstringOperator(15);

pub const OP_NEG: CffCharstringOperator = CffCharstringOperator(3086);

pub const OP_ENDCHAR: CffCharstringOperator = CffCharstringOperator(14);

pub const OP_DIV: CffCharstringOperator = CffCharstringOperator(3084);

pub const OP_SUB: CffCharstringOperator = CffCharstringOperator(3083);

pub const OP_RETURN: CffCharstringOperator = CffCharstringOperator(11);

pub const OP_ADD: CffCharstringOperator = CffCharstringOperator(3082);

pub const OP_CALLSUBR: CffCharstringOperator = CffCharstringOperator(10);

pub const OP_ABS: CffCharstringOperator = CffCharstringOperator(3081);

pub const OP_RRCURVETO: CffCharstringOperator = CffCharstringOperator(8);

pub const OP_VLINETO: CffCharstringOperator = CffCharstringOperator(7);

pub const OP_HLINETO: CffCharstringOperator = CffCharstringOperator(6);

pub const OP_NOT: CffCharstringOperator = CffCharstringOperator(3077);

pub const OP_RLINETO: CffCharstringOperator = CffCharstringOperator(5);

pub const OP_OR: CffCharstringOperator = CffCharstringOperator(3076);

pub const OP_VMOVETO: CffCharstringOperator = CffCharstringOperator(4);

pub const OP_AND: CffCharstringOperator = CffCharstringOperator(3075);

pub const OP_VSTEM: CffCharstringOperator = CffCharstringOperator(3);

pub const OP_HSTEM: CffCharstringOperator = CffCharstringOperator(1);

// The Type 2 charstring spec's implementation limits. C gave them a
// `cff_Type2Limits` type of their own, but they are not a set of anything --
// they are six unrelated capacities, only ever compared against a count -- so
// each is typed as whatever it is compared with instead, which is what removes
// the casts at the fifteen sites that use them. `TYPE2_CHARSTRING_LEN` and
// `TYPE2_STEM_HINTS` are the two otfcc never checks; they stay because the
// spec's table is easier to verify whole than with holes in it.
/// Size of [`CffStack::transient`], which it declares -- so the modulus that
/// wraps an index into that array cannot drift from the array itself.
pub const TYPE2_TRANSIENT_ARRAY: usize = 32;
pub const TYPE2_MAX_SUBRS: u32 = 65300;
pub const TYPE2_CHARSTRING_LEN: u32 = 65535;
pub const TYPE2_SUBR_NESTING: u32 = 10;
pub const TYPE2_STEM_HINTS: u32 = 96;
pub const TYPE2_ARGUMENT_STACK: u32 = 48;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffHeader {
    pub major: u8,
    pub minor: u8,
    pub hdr_size: u8,
    pub off_size: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffEncodingRangeFormat1 {
    pub first: u8,
    pub nleft: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffEncodingSupplement {
    pub code: u8,
    pub glyph: u16,
}

/// Was a `t: CffEncodingType` discriminant plus a `c2rust_unnamed:
/// CffEncodingBody` union (`f0`/`f1`/`ns`, one raw-pointer array each) --
/// same shape `Subtable` had, and the same fix: a single Rust enum,
/// discriminant and payload together, so the compiler enforces that only
/// the payload matching the current variant is ever read. `format`/
/// `ncodes`/`nranges`/`nsup` are gone too -- each was write-only (set
/// once while parsing, never read again anywhere in the crate) and
/// exactly duplicated its `Vec`'s own `.len()`; `format` doubly so,
/// since it just repeated the variant tag itself as a number.
#[derive(Clone)]
pub enum CffEncoding {
    Standard,
    Expert,
    Format0(Vec<u8>),
    Format1(Vec<CffEncodingRangeFormat1>),
    FormatSupplement(Vec<CffEncodingSupplement>),
    Unspecified,
}

// `Copy`/`Clone` dropped: `stack` now owns a `Vec` (the Type 2 CharString
// interpreter's operand stack, fixed at `0x10000` entries -- the same
// generous capacity `__caryll_allocate_clean` used to allocate up front,
// matching the spec's operand stack never actually approaching that size).
// Confirmed by grep: `CffStack` is only ever reached through `*mut
// CffStack`, constructed once in `table/cff.rs`'s `build_outline`, never
// copied. `max` is gone -- write-only (set once at construction, never read
// anywhere in the interpreter), and exactly duplicated `stack.capacity()`.
#[repr(C)]
pub struct CffStack {
    pub stack: Vec<CffValue>,
    pub transient: [CffValue; TYPE2_TRANSIENT_ARRAY],
    pub index: Arity,
    pub stem: u8,
}

// `Copy`/`Clone` dropped: `encodings: CffEncoding` now owns `Vec`s on
// three of its variants. Confirmed by grep before removing the derive --
// `CffFile` is never used by value anywhere in the crate, always through
// `*mut CffFile`/`*const CffFile`, so the derive was vestigial.
#[repr(C)]
pub struct CffFile {
    pub raw_data: *mut u8,
    pub raw_length: u32,
    pub cnt_glyph: u16,
    pub head: CffHeader,
    pub name: CffIndex,
    pub top_dict: CffIndex,
    pub string: CffIndex,
    pub global_subr: CffIndex,
    pub encodings: CffEncoding,
    pub charsets: CffCharset,
    pub fdselect: CffFdSelect,
    pub char_strings: CffIndex,
    pub font_dict: CffIndex,
    pub local_subr: CffIndex,
}

#[cfg(test)]
mod tests {
    use super::*;

    // 38 of the two tables' numbers mean one thing in a DICT and something else
    // entirely in a CharString. This test used to record that overlap with plain
    // `assert_eq!(OP_NOTICE, OP_HSTEM)` -- which compiled, because both were
    // `i32` aliases, and which is exactly the confusion the newtypes now
    // prevent. Reaching for `.0` is the point: the numbers still collide, and
    // saying so now takes an explicit unwrap on each side.
    //
    // `assert_eq!(OP_NOTICE, OP_HSTEM)` no longer compiles (mismatched types).
    // That is the check this PR bought.
    #[test]
    fn the_two_operator_tables_share_numbers() {
        assert_eq!(OP_NOTICE.0 as i32, OP_HSTEM.0);
        assert_eq!(OP_FD_ARRAY.0 as i32, OP_HFLEX1.0);
        assert_eq!(OP_FAMILY_NAME.0 as i32, OP_VSTEM.0);
        // Both encode a two-byte operator as `12 << 8 | b`.
        assert_eq!(OP_FD_ARRAY.0, 12 << 8 | 36);
        assert_eq!(OP_HFLEX1.0, 12 << 8 | 36);
        assert_eq!(OP_VERSION.0, 0);
    }
}
