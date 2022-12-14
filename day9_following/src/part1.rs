use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone)]
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

pub fn solve(input: &'static str) -> Result<usize, Box<dyn Error>> {
    let cmds = input
        .lines()
        .map(|line| line.split_once(" ").expect("Space present"))
        .map(|(dir, count)| (dir, count.parse::<usize>().expect("count as input")));

    let mut head = Position(0, 0);
    let mut tail = Position(0, 0);
    let mut pos: HashSet<Position> = HashSet::new();
    pos.insert(Position(0, 0));

    for cmd in cmds {
        for _ in 0..cmd.1 {
            match cmd.0 {
                "L" => head += Position(-1, 0),
                "R" => head += Position(1, 0),
                "D" => head += Position(0, -1),
                "U" => head += Position(0, 1),
                _ => (),
            }
            let diff = &head - &tail;

            let operation = match diff {
                Position(2, a) => Some(Position(1, a)),
                Position(-2, a) => Some(Position(-1, a)),
                Position(a, 2) => Some(Position(a, 1)),
                Position(a, -2) => Some(Position(a, -1)),
                _ => None,
            };

            if let Some(op) = operation {
                tail += op;
                pos.insert(tail.clone());
            }
            /* println!(
                "move {}, head: {:?}, tail {:?}, diff: {:?}, move: {:?}",
                cmd.0, head, tail, diff, operation
            ); */
        }
    }
    // dbg!(&pos);
    Ok(pos.len())
}
