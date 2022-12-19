use std::error::Error;

mod part1;
mod part2;

fn main() -> Result<(), Box<dyn Error>> {
    println!("ex 1: {}", part1::solve()?);
    println!("ex 2: {}", part2::solve()?);

    Ok(())
}
