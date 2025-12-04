#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    data: Vec<Option<()>>,
}

impl Map {
    fn postition(&self, index: usize) -> (i32, i32) {
        let x = index % self.width;
        let y = index / self.width;
        (x as i32, y as i32)
    }

    fn get(&self, x: i32, y: i32) -> Option<()> {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            return None;
        }
        let i = (y as usize) * self.width + (x as usize);

        self.data[i]
    }
}

fn parse(input: &str) -> Map {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let data = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '@' => Some(()),
                '.' => None,
                _ => unreachable!(),
            })
        })
        .collect();

    Map {
        width,
        height,
        data,
    }
}

const DELTAS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn count_neighbors(index: usize, map: &Map) -> u64 {
    let (x, y) = map.postition(index);

    let res = DELTAS
        .into_iter()
        .filter_map(|(dx, dy)| map.get(x + dx as i32, y + dy as i32))
        .count();

    if res < 4 {
        1
    } else {
        0
    }
}

pub fn process(input: &str) -> String {
    let map = parse(input);
    map.data
        .iter()
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, _)| count_neighbors(i, &map))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
        assert_eq!("13", process(input));
    }
}
