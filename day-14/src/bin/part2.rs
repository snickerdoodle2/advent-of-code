use std::env;

use day_14::part2::process;

fn main() {
    let input = include_str!("../../input2.txt");
    let max_iters = env::args().skip(1).next().unwrap();
    process(input, 101, 103, max_iters.parse().unwrap());
}
