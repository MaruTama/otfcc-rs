//! Benchmarks the `-O2`/`--subroutinize` build path specifically (README's
//! own speed claims name this optimization directly), comparing it
//! side-by-side against a default build of the same input --
//! `libcff/subr.rs`'s subroutinization graph algorithm is the code this
//! flag turns on that a plain `benches/build.rs` run never exercises.
//!
//! Uses `WorkSans-Regular.json` (a checked-in JSON-only fixture) rather
//! than dumping `WorkSans-Regular.otf` first: that payload is one of the
//! two documented to crash `otfccdump`'s CFF path (see `benches/dump.rs`'s
//! own comment), so its JSON form -- captured before that crash bug existed
//! -- is the only usable input for this benchmark.
mod support;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use support::{build_to_otf, payload_bytes, quiet_options, quiet_options_o2};

fn bench_subroutinize(c: &mut Criterion) {
    let mut group = c.benchmark_group("subroutinize");
    let json_bytes = payload_bytes("WorkSans-Regular.json");

    let default_options = quiet_options();
    group.bench_function("WorkSans-Regular-default", |b| {
        b.iter_batched(|| json_bytes.clone(), |json| build_to_otf(&json, &default_options), BatchSize::SmallInput);
    });

    let o2_options = quiet_options_o2();
    group.bench_function("WorkSans-Regular-O2", |b| {
        b.iter_batched(|| json_bytes.clone(), |json| build_to_otf(&json, &o2_options), BatchSize::SmallInput);
    });

    group.finish();
}

criterion_group!(benches, bench_subroutinize);
criterion_main!(benches);
