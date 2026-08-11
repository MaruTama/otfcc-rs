use crate::libcff::{OP_ABS, OP_ADD, OP_AND, OP_BASE_FONT_BLEND, OP_BASE_FONT_NAME, OP_BLUE_FUZZ, OP_BLUE_SCALE, OP_BLUE_SHIFT, OP_BLUE_VALUES, OP_CALLGSUBR, OP_CALLSUBR, OP_CHARSET, OP_CHARSTRING_TYPE, OP_CHAR_STRINGS, OP_CID_COUNT, OP_CID_FONT_REVISION, OP_CID_FONT_TYPE, OP_CID_FONT_VERSION, OP_CNTRMASK, OP_COPYRIGHT, OP_DEFAULT_WIDTH_X, OP_DIV, OP_DROP, OP_DUP, OP_ENCODING, OP_ENDCHAR, OP_EQ, OP_EXCH, OP_EXPANSION_FACTOR, OP_FAMILY_BLUES, OP_FAMILY_NAME, OP_FAMILY_OTHER_BLUES, OP_FD_ARRAY, OP_FD_SELECT, OP_FLEX, OP_FLEX1, OP_FONT_BBOX, OP_FONT_MATRIX, OP_FONT_NAME, OP_FORCE_BOLD, OP_FULL_NAME, OP_GET, OP_HFLEX, OP_HFLEX1, OP_HHCURVETO, OP_HINTMASK, OP_HLINETO, OP_HMOVETO, OP_HSTEM, OP_HSTEMHM, OP_HVCURVETO, OP_IFELSE, OP_INDEX, OP_INITIAL_RANDOM_SEED, OP_IS_FIXED_PITCH, OP_ITALIC_ANGLE, OP_LANGUAGE_GROUP, OP_MUL, OP_NEG, OP_NOMINAL_WIDTH_X, OP_NOT, OP_NOTICE, OP_OR, OP_OTHER_BLUES, OP_PAINT_TYPE, OP_POST_SCRIPT, OP_PRIVATE, OP_PUT, OP_RANDOM, OP_RCURVELINE, OP_RETURN, OP_RLINECURVE, OP_RLINETO, OP_RMOVETO, OP_ROLL, OP_ROS, OP_RRCURVETO, OP_SQRT, OP_STD_HW, OP_STD_VW, OP_STEM_SNAP_H, OP_STEM_SNAP_V, OP_STROKE_WIDTH, OP_SUB, OP_SUBRS, OP_SYNTHEIC_BASE, OP_UID_BASE, OP_UNDERLINE_POSITION, OP_UNDERLINE_THICKNESS, OP_UNIQUE_ID, OP_VERSION, OP_VHCURVETO, OP_VLINETO, OP_VMOVETO, OP_VSTEM, OP_VSTEMHM, OP_VVCURVETO, OP_WEIGHT, OP_XUID};

