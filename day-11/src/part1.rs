#[derive(Debug, Clone, Copy)]
struct Stone(u64);

impl Stone {
    fn change(self) -> Vec<Self> {
        if self.0 == 0 {
            return vec![Self(1)];
        }

        let num = self.0;

        let n = (num as f32).log10().floor() as u32 + 1;
        if n % 2 == 0 {
            let tmp = 10_u64.pow(n / 2);
            let left = num / tmp;
            let right = num % tmp;
            return vec![Self(left), Self(right)];
        }

        vec![Self(num * 2024)]
    }
}

pub fn process(input: &str) -> String {
    let mut stones: Vec<Stone> = input
        .split_whitespace()
        .map(|x| Stone(x.parse().unwrap()))
        .collect();

    for _iter in 0..25 {
        stones = stones.into_iter().flat_map(|s| s.change()).collect();
    }

    stones.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17";
        assert_eq!("55312", process(input));
    }
}
