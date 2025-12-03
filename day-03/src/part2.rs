pub fn calculate(digits: &[u8], remaining: u8) -> Option<u64> {
    if remaining == 0 {
        return Some(0);
    }
    if digits.len() < remaining as usize {
        return None;
    }

    let mut max_idx: Vec<_> = digits.iter().enumerate().collect();
    max_idx.sort_by_key(|x| std::cmp::Reverse(x.1));

    for (i, v) in max_idx {
        if let Some(mut res) = calculate(&digits[i + 1..], remaining - 1) {
            res = match res {
                0 => *v as u64,
                prev => {
                    let len = prev.ilog10() + 1;
                    let v = (*v as u64) * 10u64.pow(len);
                    v + res
                }
            };
            return Some(res);
        }
    }

    None
}

fn process_line(line: &str) -> u64 {
    let digits: Vec<u8> = line
        .chars()
        .map(|c| c.to_digit(10).expect("should be a digit") as u8)
        .collect();

    calculate(&digits, 12).expect("should find a solution")
}

pub fn process(input: &str) -> String {
    input.lines().map(process_line).sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part2() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        assert_eq!("3121910778619", process(input));
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    // test cases taken from reddit
    #[case("2232546378857275787561723292343835435343333776427842773354273372424413455462238746648634437374254318", 988887754318)]
    #[case("2232323232236223322223321222232212221212222222222332111132223222222222322133213322323133322222332224", 633333333334)]
    #[case("5345633566354453355546874555676462558526423364443535432344223165523377525665661379556365535642545245", 966642545245)]
    #[case("3312322113352322342133434233342422313224135342333232234332332232313223352233232336232233533323364322", 653333364322)]
    fn test_line(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(expected, process_line(input));
    }
}
