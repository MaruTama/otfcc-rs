//! Dump-only (SFNT -> JSON) benchmarks, mirroring `otfccdump`'s own
//! pipeline in-process (no subprocess spawn overhead, which would dominate
//! at these fixture sizes) -- see `benches/support/mod.rs::dump_to_json`.
//!
//! `Cormorant-Medium.otf`/`WorkSans-Regular.otf` are deliberately excluded:
//! both are documented (see `RUST_MIGRATION.md`) to stack-overflow in the
//! dump path, faithfully reproducing a pre-existing bug in the original C
//! CFF interpreter -- including them here would crash `cargo bench`
//! outright, not just run slowly.
mod support;

use criterion::{Criterion, criterion_group, criterion_main};
use support::{dump_to_json, payload_bytes, quiet_options};

fn bench_dump(c: &mut Criterion) {
    let mut group = c.benchmark_group("dump");
    let fixtures: &[&str] = &[
        "KRName-Regular.otf",           // small CFF
        "Molengo-Regular.ttf",          // small TTF
        "iosevka-r.ttf",                // larger TTF
        "NotoNastaliqUrdu-Regular.ttf",  // larger/complex TTF
    ];
    for &name in fixtures {
        let bytes = payload_bytes(name);
        let options = quiet_options();
        group.bench_function(name, |b| {
            b.iter(|| dump_to_json(&bytes, &options));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_dump);
criterion_main!(benches);
