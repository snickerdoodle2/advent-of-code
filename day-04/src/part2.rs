fn get_opp(c: char) -> Option<char> {
    match c {
        'M' => Some('S'),
        'S' => Some('M'),
        _ => None,
    }
}
fn check_xmas(data: &Vec<Vec<char>>, x: usize, y: usize) -> Option<()> {
    if 'A' != *data.get(y)?.get(x)? {
        return None;
    }

    if x == 0 || y == 0 {
        return None;
    }

    let upper_left = *data.get(y - 1)?.get(x - 1)?;
    let upper_left_opposing = get_opp(upper_left)?;

    if upper_left_opposing != *data.get(y + 1)?.get(x + 1)? {
        return None;
    }

    let upper_right = *data.get(y - 1)?.get(x + 1)?;
    let upper_right_opposing = get_opp(upper_right)?;

    if upper_right_opposing != *data.get(y + 1)?.get(x - 1)? {
        None
    } else {
        Some(())
    }
}

pub fn process(input: &str) -> String {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut res = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'A' {
                if let Some(_) = check_xmas(&input, x, y) {
                    res += 1;
                }
            }
        }
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!("9", process(input));
    }
}
