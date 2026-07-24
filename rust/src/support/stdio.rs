// c2rust duplicated glibc's `FILE`/`_IO_FILE` struct (and its dependent
// opaque types and `__off_t`/`__off64_t` aliases) verbatim in every file that
// touches a `FILE *` — confirmed byte-identical across all 75 occurrences
// before this dedup. Centralizing the type here means the stdio stream
// globals (`stderr`/`stdin`/`stdout`) only need one portable, per-platform
// binding each (see #13's macOS fix), instead of repeating the fix in every
// file that happens to also declare its own copy of `FILE`.

pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: usize,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type FILE = _IO_FILE;

#[cfg(target_os = "macos")]
extern "C" {
    #[link_name = "__stderrp"]
    pub static mut stderr: *mut FILE;
    #[link_name = "__stdinp"]
    pub static mut stdin: *mut FILE;
    #[link_name = "__stdoutp"]
    pub static mut stdout: *mut FILE;
}
#[cfg(not(target_os = "macos"))]
extern "C" {
    pub static mut stderr: *mut FILE;
    pub static mut stdin: *mut FILE;
    pub static mut stdout: *mut FILE;
}
