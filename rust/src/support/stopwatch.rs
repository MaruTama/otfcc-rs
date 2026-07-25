// `timespec` and the clock API come from `libc`, which describes the host.
// c2rust had copied glibc's `struct timespec` and its `__time_t` /
// `__syscall_slong_t` typedefs verbatim into every file that timed anything --
// the same mistake as the hand-copied `_IO_FILE`: it happens to have the right
// shape on x86_64 Linux and says nothing about any other target.
use libc::{clock_gettime, time_t, timespec, CLOCK_REALTIME};

use crate::vendor::sds::{sds};
extern "C" {
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
}
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
    if (*stop).tv_nsec - (*start).tv_nsec < 0 as ::core::ffi::c_long {
        (*result).tv_sec = (*stop).tv_sec - (*start).tv_sec - 1 as time_t;
        (*result).tv_nsec = (*stop).tv_nsec - (*start).tv_nsec + BILLION as ::core::ffi::c_long;
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
