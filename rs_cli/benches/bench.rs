use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rs_cli::advent_of_code::twenty_fifteen::one::*;

fn criterion_benchmark_floor(c: &mut Criterion) {
    let input = black_box(read_input());

    c.bench_function("floor_chars", |b| b.iter(|| floor_chars(&input)));
    c.bench_function("floor_bytes", |b| b.iter(|| floor_bytes(&input)));
}

fn criterion_benchmark_floor_basement(c: &mut Criterion) {
    let input = black_box(read_input());
    c.bench_function("floor_basement", |b| b.iter(|| floor_basement(&input)));
    
    c.bench_function("floor_basement_same_var", |b| {
        b.iter(|| floor_basement_same_var(&input))
    });
}

criterion_group!(
    benches,
    criterion_benchmark_floor,
    criterion_benchmark_floor_basement
);
criterion_main!(benches);
