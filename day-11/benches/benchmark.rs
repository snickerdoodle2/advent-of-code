use day_11::part1;

fn main() {
    divan::main();
}

#[divan::bench(consts=[
    10,
    15,
    25,
    50,
    75,
    100
])]
fn aoc_bench<const N: u8>() {
    part1::process(divan::black_box(include_str!("../input1.txt")), N);
}
