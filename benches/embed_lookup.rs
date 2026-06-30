//! Benchmark for `Asset::get()` over a large embedded folder.
//!
//! The fixture folder is generated at build time by `build.rs` behind the
//! `bench-fixtures` feature, and its path is exposed via the
//! `BENCH_FIXTURE_DIR` env var (set with `cargo:rustc-env`). The embed derive
//! reads that env var at macro-expansion time through `interpolate-folder-path`.
//!
//! `cargo bench` compiles with optimizations and `debug_assertions` off, so the
//! embedded (release) `get()` code path is exercised, not the dynamic one. The
//! `bench-fixtures` feature also enables `always-embed` as a belt-and-suspenders
//! guarantee that the embedded path is used.

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use rust_embed_for_web::RustEmbed;

#[derive(RustEmbed)]
#[folder = "${BENCH_FIXTURE_DIR}"]
struct Assets;

fn bench_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("embed_get");

    // Keys span the sorted table: first, middle, last, plus a missing key.
    // Linear scan's worst case is last/missing; binary search is
    // position-independent.
    let first = "file_00000.txt";
    let middle = "file_01000.txt";
    let last = "file_01999.txt";
    let missing = "file_99999.txt";

    group.bench_function("first", |b| {
        b.iter(|| black_box(Assets::get(black_box(first))).is_some())
    });
    group.bench_function("middle", |b| {
        b.iter(|| black_box(Assets::get(black_box(middle))).is_some())
    });
    group.bench_function("last", |b| {
        b.iter(|| black_box(Assets::get(black_box(last))).is_some())
    });
    group.bench_function("missing", |b| {
        b.iter(|| black_box(Assets::get(black_box(missing))).is_some())
    });

    group.finish();
}

criterion_group!(benches, bench_get);
criterion_main!(benches);
