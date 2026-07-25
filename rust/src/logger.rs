#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{fprintf, free};
unsafe extern "C" {
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscat(s: sds, t: *const ::core::ffi::c_char) -> sds;
}

use crate::support::stdio::{stderr};
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, Sds, sds, sdshdr16, sdshdr32, sdshdr64, sdshdr8};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_ILoggerTarget {
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()>,
    pub push: Option<unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()>,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum otfcc_LoggerType {
    log_type_error = 0,
    log_type_warning = 1,
    log_type_info = 2,
    log_type_progress = 3,
}
pub use otfcc_LoggerType::*;
pub type otfcc_LoggerVerbosity = ::core::ffi::c_uint;
pub const log_vl_progress: otfcc_LoggerVerbosity = 10;
pub const log_vl_info: otfcc_LoggerVerbosity = 5;
pub const log_vl_notice: otfcc_LoggerVerbosity = 2;
pub const log_vl_important: otfcc_LoggerVerbosity = 1;
pub const log_vl_critical: otfcc_LoggerVerbosity = 0;
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
            u8,
            otfcc_LoggerType,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub logSDS:
        Option<unsafe extern "C" fn(*mut otfcc_ILogger, u8, otfcc_LoggerType, sds) -> ()>,
    pub dedent: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub end: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> ()>,
    pub setVerbosity: Option<unsafe extern "C" fn(*mut otfcc_ILogger, u8) -> ()>,
    pub getTarget: Option<unsafe extern "C" fn(*mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Logger {
    pub vtable: otfcc_ILogger,
    pub target: *mut ::core::ffi::c_void,
    pub level: u16,
    pub lastLoggedLevel: u16,
    pub levelCap: u16,
    pub indents: *mut sds,
    pub verbosityLimit: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StderrTarget {
    pub vtable: otfcc_ILoggerTarget,
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
#[unsafe(no_mangle)]
pub static mut otfcc_LoggerTypeNames: [*const ::core::ffi::c_char; 3] = [
    b"[ERROR]\0" as *const u8 as *const ::core::ffi::c_char,
    b"[WARNING]\0" as *const u8 as *const ::core::ffi::c_char,
    b"[NOTE]\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn loggerIndent(
    mut _self: *mut otfcc_ILogger,
    mut segment: *const ::core::ffi::c_char,
) {
    (*_self).indentSDS.expect("non-null function pointer")(
        _self as *mut otfcc_ILogger,
        sdsnew(segment),
    );
}
unsafe extern "C" fn loggerIndentSDS(mut _self: *mut otfcc_ILogger, mut segment: sds) {
    let mut self_0: *mut Logger = _self as *mut Logger;
    let mut newLevel: u8 =
        ((*self_0).level as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u8;
    if newLevel as ::core::ffi::c_int > (*self_0).levelCap as ::core::ffi::c_int {
        (*self_0).levelCap = ((*self_0).levelCap as ::core::ffi::c_int
            + ((*self_0).levelCap as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)) as u16;
        (*self_0).indents = __caryll_reallocate(
            (*self_0).indents as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<sds>() as usize).wrapping_mul((*self_0).levelCap as usize),
            24 as ::core::ffi::c_ulong,
        ) as *mut sds;
    }
    (*self_0).level = (*self_0).level.wrapping_add(1);
    let ref mut fresh0 = *(*self_0)
        .indents
        .offset(((*self_0).level as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
    *fresh0 = segment;
}
unsafe extern "C" fn loggerDedent(mut _self: *mut otfcc_ILogger) {
    let mut self_0: *mut Logger = _self as *mut Logger;
    if (*self_0).level == 0 {
        return;
    }
    sdsfree(
        *(*self_0)
            .indents
            .offset(((*self_0).level as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize),
    );
    (*self_0).level = ((*self_0).level as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u16;
    if ((*self_0).level as ::core::ffi::c_int) < (*self_0).lastLoggedLevel as ::core::ffi::c_int {
        (*self_0).lastLoggedLevel = (*self_0).level;
    }
}
unsafe extern "C" fn loggerFinish(mut self_0: *mut otfcc_ILogger) {
    (*self_0).logSDS.expect("non-null function pointer")(
        self_0 as *mut otfcc_ILogger,
        (log_vl_progress as ::core::ffi::c_int
            + (*(self_0 as *mut Logger)).level as ::core::ffi::c_int) as u8,
        log_type_progress,
        sdsnew(b"Finish\0" as *const u8 as *const ::core::ffi::c_char),
    );
    (*self_0).dedent.expect("non-null function pointer")(self_0 as *mut otfcc_ILogger);
}
unsafe extern "C" fn loggerStart(
    mut self_0: *mut otfcc_ILogger,
    mut segment: *const ::core::ffi::c_char,
) {
    (*self_0).indentSDS.expect("non-null function pointer")(
        self_0 as *mut otfcc_ILogger,
        sdsnew(segment),
    );
    (*self_0).logSDS.expect("non-null function pointer")(
        self_0 as *mut otfcc_ILogger,
        (log_vl_progress as ::core::ffi::c_int
            + (*(self_0 as *mut Logger)).level as ::core::ffi::c_int) as u8,
        log_type_progress,
        sdsnew(b"Begin\0" as *const u8 as *const ::core::ffi::c_char),
    );
}
unsafe extern "C" fn loggerStartSDS(mut self_0: *mut otfcc_ILogger, mut segment: sds) {
    (*self_0).indentSDS.expect("non-null function pointer")(self_0 as *mut otfcc_ILogger, segment);
    (*self_0).logSDS.expect("non-null function pointer")(
        self_0 as *mut otfcc_ILogger,
        (log_vl_progress as ::core::ffi::c_int
            + (*(self_0 as *mut Logger)).level as ::core::ffi::c_int) as u8,
        log_type_progress,
        sdsnew(b"Begin\0" as *const u8 as *const ::core::ffi::c_char),
    );
}
unsafe extern "C" fn loggerLog(
    mut self_0: *mut otfcc_ILogger,
    mut verbosity: u8,
    mut type_0: otfcc_LoggerType,
    mut data: *const ::core::ffi::c_char,
) {
    (*self_0).logSDS.expect("non-null function pointer")(
        self_0 as *mut otfcc_ILogger,
        verbosity,
        type_0,
        sdsnew(data),
    );
}
unsafe extern "C" fn loggerLogSDS(
    mut _self: *mut otfcc_ILogger,
    mut verbosity: u8,
    mut type_0: otfcc_LoggerType,
    mut data: sds,
) {
    let mut self_0: *mut Logger = _self as *mut Logger;
    let mut demand: sds = sdsempty();
    let mut level: u16 = 0 as u16;
    while (level as ::core::ffi::c_int) < (*self_0).level as ::core::ffi::c_int {
        if (level as ::core::ffi::c_int)
            < (*self_0).lastLoggedLevel as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            let mut j: usize = 0 as usize;
            while j < sdslen(*(*self_0).indents.offset(level as isize)) {
                demand = sdscat(demand, b" \0" as *const u8 as *const ::core::ffi::c_char);
                j = j.wrapping_add(1);
            }
            if (level as ::core::ffi::c_int)
                < (*self_0).lastLoggedLevel as ::core::ffi::c_int - 2 as ::core::ffi::c_int
            {
                demand = sdscat(demand, b" | \0" as *const u8 as *const ::core::ffi::c_char);
            } else {
                demand = sdscat(demand, b" |-\0" as *const u8 as *const ::core::ffi::c_char);
            }
        } else {
            demand = crate::sdsbuild!(demand, Sds(*(*self_0).indents.offset(level as isize)), b" : ");
        }
        level = level.wrapping_add(1);
    }
    if (type_0 as ::core::ffi::c_uint) < 3 as ::core::ffi::c_uint {
        demand = crate::sdsbuild!(demand, otfcc_LoggerTypeNames[type_0 as usize], b" ", Sds(data));
    } else {
        demand = crate::sdsbuild!(demand, Sds(data));
    }
    sdsfree(data);
    if verbosity as ::core::ffi::c_int <= (*self_0).verbosityLimit as ::core::ffi::c_int {
        (*(*_self).getTarget.expect("non-null function pointer")(_self as *mut otfcc_ILogger))
            .push
            .expect("non-null function pointer")(
            (*self_0).target as *mut otfcc_ILoggerTarget,
            demand,
        );
        (*self_0).lastLoggedLevel = (*self_0).level;
    } else {
        sdsfree(demand);
    };
}
unsafe extern "C" fn loggerGetTarget(mut _self: *mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget {
    let mut self_0: *mut Logger = _self as *mut Logger;
    return (*self_0).target as *mut otfcc_ILoggerTarget;
}
unsafe extern "C" fn loggerSetVerbosity(mut _self: *mut otfcc_ILogger, mut verbosity: u8) {
    let mut self_0: *mut Logger = _self as *mut Logger;
    (*self_0).verbosityLimit = verbosity;
}
#[inline]
unsafe extern "C" fn loggerDispose(mut _self: *mut otfcc_ILogger) {
    let mut self_0: *mut Logger = _self as *mut Logger;
    if self_0.is_null() {
        return;
    }
    let mut target: *mut otfcc_ILoggerTarget =
        (*_self).getTarget.expect("non-null function pointer")(_self as *mut otfcc_ILogger);
    (*target).dispose.expect("non-null function pointer")(target as *mut otfcc_ILoggerTarget);
    let mut level: u16 = 0 as u16;
    while (level as ::core::ffi::c_int) < (*self_0).level as ::core::ffi::c_int {
        sdsfree(*(*self_0).indents.offset(level as isize));
        level = level.wrapping_add(1);
    }
    free((*self_0).indents as *mut ::core::ffi::c_void);
    (*self_0).indents = ::core::ptr::null_mut::<sds>();
    free(self_0 as *mut ::core::ffi::c_void);
    self_0 = ::core::ptr::null_mut::<Logger>();
}
#[unsafe(no_mangle)]
pub static VTABLE_LOGGER: otfcc_ILogger = {
    otfcc_ILogger {
        dispose: Some(loggerDispose as unsafe extern "C" fn(*mut otfcc_ILogger) -> ()),
        indent: Some(
            loggerIndent
                as unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> (),
        ),
        indentSDS: Some(loggerIndentSDS as unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()),
        start: Some(
            loggerStart
                as unsafe extern "C" fn(*mut otfcc_ILogger, *const ::core::ffi::c_char) -> (),
        ),
        startSDS: Some(loggerStartSDS as unsafe extern "C" fn(*mut otfcc_ILogger, sds) -> ()),
        log: Some(
            loggerLog
                as unsafe extern "C" fn(
                    *mut otfcc_ILogger,
                    u8,
                    otfcc_LoggerType,
                    *const ::core::ffi::c_char,
                ) -> (),
        ),
        logSDS: Some(
            loggerLogSDS
                as unsafe extern "C" fn(*mut otfcc_ILogger, u8, otfcc_LoggerType, sds) -> (),
        ),
        dedent: Some(loggerDedent as unsafe extern "C" fn(*mut otfcc_ILogger) -> ()),
        finish: Some(loggerFinish as unsafe extern "C" fn(*mut otfcc_ILogger) -> ()),
        end: None,
        setVerbosity: Some(
            loggerSetVerbosity as unsafe extern "C" fn(*mut otfcc_ILogger, u8) -> (),
        ),
        getTarget: Some(
            loggerGetTarget as unsafe extern "C" fn(*mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget,
        ),
    }
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_newLogger(
    mut target: *mut otfcc_ILoggerTarget,
) -> *mut otfcc_ILogger {
    let mut logger: *mut Logger = ::core::ptr::null_mut::<Logger>();
    logger = __caryll_allocate_clean(
        ::core::mem::size_of::<Logger>() as usize,
        120 as ::core::ffi::c_ulong,
    ) as *mut Logger;
    (*logger).target = target as *mut ::core::ffi::c_void;
    (*logger).vtable = VTABLE_LOGGER;
    return logger as *mut otfcc_ILogger;
}
trait LoggerTarget {
    unsafe fn dispose(self_: *mut otfcc_ILoggerTarget);
    unsafe fn push(self_: *mut otfcc_ILoggerTarget, data: sds);
}
struct StderrLoggerTarget;
impl LoggerTarget for StderrLoggerTarget {
    unsafe fn dispose(mut _self: *mut otfcc_ILoggerTarget) {
        let mut self_0: *mut StderrTarget = _self as *mut StderrTarget;
        if self_0.is_null() {
            return;
        }
        free(self_0 as *mut ::core::ffi::c_void);
        self_0 = ::core::ptr::null_mut::<StderrTarget>();
    }
    unsafe fn push(mut _self: *mut otfcc_ILoggerTarget, mut data: sds) {
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            data,
        );
        if *data.offset(sdslen(data).wrapping_sub(1 as usize) as isize) as ::core::ffi::c_int
            != '\n' as i32
        {
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
        sdsfree(data);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stderrTargetDispose(mut _self: *mut otfcc_ILoggerTarget) {
    <StderrLoggerTarget as LoggerTarget>::dispose(_self);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn stderrTargetPush(mut _self: *mut otfcc_ILoggerTarget, mut data: sds) {
    <StderrLoggerTarget as LoggerTarget>::push(_self, data);
}
#[unsafe(no_mangle)]
pub static VTABLE_STDERR_TARGET: otfcc_ILoggerTarget = {
    otfcc_ILoggerTarget {
        dispose: Some(stderrTargetDispose as unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()),
        push: Some(stderrTargetPush as unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()),
    }
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_newStdErrTarget() -> *mut otfcc_ILoggerTarget {
    let mut target: *mut StderrTarget = ::core::ptr::null_mut::<StderrTarget>();
    target = __caryll_allocate_clean(
        ::core::mem::size_of::<StderrTarget>() as usize,
        146 as ::core::ffi::c_ulong,
    ) as *mut StderrTarget;
    (*target).vtable = VTABLE_STDERR_TARGET;
    return target as *mut otfcc_ILoggerTarget;
}
struct EmptyLoggerTarget;
impl LoggerTarget for EmptyLoggerTarget {
    unsafe fn dispose(mut _self: *mut otfcc_ILoggerTarget) {
        let mut self_0: *mut StderrTarget = _self as *mut StderrTarget;
        if self_0.is_null() {
            return;
        }
        free(self_0 as *mut ::core::ffi::c_void);
        self_0 = ::core::ptr::null_mut::<StderrTarget>();
    }
    unsafe fn push(mut _self: *mut otfcc_ILoggerTarget, mut data: sds) {
        sdsfree(data);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emptyTargetDispose(mut _self: *mut otfcc_ILoggerTarget) {
    <EmptyLoggerTarget as LoggerTarget>::dispose(_self);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emptyTargetPush(mut _self: *mut otfcc_ILoggerTarget, mut data: sds) {
    <EmptyLoggerTarget as LoggerTarget>::push(_self, data);
}
#[unsafe(no_mangle)]
pub static VTABLE_EMPTY_TARGET: otfcc_ILoggerTarget = {
    otfcc_ILoggerTarget {
        dispose: Some(emptyTargetDispose as unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()),
        push: Some(emptyTargetPush as unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()),
    }
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn otfcc_newEmptyTarget() -> *mut otfcc_ILoggerTarget {
    let mut target: *mut StderrTarget = ::core::ptr::null_mut::<StderrTarget>();
    target = __caryll_allocate_clean(
        ::core::mem::size_of::<StderrTarget>() as usize,
        168 as ::core::ffi::c_ulong,
    ) as *mut StderrTarget;
    (*target).vtable = VTABLE_EMPTY_TARGET;
    return target as *mut otfcc_ILoggerTarget;
}
