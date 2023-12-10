use petgraph::{Graph, Undirected};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Flow {
    one: Direction,
    two: Direction,
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Pipe {
    fn new(c: char) -> Self {
        use Pipe::*;
        match c {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            '.' => Ground,
            'S' => Start,
            _ => panic!("Did not match {}", c),
        }
    }

    fn get_flow(&self) -> Flow {
        use Direction::*;
        use Pipe::*;
        match self {
            Vertical => Flow {
                one: North,
                two: South,
            },
            Horizontal => Flow {
                one: East,
                two: West,
            },
            NE => Flow {
                one: North,
                two: East,
            },
            NW => Flow {
                one: North,
                two: West,
            },
            SW => Flow {
                one: South,
                two: West,
            },
            SE => Flow {
                one: South,
                two: East,
            },
            _ => panic!(":("),
        }
    }

    fn connects_to(&self, other: &Self, xdiff: i8, ydiff: i8) -> bool {
        use Direction::*;
        use Pipe::*;
        if self == &Start || other == &Start {
            return true;
        }
        let self_flow = self.get_flow();
        let other_flow = other.get_flow();
        todo!();
    }
}

pub fn process_1(input: &str) -> String {
    let mut map: HashMap<(u32, u32), (Pipe, _)> = HashMap::new();
    let mut graph: Graph<(u32, u32), u8, Undirected> = Graph::new_undirected();

    let mut height: u32 = 0;
    let mut width: u32 = 0;

    input.lines().enumerate().for_each(|(line_idx, line)| {
        let mut tmp_width = 0;
        line.chars().enumerate().for_each(|(char_idx, c)| {
            let index = graph.add_node((char_idx as u32, line_idx as u32));
            map.insert((char_idx as u32, line_idx as u32), (Pipe::new(c), index));
            tmp_width += 1;
        });

        if tmp_width > width {
            width = tmp_width;
        }
        height += 1;
    });

    for (pos, (pipe, idx)) in &map {
        use Pipe::*;
        if (pos == &(0, 0) && pipe != &SE)
            || (pos == &(width - 1, 0) && pipe != &SW)
            || (pos == &(0, height - 1) && pipe != &NE)
            || (pos == &(width - 1, height - 1) && pipe != &NW)
            || pipe == &Ground
        {
            continue;
        }

        let (x, y) = pos;

        if (*x == 0 && (pipe == &SW || pipe == &Horizontal || pipe == &NW))
            || (*y == 0 && (pipe == &Vertical || pipe == &NE || pipe == &NW))
        {
            continue;
        }

        let to_check = match pipe {
            Vertical => vec![(*x, y + 1), (*x, y - 1)],
            Horizontal => vec![(x + 1, *y), (x - 1, *y)],
            NE => vec![(*x, y - 1), (x + 1, *y)],
            NW => vec![(*x, y - 1), (x - 1, *y)],
            SW => vec![(x - 1, *y), (*x, y + 1)],
            SE => vec![(x + 1, *y), (*x, y + 1)],
            _ => vec![],
        };

        let mut valid = true;
        for pos_new in to_check {
            let other_opt = map.get(&pos_new);
            if !match other_opt {
                Some((other, _)) => pipe.connects_to(
                    other,
                    pos.0 as i8 - pos_new.0 as i8,
                    pos.1 as i8 - pos_new.1 as i8,
                ),
                None => false,
            } {
                valid = false;
            }

            if !valid {
                break;
            }
        }
    }
    todo!();
}

pub fn process_2(_input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1_1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let res = process_1(input);
        assert_eq!("4", res);
    }

    #[test]
    #[ignore]
    fn test_process_1_2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let res = process_1(input);
        assert_eq!("8", res);
    }

    #[test]
    #[ignore]
    fn test_process_2() {
        let input = "";
        let res = process_2(input);
        assert_eq!("", res);
    }
}
