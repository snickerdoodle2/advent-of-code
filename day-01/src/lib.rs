use std::collections::HashMap;

pub fn process_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_numeric())
                .map(|d| d.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_2(input: &str) -> String {
    let dig_words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    input
        .lines()
        .map(|line| {
            let mut v = Vec::new();
            for word in dig_words.clone() {
                line.match_indices(word).for_each(|(idx, _)| {
                    v.push((idx, word));
                });
            }

            v.sort_by(|(idx_a, _), (idx_b, _)| idx_a.cmp(idx_b));

            let digits = v
                .into_iter()
                .map(|(_, d)| match d {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    x => x.parse::<u32>().unwrap(),
                })
                .collect::<Vec<u32>>();
            dbg!(line, &digits);

            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let res = process_1(input);
        assert_eq!("142", res);
    }

    #[test]
    fn test_process_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let res = process_2(input);
        assert_eq!("281", res);
    }
}
