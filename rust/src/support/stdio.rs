// The stdio types and stream globals, taken from the `libc` crate.
//
// c2rust duplicated glibc's `FILE`/`_IO_FILE` struct — all 29 fields of it,
// plus three `extern type` opaque helpers and the `__off_t`/`__off64_t`
// aliases — verbatim in every one of the 75 files that touches a `FILE *`.
// That was wrong twice over: the layout it reproduces is glibc's, so it does
// not describe `FILE` on macOS at all, and it was only harmless because
// nothing in this crate ever reads a field of it (verified). `libc::FILE` is
// opaque by construction, which is what a `FILE *` actually is to us.
//
// Retiring the `extern type`s here removes the crate's last use of the
// `extern_types` nightly feature.
//
// The stream globals stay hand-written: `libc` deliberately does not export
// `stdin`/`stdout`/`stderr`, since they are macros on some platforms. Keeping
// them here still means one binding per stream for the whole crate, with the
// macOS link names (`__stdinp` and friends) applied once.

pub use libc::FILE;

// `libc::FILE` is the Nomicon opaque-struct idiom (a `#[repr(C)]` struct whose
// only field is `()`), which `improper_ctypes` flags on sight. libc allows the
// same lint for its own declarations, for the same reason: an opaque `FILE *`
// is precisely how C hands these out.
#[allow(improper_ctypes)]
#[cfg(target_os = "macos")]
extern "C" {
    #[link_name = "__stderrp"]
    pub static mut stderr: *mut FILE;
    #[link_name = "__stdinp"]
    pub static mut stdin: *mut FILE;
    #[link_name = "__stdoutp"]
    pub static mut stdout: *mut FILE;
}
#[allow(improper_ctypes)]
#[cfg(not(target_os = "macos"))]
extern "C" {
    pub static mut stderr: *mut FILE;
    pub static mut stdin: *mut FILE;
    pub static mut stdout: *mut FILE;
}
