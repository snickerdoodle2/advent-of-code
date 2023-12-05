use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use itertools::*;
use rayon::prelude::*;

#[derive(Debug)]
enum Type {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

use Type::*;

#[derive(Debug)]
struct Map {
    from: Type,
    to: Type,
    ranges: Vec<(u32, u32, u32)>,
}

impl Map {
    fn find_next(self: &Self, from: &u32) -> u32 {
        for (dest, src, len) in &self.ranges {
            if from >= src && from < &(src + len) {
                let diff = from - src;
                return *dest + diff;
            }
        }
        *from
    }
}

fn parse_type(input: &str) -> IResult<&str, Type> {
    let (input, name) = alpha1(input)?;
    let name = match name {
        "seed" => Seed,
        "soil" => Soil,
        "fertilizer" => Fertilizer,
        "water" => Water,
        "light" => Light,
        "temperature" => Temperature,
        "humidity" => Humidity,
        "location" => Location,
        _ => panic!("Error parsing: {}", name),
    };

    Ok((input, name))
}

fn parse_range(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, first) = digit1(input).map(|(a, b)| (a, b.parse().unwrap()))?;
    let (input, second) =
        preceded(tag(" "), digit1)(input).map(|(a, b)| (a, b.parse().unwrap()))?;
    let (input, third) = preceded(tag(" "), digit1)(input).map(|(a, b)| (a, b.parse().unwrap()))?;
    Ok((input, (first, second, third)))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, (from, to)) = terminated(
        separated_pair(parse_type, tag("-to-"), parse_type),
        tuple((tag(" map:"), newline)),
    )(input)?;
    let (input, ranges) = separated_list1(newline, parse_range)(input)?;
    Ok((input, Map { from, to, ranges }))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<Map>)> {
    let mut groups = input.split("\n\n");
    let (_, seeds) =
        preceded(tag("seeds: "), separated_list1(space1, digit1))(groups.next().unwrap())?;
    let seeds = seeds.iter().map(|x| x.parse().unwrap()).collect();

    let maps = groups.map(|m| parse_map(m).unwrap().1).collect::<Vec<_>>();

    Ok((input, (seeds, maps)))
}

pub fn process_1(input: &str) -> String {
    let (_, (seeds, maps)) = parse_input(input).unwrap();
    seeds.iter().map(|seed| {
        let mut seed = *seed;
        for map in &maps {
            seed = map.find_next(&seed);
        }
        seed
    }).min().unwrap().to_string()
}

pub fn process_2(input: &str) -> String {
    let (_, (seeds, maps)) = parse_input(input).unwrap();
    let seeds = seeds.chunks(2).flat_map(|x| {
        let start = x[0];
        let stop = x[1];
        start..(start+stop)
    }).collect::<Vec<u32>>();

    // parallelize it lol
    seeds.par_iter().map(|seed| {
        let mut seed = *seed;
        for map in &maps {
            seed = map.find_next(&seed);
        }
        seed
    }).min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let res = process_1(input);
        assert_eq!("35", res);
    }

    #[test]
    fn test_process_2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let res = process_2(input);
        assert_eq!("46", res);
    }
}
