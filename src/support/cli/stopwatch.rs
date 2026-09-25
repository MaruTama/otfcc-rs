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

// `tv` is a `&mut timespec` rather than a `*mut`: the only reason the old
// signature was raw is that its callers wrote `&raw mut begin`. The one
// genuinely unsafe operation is the `clock_gettime` FFI call itself, whose
// only requirement (a valid, writable `timespec`) a `&mut` satisfies by
// construction -- so that call is the whole of the `unsafe` block, and this
// function is safe. (The `%g` formatting further down is a separate,
// deliberately deferred question and is untouched.)
pub fn time_now(tv: &mut timespec) {
    unsafe { clock_gettime(CLOCK_REALTIME, tv) };
}
pub const BILLION: i32 = 1000000000_i32;
fn timespec_diff(start: &timespec, stop: &timespec, result: &mut timespec) {
    if stop.tv_nsec - start.tv_nsec < 0 as ::core::ffi::c_long {
        result.tv_sec = stop.tv_sec - start.tv_sec - 1 as time_t;
        result.tv_nsec = stop.tv_nsec - start.tv_nsec + BILLION as ::core::ffi::c_long;
    } else {
        result.tv_sec = stop.tv_sec - start.tv_sec;
        result.tv_nsec = stop.tv_nsec - start.tv_nsec;
    };
}
pub fn push_stopwatch(sofar: &mut timespec) -> Vec<u8> {
    let mut ends: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    time_now(&mut ends);
    let mut diff: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    timespec_diff(sofar, &ends, &mut diff);
    *sofar = ends;
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
    // `secs` is already a fully owned, fixed-size local array -- `snprintf`
    // only ever writes a NUL-terminated `%g` rendering of a float into it, so
    // finding that terminator and slicing up to it is a plain byte scan over
    // data this function already owns outright. The old code instead handed
    // `secs.as_ptr()` to `CCharRef::from_ptr`, whose raw-pointer + `strlen`
    // unsafe path exists for a genuinely unknown-provenance `*const c_char`
    // (see that function's own doc comment) -- not this stack array, whose
    // length and NUL-termination are both already guaranteed here. Same
    // truncate-at-first-NUL technique `support/fmt.rs`'s own `&Vec<u8>`
    // `SdsPart` impl already uses for the same reason.
    let secs_bytes = secs.map(|c| c as u8);
    let nul_pos = secs_bytes
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(secs_bytes.len());
    return crate::bytesbuild!(b"Step time = ", &secs_bytes[..nul_pos], b"s.\n",);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Pins the exact "Step time = <g-formatted number>s.\n" shape this
    // stage's own conversion (reading `secs` directly instead of through
    // `CCharRef::from_ptr`) must keep producing byte for byte.
    #[test]
    #[cfg_attr(
        miri,
        ignore = "calls libc::clock_gettime/snprintf, unsupported under Miri"
    )]
    fn push_stopwatch_formats_step_time_and_advances_sofar() {
        let mut sofar = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        time_now(&mut sofar);
        // Move `sofar` back by exactly half a second so the elapsed time
        // `push_stopwatch` reports is deterministic and libc's `%g` always
        // renders it the same way, regardless of how fast this test runs.
        sofar.tv_nsec -= BILLION as ::core::ffi::c_long / 2;
        if sofar.tv_nsec < 0 {
            sofar.tv_sec -= 1;
            sofar.tv_nsec += BILLION as ::core::ffi::c_long;
        }
        let out = push_stopwatch(&mut sofar);
        let text = String::from_utf8(out).expect("only ASCII digits/'.'/'e'/'-' expected");
        assert!(
            text.starts_with("Step time = 0.5"),
            "expected a ~0.5s reading, got {text:?}"
        );
        assert!(text.ends_with("s.\n"), "got {text:?}");
        // No trailing NUL/garbage from the rest of the 32-byte `secs` buffer
        // leaked past the terminator this stage now finds itself.
        assert!(!text.contains('\0'), "got {text:?}");
    }

    // A zero-length reading (`sofar` == now) still round-trips through the
    // same NUL-scan safely -- exercises the shortest possible `%g` output
    // this function's buffer can hold ("0").
    #[test]
    #[cfg_attr(
        miri,
        ignore = "calls libc::clock_gettime/snprintf, unsupported under Miri"
    )]
    fn push_stopwatch_handles_a_near_zero_reading() {
        let mut sofar = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        time_now(&mut sofar);
        let out = push_stopwatch(&mut sofar);
        let text = String::from_utf8(out).expect("only ASCII digits/'.'/'e'/'-' expected");
        assert!(text.starts_with("Step time = "), "got {text:?}");
        assert!(text.ends_with("s.\n"), "got {text:?}");
        assert!(!text.contains('\0'), "got {text:?}");
    }
}
