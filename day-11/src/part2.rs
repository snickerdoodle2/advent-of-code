use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
    let stones_: Vec<_> = input
        .split_whitespace()
        .map(|x| vec![Stone(x.parse().unwrap())])
        .collect();

    let res: usize = stones_
        .par_iter()
        .map(|stones| {
            let mut stones = stones.clone();
            for _iter in 0..75 {
                stones = stones.into_iter().flat_map(|x| x.change()).collect();
            }

            stones.len()
        })
        .sum();

    res.to_string()
}

#[cfg(test)]
mod tests {}
