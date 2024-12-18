use std::{
    collections::{HashSet, VecDeque},
    isize,
};

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Option<()>>>, path: Option<&Vec<(usize, usize)>>) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if let Some(path) = path {
                if path.contains(&(x, y)) {
                    print!("O");
                    continue;
                }
            }
            let c = match c {
                Some(_) => '#',
                None => '.',
            };
            print!("{c}");
        }
        println!();
    }
}

fn in_bounds(x: isize, y: isize, width: usize, height: usize) -> bool {
    x >= 0 && (x as usize) < width && y >= 0 && (y as usize) < height
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd)]
struct Path(isize, isize, Vec<(usize, usize)>);

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.len().cmp(&other.2.len())
    }
}

fn find_path(
    map: &Vec<Vec<Option<()>>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    width: usize,
    height: usize,
) -> Option<usize> {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut queue: VecDeque<(isize, isize, usize)> = VecDeque::new();
    queue.push_back((start_x as isize, start_y as isize, 0));

    while let Some((xi, yi, path)) = queue.pop_front() {
        if visited.contains(&(xi, yi))
            || !in_bounds(xi, yi, width, height)
            || map[yi as usize][xi as usize].is_some()
        {
            continue;
        }

        visited.insert((xi, yi));
        let x = xi as usize;
        let y = yi as usize;
        if x == end_x && y == end_y {
            return Some(path);
        }

        queue.push_back((xi - 1, yi, path + 1));
        queue.push_back((xi + 1, yi, path + 1));
        queue.push_back((xi, yi - 1, path + 1));
        queue.push_back((xi, yi + 1, path + 1));
    }

    None
}

fn parse_line(line: &str) -> (usize, usize) {
    let mut split = line.split(",");
    let x: usize = split
        .next()
        .expect("Should have X coordinate")
        .parse()
        .expect("Should be positive integer");
    let y: usize = split
        .next()
        .expect("Should have Y coordinate")
        .parse()
        .expect("Should be positive integer");

    (x, y)
}

pub fn process(input: &str, width: usize, height: usize) -> String {
    let (start_x, start_y) = (0, 0);
    let (end_x, end_y) = (width - 1, height - 1);
    let bytes: Vec<_> = input.lines().map(parse_line).collect();

    let mut l = 0;
    let mut r = bytes.len() - 1;
    while l <= r {
        let mut map: Vec<Vec<Option<()>>> = vec![vec![None; width]; height];
        let candidate = (l + r) / 2;
        for i in 0..=candidate {
            let (x, y) = bytes[i];
            map[y][x] = Some(());
        }

        if find_path(&map, start_x, start_y, end_x, end_y, width, height).is_some() {
            l = candidate + 1;
        } else {
            r = candidate - 1;
        }
    }

    let res = bytes[l];

    format!("{},{}", res.0, res.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("6,1", process(input, 7, 7));
    }
}
