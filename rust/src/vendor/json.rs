extern "C" {
    fn malloc(__size: usize) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: usize,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: usize,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[cfg(not(target_os = "macos"))]
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
#[cfg(target_os = "macos")]
use crate::support::ctype_compat::__ctype_b_loc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_settings {
    pub max_memory: ::core::ffi::c_ulong,
    pub settings: ::core::ffi::c_int,
    pub mem_alloc: Option<
        unsafe extern "C" fn(
            usize,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub mem_free:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub user_data: *mut ::core::ffi::c_void,
    pub value_extra: usize,
}
pub type json_type = ::core::ffi::c_uint;
pub const json_pre_serialized: json_type = 8;
pub const json_null: json_type = 7;
pub const json_boolean: json_type = 6;
pub const json_string: json_type = 5;
pub const json_double: json_type = 4;
pub const json_integer: json_type = 3;
pub const json_array: json_type = 2;
pub const json_object: json_type = 1;
pub const json_none: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_value {
    pub parent: *mut _json_value,
    pub type_0: json_type,
    pub u: C2RustUnnamed_0,
    pub _reserved: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub next_alloc: *mut _json_value,
    pub object_mem: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub boolean: ::core::ffi::c_int,
    pub integer: i64,
    pub dbl: ::core::ffi::c_double,
    pub string: C2RustUnnamed_3,
    pub object: C2RustUnnamed_2,
    pub array: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub length: ::core::ffi::c_uint,
    pub values: *mut json_object_entry,
}
pub type json_object_entry = _json_object_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _json_object_entry {
    pub name: *mut ::core::ffi::c_char,
    pub name_length: ::core::ffi::c_uint,
    pub value: *mut _json_value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub length: ::core::ffi::c_uint,
    pub ptr: *mut ::core::ffi::c_char,
}
pub type json_value = _json_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_state {
    pub used_memory: ::core::ffi::c_ulong,
    pub uint_max: ::core::ffi::c_uint,
    pub ulong_max: ::core::ffi::c_ulong,
    pub settings: json_settings,
    pub first_pass: ::core::ffi::c_int,
    pub ptr: *const ::core::ffi::c_char,
    pub cur_line: ::core::ffi::c_uint,
    pub cur_col: ::core::ffi::c_uint,
}
pub const _ISdigit: C2RustUnnamed_4 = 2048;
pub type json_uchar = ::core::ffi::c_uint;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_4 = 8;
pub const _ISpunct: C2RustUnnamed_4 = 4;
pub const _IScntrl: C2RustUnnamed_4 = 2;
pub const _ISblank: C2RustUnnamed_4 = 1;
pub const _ISgraph: C2RustUnnamed_4 = 32768;
pub const _ISprint: C2RustUnnamed_4 = 16384;
pub const _ISspace: C2RustUnnamed_4 = 8192;
pub const _ISxdigit: C2RustUnnamed_4 = 4096;
pub const _ISalpha: C2RustUnnamed_4 = 1024;
pub const _ISlower: C2RustUnnamed_4 = 512;
pub const _ISupper: C2RustUnnamed_4 = 256;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const json_enable_comments: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut json_value_none: _json_value = _json_value {
    parent: ::core::ptr::null::<_json_value>() as *mut _json_value,
    type_0: json_none,
    u: C2RustUnnamed_0 { boolean: 0 },
    _reserved: C2RustUnnamed {
        next_alloc: ::core::ptr::null::<_json_value>() as *mut _json_value,
    },
};
unsafe extern "C" fn hex_value(mut c: ::core::ffi::c_char) -> ::core::ffi::c_uchar {
    if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        return (c as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uchar;
    }
    match c as ::core::ffi::c_int {
        97 | 65 => return 0xa as ::core::ffi::c_uchar,
        98 | 66 => return 0xb as ::core::ffi::c_uchar,
        99 | 67 => return 0xc as ::core::ffi::c_uchar,
        100 | 68 => return 0xd as ::core::ffi::c_uchar,
        101 | 69 => return 0xe as ::core::ffi::c_uchar,
        102 | 70 => return 0xf as ::core::ffi::c_uchar,
        _ => return 0xff as ::core::ffi::c_uchar,
    };
}
unsafe extern "C" fn default_alloc(
    mut size: usize,
    mut zero: ::core::ffi::c_int,
    mut _user_data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return if zero != 0 {
        calloc(1 as usize, size)
    } else {
        malloc(size)
    };
}
unsafe extern "C" fn default_free(
    mut ptr: *mut ::core::ffi::c_void,
    mut _user_data: *mut ::core::ffi::c_void,
) {
    free(ptr);
}
unsafe extern "C" fn json_alloc(
    mut state: *mut json_state,
    mut size: ::core::ffi::c_ulong,
    mut zero: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    if (*state).ulong_max.wrapping_sub((*state).used_memory) < size {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*state).settings.max_memory != 0 && {
        (*state).used_memory = (*state).used_memory.wrapping_add(size);
        (*state).used_memory > (*state).settings.max_memory
    } {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*state)
        .settings
        .mem_alloc
        .expect("non-null function pointer")(
        size as usize, zero, (*state).settings.user_data
    );
}
unsafe extern "C" fn new_value(
    mut state: *mut json_state,
    mut top: *mut *mut json_value,
    mut root: *mut *mut json_value,
    mut alloc: *mut *mut json_value,
    mut type_0: json_type,
) -> ::core::ffi::c_int {
    let mut value: *mut json_value = ::core::ptr::null_mut::<json_value>();
    let mut values_size: ::core::ffi::c_int = 0;
    if (*state).first_pass == 0 {
        *top = *alloc;
        value = *top;
        *alloc = (**alloc)._reserved.next_alloc as *mut json_value;
        if (*root).is_null() {
            *root = value;
        }
        match (*value).type_0 as ::core::ffi::c_uint {
            2 => {
                if !((*value).u.array.length == 0 as ::core::ffi::c_uint) {
                    (*value).u.array.values = json_alloc(
                        state,
                        ((*value).u.array.length as ::core::ffi::c_ulong).wrapping_mul(
                            ::core::mem::size_of::<*mut json_value>() as ::core::ffi::c_ulong,
                        ),
                        0 as ::core::ffi::c_int,
                    ) as *mut *mut json_value
                        as *mut *mut _json_value;
                    if (*value).u.array.values.is_null() {
                        return 0 as ::core::ffi::c_int;
                    }
                    (*value).u.array.length = 0 as ::core::ffi::c_uint;
                }
            }
            1 => {
                if !((*value).u.object.length == 0 as ::core::ffi::c_uint) {
                    values_size = ::core::mem::size_of::<json_object_entry>()
                        .wrapping_mul((*value).u.object.length as usize)
                        as ::core::ffi::c_int;
                    (*value).u.object.values = json_alloc(
                        state,
                        (values_size as ::core::ffi::c_ulong)
                            .wrapping_add((*value).u.object.values as ::core::ffi::c_ulong),
                        0 as ::core::ffi::c_int,
                    ) as *mut json_object_entry;
                    if (*value).u.object.values.is_null() {
                        return 0 as ::core::ffi::c_int;
                    }
                    (*value)._reserved.object_mem = (*(&raw mut (*value).u.object.values
                        as *mut *mut ::core::ffi::c_char))
                        .offset(values_size as isize)
                        as *mut ::core::ffi::c_void;
                    (*value).u.object.length = 0 as ::core::ffi::c_uint;
                }
            }
            5 => {
                (*value).u.string.ptr = json_alloc(
                    state,
                    ((*value)
                        .u
                        .string
                        .length
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        as ::core::ffi::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<::core::ffi::c_char>() as ::core::ffi::c_ulong
                        ),
                    0 as ::core::ffi::c_int,
                ) as *mut ::core::ffi::c_char;
                if (*value).u.string.ptr.is_null() {
                    return 0 as ::core::ffi::c_int;
                }
                (*value).u.string.length = 0 as ::core::ffi::c_uint;
            }
            _ => {}
        }
        return 1 as ::core::ffi::c_int;
    }
    value = json_alloc(
        state,
        (::core::mem::size_of::<json_value>() as ::core::ffi::c_ulong)
            .wrapping_add((*state).settings.value_extra as ::core::ffi::c_ulong),
        1 as ::core::ffi::c_int,
    ) as *mut json_value;
    if value.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*root).is_null() {
        *root = value;
    }
    (*value).type_0 = type_0;
    (*value).parent = *top as *mut _json_value;
    if !(*alloc).is_null() {
        (**alloc)._reserved.next_alloc = value as *mut _json_value;
    }
    *top = value;
    *alloc = *top;
    return 1 as ::core::ffi::c_int;
}
static mut flag_line_comment: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_seek_value: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_string: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_done: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_need_comma: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_need_colon: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_block_comment: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_next: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_reproc: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_escaped: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_num_negative: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_num_e_negative: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_num_e_got_sign: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_num_zero: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as ::core::ffi::c_long;
static mut flag_num_e: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as ::core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C" fn json_parse_ex(
    mut settings: *mut json_settings,
    mut json: *const ::core::ffi::c_char,
    mut length: usize,
    mut error_buf: *mut ::core::ffi::c_char,
) -> *mut json_value {
    let mut current_block: u64;
    let mut error: [::core::ffi::c_char; 128] = [0; 128];
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut top: *mut json_value = ::core::ptr::null_mut::<json_value>();
    let mut root: *mut json_value = ::core::ptr::null_mut::<json_value>();
    let mut alloc: *mut json_value = ::core::ptr::null_mut::<json_value>();
    let mut state: json_state = json_state {
        used_memory: 0,
        uint_max: 0,
        ulong_max: 0,
        settings: json_settings {
            max_memory: 0,
            settings: 0,
            mem_alloc: None,
            mem_free: None,
            user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            value_extra: 0,
        },
        first_pass: 0,
        ptr: ::core::ptr::null::<::core::ffi::c_char>(),
        cur_line: 0,
        cur_col: 0,
    };
    state.used_memory = 0 as ::core::ffi::c_ulong;
    state.uint_max = 0 as ::core::ffi::c_uint;
    state.ulong_max = 0 as ::core::ffi::c_ulong;
    state.settings.max_memory = 0 as ::core::ffi::c_ulong;
    state.settings.mem_alloc = None;
    state.settings.mem_free = None;
    state.settings.settings = 0 as ::core::ffi::c_int;
    state.settings.user_data = NULL;
    state.settings.value_extra = 0 as usize;
    state.first_pass = 0 as ::core::ffi::c_int;
    state.ptr = ::core::ptr::null::<::core::ffi::c_char>();
    state.cur_col = 0 as ::core::ffi::c_uint;
    state.cur_line = 0 as ::core::ffi::c_uint;
    let mut flags: ::core::ffi::c_long = 0;
    let mut num_digits: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut num_e: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut num_fraction: i64 = 0 as i64;
    if length >= 3 as usize
        && *json.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
            as ::core::ffi::c_int
            == 0xef as ::core::ffi::c_int
        && *json.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
            as ::core::ffi::c_int
            == 0xbb as ::core::ffi::c_int
        && *json.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
            as ::core::ffi::c_int
            == 0xbf as ::core::ffi::c_int
    {
        json = json.offset(3 as ::core::ffi::c_int as isize);
        length = length.wrapping_sub(3 as usize);
    }
    error[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    end = json.offset(length as isize);
    memcpy(
        &raw mut state.settings as *mut ::core::ffi::c_void,
        settings as *const ::core::ffi::c_void,
        ::core::mem::size_of::<json_settings>() as usize,
    );
    if state.settings.mem_alloc.is_none() {
        state.settings.mem_alloc = Some(
            default_alloc
                as unsafe extern "C" fn(
                    usize,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
        )
            as Option<
                unsafe extern "C" fn(
                    usize,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
            >;
    }
    if state.settings.mem_free.is_none() {
        state.settings.mem_free = Some(
            default_free
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        )
            as Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
            >;
    }
    memset(
        &raw mut state.uint_max as *mut ::core::ffi::c_void,
        0xff as ::core::ffi::c_int,
        ::core::mem::size_of::<::core::ffi::c_uint>() as usize,
    );
    memset(
        &raw mut state.ulong_max as *mut ::core::ffi::c_void,
        0xff as ::core::ffi::c_int,
        ::core::mem::size_of::<::core::ffi::c_ulong>() as usize,
    );
    state.uint_max = state.uint_max.wrapping_sub(8 as ::core::ffi::c_uint);
    state.ulong_max = state.ulong_max.wrapping_sub(8 as ::core::ffi::c_ulong);
    state.first_pass = 1 as ::core::ffi::c_int;
    's_107: loop {
        if !(state.first_pass >= 0 as ::core::ffi::c_int) {
            current_block = 14895111670561345133;
            break;
        }
        let mut uchar: json_uchar = 0;
        let mut uc_b1: ::core::ffi::c_uchar = 0;
        let mut uc_b2: ::core::ffi::c_uchar = 0;
        let mut uc_b3: ::core::ffi::c_uchar = 0;
        let mut uc_b4: ::core::ffi::c_uchar = 0;
        let mut string: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut string_length: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        root = ::core::ptr::null_mut::<json_value>();
        top = root;
        flags = flag_seek_value;
        state.cur_line = 1 as ::core::ffi::c_uint;
        state.ptr = json;
        loop {
            let mut b: ::core::ffi::c_char = (if state.ptr == end {
                0 as ::core::ffi::c_int
            } else {
                *state.ptr as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
            if flags & flag_string != 0 {
                if b == 0 {
                    sprintf(
                        &raw mut error as *mut ::core::ffi::c_char,
                        b"Unexpected EOF in string (at %d:%d)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        state.cur_line,
                        state.cur_col,
                    );
                    current_block = 14191169715820259248;
                    break 's_107;
                } else {
                    if string_length > state.uint_max {
                        current_block = 2680027254923815990;
                        break 's_107;
                    }
                    if flags & flag_escaped != 0 {
                        flags &= !flag_escaped;
                        match b as ::core::ffi::c_int {
                            98 => {
                                if state.first_pass == 0 {
                                    *string.offset(string_length as isize) =
                                        '\u{8}' as i32 as ::core::ffi::c_char;
                                }
                                string_length = string_length.wrapping_add(1);
                            }
                            102 => {
                                if state.first_pass == 0 {
                                    *string.offset(string_length as isize) =
                                        '\u{c}' as i32 as ::core::ffi::c_char;
                                }
                                string_length = string_length.wrapping_add(1);
                            }
                            110 => {
                                if state.first_pass == 0 {
                                    *string.offset(string_length as isize) =
                                        '\n' as i32 as ::core::ffi::c_char;
                                }
                                string_length = string_length.wrapping_add(1);
                            }
                            114 => {
                                if state.first_pass == 0 {
                                    *string.offset(string_length as isize) =
                                        '\r' as i32 as ::core::ffi::c_char;
                                }
                                string_length = string_length.wrapping_add(1);
                            }
                            116 => {
                                if state.first_pass == 0 {
                                    *string.offset(string_length as isize) =
                                        '\t' as i32 as ::core::ffi::c_char;
                                }
                                string_length = string_length.wrapping_add(1);
                            }
                            117 => {
                                if (end.offset_from(state.ptr) as ::core::ffi::c_long)
                                    < 4 as ::core::ffi::c_long
                                    || {
                                        state.ptr = state.ptr.offset(1);
                                        uc_b1 = hex_value(*state.ptr);
                                        uc_b1 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
                                    }
                                    || {
                                        state.ptr = state.ptr.offset(1);
                                        uc_b2 = hex_value(*state.ptr);
                                        uc_b2 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
                                    }
                                    || {
                                        state.ptr = state.ptr.offset(1);
                                        uc_b3 = hex_value(*state.ptr);
                                        uc_b3 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
                                    }
                                    || {
                                        state.ptr = state.ptr.offset(1);
                                        uc_b4 = hex_value(*state.ptr);
                                        uc_b4 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
                                    }
                                {
                                    sprintf(
                                        &raw mut error as *mut ::core::ffi::c_char,
                                        b"Invalid character value `%c` (at %d:%d)\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        b as ::core::ffi::c_int,
                                        state.cur_line,
                                        state.cur_col,
                                    );
                                    current_block = 14191169715820259248;
                                    break 's_107;
                                } else {
                                    uc_b1 = ((uc_b1 as ::core::ffi::c_int)
                                        << 4 as ::core::ffi::c_int
                                        | uc_b2 as ::core::ffi::c_int)
                                        as ::core::ffi::c_uchar;
                                    uc_b2 = ((uc_b3 as ::core::ffi::c_int)
                                        << 4 as ::core::ffi::c_int
                                        | uc_b4 as ::core::ffi::c_int)
                                        as ::core::ffi::c_uchar;
                                    uchar = ((uc_b1 as ::core::ffi::c_int)
                                        << 8 as ::core::ffi::c_int
                                        | uc_b2 as ::core::ffi::c_int)
                                        as json_uchar;
                                    if uchar & 0xf800 as json_uchar == 0xd800 as json_uchar {
                                        let mut uchar2: json_uchar = 0;
                                        if (end.offset_from(state.ptr) as ::core::ffi::c_long)
                                            < 6 as ::core::ffi::c_long
                                            || {
                                                state.ptr = state.ptr.offset(1);
                                                *state.ptr as ::core::ffi::c_int != '\\' as i32
                                            }
                                            || {
                                                state.ptr = state.ptr.offset(1);
                                                *state.ptr as ::core::ffi::c_int != 'u' as i32
                                            }
                                            || {
                                                state.ptr = state.ptr.offset(1);
                                                uc_b1 = hex_value(*state.ptr);
                                                uc_b1 as ::core::ffi::c_int
                                                    == 0xff as ::core::ffi::c_int
                                            }
                                            || {
                                                state.ptr = state.ptr.offset(1);
                                                uc_b2 = hex_value(*state.ptr);
                                                uc_b2 as ::core::ffi::c_int
                                                    == 0xff as ::core::ffi::c_int
                                            }
                                            || {
                                                state.ptr = state.ptr.offset(1);
                                                uc_b3 = hex_value(*state.ptr);
                                                uc_b3 as ::core::ffi::c_int
                                                    == 0xff as ::core::ffi::c_int
                                            }
                                            || {
                                                state.ptr = state.ptr.offset(1);
                                                uc_b4 = hex_value(*state.ptr);
                                                uc_b4 as ::core::ffi::c_int
                                                    == 0xff as ::core::ffi::c_int
                                            }
                                        {
                                            sprintf(
                                                &raw mut error as *mut ::core::ffi::c_char,
                                                b"Invalid character value `%c` (at %d:%d)\0"
                                                    as *const u8
                                                    as *const ::core::ffi::c_char,
                                                b as ::core::ffi::c_int,
                                                state.cur_line,
                                                state.cur_col,
                                            );
                                            current_block = 14191169715820259248;
                                            break 's_107;
                                        } else {
                                            uc_b1 = ((uc_b1 as ::core::ffi::c_int)
                                                << 4 as ::core::ffi::c_int
                                                | uc_b2 as ::core::ffi::c_int)
                                                as ::core::ffi::c_uchar;
                                            uc_b2 = ((uc_b3 as ::core::ffi::c_int)
                                                << 4 as ::core::ffi::c_int
                                                | uc_b4 as ::core::ffi::c_int)
                                                as ::core::ffi::c_uchar;
                                            uchar2 = ((uc_b1 as ::core::ffi::c_int)
                                                << 8 as ::core::ffi::c_int
                                                | uc_b2 as ::core::ffi::c_int)
                                                as json_uchar;
                                            uchar = (0x10000 as ::core::ffi::c_int as json_uchar)
                                                .wrapping_add(
                                                    (uchar & 0x3ff as json_uchar)
                                                        << 10 as ::core::ffi::c_int,
                                                )
                                                | uchar2 & 0x3ff as json_uchar;
                                        }
                                    }
                                    if ::core::mem::size_of::<::core::ffi::c_char>()
                                        >= ::core::mem::size_of::<json_uchar>()
                                        || uchar <= 0x7f as json_uchar
                                    {
                                        if state.first_pass == 0 {
                                            *string.offset(string_length as isize) =
                                                uchar as ::core::ffi::c_char;
                                        }
                                        string_length = string_length.wrapping_add(1);
                                    } else if uchar <= 0x7ff as json_uchar {
                                        if state.first_pass != 0 {
                                            string_length = string_length
                                                .wrapping_add(2 as ::core::ffi::c_uint);
                                        } else {
                                            let fresh0 = string_length;
                                            string_length = string_length.wrapping_add(1);
                                            *string.offset(fresh0 as isize) = (0xc0 as json_uchar
                                                | uchar >> 6 as ::core::ffi::c_int)
                                                as ::core::ffi::c_char;
                                            let fresh1 = string_length;
                                            string_length = string_length.wrapping_add(1);
                                            *string.offset(fresh1 as isize) = (0x80 as json_uchar
                                                | uchar & 0x3f as json_uchar)
                                                as ::core::ffi::c_char;
                                        }
                                    } else if uchar <= 0xffff as json_uchar {
                                        if state.first_pass != 0 {
                                            string_length = string_length
                                                .wrapping_add(3 as ::core::ffi::c_uint);
                                        } else {
                                            let fresh2 = string_length;
                                            string_length = string_length.wrapping_add(1);
                                            *string.offset(fresh2 as isize) = (0xe0 as json_uchar
                                                | uchar >> 12 as ::core::ffi::c_int)
                                                as ::core::ffi::c_char;
                                            let fresh3 = string_length;
                                            string_length = string_length.wrapping_add(1);
                                            *string.offset(fresh3 as isize) = (0x80 as json_uchar
                                                | uchar >> 6 as ::core::ffi::c_int
                                                    & 0x3f as json_uchar)
                                                as ::core::ffi::c_char;
                                            let fresh4 = string_length;
                                            string_length = string_length.wrapping_add(1);
                                            *string.offset(fresh4 as isize) = (0x80 as json_uchar
                                                | uchar & 0x3f as json_uchar)
                                                as ::core::ffi::c_char;
                                        }
                                    } else if state.first_pass != 0 {
                                        string_length =
                                            string_length.wrapping_add(4 as ::core::ffi::c_uint);
                                    } else {
                                        let fresh5 = string_length;
                                        string_length = string_length.wrapping_add(1);
                                        *string.offset(fresh5 as isize) = (0xf0 as json_uchar
                                            | uchar >> 18 as ::core::ffi::c_int)
                                            as ::core::ffi::c_char;
                                        let fresh6 = string_length;
                                        string_length = string_length.wrapping_add(1);
                                        *string.offset(fresh6 as isize) = (0x80 as json_uchar
                                            | uchar >> 12 as ::core::ffi::c_int
                                                & 0x3f as json_uchar)
                                            as ::core::ffi::c_char;
                                        let fresh7 = string_length;
                                        string_length = string_length.wrapping_add(1);
                                        *string.offset(fresh7 as isize) = (0x80 as json_uchar
                                            | uchar >> 6 as ::core::ffi::c_int & 0x3f as json_uchar)
                                            as ::core::ffi::c_char;
                                        let fresh8 = string_length;
                                        string_length = string_length.wrapping_add(1);
                                        *string.offset(fresh8 as isize) = (0x80 as json_uchar
                                            | uchar & 0x3f as json_uchar)
                                            as ::core::ffi::c_char;
                                    }
                                }
                            }
                            _ => {
                                if state.first_pass == 0 {
                                    *string.offset(string_length as isize) = b;
                                }
                                string_length = string_length.wrapping_add(1);
                            }
                        }
                        current_block = 11057878835866523405;
                    } else if b as ::core::ffi::c_int == '\\' as i32 {
                        flags |= flag_escaped;
                        current_block = 11057878835866523405;
                    } else if b as ::core::ffi::c_int == '"' as i32 {
                        if state.first_pass == 0 {
                            *string.offset(string_length as isize) = 0 as ::core::ffi::c_char;
                        }
                        flags &= !flag_string;
                        string = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        match (*top).type_0 as ::core::ffi::c_uint {
                            5 => {
                                current_block = 981276038192651198;
                                match current_block {
                                    17969198464700415374 => {
                                        if state.first_pass != 0 {
                                            let ref mut fresh9 = *(&raw mut (*top).u.object.values
                                                as *mut *mut ::core::ffi::c_char);
                                            *fresh9 = (*fresh9).offset(
                                                string_length.wrapping_add(1 as ::core::ffi::c_uint)
                                                    as isize,
                                            );
                                        } else {
                                            let ref mut fresh10 = (*(*top)
                                                .u
                                                .object
                                                .values
                                                .offset((*top).u.object.length as isize))
                                            .name;
                                            *fresh10 = (*top)._reserved.object_mem
                                                as *mut ::core::ffi::c_char;
                                            (*(*top)
                                                .u
                                                .object
                                                .values
                                                .offset((*top).u.object.length as isize))
                                            .name_length = string_length;
                                            let ref mut fresh11 =
                                                *(&raw mut (*top)._reserved.object_mem
                                                    as *mut *mut ::core::ffi::c_char);
                                            *fresh11 = (*fresh11).offset(
                                                string_length.wrapping_add(1 as ::core::ffi::c_uint)
                                                    as isize,
                                            );
                                        }
                                        flags |= flag_seek_value | flag_need_colon;
                                        current_block = 11057878835866523405;
                                    }
                                    _ => {
                                        (*top).u.string.length = string_length;
                                        flags |= flag_next;
                                        current_block = 16696653877814833746;
                                    }
                                }
                            }
                            1 => {
                                current_block = 17969198464700415374;
                                match current_block {
                                    17969198464700415374 => {
                                        if state.first_pass != 0 {
                                            let ref mut fresh9 = *(&raw mut (*top).u.object.values
                                                as *mut *mut ::core::ffi::c_char);
                                            *fresh9 = (*fresh9).offset(
                                                string_length.wrapping_add(1 as ::core::ffi::c_uint)
                                                    as isize,
                                            );
                                        } else {
                                            let ref mut fresh10 = (*(*top)
                                                .u
                                                .object
                                                .values
                                                .offset((*top).u.object.length as isize))
                                            .name;
                                            *fresh10 = (*top)._reserved.object_mem
                                                as *mut ::core::ffi::c_char;
                                            (*(*top)
                                                .u
                                                .object
                                                .values
                                                .offset((*top).u.object.length as isize))
                                            .name_length = string_length;
                                            let ref mut fresh11 =
                                                *(&raw mut (*top)._reserved.object_mem
                                                    as *mut *mut ::core::ffi::c_char);
                                            *fresh11 = (*fresh11).offset(
                                                string_length.wrapping_add(1 as ::core::ffi::c_uint)
                                                    as isize,
                                            );
                                        }
                                        flags |= flag_seek_value | flag_need_colon;
                                        current_block = 11057878835866523405;
                                    }
                                    _ => {
                                        (*top).u.string.length = string_length;
                                        flags |= flag_next;
                                        current_block = 16696653877814833746;
                                    }
                                }
                            }
                            _ => {
                                current_block = 16696653877814833746;
                            }
                        }
                    } else {
                        if state.first_pass == 0 {
                            *string.offset(string_length as isize) = b;
                        }
                        string_length = string_length.wrapping_add(1);
                        current_block = 11057878835866523405;
                    }
                }
            } else {
                current_block = 16696653877814833746;
            }
            match current_block {
                16696653877814833746 => {
                    if state.settings.settings & json_enable_comments != 0 {
                        if flags & (flag_line_comment | flag_block_comment) != 0 {
                            if flags & flag_line_comment != 0 {
                                if b as ::core::ffi::c_int == '\r' as i32
                                    || b as ::core::ffi::c_int == '\n' as i32
                                    || b == 0
                                {
                                    flags &= !flag_line_comment;
                                    state.ptr = state.ptr.offset(-1);
                                }
                                current_block = 11057878835866523405;
                            } else if flags & flag_block_comment != 0 {
                                if b == 0 {
                                    sprintf(
                                        &raw mut error as *mut ::core::ffi::c_char,
                                        b"%d:%d: Unexpected EOF in block comment\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        state.cur_line,
                                        state.cur_col,
                                    );
                                    current_block = 14191169715820259248;
                                    break 's_107;
                                } else if b as ::core::ffi::c_int == '*' as i32
                                    && state.ptr < end.offset(-(1 as ::core::ffi::c_int as isize))
                                    && *state.ptr.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '/' as i32
                                {
                                    flags &= !flag_block_comment;
                                    state.ptr = state.ptr.offset(1);
                                }
                                current_block = 11057878835866523405;
                            } else {
                                current_block = 4299703460566765016;
                            }
                        } else if b as ::core::ffi::c_int == '/' as i32 {
                            if flags & (flag_seek_value | flag_done) == 0
                                && (*top).type_0 as ::core::ffi::c_uint
                                    != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                sprintf(
                                    &raw mut error as *mut ::core::ffi::c_char,
                                    b"%d:%d: Comment not allowed here\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    state.cur_line,
                                    state.cur_col,
                                );
                                current_block = 14191169715820259248;
                                break 's_107;
                            } else {
                                state.ptr = state.ptr.offset(1);
                                if state.ptr == end {
                                    sprintf(
                                        &raw mut error as *mut ::core::ffi::c_char,
                                        b"%d:%d: EOF unexpected\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        state.cur_line,
                                        state.cur_col,
                                    );
                                    current_block = 14191169715820259248;
                                    break 's_107;
                                } else {
                                    b = *state.ptr;
                                    match b as ::core::ffi::c_int {
                                        47 => {
                                            flags |= flag_line_comment;
                                        }
                                        42 => {
                                            flags |= flag_block_comment;
                                        }
                                        _ => {
                                            sprintf(
                                                &raw mut error as *mut ::core::ffi::c_char,
                                                b"%d:%d: Unexpected `%c` in comment opening sequence\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                                state.cur_line,
                                                state.cur_col,
                                                b as ::core::ffi::c_int,
                                            );
                                            current_block = 14191169715820259248;
                                            break 's_107;
                                        }
                                    }
                                }
                            }
                            current_block = 11057878835866523405;
                        } else {
                            current_block = 4299703460566765016;
                        }
                    } else {
                        current_block = 4299703460566765016;
                    }
                    match current_block {
                        11057878835866523405 => {}
                        _ => {
                            if flags & flag_done != 0 {
                                if b == 0 {
                                    break;
                                }
                                match b as ::core::ffi::c_int {
                                    10 => {
                                        current_block = 9775494513900600499;
                                        match current_block {
                                            8367529978370297332 => {
                                                sprintf(
                                                    &raw mut error as *mut ::core::ffi::c_char,
                                                    b"%d:%d: Trailing garbage: `%c`\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    state.cur_line,
                                                    state.cur_col,
                                                    b as ::core::ffi::c_int,
                                                );
                                                current_block = 14191169715820259248;
                                                break 's_107;
                                            }
                                            _ => {
                                                state.cur_line = state.cur_line.wrapping_add(1);
                                                state.cur_col = 0 as ::core::ffi::c_uint;
                                            }
                                        }
                                    }
                                    32 | 9 | 13 => {}
                                    _ => {
                                        current_block = 8367529978370297332;
                                        match current_block {
                                            8367529978370297332 => {
                                                sprintf(
                                                    &raw mut error as *mut ::core::ffi::c_char,
                                                    b"%d:%d: Trailing garbage: `%c`\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    state.cur_line,
                                                    state.cur_col,
                                                    b as ::core::ffi::c_int,
                                                );
                                                current_block = 14191169715820259248;
                                                break 's_107;
                                            }
                                            _ => {
                                                state.cur_line = state.cur_line.wrapping_add(1);
                                                state.cur_col = 0 as ::core::ffi::c_uint;
                                            }
                                        }
                                    }
                                }
                            } else {
                                if flags & flag_seek_value != 0 {
                                    match b as ::core::ffi::c_int {
                                        10 => {
                                            current_block = 15953825877604003206;
                                            match current_block {
                                                15953825877604003206 => {
                                                    state.cur_line = state.cur_line.wrapping_add(1);
                                                    state.cur_col = 0 as ::core::ffi::c_uint;
                                                    current_block = 11057878835866523405;
                                                }
                                                4996464336325100333 => {
                                                    if !top.is_null()
                                                        && (*top).type_0 as ::core::ffi::c_uint
                                                            == json_array as ::core::ffi::c_int
                                                                as ::core::ffi::c_uint
                                                    {
                                                        flags = flags
                                                            & !(flag_need_comma | flag_seek_value)
                                                            | flag_next;
                                                    } else {
                                                        sprintf(
                                                            &raw mut error
                                                                as *mut ::core::ffi::c_char,
                                                            b"%d:%d: Unexpected ]\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            state.cur_line,
                                                            state.cur_col,
                                                        );
                                                        current_block = 14191169715820259248;
                                                        break 's_107;
                                                    }
                                                    current_block = 11603475171617447446;
                                                }
                                                _ => {
                                                    if flags & flag_need_comma != 0 {
                                                        if b as ::core::ffi::c_int == ',' as i32 {
                                                            flags &= !flag_need_comma;
                                                        } else {
                                                            sprintf(
                                                                &raw mut error
                                                                    as *mut ::core::ffi::c_char,
                                                                b"%d:%d: Expected , before %c\0"
                                                                    as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                state.cur_line,
                                                                state.cur_col,
                                                                b as ::core::ffi::c_int,
                                                            );
                                                            current_block = 14191169715820259248;
                                                            break 's_107;
                                                        }
                                                        current_block = 11057878835866523405;
                                                    } else if flags & flag_need_colon != 0 {
                                                        if b as ::core::ffi::c_int == ':' as i32 {
                                                            flags &= !flag_need_colon;
                                                        } else {
                                                            sprintf(
                                                                &raw mut error
                                                                    as *mut ::core::ffi::c_char,
                                                                b"%d:%d: Expected : before %c\0"
                                                                    as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                state.cur_line,
                                                                state.cur_col,
                                                                b as ::core::ffi::c_int,
                                                            );
                                                            current_block = 14191169715820259248;
                                                            break 's_107;
                                                        }
                                                        current_block = 11057878835866523405;
                                                    } else {
                                                        flags &= !flag_seek_value;
                                                        match b as ::core::ffi::c_int {
                                                            123 => {
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_object,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                current_block =
                                                                    11057878835866523405;
                                                            }
                                                            91 => {
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_array,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_seek_value;
                                                                current_block =
                                                                    11057878835866523405;
                                                            }
                                                            34 => {
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_string,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_string;
                                                                string = (*top).u.string.ptr;
                                                                string_length =
                                                                    0 as ::core::ffi::c_uint;
                                                                current_block =
                                                                    11057878835866523405;
                                                            }
                                                            116 => {
                                                                if (end.offset_from(state.ptr)
                                                                    as ::core::ffi::c_long)
                                                                    < 3 as ::core::ffi::c_long
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'r' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'u' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'e' as i32
                                                                    }
                                                                {
                                                                    current_block =
                                                                        11800234908647806265;
                                                                    break 's_107;
                                                                }
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_boolean,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                (*top).u.boolean =
                                                                    1 as ::core::ffi::c_int;
                                                                flags |= flag_next;
                                                                current_block =
                                                                    11603475171617447446;
                                                            }
                                                            102 => {
                                                                if (end.offset_from(state.ptr)
                                                                    as ::core::ffi::c_long)
                                                                    < 4 as ::core::ffi::c_long
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'a' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'l' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 's' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'e' as i32
                                                                    }
                                                                {
                                                                    current_block =
                                                                        11800234908647806265;
                                                                    break 's_107;
                                                                }
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_boolean,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_next;
                                                                current_block =
                                                                    11603475171617447446;
                                                            }
                                                            110 => {
                                                                if (end.offset_from(state.ptr)
                                                                    as ::core::ffi::c_long)
                                                                    < 3 as ::core::ffi::c_long
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'u' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'l' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'l' as i32
                                                                    }
                                                                {
                                                                    current_block =
                                                                        11800234908647806265;
                                                                    break 's_107;
                                                                }
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_null,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_next;
                                                                current_block =
                                                                    11603475171617447446;
                                                            }
                                                            _ => {
                                                                if *(*__ctype_b_loc()).offset(
                                                                    b as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                    as ::core::ffi::c_int
                                                                    & _ISdigit as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ushort
                                                                        as ::core::ffi::c_int
                                                                    != 0
                                                                    || b as ::core::ffi::c_int
                                                                        == '-' as i32
                                                                {
                                                                    if new_value(
                                                                        &raw mut state,
                                                                        &raw mut top,
                                                                        &raw mut root,
                                                                        &raw mut alloc,
                                                                        json_integer,
                                                                    ) == 0
                                                                    {
                                                                        current_block =
                                                                            5120512961492157320;
                                                                        break 's_107;
                                                                    }
                                                                    if state.first_pass == 0 {
                                                                        while *(*__ctype_b_loc())
                                                                            .offset(b as ::core::ffi::c_int as isize)
                                                                            as ::core::ffi::c_int
                                                                            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                                                                as ::core::ffi::c_int != 0
                                                                            || b as ::core::ffi::c_int == '+' as i32
                                                                            || b as ::core::ffi::c_int == '-' as i32
                                                                            || b as ::core::ffi::c_int == 'e' as i32
                                                                            || b as ::core::ffi::c_int == 'E' as i32
                                                                            || b as ::core::ffi::c_int == '.' as i32
                                                                        {
                                                                            state.ptr = state.ptr.offset(1);
                                                                            if state.ptr == end {
                                                                                b = 0 as ::core::ffi::c_char;
                                                                                break;
                                                                            } else {
                                                                                b = *state.ptr;
                                                                            }
                                                                        }
                                                                        flags |=
                                                                            flag_next | flag_reproc;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        flags &= !(flag_num_negative
                                                                            | flag_num_e
                                                                            | flag_num_e_got_sign
                                                                            | flag_num_e_negative
                                                                            | flag_num_zero);
                                                                        num_digits = 0
                                                                            as ::core::ffi::c_long;
                                                                        num_fraction = 0 as i64;
                                                                        num_e = 0
                                                                            as ::core::ffi::c_long;
                                                                        if b as ::core::ffi::c_int
                                                                            != '-' as i32
                                                                        {
                                                                            flags |= flag_reproc;
                                                                            current_block = 11603475171617447446;
                                                                        } else {
                                                                            flags |=
                                                                                flag_num_negative;
                                                                            current_block = 11057878835866523405;
                                                                        }
                                                                    }
                                                                } else {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected %c when seeking value\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        32 | 9 | 13 => {
                                            current_block = 11057878835866523405;
                                        }
                                        93 => {
                                            current_block = 4996464336325100333;
                                            match current_block {
                                                15953825877604003206 => {
                                                    state.cur_line = state.cur_line.wrapping_add(1);
                                                    state.cur_col = 0 as ::core::ffi::c_uint;
                                                    current_block = 11057878835866523405;
                                                }
                                                4996464336325100333 => {
                                                    if !top.is_null()
                                                        && (*top).type_0 as ::core::ffi::c_uint
                                                            == json_array as ::core::ffi::c_int
                                                                as ::core::ffi::c_uint
                                                    {
                                                        flags = flags
                                                            & !(flag_need_comma | flag_seek_value)
                                                            | flag_next;
                                                    } else {
                                                        sprintf(
                                                            &raw mut error
                                                                as *mut ::core::ffi::c_char,
                                                            b"%d:%d: Unexpected ]\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            state.cur_line,
                                                            state.cur_col,
                                                        );
                                                        current_block = 14191169715820259248;
                                                        break 's_107;
                                                    }
                                                    current_block = 11603475171617447446;
                                                }
                                                _ => {
                                                    if flags & flag_need_comma != 0 {
                                                        if b as ::core::ffi::c_int == ',' as i32 {
                                                            flags &= !flag_need_comma;
                                                        } else {
                                                            sprintf(
                                                                &raw mut error
                                                                    as *mut ::core::ffi::c_char,
                                                                b"%d:%d: Expected , before %c\0"
                                                                    as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                state.cur_line,
                                                                state.cur_col,
                                                                b as ::core::ffi::c_int,
                                                            );
                                                            current_block = 14191169715820259248;
                                                            break 's_107;
                                                        }
                                                        current_block = 11057878835866523405;
                                                    } else if flags & flag_need_colon != 0 {
                                                        if b as ::core::ffi::c_int == ':' as i32 {
                                                            flags &= !flag_need_colon;
                                                        } else {
                                                            sprintf(
                                                                &raw mut error
                                                                    as *mut ::core::ffi::c_char,
                                                                b"%d:%d: Expected : before %c\0"
                                                                    as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                state.cur_line,
                                                                state.cur_col,
                                                                b as ::core::ffi::c_int,
                                                            );
                                                            current_block = 14191169715820259248;
                                                            break 's_107;
                                                        }
                                                        current_block = 11057878835866523405;
                                                    } else {
                                                        flags &= !flag_seek_value;
                                                        match b as ::core::ffi::c_int {
                                                            123 => {
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_object,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                current_block =
                                                                    11057878835866523405;
                                                            }
                                                            91 => {
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_array,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_seek_value;
                                                                current_block =
                                                                    11057878835866523405;
                                                            }
                                                            34 => {
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_string,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_string;
                                                                string = (*top).u.string.ptr;
                                                                string_length =
                                                                    0 as ::core::ffi::c_uint;
                                                                current_block =
                                                                    11057878835866523405;
                                                            }
                                                            116 => {
                                                                if (end.offset_from(state.ptr)
                                                                    as ::core::ffi::c_long)
                                                                    < 3 as ::core::ffi::c_long
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'r' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'u' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'e' as i32
                                                                    }
                                                                {
                                                                    current_block =
                                                                        11800234908647806265;
                                                                    break 's_107;
                                                                }
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_boolean,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                (*top).u.boolean =
                                                                    1 as ::core::ffi::c_int;
                                                                flags |= flag_next;
                                                                current_block =
                                                                    11603475171617447446;
                                                            }
                                                            102 => {
                                                                if (end.offset_from(state.ptr)
                                                                    as ::core::ffi::c_long)
                                                                    < 4 as ::core::ffi::c_long
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'a' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'l' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 's' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'e' as i32
                                                                    }
                                                                {
                                                                    current_block =
                                                                        11800234908647806265;
                                                                    break 's_107;
                                                                }
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_boolean,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_next;
                                                                current_block =
                                                                    11603475171617447446;
                                                            }
                                                            110 => {
                                                                if (end.offset_from(state.ptr)
                                                                    as ::core::ffi::c_long)
                                                                    < 3 as ::core::ffi::c_long
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'u' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'l' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'l' as i32
                                                                    }
                                                                {
                                                                    current_block =
                                                                        11800234908647806265;
                                                                    break 's_107;
                                                                }
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_null,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_next;
                                                                current_block =
                                                                    11603475171617447446;
                                                            }
                                                            _ => {
                                                                if *(*__ctype_b_loc()).offset(
                                                                    b as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                    as ::core::ffi::c_int
                                                                    & _ISdigit as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ushort
                                                                        as ::core::ffi::c_int
                                                                    != 0
                                                                    || b as ::core::ffi::c_int
                                                                        == '-' as i32
                                                                {
                                                                    if new_value(
                                                                        &raw mut state,
                                                                        &raw mut top,
                                                                        &raw mut root,
                                                                        &raw mut alloc,
                                                                        json_integer,
                                                                    ) == 0
                                                                    {
                                                                        current_block =
                                                                            5120512961492157320;
                                                                        break 's_107;
                                                                    }
                                                                    if state.first_pass == 0 {
                                                                        while *(*__ctype_b_loc())
                                                                            .offset(b as ::core::ffi::c_int as isize)
                                                                            as ::core::ffi::c_int
                                                                            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                                                                as ::core::ffi::c_int != 0
                                                                            || b as ::core::ffi::c_int == '+' as i32
                                                                            || b as ::core::ffi::c_int == '-' as i32
                                                                            || b as ::core::ffi::c_int == 'e' as i32
                                                                            || b as ::core::ffi::c_int == 'E' as i32
                                                                            || b as ::core::ffi::c_int == '.' as i32
                                                                        {
                                                                            state.ptr = state.ptr.offset(1);
                                                                            if state.ptr == end {
                                                                                b = 0 as ::core::ffi::c_char;
                                                                                break;
                                                                            } else {
                                                                                b = *state.ptr;
                                                                            }
                                                                        }
                                                                        flags |=
                                                                            flag_next | flag_reproc;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        flags &= !(flag_num_negative
                                                                            | flag_num_e
                                                                            | flag_num_e_got_sign
                                                                            | flag_num_e_negative
                                                                            | flag_num_zero);
                                                                        num_digits = 0
                                                                            as ::core::ffi::c_long;
                                                                        num_fraction = 0 as i64;
                                                                        num_e = 0
                                                                            as ::core::ffi::c_long;
                                                                        if b as ::core::ffi::c_int
                                                                            != '-' as i32
                                                                        {
                                                                            flags |= flag_reproc;
                                                                            current_block = 11603475171617447446;
                                                                        } else {
                                                                            flags |=
                                                                                flag_num_negative;
                                                                            current_block = 11057878835866523405;
                                                                        }
                                                                    }
                                                                } else {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected %c when seeking value\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            current_block = 1137006006685247392;
                                            match current_block {
                                                15953825877604003206 => {
                                                    state.cur_line = state.cur_line.wrapping_add(1);
                                                    state.cur_col = 0 as ::core::ffi::c_uint;
                                                    current_block = 11057878835866523405;
                                                }
                                                4996464336325100333 => {
                                                    if !top.is_null()
                                                        && (*top).type_0 as ::core::ffi::c_uint
                                                            == json_array as ::core::ffi::c_int
                                                                as ::core::ffi::c_uint
                                                    {
                                                        flags = flags
                                                            & !(flag_need_comma | flag_seek_value)
                                                            | flag_next;
                                                    } else {
                                                        sprintf(
                                                            &raw mut error
                                                                as *mut ::core::ffi::c_char,
                                                            b"%d:%d: Unexpected ]\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            state.cur_line,
                                                            state.cur_col,
                                                        );
                                                        current_block = 14191169715820259248;
                                                        break 's_107;
                                                    }
                                                    current_block = 11603475171617447446;
                                                }
                                                _ => {
                                                    if flags & flag_need_comma != 0 {
                                                        if b as ::core::ffi::c_int == ',' as i32 {
                                                            flags &= !flag_need_comma;
                                                        } else {
                                                            sprintf(
                                                                &raw mut error
                                                                    as *mut ::core::ffi::c_char,
                                                                b"%d:%d: Expected , before %c\0"
                                                                    as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                state.cur_line,
                                                                state.cur_col,
                                                                b as ::core::ffi::c_int,
                                                            );
                                                            current_block = 14191169715820259248;
                                                            break 's_107;
                                                        }
                                                        current_block = 11057878835866523405;
                                                    } else if flags & flag_need_colon != 0 {
                                                        if b as ::core::ffi::c_int == ':' as i32 {
                                                            flags &= !flag_need_colon;
                                                        } else {
                                                            sprintf(
                                                                &raw mut error
                                                                    as *mut ::core::ffi::c_char,
                                                                b"%d:%d: Expected : before %c\0"
                                                                    as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                state.cur_line,
                                                                state.cur_col,
                                                                b as ::core::ffi::c_int,
                                                            );
                                                            current_block = 14191169715820259248;
                                                            break 's_107;
                                                        }
                                                        current_block = 11057878835866523405;
                                                    } else {
                                                        flags &= !flag_seek_value;
                                                        match b as ::core::ffi::c_int {
                                                            123 => {
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_object,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                current_block =
                                                                    11057878835866523405;
                                                            }
                                                            91 => {
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_array,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_seek_value;
                                                                current_block =
                                                                    11057878835866523405;
                                                            }
                                                            34 => {
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_string,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_string;
                                                                string = (*top).u.string.ptr;
                                                                string_length =
                                                                    0 as ::core::ffi::c_uint;
                                                                current_block =
                                                                    11057878835866523405;
                                                            }
                                                            116 => {
                                                                if (end.offset_from(state.ptr)
                                                                    as ::core::ffi::c_long)
                                                                    < 3 as ::core::ffi::c_long
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'r' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'u' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'e' as i32
                                                                    }
                                                                {
                                                                    current_block =
                                                                        11800234908647806265;
                                                                    break 's_107;
                                                                }
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_boolean,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                (*top).u.boolean =
                                                                    1 as ::core::ffi::c_int;
                                                                flags |= flag_next;
                                                                current_block =
                                                                    11603475171617447446;
                                                            }
                                                            102 => {
                                                                if (end.offset_from(state.ptr)
                                                                    as ::core::ffi::c_long)
                                                                    < 4 as ::core::ffi::c_long
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'a' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'l' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 's' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'e' as i32
                                                                    }
                                                                {
                                                                    current_block =
                                                                        11800234908647806265;
                                                                    break 's_107;
                                                                }
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_boolean,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_next;
                                                                current_block =
                                                                    11603475171617447446;
                                                            }
                                                            110 => {
                                                                if (end.offset_from(state.ptr)
                                                                    as ::core::ffi::c_long)
                                                                    < 3 as ::core::ffi::c_long
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'u' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'l' as i32
                                                                    }
                                                                    || {
                                                                        state.ptr =
                                                                            state.ptr.offset(1);
                                                                        *state.ptr
                                                                            as ::core::ffi::c_int
                                                                            != 'l' as i32
                                                                    }
                                                                {
                                                                    current_block =
                                                                        11800234908647806265;
                                                                    break 's_107;
                                                                }
                                                                if new_value(
                                                                    &raw mut state,
                                                                    &raw mut top,
                                                                    &raw mut root,
                                                                    &raw mut alloc,
                                                                    json_null,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        5120512961492157320;
                                                                    break 's_107;
                                                                }
                                                                flags |= flag_next;
                                                                current_block =
                                                                    11603475171617447446;
                                                            }
                                                            _ => {
                                                                if *(*__ctype_b_loc()).offset(
                                                                    b as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                    as ::core::ffi::c_int
                                                                    & _ISdigit as ::core::ffi::c_int
                                                                        as ::core::ffi::c_ushort
                                                                        as ::core::ffi::c_int
                                                                    != 0
                                                                    || b as ::core::ffi::c_int
                                                                        == '-' as i32
                                                                {
                                                                    if new_value(
                                                                        &raw mut state,
                                                                        &raw mut top,
                                                                        &raw mut root,
                                                                        &raw mut alloc,
                                                                        json_integer,
                                                                    ) == 0
                                                                    {
                                                                        current_block =
                                                                            5120512961492157320;
                                                                        break 's_107;
                                                                    }
                                                                    if state.first_pass == 0 {
                                                                        while *(*__ctype_b_loc())
                                                                            .offset(b as ::core::ffi::c_int as isize)
                                                                            as ::core::ffi::c_int
                                                                            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                                                                as ::core::ffi::c_int != 0
                                                                            || b as ::core::ffi::c_int == '+' as i32
                                                                            || b as ::core::ffi::c_int == '-' as i32
                                                                            || b as ::core::ffi::c_int == 'e' as i32
                                                                            || b as ::core::ffi::c_int == 'E' as i32
                                                                            || b as ::core::ffi::c_int == '.' as i32
                                                                        {
                                                                            state.ptr = state.ptr.offset(1);
                                                                            if state.ptr == end {
                                                                                b = 0 as ::core::ffi::c_char;
                                                                                break;
                                                                            } else {
                                                                                b = *state.ptr;
                                                                            }
                                                                        }
                                                                        flags |=
                                                                            flag_next | flag_reproc;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        flags &= !(flag_num_negative
                                                                            | flag_num_e
                                                                            | flag_num_e_got_sign
                                                                            | flag_num_e_negative
                                                                            | flag_num_zero);
                                                                        num_digits = 0
                                                                            as ::core::ffi::c_long;
                                                                        num_fraction = 0 as i64;
                                                                        num_e = 0
                                                                            as ::core::ffi::c_long;
                                                                        if b as ::core::ffi::c_int
                                                                            != '-' as i32
                                                                        {
                                                                            flags |= flag_reproc;
                                                                            current_block = 11603475171617447446;
                                                                        } else {
                                                                            flags |=
                                                                                flag_num_negative;
                                                                            current_block = 11057878835866523405;
                                                                        }
                                                                    }
                                                                } else {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected %c when seeking value\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    match (*top).type_0 as ::core::ffi::c_uint {
                                        1 => {
                                            current_block = 13760369805207408080;
                                            match current_block {
                                                13760369805207408080 => {
                                                    match b as ::core::ffi::c_int {
                                                        10 => {
                                                            current_block = 17318500399378829025;
                                                            match current_block {
                                                                10459093242147315661 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        flags &= !flag_need_comma;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        current_block =
                                                                            5906948055639026220;
                                                                    }
                                                                }
                                                                14348858175135860202 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected , before \"\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        flags |= flag_string;
                                                                        string = (*top)._reserved.object_mem
                                                                            as *mut ::core::ffi::c_char;
                                                                        string_length = 0
                                                                            as ::core::ffi::c_uint;
                                                                    }
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                17318500399378829025 => {
                                                                    state.cur_line = state
                                                                        .cur_line
                                                                        .wrapping_add(1);
                                                                    state.cur_col =
                                                                        0 as ::core::ffi::c_uint;
                                                                    current_block =
                                                                        11057878835866523405;
                                                                }
                                                                7153118028659730796 => {
                                                                    flags = flags
                                                                        & !flag_need_comma
                                                                        | flag_next;
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                _ => {}
                                                            }
                                                            match current_block {
                                                                11603475171617447446 => {}
                                                                11057878835866523405 => {}
                                                                _ => {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `%c` in object\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                        32 | 9 | 13 => {
                                                            current_block = 11057878835866523405;
                                                        }
                                                        34 => {
                                                            current_block = 14348858175135860202;
                                                            match current_block {
                                                                10459093242147315661 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        flags &= !flag_need_comma;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        current_block =
                                                                            5906948055639026220;
                                                                    }
                                                                }
                                                                14348858175135860202 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected , before \"\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        flags |= flag_string;
                                                                        string = (*top)._reserved.object_mem
                                                                            as *mut ::core::ffi::c_char;
                                                                        string_length = 0
                                                                            as ::core::ffi::c_uint;
                                                                    }
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                17318500399378829025 => {
                                                                    state.cur_line = state
                                                                        .cur_line
                                                                        .wrapping_add(1);
                                                                    state.cur_col =
                                                                        0 as ::core::ffi::c_uint;
                                                                    current_block =
                                                                        11057878835866523405;
                                                                }
                                                                7153118028659730796 => {
                                                                    flags = flags
                                                                        & !flag_need_comma
                                                                        | flag_next;
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                _ => {}
                                                            }
                                                            match current_block {
                                                                11603475171617447446 => {}
                                                                11057878835866523405 => {}
                                                                _ => {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `%c` in object\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                        125 => {
                                                            current_block = 7153118028659730796;
                                                            match current_block {
                                                                10459093242147315661 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        flags &= !flag_need_comma;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        current_block =
                                                                            5906948055639026220;
                                                                    }
                                                                }
                                                                14348858175135860202 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected , before \"\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        flags |= flag_string;
                                                                        string = (*top)._reserved.object_mem
                                                                            as *mut ::core::ffi::c_char;
                                                                        string_length = 0
                                                                            as ::core::ffi::c_uint;
                                                                    }
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                17318500399378829025 => {
                                                                    state.cur_line = state
                                                                        .cur_line
                                                                        .wrapping_add(1);
                                                                    state.cur_col =
                                                                        0 as ::core::ffi::c_uint;
                                                                    current_block =
                                                                        11057878835866523405;
                                                                }
                                                                7153118028659730796 => {
                                                                    flags = flags
                                                                        & !flag_need_comma
                                                                        | flag_next;
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                _ => {}
                                                            }
                                                            match current_block {
                                                                11603475171617447446 => {}
                                                                11057878835866523405 => {}
                                                                _ => {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `%c` in object\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                        44 => {
                                                            current_block = 10459093242147315661;
                                                            match current_block {
                                                                10459093242147315661 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        flags &= !flag_need_comma;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        current_block =
                                                                            5906948055639026220;
                                                                    }
                                                                }
                                                                14348858175135860202 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected , before \"\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        flags |= flag_string;
                                                                        string = (*top)._reserved.object_mem
                                                                            as *mut ::core::ffi::c_char;
                                                                        string_length = 0
                                                                            as ::core::ffi::c_uint;
                                                                    }
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                17318500399378829025 => {
                                                                    state.cur_line = state
                                                                        .cur_line
                                                                        .wrapping_add(1);
                                                                    state.cur_col =
                                                                        0 as ::core::ffi::c_uint;
                                                                    current_block =
                                                                        11057878835866523405;
                                                                }
                                                                7153118028659730796 => {
                                                                    flags = flags
                                                                        & !flag_need_comma
                                                                        | flag_next;
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                _ => {}
                                                            }
                                                            match current_block {
                                                                11603475171617447446 => {}
                                                                11057878835866523405 => {}
                                                                _ => {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `%c` in object\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            current_block = 5906948055639026220;
                                                            match current_block {
                                                                10459093242147315661 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        flags &= !flag_need_comma;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        current_block =
                                                                            5906948055639026220;
                                                                    }
                                                                }
                                                                14348858175135860202 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected , before \"\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        flags |= flag_string;
                                                                        string = (*top)._reserved.object_mem
                                                                            as *mut ::core::ffi::c_char;
                                                                        string_length = 0
                                                                            as ::core::ffi::c_uint;
                                                                    }
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                17318500399378829025 => {
                                                                    state.cur_line = state
                                                                        .cur_line
                                                                        .wrapping_add(1);
                                                                    state.cur_col =
                                                                        0 as ::core::ffi::c_uint;
                                                                    current_block =
                                                                        11057878835866523405;
                                                                }
                                                                7153118028659730796 => {
                                                                    flags = flags
                                                                        & !flag_need_comma
                                                                        | flag_next;
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                _ => {}
                                                            }
                                                            match current_block {
                                                                11603475171617447446 => {}
                                                                11057878835866523405 => {}
                                                                _ => {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `%c` in object\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    if *(*__ctype_b_loc())
                                                        .offset(b as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & _ISdigit as ::core::ffi::c_int
                                                            as ::core::ffi::c_ushort
                                                            as ::core::ffi::c_int
                                                        != 0
                                                    {
                                                        num_digits += 1;
                                                        if (*top).type_0 as ::core::ffi::c_uint
                                                            == json_integer as ::core::ffi::c_int
                                                                as ::core::ffi::c_uint
                                                            || flags & flag_num_e != 0
                                                        {
                                                            if flags & flag_num_e == 0 {
                                                                if flags & flag_num_zero != 0 {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `0` before `%c`\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                } else {
                                                                    if num_digits
                                                                        == 1 as ::core::ffi::c_long
                                                                        && b as ::core::ffi::c_int
                                                                            == '0' as i32
                                                                    {
                                                                        flags |= flag_num_zero;
                                                                    }
                                                                    (*top).u.integer = (*top)
                                                                        .u
                                                                        .integer
                                                                        * 10 as i64
                                                                        + (b as ::core::ffi::c_int
                                                                            - '0' as i32)
                                                                            as i64;
                                                                }
                                                            } else {
                                                                flags |= flag_num_e_got_sign;
                                                                num_e = num_e
                                                                    * 10 as ::core::ffi::c_long
                                                                    + (b as ::core::ffi::c_int
                                                                        - '0' as i32)
                                                                        as ::core::ffi::c_long;
                                                            }
                                                        } else {
                                                            num_fraction = num_fraction
                                                                * 10 as i64
                                                                + (b as ::core::ffi::c_int
                                                                    - '0' as i32)
                                                                    as i64;
                                                        }
                                                        current_block = 11057878835866523405;
                                                    } else {
                                                        if b as ::core::ffi::c_int == '+' as i32
                                                            || b as ::core::ffi::c_int == '-' as i32
                                                        {
                                                            if flags & flag_num_e != 0
                                                                && flags & flag_num_e_got_sign == 0
                                                            {
                                                                flags |= flag_num_e_got_sign;
                                                                if b as ::core::ffi::c_int
                                                                    == '-' as i32
                                                                {
                                                                    flags |= flag_num_e_negative;
                                                                }
                                                                current_block =
                                                                    11057878835866523405;
                                                            } else {
                                                                current_block =
                                                                    13139313344577584540;
                                                            }
                                                        } else if b as ::core::ffi::c_int
                                                            == '.' as i32
                                                            && (*top).type_0 as ::core::ffi::c_uint
                                                                == json_integer
                                                                    as ::core::ffi::c_int
                                                                    as ::core::ffi::c_uint
                                                        {
                                                            if num_digits == 0 {
                                                                sprintf(
                                                                    &raw mut error as *mut ::core::ffi::c_char,
                                                                    b"%d:%d: Expected digit before `.`\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                    state.cur_line,
                                                                    state.cur_col,
                                                                );
                                                                current_block =
                                                                    14191169715820259248;
                                                                break 's_107;
                                                            } else {
                                                                (*top).type_0 = json_double;
                                                                (*top).u.dbl = (*top).u.integer
                                                                    as ::core::ffi::c_double;
                                                                num_digits =
                                                                    0 as ::core::ffi::c_long;
                                                            }
                                                            current_block = 11057878835866523405;
                                                        } else {
                                                            current_block = 13139313344577584540;
                                                        }
                                                        match current_block {
                                                            11057878835866523405 => {}
                                                            _ => {
                                                                if flags & flag_num_e == 0 {
                                                                    if (*top).type_0
                                                                        as ::core::ffi::c_uint
                                                                        == json_double
                                                                            as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint
                                                                    {
                                                                        if num_digits == 0 {
                                                                            sprintf(
                                                                                &raw mut error as *mut ::core::ffi::c_char,
                                                                                b"%d:%d: Expected digit after `.`\0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                                state.cur_line,
                                                                                state.cur_col,
                                                                            );
                                                                            current_block = 14191169715820259248;
                                                                            break 's_107;
                                                                        } else {
                                                                            (*top).u.dbl
                                                                                += num_fraction as ::core::ffi::c_double
                                                                                    / pow(10.0f64, num_digits as ::core::ffi::c_double);
                                                                        }
                                                                    }
                                                                    if b as ::core::ffi::c_int
                                                                        == 'e' as i32
                                                                        || b as ::core::ffi::c_int
                                                                            == 'E' as i32
                                                                    {
                                                                        flags |= flag_num_e;
                                                                        if (*top).type_0 as ::core::ffi::c_uint
                                                                            == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                        {
                                                                            (*top).type_0 = json_double;
                                                                            (*top).u.dbl = (*top).u.integer as ::core::ffi::c_double;
                                                                        }
                                                                        num_digits = 0
                                                                            as ::core::ffi::c_long;
                                                                        flags &= !flag_num_zero;
                                                                        current_block =
                                                                            11057878835866523405;
                                                                    } else {
                                                                        current_block =
                                                                            6648503596396917841;
                                                                    }
                                                                } else {
                                                                    if num_digits == 0 {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected digit after `e`\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        (*top).u.dbl
                                                                            *= pow(
                                                                                10.0f64,
                                                                                (if flags & flag_num_e_negative != 0 {
                                                                                    -num_e
                                                                                } else {
                                                                                    num_e
                                                                                }) as ::core::ffi::c_double,
                                                                            );
                                                                    }
                                                                    current_block =
                                                                        6648503596396917841;
                                                                }
                                                                match current_block {
                                                                    11057878835866523405 => {}
                                                                    _ => {
                                                                        if flags & flag_num_negative
                                                                            != 0
                                                                        {
                                                                            if (*top).type_0 as ::core::ffi::c_uint
                                                                                == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                            {
                                                                                (*top).u.integer = -(*top).u.integer;
                                                                            } else {
                                                                                (*top).u.dbl = -(*top).u.dbl;
                                                                            }
                                                                        }
                                                                        flags |=
                                                                            flag_next | flag_reproc;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        3 | 4 => {
                                            current_block = 16809337807815302285;
                                            match current_block {
                                                13760369805207408080 => {
                                                    match b as ::core::ffi::c_int {
                                                        10 => {
                                                            current_block = 17318500399378829025;
                                                            match current_block {
                                                                10459093242147315661 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        flags &= !flag_need_comma;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        current_block =
                                                                            5906948055639026220;
                                                                    }
                                                                }
                                                                14348858175135860202 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected , before \"\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        flags |= flag_string;
                                                                        string = (*top)._reserved.object_mem
                                                                            as *mut ::core::ffi::c_char;
                                                                        string_length = 0
                                                                            as ::core::ffi::c_uint;
                                                                    }
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                17318500399378829025 => {
                                                                    state.cur_line = state
                                                                        .cur_line
                                                                        .wrapping_add(1);
                                                                    state.cur_col =
                                                                        0 as ::core::ffi::c_uint;
                                                                    current_block =
                                                                        11057878835866523405;
                                                                }
                                                                7153118028659730796 => {
                                                                    flags = flags
                                                                        & !flag_need_comma
                                                                        | flag_next;
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                _ => {}
                                                            }
                                                            match current_block {
                                                                11603475171617447446 => {}
                                                                11057878835866523405 => {}
                                                                _ => {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `%c` in object\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                        32 | 9 | 13 => {
                                                            current_block = 11057878835866523405;
                                                        }
                                                        34 => {
                                                            current_block = 14348858175135860202;
                                                            match current_block {
                                                                10459093242147315661 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        flags &= !flag_need_comma;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        current_block =
                                                                            5906948055639026220;
                                                                    }
                                                                }
                                                                14348858175135860202 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected , before \"\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        flags |= flag_string;
                                                                        string = (*top)._reserved.object_mem
                                                                            as *mut ::core::ffi::c_char;
                                                                        string_length = 0
                                                                            as ::core::ffi::c_uint;
                                                                    }
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                17318500399378829025 => {
                                                                    state.cur_line = state
                                                                        .cur_line
                                                                        .wrapping_add(1);
                                                                    state.cur_col =
                                                                        0 as ::core::ffi::c_uint;
                                                                    current_block =
                                                                        11057878835866523405;
                                                                }
                                                                7153118028659730796 => {
                                                                    flags = flags
                                                                        & !flag_need_comma
                                                                        | flag_next;
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                _ => {}
                                                            }
                                                            match current_block {
                                                                11603475171617447446 => {}
                                                                11057878835866523405 => {}
                                                                _ => {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `%c` in object\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                        125 => {
                                                            current_block = 7153118028659730796;
                                                            match current_block {
                                                                10459093242147315661 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        flags &= !flag_need_comma;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        current_block =
                                                                            5906948055639026220;
                                                                    }
                                                                }
                                                                14348858175135860202 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected , before \"\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        flags |= flag_string;
                                                                        string = (*top)._reserved.object_mem
                                                                            as *mut ::core::ffi::c_char;
                                                                        string_length = 0
                                                                            as ::core::ffi::c_uint;
                                                                    }
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                17318500399378829025 => {
                                                                    state.cur_line = state
                                                                        .cur_line
                                                                        .wrapping_add(1);
                                                                    state.cur_col =
                                                                        0 as ::core::ffi::c_uint;
                                                                    current_block =
                                                                        11057878835866523405;
                                                                }
                                                                7153118028659730796 => {
                                                                    flags = flags
                                                                        & !flag_need_comma
                                                                        | flag_next;
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                _ => {}
                                                            }
                                                            match current_block {
                                                                11603475171617447446 => {}
                                                                11057878835866523405 => {}
                                                                _ => {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `%c` in object\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                        44 => {
                                                            current_block = 10459093242147315661;
                                                            match current_block {
                                                                10459093242147315661 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        flags &= !flag_need_comma;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        current_block =
                                                                            5906948055639026220;
                                                                    }
                                                                }
                                                                14348858175135860202 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected , before \"\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        flags |= flag_string;
                                                                        string = (*top)._reserved.object_mem
                                                                            as *mut ::core::ffi::c_char;
                                                                        string_length = 0
                                                                            as ::core::ffi::c_uint;
                                                                    }
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                17318500399378829025 => {
                                                                    state.cur_line = state
                                                                        .cur_line
                                                                        .wrapping_add(1);
                                                                    state.cur_col =
                                                                        0 as ::core::ffi::c_uint;
                                                                    current_block =
                                                                        11057878835866523405;
                                                                }
                                                                7153118028659730796 => {
                                                                    flags = flags
                                                                        & !flag_need_comma
                                                                        | flag_next;
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                _ => {}
                                                            }
                                                            match current_block {
                                                                11603475171617447446 => {}
                                                                11057878835866523405 => {}
                                                                _ => {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `%c` in object\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            current_block = 5906948055639026220;
                                                            match current_block {
                                                                10459093242147315661 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        flags &= !flag_need_comma;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    } else {
                                                                        current_block =
                                                                            5906948055639026220;
                                                                    }
                                                                }
                                                                14348858175135860202 => {
                                                                    if flags & flag_need_comma != 0
                                                                    {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected , before \"\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        flags |= flag_string;
                                                                        string = (*top)._reserved.object_mem
                                                                            as *mut ::core::ffi::c_char;
                                                                        string_length = 0
                                                                            as ::core::ffi::c_uint;
                                                                    }
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                17318500399378829025 => {
                                                                    state.cur_line = state
                                                                        .cur_line
                                                                        .wrapping_add(1);
                                                                    state.cur_col =
                                                                        0 as ::core::ffi::c_uint;
                                                                    current_block =
                                                                        11057878835866523405;
                                                                }
                                                                7153118028659730796 => {
                                                                    flags = flags
                                                                        & !flag_need_comma
                                                                        | flag_next;
                                                                    current_block =
                                                                        11603475171617447446;
                                                                }
                                                                _ => {}
                                                            }
                                                            match current_block {
                                                                11603475171617447446 => {}
                                                                11057878835866523405 => {}
                                                                _ => {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `%c` in object\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    if *(*__ctype_b_loc())
                                                        .offset(b as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & _ISdigit as ::core::ffi::c_int
                                                            as ::core::ffi::c_ushort
                                                            as ::core::ffi::c_int
                                                        != 0
                                                    {
                                                        num_digits += 1;
                                                        if (*top).type_0 as ::core::ffi::c_uint
                                                            == json_integer as ::core::ffi::c_int
                                                                as ::core::ffi::c_uint
                                                            || flags & flag_num_e != 0
                                                        {
                                                            if flags & flag_num_e == 0 {
                                                                if flags & flag_num_zero != 0 {
                                                                    sprintf(
                                                                        &raw mut error as *mut ::core::ffi::c_char,
                                                                        b"%d:%d: Unexpected `0` before `%c`\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                        state.cur_line,
                                                                        state.cur_col,
                                                                        b as ::core::ffi::c_int,
                                                                    );
                                                                    current_block =
                                                                        14191169715820259248;
                                                                    break 's_107;
                                                                } else {
                                                                    if num_digits
                                                                        == 1 as ::core::ffi::c_long
                                                                        && b as ::core::ffi::c_int
                                                                            == '0' as i32
                                                                    {
                                                                        flags |= flag_num_zero;
                                                                    }
                                                                    (*top).u.integer = (*top)
                                                                        .u
                                                                        .integer
                                                                        * 10 as i64
                                                                        + (b as ::core::ffi::c_int
                                                                            - '0' as i32)
                                                                            as i64;
                                                                }
                                                            } else {
                                                                flags |= flag_num_e_got_sign;
                                                                num_e = num_e
                                                                    * 10 as ::core::ffi::c_long
                                                                    + (b as ::core::ffi::c_int
                                                                        - '0' as i32)
                                                                        as ::core::ffi::c_long;
                                                            }
                                                        } else {
                                                            num_fraction = num_fraction
                                                                * 10 as i64
                                                                + (b as ::core::ffi::c_int
                                                                    - '0' as i32)
                                                                    as i64;
                                                        }
                                                        current_block = 11057878835866523405;
                                                    } else {
                                                        if b as ::core::ffi::c_int == '+' as i32
                                                            || b as ::core::ffi::c_int == '-' as i32
                                                        {
                                                            if flags & flag_num_e != 0
                                                                && flags & flag_num_e_got_sign == 0
                                                            {
                                                                flags |= flag_num_e_got_sign;
                                                                if b as ::core::ffi::c_int
                                                                    == '-' as i32
                                                                {
                                                                    flags |= flag_num_e_negative;
                                                                }
                                                                current_block =
                                                                    11057878835866523405;
                                                            } else {
                                                                current_block =
                                                                    13139313344577584540;
                                                            }
                                                        } else if b as ::core::ffi::c_int
                                                            == '.' as i32
                                                            && (*top).type_0 as ::core::ffi::c_uint
                                                                == json_integer
                                                                    as ::core::ffi::c_int
                                                                    as ::core::ffi::c_uint
                                                        {
                                                            if num_digits == 0 {
                                                                sprintf(
                                                                    &raw mut error as *mut ::core::ffi::c_char,
                                                                    b"%d:%d: Expected digit before `.`\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                    state.cur_line,
                                                                    state.cur_col,
                                                                );
                                                                current_block =
                                                                    14191169715820259248;
                                                                break 's_107;
                                                            } else {
                                                                (*top).type_0 = json_double;
                                                                (*top).u.dbl = (*top).u.integer
                                                                    as ::core::ffi::c_double;
                                                                num_digits =
                                                                    0 as ::core::ffi::c_long;
                                                            }
                                                            current_block = 11057878835866523405;
                                                        } else {
                                                            current_block = 13139313344577584540;
                                                        }
                                                        match current_block {
                                                            11057878835866523405 => {}
                                                            _ => {
                                                                if flags & flag_num_e == 0 {
                                                                    if (*top).type_0
                                                                        as ::core::ffi::c_uint
                                                                        == json_double
                                                                            as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint
                                                                    {
                                                                        if num_digits == 0 {
                                                                            sprintf(
                                                                                &raw mut error as *mut ::core::ffi::c_char,
                                                                                b"%d:%d: Expected digit after `.`\0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                                state.cur_line,
                                                                                state.cur_col,
                                                                            );
                                                                            current_block = 14191169715820259248;
                                                                            break 's_107;
                                                                        } else {
                                                                            (*top).u.dbl
                                                                                += num_fraction as ::core::ffi::c_double
                                                                                    / pow(10.0f64, num_digits as ::core::ffi::c_double);
                                                                        }
                                                                    }
                                                                    if b as ::core::ffi::c_int
                                                                        == 'e' as i32
                                                                        || b as ::core::ffi::c_int
                                                                            == 'E' as i32
                                                                    {
                                                                        flags |= flag_num_e;
                                                                        if (*top).type_0 as ::core::ffi::c_uint
                                                                            == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                        {
                                                                            (*top).type_0 = json_double;
                                                                            (*top).u.dbl = (*top).u.integer as ::core::ffi::c_double;
                                                                        }
                                                                        num_digits = 0
                                                                            as ::core::ffi::c_long;
                                                                        flags &= !flag_num_zero;
                                                                        current_block =
                                                                            11057878835866523405;
                                                                    } else {
                                                                        current_block =
                                                                            6648503596396917841;
                                                                    }
                                                                } else {
                                                                    if num_digits == 0 {
                                                                        sprintf(
                                                                            &raw mut error as *mut ::core::ffi::c_char,
                                                                            b"%d:%d: Expected digit after `e`\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            state.cur_line,
                                                                            state.cur_col,
                                                                        );
                                                                        current_block =
                                                                            14191169715820259248;
                                                                        break 's_107;
                                                                    } else {
                                                                        (*top).u.dbl
                                                                            *= pow(
                                                                                10.0f64,
                                                                                (if flags & flag_num_e_negative != 0 {
                                                                                    -num_e
                                                                                } else {
                                                                                    num_e
                                                                                }) as ::core::ffi::c_double,
                                                                            );
                                                                    }
                                                                    current_block =
                                                                        6648503596396917841;
                                                                }
                                                                match current_block {
                                                                    11057878835866523405 => {}
                                                                    _ => {
                                                                        if flags & flag_num_negative
                                                                            != 0
                                                                        {
                                                                            if (*top).type_0 as ::core::ffi::c_uint
                                                                                == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                            {
                                                                                (*top).u.integer = -(*top).u.integer;
                                                                            } else {
                                                                                (*top).u.dbl = -(*top).u.dbl;
                                                                            }
                                                                        }
                                                                        flags |=
                                                                            flag_next | flag_reproc;
                                                                        current_block =
                                                                            11603475171617447446;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            current_block = 11603475171617447446;
                                        }
                                    }
                                }
                                match current_block {
                                    11057878835866523405 => {}
                                    _ => {
                                        if flags & flag_reproc != 0 {
                                            flags &= !flag_reproc;
                                            state.ptr = state.ptr.offset(-1);
                                        }
                                        if flags & flag_next != 0 {
                                            flags = flags & !flag_next | flag_need_comma;
                                            if (*top).parent.is_null() {
                                                flags |= flag_done;
                                            } else {
                                                if (*(*top).parent).type_0 as ::core::ffi::c_uint
                                                    == json_array as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                {
                                                    flags |= flag_seek_value;
                                                }
                                                if state.first_pass == 0 {
                                                    let mut parent: *mut json_value =
                                                        (*top).parent as *mut json_value;
                                                    match (*parent).type_0 as ::core::ffi::c_uint {
                                                        1 => {
                                                            let ref mut fresh12 = (*(*parent)
                                                                .u
                                                                .object
                                                                .values
                                                                .offset(
                                                                    (*parent).u.object.length
                                                                        as isize,
                                                                ))
                                                            .value;
                                                            *fresh12 = top as *mut _json_value;
                                                        }
                                                        2 => {
                                                            let ref mut fresh13 =
                                                                *(*parent).u.array.values.offset(
                                                                    (*parent).u.array.length
                                                                        as isize,
                                                                );
                                                            *fresh13 = top as *mut _json_value;
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                (*(*top).parent).u.array.length =
                                                    (*(*top).parent).u.array.length.wrapping_add(1);
                                                if (*(*top).parent).u.array.length > state.uint_max
                                                {
                                                    current_block = 2680027254923815990;
                                                    break 's_107;
                                                }
                                                top = (*top).parent as *mut json_value;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            state.ptr = state.ptr.offset(1);
        }
        alloc = root;
        state.first_pass -= 1;
    }
    match current_block {
        2680027254923815990 => {
            sprintf(
                &raw mut error as *mut ::core::ffi::c_char,
                b"%d:%d: Too long (caught overflow)\0" as *const u8 as *const ::core::ffi::c_char,
                state.cur_line,
                state.cur_col,
            );
        }
        11800234908647806265 => {
            sprintf(
                &raw mut error as *mut ::core::ffi::c_char,
                b"%d:%d: Unknown value\0" as *const u8 as *const ::core::ffi::c_char,
                state.cur_line,
                state.cur_col,
            );
        }
        14895111670561345133 => return root,
        5120512961492157320 => {
            strcpy(
                &raw mut error as *mut ::core::ffi::c_char,
                b"Memory allocation failure\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        _ => {}
    }
    if !error_buf.is_null() {
        if *(&raw mut error as *mut ::core::ffi::c_char) != 0 {
            strcpy(error_buf, &raw mut error as *mut ::core::ffi::c_char);
        } else {
            strcpy(
                error_buf,
                b"Unknown error\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if state.first_pass != 0 {
        alloc = root;
    }
    while !alloc.is_null() {
        top = (*alloc)._reserved.next_alloc as *mut json_value;
        state.settings.mem_free.expect("non-null function pointer")(
            alloc as *mut ::core::ffi::c_void,
            state.settings.user_data,
        );
        alloc = top;
    }
    if state.first_pass == 0 {
        json_value_free_ex(&raw mut state.settings, root);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[no_mangle]
pub unsafe extern "C" fn json_parse(
    mut json: *const ::core::ffi::c_char,
    mut length: usize,
) -> *mut json_value {
    let mut settings: json_settings = json_settings {
        max_memory: 0,
        settings: 0,
        mem_alloc: None,
        mem_free: None,
        user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        value_extra: 0,
    };
    settings.max_memory = 0 as ::core::ffi::c_ulong;
    settings.settings = 0 as ::core::ffi::c_int;
    settings.mem_alloc = None;
    settings.mem_free = None;
    settings.user_data = NULL;
    settings.value_extra = 0 as usize;
    return json_parse_ex(
        &raw mut settings,
        json,
        length,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn json_value_free_ex(
    mut settings: *mut json_settings,
    mut value: *mut json_value,
) {
    let mut cur_value: *mut json_value = ::core::ptr::null_mut::<json_value>();
    if value.is_null() {
        return;
    }
    (*value).parent = ::core::ptr::null_mut::<_json_value>();
    while !value.is_null() {
        match (*value).type_0 as ::core::ffi::c_uint {
            2 => {
                if (*value).u.array.length == 0 {
                    (*settings).mem_free.expect("non-null function pointer")(
                        (*value).u.array.values as *mut ::core::ffi::c_void,
                        (*settings).user_data,
                    );
                } else {
                    (*value).u.array.length = (*value).u.array.length.wrapping_sub(1);
                    value = *(*value)
                        .u
                        .array
                        .values
                        .offset((*value).u.array.length as isize)
                        as *mut json_value;
                    continue;
                }
            }
            1 => {
                if (*value).u.object.length == 0 {
                    (*settings).mem_free.expect("non-null function pointer")(
                        (*value).u.object.values as *mut ::core::ffi::c_void,
                        (*settings).user_data,
                    );
                } else {
                    (*value).u.object.length = (*value).u.object.length.wrapping_sub(1);
                    value = (*(*value)
                        .u
                        .object
                        .values
                        .offset((*value).u.object.length as isize))
                    .value as *mut json_value;
                    continue;
                }
            }
            5 | 8 => {
                (*settings).mem_free.expect("non-null function pointer")(
                    (*value).u.string.ptr as *mut ::core::ffi::c_void,
                    (*settings).user_data,
                );
            }
            _ => {}
        }
        cur_value = value;
        value = (*value).parent as *mut json_value;
        (*settings).mem_free.expect("non-null function pointer")(
            cur_value as *mut ::core::ffi::c_void,
            (*settings).user_data,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn json_value_free(mut value: *mut json_value) {
    let mut settings: json_settings = json_settings {
        max_memory: 0,
        settings: 0,
        mem_alloc: None,
        mem_free: None,
        user_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        value_extra: 0,
    };
    settings.max_memory = 0 as ::core::ffi::c_ulong;
    settings.settings = 0 as ::core::ffi::c_int;
    settings.mem_alloc = None;
    settings.mem_free = None;
    settings.user_data = NULL;
    settings.value_extra = 0 as usize;
    settings.mem_free = Some(
        default_free
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>;
    json_value_free_ex(&raw mut settings, value);
}
