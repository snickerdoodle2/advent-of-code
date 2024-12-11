use day_11::part1::process;

fn main() {
    let input = include_str!("../../input2.txt");
    let result = process(input, 75);
    println!("{}", result);
}
