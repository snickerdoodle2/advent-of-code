use day_13::part2::process;

const ERROR: u64 = 10_000_000_000_000;

fn main() {
    let input = include_str!("../../input2.txt");
    let result = process(input, ERROR);
    println!("{}", result);
}
