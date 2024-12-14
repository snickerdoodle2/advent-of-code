use day_14::part1::process;

fn main() {
    let input = include_str!("../../input1.txt");
    let result = process(input, 101, 103);
    println!("{}", result);
}
