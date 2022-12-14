use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Part 2 implementation extends part1
/// Part one would benefit form this
/// Part one is kept to keep original solution

#[derive(Debug, Clone, Copy)]
struct Position(i32, i32);

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        return Position(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl<'a, 'b> Sub<&'b Position> for &'a Position {
    type Output = Position;

    fn sub(self, rhs: &'b Position) -> Self::Output {
        return Position(self.0 - rhs.0, self.1 - rhs.1);
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Position {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl Default for Position {
    fn default() -> Self {
        Position(0, 0)
    }
}

impl Position {
    fn follow(&mut self, other: &Self) -> bool {
        let diff = other - self;

        let operation = match diff {
            Position(2, a) => Some(Position(1, a)),
            Position(-2, a) => Some(Position(-1, a)),
            Position(a, 2) => Some(Position(a, 1)),
            Position(a, -2) => Some(Position(a, -1)),
            _ => None,
        };

        if let Some(op) = operation {
            *self += op;
            return true;
        }
        return false;
    }

    fn move_head(&mut self, dir: &str) -> () {
        match dir {
            "L" => *self += Position(-1, 0),
            "R" => *self += Position(1, 0),
            "D" => *self += Position(0, -1),
            "U" => *self += Position(0, 1),
            _ => (),
        };
    }
}

pub fn solve(input: &'static str) -> Result<usize, Box<dyn Error>> {
    let cmds = input
        .lines()
        .map(|line| line.split_once(" ").expect("Space present"))
        .map(|(dir, count)| (dir, count.parse::<usize>().expect("count as input")));

    let mut snake = [Position::default(); 9];
    for cmd in cmds {
        for _ in 0..cmd.1 {
            snake[0].move_head(cmd.0);
            for (i, pos) in snake.iter_mut().skip(1).enumerate() {
                pos.follow(&snake[i - 1]);
            }
        }
    }
    Ok(0)
}
