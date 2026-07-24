extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn emyg_dtoa(value: ::core::ffi::c_double, buffer: *mut ::core::ffi::c_char);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type size_t = usize;
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
    pub integer: int64_t,
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
pub struct json_builder_value {
    pub value: json_value,
    pub is_builder_value: ::core::ffi::c_int,
    pub additional_length_allocated: size_t,
    pub length_iterated: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_serialize_opts {
    pub mode: ::core::ffi::c_int,
    pub opts: ::core::ffi::c_int,
    pub indent_size: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const json_serialize_mode_multiline: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const json_serialize_mode_single_line: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const json_serialize_mode_packed: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const json_serialize_opt_CRLF: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const json_serialize_opt_pack_brackets: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const json_serialize_opt_no_space_after_comma: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const json_serialize_opt_no_space_after_colon: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const json_serialize_opt_use_tabs: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
static mut default_opts: json_serialize_opts = json_serialize_opts {
    mode: json_serialize_mode_single_line,
    opts: 0 as ::core::ffi::c_int,
    indent_size: 3 as ::core::ffi::c_int,
};
unsafe extern "C" fn builderize(mut value: *mut json_value) -> ::core::ffi::c_int {
    if (*(value as *mut json_builder_value)).is_builder_value != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if (*value).type_0 as ::core::ffi::c_uint
        == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < (*value).u.object.length {
            let mut name_copy: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut entry: *mut json_object_entry =
                (*value).u.object.values.offset(i as isize) as *mut json_object_entry;
            name_copy = malloc(
                ((*entry).name_length.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            ) as *mut ::core::ffi::c_char;
            if name_copy.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            memcpy(
                name_copy as *mut ::core::ffi::c_void,
                (*entry).name as *const ::core::ffi::c_void,
                (*entry).name_length.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
            );
            (*entry).name = name_copy;
            i = i.wrapping_add(1);
        }
    }
    (*(value as *mut json_builder_value)).is_builder_value = 1 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut json_builder_extra: size_t = 0;
#[no_mangle]
pub static mut f_spaces_around_brackets: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut f_spaces_after_commas: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut f_spaces_after_colons: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
#[no_mangle]
pub static mut f_tabs: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn get_serialize_flags(mut opts: json_serialize_opts) -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if opts.mode == json_serialize_mode_packed {
        return 0 as ::core::ffi::c_int;
    }
    if opts.mode == json_serialize_mode_multiline {
        if opts.opts & json_serialize_opt_use_tabs != 0 {
            flags |= f_tabs;
        }
    } else {
        if opts.opts & json_serialize_opt_pack_brackets == 0 {
            flags |= f_spaces_around_brackets;
        }
        if opts.opts & json_serialize_opt_no_space_after_comma == 0 {
            flags |= f_spaces_after_commas;
        }
    }
    if opts.opts & json_serialize_opt_no_space_after_colon == 0 {
        flags |= f_spaces_after_colons;
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn json_array_new(mut length: size_t) -> *mut json_value {
    let mut value: *mut json_value = calloc(
        1 as size_t,
        ::core::mem::size_of::<json_builder_value>() as size_t,
    ) as *mut json_value;
    if value.is_null() {
        return ::core::ptr::null_mut::<json_value>();
    }
    (*(value as *mut json_builder_value)).is_builder_value = 1 as ::core::ffi::c_int;
    (*value).type_0 = json_array;
    (*value).u.array.values =
        malloc(length.wrapping_mul(::core::mem::size_of::<*mut json_value>() as size_t))
            as *mut *mut json_value as *mut *mut _json_value;
    if (*value).u.array.values.is_null() {
        free(value as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<json_value>();
    }
    (*(value as *mut json_builder_value)).additional_length_allocated = length;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn json_array_push(
    mut array: *mut json_value,
    mut value: *mut json_value,
) -> *mut json_value {
    if builderize(array) == 0 || builderize(value) == 0 {
        return ::core::ptr::null_mut::<json_value>();
    }
    if (*(array as *mut json_builder_value)).additional_length_allocated > 0 as size_t {
        let ref mut fresh0 = (*(array as *mut json_builder_value)).additional_length_allocated;
        *fresh0 = (*fresh0).wrapping_sub(1);
    } else {
        let mut values_new: *mut *mut json_value = realloc(
            (*array).u.array.values as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut json_value>() as size_t).wrapping_mul(
                (*array)
                    .u
                    .array
                    .length
                    .wrapping_add(1 as ::core::ffi::c_uint) as size_t,
            ),
        ) as *mut *mut json_value;
        if values_new.is_null() {
            return ::core::ptr::null_mut::<json_value>();
        }
        (*array).u.array.values = values_new as *mut *mut _json_value;
    }
    let ref mut fresh1 = *(*array)
        .u
        .array
        .values
        .offset((*array).u.array.length as isize);
    *fresh1 = value as *mut _json_value;
    (*array).u.array.length = (*array).u.array.length.wrapping_add(1);
    (*value).parent = array as *mut _json_value;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new(mut length: size_t) -> *mut json_value {
    let mut value: *mut json_value = calloc(
        1 as size_t,
        ::core::mem::size_of::<json_builder_value>() as size_t,
    ) as *mut json_value;
    if value.is_null() {
        return ::core::ptr::null_mut::<json_value>();
    }
    (*(value as *mut json_builder_value)).is_builder_value = 1 as ::core::ffi::c_int;
    (*value).type_0 = json_object;
    (*value).u.object.values = calloc(
        length,
        ::core::mem::size_of::<json_object_entry>() as size_t,
    ) as *mut json_object_entry;
    if (*value).u.object.values.is_null() {
        free(value as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<json_value>();
    }
    (*(value as *mut json_builder_value)).additional_length_allocated = length;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_push(
    mut object: *mut json_value,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut json_value,
) -> *mut json_value {
    return json_object_push_length(object, strlen(name) as ::core::ffi::c_uint, name, value);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_push_length(
    mut object: *mut json_value,
    mut name_length: ::core::ffi::c_uint,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut json_value,
) -> *mut json_value {
    let mut name_copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    name_copy = malloc(
        (name_length.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    if name_copy.is_null() {
        return ::core::ptr::null_mut::<json_value>();
    }
    memcpy(
        name_copy as *mut ::core::ffi::c_void,
        name as *const ::core::ffi::c_void,
        (name_length as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    );
    *name_copy.offset(name_length as isize) = 0 as ::core::ffi::c_char;
    if json_object_push_nocopy(object, name_length, name_copy, value).is_null() {
        free(name_copy as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<json_value>();
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_push_nocopy(
    mut object: *mut json_value,
    mut name_length: ::core::ffi::c_uint,
    mut name: *mut ::core::ffi::c_char,
    mut value: *mut json_value,
) -> *mut json_value {
    let mut entry: *mut json_object_entry = ::core::ptr::null_mut::<json_object_entry>();
    if builderize(object) == 0 || builderize(value) == 0 {
        return ::core::ptr::null_mut::<json_value>();
    }
    if (*(object as *mut json_builder_value)).additional_length_allocated > 0 as size_t {
        let ref mut fresh2 = (*(object as *mut json_builder_value)).additional_length_allocated;
        *fresh2 = (*fresh2).wrapping_sub(1);
    } else {
        let mut values_new: *mut json_object_entry = realloc(
            (*object).u.object.values as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<json_object_entry>() as size_t).wrapping_mul(
                (*object)
                    .u
                    .object
                    .length
                    .wrapping_add(1 as ::core::ffi::c_uint) as size_t,
            ),
        ) as *mut json_object_entry;
        if values_new.is_null() {
            return ::core::ptr::null_mut::<json_value>();
        }
        (*object).u.object.values = values_new;
    }
    entry = (*object)
        .u
        .object
        .values
        .offset((*object).u.object.length as isize);
    (*entry).name_length = name_length;
    (*entry).name = name;
    (*entry).value = value as *mut _json_value;
    (*object).u.object.length = (*object).u.object.length.wrapping_add(1);
    (*value).parent = object as *mut _json_value;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn json_string_new(mut buf: *const ::core::ffi::c_char) -> *mut json_value {
    return json_string_new_length(strlen(buf) as ::core::ffi::c_uint, buf);
}
#[no_mangle]
pub unsafe extern "C" fn json_string_new_length(
    mut length: ::core::ffi::c_uint,
    mut buf: *const ::core::ffi::c_char,
) -> *mut json_value {
    let mut value: *mut json_value = ::core::ptr::null_mut::<json_value>();
    let mut copy: *mut ::core::ffi::c_char = malloc(
        (length.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    if copy.is_null() {
        return ::core::ptr::null_mut::<json_value>();
    }
    memcpy(
        copy as *mut ::core::ffi::c_void,
        buf as *const ::core::ffi::c_void,
        (length as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    );
    *copy.offset(length as isize) = 0 as ::core::ffi::c_char;
    value = json_string_new_nocopy(length, copy);
    if value.is_null() {
        free(copy as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<json_value>();
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn json_string_new_nocopy(
    mut length: ::core::ffi::c_uint,
    mut buf: *mut ::core::ffi::c_char,
) -> *mut json_value {
    let mut value: *mut json_value = calloc(
        1 as size_t,
        ::core::mem::size_of::<json_builder_value>() as size_t,
    ) as *mut json_value;
    if value.is_null() {
        return ::core::ptr::null_mut::<json_value>();
    }
    (*(value as *mut json_builder_value)).is_builder_value = 1 as ::core::ffi::c_int;
    (*value).type_0 = json_string;
    (*value).u.string.length = length;
    (*value).u.string.ptr = buf;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn json_integer_new(mut integer: int64_t) -> *mut json_value {
    let mut value: *mut json_value = calloc(
        1 as size_t,
        ::core::mem::size_of::<json_builder_value>() as size_t,
    ) as *mut json_value;
    if value.is_null() {
        return ::core::ptr::null_mut::<json_value>();
    }
    (*(value as *mut json_builder_value)).is_builder_value = 1 as ::core::ffi::c_int;
    (*value).type_0 = json_integer;
    (*value).u.integer = integer;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn json_double_new(mut dbl: ::core::ffi::c_double) -> *mut json_value {
    let mut value: *mut json_value = calloc(
        1 as size_t,
        ::core::mem::size_of::<json_builder_value>() as size_t,
    ) as *mut json_value;
    if value.is_null() {
        return ::core::ptr::null_mut::<json_value>();
    }
    (*(value as *mut json_builder_value)).is_builder_value = 1 as ::core::ffi::c_int;
    (*value).type_0 = json_double;
    (*value).u.dbl = dbl;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn json_boolean_new(mut b: ::core::ffi::c_int) -> *mut json_value {
    let mut value: *mut json_value = calloc(
        1 as size_t,
        ::core::mem::size_of::<json_builder_value>() as size_t,
    ) as *mut json_value;
    if value.is_null() {
        return ::core::ptr::null_mut::<json_value>();
    }
    (*(value as *mut json_builder_value)).is_builder_value = 1 as ::core::ffi::c_int;
    (*value).type_0 = json_boolean;
    (*value).u.boolean = b;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn json_null_new() -> *mut json_value {
    let mut value: *mut json_value = calloc(
        1 as size_t,
        ::core::mem::size_of::<json_builder_value>() as size_t,
    ) as *mut json_value;
    if value.is_null() {
        return ::core::ptr::null_mut::<json_value>();
    }
    (*(value as *mut json_builder_value)).is_builder_value = 1 as ::core::ffi::c_int;
    (*value).type_0 = json_null;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_sort(mut object: *mut json_value, mut proto: *mut json_value) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut out_index: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if builderize(object) == 0 {
        return;
    }
    i = 0 as ::core::ffi::c_uint;
    while i < (*proto).u.object.length {
        let mut j: ::core::ffi::c_uint = 0;
        let mut proto_entry: json_object_entry = *(*proto).u.object.values.offset(i as isize);
        j = 0 as ::core::ffi::c_uint;
        while j < (*object).u.object.length {
            let mut entry: json_object_entry = *(*object).u.object.values.offset(j as isize);
            if !(entry.name_length != proto_entry.name_length) {
                if !(memcmp(
                    entry.name as *const ::core::ffi::c_void,
                    proto_entry.name as *const ::core::ffi::c_void,
                    entry.name_length as size_t,
                ) != 0 as ::core::ffi::c_int)
                {
                    *(*object).u.object.values.offset(j as isize) =
                        *(*object).u.object.values.offset(out_index as isize);
                    *(*object).u.object.values.offset(out_index as isize) = entry;
                    out_index = out_index.wrapping_add(1);
                }
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn json_object_merge(
    mut objectA: *mut json_value,
    mut objectB: *mut json_value,
) -> *mut json_value {
    let mut i: ::core::ffi::c_uint = 0;
    if builderize(objectA) == 0 || builderize(objectB) == 0 {
        return ::core::ptr::null_mut::<json_value>();
    }
    if (*objectB).u.object.length as size_t
        <= (*(objectA as *mut json_builder_value)).additional_length_allocated
    {
        let ref mut fresh3 = (*(objectA as *mut json_builder_value)).additional_length_allocated;
        *fresh3 = (*fresh3).wrapping_sub((*objectB).u.object.length as size_t);
    } else {
        let mut values_new: *mut json_object_entry = ::core::ptr::null_mut::<json_object_entry>();
        let mut alloc: ::core::ffi::c_uint = ((*objectA).u.object.length as size_t)
            .wrapping_add((*(objectA as *mut json_builder_value)).additional_length_allocated)
            .wrapping_add((*objectB).u.object.length as size_t)
            as ::core::ffi::c_uint;
        values_new = realloc(
            (*objectA).u.object.values as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<json_object_entry>() as size_t).wrapping_mul(alloc as size_t),
        ) as *mut json_object_entry;
        if values_new.is_null() {
            return ::core::ptr::null_mut::<json_value>();
        }
        (*objectA).u.object.values = values_new;
    }
    i = 0 as ::core::ffi::c_uint;
    while i < (*objectB).u.object.length {
        let mut entry: *mut json_object_entry = (*objectA)
            .u
            .object
            .values
            .offset((*objectA).u.object.length.wrapping_add(i) as isize)
            as *mut json_object_entry;
        *entry = *(*objectB).u.object.values.offset(i as isize);
        (*(*entry).value).parent = objectA as *mut _json_value;
        i = i.wrapping_add(1);
    }
    (*objectA).u.object.length = (*objectA)
        .u
        .object
        .length
        .wrapping_add((*objectB).u.object.length);
    free((*objectB).u.object.values as *mut ::core::ffi::c_void);
    free(objectB as *mut ::core::ffi::c_void);
    return objectA;
}
unsafe extern "C" fn measure_string(
    mut length: ::core::ffi::c_uint,
    mut str: *const ::core::ffi::c_char,
) -> size_t {
    let mut i: ::core::ffi::c_uint = 0;
    let mut measured_length: size_t = 0 as size_t;
    i = 0 as ::core::ffi::c_uint;
    while i < length {
        let mut c: ::core::ffi::c_char = *str.offset(i as isize);
        match c as ::core::ffi::c_int {
            0 | 11 => {
                measured_length = measured_length.wrapping_add(6 as size_t);
            }
            34 | 92 | 8 | 12 | 10 | 13 | 9 => {
                measured_length = measured_length.wrapping_add(2 as size_t);
            }
            _ => {
                measured_length = measured_length.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    return measured_length;
}
unsafe extern "C" fn serialize_string(
    mut buf: *mut ::core::ffi::c_char,
    mut length: ::core::ffi::c_uint,
    mut str: *const ::core::ffi::c_char,
) -> size_t {
    let mut orig_buf: *mut ::core::ffi::c_char = buf;
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < length {
        let mut c: ::core::ffi::c_char = *str.offset(i as isize);
        match c as ::core::ffi::c_int {
            0 => {
                let fresh51 = buf;
                buf = buf.offset(1);
                *fresh51 = '\\' as i32 as ::core::ffi::c_char;
                let fresh52 = buf;
                buf = buf.offset(1);
                *fresh52 = 'u' as i32 as ::core::ffi::c_char;
                let fresh53 = buf;
                buf = buf.offset(1);
                *fresh53 = '0' as i32 as ::core::ffi::c_char;
                let fresh54 = buf;
                buf = buf.offset(1);
                *fresh54 = '0' as i32 as ::core::ffi::c_char;
                let fresh55 = buf;
                buf = buf.offset(1);
                *fresh55 = '0' as i32 as ::core::ffi::c_char;
                let fresh56 = buf;
                buf = buf.offset(1);
                *fresh56 = '0' as i32 as ::core::ffi::c_char;
            }
            34 => {
                let fresh57 = buf;
                buf = buf.offset(1);
                *fresh57 = '\\' as i32 as ::core::ffi::c_char;
                let fresh58 = buf;
                buf = buf.offset(1);
                *fresh58 = '"' as i32 as ::core::ffi::c_char;
            }
            92 => {
                let fresh59 = buf;
                buf = buf.offset(1);
                *fresh59 = '\\' as i32 as ::core::ffi::c_char;
                let fresh60 = buf;
                buf = buf.offset(1);
                *fresh60 = '\\' as i32 as ::core::ffi::c_char;
            }
            8 => {
                let fresh61 = buf;
                buf = buf.offset(1);
                *fresh61 = '\\' as i32 as ::core::ffi::c_char;
                let fresh62 = buf;
                buf = buf.offset(1);
                *fresh62 = 'b' as i32 as ::core::ffi::c_char;
            }
            12 => {
                let fresh63 = buf;
                buf = buf.offset(1);
                *fresh63 = '\\' as i32 as ::core::ffi::c_char;
                let fresh64 = buf;
                buf = buf.offset(1);
                *fresh64 = 'f' as i32 as ::core::ffi::c_char;
            }
            10 => {
                let fresh65 = buf;
                buf = buf.offset(1);
                *fresh65 = '\\' as i32 as ::core::ffi::c_char;
                let fresh66 = buf;
                buf = buf.offset(1);
                *fresh66 = 'n' as i32 as ::core::ffi::c_char;
            }
            13 => {
                let fresh67 = buf;
                buf = buf.offset(1);
                *fresh67 = '\\' as i32 as ::core::ffi::c_char;
                let fresh68 = buf;
                buf = buf.offset(1);
                *fresh68 = 'r' as i32 as ::core::ffi::c_char;
            }
            9 => {
                let fresh69 = buf;
                buf = buf.offset(1);
                *fresh69 = '\\' as i32 as ::core::ffi::c_char;
                let fresh70 = buf;
                buf = buf.offset(1);
                *fresh70 = 't' as i32 as ::core::ffi::c_char;
            }
            11 => {
                let fresh71 = buf;
                buf = buf.offset(1);
                *fresh71 = '\\' as i32 as ::core::ffi::c_char;
                let fresh72 = buf;
                buf = buf.offset(1);
                *fresh72 = 'u' as i32 as ::core::ffi::c_char;
                let fresh73 = buf;
                buf = buf.offset(1);
                *fresh73 = '0' as i32 as ::core::ffi::c_char;
                let fresh74 = buf;
                buf = buf.offset(1);
                *fresh74 = '0' as i32 as ::core::ffi::c_char;
                let fresh75 = buf;
                buf = buf.offset(1);
                *fresh75 = '0' as i32 as ::core::ffi::c_char;
                let fresh76 = buf;
                buf = buf.offset(1);
                *fresh76 = 'b' as i32 as ::core::ffi::c_char;
            }
            _ => {
                let fresh77 = buf;
                buf = buf.offset(1);
                *fresh77 = c;
            }
        }
        i = i.wrapping_add(1);
    }
    return buf.offset_from(orig_buf) as ::core::ffi::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn json_measure(mut value: *mut json_value) -> size_t {
    return json_measure_ex(value, default_opts);
}
#[no_mangle]
pub unsafe extern "C" fn json_measure_ex(
    mut value: *mut json_value,
    mut opts: json_serialize_opts,
) -> size_t {
    let mut total: size_t = 1 as size_t;
    let mut newlines: size_t = 0 as size_t;
    let mut depth: size_t = 0 as size_t;
    let mut indents: size_t = 0 as size_t;
    let mut flags: ::core::ffi::c_int = 0;
    let mut bracket_size: ::core::ffi::c_int = 0;
    let mut comma_size: ::core::ffi::c_int = 0;
    let mut colon_size: ::core::ffi::c_int = 0;
    flags = get_serialize_flags(opts);
    bracket_size = if flags & f_spaces_around_brackets != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    comma_size = if flags & f_spaces_after_commas != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    colon_size = if flags & f_spaces_after_colons != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    let mut current_block_76: u64;
    while !value.is_null() {
        let mut integer: int64_t = 0;
        let mut entry: *mut json_object_entry = ::core::ptr::null_mut::<json_object_entry>();
        match (*value).type_0 as ::core::ffi::c_uint {
            2 => {
                if (*(value as *mut json_builder_value)).length_iterated == 0 as size_t {
                    if (*value).u.array.length == 0 as ::core::ffi::c_uint {
                        total = total.wrapping_add(2 as size_t);
                        current_block_76 = 9437375157805982253;
                    } else {
                        total = total.wrapping_add(bracket_size as size_t);
                        depth = depth.wrapping_add(1);
                        newlines = newlines.wrapping_add(1);
                        indents = indents.wrapping_add(depth);
                        current_block_76 = 17407779659766490442;
                    }
                } else {
                    current_block_76 = 17407779659766490442;
                }
                match current_block_76 {
                    9437375157805982253 => {}
                    _ => {
                        if (*(value as *mut json_builder_value)).length_iterated
                            == (*value).u.array.length as size_t
                        {
                            depth = depth.wrapping_sub(1);
                            newlines = newlines.wrapping_add(1);
                            indents = indents.wrapping_add(depth);
                            total = total.wrapping_add(bracket_size as size_t);
                            (*(value as *mut json_builder_value)).length_iterated = 0 as size_t;
                        } else {
                            if (*(value as *mut json_builder_value)).length_iterated > 0 as size_t {
                                total = total.wrapping_add(comma_size as size_t);
                                newlines = newlines.wrapping_add(1);
                                indents = indents.wrapping_add(depth);
                            }
                            let ref mut fresh4 =
                                (*(value as *mut json_builder_value)).length_iterated;
                            *fresh4 = (*fresh4).wrapping_add(1);
                            value = *(*value).u.array.values.offset(
                                (*(value as *mut json_builder_value))
                                    .length_iterated
                                    .wrapping_sub(1 as size_t)
                                    as isize,
                            ) as *mut json_value;
                            continue;
                        }
                    }
                }
            }
            1 => {
                if (*(value as *mut json_builder_value)).length_iterated == 0 as size_t {
                    if (*value).u.object.length == 0 as ::core::ffi::c_uint {
                        total = total.wrapping_add(2 as size_t);
                        current_block_76 = 9437375157805982253;
                    } else {
                        total = total.wrapping_add(bracket_size as size_t);
                        depth = depth.wrapping_add(1);
                        newlines = newlines.wrapping_add(1);
                        indents = indents.wrapping_add(depth);
                        current_block_76 = 13131896068329595644;
                    }
                } else {
                    current_block_76 = 13131896068329595644;
                }
                match current_block_76 {
                    9437375157805982253 => {}
                    _ => {
                        if (*(value as *mut json_builder_value)).length_iterated
                            == (*value).u.object.length as size_t
                        {
                            depth = depth.wrapping_sub(1);
                            newlines = newlines.wrapping_add(1);
                            indents = indents.wrapping_add(depth);
                            total = total.wrapping_add(bracket_size as size_t);
                            (*(value as *mut json_builder_value)).length_iterated = 0 as size_t;
                        } else {
                            if (*(value as *mut json_builder_value)).length_iterated > 0 as size_t {
                                total = total.wrapping_add(comma_size as size_t);
                                newlines = newlines.wrapping_add(1);
                                indents = indents.wrapping_add(depth);
                            }
                            let ref mut fresh5 =
                                (*(value as *mut json_builder_value)).length_iterated;
                            let fresh6 = *fresh5;
                            *fresh5 = (*fresh5).wrapping_add(1);
                            entry = (*value).u.object.values.offset(fresh6 as isize);
                            total = total
                                .wrapping_add((2 as ::core::ffi::c_int + colon_size) as size_t);
                            total = total
                                .wrapping_add(measure_string((*entry).name_length, (*entry).name));
                            value = (*entry).value as *mut json_value;
                            continue;
                        }
                    }
                }
            }
            8 => {
                total = total.wrapping_add((*value).u.string.length as size_t);
            }
            5 => {
                total = total.wrapping_add(2 as size_t);
                total = total.wrapping_add(measure_string(
                    (*value).u.string.length,
                    (*value).u.string.ptr,
                ));
            }
            3 => {
                integer = (*value).u.integer;
                if integer < 0 as int64_t {
                    total = total.wrapping_add(1 as size_t);
                    integer = -integer;
                }
                total = total.wrapping_add(1);
                while integer >= 10 as int64_t {
                    total = total.wrapping_add(1);
                    integer /= 10 as int64_t;
                }
            }
            4 => {
                let mut buffer: [::core::ffi::c_char; 256] = [0; 256];
                emyg_dtoa((*value).u.dbl, &raw mut buffer as *mut ::core::ffi::c_char);
                total = total.wrapping_add(strlen(&raw mut buffer as *mut ::core::ffi::c_char));
            }
            6 => {
                total = total.wrapping_add(
                    (if (*value).u.boolean != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        5 as ::core::ffi::c_int
                    }) as size_t,
                );
            }
            7 => {
                total = total.wrapping_add(4 as size_t);
            }
            _ => {}
        }
        value = (*value).parent as *mut json_value;
    }
    if opts.mode == json_serialize_mode_multiline {
        total = total.wrapping_add(newlines.wrapping_mul(
            ((if opts.opts & json_serialize_opt_CRLF != 0 {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) + opts.indent_size) as size_t,
        ));
        total = total.wrapping_add(indents.wrapping_mul(opts.indent_size as size_t));
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn json_serialize(
    mut buf: *mut ::core::ffi::c_char,
    mut value: *mut json_value,
) {
    json_serialize_ex(buf, value, default_opts);
}
#[no_mangle]
pub unsafe extern "C" fn json_serialize_ex(
    mut buf: *mut ::core::ffi::c_char,
    mut value: *mut json_value,
    mut opts: json_serialize_opts,
) {
    let mut integer: int64_t = 0;
    let mut orig_integer: int64_t = 0;
    let mut entry: *mut json_object_entry = ::core::ptr::null_mut::<json_object_entry>();
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut indent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut indent_char: ::core::ffi::c_char = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    flags = get_serialize_flags(opts);
    indent_char = (if flags & f_tabs != 0 {
        '\t' as i32
    } else {
        ' ' as i32
    }) as ::core::ffi::c_char;
    let mut current_block_156: u64;
    while !value.is_null() {
        match (*value).type_0 as ::core::ffi::c_uint {
            2 => {
                if (*(value as *mut json_builder_value)).length_iterated == 0 as size_t {
                    if (*value).u.array.length == 0 as ::core::ffi::c_uint {
                        let fresh7 = buf;
                        buf = buf.offset(1);
                        *fresh7 = '[' as i32 as ::core::ffi::c_char;
                        let fresh8 = buf;
                        buf = buf.offset(1);
                        *fresh8 = ']' as i32 as ::core::ffi::c_char;
                        current_block_156 = 5089124893069931607;
                    } else {
                        let fresh9 = buf;
                        buf = buf.offset(1);
                        *fresh9 = '[' as i32 as ::core::ffi::c_char;
                        if flags & f_spaces_around_brackets != 0 {
                            let fresh10 = buf;
                            buf = buf.offset(1);
                            *fresh10 = ' ' as i32 as ::core::ffi::c_char;
                        }
                        indent += opts.indent_size;
                        if opts.mode == json_serialize_mode_multiline {
                            if opts.opts & json_serialize_opt_CRLF != 0 {
                                let fresh11 = buf;
                                buf = buf.offset(1);
                                *fresh11 = '\r' as i32 as ::core::ffi::c_char;
                            }
                            let fresh12 = buf;
                            buf = buf.offset(1);
                            *fresh12 = '\n' as i32 as ::core::ffi::c_char;
                            i = 0 as ::core::ffi::c_int;
                            while i < indent {
                                let fresh13 = buf;
                                buf = buf.offset(1);
                                *fresh13 = indent_char;
                                i += 1;
                            }
                        }
                        current_block_156 = 13472856163611868459;
                    }
                } else {
                    current_block_156 = 13472856163611868459;
                }
                match current_block_156 {
                    5089124893069931607 => {}
                    _ => {
                        if (*(value as *mut json_builder_value)).length_iterated
                            == (*value).u.array.length as size_t
                        {
                            indent -= opts.indent_size;
                            if opts.mode == json_serialize_mode_multiline {
                                if opts.opts & json_serialize_opt_CRLF != 0 {
                                    let fresh14 = buf;
                                    buf = buf.offset(1);
                                    *fresh14 = '\r' as i32 as ::core::ffi::c_char;
                                }
                                let fresh15 = buf;
                                buf = buf.offset(1);
                                *fresh15 = '\n' as i32 as ::core::ffi::c_char;
                                i = 0 as ::core::ffi::c_int;
                                while i < indent {
                                    let fresh16 = buf;
                                    buf = buf.offset(1);
                                    *fresh16 = indent_char;
                                    i += 1;
                                }
                            }
                            if flags & f_spaces_around_brackets != 0 {
                                let fresh17 = buf;
                                buf = buf.offset(1);
                                *fresh17 = ' ' as i32 as ::core::ffi::c_char;
                            }
                            let fresh18 = buf;
                            buf = buf.offset(1);
                            *fresh18 = ']' as i32 as ::core::ffi::c_char;
                            (*(value as *mut json_builder_value)).length_iterated = 0 as size_t;
                        } else {
                            if (*(value as *mut json_builder_value)).length_iterated > 0 as size_t {
                                let fresh19 = buf;
                                buf = buf.offset(1);
                                *fresh19 = ',' as i32 as ::core::ffi::c_char;
                                if flags & f_spaces_after_commas != 0 {
                                    let fresh20 = buf;
                                    buf = buf.offset(1);
                                    *fresh20 = ' ' as i32 as ::core::ffi::c_char;
                                }
                                if opts.mode == json_serialize_mode_multiline {
                                    if opts.opts & json_serialize_opt_CRLF != 0 {
                                        let fresh21 = buf;
                                        buf = buf.offset(1);
                                        *fresh21 = '\r' as i32 as ::core::ffi::c_char;
                                    }
                                    let fresh22 = buf;
                                    buf = buf.offset(1);
                                    *fresh22 = '\n' as i32 as ::core::ffi::c_char;
                                    i = 0 as ::core::ffi::c_int;
                                    while i < indent {
                                        let fresh23 = buf;
                                        buf = buf.offset(1);
                                        *fresh23 = indent_char;
                                        i += 1;
                                    }
                                }
                            }
                            let ref mut fresh24 =
                                (*(value as *mut json_builder_value)).length_iterated;
                            *fresh24 = (*fresh24).wrapping_add(1);
                            value = *(*value).u.array.values.offset(
                                (*(value as *mut json_builder_value))
                                    .length_iterated
                                    .wrapping_sub(1 as size_t)
                                    as isize,
                            ) as *mut json_value;
                            continue;
                        }
                    }
                }
            }
            1 => {
                if (*(value as *mut json_builder_value)).length_iterated == 0 as size_t {
                    if (*value).u.object.length == 0 as ::core::ffi::c_uint {
                        let fresh25 = buf;
                        buf = buf.offset(1);
                        *fresh25 = '{' as i32 as ::core::ffi::c_char;
                        let fresh26 = buf;
                        buf = buf.offset(1);
                        *fresh26 = '}' as i32 as ::core::ffi::c_char;
                        current_block_156 = 5089124893069931607;
                    } else {
                        let fresh27 = buf;
                        buf = buf.offset(1);
                        *fresh27 = '{' as i32 as ::core::ffi::c_char;
                        if flags & f_spaces_around_brackets != 0 {
                            let fresh28 = buf;
                            buf = buf.offset(1);
                            *fresh28 = ' ' as i32 as ::core::ffi::c_char;
                        }
                        indent += opts.indent_size;
                        if opts.mode == json_serialize_mode_multiline {
                            if opts.opts & json_serialize_opt_CRLF != 0 {
                                let fresh29 = buf;
                                buf = buf.offset(1);
                                *fresh29 = '\r' as i32 as ::core::ffi::c_char;
                            }
                            let fresh30 = buf;
                            buf = buf.offset(1);
                            *fresh30 = '\n' as i32 as ::core::ffi::c_char;
                            i = 0 as ::core::ffi::c_int;
                            while i < indent {
                                let fresh31 = buf;
                                buf = buf.offset(1);
                                *fresh31 = indent_char;
                                i += 1;
                            }
                        }
                        current_block_156 = 3736434875406665187;
                    }
                } else {
                    current_block_156 = 3736434875406665187;
                }
                match current_block_156 {
                    5089124893069931607 => {}
                    _ => {
                        if (*(value as *mut json_builder_value)).length_iterated
                            == (*value).u.object.length as size_t
                        {
                            indent -= opts.indent_size;
                            if opts.mode == json_serialize_mode_multiline {
                                if opts.opts & json_serialize_opt_CRLF != 0 {
                                    let fresh32 = buf;
                                    buf = buf.offset(1);
                                    *fresh32 = '\r' as i32 as ::core::ffi::c_char;
                                }
                                let fresh33 = buf;
                                buf = buf.offset(1);
                                *fresh33 = '\n' as i32 as ::core::ffi::c_char;
                                i = 0 as ::core::ffi::c_int;
                                while i < indent {
                                    let fresh34 = buf;
                                    buf = buf.offset(1);
                                    *fresh34 = indent_char;
                                    i += 1;
                                }
                            }
                            if flags & f_spaces_around_brackets != 0 {
                                let fresh35 = buf;
                                buf = buf.offset(1);
                                *fresh35 = ' ' as i32 as ::core::ffi::c_char;
                            }
                            let fresh36 = buf;
                            buf = buf.offset(1);
                            *fresh36 = '}' as i32 as ::core::ffi::c_char;
                            (*(value as *mut json_builder_value)).length_iterated = 0 as size_t;
                        } else {
                            if (*(value as *mut json_builder_value)).length_iterated > 0 as size_t {
                                let fresh37 = buf;
                                buf = buf.offset(1);
                                *fresh37 = ',' as i32 as ::core::ffi::c_char;
                                if flags & f_spaces_after_commas != 0 {
                                    let fresh38 = buf;
                                    buf = buf.offset(1);
                                    *fresh38 = ' ' as i32 as ::core::ffi::c_char;
                                }
                                if opts.mode == json_serialize_mode_multiline {
                                    if opts.opts & json_serialize_opt_CRLF != 0 {
                                        let fresh39 = buf;
                                        buf = buf.offset(1);
                                        *fresh39 = '\r' as i32 as ::core::ffi::c_char;
                                    }
                                    let fresh40 = buf;
                                    buf = buf.offset(1);
                                    *fresh40 = '\n' as i32 as ::core::ffi::c_char;
                                    i = 0 as ::core::ffi::c_int;
                                    while i < indent {
                                        let fresh41 = buf;
                                        buf = buf.offset(1);
                                        *fresh41 = indent_char;
                                        i += 1;
                                    }
                                }
                            }
                            let ref mut fresh42 =
                                (*(value as *mut json_builder_value)).length_iterated;
                            let fresh43 = *fresh42;
                            *fresh42 = (*fresh42).wrapping_add(1);
                            entry = (*value).u.object.values.offset(fresh43 as isize);
                            let fresh44 = buf;
                            buf = buf.offset(1);
                            *fresh44 = '"' as i32 as ::core::ffi::c_char;
                            buf = buf.offset(serialize_string(
                                buf,
                                (*entry).name_length,
                                (*entry).name,
                            ) as isize);
                            let fresh45 = buf;
                            buf = buf.offset(1);
                            *fresh45 = '"' as i32 as ::core::ffi::c_char;
                            let fresh46 = buf;
                            buf = buf.offset(1);
                            *fresh46 = ':' as i32 as ::core::ffi::c_char;
                            if flags & f_spaces_after_colons != 0 {
                                let fresh47 = buf;
                                buf = buf.offset(1);
                                *fresh47 = ' ' as i32 as ::core::ffi::c_char;
                            }
                            value = (*entry).value as *mut json_value;
                            continue;
                        }
                    }
                }
            }
            8 => {
                memcpy(
                    buf as *mut ::core::ffi::c_void,
                    (*value).u.string.ptr as *const ::core::ffi::c_void,
                    (*value).u.string.length as size_t,
                );
                buf = buf.offset((*value).u.string.length as isize);
            }
            5 => {
                let fresh48 = buf;
                buf = buf.offset(1);
                *fresh48 = '"' as i32 as ::core::ffi::c_char;
                buf = buf.offset(serialize_string(
                    buf,
                    (*value).u.string.length,
                    (*value).u.string.ptr,
                ) as isize);
                let fresh49 = buf;
                buf = buf.offset(1);
                *fresh49 = '"' as i32 as ::core::ffi::c_char;
            }
            3 => {
                integer = (*value).u.integer;
                if integer < 0 as int64_t {
                    let fresh50 = buf;
                    buf = buf.offset(1);
                    *fresh50 = '-' as i32 as ::core::ffi::c_char;
                    integer = -integer;
                }
                orig_integer = integer;
                buf = buf.offset(1);
                while integer >= 10 as int64_t {
                    buf = buf.offset(1);
                    integer /= 10 as int64_t;
                }
                integer = orig_integer;
                ptr = buf;
                loop {
                    ptr = ptr.offset(-1);
                    *ptr = ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(
                        *b"0123456789\0",
                    )[(integer % 10 as int64_t) as usize];
                    integer /= 10 as int64_t;
                    if !(integer > 0 as int64_t) {
                        break;
                    }
                }
            }
            4 => {
                let mut tmp: [::core::ffi::c_char; 256] = [0; 256];
                emyg_dtoa((*value).u.dbl, &raw mut tmp as *mut ::core::ffi::c_char);
                memcpy(
                    buf as *mut ::core::ffi::c_void,
                    &raw mut tmp as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    strlen(&raw mut tmp as *mut ::core::ffi::c_char),
                );
                buf = buf.offset(strlen(&raw mut tmp as *mut ::core::ffi::c_char) as isize);
            }
            6 => {
                if (*value).u.boolean != 0 {
                    memcpy(
                        buf as *mut ::core::ffi::c_void,
                        b"true\0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        4 as size_t,
                    );
                    buf = buf.offset(4 as ::core::ffi::c_int as isize);
                } else {
                    memcpy(
                        buf as *mut ::core::ffi::c_void,
                        b"false\0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        5 as size_t,
                    );
                    buf = buf.offset(5 as ::core::ffi::c_int as isize);
                }
            }
            7 => {
                memcpy(
                    buf as *mut ::core::ffi::c_void,
                    b"null\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    4 as size_t,
                );
                buf = buf.offset(4 as ::core::ffi::c_int as isize);
            }
            _ => {}
        }
        value = (*value).parent as *mut json_value;
    }
    *buf = 0 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn json_builder_free(mut value: *mut json_value) {
    let mut cur_value: *mut json_value = ::core::ptr::null_mut::<json_value>();
    if value.is_null() {
        return;
    }
    (*value).parent = ::core::ptr::null_mut::<_json_value>();
    while !value.is_null() {
        match (*value).type_0 as ::core::ffi::c_uint {
            2 => {
                if (*value).u.array.length == 0 {
                    free((*value).u.array.values as *mut ::core::ffi::c_void);
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
                    free((*value).u.object.values as *mut ::core::ffi::c_void);
                } else {
                    (*value).u.object.length = (*value).u.object.length.wrapping_sub(1);
                    if (*(value as *mut json_builder_value)).is_builder_value != 0 {
                        free(
                            (*(*value)
                                .u
                                .object
                                .values
                                .offset((*value).u.object.length as isize))
                            .name as *mut ::core::ffi::c_void,
                        );
                    }
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
                free((*value).u.string.ptr as *mut ::core::ffi::c_void);
            }
            _ => {}
        }
        cur_value = value;
        value = (*value).parent as *mut json_value;
        free(cur_value as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn run_static_initializers() {
    json_builder_extra = (::core::mem::size_of::<json_builder_value>() as size_t)
        .wrapping_sub(::core::mem::size_of::<json_value>() as size_t);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
