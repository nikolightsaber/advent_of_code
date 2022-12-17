use std::error::Error;
use std::str::FromStr;

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (_, items) = lines.next().ok_or(())?.split_once(":").ok_or(())?;
        let items: Vec<usize> = items
            .split(",")
            .flat_map(|item| item.parse::<usize>())
            .collect();

        let (_, operation) = lines.next().ok_or(())?.split_once(":").ok_or(())?;
        let (_, test) = lines.next().ok_or(())?.split_once(":").ok_or(())?;
        todo!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    Ok(())
}
