use crate::part2;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Battery(u8, usize);

impl Ord for Battery {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match self.0.cmp(&other.0) {
            Equal => self.1.cmp(&other.1).reverse(),
            o => o,
        }
    }
}

impl PartialOrd for Battery {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Battery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Battery {}

fn process_line(line: &str) -> u16 {
    let n = line.len();
    let mut heap: BinaryHeap<_> = line
        .chars()
        .enumerate()
        .map(|(i, d)| Battery(d.to_digit(10).expect("should be a digit") as u8, i))
        .collect();

    let Battery(value, idx) = heap.pop().expect("should be at least 1");

    if idx == n - 1 {
        let Battery(x, _) = heap.pop().expect("should be at least 2");
        return (x * 10 + value) as u16;
    }

    loop {
        let Battery(x, i) = heap.pop().expect("should be at least 2");
        if i > idx {
            return (value * 10 + x) as u16;
        }
    }
}

fn process_line_alt(line: &str) -> u64 {
    let digits: Vec<u8> = line
        .chars()
        .map(|c| c.to_digit(10).expect("should be a digit") as u8)
        .collect();

    part2::calculate(&digits, 2).expect("should find a solution")
}

pub fn process(input: &str) -> String {
    input.lines().map(process_line).sum::<u16>().to_string()
}

pub fn process_alt(input: &str) -> String {
    input.lines().map(process_line_alt).sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        assert_eq!("357", process(input));
    }
}
