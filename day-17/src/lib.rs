use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn next_points(&self, grid_width: &usize, grid_height: &usize) -> Vec<(Self, Direction)> {
        use Direction::*;
        let mut res = vec![];

        if self.x > 0 {
            res.push((
                Point {
                    x: self.x - 1,
                    y: self.y,
                },
                West,
            ));
        }
        if self.y > 0 {
            res.push((
                Point {
                    x: self.x,
                    y: self.y - 1,
                },
                North,
            ));
        }
        if self.x < grid_width - 1 {
            res.push((
                Point {
                    x: self.x + 1,
                    y: self.y,
                },
                East,
            ));
        }

        if self.y < grid_height - 1 {
            res.push((
                Point {
                    x: self.x,
                    y: self.y + 1,
                },
                South,
            ));
        }
        res
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    position: Point,
    direction: Direction,
    count: u8,
}

impl Node {
    fn neighbors(&self, grid_width: &usize, grid_height: &usize) -> Vec<Node> {
        self.position
            .next_points(grid_width, grid_height)
            .into_iter()
            .filter_map(|(position, direction)| {
                // u can omit going back because weight > 0
                if direction == self.direction.opposite() {
                    None
                } else if self.direction != direction {
                    Some(Node {
                        position,
                        direction,
                        count: 1,
                    })
                } else if self.count < 3 {
                    Some(Node {
                        position,
                        direction,
                        count: self.count + 1,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn neighbors_2(&self, grid_width: &usize, grid_height: &usize) -> Vec<Node> {
        self.position
            .next_points(grid_width, grid_height)
            .into_iter()
            .filter_map(|(position, direction)| {
                // u can omit going back because weight > 0
                if direction == self.direction.opposite() {
                    None
                } else if self.direction != direction && self.count >= 4 {
                    Some(Node {
                        position,
                        direction,
                        count: 1,
                    })
                } else if self.direction == direction && self.count < 10 {
                    Some(Node {
                        position,
                        direction,
                        count: self.count + 1,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    node: Node,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // min pq
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.node.cmp(&self.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn debug_grid(grid: &HashMap<Node, usize>, width: &usize, height: &usize) {
    for y in 0..*height {
        for x in 0..*width {
            if let Some(min) = grid
                .iter()
                .filter_map(|(n, v)| {
                    if n.position == (Point { x, y }) {
                        Some(v)
                    } else {
                        None
                    }
                })
                .min()
            {
                print!("{:03} ", min);
            } else {
                print!("### ");
            }
        }
        println!();
    }
}

pub fn process_1(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: width - 1,
        y: height - 1,
    };

    let start_1 = Node {
        position: start.clone(),
        direction: Direction::South,
        count: 0,
    };

    let start_2 = Node {
        position: start.clone(),
        direction: Direction::East,
        count: 0,
    };

    let mut cache = HashMap::new();
    cache.insert(start_1.clone(), 0);
    cache.insert(start_2.clone(), 0);

    let mut pq = BinaryHeap::new();

    pq.push(State {
        node: start_1,
        cost: 0,
    });
    pq.push(State {
        node: start_2,
        cost: 0,
    });

    while let Some(State { node: u, cost }) = pq.pop() {
        if u.position == end {
            return cost.to_string();
        }
        for neighbor in u.neighbors(&width, &height) {
            let new_cost = cost + grid[neighbor.position.y][neighbor.position.x] as usize;
            if let Some(cur_cost) = cache.get(&neighbor) {
                if cur_cost >= cur_cost {
                    continue;
                }
            }

            cache.insert(neighbor.clone(), new_cost);
            pq.push(State {
                node: neighbor,
                cost: new_cost,
            });
        }
    }
    0.to_string()
}

pub fn process_2(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: width - 1,
        y: height - 1,
    };

    let start_1 = Node {
        position: start.clone(),
        direction: Direction::South,
        count: 0,
    };

    let start_2 = Node {
        position: start.clone(),
        direction: Direction::East,
        count: 0,
    };

    let mut cache = HashMap::new();
    cache.insert(start_1.clone(), 0);
    cache.insert(start_2.clone(), 0);

    let mut pq = BinaryHeap::new();

    pq.push(State {
        node: start_1,
        cost: 0,
    });
    pq.push(State {
        node: start_2,
        cost: 0,
    });

    while let Some(State { node: u, cost }) = pq.pop() {
        if u.position == end {
            if u.count < 4 {
                cache.remove(&u);
            } else {
                return cost.to_string();
            }
        }
        for neighbor in u.neighbors_2(&width, &height) {
            let new_cost = cost + grid[neighbor.position.y][neighbor.position.x] as usize;
            if let Some(cur_cost) = cache.get(&neighbor) {
                if cur_cost >= cur_cost {
                    continue;
                }
            }

            cache.insert(neighbor.clone(), new_cost);
            pq.push(State {
                node: neighbor,
                cost: new_cost,
            });
        }
    }
    0.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let res = process_1(input);
        assert_eq!("102", res);
    }

    #[test]
    fn test_process_2_1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let res = process_2(input);
        assert_eq!("94", res);
    }

    #[test]
    fn test_process_2_2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        let res = process_2(input);
        assert_eq!("71", res);
    }
}
