use day_14::{part1, part2};

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench() {
    part1::process(divan::black_box(include_str!("../input1.txt")), 103, 101);
}

#[divan::bench]
fn part2_bench() {
    part2::process(divan::black_box(include_str!("../input2.txt")));
}
