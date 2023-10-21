use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rs_cli::advent_of_code::twenty_fifteen::{one::*, two::*};

// fn floor_bench(c: &mut Criterion) {
//     let input = black_box(read_input());

//     c.bench_function("floor_chars", |b| b.iter(|| floor_chars(&input)));
//     c.bench_function("floor_bytes", |b| b.iter(|| floor_bytes(&input)));
// }

// fn floor_basement_bench(c: &mut Criterion) {
//     let input = black_box(read_input());
//     c.bench_function("floor_basement_same_var", |b| {
//         b.iter(|| floor_basement_same_var(&input))
//     });
    
//     c.bench_function("floor_basement", |b| b.iter(|| floor_basement(&input)));
    
// }

// fn wrapper_paper_bench(c: &mut Criterion) {
    // c.bench_function("wrapper_paper_read_input", |b| b.iter(|| wrapper_paper_read_input()));
    
    // c.bench_function("wrapper_paper_read_lines", |b| b.iter(|| wrapper_paper_read_lines()));
    
    // c.bench_function("wrapper_paper_read_input_get_side_values", |b| b.iter(|| wrapper_paper_read_input_get_side_values()));
    
    // c.bench_function("wrapper_paper_read_lines_get_side_values", |b| b.iter(|| wrapper_paper_read_lines_get_side_values()));
    
    // c.bench_function("wrapper_paper_read_input_area", |b| b.iter(|| wrapper_paper_read_input_area()));
    
    // c.bench_function("wrapper_paper_read_lines_area", |b| b.iter(|| wrapper_paper_read_lines_area()));
    
    // c.bench_function("wrapper_paper_read_input_area_sum", |b| b.iter(|| wrapper_paper_read_input_area_sum()));
    
    // c.bench_function("wrapper_paper_read_lines_area_sum", |b| b.iter(|| wrapper_paper_read_lines_area_sum()));
// }

fn ribbon_length_bench(c: &mut Criterion) {
    c.bench_function("ribbon_length", |b| {
        b.iter(|| ribbon_length())
    });

    c.bench_function("ribbon_length_2", |b| {
        b.iter(|| ribbon_length_2())
    });
}

criterion_group!(
    benches,
    // floor_bench,
    // floor_basement_bench,
    // wrapper_paper_bench,
    ribbon_length_bench
);
criterion_main!(benches);
