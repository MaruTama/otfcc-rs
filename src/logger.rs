//! otfcc's progress and diagnostic log, built on `tracing`.
//!
//! Library code reports through `tracing` events and spans:
//! - `tracing::error!` / `warn!` / `info!` / `debug!` for messages, printed
//!   with the `[ERROR]` / `[WARNING]` / `[NOTE]` / no prefix respectively;
//! - [`stage`] for a named step: verbose mode prints `Begin` when it opens
//!   and `Finish` when [`StageGuard::finish`] is called (a stage left by an
//!   early return closes without one); [`indent`] for a named scope that
//!   only indents the lines inside it.
//!
//! [`OtfccTreeLayer`] renders those as otfcc's indented stderr format
//! (`otfccdump : Read SFNT : Begin`, ` |-`, ` | `). The CLI binaries install
//! it with [`install_stderr`]. With no subscriber installed (the cdylib/FFI
//! path and library tests) nothing is printed, and disabled messages are
//! never formatted.

use std::fmt;
use std::io::Write;
use std::sync::Mutex;

use crate::support::fmt::BytePart;

use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::subscriber::Interest;
use tracing::{Event, Level, Metadata};
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::registry::LookupSpan;

/// The four kinds of log line, in prefix order.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
enum LoggerType {
    Error = 0,
    Warning = 1,
    Info = 2,
    Progress = 3,
}
// How noisy a message is: a message prints when `verbosity <=
// verbosity_limit`, so these are thresholds on a scale, not members of a
// set -- and `Begin`/`Finish` lines do arithmetic on one (`LOG_VL_PROGRESS
// + level`, deeper nesting being more verbose), which an enum could not
// express.
const LOG_VL_CRITICAL: u8 = 0;
const LOG_VL_IMPORTANT: u8 = 1;
const LOG_VL_NOTICE: u8 = 2;
const LOG_VL_PROGRESS: u8 = 10;

static OTFCC_LOGGER_TYPE_NAMES: [&str; 3] = ["[ERROR]", "[WARNING]", "[NOTE]"];

/// Displays one piece of a log message the way `bytesbuild!` renders it
/// (via `BytePart`): a `&Vec<u8>` (a glyph name) is cut at its first NUL,
/// a `&[u8]`/`&[u8; N]` is written whole, and integers print in decimal.
/// Bytes that are not valid UTF-8 show as U+FFFD. Rendered only when the
/// message is actually printed.
pub struct ByteStr<T>(pub T);

impl<T: BytePart + Copy> fmt::Display for ByteStr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes = Vec::new();
        self.0.append_to_vec(&mut bytes);
        fmt::Display::fmt(&String::from_utf8_lossy(&bytes), f)
    }
}

// Span names that `OtfccTreeLayer` treats as otfcc log scopes. Any other
// span (from a caller's own instrumentation) is ignored.
const STAGE_SPAN: &str = "otfcc_stage";
const INDENT_SPAN: &str = "otfcc_indent";

/// An open [`stage`] or [`indent`] scope. Dropping it closes the scope
/// silently; [`StageGuard::finish`] closes a stage with its `Finish` line.
#[must_use = "the scope closes as soon as this guard is dropped"]
pub struct StageGuard(tracing::span::EnteredSpan);

impl StageGuard {
    /// Closes a stage that completed, printing its `Finish` line in verbose
    /// mode. (A stage that is just dropped, e.g. by an early return on an
    /// error path, prints no `Finish`.)
    pub fn finish(self) {
        self.0.record("finished", true);
    }
}

impl fmt::Debug for StageGuard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("StageGuard").finish()
    }
}

/// Opens a named step: verbose output shows `<segment> : Begin` now and
/// `<segment> : Finish` when [`StageGuard::finish`] is called, and every
/// line logged in between is indented under `segment`.
pub fn stage(segment: impl fmt::Display) -> StageGuard {
    StageGuard(
        tracing::info_span!(STAGE_SPAN, segment = %segment, finished = tracing::field::Empty).entered(),
    )
}

/// Opens a named scope that indents every line logged inside it, without
/// `Begin`/`Finish` lines of its own.
pub fn indent(segment: impl fmt::Display) -> StageGuard {
    StageGuard(tracing::info_span!(INDENT_SPAN, segment = %segment).entered())
}

