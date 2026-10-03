//! The "Step time" lines `--verbose` prints after each stage of the two
//! binaries.
//!
//! Was `libc::clock_gettime(CLOCK_REALTIME)` into a `timespec` and
//! `libc::snprintf("%g")` into a fixed buffer. Elapsed time is now an
//! `Instant` (monotonic, so a clock adjustment mid-run cannot produce a
//! negative step), and the number is formatted by `support::fmt::format_g`,
//! the same `%g` reimplementation the CFF writer uses.
use std::time::Instant;

use crate::support::fmt::format_g;

/// The time since `sofar`, as `Step time = <%g seconds>s.\n`; `sofar` moves
/// on to now, so the next call measures the next step.
pub fn push_stopwatch(sofar: &mut Instant) -> Vec<u8> {
    let now = Instant::now();
    let secs = now.duration_since(*sofar).as_secs_f64();
    *sofar = now;
    format!("Step time = {}s.\n", format_g(secs, 6)).into_bytes()
}
pub fn log_step_time(sofar: &mut Instant) {
    tracing::debug!("{}", crate::logger::ByteStr(&push_stopwatch(sofar)[..]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn push_stopwatch_formats_step_time_and_advances_sofar() {
        let start = Instant::now();
        // Half a second ago, so the reading is deterministic to %g's six
        // significant digits however fast this test runs... almost: allow
        // for the few microseconds between the two `Instant::now()` calls.
        let mut sofar = start.checked_sub(Duration::from_millis(500)).unwrap();
        let text = String::from_utf8(push_stopwatch(&mut sofar)).unwrap();
        assert!(text.starts_with("Step time = 0.5"), "expected a ~0.5s reading, got {text:?}");
        assert!(text.ends_with("s.\n"), "got {text:?}");
        assert!(sofar >= start);
    }

    #[test]
    fn push_stopwatch_handles_a_near_zero_reading() {
        let mut sofar = Instant::now();
        let text = String::from_utf8(push_stopwatch(&mut sofar)).unwrap();
        assert!(text.starts_with("Step time = "), "got {text:?}");
        assert!(text.ends_with("s.\n"), "got {text:?}");
    }
}
