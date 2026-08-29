//! otfcc's version number.
//!
//! On the C side these come from `premake5.lua` as `-DMAIN_VER=…`, with
//! `c/lib/table/name.c` carrying `#define MAIN_VER 0` as the fallback when a
//! build system does not pass them. c2rust captured the resolved values, so all
//! three copies agreed; they are collected here so that bumping a version is one
//! edit rather than three, and so the two binaries and the `name` table's
//! "-- By OTFCC %d.%d.%d --" string cannot drift apart.

pub const MAIN_VER: i32 = 0_i32;
pub const SECONDARY_VER: i32 = 10_i32;
pub const PATCH_VER: i32 = 4_i32;
