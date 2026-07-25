// Shared zero-initialized alloc/realloc helpers, factored out of the ~50
// per-file private copies c2rust emitted (one per translation unit that
// #included c/lib/support/mem.h's `NEW_CLEAN`/`RENEW_CLEAN` macros). These
// were never externally linked (no #[no_mangle]) even in their per-file
// form, so consolidating them changes no ABI -- only support/bk (the files
// already reviewed for the idiomatization pass) have been migrated to use
// this module so far; the remaining ~47 files still carry their own private
// copy pending a future, wider pass.
use crate::support::stdio::{stderr, FILE};
pub type size_t = usize;

extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}

const EXIT_FAILURE: ::core::ffi::c_int = 1;

#[inline]
pub(crate) unsafe fn __caryll_allocate_clean(
    n: size_t,
    line: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    if n == 0 {
        return ::core::ptr::null_mut();
    }
    let p = calloc(n, 1);
    if p.is_null() {
        fprintf(
            stderr,
            b"[%ld]Out of memory(%ld bytes)\n\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            n as ::core::ffi::c_ulong,
        );
        exit(EXIT_FAILURE);
    }
    p
}

#[inline]
pub(crate) unsafe fn __caryll_reallocate(
    ptr: *mut ::core::ffi::c_void,
    n: size_t,
    line: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    if n == 0 {
        free(ptr);
        return ::core::ptr::null_mut();
    }
    if ptr.is_null() {
        return __caryll_allocate_clean(n, line);
    }
    let p = realloc(ptr, n);
    if p.is_null() {
        fprintf(
            stderr,
            b"[%ld]Out of memory(%ld bytes)\n\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            n as ::core::ffi::c_ulong,
        );
        exit(EXIT_FAILURE);
    }
    p
}
