#[derive(Debug)]
enum Problem {
    Add(Vec<u64>),
    Mult(Vec<u64>),
}

fn parse(input: &str) -> Vec<Problem> {
    let mut lines = input.lines().map(|l| l.chars().rev()).rev();
    let mut op_line = lines.next().unwrap();
    let mut lines: Vec<_> = lines.collect();

    let mut res = vec![];
    let mut items = vec![];

    loop {
        let Some(op) = op_line.next() else {
            break;
        };

        let mut num: u64 = 0;
        let mut place = 0;
        for line in lines.iter_mut() {
            if let Some(digit) = line.next().unwrap().to_digit(10) {
                num += (digit as u64) * 10u64.pow(place);
                place += 1;
            }
        }
        items.push(num);

        let cont = match op {
            '+' => {
                res.push(Problem::Add(items.clone()));
                false
            }
            '*' => {
                res.push(Problem::Mult(items.clone()));
                false
            }
            ' ' => true,
            _ => unreachable!(),
        };

        if !cont {
            items.clear();
            op_line.next();
            lines.iter_mut().for_each(|l| {
                l.next();
            });
        }
    }
    res
}

pub fn process(input: &str) -> String {
    let problems = parse(input);

    problems
        .iter()
        .map(|p| match p {
            Problem::Add(items) => items.iter().sum::<u64>(),
            Problem::Mult(items) => items.iter().product::<u64>(),
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .join("\n");
        assert_eq!("3263827", process(&input));
    }
}
