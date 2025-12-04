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

    fn remove_many(&mut self, indexes: &[usize]) {
        for i in indexes {
            self.data[*i] = None;
        }
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

fn is_accessible(index: usize, map: &Map) -> bool {
    let (x, y) = map.postition(index);

    let res = DELTAS
        .into_iter()
        .filter_map(|(dx, dy)| map.get(x + dx as i32, y + dy as i32))
        .count();

    res < 4
}

pub fn process(input: &str) -> String {
    let mut map = parse(input);
    let mut res = 0;
    let mut to_remove = vec![];

    loop {
        let mut removed = false;
        for (i, x) in map.data.iter().enumerate() {
            if x.is_none() {
                continue;
            };
            if is_accessible(i, &map) {
                removed = true;
                res += 1;
                to_remove.push(i);
            }
        }

        if !removed {
            break;
        }

        map.remove_many(&to_remove);
        to_remove.clear();
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
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
        assert_eq!("43", process(input));
    }
}
