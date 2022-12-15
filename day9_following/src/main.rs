use std::error::Error;
mod part1;
mod part2;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inp_test2.txt");

    println!("ex 1 {}", part1::solve(input)?);
    println!("ex 1 {}", part2::solve(input)?);
    Ok(())
}
