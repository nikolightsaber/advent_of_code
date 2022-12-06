use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn sol1() -> Result<(), Box<dyn Error>> {
    let file: Box<dyn BufRead> = match env::args().nth(1) {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut calories: Vec<usize> = vec![0];

    for line in file.lines() {
        if let Ok(val) = line?.parse::<usize>() {
            *calories.last_mut().unwrap() += val;
        } else {
            calories.push(0)
        }
    }

    println!("Max is {}", calories.iter().max().unwrap());
    calories.sort_by(|a, b| b.cmp(a));
    println!("Top 3 Max is {}", calories.iter().take(3).sum::<usize>());

    Ok(())
}

fn sol2() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("./../imp_official.txt");
    let mut calories = inp
        .split("\n\n")
        .map(|substr| substr.lines().flat_map(str::parse::<usize>).sum::<usize>())
        .collect::<Vec<usize>>();

    println!("Max is {}", calories.iter().max().unwrap());
    calories.sort_by(|a, b| b.cmp(a));
    println!("Top 3 Max is {}", calories.iter().take(3).sum::<usize>());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    sol1()?;
    sol2()?;
    Ok(())
}
