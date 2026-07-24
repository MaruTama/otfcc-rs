extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn bufnew() -> *mut caryll_Buffer;
    static iVQ: __caryll_vectorinterface_VQ;
    fn cff_getStandardArity(op: uint32_t) -> uint8_t;
    fn cff_mergeCS2Operator(blob: *mut caryll_Buffer, val: int32_t);
    fn cff_mergeCS2Operand(blob: *mut caryll_Buffer, val: ::core::ffi::c_double);
    fn cff_mergeCS2Special(blob: *mut caryll_Buffer, val: uint8_t);
    static glyf_iPoint: __caryll_elementinterface_glyf_Point;
    static glyf_iContour: __caryll_vectorinterface_glyf_Contour;
}

use crate::support::handle::{otfcc_Handle, otfcc_GlyphHandle};
use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut ::core::ffi::c_void,
    pub __gr_top: *mut ::core::ffi::c_void,
    pub __vr_top: *mut ::core::ffi::c_void,
    pub __gr_offs: ::core::ffi::c_int,
    pub __vr_offs: ::core::ffi::c_int,
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type glyphid_t = uint16_t;
pub type shapeid_t = uint16_t;
pub type arity_t = uint32_t;
pub type pos_t = ::core::ffi::c_double;
pub type scale_t = ::core::ffi::c_double;
pub type otfcc_FDHandle = otfcc_Handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILoggerTarget {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()>,
}
pub type otfcc_LoggerType = ::core::ffi::c_uint;
pub const log_type_progress: otfcc_LoggerType = 3;
pub const log_type_info: otfcc_LoggerType = 2;
pub const log_type_warning: otfcc_LoggerType = 1;
pub const log_type_error: otfcc_LoggerType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILogger {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub indent: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub indentSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub start: Option<unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> ()>,
    pub startSDS: Option<unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()>,
    pub log: Option<
        unsafe extern "C" fn(
            *mut otfcc_ILogger,
            uint8_t,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t) -> ()>,
    pub getTarget: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Options {
    pub debug_wait_on_start: bool,
    pub ignore_glyph_order: bool,
    pub ignore_hints: bool,
    pub has_vertical_metrics: bool,
    pub export_fdselect: bool,
    pub keep_average_char_width: bool,
    pub keep_unicode_ranges: bool,
    pub short_post: bool,
    pub dummy_DSIG: bool,
    pub keep_modified_time: bool,
    pub instr_as_bytes: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub cff_short_vmtx: bool,
    pub merge_lookups: bool,
    pub merge_features: bool,
    pub force_cid: bool,
    pub cff_rollCharString: bool,
    pub cff_doSubroutinize: bool,
    pub stub_cmap4: bool,
    pub decimal_cmap: bool,
    pub name_glyphs_by_hash: bool,
    pub name_glyphs_by_gid: bool,
    pub glyph_name_prefix: *mut ::core::ffi::c_char,
    pub logger: *mut otfcc_ILogger,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_AxisSpan {
    pub start: pos_t,
    pub peak: pos_t,
    pub end: pos_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_Region {
    pub dimensions: shapeid_t,
    pub spans: [vq_AxisSpan; 0],
}
pub type VQSegType = ::core::ffi::c_uint;
pub const VQ_DELTA: VQSegType = 1;
pub const VQ_STILL: VQSegType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_Segment {
    pub type_0: VQSegType,
    pub val: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub still: pos_t,
    pub delta: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub quantity: pos_t,
    pub touched: bool,
    pub region: *const vq_Region,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vq_SegList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut vq_Segment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VQ {
    pub kernel: pos_t,
    pub shift: vq_SegList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_VQ {
    pub init: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut VQ, *const VQ) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut VQ, *mut VQ) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> VQ>,
    pub dup: Option<unsafe extern "C" fn(VQ) -> VQ>,
    pub neutral: Option<unsafe extern "C" fn() -> VQ>,
    pub plus: Option<unsafe extern "C" fn(VQ, VQ) -> VQ>,
    pub inplacePlus: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub inplaceNegate: Option<unsafe extern "C" fn(*mut VQ) -> ()>,
    pub negate: Option<unsafe extern "C" fn(VQ) -> VQ>,
    pub inplaceMinus: Option<unsafe extern "C" fn(*mut VQ, VQ) -> ()>,
    pub minus: Option<unsafe extern "C" fn(VQ, VQ) -> VQ>,
    pub inplaceScale: Option<unsafe extern "C" fn(*mut VQ, scale_t) -> ()>,
    pub inplacePlusScale: Option<unsafe extern "C" fn(*mut VQ, scale_t, VQ) -> ()>,
    pub scale: Option<unsafe extern "C" fn(VQ, scale_t) -> VQ>,
    pub equal: Option<unsafe extern "C" fn(VQ, VQ) -> bool>,
    pub compare: Option<unsafe extern "C" fn(VQ, VQ) -> ::core::ffi::c_int>,
    pub compareRef: Option<unsafe extern "C" fn(*const VQ, *const VQ) -> ::core::ffi::c_int>,
    pub show: Option<unsafe extern "C" fn(VQ) -> ()>,
    pub getStill: Option<unsafe extern "C" fn(VQ) -> pos_t>,
    pub createStill: Option<unsafe extern "C" fn(pos_t) -> VQ>,
    pub isStill: Option<unsafe extern "C" fn(VQ) -> bool>,
    pub isZero: Option<unsafe extern "C" fn(VQ, pos_t) -> bool>,
    pub pointLinearTfm: Option<unsafe extern "C" fn(VQ, pos_t, VQ, pos_t, VQ) -> VQ>,
    pub addDelta: Option<unsafe extern "C" fn(*mut VQ, bool, *const vq_Region, pos_t) -> ()>,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const op_flex1: C2RustUnnamed_1 = 3109;
pub const op_hflex1: C2RustUnnamed_1 = 3108;
pub const op_flex: C2RustUnnamed_1 = 3107;
pub const op_hflex: C2RustUnnamed_1 = 3106;
pub const op_hvcurveto: C2RustUnnamed_1 = 31;
pub const op_roll: C2RustUnnamed_1 = 3102;
pub const op_vhcurveto: C2RustUnnamed_1 = 30;
pub const op_index: C2RustUnnamed_1 = 3101;
pub const op_callgsubr: C2RustUnnamed_1 = 29;
pub const op_exch: C2RustUnnamed_1 = 3100;
pub const op_dup: C2RustUnnamed_1 = 3099;
pub const op_hhcurveto: C2RustUnnamed_1 = 27;
pub const op_sqrt: C2RustUnnamed_1 = 3098;
pub const op_vvcurveto: C2RustUnnamed_1 = 26;
pub const op_rlinecurve: C2RustUnnamed_1 = 25;
pub const op_mul: C2RustUnnamed_1 = 3096;
pub const op_rcurveline: C2RustUnnamed_1 = 24;
pub const op_random: C2RustUnnamed_1 = 3095;
pub const op_vstemhm: C2RustUnnamed_1 = 23;
pub const op_ifelse: C2RustUnnamed_1 = 3094;
pub const op_hmoveto: C2RustUnnamed_1 = 22;
pub const op_get: C2RustUnnamed_1 = 3093;
pub const op_rmoveto: C2RustUnnamed_1 = 21;
pub const op_put: C2RustUnnamed_1 = 3092;
pub const op_cntrmask: C2RustUnnamed_1 = 20;
pub const op_hintmask: C2RustUnnamed_1 = 19;
pub const op_drop: C2RustUnnamed_1 = 3090;
pub const op_hstemhm: C2RustUnnamed_1 = 18;
pub const op_cff2blend: C2RustUnnamed_1 = 16;
pub const op_eq: C2RustUnnamed_1 = 3087;
pub const op_cff2vsidx: C2RustUnnamed_1 = 15;
pub const op_neg: C2RustUnnamed_1 = 3086;
pub const op_endchar: C2RustUnnamed_1 = 14;
pub const op_div: C2RustUnnamed_1 = 3084;
pub const op_sub: C2RustUnnamed_1 = 3083;
pub const op_return: C2RustUnnamed_1 = 11;
pub const op_add: C2RustUnnamed_1 = 3082;
pub const op_callsubr: C2RustUnnamed_1 = 10;
pub const op_abs: C2RustUnnamed_1 = 3081;
pub const op_rrcurveto: C2RustUnnamed_1 = 8;
pub const op_vlineto: C2RustUnnamed_1 = 7;
pub const op_hlineto: C2RustUnnamed_1 = 6;
pub const op_not: C2RustUnnamed_1 = 3077;
pub const op_rlineto: C2RustUnnamed_1 = 5;
pub const op_or: C2RustUnnamed_1 = 3076;
pub const op_vmoveto: C2RustUnnamed_1 = 4;
pub const op_and: C2RustUnnamed_1 = 3075;
pub const op_vstem: C2RustUnnamed_1 = 3;
pub const op_hstem: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const type2_transient_array: C2RustUnnamed_2 = 32;
pub const type2_max_subrs: C2RustUnnamed_2 = 65300;
pub const type2_charstring_len: C2RustUnnamed_2 = 65535;
pub const type2_subr_nesting: C2RustUnnamed_2 = 10;
pub const type2_stem_hints: C2RustUnnamed_2 = 96;
pub const type2_argument_stack: C2RustUnnamed_2 = 48;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Point {
    pub x: VQ,
    pub y: VQ,
    pub onCurve: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_glyf_Point {
    pub init: Option<unsafe extern "C" fn(*mut glyf_Point) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_Point, *const glyf_Point) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_Point, *mut glyf_Point) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_Point) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_Point, glyf_Point) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_Point, glyf_Point) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> glyf_Point>,
    pub dup: Option<unsafe extern "C" fn(glyf_Point) -> glyf_Point>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Contour {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_Point,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_vectorinterface_glyf_Contour {
    pub init: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut glyf_Contour, *const glyf_Contour) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut glyf_Contour, *mut glyf_Contour) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut glyf_Contour, glyf_Contour) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut glyf_Contour, glyf_Contour) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut glyf_Contour>,
    pub free: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub initN: Option<unsafe extern "C" fn(*mut glyf_Contour, size_t) -> ()>,
    pub initCapN: Option<unsafe extern "C" fn(*mut glyf_Contour, size_t) -> ()>,
    pub createN: Option<unsafe extern "C" fn(size_t) -> *mut glyf_Contour>,
    pub fill: Option<unsafe extern "C" fn(*mut glyf_Contour, size_t) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut glyf_Contour, glyf_Point) -> ()>,
    pub shrinkToFit: Option<unsafe extern "C" fn(*mut glyf_Contour) -> ()>,
    pub pop: Option<unsafe extern "C" fn(*mut glyf_Contour) -> glyf_Point>,
    pub disposeItem: Option<unsafe extern "C" fn(*mut glyf_Contour, size_t) -> ()>,
    pub filterEnv: Option<
        unsafe extern "C" fn(
            *mut glyf_Contour,
            Option<unsafe extern "C" fn(*const glyf_Point, *mut ::core::ffi::c_void) -> bool>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub sort: Option<
        unsafe extern "C" fn(
            *mut glyf_Contour,
            Option<unsafe extern "C" fn(*const glyf_Point, *const glyf_Point) -> ::core::ffi::c_int>,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ContourList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_Contour,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PostscriptStemDef {
    pub position: pos_t,
    pub width: pos_t,
    pub map: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_StemDefList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_PostscriptStemDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_PostscriptHintMask {
    pub pointsBefore: uint16_t,
    pub contoursBefore: uint16_t,
    pub maskH: [bool; 256],
    pub maskV: [bool; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_MaskList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_PostscriptHintMask,
}
pub type RefAnchorStatus = ::core::ffi::c_uint;
pub const REF_ANCHOR_CONSOLIDATING_XY: RefAnchorStatus = 5;
pub const REF_ANCHOR_CONSOLIDATING_ANCHOR: RefAnchorStatus = 4;
pub const REF_ANCHOR_CONSOLIDATED: RefAnchorStatus = 3;
pub const REF_ANCHOR_XY: RefAnchorStatus = 2;
pub const REF_ANCHOR_ANCHOR: RefAnchorStatus = 1;
pub const REF_XY: RefAnchorStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ComponentReference {
    pub x: VQ,
    pub y: VQ,
    pub roundToGrid: bool,
    pub useMyMetrics: bool,
    pub glyph: otfcc_GlyphHandle,
    pub a: scale_t,
    pub b: scale_t,
    pub c: scale_t,
    pub d: scale_t,
    pub isAnchored: RefAnchorStatus,
    pub inner: shapeid_t,
    pub outer: shapeid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_ReferenceList {
    pub length: size_t,
    pub capacity: size_t,
    pub items: *mut glyf_ComponentReference,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_GlyphStat {
    pub xMin: pos_t,
    pub xMax: pos_t,
    pub yMin: pos_t,
    pub yMax: pos_t,
    pub nestDepth: uint16_t,
    pub nPoints: uint16_t,
    pub nContours: uint16_t,
    pub nCompositePoints: uint16_t,
    pub nCompositeContours: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyf_Glyph {
    pub name: sds,
    pub horizontalOrigin: VQ,
    pub advanceWidth: VQ,
    pub verticalOrigin: VQ,
    pub advanceHeight: VQ,
    pub contours: glyf_ContourList,
    pub references: glyf_ReferenceList,
    pub stemH: glyf_StemDefList,
    pub stemV: glyf_StemDefList,
    pub hintMasks: glyf_MaskList,
    pub contourMasks: glyf_MaskList,
    pub instructionsLength: uint16_t,
    pub instructions: *mut uint8_t,
    pub yPel: uint8_t,
    pub fdSelect: otfcc_FDHandle,
    pub cid: glyphid_t,
    pub stat: glyf_GlyphStat,
}
pub type cff_InstructionType = ::core::ffi::c_uint;
pub const IL_ITEM_PHANTOM_OPERAND: cff_InstructionType = 4;
pub const IL_ITEM_PHANTOM_OPERATOR: cff_InstructionType = 3;
pub const IL_ITEM_SPECIAL: cff_InstructionType = 2;
pub const IL_ITEM_OPERATOR: cff_InstructionType = 1;
pub const IL_ITEM_OPERAND: cff_InstructionType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharstringInstruction {
    pub type_0: cff_InstructionType,
    pub arity: arity_t,
    pub c2rust_unnamed: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub d: ::core::ffi::c_double,
    pub i: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_CharstringIL {
    pub length: uint32_t,
    pub free: uint32_t,
    pub instr: *mut cff_CharstringInstruction,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn ensureThereIsSpace(mut il: *mut cff_CharstringIL) {
    if (*il).free != 0 {
        return;
    }
    (*il).free = 0x100 as uint32_t;
    (*il).instr = __caryll_reallocate(
        (*il).instr as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<cff_CharstringInstruction>() as size_t)
            .wrapping_mul((*il).length.wrapping_add((*il).free) as size_t),
        8 as ::core::ffi::c_ulong,
    ) as *mut cff_CharstringInstruction;
}
#[no_mangle]
pub unsafe extern "C" fn il_push_operand(
    mut il: *mut cff_CharstringIL,
    mut x: ::core::ffi::c_double,
) {
    ensureThereIsSpace(il);
    (*(*il).instr.offset((*il).length as isize)).type_0 = IL_ITEM_OPERAND;
    (*(*il).instr.offset((*il).length as isize))
        .c2rust_unnamed
        .d = x;
    (*(*il).instr.offset((*il).length as isize)).arity = 0 as arity_t;
    (*il).length = (*il).length.wrapping_add(1);
    (*il).free = (*il).free.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn il_push_VQ(mut il: *mut cff_CharstringIL, mut x: VQ) {
    il_push_operand(
        il,
        iVQ.getStill.expect("non-null function pointer")(x) as ::core::ffi::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn il_push_special(mut il: *mut cff_CharstringIL, mut s: int32_t) {
    ensureThereIsSpace(il);
    (*(*il).instr.offset((*il).length as isize)).type_0 = IL_ITEM_SPECIAL;
    (*(*il).instr.offset((*il).length as isize))
        .c2rust_unnamed
        .i = s;
    (*(*il).instr.offset((*il).length as isize)).arity = 0 as arity_t;
    (*il).length = (*il).length.wrapping_add(1);
    (*il).free = (*il).free.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn il_push_op(mut il: *mut cff_CharstringIL, mut op: int32_t) {
    ensureThereIsSpace(il);
    (*(*il).instr.offset((*il).length as isize)).type_0 = IL_ITEM_OPERATOR;
    (*(*il).instr.offset((*il).length as isize))
        .c2rust_unnamed
        .i = op;
    (*(*il).instr.offset((*il).length as isize)).arity =
        cff_getStandardArity(op as uint32_t) as arity_t;
    (*il).length = (*il).length.wrapping_add(1);
    (*il).free = (*il).free.wrapping_sub(1);
}
unsafe extern "C" fn il_moveto(mut il: *mut cff_CharstringIL, mut dx: VQ, mut dy: VQ) {
    il_push_VQ(il, dx);
    il_push_VQ(il, dy);
    il_push_op(il, op_rmoveto as ::core::ffi::c_int as int32_t);
}
unsafe extern "C" fn il_lineto(mut il: *mut cff_CharstringIL, mut dx: VQ, mut dy: VQ) {
    il_push_VQ(il, dx);
    il_push_VQ(il, dy);
    il_push_op(il, op_rlineto as ::core::ffi::c_int as int32_t);
}
unsafe extern "C" fn il_curveto(
    mut il: *mut cff_CharstringIL,
    mut dx1: VQ,
    mut dy1: VQ,
    mut dx2: VQ,
    mut dy2: VQ,
    mut dx3: VQ,
    mut dy3: VQ,
) {
    il_push_VQ(il, dx1);
    il_push_VQ(il, dy1);
    il_push_VQ(il, dx2);
    il_push_VQ(il, dy2);
    il_push_VQ(il, dx3);
    il_push_VQ(il, dy3);
    il_push_op(il, op_rrcurveto as ::core::ffi::c_int as int32_t);
}
unsafe extern "C" fn _il_push_maskgroup(
    mut il: *mut cff_CharstringIL,
    mut masks: *mut glyf_MaskList,
    mut contours: uint16_t,
    mut points: uint16_t,
    mut nh: uint16_t,
    mut nv: uint16_t,
    mut jm: *mut uint16_t,
    mut op: int32_t,
) {
    let mut n: shapeid_t = (*masks).length as shapeid_t;
    while (*jm as ::core::ffi::c_int) < n as ::core::ffi::c_int
        && (((*(*masks).items.offset(*jm as isize)).contoursBefore as ::core::ffi::c_int)
            < contours as ::core::ffi::c_int
            || (*(*masks).items.offset(*jm as isize)).contoursBefore as ::core::ffi::c_int
                == contours as ::core::ffi::c_int
                && (*(*masks).items.offset(*jm as isize)).pointsBefore as ::core::ffi::c_int
                    <= points as ::core::ffi::c_int)
    {
        il_push_op(il, op);
        let mut maskByte: uint8_t = 0 as uint8_t;
        let mut bits: uint8_t = 0 as uint8_t;
        let mut j: uint16_t = 0 as uint16_t;
        while (j as ::core::ffi::c_int) < nh as ::core::ffi::c_int {
            maskByte = ((maskByte as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (*(*masks).items.offset(*jm as isize)).maskH[j as usize] as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int) as uint8_t;
            bits = (bits as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
            if bits as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                il_push_special(il, maskByte as int32_t);
                bits = 0 as uint8_t;
            }
            j = j.wrapping_add(1);
        }
        let mut j_0: uint16_t = 0 as uint16_t;
        while (j_0 as ::core::ffi::c_int) < nv as ::core::ffi::c_int {
            maskByte = ((maskByte as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (*(*masks).items.offset(*jm as isize)).maskV[j_0 as usize] as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int) as uint8_t;
            bits = (bits as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
            if bits as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                il_push_special(il, maskByte as int32_t);
                bits = 0 as uint8_t;
            }
            j_0 = j_0.wrapping_add(1);
        }
        if bits != 0 {
            maskByte = ((maskByte as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - bits as ::core::ffi::c_int)
                as uint8_t;
            il_push_special(il, maskByte as int32_t);
        }
        *jm = (*jm as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint16_t;
    }
}
unsafe extern "C" fn il_push_masks(
    mut il: *mut cff_CharstringIL,
    mut g: *mut glyf_Glyph,
    mut contours: uint16_t,
    mut points: uint16_t,
    mut jh: *mut uint16_t,
    mut jm: *mut uint16_t,
) {
    if (*g).stemH.length == 0 && (*g).stemV.length == 0 {
        return;
    }
    _il_push_maskgroup(
        il,
        &raw mut (*g).contourMasks,
        contours,
        points,
        (*g).stemH.length as uint16_t,
        (*g).stemV.length as uint16_t,
        jh,
        op_cntrmask as ::core::ffi::c_int as int32_t,
    );
    _il_push_maskgroup(
        il,
        &raw mut (*g).hintMasks,
        contours,
        points,
        (*g).stemH.length as uint16_t,
        (*g).stemV.length as uint16_t,
        jm,
        op_hintmask as ::core::ffi::c_int as int32_t,
    );
}
unsafe extern "C" fn _il_push_stemgroup(
    mut il: *mut cff_CharstringIL,
    mut stems: *mut glyf_StemDefList,
    mut hasmask: bool,
    mut haswidth: bool,
    mut ophm: int32_t,
    mut oph: int32_t,
) {
    if stems.is_null() || (*stems).length == 0 {
        return;
    }
    let mut ref_0: pos_t = 0 as ::core::ffi::c_int as pos_t;
    let mut nn: uint16_t = (if haswidth as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as uint16_t;
    let mut j: uint16_t = 0 as uint16_t;
    while (j as size_t) < (*stems).length {
        il_push_operand(
            il,
            (*(*stems).items.offset(j as isize)).position as ::core::ffi::c_double
                - ref_0 as ::core::ffi::c_double,
        );
        il_push_operand(
            il,
            (*(*stems).items.offset(j as isize)).width as ::core::ffi::c_double,
        );
        ref_0 = (*(*stems).items.offset(j as isize)).position
            + (*(*stems).items.offset(j as isize)).width;
        nn = nn.wrapping_add(1);
        if nn as ::core::ffi::c_int >= type2_argument_stack as ::core::ffi::c_int {
            if hasmask {
                il_push_op(il, op_hstemhm as ::core::ffi::c_int as int32_t);
            } else {
                il_push_op(il, op_hstem as ::core::ffi::c_int as int32_t);
            }
            (*(*il)
                .instr
                .offset((*il).length.wrapping_sub(1 as uint32_t) as isize))
            .arity = nn as arity_t;
            nn = 0 as uint16_t;
        }
        j = j.wrapping_add(1);
    }
    if hasmask {
        il_push_op(il, ophm);
    } else {
        il_push_op(il, oph);
    }
    (*(*il)
        .instr
        .offset((*il).length.wrapping_sub(1 as uint32_t) as isize))
    .arity = nn as arity_t;
}
unsafe extern "C" fn il_push_stems(
    mut il: *mut cff_CharstringIL,
    mut g: *mut glyf_Glyph,
    mut hasmask: bool,
    mut haswidth: bool,
) {
    _il_push_stemgroup(
        il,
        &raw mut (*g).stemH,
        hasmask,
        haswidth,
        op_hstemhm as ::core::ffi::c_int as int32_t,
        op_hstem as ::core::ffi::c_int as int32_t,
    );
    _il_push_stemgroup(
        il,
        &raw mut (*g).stemV,
        hasmask,
        haswidth,
        op_vstemhm as ::core::ffi::c_int as int32_t,
        op_vstem as ::core::ffi::c_int as int32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cff_compileGlyphToIL(
    mut g: *mut glyf_Glyph,
    mut defaultWidth: uint16_t,
    mut nominalWidth: uint16_t,
) -> *mut cff_CharstringIL {
    let mut il: *mut cff_CharstringIL = ::core::ptr::null_mut::<cff_CharstringIL>();
    il = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_CharstringIL>() as size_t,
        143 as ::core::ffi::c_ulong,
    ) as *mut cff_CharstringIL;
    let mut tempContours: *mut glyf_Contour = ::core::ptr::null_mut::<glyf_Contour>();
    let mut x: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    let mut y: VQ =
        (iVQ.neutral.expect("non-null function pointer"))();
    tempContours = __caryll_allocate_clean(
        (::core::mem::size_of::<glyf_Contour>() as size_t).wrapping_mul((*g).contours.length),
        149 as ::core::ffi::c_ulong,
    ) as *mut glyf_Contour;
    let mut c: uint16_t = 0 as uint16_t;
    while (c as size_t) < (*g).contours.length {
        let mut contour: *mut glyf_Contour =
            (*g).contours.items.offset(c as isize) as *mut glyf_Contour;
        let mut newcontour: *mut glyf_Contour =
            tempContours.offset(c as isize) as *mut glyf_Contour;
        glyf_iContour.init.expect("non-null function pointer")(newcontour);
        let mut j: shapeid_t = 0 as shapeid_t;
        while (j as size_t) < (*contour).length {
            glyf_iContour.push.expect("non-null function pointer")(
                newcontour,
                glyf_iPoint.dup.expect("non-null function pointer")(
                    *(*contour).items.offset(j as isize),
                ),
            );
            j = j.wrapping_add(1);
        }
        if (*newcontour).length > 2 as size_t
            && (*(*newcontour)
                .items
                .offset((*newcontour).length.wrapping_sub(1 as size_t) as isize))
            .onCurve
                == 0
        {
            glyf_iContour.push.expect("non-null function pointer")(
                newcontour,
                glyf_iPoint.dup.expect("non-null function pointer")(
                    *(*newcontour).items.offset(0 as ::core::ffi::c_int as isize),
                ),
            );
        }
        let mut j_0: shapeid_t = 0 as shapeid_t;
        while (j_0 as size_t) < (*newcontour).length {
            let mut dx: VQ = iVQ.minus.expect("non-null function pointer")(
                (*(*newcontour).items.offset(j_0 as isize)).x,
                x,
            );
            let mut dy: VQ = iVQ.minus.expect("non-null function pointer")(
                (*(*newcontour).items.offset(j_0 as isize)).y,
                y,
            );
            iVQ.copyReplace.expect("non-null function pointer")(
                &raw mut x,
                (*(*newcontour).items.offset(j_0 as isize)).x,
            );
            iVQ.copyReplace.expect("non-null function pointer")(
                &raw mut y,
                (*(*newcontour).items.offset(j_0 as isize)).y,
            );
            iVQ.replace.expect("non-null function pointer")(
                &raw mut (*(*newcontour).items.offset(j_0 as isize)).x,
                dx,
            );
            iVQ.replace.expect("non-null function pointer")(
                &raw mut (*(*newcontour).items.offset(j_0 as isize)).y,
                dy,
            );
            j_0 = j_0.wrapping_add(1);
        }
        c = c.wrapping_add(1);
    }
    iVQ.dispose.expect("non-null function pointer")(&raw mut x);
    iVQ.dispose.expect("non-null function pointer")(&raw mut y);
    let mut hasmask: bool = (*g).hintMasks.length != 0 || (*g).contourMasks.length != 0;
    let glyphADWConst: pos_t =
        iVQ.getStill.expect("non-null function pointer")((*g).advanceWidth) as pos_t;
    let mut haswidth: bool = glyphADWConst != defaultWidth as ::core::ffi::c_int as pos_t;
    if haswidth {
        il_push_operand(
            il,
            (glyphADWConst as ::core::ffi::c_int - nominalWidth as ::core::ffi::c_int)
                as ::core::ffi::c_double,
        );
    }
    il_push_stems(il, g, hasmask, haswidth);
    let mut contoursSofar: shapeid_t = 0 as shapeid_t;
    let mut pointsSofar: shapeid_t = 0 as shapeid_t;
    let mut jh: shapeid_t = 0 as shapeid_t;
    let mut jm: shapeid_t = 0 as shapeid_t;
    if hasmask {
        il_push_masks(
            il,
            g,
            contoursSofar as uint16_t,
            pointsSofar as uint16_t,
            &raw mut jh,
            &raw mut jm,
        );
    }
    let mut c_0: shapeid_t = 0 as shapeid_t;
    while (c_0 as size_t) < (*g).contours.length {
        let mut contour_0: *mut glyf_Contour =
            tempContours.offset(c_0 as isize) as *mut glyf_Contour;
        let mut n: shapeid_t = (*contour_0).length as shapeid_t;
        if !(n as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
            il_moveto(
                il,
                (*(*contour_0).items.offset(0 as ::core::ffi::c_int as isize)).x,
                (*(*contour_0).items.offset(0 as ::core::ffi::c_int as isize)).y,
            );
            pointsSofar = pointsSofar.wrapping_add(1);
            if hasmask {
                il_push_masks(
                    il,
                    g,
                    contoursSofar as uint16_t,
                    pointsSofar as uint16_t,
                    &raw mut jh,
                    &raw mut jm,
                );
            }
            let mut j_1: shapeid_t = 1 as shapeid_t;
            while (j_1 as ::core::ffi::c_int) < n as ::core::ffi::c_int {
                if (*(*contour_0).items.offset(j_1 as isize)).onCurve != 0 {
                    il_lineto(
                        il,
                        (*(*contour_0).items.offset(j_1 as isize)).x,
                        (*(*contour_0).items.offset(j_1 as isize)).y,
                    );
                    pointsSofar =
                        (pointsSofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
                } else if (j_1 as ::core::ffi::c_int)
                    < n as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                    && (*(*contour_0)
                        .items
                        .offset((j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .onCurve
                        == 0
                    && (*(*contour_0)
                        .items
                        .offset((j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                    .onCurve as ::core::ffi::c_int
                        != 0
                {
                    il_curveto(
                        il,
                        (*(*contour_0).items.offset(j_1 as isize)).x,
                        (*(*contour_0).items.offset(j_1 as isize)).y,
                        (*(*contour_0).items.offset(
                            (j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .x,
                        (*(*contour_0).items.offset(
                            (j_1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .y,
                        (*(*contour_0).items.offset(
                            (j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                        ))
                        .x,
                        (*(*contour_0).items.offset(
                            (j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                        ))
                        .y,
                    );
                    pointsSofar =
                        (pointsSofar as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as shapeid_t;
                    j_1 = (j_1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as shapeid_t;
                } else {
                    il_lineto(
                        il,
                        (*(*contour_0).items.offset(j_1 as isize)).x,
                        (*(*contour_0).items.offset(j_1 as isize)).y,
                    );
                    pointsSofar = pointsSofar.wrapping_add(1);
                }
                if hasmask {
                    il_push_masks(
                        il,
                        g,
                        contoursSofar as uint16_t,
                        pointsSofar as uint16_t,
                        &raw mut jh,
                        &raw mut jm,
                    );
                }
                j_1 = j_1.wrapping_add(1);
            }
            contoursSofar =
                (contoursSofar as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as shapeid_t;
            pointsSofar = 0 as shapeid_t;
        }
        c_0 = c_0.wrapping_add(1);
    }
    il_push_op(il, op_endchar as ::core::ffi::c_int as int32_t);
    let mut c_1: shapeid_t = 0 as shapeid_t;
    while (c_1 as size_t) < (*g).contours.length {
        glyf_iContour.dispose.expect("non-null function pointer")(
            tempContours.offset(c_1 as isize) as *mut glyf_Contour,
        );
        c_1 = c_1.wrapping_add(1);
    }
    free(tempContours as *mut ::core::ffi::c_void);
    tempContours = ::core::ptr::null_mut::<glyf_Contour>();
    return il;
}
unsafe extern "C" fn il_matchtype(
    mut il: *mut cff_CharstringIL,
    mut j: uint32_t,
    mut k: uint32_t,
    mut t: cff_InstructionType,
) -> bool {
    if k >= (*il).length {
        return false;
    }
    let mut m: uint32_t = j;
    while m < k {
        if (*(*il).instr.offset(m as isize)).type_0 as ::core::ffi::c_uint
            != t as ::core::ffi::c_uint
        {
            return false;
        }
        m = m.wrapping_add(1);
    }
    return true;
}
unsafe extern "C" fn il_matchop(
    mut il: *mut cff_CharstringIL,
    mut j: uint32_t,
    mut op: int32_t,
) -> bool {
    if (*(*il).instr.offset(j as isize)).type_0 as ::core::ffi::c_uint
        != IL_ITEM_OPERATOR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false;
    }
    if (*(*il).instr.offset(j as isize)).c2rust_unnamed.i != op {
        return false;
    }
    return true;
}
unsafe extern "C" fn zroll(
    mut il: *mut cff_CharstringIL,
    mut j: uint32_t,
    mut op: int32_t,
    mut op2: int32_t,
    mut args: ...
) -> uint8_t {
    let mut arity: uint8_t = cff_getStandardArity(op as uint32_t);
    if arity as ::core::ffi::c_int > 16 as ::core::ffi::c_int
        || j.wrapping_add(arity as uint32_t) >= (*il).length
    {
        return 0 as uint8_t;
    }
    if (j == 0 as uint32_t
        || !il_matchtype(
            il,
            j.wrapping_sub(1 as uint32_t),
            j,
            IL_ITEM_PHANTOM_OPERATOR,
        ))
        && il_matchop(il, j.wrapping_add(arity as uint32_t), op) as ::core::ffi::c_int != 0
        && il_matchtype(il, j, j.wrapping_add(arity as uint32_t), IL_ITEM_OPERAND)
            as ::core::ffi::c_int
            != 0
    {
        let mut ap: ::core::ffi::VaListImpl;
        let mut check: uint8_t = true_0 as uint8_t;
        let mut resultArity: uint8_t = arity;
        let mut mask: [bool; 16] = [false; 16];
        ap = args.clone();
        let mut m: uint32_t = 0 as uint32_t;
        while m < arity as uint32_t {
            let mut checkzero: ::core::ffi::c_int = ap.arg::<::core::ffi::c_int>();
            mask[m as usize] = checkzero != 0;
            if checkzero != 0 {
                resultArity =
                    (resultArity as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint8_t;
                check = (check as ::core::ffi::c_int != 0
                    && (*(*il).instr.offset(j.wrapping_add(m) as isize))
                        .c2rust_unnamed
                        .d
                        == 0 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_int as uint8_t;
            }
            m = m.wrapping_add(1);
        }
        if check != 0 {
            let mut m_0: uint32_t = 0 as uint32_t;
            while m_0 < arity as uint32_t {
                if mask[m_0 as usize] {
                    (*(*il).instr.offset(j.wrapping_add(m_0) as isize)).type_0 =
                        IL_ITEM_PHANTOM_OPERAND;
                }
                m_0 = m_0.wrapping_add(1);
            }
            (*(*il)
                .instr
                .offset(j.wrapping_add(arity as uint32_t) as isize))
            .c2rust_unnamed
            .i = op2;
            (*(*il)
                .instr
                .offset(j.wrapping_add(arity as uint32_t) as isize))
            .arity = resultArity as arity_t;
            return arity;
        } else {
            return 0 as uint8_t;
        }
    } else {
        return 0 as uint8_t;
    };
}
unsafe extern "C" fn opop_roll(
    mut il: *mut cff_CharstringIL,
    mut j: uint32_t,
    mut op1: int32_t,
    mut arity: int32_t,
    mut op2: int32_t,
    mut resultop: int32_t,
) -> uint8_t {
    if j.wrapping_add(1 as uint32_t)
        .wrapping_add(arity as uint32_t)
        >= (*il).length
    {
        return 0 as uint8_t;
    }
    let mut current: *mut cff_CharstringInstruction =
        (*il).instr.offset(j as isize) as *mut cff_CharstringInstruction;
    let mut nextop: *mut cff_CharstringInstruction = (*il).instr.offset(
        j.wrapping_add(1 as uint32_t)
            .wrapping_add(arity as uint32_t) as isize,
    ) as *mut cff_CharstringInstruction;
    if il_matchop(il, j, op1) as ::core::ffi::c_int != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as uint32_t),
            j.wrapping_add(1 as uint32_t)
                .wrapping_add(arity as uint32_t),
            IL_ITEM_OPERAND,
        ) as ::core::ffi::c_int
            != 0
        && il_matchop(
            il,
            j.wrapping_add(1 as uint32_t)
                .wrapping_add(arity as uint32_t),
            op2,
        ) as ::core::ffi::c_int
            != 0
        && (*current).arity.wrapping_add((*nextop).arity)
            <= type2_argument_stack as ::core::ffi::c_int as arity_t
    {
        (*current).type_0 = IL_ITEM_PHANTOM_OPERATOR;
        (*nextop).c2rust_unnamed.i = resultop;
        (*nextop).arity = (*nextop).arity.wrapping_add((*current).arity);
        return (arity + 1 as int32_t) as uint8_t;
    } else {
        return 0 as uint8_t;
    };
}
unsafe extern "C" fn hvlineto_roll(mut il: *mut cff_CharstringIL, mut j: uint32_t) -> uint8_t {
    if j.wrapping_add(3 as uint32_t) >= (*il).length {
        return 0 as uint8_t;
    }
    let mut current: *mut cff_CharstringInstruction =
        (*il).instr.offset(j as isize) as *mut cff_CharstringInstruction;
    let mut checkdelta: uint32_t = (if ((*current).arity & 1 as arity_t != 0) as ::core::ffi::c_int
        ^ ((*current).c2rust_unnamed.i == op_vlineto as ::core::ffi::c_int as int32_t)
            as ::core::ffi::c_int
        != 0
    {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as uint32_t;
    if (il_matchop(il, j, op_hlineto as ::core::ffi::c_int as int32_t) as ::core::ffi::c_int != 0
        || il_matchop(il, j, op_vlineto as ::core::ffi::c_int as int32_t) as ::core::ffi::c_int
            != 0)
        && il_matchop(
            il,
            j.wrapping_add(3 as uint32_t),
            op_rlineto as ::core::ffi::c_int as int32_t,
        ) as ::core::ffi::c_int
            != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as uint32_t),
            j.wrapping_add(3 as uint32_t),
            IL_ITEM_OPERAND,
        ) as ::core::ffi::c_int
            != 0
        && (*(*il).instr.offset(j.wrapping_add(checkdelta) as isize))
            .c2rust_unnamed
            .d
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*current).arity.wrapping_add(1 as arity_t)
            <= type2_argument_stack as ::core::ffi::c_int as arity_t
    {
        (*(*il).instr.offset(j.wrapping_add(checkdelta) as isize)).type_0 = IL_ITEM_PHANTOM_OPERAND;
        (*(*il).instr.offset(j as isize)).type_0 = IL_ITEM_PHANTOM_OPERATOR;
        (*(*il).instr.offset(j.wrapping_add(3 as uint32_t) as isize))
            .c2rust_unnamed
            .i = (*current).c2rust_unnamed.i;
        (*(*il).instr.offset(j.wrapping_add(3 as uint32_t) as isize)).arity =
            (*current).arity.wrapping_add(1 as arity_t);
        return 3 as uint8_t;
    } else {
        return 0 as uint8_t;
    };
}
unsafe extern "C" fn hvvhcurve_roll(mut il: *mut cff_CharstringIL, mut j: uint32_t) -> uint8_t {
    if !il_matchop(il, j, op_hvcurveto as ::core::ffi::c_int as int32_t)
        && !il_matchop(il, j, op_vhcurveto as ::core::ffi::c_int as int32_t)
    {
        return 0 as uint8_t;
    }
    let mut current: *mut cff_CharstringInstruction =
        (*il).instr.offset(j as isize) as *mut cff_CharstringInstruction;
    if j.wrapping_add(7 as uint32_t) >= (*il).length || (*current).arity & 1 as arity_t != 0 {
        return 0 as uint8_t;
    }
    let mut hvcase: bool = ((*current).arity >> 2 as ::core::ffi::c_int & 1 as arity_t != 0)
        as ::core::ffi::c_int
        ^ ((*current).c2rust_unnamed.i == op_hvcurveto as ::core::ffi::c_int as int32_t)
            as ::core::ffi::c_int
        != 0;
    let mut checkdelta1: uint32_t = (if hvcase as ::core::ffi::c_int != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as uint32_t;
    let mut checkdelta2: uint32_t = (if hvcase as ::core::ffi::c_int != 0 {
        5 as ::core::ffi::c_int
    } else {
        6 as ::core::ffi::c_int
    }) as uint32_t;
    if il_matchop(
        il,
        j.wrapping_add(7 as uint32_t),
        op_rrcurveto as ::core::ffi::c_int as int32_t,
    ) as ::core::ffi::c_int
        != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as uint32_t),
            j.wrapping_add(7 as uint32_t),
            IL_ITEM_OPERAND,
        ) as ::core::ffi::c_int
            != 0
        && (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize))
            .c2rust_unnamed
            .d
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        if (*(*il).instr.offset(j.wrapping_add(checkdelta2) as isize))
            .c2rust_unnamed
            .d
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            && (*current).arity.wrapping_add(4 as arity_t)
                <= type2_argument_stack as ::core::ffi::c_int as arity_t
        {
            (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
                IL_ITEM_PHANTOM_OPERAND;
            (*(*il).instr.offset(j.wrapping_add(checkdelta2) as isize)).type_0 =
                IL_ITEM_PHANTOM_OPERAND;
            (*(*il).instr.offset(j as isize)).type_0 = IL_ITEM_PHANTOM_OPERATOR;
            (*(*il).instr.offset(j.wrapping_add(7 as uint32_t) as isize))
                .c2rust_unnamed
                .i = (*current).c2rust_unnamed.i;
            (*(*il).instr.offset(j.wrapping_add(7 as uint32_t) as isize)).arity =
                (*current).arity.wrapping_add(4 as arity_t);
            return 7 as uint8_t;
        } else if (*current).arity.wrapping_add(5 as arity_t)
            <= type2_argument_stack as ::core::ffi::c_int as arity_t
        {
            (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
                IL_ITEM_PHANTOM_OPERAND;
            (*(*il).instr.offset(j as isize)).type_0 = IL_ITEM_PHANTOM_OPERATOR;
            (*(*il).instr.offset(j.wrapping_add(7 as uint32_t) as isize))
                .c2rust_unnamed
                .i = (*current).c2rust_unnamed.i;
            (*(*il).instr.offset(j.wrapping_add(7 as uint32_t) as isize)).arity =
                (*current).arity.wrapping_add(5 as arity_t);
            if hvcase {
                let mut t: ::core::ffi::c_double =
                    (*(*il).instr.offset(j.wrapping_add(5 as uint32_t) as isize))
                        .c2rust_unnamed
                        .d;
                (*(*il).instr.offset(j.wrapping_add(5 as uint32_t) as isize))
                    .c2rust_unnamed
                    .d = (*(*il).instr.offset(j.wrapping_add(6 as uint32_t) as isize))
                    .c2rust_unnamed
                    .d;
                (*(*il).instr.offset(j.wrapping_add(6 as uint32_t) as isize))
                    .c2rust_unnamed
                    .d = t;
            }
            return 7 as uint8_t;
        } else {
            return 0 as uint8_t;
        }
    } else {
        return 0 as uint8_t;
    };
}
unsafe extern "C" fn hhvvcurve_roll(mut il: *mut cff_CharstringIL, mut j: uint32_t) -> uint8_t {
    if !il_matchop(il, j, op_hhcurveto as ::core::ffi::c_int as int32_t)
        && !il_matchop(il, j, op_vvcurveto as ::core::ffi::c_int as int32_t)
    {
        return 0 as uint8_t;
    }
    let mut current: *mut cff_CharstringInstruction =
        (*il).instr.offset(j as isize) as *mut cff_CharstringInstruction;
    if j.wrapping_add(7 as uint32_t) >= (*il).length {
        return 0 as uint8_t;
    }
    let mut hh: bool = (*current).c2rust_unnamed.i == op_hhcurveto as ::core::ffi::c_int as int32_t;
    let mut checkdelta1: uint32_t = (if hh as ::core::ffi::c_int != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as uint32_t;
    let mut checkdelta2: uint32_t = (if hh as ::core::ffi::c_int != 0 {
        6 as ::core::ffi::c_int
    } else {
        5 as ::core::ffi::c_int
    }) as uint32_t;
    if il_matchop(
        il,
        j.wrapping_add(7 as uint32_t),
        op_rrcurveto as ::core::ffi::c_int as int32_t,
    ) as ::core::ffi::c_int
        != 0
        && il_matchtype(
            il,
            j.wrapping_add(1 as uint32_t),
            j.wrapping_add(7 as uint32_t),
            IL_ITEM_OPERAND,
        ) as ::core::ffi::c_int
            != 0
        && (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize))
            .c2rust_unnamed
            .d
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*(*il).instr.offset(j.wrapping_add(checkdelta2) as isize))
            .c2rust_unnamed
            .d
            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && (*current).arity.wrapping_add(4 as arity_t)
            <= type2_argument_stack as ::core::ffi::c_int as arity_t
    {
        (*(*il).instr.offset(j.wrapping_add(checkdelta1) as isize)).type_0 =
            IL_ITEM_PHANTOM_OPERAND;
        (*(*il).instr.offset(j.wrapping_add(checkdelta2) as isize)).type_0 =
            IL_ITEM_PHANTOM_OPERAND;
        (*(*il).instr.offset(j as isize)).type_0 = IL_ITEM_PHANTOM_OPERATOR;
        (*(*il).instr.offset(j.wrapping_add(7 as uint32_t) as isize))
            .c2rust_unnamed
            .i = (*current).c2rust_unnamed.i;
        (*(*il).instr.offset(j.wrapping_add(7 as uint32_t) as isize)).arity =
            (*current).arity.wrapping_add(4 as arity_t);
        return 7 as uint8_t;
    } else {
        return 0 as uint8_t;
    };
}
unsafe extern "C" fn nextstop(mut il: *mut cff_CharstringIL, mut j: uint32_t) -> uint32_t {
    let mut delta: uint32_t = 0 as uint32_t;
    while j.wrapping_add(delta) < (*il).length
        && (*(*il).instr.offset(j.wrapping_add(delta) as isize)).type_0 as ::core::ffi::c_uint
            == IL_ITEM_OPERAND as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        delta = delta.wrapping_add(1);
    }
    return delta;
}
unsafe extern "C" fn decideAdvance(
    mut il: *mut cff_CharstringIL,
    mut j: uint32_t,
    mut _optimizeLevel: uint8_t,
) -> uint8_t {
    let mut r: uint8_t = 0 as uint8_t;
    r = zroll(
        il,
        j,
        op_rlineto as ::core::ffi::c_int as int32_t,
        op_hlineto as ::core::ffi::c_int as int32_t,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rlineto as ::core::ffi::c_int as int32_t,
        op_vlineto as ::core::ffi::c_int as int32_t,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rmoveto as ::core::ffi::c_int as int32_t,
        op_hmoveto as ::core::ffi::c_int as int32_t,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rmoveto as ::core::ffi::c_int as int32_t,
        op_vmoveto as ::core::ffi::c_int as int32_t,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as int32_t,
        op_hvcurveto as ::core::ffi::c_int as int32_t,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as int32_t,
        op_vhcurveto as ::core::ffi::c_int as int32_t,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as int32_t,
        op_hhcurveto as ::core::ffi::c_int as int32_t,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = zroll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as int32_t,
        op_vvcurveto as ::core::ffi::c_int as int32_t,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as int32_t,
        6 as int32_t,
        op_rrcurveto as ::core::ffi::c_int as int32_t,
        op_rrcurveto as ::core::ffi::c_int as int32_t,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_rrcurveto as ::core::ffi::c_int as int32_t,
        2 as int32_t,
        op_rlineto as ::core::ffi::c_int as int32_t,
        op_rcurveline as ::core::ffi::c_int as int32_t,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_rlineto as ::core::ffi::c_int as int32_t,
        6 as int32_t,
        op_rrcurveto as ::core::ffi::c_int as int32_t,
        op_rlinecurve as ::core::ffi::c_int as int32_t,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_rlineto as ::core::ffi::c_int as int32_t,
        2 as int32_t,
        op_rlineto as ::core::ffi::c_int as int32_t,
        op_rlineto as ::core::ffi::c_int as int32_t,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_hstemhm as ::core::ffi::c_int as int32_t,
        0 as int32_t,
        op_hintmask as ::core::ffi::c_int as int32_t,
        op_hintmask as ::core::ffi::c_int as int32_t,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_vstemhm as ::core::ffi::c_int as int32_t,
        0 as int32_t,
        op_hintmask as ::core::ffi::c_int as int32_t,
        op_hintmask as ::core::ffi::c_int as int32_t,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_hstemhm as ::core::ffi::c_int as int32_t,
        0 as int32_t,
        op_cntrmask as ::core::ffi::c_int as int32_t,
        op_cntrmask as ::core::ffi::c_int as int32_t,
    );
    if r != 0 {
        return r;
    }
    r = opop_roll(
        il,
        j,
        op_vstemhm as ::core::ffi::c_int as int32_t,
        0 as int32_t,
        op_cntrmask as ::core::ffi::c_int as int32_t,
        op_cntrmask as ::core::ffi::c_int as int32_t,
    );
    if r != 0 {
        return r;
    }
    r = hvlineto_roll(il, j);
    if r != 0 {
        return r;
    }
    r = hhvvcurve_roll(il, j);
    if r != 0 {
        return r;
    }
    r = hvvhcurve_roll(il, j);
    if r != 0 {
        return r;
    }
    r = nextstop(il, j) as uint8_t;
    if r != 0 {
        return r;
    }
    return 1 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn cff_optimizeIL(
    mut il: *mut cff_CharstringIL,
    mut options: *const otfcc_Options,
) {
    if !(*options).cff_rollCharString {
        return;
    }
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*il).length {
        j = j.wrapping_add(
            decideAdvance(il, j, (*options).cff_rollCharString as uint8_t) as uint32_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn cff_build_IL(mut il: *mut cff_CharstringIL) -> *mut caryll_Buffer {
    let mut blob: *mut caryll_Buffer = bufnew();
    let mut j: uint16_t = 0 as uint16_t;
    while (j as uint32_t) < (*il).length {
        match (*(*il).instr.offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                cff_mergeCS2Operand(blob, (*(*il).instr.offset(j as isize)).c2rust_unnamed.d);
            }
            1 => {
                cff_mergeCS2Operator(blob, (*(*il).instr.offset(j as isize)).c2rust_unnamed.i);
            }
            2 => {
                cff_mergeCS2Special(
                    blob,
                    (*(*il).instr.offset(j as isize)).c2rust_unnamed.i as uint8_t,
                );
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    return blob;
}
#[no_mangle]
pub unsafe extern "C" fn cff_shrinkIL(mut il: *mut cff_CharstringIL) -> *mut cff_CharstringIL {
    let mut out: *mut cff_CharstringIL = ::core::ptr::null_mut::<cff_CharstringIL>();
    out = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_CharstringIL>() as size_t,
        457 as ::core::ffi::c_ulong,
    ) as *mut cff_CharstringIL;
    let mut j: uint16_t = 0 as uint16_t;
    while (j as uint32_t) < (*il).length {
        match (*(*il).instr.offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                il_push_operand(out, (*(*il).instr.offset(j as isize)).c2rust_unnamed.d);
            }
            1 => {
                il_push_op(out, (*(*il).instr.offset(j as isize)).c2rust_unnamed.i);
            }
            2 => {
                il_push_special(out, (*(*il).instr.offset(j as isize)).c2rust_unnamed.i);
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn cff_ILmergeIL(
    mut self_0: *mut cff_CharstringIL,
    mut il: *mut cff_CharstringIL,
) {
    let mut j: uint16_t = 0 as uint16_t;
    while (j as uint32_t) < (*il).length {
        match (*(*il).instr.offset(j as isize)).type_0 as ::core::ffi::c_uint {
            0 => {
                il_push_operand(self_0, (*(*il).instr.offset(j as isize)).c2rust_unnamed.d);
            }
            1 => {
                il_push_op(self_0, (*(*il).instr.offset(j as isize)).c2rust_unnamed.i);
            }
            2 => {
                il_push_special(self_0, (*(*il).instr.offset(j as isize)).c2rust_unnamed.i);
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn instruction_eq(
    mut z1: *mut cff_CharstringInstruction,
    mut z2: *mut cff_CharstringInstruction,
) -> bool {
    if (*z1).type_0 as ::core::ffi::c_uint == (*z2).type_0 as ::core::ffi::c_uint {
        if (*z1).type_0 as ::core::ffi::c_uint
            == IL_ITEM_OPERAND as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*z1).type_0 as ::core::ffi::c_uint
                == IL_ITEM_PHANTOM_OPERAND as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return (*z1).c2rust_unnamed.d == (*z2).c2rust_unnamed.d;
        } else {
            return (*z1).c2rust_unnamed.i == (*z2).c2rust_unnamed.i;
        }
    } else {
        return false;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_ilEqual(
    mut a: *mut cff_CharstringIL,
    mut b: *mut cff_CharstringIL,
) -> bool {
    if a.is_null() || b.is_null() {
        return false;
    }
    if (*a).length != (*b).length {
        return false;
    }
    let mut j: uint32_t = 0 as uint32_t;
    while j < (*a).length {
        if !instruction_eq((*a).instr.offset(j as isize), (*b).instr.offset(j as isize)) {
            return false;
        }
        j = j.wrapping_add(1);
    }
    return true;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
