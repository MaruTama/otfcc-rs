#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use std::io::Write;

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
                let mut stderr = std::io::stderr();
                let _ = stderr.write_all(&data);
                if data.last() != Some(&b'\n') {
                    let _ = stderr.write_all(b"\n");
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
impl Logger {
    // Was `otfcc_new_logger`: a separate heap allocation (`__caryll_
    // allocate_clean`) that `Options` held behind a `*mut Logger`. Now that
    // `Options` owns its `Logger` inline (see `support/options.rs`'s
    // `logger: RefCell<Logger>`), construction is just building the value --
    // no allocation, no calloc-then-`ptr::write` dance needed here (that
    // hazard now lives at the one place that *does* still calloc a whole
    // struct in one shot: `otfcc_new_options`).
    pub fn new(target: LoggerTarget) -> Logger {
        Logger {
            target,
            level: 0,
            last_logged_level: 0,
            indents: Vec::new(),
            verbosity_limit: 0,
        }
    }
}
impl Default for Logger {
    // Lets `Options` (and tests that need a throwaway `Options`) derive
    // `Default` too, instead of every call site spelling out `Logger::new(
    // LoggerTarget::Empty)` -- or, worse, reaching for `mem::zeroed()`,
    // which is *not* a valid `Logger` (`indents: Vec<Vec<u8>>` needs a
    // dangling non-null sentinel at zero capacity, not an all-zero one).
    fn default() -> Self {
        Logger::new(LoggerTarget::Empty)
    }
}
pub static OTFCC_LOGGER_TYPE_NAMES: [&::core::ffi::CStr; 3] = [c"[ERROR]", c"[WARNING]", c"[NOTE]"];
pub unsafe fn logger_indent(_self: &mut Logger, mut segment: *const ::core::ffi::c_char) {
    logger_indent_sds(_self, crate::bytesbuild!(segment));
}
pub unsafe fn logger_indent_sds(self_0: &mut Logger, mut segment: Vec<u8>) {
    self_0.indents.push(segment);
    self_0.level = self_0.indents.len() as u16;
}
pub unsafe fn logger_dedent(self_0: &mut Logger) {
    if self_0.level == 0 {
        return;
    }
    self_0.indents.pop();
    self_0.level = (self_0.level as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as u16;
    if (self_0.level as ::core::ffi::c_int) < self_0.last_logged_level as ::core::ffi::c_int {
        self_0.last_logged_level = self_0.level;
    }
}
pub unsafe fn logger_finish(self_0: &mut Logger) {
    logger_log_sds(
        self_0,
        (LOG_VL_PROGRESS as ::core::ffi::c_int + self_0.level as ::core::ffi::c_int) as u8,
        LoggerType::Progress,
        crate::bytesbuild!(b"Finish"),
    );
    logger_dedent(self_0);
}
pub unsafe fn logger_start_sds(self_0: &mut Logger, segment: Vec<u8>) {
    logger_indent_sds(self_0, segment);
    logger_log_sds(
        self_0,
        (LOG_VL_PROGRESS as ::core::ffi::c_int + self_0.level as ::core::ffi::c_int) as u8,
        LoggerType::Progress,
        crate::bytesbuild!(b"Begin"),
    );
}
pub unsafe fn logger_log_sds(
    self_0: &mut Logger,
    mut verbosity: u8,
    mut type_0: LoggerType,
    mut data: Vec<u8>,
) {
    let mut demand: Vec<u8> = Vec::new();
    let mut level: u16 = 0 as u16;
    while (level as ::core::ffi::c_int) < self_0.level as ::core::ffi::c_int {
        if (level as ::core::ffi::c_int)
            < self_0.last_logged_level as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            let seg_len = (&self_0.indents)[level as usize].len();
            let mut j: usize = 0 as usize;
            while j < seg_len {
                demand.extend_from_slice(b" ");
                j = j.wrapping_add(1);
            }
            if (level as ::core::ffi::c_int)
                < self_0.last_logged_level as ::core::ffi::c_int - 2 as ::core::ffi::c_int
            {
                demand.extend_from_slice(b" | ");
            } else {
                demand.extend_from_slice(b" |-");
            }
        } else {
            demand.extend_from_slice(&(&self_0.indents)[level as usize]);
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
    if verbosity as ::core::ffi::c_int <= self_0.verbosity_limit as ::core::ffi::c_int {
        self_0.target.push(demand);
        self_0.last_logged_level = self_0.level;
    }
    // else `demand` just drops.
}
pub unsafe fn logger_set_verbosity(self_0: &mut Logger, verbosity: u8) {
    self_0.verbosity_limit = verbosity;
}
pub unsafe fn otfcc_new_std_err_target() -> LoggerTarget {
    LoggerTarget::Stderr
}
pub unsafe fn otfcc_new_empty_target() -> LoggerTarget {
    LoggerTarget::Empty
}
