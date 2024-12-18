use day_18::part1::process;

fn main() {
    let input = include_str!("../../input1.txt");
    let result = process(input, 71, 71, 1024);
    println!("{}", result);
}
