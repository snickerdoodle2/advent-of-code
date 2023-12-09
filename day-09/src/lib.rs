use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, nums) = separated_list1(space1, complete::i64)(input)?;
    Ok((input, nums))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

pub fn process_1(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();
    lines
        .iter()
        .map(|line| {
            let size = line.len();
            let mut matrix = Vec::new();
            matrix.push(line.clone());
            for _ in 0..(size - 1) {
                let v = vec![0; size];
                matrix.push(v);
            }

            let mut end_idx = 0;

            for i in 1..(size) {
                let mut fin = true;
                for j in i..(size) {
                    let left = matrix.get(i - 1).unwrap().get(j - 1).unwrap().clone();
                    let right = matrix.get(i - 1).unwrap().get(j).unwrap().clone();
                    let val = matrix.get_mut(i).unwrap().get_mut(j).unwrap();
                    *val = right - left;
                    if val != & 0 {
                        fin = false;
                    }
                }
                if fin {
                    end_idx = i;
                    break;
                }
            }

            let mut res = 0;
            for i in (1..=end_idx).rev() {
                let left = matrix.get(i - 1).unwrap().last().unwrap();
                res = res + left;
            }
            res
        })
        .sum::<i64>()
        .to_string()
}

pub fn process_2(input: &str) -> String {
    let (_, lines) = parse_input(input).unwrap();
    lines
        .iter()
        .map(|line| {
            let size = line.len();
            let mut matrix = Vec::new();
            matrix.push(line.clone());
            for _ in 0..(size - 1) {
                let v = vec![0; size];
                matrix.push(v);
            }

            let mut end_idx = 0;

            for i in 1..(size) {
                let mut fin = true;
                for j in 0..(size-i) {
                    let left = matrix.get(i - 1).unwrap().get(j).unwrap().clone();
                    let right = matrix.get(i - 1).unwrap().get(j + 1).unwrap().clone();
                    let value = matrix.get_mut(i).unwrap().get_mut(j).unwrap();
                    *value = right - left;
                    if value != &0 { fin = false; }
                }
                if fin {
                    end_idx = i;
                    break;
                }
            }

            let mut res = 0;
            for i in (1..=end_idx).rev() {
                // res = right - left
                // left = right - res
                let right = matrix.get(i - 1).unwrap().first().unwrap();
                res = right - res;
            }
            res
        })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let res = process_1(input);
        assert_eq!("114", res);
    }

    #[test]
    fn test_process_2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let res = process_2(input);
        assert_eq!("2", res);
    }
}
