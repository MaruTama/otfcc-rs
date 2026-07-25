pub type cff_DictOperator = ::core::ffi::c_uint;
pub const op_FontName: cff_DictOperator = 3110;
pub const op_FDSelect: cff_DictOperator = 3109;
pub const op_FDArray: cff_DictOperator = 3108;
pub const op_UIDBase: cff_DictOperator = 3107;
pub const op_CIDCount: cff_DictOperator = 3106;
pub const op_CIDFontType: cff_DictOperator = 3105;
pub const op_CIDFontRevision: cff_DictOperator = 3104;
pub const op_CIDFontVersion: cff_DictOperator = 3103;
pub const op_ROS: cff_DictOperator = 3102;
pub const op_maxstack: cff_DictOperator = 25;
pub const op_vstore: cff_DictOperator = 24;
pub const op_BaseFontBlend: cff_DictOperator = 3095;
pub const op_blend: cff_DictOperator = 23;
pub const op_BaseFontName: cff_DictOperator = 3094;
pub const op_vsindex: cff_DictOperator = 22;
pub const op_PostScript: cff_DictOperator = 3093;
pub const op_nominalWidthX: cff_DictOperator = 21;
pub const op_SyntheicBase: cff_DictOperator = 3092;
pub const op_defaultWidthX: cff_DictOperator = 20;
pub const op_initialRandomSeed: cff_DictOperator = 3091;
pub const op_Subrs: cff_DictOperator = 19;
pub const op_ExpansionFactor: cff_DictOperator = 3090;
pub const op_Private: cff_DictOperator = 18;
pub const op_LanguageGroup: cff_DictOperator = 3089;
pub const op_CharStrings: cff_DictOperator = 17;
pub const op_Encoding: cff_DictOperator = 16;
pub const op_charset: cff_DictOperator = 15;
pub const op_ForceBold: cff_DictOperator = 3086;
pub const op_XUID: cff_DictOperator = 14;
pub const op_StemSnapV: cff_DictOperator = 3085;
pub const op_UniqueID: cff_DictOperator = 13;
pub const op_StemSnapH: cff_DictOperator = 3084;
pub const op_BlueFuzz: cff_DictOperator = 3083;
pub const op_StdVW: cff_DictOperator = 11;
pub const op_BlueShift: cff_DictOperator = 3082;
pub const op_StdHW: cff_DictOperator = 10;
pub const op_BlueScale: cff_DictOperator = 3081;
pub const op_FamilyOtherBlues: cff_DictOperator = 9;
pub const op_StrokeWidth: cff_DictOperator = 3080;
pub const op_FamilyBlues: cff_DictOperator = 8;
pub const op_FontMatrix: cff_DictOperator = 3079;
pub const op_OtherBlues: cff_DictOperator = 7;
pub const op_CharstringType: cff_DictOperator = 3078;
pub const op_BlueValues: cff_DictOperator = 6;
pub const op_PaintType: cff_DictOperator = 3077;
pub const op_FontBBox: cff_DictOperator = 5;
pub const op_UnderlineThickness: cff_DictOperator = 3076;
pub const op_Weight: cff_DictOperator = 4;
pub const op_UnderlinePosition: cff_DictOperator = 3075;
pub const op_FamilyName: cff_DictOperator = 3;
pub const op_ItalicAngle: cff_DictOperator = 3074;
pub const op_FullName: cff_DictOperator = 2;
pub const op_isFixedPitch: cff_DictOperator = 3073;
pub const op_Notice: cff_DictOperator = 1;
pub const op_Copyright: cff_DictOperator = 3072;
pub const op_version: cff_DictOperator = 0;
pub type cff_CharstringOperator = ::core::ffi::c_uint;
pub const op_flex1: cff_CharstringOperator = 3109;
pub const op_hflex1: cff_CharstringOperator = 3108;
pub const op_flex: cff_CharstringOperator = 3107;
pub const op_hflex: cff_CharstringOperator = 3106;
pub const op_hvcurveto: cff_CharstringOperator = 31;
pub const op_roll: cff_CharstringOperator = 3102;
pub const op_vhcurveto: cff_CharstringOperator = 30;
pub const op_index: cff_CharstringOperator = 3101;
pub const op_callgsubr: cff_CharstringOperator = 29;
pub const op_exch: cff_CharstringOperator = 3100;
pub const op_dup: cff_CharstringOperator = 3099;
pub const op_hhcurveto: cff_CharstringOperator = 27;
pub const op_sqrt: cff_CharstringOperator = 3098;
pub const op_vvcurveto: cff_CharstringOperator = 26;
pub const op_rlinecurve: cff_CharstringOperator = 25;
pub const op_mul: cff_CharstringOperator = 3096;
pub const op_rcurveline: cff_CharstringOperator = 24;
pub const op_random: cff_CharstringOperator = 3095;
pub const op_vstemhm: cff_CharstringOperator = 23;
pub const op_ifelse: cff_CharstringOperator = 3094;
pub const op_hmoveto: cff_CharstringOperator = 22;
pub const op_get: cff_CharstringOperator = 3093;
pub const op_rmoveto: cff_CharstringOperator = 21;
pub const op_put: cff_CharstringOperator = 3092;
pub const op_cntrmask: cff_CharstringOperator = 20;
pub const op_hintmask: cff_CharstringOperator = 19;
pub const op_drop: cff_CharstringOperator = 3090;
pub const op_hstemhm: cff_CharstringOperator = 18;
pub const op_cff2blend: cff_CharstringOperator = 16;
pub const op_eq: cff_CharstringOperator = 3087;
pub const op_cff2vsidx: cff_CharstringOperator = 15;
pub const op_neg: cff_CharstringOperator = 3086;
pub const op_endchar: cff_CharstringOperator = 14;
pub const op_div: cff_CharstringOperator = 3084;
pub const op_sub: cff_CharstringOperator = 3083;
pub const op_return: cff_CharstringOperator = 11;
pub const op_add: cff_CharstringOperator = 3082;
pub const op_callsubr: cff_CharstringOperator = 10;
pub const op_abs: cff_CharstringOperator = 3081;
pub const op_rrcurveto: cff_CharstringOperator = 8;
pub const op_vlineto: cff_CharstringOperator = 7;
pub const op_hlineto: cff_CharstringOperator = 6;
pub const op_not: cff_CharstringOperator = 3077;
pub const op_rlineto: cff_CharstringOperator = 5;
pub const op_or: cff_CharstringOperator = 3076;
pub const op_vmoveto: cff_CharstringOperator = 4;
pub const op_and: cff_CharstringOperator = 3075;
pub const op_vstem: cff_CharstringOperator = 3;
pub const op_hstem: cff_CharstringOperator = 1;
#[no_mangle]
pub unsafe extern "C" fn op_cff_name(mut op: u32) -> *mut ::core::ffi::c_char {
    match op {
        0 => {
            return b"Version\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        1 => {
            return b"Notice\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        2 => {
            return b"FullName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3 => {
            return b"FamilyName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        4 => {
            return b"Weight\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        5 => {
            return b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        6 => {
            return b"BlueValues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        7 => {
            return b"OtherBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        8 => {
            return b"FamilyBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        9 => {
            return b"FamilyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        10 => {
            return b"StdHW\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        11 => {
            return b"StdVW\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        13 => {
            return b"UniqueID\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        14 => {
            return b"XUID\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        15 => {
            return b"charset\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        16 => {
            return b"Encoding\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        17 => {
            return b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        18 => {
            return b"Private\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        19 => {
            return b"Subrs\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        20 => {
            return b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        21 => {
            return b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3072 => {
            return b"Copyright\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3073 => {
            return b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3074 => {
            return b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3075 => {
            return b"UnderlinePosition\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3076 => {
            return b"UnderlineThickness\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3077 => {
            return b"PaintType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3078 => {
            return b"CharstringType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3079 => {
            return b"FontMatrix\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3080 => {
            return b"StrokeWidth\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3081 => {
            return b"BlueScale\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3082 => {
            return b"BlueShift\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3083 => {
            return b"BlueFuzz\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3084 => {
            return b"StemSnapH\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3085 => {
            return b"StemSnapV\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3086 => {
            return b"ForceBold\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3089 => {
            return b"LanguageGroup\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3090 => {
            return b"ExpansionFactor\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3091 => {
            return b"initialRandomSeed\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3092 => {
            return b"SyntheicBase\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3093 => {
            return b"PostScript\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3094 => {
            return b"BaseFontName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3095 => {
            return b"BaseFontBlend\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3102 => {
            return b"ROS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3103 => {
            return b"CIDFontVersion\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3104 => {
            return b"CIDFontReversion\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3105 => {
            return b"CIDFontType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3106 => {
            return b"CIDCount\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3107 => {
            return b"UIDBase\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3108 => {
            return b"FDArray\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3109 => {
            return b"FDSelect\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3110 => {
            return b"FontName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        _ => {
            return b"Unkown\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn op_cs2_name(mut op: u32) -> *mut ::core::ffi::c_char {
    match op {
        1 => {
            return b"hstem\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3 => {
            return b"vstem\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        4 => {
            return b"vmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        5 => {
            return b"rlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        6 => {
            return b"hlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        7 => {
            return b"vlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        8 => {
            return b"rrcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        10 => {
            return b"callsubr\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        11 => {
            return b"return\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        14 => {
            return b"endchar\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        18 => {
            return b"hstemhm\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        19 => {
            return b"hintmask\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        20 => {
            return b"cntrmask\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        21 => {
            return b"rmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        22 => {
            return b"hmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        23 => {
            return b"vstemhm\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        24 => {
            return b"rcurveline\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        25 => {
            return b"rlinecurve\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        26 => {
            return b"vvcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        27 => {
            return b"hhcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        29 => {
            return b"callgsubr\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        30 => {
            return b"vhcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        31 => {
            return b"hvcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3075 => {
            return b"and\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3076 => {
            return b"or\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3077 => {
            return b"not\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3081 => {
            return b"abs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3082 => {
            return b"add\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3083 => {
            return b"sub\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3084 => {
            return b"div\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3086 => {
            return b"neg\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3087 => {
            return b"eq\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3090 => {
            return b"drop\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3092 => {
            return b"put\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3093 => {
            return b"get\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3094 => {
            return b"ifelse\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3095 => {
            return b"random\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3096 => {
            return b"mul\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3098 => {
            return b"sqrt\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3099 => {
            return b"dup\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3100 => {
            return b"exch\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3101 => {
            return b"index\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3102 => {
            return b"roll\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3106 => {
            return b"hflex\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3107 => {
            return b"fles\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3108 => {
            return b"hflex1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3109 => {
            return b"flex1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        _ => {
            return b"Unknown\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_getStandardArity(mut op: u32) -> u8 {
    match op {
        5 | 21 => return 2 as u8,
        6 | 7 => return 1 as u8,
        27 | 26 | 31 | 30 => return 4 as u8,
        8 => return 6 as u8,
        19 | 20 => return 0 as u8,
        _ => return 2 as u8,
    };
}
