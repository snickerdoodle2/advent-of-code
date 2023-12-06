use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, multispace1, newline},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

struct Race {
    time: u64,
    distance: u64,
}

fn parse_list(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, nums) = separated_list1(multispace1, complete::u64)(input)?;
    Ok((input, nums))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = preceded(tuple((tag("Time:"), multispace1)), parse_list)(input)?;
    let (input, distances) =
        preceded(tuple((newline, tag("Distance:"), multispace1)), parse_list)(input)?;
    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect();
    Ok((input, races))
}

fn parse_list_2(input: &str) -> IResult<&str, u64> {
    let (input, nums) = separated_list1(multispace1, digit1)(input)?;
    let nums = nums
        .iter()
        .fold("".to_string(), |acc, n| acc + n)
        .parse()
        .unwrap();
    Ok((input, nums))
}

fn parse_input_2(input: &str) -> IResult<&str, Race> {
    let (input, time) = preceded(tuple((tag("Time:"), multispace1)), parse_list_2)(input)?;
    let (input, distance) = preceded(
        tuple((newline, tag("Distance:"), multispace1)),
        parse_list_2,
    )(input)?;
    Ok((input, Race { time, distance }))
}

pub fn process_1(input: &str) -> String {
    let (_, races) = parse_input(input).unwrap();
    races
        .iter()
        .map(|Race { time, distance }| {
            (0..=*time)
                .filter_map(|dur| {
                    let time_left = *time - dur;
                    let travelled = time_left * dur;
                    if travelled > *distance {
                        Some(travelled)
                    } else {
                        None
                    }
                })
                .count()
        })
        .product::<usize>()
        .to_string()
}

pub fn process_2(input: &str) -> String {
    let (_, Race { time, distance }) = parse_input_2(input).unwrap();
    (0..=time)
        .filter_map(|dur| {
            let time_left = time - dur;
            let travelled = time_left * dur;
            if travelled > distance {
                Some(travelled)
            } else {
                None
            }
        })
        .count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let res = process_1(input);
        assert_eq!("288", res);
    }

    #[test]
    fn test_process_2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let res = process_2(input);
        assert_eq!("71503", res);
    }
}
