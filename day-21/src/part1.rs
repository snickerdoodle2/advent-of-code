use std::thread::current;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Action {
    Left,
    Right,
    Up,
    Down,
    Press,
}

impl Action {
    fn to_vec(&self) -> Option<(i8, i8)> {
        match self {
            Action::Left => Some((-1, 0)),
            Action::Right => Some((1, 0)),
            Action::Up => Some((0, -1)),
            Action::Down => Some((0, 1)),
            Action::Press => None,
        }
    }
}

trait ButtonPresser {
    fn on_illegal(&self) -> bool;
    fn done(&self) -> bool;
    fn shortest_path(&self) -> Action;
    fn change_current(&mut self);
    fn move_next(&mut self, action: &Action);

    fn next_action(&mut self) -> Option<Action> {
        if self.on_illegal() {
            unreachable!(); // WE CANNOT STEP ON EMPTY SPACE
        }
        if self.done() {
            return None;
        }

        let action = self.shortest_path();
        if action == Action::Press {
            self.change_current();
        } else {
            self.move_next(&action);
        }

        Some(action)
    }
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
struct Directional<T: ButtonPresser> {
    inner: T,
    current: Option<Action>,
    cur_x: i8,
    cur_y: i8,
}

impl<T: ButtonPresser> Directional<T> {
    fn new(mut inner: T) -> Self {
        let current = inner.next_action();
        Self {
            inner,
            current,
            cur_x: 2,
            cur_y: 0,
        }
    }

    fn get_target_pos(&self) -> (i8, i8) {
        match self.current.expect("Should be called if Some") {
            Action::Up => (1, 0),
            Action::Press => (2, 0),
            Action::Left => (0, 1),
            Action::Down => (1, 1),
            Action::Right => (2, 1),
        }
    }
}

impl<T: ButtonPresser> ButtonPresser for Directional<T> {
    fn on_illegal(&self) -> bool {
        self.cur_x == 0 && self.cur_y == 0
    }

    fn done(&self) -> bool {
        self.current.is_none()
    }

    fn shortest_path(&self) -> Action {
        let (x, y) = self.get_target_pos();

        if self.cur_x == x && self.cur_y == y {
            return Action::Press;
        }

        if self.cur_y < y {
            return Action::Down;
        }

        if self.cur_x < x {
            return Action::Right;
        }

        if self.cur_x > x {
            return Action::Left;
        }

        Action::Up
    }

    fn change_current(&mut self) {
        self.current = self.inner.next_action();
    }

    fn move_next(&mut self, action: &Action) {
        let (dx, dy) = action.to_vec().expect("Not press");
        self.cur_x += dx;
        self.cur_y += dy;
    }
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
struct Numeric<'a> {
    code: Box<dyn Iterator<Item = char> + 'a>,
    current: Option<char>,
    cur_x: i8,
    cur_y: i8,
}

impl<'a> Numeric<'a> {
    fn new(input: &'a str) -> Self {
        let mut code = input.chars();
        let current = code.next();

        Self {
            code: Box::new(code),
            current,
            cur_x: 2,
            cur_y: 3,
        }
    }

    fn get_target_pos(&self) -> (i8, i8) {
        match self.current.expect("Should be called if Some") {
            '7' => (0, 0),
            '8' => (1, 0),
            '9' => (2, 0),
            '4' => (0, 1),
            '5' => (1, 1),
            '6' => (2, 1),
            '1' => (0, 2),
            '2' => (1, 2),
            '3' => (2, 2),
            '0' => (1, 3),
            'A' => (2, 3),
            _ => unreachable!(),
        }
    }
}

impl ButtonPresser for Numeric<'_> {
    fn on_illegal(&self) -> bool {
        self.cur_x == 0 && self.cur_y == 3
    }

    fn done(&self) -> bool {
        self.current.is_none()
    }

    fn shortest_path(&self) -> Action {
        let (x, y) = self.get_target_pos();
        if self.cur_x == x && self.cur_y == y {
            return Action::Press;
        }
        if self.cur_y > y {
            return Action::Up;
        }
        if self.cur_x < x {
            return Action::Right;
        }
        if self.cur_x > x {
            return Action::Left;
        }

        Action::Down
    }

    fn change_current(&mut self) {
        self.current = self.code.next();
    }

    fn move_next(&mut self, action: &Action) {
        let (dx, dy) = action.to_vec().expect("Not press");
        self.cur_x += dx;
        self.cur_y += dy;
    }
}

fn get_min_count(code: &'static str) -> usize {
    let n = Numeric::new(code);
    let r = Directional::new(n);
    let mut r = Directional::new(r);
    let mut res = 0;
    while let Some(a) = r.next_action() {
        let c = match a {
            Action::Left => '<',
            Action::Right => '>',
            Action::Up => '^',
            Action::Down => 'v',
            Action::Press => 'A',
        };
        print!("{c}");
        res += 1;
    }
    println!();
    res
}

fn get_complexity(input: &str) -> u32 {
    let mut res = 0;
    for c in input.chars() {
        if let Some(digit) = c.to_digit(10) {
            res = res * 10 + digit;
        }
    }

    res
}

pub fn process(input: &'static str) -> String {
    input
        .lines()
        .map(|code| {
            let shortest = get_min_count(code);
            let complexity = get_complexity(code);
            shortest * complexity as usize
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("029A", 68)]
    #[case("980A", 60)]
    #[case("179A", 68)]
    #[case("456A", 64)]
    #[case("379A", 64)]
    fn test_shortest_sequence(#[case] input: &'static str, #[case] res: usize) {
        assert_eq!(res, get_min_count(input));
    }

    #[rstest]
    #[case("029A", 29)]
    #[case("980A", 980)]
    #[case("179A", 179)]
    #[case("456A", 456)]
    #[case("379A", 379)]
    fn test_complexity(#[case] input: &str, #[case] res: u32) {
        assert_eq!(res, get_complexity(input));
    }

    #[test]
    fn test_part1() {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("126384", process(input));
    }
}