/// The indentation state machine: one segment per open scope, and
/// `last_logged_level` (the depth of the last line actually printed)
/// deciding which segments are written out in full and which are
/// abbreviated to ` |-` / ` | ` guides.
struct TreeFormatter {
    level: u16,
    last_logged_level: u16,
    indents: Vec<Vec<u8>>,
    verbosity_limit: u8,
}

impl TreeFormatter {
    fn new(verbosity_limit: u8) -> Self {
        TreeFormatter { level: 0, last_logged_level: 0, indents: Vec::new(), verbosity_limit }
    }

    fn indent(&mut self, segment: Vec<u8>) {
        self.indents.push(segment);
        self.level = self.indents.len() as u16;
    }

    fn dedent(&mut self) {
        if self.level == 0 {
            return;
        }
        self.indents.pop();
        self.level -= 1;
        if self.level < self.last_logged_level {
            self.last_logged_level = self.level;
        }
    }

    fn progress_verbosity(&self) -> u8 {
        (LOG_VL_PROGRESS as i32 + self.level as i32) as u8
    }

    /// Renders one message at the current depth, or returns `None` if
    /// `verbosity` is above the limit. A printed line always ends in `\n`.
    fn log(&mut self, verbosity: u8, kind: LoggerType, data: &[u8]) -> Option<Vec<u8>> {
        if verbosity > self.verbosity_limit {
            return None;
        }
        let mut line: Vec<u8> = Vec::new();
        for (level, indent) in self.indents.iter().enumerate() {
            if (level as i32) < self.last_logged_level as i32 - 1 {
                line.resize(line.len() + indent.len(), b' ');
                if (level as i32) < self.last_logged_level as i32 - 2 {
                    line.extend_from_slice(b" | ");
                } else {
                    line.extend_from_slice(b" |-");
                }
            } else {
                line.extend_from_slice(indent);
                line.extend_from_slice(b" : ");
            }
        }
        if (kind as usize) < OTFCC_LOGGER_TYPE_NAMES.len() {
            line.extend_from_slice(OTFCC_LOGGER_TYPE_NAMES[kind as usize].as_bytes());
            line.push(b' ');
        }
        line.extend_from_slice(data);
        if line.last() != Some(&b'\n') {
            line.push(b'\n');
        }
        self.last_logged_level = self.level;
        Some(line)
    }

    fn start(&mut self, segment: Vec<u8>) -> Option<Vec<u8>> {
        self.indent(segment);
        self.log(self.progress_verbosity(), LoggerType::Progress, b"Begin")
    }

    fn finish(&mut self) -> Option<Vec<u8>> {
        let line = self.log(self.progress_verbosity(), LoggerType::Progress, b"Finish");
        self.dedent();
        line
    }
}

/// The verbosity and prefix a `tracing` level maps to.
fn verbosity_of(level: &Level) -> (u8, LoggerType) {
    match *level {
        Level::ERROR => (LOG_VL_CRITICAL, LoggerType::Error),
        Level::WARN => (LOG_VL_IMPORTANT, LoggerType::Warning),
        Level::INFO => (LOG_VL_NOTICE, LoggerType::Info),
        _ => (LOG_VL_PROGRESS, LoggerType::Progress),
    }
}

/// What the layer remembers about one open [`stage`]/[`indent`] span.
struct ScopeInfo {
    segment: Vec<u8>,
    is_stage: bool,
    finished: bool,
}

#[derive(Default)]
struct ScopeVisitor {
    segment: Option<String>,
    finished: bool,
}

impl Visit for ScopeVisitor {
    fn record_bool(&mut self, field: &Field, value: bool) {
        if field.name() == "finished" {
            self.finished = value;
        }
    }
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "segment" {
            self.segment = Some(format!("{value:?}"));
        }
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "segment" {
            self.segment = Some(value.to_owned());
        }
    }
}

#[derive(Default)]
struct MessageVisitor(String);

impl Visit for MessageVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            use fmt::Write as _;
            let _ = write!(self.0, "{value:?}");
        }
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.0.push_str(value);
        }
    }
}

/// Renders otfcc's [`stage`]/[`indent`] scopes and `tracing` events as the
/// original indented log format, writing each line to `W`.
///
/// Events are filtered by `verbosity` (0 = errors only, 1 = also warnings,
/// 2 = also notes, 10 and up = also progress) before they are formatted;
/// otfcc scopes are always tracked so that a warning inside one is indented
/// correctly. The indentation state is shared by every thread, so only one
/// thread should be logging at a time -- true of the CLI binaries.
pub struct OtfccTreeLayer<W> {
    state: Mutex<(TreeFormatter, W)>,
}

