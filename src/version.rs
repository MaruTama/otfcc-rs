//! otfcc's version number.
//!
//! Kept in one place so the two binaries and the `name` table's
//! "-- By OTFCC %d.%d.%d --" string cannot drift apart.

pub const MAIN_VER: i32 = 0_i32;
pub const SECONDARY_VER: i32 = 10_i32;
pub const PATCH_VER: i32 = 4_i32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_cargo_package_version() {
        let ours = format!("{MAIN_VER}.{SECONDARY_VER}.{PATCH_VER}");
        assert_eq!(ours, env!("CARGO_PKG_VERSION"));
    }
}
