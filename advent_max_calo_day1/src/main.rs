use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
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
    Ok(())
}