pub unsafe extern "C" fn op_cff_name(mut op: u32) -> *mut ::core::ffi::c_char {
    match op as i32 {
        OP_VERSION => {
            return b"Version\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_NOTICE => {
            return b"Notice\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FULL_NAME => {
            return b"FullName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FAMILY_NAME => {
            return b"FamilyName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_WEIGHT => {
            return b"Weight\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FONT_BBOX => {
            return b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_BLUE_VALUES => {
            return b"BlueValues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_OTHER_BLUES => {
            return b"OtherBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FAMILY_BLUES => {
            return b"FamilyBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FAMILY_OTHER_BLUES => {
            return b"FamilyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_STD_HW => {
            return b"StdHW\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_STD_VW => {
            return b"StdVW\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_UNIQUE_ID => {
            return b"UniqueID\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_XUID => {
            return b"XUID\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_CHARSET => {
            return b"charset\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_ENCODING => {
            return b"Encoding\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_CHAR_STRINGS => {
            return b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_PRIVATE => {
            return b"Private\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_SUBRS => {
            return b"Subrs\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_DEFAULT_WIDTH_X => {
            return b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_NOMINAL_WIDTH_X => {
            return b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_COPYRIGHT => {
            return b"Copyright\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_IS_FIXED_PITCH => {
            return b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_ITALIC_ANGLE => {
            return b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_UNDERLINE_POSITION => {
            return b"UnderlinePosition\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_UNDERLINE_THICKNESS => {
            return b"UnderlineThickness\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_PAINT_TYPE => {
            return b"PaintType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_CHARSTRING_TYPE => {
            return b"CharstringType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FONT_MATRIX => {
            return b"FontMatrix\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_STROKE_WIDTH => {
            return b"StrokeWidth\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_BLUE_SCALE => {
            return b"BlueScale\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_BLUE_SHIFT => {
            return b"BlueShift\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_BLUE_FUZZ => {
            return b"BlueFuzz\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_STEM_SNAP_H => {
            return b"StemSnapH\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_STEM_SNAP_V => {
            return b"StemSnapV\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FORCE_BOLD => {
            return b"ForceBold\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_LANGUAGE_GROUP => {
            return b"LanguageGroup\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_EXPANSION_FACTOR => {
            return b"ExpansionFactor\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_INITIAL_RANDOM_SEED => {
            return b"initialRandomSeed\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_SYNTHEIC_BASE => {
            return b"SyntheicBase\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_POST_SCRIPT => {
            return b"PostScript\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_BASE_FONT_NAME => {
            return b"BaseFontName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_BASE_FONT_BLEND => {
            return b"BaseFontBlend\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_ROS => {
            return b"ROS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_CID_FONT_VERSION => {
            return b"CIDFontVersion\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_CID_FONT_REVISION => {
            return b"CIDFontReversion\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_CID_FONT_TYPE => {
            return b"CIDFontType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_CID_COUNT => {
            return b"CIDCount\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_UID_BASE => {
            return b"UIDBase\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FD_ARRAY => {
            return b"FDArray\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FD_SELECT => {
            return b"FDSelect\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FONT_NAME => {
            return b"FontName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        _ => {
            return b"Unkown\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
    };
}
pub unsafe extern "C" fn op_cs2_name(mut op: u32) -> *mut ::core::ffi::c_char {
    match op as i32 {
        OP_HSTEM => {
            return b"hstem\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_VSTEM => {
            return b"vstem\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_VMOVETO => {
            return b"vmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_RLINETO => {
            return b"rlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_HLINETO => {
            return b"hlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_VLINETO => {
            return b"vlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_RRCURVETO => {
            return b"rrcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_CALLSUBR => {
            return b"callsubr\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_RETURN => {
            return b"return\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_ENDCHAR => {
            return b"endchar\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_HSTEMHM => {
            return b"hstemhm\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_HINTMASK => {
            return b"hintmask\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_CNTRMASK => {
            return b"cntrmask\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_RMOVETO => {
            return b"rmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_HMOVETO => {
            return b"hmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_VSTEMHM => {
            return b"vstemhm\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_RCURVELINE => {
            return b"rcurveline\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_RLINECURVE => {
            return b"rlinecurve\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_VVCURVETO => {
            return b"vvcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_HHCURVETO => {
            return b"hhcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_CALLGSUBR => {
            return b"callgsubr\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_VHCURVETO => {
            return b"vhcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_HVCURVETO => {
            return b"hvcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_AND => {
            return b"and\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_OR => {
            return b"or\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_NOT => {
            return b"not\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_ABS => {
            return b"abs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_ADD => {
            return b"add\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_SUB => {
            return b"sub\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_DIV => {
            return b"div\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_NEG => {
            return b"neg\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_EQ => {
            return b"eq\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_DROP => {
            return b"drop\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_PUT => {
            return b"put\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_GET => {
            return b"get\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_IFELSE => {
            return b"ifelse\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_RANDOM => {
            return b"random\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_MUL => {
            return b"mul\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_SQRT => {
            return b"sqrt\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_DUP => {
            return b"dup\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        OP_EXCH => {
            return b"exch\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_INDEX => {
            return b"index\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_ROLL => {
            return b"roll\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_HFLEX => {
            return b"hflex\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FLEX => {
            return b"fles\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_HFLEX1 => {
            return b"hflex1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        OP_FLEX1 => {
            return b"flex1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        _ => {
            return b"Unknown\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
    };
}
pub unsafe extern "C" fn cff_get_standard_arity(mut op: u32) -> u8 {
    match op as i32 {
        OP_RLINETO | OP_RMOVETO => return 2 as u8,
        OP_HLINETO | OP_VLINETO => return 1 as u8,
        OP_HHCURVETO | OP_VVCURVETO | OP_HVCURVETO | OP_VHCURVETO => return 4 as u8,
        OP_RRCURVETO => return 6 as u8,
        OP_HINTMASK | OP_CNTRMASK => return 0 as u8,
        _ => return 2 as u8,
    };
}
