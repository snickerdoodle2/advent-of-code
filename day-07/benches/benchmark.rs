use day_07::{part1, part1_recursive, part2, part2_recursive};

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench() {
    part1::process(divan::black_box(include_str!("../input1.txt")));
}

#[divan::bench]
fn part2_bench() {
    part2::process(divan::black_box(include_str!("../input2.txt")));
}

#[divan::bench]
fn part1_recursive_bench() {
    part1_recursive::process(divan::black_box(include_str!("../input2.txt")));
}

#[divan::bench]
fn part2_recursive_bench() {
    part2_recursive::process(divan::black_box(include_str!("../input2.txt")));
}
