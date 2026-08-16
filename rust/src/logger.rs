#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
// `ILogger`/`ILoggerTarget` now pass `Vec<u8>` through their `extern "C"`
// vtable slots -- internal-only (vtable dispatch within this crate, no
// real FFI boundary; see rust/README.md's Stage 6-2 sds sweep) -- goes
// away with the vtable/extern "C" cleanup, same as every other instance
// of this allow in the crate.
#![allow(improper_ctypes_definitions)]
use libc::{fprintf, free, fwrite};

use crate::support::stdio::{stderr};
use crate::support::alloc::{__caryll_allocate_clean};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ILoggerTarget {
    pub dispose: Option<unsafe extern "C" fn(*mut ILoggerTarget) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut ILoggerTarget, Vec<u8>) -> ()>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ILogger {
    pub dispose: Option<unsafe extern "C" fn(*mut ILogger) -> ()>,
    pub indent: Option<unsafe extern "C" fn(*mut ILogger, *const ::core::ffi::c_char) -> ()>,
    pub indent_sds: Option<unsafe extern "C" fn(*mut ILogger, Vec<u8>) -> ()>,
    pub start: Option<unsafe extern "C" fn(*mut ILogger, *const ::core::ffi::c_char) -> ()>,
    pub start_sds: Option<unsafe extern "C" fn(*mut ILogger, Vec<u8>) -> ()>,
    pub log: Option<
        unsafe extern "C" fn(
            *mut ILogger,
            u8,
            LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub log_sds:
        Option<unsafe extern "C" fn(*mut ILogger, u8, LoggerType, Vec<u8>) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut ILogger) -> ()>,
    pub set_verbosity: Option<unsafe extern "C" fn(*mut ILogger, u8) -> ()>,
    pub get_target: Option<unsafe extern "C" fn(*mut ILogger) -> *mut ILoggerTarget>,
}
#[repr(C)]
pub struct Logger {
    pub vtable: ILogger,
    pub target: *mut ::core::ffi::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StderrTarget {
    pub vtable: ILoggerTarget,
}
pub static OTFCC_LOGGER_TYPE_NAMES: [&::core::ffi::CStr; 3] = [
    c"[ERROR]",
    c"[WARNING]",
    c"[NOTE]",
];
unsafe extern "C" fn logger_indent(
    mut _self: *mut ILogger,
    mut segment: *const ::core::ffi::c_char,
) {
    (*_self).indent_sds.expect("non-null function pointer")(
        _self as *mut ILogger,
        crate::bytesbuild!(segment),
    );
}
unsafe extern "C" fn logger_indent_sds(mut _self: *mut ILogger, mut segment: Vec<u8>) {
    let mut self_0: *mut Logger = _self as *mut Logger;
    (*self_0).indents.push(segment);
    (*self_0).level = (*self_0).indents.len() as u16;
}
unsafe extern "C" fn logger_dedent(mut _self: *mut ILogger) {
    let mut self_0: *mut Logger = _self as *mut Logger;
    if (*self_0).level == 0 {
        return;
    }
    (*self_0).indents.pop();
    (*self_0).level = ((*self_0).level as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u16;
    if ((*self_0).level as ::core::ffi::c_int) < (*self_0).last_logged_level as ::core::ffi::c_int {
        (*self_0).last_logged_level = (*self_0).level;
    }
}
unsafe extern "C" fn logger_finish(mut self_0: *mut ILogger) {
    (*self_0).log_sds.expect("non-null function pointer")(
        self_0 as *mut ILogger,
        (LOG_VL_PROGRESS as ::core::ffi::c_int
            + (*(self_0 as *mut Logger)).level as ::core::ffi::c_int) as u8,
        LoggerType::Progress,
        crate::bytesbuild!(b"Finish"),
    );
    (*self_0).dedent.expect("non-null function pointer")(self_0 as *mut ILogger);
}
unsafe extern "C" fn logger_start(
    mut self_0: *mut ILogger,
    mut segment: *const ::core::ffi::c_char,
) {
    (*self_0).indent_sds.expect("non-null function pointer")(
        self_0 as *mut ILogger,
        crate::bytesbuild!(segment),
    );
    (*self_0).log_sds.expect("non-null function pointer")(
        self_0 as *mut ILogger,
        (LOG_VL_PROGRESS as ::core::ffi::c_int
            + (*(self_0 as *mut Logger)).level as ::core::ffi::c_int) as u8,
        LoggerType::Progress,
        crate::bytesbuild!(b"Begin"),
    );
}
unsafe extern "C" fn logger_start_sds(mut self_0: *mut ILogger, mut segment: Vec<u8>) {
    (*self_0).indent_sds.expect("non-null function pointer")(self_0 as *mut ILogger, segment);
    (*self_0).log_sds.expect("non-null function pointer")(
        self_0 as *mut ILogger,
        (LOG_VL_PROGRESS as ::core::ffi::c_int
            + (*(self_0 as *mut Logger)).level as ::core::ffi::c_int) as u8,
        LoggerType::Progress,
        crate::bytesbuild!(b"Begin"),
    );
}
unsafe extern "C" fn logger_log(
    mut self_0: *mut ILogger,
    mut verbosity: u8,
    mut type_0: LoggerType,
    mut data: *const ::core::ffi::c_char,
) {
    (*self_0).log_sds.expect("non-null function pointer")(
        self_0 as *mut ILogger,
        verbosity,
        type_0,
        crate::bytesbuild!(data),
    );
}
unsafe extern "C" fn logger_log_sds(
    mut _self: *mut ILogger,
    mut verbosity: u8,
    mut type_0: LoggerType,
    mut data: Vec<u8>,
) {
    let mut self_0: *mut Logger = _self as *mut Logger;
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
        (*(*_self).get_target.expect("non-null function pointer")(_self as *mut ILogger))
            .push
            .expect("non-null function pointer")(
            (*self_0).target as *mut ILoggerTarget,
            demand,
        );
        (*self_0).last_logged_level = (*self_0).level;
    }
    // else `demand` just drops.
}
unsafe extern "C" fn logger_get_target(mut _self: *mut ILogger) -> *mut ILoggerTarget {
    let mut self_0: *mut Logger = _self as *mut Logger;
    return (*self_0).target as *mut ILoggerTarget;
}
unsafe extern "C" fn logger_set_verbosity(mut _self: *mut ILogger, mut verbosity: u8) {
    let mut self_0: *mut Logger = _self as *mut Logger;
    (*self_0).verbosity_limit = verbosity;
}
#[inline]
unsafe extern "C" fn logger_dispose(mut _self: *mut ILogger) {
    let mut self_0: *mut Logger = _self as *mut Logger;
    if self_0.is_null() {
        return;
    }
    let mut target: *mut ILoggerTarget =
        (*_self).get_target.expect("non-null function pointer")(_self as *mut ILogger);
    (*target).dispose.expect("non-null function pointer")(target as *mut ILoggerTarget);
    // Runs `Vec<Vec<u8>>`'s own destructor (frees every segment's backing
    // buffer, then the outer `Vec`'s) before the raw `free()` below drops
    // the malloc'd `Logger` struct out from under it -- matches the old
    // per-entry `sdsfree` loop plus the final `free(indents)`.
    drop(::core::mem::take(&mut (*self_0).indents));
    free(self_0 as *mut ::core::ffi::c_void);
    self_0 = ::core::ptr::null_mut::<Logger>();
}
pub static VTABLE_LOGGER: ILogger = {
    ILogger {
        dispose: Some(logger_dispose as unsafe extern "C" fn(*mut ILogger) -> ()),
        indent: Some(
            logger_indent
                as unsafe extern "C" fn(*mut ILogger, *const ::core::ffi::c_char) -> (),
        ),
        indent_sds: Some(logger_indent_sds as unsafe extern "C" fn(*mut ILogger, Vec<u8>) -> ()),
        start: Some(
            logger_start
                as unsafe extern "C" fn(*mut ILogger, *const ::core::ffi::c_char) -> (),
        ),
        start_sds: Some(logger_start_sds as unsafe extern "C" fn(*mut ILogger, Vec<u8>) -> ()),
        log: Some(
            logger_log
                as unsafe extern "C" fn(
                    *mut ILogger,
                    u8,
                    LoggerType,
                    *const ::core::ffi::c_char,
                ) -> (),
        ),
        log_sds: Some(
            logger_log_sds
                as unsafe extern "C" fn(*mut ILogger, u8, LoggerType, Vec<u8>) -> (),
        ),
        dedent: Some(logger_dedent as unsafe extern "C" fn(*mut ILogger) -> ()),
        finish: Some(logger_finish as unsafe extern "C" fn(*mut ILogger) -> ()),
        end: None,
        set_verbosity: Some(
            logger_set_verbosity as unsafe extern "C" fn(*mut ILogger, u8) -> (),
        ),
        get_target: Some(
            logger_get_target as unsafe extern "C" fn(*mut ILogger) -> *mut ILoggerTarget,
        ),
    }
};
pub unsafe fn otfcc_new_logger(
    mut target: *mut ILoggerTarget,
) -> *mut ILogger {
    let mut logger: *mut Logger = ::core::ptr::null_mut::<Logger>();
    logger = __caryll_allocate_clean(
        ::core::mem::size_of::<Logger>() as usize,
        120 as ::core::ffi::c_ulong,
    ) as *mut Logger;
    (*logger).target = target as *mut ::core::ffi::c_void;
    (*logger).vtable = VTABLE_LOGGER;
    // `__caryll_allocate_clean` calloc's the struct, which is not a valid
    // `Vec<Vec<u8>>` bit pattern (a zero-capacity `Vec` uses a dangling
    // *non-null* sentinel pointer, not a null one) -- must be assigned for
    // real, matching every other malloc'd-struct-plus-`Vec`-field
    // conversion in this migration.
    (*logger).indents = Vec::new();
    return logger as *mut ILogger;
}
trait LoggerTarget {
    unsafe fn dispose(self_: *mut ILoggerTarget);
    unsafe fn push(self_: *mut ILoggerTarget, data: Vec<u8>);
}
struct StderrLoggerTarget;
impl LoggerTarget for StderrLoggerTarget {
    unsafe fn dispose(mut _self: *mut ILoggerTarget) {
        let mut self_0: *mut StderrTarget = _self as *mut StderrTarget;
        if self_0.is_null() {
            return;
        }
        free(self_0 as *mut ::core::ffi::c_void);
        self_0 = ::core::ptr::null_mut::<StderrTarget>();
    }
    unsafe fn push(mut _self: *mut ILoggerTarget, mut data: Vec<u8>) {
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
}
pub unsafe extern "C" fn stderr_target_dispose(mut _self: *mut ILoggerTarget) {
    <StderrLoggerTarget as LoggerTarget>::dispose(_self);
}
pub unsafe extern "C" fn stderr_target_push(mut _self: *mut ILoggerTarget, mut data: Vec<u8>) {
    <StderrLoggerTarget as LoggerTarget>::push(_self, data);
}
pub static VTABLE_STDERR_TARGET: ILoggerTarget = {
    ILoggerTarget {
        dispose: Some(stderr_target_dispose as unsafe extern "C" fn(*mut ILoggerTarget) -> ()),
        push: Some(stderr_target_push as unsafe extern "C" fn(*mut ILoggerTarget, Vec<u8>) -> ()),
    }
};
pub unsafe fn otfcc_new_std_err_target() -> *mut ILoggerTarget {
    let mut target: *mut StderrTarget = ::core::ptr::null_mut::<StderrTarget>();
    target = __caryll_allocate_clean(
        ::core::mem::size_of::<StderrTarget>() as usize,
        146 as ::core::ffi::c_ulong,
    ) as *mut StderrTarget;
    (*target).vtable = VTABLE_STDERR_TARGET;
    return target as *mut ILoggerTarget;
}
struct EmptyLoggerTarget;
impl LoggerTarget for EmptyLoggerTarget {
    unsafe fn dispose(mut _self: *mut ILoggerTarget) {
        let mut self_0: *mut StderrTarget = _self as *mut StderrTarget;
        if self_0.is_null() {
            return;
        }
        free(self_0 as *mut ::core::ffi::c_void);
        self_0 = ::core::ptr::null_mut::<StderrTarget>();
    }
    unsafe fn push(mut _self: *mut ILoggerTarget, mut data: Vec<u8>) {
        drop(data);
    }
}
pub unsafe extern "C" fn empty_target_dispose(mut _self: *mut ILoggerTarget) {
    <EmptyLoggerTarget as LoggerTarget>::dispose(_self);
}
pub unsafe extern "C" fn empty_target_push(mut _self: *mut ILoggerTarget, mut data: Vec<u8>) {
    <EmptyLoggerTarget as LoggerTarget>::push(_self, data);
}
pub static VTABLE_EMPTY_TARGET: ILoggerTarget = {
    ILoggerTarget {
        dispose: Some(empty_target_dispose as unsafe extern "C" fn(*mut ILoggerTarget) -> ()),
        push: Some(empty_target_push as unsafe extern "C" fn(*mut ILoggerTarget, Vec<u8>) -> ()),
    }
};
pub unsafe fn otfcc_new_empty_target() -> *mut ILoggerTarget {
    let mut target: *mut StderrTarget = ::core::ptr::null_mut::<StderrTarget>();
    target = __caryll_allocate_clean(
        ::core::mem::size_of::<StderrTarget>() as usize,
        168 as ::core::ffi::c_ulong,
    ) as *mut StderrTarget;
    (*target).vtable = VTABLE_EMPTY_TARGET;
    return target as *mut ILoggerTarget;
}
