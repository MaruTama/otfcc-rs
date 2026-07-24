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
    fn sdsnew(init: *const ::core::ffi::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscat(s: sds, t: *const ::core::ffi::c_char) -> sds;
    fn sdscatfmt(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
}

use crate::src::lib::support::stdio::{FILE, stderr};
use crate::src::lib::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const log_vl_progress: C2RustUnnamed = 10;
pub const log_vl_info: C2RustUnnamed = 5;
pub const log_vl_notice: C2RustUnnamed = 2;
pub const log_vl_important: C2RustUnnamed = 1;
pub const log_vl_critical: C2RustUnnamed = 0;
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
pub struct Logger {
    pub vtable: otfcc_ILogger,
    pub target: *mut ::core::ffi::c_void,
    pub level: uint16_t,
    pub lastLoggedLevel: uint16_t,
    pub levelCap: uint16_t,
    pub indents: *mut sds,
    pub verbosityLimit: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StderrTarget {
    pub vtable: otfcc_ILoggerTarget,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SDS_TYPE_5: ::core::ffi::c_int = 0;
pub const SDS_TYPE_8: ::core::ffi::c_int = 1;
pub const SDS_TYPE_16: ::core::ffi::c_int = 2;
pub const SDS_TYPE_32: ::core::ffi::c_int = 3;
pub const SDS_TYPE_64: ::core::ffi::c_int = 4;
pub const SDS_TYPE_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SDS_TYPE_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as size_t,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as size_t;
        }
        _ => {}
    }
    return 0 as size_t;
}
#[no_mangle]
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
    let mut newLevel: uint8_t =
        ((*self_0).level as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
    if newLevel as ::core::ffi::c_int > (*self_0).levelCap as ::core::ffi::c_int {
        (*self_0).levelCap = ((*self_0).levelCap as ::core::ffi::c_int
            + ((*self_0).levelCap as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int)) as uint16_t;
        (*self_0).indents = __caryll_reallocate(
            (*self_0).indents as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<sds>() as size_t).wrapping_mul((*self_0).levelCap as size_t),
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
    (*self_0).level = ((*self_0).level as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint16_t;
    if ((*self_0).level as ::core::ffi::c_int) < (*self_0).lastLoggedLevel as ::core::ffi::c_int {
        (*self_0).lastLoggedLevel = (*self_0).level;
    }
}
unsafe extern "C" fn loggerFinish(mut self_0: *mut otfcc_ILogger) {
    (*self_0).logSDS.expect("non-null function pointer")(
        self_0 as *mut otfcc_ILogger,
        (log_vl_progress as ::core::ffi::c_int
            + (*(self_0 as *mut Logger)).level as ::core::ffi::c_int) as uint8_t,
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
            + (*(self_0 as *mut Logger)).level as ::core::ffi::c_int) as uint8_t,
        log_type_progress,
        sdsnew(b"Begin\0" as *const u8 as *const ::core::ffi::c_char),
    );
}
unsafe extern "C" fn loggerStartSDS(mut self_0: *mut otfcc_ILogger, mut segment: sds) {
    (*self_0).indentSDS.expect("non-null function pointer")(self_0 as *mut otfcc_ILogger, segment);
    (*self_0).logSDS.expect("non-null function pointer")(
        self_0 as *mut otfcc_ILogger,
        (log_vl_progress as ::core::ffi::c_int
            + (*(self_0 as *mut Logger)).level as ::core::ffi::c_int) as uint8_t,
        log_type_progress,
        sdsnew(b"Begin\0" as *const u8 as *const ::core::ffi::c_char),
    );
}
unsafe extern "C" fn loggerLog(
    mut self_0: *mut otfcc_ILogger,
    mut verbosity: uint8_t,
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
    mut verbosity: uint8_t,
    mut type_0: otfcc_LoggerType,
    mut data: sds,
) {
    let mut self_0: *mut Logger = _self as *mut Logger;
    let mut demand: sds = sdsempty();
    let mut level: uint16_t = 0 as uint16_t;
    while (level as ::core::ffi::c_int) < (*self_0).level as ::core::ffi::c_int {
        if (level as ::core::ffi::c_int)
            < (*self_0).lastLoggedLevel as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            let mut j: size_t = 0 as size_t;
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
            demand = sdscatfmt(
                demand,
                b"%S : \0" as *const u8 as *const ::core::ffi::c_char,
                *(*self_0).indents.offset(level as isize),
            );
        }
        level = level.wrapping_add(1);
    }
    if (type_0 as ::core::ffi::c_uint) < 3 as ::core::ffi::c_uint {
        demand = sdscatfmt(
            demand,
            b"%s %S\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_LoggerTypeNames[type_0 as usize],
            data,
        );
    } else {
        demand = sdscatfmt(
            demand,
            b"%S\0" as *const u8 as *const ::core::ffi::c_char,
            data,
        );
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
unsafe extern "C" fn loggerSetVerbosity(mut _self: *mut otfcc_ILogger, mut verbosity: uint8_t) {
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
    let mut level: uint16_t = 0 as uint16_t;
    while (level as ::core::ffi::c_int) < (*self_0).level as ::core::ffi::c_int {
        sdsfree(*(*self_0).indents.offset(level as isize));
        level = level.wrapping_add(1);
    }
    free((*self_0).indents as *mut ::core::ffi::c_void);
    (*self_0).indents = ::core::ptr::null_mut::<sds>();
    free(self_0 as *mut ::core::ffi::c_void);
    self_0 = ::core::ptr::null_mut::<Logger>();
}
#[no_mangle]
pub static mut VTABLE_LOGGER: otfcc_ILogger = {
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
                    uint8_t,
                    otfcc_LoggerType,
                    *const ::core::ffi::c_char,
                ) -> (),
        ),
        logSDS: Some(
            loggerLogSDS
                as unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t, otfcc_LoggerType, sds) -> (),
        ),
        dedent: Some(loggerDedent as unsafe extern "C" fn(*mut otfcc_ILogger) -> ()),
        finish: Some(loggerFinish as unsafe extern "C" fn(*mut otfcc_ILogger) -> ()),
        end: None,
        setVerbosity: Some(
            loggerSetVerbosity as unsafe extern "C" fn(*mut otfcc_ILogger, uint8_t) -> (),
        ),
        getTarget: Some(
            loggerGetTarget as unsafe extern "C" fn(*mut otfcc_ILogger) -> *mut otfcc_ILoggerTarget,
        ),
    }
};
#[no_mangle]
pub unsafe extern "C" fn otfcc_newLogger(
    mut target: *mut otfcc_ILoggerTarget,
) -> *mut otfcc_ILogger {
    let mut logger: *mut Logger = ::core::ptr::null_mut::<Logger>();
    logger = __caryll_allocate_clean(
        ::core::mem::size_of::<Logger>() as size_t,
        120 as ::core::ffi::c_ulong,
    ) as *mut Logger;
    (*logger).target = target as *mut ::core::ffi::c_void;
    (*logger).vtable = VTABLE_LOGGER;
    return logger as *mut otfcc_ILogger;
}
#[no_mangle]
pub unsafe extern "C" fn stderrTargetDispose(mut _self: *mut otfcc_ILoggerTarget) {
    let mut self_0: *mut StderrTarget = _self as *mut StderrTarget;
    if self_0.is_null() {
        return;
    }
    free(self_0 as *mut ::core::ffi::c_void);
    self_0 = ::core::ptr::null_mut::<StderrTarget>();
}
#[no_mangle]
pub unsafe extern "C" fn stderrTargetPush(mut _self: *mut otfcc_ILoggerTarget, mut data: sds) {
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        data,
    );
    if *data.offset(sdslen(data).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        != '\n' as i32
    {
        fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    sdsfree(data);
}
#[no_mangle]
pub static mut VTABLE_STDERR_TARGET: otfcc_ILoggerTarget = {
    otfcc_ILoggerTarget {
        dispose: Some(stderrTargetDispose as unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()),
        push: Some(stderrTargetPush as unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()),
    }
};
#[no_mangle]
pub unsafe extern "C" fn otfcc_newStdErrTarget() -> *mut otfcc_ILoggerTarget {
    let mut target: *mut StderrTarget = ::core::ptr::null_mut::<StderrTarget>();
    target = __caryll_allocate_clean(
        ::core::mem::size_of::<StderrTarget>() as size_t,
        146 as ::core::ffi::c_ulong,
    ) as *mut StderrTarget;
    (*target).vtable = VTABLE_STDERR_TARGET;
    return target as *mut otfcc_ILoggerTarget;
}
#[no_mangle]
pub unsafe extern "C" fn emptyTargetDispose(mut _self: *mut otfcc_ILoggerTarget) {
    let mut self_0: *mut StderrTarget = _self as *mut StderrTarget;
    if self_0.is_null() {
        return;
    }
    free(self_0 as *mut ::core::ffi::c_void);
    self_0 = ::core::ptr::null_mut::<StderrTarget>();
}
#[no_mangle]
pub unsafe extern "C" fn emptyTargetPush(mut _self: *mut otfcc_ILoggerTarget, mut data: sds) {
    sdsfree(data);
}
#[no_mangle]
pub static mut VTABLE_EMPTY_TARGET: otfcc_ILoggerTarget = {
    otfcc_ILoggerTarget {
        dispose: Some(emptyTargetDispose as unsafe extern "C" fn(*mut otfcc_ILoggerTarget) -> ()),
        push: Some(emptyTargetPush as unsafe extern "C" fn(*mut otfcc_ILoggerTarget, sds) -> ()),
    }
};
#[no_mangle]
pub unsafe extern "C" fn otfcc_newEmptyTarget() -> *mut otfcc_ILoggerTarget {
    let mut target: *mut StderrTarget = ::core::ptr::null_mut::<StderrTarget>();
    target = __caryll_allocate_clean(
        ::core::mem::size_of::<StderrTarget>() as size_t,
        168 as ::core::ffi::c_ulong,
    ) as *mut StderrTarget;
    (*target).vtable = VTABLE_EMPTY_TARGET;
    return target as *mut otfcc_ILoggerTarget;
}
