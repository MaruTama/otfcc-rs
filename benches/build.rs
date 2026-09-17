//! Build-only (JSON -> SFNT/TTF) benchmarks, mirroring `otfccbuild`'s own
//! pipeline in-process -- see `benches/support/mod.rs::build_to_otf`.
//!
//! JSON input for the binary fixtures is produced once via `dump_to_json`
//! (setup, not measured -- `criterion`'s `iter_batched` keeps this out of
//! the timed portion) rather than shipping a second, possibly-drifting copy
//! of the fixture as committed JSON.
mod support;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use support::{build_to_otf, dump_to_json, free_options, payload_bytes, quiet_options};

fn bench_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("build");

    // Binary fixtures: dump once up front to get JSON input, then only the
    // build half is measured.
    let binary_fixtures: &[&str] = &[
        "Molengo-Regular.ttf",     // small TTF
        "KRName-Regular.otf",      // small CFF
        "FDArrayTest257.otf",      // CFF CID/FDSelect edge case
        "FDArrayTest65535.otf",    // CFF CID/FDSelect edge case, max count
    ];
    for &name in binary_fixtures {
        let sfnt_bytes = payload_bytes(name);
        let dump_options = quiet_options();
        let json_bytes = dump_to_json(&sfnt_bytes, dump_options);
        free_options(dump_options);

        let options = quiet_options();
        group.bench_function(name, |b| {
            b.iter_batched(|| json_bytes.clone(), |json| build_to_otf(&json, options), BatchSize::SmallInput);
        });
        free_options(options);
    }

    // JSON-only fixtures: no corresponding binary dump path exists (or, for
    // cid-fdselect-test.json, none is needed), so they go straight in.
    let json_fixtures: &[&str] = &["cid-fdselect-test.json"];
    for &name in json_fixtures {
        let json_bytes = payload_bytes(name);
        let options = quiet_options();
        group.bench_function(name, |b| {
            b.iter_batched(|| json_bytes.clone(), |json| build_to_otf(&json, options), BatchSize::SmallInput);
        });
        free_options(options);
    }

    group.finish();
}

criterion_group!(benches, bench_build);
criterion_main!(benches);
