use day_20::part2::process;

fn main() {
    let input = include_str!("../../input2.txt");
    let result = process(input, 100);
    println!("{}", result);
}
