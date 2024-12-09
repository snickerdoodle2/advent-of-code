use day_07::part1_recursive::process;

fn main() {
    let input = include_str!("../../input1.txt");
    let result = process(input);
    println!("{}", result);
}
