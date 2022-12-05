use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Piles(Vec<String>);

impl Piles {
    fn new(size: usize) -> Self {
        let piles: Vec<String> = vec![String::new(); size];
        Self(piles)
    }

    fn add_values(&mut self, s: &str) -> Result<(), ParseIntError> {
        for (i, m) in s.as_bytes().chunks(4).enumerate() {
            if let Some(v) = m.get(1) {
                if v.is_ascii_uppercase() {
                    if let Some(pile) = self.0.get_mut(i) {
                        pile.push(*v as char);
                    }
                }
            }
        }
        Ok(())
    }

    fn get_actions_from_str(s: &str) -> Result<(usize, usize, usize), &'static str> {
        let actions = s
            .split(" ")
            .skip(1)
            .step_by(2)
            .flat_map(str::parse::<usize>)
            .collect::<Vec<usize>>();

        if actions.len() != 3 {
            return Err("Invalid input");
        }
        Ok((actions[0], actions[1] - 1, actions[2] - 1))
    }

    fn apply_action_9000(&mut self, s: &str) -> Result<(), &'static str> {
        let (count, from_i, to_i) = Self::get_actions_from_str(s)?;

        for _ in 0..count {
            let from = self.0.get_mut(from_i).ok_or("Invalid Input")?;
            let v = from.pop().ok_or("Invalid input")?;
            let to = self.0.get_mut(to_i).ok_or("Invalid Input")?;
            to.push(v);
        }
        Ok(())
    }

    fn apply_action_9001(&mut self, s: &str) -> Result<(), &'static str> {
        let (count, from_i, to_i) = Self::get_actions_from_str(s)?;

        let mut v: String = String::new();
        for _ in 0..count {
            let from = self.0.get_mut(from_i).ok_or("Invalid Input")?;
            v.push(from.pop().ok_or("Invalid input")?);
        }

        for v in v.chars().rev() {
            let to = self.0.get_mut(to_i).ok_or("Invalid Input")?;
            to.push(v);
        }
        Ok(())
    }

    fn get_top_crates(&self) -> Option<String> {
        let mut ret = String::new();
        for s in self.0.iter() {
            ret.push(s.chars().last()?);
        }
        Some(ret)
    }
}

impl FromStr for Piles {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s[s.len() - 2..s.len() - 1].parse::<usize>()?))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inp_off.txt");
    let (setup_str, commands) = input.split_once("\n\n").expect("Wrong input");
    let mut setup_iter = setup_str.lines().into_iter().rev();
    let mut piles: Piles = setup_iter
        .next()
        .expect("Invalid input")
        .parse()
        .expect("Invalid input");

    for line in setup_iter {
        piles.add_values(line).expect("Invalid input");
    }

    let mut piles2 = piles.clone();
    dbg!(&piles);

    for line in commands.lines() {
        piles.apply_action_9000(line)?;
    }

    for line in commands.lines() {
        piles2.apply_action_9001(line)?;
    }

    dbg!(&piles);
    dbg!(&piles2);

    println!("ex 1: {}", piles.get_top_crates().expect("Internal Error"));
    println!("ex 2: {}", piles2.get_top_crates().expect("Internal Error"));

    Ok(())
}
