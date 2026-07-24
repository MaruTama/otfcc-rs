extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
}
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type sds = *mut ::core::ffi::c_char;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn time_now(mut tv: *mut timespec) {
    clock_gettime(CLOCK_REALTIME, tv);
}
pub const BILLION: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
unsafe extern "C" fn timespec_diff(
    mut start: *mut timespec,
    mut stop: *mut timespec,
    mut result: *mut timespec,
) {
    if (*stop).tv_nsec - (*start).tv_nsec < 0 as __syscall_slong_t {
        (*result).tv_sec = (*stop).tv_sec - (*start).tv_sec - 1 as __time_t;
        (*result).tv_nsec = (*stop).tv_nsec - (*start).tv_nsec + BILLION as __syscall_slong_t;
    } else {
        (*result).tv_sec = (*stop).tv_sec - (*start).tv_sec;
        (*result).tv_nsec = (*stop).tv_nsec - (*start).tv_nsec;
    };
}
#[no_mangle]
pub unsafe extern "C" fn push_stopwatch(mut sofar: *mut timespec) -> sds {
    let mut ends: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    time_now(&raw mut ends);
    let mut diff: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    timespec_diff(sofar, &raw mut ends, &raw mut diff);
    *sofar = ends;
    let mut log: sds = sdscatprintf(
        sdsempty(),
        b"Step time = %gs.\n\0" as *const u8 as *const ::core::ffi::c_char,
        diff.tv_sec as ::core::ffi::c_double
            + diff.tv_nsec as ::core::ffi::c_double / BILLION as ::core::ffi::c_double,
    );
    return log;
}
