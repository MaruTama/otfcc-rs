#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{fprintf, free, fwrite};

use crate::support::stdio::{stderr};
use crate::support::alloc::{__caryll_allocate_clean};

// Was `ILoggerTarget`, a 2-field vtable (`dispose`/`push`) with exactly two
// static instances (stderr output vs no-op) selected once at `Logger`
// construction and never switched afterward -- the same "genuine but
// call-path-fixed" 2-way choice already converted to enum+match elsewhere
// in this migration. Neither variant carries payload or owns a heap
// allocation (the old `StderrTarget` shell existed only to give the
// vtable pointer somewhere to live), so `dispose` needs no counterpart:
// dropping a `Logger` drops its inline `target` field for free.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LoggerTarget {
    Stderr,
    Empty,
}
impl LoggerTarget {
    unsafe fn push(self, data: Vec<u8>) {
        match self {
            LoggerTarget::Stderr => {
                // Writes the exact byte count rather than `fprintf("%s", ...)`:
                // `data` is no longer NUL-terminated storage, and this crate's log
                // text is not expected to contain an embedded NUL either way.
                fwrite(
                    data.as_ptr() as *const ::core::ffi::c_void,
                    1,
                    data.len(),
                    stderr,
                );
                if data.last() != Some(&b'\n') {
                    fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                }
            }
            LoggerTarget::Empty => {
                drop(data);
            }
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum LoggerType {
    Error = 0,
    Warning = 1,
    Info = 2,
    Progress = 3,
}
// How noisy a message is: `logger_log_sds` prints it when
// `verbosity <= self->verbosity_limit`, so these are thresholds on a scale, not
// members of a set -- and `logger_start`/`logger_finish` do arithmetic on one
// (`LOG_VL_PROGRESS + level`, deeper nesting being more verbose), which an enum
// could not express. So they stay plain integers, typed as the `u8` every
// logging entry point takes; that is what drops the `as c_int as u8` pair from
// the 149 call sites.
pub const LOG_VL_CRITICAL: u8 = 0;
pub const LOG_VL_IMPORTANT: u8 = 1;
pub const LOG_VL_NOTICE: u8 = 2;
pub const LOG_VL_INFO: u8 = 5;
pub const LOG_VL_PROGRESS: u8 = 10;
#[repr(C)]
pub struct Logger {
    pub target: LoggerTarget,
    pub level: u16,
    pub last_logged_level: u16,
    /// One entry per indent level, holding that level's own segment text.
    /// Was a manually malloc/realloc'd `*mut SdsRaw` array (with a
    /// separately-tracked `level_cap`); `Vec::push`/`.pop()` manage growth
    /// themselves, so `level_cap` has no replacement -- `level` stays a
    /// separate field rather than becoming `indents.len()` purely to keep
    /// this conversion's diff to "change the storage", not "also fold two
    /// fields into one".
    pub indents: Vec<Vec<u8>>,
    pub verbosity_limit: u8,
}
pub static OTFCC_LOGGER_TYPE_NAMES: [&::core::ffi::CStr; 3] = [
    c"[ERROR]",
    c"[WARNING]",
    c"[NOTE]",
];
pub unsafe fn logger_indent(
    mut _self: *mut Logger,
    mut segment: *const ::core::ffi::c_char,
) {
    logger_indent_sds(_self, crate::bytesbuild!(segment));
}
pub unsafe fn logger_indent_sds(mut self_0: *mut Logger, mut segment: Vec<u8>) {
    (*self_0).indents.push(segment);
    (*self_0).level = (*self_0).indents.len() as u16;
}
pub unsafe fn logger_dedent(mut self_0: *mut Logger) {
    if (*self_0).level == 0 {
        return;
    }
    (*self_0).indents.pop();
    (*self_0).level = ((*self_0).level as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u16;
    if ((*self_0).level as ::core::ffi::c_int) < (*self_0).last_logged_level as ::core::ffi::c_int {
        (*self_0).last_logged_level = (*self_0).level;
    }
}
pub unsafe fn logger_finish(mut self_0: *mut Logger) {
    logger_log_sds(
        self_0,
        (LOG_VL_PROGRESS as ::core::ffi::c_int
            + (*self_0).level as ::core::ffi::c_int) as u8,
        LoggerType::Progress,
        crate::bytesbuild!(b"Finish"),
    );
    logger_dedent(self_0);
}
pub unsafe fn logger_start_sds(mut self_0: *mut Logger, mut segment: Vec<u8>) {
    logger_indent_sds(self_0, segment);
    logger_log_sds(
        self_0,
        (LOG_VL_PROGRESS as ::core::ffi::c_int
            + (*self_0).level as ::core::ffi::c_int) as u8,
        LoggerType::Progress,
        crate::bytesbuild!(b"Begin"),
    );
}
pub unsafe fn logger_log_sds(
    mut self_0: *mut Logger,
    mut verbosity: u8,
    mut type_0: LoggerType,
    mut data: Vec<u8>,
) {
    let mut demand: Vec<u8> = Vec::new();
    let mut level: u16 = 0 as u16;
    while (level as ::core::ffi::c_int) < (*self_0).level as ::core::ffi::c_int {
        if (level as ::core::ffi::c_int)
            < (*self_0).last_logged_level as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            let seg_len = (&(*self_0).indents)[level as usize].len();
            let mut j: usize = 0 as usize;
            while j < seg_len {
                demand.extend_from_slice(b" ");
                j = j.wrapping_add(1);
            }
            if (level as ::core::ffi::c_int)
                < (*self_0).last_logged_level as ::core::ffi::c_int - 2 as ::core::ffi::c_int
            {
                demand.extend_from_slice(b" | ");
            } else {
                demand.extend_from_slice(b" |-");
            }
        } else {
            demand.extend_from_slice(&(&(*self_0).indents)[level as usize]);
            demand.extend_from_slice(b" : ");
        }
        level = level.wrapping_add(1);
    }
    if (type_0 as ::core::ffi::c_uint) < 3 as ::core::ffi::c_uint {
        demand.extend_from_slice(OTFCC_LOGGER_TYPE_NAMES[type_0 as usize].to_bytes());
        demand.extend_from_slice(b" ");
        demand.extend_from_slice(&data);
    } else {
        demand.extend_from_slice(&data);
    }
    // `data` (an owned `Vec<u8>` parameter) drops here, at the same point
    // the old `sdsfree(data)` ran -- no explicit free needed.
    if verbosity as ::core::ffi::c_int <= (*self_0).verbosity_limit as ::core::ffi::c_int {
        (*self_0).target.push(demand);
        (*self_0).last_logged_level = (*self_0).level;
    }
    // else `demand` just drops.
}
pub unsafe fn logger_set_verbosity(mut self_0: *mut Logger, mut verbosity: u8) {
    (*self_0).verbosity_limit = verbosity;
}
#[inline]
pub unsafe fn logger_dispose(mut self_0: *mut Logger) {
    if self_0.is_null() {
        return;
    }
    // `target` is an inline `LoggerTarget` (no heap allocation of its own
    // to free) -- runs `Vec<Vec<u8>>`'s own destructor (frees every segment's backing
    // buffer, then the outer `Vec`'s) before the raw `free()` below drops
    // the malloc'd `Logger` struct out from under it -- matches the old
    // per-entry `sdsfree` loop plus the final `free(indents)`.
    drop(::core::mem::take(&mut (*self_0).indents));
    free(self_0 as *mut ::core::ffi::c_void);
    self_0 = ::core::ptr::null_mut::<Logger>();
}
pub unsafe fn otfcc_new_logger(
    mut target: LoggerTarget,
) -> *mut Logger {
    let mut logger: *mut Logger = ::core::ptr::null_mut::<Logger>();
    logger = __caryll_allocate_clean(
        ::core::mem::size_of::<Logger>() as usize,
        120 as ::core::ffi::c_ulong,
    ) as *mut Logger;
    (*logger).target = target;
    // `__caryll_allocate_clean` calloc's the struct, which is not a valid
    // `Vec<Vec<u8>>` bit pattern (a zero-capacity `Vec` uses a dangling
    // *non-null* sentinel pointer, not a null one). A plain `=` assignment
    // here is UB, not just wrong -- Rust drops the old place's value before
    // writing the new one, and dropping an all-zero-bytes `Vec` means
    // constructing a `Unique<u8>`/`NonNull<u8>` from a null pointer, which
    // is instant UB the moment that typed value exists (caught by Miri:
    // "constructing invalid value ... encountered 0, but expected
    // something greater or equal to 1"), independent of whether the drop
    // glue would have gone on to actually dereference it. `ptr::write`
    // performs the same bitwise store without reading/dropping what was
    // there before, which is what a freshly-calloc'd, not-yet-Rust-valid
    // field actually needs.
    ::core::ptr::write(&raw mut (*logger).indents, Vec::new());
    return logger;
}
pub unsafe fn otfcc_new_std_err_target() -> LoggerTarget {
    LoggerTarget::Stderr
}
pub unsafe fn otfcc_new_empty_target() -> LoggerTarget {
    LoggerTarget::Empty
}
