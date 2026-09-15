// Shared zero-initialized alloc/realloc helpers, factored out of the ~50
// per-file private copies c2rust emitted (one per translation unit that
// #included c/lib/support/mem.h's `NEW_CLEAN`/`RENEW_CLEAN` macros). These
// were never externally linked (no #[no_mangle]) even in their per-file
// form, so consolidating them changes no ABI -- only support/bk (the files
// already reviewed for the idiomatization pass) have been migrated to use
// this module so far; the remaining ~47 files still carry their own private
// copy pending a future, wider pass.
use libc::{calloc, free, realloc};
use std::alloc::{Layout, handle_alloc_error};

#[inline]
pub(crate) unsafe fn __caryll_allocate_clean(
    n: usize,
    // Was embedded in the old OOM message ("[<line>]Out of memory(...)"),
    // one per c/lib/support/mem.h call site. `handle_alloc_error` reports
    // the allocation itself (size, and the standard "memory allocation of N
    // bytes failed" abort message) instead of a call-site line number, so
    // this is now unused -- kept as a parameter so the ~50 call sites across
    // the crate don't need touching, the same choice `bk/bkgraph.rs`'s
    // `compute_block_offsets` made for its own now-vestigial `_line`.
    _line: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    if n == 0 {
        return ::core::ptr::null_mut();
    }
    let p = unsafe { calloc(n, 1) };
    if p.is_null() {
        handle_alloc_error(Layout::from_size_align(n, 1).unwrap());
    }
    p
}

#[inline]
pub(crate) unsafe fn __caryll_reallocate(
    ptr: *mut ::core::ffi::c_void,
    n: usize,
    line: ::core::ffi::c_ulong,
) -> *mut ::core::ffi::c_void {
    if n == 0 {
        unsafe { free(ptr) };
        return ::core::ptr::null_mut();
    }
    if ptr.is_null() {
        return unsafe { __caryll_allocate_clean(n, line) };
    }
    let p = unsafe { realloc(ptr, n) };
    if p.is_null() {
        handle_alloc_error(Layout::from_size_align(n, 1).unwrap());
    }
    p
}
