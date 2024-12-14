use std::collections::HashMap;

use glam::I64Vec2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: I64Vec2,
    velocity: I64Vec2,
}

impl Robot {
    // NOTE: propably you could do all the iterations at once and modulo the diff??
    fn fly(&mut self, width: i64, height: i64) {
        assert!(self.pos.x >= 0 && self.pos.x <= width && self.pos.y >= 0 && self.pos.y <= height);
        self.pos += self.velocity;

        if self.pos.x < 0 {
            self.pos.x += width + 1;
        }
        if self.pos.x > width {
            self.pos.x -= width + 1;
        }
        if self.pos.y < 0 {
            self.pos.y += height + 1;
        }
        if self.pos.y > height {
            self.pos.y -= height + 1;
        }
    }
}

fn parse_nums(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(complete::i64, complete::char(','), complete::i64)(input)
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, (px, py)) = preceded(tag("p="), parse_nums)(input)?;
    let (input, (vx, vy)) = preceded(tag(" v="), parse_nums)(input)?;

    Ok((
        input,
        Robot {
            pos: I64Vec2 { x: px, y: py },
            velocity: I64Vec2 { x: vx, y: vy },
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(newline, parse_robot)(input)
}

fn show_robots(robots: &Vec<Robot>, width: i64, height: i64) {
    let mut map: HashMap<I64Vec2, Vec<Robot>> = HashMap::new();
    for r in robots {
        map.entry(r.pos).or_default().push(*r);
    }
    for y in 0..=height {
        for x in 0..=width {
            let pos = I64Vec2 { x, y };
            if let Some(count) = map.get(&pos) {
                print!("{}", count.len())
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub fn process(input: &str, mut width: i64, mut height: i64) -> String {
    width -= 1;
    height -= 1;
    let (_, mut robots) = parse(input).unwrap();

    let middle_x = width / 2;
    let middle_y = height / 2;

    let mut res = [0_usize; 4];

    for i in 1..=100 {
        for robot in robots.iter_mut() {
            robot.fly(width, height);
        }

        println!("{}", i);
        show_robots(&robots, width, height);
        println!();
        println!();
        println!();
    }

    for robot in robots {
        if robot.pos.x < middle_x && robot.pos.y < middle_y {
            res[0] += 1;
        } else if robot.pos.x > middle_x && robot.pos.y < middle_y {
            res[1] += 1;
        } else if robot.pos.x < middle_x && robot.pos.y > middle_y {
            res[2] += 1;
        } else if robot.pos.x > middle_x && robot.pos.y > middle_y {
            res[3] += 1;
        }
    }

    res.iter().product::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input, 11, 7));
    }
}
