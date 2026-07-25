//! otfcc's version number.
//!
//! On the C side these come from `premake5.lua` as `-DMAIN_VER=…`, with
//! `c/lib/table/name.c` carrying `#define MAIN_VER 0` as the fallback when a
//! build system does not pass them. c2rust captured the resolved values, so all
//! three copies agreed; they are collected here so that bumping a version is one
//! edit rather than three, and so the two binaries and the `name` table's
//! "-- By OTFCC %d.%d.%d --" string cannot drift apart.

pub const MAIN_VER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SECONDARY_VER: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PATCH_VER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
