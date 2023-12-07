use std::{collections::VecDeque, error::Error};

fn to_long(string_array: &str, delim: &str) -> u128 {
    string_array
        .split(delim)
        .flat_map(str::parse::<u32>)
        .fold(0u128, |mut acc, v| {
            acc |= 1 << v;
            acc
        })
}

macro_rules! to_long {
    ($a: expr) => {
        to_long($a, " ")
    };
    ($a: expr, $b: expr) => {
        to_long($a, $b)
    };
}

// not used
fn _pop_first<T>(vec: &mut Vec<T>) -> Option<T> {
    if vec.len() == 0 {
        return None;
    }
    Some(vec.remove(0))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("official");
    let out = input
        .lines()
        .flat_map(|s| {
            let (_, cards) = s.split_once(": ")?;
            let (winning, cards) = cards.split_once(" | ")?;

            let winning = to_long!(winning);
            let cards = to_long!(cards);

            let mut won = winning & cards;
            let mut ret: u32 = 1;

            while won != 0 {
                ret <<= won & 0x1;
                won >>= 1;
            }

            Some(ret >> 1)
        })
        .sum::<u32>();
    println!("{}", out);

    let (out, _) = input
        .lines()
        .flat_map(|s| {
            let (_, cards) = s.split_once(": ")?;
            let (winning, cards) = cards.split_once(" | ")?;
            let winning = to_long!(winning);
            let cards = to_long!(cards);

            let mut won = winning & cards;
            let mut count = 0;

            while won != 0 {
                let yes = (won & 0x1) as usize;
                won >>= 1;
                count += yes;
            }

            Some(count)
        })
        .fold((0u32, VecDeque::new()), |(mut out, mut adding), count| {
            let instances = 1 + adding.pop_front().unwrap_or(0);
            out += instances;
            for i in 0..count {
                if let Some(v) = adding.get_mut(i) {
                    *v += instances;
                } else {
                    adding.push_back(instances);
                }
            }
            (out, adding)
        });
    println!("{}", out);
    Ok(())
}
