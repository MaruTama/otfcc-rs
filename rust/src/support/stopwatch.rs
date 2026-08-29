// `push_stopwatch` now returns `Vec<u8>`, its only caller a direct Rust
// call site (never a real FFI boundary) -- goes away with the vtable/
// extern "C" cleanup, same as every other instance of this allow.
#![allow(improper_ctypes_definitions)]
// `timespec` and the clock API come from `libc`, which describes the host.
// c2rust had copied glibc's `struct timespec` and its `__time_t` /
// `__syscall_slong_t` typedefs verbatim into every file that timed anything --
// the same mistake as the hand-copied `_IO_FILE`: it happens to have the right
// shape on x86_64 Linux and says nothing about any other target.
use libc::{CLOCK_REALTIME, clock_gettime, snprintf, time_t, timespec};

pub unsafe fn time_now(tv: *mut timespec) {
    unsafe { clock_gettime(CLOCK_REALTIME, tv) };
}
pub const BILLION: i32 = 1000000000_i32;
unsafe fn timespec_diff(
    start: *mut timespec,
    stop: *mut timespec,
    result: *mut timespec,
) {
    unsafe {
        if (*stop).tv_nsec - (*start).tv_nsec < 0 as ::core::ffi::c_long {
            (*result).tv_sec = (*stop).tv_sec - (*start).tv_sec - 1 as time_t;
            (*result).tv_nsec = (*stop).tv_nsec - (*start).tv_nsec + BILLION as ::core::ffi::c_long;
        } else {
            (*result).tv_sec = (*stop).tv_sec - (*start).tv_sec;
            (*result).tv_nsec = (*stop).tv_nsec - (*start).tv_nsec;
        };
    }
}
pub unsafe fn push_stopwatch(sofar: *mut timespec) -> Vec<u8> {
    let mut ends: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe { time_now(&raw mut ends) };
    let mut diff: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe { timespec_diff(sofar, &raw mut ends, &raw mut diff) };
    unsafe { *sofar = ends };
    // The one `%g` that ever reached `sdscatprintf`, and the only reason libc's
    // formatting is still called here: Rust has no `%g`, and this crate does not
    // format floats itself on purpose -- JSON numbers go through the vendored
    // `emyg_dtoa` so that their spelling matches the C build byte for byte.
    // Calling a variadic C function is stable Rust; only *defining* one needs
    // `feature(c_variadic)`, which is what the rest of this change removes.
    let mut secs: [::core::ffi::c_char; 32] = [0; 32];
    unsafe {
        snprintf(
            secs.as_mut_ptr(),
            ::core::mem::size_of_val(&secs),
            b"%g\0" as *const u8 as *const ::core::ffi::c_char,
            diff.tv_sec as ::core::ffi::c_double
                + diff.tv_nsec as ::core::ffi::c_double / BILLION as ::core::ffi::c_double,
        );
    }
    return crate::bytesbuild!(
        b"Step time = ",
        secs.as_ptr() as *const ::core::ffi::c_char,
        b"s.\n",
    );
}
