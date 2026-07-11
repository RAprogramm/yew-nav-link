// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use yew_nav_link::active_link::is_path_prefix;

fn bench_is_path_prefix(c: &mut Criterion) {
    c.bench_function("path_prefix_exact_match", |b| {
        b.iter(|| is_path_prefix(black_box("/docs"), black_box("/docs")));
    });

    c.bench_function("path_prefix_nested", |b| {
        b.iter(|| is_path_prefix(black_box("/docs"), black_box("/docs/api")));
    });

    c.bench_function("path_prefix_root", |b| {
        b.iter(|| is_path_prefix(black_box("/"), black_box("/docs/api")));
    });

    c.bench_function("path_prefix_no_match", |b| {
        b.iter(|| is_path_prefix(black_box("/about"), black_box("/docs")));
    });

    c.bench_function("path_prefix_segment_boundary", |b| {
        b.iter(|| is_path_prefix(black_box("/doc"), black_box("/documents")));
    });

    c.bench_function("path_prefix_deep_nested", |b| {
        b.iter(|| is_path_prefix(black_box("/a"), black_box("/a/b/c/d/e/f")));
    });

    c.bench_function("path_prefix_trailing_slash", |b| {
        b.iter(|| is_path_prefix(black_box("/docs/"), black_box("/docs/api")));
    });

    c.bench_function("path_prefix_empty_target", |b| {
        b.iter(|| is_path_prefix(black_box(""), black_box("/docs")));
    });
}

criterion_group!(benches, bench_is_path_prefix);
criterion_main!(benches);
