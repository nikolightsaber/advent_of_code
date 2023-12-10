use std::{error::Error, num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Map {
    out: u64,
    seed: u64,
    len: u64,
}
impl Map {
    fn inside(&self, seed: u64) -> Option<u64> {
        if self.seed <= seed && seed < self.seed + self.len {
            return Some(self.out + seed - self.seed);
        }
        None
    }
}

#[derive(Debug)]
struct Adaptor {
    maps: Vec<Map>,
}

impl Adaptor {
    fn adapt(&self, seed: u64) -> u64 {
        for map in self.maps.iter() {
            if let Some(out) = map.inside(seed) {
                return out;
            }
        }
        seed
    }
}

#[derive(Debug)]
struct AdaptorParseError;
impl From<ParseIntError> for AdaptorParseError {
    fn from(_: ParseIntError) -> Self {
        AdaptorParseError
    }
}

impl FromStr for Adaptor {
    type Err = AdaptorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        iter.next().ok_or(AdaptorParseError)?;
        Ok(Adaptor {
            maps: iter
                .map(|l| {
                    let mut map = l.split_whitespace();
                    Ok(Map {
                        out: map.next().ok_or(AdaptorParseError)?.parse()?,
                        seed: map.next().ok_or(AdaptorParseError)?.parse()?,
                        len: map.next().ok_or(AdaptorParseError)?.parse()?,
                    })
                })
                .collect::<Result<_, AdaptorParseError>>()?,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("official");

    let mut input_iter = input.split("\n\n");
    let mut seeds = input_iter
        .next()
        .expect("Wrong input")
        .split_once(": ")
        .map(|(_, seeds)| {
            seeds
                .split_whitespace()
                .map(str::parse::<u64>)
                .collect::<Result<Vec<_>, _>>()
        })
        .expect("Wrong input")
        .expect("Wrong input");

    for adaptor in input_iter {
        let adaptor: Adaptor = adaptor.parse().expect("Wrong input");
        for seed in seeds.iter_mut() {
            *seed = adaptor.adapt(*seed);
        }
    }
    println!("{}", seeds.iter().min().unwrap());
    Ok(())
}
