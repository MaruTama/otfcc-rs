//! `getopt_long`'s interface, which `libc` does not expose on this crate's CI
//! target.
//!
//! `libc` declares `struct LongOption` and `getopt_long` for the BSDs, Apple,
//! Solaris, Android and Hurd, but **not** for `*-unknown-linux-gnu` — so
//! delegating these the way `timespec` and `SEEK_SET` were delegated would
//! break the Linux build. They live here instead, declared once, rather than
//! separately in each of the two binaries.
//!
//! The binaries are separate crates, so they reach this as
//! `otfcc_rust::support::getopt::{LongOption, no_argument, required_argument}`.

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LongOption {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}

pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
