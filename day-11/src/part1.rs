use std::collections::HashMap;

type Cache = HashMap<(u8, u64), usize>;

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn mutate(stone: u64, iters: u8, cache: &mut Cache) -> usize {
    if iters == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(&(iters, stone)) {
        return *cached;
    }

    if stone == 0 {
        let res = mutate(1, iters - 1, cache);
        cache.insert((iters, stone), res);
        return res;
    }

    let n = (stone as f32).log10().floor() as u32 + 1;
    if n % 2 == 0 {
        let tmp = 10_u64.pow(n / 2);
        let res = mutate(stone / tmp, iters - 1, cache) + mutate(stone % tmp, iters - 1, cache);
        cache.insert((iters, stone), res);
        return res;
    }

    let res = mutate(stone * 2024, iters - 1, cache);
    cache.insert((iters, stone), res);
    res
}

pub fn process(input: &str) -> String {
    let stones = parse(input);

    let mut cache = Cache::new();

    let res: usize = stones
        .into_iter()
        .map(|stone| mutate(stone, 25, &mut cache))
        .sum();
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "125 17";
        let stones = parse(input);
        let mut cache = Cache::new();

        let res: usize = stones
            .into_iter()
            .map(|stone| mutate(stone, 25, &mut cache))
            .sum();

        dbg!(cache);
        assert_eq!("55312", res.to_string());
    }
}
