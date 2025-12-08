use day_08::part1::process;

fn main() {
    let input = include_str!("../../input1.txt");
    let result = process(input, 1000);
    println!("{result}");
}
