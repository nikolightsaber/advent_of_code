use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("official");
    let out = input
        .lines()
        .flat_map(|s| {
            let (_, cards) = s.split_once(": ")?;
            let (winning, cards) = cards.split_once(" | ")?;
            let winning =
                winning
                    .split(" ")
                    .flat_map(str::parse::<u32>)
                    .fold(0u128, |mut acc, v| {
                        acc |= 1 << v;
                        acc
                    });
            let cards = cards
                .split(" ")
                .flat_map(str::parse::<u32>)
                .fold(0u128, |mut acc, v| {
                    acc |= 1 << v;
                    acc
                });

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
    Ok(())
}
