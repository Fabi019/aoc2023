use criterion::{criterion_group, criterion_main};

// To run individual benchmarks use:
// $ cargo bench --bench bench -- <name>
// where <name> can be like: day07, 07, 07/1, 7/2

aoc2023::bench!(day07);
aoc2023::bench!(day08);
aoc2023::bench!(day09);
aoc2023::bench!(day10);
aoc2023::bench!(day11);

criterion_group!(
    benches,  
    day07::bench,
    day08::bench,
    day09::bench,
    day10::bench,
    day11::bench
);

criterion_main!(benches);
