use aoc_2021::day03;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

const BENCH_INPUT: &str = "101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n010101010101\n101010101010\n";

fn bench_day03_add_lines(c: &mut Criterion) {
    // prepare inputs
    let short_input = BENCH_INPUT;
    let medium_input = &BENCH_INPUT.repeat(101);
    let long_input = &BENCH_INPUT.repeat(1001);

    let mut group = c.benchmark_group("day03 add_lines");
    for &i in [27, 2727, 27027].iter() {
        group.bench_with_input(
            BenchmarkId::new("simd", i),
            match i {
                27 => short_input,
                2727 => medium_input,
                27027 => long_input,
                _ => panic!(),
            },
            |b, input| b.iter(|| day03::add_lines_simd(black_box(input), black_box(i))),
        );
        group.bench_with_input(
            BenchmarkId::new("naive", i),
            match i {
                27 => short_input,
                2727 => medium_input,
                27027 => long_input,
                _ => panic!(),
            },
            |b, input| b.iter(|| day03::add_lines_naive(black_box(input), black_box(i))),
        );
    }
    group.finish();
}

criterion_group!(day03_step1, bench_day03_add_lines);
criterion_main!(day03_step1);
