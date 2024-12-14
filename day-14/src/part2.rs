use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufWriter},
    path::Path,
};

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

fn show_robots(robots: &Vec<Robot>, width: i64, height: i64) -> Vec<u8> {
    let mut map: HashMap<I64Vec2, Vec<Robot>> = HashMap::new();
    for r in robots {
        map.entry(r.pos).or_default().push(*r);
    }
    let mut res = Vec::with_capacity(height as usize * width as usize + 2);
    for y in 0..=height {
        for x in 0..=width {
            let pos = I64Vec2 { x, y };
            if let Some(_) = map.get(&pos) {
                res.push(255);
            } else {
                res.push(0);
            }
        }
    }

    res
}

fn write_image(image_data: &[u8], iteration: usize, width: u32, height: u32) {
    let path_str = format!("./output/{}.png", iteration);
    let path = Path::new(&path_str);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width + 1, height + 1);
    encoder.set_color(png::ColorType::Grayscale);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(image_data).unwrap();
}

pub fn process(input: &str, mut width: i64, mut height: i64, max_iters: usize) {
    width -= 1;
    height -= 1;
    let (_, mut robots) = parse(input).unwrap();

    for second in 0..max_iters {
        let img_data = show_robots(&robots, width, height);
        write_image(&img_data, second, width as u32, height as u32);
        for robot in robots.iter_mut() {
            robot.fly(width, height);
        }
    }
}
