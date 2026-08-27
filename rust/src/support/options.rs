use libc::free;
use std::cell::RefCell;

use crate::logger::{Logger, LoggerTarget};
use crate::support::alloc::__caryll_allocate_clean;

#[derive(Default)]
pub struct Options {
    pub debug_wait_on_start: bool,
    pub ignore_glyph_order: bool,
    pub ignore_hints: bool,
    pub has_vertical_metrics: bool,
    pub export_fdselect: bool,
    pub keep_average_char_width: bool,
    pub keep_unicode_ranges: bool,
    pub short_post: bool,
    pub dummy_dsig: bool,
    pub keep_modified_time: bool,
    pub instr_as_bytes: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub cff_short_vmtx: bool,
    pub merge_lookups: bool,
    pub merge_features: bool,
    pub force_cid: bool,
    pub cff_roll_char_string: bool,
    pub cff_do_subroutinize: bool,
    pub stub_cmap4: bool,
    pub decimal_cmap: bool,
    pub name_glyphs_by_hash: bool,
    pub name_glyphs_by_gid: bool,
    pub glyph_name_prefix: *mut ::core::ffi::c_char,
    // Was `*mut Logger`, a second heap allocation `Options` merely pointed
    // at (built via the now-removed `otfcc_new_logger`, freed via the
    // now-removed `logger_dispose`). `Options` owns its `Logger` inline
    // now; `RefCell` gives every call site holding only `&Options` (the
    // norm since Stage 7-2-a) a way to still get `&mut Logger` out to log
    // with, without needing `&mut Options` threaded through every read/
    // dump/build/parse function purely for logging. Every existing call
    // site already logs with a single short-lived borrow per statement
    // (`logger_log_sds(&mut *options.logger.borrow_mut(), ...)`, immediately
    // released), never nested re-entrantly into another borrow of the same
    // `Logger` -- confirmed by full pipeline + Miri after the conversion,
    // which would surface a `RefCell` double-borrow as a panic, not silent
    // UB. Single-threaded throughout (this crate has no threading), so
    // `RefCell` over `Mutex` costs nothing and needs no `Sync` bound.
    pub logger: RefCell<Logger>,
    // Bounds the total number of "invalid lookup reference" warnings
    // `consolidate_chaining` will actually log for one `otfcc_consolidate_
    // font` call. Each individual contextual/chaining rule's own lookup-
    // application count is already capped at parse time
    // (`chaining/read.rs`'s `MAX_APPLY_PER_RULE`), and rules built across a
    // whole table and one lookup's subtable count are capped too
    // (`MAX_TOTAL_RULES_PER_TABLE`, `MAX_TOTAL_SUBTABLES_PER_LOOKUP`) --
    // but those three caps multiply, and fuzzing found a font that rode
    // all three near their ceiling at once (many subtables, each with many
    // rules, each with many unresolvable lookup applications), reaching
    // millions of warnings -- each one a heap-allocating `bytesbuild!` call
    // plus a stderr write, tens of seconds of pure logging overhead. This
    // is the backstop that bounds the *product*, not just each factor.
    // Reset to `CONSOLIDATE_WARNING_BUDGET` at the start of every
    // `otfcc_consolidate_font` call (not just once at `Options` creation),
    // since one `Options` can drive many font conversions over its life
    // (the FFI/DLL entry points, in particular).
    pub consolidate_warning_budget: std::cell::Cell<u32>,
}
pub unsafe fn otfcc_new_options() -> *mut Options {
    let options: *mut Options = unsafe {
        __caryll_allocate_clean(
            ::core::mem::size_of::<Options>() as usize,
            6 as ::core::ffi::c_ulong,
        )
    } as *mut Options;
    // `__caryll_allocate_clean` calloc's the struct: every all-zero `bool`
    // and the null `glyph_name_prefix` are valid as-is, but an all-zero
    // `RefCell<Logger>` is not -- `Logger.indents: Vec<Vec<u8>>` needs a
    // dangling *non-null* sentinel at zero capacity, not a null one (the
    // same calloc-then-`ptr::write` hazard `logger_indent_sds`'s old
    // `otfcc_new_logger` counterpart used to guard against; see [[otfcc-
    // vec-field-assign-needs-calloc]]). A plain `=` here would drop the
    // invalid zeroed place first -- UB the instant that typed value exists,
    // independent of whether anything then dereferences it.
    unsafe {
        ::core::ptr::write(
            &raw mut (*options).logger,
            RefCell::new(Logger::new(LoggerTarget::Empty)),
        );
    }
    return options;
}
pub unsafe fn otfcc_delete_options(options: *mut Options) {
    unsafe {
        if !options.is_null() {
            free((*options).glyph_name_prefix as *mut ::core::ffi::c_void);
            (*options).glyph_name_prefix = ::core::ptr::null_mut::<::core::ffi::c_char>();
            // `otfcc_new_options`/here both use the manual calloc/`free`
            // pair, not `Box` (that's Stage 7-2-d's job, across every
            // `_create()`-shaped allocator, not just this one) -- so the
            // raw `free()` below does NOT run `Options`'s field destructors
            // the way dropping a `Box<Options>` would. `glyph_name_prefix`
            // was always handled by hand for the same reason (see the
            // `free()` two lines up); `logger`'s `Vec<Vec<u8>>` needs the
            // same explicit treatment now that it's owned inline instead of
            // being a raw pointer with nothing to drop. Swap in a fresh,
            // non-allocating empty `Logger` (`Vec::new()` never allocates)
            // and let the replaced one's `Drop` run here, before the raw
            // `free()` reclaims the struct's own memory out from under it.
            drop(::core::mem::replace(
                &mut (*options).logger,
                RefCell::new(Logger::new(LoggerTarget::Empty)),
            ));
        }
        free(options as *mut ::core::ffi::c_void);
    }
}
pub unsafe fn otfcc_options_optimize_to(options: *mut Options, level: u8) {
    unsafe {
        (*options).cff_roll_char_string = false;
        (*options).short_post = false;
        (*options).ignore_glyph_order = false;
        (*options).cff_short_vmtx = false;
        (*options).merge_features = false;
        (*options).force_cid = false;
        (*options).cff_do_subroutinize = false;
        if level as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            (*options).cff_roll_char_string = true;
            (*options).cff_short_vmtx = true;
        }
        if level as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
            (*options).short_post = true;
            (*options).cff_do_subroutinize = true;
            (*options).merge_features = true;
        }
        if level as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
            (*options).ignore_glyph_order = true;
            (*options).force_cid = true;
        }
    }
}
