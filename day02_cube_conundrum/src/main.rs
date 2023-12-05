use std::{error::Error, str::FromStr};

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum ParseBagError {
    InvalidInput,
    InvalidColor,
    InvalidAmount,
}

impl Bag {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn fits_in(&self, other: &Bag) -> bool {
        return self.red <= other.red && self.green <= other.green && self.blue <= other.blue;
    }

    fn add_min(&mut self, other: &Bag) {
        self.red = std::cmp::max(self.red, other.red);
        self.green = std::cmp::max(self.green, other.green);
        self.blue = std::cmp::max(self.blue, other.blue);
    }

    fn power(&self) -> u32 {
        return self.red * self.green * self.blue;
    }
}

impl FromStr for Bag {
    type Err = ParseBagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bag = Bag::new();
        for cube in s.split(", ") {
            let (nr, color) = cube.split_once(" ").ok_or(ParseBagError::InvalidInput)?;
            let nr: u32 = nr.parse().map_err(|_| ParseBagError::InvalidAmount)?;
            match color {
                "red" => bag.red = nr,
                "green" => bag.green = nr,
                "blue" => bag.blue = nr,
                _ => return Err(ParseBagError::InvalidColor),
            }
        }
        Ok(bag)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("off_inp.txt");
    let ref_bag: Bag = "12 red, 13 green, 14 blue"
        .parse()
        .map_err(|e| format!("Invalid input, fail reason: {:?}", e))?;

    let out: usize = input
        .lines()
        .enumerate()
        .filter_map(|(i, l)| {
            let (_, bags) = l.split_once(": ")?;
            for bag_str in bags.split("; ") {
                let bag: Bag = bag_str.parse().ok()?;
                if !bag.fits_in(&ref_bag) {
                    return None;
                }
            }
            return Some(i + 1);
        })
        .sum();
    println!("{}", out);

    let out: u32 = input
        .lines()
        .flat_map(|l| {
            let (_, bags) = l.split_once(": ")?;
            let mut min_bag = Bag::new();
            for bag_str in bags.split("; ") {
                let bag: Bag = bag_str.parse().ok()?;
                min_bag.add_min(&bag);
            }
            Some(min_bag.power())
        })
        .sum();
    println!("{}", out);
    Ok(())
}