impl<W: Write> OtfccTreeLayer<W> {
    pub fn new(verbosity: u8, out: W) -> Self {
        OtfccTreeLayer { state: Mutex::new((TreeFormatter::new(verbosity), out)) }
    }

    fn verbosity_limit(&self) -> u8 {
        self.with_state(|formatter, _| formatter.verbosity_limit)
    }

    fn with_state<R>(&self, f: impl FnOnce(&mut TreeFormatter, &mut W) -> R) -> R {
        let mut guard = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let (formatter, out) = &mut *guard;
        f(formatter, out)
    }
}

fn is_otfcc_scope(metadata: &Metadata<'_>) -> bool {
    metadata.is_span() && (metadata.name() == STAGE_SPAN || metadata.name() == INDENT_SPAN)
}

impl<S, W> Layer<S> for OtfccTreeLayer<W>
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    W: Write + Send + 'static,
{
    fn register_callsite(&self, _metadata: &'static Metadata<'static>) -> Interest {
        // Decided per call in `enabled`: tests install layers with different
        // verbosities, so a cached answer could be stale.
        Interest::sometimes()
    }

    fn enabled(&self, metadata: &Metadata<'_>, _ctx: Context<'_, S>) -> bool {
        if metadata.is_span() {
            return true;
        }
        verbosity_of(metadata.level()).0 <= self.verbosity_limit()
    }

    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        if !is_otfcc_scope(attrs.metadata()) {
            return;
        }
        let mut visitor = ScopeVisitor::default();
        attrs.record(&mut visitor);
        if let Some(span) = ctx.span(id) {
            span.extensions_mut().insert(ScopeInfo {
                segment: visitor.segment.unwrap_or_default().into_bytes(),
                is_stage: attrs.metadata().name() == STAGE_SPAN,
                finished: visitor.finished,
            });
        }
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let mut visitor = ScopeVisitor::default();
        values.record(&mut visitor);
        if !visitor.finished {
            return;
        }
        if let Some(span) = ctx.span(id)
            && let Some(info) = span.extensions_mut().get_mut::<ScopeInfo>()
        {
            info.finished = true;
        }
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        let Some(span) = ctx.span(id) else { return };
        let extensions = span.extensions();
        let Some(info) = extensions.get::<ScopeInfo>() else { return };
        let segment = info.segment.clone();
        let is_stage = info.is_stage;
        drop(extensions);
        self.with_state(|formatter, out| {
            if is_stage {
                if let Some(line) = formatter.start(segment) {
                    let _ = out.write_all(&line);
                }
            } else {
                formatter.indent(segment);
            }
        });
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        let Some(span) = ctx.span(id) else { return };
        let extensions = span.extensions();
        let Some(info) = extensions.get::<ScopeInfo>() else { return };
        let prints_finish = info.is_stage && info.finished;
        drop(extensions);
        self.with_state(|formatter, out| {
            if prints_finish {
                if let Some(line) = formatter.finish() {
                    let _ = out.write_all(&line);
                }
            } else {
                formatter.dedent();
            }
        });
    }

    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let (verbosity, kind) = verbosity_of(event.metadata().level());
        let mut visitor = MessageVisitor::default();
        event.record(&mut visitor);
        self.with_state(|formatter, out| {
            if let Some(line) = formatter.log(verbosity, kind, visitor.0.as_bytes()) {
                let _ = out.write_all(&line);
            }
        });
    }
}

