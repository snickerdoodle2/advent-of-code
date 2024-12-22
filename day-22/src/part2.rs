use std::collections::{HashMap, VecDeque};

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

fn find_monkey_pattern(starting: u64) -> HashMap<[i8; 4], i8> {
    let mut cur_price = (starting % 10) as i8;
    let mut diff = VecDeque::new();
    let mut monkey = Monkey(starting).into_iter().take(2000);
    let mut res = HashMap::new();

    while let Some(next) = monkey.next() {
        let new_price = (next % 10) as i8;

        diff.push_back(new_price - cur_price);
        if diff.len() > 4 {
            diff.pop_front();
            let key = (
                *diff.get(0).unwrap(),
                *diff.get(1).unwrap(),
                *diff.get(2).unwrap(),
                *diff.get(3).unwrap(),
            )
                .into();
            if !res.contains_key(&key) {
                res.insert(key, new_price);
            }
        }
        cur_price = new_price;
    }

    res
}

pub fn process(input: &str) -> String {
    let mut res: HashMap<[i8; 4], u32> = HashMap::new();
    for line in input.lines() {
        let line_res = find_monkey_pattern(line.parse().unwrap());
        for (key, v) in line_res {
            res.entry(key)
                .and_modify(|x| {
                    *x += v as u32;
                })
                .or_insert(v as u32);
        }
    }

    res.values().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "1
2
3
2024";
        assert_eq!("23", process(input));
    }
}
