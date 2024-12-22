struct Monkey(u64);

impl Monkey {
    fn mix(&mut self, rhs: u64) {
        self.0 ^= rhs;
    }

    fn prune(&mut self) {
        self.0 = self.0 % 16777216;
    }
}

impl Iterator for Monkey {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // step 1
        let res = self.0 * 64;
        self.mix(res);
        self.prune();

        // step 2
        let res = self.0 / 32;
        self.mix(res);
        self.prune();

        // step 3
        let res = self.0 * 2048;
        self.mix(res);
        self.prune();

        Some(self.0)
    }
}

fn get_2000th(starting: u64) -> u64 {
    let monkey = Monkey(starting);
    monkey.into_iter().skip(1999).next().unwrap()
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let starting: u64 = line.parse().unwrap();
            get_2000th(starting)
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, 8685429)]
    #[case(10, 4700978)]
    #[case(100, 15273692)]
    #[case(2024, 8667524)]
    fn test_monkey(#[case] input: u64, #[case] expected: u64) {
        assert_eq!(expected, get_2000th(input));
    }

    #[test]
    fn test_part1() {
        let input = "1
10
100
2024";
        assert_eq!("37327623", process(input));
    }
}
