use day_14::part1;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench() {
    part1::process(divan::black_box(include_str!("../input1.txt")), 103, 101);
}