/// Installs an [`OtfccTreeLayer`] writing to stderr as the process-wide
/// `tracing` subscriber. Called once by each CLI binary after its options
/// are parsed; a second call (or another subscriber already installed) is
/// ignored.
pub fn install_stderr(verbosity: u8) {
    use tracing_subscriber::layer::SubscriberExt as _;
    let subscriber =
        tracing_subscriber::Registry::default().with(OtfccTreeLayer::new(verbosity, std::io::stderr()));
    let _ = tracing::subscriber::set_global_default(subscriber);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tracing_subscriber::layer::SubscriberExt as _;

    /// The pre-`tracing` logger, copied unchanged as the reference the new
    /// layer must reproduce byte for byte.
    struct ReferenceLogger {
        level: u16,
        last_logged_level: u16,
        indents: Vec<Vec<u8>>,
        verbosity_limit: u8,
        out: Vec<u8>,
    }

    impl ReferenceLogger {
        fn indent(&mut self, segment: &[u8]) {
            self.indents.push(segment.to_vec());
            self.level = self.indents.len() as u16;
        }
        fn dedent(&mut self) {
            if self.level == 0 {
                return;
            }
            self.indents.pop();
            self.level = (self.level as i32 - 1_i32) as u16;
            if (self.level as i32) < self.last_logged_level as i32 {
                self.last_logged_level = self.level;
            }
        }
        fn start(&mut self, segment: &[u8]) {
            self.indent(segment);
            let v = (LOG_VL_PROGRESS as i32 + self.level as i32) as u8;
            self.log(v, LoggerType::Progress, b"Begin");
        }
        fn finish(&mut self) {
            let v = (LOG_VL_PROGRESS as i32 + self.level as i32) as u8;
            self.log(v, LoggerType::Progress, b"Finish");
            self.dedent();
        }
        fn log(&mut self, verbosity: u8, kind: LoggerType, data: &[u8]) {
            let mut demand: Vec<u8> = Vec::new();
            for (level, indent) in self.indents.iter().enumerate() {
                if (level as i32) < self.last_logged_level as i32 - 1_i32 {
                    demand.resize(demand.len() + indent.len(), b' ');
                    if (level as i32) < self.last_logged_level as i32 - 2_i32 {
                        demand.extend_from_slice(b" | ");
                    } else {
                        demand.extend_from_slice(b" |-");
                    }
                } else {
                    demand.extend_from_slice(indent);
                    demand.extend_from_slice(b" : ");
                }
            }
            if (kind as u32) < 3 {
                demand.extend_from_slice(OTFCC_LOGGER_TYPE_NAMES[kind as usize].as_bytes());
                demand.extend_from_slice(b" ");
            }
            demand.extend_from_slice(data);
            if verbosity as i32 <= self.verbosity_limit as i32 {
                self.out.extend_from_slice(&demand);
                if demand.last() != Some(&b'\n') {
                    self.out.push(b'\n');
                }
                self.last_logged_level = self.level;
            }
        }
    }

    enum Op {
        Indent(&'static str),
        Start(&'static str),
        Finish,
        Dedent,
        Log(LoggerType, &'static str),
    }

    fn level_of(kind: LoggerType) -> u8 {
        match kind {
            LoggerType::Error => LOG_VL_CRITICAL,
            LoggerType::Warning => LOG_VL_IMPORTANT,
            LoggerType::Info => LOG_VL_NOTICE,
            LoggerType::Progress => LOG_VL_PROGRESS,
        }
    }

    fn reference_output(ops: &[Op], verbosity: u8) -> String {
        let mut r = ReferenceLogger {
            level: 0,
            last_logged_level: 0,
            indents: Vec::new(),
            verbosity_limit: verbosity,
            out: Vec::new(),
        };
        for op in ops {
            match op {
                Op::Indent(s) => r.indent(s.as_bytes()),
                Op::Start(s) => r.start(s.as_bytes()),
                Op::Finish => r.finish(),
                Op::Dedent => r.dedent(),
                Op::Log(kind, msg) => r.log(level_of(*kind), *kind, msg.as_bytes()),
            }
        }
        String::from_utf8(r.out).unwrap()
    }

    #[derive(Clone, Default)]
    struct SharedBuf(Arc<Mutex<Vec<u8>>>);
    impl Write for SharedBuf {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn layer_output(ops: &[Op], verbosity: u8) -> String {
        let buf = SharedBuf::default();
        let subscriber =
            tracing_subscriber::Registry::default().with(OtfccTreeLayer::new(verbosity, buf.clone()));
        tracing::subscriber::with_default(subscriber, || {
            let mut open: Vec<StageGuard> = Vec::new();
            for op in ops {
                match op {
                    Op::Indent(s) => open.push(indent(s)),
                    Op::Start(s) => open.push(stage(s)),
                    Op::Finish => open.pop().unwrap().finish(),
                    Op::Dedent => drop(open.pop()),
                    Op::Log(LoggerType::Error, m) => tracing::error!("{m}"),
                    Op::Log(LoggerType::Warning, m) => tracing::warn!("{m}"),
                    Op::Log(LoggerType::Info, m) => tracing::info!("{m}"),
                    Op::Log(LoggerType::Progress, m) => tracing::debug!("{m}"),
                }
            }
        });
        String::from_utf8(buf.0.lock().unwrap().clone()).unwrap()
    }

    /// A run shaped like `otfccdump`'s: a root indent, nested stages with
    /// progress, warnings and errors at several depths, a stage closed
    /// without `Finish` (a plain drop), and messages with and without a
    /// trailing `\n`.
    fn script() -> Vec<Op> {
        use LoggerType::*;
        vec![
            Op::Indent("otfccdump"),
            Op::Start("Read SFNT"),
            Op::Log(Progress, "From file a.ttf"),
            Op::Finish,
            Op::Start("Consolidate"),
            Op::Start("glyf"),
            Op::Start("A"),
            Op::Log(Warning, "Bad contour in A\n"),
            Op::Finish,
            Op::Start("B"),
            Op::Finish,
            Op::Finish,
            Op::Start("GSUB"),
            Op::Start("lookup_0"),
            Op::Log(Info, "Merged two subtables"),
            Op::Log(Warning, "Ignored empty subtable"),
            Op::Finish,
            Op::Log(Error, "Lookup type unknown"),
            Op::Dedent,
            Op::Log(Warning, "after an abandoned stage"),
            Op::Finish,
            Op::Start("Dump"),
            Op::Log(Progress, "Step time = 0.1s.\n"),
            Op::Finish,
            Op::Log(Error, "at the root"),
        ]
    }

    #[test]
    fn layer_matches_the_original_logger_at_every_cli_verbosity() {
        for verbosity in [0u8, 1, 2, 0xff] {
            let want = reference_output(&script(), verbosity);
            let got = layer_output(&script(), verbosity);
            assert_eq!(got, want, "verbosity {verbosity}");
        }
        // Not vacuous: normal mode prints indented warnings, verbose prints
        // the Begin/Finish tree.
        assert!(reference_output(&script(), 1).contains("[WARNING] Bad contour in A"));
        assert!(reference_output(&script(), 0xff).contains("Read SFNT : Begin"));
    }

    #[test]
    fn disabled_messages_are_not_formatted() {
        struct Panics;
        impl fmt::Display for Panics {
            fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
                panic!("a message below the verbosity limit was formatted");
            }
        }
        let subscriber =
            tracing_subscriber::Registry::default().with(OtfccTreeLayer::new(1, SharedBuf::default()));
        tracing::subscriber::with_default(subscriber, || {
            tracing::info!("{}", Panics);
            tracing::debug!("{}", Panics);
        });
    }

    #[test]
    fn a_stage_left_by_an_early_return_closes_without_finish() {
        // `otfccdump`'s "Cannot read SFNT file" path: the stage is dropped
        // by the error return, never finished.
        fn read_sfnt() -> Result<(), ()> {
            let stage = stage("Read SFNT");
            tracing::error!("Cannot read");
            Err(())?;
            stage.finish();
            Ok(())
        }
        let buf = SharedBuf::default();
        let subscriber =
            tracing_subscriber::Registry::default().with(OtfccTreeLayer::new(0xff, buf.clone()));
        tracing::subscriber::with_default(subscriber, || {
            let _root = indent("otfccdump");
            assert!(read_sfnt().is_err());
            tracing::error!("after");
        });
        assert_eq!(
            String::from_utf8(buf.0.lock().unwrap().clone()).unwrap(),
            "otfccdump : Read SFNT : Begin\n          |-Read SFNT : [ERROR] Cannot read\notfccdump : [ERROR] after\n"
        );
    }

    #[test]
    fn byte_str_renders_like_the_old_message_parts() {
        let name: Vec<u8> = b"abc\0def".to_vec();
        assert_eq!(ByteStr(&name).to_string(), "abc", "a Vec is cut at NUL");
        assert_eq!(ByteStr(&name[..]).to_string(), "abc\u{0}def", "a slice is not");
        assert_eq!(ByteStr(b"a\xffb").to_string(), "a\u{fffd}b");
        assert_eq!(ByteStr(-7_i32).to_string(), "-7");
    }
}
