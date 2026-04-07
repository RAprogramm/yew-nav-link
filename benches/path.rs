use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn is_path_prefix(target: &str, current: &str) -> bool {
    let mut target_iter = target.split('/').filter(|s| !s.is_empty());
    let mut current_iter = current.split('/').filter(|s| !s.is_empty());

    loop {
        match (target_iter.next(), current_iter.next()) {
            (Some(t), Some(c)) if t == c => continue,
            (Some(_), Some(_)) => return false,
            (Some(_), None) => return false,
            (None, _) => return true
        }
    }
}

fn bench_is_path_prefix(c: &mut Criterion) {
    c.bench_function("path_prefix_exact_match", |b| {
        b.iter(|| is_path_prefix(black_box("/docs"), black_box("/docs")))
    });

    c.bench_function("path_prefix_nested", |b| {
        b.iter(|| is_path_prefix(black_box("/docs"), black_box("/docs/api")))
    });

    c.bench_function("path_prefix_root", |b| {
        b.iter(|| is_path_prefix(black_box("/"), black_box("/docs/api")))
    });

    c.bench_function("path_prefix_no_match", |b| {
        b.iter(|| is_path_prefix(black_box("/about"), black_box("/docs")))
    });

    c.bench_function("path_prefix_segment_boundary", |b| {
        b.iter(|| is_path_prefix(black_box("/doc"), black_box("/documents")))
    });

    c.bench_function("path_prefix_deep_nested", |b| {
        b.iter(|| is_path_prefix(black_box("/a"), black_box("/a/b/c/d/e/f")))
    });

    c.bench_function("path_prefix_trailing_slash", |b| {
        b.iter(|| is_path_prefix(black_box("/docs/"), black_box("/docs/api")))
    });

    c.bench_function("path_prefix_empty_target", |b| {
        b.iter(|| is_path_prefix(black_box(""), black_box("/docs")))
    });
}

criterion_group!(benches, bench_is_path_prefix);
criterion_main!(benches);
