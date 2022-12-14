use std::error::Error;
mod part1;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inp_off.txt");

    println!("ex 1 {}", part1::solve(input)?);
    Ok(())
}
